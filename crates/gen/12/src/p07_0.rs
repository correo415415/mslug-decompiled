// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $070004: lea.l $2d6d70.l, a0
pub fn i_070004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7000A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D6D70;
    20
}
/// $07000A: jsr $28cd4.l
pub fn i_07000a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70010;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70010);
    cpu.pc = 0x28CD4;
    16
}
/// $070010: lea.l $70016(pc), a1
pub fn i_070010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70014;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70016;
    16
}
/// $070014: move.l a1, (a6)
pub fn i_070014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70016;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070016: bset.b #$6, $13(a6)
pub fn i_070016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7001C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07001C: jsr $27d50.l
pub fn i_07001c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70022);
    cpu.pc = 0x27D50;
    16
}
/// $070022: bcc.w $7002c
pub fn i_070022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70026;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7002C; return 10; }
    8
}
/// $070026: lea.l $70046(pc), a1
pub fn i_070026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7002A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70046;
    16
}
/// $07002A: move.l a1, (a6)
pub fn i_07002a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7002C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07002C: jsr $28d70.l
pub fn i_07002c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70032;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70032);
    cpu.pc = 0x28D70;
    16
}
/// $070032: cmpi.w #$150, $24(a6)
pub fn i_070032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70038;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $070038: bgt.w $70042
pub fn i_070038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7003C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x70042; return 10; }
    8
}
/// $07003C: lea.l $70046(pc), a1
pub fn i_07003c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70040;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70046;
    16
}
/// $070040: move.l a1, (a6)
pub fn i_070040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70042;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070042: bra.w $6fe64
pub fn i_070042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70046;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FE64;
    10
}
// ===== M5Boss_ShellFall_070046  ($070046, 46 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070046: clr.w $2a(a6)
pub fn i_070046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7004A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07004A: clr.w $2e(a6)
pub fn i_07004a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7004E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07004E: move.w $28(a6), d0
pub fn i_07004e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70052;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070052: lsl.w #$4, d0
pub fn i_070052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70054;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $070054: move.w d0, $28(a6)
pub fn i_070054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70058;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070058: lea.l $7005e(pc), a1
pub fn i_070058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7005C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7005E;
    16
}
/// $07005C: move.l a1, (a6)
pub fn i_07005c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7005E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07005E: bset.b #$6, $13(a6)
pub fn i_07005e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70064;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070064: jsr $27cee.l
pub fn i_070064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7006A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7006A);
    cpu.pc = 0x27CEE;
    16
}
/// $07006A: jsr $28d70.l
pub fn i_07006a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70070;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70070);
    cpu.pc = 0x28D70;
    16
}
/// $070070: bra.w $6fe64
pub fn i_070070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70074;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FE64;
    10
}
// ===== M5Boss_ProjExplode_070074  ($070074, 50 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070074: move.w #$4000, d0
pub fn i_070074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70078;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070078: jsr $28134.l
pub fn i_070078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7007E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7007E);
    cpu.pc = 0x28134;
    16
}
/// $07007E: andi.w #$ffe3, $38(a6)
pub fn i_07007e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70084;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $070084: ori.w #$0, $38(a6)
pub fn i_070084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7008A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07008A: move.w #$1021, d0
pub fn i_07008a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7008E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07008E: jsr $2352.l
pub fn i_07008e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70094;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70094);
    cpu.pc = 0x2352;
    16
}
/// $070094: lea.l $77fd6.l, a1
pub fn i_070094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7009A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07009A: jsr $4ae.l
pub fn i_07009a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x700A0);
    cpu.pc = 0x4AE;
    16
}
/// $0700A0: jsr $5dd02.l
pub fn i_0700a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x700A6);
    cpu.pc = 0x5DD02;
    16
}
// ===== M5Boss_Free_0700a6  ($0700A6, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0700A6: jmp $518.l
pub fn i_0700a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700AC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M5Boss_Nop_0700ac  ($0700AC, 2 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0700AC: rts
pub fn i_0700ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Fx2D78E8_0700ae  ($0700AE, 88 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0700AE: move.w #$4, d1
pub fn i_0700ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700B2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0700B2: jsr $236e.l
pub fn i_0700b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x700B8);
    cpu.pc = 0x236E;
    16
}
/// $0700B8: lea.l $2d81f8.l, a0
pub fn i_0700b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D81F8;
    20
}
/// $0700BE: jsr $28cd4.l
pub fn i_0700be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x700C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0700C4: lea.l $700ca(pc), a1
pub fn i_0700c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x700CA;
    16
}
/// $0700C8: move.l a1, (a6)
pub fn i_0700c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0700CA: jsr $5e506.l
pub fn i_0700ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x700D0);
    cpu.pc = 0x5E506;
    16
}
/// $0700D0: move.w $76(a0), $76(a6)
pub fn i_0700d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700D6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0700D6: move.b $89(a0), $89(a6)
pub fn i_0700d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700DC;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0700DC: jsr $28d70.l
pub fn i_0700dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x700E2);
    cpu.pc = 0x28D70;
    16
}
/// $0700E2: bcc.w $700fc
pub fn i_0700e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x700FC; return 10; }
    8
}
/// $0700E6: btst.b #$0, $89(a6)
pub fn i_0700e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700EC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0700EC: beq.w $700fc
pub fn i_0700ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x700FC; return 10; }
    8
}
/// $0700F0: lea.l $2d78e8.l, a0
pub fn i_0700f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D78E8;
    20
}
/// $0700F6: jsr $28cd4.l
pub fn i_0700f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x700FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x700FC);
    cpu.pc = 0x28CD4;
    16
}
/// $0700FC: jsr $5e45a.l
pub fn i_0700fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70102);
    cpu.pc = 0x5E45A;
    16
}
/// $070102: bcc.w $7010c
pub fn i_070102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70106;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7010C; return 10; }
    8
}
// ===== SetTaskHandler_070106  ($070106, 8 B, kernel/task_handlers.c)
/// $070106: lea.l $6fb02(pc), a1
pub fn i_070106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7010A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $07010A: move.l a1, (a6)
pub fn i_07010a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7010C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07010C: rts
pub fn i_07010c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7010E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Fx2D6FCC_07010e  ($07010E, 88 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07010E: move.w #$4, d1
pub fn i_07010e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70112;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070112: jsr $236e.l
pub fn i_070112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70118;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70118);
    cpu.pc = 0x236E;
    16
}
/// $070118: lea.l $2d81f8.l, a0
pub fn i_070118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7011E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D81F8;
    20
}
/// $07011E: jsr $28cd4.l
pub fn i_07011e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70124);
    cpu.pc = 0x28CD4;
    16
}
/// $070124: lea.l $7012a(pc), a1
pub fn i_070124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70128;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7012A;
    16
}
/// $070128: move.l a1, (a6)
pub fn i_070128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7012A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07012A: jsr $5e506.l
pub fn i_07012a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70130);
    cpu.pc = 0x5E506;
    16
}
/// $070130: move.w $76(a0), $76(a6)
pub fn i_070130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70136;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $070136: move.b $89(a0), $89(a6)
pub fn i_070136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7013C;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad4, v3 as u8);
    20
}
/// $07013C: jsr $28d70.l
pub fn i_07013c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70142;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70142);
    cpu.pc = 0x28D70;
    16
}
/// $070142: bcc.w $7015c
pub fn i_070142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70146;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7015C; return 10; }
    8
}
/// $070146: btst.b #$1, $89(a6)
pub fn i_070146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7014C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07014C: beq.w $7015c
pub fn i_07014c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70150;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7015C; return 10; }
    8
}
/// $070150: lea.l $2d6fcc.l, a0
pub fn i_070150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70156;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D6FCC;
    20
}
/// $070156: jsr $28cd4.l
pub fn i_070156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7015C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7015C);
    cpu.pc = 0x28CD4;
    16
}
/// $07015C: jsr $5e45a.l
pub fn i_07015c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70162);
    cpu.pc = 0x5E45A;
    16
}
/// $070162: bcc.w $7016c
pub fn i_070162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70166;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7016C; return 10; }
    8
}
// ===== SetTaskHandler_070166  ($070166, 8 B, kernel/task_handlers.c)
/// $070166: lea.l $6fb02(pc), a1
pub fn i_070166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7016A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $07016A: move.l a1, (a6)
pub fn i_07016a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7016C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07016C: rts
pub fn i_07016c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7016E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SparkBurst_07016e  ($07016E, 152 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07016E: jsr $5e9b6.l
pub fn i_07016e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70174);
    cpu.pc = 0x5E9B6;
    16
}
/// $070174: andi.b #$7, d0
pub fn i_070174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70178;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $070178: move.b d0, $99(a6)
pub fn i_070178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7017C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07017C: clr.w $70(a6)
pub fn i_07017c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70180;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $070180: lea.l $70186(pc), a1
pub fn i_070180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70184;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70186;
    16
}
/// $070184: move.l a1, (a6)
pub fn i_070184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70186;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070186: movea.l $c(a6), a0
pub fn i_070186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7018A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07018A: tst.b $20(a0)
pub fn i_07018a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7018E;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07018E: bne.w $701e8
pub fn i_07018e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70192;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x701E8; return 10; }
    8
}
/// $070192: addq.w #$1, $70(a6)
pub fn i_070192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70196;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $070196: andi.w #$3, $70(a6)
pub fn i_070196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7019C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07019C: bne.w $701e8
pub fn i_07019c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x701E8; return 10; }
    8
}
/// $0701A0: jsr $5e506.l
pub fn i_0701a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x701A6);
    cpu.pc = 0x5E506;
    16
}
/// $0701A6: move.w $76(a0), d0
pub fn i_0701a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701AA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0701AA: move.w d0, $76(a6)
pub fn i_0701aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701AE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0701AE: andi.w #$1f, d0
pub fn i_0701ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701B2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0701B2: lsl.w #$2, d0
pub fn i_0701b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701B4;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0701B4: lea.l $2d554e.l, a0
pub fn i_0701b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D554E;
    20
}
/// $0701BA: move.w (a0, d0.w), d1
pub fn i_0701ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701BE;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0701BE: move.w $2(a0, d0.w), d2
pub fn i_0701be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701C2;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0701C2: add.w d1, $22(a6)
pub fn i_0701c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701C6;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0701C6: addq.w #$4, d2
pub fn i_0701c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701C8;
    cpu.ir = 0x5842;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0701C8: add.w d2, $24(a6)
