// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $04C880: jsr $28cd4.l
pub fn i_04c880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C886;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C886);
    cpu.pc = 0x28CD4;
    16
}
/// $04C886: btst.b #$0, $3a(a6)
pub fn i_04c886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C88C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C88C: bne.w $4c892
pub fn i_04c88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C890;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C892; return 10; }
    8
}
/// $04C890: neg.w d0
pub fn i_04c890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C892;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04C892: move.w d0, $28(a6)
pub fn i_04c892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C896;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C896: move.w #$1cf, d1
pub fn i_04c896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C89A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C89A: jsr $236e.l
pub fn i_04c89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8A0);
    cpu.pc = 0x236E;
    16
}
/// $04C8A0: bset.b #$4, $6b(a6)
pub fn i_04c8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8A6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C8A6: move.w #$d000, d0
pub fn i_04c8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C8AA: jsr $28134.l
pub fn i_04c8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8B0);
    cpu.pc = 0x28134;
    16
}
/// $04C8B0: andi.w #$ffe3, $38(a6)
pub fn i_04c8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8B6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C8B6: ori.w #$14, $38(a6)
pub fn i_04c8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8BC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C8BC: lea.l $4c8c2(pc), a1
pub fn i_04c8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C8C2;
    16
}
/// $04C8C0: move.l a1, (a6)
pub fn i_04c8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C8C2: jsr $27cee.l
pub fn i_04c8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8C8);
    cpu.pc = 0x27CEE;
    16
}
/// $04C8C8: bcs.w $4c8d6
pub fn i_04c8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4C8D6; return 10; }
    8
}
/// $04C8CC: jsr $27cee.l
pub fn i_04c8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8D2);
    cpu.pc = 0x27CEE;
    16
}
/// $04C8D2: bcc.w $4c8dc
pub fn i_04c8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C8DC; return 10; }
    8
}
/// $04C8D6: lea.l $4c910(pc), a1
pub fn i_04c8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C910;
    16
}
/// $04C8DA: move.l a1, (a6)
pub fn i_04c8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C8DC: jsr $28d70.l
pub fn i_04c8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8E2);
    cpu.pc = 0x28D70;
    16
}
/// $04C8E2: jsr $283d8.l
pub fn i_04c8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8E8);
    cpu.pc = 0x283D8;
    16
}
/// $04C8E8: btst.b #$1, $13(a6)
pub fn i_04c8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8EE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C8EE: beq.w $4c8f8
pub fn i_04c8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C8F8; return 10; }
    8
}
/// $04C8F2: lea.l $4c910(pc), a1
pub fn i_04c8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C910;
    16
}
/// $04C8F6: move.l a1, (a6)
pub fn i_04c8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C8F8: movea.l #$ffffffff, a0
pub fn i_04c8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8FE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C8FE: jsr $5dd56.l
pub fn i_04c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C904);
    cpu.pc = 0x5DD56;
    16
}
/// $04C904: bcc.w $4c90e
pub fn i_04c904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C908;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C90E; return 10; }
    8
}
// ===== SetTaskHandler_04c908  ($04C908, 8 B, kernel/task_handlers.c)
/// $04C908: lea.l $4c934(pc), a1
pub fn i_04c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C90C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C934;
    16
}
/// $04C90C: move.l a1, (a6)
pub fn i_04c90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C90E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C90E: rts
pub fn i_04c90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C910;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_FlyingPartLand_04c910  ($04C910, 36 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C910: move.w #$4000, d0
pub fn i_04c910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C914;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C914: jsr $28134.l
pub fn i_04c914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C91A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C91A);
    cpu.pc = 0x28134;
    16
}
/// $04C91A: andi.w #$ffe3, $38(a6)
pub fn i_04c91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C920;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C920: ori.w #$0, $38(a6)
pub fn i_04c920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C926;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C926: move.l #$ffffffff, $48(a6)
pub fn i_04c926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C92E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04C92E: jmp $77f6a.l
pub fn i_04c92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C934;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Jsr5B6ThenJmpScheduler_04c934  ($04C934, 14 B, kernel/jsr5b6_jmp518.c)
/// $04C934: jsr $5b6.l
pub fn i_04c934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C93A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C93A);
    cpu.pc = 0x5B6;
    16
}
/// $04C93A: jmp $518.l
pub fn i_04c93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C940;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04C940: rts
pub fn i_04c940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C942;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_FreeIfParentDead_04c942  ($04C942, 14 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C942: movea.l $c(a6), a0
pub fn i_04c942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C946;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C946: cmpi.b #$ff, $20(a0)
pub fn i_04c946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C94C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04C94C: bne.w $4c956
pub fn i_04c94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C950;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C956; return 10; }
    8
}
// ===== SetTaskHandler_04c950  ($04C950, 8 B, kernel/task_handlers.c)
/// $04C950: lea.l $4c58c(pc), a1
pub fn i_04c950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C954;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C954: move.l a1, (a6)
pub fn i_04c954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C956;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C956: rts
pub fn i_04c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C958;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_PartHitCheck_04c958  ($04C958, 50 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C958: jsr $2870a.l
pub fn i_04c958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C95E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C95E);
    cpu.pc = 0x2870A;
    16
}
/// $04C95E: bcc.w $4c974
pub fn i_04c95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C962;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C974; return 10; }
    8
}
/// $04C962: lea.l $5e766.l, a0
pub fn i_04c962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C968;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04C968: jsr $5e770.l
pub fn i_04c968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C96E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C96E);
    cpu.pc = 0x5E770;
    16
}
/// $04C96E: bclr.b #$3, $13(a6)
pub fn i_04c96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C974;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C974: jsr $28758.l
pub fn i_04c974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C97A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C97A);
    cpu.pc = 0x28758;
    16
}
/// $04C97A: bcc.w $4c988
pub fn i_04c97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C97E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C988; return 10; }
    8
}
/// $04C97E: movea.l $c(a6), a0
pub fn i_04c97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C982;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C982: move.b #$ff, $20(a0)
pub fn i_04c982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C988;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C988: rts
pub fn i_04c988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C98A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpriteByStateVariant_04c98a  ($04C98A, 28 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C98A: move.b $21(a6), d1
pub fn i_04c98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C98E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04C98E: andi.w #$ff, d1
pub fn i_04c98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C992;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C992: add.w d1, d1
pub fn i_04c992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C994;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C994: add.w d1, d1
pub fn i_04c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C996;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C996: movea.l (a1, d1.w), a0
pub fn i_04c996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C99A;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C99A: move.w $70(a6), d0
pub fn i_04c99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C99E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C99E: add.w d0, d0
pub fn i_04c99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9A0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9A0: add.w d0, d0
pub fn i_04c9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9A2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9A2: movea.l (a0, d0.w), a0
pub fn i_04c9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9A6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
// ===== JsrAbsThunk_04c9a6  ($04C9A6, 8 B, thunks/jsr_abs_thunks.c)
/// $04C9A6: jsr $28cd4.l
pub fn i_04c9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C9AC);
    cpu.pc = 0x28CD4;
    16
}
/// $04C9AC: rts
pub fn i_04c9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpriteByParentState_04c9ae  ($04C9AE, 32 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C9AE: movea.l $c(a6), a0
pub fn i_04c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9B2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C9B2: move.b $21(a0), d1
pub fn i_04c9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9B6;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04C9B6: andi.w #$ff, d1
pub fn i_04c9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9BA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C9BA: add.w d1, d1
pub fn i_04c9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9BC;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9BC: add.w d1, d1
pub fn i_04c9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9BE;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9BE: movea.l (a1, d1.w), a0
pub fn i_04c9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9C2;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C9C2: move.w $70(a6), d0
pub fn i_04c9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9C6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C9C6: add.w d0, d0
pub fn i_04c9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9C8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9C8: add.w d0, d0
pub fn i_04c9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9CA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9CA: movea.l (a0, d0.w), a0
pub fn i_04c9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9CE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
// ===== JsrAbsThunk_04c9ce  ($04C9CE, 8 B, thunks/jsr_abs_thunks.c)
/// $04C9CE: jsr $28cd4.l
pub fn i_04c9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C9D4);
    cpu.pc = 0x28CD4;
    16
}
/// $04C9D4: rts
pub fn i_04c9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_TickTimer_04c9d6  ($04C9D6, 12 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C9D6: subq.w #$1, $80(a6)
pub fn i_04c9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9DA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C9DA: bgt.w $4c9e8
pub fn i_04c9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4C9E8; return 10; }
    8
}
/// $04C9DE: clr.w $80(a6)
pub fn i_04c9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9E2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetXN_04c9e2  ($04C9E2, 6 B, thunks/ccr_helpers.c)
/// $04C9E2: ori.b #$11, ccr
pub fn i_04c9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9E6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04C9E6: rts
pub fn i_04c9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04c9e8  ($04C9E8, 6 B, thunks/ccr_helpers.c)
/// $04C9E8: andi.b #$ee, ccr
pub fn i_04c9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04C9EC: rts
pub fn i_04c9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_TickFireInterval_04c9ee  ($04C9EE, 20 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C9EE: addq.w #$1, $86(a6)
pub fn i_04c9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9F2;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C9F2: move.w $86(a6), d0
pub fn i_04c9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9F6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C9F6: cmp.w $84(a6), d0
pub fn i_04c9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9FA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04C9FA: blt.w $4ca08
pub fn i_04c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9FE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4CA08; return 10; }
    8
}
/// $04C9FE: clr.w $86(a6)
pub fn i_04c9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetXN_04ca02  ($04CA02, 6 B, thunks/ccr_helpers.c)
/// $04CA02: ori.b #$11, ccr
pub fn i_04ca02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA06;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04CA06: rts
pub fn i_04ca06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04ca08  ($04CA08, 6 B, thunks/ccr_helpers.c)
/// $04CA08: andi.b #$ee, ccr
pub fn i_04ca08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA0C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04CA0C: rts
pub fn i_04ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_LoadIdleTimer_04ca0e  ($04CA0E, 22 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA0E: lea.l $2bd260.l, a0
pub fn i_04ca0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA14;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD260;
    20
}
/// $04CA14: jsr $799de.l
pub fn i_04ca14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA1A);
    cpu.pc = 0x799DE;
    16
}
/// $04CA1A: move.w d0, $80(a6)
pub fn i_04ca1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CA1E: clr.b $8a(a6)
pub fn i_04ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA22;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CA22: rts
pub fn i_04ca22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_LoadBurstParams_04ca24  ($04CA24, 38 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA24: lea.l $2bd364.l, a0
pub fn i_04ca24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD364;
    20
}
/// $04CA2A: jsr $799de.l
pub fn i_04ca2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA30);
    cpu.pc = 0x799DE;
    16
}
/// $04CA30: move.b d0, $82(a6)
pub fn i_04ca30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA34;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CA34: lea.l $2bd2e2.l, a0
pub fn i_04ca34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD2E2;
    20
}
/// $04CA3A: jsr $799de.l
pub fn i_04ca3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA40);
    cpu.pc = 0x799DE;
    16
}
/// $04CA40: move.w d0, $84(a6)
pub fn i_04ca40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA44;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CA44: clr.w $86(a6)
pub fn i_04ca44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA48;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CA48: rts
pub fn i_04ca48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_RegisterKill_04ca4a  ($04CA4A, 6 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA4A: lea.l $290e6c.l, a1
pub fn i_04ca4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290E6C;
    20
}
// ===== JsrAbsThunk_04ca50  ($04CA50, 8 B, thunks/jsr_abs_thunks.c)
/// $04CA50: jsr $43fac.l
pub fn i_04ca50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA56);
    cpu.pc = 0x43FAC;
    16
}
/// $04CA56: rts
pub fn i_04ca56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpawnExplosionMusic_04ca58  ($04CA58, 36 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA58: move.w #$1067, d0
pub fn i_04ca58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA5C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1067;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CA5C: jsr $2352.l
pub fn i_04ca5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA62);
    cpu.pc = 0x2352;
    16
}
/// $04CA62: lea.l $620da.l, a1
pub fn i_04ca62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x620DA;
    20
}
/// $04CA68: jsr $4ae.l
pub fn i_04ca68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA6E);
    cpu.pc = 0x4AE;
    16
}
/// $04CA6E: jsr $5dd02.l
pub fn i_04ca6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA74);
    cpu.pc = 0x5DD02;
    16
}
/// $04CA74: subi.w #$20, $24(a0)
pub fn i_04ca74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA7A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04CA7A: rts
pub fn i_04ca7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpawnFlyingPart_04ca7c  ($04CA7C, 28 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA7C: lea.l $4c86e(pc), a1
pub fn i_04ca7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C86E;
    16
}
/// $04CA80: jsr $4ae.l
pub fn i_04ca80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA86);
    cpu.pc = 0x4AE;
    16
}
/// $04CA86: jsr $5dd02.l
pub fn i_04ca86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA8C);
    cpu.pc = 0x5DD02;
    16
}
/// $04CA8C: subi.w #$20, $22(a0)
pub fn i_04ca8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA92;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04CA92: addq.w #$4, $24(a0)
pub fn i_04ca92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA96;
    cpu.ir = 0x5868;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CA96: rts
pub fn i_04ca96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SndByVariant_04ca98  ($04CA98, 8 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA98: move.b $98(a6), d0
pub fn i_04ca98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA9C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CA9C: move.w #$99, d1
pub fn i_04ca9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAA0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_04caa0  ($04CAA0, 8 B, thunks/jsr_abs_thunks.c)
/// $04CAA0: jsr $9a7aa.l
pub fn i_04caa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CAA6);
    cpu.pc = 0x9A7AA;
    16
}
/// $04CAA6: rts
pub fn i_04caa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SlotPrioCheck_04caa8  ($04CAA8, 16 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CAA8: movea.l $8(a6), a1
pub fn i_04caa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAAC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04CAAC: move.b $10(a6), d0
pub fn i_04caac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAB0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CAB0: cmp.b $10(a1), d0
pub fn i_04cab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAB4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04CAB4: bcs.w $4cabe
pub fn i_04cab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAB8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CABE; return 10; }
    8
}
// ===== ClearXN_04cab8  ($04CAB8, 6 B, thunks/ccr_helpers.c)
/// $04CAB8: andi.b #$ee, ccr
pub fn i_04cab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CABC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04CABC: rts
pub fn i_04cabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CABE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04cabe  ($04CABE, 6 B, thunks/ccr_helpers.c)
/// $04CABE: ori.b #$11, ccr
pub fn i_04cabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAC2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04CAC2: rts
pub fn i_04cac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpawnStream_ReadNext_04cac4  ($04CAC4, 128 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CAC4: movea.l $1081b2.l, a0
pub fn i_04cac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CACA;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x1081B2);
    cpu.a[0] = v2;
    20
}
/// $04CACA: cmpa.w #$0, a0
pub fn i_04caca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CACE;
    cpu.ir = 0xB0FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, (v1 as u16 as i16 as i32 as u32), Size::Long, false);
    6
}
/// $04CACE: bpl.w $4caec
pub fn i_04cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAD2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4CAEC; return 10; }
    8
}
/// $04CAD2: move.w #$50, d0
pub fn i_04cad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAD6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CAD6: move.w #$1f8, d1
pub fn i_04cad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CADA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CADA: move.b #$0, d2
pub fn i_04cada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CADE;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04CADE: move.w #$10, d3
pub fn i_04cade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE2;
    cpu.ir = 0x363C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CAE2: clr.b d4
pub fn i_04cae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE4;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE4: clr.b d5
pub fn i_04cae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE6;
    cpu.ir = 0x4205;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE6: clr.b d6
pub fn i_04cae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE8;
    cpu.ir = 0x4206;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE8: bra.w $4cb42
pub fn i_04cae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x4CB42;
    10
}
/// $04CAEC: move.w $106f5c.l, d0
pub fn i_04caec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAF2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04CAF2: cmp.w $10(a0), d0
pub fn i_04caf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAF6;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04CAF6: bcs.w $4cb04
pub fn i_04caf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAFA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CB04; return 10; }
    8
}
/// $04CAFA: adda.w #$10, a0
pub fn i_04cafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAFE;
    cpu.ir = 0xD0FC;
    let v1: u32 = 0x10;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $04CAFE: move.l a0, $1081b2.l
pub fn i_04cafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB04;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x1081B2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04CB04: move.w $4(a0), d0
pub fn i_04cb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB08;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB08: tst.b $2(a0)
pub fn i_04cb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB0C;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04CB0C: beq.w $4cb16
pub fn i_04cb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB10;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CB16; return 10; }
    8
}
/// $04CB10: sub.w $106f50.l, d0
pub fn i_04cb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB16;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04CB16: move.w $6(a0), d1
pub fn i_04cb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB1A;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB1A: tst.b $3(a0)
pub fn i_04cb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB1E;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04CB1E: beq.w $4cb28
pub fn i_04cb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CB28; return 10; }
    8
}
/// $04CB22: sub.w $106f54.l, d1
pub fn i_04cb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB28;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04CB28: neg.w d1
pub fn i_04cb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB2A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04CB2A: addi.w #$200, d1
pub fn i_04cb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB2E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04CB2E: move.w $8(a0), d2
pub fn i_04cb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB32;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB32: move.w $a(a0), d3
pub fn i_04cb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB36;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB36: move.b $c(a0), d4
pub fn i_04cb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB3A;
    cpu.ir = 0x1828;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB3A: move.b $d(a0), d5
pub fn i_04cb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB3E;
    cpu.ir = 0x1A28;
    let ad2 = cpu.a[0].wrapping_add(0xD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB3E: move.b $e(a0), d6
pub fn i_04cb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB42;
    cpu.ir = 0x1C28;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB42: rts
pub fn i_04cb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Table_LoadPtrByIdxClamp6_04CB5C  ($04CB5C, 44 B, kernel/table_load_ptr_by_idx_04cb5c.s)
/// $04CB5C: move.l #$ffffffff, $1081b2.l
pub fn i_04cb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB66;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xFFFFFFFF;
    bus.write32(0x1081B2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04CB66: moveq #$0, d0
pub fn i_04cb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB68;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CB68: move.b $106ece.l, d0
pub fn i_04cb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB6E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $04CB6E: cmpi.b #$6, d0
pub fn i_04cb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB72;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $04CB72: bcc.w $4cb86
pub fn i_04cb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CB86; return 10; }
    8
}
/// $04CB76: lea.l $4cb44(pc), a0
pub fn i_04cb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4CB44;
    16
}
/// $04CB7A: add.w d0, d0
pub fn i_04cb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04CB7C: add.w d0, d0
pub fn i_04cb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04CB7E: move.l (a0, d0.w), $1081b2.l
pub fn i_04cb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB86;
    cpu.ir = 0x23F0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(0x1081B2, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    34
}
/// $04CB86: rts
pub fn i_04cb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpawnStream_Dispatch_04cb88  ($04CB88, 40 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CB88: jsr $4cac4(pc)
pub fn i_04cb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CB8C);
    cpu.pc = 0x4CAC4;
    16
}
/// $04CB8C: btst.b #$0, $100001.l
pub fn i_04cb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB94;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $04CB94: beq.w $4cbb6
pub fn i_04cb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CBB6; return 10; }
    8
}
/// $04CB98: swap d0
pub fn i_04cb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB9A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CB9A: move.w d1, d0
pub fn i_04cb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB9C;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CB9C: move.w #$ffff, d1
pub fn i_04cb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CBA0: move.w #$0, d2
pub fn i_04cba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CBA4: moveq #$ff, d3
pub fn i_04cba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA6;
    cpu.ir = 0x76FF;
    cpu.d[3] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBA6: moveq #$ff, d4
pub fn i_04cba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA8;
    cpu.ir = 0x78FF;
    cpu.d[4] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBA8: moveq #$0, d5
pub fn i_04cba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBAA;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBAA: lea.l $243608.l, a0
pub fn i_04cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x243608;
    20
}
// ===== JsrAbsThunk_04cbb0  ($04CBB0, 8 B, thunks/jsr_abs_thunks.c)
/// $04CBB0: jsr $5a9d6.l
pub fn i_04cbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CBB6);
    cpu.pc = 0x5A9D6;
    16
}
/// $04CBB6: rts
pub fn i_04cbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SlotPrioCheck_04cbb8  ($04CBB8, 16 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CBB8: movea.l $8(a6), a1
pub fn i_04cbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBBC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04CBBC: move.b $10(a6), d0
pub fn i_04cbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBC0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CBC0: cmp.b $10(a1), d0
pub fn i_04cbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBC4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04CBC4: bcs.w $4cbce
pub fn i_04cbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBC8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CBCE; return 10; }
    8
}
// ===== ClearXN_04cbc8  ($04CBC8, 6 B, thunks/ccr_helpers.c)
/// $04CBC8: andi.b #$ee, ccr
pub fn i_04cbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBCC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04CBCC: rts
pub fn i_04cbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04cbce  ($04CBCE, 6 B, thunks/ccr_helpers.c)
/// $04CBCE: ori.b #$11, ccr
pub fn i_04cbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBD2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04CBD2: rts
pub fn i_04cbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Spawn_04cbd4  ($04CBD4, 114 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CBD4: move.b #$44, $20(a6)
pub fn i_04cbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x44;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CBDA: move.b #$ff, $80(a6)
pub fn i_04cbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBE0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CBE0: move.b #$1, $98(a6)
pub fn i_04cbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CBE6: bra.w $4cbfc
pub fn i_04cbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4CBFC;
    10
}
/// $04CBEA: jsr $2783a.l
pub fn i_04cbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CBF0);
    cpu.pc = 0x2783A;
    16
}
/// $04CBF0: cmpi.w #$140, $22(a6)
pub fn i_04cbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBF6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $04CBF6: bgt.w $4cbfc
pub fn i_04cbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBFA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4CBFC; return 10; }
    8
}
/// $04CBFA: rts
pub fn i_04cbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04CBFC: move.w #$e, d1
pub fn i_04cbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC00;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC00: jsr $236e.l
pub fn i_04cc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC06);
    cpu.pc = 0x236E;
    16
}
/// $04CC06: move.b #$ff, $32(a6)
pub fn i_04cc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC0C: move.b #$ff, $33(a6)
pub fn i_04cc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC12: move.w #$8000, $38(a6)
pub fn i_04cc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC18;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CC18: move.b #$ff, $7d(a6)
pub fn i_04cc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC1E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC1E: move.b #$0, $7c(a6)
pub fn i_04cc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC24;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC24: move.b #$0, $21(a6)
pub fn i_04cc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC2A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC2A: move.w #$0, $34(a6)
pub fn i_04cc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CC30: jsr $267e2.l
pub fn i_04cc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC36);
    cpu.pc = 0x267E2;
    16
}
/// $04CC36: lea.l $4cf2e(pc), a1
pub fn i_04cc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CF2E;
    16
}
/// $04CC3A: jsr $4ae.l
pub fn i_04cc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC40);
    cpu.pc = 0x4AE;
    16
}
/// $04CC40: jsr $5dd02.l
pub fn i_04cc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC46);
    cpu.pc = 0x5DD02;
    16
}
// ===== TurretCar_Idle_04cc46  ($04CC46, 314 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CC46: move.w #$1f, d0
pub fn i_04cc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC4A: move.w $34(a6), d1
pub fn i_04cc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC4E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CC4E: btst.b #$3, d1
pub fn i_04cc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC52;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04CC52: bne.w $4cc5a
pub fn i_04cc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CC5A; return 10; }
    8
}
/// $04CC56: move.w #$11, d0
pub fn i_04cc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC5A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC5A: move.w d0, $78(a6)
pub fn i_04cc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC5E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CC5E: move.b #$0, $7b(a6)
pub fn i_04cc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC64;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC64: move.b #$0, $75(a6)
pub fn i_04cc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC6A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC6A: move.w #$0, $70(a6)
pub fn i_04cc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CC70: lea.l $290f6e.l, a0
pub fn i_04cc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290F6E;
    20
}
/// $04CC76: jsr $28cd4.l
pub fn i_04cc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC7C);
    cpu.pc = 0x28CD4;
    16
}
/// $04CC7C: lea.l $4cc82(pc), a1
pub fn i_04cc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CC82;
    16
}
/// $04CC80: move.l a1, (a6)
pub fn i_04cc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CC82: jsr $4d3ce(pc)
pub fn i_04cc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CC86);
    cpu.pc = 0x4D3CE;
    16
}
/// $04CC86: addq.w #$1, $70(a6)
pub fn i_04cc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC8A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CC8A: move.w #$7, d0
pub fn i_04cc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC8E: cmpi.b #$0, $98(a6)
pub fn i_04cc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC94;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04CC94: beq.w $4cc9c
pub fn i_04cc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CC9C; return 10; }
    8
}
/// $04CC98: move.w #$1, d0
pub fn i_04cc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC9C: and.w d0, $70(a6)
pub fn i_04cc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCA0;
    cpu.ir = 0xC16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CCA0: bne.w $4ccba
pub fn i_04cca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CCBA; return 10; }
    8
}
/// $04CCA4: move.w $78(a6), d0
pub fn i_04cca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCA8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CCA8: jsr $4d4b0(pc)
pub fn i_04cca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCAC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CCAC);
    cpu.pc = 0x4D4B0;
    16
}
/// $04CCAC: move.w $34(a6), d1
pub fn i_04ccac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCB0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CCB0: add.w d0, $34(a6)
pub fn i_04ccb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCB4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CCB4: andi.w #$1f, $34(a6)
pub fn i_04ccb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCBA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04CCBA: jsr $28d70.l
pub fn i_04ccba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CCC0);
    cpu.pc = 0x28D70;
    16
}
/// $04CCC0: cmpi.b #$ff, $21(a6)
pub fn i_04ccc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCC6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CCC6: bne.w $4cd04
pub fn i_04ccc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CD04; return 10; }
    8
}
/// $04CCCA: cmpi.b #$88, $20(a6)
pub fn i_04ccca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCD0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04CCD0: bne.w $4ccdc
pub fn i_04ccd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CCDC; return 10; }
    8
}
/// $04CCD4: movea.l $c(a6), a0
pub fn i_04ccd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCD8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CCD8: addq.b #$1, $21(a0)
pub fn i_04ccd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCDC;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CCDC: move.b #$ff, $7c(a6)
pub fn i_04ccdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCE2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CCE2: lea.l $58f82.l, a1
pub fn i_04cce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $04CCE8: move.l a1, (a6)
pub fn i_04cce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CCEA: lea.l $776e2.l, a1
pub fn i_04ccea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCF0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $04CCF0: jsr $4ae.l
pub fn i_04ccf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CCF6);
    cpu.pc = 0x4AE;
    16
}
/// $04CCF6: jsr $5dd02.l
pub fn i_04ccf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CCFC);
    cpu.pc = 0x5DD02;
    16
}
/// $04CCFC: jsr $4d646(pc)
pub fn i_04ccfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD00);
    cpu.pc = 0x4D646;
    16
}
/// $04CD00: jsr $4d474(pc)
pub fn i_04cd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD04);
    cpu.pc = 0x4D474;
    16
}
/// $04CD04: move.w $78(a6), d0
pub fn i_04cd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD08;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CD08: cmp.w $34(a6), d0
pub fn i_04cd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD0C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04CD0C: bne.w $4cd5e
pub fn i_04cd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CD5E; return 10; }
    8
}
/// $04CD10: cmpi.b #$ff, $75(a6)
pub fn i_04cd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD16;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CD16: bne.w $4cd24
pub fn i_04cd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CD24; return 10; }
    8
}
/// $04CD1A: lea.l $4cd80(pc), a1
pub fn i_04cd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CD80;
    16
}
/// $04CD1E: move.l a1, (a6)
pub fn i_04cd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CD20: bra.w $4cd5e
pub fn i_04cd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD24;
    cpu.ir = 0x6000;
    cpu.pc = 0x4CD5E;
    10
}
/// $04CD24: cmpi.b #$80, $75(a6)
pub fn i_04cd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD2A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $04CD2A: beq.w $4cd5e
pub fn i_04cd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CD5E; return 10; }
    8
}
/// $04CD2E: lea.l $290f0e.l, a0
pub fn i_04cd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290F0E;
    20
}
/// $04CD34: jsr $28cd4.l
pub fn i_04cd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CD3A);
    cpu.pc = 0x28CD4;
    16
}
/// $04CD3A: move.w $34(a6), d0
pub fn i_04cd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD3E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CD3E: asr.b #$3, d0
pub fn i_04cd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD40;
    cpu.ir = 0xE600;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $04CD40: andi.b #$1, d0
pub fn i_04cd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD44;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04CD44: or.b d0, $3a(a6)
pub fn i_04cd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD48;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CD48: lea.l $4d048(pc), a1
pub fn i_04cd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D048;
    16
}
/// $04CD4C: jsr $6fe.l
pub fn i_04cd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CD52);
    cpu.pc = 0x6FE;
    16
}
/// $04CD52: jsr $5dd02.l
pub fn i_04cd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CD58);
    cpu.pc = 0x5DD02;
    16
}
/// $04CD58: move.b #$80, $75(a6)
pub fn i_04cd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD5E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CD5E: jsr $4d454(pc)
pub fn i_04cd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD62);
    cpu.pc = 0x4D454;
    16
}
/// $04CD62: bcs.w $4cd6e
pub fn i_04cd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD66;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CD6E; return 10; }
    8
}
/// $04CD66: jsr $4d5ac(pc)
pub fn i_04cd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD6A);
    cpu.pc = 0x4D5AC;
    16
}
/// $04CD6A: bcc.w $4cd7e
pub fn i_04cd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CD7E; return 10; }
    8
}
/// $04CD6E: move.b #$ff, $7c(a6)
pub fn i_04cd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CD74: jsr $4d474(pc)
pub fn i_04cd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD78);
    cpu.pc = 0x4D474;
    16
}
/// $04CD78: jmp $518.l
pub fn i_04cd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD7E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04CD7E: rts
pub fn i_04cd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Track_04cd80  ($04CD80, 226 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CD80: lea.l $2b791e.l, a0
pub fn i_04cd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B791E;
    20
}
/// $04CD86: jsr $799de.l
pub fn i_04cd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CD8C);
    cpu.pc = 0x799DE;
    16
}
/// $04CD8C: move.w d0, $72(a6)
pub fn i_04cd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD90;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CD90: lea.l $4cd96(pc), a1
pub fn i_04cd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CD96;
    16
}
/// $04CD94: move.l a1, (a6)
pub fn i_04cd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CD96: jsr $4d3ce(pc)
pub fn i_04cd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD9A);
    cpu.pc = 0x4D3CE;
    16
}
/// $04CD9A: addq.w #$1, $70(a6)
pub fn i_04cd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD9E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CD9E: move.w #$7, d0
pub fn i_04cd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDA2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CDA2: cmpi.b #$0, $98(a6)
pub fn i_04cda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDA8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04CDA8: beq.w $4cdb0
pub fn i_04cda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CDB0; return 10; }
    8
}
/// $04CDAC: move.w #$1, d0
pub fn i_04cdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDB0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CDB0: and.w d0, $70(a6)
pub fn i_04cdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDB4;
    cpu.ir = 0xC16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CDB4: bne.w $4cdd2
pub fn i_04cdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CDD2; return 10; }
    8
}
/// $04CDB8: jsr $4d58a(pc)
pub fn i_04cdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CDBC);
    cpu.pc = 0x4D58A;
    16
}
/// $04CDBC: jsr $4d612(pc)
pub fn i_04cdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CDC0);
    cpu.pc = 0x4D612;
    16
}
/// $04CDC0: jsr $4d4b0(pc)
pub fn i_04cdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CDC4);
    cpu.pc = 0x4D4B0;
    16
}
/// $04CDC4: move.w $34(a6), d1
pub fn i_04cdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDC8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CDC8: add.w d0, $34(a6)
pub fn i_04cdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDCC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CDCC: andi.w #$1f, $34(a6)
pub fn i_04cdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDD2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04CDD2: jsr $28d70.l
pub fn i_04cdd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CDD8);
    cpu.pc = 0x28D70;
    16
}
/// $04CDD8: cmpi.b #$ff, $21(a6)
pub fn i_04cdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDDE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CDDE: bne.w $4ce1c
pub fn i_04cdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDE2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CE1C; return 10; }
    8
}
/// $04CDE2: cmpi.b #$88, $20(a6)
pub fn i_04cde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDE8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04CDE8: bne.w $4cdf4
pub fn i_04cde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CDF4; return 10; }
    8
}
/// $04CDEC: movea.l $c(a6), a0
pub fn i_04cdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDF0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CDF0: addq.b #$1, $21(a0)
pub fn i_04cdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDF4;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CDF4: move.b #$ff, $7c(a6)
pub fn i_04cdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDFA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CDFA: lea.l $58f82.l, a1
pub fn i_04cdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $04CE00: move.l a1, (a6)
pub fn i_04ce00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CE02: lea.l $776e2.l, a1
pub fn i_04ce02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE08;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $04CE08: jsr $4ae.l
pub fn i_04ce08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CE0E);
    cpu.pc = 0x4AE;
    16
}
/// $04CE0E: jsr $5dd02.l
pub fn i_04ce0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CE14);
    cpu.pc = 0x5DD02;
    16
}
/// $04CE14: jsr $4d646(pc)
pub fn i_04ce14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE18);
    cpu.pc = 0x4D646;
    16
}
/// $04CE18: jsr $4d474(pc)
pub fn i_04ce18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE1C);
    cpu.pc = 0x4D474;
    16
}
/// $04CE1C: cmpi.b #$ff, $7d(a6)
pub fn i_04ce1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE22;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CE22: bne.w $4ce40
pub fn i_04ce22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CE40; return 10; }
    8
}
/// $04CE26: subq.w #$1, $72(a6)
pub fn i_04ce26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE2A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CE2A: bne.w $4ce40
pub fn i_04ce2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CE40; return 10; }
    8
}
/// $04CE2E: move.b #$0, $7d(a6)
pub fn i_04ce2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CE34: move.b #$ff, $7b(a6)
pub fn i_04ce34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CE3A: lea.l $4ce62(pc), a1
pub fn i_04ce3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CE62;
    16
}
/// $04CE3E: move.l a1, (a6)
pub fn i_04ce3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CE40: jsr $4d454(pc)
pub fn i_04ce40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE44);
    cpu.pc = 0x4D454;
    16
}
/// $04CE44: bcs.w $4ce50
pub fn i_04ce44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE48;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CE50; return 10; }
    8
}
/// $04CE48: jsr $4d5ac(pc)
pub fn i_04ce48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE4C);
    cpu.pc = 0x4D5AC;
    16
}
/// $04CE4C: bcc.w $4ce60
pub fn i_04ce4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CE60; return 10; }
    8
}
/// $04CE50: move.b #$ff, $7c(a6)
pub fn i_04ce50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE56;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CE56: jsr $4d474(pc)
pub fn i_04ce56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE5A);
    cpu.pc = 0x4D474;
    16
}
/// $04CE5A: jmp $518.l
pub fn i_04ce5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE60;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04CE60: rts
pub fn i_04ce60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Recoil_04ce62  ($04CE62, 204 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CE62: move.b #$0, $7b(a6)
pub fn i_04ce62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CE68: move.w $34(a6), d0
pub fn i_04ce68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE6C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CE6C: asr.b #$3, d0
pub fn i_04ce6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE6E;
    cpu.ir = 0xE600;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $04CE6E: andi.b #$1, d0
pub fn i_04ce6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE72;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04CE72: or.b d0, $3a(a6)
pub fn i_04ce72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE76;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CE76: move.w #$32, $7e(a6)
pub fn i_04ce76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CE7C: lea.l $29bfc4.l, a0
pub fn i_04ce7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFC4;
    20
}
/// $04CE82: jsr $28cd4.l
pub fn i_04ce82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CE88);
    cpu.pc = 0x28CD4;
    16
}
/// $04CE88: lea.l $4ce8e(pc), a1
pub fn i_04ce88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CE8E;
    16
}
/// $04CE8C: move.l a1, (a6)
pub fn i_04ce8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CE8E: jsr $4d3ce(pc)
pub fn i_04ce8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE92);
    cpu.pc = 0x4D3CE;
    16
}
/// $04CE92: move.w $22(a6), d0
pub fn i_04ce92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE96;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CE96: move.w $24(a6), d1
pub fn i_04ce96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE9A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CE9A: movem.w d0-d1, -(a7)
pub fn i_04ce9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE9E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $04CE9E: jsr $4d646(pc)
pub fn i_04ce9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CEA2);
    cpu.pc = 0x4D646;
    16
}
/// $04CEA2: jsr $28d70.l
pub fn i_04cea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CEA8);
    cpu.pc = 0x28D70;
    16
}
/// $04CEA8: subq.w #$1, $7e(a6)
pub fn i_04cea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEAC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CEAC: bne.w $4cebc
pub fn i_04ceac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEB0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CEBC; return 10; }
    8
}
/// $04CEB0: move.b #$0, $3a(a6)
pub fn i_04ceb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CEB6: lea.l $4cc46(pc), a1
pub fn i_04ceb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CC46;
    16
}
/// $04CEBA: move.l a1, (a6)
pub fn i_04ceba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CEBC: movem.w (a7)+, d0-d1
pub fn i_04cebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEC0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04CEC0: move.w d0, $22(a6)
pub fn i_04cec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEC4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CEC4: move.w d1, $24(a6)
pub fn i_04cec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEC8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CEC8: cmpi.b #$ff, $21(a6)
pub fn i_04cec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CECE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CECE: bne.w $4cf0c
pub fn i_04cece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CED2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CF0C; return 10; }
    8
}
/// $04CED2: cmpi.b #$88, $20(a6)
pub fn i_04ced2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CED8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04CED8: bne.w $4cee4
pub fn i_04ced8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CEE4; return 10; }
    8
}
/// $04CEDC: movea.l $c(a6), a0
pub fn i_04cedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEE0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CEE0: addq.b #$1, $21(a0)
pub fn i_04cee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEE4;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CEE4: move.b #$ff, $7c(a6)
pub fn i_04cee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CEEA: lea.l $58f82.l, a1
pub fn i_04ceea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEF0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $04CEF0: move.l a1, (a6)
pub fn i_04cef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CEF2: lea.l $776e2.l, a1
pub fn i_04cef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEF8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $04CEF8: jsr $4ae.l
pub fn i_04cef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CEFE);
    cpu.pc = 0x4AE;
    16
}
/// $04CEFE: jsr $5dd02.l
pub fn i_04cefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF04);
    cpu.pc = 0x5DD02;
    16
}
/// $04CF04: jsr $4d646(pc)
pub fn i_04cf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF08);
    cpu.pc = 0x4D646;
    16
}
/// $04CF08: jsr $4d474(pc)
pub fn i_04cf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF0C);
    cpu.pc = 0x4D474;
    16
}
/// $04CF0C: jsr $4d454(pc)
pub fn i_04cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF10);
    cpu.pc = 0x4D454;
    16
}
/// $04CF10: bcs.w $4cf1c
pub fn i_04cf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF14;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CF1C; return 10; }
    8
}
/// $04CF14: jsr $4d5ac(pc)
pub fn i_04cf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF18);
    cpu.pc = 0x4D5AC;
    16
}
/// $04CF18: bcc.w $4cf2c
pub fn i_04cf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CF2C; return 10; }
    8
}
/// $04CF1C: move.b #$ff, $7c(a6)
pub fn i_04cf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF22;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CF22: jsr $4d474(pc)
pub fn i_04cf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF26);
    cpu.pc = 0x4D474;
    16
}
/// $04CF26: jmp $518.l
pub fn i_04cf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF2C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04CF2C: rts
pub fn i_04cf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Body_04cf2e  ($04CF2E, 282 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CF2E: move.w #$22, d1
pub fn i_04cf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF32;
    cpu.ir = 0x323C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CF32: jsr $236e.l
pub fn i_04cf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF38);
    cpu.pc = 0x236E;
    16
}
/// $04CF38: move.b #$ff, $32(a6)
pub fn i_04cf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CF3E: move.b #$ff, $33(a6)
pub fn i_04cf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF44;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CF44: move.w #$c, $66(a6)
pub fn i_04cf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF4A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CF4A: lea.l $2b7798.l, a0
pub fn i_04cf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF50;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7798;
    20
}
/// $04CF50: jsr $799de.l
pub fn i_04cf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF56);
    cpu.pc = 0x799DE;
    16
}
/// $04CF56: move.w d0, $66(a6)
pub fn i_04cf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF5A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CF5A: jsr $267e2.l
pub fn i_04cf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF60);
    cpu.pc = 0x267E2;
    16
}
/// $04CF60: jsr $4d5e4(pc)
pub fn i_04cf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF64);
    cpu.pc = 0x4D5E4;
    16
}
/// $04CF64: lea.l $2912d6.l, a0
pub fn i_04cf64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2912D6;
    20
}
/// $04CF6A: jsr $28cd4.l
pub fn i_04cf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF70);
    cpu.pc = 0x28CD4;
    16
}
/// $04CF70: lea.l $4cf76(pc), a1
pub fn i_04cf70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CF76;
    16
}
/// $04CF74: move.l a1, (a6)
pub fn i_04cf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CF76: movea.l $c(a6), a0
pub fn i_04cf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF7A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CF7A: move.w $22(a0), $22(a6)
pub fn i_04cf7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF80;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04CF80: move.w $24(a0), $24(a6)
pub fn i_04cf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF86;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04CF86: move.w $38(a0), $38(a6)
pub fn i_04cf86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF8C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04CF8C: jsr $4d5e4(pc)
pub fn i_04cf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF90);
    cpu.pc = 0x4D5E4;
    16
}
/// $04CF90: movea.l $c(a6), a0
pub fn i_04cf90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF94;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CF94: cmpi.b #$ff, $7b(a0)
pub fn i_04cf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF9A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CF9A: bne.w $4cfaa
pub fn i_04cf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CFAA; return 10; }
    8
}
/// $04CF9E: lea.l $291526.l, a0
pub fn i_04cf9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x291526;
    20
}
/// $04CFA4: jsr $28cd4.l
pub fn i_04cfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFAA);
    cpu.pc = 0x28CD4;
    16
}
/// $04CFAA: jsr $28d70.l
pub fn i_04cfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFB0);
    cpu.pc = 0x28D70;
    16
}
/// $04CFB0: movea.l $c(a6), a0
pub fn i_04cfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFB4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CFB4: cmpi.b #$44, $20(a0)
pub fn i_04cfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFBA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04CFBA: bne.w $4cfc6
pub fn i_04cfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CFC6; return 10; }
    8
}
/// $04CFBE: jsr $4d48a(pc)
pub fn i_04cfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CFC2);
    cpu.pc = 0x4D48A;
    16
}
/// $04CFC2: bcs.w $4cfde
pub fn i_04cfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFC6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CFDE; return 10; }
    8
}
/// $04CFC6: movea.l $c(a6), a0
pub fn i_04cfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFCA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CFCA: cmpi.b #$ff, $7c(a0)
pub fn i_04cfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFD0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CFD0: beq.w $4d040
pub fn i_04cfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D040; return 10; }
    8
}
/// $04CFD4: jsr $2870a.l
pub fn i_04cfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFDA);
    cpu.pc = 0x2870A;
    16
}
/// $04CFDA: bcc.w $4cff0
pub fn i_04cfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFDE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CFF0; return 10; }
    8
}
/// $04CFDE: lea.l $5e766.l, a0
pub fn i_04cfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04CFE4: jsr $5e770.l
pub fn i_04cfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFEA);
    cpu.pc = 0x5E770;
    16
}
/// $04CFEA: bclr.b #$3, $13(a6)
pub fn i_04cfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFF0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04CFF0: jsr $28758.l
pub fn i_04cff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFF6);
    cpu.pc = 0x28758;
    16
}
/// $04CFF6: bcc.w $4d046
pub fn i_04cff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D046; return 10; }
    8
}
/// $04CFFA: move.w #$1023, d0
pub fn i_04cffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFFE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CFFE: jsr $2352.l
pub fn i_04cffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D004;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D004);
    cpu.pc = 0x2352;
    16
}
/// $04D004: movea.l $c(a6), a0
pub fn i_04d004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D008;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D008: move.b #$ff, $21(a0)
pub fn i_04d008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D00E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D00E: lea.l $293eb4.l, a1
pub fn i_04d00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D014;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x293EB4;
    20
}
/// $04D014: jsr $77c7e.l
pub fn i_04d014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D01A);
    cpu.pc = 0x77C7E;
    16
}
/// $04D01A: lea.l $293ec6.l, a1
pub fn i_04d01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D020;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x293EC6;
    20
}
/// $04D020: jsr $77c7e.l
pub fn i_04d020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D026);
    cpu.pc = 0x77C7E;
    16
}
/// $04D026: movea.l $c(a6), a0
pub fn i_04d026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D02A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D02A: cmpi.b #$88, $20(a0)
pub fn i_04d02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D030;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04D030: bne.w $4d040
pub fn i_04d030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D034;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D040; return 10; }
    8
}
/// $04D034: lea.l $295682.l, a2
pub fn i_04d034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D03A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295682;
    20
}
/// $04D03A: jsr $5022a.l
pub fn i_04d03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D040);
    cpu.pc = 0x5022A;
    16
}
/// $04D040: jmp $518.l
pub fn i_04d040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D046;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D046: rts
pub fn i_04d046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D048;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Driver_04d048  ($04D048, 154 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D048: move.w #$23, d1
pub fn i_04d048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D04C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D04C: jsr $236e.l
pub fn i_04d04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D052);
    cpu.pc = 0x236E;
    16
}
/// $04D052: movea.l $c(a6), a0
pub fn i_04d052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D056;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D056: move.w $34(a0), d0
pub fn i_04d056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D05A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D05A: asr.b #$3, d0
pub fn i_04d05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D05C;
    cpu.ir = 0xE600;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $04D05C: andi.b #$1, d0
pub fn i_04d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D060;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04D060: or.b d0, $3a(a6)
pub fn i_04d060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D064;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04D064: move.b #$ff, $32(a6)
pub fn i_04d064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D06A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D06A: move.b #$ff, $33(a6)
pub fn i_04d06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D070;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D070: lea.l $2b789c.l, a0
pub fn i_04d070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D076;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B789C;
    20
}
/// $04D076: jsr $799de.l
pub fn i_04d076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D07C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D07C);
    cpu.pc = 0x799DE;
    16
}
/// $04D07C: move.w d0, $66(a6)
pub fn i_04d07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D080;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D080: lea.l $293ed8.l, a0
pub fn i_04d080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D086;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x293ED8;
    20
}
/// $04D086: move.l a0, $4c(a6)
pub fn i_04d086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D08A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04D08A: jsr $283ca.l
pub fn i_04d08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D090;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D090);
    cpu.pc = 0x283CA;
    16
}
/// $04D090: jsr $267e2.l
pub fn i_04d090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D096);
    cpu.pc = 0x267E2;
    16
}
/// $04D096: lea.l $29276a.l, a0
pub fn i_04d096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D09C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29276A;
    20
}
/// $04D09C: jsr $28cd4.l
pub fn i_04d09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0A2);
    cpu.pc = 0x28CD4;
    16
}
/// $04D0A2: lea.l $4d0a8(pc), a1
pub fn i_04d0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D0A8;
    16
}
/// $04D0A6: move.l a1, (a6)
pub fn i_04d0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D0A8: jsr $5e4dc.l
pub fn i_04d0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0AE);
    cpu.pc = 0x5E4DC;
    16
}
/// $04D0AE: jsr $4d5e4(pc)
pub fn i_04d0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D0B2);
    cpu.pc = 0x4D5E4;
    16
}
/// $04D0B2: jsr $28d70.l
pub fn i_04d0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0B8);
    cpu.pc = 0x28D70;
    16
}
/// $04D0B8: bcc.w $4d0cc
pub fn i_04d0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D0CC; return 10; }
    8
}
/// $04D0BC: movea.l $c(a6), a0
pub fn i_04d0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0C0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D0C0: move.b #$ff, $75(a0)
pub fn i_04d0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0C6;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D0C6: lea.l $4d0e2(pc), a1
pub fn i_04d0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D0E2;
    16
}
/// $04D0CA: move.l a1, (a6)
pub fn i_04d0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D0CC: movea.l $c(a6), a0
pub fn i_04d0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0D0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D0D0: cmpi.b #$ff, $7c(a0)
pub fn i_04d0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0D6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D0D6: bne.w $4d0e0
pub fn i_04d0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D0E0; return 10; }
    8
}
/// $04D0DA: jmp $518.l
pub fn i_04d0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0E0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D0E0: rts
pub fn i_04d0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_DriverPanic_04d0e2  ($04D0E2, 88 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D0E2: move.b #$0, $3a(a6)
pub fn i_04d0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D0E8: move.w #$12c, $70(a6)
pub fn i_04d0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D0EE: lea.l $292a48.l, a0
pub fn i_04d0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x292A48;
    20
}
/// $04D0F4: jsr $28cd4.l
pub fn i_04d0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0FA);
    cpu.pc = 0x28CD4;
    16
}
/// $04D0FA: lea.l $4d100(pc), a1
pub fn i_04d0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D100;
    16
}
/// $04D0FE: move.l a1, (a6)
pub fn i_04d0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D100;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D100: jsr $5e4dc.l
pub fn i_04d100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D106);
    cpu.pc = 0x5E4DC;
    16
}
/// $04D106: jsr $4d5e4(pc)
pub fn i_04d106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D10A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D10A);
    cpu.pc = 0x4D5E4;
    16
}
/// $04D10A: jsr $28d70.l
pub fn i_04d10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D110;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D110);
    cpu.pc = 0x28D70;
    16
}
/// $04D110: movea.l $c(a6), a0
pub fn i_04d110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D114;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D114: cmpi.b #$ff, $7b(a0)
pub fn i_04d114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D11A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D11A: bne.w $4d124
pub fn i_04d11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D11E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D124; return 10; }
    8
}
/// $04D11E: lea.l $4d13a(pc), a1
pub fn i_04d11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D122;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D13A;
    16
}
/// $04D122: move.l a1, (a6)
pub fn i_04d122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D124;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D124: movea.l $c(a6), a0
pub fn i_04d124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D128;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D128: cmpi.b #$ff, $7c(a0)
pub fn i_04d128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D12E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D12E: bne.w $4d138
pub fn i_04d12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D132;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D138; return 10; }
    8
}
/// $04D132: jmp $518.l
pub fn i_04d132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D138;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D138: rts
pub fn i_04d138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D13A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_DriverFlee_04d13a  ($04D13A, 76 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D13A: move.w #$10a3, d0
pub fn i_04d13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D13E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D13E: jsr $2352.l
pub fn i_04d13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D144;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D144);
    cpu.pc = 0x2352;
    16
}
/// $04D144: lea.l $292c92.l, a0
pub fn i_04d144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D14A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x292C92;
    20
}
/// $04D14A: jsr $28cd4.l
pub fn i_04d14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D150);
    cpu.pc = 0x28CD4;
    16
}
/// $04D150: lea.l $4d156(pc), a1
pub fn i_04d150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D154;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D156;
    16
}
/// $04D154: move.l a1, (a6)
pub fn i_04d154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D156;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D156: jsr $5e4dc.l
pub fn i_04d156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D15C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D15C);
    cpu.pc = 0x5E4DC;
    16
}
/// $04D15C: jsr $4d5e4(pc)
pub fn i_04d15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D160;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D160);
    cpu.pc = 0x4D5E4;
    16
}
/// $04D160: jsr $28d70.l
pub fn i_04d160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D166;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D166);
    cpu.pc = 0x28D70;
    16
}
/// $04D166: bcc.w $4d170
pub fn i_04d166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D16A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D170; return 10; }
    8
}
/// $04D16A: lea.l $4d186(pc), a1
pub fn i_04d16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D16E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D186;
    16
}
/// $04D16E: move.l a1, (a6)
pub fn i_04d16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D170;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D170: movea.l $c(a6), a0
pub fn i_04d170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D174;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D174: cmpi.b #$ff, $7c(a0)
pub fn i_04d174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D17A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D17A: bne.w $4d184
pub fn i_04d17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D17E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D184; return 10; }
    8
}
/// $04D17E: jmp $518.l
pub fn i_04d17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D184;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D184: rts
pub fn i_04d184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D186;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Cannon_04d186  ($04D186, 442 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D186: jsr $4d624(pc)
pub fn i_04d186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D18A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D18A);
    cpu.pc = 0x4D624;
    16
}
/// $04D18A: addq.w #$2, $38(a6)
pub fn i_04d18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D18E;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D18E: move.b #$0, $7a(a6)
pub fn i_04d18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D194;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D194: move.w #$ffff, $70(a6)
pub fn i_04d194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D19A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D19A: lea.l $2b781a.l, a0
pub fn i_04d19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B781A;
    20
}
/// $04D1A0: jsr $799de.l
pub fn i_04d1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D1A6);
    cpu.pc = 0x799DE;
    16
}
/// $04D1A6: move.w d0, $36(a6)
pub fn i_04d1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D1AA: jsr $4d5e4(pc)
pub fn i_04d1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D1AE);
    cpu.pc = 0x4D5E4;
    16
}
/// $04D1AE: lea.l $2927f8.l, a0
pub fn i_04d1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2927F8;
    20
}
/// $04D1B4: jsr $28cd4.l
pub fn i_04d1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D1BA);
    cpu.pc = 0x28CD4;
    16
}
/// $04D1BA: move.w $34(a6), d0
pub fn i_04d1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1BE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D1BE: move.w #$97, d1
pub fn i_04d1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1C2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x97;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D1C2: move.b d0, (a6, d1.w)
pub fn i_04d1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1C6;
    cpu.ir = 0x1D80;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $04D1C6: subq.w #$1, d1