pub fn i_0701c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701CC;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0701CC: lea.l $7020e(pc), a1
pub fn i_0701cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7020E;
    16
}
/// $0701D0: jsr $6fe.l
pub fn i_0701d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x701D6);
    cpu.pc = 0x6FE;
    16
}
/// $0701D6: jsr $5dd02.l
pub fn i_0701d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x701DC);
    cpu.pc = 0x5DD02;
    16
}
/// $0701DC: move.b $99(a6), $99(a0)
pub fn i_0701dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701E2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0701E2: move.w $76(a6), $76(a0)
pub fn i_0701e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701E8;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0701E8: jsr $5e45a.l
pub fn i_0701e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x701EE);
    cpu.pc = 0x5E45A;
    16
}
/// $0701EE: bcc.w $701f8
pub fn i_0701ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x701F8; return 10; }
    8
}
/// $0701F2: lea.l $700a6(pc), a1
pub fn i_0701f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x700A6;
    16
}
/// $0701F6: move.l a1, (a6)
pub fn i_0701f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0701F8: movea.l $c(a6), a0
pub fn i_0701f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x701FC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0701FC: cmpi.l #$6fef0, (a0)
pub fn i_0701fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70202;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x6FEF0, Size::Long, false);
    16
}
/// $070202: bne.w $7020c
pub fn i_070202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70206;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7020C; return 10; }
    8
}
// ===== SetTaskHandler_070206  ($070206, 8 B, kernel/task_handlers.c)
/// $070206: lea.l $700a6(pc), a1
pub fn i_070206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7020A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x700A6;
    16
}
/// $07020A: move.l a1, (a6)
pub fn i_07020a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7020C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07020C: rts
pub fn i_07020c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7020E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Spark_07020e  ($07020E, 122 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07020E: move.w $76(a6), d0
pub fn i_07020e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70212;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070212: lsl.w #$4, d0
pub fn i_070212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70214;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $070214: lea.l $2c07ac.l, a1
pub fn i_070214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7021A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $07021A: lea.l $2c072c.l, a2
pub fn i_07021a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70220;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $070220: move.w (a1, d0.w), d1
pub fn i_070220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70224;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $070224: move.w (a2, d0.w), d2
pub fn i_070224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70228;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $070228: lsl.w #$2, d1
pub fn i_070228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7022A;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07022A: lsl.w #$2, d2
pub fn i_07022a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7022C;
    cpu.ir = 0xE54A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07022C: neg.w d1
pub fn i_07022c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7022E;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07022E: neg.w d2
pub fn i_07022e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70230;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $070230: move.w d1, $28(a6)
pub fn i_070230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70234;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070234: move.w d2, $2a(a6)
pub fn i_070234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70238;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070238: subq.w #$8, $24(a6)
pub fn i_070238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7023C;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07023C: move.b $99(a6), d0
pub fn i_07023c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70240;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $070240: andi.w #$7, d0
pub fn i_070240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70244;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070244: add.w d0, d0
pub fn i_070244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70246;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $070246: lea.l $2d553e.l, a0
pub fn i_070246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7024C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D553E;
    20
}
/// $07024C: move.w (a0, d0.w), d1
pub fn i_07024c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70250;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $070250: jsr $236e.l
pub fn i_070250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70256);
    cpu.pc = 0x236E;
    16
}
/// $070256: move.w #$8, $70(a6)
pub fn i_070256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7025C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07025C: lea.l $2d8204.l, a0
pub fn i_07025c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70262;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D8204;
    20
}
/// $070262: jsr $28cd4.l
pub fn i_070262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70268;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70268);
    cpu.pc = 0x28CD4;
    16
}
/// $070268: lea.l $7026e(pc), a1
pub fn i_070268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7026C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7026E;
    16
}
/// $07026C: move.l a1, (a6)
pub fn i_07026c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7026E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07026E: jsr $27cee.l
pub fn i_07026e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70274);
    cpu.pc = 0x27CEE;
    16
}
/// $070274: jsr $28d70.l
pub fn i_070274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7027A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7027A);
    cpu.pc = 0x28D70;
    16
}
/// $07027A: subq.w #$1, $70(a6)
pub fn i_07027a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7027E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07027E: cmpi.w #$0, $70(a6)
pub fn i_07027e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70284;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $070284: bgt.w $7028e
pub fn i_070284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70288;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7028E; return 10; }
    8
}
// ===== SetTaskHandler_070288  ($070288, 8 B, kernel/task_handlers.c)
/// $070288: lea.l $70290(pc), a1
pub fn i_070288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7028C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70290;
    16
}
/// $07028C: move.l a1, (a6)
pub fn i_07028c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7028E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07028E: rts
pub fn i_07028e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70290;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SparkEnd_070290  ($070290, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070290: lea.l $70296(pc), a1
pub fn i_070290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70294;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70296;
    16
}
/// $070294: move.l a1, (a6)
pub fn i_070294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70296;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070296: jsr $28d70.l
pub fn i_070296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7029C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7029C);
    cpu.pc = 0x28D70;
    16
}
/// $07029C: bcc.w $702a6
pub fn i_07029c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x702A6; return 10; }
    8
}
// ===== SetTaskHandler_0702a0  ($0702A0, 8 B, kernel/task_handlers.c)
/// $0702A0: lea.l $6fb02(pc), a1
pub fn i_0702a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $0702A4: move.l a1, (a6)
pub fn i_0702a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0702A6: rts
pub fn i_0702a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Fx2D82BA_0702a8  ($0702A8, 52 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0702A8: move.w #$10a4, d0
pub fn i_0702a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702AC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0702AC: jsr $2352.l
pub fn i_0702ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x702B2);
    cpu.pc = 0x2352;
    16
}
/// $0702B2: move.w #$4, d1
pub fn i_0702b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0702B6: jsr $236e.l
pub fn i_0702b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x702BC);
    cpu.pc = 0x236E;
    16
}
/// $0702BC: lea.l $2d82ba.l, a0
pub fn i_0702bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D82BA;
    20
}
/// $0702C2: jsr $28cd4.l
pub fn i_0702c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x702C8);
    cpu.pc = 0x28CD4;
    16
}
/// $0702C8: lea.l $702ce(pc), a1
pub fn i_0702c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x702CE;
    16
}
/// $0702CC: move.l a1, (a6)
pub fn i_0702cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0702CE: jsr $70d9c(pc)
pub fn i_0702ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x702D2);
    cpu.pc = 0x70D9C;
    16
}
/// $0702D2: jsr $28d70.l
pub fn i_0702d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x702D8);
    cpu.pc = 0x28D70;
    16
}
/// $0702D8: bcc.w $702e2
pub fn i_0702d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x702E2; return 10; }
    8
}
// ===== SetTaskHandler_0702dc  ($0702DC, 8 B, kernel/task_handlers.c)
/// $0702DC: lea.l $6fb02(pc), a1
pub fn i_0702dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $0702E0: move.l a1, (a6)
pub fn i_0702e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0702E2: rts
pub fn i_0702e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Fx2D8320_0702e4  ($0702E4, 24 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0702E4: move.w #$4, d1
pub fn i_0702e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702E8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0702E8: jsr $236e.l
pub fn i_0702e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x702EE);
    cpu.pc = 0x236E;
    16
}
/// $0702EE: lea.l $2d8320.l, a0
pub fn i_0702ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D8320;
    20
}
/// $0702F4: jsr $28cd4.l
pub fn i_0702f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x702FA);
    cpu.pc = 0x28CD4;
    16
}
/// $0702FA: bra.b $702c8
pub fn i_0702fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x702FC;
    cpu.ir = 0x60CC;
    cpu.pc = 0x702C8;
    10
}
// ===== M5Boss_Soldier_0702fc  ($0702FC, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0702FC: jsr $5e1ea.l
pub fn i_0702fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70302);
    cpu.pc = 0x5E1EA;
    16
}
/// $070302: move.l a0, $9c(a6)
pub fn i_070302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70306;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $070306: jsr $4a110.l
pub fn i_070306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7030C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7030C);
    cpu.pc = 0x4A110;
    16
}
// ===== M5Boss_SoldierWalk_07030c  ($07030C, 116 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07030C: lea.l $2b9dfa.l, a0
pub fn i_07030c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70312;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9DFA;
    20
}
/// $070312: jsr $799de.l
pub fn i_070312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70318);
    cpu.pc = 0x799DE;
    16
}
/// $070318: move.w d0, $70(a6)
pub fn i_070318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7031C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07031C: lea.l $29b816.l, a0
pub fn i_07031c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70322;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B816;
    20
}
/// $070322: jsr $28cd4.l
pub fn i_070322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70328);
    cpu.pc = 0x28CD4;
    16
}
/// $070328: lea.l $7032e(pc), a1
pub fn i_070328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7032C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7032E;
    16
}
/// $07032C: move.l a1, (a6)
pub fn i_07032c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7032E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07032E: jsr $70d1e(pc)
pub fn i_07032e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70332;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x70332);
    cpu.pc = 0x70D1E;
    16
}
/// $070332: jsr $28d70.l
pub fn i_070332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70338);
    cpu.pc = 0x28D70;
    16
}
/// $070338: subq.w #$1, $70(a6)
pub fn i_070338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7033C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07033C: cmpi.w #$0, $70(a6)
pub fn i_07033c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70342;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $070342: bgt.w $7034c
pub fn i_070342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70346;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7034C; return 10; }
    8
}
/// $070346: lea.l $70380(pc), a1
pub fn i_070346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7034A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70380;
    16
}
/// $07034A: move.l a1, (a6)
pub fn i_07034a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7034C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07034C: bra.w $70492
pub fn i_07034c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70350;
    cpu.ir = 0x6000;
    cpu.pc = 0x70492;
    10
}
/// $070350: eori.b #$1, $3a(a6)
pub fn i_070350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70356;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070356: lea.l $29bfb8.l, a0
pub fn i_070356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7035C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFB8;
    20
}
/// $07035C: jsr $28cd4.l
pub fn i_07035c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70362;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70362);
    cpu.pc = 0x28CD4;
    16
}
/// $070362: lea.l $70368(pc), a1
pub fn i_070362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70366;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70368;
    16
}
/// $070366: move.l a1, (a6)
pub fn i_070366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70368;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070368: jsr $70d1e(pc)
pub fn i_070368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7036C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7036C);
    cpu.pc = 0x70D1E;
    16
}
/// $07036C: jsr $28d70.l
pub fn i_07036c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70372;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70372);
    cpu.pc = 0x28D70;
    16
}
/// $070372: bcc.w $7037c
pub fn i_070372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70376;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7037C; return 10; }
    8
}
/// $070376: lea.l $70392(pc), a1
pub fn i_070376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7037A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70392;
    16
}
/// $07037A: move.l a1, (a6)
pub fn i_07037a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7037C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07037C: bra.w $70492
pub fn i_07037c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70380;
    cpu.ir = 0x6000;
    cpu.pc = 0x70492;
    10
}
// ===== M5Boss_SoldierAcquire_070380  ($070380, 18 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070380: jsr $5e1ea.l
pub fn i_070380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70386;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70386);
    cpu.pc = 0x5E1EA;
    16
}
/// $070386: move.l a0, $9c(a6)
pub fn i_070386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7038A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07038A: jsr $5e618.l
pub fn i_07038a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70390);
    cpu.pc = 0x5E618;
    16
}
/// $070390: bcs.b $70350
pub fn i_070390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70392;
    cpu.ir = 0x65BE;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x70350; return 10; }
    8
}
// ===== M5Boss_SoldierAim_070392  ($070392, 134 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070392: movea.l $9c(a6), a0
pub fn i_070392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70396;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070396: move.w $22(a0), d0
pub fn i_070396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7039A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07039A: sub.w $22(a6), d0
pub fn i_07039a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7039E;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07039E: cmpi.w #$0, d0
pub fn i_07039e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703A2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0703A2: bgt.w $703a8
pub fn i_0703a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703A6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x703A8; return 10; }
    8
}
/// $0703A6: neg.w d0
pub fn i_0703a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703A8;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0703A8: clr.w d1
pub fn i_0703a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703AA;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0703AA: cmpi.w #$20, d0
pub fn i_0703aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703AE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $0703AE: blt.w $703ce
pub fn i_0703ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703B2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x703CE; return 10; }
    8
}
/// $0703B2: move.w #$1, d1
pub fn i_0703b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0703B6: cmpi.w #$50, d0
pub fn i_0703b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703BA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x50, Size::Word, false);
    8
}
/// $0703BA: blt.w $703ce
pub fn i_0703ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703BE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x703CE; return 10; }
    8
}
/// $0703BE: move.w #$2, d1
pub fn i_0703be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703C2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0703C2: cmpi.w #$80, d0
pub fn i_0703c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703C6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $0703C6: blt.w $703ce
pub fn i_0703c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703CA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x703CE; return 10; }
    8
}
/// $0703CA: move.w #$3, d1
pub fn i_0703ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0703CE: addi.b #$c, d1
pub fn i_0703ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703D2;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0703D2: move.b d1, $5c(a6)
pub fn i_0703d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703D6;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0703D6: lea.l $29ba70.l, a0
pub fn i_0703d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BA70;
    20
}
/// $0703DC: jsr $28cd4.l
pub fn i_0703dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x703E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0703E2: lea.l $703e8(pc), a1
pub fn i_0703e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x703E8;
    16
}
/// $0703E6: move.l a1, (a6)
pub fn i_0703e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0703E8: jsr $70d1e(pc)
pub fn i_0703e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x703EC);
    cpu.pc = 0x70D1E;
    16
}
/// $0703EC: jsr $28d70.l
pub fn i_0703ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x703F2);
    cpu.pc = 0x28D70;
    16
}
/// $0703F2: bcc.w $70414
pub fn i_0703f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70414; return 10; }
    8
}
/// $0703F6: lea.l $7030c(pc), a1
pub fn i_0703f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7030C;
    16
}
/// $0703FA: move.l a1, (a6)
pub fn i_0703fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x703FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0703FC: movea.l $c(a6), a0
pub fn i_0703fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70400;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070400: subq.b #$1, $72(a0)
pub fn i_070400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70404;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $070404: cmpi.b #$0, $72(a0)
pub fn i_070404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7040A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07040A: bgt.w $70414
pub fn i_07040a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7040E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x70414; return 10; }
    8
}
/// $07040E: lea.l $70418(pc), a1
pub fn i_07040e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70412;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70418;
    16
}
/// $070412: move.l a1, (a6)
pub fn i_070412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70414;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070414: bra.w $70492
pub fn i_070414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70418;
    cpu.ir = 0x6000;
    cpu.pc = 0x70492;
    10
}
// ===== M5Boss_SoldierFire_070418  ($070418, 80 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070418: lea.l $29b816.l, a0
pub fn i_070418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7041E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B816;
    20
}
/// $07041E: jsr $28cd4.l
pub fn i_07041e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70424;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70424);
    cpu.pc = 0x28CD4;
    16
}
/// $070424: lea.l $7042a(pc), a1
pub fn i_070424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70428;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7042A;
    16
}
/// $070428: move.l a1, (a6)
pub fn i_070428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7042A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07042A: jsr $70d1e(pc)
pub fn i_07042a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7042E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7042E);
    cpu.pc = 0x70D1E;
    16
}
/// $07042E: jsr $28d70.l
pub fn i_07042e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70434);
    cpu.pc = 0x28D70;
    16
}
/// $070434: bra.w $70492
pub fn i_070434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70438;
    cpu.ir = 0x6000;
    cpu.pc = 0x70492;
    10
}
// ===== M5Boss_SoldierDie_070468  ($070468, 92 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070468: lea.l $29bd40.l, a0
pub fn i_070468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7046E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BD40;
    20
}
/// $07046E: jsr $28cd4.l
pub fn i_07046e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70474;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70474);
    cpu.pc = 0x28CD4;
    16
}
/// $070474: lea.l $7047a(pc), a1
pub fn i_070474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70478;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7047A;
    16
}
/// $070478: move.l a1, (a6)
pub fn i_070478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7047A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07047A: jsr $70d1e(pc)
pub fn i_07047a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7047E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7047E);
    cpu.pc = 0x70D1E;
    16
}
/// $07047E: jsr $28d70.l
pub fn i_07047e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70484;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70484);
    cpu.pc = 0x28D70;
    16
}
/// $070484: bcc.w $7048e
pub fn i_070484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70488;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7048E; return 10; }
    8
}
/// $070488: lea.l $70380(pc), a1
pub fn i_070488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7048C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70380;
    16
}
/// $07048C: move.l a1, (a6)
pub fn i_07048c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7048E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07048E: bra.w $70492
pub fn i_07048e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70492;
    cpu.ir = 0x6000;
    cpu.pc = 0x70492;
    10
}
/// $070492: jsr $2870a.l
pub fn i_070492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70498);
    cpu.pc = 0x2870A;
    16
}
/// $070498: bcc.w $704cc
pub fn i_070498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7049C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x704CC; return 10; }
    8
}
/// $07049C: movea.l $c(a6), a0
pub fn i_07049c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704A0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0704A0: move.b #$1, $8a(a0)
pub fn i_0704a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704A6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0704A6: move.w #$8000, d0
pub fn i_0704a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0704AA: jsr $28134.l
pub fn i_0704aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x704B0);
    cpu.pc = 0x28134;
    16
}
/// $0704B0: andi.w #$ffe3, $38(a6)
pub fn i_0704b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704B6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0704B6: ori.w #$1c, $38(a6)
pub fn i_0704b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704BC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0704BC: movea.l $c(a6), a0
pub fn i_0704bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704C0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0704C0: subq.b #$1, $72(a0)
pub fn i_0704c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704C4;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
// ===== JsrAbsThunk_0704c4  ($0704C4, 8 B, thunks/jsr_abs_thunks.c)
/// $0704C4: jsr $49fd0.l
pub fn i_0704c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x704CA);
    cpu.pc = 0x49FD0;
    16
}
/// $0704CA: rts
pub fn i_0704ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SoldierWaitParent_0704cc  ($0704CC, 30 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0704CC: movea.l $c(a6), a0
pub fn i_0704cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704D0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0704D0: cmpi.b #$2, $8a(a0)
pub fn i_0704d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704D6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0704D6: bne.w $704e0
pub fn i_0704d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x704E0; return 10; }
    8
}
/// $0704DA: lea.l $704f2(pc), a1
pub fn i_0704da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x704F2;
    16
}
/// $0704DE: move.l a1, (a6)
pub fn i_0704de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0704E0: jsr $5e45a.l
pub fn i_0704e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x704E6);
    cpu.pc = 0x5E45A;
    16
}
/// $0704E6: bcc.w $704f0
pub fn i_0704e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x704F0; return 10; }
    8
}
// ===== SetTaskHandler_0704ea  ($0704EA, 8 B, kernel/task_handlers.c)
/// $0704EA: lea.l $704f2(pc), a1
pub fn i_0704ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x704F2;
    16
}
/// $0704EE: move.l a1, (a6)
pub fn i_0704ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0704F0: rts
pub fn i_0704f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0704f2  ($0704F2, 8 B, kernel/jmp_to_scheduler.c)
/// $0704F2: jmp $518.l
pub fn i_0704f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704F8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M5Boss_Grenadier_0704fa  ($0704FA, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0704FA: move.w #$98, d1
pub fn i_0704fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x98;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0704FE: jsr $236e.l
pub fn i_0704fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70504);
    cpu.pc = 0x236E;
    16
}
/// $070504: move.w #$2, $76(a6)
pub fn i_070504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7050A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== M5Boss_GrenadierAcquire_07050a  ($07050A, 26 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07050A: jsr $5e1ea.l
pub fn i_07050a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70510);
    cpu.pc = 0x5E1EA;
    16
}
/// $070510: move.l a0, $9c(a6)
pub fn i_070510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70514;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $070514: lea.l $2b9ecc.l, a0
pub fn i_070514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7051A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9ECC;
    20
}
/// $07051A: jsr $799de.l
pub fn i_07051a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70520;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70520);
    cpu.pc = 0x799DE;
    16
}
/// $070520: move.w d0, $70(a6)
pub fn i_070520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70524;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== M5Boss_GrenadierIdle_070524  ($070524, 142 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070524: lea.l $2d83ae.l, a0
pub fn i_070524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7052A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D83AE;
    20
}
/// $07052A: jsr $28cd4.l
pub fn i_07052a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70530);
    cpu.pc = 0x28CD4;
    16
}
/// $070530: lea.l $70536(pc), a1
pub fn i_070530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70534;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70536;
    16
}
/// $070534: move.l a1, (a6)
pub fn i_070534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70536;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070536: jsr $70d7c(pc)
pub fn i_070536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7053A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7053A);
    cpu.pc = 0x70D7C;
    16
}
/// $07053A: jsr $28d70.l
pub fn i_07053a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70540);
    cpu.pc = 0x28D70;
    16
}
/// $070540: tst.b $9a(a6)
pub fn i_070540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70544;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $070544: bne.w $7056c
pub fn i_070544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70548;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7056C; return 10; }
    8
}
/// $070548: subq.w #$1, $70(a6)
pub fn i_070548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7054C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07054C: cmpi.w #$0, $70(a6)
pub fn i_07054c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70552;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $070552: bgt.w $7056c
pub fn i_070552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70556;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7056C; return 10; }
    8
}
/// $070556: lea.l $705b2(pc), a1
pub fn i_070556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7055A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x705B2;
    16
}
/// $07055A: move.l a1, (a6)
pub fn i_07055a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7055C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07055C: cmpi.b #$0, $72(a6)
pub fn i_07055c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70562;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $070562: bgt.w $7056c
pub fn i_070562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70566;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7056C; return 10; }
    8
}
/// $070566: lea.l $705c2(pc), a1
pub fn i_070566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7056A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x705C2;
    16
}
/// $07056A: move.l a1, (a6)
pub fn i_07056a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7056C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07056C: movea.l $9c(a6), a0
pub fn i_07056c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70570;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070570: move.w $22(a0), d0
pub fn i_070570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70574;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070574: move.w $24(a0), d1
pub fn i_070574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70578;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070578: sub.w $22(a6), d0
pub fn i_070578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7057C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07057C: sub.w $24(a6), d1
pub fn i_07057c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70580;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $070580: addi.w #$20, d0
pub fn i_070580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70584;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070584: jsr $5e018.l
pub fn i_070584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7058A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7058A);
    cpu.pc = 0x5E018;
    16
}
/// $07058A: jsr $4206a.l
pub fn i_07058a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70590);
    cpu.pc = 0x4206A;
    16
}
/// $070590: move.w d0, $78(a6)
pub fn i_070590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70594;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070594: move.w #$ffff, d1
pub fn i_070594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70598;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070598: move.w $78(a6), d0
pub fn i_070598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7059C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07059C: cmp.w $76(a6), d0
pub fn i_07059c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705A0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0705A0: beq.w $705ae
pub fn i_0705a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x705AE; return 10; }
    8
}
/// $0705A4: bcs.w $705aa
pub fn i_0705a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x705AA; return 10; }
    8
}
/// $0705A8: neg.w d1
pub fn i_0705a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705AA;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0705AA: add.w d1, $76(a6)
pub fn i_0705aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705AE;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0705AE: bra.w $707d0
pub fn i_0705ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x707D0;
    10
}
// ===== M5Boss_GrenadierCount_0705b2  ($0705B2, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0705B2: lea.l $2b9f4e.l, a0
pub fn i_0705b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9F4E;
    20
}
/// $0705B8: jsr $799de.l
pub fn i_0705b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x705BE);
    cpu.pc = 0x799DE;
    16
}
/// $0705BE: move.b d0, $72(a6)
pub fn i_0705be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705C2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== M5Boss_GrenadierThrow_0705c2  ($0705C2, 118 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0705C2: jsr $5e1ea.l
pub fn i_0705c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x705C8);
    cpu.pc = 0x5E1EA;
    16
}
/// $0705C8: move.l a0, $9c(a6)
pub fn i_0705c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705CC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0705CC: lea.l $2b9fd0.l, a0
pub fn i_0705cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9FD0;
    20
}
/// $0705D2: jsr $799de.l
pub fn i_0705d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x705D8);
    cpu.pc = 0x799DE;
    16
}
/// $0705D8: move.w d0, $70(a6)
pub fn i_0705d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0705DC: move.w $76(a6), d0
pub fn i_0705dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705E0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0705E0: lsr.w #$1, d0
pub fn i_0705e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705E2;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0705E2: movea.l #$2d5402, a0
pub fn i_0705e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705E8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D5402;
    cpu.a[0] = v1;
    4
}
/// $0705E8: lsl.w #$2, d0
pub fn i_0705e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705EA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0705EA: movea.l (a0, d0.w), a0
pub fn i_0705ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705EE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0705EE: cmpa.l #$ffffffff, a0
pub fn i_0705ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705F4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0705F4: beq.w $705fe
pub fn i_0705f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x705FE; return 10; }
    8
}
/// $0705F8: jsr $28cd4.l
pub fn i_0705f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x705FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x705FE);
    cpu.pc = 0x28CD4;
    16
}
/// $0705FE: lea.l $70604(pc), a1
pub fn i_0705fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70602;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70604;
    16
}
/// $070602: move.l a1, (a6)
pub fn i_070602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70604;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070604: jsr $70d7c(pc)
pub fn i_070604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70608;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x70608);
    cpu.pc = 0x70D7C;
    16
}
/// $070608: jsr $28d70.l
pub fn i_070608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7060E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7060E);
    cpu.pc = 0x28D70;
    16
}
/// $07060E: bcc.w $70634
pub fn i_07060e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70612;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70634; return 10; }
    8
}
/// $070612: lea.l $70524(pc), a1
pub fn i_070612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70616;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70524;
    16
}
/// $070616: move.l a1, (a6)
pub fn i_070616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70618;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070618: subq.b #$1, $72(a6)
pub fn i_070618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7061C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $07061C: cmpi.b #$0, $72(a6)
pub fn i_07061c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70622;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $070622: bgt.w $70634
pub fn i_070622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70626;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x70634; return 10; }
    8
}
/// $070626: lea.l $7050a(pc), a1
pub fn i_070626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7062A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7050A;
    16
}
/// $07062A: move.l a1, (a6)
pub fn i_07062a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7062C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07062C: movea.l $c(a6), a0
pub fn i_07062c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70630;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070630: clr.b $72(a0)
pub fn i_070630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70634;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $070634: bra.w $70816
pub fn i_070634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70638;
    cpu.ir = 0x6000;
    cpu.pc = 0x70816;
    10
}
// ===== M5Boss_GrenadierHurt_070638  ($070638, 84 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070638: subi.w #$1e, $70(a6)
pub fn i_070638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7063E;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07063E: lea.l $2d8714.l, a0
pub fn i_07063e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70644;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D8714;
    20
}
/// $070644: jsr $28cd4.l
pub fn i_070644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7064A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7064A);
    cpu.pc = 0x28CD4;
    16
}
/// $07064A: lea.l $70650(pc), a1
pub fn i_07064a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7064E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70650;
    16
}
/// $07064E: move.l a1, (a6)
pub fn i_07064e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70650;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070650: jsr $70d7c(pc)
pub fn i_070650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70654;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x70654);
    cpu.pc = 0x70D7C;
    16
}
/// $070654: jsr $28d70.l
pub fn i_070654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7065A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7065A);
    cpu.pc = 0x28D70;
    16
}
/// $07065A: bcc.w $7066a
pub fn i_07065a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7065E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7066A; return 10; }
    8
}
/// $07065E: bclr.b #$3, $13(a6)
pub fn i_07065e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70664;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070664: lea.l $70524(pc), a1
pub fn i_070664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70668;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70524;
    16
}
/// $070668: move.l a1, (a6)
pub fn i_070668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7066A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07066A: movea.l $c(a6), a0
pub fn i_07066a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7066E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07066E: cmpi.b #$2, $8a(a0)
pub fn i_07066e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70674;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $070674: bne.w $7067e
pub fn i_070674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70678;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7067E; return 10; }
    8
}
/// $070678: lea.l $704f2(pc), a1
pub fn i_070678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7067C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x704F2;
    16
}
/// $07067C: move.l a1, (a6)
pub fn i_07067c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7067E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07067E: movea.l $c(a6), a0
pub fn i_07067e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70682;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070682: btst.b #$0, $13(a0)
pub fn i_070682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70688;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $070688: beq.w $70692
pub fn i_070688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7068C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x70692; return 10; }
    8
}
// ===== SetTaskHandler_07068c  ($07068C, 8 B, kernel/task_handlers.c)
/// $07068C: lea.l $70694(pc), a1
pub fn i_07068c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70690;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70694;
    16
}
/// $070690: move.l a1, (a6)
pub fn i_070690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70692;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070692: rts
pub fn i_070692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70694;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Grenade_070694  ($070694, 160 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070694: clr.b $21(a6)
pub fn i_070694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70698;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $070698: move.w #$d000, d0
pub fn i_070698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7069C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07069C: jsr $28134.l
pub fn i_07069c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x706A2);
    cpu.pc = 0x28134;
    16
}
/// $0706A2: andi.w #$ffe3, $38(a6)
pub fn i_0706a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706A8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0706A8: ori.w #$18, $38(a6)
pub fn i_0706a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706AE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0706AE: move.l #$ffffffff, $48(a6)
pub fn i_0706ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706B6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0706B6: move.w #$f800, $28(a6)
pub fn i_0706b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0706BC: move.w #$40, $2c(a6)
pub fn i_0706bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0706C2: move.w #$200, $2a(a6)
pub fn i_0706c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0706C8: move.w #$ffc0, $2e(a6)
pub fn i_0706c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0706CE: move.w #$98, d1
pub fn i_0706ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x98;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0706D2: jsr $236e.l
pub fn i_0706d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x706D8);
    cpu.pc = 0x236E;
    16
}
/// $0706D8: bset.b #$6, $12(a6)
pub fn i_0706d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0706DE: lea.l $6e820.l, a1
pub fn i_0706de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x6E820;
    20
}
/// $0706E4: jsr $4ae.l
pub fn i_0706e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x706EA);
    cpu.pc = 0x4AE;
    16
}
/// $0706EA: jsr $5dd02.l
pub fn i_0706ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x706F0);
    cpu.pc = 0x5DD02;
    16
}
/// $0706F0: lea.l $2d875c.l, a0
pub fn i_0706f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D875C;
    20
}
/// $0706F6: jsr $28cd4.l
pub fn i_0706f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x706FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x706FC);
    cpu.pc = 0x28CD4;
    16
}
/// $0706FC: lea.l $70702(pc), a1
pub fn i_0706fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70700;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70702;
    16
}
/// $070700: move.l a1, (a6)
pub fn i_070700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70702;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070702: bset.b #$6, $13(a6)
pub fn i_070702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70708;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070708: jsr $27d50.l
pub fn i_070708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7070E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7070E);
    cpu.pc = 0x27D50;
    16
}
/// $07070E: bcc.w $70718
pub fn i_07070e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70712;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70718; return 10; }
    8
}
/// $070712: lea.l $7073c(pc), a1
pub fn i_070712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70716;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7073C;
    16
}
/// $070716: move.l a1, (a6)
pub fn i_070716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70718;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070718: jsr $28d70.l
pub fn i_070718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7071E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7071E);
    cpu.pc = 0x28D70;
    16
}
/// $07071E: cmpi.w #$ff80, $28(a6)
pub fn i_07071e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70724;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $070724: blt.w $7072c
pub fn i_070724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70728;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7072C; return 10; }
    8
}
/// $070728: clr.w $2c(a6)
pub fn i_070728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7072C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07072C: tst.b $21(a6)
pub fn i_07072c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70730;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $070730: beq.w $7073a
pub fn i_070730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70734;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7073A; return 10; }
    8
}
// ===== SetTaskHandler_070734  ($070734, 8 B, kernel/task_handlers.c)
/// $070734: lea.l $7079e(pc), a1
pub fn i_070734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70738;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7079E;
    16
}
/// $070738: move.l a1, (a6)
pub fn i_070738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7073A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07073A: rts
pub fn i_07073a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7073C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_GrenadeBounce_07073c  ($07073C, 90 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07073C: jsr $267e2.l
pub fn i_07073c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70742);
    cpu.pc = 0x267E2;
    16
}
/// $070742: move.w #$21c, $2a(a6)
pub fn i_070742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70748;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x21C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070748: move.w #$ffdc, $2e(a6)
pub fn i_070748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7074E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFDC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07074E: lea.l $28732c.l, a0
pub fn i_07074e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70754;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28732C;
    20
}
/// $070754: jsr $28cd4.l
pub fn i_070754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7075A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7075A);
    cpu.pc = 0x28CD4;
    16
}
/// $07075A: lea.l $70760(pc), a1
pub fn i_07075a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7075E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70760;
    16
}
/// $07075E: move.l a1, (a6)
pub fn i_07075e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70760;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070760: jsr $27d50.l
pub fn i_070760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70766;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70766);
    cpu.pc = 0x27D50;
    16
}
/// $070766: bcc.w $70788
pub fn i_070766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7076A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70788; return 10; }
    8
}
/// $07076A: clr.b $106ed2.l
pub fn i_07076a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70770;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $070770: lea.l $28735e.l, a0
pub fn i_070770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70776;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28735E;
    20
}
/// $070776: jsr $28cd4.l
pub fn i_070776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7077C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7077C);
    cpu.pc = 0x28CD4;
    16
}
/// $07077C: lea.l $70782(pc), a1
pub fn i_07077c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70780;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70782;
    16
}
/// $070780: move.l a1, (a6)
pub fn i_070780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70782;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070782: jsr $2783a.l
pub fn i_070782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70788);
    cpu.pc = 0x2783A;
    16
}
/// $070788: jsr $28d70.l
pub fn i_070788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7078E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7078E);
    cpu.pc = 0x28D70;
    16
}
/// $07078E: tst.b $21(a6)
pub fn i_07078e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70792;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $070792: beq.w $7079c
pub fn i_070792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70796;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7079C; return 10; }
    8
}
// ===== SetTaskHandler_070796  ($070796, 8 B, kernel/task_handlers.c)
/// $070796: lea.l $7079e(pc), a1
pub fn i_070796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7079A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7079E;
    16
}
/// $07079A: move.l a1, (a6)
pub fn i_07079a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7079C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07079C: rts
pub fn i_07079c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7079E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_GrenadeRest_07079e  ($07079E, 34 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07079E: clr.b $106ed2.l
pub fn i_07079e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707A4;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $0707A4: jsr $267e2.l
pub fn i_0707a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x707AA);
    cpu.pc = 0x267E2;
    16
}
/// $0707AA: lea.l $707b0(pc), a1
pub fn i_0707aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x707B0;
    16
}
/// $0707AE: move.l a1, (a6)
pub fn i_0707ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0707B0: jsr $2783a.l
pub fn i_0707b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x707B6);
    cpu.pc = 0x2783A;
    16
}
/// $0707B6: cmpi.b #$1, $21(a6)
pub fn i_0707b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707BC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0707BC: beq.w $707c6
pub fn i_0707bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x707C6; return 10; }
    8
}
// ===== SetTaskHandler_0707c0  ($0707C0, 8 B, kernel/task_handlers.c)
/// $0707C0: lea.l $707c8(pc), a1
pub fn i_0707c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x707C8;
    16
}
/// $0707C4: move.l a1, (a6)
pub fn i_0707c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0707C6: rts
pub fn i_0707c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0707c8  ($0707C8, 8 B, kernel/jmp_to_scheduler.c)
/// $0707C8: jmp $518.l
pub fn i_0707c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707CE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M5Boss_GrenadeHit_0707d0  ($0707D0, 62 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0707D0: movea.l $c(a6), a0
pub fn i_0707d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707D4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0707D4: cmpi.b #$2, $8a(a0)
pub fn i_0707d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707DA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0707DA: bne.w $707e4
pub fn i_0707da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x707E4; return 10; }
    8
}
/// $0707DE: lea.l $704f2(pc), a1
pub fn i_0707de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x704F2;
    16
}
/// $0707E2: move.l a1, (a6)
pub fn i_0707e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0707E4: jsr $2870a.l
pub fn i_0707e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x707EA);
    cpu.pc = 0x2870A;
    16
}
/// $0707EA: bcc.w $70800
pub fn i_0707ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70800; return 10; }
    8
}
/// $0707EE: bclr.b #$0, $13(a6)
pub fn i_0707ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0707F4: move.w #$1, $66(a6)
pub fn i_0707f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0707FA: lea.l $70638(pc), a1
pub fn i_0707fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x707FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70638;
    16
}
/// $0707FE: move.l a1, (a6)
pub fn i_0707fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70800;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070800: movea.l $c(a6), a0
pub fn i_070800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70804;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070804: btst.b #$0, $13(a0)
pub fn i_070804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7080A;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07080A: beq.w $70814
pub fn i_07080a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7080E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x70814; return 10; }
    8
}
// ===== SetTaskHandler_07080e  ($07080E, 8 B, kernel/task_handlers.c)
/// $07080E: lea.l $70694(pc), a1
pub fn i_07080e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70812;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70694;
    16
}
/// $070812: move.l a1, (a6)
pub fn i_070812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70814;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070814: rts
pub fn i_070814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70816;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_GrenadeHitB_070816  ($070816, 62 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070816: movea.l $c(a6), a0
pub fn i_070816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7081A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07081A: cmpi.b #$2, $8a(a0)
pub fn i_07081a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70820;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $070820: bne.w $7082a
pub fn i_070820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70824;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7082A; return 10; }
    8
}
/// $070824: lea.l $704f2(pc), a1
pub fn i_070824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70828;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x704F2;
    16
}
/// $070828: move.l a1, (a6)
pub fn i_070828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7082A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07082A: jsr $2870a.l
pub fn i_07082a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70830);
    cpu.pc = 0x2870A;
    16
}
/// $070830: bcc.w $70846
pub fn i_070830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70834;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70846; return 10; }
    8
}
/// $070834: bclr.b #$3, $13(a6)
pub fn i_070834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7083A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07083A: bclr.b #$0, $13(a6)
pub fn i_07083a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70840;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070840: move.w #$1, $66(a6)
pub fn i_070840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70846;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070846: movea.l $c(a6), a0
pub fn i_070846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7084A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07084A: btst.b #$0, $13(a0)
pub fn i_07084a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70850;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $070850: beq.w $7085a
pub fn i_070850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70854;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7085A; return 10; }
    8
}
// ===== SetTaskHandler_070854  ($070854, 8 B, kernel/task_handlers.c)
/// $070854: lea.l $70694(pc), a1
pub fn i_070854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70858;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70694;
    16
}
/// $070858: move.l a1, (a6)
pub fn i_070858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7085A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07085A: rts
pub fn i_07085a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7085C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_GrenadeExplode_07085c  ($07085C, 28 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07085C: move.w #$1022, d0
pub fn i_07085c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70860;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070860: jsr $2352.l
pub fn i_070860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70866);
    cpu.pc = 0x2352;
    16
}
/// $070866: jsr $13600.l
pub fn i_070866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7086C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7086C);
    cpu.pc = 0x13600;
    16
}
/// $07086C: move.l #$ffffffff, $48(a6)
pub fn i_07086c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70874;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $070874: bra.w $77f6a
pub fn i_070874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70878;
    cpu.ir = 0x6000;
    cpu.pc = 0x77F6A;
    10
}
// ===== M5Boss_Fx2D8770_070878  ($070878, 50 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070878: lea.l $2d8770.l, a0
pub fn i_070878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7087E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D8770;
    20
}
/// $07087E: jsr $28cd4.l
pub fn i_07087e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70884);
    cpu.pc = 0x28CD4;
    16
}
/// $070884: move.w #$4, d1
pub fn i_070884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70888;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070888: jsr $236e.l
pub fn i_070888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7088E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7088E);
    cpu.pc = 0x236E;
    16
}
/// $07088E: lea.l $70894(pc), a1
pub fn i_07088e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70892;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70894;
    16
}
/// $070892: move.l a1, (a6)
pub fn i_070892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70894;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070894: jsr $5e506.l
pub fn i_070894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7089A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7089A);
    cpu.pc = 0x5E506;
    16
}
/// $07089A: addi.w #$2c, $24(a6)
pub fn i_07089a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708A0;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0708A0: jsr $28d70.l
pub fn i_0708a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x708A6);
    cpu.pc = 0x28D70;
    16
}
/// $0708A6: bcc.w $708b0
pub fn i_0708a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x708B0; return 10; }
    8
}
// ===== SetTaskHandler_0708aa  ($0708AA, 8 B, kernel/task_handlers.c)
/// $0708AA: lea.l $6fb02(pc), a1
pub fn i_0708aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $0708AE: move.l a1, (a6)
pub fn i_0708ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0708B0: rts
pub fn i_0708b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Fx2D87AE_0708b2  ($0708B2, 14 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0708B2: lea.l $2d87ae.l, a0
pub fn i_0708b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D87AE;
    20
}
/// $0708B8: jsr $28cd4.l
pub fn i_0708b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x708BE);
    cpu.pc = 0x28CD4;
    16
}
/// $0708BE: bra.b $70884
pub fn i_0708be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708C0;
    cpu.ir = 0x60C4;
    cpu.pc = 0x70884;
    10
}
// ===== M5Boss_Fx2D87D8_0708c0  ($0708C0, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0708C0: move.w #$2, d1
pub fn i_0708c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708C4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0708C4: jsr $236e.l
pub fn i_0708c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x708CA);
    cpu.pc = 0x236E;
    16
}
/// $0708CA: lea.l $2d87d8.l, a0
pub fn i_0708ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D87D8;
    20
}
/// $0708D0: jsr $28cd4.l
pub fn i_0708d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x708D6);
    cpu.pc = 0x28CD4;
    16
}
/// $0708D6: lea.l $708dc(pc), a1
pub fn i_0708d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x708DC;
    16
}
/// $0708DA: move.l a1, (a6)
pub fn i_0708da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0708DC: jsr $5e506.l
pub fn i_0708dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x708E2);
    cpu.pc = 0x5E506;
    16
}
/// $0708E2: jsr $28d70.l
pub fn i_0708e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x708E8);
    cpu.pc = 0x28D70;
    16
}
/// $0708E8: bcc.w $708f2
pub fn i_0708e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x708F2; return 10; }
    8
}
// ===== SetTaskHandler_0708ec  ($0708EC, 8 B, kernel/task_handlers.c)
/// $0708EC: lea.l $6fb02(pc), a1
pub fn i_0708ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $0708F0: move.l a1, (a6)
pub fn i_0708f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0708F2: rts
pub fn i_0708f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Fx2DD8B6_0708f4  ($0708F4, 38 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0708F4: move.w #$4, d1
pub fn i_0708f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708F8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0708F8: jsr $236e.l
pub fn i_0708f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x708FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x708FE);
    cpu.pc = 0x236E;
    16
}
/// $0708FE: lea.l $2dd8b6.l, a0
pub fn i_0708fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70904;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD8B6;
    20
}
/// $070904: jsr $28cd4.l
pub fn i_070904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7090A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7090A);
    cpu.pc = 0x28CD4;
    16
}
/// $07090A: lea.l $70910(pc), a1
pub fn i_07090a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7090E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70910;
    16
}
/// $07090E: move.l a1, (a6)
pub fn i_07090e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70910;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070910: jsr $28d70.l
pub fn i_070910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70916;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70916);
    cpu.pc = 0x28D70;
    16
}
/// $070916: bcc.w $70920
pub fn i_070916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7091A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70920; return 10; }
    8
}
// ===== SetTaskHandler_07091a  ($07091A, 8 B, kernel/task_handlers.c)
/// $07091A: lea.l $6fb02(pc), a1
pub fn i_07091a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7091E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $07091E: move.l a1, (a6)
pub fn i_07091e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70920;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070920: rts
pub fn i_070920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70922;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Fx2DD944_070922  ($070922, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070922: move.w #$4, d1
pub fn i_070922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70926;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070926: jsr $236e.l
pub fn i_070926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7092C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7092C);
    cpu.pc = 0x236E;
    16
}
/// $07092C: lea.l $2dd944.l, a0
pub fn i_07092c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70932;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD944;
    20
}
/// $070932: jsr $28cd4.l
pub fn i_070932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70938;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70938);
    cpu.pc = 0x28CD4;
    16
}
/// $070938: lea.l $7093e(pc), a1
pub fn i_070938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7093C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7093E;
    16
}
/// $07093C: move.l a1, (a6)
pub fn i_07093c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7093E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07093E: jsr $2783a.l
pub fn i_07093e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70944;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70944);
    cpu.pc = 0x2783A;
    16
}
/// $070944: jsr $28d70.l
pub fn i_070944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7094A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7094A);
    cpu.pc = 0x28D70;
    16
}
/// $07094A: bcc.w $70954
pub fn i_07094a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7094E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70954; return 10; }
    8
}
// ===== SetTaskHandler_07094e  ($07094E, 8 B, kernel/task_handlers.c)
/// $07094E: lea.l $6fb02(pc), a1
pub fn i_07094e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70952;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $070952: move.l a1, (a6)
pub fn i_070952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70954;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070954: rts
pub fn i_070954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70956;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Rocket_070956  ($070956, 212 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070956: eori.b #$1, $3a(a6)
pub fn i_070956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7095C;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07095C: move.w #$fe00, $28(a6)
pub fn i_07095c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70962;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070962: move.w #$1e, $66(a6)
pub fn i_070962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70968;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070968: move.w #$1de, d1
pub fn i_070968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7096C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1DE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07096C: jsr $236e.l
pub fn i_07096c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70972);
    cpu.pc = 0x236E;
    16
}
/// $070972: move.w #$173, d1
pub fn i_070972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70976;
    cpu.ir = 0x323C;
    let v1: u32 = 0x173;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070976: jsr $236e.l
pub fn i_070976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7097C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7097C);
    cpu.pc = 0x236E;
    16
}
/// $07097C: bset.b #$4, $6b(a6)
pub fn i_07097c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70982;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070982: move.w #$d000, d0
pub fn i_070982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70986;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070986: jsr $28134.l
pub fn i_070986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7098C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7098C);
    cpu.pc = 0x28134;
    16
}
/// $07098C: andi.w #$ffe3, $38(a6)
pub fn i_07098c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70992;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $070992: ori.w #$14, $38(a6)
pub fn i_070992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70998;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $070998: lea.l $2d883c.l, a0
pub fn i_070998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7099E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D883C;
    20
}
/// $07099E: jsr $28cd4.l
pub fn i_07099e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x709A4);
    cpu.pc = 0x28CD4;
    16
}
/// $0709A4: move.w #$ffc0, $2e(a6)
pub fn i_0709a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0709AA: lea.l $709b0(pc), a1
pub fn i_0709aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x709B0;
    16
}
/// $0709AE: move.l a1, (a6)
pub fn i_0709ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0709B0: bset.b #$6, $13(a6)
pub fn i_0709b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709B6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0709B6: jsr $27bc8.l
pub fn i_0709b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x709BC);
    cpu.pc = 0x27BC8;
    16
}
/// $0709BC: bcc.w $709c6
pub fn i_0709bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x709C6; return 10; }
    8
}
/// $0709C0: lea.l $70a64(pc), a1
pub fn i_0709c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70A64;
    16
}
/// $0709C4: move.l a1, (a6)
pub fn i_0709c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0709C6: btst.b #$1, $106f28.l
pub fn i_0709c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709CE;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((1) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0709CE: bne.w $709da
pub fn i_0709ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x709DA; return 10; }
    8
}
/// $0709D2: move.w $16(a6), d0
pub fn i_0709d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709D6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0709D6: bra.w $709de
pub fn i_0709d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x709DE;
    10
}
/// $0709DA: move.w $18(a6), d0
pub fn i_0709da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0709DE: move.w d0, $14(a6)
pub fn i_0709de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709E2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0709E2: jsr $28d70.l
pub fn i_0709e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x709E8);
    cpu.pc = 0x28D70;
    16
}
/// $0709E8: jsr $283d8.l
pub fn i_0709e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x709EE);
    cpu.pc = 0x283D8;
    16
}
/// $0709EE: btst.b #$1, $13(a6)
pub fn i_0709ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709F4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0709F4: beq.w $709fe
pub fn i_0709f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x709FE; return 10; }
    8
}
/// $0709F8: lea.l $7085c(pc), a1
pub fn i_0709f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7085C;
    16
}
/// $0709FC: move.l a1, (a6)
pub fn i_0709fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x709FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0709FE: jsr $2870a.l
pub fn i_0709fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70A04);
    cpu.pc = 0x2870A;
    16
}
/// $070A04: bclr.b #$3, $13(a6)
pub fn i_070a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A0A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070A0A: jsr $28758.l
pub fn i_070a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70A10);
    cpu.pc = 0x28758;
    16
}
/// $070A10: bcc.w $70a1a
pub fn i_070a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70A1A; return 10; }
    8
}
/// $070A14: lea.l $7085c(pc), a1
pub fn i_070a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7085C;
    16
}
/// $070A18: move.l a1, (a6)
pub fn i_070a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070A1A: movea.l #$ffffffff, a0
pub fn i_070a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A20;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $070A20: jsr $5dd5c.l
pub fn i_070a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70A26);
    cpu.pc = 0x5DD5C;
    16
}
/// $070A26: bcc.w $70a30
pub fn i_070a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70A30; return 10; }
    8
}
// ===== SetTaskHandler_070a2a  ($070A2A, 8 B, kernel/task_handlers.c)
/// $070A2A: lea.l $6fb02(pc), a1
pub fn i_070a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $070A2E: move.l a1, (a6)
pub fn i_070a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070A30: rts
pub fn i_070a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_RocketGround_070a32  ($070A32, 50 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070A32: clr.w $2e(a6)
pub fn i_070a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A36;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $070A36: clr.w $2a(a6)
pub fn i_070a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A3A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $070A3A: lea.l $70a40(pc), a1
pub fn i_070a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70A40;
    16
}
/// $070A3E: move.l a1, (a6)
pub fn i_070a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070A40: bset.b #$6, $13(a6)
pub fn i_070a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A46;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070A46: jsr $27eba.l
pub fn i_070a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70A4C);
    cpu.pc = 0x27EBA;
    16
}
/// $070A4C: bcc.w $70a5a
pub fn i_070a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70A5A; return 10; }
    8
}
/// $070A50: jsr $27a92.l
pub fn i_070a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70A56);
    cpu.pc = 0x27A92;
    16
}
/// $070A56: bra.w $70a60
pub fn i_070a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x70A60;
    10
}
/// $070A5A: jsr $27c8c.l
pub fn i_070a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70A60);
    cpu.pc = 0x27C8C;
    16
}
/// $070A60: bra.w $709c6
pub fn i_070a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A64;
    cpu.ir = 0x6000;
    cpu.pc = 0x709C6;
    10
}
// ===== M5Boss_RocketJitter_070a64  ($070A64, 76 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070A64: jsr $5e9b6.l
pub fn i_070a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70A6A);
    cpu.pc = 0x5E9B6;
    16
}
/// $070A6A: andi.w #$700, d0
pub fn i_070a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A6E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070A6E: subi.w #$280, d0
pub fn i_070a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A72;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x280, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070A72: neg.w d0
pub fn i_070a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A74;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $070A74: move.w d0, $28(a6)
pub fn i_070a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A78;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070A78: jsr $5e9b6.l
pub fn i_070a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70A7E);
    cpu.pc = 0x5E9B6;
    16
}
/// $070A7E: andi.w #$700, d0
pub fn i_070a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A82;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070A82: addi.w #$400, d0
pub fn i_070a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A86;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x400, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070A86: move.w d0, $2a(a6)
pub fn i_070a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A8A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070A8A: move.w #$ffc0, $2e(a6)
pub fn i_070a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A90;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070A90: lea.l $70a96(pc), a1
pub fn i_070a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70A96;
    16
}
/// $070A94: move.l a1, (a6)
pub fn i_070a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070A96: bset.b #$6, $13(a6)
pub fn i_070a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A9C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070A9C: jsr $27bc8.l
pub fn i_070a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70AA2);
    cpu.pc = 0x27BC8;
    16
}
/// $070AA2: bcc.w $70aac
pub fn i_070aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70AAC; return 10; }
    8
}
/// $070AA6: lea.l $7085c(pc), a1
pub fn i_070aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7085C;
    16
}
/// $070AAA: move.l a1, (a6)
pub fn i_070aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $070AAC: bra.w $709c6
pub fn i_070aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AB0;
    cpu.ir = 0x6000;
    cpu.pc = 0x709C6;
    10
}
// ===== M5Boss_AnimStep_070ab0  ($070AB0, 22 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070AB0: addi.w #$2c, $24(a6)
pub fn i_070ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AB6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $070AB6: jsr $28d70.l
pub fn i_070ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70ABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70ABC);
    cpu.pc = 0x28D70;
    16
}
/// $070ABC: bcs.w $70acc
pub fn i_070abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AC0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x70ACC; return 10; }
    8
}
/// $070AC0: subi.w #$2c, $24(a6)
pub fn i_070ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AC6;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== ClearXN_070ac6  ($070AC6, 6 B, thunks/ccr_helpers.c)
/// $070AC6: andi.b #$ee, ccr
pub fn i_070ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70ACA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $070ACA: rts
pub fn i_070aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70ACC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SubY2C_070acc  ($070ACC, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070ACC: subi.w #$2c, $24(a6)
pub fn i_070acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AD2;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetXN_070ad2  ($070AD2, 6 B, thunks/ccr_helpers.c)
/// $070AD2: ori.b #$11, ccr
pub fn i_070ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AD6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $070AD6: rts
pub fn i_070ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_DispatchBy8C_070ad8  ($070AD8, 22 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070AD8: move.b $8c(a6), d0
pub fn i_070ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70ADC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $070ADC: andi.w #$7, d0
pub fn i_070adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AE0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070AE0: lsl.w #$2, d0
pub fn i_070ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AE2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $070AE2: lea.l $2d541a.l, a0
pub fn i_070ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D541A;
    20
}
/// $070AE8: movea.l (a0, d0.w), a1
pub fn i_070ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AEC;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $070AEC: jmp (a1)
pub fn i_070aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AEE;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
// ===== M5Boss_MoveByAngle_070aee  ($070AEE, 114 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070AEE: move.w #$10, d3
pub fn i_070aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AF2;
    cpu.ir = 0x363C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070AF2: move.w $80(a6), d0
pub fn i_070af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AF6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070AF6: move.w $82(a6), d1
pub fn i_070af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AFA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070AFA: sub.w $22(a6), d0
pub fn i_070afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70AFE;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $070AFE: sub.w $24(a6), d1
pub fn i_070afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B02;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $070B02: move.w d0, d2
pub fn i_070b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B04;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070B04: add.w d1, d2
pub fn i_070b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B06;
    cpu.ir = 0xD441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $070B06: cmpi.w #$0, d2
pub fn i_070b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B0A;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $070B0A: bge.w $70b10
pub fn i_070b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B0E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x70B10; return 10; }
    8
}
/// $070B0E: neg.w d2
pub fn i_070b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B10;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $070B10: cmp.w d3, d2
pub fn i_070b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B12;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $070B12: ble.w $70b66
pub fn i_070b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B16;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x70B66; return 10; }
    8
}
/// $070B16: jsr $5e018.l
pub fn i_070b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70B1C);
    cpu.pc = 0x5E018;
    16
}
/// $070B1C: add.w d0, d0
pub fn i_070b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B1E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $070B1E: lea.l $2c072c.l, a1
pub fn i_070b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B24;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $070B24: lea.l $2c07ac.l, a2
pub fn i_070b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B2A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C07AC;
    20
}
/// $070B2A: move.w (a1, d0.w), d1
pub fn i_070b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B2E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $070B2E: move.w (a2, d0.w), d2
pub fn i_070b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B32;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $070B32: move.w $36(a6), d0
pub fn i_070b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B36;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070B36: muls.w d0, d1
pub fn i_070b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B38;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $070B38: muls.w d0, d2
pub fn i_070b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B3A;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $070B3A: asr.l #$8, d1
pub fn i_070b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B3C;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $070B3C: asr.l #$8, d2
pub fn i_070b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B3E;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $070B3E: move.w d1, $2a(a6)
pub fn i_070b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B42;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070B42: move.w d2, $28(a6)
pub fn i_070b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B46;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070B46: bset.b #$6, $13(a6)
pub fn i_070b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B4C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070B4C: jsr $27d50.l
pub fn i_070b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70B52);
    cpu.pc = 0x27D50;
    16
}
/// $070B52: subq.w #$1, $94(a6)
pub fn i_070b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B56;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $070B56: cmpi.w #$0, $94(a6)
pub fn i_070b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B5C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $070B5C: ble.w $70b66
pub fn i_070b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B60;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x70B66; return 10; }
    8
}
// ===== ClearXN_070b60  ($070B60, 6 B, thunks/ccr_helpers.c)
/// $070B60: andi.b #$ee, ccr
pub fn i_070b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B64;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $070B64: rts
pub fn i_070b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Clr74_070b66  ($070B66, 4 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070B66: clr.b $74(a6)
pub fn i_070b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B6A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetXN_070b6a  ($070B6A, 6 B, thunks/ccr_helpers.c)
/// $070B6A: ori.b #$11, ccr
pub fn i_070b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B6E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $070B6E: rts
pub fn i_070b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_MoveStep8_070b70  ($070B70, 8 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070B70: move.w #$8, d3
pub fn i_070b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B74;
    cpu.ir = 0x363C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070B74: bra.w $70af2
pub fn i_070b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B78;
    cpu.ir = 0x6000;
    cpu.pc = 0x70AF2;
    10
}
// ===== M5Boss_MoveStep1_070b78  ($070B78, 8 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070B78: move.w #$1, d3
pub fn i_070b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B7C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070B7C: bra.w $70af2
pub fn i_070b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B80;
    cpu.ir = 0x6000;
    cpu.pc = 0x70AF2;
    10
}
// ===== ClearXN_070b80  ($070B80, 6 B, thunks/ccr_helpers.c)
/// $070B80: andi.b #$ee, ccr
pub fn i_070b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B84;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $070B84: rts
pub fn i_070b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_MoveSin_070b86  ($070B86, 54 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070B86: move.b $74(a6), d0
pub fn i_070b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B8A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $070B8A: addq.b #$8, $74(a6)
pub fn i_070b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B8E;
    cpu.ir = 0x502E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $070B8E: lea.l $2c072c.l, a0
pub fn i_070b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C072C;
    20
}
/// $070B94: andi.w #$ff, d0
pub fn i_070b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B98;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070B98: add.w d0, d0
pub fn i_070b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70B9A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $070B9A: move.w (a0, d0.w), $2a(a6)
pub fn i_070b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BA0;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $070BA0: clr.w $28(a6)
pub fn i_070ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BA4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $070BA4: bset.b #$6, $13(a6)
pub fn i_070ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BAA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070BAA: jsr $27d50.l
pub fn i_070baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70BB0);
    cpu.pc = 0x27D50;
    16
}
/// $070BB0: bcc.w $70bbc
pub fn i_070bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70BBC; return 10; }
    8
}
/// $070BB4: clr.w $2e(a6)
pub fn i_070bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BB8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $070BB8: clr.w $2a(a6)
pub fn i_070bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BBC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_070bbc  ($070BBC, 6 B, thunks/ccr_helpers.c)
/// $070BBC: andi.b #$ee, ccr
pub fn i_070bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BC0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $070BC0: rts
pub fn i_070bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_MoveSinB_070bc2  ($070BC2, 58 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070BC2: move.b $74(a6), d0
pub fn i_070bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BC6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $070BC6: addq.b #$8, $74(a6)
pub fn i_070bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BCA;
    cpu.ir = 0x502E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $070BCA: lea.l $2c072c.l, a0
pub fn i_070bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C072C;
    20
}
/// $070BD0: andi.w #$ff, d0
pub fn i_070bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BD4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070BD4: add.w d0, d0
pub fn i_070bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BD6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $070BD6: move.w (a0, d0.w), d0
pub fn i_070bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BDA;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $070BDA: asr.w #$1, d0
pub fn i_070bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BDC;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $070BDC: move.w d0, $2a(a6)
pub fn i_070bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BE0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070BE0: clr.w $28(a6)
pub fn i_070be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BE4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $070BE4: bset.b #$6, $13(a6)
pub fn i_070be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BEA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070BEA: jsr $27d50.l
pub fn i_070bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70BF0);
    cpu.pc = 0x27D50;
    16
}
/// $070BF0: bcc.w $70bfc
pub fn i_070bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70BFC; return 10; }
    8
}
/// $070BF4: clr.w $2e(a6)
pub fn i_070bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BF8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $070BF8: clr.w $2a(a6)
pub fn i_070bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70BFC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_070bfc  ($070BFC, 6 B, thunks/ccr_helpers.c)
/// $070BFC: andi.b #$ee, ccr
pub fn i_070bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C00;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $070C00: rts
pub fn i_070c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_PickTimers_070c02  ($070C02, 62 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070C02: subq.w #$1, $70(a6)
pub fn i_070c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C06;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $070C06: cmpi.w #$0, $70(a6)
pub fn i_070c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C0C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $070C0C: bgt.w $70c46
pub fn i_070c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C10;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x70C46; return 10; }
    8
}
/// $070C10: move.w #$2, d0
pub fn i_070c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C14;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070C14: cmpi.b #$2, $73(a6)
pub fn i_070c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C1A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $070C1A: bge.w $70c40
pub fn i_070c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C1E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x70C40; return 10; }
    8
}
/// $070C1E: lea.l $2ba166.l, a0
pub fn i_070c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA166;
    20
}
/// $070C24: jsr $799de.l
pub fn i_070c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70C2A);
    cpu.pc = 0x799DE;
    16
}
/// $070C2A: cmpi.b #$1, $73(a6)
pub fn i_070c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C30;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $070C30: beq.w $70c40
pub fn i_070c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x70C40; return 10; }
    8
}
/// $070C34: lea.l $2ba0e4.l, a0
pub fn i_070c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA0E4;
    20
}
/// $070C3A: jsr $799de.l
pub fn i_070c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70C40);
    cpu.pc = 0x799DE;
    16
}
// ===== SetXN_070c40  ($070C40, 6 B, thunks/ccr_helpers.c)
/// $070C40: ori.b #$11, ccr
pub fn i_070c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C44;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $070C44: rts
pub fn i_070c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Rts_070c46  ($070C46, 2 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070C46: clr.w d0
pub fn i_070c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C48;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== ClearXN_070c48  ($070C48, 6 B, thunks/ccr_helpers.c)
/// $070C48: andi.b #$ee, ccr
pub fn i_070c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C4C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $070C4C: rts
pub fn i_070c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_HitFlash_070c4e  ($070C4E, 112 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070C4E: jsr $2870a.l
pub fn i_070c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70C54);
    cpu.pc = 0x2870A;
    16
}
/// $070C54: bcc.w $70cbc
pub fn i_070c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x70CBC; return 10; }
    8
}
/// $070C58: bclr.b #$3, $13(a6)
pub fn i_070c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C5E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070C5E: lea.l $5e766.l, a0
pub fn i_070c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $070C64: jsr $5e770.l
pub fn i_070c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70C6A);
    cpu.pc = 0x5E770;
    16
}
/// $070C6A: move.w $66(a6), d0
pub fn i_070c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C6E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070C6E: cmp.w $7a(a6), d0
pub fn i_070c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C72;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $070C72: bgt.w $70cbc
pub fn i_070c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C76;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x70CBC; return 10; }
    8
}
/// $070C76: move.b $73(a6), d1
pub fn i_070c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C7A;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $070C7A: cmp.b $8e(a6), d1
pub fn i_070c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C7E;
    cpu.ir = 0xB22E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $070C7E: beq.w $70c9c