pub fn i_04d1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1C8;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D1C8: cmpi.w #$88, d1
pub fn i_04d1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1CC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x88, Size::Word, false);
    8
}
/// $04D1CC: bgt.b $4d1c2
pub fn i_04d1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1CE;
    cpu.ir = 0x6EF4;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D1C2; return 10; }
    8
}
/// $04D1CE: jsr $4d6ac(pc)
pub fn i_04d1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D1D2);
    cpu.pc = 0x4D6AC;
    16
}
/// $04D1D2: lea.l $4d1d8(pc), a1
pub fn i_04d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D1D8;
    16
}
/// $04D1D6: move.l a1, (a6)
pub fn i_04d1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D1D8: addq.w #$1, $70(a6)
pub fn i_04d1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1DC;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D1DC: move.w $78(a6), d0
pub fn i_04d1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1E0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D1E0: cmpi.w #$258, $70(a6)
pub fn i_04d1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x258, Size::Word, false);
    16
}
/// $04D1E6: bgt.w $4d228
pub fn i_04d1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D228; return 10; }
    8
}
/// $04D1EA: move.w $70(a6), d0
pub fn i_04d1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D1EE: andi.w #$f, d0
pub fn i_04d1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1F2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D1F2: bne.w $4d240
pub fn i_04d1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D240; return 10; }
    8
}
/// $04D1F6: jsr $4d4f0(pc)
pub fn i_04d1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D1FA);
    cpu.pc = 0x4D4F0;
    16
}
/// $04D1FA: cmpi.b #$ff, $7a(a6)
pub fn i_04d1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D200;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D200: beq.w $4d218
pub fn i_04d200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D204;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D218; return 10; }
    8
}
/// $04D204: jsr $4d57e(pc)
pub fn i_04d204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D208;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D208);
    cpu.pc = 0x4D57E;
    16
}
/// $04D208: cmpi.w #$1c0, $24(a6)
pub fn i_04d208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D20E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1C0, Size::Word, false);
    16
}
/// $04D20E: blt.w $4d218
pub fn i_04d20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D212;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4D218; return 10; }
    8
}
/// $04D212: move.b #$ff, $7a(a6)
pub fn i_04d212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D218;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D218: jsr $4d668(pc)
pub fn i_04d218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D21C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D21C);
    cpu.pc = 0x4D668;
    16
}
/// $04D21C: bcc.w $4d224
pub fn i_04d21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D220;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D224; return 10; }
    8
}
/// $04D220: move.w #$8, d0
pub fn i_04d220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D224;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D224: move.w d0, $78(a6)
pub fn i_04d224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D228;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D228: jsr $4d4b0(pc)
pub fn i_04d228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D22C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D22C);
    cpu.pc = 0x4D4B0;
    16
}
/// $04D22C: ori.w #$1, d0
pub fn i_04d22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D230;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D230: move.w $70(a6), d1
pub fn i_04d230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D234;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D234: andi.w #$1f, d1
pub fn i_04d234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D238;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D238: bne.w $4d240
pub fn i_04d238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D23C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D240; return 10; }
    8
}
/// $04D23C: move.w d0, $76(a6)
pub fn i_04d23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D240;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D240: move.w $70(a6), d0
pub fn i_04d240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D244;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D244: andi.w #$3, d0
pub fn i_04d244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D248;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D248: bne.w $4d25e
pub fn i_04d248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D24C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D25E; return 10; }
    8
}
/// $04D24C: move.w $34(a6), d1
pub fn i_04d24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D250;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D250: move.w $76(a6), d0
pub fn i_04d250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D254;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D254: add.w d0, $34(a6)
pub fn i_04d254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D258;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D258: andi.w #$1f, $34(a6)
pub fn i_04d258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D25E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D25E: jsr $4d51c(pc)
pub fn i_04d25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D262;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D262);
    cpu.pc = 0x4D51C;
    16
}
/// $04D262: jsr $4d3fe(pc)
pub fn i_04d262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D266;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D266);
    cpu.pc = 0x4D3FE;
    16
}
/// $04D266: bcs.w $4d2f4
pub fn i_04d266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D26A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D2F4; return 10; }
    8
}
/// $04D26A: jsr $28d70.l
pub fn i_04d26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D270);
    cpu.pc = 0x28D70;
    16
}
/// $04D270: move.w $70(a6), d0
pub fn i_04d270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D274;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D274: andi.w #$7, d0
pub fn i_04d274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D278;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D278: bne.w $4d2be
pub fn i_04d278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D27C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D2BE; return 10; }
    8
}
/// $04D27C: cmpi.w #$10, $34(a6)
pub fn i_04d27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D282;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $04D282: bgt.w $4d29a
pub fn i_04d282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D286;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D29A; return 10; }
    8
}
/// $04D286: lea.l $4d340(pc), a1
pub fn i_04d286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D28A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D340;
    16
}
/// $04D28A: jsr $4ae.l
pub fn i_04d28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D290;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D290);
    cpu.pc = 0x4AE;
    16
}
/// $04D290: jsr $5dd02.l
pub fn i_04d290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D296;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D296);
    cpu.pc = 0x5DD02;
    16
}
/// $04D296: bra.w $4d2ae
pub fn i_04d296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D29A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D2AE;
    10
}
/// $04D29A: lea.l $4d340(pc), a1
pub fn i_04d29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D29E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D340;
    16
}
/// $04D29E: jsr $6fe.l
pub fn i_04d29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2A4);
    cpu.pc = 0x6FE;
    16
}
/// $04D2A4: jsr $5dd02.l
pub fn i_04d2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2AA);
    cpu.pc = 0x5DD02;
    16
}
/// $04D2AA: subq.w #$1, $38(a0)
pub fn i_04d2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2AE;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D2AE: jsr $4d5f0(pc)
pub fn i_04d2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D2B2);
    cpu.pc = 0x4D5F0;
    16
}
/// $04D2B2: move.b $97(a6), d1
pub fn i_04d2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2B6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04D2B6: andi.w #$1f, d1
pub fn i_04d2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2BA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D2BA: move.w d1, $34(a0)
pub fn i_04d2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2BE;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D2BE: jsr $283d8.l
pub fn i_04d2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2C4);
    cpu.pc = 0x283D8;
    16
}
/// $04D2C4: btst.b #$1, $13(a6)
pub fn i_04d2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04D2CA: bne.w $4d2f4
pub fn i_04d2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D2F4; return 10; }
    8
}
/// $04D2CE: jsr $2870a.l
pub fn i_04d2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2D4);
    cpu.pc = 0x2870A;
    16
}
/// $04D2D4: bcc.w $4d2ea
pub fn i_04d2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D2EA; return 10; }
    8
}
/// $04D2D8: lea.l $5e766.l, a0
pub fn i_04d2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04D2DE: jsr $5e770.l
pub fn i_04d2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2E4);
    cpu.pc = 0x5E770;
    16
}
/// $04D2E4: bclr.b #$3, $13(a6)
pub fn i_04d2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2EA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D2EA: jsr $28758.l
pub fn i_04d2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2F0);
    cpu.pc = 0x28758;
    16
}
/// $04D2F0: bcc.w $4d326
pub fn i_04d2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D326; return 10; }
    8
}
/// $04D2F4: lea.l $77f6a.l, a1
pub fn i_04d2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $04D2FA: jsr $4ae.l
pub fn i_04d2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D300);
    cpu.pc = 0x4AE;
    16
}
/// $04D300: jsr $5dd02.l
pub fn i_04d300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D306;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D306);
    cpu.pc = 0x5DD02;
    16
}
/// $04D306: move.w #$4000, $38(a0)
pub fn i_04d306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D30C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D30C: lea.l $293ff2.l, a1
pub fn i_04d30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D312;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x293FF2;
    20
}
/// $04D312: jsr $77c7e.l
pub fn i_04d312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D318);
    cpu.pc = 0x77C7E;
    16
}
/// $04D318: move.w #$1022, d0
pub fn i_04d318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D31C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D31C: jsr $2352.l
pub fn i_04d31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D322);
    cpu.pc = 0x2352;
    16
}
/// $04D322: bra.w $4d32e
pub fn i_04d322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D326;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D32E;
    10
}
/// $04D326: jsr $4d5c8(pc)
pub fn i_04d326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D32A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D32A);
    cpu.pc = 0x4D5C8;
    16
}
/// $04D32A: bcc.w $4d33e
pub fn i_04d32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D32E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D33E; return 10; }
    8
}
/// $04D32E: movea.l $c(a6), a0
pub fn i_04d32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D332;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D332: move.b #$ff, $7d(a0)
pub fn i_04d332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D338;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D338: jmp $518.l
pub fn i_04d338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D33E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D33E: rts
pub fn i_04d33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D340;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Shell_04d340  ($04D340, 142 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D340: lea.l $2911bc.l, a0
pub fn i_04d340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D346;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2911BC;
    20
}
/// $04D346: jsr $28cd4.l
pub fn i_04d346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D34C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D34C);
    cpu.pc = 0x28CD4;
    16
}
/// $04D34C: lea.l $293fe8.l, a0
pub fn i_04d34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D352;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x293FE8;
    20
}
/// $04D352: move.l a0, $70(a6)
pub fn i_04d352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D356;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04D356: move.w #$d, d1
pub fn i_04d356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D35A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D35A: jsr $236e.l
pub fn i_04d35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D360);
    cpu.pc = 0x236E;
    16
}
/// $04D360: move.b #$ff, $32(a6)
pub fn i_04d360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D366;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D366: move.b #$ff, $33(a6)
pub fn i_04d366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D36C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D36C: move.w #$80, $36(a6)
pub fn i_04d36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D372;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D372: move.w #$2, d0
pub fn i_04d372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D376;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D376: jsr $5e9e4.l
pub fn i_04d376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D37C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D37C);
    cpu.pc = 0x5E9E4;
    16
}
/// $04D37C: btst.b #$0, d0
pub fn i_04d37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D380;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D380: bne.w $4d386
pub fn i_04d380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D384;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D386; return 10; }
    8
}
/// $04D384: neg.w d0
pub fn i_04d384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D386;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D386: add.w d0, $34(a6)
pub fn i_04d386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D38A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D38A: andi.w #$1f, $34(a6)
pub fn i_04d38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D390;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D390: jsr $4d502(pc)
pub fn i_04d390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D394;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D394);
    cpu.pc = 0x4D502;
    16
}
/// $04D394: movea.l $70(a6), a0
pub fn i_04d394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D398;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D398: jsr $5dd56.l
pub fn i_04d398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D39E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D39E);
    cpu.pc = 0x5DD56;
    16
}
/// $04D39E: bcs.w $4d3c6
pub fn i_04d39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3A2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D3C6; return 10; }
    8
}
/// $04D3A2: lea.l $4d3a8(pc), a1
pub fn i_04d3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D3A8;
    16
}
/// $04D3A6: move.l a1, (a6)
pub fn i_04d3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D3A8: jsr $27cee.l
pub fn i_04d3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D3AE);
    cpu.pc = 0x27CEE;
    16
}
/// $04D3AE: jsr $28d70.l
pub fn i_04d3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D3B4);
    cpu.pc = 0x28D70;
    16
}
/// $04D3B4: bcs.w $4d3c6
pub fn i_04d3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3B8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D3C6; return 10; }
    8
}
/// $04D3B8: movea.l $70(a6), a0
pub fn i_04d3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3BC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D3BC: jsr $5dd56.l
pub fn i_04d3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D3C2);
    cpu.pc = 0x5DD56;
    16
}
/// $04D3C2: bcc.w $4d3cc
pub fn i_04d3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D3CC; return 10; }
    8
}
/// $04D3C6: jmp $518.l
pub fn i_04d3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3CC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D3CC: rts
pub fn i_04d3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SyncPosIfBase_04d3ce  ($04D3CE, 10 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D3CE: cmpi.b #$44, $20(a6)
pub fn i_04d3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04D3D4: beq.w $4d3e0
pub fn i_04d3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D3E0; return 10; }
    8
}
// ===== JsrAbsThunk_04d3d8  ($04D3D8, 8 B, thunks/jsr_abs_thunks.c)
/// $04D3D8: jsr $2783a.l
pub fn i_04d3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D3DE);
    cpu.pc = 0x2783A;
    16
}
/// $04D3DE: rts
pub fn i_04d3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SyncPosToParent_04d3e0  ($04D3E0, 24 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D3E0: movea.l $c(a6), a0
pub fn i_04d3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3E4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D3E4: move.w $22(a0), d0
pub fn i_04d3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3E8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D3E8: subi.w #$23, d0
pub fn i_04d3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3EC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x23, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D3EC: move.w d0, $22(a6)
pub fn i_04d3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3F0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D3F0: move.w $24(a0), d0
pub fn i_04d3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3F4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D3F4: addi.w #$2d, d0
pub fn i_04d3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3F8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2D, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== SetTaskW_04d3f8  ($04D3F8, 6 B, kernel/task_setters_w.c)
/// $04D3F8: move.w d0, $24(a6)
pub fn i_04d3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D3FC: rts
pub fn i_04d3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D3FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ProbeGuided_04d3fe  ($04D3FE, 66 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D3FE: movea.l $c(a6), a0
pub fn i_04d3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D402;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D402: cmpi.b #$44, $20(a0)
pub fn i_04d402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D408;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04D408: bne.w $4d44c
pub fn i_04d408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D40C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D44C; return 10; }
    8
}
/// $04D40C: bset.b #$6, $13(a6)
pub fn i_04d40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D412;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D412: move.w $34(a6), d0
pub fn i_04d412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D416;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D416: movem.w d0, -(a7)
pub fn i_04d416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D41A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $04D41A: jsr $27bc8.l
pub fn i_04d41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D420);
    cpu.pc = 0x27BC8;
    16
}
/// $04D420: bcc.w $4d42c
pub fn i_04d420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D42C; return 10; }
    8
}
/// $04D424: move.b #$1, d1
pub fn i_04d424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D428;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04D428: bra.w $4d430
pub fn i_04d428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D42C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D430;
    10
}
/// $04D42C: move.b #$0, d1
pub fn i_04d42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D430;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04D430: movem.w (a7)+, d0
pub fn i_04d430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D434;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04D434: move.w d0, $34(a6)
pub fn i_04d434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D438;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D438: cmpi.b #$0, d1
pub fn i_04d438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D43C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $04D43C: beq.w $4d446
pub fn i_04d43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D440;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D446; return 10; }
    8
}
// ===== SetXN_04d440  ($04D440, 6 B, thunks/ccr_helpers.c)
/// $04D440: ori.b #$11, ccr
pub fn i_04d440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D444;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04D444: rts
pub fn i_04d444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D446;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04d446  ($04D446, 6 B, thunks/ccr_helpers.c)
/// $04D446: andi.b #$ee, ccr
pub fn i_04d446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D44A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04D44A: rts
pub fn i_04d44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D44C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_04d44c  ($04D44C, 8 B, thunks/jsr_abs_thunks.c)
/// $04D44C: jsr $27cee.l
pub fn i_04d44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D452);
    cpu.pc = 0x27CEE;
    16
}
/// $04D452: rts
pub fn i_04d452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_IsBaseIdleDone_04d454  ($04D454, 20 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D454: cmpi.b #$44, $20(a6)
pub fn i_04d454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D45A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04D45A: bne.w $4d46e
pub fn i_04d45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D45E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D46E; return 10; }
    8
}
/// $04D45E: cmpi.b #$0, $80(a6)
pub fn i_04d45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D464;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04D464: bne.w $4d46e
pub fn i_04d464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D468;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D46E; return 10; }
    8
}
// ===== SetXN_04d468  ($04D468, 6 B, thunks/ccr_helpers.c)
/// $04D468: ori.b #$11, ccr
pub fn i_04d468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D46C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04D46C: rts
pub fn i_04d46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D46E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04d46e  ($04D46E, 6 B, thunks/ccr_helpers.c)
/// $04D46E: andi.b #$ee, ccr
pub fn i_04d46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D472;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04D472: rts
pub fn i_04d472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D474;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ClearParentStateIfBase_04d474  ($04D474, 22 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D474: cmpi.b #$44, $20(a6)
pub fn i_04d474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D47A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04D47A: bne.w $4d488
pub fn i_04d47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D47E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D488; return 10; }
    8
}
/// $04D47E: movea.l $c(a6), a0
pub fn i_04d47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D482;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D482: move.b #$0, $20(a0)
pub fn i_04d482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D488;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D488: rts
pub fn i_04d488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D48A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ParentOffworld_04d48a  ($04D48A, 22 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D48A: movem.l a6, -(a7)
pub fn i_04d48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D48E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04D48E: movea.l $c(a6), a6
pub fn i_04d48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D492;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $04D492: jsr $5e45a.l
pub fn i_04d492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D498);
    cpu.pc = 0x5E45A;
    16
}
/// $04D498: bcs.w $4d4a6
pub fn i_04d498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D49C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D4A6; return 10; }
    8
}
/// $04D49C: movem.l (a7)+, a6
pub fn i_04d49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4A0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_04d4a0  ($04D4A0, 6 B, thunks/ccr_helpers.c)
/// $04D4A0: andi.b #$ee, ccr
pub fn i_04d4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4A4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04D4A4: rts
pub fn i_04d4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ParentOffworldRestore_04d4a6  ($04D4A6, 4 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D4A6: movem.l (a7)+, a6
pub fn i_04d4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4AA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_04d4aa  ($04D4AA, 6 B, thunks/ccr_helpers.c)
/// $04D4AA: ori.b #$11, ccr
pub fn i_04d4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4AE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04D4AE: rts
pub fn i_04d4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AngleStep16_04d4b0  ($04D4B0, 32 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D4B0: move.w $34(a6), d1
pub fn i_04d4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4B4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D4B4: sub.w d1, d0
pub fn i_04d4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4B6;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04D4B6: beq.w $4d4ce
pub fn i_04d4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D4CE; return 10; }
    8
}
/// $04D4BA: btst.b #$4, d0
pub fn i_04d4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4BE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D4BE: bne.w $4d4ca
pub fn i_04d4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D4CA; return 10; }
    8
}
/// $04D4C2: move.w #$1, d0
pub fn i_04d4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D4C6: bra.w $4d4ce
pub fn i_04d4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D4CE;
    10
}
/// $04D4CA: move.w #$ff, d0
pub fn i_04d4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D4CE: rts
pub fn i_04d4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AngleStep8_04d4d0  ($04D4D0, 32 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D4D0: move.w $34(a6), d1
pub fn i_04d4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D4D4: sub.w d1, d0
pub fn i_04d4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4D6;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04D4D6: beq.w $4d4ee
pub fn i_04d4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D4EE; return 10; }
    8
}
/// $04D4DA: btst.b #$3, d0
pub fn i_04d4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4DE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D4DE: bne.w $4d4ea
pub fn i_04d4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D4EA; return 10; }
    8
}
/// $04D4E2: move.w #$1, d0
pub fn i_04d4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D4E6: bra.w $4d4ee
pub fn i_04d4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D4EE;
    10
}
/// $04D4EA: move.w #$ff, d0
pub fn i_04d4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D4EE: rts
pub fn i_04d4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AngleToTarget_04d4f0  ($04D4F0, 18 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D4F0: jsr $5e136.l
pub fn i_04d4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D4F6);
    cpu.pc = 0x5E136;
    16
}
/// $04D4F6: addi.w #$8, d0
pub fn i_04d4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4FA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D4FA: asr.w #$3, d0
pub fn i_04d4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D4FC;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04D4FC: andi.w #$1f, d0
pub fn i_04d4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D500;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D500: rts
pub fn i_04d500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D502;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_VelFromAngle_04d502  ($04D502, 26 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D502: move.w $34(a6), d0
pub fn i_04d502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D506;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D506: asl.w #$3, d0
pub fn i_04d506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D508;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04D508: move.w $36(a6), d1
pub fn i_04d508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D50C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D50C: jsr $13c0e.l
pub fn i_04d50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D512);
    cpu.pc = 0x13C0E;
    16
}
/// $04D512: move.w d1, $28(a6)
pub fn i_04d512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D516;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D516: move.w d2, $2a(a6)
pub fn i_04d516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D51A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D51A: rts
pub fn i_04d51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D51C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ShiftAngleHistory_04d51c  ($04D51C, 98 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D51C: move.w #$97, d1
pub fn i_04d51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D520;
    cpu.ir = 0x323C;
    let v1: u32 = 0x97;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D520: move.b -$1(a6, d1.w), (a6, d1.w)
pub fn i_04d520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D526;
    cpu.ir = 0x1DB6;
    let ad2 = cpu.a[6].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    24
}
/// $04D526: subq.w #$1, d1
pub fn i_04d526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D528;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D528: cmpi.w #$88, d1
pub fn i_04d528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D52C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x88, Size::Word, false);
    8
}
/// $04D52C: bgt.b $4d520
pub fn i_04d52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D52E;
    cpu.ir = 0x6EF2;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D520; return 10; }
    8
}
/// $04D52E: move.w $34(a6), d0
pub fn i_04d52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D532;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D532: move.b d0, $88(a6)
pub fn i_04d532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D536;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D536: move.b $97(a6), d0
pub fn i_04d536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D53A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04D53A: andi.w #$1f, d0
pub fn i_04d53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D53E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D53E: asl.w #$3, d0
pub fn i_04d53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D540;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04D540: move.w $36(a6), d1
pub fn i_04d540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D544;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D544: jsr $4d668(pc)
pub fn i_04d544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D548;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D548);
    cpu.pc = 0x4D668;
    16
}
/// $04D548: bcc.w $4d558
pub fn i_04d548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D54C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D558; return 10; }
    8
}
/// $04D54C: btst.b #$4, $97(a6)
pub fn i_04d54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D552;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04D552: beq.w $4d558
pub fn i_04d552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D556;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D558; return 10; }
    8
}
/// $04D556: asr.w #$1, d1
pub fn i_04d556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D558;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04D558: jsr $13c0e.l
pub fn i_04d558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D55E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D55E);
    cpu.pc = 0x13C0E;
    16
}
/// $04D55E: move.w d1, $28(a6)
pub fn i_04d55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D562;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D562: move.w d2, $2a(a6)
pub fn i_04d562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D566;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D566: move.w #$0, $2e(a6)
pub fn i_04d566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D56C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D56C: btst.b #$4, $97(a6)
pub fn i_04d56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D572;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04D572: beq.w $4d57c
pub fn i_04d572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D576;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D57C; return 10; }
    8
}
/// $04D576: move.w #$ffa0, $2e(a6)
pub fn i_04d576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D57C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D57C: rts
pub fn i_04d57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D57E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Angle16_04d57e  ($04D57E, 4 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D57E: move.w #$10, d0
pub fn i_04d57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D582;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_04d582  ($04D582, 8 B, thunks/jsr_abs_thunks.c)
/// $04D582: jsr $5e9e4.l
pub fn i_04d582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D588;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D588);
    cpu.pc = 0x5E9E4;
    16
}
/// $04D588: rts
pub fn i_04d588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D58A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AngleToTargetMirror_04d58a  ($04D58A, 34 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D58A: jsr $5e136.l
pub fn i_04d58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D590);
    cpu.pc = 0x5E136;
    16
}
/// $04D590: addi.w #$8, d0
pub fn i_04d590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D594;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D594: asr.w #$3, d0
pub fn i_04d594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D596;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04D596: btst.b #$4, d0
pub fn i_04d596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D59A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D59A: bne.w $4d5a6
pub fn i_04d59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D59E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D5A6; return 10; }
    8
}
/// $04D59E: neg.w d0
pub fn i_04d59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5A0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D5A0: clr.w d1
pub fn i_04d5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5A2;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D5A2: add.w d0, d1
pub fn i_04d5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5A4;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04D5A4: move.w d1, d0
pub fn i_04d5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5A6;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D5A6: andi.w #$1f, d0
pub fn i_04d5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5AA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D5AA: rts
pub fn i_04d5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_OffworldA_04d5ac  ($04D5AC, 16 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D5AC: lea.l $293fd4.l, a0
pub fn i_04d5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x293FD4;
    20
}
/// $04D5B2: jsr $5dd5c.l
pub fn i_04d5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D5B8);
    cpu.pc = 0x5DD5C;
    16
}
/// $04D5B8: bcc.w $4d5c2
pub fn i_04d5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D5C2; return 10; }
    8
}
// ===== SetC_04d5bc  ($04D5BC, 6 B, thunks/ccr_helpers.c)
/// $04D5BC: ori.b #$1, ccr
pub fn i_04d5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5C0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $04D5C0: rts
pub fn i_04d5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_04d5c2  ($04D5C2, 6 B, thunks/ccr_helpers.c)
/// $04D5C2: andi.b #$fe, ccr
pub fn i_04d5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5C6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $04D5C6: rts
pub fn i_04d5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_OffworldB_04d5c8  ($04D5C8, 16 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D5C8: lea.l $293fde.l, a0
pub fn i_04d5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x293FDE;
    20
}
/// $04D5CE: jsr $5dd56.l
pub fn i_04d5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D5D4);
    cpu.pc = 0x5DD56;
    16
}
/// $04D5D4: bcc.w $4d5de
pub fn i_04d5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D5DE; return 10; }
    8
}
// ===== SetC_04d5d8  ($04D5D8, 6 B, thunks/ccr_helpers.c)
/// $04D5D8: ori.b #$1, ccr
pub fn i_04d5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5DC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $04D5DC: rts
pub fn i_04d5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_04d5de  ($04D5DE, 6 B, thunks/ccr_helpers.c)
/// $04D5DE: andi.b #$fe, ccr
pub fn i_04d5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5E2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $04D5E2: rts
pub fn i_04d5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_CopyParentAngle_04d5e4  ($04D5E4, 12 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D5E4: movea.l $c(a6), a0
pub fn i_04d5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D5E8: move.w $34(a0), $34(a6)
pub fn i_04d5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5EE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04D5EE: rts
pub fn i_04d5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AddMuzzleOffset_04d5f0  ($04D5F0, 34 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D5F0: lea.l $294044.l, a1
pub fn i_04d5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x294044;
    20
}
/// $04D5F6: move.w $34(a6), d0
pub fn i_04d5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D5FA: andi.w #$1f, d0
pub fn i_04d5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D5FE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D5FE: asl.w #$2, d0
pub fn i_04d5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D600;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04D600: move.w (a1, d0.w), d1
pub fn i_04d600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D604;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D604: add.w d1, $22(a0)
pub fn i_04d604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D608;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D608: move.w $2(a1, d0.w), d1
pub fn i_04d608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D60C;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D60C: add.w d1, $24(a0)
pub fn i_04d60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D610;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D610: rts
pub fn i_04d610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D612;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SpriteIdxByAngle_04d612  ($04D612, 18 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D612: andi.w #$1f, d0
pub fn i_04d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D616;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D616: lea.l $2941c4.l, a0
pub fn i_04d616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D61C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2941C4;
    20
}
/// $04D61C: asl.w #$1, d0
pub fn i_04d61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D61E;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04D61E: move.w (a0, d0.w), d0
pub fn i_04d61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D622;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D622: rts
pub fn i_04d622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D624;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AddOffsetA_04d624  ($04D624, 34 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D624: lea.l $2940c4.l, a1
pub fn i_04d624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D62A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2940C4;
    20
}
/// $04D62A: move.w $34(a6), d0
pub fn i_04d62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D62E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D62E: andi.w #$1f, d0
pub fn i_04d62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D632;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D632: asl.w #$2, d0
pub fn i_04d632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D634;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04D634: move.w (a1, d0.w), d1
pub fn i_04d634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D638;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D638: add.w d1, $22(a6)
pub fn i_04d638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D63C;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D63C: move.w $2(a1, d0.w), d1
pub fn i_04d63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D640;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D640: add.w d1, $24(a6)
pub fn i_04d640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D644;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D644: rts
pub fn i_04d644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D646;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_AddOffsetB_04d646  ($04D646, 34 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D646: lea.l $294144.l, a1
pub fn i_04d646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D64C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x294144;
    20
}
/// $04D64C: move.w $34(a6), d0
pub fn i_04d64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D650;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D650: andi.w #$1f, d0
pub fn i_04d650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D654;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D654: asl.w #$2, d0
pub fn i_04d654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D656;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04D656: move.w (a1, d0.w), d1
pub fn i_04d656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D65A;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D65A: add.w d1, $22(a6)
pub fn i_04d65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D65E;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D65E: move.w $2(a1, d0.w), d1
pub fn i_04d65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D662;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04D662: add.w d1, $24(a6)
pub fn i_04d662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D666;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D666: rts
pub fn i_04d666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D668;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_GroundProbeUp_04d668  ($04D668, 52 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D668: movem.w d0-d1, -(a7)
pub fn i_04d668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D66C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $04D66C: move.w $22(a6), d1
pub fn i_04d66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D670;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D670: move.w $24(a6), d2
pub fn i_04d670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D674;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D674: move.w #$7, d5
pub fn i_04d674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D678;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D678: subi.w #$8, d2
pub fn i_04d678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D67C;
    cpu.ir = 0x442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D67C: subi.w #$1, d5
pub fn i_04d67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D680;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D680: beq.w $4d6a2
pub fn i_04d680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D684;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D6A2; return 10; }
    8
}
/// $04D684: movem.w d5, -(a7)
pub fn i_04d684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D688;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    cpu.a[7] = a;
    20
}
/// $04D688: jsr $280c6.l
pub fn i_04d688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D68E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D68E);
    cpu.pc = 0x280C6;
    16
}
/// $04D68E: movem.w (a7)+, d5
pub fn i_04d68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D692;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04D692: cmpi.b #$1, d0
pub fn i_04d692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D696;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $04D696: bne.b $4d678
pub fn i_04d696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D698;
    cpu.ir = 0x66E0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D678; return 10; }
    8
}
/// $04D698: movem.w (a7)+, d0-d1
pub fn i_04d698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D69C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== SetC_04d69c  ($04D69C, 6 B, thunks/ccr_helpers.c)
/// $04D69C: ori.b #$1, ccr
pub fn i_04d69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6A0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $04D6A0: rts
pub fn i_04d6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_GroundProbeRestore_04d6a2  ($04D6A2, 4 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D6A2: movem.w (a7)+, d0-d1
pub fn i_04d6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6A6;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== ClearC_04d6a6  ($04D6A6, 6 B, thunks/ccr_helpers.c)
/// $04D6A6: andi.b #$fe, ccr
pub fn i_04d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6AA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $04D6AA: rts
pub fn i_04d6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_ClampAngle_04d6ac  ($04D6AC, 36 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D6AC: cmpi.w #$11, $34(a6)
pub fn i_04d6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6B2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x11, Size::Word, false);
    16
}
/// $04D6B2: bne.w $4d6be
pub fn i_04d6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D6BE; return 10; }
    8
}
/// $04D6B6: addi.w #$2, $34(a6)
pub fn i_04d6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6BC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D6BC: rts
pub fn i_04d6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04D6BE: cmpi.w #$1f, $34(a6)
pub fn i_04d6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6C4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F, Size::Word, false);
    16
}
/// $04D6C4: bne.w $4d6ce
pub fn i_04d6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D6CE; return 10; }
    8
}
/// $04D6C8: subi.w #$2, $34(a6)
pub fn i_04d6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6CE;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D6CE: rts
pub fn i_04d6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_SlotPrioCheck_04d6d0  ($04D6D0, 16 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D6D0: movea.l $8(a6), a1
pub fn i_04d6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6D4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D6D4: move.b $10(a6), d0
pub fn i_04d6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04D6D8: cmp.b $10(a1), d0
pub fn i_04d6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6DC;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04D6DC: bcs.w $4d6e6
pub fn i_04d6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D6E6; return 10; }
    8
}
// ===== ClearXN_04d6e0  ($04D6E0, 6 B, thunks/ccr_helpers.c)
/// $04D6E0: andi.b #$ee, ccr
pub fn i_04d6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6E4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04D6E4: rts
pub fn i_04d6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04d6e6  ($04D6E6, 6 B, thunks/ccr_helpers.c)
/// $04D6E6: ori.b #$11, ccr
pub fn i_04d6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6EA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04D6EA: rts
pub fn i_04d6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Static_04d6ec  ($04D6EC, 94 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D6EC: movea.l $3c(a6), a1
pub fn i_04d6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6F0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D6F0: jsr $2942a.l
pub fn i_04d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D6F6);
    cpu.pc = 0x2942A;
    16
}
/// $04D6F6: move.w #$8000, $38(a6)
pub fn i_04d6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D6FC: jsr $267e2.l
pub fn i_04d6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D702);
    cpu.pc = 0x267E2;
    16
}
/// $04D702: jsr $27cee.l
pub fn i_04d702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D708);
    cpu.pc = 0x27CEE;
    16
}
/// $04D708: bra.w $4d716
pub fn i_04d708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D70C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D716;
    10
}
/// $04D70C: movea.l $3c(a6), a1
pub fn i_04d70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D710;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D710: jsr $2942a.l
pub fn i_04d710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D716);
    cpu.pc = 0x2942A;
    16
}
/// $04D716: move.b #$ff, $32(a6)
pub fn i_04d716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D71C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D71C: move.b #$ff, $33(a6)
pub fn i_04d71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D722;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D722: move.b #$0, $3a(a6)
pub fn i_04d722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D728;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D728: lea.l $4d72e(pc), a1
pub fn i_04d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D72C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D72E;
    16
}
/// $04D72C: move.l a1, (a6)
pub fn i_04d72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D72E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D72E: jsr $2783a.l
pub fn i_04d72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D734);
    cpu.pc = 0x2783A;
    16
}
/// $04D734: jsr $28d70.l
pub fn i_04d734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D73A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D73A);
    cpu.pc = 0x28D70;
    16
}
/// $04D73A: jsr $4fa70(pc)
pub fn i_04d73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D73E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D73E);
    cpu.pc = 0x4FA70;
    16
}
/// $04D73E: bcc.w $4d748
pub fn i_04d73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D742;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D748; return 10; }
    8
}
/// $04D742: jmp $518.l
pub fn i_04d742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D748;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D748: rts
pub fn i_04d748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D74A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Lamp_04d74a  ($04D74A, 56 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D74A: movea.l $3c(a6), a1
pub fn i_04d74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D74E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D74E: jsr $2942a.l
pub fn i_04d74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D754);
    cpu.pc = 0x2942A;
    16
}
/// $04D754: move.w #$41, d1
pub fn i_04d754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D758;
    cpu.ir = 0x323C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D758: jsr $236e.l