pub fn i_070c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x70C9C; return 10; }
    8
}
/// $070C82: move.b d1, $8e(a6)
pub fn i_070c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C86;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070C86: move.w #$1023, d0
pub fn i_070c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C8A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070C8A: jsr $2352.l
pub fn i_070c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70C90);
    cpu.pc = 0x2352;
    16
}
/// $070C90: lea.l $2d543a.l, a1
pub fn i_070c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D543A;
    20
}
/// $070C96: jsr $77c7e.l
pub fn i_070c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70C9C);
    cpu.pc = 0x77C7E;
    16
}
/// $070C9C: clr.w d2
pub fn i_070c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70C9E;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $070C9E: move.b #$2, d1
pub fn i_070c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CA2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $070CA2: move.w $7c(a6), d0
pub fn i_070ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CA6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070CA6: cmp.w $7a(a6), d0
pub fn i_070ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CAA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $070CAA: beq.w $70cb4
pub fn i_070caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x70CB4; return 10; }
    8
}
/// $070CAE: move.b #$1, d1
pub fn i_070cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CB2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $070CB2: move.w d0, d2
pub fn i_070cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CB4;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $070CB4: move.w d2, $7a(a6)
pub fn i_070cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CB8;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070CB8: move.b d1, $73(a6)
pub fn i_070cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CBC;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070CBC: rts
pub fn i_070cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_RandSprite54BE_070cbe  ($070CBE, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070CBE: jsr $5e9b6.l
pub fn i_070cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70CC4);
    cpu.pc = 0x5E9B6;
    16
}
/// $070CC4: andi.w #$7, d0
pub fn i_070cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CC8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070CC8: lsl.w #$2, d0
pub fn i_070cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CCA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $070CCA: lea.l $2d54be.l, a0
pub fn i_070cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D54BE;
    20
}
/// $070CD0: move.w (a0, d0.w), $80(a6)
pub fn i_070cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CD6;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    22
}
/// $070CD6: move.w $2(a0, d0.w), $82(a6)
pub fn i_070cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CDC;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    22
}
/// $070CDC: rts
pub fn i_070cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CDE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_RandSprite551E_070cde  ($070CDE, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070CDE: jsr $5e9b6.l
pub fn i_070cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70CE4);
    cpu.pc = 0x5E9B6;
    16
}
/// $070CE4: andi.w #$3, d0
pub fn i_070ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CE8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070CE8: lsl.w #$2, d0
pub fn i_070ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CEA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $070CEA: lea.l $2d551e.l, a0
pub fn i_070cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D551E;
    20
}
/// $070CF0: move.w (a0, d0.w), $80(a6)
pub fn i_070cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CF6;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    22
}
/// $070CF6: move.w $2(a0, d0.w), $82(a6)
pub fn i_070cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CFC;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    22
}
/// $070CFC: rts
pub fn i_070cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70CFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_RandSprite54FE_070cfe  ($070CFE, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070CFE: jsr $5e9b6.l
pub fn i_070cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70D04);
    cpu.pc = 0x5E9B6;
    16
}
/// $070D04: andi.w #$7, d0
pub fn i_070d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D08;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070D08: lsl.w #$2, d0
pub fn i_070d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D0A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $070D0A: lea.l $2d54fe.l, a0
pub fn i_070d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D54FE;
    20
}
/// $070D10: move.w (a0, d0.w), $80(a6)
pub fn i_070d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D16;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    22
}
/// $070D16: move.w $2(a0, d0.w), $82(a6)
pub fn i_070d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D1C;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    22
}
/// $070D1C: rts
pub fn i_070d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_CopyParentPos_070d1e  ($070D1E, 60 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070D1E: movea.l $c(a6), a0
pub fn i_070d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D22;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070D22: move.w $22(a0), $22(a6)
pub fn i_070d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D28;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $070D28: move.w $24(a0), $24(a6)
pub fn i_070d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D2E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $070D2E: move.b $26(a0), $26(a6)
pub fn i_070d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D34;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $070D34: move.b $27(a0), $27(a6)
pub fn i_070d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D3A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $070D3A: move.w $38(a0), $38(a6)
pub fn i_070d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D40;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $070D40: move.b $85(a0), d0
pub fn i_070d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D44;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $070D44: ext.w d0
pub fn i_070d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D46;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $070D46: addi.w #$2e, d0
pub fn i_070d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D4A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070D4A: add.w d0, $24(a6)
pub fn i_070d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D4E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $070D4E: addi.w #$ffdd, $22(a6)
pub fn i_070d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D54;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFDD, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $070D54: addq.w #$2, $38(a6)
pub fn i_070d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D58;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $070D58: rts
pub fn i_070d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_FollowParentY2C_070d5a  ($070D5A, 34 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070D5A: jsr $5e506.l
pub fn i_070d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70D60);
    cpu.pc = 0x5E506;
    16
}
/// $070D60: addi.w #$2c, $24(a6)
pub fn i_070d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D66;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $070D66: movea.l $c(a6), a0
pub fn i_070d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D6A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070D6A: btst.b #$7, $5a(a0)
pub fn i_070d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D70;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $070D70: beq.w $70d7a
pub fn i_070d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x70D7A; return 10; }
    8
}
/// $070D74: bset.b #$0, $5a(a6)
pub fn i_070d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D7A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070D7A: rts
pub fn i_070d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_FollowParent85_070d7c  ($070D7C, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070D7C: jsr $5e506.l
pub fn i_070d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70D82);
    cpu.pc = 0x5E506;
    16
}
/// $070D82: move.b $85(a0), d0
pub fn i_070d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D86;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $070D86: ext.w d0
pub fn i_070d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D88;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $070D88: addi.w #$35, d0
pub fn i_070d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D8C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x35, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070D8C: add.w d0, $24(a6)
pub fn i_070d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D90;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $070D90: addi.w #$fff0, $22(a6)
pub fn i_070d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D96;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $070D96: addq.w #$2, $38(a6)
pub fn i_070d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D9A;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $070D9A: rts
pub fn i_070d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70D9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_FollowParentPrio_070d9c  ($070D9C, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070D9C: movea.l $c(a6), a0
pub fn i_070d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DA0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070DA0: move.b $86(a6), d0
pub fn i_070da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DA4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $070DA4: ext.w d0
pub fn i_070da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DA6;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $070DA6: add.w $38(a0), d0
pub fn i_070da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DAA;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $070DAA: move.w d0, $38(a6)
pub fn i_070daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DAE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070DAE: move.w $22(a0), $22(a6)
pub fn i_070dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DB4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $070DB4: move.w $24(a0), $24(a6)
pub fn i_070db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DBA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $070DBA: rts
pub fn i_070dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_InScreenB_070dbc  ($070DBC, 68 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070DBC: move.w $24(a6), d2
pub fn i_070dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DC0;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070DC0: addq.w #$8, d2
pub fn i_070dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DC2;
    cpu.ir = 0x5042;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $070DC2: subq.w #$8, d2
pub fn i_070dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DC4;
    cpu.ir = 0x5142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $070DC4: cmpi.w #$100, d2
pub fn i_070dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DC8;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $070DC8: blt.w $70df2
pub fn i_070dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DCC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x70DF2; return 10; }
    8
}
/// $070DCC: move.w $22(a6), d1
pub fn i_070dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DD0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070DD0: move.w d2, $5c(a6)
pub fn i_070dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DD4;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070DD4: jsr $280c6.l
pub fn i_070dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70DDA);
    cpu.pc = 0x280C6;
    16
}
/// $070DDA: move.w $5c(a6), d2
pub fn i_070dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DDE;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070DDE: tst.b d0
pub fn i_070dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DE0;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $070DE0: beq.b $70dc2
pub fn i_070de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DE2;
    cpu.ir = 0x67E0;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x70DC2; return 10; }
    8
}
/// $070DE2: andi.w #$30, d0
pub fn i_070de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DE6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x30) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $070DE6: tst.w d0
pub fn i_070de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DE8;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $070DE8: beq.w $70df2
pub fn i_070de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x70DF2; return 10; }
    8
}
/// $070DEC: cmpi.w #$20, d0
pub fn i_070dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DF0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $070DF0: bne.b $70dc2
pub fn i_070df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DF2;
    cpu.ir = 0x66D0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x70DC2; return 10; }
    8
}
/// $070DF2: move.w $24(a6), d1
pub fn i_070df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DF6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070DF6: sub.w d2, d1
pub fn i_070df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DF8;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $070DF8: cmp.w $82(a6), d1
pub fn i_070df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70DFC;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $070DFC: bgt.w $70e06
pub fn i_070dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E00;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x70E06; return 10; }
    8
}
// ===== SetXN_070e00  ($070E00, 6 B, thunks/ccr_helpers.c)
/// $070E00: ori.b #$11, ccr
pub fn i_070e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E04;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $070E04: rts
pub fn i_070e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_070e06  ($070E06, 6 B, thunks/ccr_helpers.c)
/// $070E06: andi.b #$ee, ccr
pub fn i_070e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E0A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $070E0A: rts
pub fn i_070e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_VelByAngle76_070e0c  ($070E0C, 48 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070E0C: move.w $76(a6), d0
pub fn i_070e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E10;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070E10: lsl.w #$4, d0
pub fn i_070e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E12;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $070E12: lea.l $2c072c.l, a1
pub fn i_070e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E18;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $070E18: lea.l $2c07ac.l, a2
pub fn i_070e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E1E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C07AC;
    20
}
/// $070E1E: move.w (a1, d0.w), d1
pub fn i_070e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E22;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $070E22: move.w (a2, d0.w), d2
pub fn i_070e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E26;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $070E26: move.w $36(a6), d0
pub fn i_070e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E2A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $070E2A: muls.w d0, d1
pub fn i_070e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E2C;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $070E2C: muls.w d0, d2
pub fn i_070e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E2E;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $070E2E: asr.l #$8, d1
pub fn i_070e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E30;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $070E30: asr.l #$8, d2
pub fn i_070e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E32;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $070E32: move.w d1, $2a(a6)
pub fn i_070e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E36;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070E36: move.w d2, $28(a6)
pub fn i_070e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E3A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $070E3A: rts
pub fn i_070e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnSoldier_070e3c  ($070E3C, 30 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070E3C: move.b #$0, $8a(a6)
pub fn i_070e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070E42: lea.l $702fc(pc), a1
pub fn i_070e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x702FC;
    16
}
/// $070E46: jsr $4ae.l
pub fn i_070e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70E4C);
    cpu.pc = 0x4AE;
    16
}
/// $070E4C: jsr $5dd02.l
pub fn i_070e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70E52);
    cpu.pc = 0x5DD02;
    16
}
/// $070E52: bset.b #$1, $12(a0)
pub fn i_070e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E58;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070E58: rts
pub fn i_070e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnSoldierFire_070e5a  ($070E5A, 30 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070E5A: move.b #$0, $8a(a6)
pub fn i_070e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E60;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070E60: lea.l $70438(pc), a1
pub fn i_070e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70438;
    16
}
/// $070E64: jsr $4ae.l
pub fn i_070e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70E6A);
    cpu.pc = 0x4AE;
    16
}
/// $070E6A: jsr $5dd02.l
pub fn i_070e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70E70);
    cpu.pc = 0x5DD02;
    16
}
/// $070E70: bset.b #$1, $12(a0)
pub fn i_070e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E76;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070E76: rts
pub fn i_070e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnGrenadier_070e78  ($070E78, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070E78: move.b #$0, $8a(a6)
pub fn i_070e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E7E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070E7E: lea.l $704fa(pc), a1
pub fn i_070e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x704FA;
    16
}
/// $070E82: jsr $4ae.l
pub fn i_070e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70E88);
    cpu.pc = 0x4AE;
    16
}
/// $070E88: jsr $5dd02.l
pub fn i_070e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70E8E);
    cpu.pc = 0x5DD02;
    16
}
/// $070E8E: bset.b #$1, $12(a0)
pub fn i_070e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E94;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070E94: move.b $9a(a6), $9a(a0)
pub fn i_070e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E9A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $070E9A: move.w $66(a6), $66(a0)
pub fn i_070e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EA0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    20
}
/// $070EA0: rts
pub fn i_070ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnRocket_070ea2  ($070EA2, 28 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070EA2: lea.l $70956(pc), a1
pub fn i_070ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70956;
    16
}
/// $070EA6: jsr $4ae.l
pub fn i_070ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70EAC);
    cpu.pc = 0x4AE;
    16
}
/// $070EAC: jsr $5dd02.l
pub fn i_070eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70EB2);
    cpu.pc = 0x5DD02;
    16
}
/// $070EB2: addq.w #$8, $24(a0)
pub fn i_070eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EB6;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $070EB6: addi.w #$ffc5, $22(a0)
pub fn i_070eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EBC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFC5, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $070EBC: rts
pub fn i_070ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnTurretCar_070ebe  ($070EBE, 24 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070EBE: lea.l $4cbd4.l, a1
pub fn i_070ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EC4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4CBD4;
    20
}
/// $070EC4: jsr $4ae.l
pub fn i_070ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70ECA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70ECA);
    cpu.pc = 0x4AE;
    16
}
/// $070ECA: jsr $5dd02.l
pub fn i_070eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70ED0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70ED0);
    cpu.pc = 0x5DD02;
    16
}
/// $070ED0: move.l a0, $90(a6)
pub fn i_070ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70ED4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $070ED4: rts
pub fn i_070ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70ED6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnNone_070ed6  ($070ED6, 10 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070ED6: movea.l $90(a6), a0
pub fn i_070ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EDA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $070EDA: clr.b $80(a0)
pub fn i_070eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EDE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $070EDE: rts
pub fn i_070ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnFx8770_070ee0  ($070EE0, 10 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070EE0: lea.l $70878(pc), a1
pub fn i_070ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70878;
    16
}
/// $070EE4: jsr $4ae.l
pub fn i_070ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70EEA);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_070eea  ($070EEA, 8 B, thunks/jsr_abs_thunks.c)
/// $070EEA: jsr $5dd02.l
pub fn i_070eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70EF0);
    cpu.pc = 0x5DD02;
    16
}
/// $070EF0: rts
pub fn i_070ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnFx87AE_070ef2  ($070EF2, 10 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070EF2: lea.l $708b2(pc), a1
pub fn i_070ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x708B2;
    16
}
/// $070EF6: jsr $4ae.l
pub fn i_070ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70EFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70EFC);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_070efc  ($070EFC, 8 B, thunks/jsr_abs_thunks.c)
/// $070EFC: jsr $5dd02.l
pub fn i_070efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F02);
    cpu.pc = 0x5DD02;
    16
}
/// $070F02: rts
pub fn i_070f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnFxD8B6_070f04  ($070F04, 30 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070F04: lea.l $708f4(pc), a1
pub fn i_070f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x708F4;
    16
}
/// $070F08: jsr $4ae.l
pub fn i_070f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F0E);
    cpu.pc = 0x4AE;
    16
}
/// $070F0E: jsr $5dd02.l
pub fn i_070f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F14);
    cpu.pc = 0x5DD02;
    16
}
/// $070F14: addi.w #$ffd4, $22(a0)
pub fn i_070f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F1A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFD4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $070F1A: addi.w #$ffc8, $24(a0)
pub fn i_070f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F20;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFC8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $070F20: rts
pub fn i_070f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnBomb_070f22  ($070F22, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070F22: lea.l $6fcfa(pc), a1
pub fn i_070f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FCFA;
    16
}
/// $070F26: jsr $4ae.l
pub fn i_070f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F2C);
    cpu.pc = 0x4AE;
    16
}
/// $070F2C: jsr $5dd02.l
pub fn i_070f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F32);
    cpu.pc = 0x5DD02;
    16
}
/// $070F32: move.b $9b(a6), $9b(a0)
pub fn i_070f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F38;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $070F38: move.b $73(a6), $73(a0)
pub fn i_070f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F3E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $070F3E: move.b $87(a6), $87(a0)
pub fn i_070f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F44;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x87);
    bus.write8(ad4, v3 as u8);
    20
}
/// $070F44: move.b $88(a6), $88(a0)
pub fn i_070f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F4A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x88);
    bus.write8(ad4, v3 as u8);
    20
}
/// $070F4A: rts
pub fn i_070f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnFx82BA_070f4c  ($070F4C, 36 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070F4C: lea.l $702a8(pc), a1
pub fn i_070f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x702A8;
    16
}
/// $070F50: jsr $4ae.l
pub fn i_070f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F56);
    cpu.pc = 0x4AE;
    16
}
/// $070F56: jsr $5dd02.l
pub fn i_070f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F5C);
    cpu.pc = 0x5DD02;
    16
}
/// $070F5C: move.b #$d7, $84(a0)
pub fn i_070f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F62;
    cpu.ir = 0x117C;
    let v1: u32 = 0xD7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070F62: move.b #$f1, $85(a0)
pub fn i_070f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F68;
    cpu.ir = 0x117C;
    let v1: u32 = 0xF1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070F68: move.b #$1, $86(a0)
pub fn i_070f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F6E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070F6E: rts
pub fn i_070f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnFx82BAB_070f70  ($070F70, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070F70: lea.l $702a8(pc), a1
pub fn i_070f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x702A8;
    16
}
/// $070F74: jsr $4ae.l
pub fn i_070f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F7A);
    cpu.pc = 0x4AE;
    16
}
/// $070F7A: jsr $5dd02.l
pub fn i_070f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70F80);
    cpu.pc = 0x5DD02;
    16
}
/// $070F80: ori.b #$1, $3a(a0)
pub fn i_070f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F86;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070F86: move.b #$df, $84(a0)
pub fn i_070f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F8C;
    cpu.ir = 0x117C;
    let v1: u32 = 0xDF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070F8C: move.b #$f1, $85(a0)
pub fn i_070f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F92;
    cpu.ir = 0x117C;
    let v1: u32 = 0xF1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070F92: move.b #$1, $86(a0)
pub fn i_070f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F98;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070F98: rts
pub fn i_070f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnFx8320_070f9a  ($070F9A, 36 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070F9A: lea.l $702e4(pc), a1
pub fn i_070f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70F9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x702E4;
    16
}
/// $070F9E: jsr $4ae.l
pub fn i_070f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70FA4);
    cpu.pc = 0x4AE;
    16
}
/// $070FA4: jsr $5dd02.l
pub fn i_070fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70FAA);
    cpu.pc = 0x5DD02;
    16
}
/// $070FAA: move.b #$d7, $84(a0)
pub fn i_070faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FB0;
    cpu.ir = 0x117C;
    let v1: u32 = 0xD7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070FB0: move.b #$f1, $85(a0)
pub fn i_070fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FB6;
    cpu.ir = 0x117C;
    let v1: u32 = 0xF1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070FB6: move.b #$1, $86(a0)
pub fn i_070fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FBC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070FBC: rts
pub fn i_070fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnFx8320B_070fbe  ($070FBE, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070FBE: lea.l $702e4(pc), a1
pub fn i_070fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x702E4;
    16
}
/// $070FC2: jsr $4ae.l
pub fn i_070fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70FC8);
    cpu.pc = 0x4AE;
    16
}
/// $070FC8: jsr $5dd02.l
pub fn i_070fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70FCE);
    cpu.pc = 0x5DD02;
    16
}
/// $070FCE: ori.b #$1, $3a(a0)
pub fn i_070fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FD4;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $070FD4: move.b #$df, $84(a0)
pub fn i_070fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FDA;
    cpu.ir = 0x117C;
    let v1: u32 = 0xDF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070FDA: move.b #$f1, $85(a0)
pub fn i_070fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FE0;
    cpu.ir = 0x117C;
    let v1: u32 = 0xF1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070FE0: move.b #$1, $86(a0)
pub fn i_070fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FE6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $070FE6: rts
pub fn i_070fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnSparkBurst_070fe8  ($070FE8, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $070FE8: lea.l $7016e(pc), a1
pub fn i_070fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7016E;
    16
}
/// $070FEC: jsr $4ae.l
pub fn i_070fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70FF2);
    cpu.pc = 0x4AE;
    16
}
/// $070FF2: jsr $5dd02.l
pub fn i_070ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x70FF8);
    cpu.pc = 0x5DD02;
    16
}
/// $070FF8: lea.l $700ae(pc), a1
pub fn i_070ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70FFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x700AE;
    16
}
/// $070FFC: jsr $4ae.l
pub fn i_070ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71002;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71002);
    cpu.pc = 0x4AE;
    16
}
/// $071002: jsr $5dd02.l
pub fn i_071002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71008;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71008);
    cpu.pc = 0x5DD02;
    16
}
/// $071008: lea.l $7010e(pc), a1
pub fn i_071008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7100C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7010E;
    16
}
/// $07100C: jsr $4ae.l
pub fn i_07100c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71012);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_071012  ($071012, 8 B, thunks/jsr_abs_thunks.c)
/// $071012: jsr $5dd02.l
pub fn i_071012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71018);
    cpu.pc = 0x5DD02;
    16
}
/// $071018: rts
pub fn i_071018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7101A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_SpawnSmoke_07101a  ($07101A, 30 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07101A: lea.l $77efe.l, a1
pub fn i_07101a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71020;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EFE;
    20
}
/// $071020: jsr $4ae.l
pub fn i_071020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71026);
    cpu.pc = 0x4AE;
    16
}
/// $071026: jsr $5dd02.l
pub fn i_071026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7102C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7102C);
    cpu.pc = 0x5DD02;
    16
}
/// $07102C: addi.w #$38, $24(a0)
pub fn i_07102c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71032;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $071032: addq.w #$4, $38(a0)
pub fn i_071032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71036;
    cpu.ir = 0x5868;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071036: rts
pub fn i_071036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71038;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_AssertAngle_071038  ($071038, 20 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071038: cmpi.w #$b, $76(a6)
pub fn i_071038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7103E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB, Size::Word, false);
    16
}
/// $07103E: bcs.w $7104a
pub fn i_07103e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71042;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7104A; return 10; }
    8
}
/// $071042: move.w d0, d0
pub fn i_071042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71044;
    cpu.ir = 0x3000;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071044: move.w d1, d1
pub fn i_071044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71046;
    cpu.ir = 0x3201;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071046: move.w d2, d2
pub fn i_071046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71048;
    cpu.ir = 0x3402;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071048: trap #$f
pub fn i_071048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7104A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $07104A: rts
pub fn i_07104a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7104C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_CmpSlotWithParent_07104c  ($07104C, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07104C: movea.l $8(a6), a1
pub fn i_07104c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71050;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $071050: move.b $10(a6), d0
pub fn i_071050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71054;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $071054: cmp.b $10(a1), d0
pub fn i_071054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71058;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $071058: bcs.w $71062
pub fn i_071058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7105C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x71062; return 10; }
    8
}
// ===== ClearXN_07105c  ($07105C, 6 B, thunks/ccr_helpers.c)
/// $07105C: andi.b #$ee, ccr
pub fn i_07105c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71060;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $071060: rts
pub fn i_071060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71062;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_071062  ($071062, 6 B, thunks/ccr_helpers.c)
/// $071062: ori.b #$11, ccr
pub fn i_071062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71066;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $071066: rts
pub fn i_071066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71068;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Tmpl72_071068  ($071068, 116 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071068: move.w #$28, d0
pub fn i_071068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7106C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07106C: jsr $2352.l
pub fn i_07106c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71072);
    cpu.pc = 0x2352;
    16
}
/// $071072: lea.l $71160(pc), a1
pub fn i_071072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71076;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71160;
    16
}
/// $071076: jsr $4ae.l
pub fn i_071076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7107C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7107C);
    cpu.pc = 0x4AE;
    16
}
/// $07107C: jsr $5dd02.l
pub fn i_07107c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71082);
    cpu.pc = 0x5DD02;
    16
}
/// $071082: move.w #$ffc0, $22(a0)
pub fn i_071082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71088;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071088: move.w #$140, $24(a0)
pub fn i_071088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7108E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07108E: move.b #$1, $3a(a0)
pub fn i_07108e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71094;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $071094: move.b #$1, $77(a0)
pub fn i_071094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7109A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07109A: move.b #$1, $9f(a0)
pub fn i_07109a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710A0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0710A0: lea.l $7122e(pc), a1
pub fn i_0710a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7122E;
    16
}
/// $0710A4: jsr $4ae.l
pub fn i_0710a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x710AA);
    cpu.pc = 0x4AE;
    16
}
/// $0710AA: jsr $5dd02.l
pub fn i_0710aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x710B0);
    cpu.pc = 0x5DD02;
    16
}
/// $0710B0: move.w #$180, $22(a0)
pub fn i_0710b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710B6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0710B6: move.w #$1b0, $24(a0)
pub fn i_0710b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710BC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0710BC: move.b #$1, $77(a0)
pub fn i_0710bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710C2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0710C2: clr.b $9f(a0)
pub fn i_0710c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710C6;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0710C6: move.b #$0, $20(a6)
pub fn i_0710c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0710CC: lea.l $710dc(pc), a1
pub fn i_0710cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x710DC;
    16
}
/// $0710D0: move.l a1, (a6)
pub fn i_0710d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0710D2: clr.b $21(a6)
pub fn i_0710d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710D6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0710D6: clr.b $76(a6)
pub fn i_0710d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710DA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0710DA: rts
pub fn i_0710da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_WaitChildren_0710dc  ($0710DC, 64 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0710DC: cmpi.b #$1, $76(a6)
pub fn i_0710dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710E2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0710E2: bge.w $710f2
pub fn i_0710e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710E6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x710F2; return 10; }
    8
}
/// $0710E6: clr.b $106ed3.l
pub fn i_0710e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710EC;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED3) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED3, r2 as u8);
    16
}
/// $0710EC: lea.l $710f2(pc), a1
pub fn i_0710ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x710F2;
    16
}
/// $0710F0: move.l a1, (a6)
pub fn i_0710f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0710F2: cmpi.b #$2, $21(a6)
pub fn i_0710f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710F8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0710F8: bge.w $71112
pub fn i_0710f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x710FC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x71112; return 10; }
    8
}
/// $0710FC: move.b #$1, $20(a6)
pub fn i_0710fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71102;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $071102: cmpi.b #$0, $21(a6)
pub fn i_071102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71108;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $071108: bne.w $71112
pub fn i_071108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7110C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71112; return 10; }
    8
}
/// $07110C: lea.l $7111c(pc), a1
pub fn i_07110c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71110;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7111C;
    16
}
/// $071110: move.l a1, (a6)
pub fn i_071110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71112;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071112: clr.b $21(a6)
pub fn i_071112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71116;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $071116: clr.b $76(a6)
pub fn i_071116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7111A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07111A: rts
pub fn i_07111a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7111C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_End_07111c  ($07111C, 50 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07111C: move.w #$1071, d0
pub fn i_07111c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71120;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1071;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071120: jsr $2222.l
pub fn i_071120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71126;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71126);
    cpu.pc = 0x2222;
    16
}
/// $071126: move.w #$1e, $70(a6)
pub fn i_071126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7112C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07112C: lea.l $71132(pc), a1
pub fn i_07112c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71130;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71132;
    16
}
/// $071130: move.l a1, (a6)
pub fn i_071130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71132;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071132: subq.w #$1, $70(a6)
pub fn i_071132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71136;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071136: cmpi.w #$0, $70(a6)
pub fn i_071136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7113C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $07113C: bgt.w $7114c
pub fn i_07113c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71140;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7114C; return 10; }
    8
}
/// $071140: clr.b $106ed2.l
pub fn i_071140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71146;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $071146: jmp $518.l
pub fn i_071146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7114C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07114C: rts
pub fn i_07114c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7114E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Tmpl73_07114e  ($07114E, 18 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07114E: clr.b $77(a6)
pub fn i_07114e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71152;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $071152: move.w #$1e5, d1
pub fn i_071152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71156;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071156: jsr $236e.l
pub fn i_071156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7115C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7115C);
    cpu.pc = 0x236E;
    16
}
/// $07115C: bra.w $71182
pub fn i_07115c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71160;
    cpu.ir = 0x6000;
    cpu.pc = 0x71182;
    10
}
// ===== FinalBoss_Left_071160  ($071160, 206 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071160: bset.b #$1, $12(a6)
pub fn i_071160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71166;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071166: move.w #$eac, d0
pub fn i_071166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7116A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xEAC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07116A: move.w #$ea, d1
pub fn i_07116a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7116E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07116E: jsr $440d0.l
pub fn i_07116e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71174);
    cpu.pc = 0x440D0;
    16
}
/// $071174: move.w d1, $24(a6)
pub fn i_071174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71178;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071178: move.w #$5b, d1
pub fn i_071178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7117C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07117C: jsr $236e.l
pub fn i_07117c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71182);
    cpu.pc = 0x236E;
    16
}
/// $071182: move.b #$0, $80(a6)
pub fn i_071182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71188;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $071188: move.w #$c, $1c(a6)
pub fn i_071188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7118E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07118E: jsr $138fe.l
pub fn i_07118e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71194);
    cpu.pc = 0x138FE;
    16
}
/// $071194: lea.l $71940(pc), a1
pub fn i_071194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71198;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71940;
    16
}
/// $071198: jsr $4ae.l
pub fn i_071198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7119E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7119E);
    cpu.pc = 0x4AE;
    16
}
/// $07119E: jsr $5dd02.l
pub fn i_07119e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711A4);
    cpu.pc = 0x5DD02;
    16
}
/// $0711A4: lea.l $7186e(pc), a1
pub fn i_0711a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7186E;
    16
}
/// $0711A8: jsr $4ae.l
pub fn i_0711a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711AE);
    cpu.pc = 0x4AE;
    16
}
/// $0711AE: jsr $5dd02.l
pub fn i_0711ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711B4);
    cpu.pc = 0x5DD02;
    16
}
/// $0711B4: lea.l $71898(pc), a1
pub fn i_0711b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71898;
    16
}
/// $0711B8: jsr $4ae.l
pub fn i_0711b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711BE);
    cpu.pc = 0x4AE;
    16
}
/// $0711BE: jsr $5dd02.l
pub fn i_0711be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711C4);
    cpu.pc = 0x5DD02;
    16
}
/// $0711C4: lea.l $718c2(pc), a1
pub fn i_0711c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x718C2;
    16
}
/// $0711C8: jsr $4ae.l
pub fn i_0711c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711CE);
    cpu.pc = 0x4AE;
    16
}
/// $0711CE: jsr $5dd02.l
pub fn i_0711ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711D4);
    cpu.pc = 0x5DD02;
    16
}
/// $0711D4: lea.l $718ec(pc), a1
pub fn i_0711d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x718EC;
    16
}
/// $0711D8: jsr $4ae.l
pub fn i_0711d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711DE);
    cpu.pc = 0x4AE;
    16
}
/// $0711DE: jsr $5dd02.l
pub fn i_0711de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x711E4);
    cpu.pc = 0x5DD02;
    16
}
/// $0711E4: move.b #$1, $84(a6)
pub fn i_0711e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0711EA: move.b #$1, $85(a6)
pub fn i_0711ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0711F0: move.b #$1, $86(a6)
pub fn i_0711f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711F6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0711F6: move.b #$1, $87(a6)
pub fn i_0711f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x711FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0711FC: move.b #$0, $83(a6)
pub fn i_0711fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71202;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $071202: lea.l $2ba79e.l, a0
pub fn i_071202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71208;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA79E;
    20
}
/// $071208: jsr $799de.l
pub fn i_071208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7120E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7120E);
    cpu.pc = 0x799DE;
    16
}
/// $07120E: move.w d0, $66(a6)
pub fn i_07120e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71212;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071212: bra.w $712e0
pub fn i_071212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71216;
    cpu.ir = 0x6000;
    cpu.pc = 0x712E0;
    10
}
// ===== FinalBoss_Right_Tmpl74_07122e  ($07122E, 364 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07122E: bset.b #$1, $12(a6)
pub fn i_07122e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71234;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071234: move.w #$eac, d0
pub fn i_071234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71238;
    cpu.ir = 0x303C;
    let v1: u32 = 0xEAC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071238: move.w #$97, d1
pub fn i_071238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7123C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x97;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07123C: jsr $440d0.l
pub fn i_07123c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71242);
    cpu.pc = 0x440D0;
    16
}
/// $071242: move.w d1, $24(a6)
pub fn i_071242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71246;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071246: move.w #$5c, d1
pub fn i_071246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7124A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07124A: jsr $236e.l
pub fn i_07124a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71250);
    cpu.pc = 0x236E;
    16
}
/// $071250: move.b #$1, $80(a6)
pub fn i_071250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71256;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $071256: move.w #$c, $1c(a6)
pub fn i_071256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7125C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07125C: jsr $138fe.l
pub fn i_07125c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71262;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71262);
    cpu.pc = 0x138FE;
    16
}
/// $071262: lea.l $71940(pc), a1
pub fn i_071262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71266;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71940;
    16
}
/// $071266: jsr $4ae.l
pub fn i_071266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7126C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7126C);
    cpu.pc = 0x4AE;
    16
}
/// $07126C: jsr $5dd02.l
pub fn i_07126c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71272;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71272);
    cpu.pc = 0x5DD02;
    16
}
/// $071272: lea.l $7186e(pc), a1
pub fn i_071272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71276;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7186E;
    16
}
/// $071276: jsr $4ae.l
pub fn i_071276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7127C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7127C);
    cpu.pc = 0x4AE;
    16
}
/// $07127C: jsr $5dd02.l
pub fn i_07127c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71282;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71282);
    cpu.pc = 0x5DD02;
    16
}
/// $071282: lea.l $71898(pc), a1
pub fn i_071282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71286;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71898;
    16
}
/// $071286: jsr $4ae.l
pub fn i_071286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7128C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7128C);
    cpu.pc = 0x4AE;
    16
}
/// $07128C: jsr $5dd02.l
pub fn i_07128c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71292);
    cpu.pc = 0x5DD02;
    16
}
/// $071292: lea.l $718c2(pc), a1
pub fn i_071292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71296;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x718C2;
    16
}
/// $071296: jsr $4ae.l
pub fn i_071296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7129C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7129C);
    cpu.pc = 0x4AE;
    16
}
/// $07129C: jsr $5dd02.l
pub fn i_07129c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x712A2);
    cpu.pc = 0x5DD02;
    16
}
/// $0712A2: lea.l $71916(pc), a1
pub fn i_0712a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71916;
    16
}
/// $0712A6: jsr $4ae.l
pub fn i_0712a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x712AC);
    cpu.pc = 0x4AE;
    16
}
/// $0712AC: jsr $5dd02.l
pub fn i_0712ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x712B2);
    cpu.pc = 0x5DD02;
    16
}
/// $0712B2: move.b #$1, $84(a6)
pub fn i_0712b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712B8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0712B8: move.b #$1, $85(a6)
pub fn i_0712b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0712BE: move.b #$1, $86(a6)
pub fn i_0712be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0712C4: move.b #$1, $87(a6)
pub fn i_0712c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0712CA: move.b #$0, $83(a6)
pub fn i_0712ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0712D0: lea.l $2ba820.l, a0
pub fn i_0712d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA820;
    20
}
/// $0712D6: jsr $799de.l
pub fn i_0712d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x712DC);
    cpu.pc = 0x799DE;
    16
}
/// $0712DC: move.w d0, $66(a6)
pub fn i_0712dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712E0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0712E0: lea.l $723d2(pc), a1
pub fn i_0712e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x723D2;
    16
}
/// $0712E4: jsr $4ae.l
pub fn i_0712e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x712EA);
    cpu.pc = 0x4AE;
    16
}
/// $0712EA: jsr $5dd02.l
pub fn i_0712ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x712F0);
    cpu.pc = 0x5DD02;
    16
}
/// $0712F0: clr.w $98(a0)
pub fn i_0712f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712F4;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0712F4: move.w #$4001, d0
pub fn i_0712f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4001;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0712F8: jsr $28134.l
pub fn i_0712f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x712FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x712FE);
    cpu.pc = 0x28134;
    16
}
/// $0712FE: andi.w #$ffe3, $38(a6)
pub fn i_0712fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71304;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071304: ori.w #$14, $38(a6)
pub fn i_071304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7130A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07130A: tst.b $9f(a6)
pub fn i_07130a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7130E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07130E: beq.w $71328
pub fn i_07130e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71312;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x71328; return 10; }
    8
}
/// $071312: move.w #$8001, d0
pub fn i_071312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71316;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8001;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071316: jsr $28134.l
pub fn i_071316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7131C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7131C);
    cpu.pc = 0x28134;
    16
}
/// $07131C: andi.w #$ffe3, $38(a6)
pub fn i_07131c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71322;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071322: ori.w #$14, $38(a6)
pub fn i_071322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71328;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071328: jsr $5e9b6.l
pub fn i_071328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7132E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7132E);
    cpu.pc = 0x5E9B6;
    16
}
/// $07132E: andi.w #$3f, d0
pub fn i_07132e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71332;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $071332: move.w d0, $72(a6)
pub fn i_071332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71336;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071336: jsr $5e9b6.l
pub fn i_071336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7133C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7133C);
    cpu.pc = 0x5E9B6;
    16
}
/// $07133C: andi.w #$3f, d0
pub fn i_07133c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71340;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $071340: move.w d0, $74(a6)
pub fn i_071340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71344;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071344: move.b #$0, $88(a6)
pub fn i_071344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7134A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07134A: move.b #$0, $8a(a6)
pub fn i_07134a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71350;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $071350: move.l #$2d90e4, $60(a6)
pub fn i_071350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71358;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D90E4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $071358: move.l #$2d9130, $94(a6)
pub fn i_071358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71360;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D9130;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $071360: move.l #$2d898c, $48(a6)
pub fn i_071360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71368;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D898C;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $071368: move.l #$2d9068, $4c(a6)
pub fn i_071368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71370;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D9068;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $071370: move.w #$1, $70(a6)
pub fn i_071370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71376;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071376: move.w #$ff40, d0
pub fn i_071376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7137A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07137A: jsr $72782(pc)
pub fn i_07137a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7137E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7137E);
    cpu.pc = 0x72782;
    16
}
/// $07137E: move.w d0, $28(a6)
pub fn i_07137e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71382;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071382: lea.l $71388(pc), a1
pub fn i_071382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71386;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71388;
    16
}
/// $071386: move.l a1, (a6)
pub fn i_071386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71388;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071388: jsr $7279a(pc)
pub fn i_071388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7138C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7138C);
    cpu.pc = 0x7279A;
    16
}
/// $07138C: subq.w #$1, $70(a6)
pub fn i_07138c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71390;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071390: cmpi.w #$0, $70(a6)
pub fn i_071390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71396;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $071396: bgt.w $713a0
pub fn i_071396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7139A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x713A0; return 10; }
    8
}
// ===== SetTaskHandler_07139a  ($07139A, 8 B, kernel/task_handlers.c)
/// $07139A: lea.l $713a2(pc), a1
pub fn i_07139a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7139E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x713A2;
    16
}
/// $07139E: move.l a1, (a6)
pub fn i_07139e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0713A0: rts
pub fn i_0713a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SnapIdle_0713a2  ($0713A2, 10 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0713A2: jsr $5e7c0.l
pub fn i_0713a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x713A8);
    cpu.pc = 0x5E7C0;
    16
}
/// $0713A8: bra.w $713ac
pub fn i_0713a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x713AC;
    10
}
// ===== FinalBoss_Idle_0713ac  ($0713AC, 146 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0713AC: move.w $28(a6), d0
pub fn i_0713ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713B0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0713B0: btst.b #$0, $3a(a6)
pub fn i_0713b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713B6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0713B6: beq.w $713bc
pub fn i_0713b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x713BC; return 10; }
    8
}
/// $0713BA: neg.w d0
pub fn i_0713ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713BC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0713BC: cmpi.w #$0, d0
pub fn i_0713bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713C0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0713C0: bgt.w $713d4
pub fn i_0713c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713C4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x713D4; return 10; }
    8
}
/// $0713C4: lea.l $2d9332.l, a0
pub fn i_0713c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9332;
    20
}
/// $0713CA: jsr $28cd4.l
pub fn i_0713ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x713D0);
    cpu.pc = 0x28CD4;
    16
}
/// $0713D0: bra.w $713e0
pub fn i_0713d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x713E0;
    10
}
/// $0713D4: lea.l $2d9362.l, a0
pub fn i_0713d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9362;
    20
}
/// $0713DA: jsr $28cd4.l
pub fn i_0713da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x713E0);
    cpu.pc = 0x28CD4;
    16
}
/// $0713E0: move.w #$1071, d0
pub fn i_0713e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1071;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0713E4: jsr $2352.l
pub fn i_0713e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x713EA);
    cpu.pc = 0x2352;
    16
}
/// $0713EA: lea.l $713f0(pc), a1
pub fn i_0713ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x713F0;
    16
}
/// $0713EE: move.l a1, (a6)
pub fn i_0713ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0713F0: jsr $28998.l
pub fn i_0713f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x713F6);
    cpu.pc = 0x28998;
    16
}
/// $0713F6: jsr $72c44(pc)
pub fn i_0713f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x713FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x713FA);
    cpu.pc = 0x72C44;
    16
}
/// $0713FA: jsr $27afc.l
pub fn i_0713fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71400);
    cpu.pc = 0x27AFC;
    16
}
/// $071400: bcc.w $7140a
pub fn i_071400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71404;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7140A; return 10; }
    8
}
/// $071404: lea.l $71448(pc), a1
pub fn i_071404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71408;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71448;
    16
}
/// $071408: move.l a1, (a6)
pub fn i_071408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7140A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07140A: jsr $28d70.l
pub fn i_07140a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71410);
    cpu.pc = 0x28D70;
    16
}
/// $071410: jsr $726d4(pc)
pub fn i_071410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71414;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71414);
    cpu.pc = 0x726D4;
    16
}
/// $071414: bcc.w $7141e
pub fn i_071414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71418;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7141E; return 10; }
    8
}
/// $071418: lea.l $71448(pc), a1
pub fn i_071418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7141C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71448;
    16
}
/// $07141C: move.l a1, (a6)
pub fn i_07141c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7141E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07141E: jsr $72750(pc)
pub fn i_07141e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71422;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71422);
    cpu.pc = 0x72750;
    16
}
/// $071422: bcc.w $7142c
pub fn i_071422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71426;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7142C; return 10; }
    8
}
/// $071426: lea.l $71448(pc), a1
pub fn i_071426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7142A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71448;
    16
}
/// $07142A: move.l a1, (a6)
pub fn i_07142a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7142C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07142C: jsr $283ca.l
pub fn i_07142c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71432);
    cpu.pc = 0x283CA;
    16
}
/// $071432: jsr $283d8.l
pub fn i_071432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71438;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71438);
    cpu.pc = 0x283D8;
    16
}
/// $071438: bra.w $71664
pub fn i_071438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7143C;
    cpu.ir = 0x6000;
    cpu.pc = 0x71664;
    10
}
// ===== FinalBoss_Count1_07143e  ($07143E, 10 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07143E: move.w #$1, $70(a6)
pub fn i_07143e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71444;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071444: bra.w $7144e
pub fn i_071444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71448;
    cpu.ir = 0x6000;
    cpu.pc = 0x7144E;
    10
}
// ===== FinalBoss_Walk_071448  ($071448, 86 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071448: move.w #$3c, $70(a6)
pub fn i_071448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7144E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07144E: neg.w $28(a6)
pub fn i_07144e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71452;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071452: move.w #$1071, d0
pub fn i_071452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71456;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1071;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071456: jsr $2222.l
pub fn i_071456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7145C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7145C);
    cpu.pc = 0x2222;
    16
}
/// $07145C: lea.l $2d9392.l, a0
pub fn i_07145c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71462;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9392;
    20
}
/// $071462: jsr $28cd4.l
pub fn i_071462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71468);
    cpu.pc = 0x28CD4;
    16
}
/// $071468: lea.l $7146e(pc), a1
pub fn i_071468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7146C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7146E;
    16
}
/// $07146C: move.l a1, (a6)
pub fn i_07146c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7146E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07146E: jsr $28998.l
pub fn i_07146e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71474;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71474);
    cpu.pc = 0x28998;
    16
}
/// $071474: jsr $72c44(pc)
pub fn i_071474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71478;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71478);
    cpu.pc = 0x72C44;
    16
}
/// $071478: jsr $2783a.l
pub fn i_071478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7147E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7147E);
    cpu.pc = 0x2783A;
    16
}
/// $07147E: jsr $28d70.l
pub fn i_07147e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71484;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71484);
    cpu.pc = 0x28D70;
    16
}
/// $071484: bcc.w $7148e
pub fn i_071484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71488;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7148E; return 10; }
    8
}
/// $071488: lea.l $714a4(pc), a1
pub fn i_071488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7148C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x714A4;
    16
}
/// $07148C: move.l a1, (a6)
pub fn i_07148c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7148E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07148E: jsr $283ca.l
pub fn i_07148e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71494);
    cpu.pc = 0x283CA;
    16
}
/// $071494: jsr $283d8.l
pub fn i_071494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7149A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7149A);
    cpu.pc = 0x283D8;
    16
}
/// $07149A: bra.w $71664
pub fn i_07149a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7149E;
    cpu.ir = 0x6000;
    cpu.pc = 0x71664;
    10
}
// ===== FinalBoss_Rts_07149e  ($07149E, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07149E: move.w #$3c, $70(a6)
pub fn i_07149e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== FinalBoss_WalkB_0714a4  ($0714A4, 64 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0714A4: lea.l $2d93cc.l, a0
pub fn i_0714a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D93CC;
    20
}
/// $0714AA: jsr $28cd4.l
pub fn i_0714aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x714B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0714B0: lea.l $714b6(pc), a1
pub fn i_0714b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x714B6;
    16
}
/// $0714B4: move.l a1, (a6)
pub fn i_0714b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0714B6: jsr $28998.l
pub fn i_0714b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x714BC);
    cpu.pc = 0x28998;
    16
}
/// $0714BC: jsr $72c44(pc)
pub fn i_0714bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x714C0);
    cpu.pc = 0x72C44;
    16
}
/// $0714C0: jsr $2783a.l
pub fn i_0714c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x714C6);
    cpu.pc = 0x2783A;
    16
}
/// $0714C6: jsr $28d70.l
pub fn i_0714c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x714CC);
    cpu.pc = 0x28D70;
    16
}
/// $0714CC: subq.w #$1, $70(a6)
pub fn i_0714cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714D0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0714D0: cmpi.w #$0, $70(a6)
pub fn i_0714d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714D6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0714D6: bne.w $714e0
pub fn i_0714d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x714E0; return 10; }
    8
}
/// $0714DA: lea.l $714e4(pc), a1
pub fn i_0714da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x714E4;
    16
}
/// $0714DE: move.l a1, (a6)
pub fn i_0714de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0714E0: bra.w $71664
pub fn i_0714e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x71664;
    10
}
// ===== FinalBoss_WalkC_0714e4  ($0714E4, 66 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0714E4: lea.l $2d93ee.l, a0
pub fn i_0714e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D93EE;
    20
}
/// $0714EA: jsr $28cd4.l
pub fn i_0714ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x714F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0714F0: lea.l $714f6(pc), a1
pub fn i_0714f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x714F6;
    16
}
/// $0714F4: move.l a1, (a6)
pub fn i_0714f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0714F6: jsr $28998.l
pub fn i_0714f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x714FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x714FC);
    cpu.pc = 0x28998;
    16
}
/// $0714FC: jsr $72c44(pc)
pub fn i_0714fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71500;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71500);
    cpu.pc = 0x72C44;
    16
}
/// $071500: jsr $2783a.l
pub fn i_071500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71506;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71506);
    cpu.pc = 0x2783A;
    16
}
/// $071506: jsr $28d70.l
pub fn i_071506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7150C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7150C);
    cpu.pc = 0x28D70;
    16
}
/// $07150C: bcc.w $71516
pub fn i_07150c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71510;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71516; return 10; }
    8
}
/// $071510: lea.l $713ac(pc), a1
pub fn i_071510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71514;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x713AC;
    16
}
/// $071514: move.l a1, (a6)
pub fn i_071514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71516;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071516: jsr $283ca.l
pub fn i_071516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7151C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7151C);
    cpu.pc = 0x283CA;
    16
}
/// $07151C: jsr $283d8.l
pub fn i_07151c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71522;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71522);
    cpu.pc = 0x283D8;
    16
}
/// $071522: bra.w $71664
pub fn i_071522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71526;
    cpu.ir = 0x6000;
    cpu.pc = 0x71664;
    10
}
// ===== FinalBoss_Stand_071526  ($071526, 92 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071526: bclr.b #$3, $13(a6)
pub fn i_071526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7152C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07152C: lea.l $2d9428.l, a0
pub fn i_07152c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71532;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9428;
    20
}
/// $071532: jsr $28cd4.l
pub fn i_071532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71538;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71538);
    cpu.pc = 0x28CD4;
    16
}
/// $071538: bra.w $7154e
pub fn i_071538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7153C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7154E;
    10
}
/// $07154E: move.w #$1071, d0
pub fn i_07154e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71552;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1071;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071552: jsr $2222.l
pub fn i_071552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71558);
    cpu.pc = 0x2222;
    16
}
/// $071558: lea.l $7155e(pc), a1
pub fn i_071558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7155C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7155E;
    16
}
/// $07155C: move.l a1, (a6)
pub fn i_07155c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7155E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07155E: jsr $28998.l
pub fn i_07155e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71564);
    cpu.pc = 0x28998;
    16
}
/// $071564: jsr $72c44(pc)
pub fn i_071564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71568;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71568);
    cpu.pc = 0x72C44;
    16
}
/// $071568: jsr $2783a.l
pub fn i_071568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7156E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7156E);
    cpu.pc = 0x2783A;
    16
}
/// $07156E: jsr $28d70.l
pub fn i_07156e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71574);
    cpu.pc = 0x28D70;
    16
}
/// $071574: bcc.w $7157e
pub fn i_071574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71578;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7157E; return 10; }
    8
}
/// $071578: lea.l $714e4(pc), a1
pub fn i_071578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7157C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x714E4;
    16
}
/// $07157C: move.l a1, (a6)
pub fn i_07157c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7157E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07157E: bra.w $71664
pub fn i_07157e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71582;
    cpu.ir = 0x6000;
    cpu.pc = 0x71664;
    10
}
// ===== FinalBoss_Die_071582  ($071582, 112 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071582: move.w #$1033, d0
pub fn i_071582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71586;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071586: jsr $2352.l
pub fn i_071586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7158C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7158C);
    cpu.pc = 0x2352;
    16
}
/// $07158C: bclr.b #$1, $12(a6)
pub fn i_07158c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71592;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071592: lea.l $2d94f0.l, a0
pub fn i_071592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71598;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D94F0;
    20
}
/// $071598: jsr $28cd4.l
pub fn i_071598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7159E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7159E);
    cpu.pc = 0x28CD4;
    16
}
/// $07159E: move.b #$5, $84(a6)
pub fn i_07159e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0715A4: move.b #$5, $85(a6)
pub fn i_0715a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0715AA: move.b #$5, $86(a6)
pub fn i_0715aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715B0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0715B0: move.b #$5, $87(a6)
pub fn i_0715b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0715B6: move.b #$5, $83(a6)
pub fn i_0715b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0715BC: lea.l $715c2(pc), a1
pub fn i_0715bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x715C2;
    16
}
/// $0715C0: move.l a1, (a6)
pub fn i_0715c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0715C2: jsr $2783a.l
pub fn i_0715c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x715C8);
    cpu.pc = 0x2783A;
    16
}
/// $0715C8: jsr $28d70.l
pub fn i_0715c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x715CE);
    cpu.pc = 0x28D70;
    16
}
/// $0715CE: bcc.w $715d8
pub fn i_0715ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x715D8; return 10; }
    8
}
/// $0715D2: lea.l $715fa(pc), a1
pub fn i_0715d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x715FA;
    16
}
/// $0715D6: move.l a1, (a6)
pub fn i_0715d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0715D8: jsr $727c4(pc)
pub fn i_0715d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x715DC);
    cpu.pc = 0x727C4;
    16
}
/// $0715DC: movea.l #$ffffffff, a0
pub fn i_0715dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715E2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0715E2: lea.l $2d90bc.l, a0
pub fn i_0715e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90BC;
    20
}
/// $0715E8: jsr $5dd5c.l
pub fn i_0715e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x715EE);
    cpu.pc = 0x5DD5C;
    16
}
/// $0715EE: bcc.w $715f8
pub fn i_0715ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x715F8; return 10; }
    8
}
// ===== SetTaskHandler_0715f2  ($0715F2, 8 B, kernel/task_handlers.c)
/// $0715F2: lea.l $716b6(pc), a1
pub fn i_0715f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716B6;
    16
}
/// $0715F6: move.l a1, (a6)
pub fn i_0715f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0715F8: rts
pub fn i_0715f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x715FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Explode_0715fa  ($0715FA, 104 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0715FA: bclr.b #$1, $12(a6)
pub fn i_0715fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71600;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071600: move.w #$102f, d0
pub fn i_071600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71604;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071604: jsr $2352.l
pub fn i_071604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7160A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7160A);
    cpu.pc = 0x2352;
    16
}
/// $07160A: lea.l $77fd6.l, a1
pub fn i_07160a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71610;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $071610: jsr $4ae.l
pub fn i_071610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71616;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71616);
    cpu.pc = 0x4AE;
    16
}
/// $071616: jsr $5dd02.l
pub fn i_071616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7161C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7161C);
    cpu.pc = 0x5DD02;
    16
}
/// $07161C: addi.w #$ffc8, $22(a0)
pub fn i_07161c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71622;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFC8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $071622: lea.l $77fd6.l, a1
pub fn i_071622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71628;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $071628: jsr $4ae.l
pub fn i_071628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7162E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7162E);
    cpu.pc = 0x4AE;
    16
}
/// $07162E: jsr $5dd02.l
pub fn i_07162e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71634);
    cpu.pc = 0x5DD02;
    16
}
/// $071634: addi.w #$0, $22(a0)
pub fn i_071634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7163A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07163A: lea.l $77fd6.l, a1
pub fn i_07163a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71640;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $071640: jsr $4ae.l
pub fn i_071640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71646);
    cpu.pc = 0x4AE;
    16
}
/// $071646: jsr $5dd02.l
pub fn i_071646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7164C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7164C);
    cpu.pc = 0x5DD02;
    16
}
/// $07164C: addi.w #$38, $22(a0)
pub fn i_07164c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71652;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $071652: jsr $72db8(pc)
pub fn i_071652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71656;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71656);
    cpu.pc = 0x72DB8;
    16
}
/// $071656: jsr $28d70.l
pub fn i_071656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7165C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7165C);
    cpu.pc = 0x28D70;
    16
}
/// $07165C: jmp $518.l
pub fn i_07165c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71662;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== FinalBoss_Nop_071662  ($071662, 2 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071662: rts
pub fn i_071662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71664;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_HitTest_071664  ($071664, 74 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071664: jsr $7279a(pc)
pub fn i_071664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71668;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71668);
    cpu.pc = 0x7279A;
    16
}
/// $071668: jsr $727ea(pc)
pub fn i_071668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7166C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7166C);
    cpu.pc = 0x727EA;
    16
}
/// $07166C: jsr $2870a.l
pub fn i_07166c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71672;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71672);
    cpu.pc = 0x2870A;
    16
}
/// $071672: bcc.w $71688
pub fn i_071672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71676;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71688; return 10; }
    8
}
/// $071676: lea.l $5e766.l, a0
pub fn i_071676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7167C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07167C: jsr $5e770.l
pub fn i_07167c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71682;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71682);
    cpu.pc = 0x5E770;
    16
}
/// $071682: bclr.b #$3, $13(a6)
pub fn i_071682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71688;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071688: jsr $28758.l
pub fn i_071688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7168E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7168E);
    cpu.pc = 0x28758;
    16
}
/// $07168E: bcc.w $71698
pub fn i_07168e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71692;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71698; return 10; }
    8
}
/// $071692: lea.l $71582(pc), a1
pub fn i_071692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71696;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71582;
    16
}
/// $071696: move.l a1, (a6)
pub fn i_071696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71698;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071698: movea.l #$ffffffff, a0
pub fn i_071698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7169E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07169E: lea.l $2d90bc.l, a0
pub fn i_07169e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90BC;
    20
}
/// $0716A4: jsr $5dd5c.l
pub fn i_0716a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x716AA);
    cpu.pc = 0x5DD5C;
    16
}
/// $0716AA: bcc.w $716b4
pub fn i_0716aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x716B4; return 10; }
    8
}
// ===== SetTaskHandler_0716ae  ($0716AE, 8 B, kernel/task_handlers.c)
/// $0716AE: lea.l $716b6(pc), a1
pub fn i_0716ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716B6;
    16
}
/// $0716B2: move.l a1, (a6)
pub fn i_0716b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0716B4: rts
pub fn i_0716b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_ResetParts_0716b6  ($0716B6, 36 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0716B6: bclr.b #$1, $12(a6)
pub fn i_0716b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716BC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0716BC: move.b #$5, $84(a6)
pub fn i_0716bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0716C2: move.b #$5, $85(a6)
pub fn i_0716c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716C8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0716C8: move.b #$5, $86(a6)
pub fn i_0716c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716CE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0716CE: move.b #$5, $87(a6)
pub fn i_0716ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716D4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0716D4: move.b #$5, $83(a6)
pub fn i_0716d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_0716da  ($0716DA, 8 B, kernel/task_handlers.c)
/// $0716DA: lea.l $716e2(pc), a1
pub fn i_0716da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716E2;
    16
}
/// $0716DE: move.l a1, (a6)
pub fn i_0716de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0716E0: rts
pub fn i_0716e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0716e2  ($0716E2, 8 B, kernel/jmp_to_scheduler.c)
/// $0716E2: jmp $518.l
pub fn i_0716e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716E8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JmpToScheduler_0716ea  ($0716EA, 8 B, kernel/jmp_to_scheduler.c)
/// $0716EA: jmp $518.l
pub fn i_0716ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Jsr5B6ThenJmpScheduler_0716f2  ($0716F2, 14 B, kernel/jsr5b6_jmp518.c)
/// $0716F2: jsr $5b6.l
pub fn i_0716f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x716F8);
    cpu.pc = 0x5B6;
    16
}
/// $0716F8: jmp $518.l
pub fn i_0716f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716FE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== FinalBoss_PartDispatch_071700  ($071700, 126 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071700: movea.l $c(a6), a0
pub fn i_071700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71704;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $071704: move.b $77(a0), $77(a6)
pub fn i_071704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7170A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad4, v3 as u8);
    20
}
/// $07170A: jsr $72b96(pc)
pub fn i_07170a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7170E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7170E);
    cpu.pc = 0x72B96;
    16
}
/// $07170E: move.w $88(a6), d0
pub fn i_07170e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71712;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071712: cmpi.w #$5, d0
pub fn i_071712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71716;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $071716: bcs.w $71726
pub fn i_071716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7171A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x71726; return 10; }
    8
}
/// $07171A: nop
pub fn i_07171a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7171C;
    cpu.ir = 0x4E71;
    4
}
/// $07171C: nop
pub fn i_07171c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7171E;
    cpu.ir = 0x4E71;
    4
}
/// $07171E: cmpi.w #$5, d0
pub fn i_07171e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71722;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $071722: nop
pub fn i_071722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71724;
    cpu.ir = 0x4E71;
    4
}
/// $071724: trap #$f
pub fn i_071724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71726;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $071726: movea.l #$2d92a6, a0
pub fn i_071726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7172C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D92A6;
    cpu.a[0] = v1;
    4
}
/// $07172C: lsl.w #$2, d0
pub fn i_07172c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7172E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07172E: movea.l (a0, d0.w), a0
pub fn i_07172e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71732;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $071732: cmpa.l #$ffffffff, a0
pub fn i_071732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71738;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $071738: beq.w $71742
pub fn i_071738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7173C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x71742; return 10; }
    8
}
/// $07173C: jsr $28cd4.l
pub fn i_07173c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71742);
    cpu.pc = 0x28CD4;
    16
}
/// $071742: lea.l $71748(pc), a1
pub fn i_071742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71746;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71748;
    16
}
/// $071746: move.l a1, (a6)
pub fn i_071746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71748;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071748: movea.l $c(a6), a0
pub fn i_071748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7174C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07174C: move.b $20(a0), $20(a6)
pub fn i_07174c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71752;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $071752: jsr $72bce(pc)
pub fn i_071752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71756;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71756);
    cpu.pc = 0x72BCE;
    16
}
/// $071756: jsr $72bde(pc)
pub fn i_071756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7175A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7175A);
    cpu.pc = 0x72BDE;
    16
}
/// $07175A: lea.l $2d9272.l, a0
pub fn i_07175a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71760;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9272;
    20
}
/// $071760: jsr $72c08(pc)
pub fn i_071760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71764;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71764);
    cpu.pc = 0x72C08;
    16
}
/// $071764: bcc.w $7176a
pub fn i_071764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71768;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7176A; return 10; }
    8
}
/// $071768: jmp (a0)
pub fn i_071768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7176A;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $07176A: movea.l $c(a6), a0
pub fn i_07176a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7176E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07176E: btst.b #$7, $5a(a0)
pub fn i_07176e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71774;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $071774: beq.w $7177e
pub fn i_071774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71778;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7177E; return 10; }
    8
}
/// $071778: bset.b #$0, $5a(a6)
pub fn i_071778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7177E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== JsrAbsThunk_07177e  ($07177E, 8 B, thunks/jsr_abs_thunks.c)
/// $07177E: jsr $28d70.l
pub fn i_07177e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71784;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71784);
    cpu.pc = 0x28D70;
    16
}
/// $071784: rts
pub fn i_071784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71786;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_PartA_071786  ($071786, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071786: move.w $88(a6), d0
pub fn i_071786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7178A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07178A: cmpi.w #$5, d0
pub fn i_07178a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7178E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $07178E: bcs.w $7179e
pub fn i_07178e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71792;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7179E; return 10; }
    8
}
/// $071792: nop
pub fn i_071792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71794;
    cpu.ir = 0x4E71;
    4
}
/// $071794: nop
pub fn i_071794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71796;
    cpu.ir = 0x4E71;
    4
}
/// $071796: cmpi.w #$5, d0
pub fn i_071796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7179A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $07179A: nop
pub fn i_07179a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7179C;
    cpu.ir = 0x4E71;
    4
}
/// $07179C: trap #$f
pub fn i_07179c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7179E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $07179E: movea.l #$2d92ba, a0
pub fn i_07179e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717A4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D92BA;
    cpu.a[0] = v1;
    4
}
/// $0717A4: lsl.w #$2, d0
pub fn i_0717a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717A6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0717A6: movea.l (a0, d0.w), a0
pub fn i_0717a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717AA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0717AA: cmpa.l #$ffffffff, a0
pub fn i_0717aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717B0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0717B0: beq.w $717ba
pub fn i_0717b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x717BA; return 10; }
    8
}
/// $0717B4: jsr $28cd4.l
pub fn i_0717b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x717BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0717BA: bra.w $71742
pub fn i_0717ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x71742;
    10
}
// ===== FinalBoss_PartB_0717be  ($0717BE, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0717BE: move.w $88(a6), d0
pub fn i_0717be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717C2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0717C2: cmpi.w #$5, d0
pub fn i_0717c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717C6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $0717C6: bcs.w $717d6
pub fn i_0717c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717CA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x717D6; return 10; }
    8
}
/// $0717CA: nop
pub fn i_0717ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717CC;
    cpu.ir = 0x4E71;
    4
}
/// $0717CC: nop
pub fn i_0717cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717CE;
    cpu.ir = 0x4E71;
    4
}
/// $0717CE: cmpi.w #$5, d0
pub fn i_0717ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717D2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $0717D2: nop
pub fn i_0717d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717D4;
    cpu.ir = 0x4E71;
    4
}
/// $0717D4: trap #$f
pub fn i_0717d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717D6;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0717D6: movea.l #$2d92ce, a0
pub fn i_0717d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717DC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D92CE;
    cpu.a[0] = v1;
    4
}
/// $0717DC: lsl.w #$2, d0
pub fn i_0717dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717DE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0717DE: movea.l (a0, d0.w), a0
pub fn i_0717de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717E2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0717E2: cmpa.l #$ffffffff, a0
pub fn i_0717e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717E8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0717E8: beq.w $717f2
pub fn i_0717e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x717F2; return 10; }
    8
}
/// $0717EC: jsr $28cd4.l
pub fn i_0717ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x717F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0717F2: bra.w $71742
pub fn i_0717f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x71742;
    10
}
// ===== FinalBoss_PartC_0717f6  ($0717F6, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0717F6: move.w $88(a6), d0
pub fn i_0717f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0717FA: cmpi.w #$5, d0
pub fn i_0717fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x717FE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $0717FE: bcs.w $7180e
pub fn i_0717fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71802;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7180E; return 10; }
    8
}
/// $071802: nop
pub fn i_071802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71804;
    cpu.ir = 0x4E71;
    4
}
/// $071804: nop
pub fn i_071804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71806;
    cpu.ir = 0x4E71;
    4
}
/// $071806: cmpi.w #$5, d0
pub fn i_071806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7180A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $07180A: nop
pub fn i_07180a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7180C;
    cpu.ir = 0x4E71;
    4
}
/// $07180C: trap #$f
pub fn i_07180c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7180E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $07180E: movea.l #$2d92e2, a0
pub fn i_07180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71814;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D92E2;
    cpu.a[0] = v1;
    4
}
/// $071814: lsl.w #$2, d0
pub fn i_071814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71816;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $071816: movea.l (a0, d0.w), a0
pub fn i_071816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7181A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07181A: cmpa.l #$ffffffff, a0
pub fn i_07181a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71820;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $071820: beq.w $7182a
pub fn i_071820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71824;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7182A; return 10; }
    8
}
/// $071824: jsr $28cd4.l
pub fn i_071824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7182A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7182A);
    cpu.pc = 0x28CD4;
    16
}
/// $07182A: bra.w $71742
pub fn i_07182a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7182E;
    cpu.ir = 0x6000;
    cpu.pc = 0x71742;
    10
}
// ===== FinalBoss_PartD_07182e  ($07182E, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07182E: move.w $88(a6), d0
pub fn i_07182e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71832;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071832: cmpi.w #$5, d0
pub fn i_071832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71836;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $071836: bcs.w $71846
pub fn i_071836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7183A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x71846; return 10; }
    8
}
/// $07183A: nop
pub fn i_07183a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7183C;
    cpu.ir = 0x4E71;
    4
}
/// $07183C: nop
pub fn i_07183c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7183E;
    cpu.ir = 0x4E71;
    4
}
/// $07183E: cmpi.w #$5, d0
pub fn i_07183e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71842;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $071842: nop
pub fn i_071842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71844;
    cpu.ir = 0x4E71;
    4
}
/// $071844: trap #$f
pub fn i_071844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71846;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $071846: movea.l #$2d92f6, a0
pub fn i_071846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7184C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D92F6;
    cpu.a[0] = v1;
    4
}
/// $07184C: lsl.w #$2, d0
pub fn i_07184c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7184E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07184E: movea.l (a0, d0.w), a0
pub fn i_07184e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71852;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $071852: cmpa.l #$ffffffff, a0
pub fn i_071852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71858;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $071858: beq.w $71862
pub fn i_071858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7185C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x71862; return 10; }
    8
}
/// $07185C: jsr $28cd4.l
pub fn i_07185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71862;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71862);
    cpu.pc = 0x28CD4;
    16
}
/// $071862: bra.w $71742
pub fn i_071862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71866;
    cpu.ir = 0x6000;
    cpu.pc = 0x71742;
    10
}
// ===== SetTaskHandler_071866  ($071866, 8 B, kernel/task_handlers.c)
/// $071866: lea.l $716ea(pc), a1
pub fn i_071866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7186A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716EA;
    16
}
/// $07186A: move.l a1, (a6)
pub fn i_07186a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7186C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07186C: rts
pub fn i_07186c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7186E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Limb1_07186e  ($07186E, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $07186E: move.w #$0, $84(a6)
pub fn i_07186e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71874;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071874: move.w #$84, $86(a6)
pub fn i_071874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7187A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07187A: move.l #$2d9138, $80(a6)
pub fn i_07187a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71882;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D9138;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $071882: move.w #$0, $88(a6)
pub fn i_071882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71888;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071888: lea.l $2d9508.l, a0
pub fn i_071888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7188E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9508;
    20
}
/// $07188E: jsr $28cd4.l
pub fn i_07188e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71894);
    cpu.pc = 0x28CD4;
    16
}
/// $071894: bra.w $71700
pub fn i_071894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71898;
    cpu.ir = 0x6000;
    cpu.pc = 0x71700;
    10
}
// ===== FinalBoss_Limb2_071898  ($071898, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071898: move.w #$ffff, $84(a6)
pub fn i_071898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7189E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07189E: move.w #$85, $86(a6)
pub fn i_07189e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x85;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0718A4: move.l #$2d9138, $80(a6)
pub fn i_0718a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718AC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D9138;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0718AC: move.w #$1, $88(a6)
pub fn i_0718ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0718B2: lea.l $2d961c.l, a0
pub fn i_0718b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D961C;
    20
}
/// $0718B8: jsr $28cd4.l
pub fn i_0718b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x718BE);
    cpu.pc = 0x28CD4;
    16
}
/// $0718BE: bra.w $71700
pub fn i_0718be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x71700;
    10
}
// ===== FinalBoss_Limb3_0718c2  ($0718C2, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0718C2: move.w #$ffff, $84(a6)
pub fn i_0718c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0718C8: move.w #$86, $86(a6)
pub fn i_0718c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x86;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0718CE: move.l #$2d9168, $80(a6)
pub fn i_0718ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718D6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D9168;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0718D6: move.w #$2, $88(a6)
pub fn i_0718d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0718DC: lea.l $2d9714.l, a0
pub fn i_0718dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9714;
    20
}
/// $0718E2: jsr $28cd4.l
pub fn i_0718e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x718E8);
    cpu.pc = 0x28CD4;
    16
}
/// $0718E8: bra.w $71700
pub fn i_0718e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x71700;
    10
}
// ===== FinalBoss_Limb4_0718ec  ($0718EC, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0718EC: move.w #$ffff, $84(a6)
pub fn i_0718ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0718F2: move.w #$87, $86(a6)
pub fn i_0718f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x718F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0718F8: move.l #$2d9198, $80(a6)
pub fn i_0718f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71900;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D9198;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $071900: move.w #$3, $88(a6)
pub fn i_071900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71906;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071906: lea.l $2d9828.l, a0
pub fn i_071906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7190C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9828;
    20
}
/// $07190C: jsr $28cd4.l
pub fn i_07190c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71912);
    cpu.pc = 0x28CD4;
    16
}
/// $071912: bra.w $71700
pub fn i_071912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71916;
    cpu.ir = 0x6000;
    cpu.pc = 0x71700;
    10
}
// ===== FinalBoss_Limb5_071916  ($071916, 42 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071916: move.w #$ffff, $84(a6)
pub fn i_071916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7191C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07191C: move.w #$87, $86(a6)
pub fn i_07191c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71922;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071922: move.l #$2d9198, $80(a6)
pub fn i_071922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7192A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D9198;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07192A: move.w #$4, $88(a6)
pub fn i_07192a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71930;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071930: lea.l $2d990a.l, a0
pub fn i_071930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71936;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D990A;
    20
}
/// $071936: jsr $28cd4.l
pub fn i_071936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7193C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7193C);
    cpu.pc = 0x28CD4;
    16
}
/// $07193C: bra.w $71700
pub fn i_07193c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71940;
    cpu.ir = 0x6000;
    cpu.pc = 0x71700;
    10
}
// ===== FinalBoss_Head_071940  ($071940, 54 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071940: move.w #$0, $84(a6)
pub fn i_071940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71946;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071946: move.w #$83, $86(a6)
pub fn i_071946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7194C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07194C: move.l #$2d91c8, $80(a6)
pub fn i_07194c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71954;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D91C8;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $071954: move.w #$5, $88(a6)
pub fn i_071954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7195A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07195A: jsr $5e9b6.l
pub fn i_07195a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71960);
    cpu.pc = 0x5E9B6;
    16
}
/// $071960: andi.w #$3f, d0
pub fn i_071960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71964;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $071964: addi.w #$1e, d0
pub fn i_071964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71968;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $071968: move.w d0, $72(a6)
pub fn i_071968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7196C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07196C: move.w #$38, d1
pub fn i_07196c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71970;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071970: jsr $236e.l
pub fn i_071970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71976);
    cpu.pc = 0x236E;
    16
}
// ===== FinalBoss_HeadIdle_071976  ($071976, 114 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071976: move.b #$1, $8a(a6)
pub fn i_071976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7197C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07197C: move.w #$1, $66(a6)
pub fn i_07197c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71982;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071982: bclr.b #$3, $13(a6)
pub fn i_071982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71988;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071988: bclr.b #$0, $13(a6)
pub fn i_071988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7198E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07198E: lea.l $2da2d0.l, a0
pub fn i_07198e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71994;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DA2D0;
    20
}
/// $071994: jsr $28cd4.l
pub fn i_071994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7199A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7199A);
    cpu.pc = 0x28CD4;
    16
}
/// $07199A: jsr $28d70.l
pub fn i_07199a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x719A0);
    cpu.pc = 0x28D70;
    16
}
/// $0719A0: lea.l $719a6(pc), a1
pub fn i_0719a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x719A6;
    16
}
/// $0719A4: move.l a1, (a6)
pub fn i_0719a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0719A6: movea.l $c(a6), a0
pub fn i_0719a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719AA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0719AA: move.b $20(a0), $20(a6)
pub fn i_0719aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719B0;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0719B0: jsr $72b5a(pc)
pub fn i_0719b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x719B4);
    cpu.pc = 0x72B5A;
    16
}
/// $0719B4: bcc.w $719be
pub fn i_0719b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x719BE; return 10; }
    8
}
/// $0719B8: lea.l $71a7a(pc), a1
pub fn i_0719b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71A7A;
    16
}
/// $0719BC: move.l a1, (a6)
pub fn i_0719bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0719BE: movea.l $c(a6), a1
pub fn i_0719be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719C2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0719C2: move.w $86(a6), d1
pub fn i_0719c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719C6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0719C6: cmpi.b #$5, (a1, d1.w)
pub fn i_0719c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719CC;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    18
}
/// $0719CC: bne.w $719d6
pub fn i_0719cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x719D6; return 10; }
    8
}
/// $0719D0: lea.l $71b9a(pc), a1
pub fn i_0719d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71B9A;
    16
}
/// $0719D4: move.l a1, (a6)
pub fn i_0719d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0719D6: lea.l $2d928e.l, a0
pub fn i_0719d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D928E;
    20
}
/// $0719DC: jsr $72c08(pc)
pub fn i_0719dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x719E0);
    cpu.pc = 0x72C08;
    16
}
/// $0719E0: bcc.w $719e6
pub fn i_0719e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x719E6; return 10; }
    8
}
/// $0719E4: jmp (a0)
pub fn i_0719e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719E6;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $0719E6: rts
pub fn i_0719e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_HeadHit_0719e8  ($0719E8, 94 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $0719E8: move.w #$1, $66(a6)
pub fn i_0719e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0719EE: bclr.b #$3, $13(a6)
pub fn i_0719ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0719F4: bclr.b #$0, $13(a6)
pub fn i_0719f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719FA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0719FA: lea.l $71a00(pc), a1
pub fn i_0719fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x719FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71A00;
    16
}
/// $0719FE: move.l a1, (a6)
pub fn i_0719fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071A00: jsr $72bce(pc)
pub fn i_071a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71A04);
    cpu.pc = 0x72BCE;
    16
}
/// $071A04: jsr $72bde(pc)
pub fn i_071a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71A08);
    cpu.pc = 0x72BDE;
    16
}
/// $071A08: jsr $28d70.l
pub fn i_071a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71A0E);
    cpu.pc = 0x28D70;
    16
}
/// $071A0E: bcc.w $71a18
pub fn i_071a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71A18; return 10; }
    8
}
/// $071A12: lea.l $71976(pc), a1
pub fn i_071a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71976;
    16
}
/// $071A16: move.l a1, (a6)
pub fn i_071a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071A18: jsr $2870a.l
pub fn i_071a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71A1E);
    cpu.pc = 0x2870A;
    16
}
/// $071A1E: bcc.w $71a2e
pub fn i_071a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71A2E; return 10; }
    8
}
/// $071A22: jsr $519be.l
pub fn i_071a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71A28);
    cpu.pc = 0x519BE;
    16
}
/// $071A28: lea.l $71a90(pc), a1
pub fn i_071a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71A90;
    16
}
/// $071A2C: move.l a1, (a6)
pub fn i_071a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071A2E: movea.l $c(a6), a1
pub fn i_071a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A32;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $071A32: move.w $86(a6), d1
pub fn i_071a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A36;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071A36: move.b (a1, d1.w), d0
pub fn i_071a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A3A;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $071A3A: clr.b (a1, d1.w)
pub fn i_071a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A3E;
    cpu.ir = 0x4231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $071A3E: cmpi.b #$5, d0