pub fn i_04d758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D75E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D75E);
    cpu.pc = 0x236E;
    16
}
/// $04D75E: move.b #$ff, $32(a6)
pub fn i_04d75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D764;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D764: move.b #$ff, $33(a6)
pub fn i_04d764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D76A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D76A: move.b #$0, $3a(a6)
pub fn i_04d76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D770;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D770: lea.l $294280.l, a0
pub fn i_04d770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D776;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294280;
    20
}
/// $04D776: jsr $28cd4.l
pub fn i_04d776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D77C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D77C);
    cpu.pc = 0x28CD4;
    16
}
/// $04D77C: lea.l $4d782(pc), a1
pub fn i_04d77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D780;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D782;
    16
}
/// $04D780: move.l a1, (a6)
pub fn i_04d780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D782;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Prop_LampIdle_04d782  ($04D782, 90 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D782: jsr $2783a.l
pub fn i_04d782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D788);
    cpu.pc = 0x2783A;
    16
}
/// $04D788: jsr $28d70.l
pub fn i_04d788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D78E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D78E);
    cpu.pc = 0x28D70;
    16
}
/// $04D78E: jsr $2870a.l
pub fn i_04d78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D794;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D794);
    cpu.pc = 0x2870A;
    16
}
/// $04D794: bcc.w $4d7cc
pub fn i_04d794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D798;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D7CC; return 10; }
    8
}
/// $04D798: move.w #$10a9, d0
pub fn i_04d798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D79C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D79C: jsr $2352.l
pub fn i_04d79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7A2);
    cpu.pc = 0x2352;
    16
}
/// $04D7A2: jsr $5e9b6.l
pub fn i_04d7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7A8);
    cpu.pc = 0x5E9B6;
    16
}
/// $04D7A8: andi.w #$3, d0
pub fn i_04d7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7AC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D7AC: bne.w $4d7c6
pub fn i_04d7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D7C6; return 10; }
    8
}
/// $04D7B0: lea.l $4d81a(pc), a1
pub fn i_04d7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D81A;
    16
}
/// $04D7B4: jsr $6fe.l
pub fn i_04d7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7BA);
    cpu.pc = 0x6FE;
    16
}
/// $04D7BA: jsr $5dd02.l
pub fn i_04d7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7C0);
    cpu.pc = 0x5DD02;
    16
}
/// $04D7C0: addi.w #$18, $24(a0)
pub fn i_04d7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7C6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D7C6: lea.l $4d7dc(pc), a1
pub fn i_04d7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D7DC;
    16
}
/// $04D7CA: move.l a1, (a6)
pub fn i_04d7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D7CC: jsr $4fa70(pc)
pub fn i_04d7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D7D0);
    cpu.pc = 0x4FA70;
    16
}
/// $04D7D0: bcc.w $4d7da
pub fn i_04d7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D7DA; return 10; }
    8
}
/// $04D7D4: jmp $518.l
pub fn i_04d7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D7DA: rts
pub fn i_04d7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_LampHit_04d7dc  ($04D7DC, 62 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D7DC: lea.l $294296.l, a0
pub fn i_04d7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294296;
    20
}
/// $04D7E2: jsr $28cd4.l
pub fn i_04d7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7E8);
    cpu.pc = 0x28CD4;
    16
}
/// $04D7E8: lea.l $4d7ee(pc), a1
pub fn i_04d7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D7EE;
    16
}
/// $04D7EC: move.l a1, (a6)
pub fn i_04d7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D7EE: jsr $2783a.l
pub fn i_04d7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7F4);
    cpu.pc = 0x2783A;
    16
}
/// $04D7F4: jsr $28d70.l
pub fn i_04d7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D7FA);
    cpu.pc = 0x28D70;
    16
}
/// $04D7FA: bcc.w $4d80a
pub fn i_04d7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D7FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D80A; return 10; }
    8
}
/// $04D7FE: bclr.b #$3, $13(a6)
pub fn i_04d7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D804;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D804: lea.l $4d782(pc), a1
pub fn i_04d804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D808;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D782;
    16
}
/// $04D808: move.l a1, (a6)
pub fn i_04d808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D80A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D80A: jsr $4fa70(pc)
pub fn i_04d80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D80E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D80E);
    cpu.pc = 0x4FA70;
    16
}
/// $04D80E: bcc.w $4d818
pub fn i_04d80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D812;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D818; return 10; }
    8
}
/// $04D812: jmp $518.l
pub fn i_04d812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D818;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D818: rts
pub fn i_04d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D81A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_LampSpark_04d81a  ($04D81A, 216 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D81A: move.w #$41, d1
pub fn i_04d81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D81E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D81E: jsr $236e.l
pub fn i_04d81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D824;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D824);
    cpu.pc = 0x236E;
    16
}
/// $04D824: bset.b #$6, $12(a6)
pub fn i_04d824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D82A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D82A: move.b #$ff, $32(a6)
pub fn i_04d82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D830;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D830: move.b #$ff, $33(a6)
pub fn i_04d830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D836;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D836: move.b #$0, $3a(a6)
pub fn i_04d836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D83C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D83C: jsr $267e2.l
pub fn i_04d83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D842;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D842);
    cpu.pc = 0x267E2;
    16
}
/// $04D842: move.w #$f0, $72(a6)
pub fn i_04d842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D848;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D848: jsr $5e9b6.l
pub fn i_04d848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D84E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D84E);
    cpu.pc = 0x5E9B6;
    16
}
/// $04D84E: andi.w #$1f, d0
pub fn i_04d84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D852;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D852: add.w d0, $72(a6)
pub fn i_04d852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D856;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D856: jsr $5e9b6.l
pub fn i_04d856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D85C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D85C);
    cpu.pc = 0x5E9B6;
    16
}
/// $04D85C: andi.w #$1f, d0
pub fn i_04d85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D860;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D860: btst.b #$0, d0
pub fn i_04d860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D864;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D864: beq.w $4d86a
pub fn i_04d864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D868;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D86A; return 10; }
    8
}
/// $04D868: neg.w d0
pub fn i_04d868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D86A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D86A: move.w d0, $28(a6)
pub fn i_04d86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D86E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D86E: jsr $5e9b6.l
pub fn i_04d86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D874);
    cpu.pc = 0x5E9B6;
    16
}
/// $04D874: andi.w #$3f, d0
pub fn i_04d874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D878;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D878: btst.b #$0, d0
pub fn i_04d878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D87C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D87C: beq.w $4d882
pub fn i_04d87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D880;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D882; return 10; }
    8
}
/// $04D880: neg.w d0
pub fn i_04d880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D882;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D882: move.w d0, $2a(a6)
pub fn i_04d882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D886;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D886: lea.l $294378.l, a0
pub fn i_04d886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D88C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294378;
    20
}
/// $04D88C: jsr $28cd4.l
pub fn i_04d88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D892);
    cpu.pc = 0x28CD4;
    16
}
/// $04D892: lea.l $4d898(pc), a1
pub fn i_04d892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D896;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D898;
    16
}
/// $04D896: move.l a1, (a6)
pub fn i_04d896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D898;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D898: jsr $27cee.l
pub fn i_04d898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D89E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D89E);
    cpu.pc = 0x27CEE;
    16
}
/// $04D89E: cmpi.w #$14, $72(a6)
pub fn i_04d89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8A4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $04D8A4: bgt.w $4d8be
pub fn i_04d8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8A8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D8BE; return 10; }
    8
}
/// $04D8A8: move.w $72(a6), d0
pub fn i_04d8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8AC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D8AC: btst.b #$0, d0
pub fn i_04d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8B0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04D8B0: beq.w $4d8be
pub fn i_04d8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D8BE; return 10; }
    8
}
/// $04D8B4: subi.b #$1, $46(a6)
pub fn i_04d8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8BA;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D8BA: bra.w $4d8c4
pub fn i_04d8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D8C4;
    10
}
/// $04D8BE: jsr $28d70.l
pub fn i_04d8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D8C4);
    cpu.pc = 0x28D70;
    16
}
/// $04D8C4: subi.w #$1, $72(a6)
pub fn i_04d8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8CA;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D8CA: cmpi.w #$0, $72(a6)
pub fn i_04d8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8D0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04D8D0: bgt.w $4d8da
pub fn i_04d8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8D4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D8DA; return 10; }
    8
}
/// $04D8D4: jmp $518.l
pub fn i_04d8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D8DA: lea.l $296254.l, a0
pub fn i_04d8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296254;
    20
}
/// $04D8E0: jsr $5dd5c.l
pub fn i_04d8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D8E6);
    cpu.pc = 0x5DD5C;
    16
}
/// $04D8E6: bcc.w $4d8f0
pub fn i_04d8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D8F0; return 10; }
    8
}
/// $04D8EA: jmp $518.l
pub fn i_04d8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D8F0: rts
pub fn i_04d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Hut_04d8f2  ($04D8F2, 164 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D8F2: movea.l $3c(a6), a1
pub fn i_04d8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8F6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D8F6: jsr $2942a.l
pub fn i_04d8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D8FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D8FC);
    cpu.pc = 0x2942A;
    16
}
/// $04D8FC: move.w #$3f, d1
pub fn i_04d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D900;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D900: jsr $236e.l
pub fn i_04d900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D906);
    cpu.pc = 0x236E;
    16
}
/// $04D906: move.b #$ff, $32(a6)
pub fn i_04d906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D90C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D90C: move.b #$ff, $33(a6)
pub fn i_04d90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D912;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D912: move.b #$0, $3a(a6)
pub fn i_04d912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D918;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D918: move.b #$0, $21(a6)
pub fn i_04d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D91E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D91E: lea.l $5fa00.l, a1
pub fn i_04d91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D924;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5FA00;
    20
}
/// $04D924: jsr $4ae.l
pub fn i_04d924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D92A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D92A);
    cpu.pc = 0x4AE;
    16
}
/// $04D92A: jsr $5dd22.l
pub fn i_04d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D930);
    cpu.pc = 0x5DD22;
    16
}
/// $04D930: addi.w #$28, $22(a0)
pub fn i_04d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D936;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D936: addi.w #$1, $24(a0)
pub fn i_04d936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D93C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D93C: lea.l $4d9d6(pc), a1
pub fn i_04d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D940;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D9D6;
    16
}
/// $04D940: jsr $4ae.l
pub fn i_04d940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D946);
    cpu.pc = 0x4AE;
    16
}
/// $04D946: jsr $5dd22.l
pub fn i_04d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D94C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D94C);
    cpu.pc = 0x5DD22;
    16
}
/// $04D94C: addi.w #$28, $22(a0)
pub fn i_04d94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D952;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D952: addi.w #$1, $24(a0)
pub fn i_04d952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D958;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D958: lea.l $294234.l, a0
pub fn i_04d958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D95E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294234;
    20
}
/// $04D95E: jsr $28cd4.l
pub fn i_04d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D964);
    cpu.pc = 0x28CD4;
    16
}
/// $04D964: lea.l $4d96a(pc), a1
pub fn i_04d964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D968;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D96A;
    16
}
/// $04D968: move.l a1, (a6)
pub fn i_04d968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D96A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D96A: jsr $2783a.l
pub fn i_04d96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D970);
    cpu.pc = 0x2783A;
    16
}
/// $04D970: jsr $28d70.l
pub fn i_04d970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D976);
    cpu.pc = 0x28D70;
    16
}
/// $04D976: cmpi.b #$ff, $21(a6)
pub fn i_04d976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D97C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D97C: bne.w $4d986
pub fn i_04d97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D980;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D986; return 10; }
    8
}
/// $04D980: lea.l $4d996(pc), a1
pub fn i_04d980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D984;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D996;
    16
}
/// $04D984: move.l a1, (a6)
pub fn i_04d984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D986;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D986: jsr $4fa70(pc)
pub fn i_04d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D98A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D98A);
    cpu.pc = 0x4FA70;
    16
}
/// $04D98A: bcc.w $4d994
pub fn i_04d98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D98E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D994; return 10; }
    8
}
/// $04D98E: jmp $518.l
pub fn i_04d98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D994;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D994: rts
pub fn i_04d994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D996;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HutWreck_04d996  ($04D996, 64 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D996: jsr $2783a.l
pub fn i_04d996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D99C);
    cpu.pc = 0x2783A;
    16
}
/// $04D99C: lea.l $295d38.l, a1
pub fn i_04d99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9A2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295D38;
    20
}
/// $04D9A2: jsr $43fac.l
pub fn i_04d9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9A8);
    cpu.pc = 0x43FAC;
    16
}
/// $04D9A8: lea.l $294244.l, a0
pub fn i_04d9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294244;
    20
}
/// $04D9AE: jsr $28cd4.l
pub fn i_04d9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9B4);
    cpu.pc = 0x28CD4;
    16
}
/// $04D9B4: lea.l $4d9ba(pc), a1
pub fn i_04d9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D9BA;
    16
}
/// $04D9B8: move.l a1, (a6)
pub fn i_04d9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D9BA: jsr $2783a.l
pub fn i_04d9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9C0);
    cpu.pc = 0x2783A;
    16
}
/// $04D9C0: jsr $28d70.l
pub fn i_04d9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9C6);
    cpu.pc = 0x28D70;
    16
}
/// $04D9C6: jsr $4fa70(pc)
pub fn i_04d9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D9CA);
    cpu.pc = 0x4FA70;
    16
}
/// $04D9CA: bcc.w $4d9d4
pub fn i_04d9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D9D4; return 10; }
    8
}
/// $04D9CE: jmp $518.l
pub fn i_04d9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9D4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D9D4: rts
pub fn i_04d9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HutRoof_04d9d6  ($04D9D6, 124 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D9D6: move.w #$3d, d1
pub fn i_04d9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9DA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D9DA: jsr $236e.l
pub fn i_04d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D9E0);
    cpu.pc = 0x236E;
    16
}
/// $04D9E0: move.b #$ff, $32(a6)
pub fn i_04d9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D9E6: move.b #$ff, $33(a6)
pub fn i_04d9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D9EC: move.b #$0, $3a(a6)
pub fn i_04d9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D9F2: move.w #$14, $66(a6)
pub fn i_04d9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D9F8: lea.l $294254.l, a0
pub fn i_04d9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D9FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294254;
    20
}
/// $04D9FE: jsr $28cd4.l
pub fn i_04d9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA04);
    cpu.pc = 0x28CD4;
    16
}
/// $04DA04: lea.l $4da0a(pc), a1
pub fn i_04da04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DA0A;
    16
}
/// $04DA08: move.l a1, (a6)
pub fn i_04da08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DA0A: jsr $2783a.l
pub fn i_04da0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA10);
    cpu.pc = 0x2783A;
    16
}
/// $04DA10: jsr $28d70.l
pub fn i_04da10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA16);
    cpu.pc = 0x28D70;
    16
}
/// $04DA16: jsr $2870a.l
pub fn i_04da16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA1C);
    cpu.pc = 0x2870A;
    16
}
/// $04DA1C: bcc.w $4da32
pub fn i_04da1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DA32; return 10; }
    8
}
/// $04DA20: lea.l $5e766.l, a0
pub fn i_04da20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04DA26: jsr $5e770.l
pub fn i_04da26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA2C);
    cpu.pc = 0x5E770;
    16
}
/// $04DA2C: bclr.b #$3, $13(a6)
pub fn i_04da2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA32;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DA32: jsr $28758.l
pub fn i_04da32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA38);
    cpu.pc = 0x28758;
    16
}
/// $04DA38: bcc.w $4da42
pub fn i_04da38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DA42; return 10; }
    8
}
/// $04DA3C: lea.l $4da52(pc), a1
pub fn i_04da3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DA52;
    16
}
/// $04DA40: move.l a1, (a6)
pub fn i_04da40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DA42: jsr $4fa70(pc)
pub fn i_04da42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DA46);
    cpu.pc = 0x4FA70;
    16
}
/// $04DA46: bcc.w $4da50
pub fn i_04da46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DA50; return 10; }
    8
}
/// $04DA4A: jmp $518.l
pub fn i_04da4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA50;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04DA50: rts
pub fn i_04da50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HutRoofFall_04da52  ($04DA52, 120 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DA52: move.l #$1000, d0
pub fn i_04da52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA58;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04DA58: jsr $51a28.l
pub fn i_04da58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA5E);
    cpu.pc = 0x51A28;
    16
}
/// $04DA5E: lea.l $295fba.l, a0
pub fn i_04da5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x295FBA;
    20
}
/// $04DA64: move.l a0, $4c(a6)
pub fn i_04da64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA68;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DA68: jsr $283ca.l
pub fn i_04da68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA6E);
    cpu.pc = 0x283CA;
    16
}
/// $04DA6E: jsr $283ca.l
pub fn i_04da6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA74);
    cpu.pc = 0x283CA;
    16
}
/// $04DA74: jsr $283d8.l
pub fn i_04da74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA7A);
    cpu.pc = 0x283D8;
    16
}
/// $04DA7A: lea.l $29426a.l, a0
pub fn i_04da7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29426A;
    20
}
/// $04DA80: jsr $28cd4.l
pub fn i_04da80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA86);
    cpu.pc = 0x28CD4;
    16
}
/// $04DA86: lea.l $4da8c(pc), a1
pub fn i_04da86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DA8C;
    16
}
/// $04DA8A: move.l a1, (a6)
pub fn i_04da8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DA8C: jsr $2783a.l
pub fn i_04da8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA92);
    cpu.pc = 0x2783A;
    16
}
/// $04DA92: jsr $28d70.l
pub fn i_04da92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DA98);
    cpu.pc = 0x28D70;
    16
}
/// $04DA98: bcc.w $4dad0
pub fn i_04da98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DAD0; return 10; }
    8
}
/// $04DA9C: lea.l $ffff.w, a0
pub fn i_04da9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAA0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04DAA0: move.l a0, $4c(a6)
pub fn i_04daa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAA4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DAA4: jsr $283ca.l
pub fn i_04daa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DAAA);
    cpu.pc = 0x283CA;
    16
}
/// $04DAAA: move.w #$1028, d0
pub fn i_04daaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DAAE: jsr $2352.l
pub fn i_04daae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DAB4);
    cpu.pc = 0x2352;
    16
}
/// $04DAB4: movea.l $c(a6), a0
pub fn i_04dab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAB8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DAB8: move.b #$ff, $21(a0)
pub fn i_04dab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DABE;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DABE: lea.l $295bd8.l, a1
pub fn i_04dabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAC4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BD8;
    20
}
/// $04DAC4: jsr $77c7e.l
pub fn i_04dac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DACA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DACA);
    cpu.pc = 0x77C7E;
    16
}
// ===== SetTaskHandler_04daca  ($04DACA, 8 B, kernel/task_handlers.c)
/// $04DACA: lea.l $4f2a4(pc), a1
pub fn i_04daca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DACE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04DACE: move.l a1, (a6)
pub fn i_04dace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DAD0: rts
pub fn i_04dad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Tower_04dad2  ($04DAD2, 154 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DAD2: movea.l $3c(a6), a1
pub fn i_04dad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAD6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04DAD6: jsr $2942a.l
pub fn i_04dad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DADC);
    cpu.pc = 0x2942A;
    16
}
/// $04DADC: lea.l $4f2c2(pc), a1
pub fn i_04dadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2C2;
    16
}
/// $04DAE0: jsr $4ae.l
pub fn i_04dae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DAE6);
    cpu.pc = 0x4AE;
    16
}
/// $04DAE6: jsr $5dd22.l
pub fn i_04dae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DAEC);
    cpu.pc = 0x5DD22;
    16
}
/// $04DAEC: addi.w #$5c, $22(a0)
pub fn i_04daec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAF2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x5C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DAF2: addi.w #$22, $24(a0)
pub fn i_04daf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAF8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x22, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DAF8: lea.l $4db72(pc), a1
pub fn i_04daf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DAFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DB72;
    16
}
/// $04DAFC: jsr $4ae.l
pub fn i_04dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB02);
    cpu.pc = 0x4AE;
    16
}
/// $04DB02: jsr $5dd22.l
pub fn i_04db02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB08);
    cpu.pc = 0x5DD22;
    16
}
/// $04DB08: lea.l $4dce6(pc), a1
pub fn i_04db08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DCE6;
    16
}
/// $04DB0C: jsr $4ae.l
pub fn i_04db0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB12);
    cpu.pc = 0x4AE;
    16
}
/// $04DB12: jsr $5dd22.l
pub fn i_04db12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB18);
    cpu.pc = 0x5DD22;
    16
}
/// $04DB18: addi.w #$80, $22(a0)
pub fn i_04db18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB1E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DB1E: addi.w #$30, $24(a0)
pub fn i_04db1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB24;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DB24: move.b #$0, $21(a6)
pub fn i_04db24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB2A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB2A: move.b #$0, $20(a6)
pub fn i_04db2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB30: lea.l $4db36(pc), a1
pub fn i_04db30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DB36;
    16
}
/// $04DB34: move.l a1, (a6)
pub fn i_04db34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DB36: cmpi.b #$ff, $21(a6)
pub fn i_04db36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB3C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04DB3C: bne.w $4db48
pub fn i_04db3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4DB48; return 10; }
    8
}
/// $04DB40: jmp $518.l
pub fn i_04db40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB46;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04DB46: rts
pub fn i_04db46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04DB48: move.b $20(a6), d0
pub fn i_04db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB4C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04DB4C: cmp.b $21(a6), d0
pub fn i_04db4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB50;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04DB50: beq.w $4db58
pub fn i_04db50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB54;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4DB58; return 10; }
    8
}
/// $04DB54: jsr $4fa8a(pc)
pub fn i_04db54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DB58);
    cpu.pc = 0x4FA8A;
    16
}
/// $04DB58: cmpi.b #$12, $21(a6)
pub fn i_04db58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB5E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x12, Size::Byte, false);
    16
}
/// $04DB5E: bne.w $4db68
pub fn i_04db5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4DB68; return 10; }
    8
}
/// $04DB62: jmp $518.l
pub fn i_04db62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04DB68: move.b $21(a6), d0
pub fn i_04db68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB6C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
// ===== SetTaskB_04db6c  ($04DB6C, 6 B, kernel/task_setters_b.c)
/// $04DB6C: move.b d0, $20(a6)
pub fn i_04db6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB70;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB70: rts
pub fn i_04db70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerTop_04db72  ($04DB72, 208 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DB72: lea.l $e92b2.l, a1
pub fn i_04db72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE92B2;
    20
}
/// $04DB78: move.w #$75, d0
pub fn i_04db78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB7C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DB7C: move.b #$0, $75(a6)
pub fn i_04db7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB82;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB82: jsr $4429e.l
pub fn i_04db82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DB88);
    cpu.pc = 0x4429E;
    16
}
/// $04DB88: move.w #$50, $70(a6)
pub fn i_04db88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DB8E: move.w #$3c, $66(a6)
pub fn i_04db8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DB94: move.b #$0, $20(a6)
pub fn i_04db94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DB9A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DB9A: lea.l $294446.l, a0
pub fn i_04db9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294446;
    20
}
/// $04DBA0: jsr $28cd4.l
pub fn i_04dba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBA6);
    cpu.pc = 0x28CD4;
    16
}
/// $04DBA6: lea.l $4dbac(pc), a1
pub fn i_04dba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DBAC;
    16
}
/// $04DBAA: move.l a1, (a6)
pub fn i_04dbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DBAC: jsr $2783a.l
pub fn i_04dbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBB2);
    cpu.pc = 0x2783A;
    16
}
/// $04DBB2: jsr $28d70.l
pub fn i_04dbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBB8);
    cpu.pc = 0x28D70;
    16
}
/// $04DBB8: jsr $2870a.l
pub fn i_04dbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBBE);
    cpu.pc = 0x2870A;
    16
}
/// $04DBBE: bcc.w $4dbd4
pub fn i_04dbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DBD4; return 10; }
    8
}
/// $04DBC2: lea.l $5e766.l, a0
pub fn i_04dbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04DBC8: jsr $5e770.l
pub fn i_04dbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBCE);
    cpu.pc = 0x5E770;
    16
}
/// $04DBCE: bclr.b #$3, $13(a6)
pub fn i_04dbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBD4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DBD4: jsr $28758.l
pub fn i_04dbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBDA);
    cpu.pc = 0x28758;
    16
}
/// $04DBDA: bcc.w $4dc2a
pub fn i_04dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBDE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DC2A; return 10; }
    8
}
/// $04DBDE: addi.w #$10, $22(a6)
pub fn i_04dbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBE4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DBE4: lea.l $295b5a.l, a1
pub fn i_04dbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBEA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B5A;
    20
}
/// $04DBEA: jsr $77c7e.l
pub fn i_04dbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBF0);
    cpu.pc = 0x77C7E;
    16
}
/// $04DBF0: lea.l $295b6c.l, a1
pub fn i_04dbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBF6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B6C;
    20
}
/// $04DBF6: jsr $77c7e.l
pub fn i_04dbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DBFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DBFC);
    cpu.pc = 0x77C7E;
    16
}
/// $04DBFC: lea.l $295cb0.l, a1
pub fn i_04dbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC02;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04DC02: jsr $77c7e.l
pub fn i_04dc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DC08);
    cpu.pc = 0x77C7E;
    16
}
/// $04DC08: addi.w #$20, $22(a0)
pub fn i_04dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC0E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DC0E: addi.w #$30, $24(a0)
pub fn i_04dc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC14;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DC14: movea.l $c(a6), a0
pub fn i_04dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC18;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DC18: addi.b #$10, $21(a0)
pub fn i_04dc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC1E;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DC1E: move.b #$ff, $20(a6)
pub fn i_04dc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC24;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DC24: lea.l $4dc4a(pc), a1
pub fn i_04dc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DC4A;
    16
}
/// $04DC28: move.l a1, (a6)
pub fn i_04dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DC2A: jsr $4fa70(pc)
pub fn i_04dc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DC2E);
    cpu.pc = 0x4FA70;
    16
}
/// $04DC2E: bcc.w $4dc48
pub fn i_04dc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DC48; return 10; }
    8
}
/// $04DC32: move.b #$ff, $20(a6)
pub fn i_04dc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC38;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DC38: movea.l $c(a6), a0
pub fn i_04dc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC3C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DC3C: ori.b #$f0, $21(a0)
pub fn i_04dc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC42;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_04dc42  ($04DC42, 8 B, kernel/task_handlers.c)
/// $04DC42: lea.l $4dc88(pc), a1
pub fn i_04dc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DC88;
    16
}
/// $04DC46: move.l a1, (a6)
pub fn i_04dc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DC48: rts
pub fn i_04dc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerTopWreck_04dc4a  ($04DC4A, 62 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DC4A: move.l #$1000, d0
pub fn i_04dc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC50;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04DC50: jsr $51a28.l
pub fn i_04dc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DC56);
    cpu.pc = 0x51A28;
    16
}
/// $04DC56: lea.l $ffff.w, a0
pub fn i_04dc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC5A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04DC5A: move.l a0, $48(a6)
pub fn i_04dc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC5E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DC5E: move.b #$ff, $75(a6)
pub fn i_04dc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC64;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DC64: move.w #$1030, d0
pub fn i_04dc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DC68: jsr $2352.l
pub fn i_04dc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DC6E);
    cpu.pc = 0x2352;
    16
}
/// $04DC6E: lea.l $29556a.l, a2
pub fn i_04dc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC74;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29556A;
    20
}
/// $04DC74: jsr $5022a(pc)
pub fn i_04dc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DC78);
    cpu.pc = 0x5022A;
    16
}
/// $04DC78: lea.l $2955e2.l, a2
pub fn i_04dc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC7E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2955E2;
    20
}
/// $04DC7E: jsr $5022a(pc)
pub fn i_04dc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DC82);
    cpu.pc = 0x5022A;
    16
}
/// $04DC82: lea.l $4dc88(pc), a1
pub fn i_04dc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DC88;
    16
}
/// $04DC86: move.l a1, (a6)
pub fn i_04dc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Prop_TowerTopWreckLoop_04dc88  ($04DC88, 6 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DC88: jsr $2783a.l
pub fn i_04dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DC8E);
    cpu.pc = 0x2783A;
    16
}
// ===== SetTaskHandler_04dc8e  ($04DC8E, 8 B, kernel/task_handlers.c)
/// $04DC8E: lea.l $4f2a4(pc), a1
pub fn i_04dc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04DC92: move.l a1, (a6)
pub fn i_04dc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DC94: rts
pub fn i_04dc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerFlag_04dc96  ($04DC96, 80 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DC96: move.w #$40, d1
pub fn i_04dc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC9A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DC9A: jsr $236e.l
pub fn i_04dc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCA0);
    cpu.pc = 0x236E;
    16
}
/// $04DCA0: move.b #$ff, $32(a6)
pub fn i_04dca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCA6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DCA6: move.b #$ff, $33(a6)
pub fn i_04dca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCAC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DCAC: move.b #$0, $3a(a6)
pub fn i_04dcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCB2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DCB2: lea.l $29440a.l, a0
pub fn i_04dcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29440A;
    20
}
/// $04DCB8: jsr $28cd4.l
pub fn i_04dcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCBE);
    cpu.pc = 0x28CD4;
    16
}
/// $04DCBE: lea.l $4dcc4(pc), a1
pub fn i_04dcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DCC4;
    16
}
/// $04DCC2: move.l a1, (a6)
pub fn i_04dcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DCC4: jsr $2783a.l
pub fn i_04dcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCCA);
    cpu.pc = 0x2783A;
    16
}
/// $04DCCA: jsr $28d70.l
pub fn i_04dcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCD0);
    cpu.pc = 0x28D70;
    16
}
/// $04DCD0: movea.l $c(a6), a0
pub fn i_04dcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DCD4: cmpi.b #$ff, $20(a0)
pub fn i_04dcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCDA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04DCDA: bne.w $4dce4
pub fn i_04dcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCDE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4DCE4; return 10; }
    8
}
/// $04DCDE: jmp $518.l
pub fn i_04dcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCE4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04DCE4: rts
pub fn i_04dce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBase_04dce6  ($04DCE6, 254 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DCE6: lea.l $5f384.l, a1
pub fn i_04dce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCEC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5F384;
    20
}
/// $04DCEC: jsr $4ae.l
pub fn i_04dcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCF2);
    cpu.pc = 0x4AE;
    16
}
/// $04DCF2: jsr $5dd22.l
pub fn i_04dcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DCF8);
    cpu.pc = 0x5DD22;
    16
}
/// $04DCF8: addi.w #$29, $22(a0)
pub fn i_04dcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DCFE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x29, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DCFE: addi.w #$9, $24(a0)
pub fn i_04dcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD04;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x9, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DD04: move.w #$50, $70(a6)
pub fn i_04dd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DD0A: move.w #$3c, $66(a6)
pub fn i_04dd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DD10: move.b #$0, $21(a6)
pub fn i_04dd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DD16: move.b #$0, $20(a6)
pub fn i_04dd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD1C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DD1C: lea.l $29445c.l, a0
pub fn i_04dd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29445C;
    20
}
/// $04DD22: jsr $28cd4.l
pub fn i_04dd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD28);
    cpu.pc = 0x28CD4;
    16
}
/// $04DD28: lea.l $4dd2e(pc), a1
pub fn i_04dd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DD2E;
    16
}
/// $04DD2C: move.l a1, (a6)
pub fn i_04dd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DD2E: jsr $2783a.l
pub fn i_04dd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD34);
    cpu.pc = 0x2783A;
    16
}
/// $04DD34: jsr $28d70.l
pub fn i_04dd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD3A);
    cpu.pc = 0x28D70;
    16
}
/// $04DD3A: cmpi.b #$1, $20(a6)
pub fn i_04dd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD40;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04DD40: bge.w $4dd84
pub fn i_04dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD44;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x4DD84; return 10; }
    8
}
/// $04DD44: cmpi.w #$1e, $66(a6)
pub fn i_04dd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD4A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $04DD4A: bgt.w $4dd84
pub fn i_04dd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD4E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4DD84; return 10; }
    8
}
/// $04DD4E: move.w #$1028, d0
pub fn i_04dd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DD52: jsr $2352.l
pub fn i_04dd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD58);
    cpu.pc = 0x2352;
    16
}
/// $04DD58: lea.l $295ba2.l, a1
pub fn i_04dd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD5E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BA2;
    20
}
/// $04DD5E: jsr $77c7e.l
pub fn i_04dd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD64);
    cpu.pc = 0x77C7E;
    16
}
/// $04DD64: lea.l $2955f6.l, a2
pub fn i_04dd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD6A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2955F6;
    20
}
/// $04DD6A: jsr $5022a(pc)
pub fn i_04dd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DD6E);
    cpu.pc = 0x5022A;
    16
}
/// $04DD6E: move.b #$ff, $21(a6)
pub fn i_04dd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DD74: movea.l $c(a6), a0
pub fn i_04dd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD78;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DD78: addi.b #$1, $21(a0)
pub fn i_04dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD7E;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DD7E: move.b #$1, $20(a6)
pub fn i_04dd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD84;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DD84: jsr $2870a.l
pub fn i_04dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD8A);
    cpu.pc = 0x2870A;
    16
}
/// $04DD8A: bcc.w $4dda0
pub fn i_04dd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DDA0; return 10; }
    8
}
/// $04DD8E: lea.l $5e766.l, a0
pub fn i_04dd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04DD94: jsr $5e770.l
pub fn i_04dd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DD9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DD9A);
    cpu.pc = 0x5E770;
    16
}
/// $04DD9A: bclr.b #$3, $13(a6)
pub fn i_04dd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDA0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DDA0: jsr $28758.l
pub fn i_04dda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DDA6);
    cpu.pc = 0x28758;
    16
}
/// $04DDA6: bcc.w $4ddd2
pub fn i_04dda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DDD2; return 10; }
    8
}
/// $04DDAA: lea.l $295ba2.l, a1
pub fn i_04ddaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDB0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BA2;
    20
}
/// $04DDB0: jsr $77c7e.l
pub fn i_04ddb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DDB6);
    cpu.pc = 0x77C7E;
    16
}
/// $04DDB6: lea.l $295bb4.l, a1
pub fn i_04ddb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDBC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BB4;
    20
}
/// $04DDBC: jsr $77c7e.l
pub fn i_04ddbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DDC2);
    cpu.pc = 0x77C7E;
    16
}
/// $04DDC2: movea.l $c(a6), a0
pub fn i_04ddc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDC6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DDC6: addi.b #$1, $21(a0)
pub fn i_04ddc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDCC;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DDCC: lea.l $4ddec(pc), a1
pub fn i_04ddcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DDEC;
    16
}
/// $04DDD0: move.l a1, (a6)
pub fn i_04ddd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DDD2: jsr $4fa70(pc)
pub fn i_04ddd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DDD6);
    cpu.pc = 0x4FA70;
    16
}
/// $04DDD6: bcc.w $4ddea
pub fn i_04ddd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DDEA; return 10; }
    8
}
/// $04DDDA: movea.l $c(a6), a0
pub fn i_04ddda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDDE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04DDDE: ori.b #$f, $21(a0)
pub fn i_04ddde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDE4;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_04dde4  ($04DDE4, 8 B, kernel/task_handlers.c)
/// $04DDE4: lea.l $4de32(pc), a1
pub fn i_04dde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DE32;
    16
}
/// $04DDE8: move.l a1, (a6)
pub fn i_04dde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DDEA: rts
pub fn i_04ddea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerBaseWreck_04ddec  ($04DDEC, 70 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DDEC: move.l #$1000, d0
pub fn i_04ddec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDF2;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04DDF2: jsr $51a28.l
pub fn i_04ddf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DDF8);
    cpu.pc = 0x51A28;
    16
}
/// $04DDF8: lea.l $ffff.w, a0
pub fn i_04ddf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DDFC;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04DDFC: move.l a0, $48(a6)
pub fn i_04ddfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE00;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DE00: move.w #$1028, d0
pub fn i_04de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DE04: jsr $2352.l
pub fn i_04de04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE0A);
    cpu.pc = 0x2352;
    16
}
/// $04DE0A: lea.l $29560a.l, a2
pub fn i_04de0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE10;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29560A;
    20
}
/// $04DE10: jsr $5022a(pc)
pub fn i_04de10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DE14);
    cpu.pc = 0x5022A;
    16
}
/// $04DE14: lea.l $295cb0.l, a1
pub fn i_04de14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE1A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04DE1A: jsr $77c7e.l
pub fn i_04de1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE20);
    cpu.pc = 0x77C7E;
    16
}
/// $04DE20: addi.w #$ffe0, $22(a0)
pub fn i_04de20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE26;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DE26: addi.w #$30, $24(a0)
pub fn i_04de26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE2C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DE2C: lea.l $4de32(pc), a1
pub fn i_04de2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DE32;
    16
}
/// $04DE30: move.l a1, (a6)
pub fn i_04de30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Prop_TowerBaseWreckLoop_04de32  ($04DE32, 6 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DE32: jsr $2783a.l
pub fn i_04de32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE38);
    cpu.pc = 0x2783A;
    16
}
// ===== SetTaskHandler_04de38  ($04DE38, 8 B, kernel/task_handlers.c)
/// $04DE38: lea.l $4f2a4(pc), a1
pub fn i_04de38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04DE3C: move.l a1, (a6)
pub fn i_04de3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DE3E: rts
pub fn i_04de3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Bunker_04de40  ($04DE40, 234 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DE40: movea.l $3c(a6), a1
pub fn i_04de40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE44;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04DE44: jsr $2942a.l
pub fn i_04de44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE4A);
    cpu.pc = 0x2942A;
    16
}
/// $04DE4A: lea.l $e92ea.l, a1
pub fn i_04de4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE92EA;
    20
}
/// $04DE50: move.w #$75, d0
pub fn i_04de50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE54;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DE54: move.b #$0, $75(a6)
pub fn i_04de54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE5A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DE5A: jsr $4429e.l
pub fn i_04de5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE60);
    cpu.pc = 0x4429E;
    16
}
/// $04DE60: move.w #$70, $70(a6)
pub fn i_04de60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE66;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DE66: move.w #$3c, $66(a6)
pub fn i_04de66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DE6C: move.b #$0, $20(a6)
pub fn i_04de6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE72;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DE72: move.b #$0, $21(a6)
pub fn i_04de72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DE78: lea.l $294472.l, a0
pub fn i_04de78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294472;
    20
}
/// $04DE7E: jsr $28cd4.l
pub fn i_04de7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE84);
    cpu.pc = 0x28CD4;
    16
}
/// $04DE84: lea.l $4de8a(pc), a1
pub fn i_04de84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DE8A;
    16
}
/// $04DE88: move.l a1, (a6)
pub fn i_04de88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DE8A: jsr $2783a.l
pub fn i_04de8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE90);
    cpu.pc = 0x2783A;
    16
}
/// $04DE90: jsr $28d70.l
pub fn i_04de90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE96);
    cpu.pc = 0x28D70;
    16
}
/// $04DE96: jsr $2870a.l
pub fn i_04de96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DE9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DE9C);
    cpu.pc = 0x2870A;
    16
}
/// $04DE9C: bcc.w $4deb2
pub fn i_04de9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DEB2; return 10; }
    8
}
/// $04DEA0: lea.l $5e766.l, a0
pub fn i_04dea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04DEA6: jsr $5e770.l
pub fn i_04dea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEAC);
    cpu.pc = 0x5E770;
    16
}
/// $04DEAC: bclr.b #$3, $13(a6)
pub fn i_04deac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEB2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DEB2: jsr $28758.l
pub fn i_04deb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEB8);
    cpu.pc = 0x28758;
    16
}
/// $04DEB8: bcc.w $4df0a
pub fn i_04deb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DF0A; return 10; }
    8
}
/// $04DEBC: move.b #$ff, $75(a6)
pub fn i_04debc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DEC2: addi.b #$10, $21(a6)
pub fn i_04dec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEC8;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DEC8: move.b #$ff, $20(a6)
pub fn i_04dec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DECE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DECE: addi.w #$10, $22(a6)
pub fn i_04dece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DED4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DED4: lea.l $295b5a.l, a1
pub fn i_04ded4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEDA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B5A;
    20
}
/// $04DEDA: jsr $77c7e.l
pub fn i_04deda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEE0);
    cpu.pc = 0x77C7E;
    16
}
/// $04DEE0: lea.l $295b6c.l, a1
pub fn i_04dee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEE6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B6C;
    20
}
/// $04DEE6: jsr $77c7e.l
pub fn i_04dee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEEC);
    cpu.pc = 0x77C7E;
    16
}
/// $04DEEC: lea.l $295cb0.l, a1
pub fn i_04deec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEF2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04DEF2: jsr $77c7e.l
pub fn i_04def2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DEF8);
    cpu.pc = 0x77C7E;
    16
}
/// $04DEF8: addi.w #$20, $22(a0)
pub fn i_04def8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DEFE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DEFE: addi.w #$20, $24(a0)
pub fn i_04defe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF04;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DF04: lea.l $4df30(pc), a1
pub fn i_04df04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DF30;
    16
}
/// $04DF08: move.l a1, (a6)
pub fn i_04df08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DF0A: jsr $4fa70(pc)
pub fn i_04df0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF0E);
    cpu.pc = 0x4FA70;
    16
}
/// $04DF0E: bcc.w $4df1e
pub fn i_04df0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DF1E; return 10; }
    8
}
/// $04DF12: move.b #$ff, $20(a6)
pub fn i_04df12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DF18: lea.l $4df72(pc), a1
pub fn i_04df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DF72;
    16
}
/// $04DF1C: move.l a1, (a6)
pub fn i_04df1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DF1E: move.b $74(a6), d0
pub fn i_04df1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF22;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04DF22: cmp.b $21(a6), d0
pub fn i_04df22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF26;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04DF26: beq.w $4df2e
pub fn i_04df26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4DF2E; return 10; }
    8
}
// ===== JsrPcThunk_04df2a  ($04DF2A, 6 B, thunks/jsr_pc_thunks.c)
/// $04DF2A: jsr $4faf8(pc)
pub fn i_04df2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF2E);
    cpu.pc = 0x4FAF8;
    16
}
/// $04DF2E: rts
pub fn i_04df2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BunkerWreck_04df30  ($04DF30, 98 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DF30: move.l #$1000, d0
pub fn i_04df30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF36;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04DF36: jsr $51a28.l
pub fn i_04df36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DF3C);
    cpu.pc = 0x51A28;
    16
}
/// $04DF3C: lea.l $ffff.w, a0
pub fn i_04df3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF40;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04DF40: move.l a0, $48(a6)
pub fn i_04df40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF44;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04DF44: move.w #$1030, d0
pub fn i_04df44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DF48: jsr $2352.l
pub fn i_04df48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DF4E);
    cpu.pc = 0x2352;
    16
}
/// $04DF4E: cmpi.w #$0, $22(a6)
pub fn i_04df4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF54;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04DF54: bmi.w $4df62
pub fn i_04df54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF58;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4DF62; return 10; }
    8
}
/// $04DF58: lea.l $29561e.l, a2
pub fn i_04df58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF5E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29561E;
    20
}
/// $04DF5E: bra.w $4df68
pub fn i_04df5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF62;
    cpu.ir = 0x6000;
    cpu.pc = 0x4DF68;
    10
}
/// $04DF62: lea.l $295632.l, a2
pub fn i_04df62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF68;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295632;
    20
}
/// $04DF68: jsr $5022a(pc)
pub fn i_04df68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF6C);
    cpu.pc = 0x5022A;
    16
}
/// $04DF6C: lea.l $4df72(pc), a1
pub fn i_04df6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DF72;
    16
}
/// $04DF70: move.l a1, (a6)
pub fn i_04df70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DF72: jsr $2783a.l
pub fn i_04df72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DF78);
    cpu.pc = 0x2783A;
    16
}
/// $04DF78: jsr $4fa70(pc)
pub fn i_04df78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF7C);
    cpu.pc = 0x4FA70;
    16
}
/// $04DF7C: bcc.w $4df86
pub fn i_04df7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4DF86; return 10; }
    8
}
/// $04DF80: lea.l $4f2a4(pc), a1
pub fn i_04df80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2A4;
    16
}
/// $04DF84: move.l a1, (a6)
pub fn i_04df84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DF86: move.b $74(a6), d0
pub fn i_04df86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF8A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04DF8A: cmp.b $21(a6), d0
pub fn i_04df8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF8E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04DF8E: beq.w $4df96
pub fn i_04df8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4DF96; return 10; }
    8
}
// ===== JsrPcThunk_04df92  ($04DF92, 6 B, thunks/jsr_pc_thunks.c)
/// $04DF92: jsr $4faf8(pc)
pub fn i_04df92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4DF96);
    cpu.pc = 0x4FAF8;
    16
}
/// $04DF96: rts
pub fn i_04df96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Bridge_04df98  ($04DF98, 198 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04DF98: movea.l $3c(a6), a1
pub fn i_04df98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DF9C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04DF9C: jsr $2942a.l
pub fn i_04df9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFA2);
    cpu.pc = 0x2942A;
    16
}
/// $04DFA2: lea.l $4e12c(pc), a1
pub fn i_04dfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E12C;
    16
}
/// $04DFA6: jsr $4ae.l
pub fn i_04dfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFAC);
    cpu.pc = 0x4AE;
    16
}
/// $04DFAC: jsr $5dd22.l
pub fn i_04dfac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFB2);
    cpu.pc = 0x5DD22;
    16
}
/// $04DFB2: lea.l $4e12c(pc), a1
pub fn i_04dfb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E12C;
    16
}
/// $04DFB6: jsr $4ae.l
pub fn i_04dfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFBC);
    cpu.pc = 0x4AE;
    16
}
/// $04DFBC: jsr $5dd22.l
pub fn i_04dfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFC2);
    cpu.pc = 0x5DD22;
    16
}
/// $04DFC2: addi.w #$d0, $22(a0)
pub fn i_04dfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFC8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xD0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04DFC8: ori.b #$1, $3a(a0)
pub fn i_04dfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFCE;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04DFCE: move.w #$60, $70(a6)
pub fn i_04dfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFD4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04DFD4: move.b #$0, $21(a6)
pub fn i_04dfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DFDA: lea.l $e9310.l, a1
pub fn i_04dfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFE0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xE9310;
    20
}
/// $04DFE0: move.w #$75, d0
pub fn i_04dfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04DFE4: move.b #$0, $75(a6)
pub fn i_04dfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04DFEA: jsr $4429e.l
pub fn i_04dfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFF0);
    cpu.pc = 0x4429E;
    16
}
/// $04DFF0: lea.l $4dff6(pc), a1
pub fn i_04dff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4DFF6;
    16
}
/// $04DFF4: move.l a1, (a6)
pub fn i_04dff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04DFF6: jsr $2783a.l
pub fn i_04dff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DFFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4DFFC);
    cpu.pc = 0x2783A;
    16
}
/// $04DFFC: cmpi.b #$11, $21(a6)
pub fn i_04dffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E002;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x11, Size::Byte, false);
    16
}
/// $04E002: bne.w $4e04e
pub fn i_04e002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E006;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4E04E; return 10; }
    8
}
/// $04E006: move.w #$1030, d0
pub fn i_04e006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E00A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E00A: jsr $2352.l
pub fn i_04e00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E010;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E010);
    cpu.pc = 0x2352;
    16
}
/// $04E010: lea.l $295bea.l, a1
pub fn i_04e010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E016;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BEA;
    20
}
/// $04E016: jsr $77c7e.l
pub fn i_04e016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E01C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E01C);
    cpu.pc = 0x77C7E;
    16
}
/// $04E01C: lea.l $295bfc.l, a1
pub fn i_04e01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E022;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295BFC;
    20
}
/// $04E022: jsr $77c7e.l
pub fn i_04e022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E028;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E028);
    cpu.pc = 0x77C7E;
    16
}
/// $04E028: lea.l $295cc2.l, a1
pub fn i_04e028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E02E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CC2;
    20
}
/// $04E02E: jsr $77c7e.l
pub fn i_04e02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E034);
    cpu.pc = 0x77C7E;
    16
}
/// $04E034: lea.l $2956aa.l, a2
pub fn i_04e034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E03A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956AA;
    20
}
/// $04E03A: jsr $5022a(pc)
pub fn i_04e03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E03E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E03E);
    cpu.pc = 0x5022A;
    16
}
/// $04E03E: lea.l $2956e6.l, a2
pub fn i_04e03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E044;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956E6;
    20
}
/// $04E044: jsr $5022a(pc)
pub fn i_04e044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E048;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E048);
    cpu.pc = 0x5022A;
    16
}
/// $04E048: lea.l $4e05e(pc), a1
pub fn i_04e048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E04C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E05E;
    16
}
/// $04E04C: move.l a1, (a6)
pub fn i_04e04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E04E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E04E: jsr $4fa70(pc)
pub fn i_04e04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E052;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E052);
    cpu.pc = 0x4FA70;
    16
}
/// $04E052: bcc.w $4e05c
pub fn i_04e052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E056;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E05C; return 10; }
    8
}
/// $04E056: jmp $518.l
pub fn i_04e056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E05C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E05C: rts
pub fn i_04e05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E05E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BridgeCollapse_04e05e  ($04E05E, 206 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E05E: move.w #$3c, $66(a6)
pub fn i_04e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E064;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E064: lea.l $294488.l, a0
pub fn i_04e064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E06A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294488;
    20
}
/// $04E06A: jsr $28cd4.l
pub fn i_04e06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E070;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E070);
    cpu.pc = 0x28CD4;
    16
}
/// $04E070: lea.l $4e076(pc), a1
pub fn i_04e070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E074;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E076;
    16
}
/// $04E074: move.l a1, (a6)
pub fn i_04e074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E076;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E076: jsr $2783a.l
pub fn i_04e076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E07C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E07C);
    cpu.pc = 0x2783A;
    16
}
/// $04E07C: jsr $28d70.l
pub fn i_04e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E082);
    cpu.pc = 0x28D70;
    16
}
/// $04E082: jsr $2870a.l
pub fn i_04e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E088);
    cpu.pc = 0x2870A;
    16
}
/// $04E088: bcc.w $4e09e
pub fn i_04e088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E08C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E09E; return 10; }
    8
}
/// $04E08C: lea.l $5e766.l, a0
pub fn i_04e08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E092;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E092: jsr $5e770.l
pub fn i_04e092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E098);
    cpu.pc = 0x5E770;
    16
}
/// $04E098: bclr.b #$3, $13(a6)
pub fn i_04e098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E09E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E09E: jsr $28758.l
pub fn i_04e09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0A4);
    cpu.pc = 0x28758;
    16
}
/// $04E0A4: bcc.w $4e116
pub fn i_04e0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E116; return 10; }
    8
}
/// $04E0A8: lea.l $ffff.w, a0
pub fn i_04e0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0AC;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04E0AC: move.l a0, $48(a6)
pub fn i_04e0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0B0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E0B0: move.l #$5000, d0
pub fn i_04e0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0B6;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04E0B6: jsr $51a28.l
pub fn i_04e0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0BC);
    cpu.pc = 0x51A28;
    16
}
/// $04E0BC: move.w #$1028, d0
pub fn i_04e0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E0C0: jsr $2352.l
pub fn i_04e0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0C6);
    cpu.pc = 0x2352;
    16
}
/// $04E0C6: lea.l $295c0e.l, a1
pub fn i_04e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C0E;
    20
}
/// $04E0CC: jsr $77c7e.l
pub fn i_04e0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0D2);
    cpu.pc = 0x77C7E;
    16
}
/// $04E0D2: lea.l $295c20.l, a1
pub fn i_04e0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C20;
    20
}
/// $04E0D8: jsr $77c7e.l
pub fn i_04e0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0DE);
    cpu.pc = 0x77C7E;
    16
}
/// $04E0DE: lea.l $295cc2.l, a1
pub fn i_04e0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CC2;
    20
}
/// $04E0E4: jsr $77c7e.l
pub fn i_04e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E0EA);
    cpu.pc = 0x77C7E;
    16
}
/// $04E0EA: lea.l $2956be.l, a2
pub fn i_04e0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0F0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956BE;
    20
}
/// $04E0F0: jsr $5022a(pc)
pub fn i_04e0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E0F4);
    cpu.pc = 0x5022A;
    16
}
/// $04E0F4: lea.l $2956fa.l, a2
pub fn i_04e0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0FA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2956FA;
    20
}
/// $04E0FA: jsr $5022a(pc)
pub fn i_04e0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E0FE);
    cpu.pc = 0x5022A;
    16
}
/// $04E0FE: lea.l $295722.l, a2
pub fn i_04e0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E104;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295722;
    20
}
/// $04E104: jsr $5022a(pc)
pub fn i_04e104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E108;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E108);
    cpu.pc = 0x5022A;
    16
}
/// $04E108: lea.l $29574a.l, a2
pub fn i_04e108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E10E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29574A;
    20
}
/// $04E10E: jsr $5022a(pc)
pub fn i_04e10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E112;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E112);
    cpu.pc = 0x5022A;
    16
}
/// $04E112: bra.w $4e11e
pub fn i_04e112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E116;
    cpu.ir = 0x6000;
    cpu.pc = 0x4E11E;
    10
}
/// $04E116: jsr $4fa70(pc)
pub fn i_04e116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E11A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E11A);
    cpu.pc = 0x4FA70;
    16
}
/// $04E11A: bcc.w $4e12a
pub fn i_04e11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E11E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E12A; return 10; }
    8
}
/// $04E11E: move.b #$ff, $75(a6)
pub fn i_04e11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E124;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E124: jmp $518.l
pub fn i_04e124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E12A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E12A: rts
pub fn i_04e12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E12C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BridgePillar_04e12c  ($04E12C, 284 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E12C: lea.l $5f38a.l, a1
pub fn i_04e12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E132;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5F38A;
    20
}
/// $04E132: jsr $4ae.l
pub fn i_04e132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E138;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E138);
    cpu.pc = 0x4AE;
    16
}
/// $04E138: jsr $5dd22.l
pub fn i_04e138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E13E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E13E);
    cpu.pc = 0x5DD22;
    16
}
/// $04E13E: btst.b #$0, $3a(a6)
pub fn i_04e13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E144;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04E144: bne.w $4e158
pub fn i_04e144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E148;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4E158; return 10; }
    8
}
/// $04E148: addi.w #$2b, $22(a0)
pub fn i_04e148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E14E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2B, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E14E: subi.w #$17, $24(a0)
pub fn i_04e14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E154;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x17, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E154: bra.w $4e164
pub fn i_04e154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E158;
    cpu.ir = 0x6000;
    cpu.pc = 0x4E164;
    10
}
/// $04E158: subi.w #$14, $22(a0)
pub fn i_04e158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E15E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E15E: subi.w #$17, $24(a0)
pub fn i_04e15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E164;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x17, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E164: move.b #$0, $21(a6)
pub fn i_04e164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E16A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E16A: move.w #$3c, $66(a6)
pub fn i_04e16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E170;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E170: move.w #$70, $70(a6)
pub fn i_04e170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E176;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E176: lea.l $29449e.l, a0
pub fn i_04e176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E17C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29449E;
    20
}
/// $04E17C: jsr $28cd4.l
pub fn i_04e17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E182);
    cpu.pc = 0x28CD4;
    16
}
/// $04E182: lea.l $4e188(pc), a1
pub fn i_04e182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E186;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E188;
    16
}
/// $04E186: move.l a1, (a6)
pub fn i_04e186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E188;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E188: jsr $2783a.l
pub fn i_04e188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E18E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E18E);
    cpu.pc = 0x2783A;
    16
}
/// $04E18E: jsr $28d70.l
pub fn i_04e18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E194);
    cpu.pc = 0x28D70;
    16
}
/// $04E194: jsr $2870a.l
pub fn i_04e194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E19A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E19A);
    cpu.pc = 0x2870A;
    16
}
/// $04E19A: bcc.w $4e1b0
pub fn i_04e19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E19E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E1B0; return 10; }
    8
}
/// $04E19E: lea.l $5e766.l, a0
pub fn i_04e19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E1A4: jsr $5e770.l
pub fn i_04e1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1AA);
    cpu.pc = 0x5E770;
    16
}
/// $04E1AA: bclr.b #$3, $13(a6)
pub fn i_04e1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1B0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E1B0: jsr $28758.l
pub fn i_04e1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1B6);
    cpu.pc = 0x28758;
    16
}
/// $04E1B6: bcc.w $4e238
pub fn i_04e1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E238; return 10; }
    8
}
/// $04E1BA: move.w #$102f, d0
pub fn i_04e1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1BE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E1BE: jsr $2352.l
pub fn i_04e1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1C4);
    cpu.pc = 0x2352;
    16
}
/// $04E1C4: lea.l $ffff.w, a0
pub fn i_04e1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1C8;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04E1C8: move.l a0, $48(a6)
pub fn i_04e1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1CC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E1CC: lea.l $295b7e.l, a1
pub fn i_04e1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B7E;
    20
}
/// $04E1D2: jsr $77c7e.l
pub fn i_04e1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1D8);
    cpu.pc = 0x77C7E;
    16
}
/// $04E1D8: subi.w #$20, $24(a0)
pub fn i_04e1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1DE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E1DE: lea.l $295b90.l, a1
pub fn i_04e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295B90;
    20
}
/// $04E1E4: jsr $77c7e.l
pub fn i_04e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1EA);
    cpu.pc = 0x77C7E;
    16
}
/// $04E1EA: subi.w #$20, $24(a0)
pub fn i_04e1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1F0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E1F0: lea.l $295cb0.l, a1
pub fn i_04e1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CB0;
    20
}
/// $04E1F6: jsr $77c7e.l
pub fn i_04e1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E1FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E1FC);
    cpu.pc = 0x77C7E;
    16
}
/// $04E1FC: move.w #$20, d0
pub fn i_04e1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E200;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E200: btst.b #$0, $3a(a6)
pub fn i_04e200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E206;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04E206: beq.w $4e20c
pub fn i_04e206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E20A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4E20C; return 10; }
    8
}
/// $04E20A: neg.w d0
pub fn i_04e20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E20C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04E20C: add.w d0, $22(a0)
pub fn i_04e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E210;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04E210: jsr $4fb3c(pc)
pub fn i_04e210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E214;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E214);
    cpu.pc = 0x4FB3C;
    16
}
/// $04E214: movea.l $c(a6), a0
pub fn i_04e214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E218;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04E218: move.b #$10, d0
pub fn i_04e218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E21C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04E21C: btst.b #$0, $3a(a6)
pub fn i_04e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E222;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04E222: beq.w $4e22a
pub fn i_04e222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E226;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4E22A; return 10; }
    8
}
/// $04E226: move.b #$1, d0
pub fn i_04e226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E22A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04E22A: or.b d0, $21(a0)
pub fn i_04e22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E22E;
    cpu.ir = 0x8128;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04E22E: move.b #$ff, $21(a6)