pub fn i_071a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A42;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $071A42: bne.w $71a4c
pub fn i_071a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71A4C; return 10; }
    8
}
// ===== SetTaskHandler_071a46  ($071A46, 8 B, kernel/task_handlers.c)
/// $071A46: lea.l $71b9a(pc), a1
pub fn i_071a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71B9A;
    16
}
/// $071A4A: move.l a1, (a6)
pub fn i_071a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071A4C: rts
pub fn i_071a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_HeadDie_071a90  ($071A90, 108 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071A90: cmpi.b #$4, $8a(a6)
pub fn i_071a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A96;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $071A96: bne.w $71aaa
pub fn i_071a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A9A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71AAA; return 10; }
    8
}
/// $071A9A: lea.l $7229c(pc), a1
pub fn i_071a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7229C;
    16
}
/// $071A9E: jsr $4ae.l
pub fn i_071a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71AA4);
    cpu.pc = 0x4AE;
    16
}
/// $071AA4: jsr $5dd02.l
pub fn i_071aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71AAA);
    cpu.pc = 0x5DD02;
    16
}
/// $071AAA: move.b #$5, $8a(a6)
pub fn i_071aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $071AB0: move.w #$1041, d0
pub fn i_071ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1041;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071AB4: jsr $2352.l
pub fn i_071ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ABA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71ABA);
    cpu.pc = 0x2352;
    16
}
/// $071ABA: lea.l $2c41ea.l, a0
pub fn i_071aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C41EA;
    20
}
/// $071AC0: jsr $28cd4.l
pub fn i_071ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71AC6);
    cpu.pc = 0x28CD4;
    16
}
/// $071AC6: lea.l $71acc(pc), a1
pub fn i_071ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ACA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71ACC;
    16
}
/// $071ACA: move.l a1, (a6)
pub fn i_071aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ACC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071ACC: jsr $72bce(pc)
pub fn i_071acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71AD0);
    cpu.pc = 0x72BCE;
    16
}
/// $071AD0: jsr $72bde(pc)
pub fn i_071ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71AD4);
    cpu.pc = 0x72BDE;
    16
}
/// $071AD4: jsr $28d70.l
pub fn i_071ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ADA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71ADA);
    cpu.pc = 0x28D70;
    16
}
/// $071ADA: bcc.w $71ae4
pub fn i_071ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ADE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71AE4; return 10; }
    8
}
/// $071ADE: lea.l $71b04(pc), a1
pub fn i_071ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71B04;
    16
}
/// $071AE2: move.l a1, (a6)
pub fn i_071ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071AE4: movea.l $c(a6), a1
pub fn i_071ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AE8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $071AE8: move.w $86(a6), d1
pub fn i_071ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AEC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071AEC: move.b (a1, d1.w), d0
pub fn i_071aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AF0;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $071AF0: clr.b (a1, d1.w)
pub fn i_071af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AF4;
    cpu.ir = 0x4231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $071AF4: cmpi.b #$5, d0