pub fn i_04e22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E234;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04E234: bra.w $4e240
pub fn i_04e234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E238;
    cpu.ir = 0x6000;
    cpu.pc = 0x4E240;
    10
}
/// $04E238: jsr $4fa70(pc)
pub fn i_04e238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E23C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E23C);
    cpu.pc = 0x4FA70;
    16
}
/// $04E23C: bcc.w $4e246
pub fn i_04e23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E240;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E246; return 10; }
    8
}
/// $04E240: jmp $518.l
pub fn i_04e240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E246;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E246: rts
pub fn i_04e246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E248;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Nest_04e248  ($04E248, 96 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E248: movea.l $3c(a6), a1
pub fn i_04e248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E24C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04E24C: jsr $2942a.l
pub fn i_04e24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E252);
    cpu.pc = 0x2942A;
    16
}
/// $04E252: move.w #$40, d1
pub fn i_04e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E256;
    cpu.ir = 0x323C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E256: jsr $236e.l
pub fn i_04e256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E25C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E25C);
    cpu.pc = 0x236E;
    16
}
/// $04E25C: move.w #$70, $70(a6)
pub fn i_04e25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E262;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E262: move.w #$12c, $66(a6)
pub fn i_04e262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E268;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04E268: lea.l $29441a.l, a0
pub fn i_04e268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E26E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29441A;
    20
}
/// $04E26E: jsr $28cd4.l
pub fn i_04e26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E274);
    cpu.pc = 0x28CD4;
    16
}
/// $04E274: lea.l $8e738.l, a1
pub fn i_04e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E27A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x8E738;
    20
}
/// $04E27A: jsr $4ae.l
pub fn i_04e27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E280);
    cpu.pc = 0x4AE;
    16
}
/// $04E280: jsr $5dd22.l
pub fn i_04e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E286;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E286);
    cpu.pc = 0x5DD22;
    16
}
/// $04E286: addi.w #$98, $22(a0)
pub fn i_04e286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E28C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x98, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E28C: addi.w #$38, $24(a0)
pub fn i_04e28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E292;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04E292: lea.l $4e298(pc), a1
pub fn i_04e292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E296;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E298;
    16
}
/// $04E296: move.l a1, (a6)
pub fn i_04e296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E298;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E298: cmpi.w #$110, $22(a6)
pub fn i_04e298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E29E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $04E29E: blt.w $4e2b0
pub fn i_04e29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2A2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4E2B0; return 10; }
    8
}
/// $04E2A2: jsr $2783a.l
pub fn i_04e2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2A8);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_04e2a8  ($04E2A8, 8 B, thunks/jsr_abs_thunks.c)
/// $04E2A8: jsr $28d70.l
pub fn i_04e2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2AE);
    cpu.pc = 0x28D70;
    16
}
/// $04E2AE: rts
pub fn i_04e2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NestActive_04e2b0  ($04E2B0, 124 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E2B0: lea.l $4e2b6(pc), a1
pub fn i_04e2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E2B6;
    16
}
/// $04E2B4: move.l a1, (a6)
pub fn i_04e2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E2B6: jsr $2783a.l
pub fn i_04e2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2BC);
    cpu.pc = 0x2783A;
    16
}
/// $04E2BC: jsr $28d70.l
pub fn i_04e2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2C2);
    cpu.pc = 0x28D70;
    16
}
/// $04E2C2: jsr $2870a.l
pub fn i_04e2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2C8);
    cpu.pc = 0x2870A;
    16
}
/// $04E2C8: bcc.w $4e2de
pub fn i_04e2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E2DE; return 10; }
    8
}
/// $04E2CC: lea.l $5e766.l, a0
pub fn i_04e2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04E2D2: jsr $5e770.l
pub fn i_04e2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2D8);
    cpu.pc = 0x5E770;
    16
}
/// $04E2D8: bclr.b #$3, $13(a6)
pub fn i_04e2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2DE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04E2DE: jsr $28758.l
pub fn i_04e2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2E4);
    cpu.pc = 0x28758;
    16
}
/// $04E2E4: bcc.w $4e31c
pub fn i_04e2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E31C; return 10; }
    8
}
/// $04E2E8: move.w #$102f, d0
pub fn i_04e2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E2EC: jsr $2352.l
pub fn i_04e2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2F2);
    cpu.pc = 0x2352;
    16
}
/// $04E2F2: lea.l $295c56.l, a1
pub fn i_04e2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2F8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C56;
    20
}
/// $04E2F8: jsr $77c7e.l
pub fn i_04e2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E2FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E2FE);
    cpu.pc = 0x77C7E;
    16
}
/// $04E2FE: lea.l $295c68.l, a1
pub fn i_04e2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E304;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295C68;
    20
}
/// $04E304: jsr $77c7e.l
pub fn i_04e304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E30A);
    cpu.pc = 0x77C7E;
    16
}
/// $04E30A: lea.l $295ce6.l, a1
pub fn i_04e30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E310;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295CE6;
    20
}
/// $04E310: jsr $77c7e.l
pub fn i_04e310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E316);
    cpu.pc = 0x77C7E;
    16
}
/// $04E316: lea.l $4e32c(pc), a1
pub fn i_04e316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E31A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E32C;
    16
}
/// $04E31A: move.l a1, (a6)
pub fn i_04e31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E31C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E31C: jsr $4fa70(pc)
pub fn i_04e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E320;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E320);
    cpu.pc = 0x4FA70;
    16
}
/// $04E320: bcc.w $4e32a
pub fn i_04e320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E324;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E32A; return 10; }
    8
}
/// $04E324: jmp $518.l
pub fn i_04e324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E32A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E32A: rts
pub fn i_04e32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E32C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NestWreck_04e32c  ($04E32C, 94 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E32C: move.l #$3000, d0
pub fn i_04e32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E332;
    cpu.ir = 0x203C;
    let v1: u32 = 0x3000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04E332: jsr $51a28.l
pub fn i_04e332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E338);
    cpu.pc = 0x51A28;
    16
}
/// $04E338: lea.l $ffff.w, a0
pub fn i_04e338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E33C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04E33C: move.l a0, $48(a6)
pub fn i_04e33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E340;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04E340: jsr $2783a.l
pub fn i_04e340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E346);
    cpu.pc = 0x2783A;
    16
}
/// $04E346: lea.l $295d78.l, a1
pub fn i_04e346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E34C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295D78;
    20
}
/// $04E34C: jsr $43fac.l
pub fn i_04e34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E352);
    cpu.pc = 0x43FAC;
    16
}
/// $04E352: move.w #$1030, d0
pub fn i_04e352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E356;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04E356: jsr $2352.l
pub fn i_04e356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E35C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E35C);
    cpu.pc = 0x2352;
    16
}
/// $04E35C: lea.l $294430.l, a0
pub fn i_04e35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E362;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294430;
    20
}
/// $04E362: jsr $28cd4.l
pub fn i_04e362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E368);
    cpu.pc = 0x28CD4;
    16
}
/// $04E368: lea.l $4e36e(pc), a1
pub fn i_04e368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E36C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4E36E;
    16
}
/// $04E36C: move.l a1, (a6)
pub fn i_04e36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E36E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04E36E: jsr $2783a.l
pub fn i_04e36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E374);
    cpu.pc = 0x2783A;
    16
}
/// $04E374: jsr $28d70.l
pub fn i_04e374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E37A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E37A);
    cpu.pc = 0x28D70;
    16
}
/// $04E37A: jsr $4fa70(pc)
pub fn i_04e37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E37E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4E37E);
    cpu.pc = 0x4FA70;
    16
}
/// $04E37E: bcc.w $4e388
pub fn i_04e37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E382;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4E388; return 10; }
    8
}
/// $04E382: jmp $518.l
pub fn i_04e382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E388;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04E388: rts
pub fn i_04e388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E38A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Shed_04e38a  ($04E38A, 198 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04E38A: movea.l $3c(a6), a1
pub fn i_04e38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E38E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04E38E: jsr $2942a.l
pub fn i_04e38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E394);
    cpu.pc = 0x2942A;
    16
}
/// $04E394: lea.l $4f2c2(pc), a1
pub fn i_04e394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E398;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F2C2;
    16
}
/// $04E398: jsr $4ae.l
pub fn i_04e398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E39E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E39E);
    cpu.pc = 0x4AE;
    16
}
/// $04E39E: jsr $5dd22.l
pub fn i_04e39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4E3A4);
    cpu.pc = 0x5DD22;
    16
}
/// $04E3A4: addi.w #$40, $22(a0)
pub fn i_04e3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E3AA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x04C880, i_04c880),
    (0x04C886, i_04c886),
    (0x04C88C, i_04c88c),
    (0x04C890, i_04c890),
    (0x04C892, i_04c892),
    (0x04C896, i_04c896),
    (0x04C89A, i_04c89a),
    (0x04C8A0, i_04c8a0),
    (0x04C8A6, i_04c8a6),
    (0x04C8AA, i_04c8aa),
    (0x04C8B0, i_04c8b0),
    (0x04C8B6, i_04c8b6),
    (0x04C8BC, i_04c8bc),
    (0x04C8C0, i_04c8c0),
    (0x04C8C2, i_04c8c2),
    (0x04C8C8, i_04c8c8),
    (0x04C8CC, i_04c8cc),
    (0x04C8D2, i_04c8d2),
    (0x04C8D6, i_04c8d6),
    (0x04C8DA, i_04c8da),
    (0x04C8DC, i_04c8dc),
    (0x04C8E2, i_04c8e2),
    (0x04C8E8, i_04c8e8),
    (0x04C8EE, i_04c8ee),
    (0x04C8F2, i_04c8f2),
    (0x04C8F6, i_04c8f6),
    (0x04C8F8, i_04c8f8),
    (0x04C8FE, i_04c8fe),
    (0x04C904, i_04c904),
    (0x04C908, i_04c908),
    (0x04C90C, i_04c90c),
    (0x04C90E, i_04c90e),
    (0x04C910, i_04c910),
    (0x04C914, i_04c914),
    (0x04C91A, i_04c91a),
    (0x04C920, i_04c920),
    (0x04C926, i_04c926),
    (0x04C92E, i_04c92e),
    (0x04C934, i_04c934),
    (0x04C93A, i_04c93a),
    (0x04C940, i_04c940),
    (0x04C942, i_04c942),
    (0x04C946, i_04c946),
    (0x04C94C, i_04c94c),
    (0x04C950, i_04c950),
    (0x04C954, i_04c954),
    (0x04C956, i_04c956),
    (0x04C958, i_04c958),
    (0x04C95E, i_04c95e),
    (0x04C962, i_04c962),
    (0x04C968, i_04c968),
    (0x04C96E, i_04c96e),
    (0x04C974, i_04c974),
    (0x04C97A, i_04c97a),
    (0x04C97E, i_04c97e),
    (0x04C982, i_04c982),
    (0x04C988, i_04c988),
    (0x04C98A, i_04c98a),
    (0x04C98E, i_04c98e),
    (0x04C992, i_04c992),
    (0x04C994, i_04c994),
    (0x04C996, i_04c996),
    (0x04C99A, i_04c99a),
    (0x04C99E, i_04c99e),
    (0x04C9A0, i_04c9a0),
    (0x04C9A2, i_04c9a2),
    (0x04C9A6, i_04c9a6),
    (0x04C9AC, i_04c9ac),
    (0x04C9AE, i_04c9ae),
    (0x04C9B2, i_04c9b2),
    (0x04C9B6, i_04c9b6),
    (0x04C9BA, i_04c9ba),
    (0x04C9BC, i_04c9bc),
    (0x04C9BE, i_04c9be),
    (0x04C9C2, i_04c9c2),
    (0x04C9C6, i_04c9c6),
    (0x04C9C8, i_04c9c8),
    (0x04C9CA, i_04c9ca),
    (0x04C9CE, i_04c9ce),
    (0x04C9D4, i_04c9d4),
    (0x04C9D6, i_04c9d6),
    (0x04C9DA, i_04c9da),
    (0x04C9DE, i_04c9de),
    (0x04C9E2, i_04c9e2),
    (0x04C9E6, i_04c9e6),
    (0x04C9E8, i_04c9e8),
    (0x04C9EC, i_04c9ec),
    (0x04C9EE, i_04c9ee),
    (0x04C9F2, i_04c9f2),
    (0x04C9F6, i_04c9f6),
    (0x04C9FA, i_04c9fa),
    (0x04C9FE, i_04c9fe),
    (0x04CA02, i_04ca02),
    (0x04CA06, i_04ca06),
    (0x04CA08, i_04ca08),
    (0x04CA0C, i_04ca0c),
    (0x04CA0E, i_04ca0e),
    (0x04CA14, i_04ca14),
    (0x04CA1A, i_04ca1a),
    (0x04CA1E, i_04ca1e),
    (0x04CA22, i_04ca22),
    (0x04CA24, i_04ca24),
    (0x04CA2A, i_04ca2a),
    (0x04CA30, i_04ca30),
    (0x04CA34, i_04ca34),
    (0x04CA3A, i_04ca3a),
    (0x04CA40, i_04ca40),
    (0x04CA44, i_04ca44),
    (0x04CA48, i_04ca48),
    (0x04CA4A, i_04ca4a),
    (0x04CA50, i_04ca50),
    (0x04CA56, i_04ca56),
    (0x04CA58, i_04ca58),
    (0x04CA5C, i_04ca5c),
    (0x04CA62, i_04ca62),
    (0x04CA68, i_04ca68),
    (0x04CA6E, i_04ca6e),
    (0x04CA74, i_04ca74),
    (0x04CA7A, i_04ca7a),
    (0x04CA7C, i_04ca7c),
    (0x04CA80, i_04ca80),
    (0x04CA86, i_04ca86),
    (0x04CA8C, i_04ca8c),
    (0x04CA92, i_04ca92),
    (0x04CA96, i_04ca96),
    (0x04CA98, i_04ca98),
    (0x04CA9C, i_04ca9c),
    (0x04CAA0, i_04caa0),
    (0x04CAA6, i_04caa6),
    (0x04CAA8, i_04caa8),
    (0x04CAAC, i_04caac),
    (0x04CAB0, i_04cab0),
    (0x04CAB4, i_04cab4),
    (0x04CAB8, i_04cab8),
    (0x04CABC, i_04cabc),
    (0x04CABE, i_04cabe),
    (0x04CAC2, i_04cac2),
    (0x04CAC4, i_04cac4),
    (0x04CACA, i_04caca),
    (0x04CACE, i_04cace),
    (0x04CAD2, i_04cad2),
    (0x04CAD6, i_04cad6),
    (0x04CADA, i_04cada),
    (0x04CADE, i_04cade),
    (0x04CAE2, i_04cae2),
    (0x04CAE4, i_04cae4),
    (0x04CAE6, i_04cae6),
    (0x04CAE8, i_04cae8),
    (0x04CAEC, i_04caec),
    (0x04CAF2, i_04caf2),
    (0x04CAF6, i_04caf6),
    (0x04CAFA, i_04cafa),
    (0x04CAFE, i_04cafe),
    (0x04CB04, i_04cb04),
    (0x04CB08, i_04cb08),
    (0x04CB0C, i_04cb0c),
    (0x04CB10, i_04cb10),
    (0x04CB16, i_04cb16),
    (0x04CB1A, i_04cb1a),
    (0x04CB1E, i_04cb1e),
    (0x04CB22, i_04cb22),
    (0x04CB28, i_04cb28),
    (0x04CB2A, i_04cb2a),
    (0x04CB2E, i_04cb2e),
    (0x04CB32, i_04cb32),
    (0x04CB36, i_04cb36),
    (0x04CB3A, i_04cb3a),
    (0x04CB3E, i_04cb3e),
    (0x04CB42, i_04cb42),
    (0x04CB5C, i_04cb5c),
    (0x04CB66, i_04cb66),
    (0x04CB68, i_04cb68),
    (0x04CB6E, i_04cb6e),
    (0x04CB72, i_04cb72),
    (0x04CB76, i_04cb76),
    (0x04CB7A, i_04cb7a),
    (0x04CB7C, i_04cb7c),
    (0x04CB7E, i_04cb7e),
    (0x04CB86, i_04cb86),
    (0x04CB88, i_04cb88),
    (0x04CB8C, i_04cb8c),
    (0x04CB94, i_04cb94),
    (0x04CB98, i_04cb98),
    (0x04CB9A, i_04cb9a),
    (0x04CB9C, i_04cb9c),
    (0x04CBA0, i_04cba0),
    (0x04CBA4, i_04cba4),
    (0x04CBA6, i_04cba6),
    (0x04CBA8, i_04cba8),
    (0x04CBAA, i_04cbaa),
    (0x04CBB0, i_04cbb0),
    (0x04CBB6, i_04cbb6),
    (0x04CBB8, i_04cbb8),
    (0x04CBBC, i_04cbbc),
    (0x04CBC0, i_04cbc0),
    (0x04CBC4, i_04cbc4),
    (0x04CBC8, i_04cbc8),
    (0x04CBCC, i_04cbcc),
    (0x04CBCE, i_04cbce),
    (0x04CBD2, i_04cbd2),
    (0x04CBD4, i_04cbd4),
    (0x04CBDA, i_04cbda),
    (0x04CBE0, i_04cbe0),
    (0x04CBE6, i_04cbe6),
    (0x04CBEA, i_04cbea),
    (0x04CBF0, i_04cbf0),
    (0x04CBF6, i_04cbf6),
    (0x04CBFA, i_04cbfa),
    (0x04CBFC, i_04cbfc),
    (0x04CC00, i_04cc00),
    (0x04CC06, i_04cc06),
    (0x04CC0C, i_04cc0c),
    (0x04CC12, i_04cc12),
    (0x04CC18, i_04cc18),
    (0x04CC1E, i_04cc1e),
    (0x04CC24, i_04cc24),
    (0x04CC2A, i_04cc2a),
    (0x04CC30, i_04cc30),
    (0x04CC36, i_04cc36),
    (0x04CC3A, i_04cc3a),
    (0x04CC40, i_04cc40),
    (0x04CC46, i_04cc46),
    (0x04CC4A, i_04cc4a),
    (0x04CC4E, i_04cc4e),
    (0x04CC52, i_04cc52),
    (0x04CC56, i_04cc56),
    (0x04CC5A, i_04cc5a),
    (0x04CC5E, i_04cc5e),
    (0x04CC64, i_04cc64),
    (0x04CC6A, i_04cc6a),
    (0x04CC70, i_04cc70),
    (0x04CC76, i_04cc76),
    (0x04CC7C, i_04cc7c),
    (0x04CC80, i_04cc80),
    (0x04CC82, i_04cc82),
    (0x04CC86, i_04cc86),
    (0x04CC8A, i_04cc8a),
    (0x04CC8E, i_04cc8e),
    (0x04CC94, i_04cc94),
    (0x04CC98, i_04cc98),
    (0x04CC9C, i_04cc9c),
    (0x04CCA0, i_04cca0),
    (0x04CCA4, i_04cca4),
    (0x04CCA8, i_04cca8),
    (0x04CCAC, i_04ccac),
    (0x04CCB0, i_04ccb0),
    (0x04CCB4, i_04ccb4),
    (0x04CCBA, i_04ccba),
    (0x04CCC0, i_04ccc0),
    (0x04CCC6, i_04ccc6),
    (0x04CCCA, i_04ccca),
    (0x04CCD0, i_04ccd0),
    (0x04CCD4, i_04ccd4),
    (0x04CCD8, i_04ccd8),
    (0x04CCDC, i_04ccdc),
    (0x04CCE2, i_04cce2),
    (0x04CCE8, i_04cce8),
    (0x04CCEA, i_04ccea),
    (0x04CCF0, i_04ccf0),
    (0x04CCF6, i_04ccf6),
    (0x04CCFC, i_04ccfc),
    (0x04CD00, i_04cd00),
    (0x04CD04, i_04cd04),
    (0x04CD08, i_04cd08),
    (0x04CD0C, i_04cd0c),
    (0x04CD10, i_04cd10),
    (0x04CD16, i_04cd16),
    (0x04CD1A, i_04cd1a),
    (0x04CD1E, i_04cd1e),
    (0x04CD20, i_04cd20),
    (0x04CD24, i_04cd24),
    (0x04CD2A, i_04cd2a),
    (0x04CD2E, i_04cd2e),
    (0x04CD34, i_04cd34),
    (0x04CD3A, i_04cd3a),
    (0x04CD3E, i_04cd3e),
    (0x04CD40, i_04cd40),
    (0x04CD44, i_04cd44),
    (0x04CD48, i_04cd48),
    (0x04CD4C, i_04cd4c),
    (0x04CD52, i_04cd52),
    (0x04CD58, i_04cd58),
    (0x04CD5E, i_04cd5e),
    (0x04CD62, i_04cd62),
    (0x04CD66, i_04cd66),
    (0x04CD6A, i_04cd6a),
    (0x04CD6E, i_04cd6e),
    (0x04CD74, i_04cd74),
    (0x04CD78, i_04cd78),
    (0x04CD7E, i_04cd7e),
    (0x04CD80, i_04cd80),
    (0x04CD86, i_04cd86),
    (0x04CD8C, i_04cd8c),
    (0x04CD90, i_04cd90),
    (0x04CD94, i_04cd94),
    (0x04CD96, i_04cd96),
    (0x04CD9A, i_04cd9a),
    (0x04CD9E, i_04cd9e),
    (0x04CDA2, i_04cda2),
    (0x04CDA8, i_04cda8),
    (0x04CDAC, i_04cdac),
    (0x04CDB0, i_04cdb0),
    (0x04CDB4, i_04cdb4),
    (0x04CDB8, i_04cdb8),
    (0x04CDBC, i_04cdbc),
    (0x04CDC0, i_04cdc0),
    (0x04CDC4, i_04cdc4),
    (0x04CDC8, i_04cdc8),
    (0x04CDCC, i_04cdcc),
    (0x04CDD2, i_04cdd2),
    (0x04CDD8, i_04cdd8),
    (0x04CDDE, i_04cdde),
    (0x04CDE2, i_04cde2),
    (0x04CDE8, i_04cde8),
    (0x04CDEC, i_04cdec),
    (0x04CDF0, i_04cdf0),
    (0x04CDF4, i_04cdf4),
    (0x04CDFA, i_04cdfa),
    (0x04CE00, i_04ce00),
    (0x04CE02, i_04ce02),
    (0x04CE08, i_04ce08),
    (0x04CE0E, i_04ce0e),
    (0x04CE14, i_04ce14),
    (0x04CE18, i_04ce18),
    (0x04CE1C, i_04ce1c),
    (0x04CE22, i_04ce22),
    (0x04CE26, i_04ce26),
    (0x04CE2A, i_04ce2a),
    (0x04CE2E, i_04ce2e),
    (0x04CE34, i_04ce34),
    (0x04CE3A, i_04ce3a),
    (0x04CE3E, i_04ce3e),
    (0x04CE40, i_04ce40),
    (0x04CE44, i_04ce44),
    (0x04CE48, i_04ce48),
    (0x04CE4C, i_04ce4c),
    (0x04CE50, i_04ce50),
    (0x04CE56, i_04ce56),
    (0x04CE5A, i_04ce5a),
    (0x04CE60, i_04ce60),
    (0x04CE62, i_04ce62),
    (0x04CE68, i_04ce68),
    (0x04CE6C, i_04ce6c),
    (0x04CE6E, i_04ce6e),
    (0x04CE72, i_04ce72),
    (0x04CE76, i_04ce76),
    (0x04CE7C, i_04ce7c),
    (0x04CE82, i_04ce82),
    (0x04CE88, i_04ce88),
    (0x04CE8C, i_04ce8c),
    (0x04CE8E, i_04ce8e),
    (0x04CE92, i_04ce92),
    (0x04CE96, i_04ce96),
    (0x04CE9A, i_04ce9a),
    (0x04CE9E, i_04ce9e),
    (0x04CEA2, i_04cea2),
    (0x04CEA8, i_04cea8),
    (0x04CEAC, i_04ceac),
    (0x04CEB0, i_04ceb0),
    (0x04CEB6, i_04ceb6),
    (0x04CEBA, i_04ceba),
    (0x04CEBC, i_04cebc),
    (0x04CEC0, i_04cec0),
    (0x04CEC4, i_04cec4),
    (0x04CEC8, i_04cec8),
    (0x04CECE, i_04cece),
    (0x04CED2, i_04ced2),
    (0x04CED8, i_04ced8),
    (0x04CEDC, i_04cedc),
    (0x04CEE0, i_04cee0),
    (0x04CEE4, i_04cee4),
    (0x04CEEA, i_04ceea),
    (0x04CEF0, i_04cef0),
    (0x04CEF2, i_04cef2),
    (0x04CEF8, i_04cef8),
    (0x04CEFE, i_04cefe),
    (0x04CF04, i_04cf04),
    (0x04CF08, i_04cf08),
    (0x04CF0C, i_04cf0c),
    (0x04CF10, i_04cf10),
    (0x04CF14, i_04cf14),
    (0x04CF18, i_04cf18),
    (0x04CF1C, i_04cf1c),
    (0x04CF22, i_04cf22),
    (0x04CF26, i_04cf26),
    (0x04CF2C, i_04cf2c),
    (0x04CF2E, i_04cf2e),
    (0x04CF32, i_04cf32),
    (0x04CF38, i_04cf38),
    (0x04CF3E, i_04cf3e),
    (0x04CF44, i_04cf44),
    (0x04CF4A, i_04cf4a),
    (0x04CF50, i_04cf50),
    (0x04CF56, i_04cf56),
    (0x04CF5A, i_04cf5a),
    (0x04CF60, i_04cf60),
    (0x04CF64, i_04cf64),
    (0x04CF6A, i_04cf6a),
    (0x04CF70, i_04cf70),
    (0x04CF74, i_04cf74),
    (0x04CF76, i_04cf76),
    (0x04CF7A, i_04cf7a),
    (0x04CF80, i_04cf80),
    (0x04CF86, i_04cf86),
    (0x04CF8C, i_04cf8c),
    (0x04CF90, i_04cf90),
    (0x04CF94, i_04cf94),
    (0x04CF9A, i_04cf9a),
    (0x04CF9E, i_04cf9e),
    (0x04CFA4, i_04cfa4),
    (0x04CFAA, i_04cfaa),
    (0x04CFB0, i_04cfb0),
    (0x04CFB4, i_04cfb4),
    (0x04CFBA, i_04cfba),
    (0x04CFBE, i_04cfbe),
    (0x04CFC2, i_04cfc2),
    (0x04CFC6, i_04cfc6),
    (0x04CFCA, i_04cfca),
    (0x04CFD0, i_04cfd0),
    (0x04CFD4, i_04cfd4),
    (0x04CFDA, i_04cfda),
    (0x04CFDE, i_04cfde),
    (0x04CFE4, i_04cfe4),
    (0x04CFEA, i_04cfea),
    (0x04CFF0, i_04cff0),
    (0x04CFF6, i_04cff6),
    (0x04CFFA, i_04cffa),
    (0x04CFFE, i_04cffe),
    (0x04D004, i_04d004),
    (0x04D008, i_04d008),
    (0x04D00E, i_04d00e),
    (0x04D014, i_04d014),
    (0x04D01A, i_04d01a),
    (0x04D020, i_04d020),
    (0x04D026, i_04d026),
    (0x04D02A, i_04d02a),
    (0x04D030, i_04d030),
    (0x04D034, i_04d034),
    (0x04D03A, i_04d03a),
    (0x04D040, i_04d040),
    (0x04D046, i_04d046),
    (0x04D048, i_04d048),
    (0x04D04C, i_04d04c),
    (0x04D052, i_04d052),
    (0x04D056, i_04d056),
    (0x04D05A, i_04d05a),
    (0x04D05C, i_04d05c),
    (0x04D060, i_04d060),
    (0x04D064, i_04d064),
    (0x04D06A, i_04d06a),
    (0x04D070, i_04d070),
    (0x04D076, i_04d076),
    (0x04D07C, i_04d07c),
    (0x04D080, i_04d080),
    (0x04D086, i_04d086),
    (0x04D08A, i_04d08a),
    (0x04D090, i_04d090),
    (0x04D096, i_04d096),
    (0x04D09C, i_04d09c),
    (0x04D0A2, i_04d0a2),
    (0x04D0A6, i_04d0a6),
    (0x04D0A8, i_04d0a8),
    (0x04D0AE, i_04d0ae),
    (0x04D0B2, i_04d0b2),
    (0x04D0B8, i_04d0b8),
    (0x04D0BC, i_04d0bc),
    (0x04D0C0, i_04d0c0),
    (0x04D0C6, i_04d0c6),
    (0x04D0CA, i_04d0ca),
    (0x04D0CC, i_04d0cc),
    (0x04D0D0, i_04d0d0),
    (0x04D0D6, i_04d0d6),
    (0x04D0DA, i_04d0da),
    (0x04D0E0, i_04d0e0),
    (0x04D0E2, i_04d0e2),
    (0x04D0E8, i_04d0e8),
    (0x04D0EE, i_04d0ee),
    (0x04D0F4, i_04d0f4),
    (0x04D0FA, i_04d0fa),
    (0x04D0FE, i_04d0fe),
    (0x04D100, i_04d100),
    (0x04D106, i_04d106),
    (0x04D10A, i_04d10a),
    (0x04D110, i_04d110),
    (0x04D114, i_04d114),
    (0x04D11A, i_04d11a),
    (0x04D11E, i_04d11e),
    (0x04D122, i_04d122),
    (0x04D124, i_04d124),
    (0x04D128, i_04d128),
    (0x04D12E, i_04d12e),
    (0x04D132, i_04d132),
    (0x04D138, i_04d138),
    (0x04D13A, i_04d13a),
    (0x04D13E, i_04d13e),
    (0x04D144, i_04d144),
    (0x04D14A, i_04d14a),
    (0x04D150, i_04d150),
    (0x04D154, i_04d154),
    (0x04D156, i_04d156),
    (0x04D15C, i_04d15c),
    (0x04D160, i_04d160),
    (0x04D166, i_04d166),
    (0x04D16A, i_04d16a),
    (0x04D16E, i_04d16e),
    (0x04D170, i_04d170),
    (0x04D174, i_04d174),
    (0x04D17A, i_04d17a),
    (0x04D17E, i_04d17e),
    (0x04D184, i_04d184),
    (0x04D186, i_04d186),
    (0x04D18A, i_04d18a),
    (0x04D18E, i_04d18e),
    (0x04D194, i_04d194),
    (0x04D19A, i_04d19a),
    (0x04D1A0, i_04d1a0),
    (0x04D1A6, i_04d1a6),
    (0x04D1AA, i_04d1aa),
    (0x04D1AE, i_04d1ae),
    (0x04D1B4, i_04d1b4),
    (0x04D1BA, i_04d1ba),
    (0x04D1BE, i_04d1be),
    (0x04D1C2, i_04d1c2),
    (0x04D1C6, i_04d1c6),
    (0x04D1C8, i_04d1c8),
    (0x04D1CC, i_04d1cc),
    (0x04D1CE, i_04d1ce),
    (0x04D1D2, i_04d1d2),
    (0x04D1D6, i_04d1d6),
    (0x04D1D8, i_04d1d8),
    (0x04D1DC, i_04d1dc),
    (0x04D1E0, i_04d1e0),
    (0x04D1E6, i_04d1e6),
    (0x04D1EA, i_04d1ea),
    (0x04D1EE, i_04d1ee),
    (0x04D1F2, i_04d1f2),
    (0x04D1F6, i_04d1f6),
    (0x04D1FA, i_04d1fa),
    (0x04D200, i_04d200),
    (0x04D204, i_04d204),
    (0x04D208, i_04d208),
    (0x04D20E, i_04d20e),
    (0x04D212, i_04d212),
    (0x04D218, i_04d218),
    (0x04D21C, i_04d21c),
    (0x04D220, i_04d220),
    (0x04D224, i_04d224),
    (0x04D228, i_04d228),
    (0x04D22C, i_04d22c),
    (0x04D230, i_04d230),
    (0x04D234, i_04d234),
    (0x04D238, i_04d238),
    (0x04D23C, i_04d23c),
    (0x04D240, i_04d240),
    (0x04D244, i_04d244),
    (0x04D248, i_04d248),
    (0x04D24C, i_04d24c),
    (0x04D250, i_04d250),
    (0x04D254, i_04d254),
    (0x04D258, i_04d258),
    (0x04D25E, i_04d25e),
    (0x04D262, i_04d262),
    (0x04D266, i_04d266),
    (0x04D26A, i_04d26a),
    (0x04D270, i_04d270),
    (0x04D274, i_04d274),
    (0x04D278, i_04d278),
    (0x04D27C, i_04d27c),
    (0x04D282, i_04d282),
    (0x04D286, i_04d286),
    (0x04D28A, i_04d28a),
    (0x04D290, i_04d290),
    (0x04D296, i_04d296),
    (0x04D29A, i_04d29a),
    (0x04D29E, i_04d29e),
    (0x04D2A4, i_04d2a4),
    (0x04D2AA, i_04d2aa),
    (0x04D2AE, i_04d2ae),
    (0x04D2B2, i_04d2b2),
    (0x04D2B6, i_04d2b6),
    (0x04D2BA, i_04d2ba),
    (0x04D2BE, i_04d2be),
    (0x04D2C4, i_04d2c4),
    (0x04D2CA, i_04d2ca),
    (0x04D2CE, i_04d2ce),
    (0x04D2D4, i_04d2d4),
    (0x04D2D8, i_04d2d8),
    (0x04D2DE, i_04d2de),
    (0x04D2E4, i_04d2e4),
    (0x04D2EA, i_04d2ea),
    (0x04D2F0, i_04d2f0),
    (0x04D2F4, i_04d2f4),
    (0x04D2FA, i_04d2fa),
    (0x04D300, i_04d300),
    (0x04D306, i_04d306),
    (0x04D30C, i_04d30c),
    (0x04D312, i_04d312),
    (0x04D318, i_04d318),
    (0x04D31C, i_04d31c),
    (0x04D322, i_04d322),
    (0x04D326, i_04d326),
    (0x04D32A, i_04d32a),
    (0x04D32E, i_04d32e),
    (0x04D332, i_04d332),
    (0x04D338, i_04d338),
    (0x04D33E, i_04d33e),
    (0x04D340, i_04d340),
    (0x04D346, i_04d346),
    (0x04D34C, i_04d34c),
    (0x04D352, i_04d352),
    (0x04D356, i_04d356),
    (0x04D35A, i_04d35a),
    (0x04D360, i_04d360),
    (0x04D366, i_04d366),
    (0x04D36C, i_04d36c),
    (0x04D372, i_04d372),
    (0x04D376, i_04d376),
    (0x04D37C, i_04d37c),
    (0x04D380, i_04d380),
    (0x04D384, i_04d384),
    (0x04D386, i_04d386),
    (0x04D38A, i_04d38a),
    (0x04D390, i_04d390),
    (0x04D394, i_04d394),
    (0x04D398, i_04d398),
    (0x04D39E, i_04d39e),
    (0x04D3A2, i_04d3a2),
    (0x04D3A6, i_04d3a6),
    (0x04D3A8, i_04d3a8),
    (0x04D3AE, i_04d3ae),
    (0x04D3B4, i_04d3b4),
    (0x04D3B8, i_04d3b8),
    (0x04D3BC, i_04d3bc),
    (0x04D3C2, i_04d3c2),
    (0x04D3C6, i_04d3c6),
    (0x04D3CC, i_04d3cc),
    (0x04D3CE, i_04d3ce),
    (0x04D3D4, i_04d3d4),
    (0x04D3D8, i_04d3d8),
    (0x04D3DE, i_04d3de),
    (0x04D3E0, i_04d3e0),
    (0x04D3E4, i_04d3e4),
    (0x04D3E8, i_04d3e8),
    (0x04D3EC, i_04d3ec),
    (0x04D3F0, i_04d3f0),
    (0x04D3F4, i_04d3f4),
    (0x04D3F8, i_04d3f8),
    (0x04D3FC, i_04d3fc),
    (0x04D3FE, i_04d3fe),
    (0x04D402, i_04d402),
    (0x04D408, i_04d408),
    (0x04D40C, i_04d40c),
    (0x04D412, i_04d412),
    (0x04D416, i_04d416),
    (0x04D41A, i_04d41a),
    (0x04D420, i_04d420),
    (0x04D424, i_04d424),
    (0x04D428, i_04d428),
    (0x04D42C, i_04d42c),
    (0x04D430, i_04d430),
    (0x04D434, i_04d434),
    (0x04D438, i_04d438),
    (0x04D43C, i_04d43c),
    (0x04D440, i_04d440),
    (0x04D444, i_04d444),
    (0x04D446, i_04d446),
    (0x04D44A, i_04d44a),
    (0x04D44C, i_04d44c),
    (0x04D452, i_04d452),
    (0x04D454, i_04d454),
    (0x04D45A, i_04d45a),
    (0x04D45E, i_04d45e),
    (0x04D464, i_04d464),
    (0x04D468, i_04d468),
    (0x04D46C, i_04d46c),
    (0x04D46E, i_04d46e),
    (0x04D472, i_04d472),
    (0x04D474, i_04d474),
    (0x04D47A, i_04d47a),
    (0x04D47E, i_04d47e),
    (0x04D482, i_04d482),
    (0x04D488, i_04d488),
    (0x04D48A, i_04d48a),
    (0x04D48E, i_04d48e),
    (0x04D492, i_04d492),
    (0x04D498, i_04d498),
    (0x04D49C, i_04d49c),
    (0x04D4A0, i_04d4a0),
    (0x04D4A4, i_04d4a4),
    (0x04D4A6, i_04d4a6),
    (0x04D4AA, i_04d4aa),
    (0x04D4AE, i_04d4ae),
    (0x04D4B0, i_04d4b0),
    (0x04D4B4, i_04d4b4),
    (0x04D4B6, i_04d4b6),
    (0x04D4BA, i_04d4ba),
    (0x04D4BE, i_04d4be),
    (0x04D4C2, i_04d4c2),
    (0x04D4C6, i_04d4c6),
    (0x04D4CA, i_04d4ca),
    (0x04D4CE, i_04d4ce),
    (0x04D4D0, i_04d4d0),
    (0x04D4D4, i_04d4d4),
    (0x04D4D6, i_04d4d6),
    (0x04D4DA, i_04d4da),
    (0x04D4DE, i_04d4de),
    (0x04D4E2, i_04d4e2),
    (0x04D4E6, i_04d4e6),
    (0x04D4EA, i_04d4ea),
    (0x04D4EE, i_04d4ee),
    (0x04D4F0, i_04d4f0),
    (0x04D4F6, i_04d4f6),
    (0x04D4FA, i_04d4fa),
    (0x04D4FC, i_04d4fc),
    (0x04D500, i_04d500),
    (0x04D502, i_04d502),
    (0x04D506, i_04d506),
    (0x04D508, i_04d508),
    (0x04D50C, i_04d50c),
    (0x04D512, i_04d512),
    (0x04D516, i_04d516),
    (0x04D51A, i_04d51a),
    (0x04D51C, i_04d51c),
    (0x04D520, i_04d520),
    (0x04D526, i_04d526),
    (0x04D528, i_04d528),
    (0x04D52C, i_04d52c),
    (0x04D52E, i_04d52e),
    (0x04D532, i_04d532),
    (0x04D536, i_04d536),
    (0x04D53A, i_04d53a),
    (0x04D53E, i_04d53e),
    (0x04D540, i_04d540),
    (0x04D544, i_04d544),
    (0x04D548, i_04d548),
    (0x04D54C, i_04d54c),
    (0x04D552, i_04d552),
    (0x04D556, i_04d556),
    (0x04D558, i_04d558),
    (0x04D55E, i_04d55e),
    (0x04D562, i_04d562),
    (0x04D566, i_04d566),
    (0x04D56C, i_04d56c),
    (0x04D572, i_04d572),
    (0x04D576, i_04d576),
    (0x04D57C, i_04d57c),
    (0x04D57E, i_04d57e),
    (0x04D582, i_04d582),
    (0x04D588, i_04d588),
    (0x04D58A, i_04d58a),
    (0x04D590, i_04d590),
    (0x04D594, i_04d594),
    (0x04D596, i_04d596),
    (0x04D59A, i_04d59a),
    (0x04D59E, i_04d59e),
    (0x04D5A0, i_04d5a0),
    (0x04D5A2, i_04d5a2),
    (0x04D5A4, i_04d5a4),
    (0x04D5A6, i_04d5a6),
    (0x04D5AA, i_04d5aa),
    (0x04D5AC, i_04d5ac),
    (0x04D5B2, i_04d5b2),
    (0x04D5B8, i_04d5b8),
    (0x04D5BC, i_04d5bc),
    (0x04D5C0, i_04d5c0),
    (0x04D5C2, i_04d5c2),
    (0x04D5C6, i_04d5c6),
    (0x04D5C8, i_04d5c8),
    (0x04D5CE, i_04d5ce),
    (0x04D5D4, i_04d5d4),
    (0x04D5D8, i_04d5d8),
    (0x04D5DC, i_04d5dc),
    (0x04D5DE, i_04d5de),
    (0x04D5E2, i_04d5e2),
    (0x04D5E4, i_04d5e4),
    (0x04D5E8, i_04d5e8),
    (0x04D5EE, i_04d5ee),
    (0x04D5F0, i_04d5f0),
    (0x04D5F6, i_04d5f6),
    (0x04D5FA, i_04d5fa),
    (0x04D5FE, i_04d5fe),
    (0x04D600, i_04d600),
    (0x04D604, i_04d604),
    (0x04D608, i_04d608),
    (0x04D60C, i_04d60c),
    (0x04D610, i_04d610),
    (0x04D612, i_04d612),
    (0x04D616, i_04d616),
    (0x04D61C, i_04d61c),
    (0x04D61E, i_04d61e),
    (0x04D622, i_04d622),
    (0x04D624, i_04d624),
    (0x04D62A, i_04d62a),
    (0x04D62E, i_04d62e),
    (0x04D632, i_04d632),
    (0x04D634, i_04d634),
    (0x04D638, i_04d638),
    (0x04D63C, i_04d63c),
    (0x04D640, i_04d640),
    (0x04D644, i_04d644),
    (0x04D646, i_04d646),
    (0x04D64C, i_04d64c),
    (0x04D650, i_04d650),
    (0x04D654, i_04d654),
    (0x04D656, i_04d656),
    (0x04D65A, i_04d65a),
    (0x04D65E, i_04d65e),
    (0x04D662, i_04d662),
    (0x04D666, i_04d666),
    (0x04D668, i_04d668),
    (0x04D66C, i_04d66c),
    (0x04D670, i_04d670),
    (0x04D674, i_04d674),
    (0x04D678, i_04d678),
    (0x04D67C, i_04d67c),
    (0x04D680, i_04d680),
    (0x04D684, i_04d684),
    (0x04D688, i_04d688),
    (0x04D68E, i_04d68e),
    (0x04D692, i_04d692),
    (0x04D696, i_04d696),
    (0x04D698, i_04d698),
    (0x04D69C, i_04d69c),
    (0x04D6A0, i_04d6a0),
    (0x04D6A2, i_04d6a2),
    (0x04D6A6, i_04d6a6),
    (0x04D6AA, i_04d6aa),
    (0x04D6AC, i_04d6ac),
    (0x04D6B2, i_04d6b2),
    (0x04D6B6, i_04d6b6),
    (0x04D6BC, i_04d6bc),
    (0x04D6BE, i_04d6be),
    (0x04D6C4, i_04d6c4),
    (0x04D6C8, i_04d6c8),
    (0x04D6CE, i_04d6ce),
    (0x04D6D0, i_04d6d0),
    (0x04D6D4, i_04d6d4),
    (0x04D6D8, i_04d6d8),
    (0x04D6DC, i_04d6dc),
    (0x04D6E0, i_04d6e0),
    (0x04D6E4, i_04d6e4),
    (0x04D6E6, i_04d6e6),
    (0x04D6EA, i_04d6ea),
    (0x04D6EC, i_04d6ec),
    (0x04D6F0, i_04d6f0),
    (0x04D6F6, i_04d6f6),
    (0x04D6FC, i_04d6fc),
    (0x04D702, i_04d702),
    (0x04D708, i_04d708),
    (0x04D70C, i_04d70c),
    (0x04D710, i_04d710),
    (0x04D716, i_04d716),
    (0x04D71C, i_04d71c),
    (0x04D722, i_04d722),
    (0x04D728, i_04d728),
    (0x04D72C, i_04d72c),
    (0x04D72E, i_04d72e),
    (0x04D734, i_04d734),
    (0x04D73A, i_04d73a),
    (0x04D73E, i_04d73e),
    (0x04D742, i_04d742),
    (0x04D748, i_04d748),
    (0x04D74A, i_04d74a),
    (0x04D74E, i_04d74e),
    (0x04D754, i_04d754),
    (0x04D758, i_04d758),
    (0x04D75E, i_04d75e),
    (0x04D764, i_04d764),
    (0x04D76A, i_04d76a),
    (0x04D770, i_04d770),
    (0x04D776, i_04d776),
    (0x04D77C, i_04d77c),
    (0x04D780, i_04d780),
    (0x04D782, i_04d782),
    (0x04D788, i_04d788),
    (0x04D78E, i_04d78e),
    (0x04D794, i_04d794),
    (0x04D798, i_04d798),
    (0x04D79C, i_04d79c),
    (0x04D7A2, i_04d7a2),
    (0x04D7A8, i_04d7a8),
    (0x04D7AC, i_04d7ac),
    (0x04D7B0, i_04d7b0),
    (0x04D7B4, i_04d7b4),
    (0x04D7BA, i_04d7ba),
    (0x04D7C0, i_04d7c0),
    (0x04D7C6, i_04d7c6),
    (0x04D7CA, i_04d7ca),
    (0x04D7CC, i_04d7cc),
    (0x04D7D0, i_04d7d0),
    (0x04D7D4, i_04d7d4),
    (0x04D7DA, i_04d7da),
    (0x04D7DC, i_04d7dc),
    (0x04D7E2, i_04d7e2),
    (0x04D7E8, i_04d7e8),
    (0x04D7EC, i_04d7ec),
    (0x04D7EE, i_04d7ee),
    (0x04D7F4, i_04d7f4),
    (0x04D7FA, i_04d7fa),
    (0x04D7FE, i_04d7fe),
    (0x04D804, i_04d804),
    (0x04D808, i_04d808),
    (0x04D80A, i_04d80a),
    (0x04D80E, i_04d80e),
    (0x04D812, i_04d812),
    (0x04D818, i_04d818),
    (0x04D81A, i_04d81a),
    (0x04D81E, i_04d81e),
    (0x04D824, i_04d824),
    (0x04D82A, i_04d82a),
    (0x04D830, i_04d830),
    (0x04D836, i_04d836),
    (0x04D83C, i_04d83c),
    (0x04D842, i_04d842),
    (0x04D848, i_04d848),
    (0x04D84E, i_04d84e),
    (0x04D852, i_04d852),
    (0x04D856, i_04d856),
    (0x04D85C, i_04d85c),
    (0x04D860, i_04d860),
    (0x04D864, i_04d864),
    (0x04D868, i_04d868),
    (0x04D86A, i_04d86a),
    (0x04D86E, i_04d86e),
    (0x04D874, i_04d874),
    (0x04D878, i_04d878),
    (0x04D87C, i_04d87c),
    (0x04D880, i_04d880),
    (0x04D882, i_04d882),
    (0x04D886, i_04d886),
    (0x04D88C, i_04d88c),
    (0x04D892, i_04d892),
    (0x04D896, i_04d896),
    (0x04D898, i_04d898),
    (0x04D89E, i_04d89e),
    (0x04D8A4, i_04d8a4),
    (0x04D8A8, i_04d8a8),
    (0x04D8AC, i_04d8ac),
    (0x04D8B0, i_04d8b0),
    (0x04D8B4, i_04d8b4),
    (0x04D8BA, i_04d8ba),
    (0x04D8BE, i_04d8be),
    (0x04D8C4, i_04d8c4),
    (0x04D8CA, i_04d8ca),
    (0x04D8D0, i_04d8d0),
    (0x04D8D4, i_04d8d4),
    (0x04D8DA, i_04d8da),
    (0x04D8E0, i_04d8e0),
    (0x04D8E6, i_04d8e6),
    (0x04D8EA, i_04d8ea),
    (0x04D8F0, i_04d8f0),
    (0x04D8F2, i_04d8f2),
    (0x04D8F6, i_04d8f6),
    (0x04D8FC, i_04d8fc),
    (0x04D900, i_04d900),
    (0x04D906, i_04d906),
    (0x04D90C, i_04d90c),
    (0x04D912, i_04d912),
    (0x04D918, i_04d918),
    (0x04D91E, i_04d91e),
    (0x04D924, i_04d924),
    (0x04D92A, i_04d92a),
    (0x04D930, i_04d930),
    (0x04D936, i_04d936),
    (0x04D93C, i_04d93c),
    (0x04D940, i_04d940),
    (0x04D946, i_04d946),
    (0x04D94C, i_04d94c),
    (0x04D952, i_04d952),
    (0x04D958, i_04d958),
    (0x04D95E, i_04d95e),
    (0x04D964, i_04d964),
    (0x04D968, i_04d968),
    (0x04D96A, i_04d96a),
    (0x04D970, i_04d970),
    (0x04D976, i_04d976),
    (0x04D97C, i_04d97c),
    (0x04D980, i_04d980),
    (0x04D984, i_04d984),
    (0x04D986, i_04d986),
    (0x04D98A, i_04d98a),
    (0x04D98E, i_04d98e),
    (0x04D994, i_04d994),
    (0x04D996, i_04d996),
    (0x04D99C, i_04d99c),
    (0x04D9A2, i_04d9a2),
    (0x04D9A8, i_04d9a8),
    (0x04D9AE, i_04d9ae),
    (0x04D9B4, i_04d9b4),
    (0x04D9B8, i_04d9b8),
    (0x04D9BA, i_04d9ba),
    (0x04D9C0, i_04d9c0),
    (0x04D9C6, i_04d9c6),
    (0x04D9CA, i_04d9ca),
    (0x04D9CE, i_04d9ce),
    (0x04D9D4, i_04d9d4),
    (0x04D9D6, i_04d9d6),
    (0x04D9DA, i_04d9da),
    (0x04D9E0, i_04d9e0),
    (0x04D9E6, i_04d9e6),
    (0x04D9EC, i_04d9ec),
    (0x04D9F2, i_04d9f2),
    (0x04D9F8, i_04d9f8),
    (0x04D9FE, i_04d9fe),
    (0x04DA04, i_04da04),
    (0x04DA08, i_04da08),
    (0x04DA0A, i_04da0a),
    (0x04DA10, i_04da10),
    (0x04DA16, i_04da16),
    (0x04DA1C, i_04da1c),
    (0x04DA20, i_04da20),
    (0x04DA26, i_04da26),
    (0x04DA2C, i_04da2c),
    (0x04DA32, i_04da32),
    (0x04DA38, i_04da38),
    (0x04DA3C, i_04da3c),
    (0x04DA40, i_04da40),
    (0x04DA42, i_04da42),
    (0x04DA46, i_04da46),
    (0x04DA4A, i_04da4a),
    (0x04DA50, i_04da50),
    (0x04DA52, i_04da52),
    (0x04DA58, i_04da58),
    (0x04DA5E, i_04da5e),
    (0x04DA64, i_04da64),
    (0x04DA68, i_04da68),
    (0x04DA6E, i_04da6e),
    (0x04DA74, i_04da74),
    (0x04DA7A, i_04da7a),
    (0x04DA80, i_04da80),
    (0x04DA86, i_04da86),
    (0x04DA8A, i_04da8a),
    (0x04DA8C, i_04da8c),
    (0x04DA92, i_04da92),
    (0x04DA98, i_04da98),
    (0x04DA9C, i_04da9c),
    (0x04DAA0, i_04daa0),
    (0x04DAA4, i_04daa4),
    (0x04DAAA, i_04daaa),
    (0x04DAAE, i_04daae),
    (0x04DAB4, i_04dab4),
    (0x04DAB8, i_04dab8),
    (0x04DABE, i_04dabe),
    (0x04DAC4, i_04dac4),
    (0x04DACA, i_04daca),
    (0x04DACE, i_04dace),
    (0x04DAD0, i_04dad0),
    (0x04DAD2, i_04dad2),
    (0x04DAD6, i_04dad6),
    (0x04DADC, i_04dadc),
    (0x04DAE0, i_04dae0),
    (0x04DAE6, i_04dae6),
    (0x04DAEC, i_04daec),
    (0x04DAF2, i_04daf2),
    (0x04DAF8, i_04daf8),
    (0x04DAFC, i_04dafc),
    (0x04DB02, i_04db02),
    (0x04DB08, i_04db08),
    (0x04DB0C, i_04db0c),
    (0x04DB12, i_04db12),
    (0x04DB18, i_04db18),
    (0x04DB1E, i_04db1e),
    (0x04DB24, i_04db24),
    (0x04DB2A, i_04db2a),
    (0x04DB30, i_04db30),
    (0x04DB34, i_04db34),
    (0x04DB36, i_04db36),
    (0x04DB3C, i_04db3c),
    (0x04DB40, i_04db40),
    (0x04DB46, i_04db46),
    (0x04DB48, i_04db48),
    (0x04DB4C, i_04db4c),
    (0x04DB50, i_04db50),
    (0x04DB54, i_04db54),
    (0x04DB58, i_04db58),
    (0x04DB5E, i_04db5e),
    (0x04DB62, i_04db62),
    (0x04DB68, i_04db68),
    (0x04DB6C, i_04db6c),
    (0x04DB70, i_04db70),
    (0x04DB72, i_04db72),
    (0x04DB78, i_04db78),
    (0x04DB7C, i_04db7c),
    (0x04DB82, i_04db82),
    (0x04DB88, i_04db88),
    (0x04DB8E, i_04db8e),
    (0x04DB94, i_04db94),
    (0x04DB9A, i_04db9a),
    (0x04DBA0, i_04dba0),
    (0x04DBA6, i_04dba6),
    (0x04DBAA, i_04dbaa),
    (0x04DBAC, i_04dbac),
    (0x04DBB2, i_04dbb2),
    (0x04DBB8, i_04dbb8),
    (0x04DBBE, i_04dbbe),
    (0x04DBC2, i_04dbc2),
    (0x04DBC8, i_04dbc8),
    (0x04DBCE, i_04dbce),
    (0x04DBD4, i_04dbd4),
    (0x04DBDA, i_04dbda),
    (0x04DBDE, i_04dbde),
    (0x04DBE4, i_04dbe4),
    (0x04DBEA, i_04dbea),
    (0x04DBF0, i_04dbf0),
    (0x04DBF6, i_04dbf6),
    (0x04DBFC, i_04dbfc),
    (0x04DC02, i_04dc02),
    (0x04DC08, i_04dc08),
    (0x04DC0E, i_04dc0e),
    (0x04DC14, i_04dc14),
    (0x04DC18, i_04dc18),
    (0x04DC1E, i_04dc1e),
    (0x04DC24, i_04dc24),
    (0x04DC28, i_04dc28),
    (0x04DC2A, i_04dc2a),
    (0x04DC2E, i_04dc2e),
    (0x04DC32, i_04dc32),
    (0x04DC38, i_04dc38),
    (0x04DC3C, i_04dc3c),
    (0x04DC42, i_04dc42),
    (0x04DC46, i_04dc46),
    (0x04DC48, i_04dc48),
    (0x04DC4A, i_04dc4a),
    (0x04DC50, i_04dc50),
    (0x04DC56, i_04dc56),
    (0x04DC5A, i_04dc5a),
    (0x04DC5E, i_04dc5e),
    (0x04DC64, i_04dc64),
    (0x04DC68, i_04dc68),
    (0x04DC6E, i_04dc6e),
    (0x04DC74, i_04dc74),
    (0x04DC78, i_04dc78),
    (0x04DC7E, i_04dc7e),
    (0x04DC82, i_04dc82),
    (0x04DC86, i_04dc86),
    (0x04DC88, i_04dc88),
    (0x04DC8E, i_04dc8e),
    (0x04DC92, i_04dc92),
    (0x04DC94, i_04dc94),
    (0x04DC96, i_04dc96),
    (0x04DC9A, i_04dc9a),
    (0x04DCA0, i_04dca0),
    (0x04DCA6, i_04dca6),
    (0x04DCAC, i_04dcac),
    (0x04DCB2, i_04dcb2),
    (0x04DCB8, i_04dcb8),
    (0x04DCBE, i_04dcbe),
    (0x04DCC2, i_04dcc2),
    (0x04DCC4, i_04dcc4),
    (0x04DCCA, i_04dcca),
    (0x04DCD0, i_04dcd0),
    (0x04DCD4, i_04dcd4),
    (0x04DCDA, i_04dcda),
    (0x04DCDE, i_04dcde),
    (0x04DCE4, i_04dce4),
    (0x04DCE6, i_04dce6),
    (0x04DCEC, i_04dcec),
    (0x04DCF2, i_04dcf2),
    (0x04DCF8, i_04dcf8),
    (0x04DCFE, i_04dcfe),
    (0x04DD04, i_04dd04),
    (0x04DD0A, i_04dd0a),
    (0x04DD10, i_04dd10),
    (0x04DD16, i_04dd16),
    (0x04DD1C, i_04dd1c),
    (0x04DD22, i_04dd22),
    (0x04DD28, i_04dd28),
    (0x04DD2C, i_04dd2c),
    (0x04DD2E, i_04dd2e),
    (0x04DD34, i_04dd34),
    (0x04DD3A, i_04dd3a),
    (0x04DD40, i_04dd40),
    (0x04DD44, i_04dd44),
    (0x04DD4A, i_04dd4a),
    (0x04DD4E, i_04dd4e),
    (0x04DD52, i_04dd52),
    (0x04DD58, i_04dd58),
    (0x04DD5E, i_04dd5e),
    (0x04DD64, i_04dd64),
    (0x04DD6A, i_04dd6a),
    (0x04DD6E, i_04dd6e),
    (0x04DD74, i_04dd74),
    (0x04DD78, i_04dd78),
    (0x04DD7E, i_04dd7e),
    (0x04DD84, i_04dd84),
    (0x04DD8A, i_04dd8a),
    (0x04DD8E, i_04dd8e),
    (0x04DD94, i_04dd94),
    (0x04DD9A, i_04dd9a),
    (0x04DDA0, i_04dda0),
    (0x04DDA6, i_04dda6),
    (0x04DDAA, i_04ddaa),
    (0x04DDB0, i_04ddb0),
    (0x04DDB6, i_04ddb6),
    (0x04DDBC, i_04ddbc),
    (0x04DDC2, i_04ddc2),
    (0x04DDC6, i_04ddc6),
    (0x04DDCC, i_04ddcc),
    (0x04DDD0, i_04ddd0),
    (0x04DDD2, i_04ddd2),
    (0x04DDD6, i_04ddd6),
    (0x04DDDA, i_04ddda),
    (0x04DDDE, i_04ddde),
    (0x04DDE4, i_04dde4),
    (0x04DDE8, i_04dde8),
    (0x04DDEA, i_04ddea),
    (0x04DDEC, i_04ddec),
    (0x04DDF2, i_04ddf2),
    (0x04DDF8, i_04ddf8),
    (0x04DDFC, i_04ddfc),
    (0x04DE00, i_04de00),
    (0x04DE04, i_04de04),
    (0x04DE0A, i_04de0a),
    (0x04DE10, i_04de10),
    (0x04DE14, i_04de14),
    (0x04DE1A, i_04de1a),
    (0x04DE20, i_04de20),
    (0x04DE26, i_04de26),
    (0x04DE2C, i_04de2c),
    (0x04DE30, i_04de30),
    (0x04DE32, i_04de32),
    (0x04DE38, i_04de38),
    (0x04DE3C, i_04de3c),
    (0x04DE3E, i_04de3e),
    (0x04DE40, i_04de40),
    (0x04DE44, i_04de44),
    (0x04DE4A, i_04de4a),
    (0x04DE50, i_04de50),
    (0x04DE54, i_04de54),
    (0x04DE5A, i_04de5a),
    (0x04DE60, i_04de60),
    (0x04DE66, i_04de66),
    (0x04DE6C, i_04de6c),
    (0x04DE72, i_04de72),
    (0x04DE78, i_04de78),
    (0x04DE7E, i_04de7e),
    (0x04DE84, i_04de84),
    (0x04DE88, i_04de88),
    (0x04DE8A, i_04de8a),
    (0x04DE90, i_04de90),
    (0x04DE96, i_04de96),
    (0x04DE9C, i_04de9c),
    (0x04DEA0, i_04dea0),
    (0x04DEA6, i_04dea6),
    (0x04DEAC, i_04deac),
    (0x04DEB2, i_04deb2),
    (0x04DEB8, i_04deb8),
    (0x04DEBC, i_04debc),
    (0x04DEC2, i_04dec2),
    (0x04DEC8, i_04dec8),
    (0x04DECE, i_04dece),
    (0x04DED4, i_04ded4),
    (0x04DEDA, i_04deda),
    (0x04DEE0, i_04dee0),
    (0x04DEE6, i_04dee6),
    (0x04DEEC, i_04deec),
    (0x04DEF2, i_04def2),
    (0x04DEF8, i_04def8),
    (0x04DEFE, i_04defe),
    (0x04DF04, i_04df04),
    (0x04DF08, i_04df08),
    (0x04DF0A, i_04df0a),
    (0x04DF0E, i_04df0e),
    (0x04DF12, i_04df12),
    (0x04DF18, i_04df18),
    (0x04DF1C, i_04df1c),
    (0x04DF1E, i_04df1e),
    (0x04DF22, i_04df22),
    (0x04DF26, i_04df26),
    (0x04DF2A, i_04df2a),
    (0x04DF2E, i_04df2e),
    (0x04DF30, i_04df30),
    (0x04DF36, i_04df36),
    (0x04DF3C, i_04df3c),
    (0x04DF40, i_04df40),
    (0x04DF44, i_04df44),
    (0x04DF48, i_04df48),
    (0x04DF4E, i_04df4e),
    (0x04DF54, i_04df54),
    (0x04DF58, i_04df58),
    (0x04DF5E, i_04df5e),
    (0x04DF62, i_04df62),
    (0x04DF68, i_04df68),
    (0x04DF6C, i_04df6c),
    (0x04DF70, i_04df70),
    (0x04DF72, i_04df72),
    (0x04DF78, i_04df78),
    (0x04DF7C, i_04df7c),
    (0x04DF80, i_04df80),
    (0x04DF84, i_04df84),
    (0x04DF86, i_04df86),
    (0x04DF8A, i_04df8a),
    (0x04DF8E, i_04df8e),
    (0x04DF92, i_04df92),
    (0x04DF96, i_04df96),
    (0x04DF98, i_04df98),
    (0x04DF9C, i_04df9c),
    (0x04DFA2, i_04dfa2),
    (0x04DFA6, i_04dfa6),
    (0x04DFAC, i_04dfac),
    (0x04DFB2, i_04dfb2),
    (0x04DFB6, i_04dfb6),
    (0x04DFBC, i_04dfbc),
    (0x04DFC2, i_04dfc2),
    (0x04DFC8, i_04dfc8),
    (0x04DFCE, i_04dfce),
    (0x04DFD4, i_04dfd4),
    (0x04DFDA, i_04dfda),
    (0x04DFE0, i_04dfe0),
    (0x04DFE4, i_04dfe4),
    (0x04DFEA, i_04dfea),
    (0x04DFF0, i_04dff0),
    (0x04DFF4, i_04dff4),
    (0x04DFF6, i_04dff6),
    (0x04DFFC, i_04dffc),
    (0x04E002, i_04e002),
    (0x04E006, i_04e006),
    (0x04E00A, i_04e00a),
    (0x04E010, i_04e010),
    (0x04E016, i_04e016),
    (0x04E01C, i_04e01c),
    (0x04E022, i_04e022),
    (0x04E028, i_04e028),
    (0x04E02E, i_04e02e),
    (0x04E034, i_04e034),
    (0x04E03A, i_04e03a),
    (0x04E03E, i_04e03e),
    (0x04E044, i_04e044),
    (0x04E048, i_04e048),
    (0x04E04C, i_04e04c),
    (0x04E04E, i_04e04e),
    (0x04E052, i_04e052),
    (0x04E056, i_04e056),
    (0x04E05C, i_04e05c),
    (0x04E05E, i_04e05e),
    (0x04E064, i_04e064),
    (0x04E06A, i_04e06a),
    (0x04E070, i_04e070),
    (0x04E074, i_04e074),
    (0x04E076, i_04e076),
    (0x04E07C, i_04e07c),
    (0x04E082, i_04e082),
    (0x04E088, i_04e088),
    (0x04E08C, i_04e08c),
    (0x04E092, i_04e092),
    (0x04E098, i_04e098),
    (0x04E09E, i_04e09e),
    (0x04E0A4, i_04e0a4),
    (0x04E0A8, i_04e0a8),
    (0x04E0AC, i_04e0ac),
    (0x04E0B0, i_04e0b0),
    (0x04E0B6, i_04e0b6),
    (0x04E0BC, i_04e0bc),
    (0x04E0C0, i_04e0c0),
    (0x04E0C6, i_04e0c6),
    (0x04E0CC, i_04e0cc),
    (0x04E0D2, i_04e0d2),
    (0x04E0D8, i_04e0d8),
    (0x04E0DE, i_04e0de),
    (0x04E0E4, i_04e0e4),
    (0x04E0EA, i_04e0ea),
    (0x04E0F0, i_04e0f0),
    (0x04E0F4, i_04e0f4),
    (0x04E0FA, i_04e0fa),
    (0x04E0FE, i_04e0fe),
    (0x04E104, i_04e104),
    (0x04E108, i_04e108),
    (0x04E10E, i_04e10e),
    (0x04E112, i_04e112),
    (0x04E116, i_04e116),
    (0x04E11A, i_04e11a),
    (0x04E11E, i_04e11e),
    (0x04E124, i_04e124),
    (0x04E12A, i_04e12a),
    (0x04E12C, i_04e12c),
    (0x04E132, i_04e132),
    (0x04E138, i_04e138),
    (0x04E13E, i_04e13e),
    (0x04E144, i_04e144),
    (0x04E148, i_04e148),
    (0x04E14E, i_04e14e),
    (0x04E154, i_04e154),
    (0x04E158, i_04e158),
    (0x04E15E, i_04e15e),
    (0x04E164, i_04e164),
    (0x04E16A, i_04e16a),
    (0x04E170, i_04e170),
    (0x04E176, i_04e176),
    (0x04E17C, i_04e17c),
    (0x04E182, i_04e182),
    (0x04E186, i_04e186),
    (0x04E188, i_04e188),
    (0x04E18E, i_04e18e),
    (0x04E194, i_04e194),
    (0x04E19A, i_04e19a),
    (0x04E19E, i_04e19e),
    (0x04E1A4, i_04e1a4),
    (0x04E1AA, i_04e1aa),
    (0x04E1B0, i_04e1b0),
    (0x04E1B6, i_04e1b6),
    (0x04E1BA, i_04e1ba),
    (0x04E1BE, i_04e1be),
    (0x04E1C4, i_04e1c4),
    (0x04E1C8, i_04e1c8),
    (0x04E1CC, i_04e1cc),
    (0x04E1D2, i_04e1d2),
    (0x04E1D8, i_04e1d8),
    (0x04E1DE, i_04e1de),
    (0x04E1E4, i_04e1e4),
    (0x04E1EA, i_04e1ea),
    (0x04E1F0, i_04e1f0),
    (0x04E1F6, i_04e1f6),
    (0x04E1FC, i_04e1fc),
    (0x04E200, i_04e200),
    (0x04E206, i_04e206),
    (0x04E20A, i_04e20a),
    (0x04E20C, i_04e20c),
    (0x04E210, i_04e210),
    (0x04E214, i_04e214),
    (0x04E218, i_04e218),
    (0x04E21C, i_04e21c),
    (0x04E222, i_04e222),
    (0x04E226, i_04e226),
    (0x04E22A, i_04e22a),
    (0x04E22E, i_04e22e),
    (0x04E234, i_04e234),
    (0x04E238, i_04e238),
    (0x04E23C, i_04e23c),
    (0x04E240, i_04e240),
    (0x04E246, i_04e246),
    (0x04E248, i_04e248),
    (0x04E24C, i_04e24c),
    (0x04E252, i_04e252),
    (0x04E256, i_04e256),
    (0x04E25C, i_04e25c),
    (0x04E262, i_04e262),
    (0x04E268, i_04e268),
    (0x04E26E, i_04e26e),
    (0x04E274, i_04e274),
    (0x04E27A, i_04e27a),
    (0x04E280, i_04e280),
    (0x04E286, i_04e286),
    (0x04E28C, i_04e28c),
    (0x04E292, i_04e292),
    (0x04E296, i_04e296),
    (0x04E298, i_04e298),
    (0x04E29E, i_04e29e),
    (0x04E2A2, i_04e2a2),
    (0x04E2A8, i_04e2a8),
    (0x04E2AE, i_04e2ae),
    (0x04E2B0, i_04e2b0),
    (0x04E2B4, i_04e2b4),
    (0x04E2B6, i_04e2b6),
    (0x04E2BC, i_04e2bc),
    (0x04E2C2, i_04e2c2),
    (0x04E2C8, i_04e2c8),
    (0x04E2CC, i_04e2cc),
    (0x04E2D2, i_04e2d2),
    (0x04E2D8, i_04e2d8),
    (0x04E2DE, i_04e2de),
    (0x04E2E4, i_04e2e4),
    (0x04E2E8, i_04e2e8),
    (0x04E2EC, i_04e2ec),
    (0x04E2F2, i_04e2f2),
    (0x04E2F8, i_04e2f8),
    (0x04E2FE, i_04e2fe),
    (0x04E304, i_04e304),
    (0x04E30A, i_04e30a),
    (0x04E310, i_04e310),
    (0x04E316, i_04e316),
    (0x04E31A, i_04e31a),
    (0x04E31C, i_04e31c),
    (0x04E320, i_04e320),
    (0x04E324, i_04e324),
    (0x04E32A, i_04e32a),
    (0x04E32C, i_04e32c),
    (0x04E332, i_04e332),
    (0x04E338, i_04e338),
    (0x04E33C, i_04e33c),
    (0x04E340, i_04e340),
    (0x04E346, i_04e346),
    (0x04E34C, i_04e34c),
    (0x04E352, i_04e352),
    (0x04E356, i_04e356),
    (0x04E35C, i_04e35c),
    (0x04E362, i_04e362),
    (0x04E368, i_04e368),
    (0x04E36C, i_04e36c),
    (0x04E36E, i_04e36e),
    (0x04E374, i_04e374),
    (0x04E37A, i_04e37a),
    (0x04E37E, i_04e37e),
    (0x04E382, i_04e382),
    (0x04E388, i_04e388),
    (0x04E38A, i_04e38a),
    (0x04E38E, i_04e38e),
    (0x04E394, i_04e394),
    (0x04E398, i_04e398),
    (0x04E39E, i_04e39e),
    (0x04E3A4, i_04e3a4),
];