pub fn i_071af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AF8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $071AF8: bne.w $71b02
pub fn i_071af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71AFC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71B02; return 10; }
    8
}
// ===== SetTaskHandler_071afc  ($071AFC, 8 B, kernel/task_handlers.c)
/// $071AFC: lea.l $71b9a(pc), a1
pub fn i_071afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71B9A;
    16
}
/// $071B00: move.l a1, (a6)
pub fn i_071b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071B02: rts
pub fn i_071b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_HeadFlash_071b04  ($071B04, 74 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071B04: move.b #$3c, $70(a6)
pub fn i_071b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B0A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x070004, i_070004),
    (0x07000A, i_07000a),
    (0x070010, i_070010),
    (0x070014, i_070014),
    (0x070016, i_070016),
    (0x07001C, i_07001c),
    (0x070022, i_070022),
    (0x070026, i_070026),
    (0x07002A, i_07002a),
    (0x07002C, i_07002c),
    (0x070032, i_070032),
    (0x070038, i_070038),
    (0x07003C, i_07003c),
    (0x070040, i_070040),
    (0x070042, i_070042),
    (0x070046, i_070046),
    (0x07004A, i_07004a),
    (0x07004E, i_07004e),
    (0x070052, i_070052),
    (0x070054, i_070054),
    (0x070058, i_070058),
    (0x07005C, i_07005c),
    (0x07005E, i_07005e),
    (0x070064, i_070064),
    (0x07006A, i_07006a),
    (0x070070, i_070070),
    (0x070074, i_070074),
    (0x070078, i_070078),
    (0x07007E, i_07007e),
    (0x070084, i_070084),
    (0x07008A, i_07008a),
    (0x07008E, i_07008e),
    (0x070094, i_070094),
    (0x07009A, i_07009a),
    (0x0700A0, i_0700a0),
    (0x0700A6, i_0700a6),
    (0x0700AC, i_0700ac),
    (0x0700AE, i_0700ae),
    (0x0700B2, i_0700b2),
    (0x0700B8, i_0700b8),
    (0x0700BE, i_0700be),
    (0x0700C4, i_0700c4),
    (0x0700C8, i_0700c8),
    (0x0700CA, i_0700ca),
    (0x0700D0, i_0700d0),
    (0x0700D6, i_0700d6),
    (0x0700DC, i_0700dc),
    (0x0700E2, i_0700e2),
    (0x0700E6, i_0700e6),
    (0x0700EC, i_0700ec),
    (0x0700F0, i_0700f0),
    (0x0700F6, i_0700f6),
    (0x0700FC, i_0700fc),
    (0x070102, i_070102),
    (0x070106, i_070106),
    (0x07010A, i_07010a),
    (0x07010C, i_07010c),
    (0x07010E, i_07010e),
    (0x070112, i_070112),
    (0x070118, i_070118),
    (0x07011E, i_07011e),
    (0x070124, i_070124),
    (0x070128, i_070128),
    (0x07012A, i_07012a),
    (0x070130, i_070130),
    (0x070136, i_070136),
    (0x07013C, i_07013c),
    (0x070142, i_070142),
    (0x070146, i_070146),
    (0x07014C, i_07014c),
    (0x070150, i_070150),
    (0x070156, i_070156),
    (0x07015C, i_07015c),
    (0x070162, i_070162),
    (0x070166, i_070166),
    (0x07016A, i_07016a),
    (0x07016C, i_07016c),
    (0x07016E, i_07016e),
    (0x070174, i_070174),
    (0x070178, i_070178),
    (0x07017C, i_07017c),
    (0x070180, i_070180),
    (0x070184, i_070184),
    (0x070186, i_070186),
    (0x07018A, i_07018a),
    (0x07018E, i_07018e),
    (0x070192, i_070192),
    (0x070196, i_070196),
    (0x07019C, i_07019c),
    (0x0701A0, i_0701a0),
    (0x0701A6, i_0701a6),
    (0x0701AA, i_0701aa),
    (0x0701AE, i_0701ae),
    (0x0701B2, i_0701b2),
    (0x0701B4, i_0701b4),
    (0x0701BA, i_0701ba),
    (0x0701BE, i_0701be),
    (0x0701C2, i_0701c2),
    (0x0701C6, i_0701c6),
    (0x0701C8, i_0701c8),
    (0x0701CC, i_0701cc),
    (0x0701D0, i_0701d0),
    (0x0701D6, i_0701d6),
    (0x0701DC, i_0701dc),
    (0x0701E2, i_0701e2),
    (0x0701E8, i_0701e8),
    (0x0701EE, i_0701ee),
    (0x0701F2, i_0701f2),
    (0x0701F6, i_0701f6),
    (0x0701F8, i_0701f8),
    (0x0701FC, i_0701fc),
    (0x070202, i_070202),
    (0x070206, i_070206),
    (0x07020A, i_07020a),
    (0x07020C, i_07020c),
    (0x07020E, i_07020e),
    (0x070212, i_070212),
    (0x070214, i_070214),
    (0x07021A, i_07021a),
    (0x070220, i_070220),
    (0x070224, i_070224),
    (0x070228, i_070228),
    (0x07022A, i_07022a),
    (0x07022C, i_07022c),
    (0x07022E, i_07022e),
    (0x070230, i_070230),
    (0x070234, i_070234),
    (0x070238, i_070238),
    (0x07023C, i_07023c),
    (0x070240, i_070240),
    (0x070244, i_070244),
    (0x070246, i_070246),
    (0x07024C, i_07024c),
    (0x070250, i_070250),
    (0x070256, i_070256),
    (0x07025C, i_07025c),
    (0x070262, i_070262),
    (0x070268, i_070268),
    (0x07026C, i_07026c),
    (0x07026E, i_07026e),
    (0x070274, i_070274),
    (0x07027A, i_07027a),
    (0x07027E, i_07027e),
    (0x070284, i_070284),
    (0x070288, i_070288),
    (0x07028C, i_07028c),
    (0x07028E, i_07028e),
    (0x070290, i_070290),
    (0x070294, i_070294),
    (0x070296, i_070296),
    (0x07029C, i_07029c),
    (0x0702A0, i_0702a0),
    (0x0702A4, i_0702a4),
    (0x0702A6, i_0702a6),
    (0x0702A8, i_0702a8),
    (0x0702AC, i_0702ac),
    (0x0702B2, i_0702b2),
    (0x0702B6, i_0702b6),
    (0x0702BC, i_0702bc),
    (0x0702C2, i_0702c2),
    (0x0702C8, i_0702c8),
    (0x0702CC, i_0702cc),
    (0x0702CE, i_0702ce),
    (0x0702D2, i_0702d2),
    (0x0702D8, i_0702d8),
    (0x0702DC, i_0702dc),
    (0x0702E0, i_0702e0),
    (0x0702E2, i_0702e2),
    (0x0702E4, i_0702e4),
    (0x0702E8, i_0702e8),
    (0x0702EE, i_0702ee),
    (0x0702F4, i_0702f4),
    (0x0702FA, i_0702fa),
    (0x0702FC, i_0702fc),
    (0x070302, i_070302),
    (0x070306, i_070306),
    (0x07030C, i_07030c),
    (0x070312, i_070312),
    (0x070318, i_070318),
    (0x07031C, i_07031c),
    (0x070322, i_070322),
    (0x070328, i_070328),
    (0x07032C, i_07032c),
    (0x07032E, i_07032e),
    (0x070332, i_070332),
    (0x070338, i_070338),
    (0x07033C, i_07033c),
    (0x070342, i_070342),
    (0x070346, i_070346),
    (0x07034A, i_07034a),
    (0x07034C, i_07034c),
    (0x070350, i_070350),
    (0x070356, i_070356),
    (0x07035C, i_07035c),
    (0x070362, i_070362),
    (0x070366, i_070366),
    (0x070368, i_070368),
    (0x07036C, i_07036c),
    (0x070372, i_070372),
    (0x070376, i_070376),
    (0x07037A, i_07037a),
    (0x07037C, i_07037c),
    (0x070380, i_070380),
    (0x070386, i_070386),
    (0x07038A, i_07038a),
    (0x070390, i_070390),
    (0x070392, i_070392),
    (0x070396, i_070396),
    (0x07039A, i_07039a),
    (0x07039E, i_07039e),
    (0x0703A2, i_0703a2),
    (0x0703A6, i_0703a6),
    (0x0703A8, i_0703a8),
    (0x0703AA, i_0703aa),
    (0x0703AE, i_0703ae),
    (0x0703B2, i_0703b2),
    (0x0703B6, i_0703b6),
    (0x0703BA, i_0703ba),
    (0x0703BE, i_0703be),
    (0x0703C2, i_0703c2),
    (0x0703C6, i_0703c6),
    (0x0703CA, i_0703ca),
    (0x0703CE, i_0703ce),
    (0x0703D2, i_0703d2),
    (0x0703D6, i_0703d6),
    (0x0703DC, i_0703dc),
    (0x0703E2, i_0703e2),
    (0x0703E6, i_0703e6),
    (0x0703E8, i_0703e8),
    (0x0703EC, i_0703ec),
    (0x0703F2, i_0703f2),
    (0x0703F6, i_0703f6),
    (0x0703FA, i_0703fa),
    (0x0703FC, i_0703fc),
    (0x070400, i_070400),
    (0x070404, i_070404),
    (0x07040A, i_07040a),
    (0x07040E, i_07040e),
    (0x070412, i_070412),
    (0x070414, i_070414),
    (0x070418, i_070418),
    (0x07041E, i_07041e),
    (0x070424, i_070424),
    (0x070428, i_070428),
    (0x07042A, i_07042a),
    (0x07042E, i_07042e),
    (0x070434, i_070434),
    (0x070468, i_070468),
    (0x07046E, i_07046e),
    (0x070474, i_070474),
    (0x070478, i_070478),
    (0x07047A, i_07047a),
    (0x07047E, i_07047e),
    (0x070484, i_070484),
    (0x070488, i_070488),
    (0x07048C, i_07048c),
    (0x07048E, i_07048e),
    (0x070492, i_070492),
    (0x070498, i_070498),
    (0x07049C, i_07049c),
    (0x0704A0, i_0704a0),
    (0x0704A6, i_0704a6),
    (0x0704AA, i_0704aa),
    (0x0704B0, i_0704b0),
    (0x0704B6, i_0704b6),
    (0x0704BC, i_0704bc),
    (0x0704C0, i_0704c0),
    (0x0704C4, i_0704c4),
    (0x0704CA, i_0704ca),
    (0x0704CC, i_0704cc),
    (0x0704D0, i_0704d0),
    (0x0704D6, i_0704d6),
    (0x0704DA, i_0704da),
    (0x0704DE, i_0704de),
    (0x0704E0, i_0704e0),
    (0x0704E6, i_0704e6),
    (0x0704EA, i_0704ea),
    (0x0704EE, i_0704ee),
    (0x0704F0, i_0704f0),
    (0x0704F2, i_0704f2),
    (0x0704FA, i_0704fa),
    (0x0704FE, i_0704fe),
    (0x070504, i_070504),
    (0x07050A, i_07050a),
    (0x070510, i_070510),
    (0x070514, i_070514),
    (0x07051A, i_07051a),
    (0x070520, i_070520),
    (0x070524, i_070524),
    (0x07052A, i_07052a),
    (0x070530, i_070530),
    (0x070534, i_070534),
    (0x070536, i_070536),
    (0x07053A, i_07053a),
    (0x070540, i_070540),
    (0x070544, i_070544),
    (0x070548, i_070548),
    (0x07054C, i_07054c),
    (0x070552, i_070552),
    (0x070556, i_070556),
    (0x07055A, i_07055a),
    (0x07055C, i_07055c),
    (0x070562, i_070562),
    (0x070566, i_070566),
    (0x07056A, i_07056a),
    (0x07056C, i_07056c),
    (0x070570, i_070570),
    (0x070574, i_070574),
    (0x070578, i_070578),
    (0x07057C, i_07057c),
    (0x070580, i_070580),
    (0x070584, i_070584),
    (0x07058A, i_07058a),
    (0x070590, i_070590),
    (0x070594, i_070594),
    (0x070598, i_070598),
    (0x07059C, i_07059c),
    (0x0705A0, i_0705a0),
    (0x0705A4, i_0705a4),
    (0x0705A8, i_0705a8),
    (0x0705AA, i_0705aa),
    (0x0705AE, i_0705ae),
    (0x0705B2, i_0705b2),
    (0x0705B8, i_0705b8),
    (0x0705BE, i_0705be),
    (0x0705C2, i_0705c2),
    (0x0705C8, i_0705c8),
    (0x0705CC, i_0705cc),
    (0x0705D2, i_0705d2),
    (0x0705D8, i_0705d8),
    (0x0705DC, i_0705dc),
    (0x0705E0, i_0705e0),
    (0x0705E2, i_0705e2),
    (0x0705E8, i_0705e8),
    (0x0705EA, i_0705ea),
    (0x0705EE, i_0705ee),
    (0x0705F4, i_0705f4),
    (0x0705F8, i_0705f8),
    (0x0705FE, i_0705fe),
    (0x070602, i_070602),
    (0x070604, i_070604),
    (0x070608, i_070608),
    (0x07060E, i_07060e),
    (0x070612, i_070612),
    (0x070616, i_070616),
    (0x070618, i_070618),
    (0x07061C, i_07061c),
    (0x070622, i_070622),
    (0x070626, i_070626),
    (0x07062A, i_07062a),
    (0x07062C, i_07062c),
    (0x070630, i_070630),
    (0x070634, i_070634),
    (0x070638, i_070638),
    (0x07063E, i_07063e),
    (0x070644, i_070644),
    (0x07064A, i_07064a),
    (0x07064E, i_07064e),
    (0x070650, i_070650),
    (0x070654, i_070654),
    (0x07065A, i_07065a),
    (0x07065E, i_07065e),
    (0x070664, i_070664),
    (0x070668, i_070668),
    (0x07066A, i_07066a),
    (0x07066E, i_07066e),
    (0x070674, i_070674),
    (0x070678, i_070678),
    (0x07067C, i_07067c),
    (0x07067E, i_07067e),
    (0x070682, i_070682),
    (0x070688, i_070688),
    (0x07068C, i_07068c),
    (0x070690, i_070690),
    (0x070692, i_070692),
    (0x070694, i_070694),
    (0x070698, i_070698),
    (0x07069C, i_07069c),
    (0x0706A2, i_0706a2),
    (0x0706A8, i_0706a8),
    (0x0706AE, i_0706ae),
    (0x0706B6, i_0706b6),
    (0x0706BC, i_0706bc),
    (0x0706C2, i_0706c2),
    (0x0706C8, i_0706c8),
    (0x0706CE, i_0706ce),
    (0x0706D2, i_0706d2),
    (0x0706D8, i_0706d8),
    (0x0706DE, i_0706de),
    (0x0706E4, i_0706e4),
    (0x0706EA, i_0706ea),
    (0x0706F0, i_0706f0),
    (0x0706F6, i_0706f6),
    (0x0706FC, i_0706fc),
    (0x070700, i_070700),
    (0x070702, i_070702),
    (0x070708, i_070708),
    (0x07070E, i_07070e),
    (0x070712, i_070712),
    (0x070716, i_070716),
    (0x070718, i_070718),
    (0x07071E, i_07071e),
    (0x070724, i_070724),
    (0x070728, i_070728),
    (0x07072C, i_07072c),
    (0x070730, i_070730),
    (0x070734, i_070734),
    (0x070738, i_070738),
    (0x07073A, i_07073a),
    (0x07073C, i_07073c),
    (0x070742, i_070742),
    (0x070748, i_070748),
    (0x07074E, i_07074e),
    (0x070754, i_070754),
    (0x07075A, i_07075a),
    (0x07075E, i_07075e),
    (0x070760, i_070760),
    (0x070766, i_070766),
    (0x07076A, i_07076a),
    (0x070770, i_070770),
    (0x070776, i_070776),
    (0x07077C, i_07077c),
    (0x070780, i_070780),
    (0x070782, i_070782),
    (0x070788, i_070788),
    (0x07078E, i_07078e),
    (0x070792, i_070792),
    (0x070796, i_070796),
    (0x07079A, i_07079a),
    (0x07079C, i_07079c),
    (0x07079E, i_07079e),
    (0x0707A4, i_0707a4),
    (0x0707AA, i_0707aa),
    (0x0707AE, i_0707ae),
    (0x0707B0, i_0707b0),
    (0x0707B6, i_0707b6),
    (0x0707BC, i_0707bc),
    (0x0707C0, i_0707c0),
    (0x0707C4, i_0707c4),
    (0x0707C6, i_0707c6),
    (0x0707C8, i_0707c8),
    (0x0707D0, i_0707d0),
    (0x0707D4, i_0707d4),
    (0x0707DA, i_0707da),
    (0x0707DE, i_0707de),
    (0x0707E2, i_0707e2),
    (0x0707E4, i_0707e4),
    (0x0707EA, i_0707ea),
    (0x0707EE, i_0707ee),
    (0x0707F4, i_0707f4),
    (0x0707FA, i_0707fa),
    (0x0707FE, i_0707fe),
    (0x070800, i_070800),
    (0x070804, i_070804),
    (0x07080A, i_07080a),
    (0x07080E, i_07080e),
    (0x070812, i_070812),
    (0x070814, i_070814),
    (0x070816, i_070816),
    (0x07081A, i_07081a),
    (0x070820, i_070820),
    (0x070824, i_070824),
    (0x070828, i_070828),
    (0x07082A, i_07082a),
    (0x070830, i_070830),
    (0x070834, i_070834),
    (0x07083A, i_07083a),
    (0x070840, i_070840),
    (0x070846, i_070846),
    (0x07084A, i_07084a),
    (0x070850, i_070850),
    (0x070854, i_070854),
    (0x070858, i_070858),
    (0x07085A, i_07085a),
    (0x07085C, i_07085c),
    (0x070860, i_070860),
    (0x070866, i_070866),
    (0x07086C, i_07086c),
    (0x070874, i_070874),
    (0x070878, i_070878),
    (0x07087E, i_07087e),
    (0x070884, i_070884),
    (0x070888, i_070888),
    (0x07088E, i_07088e),
    (0x070892, i_070892),
    (0x070894, i_070894),
    (0x07089A, i_07089a),
    (0x0708A0, i_0708a0),
    (0x0708A6, i_0708a6),
    (0x0708AA, i_0708aa),
    (0x0708AE, i_0708ae),
    (0x0708B0, i_0708b0),
    (0x0708B2, i_0708b2),
    (0x0708B8, i_0708b8),
    (0x0708BE, i_0708be),
    (0x0708C0, i_0708c0),
    (0x0708C4, i_0708c4),
    (0x0708CA, i_0708ca),
    (0x0708D0, i_0708d0),
    (0x0708D6, i_0708d6),
    (0x0708DA, i_0708da),
    (0x0708DC, i_0708dc),
    (0x0708E2, i_0708e2),
    (0x0708E8, i_0708e8),
    (0x0708EC, i_0708ec),
    (0x0708F0, i_0708f0),
    (0x0708F2, i_0708f2),
    (0x0708F4, i_0708f4),
    (0x0708F8, i_0708f8),
    (0x0708FE, i_0708fe),
    (0x070904, i_070904),
    (0x07090A, i_07090a),
    (0x07090E, i_07090e),
    (0x070910, i_070910),
    (0x070916, i_070916),
    (0x07091A, i_07091a),
    (0x07091E, i_07091e),
    (0x070920, i_070920),
    (0x070922, i_070922),
    (0x070926, i_070926),
    (0x07092C, i_07092c),
    (0x070932, i_070932),
    (0x070938, i_070938),
    (0x07093C, i_07093c),
    (0x07093E, i_07093e),
    (0x070944, i_070944),
    (0x07094A, i_07094a),
    (0x07094E, i_07094e),
    (0x070952, i_070952),
    (0x070954, i_070954),
    (0x070956, i_070956),
    (0x07095C, i_07095c),
    (0x070962, i_070962),
    (0x070968, i_070968),
    (0x07096C, i_07096c),
    (0x070972, i_070972),
    (0x070976, i_070976),
    (0x07097C, i_07097c),
    (0x070982, i_070982),
    (0x070986, i_070986),
    (0x07098C, i_07098c),
    (0x070992, i_070992),
    (0x070998, i_070998),
    (0x07099E, i_07099e),
    (0x0709A4, i_0709a4),
    (0x0709AA, i_0709aa),
    (0x0709AE, i_0709ae),
    (0x0709B0, i_0709b0),
    (0x0709B6, i_0709b6),
    (0x0709BC, i_0709bc),
    (0x0709C0, i_0709c0),
    (0x0709C4, i_0709c4),
    (0x0709C6, i_0709c6),
    (0x0709CE, i_0709ce),
    (0x0709D2, i_0709d2),
    (0x0709D6, i_0709d6),
    (0x0709DA, i_0709da),
    (0x0709DE, i_0709de),
    (0x0709E2, i_0709e2),
    (0x0709E8, i_0709e8),
    (0x0709EE, i_0709ee),
    (0x0709F4, i_0709f4),
    (0x0709F8, i_0709f8),
    (0x0709FC, i_0709fc),
    (0x0709FE, i_0709fe),
    (0x070A04, i_070a04),
    (0x070A0A, i_070a0a),
    (0x070A10, i_070a10),
    (0x070A14, i_070a14),
    (0x070A18, i_070a18),
    (0x070A1A, i_070a1a),
    (0x070A20, i_070a20),
    (0x070A26, i_070a26),
    (0x070A2A, i_070a2a),
    (0x070A2E, i_070a2e),
    (0x070A30, i_070a30),
    (0x070A32, i_070a32),
    (0x070A36, i_070a36),
    (0x070A3A, i_070a3a),
    (0x070A3E, i_070a3e),
    (0x070A40, i_070a40),
    (0x070A46, i_070a46),
    (0x070A4C, i_070a4c),
    (0x070A50, i_070a50),
    (0x070A56, i_070a56),
    (0x070A5A, i_070a5a),
    (0x070A60, i_070a60),
    (0x070A64, i_070a64),
    (0x070A6A, i_070a6a),
    (0x070A6E, i_070a6e),
    (0x070A72, i_070a72),
    (0x070A74, i_070a74),
    (0x070A78, i_070a78),
    (0x070A7E, i_070a7e),
    (0x070A82, i_070a82),
    (0x070A86, i_070a86),
    (0x070A8A, i_070a8a),
    (0x070A90, i_070a90),
    (0x070A94, i_070a94),
    (0x070A96, i_070a96),
    (0x070A9C, i_070a9c),
    (0x070AA2, i_070aa2),
    (0x070AA6, i_070aa6),
    (0x070AAA, i_070aaa),
    (0x070AAC, i_070aac),
    (0x070AB0, i_070ab0),
    (0x070AB6, i_070ab6),
    (0x070ABC, i_070abc),
    (0x070AC0, i_070ac0),
    (0x070AC6, i_070ac6),
    (0x070ACA, i_070aca),
    (0x070ACC, i_070acc),
    (0x070AD2, i_070ad2),
    (0x070AD6, i_070ad6),
    (0x070AD8, i_070ad8),
    (0x070ADC, i_070adc),
    (0x070AE0, i_070ae0),
    (0x070AE2, i_070ae2),
    (0x070AE8, i_070ae8),
    (0x070AEC, i_070aec),
    (0x070AEE, i_070aee),
    (0x070AF2, i_070af2),
    (0x070AF6, i_070af6),
    (0x070AFA, i_070afa),
    (0x070AFE, i_070afe),
    (0x070B02, i_070b02),
    (0x070B04, i_070b04),
    (0x070B06, i_070b06),
    (0x070B0A, i_070b0a),
    (0x070B0E, i_070b0e),
    (0x070B10, i_070b10),
    (0x070B12, i_070b12),
    (0x070B16, i_070b16),
    (0x070B1C, i_070b1c),
    (0x070B1E, i_070b1e),
    (0x070B24, i_070b24),
    (0x070B2A, i_070b2a),
    (0x070B2E, i_070b2e),
    (0x070B32, i_070b32),
    (0x070B36, i_070b36),
    (0x070B38, i_070b38),
    (0x070B3A, i_070b3a),
    (0x070B3C, i_070b3c),
    (0x070B3E, i_070b3e),
    (0x070B42, i_070b42),
    (0x070B46, i_070b46),
    (0x070B4C, i_070b4c),
    (0x070B52, i_070b52),
    (0x070B56, i_070b56),
    (0x070B5C, i_070b5c),
    (0x070B60, i_070b60),
    (0x070B64, i_070b64),
    (0x070B66, i_070b66),
    (0x070B6A, i_070b6a),
    (0x070B6E, i_070b6e),
    (0x070B70, i_070b70),
    (0x070B74, i_070b74),
    (0x070B78, i_070b78),
    (0x070B7C, i_070b7c),
    (0x070B80, i_070b80),
    (0x070B84, i_070b84),
    (0x070B86, i_070b86),
    (0x070B8A, i_070b8a),
    (0x070B8E, i_070b8e),
    (0x070B94, i_070b94),
    (0x070B98, i_070b98),
    (0x070B9A, i_070b9a),
    (0x070BA0, i_070ba0),
    (0x070BA4, i_070ba4),
    (0x070BAA, i_070baa),
    (0x070BB0, i_070bb0),
    (0x070BB4, i_070bb4),
    (0x070BB8, i_070bb8),
    (0x070BBC, i_070bbc),
    (0x070BC0, i_070bc0),
    (0x070BC2, i_070bc2),
    (0x070BC6, i_070bc6),
    (0x070BCA, i_070bca),
    (0x070BD0, i_070bd0),
    (0x070BD4, i_070bd4),
    (0x070BD6, i_070bd6),
    (0x070BDA, i_070bda),
    (0x070BDC, i_070bdc),
    (0x070BE0, i_070be0),
    (0x070BE4, i_070be4),
    (0x070BEA, i_070bea),
    (0x070BF0, i_070bf0),
    (0x070BF4, i_070bf4),
    (0x070BF8, i_070bf8),
    (0x070BFC, i_070bfc),
    (0x070C00, i_070c00),
    (0x070C02, i_070c02),
    (0x070C06, i_070c06),
    (0x070C0C, i_070c0c),
    (0x070C10, i_070c10),
    (0x070C14, i_070c14),
    (0x070C1A, i_070c1a),
    (0x070C1E, i_070c1e),
    (0x070C24, i_070c24),
    (0x070C2A, i_070c2a),
    (0x070C30, i_070c30),
    (0x070C34, i_070c34),
    (0x070C3A, i_070c3a),
    (0x070C40, i_070c40),
    (0x070C44, i_070c44),
    (0x070C46, i_070c46),
    (0x070C48, i_070c48),
    (0x070C4C, i_070c4c),
    (0x070C4E, i_070c4e),
    (0x070C54, i_070c54),
    (0x070C58, i_070c58),
    (0x070C5E, i_070c5e),
    (0x070C64, i_070c64),
    (0x070C6A, i_070c6a),
    (0x070C6E, i_070c6e),
    (0x070C72, i_070c72),
    (0x070C76, i_070c76),
    (0x070C7A, i_070c7a),
    (0x070C7E, i_070c7e),
    (0x070C82, i_070c82),
    (0x070C86, i_070c86),
    (0x070C8A, i_070c8a),
    (0x070C90, i_070c90),
    (0x070C96, i_070c96),
    (0x070C9C, i_070c9c),
    (0x070C9E, i_070c9e),
    (0x070CA2, i_070ca2),
    (0x070CA6, i_070ca6),
    (0x070CAA, i_070caa),
    (0x070CAE, i_070cae),
    (0x070CB2, i_070cb2),
    (0x070CB4, i_070cb4),
    (0x070CB8, i_070cb8),
    (0x070CBC, i_070cbc),
    (0x070CBE, i_070cbe),
    (0x070CC4, i_070cc4),
    (0x070CC8, i_070cc8),
    (0x070CCA, i_070cca),
    (0x070CD0, i_070cd0),
    (0x070CD6, i_070cd6),
    (0x070CDC, i_070cdc),
    (0x070CDE, i_070cde),
    (0x070CE4, i_070ce4),
    (0x070CE8, i_070ce8),
    (0x070CEA, i_070cea),
    (0x070CF0, i_070cf0),
    (0x070CF6, i_070cf6),
    (0x070CFC, i_070cfc),
    (0x070CFE, i_070cfe),
    (0x070D04, i_070d04),
    (0x070D08, i_070d08),
    (0x070D0A, i_070d0a),
    (0x070D10, i_070d10),
    (0x070D16, i_070d16),
    (0x070D1C, i_070d1c),
    (0x070D1E, i_070d1e),
    (0x070D22, i_070d22),
    (0x070D28, i_070d28),
    (0x070D2E, i_070d2e),
    (0x070D34, i_070d34),
    (0x070D3A, i_070d3a),
    (0x070D40, i_070d40),
    (0x070D44, i_070d44),
    (0x070D46, i_070d46),
    (0x070D4A, i_070d4a),
    (0x070D4E, i_070d4e),
    (0x070D54, i_070d54),
    (0x070D58, i_070d58),
    (0x070D5A, i_070d5a),
    (0x070D60, i_070d60),
    (0x070D66, i_070d66),
    (0x070D6A, i_070d6a),
    (0x070D70, i_070d70),
    (0x070D74, i_070d74),
    (0x070D7A, i_070d7a),
    (0x070D7C, i_070d7c),
    (0x070D82, i_070d82),
    (0x070D86, i_070d86),
    (0x070D88, i_070d88),
    (0x070D8C, i_070d8c),
    (0x070D90, i_070d90),
    (0x070D96, i_070d96),
    (0x070D9A, i_070d9a),
    (0x070D9C, i_070d9c),
    (0x070DA0, i_070da0),
    (0x070DA4, i_070da4),
    (0x070DA6, i_070da6),
    (0x070DAA, i_070daa),
    (0x070DAE, i_070dae),
    (0x070DB4, i_070db4),
    (0x070DBA, i_070dba),
    (0x070DBC, i_070dbc),
    (0x070DC0, i_070dc0),
    (0x070DC2, i_070dc2),
    (0x070DC4, i_070dc4),
    (0x070DC8, i_070dc8),
    (0x070DCC, i_070dcc),
    (0x070DD0, i_070dd0),
    (0x070DD4, i_070dd4),
    (0x070DDA, i_070dda),
    (0x070DDE, i_070dde),
    (0x070DE0, i_070de0),
    (0x070DE2, i_070de2),
    (0x070DE6, i_070de6),
    (0x070DE8, i_070de8),
    (0x070DEC, i_070dec),
    (0x070DF0, i_070df0),
    (0x070DF2, i_070df2),
    (0x070DF6, i_070df6),
    (0x070DF8, i_070df8),
    (0x070DFC, i_070dfc),
    (0x070E00, i_070e00),
    (0x070E04, i_070e04),
    (0x070E06, i_070e06),
    (0x070E0A, i_070e0a),
    (0x070E0C, i_070e0c),
    (0x070E10, i_070e10),
    (0x070E12, i_070e12),
    (0x070E18, i_070e18),
    (0x070E1E, i_070e1e),
    (0x070E22, i_070e22),
    (0x070E26, i_070e26),
    (0x070E2A, i_070e2a),
    (0x070E2C, i_070e2c),
    (0x070E2E, i_070e2e),
    (0x070E30, i_070e30),
    (0x070E32, i_070e32),
    (0x070E36, i_070e36),
    (0x070E3A, i_070e3a),
    (0x070E3C, i_070e3c),
    (0x070E42, i_070e42),
    (0x070E46, i_070e46),
    (0x070E4C, i_070e4c),
    (0x070E52, i_070e52),
    (0x070E58, i_070e58),
    (0x070E5A, i_070e5a),
    (0x070E60, i_070e60),
    (0x070E64, i_070e64),
    (0x070E6A, i_070e6a),
    (0x070E70, i_070e70),
    (0x070E76, i_070e76),
    (0x070E78, i_070e78),
    (0x070E7E, i_070e7e),
    (0x070E82, i_070e82),
    (0x070E88, i_070e88),
    (0x070E8E, i_070e8e),
    (0x070E94, i_070e94),
    (0x070E9A, i_070e9a),
    (0x070EA0, i_070ea0),
    (0x070EA2, i_070ea2),
    (0x070EA6, i_070ea6),
    (0x070EAC, i_070eac),
    (0x070EB2, i_070eb2),
    (0x070EB6, i_070eb6),
    (0x070EBC, i_070ebc),
    (0x070EBE, i_070ebe),
    (0x070EC4, i_070ec4),
    (0x070ECA, i_070eca),
    (0x070ED0, i_070ed0),
    (0x070ED4, i_070ed4),
    (0x070ED6, i_070ed6),
    (0x070EDA, i_070eda),
    (0x070EDE, i_070ede),
    (0x070EE0, i_070ee0),
    (0x070EE4, i_070ee4),
    (0x070EEA, i_070eea),
    (0x070EF0, i_070ef0),
    (0x070EF2, i_070ef2),
    (0x070EF6, i_070ef6),
    (0x070EFC, i_070efc),
    (0x070F02, i_070f02),
    (0x070F04, i_070f04),
    (0x070F08, i_070f08),
    (0x070F0E, i_070f0e),
    (0x070F14, i_070f14),
    (0x070F1A, i_070f1a),
    (0x070F20, i_070f20),
    (0x070F22, i_070f22),
    (0x070F26, i_070f26),
    (0x070F2C, i_070f2c),
    (0x070F32, i_070f32),
    (0x070F38, i_070f38),
    (0x070F3E, i_070f3e),
    (0x070F44, i_070f44),
    (0x070F4A, i_070f4a),
    (0x070F4C, i_070f4c),
    (0x070F50, i_070f50),
    (0x070F56, i_070f56),
    (0x070F5C, i_070f5c),
    (0x070F62, i_070f62),
    (0x070F68, i_070f68),
    (0x070F6E, i_070f6e),
    (0x070F70, i_070f70),
    (0x070F74, i_070f74),
    (0x070F7A, i_070f7a),
    (0x070F80, i_070f80),
    (0x070F86, i_070f86),
    (0x070F8C, i_070f8c),
    (0x070F92, i_070f92),
    (0x070F98, i_070f98),
    (0x070F9A, i_070f9a),
    (0x070F9E, i_070f9e),
    (0x070FA4, i_070fa4),
    (0x070FAA, i_070faa),
    (0x070FB0, i_070fb0),
    (0x070FB6, i_070fb6),
    (0x070FBC, i_070fbc),
    (0x070FBE, i_070fbe),
    (0x070FC2, i_070fc2),
    (0x070FC8, i_070fc8),
    (0x070FCE, i_070fce),
    (0x070FD4, i_070fd4),
    (0x070FDA, i_070fda),
    (0x070FE0, i_070fe0),
    (0x070FE6, i_070fe6),
    (0x070FE8, i_070fe8),
    (0x070FEC, i_070fec),
    (0x070FF2, i_070ff2),
    (0x070FF8, i_070ff8),
    (0x070FFC, i_070ffc),
    (0x071002, i_071002),
    (0x071008, i_071008),
    (0x07100C, i_07100c),
    (0x071012, i_071012),
    (0x071018, i_071018),
    (0x07101A, i_07101a),
    (0x071020, i_071020),
    (0x071026, i_071026),
    (0x07102C, i_07102c),
    (0x071032, i_071032),
    (0x071036, i_071036),
    (0x071038, i_071038),
    (0x07103E, i_07103e),
    (0x071042, i_071042),
    (0x071044, i_071044),
    (0x071046, i_071046),
    (0x071048, i_071048),
    (0x07104A, i_07104a),
    (0x07104C, i_07104c),
    (0x071050, i_071050),
    (0x071054, i_071054),
    (0x071058, i_071058),
    (0x07105C, i_07105c),
    (0x071060, i_071060),
    (0x071062, i_071062),
    (0x071066, i_071066),
    (0x071068, i_071068),
    (0x07106C, i_07106c),
    (0x071072, i_071072),
    (0x071076, i_071076),
    (0x07107C, i_07107c),
    (0x071082, i_071082),
    (0x071088, i_071088),
    (0x07108E, i_07108e),
    (0x071094, i_071094),
    (0x07109A, i_07109a),
    (0x0710A0, i_0710a0),
    (0x0710A4, i_0710a4),
    (0x0710AA, i_0710aa),
    (0x0710B0, i_0710b0),
    (0x0710B6, i_0710b6),
    (0x0710BC, i_0710bc),
    (0x0710C2, i_0710c2),
    (0x0710C6, i_0710c6),
    (0x0710CC, i_0710cc),
    (0x0710D0, i_0710d0),
    (0x0710D2, i_0710d2),
    (0x0710D6, i_0710d6),
    (0x0710DA, i_0710da),
    (0x0710DC, i_0710dc),
    (0x0710E2, i_0710e2),
    (0x0710E6, i_0710e6),
    (0x0710EC, i_0710ec),
    (0x0710F0, i_0710f0),
    (0x0710F2, i_0710f2),
    (0x0710F8, i_0710f8),
    (0x0710FC, i_0710fc),
    (0x071102, i_071102),
    (0x071108, i_071108),
    (0x07110C, i_07110c),
    (0x071110, i_071110),
    (0x071112, i_071112),
    (0x071116, i_071116),
    (0x07111A, i_07111a),
    (0x07111C, i_07111c),
    (0x071120, i_071120),
    (0x071126, i_071126),
    (0x07112C, i_07112c),
    (0x071130, i_071130),
    (0x071132, i_071132),
    (0x071136, i_071136),
    (0x07113C, i_07113c),
    (0x071140, i_071140),
    (0x071146, i_071146),
    (0x07114C, i_07114c),
    (0x07114E, i_07114e),
    (0x071152, i_071152),
    (0x071156, i_071156),
    (0x07115C, i_07115c),
    (0x071160, i_071160),
    (0x071166, i_071166),
    (0x07116A, i_07116a),
    (0x07116E, i_07116e),
    (0x071174, i_071174),
    (0x071178, i_071178),
    (0x07117C, i_07117c),
    (0x071182, i_071182),
    (0x071188, i_071188),
    (0x07118E, i_07118e),
    (0x071194, i_071194),
    (0x071198, i_071198),
    (0x07119E, i_07119e),
    (0x0711A4, i_0711a4),
    (0x0711A8, i_0711a8),
    (0x0711AE, i_0711ae),
    (0x0711B4, i_0711b4),
    (0x0711B8, i_0711b8),
    (0x0711BE, i_0711be),
    (0x0711C4, i_0711c4),
    (0x0711C8, i_0711c8),
    (0x0711CE, i_0711ce),
    (0x0711D4, i_0711d4),
    (0x0711D8, i_0711d8),
    (0x0711DE, i_0711de),
    (0x0711E4, i_0711e4),
    (0x0711EA, i_0711ea),
    (0x0711F0, i_0711f0),
    (0x0711F6, i_0711f6),
    (0x0711FC, i_0711fc),
    (0x071202, i_071202),
    (0x071208, i_071208),
    (0x07120E, i_07120e),
    (0x071212, i_071212),
    (0x07122E, i_07122e),
    (0x071234, i_071234),
    (0x071238, i_071238),
    (0x07123C, i_07123c),
    (0x071242, i_071242),
    (0x071246, i_071246),
    (0x07124A, i_07124a),
    (0x071250, i_071250),
    (0x071256, i_071256),
    (0x07125C, i_07125c),
    (0x071262, i_071262),
    (0x071266, i_071266),
    (0x07126C, i_07126c),
    (0x071272, i_071272),
    (0x071276, i_071276),
    (0x07127C, i_07127c),
    (0x071282, i_071282),
    (0x071286, i_071286),
    (0x07128C, i_07128c),
    (0x071292, i_071292),
    (0x071296, i_071296),
    (0x07129C, i_07129c),
    (0x0712A2, i_0712a2),
    (0x0712A6, i_0712a6),
    (0x0712AC, i_0712ac),
    (0x0712B2, i_0712b2),
    (0x0712B8, i_0712b8),
    (0x0712BE, i_0712be),
    (0x0712C4, i_0712c4),
    (0x0712CA, i_0712ca),
    (0x0712D0, i_0712d0),
    (0x0712D6, i_0712d6),
    (0x0712DC, i_0712dc),
    (0x0712E0, i_0712e0),
    (0x0712E4, i_0712e4),
    (0x0712EA, i_0712ea),
    (0x0712F0, i_0712f0),
    (0x0712F4, i_0712f4),
    (0x0712F8, i_0712f8),
    (0x0712FE, i_0712fe),
    (0x071304, i_071304),
    (0x07130A, i_07130a),
    (0x07130E, i_07130e),
    (0x071312, i_071312),
    (0x071316, i_071316),
    (0x07131C, i_07131c),
    (0x071322, i_071322),
    (0x071328, i_071328),
    (0x07132E, i_07132e),
    (0x071332, i_071332),
    (0x071336, i_071336),
    (0x07133C, i_07133c),
    (0x071340, i_071340),
    (0x071344, i_071344),
    (0x07134A, i_07134a),
    (0x071350, i_071350),
    (0x071358, i_071358),
    (0x071360, i_071360),
    (0x071368, i_071368),
    (0x071370, i_071370),
    (0x071376, i_071376),
    (0x07137A, i_07137a),
    (0x07137E, i_07137e),
    (0x071382, i_071382),
    (0x071386, i_071386),
    (0x071388, i_071388),
    (0x07138C, i_07138c),
    (0x071390, i_071390),
    (0x071396, i_071396),
    (0x07139A, i_07139a),
    (0x07139E, i_07139e),
    (0x0713A0, i_0713a0),
    (0x0713A2, i_0713a2),
    (0x0713A8, i_0713a8),
    (0x0713AC, i_0713ac),
    (0x0713B0, i_0713b0),
    (0x0713B6, i_0713b6),
    (0x0713BA, i_0713ba),
    (0x0713BC, i_0713bc),
    (0x0713C0, i_0713c0),
    (0x0713C4, i_0713c4),
    (0x0713CA, i_0713ca),
    (0x0713D0, i_0713d0),
    (0x0713D4, i_0713d4),
    (0x0713DA, i_0713da),
    (0x0713E0, i_0713e0),
    (0x0713E4, i_0713e4),
    (0x0713EA, i_0713ea),
    (0x0713EE, i_0713ee),
    (0x0713F0, i_0713f0),
    (0x0713F6, i_0713f6),
    (0x0713FA, i_0713fa),
    (0x071400, i_071400),
    (0x071404, i_071404),
    (0x071408, i_071408),
    (0x07140A, i_07140a),
    (0x071410, i_071410),
    (0x071414, i_071414),
    (0x071418, i_071418),
    (0x07141C, i_07141c),
    (0x07141E, i_07141e),
    (0x071422, i_071422),
    (0x071426, i_071426),
    (0x07142A, i_07142a),
    (0x07142C, i_07142c),
    (0x071432, i_071432),
    (0x071438, i_071438),
    (0x07143E, i_07143e),
    (0x071444, i_071444),
    (0x071448, i_071448),
    (0x07144E, i_07144e),
    (0x071452, i_071452),
    (0x071456, i_071456),
    (0x07145C, i_07145c),
    (0x071462, i_071462),
    (0x071468, i_071468),
    (0x07146C, i_07146c),
    (0x07146E, i_07146e),
    (0x071474, i_071474),
    (0x071478, i_071478),
    (0x07147E, i_07147e),
    (0x071484, i_071484),
    (0x071488, i_071488),
    (0x07148C, i_07148c),
    (0x07148E, i_07148e),
    (0x071494, i_071494),
    (0x07149A, i_07149a),
    (0x07149E, i_07149e),
    (0x0714A4, i_0714a4),
    (0x0714AA, i_0714aa),
    (0x0714B0, i_0714b0),
    (0x0714B4, i_0714b4),
    (0x0714B6, i_0714b6),
    (0x0714BC, i_0714bc),
    (0x0714C0, i_0714c0),
    (0x0714C6, i_0714c6),
    (0x0714CC, i_0714cc),
    (0x0714D0, i_0714d0),
    (0x0714D6, i_0714d6),
    (0x0714DA, i_0714da),
    (0x0714DE, i_0714de),
    (0x0714E0, i_0714e0),
    (0x0714E4, i_0714e4),
    (0x0714EA, i_0714ea),
    (0x0714F0, i_0714f0),
    (0x0714F4, i_0714f4),
    (0x0714F6, i_0714f6),
    (0x0714FC, i_0714fc),
    (0x071500, i_071500),
    (0x071506, i_071506),
    (0x07150C, i_07150c),
    (0x071510, i_071510),
    (0x071514, i_071514),
    (0x071516, i_071516),
    (0x07151C, i_07151c),
    (0x071522, i_071522),
    (0x071526, i_071526),
    (0x07152C, i_07152c),
    (0x071532, i_071532),
    (0x071538, i_071538),
    (0x07154E, i_07154e),
    (0x071552, i_071552),
    (0x071558, i_071558),
    (0x07155C, i_07155c),
    (0x07155E, i_07155e),
    (0x071564, i_071564),
    (0x071568, i_071568),
    (0x07156E, i_07156e),
    (0x071574, i_071574),
    (0x071578, i_071578),
    (0x07157C, i_07157c),
    (0x07157E, i_07157e),
    (0x071582, i_071582),
    (0x071586, i_071586),
    (0x07158C, i_07158c),
    (0x071592, i_071592),
    (0x071598, i_071598),
    (0x07159E, i_07159e),
    (0x0715A4, i_0715a4),
    (0x0715AA, i_0715aa),
    (0x0715B0, i_0715b0),
    (0x0715B6, i_0715b6),
    (0x0715BC, i_0715bc),
    (0x0715C0, i_0715c0),
    (0x0715C2, i_0715c2),
    (0x0715C8, i_0715c8),
    (0x0715CE, i_0715ce),
    (0x0715D2, i_0715d2),
    (0x0715D6, i_0715d6),
    (0x0715D8, i_0715d8),
    (0x0715DC, i_0715dc),
    (0x0715E2, i_0715e2),
    (0x0715E8, i_0715e8),
    (0x0715EE, i_0715ee),
    (0x0715F2, i_0715f2),
    (0x0715F6, i_0715f6),
    (0x0715F8, i_0715f8),
    (0x0715FA, i_0715fa),
    (0x071600, i_071600),
    (0x071604, i_071604),
    (0x07160A, i_07160a),
    (0x071610, i_071610),
    (0x071616, i_071616),
    (0x07161C, i_07161c),
    (0x071622, i_071622),
    (0x071628, i_071628),
    (0x07162E, i_07162e),
    (0x071634, i_071634),
    (0x07163A, i_07163a),
    (0x071640, i_071640),
    (0x071646, i_071646),
    (0x07164C, i_07164c),
    (0x071652, i_071652),
    (0x071656, i_071656),
    (0x07165C, i_07165c),
    (0x071662, i_071662),
    (0x071664, i_071664),
    (0x071668, i_071668),
    (0x07166C, i_07166c),
    (0x071672, i_071672),
    (0x071676, i_071676),
    (0x07167C, i_07167c),
    (0x071682, i_071682),
    (0x071688, i_071688),
    (0x07168E, i_07168e),
    (0x071692, i_071692),
    (0x071696, i_071696),
    (0x071698, i_071698),
    (0x07169E, i_07169e),
    (0x0716A4, i_0716a4),
    (0x0716AA, i_0716aa),
    (0x0716AE, i_0716ae),
    (0x0716B2, i_0716b2),
    (0x0716B4, i_0716b4),
    (0x0716B6, i_0716b6),
    (0x0716BC, i_0716bc),
    (0x0716C2, i_0716c2),
    (0x0716C8, i_0716c8),
    (0x0716CE, i_0716ce),
    (0x0716D4, i_0716d4),
    (0x0716DA, i_0716da),
    (0x0716DE, i_0716de),
    (0x0716E0, i_0716e0),
    (0x0716E2, i_0716e2),
    (0x0716EA, i_0716ea),
    (0x0716F2, i_0716f2),
    (0x0716F8, i_0716f8),
    (0x071700, i_071700),
    (0x071704, i_071704),
    (0x07170A, i_07170a),
    (0x07170E, i_07170e),
    (0x071712, i_071712),
    (0x071716, i_071716),
    (0x07171A, i_07171a),
    (0x07171C, i_07171c),
    (0x07171E, i_07171e),
    (0x071722, i_071722),
    (0x071724, i_071724),
    (0x071726, i_071726),
    (0x07172C, i_07172c),
    (0x07172E, i_07172e),
    (0x071732, i_071732),
    (0x071738, i_071738),
    (0x07173C, i_07173c),
    (0x071742, i_071742),
    (0x071746, i_071746),
    (0x071748, i_071748),
    (0x07174C, i_07174c),
    (0x071752, i_071752),
    (0x071756, i_071756),
    (0x07175A, i_07175a),
    (0x071760, i_071760),
    (0x071764, i_071764),
    (0x071768, i_071768),
    (0x07176A, i_07176a),
    (0x07176E, i_07176e),
    (0x071774, i_071774),
    (0x071778, i_071778),
    (0x07177E, i_07177e),
    (0x071784, i_071784),
    (0x071786, i_071786),
    (0x07178A, i_07178a),
    (0x07178E, i_07178e),
    (0x071792, i_071792),
    (0x071794, i_071794),
    (0x071796, i_071796),
    (0x07179A, i_07179a),
    (0x07179C, i_07179c),
    (0x07179E, i_07179e),
    (0x0717A4, i_0717a4),
    (0x0717A6, i_0717a6),
    (0x0717AA, i_0717aa),
    (0x0717B0, i_0717b0),
    (0x0717B4, i_0717b4),
    (0x0717BA, i_0717ba),
    (0x0717BE, i_0717be),
    (0x0717C2, i_0717c2),
    (0x0717C6, i_0717c6),
    (0x0717CA, i_0717ca),
    (0x0717CC, i_0717cc),
    (0x0717CE, i_0717ce),
    (0x0717D2, i_0717d2),
    (0x0717D4, i_0717d4),
    (0x0717D6, i_0717d6),
    (0x0717DC, i_0717dc),
    (0x0717DE, i_0717de),
    (0x0717E2, i_0717e2),
    (0x0717E8, i_0717e8),
    (0x0717EC, i_0717ec),
    (0x0717F2, i_0717f2),
    (0x0717F6, i_0717f6),
    (0x0717FA, i_0717fa),
    (0x0717FE, i_0717fe),
    (0x071802, i_071802),
    (0x071804, i_071804),
    (0x071806, i_071806),
    (0x07180A, i_07180a),
    (0x07180C, i_07180c),
    (0x07180E, i_07180e),
    (0x071814, i_071814),
    (0x071816, i_071816),
    (0x07181A, i_07181a),
    (0x071820, i_071820),
    (0x071824, i_071824),
    (0x07182A, i_07182a),
    (0x07182E, i_07182e),
    (0x071832, i_071832),
    (0x071836, i_071836),
    (0x07183A, i_07183a),
    (0x07183C, i_07183c),
    (0x07183E, i_07183e),
    (0x071842, i_071842),
    (0x071844, i_071844),
    (0x071846, i_071846),
    (0x07184C, i_07184c),
    (0x07184E, i_07184e),
    (0x071852, i_071852),
    (0x071858, i_071858),
    (0x07185C, i_07185c),
    (0x071862, i_071862),
    (0x071866, i_071866),
    (0x07186A, i_07186a),
    (0x07186C, i_07186c),
    (0x07186E, i_07186e),
    (0x071874, i_071874),
    (0x07187A, i_07187a),
    (0x071882, i_071882),
    (0x071888, i_071888),
    (0x07188E, i_07188e),
    (0x071894, i_071894),
    (0x071898, i_071898),
    (0x07189E, i_07189e),
    (0x0718A4, i_0718a4),
    (0x0718AC, i_0718ac),
    (0x0718B2, i_0718b2),
    (0x0718B8, i_0718b8),
    (0x0718BE, i_0718be),
    (0x0718C2, i_0718c2),
    (0x0718C8, i_0718c8),
    (0x0718CE, i_0718ce),
    (0x0718D6, i_0718d6),
    (0x0718DC, i_0718dc),
    (0x0718E2, i_0718e2),
    (0x0718E8, i_0718e8),
    (0x0718EC, i_0718ec),
    (0x0718F2, i_0718f2),
    (0x0718F8, i_0718f8),
    (0x071900, i_071900),
    (0x071906, i_071906),
    (0x07190C, i_07190c),
    (0x071912, i_071912),
    (0x071916, i_071916),
    (0x07191C, i_07191c),
    (0x071922, i_071922),
    (0x07192A, i_07192a),
    (0x071930, i_071930),
    (0x071936, i_071936),
    (0x07193C, i_07193c),
    (0x071940, i_071940),
    (0x071946, i_071946),
    (0x07194C, i_07194c),
    (0x071954, i_071954),
    (0x07195A, i_07195a),
    (0x071960, i_071960),
    (0x071964, i_071964),
    (0x071968, i_071968),
    (0x07196C, i_07196c),
    (0x071970, i_071970),
    (0x071976, i_071976),
    (0x07197C, i_07197c),
    (0x071982, i_071982),
    (0x071988, i_071988),
    (0x07198E, i_07198e),
    (0x071994, i_071994),
    (0x07199A, i_07199a),
    (0x0719A0, i_0719a0),
    (0x0719A4, i_0719a4),
    (0x0719A6, i_0719a6),
    (0x0719AA, i_0719aa),
    (0x0719B0, i_0719b0),
    (0x0719B4, i_0719b4),
    (0x0719B8, i_0719b8),
    (0x0719BC, i_0719bc),
    (0x0719BE, i_0719be),
    (0x0719C2, i_0719c2),
    (0x0719C6, i_0719c6),
    (0x0719CC, i_0719cc),
    (0x0719D0, i_0719d0),
    (0x0719D4, i_0719d4),
    (0x0719D6, i_0719d6),
    (0x0719DC, i_0719dc),
    (0x0719E0, i_0719e0),
    (0x0719E4, i_0719e4),
    (0x0719E6, i_0719e6),
    (0x0719E8, i_0719e8),
    (0x0719EE, i_0719ee),
    (0x0719F4, i_0719f4),
    (0x0719FA, i_0719fa),
    (0x0719FE, i_0719fe),
    (0x071A00, i_071a00),
    (0x071A04, i_071a04),
    (0x071A08, i_071a08),
    (0x071A0E, i_071a0e),
    (0x071A12, i_071a12),
    (0x071A16, i_071a16),
    (0x071A18, i_071a18),
    (0x071A1E, i_071a1e),
    (0x071A22, i_071a22),
    (0x071A28, i_071a28),
    (0x071A2C, i_071a2c),
    (0x071A2E, i_071a2e),
    (0x071A32, i_071a32),
    (0x071A36, i_071a36),
    (0x071A3A, i_071a3a),
    (0x071A3E, i_071a3e),
    (0x071A42, i_071a42),
    (0x071A46, i_071a46),
    (0x071A4A, i_071a4a),
    (0x071A4C, i_071a4c),
    (0x071A90, i_071a90),
    (0x071A96, i_071a96),
    (0x071A9A, i_071a9a),
    (0x071A9E, i_071a9e),
    (0x071AA4, i_071aa4),
    (0x071AAA, i_071aaa),
    (0x071AB0, i_071ab0),
    (0x071AB4, i_071ab4),
    (0x071ABA, i_071aba),
    (0x071AC0, i_071ac0),
    (0x071AC6, i_071ac6),
    (0x071ACA, i_071aca),
    (0x071ACC, i_071acc),
    (0x071AD0, i_071ad0),
    (0x071AD4, i_071ad4),
    (0x071ADA, i_071ada),
    (0x071ADE, i_071ade),
    (0x071AE2, i_071ae2),
    (0x071AE4, i_071ae4),
    (0x071AE8, i_071ae8),
    (0x071AEC, i_071aec),
    (0x071AF0, i_071af0),
    (0x071AF4, i_071af4),
    (0x071AF8, i_071af8),
    (0x071AFC, i_071afc),
    (0x071B00, i_071b00),
    (0x071B02, i_071b02),
    (0x071B04, i_071b04),
];
