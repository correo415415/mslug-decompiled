// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $057108: move.b $9b(a6), d0
pub fn i_057108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5710C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05710C: bne.w $57114
pub fn i_05710c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57110;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57114; return 10; }
    8
}
/// $057110: move.w #$a, d0
pub fn i_057110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57114;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057114: lsl.w #$6, d0
pub fn i_057114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57116;
    cpu.ir = 0xED48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $057116: move.w d0, $8e(a6)
pub fn i_057116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5711A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05711A: jsr $13600.l
pub fn i_05711a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57120);
    cpu.pc = 0x13600;
    16
}
/// $057120: move.w #$e, d1
pub fn i_057120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57124;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057124: jsr $236e.l
pub fn i_057124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5712A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5712A);
    cpu.pc = 0x236E;
    16
}
/// $05712A: clr.w $2c(a6)
pub fn i_05712a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5712E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05712E: clr.w $2e(a6)
pub fn i_05712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57132;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $057132: move.w #$8000, d0
pub fn i_057132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57136;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057136: jsr $28134.l
pub fn i_057136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5713C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5713C);
    cpu.pc = 0x28134;
    16
}
/// $05713C: andi.w #$ffe3, $38(a6)
pub fn i_05713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57142;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $057142: ori.w #$18, $38(a6)
pub fn i_057142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57148;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $057148: lea.l $776e2.l, a1
pub fn i_057148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5714E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $05714E: jsr $4ae.l
pub fn i_05714e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57154);
    cpu.pc = 0x4AE;
    16
}
/// $057154: clr.w $84(a6)
pub fn i_057154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57158;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $057158: clr.w $86(a6)
pub fn i_057158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5715C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05715C: bsr.w $56b92
pub fn i_05715c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57160;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57160);
    cpu.pc = 0x56B92;
    18
}
/// $057160: clr.b $74(a6)
pub fn i_057160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57164;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $057164: jsr $5e9b6.l
pub fn i_057164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5716A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5716A);
    cpu.pc = 0x5E9B6;
    16
}
/// $05716A: move.w d0, $70(a6)
pub fn i_05716a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5716E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05716E: jsr $5e9b6.l
pub fn i_05716e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57174);
    cpu.pc = 0x5E9B6;
    16
}
// ===== SetTaskW_057174  ($057174, 6 B, kernel/task_setters_w.c)
/// $057174: move.w d0, $90(a6)
pub fn i_057174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57178;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057178: rts
pub fn i_057178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5717A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_PublishByProbeN_05717A  ($05717A, 34 B, entity/state_publishers_057044.s)
/// $05717A: jsr $5e9b6.l
pub fn i_05717a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57180;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57180);
    cpu.pc = 0x5E9B6;
    16
}
/// $057180: bpl.w $5718e
pub fn i_057180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57184;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5718E; return 10; }
    8
}
/// $057184: bclr.b #$4, $72(a6)
pub fn i_057184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5718A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05718A: bra.w $57194
pub fn i_05718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5718E;
    cpu.ir = 0x6000;
    cpu.pc = 0x57194;
    10
}
/// $05718E: bset.b #$4, $72(a6)
pub fn i_05718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57194;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057194: bset.b #$3, $74(a6)
pub fn i_057194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5719A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05719A: rts
pub fn i_05719a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5719C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_PublishByProbeN_ClearSub75_05719C  ($05719C, 40 B, entity/state_publishers_057044.s)
/// $05719C: jsr $5e9b6.l
pub fn i_05719c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x571A2);
    cpu.pc = 0x5E9B6;
    16
}
/// $0571A2: bpl.w $571b0
pub fn i_0571a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571A6;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x571B0; return 10; }
    8
}
/// $0571A6: bclr.b #$4, $72(a6)
pub fn i_0571a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571AC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571AC: bra.w $571b6
pub fn i_0571ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x571B6;
    10
}
/// $0571B0: bset.b #$4, $72(a6)
pub fn i_0571b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571B6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571B6: bset.b #$3, $74(a6)
pub fn i_0571b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571BC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571BC: move.b #$0, $75(a6)
pub fn i_0571bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0571C2: rts
pub fn i_0571c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_SetSubstate2_0571C4  ($0571C4, 20 B, entity/state_publishers_057044.s)
/// $0571C4: bset.b #$4, $72(a6)
pub fn i_0571c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571CA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571CA: bset.b #$3, $74(a6)
pub fn i_0571ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571D0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571D0: move.b #$2, $75(a6)
pub fn i_0571d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0571D6: rts
pub fn i_0571d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_SetSubstate1_0571D8  ($0571D8, 20 B, entity/state_publishers_057044.s)
/// $0571D8: bset.b #$4, $72(a6)
pub fn i_0571d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571DE: bset.b #$3, $74(a6)
pub fn i_0571de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571E4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571E4: move.b #$1, $75(a6)
pub fn i_0571e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0571EA: rts
pub fn i_0571ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_SetSubstate3_0571EC  ($0571EC, 20 B, entity/state_publishers_057044.s)
/// $0571EC: bset.b #$4, $72(a6)
pub fn i_0571ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571F2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571F2: bset.b #$3, $74(a6)
pub fn i_0571f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571F8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0571F8: move.b #$3, $75(a6)
pub fn i_0571f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x571FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0571FE: rts
pub fn i_0571fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57200;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_SetState74Bit4_057200  ($057200, 14 B, entity/state_publishers_057044.s)
/// $057200: bset.b #$4, $72(a6)
pub fn i_057200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57206;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057206: bset.b #$4, $74(a6)
pub fn i_057206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5720C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05720C: rts
pub fn i_05720c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5720E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_SetState74Bit0_05720E  ($05720E, 8 B, entity/state_publishers_057044.s)
/// $05720E: bset.b #$0, $74(a6)
pub fn i_05720e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57214;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057214: rts
pub fn i_057214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57216;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_SetState74Bit2_057216  ($057216, 8 B, entity/state_publishers_057044.s)
/// $057216: bset.b #$2, $74(a6)
pub fn i_057216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5721C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05721C: rts
pub fn i_05721c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5721E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityState_SetState74Bit1_05721E  ($05721E, 8 B, entity/state_publishers_057044.s)
/// $05721E: bset.b #$1, $74(a6)
pub fn i_05721e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57224;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057224: rts
pub fn i_057224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57226;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnVariants_057226  ($057226, 392 B, enemies/soldiers/helpers_056acc.s)
/// $057226: jsr $13600.l
pub fn i_057226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5722C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5722C);
    cpu.pc = 0x13600;
    16
}
/// $05722C: bset.b #$1, $12(a6)
pub fn i_05722c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57232;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057232: move.b #$2, $98(a6)
pub fn i_057232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57238;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057238: move.b #$a, $99(a6)
pub fn i_057238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5723E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05723E: move.b #$1e, $9a(a6)
pub fn i_05723e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57244;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057244: move.b #$a, $9b(a6)
pub fn i_057244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5724A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05724A: bra.w $573a2
pub fn i_05724a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5724E;
    cpu.ir = 0x6000;
    cpu.pc = 0x573A2;
    10
}
/// $05724E: jsr $13600.l
pub fn i_05724e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57254;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57254);
    cpu.pc = 0x13600;
    16
}
/// $057254: bset.b #$1, $12(a6)
pub fn i_057254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5725A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05725A: move.b #$2, $98(a6)
pub fn i_05725a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57260;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057260: move.b #$a, $99(a6)
pub fn i_057260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57266;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057266: move.b #$1e, $9a(a6)
pub fn i_057266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5726C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05726C: move.b #$a, $9b(a6)
pub fn i_05726c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57272;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057272: bra.w $572a6
pub fn i_057272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57276;
    cpu.ir = 0x6000;
    cpu.pc = 0x572A6;
    10
}
/// $057276: bsr.w $570a8
pub fn i_057276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5727A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5727A);
    cpu.pc = 0x570A8;
    18
}
/// $05727A: bsr.w $571d8
pub fn i_05727a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5727E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5727E);
    cpu.pc = 0x571D8;
    18
}
/// $05727E: bra.w $5752c
pub fn i_05727e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57282;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $057282: bsr.w $570a8
pub fn i_057282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57286;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57286);
    cpu.pc = 0x570A8;
    18
}
/// $057286: bsr.w $571c4
pub fn i_057286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5728A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5728A);
    cpu.pc = 0x571C4;
    18
}
/// $05728A: bra.w $5752c
pub fn i_05728a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5728E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $05728E: bsr.w $570a8
pub fn i_05728e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57292;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57292);
    cpu.pc = 0x570A8;
    18
}
/// $057292: bsr.w $571ec
pub fn i_057292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57296;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57296);
    cpu.pc = 0x571EC;
    18
}
/// $057296: bra.w $5752c
pub fn i_057296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5729A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $05729A: bsr.w $570a8
pub fn i_05729a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5729E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5729E);
    cpu.pc = 0x570A8;
    18
}
/// $05729E: bsr.w $5720e
pub fn i_05729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572A2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572A2);
    cpu.pc = 0x5720E;
    18
}
/// $0572A2: bra.w $5752c
pub fn i_0572a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $0572A6: bsr.w $570a8
pub fn i_0572a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572AA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572AA);
    cpu.pc = 0x570A8;
    18
}
/// $0572AA: bsr.w $5721e
pub fn i_0572aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572AE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572AE);
    cpu.pc = 0x5721E;
    18
}
/// $0572AE: bra.w $5752c
pub fn i_0572ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $0572B2: bsr.w $570a8
pub fn i_0572b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572B6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572B6);
    cpu.pc = 0x570A8;
    18
}
/// $0572B6: bsr.w $57216
pub fn i_0572b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572BA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572BA);
    cpu.pc = 0x57216;
    18
}
/// $0572BA: bra.w $5752c
pub fn i_0572ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $0572BE: bsr.w $570a8
pub fn i_0572be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572C2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572C2);
    cpu.pc = 0x570A8;
    18
}
/// $0572C2: bsr.w $5720e
pub fn i_0572c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572C6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572C6);
    cpu.pc = 0x5720E;
    18
}
/// $0572C6: bsr.w $5721e
pub fn i_0572c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572CA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572CA);
    cpu.pc = 0x5721E;
    18
}
/// $0572CA: bsr.w $57216
pub fn i_0572ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572CE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572CE);
    cpu.pc = 0x57216;
    18
}
/// $0572CE: bra.w $5752c
pub fn i_0572ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $0572D2: bsr.w $570a8
pub fn i_0572d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572D6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572D6);
    cpu.pc = 0x570A8;
    18
}
/// $0572D6: bsr.w $5720e
pub fn i_0572d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572DA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572DA);
    cpu.pc = 0x5720E;
    18
}
/// $0572DA: bsr.w $5719c
pub fn i_0572da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572DE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572DE);
    cpu.pc = 0x5719C;
    18
}
/// $0572DE: bra.w $5752c
pub fn i_0572de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $0572E2: bsr.w $570a8
pub fn i_0572e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572E6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572E6);
    cpu.pc = 0x570A8;
    18
}
/// $0572E6: bsr.w $5721e
pub fn i_0572e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572EA);
    cpu.pc = 0x5721E;
    18
}
/// $0572EA: bsr.w $5719c
pub fn i_0572ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572EE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572EE);
    cpu.pc = 0x5719C;
    18
}
/// $0572EE: bra.w $5752c
pub fn i_0572ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $0572F2: bsr.w $570a8
pub fn i_0572f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572F6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572F6);
    cpu.pc = 0x570A8;
    18
}
/// $0572F6: bsr.w $57216
pub fn i_0572f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572FA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572FA);
    cpu.pc = 0x57216;
    18
}
/// $0572FA: bsr.w $5719c
pub fn i_0572fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x572FE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x572FE);
    cpu.pc = 0x5719C;
    18
}
/// $0572FE: bra.w $5752c
pub fn i_0572fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57302;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $057302: bsr.w $570a8
pub fn i_057302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57306;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57306);
    cpu.pc = 0x570A8;
    18
}
/// $057306: bsr.w $5720e
pub fn i_057306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5730A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5730A);
    cpu.pc = 0x5720E;
    18
}
/// $05730A: bsr.w $5721e
pub fn i_05730a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5730E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5730E);
    cpu.pc = 0x5721E;
    18
}
/// $05730E: bsr.w $57216
pub fn i_05730e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57312;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57312);
    cpu.pc = 0x57216;
    18
}
/// $057312: bsr.w $5719c
pub fn i_057312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57316;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57316);
    cpu.pc = 0x5719C;
    18
}
/// $057316: bra.w $5752c
pub fn i_057316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5731A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $05731A: bsr.w $570a8
pub fn i_05731a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5731E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5731E);
    cpu.pc = 0x570A8;
    18
}
/// $05731E: bsr.w $5720e
pub fn i_05731e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57322;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57322);
    cpu.pc = 0x5720E;
    18
}
/// $057322: bra.w $5752c
pub fn i_057322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57326;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $057326: bsr.w $570a8
pub fn i_057326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5732A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5732A);
    cpu.pc = 0x570A8;
    18
}
/// $05732A: bsr.w $5721e
pub fn i_05732a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5732E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5732E);
    cpu.pc = 0x5721E;
    18
}
/// $05732E: bsr.w $5719c
pub fn i_05732e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57332;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57332);
    cpu.pc = 0x5719C;
    18
}
/// $057332: bsr.w $57200
pub fn i_057332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57336;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57336);
    cpu.pc = 0x57200;
    18
}
/// $057336: bra.w $5752c
pub fn i_057336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5733A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $05733A: bsr.w $570a8
pub fn i_05733a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5733E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5733E);
    cpu.pc = 0x570A8;
    18
}
/// $05733E: bsr.w $57216
pub fn i_05733e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57342;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57342);
    cpu.pc = 0x57216;
    18
}
/// $057342: bsr.w $5719c
pub fn i_057342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57346;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57346);
    cpu.pc = 0x5719C;
    18
}
/// $057346: bsr.w $57200
pub fn i_057346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5734A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5734A);
    cpu.pc = 0x57200;
    18
}
/// $05734A: bra.w $5752c
pub fn i_05734a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5734E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $05734E: bsr.w $570a8
pub fn i_05734e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57352;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57352);
    cpu.pc = 0x570A8;
    18
}
/// $057352: bsr.w $5720e
pub fn i_057352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57356;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57356);
    cpu.pc = 0x5720E;
    18
}
/// $057356: bsr.w $5721e
pub fn i_057356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5735A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5735A);
    cpu.pc = 0x5721E;
    18
}
/// $05735A: bsr.w $57216
pub fn i_05735a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5735E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5735E);
    cpu.pc = 0x57216;
    18
}
/// $05735E: bsr.w $5719c
pub fn i_05735e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57362;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57362);
    cpu.pc = 0x5719C;
    18
}
/// $057362: bsr.w $57200
pub fn i_057362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57366;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57366);
    cpu.pc = 0x57200;
    18
}
/// $057366: bra.w $5752c
pub fn i_057366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5736A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $05736A: bclr.b #$0, $3a(a6)
pub fn i_05736a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57370;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057370: bra.w $5737a
pub fn i_057370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57374;
    cpu.ir = 0x6000;
    cpu.pc = 0x5737A;
    10
}
/// $057374: bset.b #$0, $3a(a6)
pub fn i_057374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5737A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05737A: bsr.w $570a8
pub fn i_05737a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5737E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5737E);
    cpu.pc = 0x570A8;
    18
}
/// $05737E: bsr.w $5719c
pub fn i_05737e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57382;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57382);
    cpu.pc = 0x5719C;
    18
}
/// $057382: bra.w $58e08
pub fn i_057382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57386;
    cpu.ir = 0x6000;
    cpu.pc = 0x58E08;
    10
}
/// $057386: bsr.w $570a8
pub fn i_057386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5738A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5738A);
    cpu.pc = 0x570A8;
    18
}
/// $05738A: bsr.w $5719c
pub fn i_05738a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5738E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5738E);
    cpu.pc = 0x5719C;
    18
}
/// $05738E: bsr.w $57200
pub fn i_05738e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57392;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57392);
    cpu.pc = 0x57200;
    18
}
/// $057392: bra.w $5752c
pub fn i_057392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57396;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $057396: bsr.w $570a8
pub fn i_057396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5739A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5739A);
    cpu.pc = 0x570A8;
    18
}
/// $05739A: bsr.w $57200
pub fn i_05739a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5739E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5739E);
    cpu.pc = 0x57200;
    18
}
/// $05739E: bra.w $5752c
pub fn i_05739e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
/// $0573A2: bsr.w $570a8
pub fn i_0573a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x573A6);
    cpu.pc = 0x570A8;
    18
}
/// $0573A6: bsr.w $5719c
pub fn i_0573a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573AA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x573AA);
    cpu.pc = 0x5719C;
    18
}
/// $0573AA: bra.w $5752c
pub fn i_0573aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5752C;
    10
}
// ===== Soldier_SpawnAtGroundA_0573ae  ($0573AE, 40 B, enemies/soldiers/helpers_056acc.s)
/// $0573AE: bsr.w $570a8
pub fn i_0573ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573B2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x573B2);
    cpu.pc = 0x570A8;
    18
}
/// $0573B2: bsr.w $5719c
pub fn i_0573b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573B6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x573B6);
    cpu.pc = 0x5719C;
    18
}
/// $0573B6: jsr $77148.l
pub fn i_0573b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x573BC);
    cpu.pc = 0x77148;
    16
}
/// $0573BC: tst.w d0
pub fn i_0573bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573BE;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0573BE: bpl.w $573ce
pub fn i_0573be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573C2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x573CE; return 10; }
    8
}
/// $0573C2: jsr $5b6.l
pub fn i_0573c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x573C8);
    cpu.pc = 0x5B6;
    16
}
/// $0573C8: jmp $518.l
pub fn i_0573c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573CE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0573CE: move.w d6, $22(a6)
pub fn i_0573ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573D2;
    cpu.ir = 0x3D46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0573D2: move.b d0, $77(a6)
pub fn i_0573d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573D6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_0573d6  ($0573D6, 8 B, kernel/task_handlers.c)
/// $0573D6: lea.l $58b1e(pc), a1
pub fn i_0573d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58B1E;
    16
}
/// $0573DA: move.l a1, (a6)
pub fn i_0573da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0573DC: rts
pub fn i_0573dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnAtGroundB_0573de  ($0573DE, 40 B, enemies/soldiers/helpers_056acc.s)
/// $0573DE: bsr.w $570a8
pub fn i_0573de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x573E2);
    cpu.pc = 0x570A8;
    18
}
/// $0573E2: bsr.w $5719c
pub fn i_0573e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573E6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x573E6);
    cpu.pc = 0x5719C;
    18
}
/// $0573E6: jsr $77148.l
pub fn i_0573e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x573EC);
    cpu.pc = 0x77148;
    16
}
/// $0573EC: tst.w d0
pub fn i_0573ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573EE;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0573EE: bpl.w $573fe
pub fn i_0573ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573F2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x573FE; return 10; }
    8
}
/// $0573F2: jsr $5b6.l
pub fn i_0573f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x573F8);
    cpu.pc = 0x5B6;
    16
}
/// $0573F8: jmp $518.l
pub fn i_0573f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x573FE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0573FE: move.w d6, $22(a6)
pub fn i_0573fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57402;
    cpu.ir = 0x3D46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057402: move.b d0, $77(a6)
pub fn i_057402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57406;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_057406  ($057406, 8 B, kernel/task_handlers.c)
/// $057406: lea.l $58cce(pc), a1
pub fn i_057406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5740A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58CCE;
    16
}
/// $05740A: move.l a1, (a6)
pub fn i_05740a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5740C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05740C: rts
pub fn i_05740c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5740E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_PickFallAnim_05740e  ($05740E, 42 B, enemies/soldiers/helpers_056acc.s)
/// $05740E: move.w $28(a6), d0
pub fn i_05740e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57412;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057412: bpl.w $57418
pub fn i_057412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57416;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57418; return 10; }
    8
}
/// $057416: neg.w d0
pub fn i_057416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57418;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $057418: move.w $2a(a6), d1
pub fn i_057418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5741C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05741C: bpl.w $57422
pub fn i_05741c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57420;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57422; return 10; }
    8
}
/// $057420: neg.w d1
pub fn i_057420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57422;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $057422: cmp.w d1, d0
pub fn i_057422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57424;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $057424: bcc.w $57432
pub fn i_057424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57428;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57432; return 10; }
    8
}
/// $057428: lea.l $2b5c22.l, a0
pub fn i_057428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5742E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C22;
    20
}
/// $05742E: bra.w $57438
pub fn i_05742e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57432;
    cpu.ir = 0x6000;
    cpu.pc = 0x57438;
    10
}
/// $057432: lea.l $29b7c8.l, a0
pub fn i_057432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57438;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B7C8;
    20
}
// ===== JsrAbsThunk_057438  ($057438, 8 B, thunks/jsr_abs_thunks.c)
/// $057438: jsr $28cd4.l
pub fn i_057438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5743E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5743E);
    cpu.pc = 0x28CD4;
    16
}
/// $05743E: rts
pub fn i_05743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57440;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_TestMeleeRange_0574e8  ($0574E8, 56 B, enemies/soldiers/helpers_056acc.s)
/// $0574E8: move.w $22(a6), d0
pub fn i_0574e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x574EC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0574EC: cmpi.w #$20, d0
pub fn i_0574ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x574F0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $0574F0: bmi.w $57526
pub fn i_0574f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x574F4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x57526; return 10; }
    8
}
/// $0574F4: cmpi.w #$300, d0
pub fn i_0574f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x574F8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $0574F8: bpl.w $57526
pub fn i_0574f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x574FC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57526; return 10; }
    8
}
/// $0574FC: lea.l $57440(pc), a0
pub fn i_0574fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57500;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x57440;
    16
}
/// $057500: move.l a0, $4c(a6)
pub fn i_057500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57504;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057504: jsr $283ca.l
pub fn i_057504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5750A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5750A);
    cpu.pc = 0x283CA;
    16
}
/// $05750A: jsr $283ca.l
pub fn i_05750a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57510);
    cpu.pc = 0x283CA;
    16
}
/// $057510: jsr $283d8.l
pub fn i_057510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57516);
    cpu.pc = 0x283D8;
    16
}
/// $057516: btst.b #$1, $13(a6)
pub fn i_057516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5751C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05751C: beq.w $57526
pub fn i_05751c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57520;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57526; return 10; }
    8
}
// ===== SetC_057520  ($057520, 6 B, thunks/ccr_helpers.c)
/// $057520: ori.b #$1, ccr
pub fn i_057520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57524;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $057524: rts
pub fn i_057524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57526;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_057526  ($057526, 6 B, thunks/ccr_helpers.c)
/// $057526: andi.b #$fe, ccr
pub fn i_057526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5752A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05752A: rts
pub fn i_05752a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5752C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnDispatch_05752c  ($05752C, 36 B, enemies/soldiers/helpers_056acc.s)
/// $05752C: move.w $22(a6), d0
pub fn i_05752c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57530;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057530: cmpi.w #$a0, d0
pub fn i_057530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57534;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $057534: bpl.w $5753e
pub fn i_057534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57538;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5753E; return 10; }
    8
}
/// $057538: bset.b #$0, $3a(a6)
pub fn i_057538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5753E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05753E: tst.b $78(a6)
pub fn i_05753e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57542;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $057542: beq.w $57550
pub fn i_057542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57546;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57550; return 10; }
    8
}
/// $057546: lea.l $57558(pc), a1
pub fn i_057546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5754A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $05754A: move.l a1, (a6)
pub fn i_05754a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5754C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05754C: bra.w $57556
pub fn i_05754c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57550;
    cpu.ir = 0x6000;
    cpu.pc = 0x57556;
    10
}
// ===== SetTaskHandler_057550  ($057550, 8 B, kernel/task_handlers.c)
/// $057550: lea.l $58412(pc), a1
pub fn i_057550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57554;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $057554: move.l a1, (a6)
pub fn i_057554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57556;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057556: rts
pub fn i_057556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57558;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_WalkStart_057558  ($057558, 42 B, enemies/soldiers/helpers_056acc.s)
/// $057558: lea.l $29b744.l, a0
pub fn i_057558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5755E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $05755E: jsr $28cd4.l
pub fn i_05755e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57564);
    cpu.pc = 0x28CD4;
    16
}
/// $057564: lea.l $2b756e.l, a0
pub fn i_057564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5756A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B756E;
    20
}
/// $05756A: jsr $799de.l
pub fn i_05756a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57570;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57570);
    cpu.pc = 0x799DE;
    16
}
/// $057570: move.w d0, $36(a6)
pub fn i_057570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57574;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057574: bsr.w $56f8a
pub fn i_057574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57578;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57578);
    cpu.pc = 0x56F8A;
    18
}
/// $057578: clr.w $2a(a6)
pub fn i_057578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5757C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05757C: lea.l $57582(pc), a1
pub fn i_05757c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57580;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57582;
    16
}
/// $057580: move.l a1, (a6)
pub fn i_057580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57582;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Walk_Loop_057582  ($057582, 388 B, enemies/soldiers/helpers_056acc.s)
/// $057582: bsr.w $56acc
pub fn i_057582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57586;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57586);
    cpu.pc = 0x56ACC;
    18
}
/// $057586: bsr.w $56b92
pub fn i_057586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5758A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5758A);
    cpu.pc = 0x56B92;
    18
}
/// $05758A: bsr.w $56e4a
pub fn i_05758a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5758E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5758E);
    cpu.pc = 0x56E4A;
    18
}
/// $05758E: bcc.w $5759c
pub fn i_05758e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57592;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5759C; return 10; }
    8
}
/// $057592: lea.l $57728(pc), a1
pub fn i_057592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57596;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57728;
    16
}
/// $057596: move.l a1, (a6)
pub fn i_057596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57598;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057598: bra.w $57728
pub fn i_057598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5759C;
    cpu.ir = 0x6000;
    cpu.pc = 0x57728;
    10
}
/// $05759C: bsr.w $56eda
pub fn i_05759c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575A0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x575A0);
    cpu.pc = 0x56EDA;
    18
}
/// $0575A0: bcc.w $575ae
pub fn i_0575a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x575AE; return 10; }
    8
}
/// $0575A4: lea.l $577bc(pc), a1
pub fn i_0575a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x577BC;
    16
}
/// $0575A8: move.l a1, (a6)
pub fn i_0575a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0575AA: bra.w $577bc
pub fn i_0575aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x577BC;
    10
}
/// $0575AE: jsr $28d70.l
pub fn i_0575ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x575B4);
    cpu.pc = 0x28D70;
    16
}
/// $0575B4: bsr.w $56fec
pub fn i_0575b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575B8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x575B8);
    cpu.pc = 0x56FEC;
    18
}
/// $0575B8: btst.b #$0, $72(a6)
pub fn i_0575b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575BE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0575BE: beq.w $5767c
pub fn i_0575be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5767C; return 10; }
    8
}
/// $0575C2: move.w $82(a6), d1
pub fn i_0575c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575C6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0575C6: cmp.w $84(a6), d1
pub fn i_0575c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575CA;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0575CA: bcc.w $575d2
pub fn i_0575ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x575D2; return 10; }
    8
}
/// $0575CE: move.w $84(a6), d1
pub fn i_0575ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575D2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0575D2: neg.w d1
pub fn i_0575d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575D4;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0575D4: addi.w #$100, d1
pub fn i_0575d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575D8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0575D8: jsr $5e9b6.l
pub fn i_0575d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x575DE);
    cpu.pc = 0x5E9B6;
    16
}
/// $0575DE: move.w d0, -(a7)
pub fn i_0575de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575E0;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0575E0: andi.w #$ff, d0
pub fn i_0575e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575E4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0575E4: cmp.w d1, d0
pub fn i_0575e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575E6;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0575E6: bcc.w $575ee
pub fn i_0575e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x575EE; return 10; }
    8
}
/// $0575EA: move.b (a7), d0
pub fn i_0575ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575EC;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0575EC: cmp.w d1, d0
pub fn i_0575ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575EE;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0575EE: addq.w #$2, a7
pub fn i_0575ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575F0;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0575F0: bcc.w $575fa
pub fn i_0575f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x575FA; return 10; }
    8
}
/// $0575F4: lea.l $5804c(pc), a1
pub fn i_0575f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5804C;
    16
}
/// $0575F8: move.l a1, (a6)
pub fn i_0575f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x575FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0575FA: jsr $5e9b6.l
pub fn i_0575fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57600);
    cpu.pc = 0x5E9B6;
    16
}
/// $057600: move.w d0, -(a7)
pub fn i_057600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57602;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $057602: andi.w #$ff, d0
pub fn i_057602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57606;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057606: cmp.w $8a(a6), d0
pub fn i_057606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5760A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05760A: bcc.w $57614
pub fn i_05760a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5760E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57614; return 10; }
    8
}
/// $05760E: move.b (a7), d0
pub fn i_05760e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57610;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $057610: cmp.w $8a(a6), d0
pub fn i_057610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57614;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $057614: addq.w #$2, a7
pub fn i_057614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57616;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $057616: bcc.w $57620
pub fn i_057616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5761A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57620; return 10; }
    8
}
/// $05761A: lea.l $58658(pc), a1
pub fn i_05761a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5761E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58658;
    16
}
/// $05761E: move.l a1, (a6)
pub fn i_05761e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57620;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057620: move.w $86(a6), d1
pub fn i_057620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57624;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057624: cmp.w $80(a6), d1
pub fn i_057624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57628;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $057628: bcc.w $57630
pub fn i_057628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5762C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57630; return 10; }
    8
}
/// $05762C: move.w $80(a6), d1
pub fn i_05762c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57630;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057630: jsr $5e9b6.l
pub fn i_057630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57636);
    cpu.pc = 0x5E9B6;
    16
}
/// $057636: move.w d0, -(a7)
pub fn i_057636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57638;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $057638: andi.w #$ff, d0
pub fn i_057638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5763C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05763C: cmp.w d1, d0
pub fn i_05763c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5763E;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05763E: bcc.w $57646
pub fn i_05763e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57642;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57646; return 10; }
    8
}
/// $057642: move.b (a7), d0
pub fn i_057642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57644;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $057644: cmp.w d1, d0
pub fn i_057644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57646;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $057646: addq.w #$2, a7
pub fn i_057646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57648;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $057648: bcc.w $57652
pub fn i_057648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5764C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57652; return 10; }
    8
}
/// $05764C: lea.l $585f6(pc), a1
pub fn i_05764c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57650;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $057650: move.l a1, (a6)
pub fn i_057650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57652;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057652: jsr $5e9b6.l
pub fn i_057652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57658);
    cpu.pc = 0x5E9B6;
    16
}
/// $057658: move.w d0, -(a7)
pub fn i_057658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5765A;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05765A: andi.w #$ff, d0
pub fn i_05765a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5765E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05765E: cmp.w $88(a6), d0
pub fn i_05765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57662;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $057662: bcc.w $5766c
pub fn i_057662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57666;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5766C; return 10; }
    8
}
/// $057666: move.b (a7), d0
pub fn i_057666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57668;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $057668: cmp.w $88(a6), d0
pub fn i_057668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5766C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05766C: addq.w #$2, a7
pub fn i_05766c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5766E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05766E: bcc.w $57678
pub fn i_05766e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57672;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57678; return 10; }
    8
}
/// $057672: lea.l $588f6(pc), a1
pub fn i_057672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57676;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588F6;
    16
}
/// $057676: move.l a1, (a6)
pub fn i_057676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57678;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057678: bra.w $576a2
pub fn i_057678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5767C;
    cpu.ir = 0x6000;
    cpu.pc = 0x576A2;
    10
}
/// $05767C: move.w $82(a6), d1
pub fn i_05767c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57680;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057680: jsr $5e9b6.l
pub fn i_057680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57686;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57686);
    cpu.pc = 0x5E9B6;
    16
}
/// $057686: move.w d0, -(a7)
pub fn i_057686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57688;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $057688: andi.w #$ff, d0
pub fn i_057688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5768C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05768C: cmp.w d1, d0
pub fn i_05768c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5768E;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05768E: bcc.w $57696
pub fn i_05768e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57692;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57696; return 10; }
    8
}
/// $057692: move.b (a7), d0
pub fn i_057692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57694;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $057694: cmp.w d1, d0
pub fn i_057694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57696;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $057696: addq.w #$2, a7
pub fn i_057696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57698;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $057698: bcc.w $576a2
pub fn i_057698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5769C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x576A2; return 10; }
    8
}
/// $05769C: lea.l $5804c(pc), a1
pub fn i_05769c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5804C;
    16
}
/// $0576A0: move.l a1, (a6)
pub fn i_0576a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0576A2: btst.b #$5, $72(a6)
pub fn i_0576a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576A8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0576A8: beq.w $576b2
pub fn i_0576a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x576B2; return 10; }
    8
}
/// $0576AC: lea.l $585f6(pc), a1
pub fn i_0576ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $0576B0: move.l a1, (a6)
pub fn i_0576b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0576B2: btst.b #$6, $72(a6)
pub fn i_0576b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576B8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0576B8: beq.w $576c2
pub fn i_0576b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x576C2; return 10; }
    8
}
/// $0576BC: lea.l $58a50(pc), a1
pub fn i_0576bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A50;
    16
}
/// $0576C0: move.l a1, (a6)
pub fn i_0576c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0576C2: jsr $56e36(pc)
pub fn i_0576c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x576C6);
    cpu.pc = 0x56E36;
    16
}
/// $0576C6: bcc.w $576d0
pub fn i_0576c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x576D0; return 10; }
    8
}
/// $0576CA: lea.l $59062(pc), a1
pub fn i_0576ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59062;
    16
}
/// $0576CE: move.l a1, (a6)
pub fn i_0576ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0576D0: jsr $56fa0(pc)
pub fn i_0576d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x576D4);
    cpu.pc = 0x56FA0;
    16
}
/// $0576D4: bcc.w $576de
pub fn i_0576d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x576DE; return 10; }
    8
}
/// $0576D8: lea.l $58968(pc), a1
pub fn i_0576d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58968;
    16
}
/// $0576DC: move.l a1, (a6)
pub fn i_0576dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0576DE: tst.b $78(a6)
pub fn i_0576de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576E2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0576E2: bne.w $576ec
pub fn i_0576e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x576EC; return 10; }
    8
}
/// $0576E6: lea.l $57706(pc), a1
pub fn i_0576e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57706;
    16
}
/// $0576EA: move.l a1, (a6)
pub fn i_0576ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0576EC: jsr $574e8(pc)
pub fn i_0576ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x576F0);
    cpu.pc = 0x574E8;
    16
}
/// $0576F0: bcc.w $576fa
pub fn i_0576f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x576FA; return 10; }
    8
}
/// $0576F4: lea.l $585ae(pc), a1
pub fn i_0576f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585AE;
    16
}
/// $0576F8: move.l a1, (a6)
pub fn i_0576f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0576FA: jsr $49fd0.l
pub fn i_0576fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57700;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57700);
    cpu.pc = 0x49FD0;
    16
}
/// $057700: bsr.w $56e1e
pub fn i_057700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57704;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57704);
    cpu.pc = 0x56E1E;
    18
}
/// $057704: rts
pub fn i_057704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57706;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_AirborneDispatch_057706  ($057706, 18 B, enemies/soldiers/helpers_056acc.s)
/// $057706: jsr $5e9b6.l
pub fn i_057706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5770C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5770C);
    cpu.pc = 0x5E9B6;
    16
}
/// $05770C: lea.l $57718(pc), a0
pub fn i_05770c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57710;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x57718;
    16
}
/// $057710: andi.w #$c, d0
pub fn i_057710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57714;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057714: jmp (a0, d0.w)
pub fn i_057714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57718;
    cpu.ir = 0x4EF0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
// ===== Soldier_AirborneJT_057718  ($057718, 16 B, enemies/soldiers/helpers_056acc.s)
/// $057718: bra.w $58a50
pub fn i_057718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5771C;
    cpu.ir = 0x6000;
    cpu.pc = 0x58A50;
    10
}
/// $05771C: bra.w $58658
pub fn i_05771c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57720;
    cpu.ir = 0x6000;
    cpu.pc = 0x58658;
    10
}
/// $057720: bra.w $58412
pub fn i_057720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57724;
    cpu.ir = 0x6000;
    cpu.pc = 0x58412;
    10
}
/// $057724: bra.w $58a50
pub fn i_057724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57728;
    cpu.ir = 0x6000;
    cpu.pc = 0x58A50;
    10
}
// ===== Soldier_GrabApproach_057728  ($057728, 142 B, enemies/soldiers/helpers_056acc.s)
/// $057728: bsr.w $56acc
pub fn i_057728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5772C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5772C);
    cpu.pc = 0x56ACC;
    18
}
/// $05772C: bsr.w $56b92
pub fn i_05772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57730;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57730);
    cpu.pc = 0x56B92;
    18
}
/// $057730: jsr $28d70.l
pub fn i_057730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57736);
    cpu.pc = 0x28D70;
    16
}
/// $057736: bsr.w $56e4a
pub fn i_057736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5773A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5773A);
    cpu.pc = 0x56E4A;
    18
}
/// $05773A: bcc.w $577aa
pub fn i_05773a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5773E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x577AA; return 10; }
    8
}
/// $05773E: move.w $80(a6), d1
pub fn i_05773e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57742;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057742: mulu.w d1, d1
pub fn i_057742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57744;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $057744: lsr.l #$8, d1
pub fn i_057744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57746;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $057746: jsr $5e9b6.l
pub fn i_057746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5774C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5774C);
    cpu.pc = 0x5E9B6;
    16
}
/// $05774C: move.w d0, -(a7)
pub fn i_05774c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5774E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05774E: andi.w #$ff, d0
pub fn i_05774e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57752;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057752: cmp.w d1, d0
pub fn i_057752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57754;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $057754: bcc.w $5775c
pub fn i_057754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57758;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5775C; return 10; }
    8
}
/// $057758: move.b (a7), d0
pub fn i_057758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5775A;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05775A: cmp.w d1, d0
pub fn i_05775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5775C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05775C: addq.w #$2, a7
pub fn i_05775c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5775E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05775E: bcc.w $57768
pub fn i_05775e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57762;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57768; return 10; }
    8
}
/// $057762: lea.l $578dc(pc), a1
pub fn i_057762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x578DC;
    16
}
/// $057766: move.l a1, (a6)
pub fn i_057766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057768: btst.b #$0, $72(a6)
pub fn i_057768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5776E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05776E: bne.w $57778
pub fn i_05776e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57772;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57778; return 10; }
    8
}
/// $057772: lea.l $585f6(pc), a1
pub fn i_057772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57776;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $057776: move.l a1, (a6)
pub fn i_057776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57778;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057778: tst.b $78(a6)
pub fn i_057778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5777C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05777C: bne.w $57786
pub fn i_05777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57780;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57786; return 10; }
    8
}
/// $057780: lea.l $578dc(pc), a1
pub fn i_057780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57784;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x578DC;
    16
}
/// $057784: move.l a1, (a6)
pub fn i_057784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57786;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057786: btst.b #$6, $72(a6)
pub fn i_057786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5778C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05778C: beq.w $57796
pub fn i_05778c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57790;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57796; return 10; }
    8
}
/// $057790: lea.l $58a50(pc), a1
pub fn i_057790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57794;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A50;
    16
}
/// $057794: move.l a1, (a6)
pub fn i_057794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57796;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057796: btst.b #$5, $72(a6)
pub fn i_057796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5779C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05779C: beq.w $577a6
pub fn i_05779c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x577A6; return 10; }
    8
}
/// $0577A0: lea.l $5804c(pc), a1
pub fn i_0577a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5804C;
    16
}
/// $0577A4: move.l a1, (a6)
pub fn i_0577a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0577A6: bra.w $577b0
pub fn i_0577a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x577B0;
    10
}
/// $0577AA: lea.l $57582(pc), a1
pub fn i_0577aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57582;
    16
}
/// $0577AE: move.l a1, (a6)
pub fn i_0577ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0577B0: jsr $49fd0.l
pub fn i_0577b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x577B6);
    cpu.pc = 0x49FD0;
    16
}
// ===== JsrPcThunk_0577b6  ($0577B6, 6 B, thunks/jsr_pc_thunks.c)
/// $0577B6: jsr $56e1e(pc)
pub fn i_0577b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x577BA);
    cpu.pc = 0x56E1E;
    16
}
/// $0577BA: rts
pub fn i_0577ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SlugApproach_0577bc  ($0577BC, 124 B, enemies/soldiers/helpers_056acc.s)
/// $0577BC: bsr.w $56acc
pub fn i_0577bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577C0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x577C0);
    cpu.pc = 0x56ACC;
    18
}
/// $0577C0: bsr.w $56b92
pub fn i_0577c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577C4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x577C4);
    cpu.pc = 0x56B92;
    18
}
/// $0577C4: jsr $28d70.l
pub fn i_0577c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x577CA);
    cpu.pc = 0x28D70;
    16
}
/// $0577CA: move.w $80(a6), d1
pub fn i_0577ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577CE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0577CE: mulu.w d1, d1
pub fn i_0577ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577D0;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0577D0: lsr.l #$8, d1
pub fn i_0577d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577D2;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $0577D2: jsr $5e9b6.l
pub fn i_0577d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x577D8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0577D8: move.w d0, -(a7)
pub fn i_0577d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577DA;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0577DA: andi.w #$ff, d0
pub fn i_0577da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577DE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0577DE: cmp.w d1, d0
pub fn i_0577de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577E0;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0577E0: bcc.w $577e8
pub fn i_0577e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x577E8; return 10; }
    8
}
/// $0577E4: move.b (a7), d0
pub fn i_0577e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577E6;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0577E6: cmp.w d1, d0
pub fn i_0577e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577E8;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0577E8: addq.w #$2, a7
pub fn i_0577e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577EA;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0577EA: bcc.w $577f4
pub fn i_0577ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x577F4; return 10; }
    8
}
/// $0577EE: lea.l $5799e(pc), a1
pub fn i_0577ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5799E;
    16
}
/// $0577F2: move.l a1, (a6)
pub fn i_0577f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0577F4: btst.b #$0, $72(a6)
pub fn i_0577f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577FA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0577FA: bne.w $57804
pub fn i_0577fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x577FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57804; return 10; }
    8
}
/// $0577FE: lea.l $585f6(pc), a1
pub fn i_0577fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57802;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $057802: move.l a1, (a6)
pub fn i_057802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57804;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057804: tst.b $78(a6)
pub fn i_057804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57808;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $057808: bne.w $57812
pub fn i_057808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5780C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57812; return 10; }
    8
}
/// $05780C: lea.l $5799e(pc), a1
pub fn i_05780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57810;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5799E;
    16
}
/// $057810: move.l a1, (a6)
pub fn i_057810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57812;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057812: btst.b #$6, $72(a6)
pub fn i_057812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57818;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057818: beq.w $57822
pub fn i_057818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5781C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57822; return 10; }
    8
}
/// $05781C: lea.l $58a50(pc), a1
pub fn i_05781c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57820;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A50;
    16
}
/// $057820: move.l a1, (a6)
pub fn i_057820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57822;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057822: btst.b #$5, $72(a6)
pub fn i_057822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57828;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057828: beq.w $57832
pub fn i_057828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5782C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57832; return 10; }
    8
}
/// $05782C: lea.l $5804c(pc), a1
pub fn i_05782c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57830;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5804C;
    16
}
/// $057830: move.l a1, (a6)
pub fn i_057830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57832;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057832: jsr $49fd0.l
pub fn i_057832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57838);
    cpu.pc = 0x49FD0;
    16
}
// ===== JsrPcThunk_057838  ($057838, 6 B, thunks/jsr_pc_thunks.c)
/// $057838: jsr $56e1e(pc)
pub fn i_057838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5783C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5783C);
    cpu.pc = 0x56E1E;
    16
}
/// $05783C: rts
pub fn i_05783c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5783E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnJumpIn_05783e  ($05783E, 66 B, enemies/soldiers/helpers_056acc.s)
/// $05783E: bsr.w $570a8
pub fn i_05783e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57842;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57842);
    cpu.pc = 0x570A8;
    18
}
/// $057842: bsr.w $5719c
pub fn i_057842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57846;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57846);
    cpu.pc = 0x5719C;
    18
}
/// $057846: lea.l $2b69d8.l, a0
pub fn i_057846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5784C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B69D8;
    20
}
/// $05784C: jsr $28cd4.l
pub fn i_05784c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57852);
    cpu.pc = 0x28CD4;
    16
}
/// $057852: move.w #$fe67, d0
pub fn i_057852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57856;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057856: jsr $5dca4.l
pub fn i_057856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5785C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5785C);
    cpu.pc = 0x5DCA4;
    16
}
/// $05785C: move.w d0, $28(a6)
pub fn i_05785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57860;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057860: move.w #$190, $2a(a6)
pub fn i_057860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57866;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057866: move.w #$ffd8, $2e(a6)
pub fn i_057866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5786C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05786C: move.w #$0, $2c(a6)
pub fn i_05786c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57872;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057872: clr.w $2a(a6)
pub fn i_057872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57876;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $057876: lea.l $58a82(pc), a1
pub fn i_057876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5787A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58A82;
    16
}
/// $05787A: move.l a1, (a6)
pub fn i_05787a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5787C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05787C: bra.w $58a82
pub fn i_05787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57880;
    cpu.ir = 0x6000;
    cpu.pc = 0x58A82;
    10
}
// ===== Soldier_Leap_057880  ($057880, 542 B, enemies/soldiers/helpers_056acc.s)
/// $057880: bsr.w $570a8
pub fn i_057880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57884;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57884);
    cpu.pc = 0x570A8;
    18
}
/// $057884: bsr.w $5720e
pub fn i_057884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57888;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57888);
    cpu.pc = 0x5720E;
    18
}
/// $057888: bsr.w $5721e
pub fn i_057888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5788C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5788C);
    cpu.pc = 0x5721E;
    18
}
/// $05788C: bsr.w $57216
pub fn i_05788c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57890;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57890);
    cpu.pc = 0x57216;
    18
}
/// $057890: lea.l $2b69aa.l, a0
pub fn i_057890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57896;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B69AA;
    20
}
/// $057896: jsr $28cd4.l
pub fn i_057896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5789C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5789C);
    cpu.pc = 0x28CD4;
    16
}
/// $05789C: move.w #$fe67, d0
pub fn i_05789c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0578A0: jsr $5dca4.l
pub fn i_0578a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x578A6);
    cpu.pc = 0x5DCA4;
    16
}
/// $0578A6: move.w d0, $28(a6)
pub fn i_0578a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578AA: move.w #$190, $2a(a6)
pub fn i_0578aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578B0: move.w #$ffd8, $2e(a6)
pub fn i_0578b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578B6: move.w #$0, $2c(a6)
pub fn i_0578b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578BC: clr.w $2a(a6)
pub fn i_0578bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578C0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0578C0: bsr.w $56b92
pub fn i_0578c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578C4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x578C4);
    cpu.pc = 0x56B92;
    18
}
/// $0578C4: bsr.w $56eda
pub fn i_0578c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578C8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x578C8);
    cpu.pc = 0x56EDA;
    18
}
/// $0578C8: bcc.w $57908
pub fn i_0578c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57908; return 10; }
    8
}
/// $0578CC: lea.l $2b6fee.l, a0
pub fn i_0578cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6FEE;
    20
}
/// $0578D2: jsr $28cd4.l
pub fn i_0578d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x578D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0578D8: bra.w $57a5a
pub fn i_0578d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x57A5A;
    10
}
/// $0578DC: lea.l $2b6922.l, a0
pub fn i_0578dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6922;
    20
}
/// $0578E2: jsr $28cd4.l
pub fn i_0578e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x578E8);
    cpu.pc = 0x28CD4;
    16
}
/// $0578E8: move.w #$fdde, d0
pub fn i_0578e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFDDE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0578EC: jsr $5dca4.l
pub fn i_0578ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x578F2);
    cpu.pc = 0x5DCA4;
    16
}
/// $0578F2: move.w d0, $28(a6)
pub fn i_0578f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578F6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578F6: move.w #$663, $2a(a6)
pub fn i_0578f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x578FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x663;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0578FC: move.w #$ff93, $2e(a6)
pub fn i_0578fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57902;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF93;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057902: move.w #$0, $2c(a6)
pub fn i_057902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57908;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057908: move.l $7a(a6), d0
pub fn i_057908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5790C;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05790C: beq.w $5793e
pub fn i_05790c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57910;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5793E; return 10; }
    8
}
/// $057910: cmpi.l #$100440, d0
pub fn i_057910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57916;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100440, Size::Long, false);
    8
}
/// $057916: bne.w $5792e
pub fn i_057916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5791A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5792E; return 10; }
    8
}
/// $05791A: lea.l $2b71dc.l, a0
pub fn i_05791a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57920;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B71DC;
    20
}
/// $057920: move.l a0, $4c(a6)
pub fn i_057920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57924;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057924: jsr $283ca.l
pub fn i_057924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5792A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5792A);
    cpu.pc = 0x283CA;
    16
}
/// $05792A: bra.w $5793e
pub fn i_05792a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5792E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5793E;
    10
}
/// $05792E: lea.l $2b7230.l, a0
pub fn i_05792e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57934;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7230;
    20
}
/// $057934: move.l a0, $4c(a6)
pub fn i_057934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57938;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057938: jsr $283ca.l
pub fn i_057938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5793E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5793E);
    cpu.pc = 0x283CA;
    16
}
/// $05793E: lea.l $57944(pc), a1
pub fn i_05793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57944;
    16
}
/// $057942: move.l a1, (a6)
pub fn i_057942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057944: jsr $27d50.l
pub fn i_057944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5794A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5794A);
    cpu.pc = 0x27D50;
    16
}
/// $05794A: bcc.w $57954
pub fn i_05794a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5794E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57954; return 10; }
    8
}
/// $05794E: lea.l $58464(pc), a1
pub fn i_05794e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57952;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58464;
    16
}
/// $057952: move.l a1, (a6)
pub fn i_057952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57954;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057954: jsr $28d70.l
pub fn i_057954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5795A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5795A);
    cpu.pc = 0x28D70;
    16
}
/// $05795A: bsr.w $56e4a
pub fn i_05795a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5795E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5795E);
    cpu.pc = 0x56E4A;
    18
}
/// $05795E: bcc.w $57994
pub fn i_05795e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57962;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57994; return 10; }
    8
}
/// $057962: tst.w $2a(a6)
pub fn i_057962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57966;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $057966: bpl.w $57994
pub fn i_057966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5796A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57994; return 10; }
    8
}
/// $05796A: jsr $283d8.l
pub fn i_05796a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57970);
    cpu.pc = 0x283D8;
    16
}
/// $057970: btst.b #$1, $13(a6)
pub fn i_057970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57976;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057976: beq.w $57994
pub fn i_057976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5797A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57994; return 10; }
    8
}
/// $05797A: movea.l $7a(a6), a0
pub fn i_05797a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5797E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05797E: move.w $24(a0), d0
pub fn i_05797e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57982;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057982: addi.w #$10, d0
pub fn i_057982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57986;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057986: cmp.w $24(a6), d0
pub fn i_057986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5798A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05798A: bpl.w $57994
pub fn i_05798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5798E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57994; return 10; }
    8
}
/// $05798E: lea.l $57b06(pc), a1
pub fn i_05798e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57992;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57B06;
    16
}
/// $057992: move.l a1, (a6)
pub fn i_057992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57994;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057994: jsr $49fd0.l
pub fn i_057994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5799A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5799A);
    cpu.pc = 0x49FD0;
    16
}
/// $05799A: bra.w $56e1e
pub fn i_05799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5799E;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
/// $05799E: lea.l $2b6f40.l, a0
pub fn i_05799e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6F40;
    20
}
/// $0579A4: jsr $28cd4.l
pub fn i_0579a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x579AA);
    cpu.pc = 0x28CD4;
    16
}
/// $0579AA: cmpi.b #$ff, $9d(a6)
pub fn i_0579aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579B0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0579B0: bne.w $579d8
pub fn i_0579b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x579D8; return 10; }
    8
}
/// $0579B4: move.w #$fdde, d0
pub fn i_0579b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579B8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFDDE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0579B8: jsr $5dca4.l
pub fn i_0579b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x579BE);
    cpu.pc = 0x5DCA4;
    16
}
/// $0579BE: move.w d0, $28(a6)
pub fn i_0579be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579C2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579C2: move.w #$438, $2a(a6)
pub fn i_0579c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x438;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579C8: move.w #$ffb8, $2e(a6)
pub fn i_0579c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579CE: move.w #$0, $2c(a6)
pub fn i_0579ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579D4: bra.w $57a54
pub fn i_0579d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x57A54;
    10
}
/// $0579D8: cmpi.b #$1, $9d(a6)
pub fn i_0579d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0579DE: bne.w $57a06
pub fn i_0579de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57A06; return 10; }
    8
}
/// $0579E2: move.w #$fe67, d0
pub fn i_0579e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0579E6: jsr $5dca4.l
pub fn i_0579e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x579EC);
    cpu.pc = 0x5DCA4;
    16
}
/// $0579EC: move.w d0, $28(a6)
pub fn i_0579ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579F0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579F0: move.w #$7f8, $2a(a6)
pub fn i_0579f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579F6: move.w #$ff9a, $2e(a6)
pub fn i_0579f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x579FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0579FC: move.w #$0, $2c(a6)
pub fn i_0579fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A02;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A02: bra.w $57a54
pub fn i_057a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A06;
    cpu.ir = 0x6000;
    cpu.pc = 0x57A54;
    10
}
/// $057A06: cmpi.b #$2, $9d(a6)
pub fn i_057a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A0C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $057A0C: bne.w $57a34
pub fn i_057a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57A34; return 10; }
    8
}
/// $057A10: move.w #$fedc, d0
pub fn i_057a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A14;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFEDC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057A14: jsr $5dca4.l
pub fn i_057a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A1A);
    cpu.pc = 0x5DCA4;
    16
}
/// $057A1A: move.w d0, $28(a6)
pub fn i_057a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A1E: move.w #$7fc, $2a(a6)
pub fn i_057a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A24: move.w #$ffb7, $2e(a6)
pub fn i_057a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A2A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A2A: move.w #$0, $2c(a6)
pub fn i_057a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A30: bra.w $57a54
pub fn i_057a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A34;
    cpu.ir = 0x6000;
    cpu.pc = 0x57A54;
    10
}
/// $057A34: move.w #$fd56, d0
pub fn i_057a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A38;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFD56;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057A38: jsr $5dca4.l
pub fn i_057a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A3E);
    cpu.pc = 0x5DCA4;
    16
}
/// $057A3E: move.w d0, $28(a6)
pub fn i_057a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A42: move.w #$7f8, $2a(a6)
pub fn i_057a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A48;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A48: move.w #$ff56, $2e(a6)
pub fn i_057a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF56;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A4E: move.w #$0, $2c(a6)
pub fn i_057a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A54;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057A54: jsr $283ca.l
pub fn i_057a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A5A);
    cpu.pc = 0x283CA;
    16
}
/// $057A5A: lea.l $57a60(pc), a1
pub fn i_057a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57A60;
    16
}
/// $057A5E: move.l a1, (a6)
pub fn i_057a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057A60: jsr $27d50.l
pub fn i_057a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A66);
    cpu.pc = 0x27D50;
    16
}
/// $057A66: bcc.w $57a70
pub fn i_057a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57A70; return 10; }
    8
}
/// $057A6A: lea.l $58454(pc), a1
pub fn i_057a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58454;
    16
}
/// $057A6E: move.l a1, (a6)
pub fn i_057a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057A70: jsr $28d70.l
pub fn i_057a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A76);
    cpu.pc = 0x28D70;
    16
}
/// $057A76: tst.w $2a(a6)
pub fn i_057a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A7A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $057A7A: bpl.w $57a84
pub fn i_057a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A7E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57A84; return 10; }
    8
}
/// $057A7E: jsr $283d8.l
pub fn i_057a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A84);
    cpu.pc = 0x283D8;
    16
}
/// $057A84: btst.b #$1, $13(a6)
pub fn i_057a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A8A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057A8A: beq.w $57a94
pub fn i_057a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57A94; return 10; }
    8
}
/// $057A8E: lea.l $57a9e(pc), a1
pub fn i_057a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57A9E;
    16
}
/// $057A92: move.l a1, (a6)
pub fn i_057a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057A94: jsr $49fd0.l
pub fn i_057a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57A9A);
    cpu.pc = 0x49FD0;
    16
}
/// $057A9A: bra.w $56e1e
pub fn i_057a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57A9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_LeapLand_057a9e  ($057A9E, 70 B, enemies/soldiers/helpers_056acc.s)
/// $057A9E: lea.l $2b6fba.l, a0
pub fn i_057a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6FBA;
    20
}
/// $057AA4: jsr $28cd4.l
pub fn i_057aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AAA);
    cpu.pc = 0x28CD4;
    16
}
/// $057AAA: jsr $283ca.l
pub fn i_057aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AB0);
    cpu.pc = 0x283CA;
    16
}
/// $057AB0: lea.l $57ab6(pc), a1
pub fn i_057ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57AB6;
    16
}
/// $057AB4: move.l a1, (a6)
pub fn i_057ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057AB6: jsr $27d50.l
pub fn i_057ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57ABC);
    cpu.pc = 0x27D50;
    16
}
/// $057ABC: scs.b $78(a6)
pub fn i_057abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AC0;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $057AC0: jsr $28d70.l
pub fn i_057ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AC6);
    cpu.pc = 0x28D70;
    16
}
/// $057AC6: jsr $5e8da.l
pub fn i_057ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57ACC);
    cpu.pc = 0x5E8DA;
    16
}
/// $057ACC: tst.b $78(a6)
pub fn i_057acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AD0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $057AD0: beq.w $57ada
pub fn i_057ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57ADA; return 10; }
    8
}
/// $057AD4: lea.l $58454(pc), a1
pub fn i_057ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58454;
    16
}
/// $057AD8: move.l a1, (a6)
pub fn i_057ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ADA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057ADA: jsr $49fd0.l
pub fn i_057ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AE0);
    cpu.pc = 0x49FD0;
    16
}
/// $057AE0: bra.w $56e1e
pub fn i_057ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_HitCheckTail_057ae4  ($057AE4, 34 B, enemies/soldiers/helpers_056acc.s)
/// $057AE4: jsr $2870a.l
pub fn i_057ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AEA);
    cpu.pc = 0x2870A;
    16
}
/// $057AEA: bcc.w $57b04
pub fn i_057aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57B04; return 10; }
    8
}
/// $057AEE: cmpi.b #$1c, $58(a6)
pub fn i_057aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AF4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1C, Size::Byte, false);
    16
}
/// $057AF4: bne.w $57afe
pub fn i_057af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57AFE; return 10; }
    8
}
/// $057AF8: jsr $49fd0.l
pub fn i_057af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57AFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57AFE);
    cpu.pc = 0x49FD0;
    16
}
/// $057AFE: bclr.b #$3, $13(a6)
pub fn i_057afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B04;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057B04: rts
pub fn i_057b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabLatch_057b06  ($057B06, 174 B, enemies/soldiers/helpers_056acc.s)
/// $057B06: lea.l $2b6a02.l, a0
pub fn i_057b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6A02;
    20
}
/// $057B0C: jsr $28cd4.l
pub fn i_057b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57B12);
    cpu.pc = 0x28CD4;
    16
}
/// $057B12: lea.l $2b67de.l, a0
pub fn i_057b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B67DE;
    20
}
/// $057B18: move.l a0, $48(a6)
pub fn i_057b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B1C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057B1C: bclr.b #$3, $13(a6)
pub fn i_057b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B22;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057B22: lea.l $ffff.w, a0
pub fn i_057b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B26;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $057B26: move.l a0, $4c(a6)
pub fn i_057b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B2A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057B2A: jsr $283ca.l
pub fn i_057b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57B30);
    cpu.pc = 0x283CA;
    16
}
/// $057B30: clr.w $80(a6)
pub fn i_057b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B34;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $057B34: bsr.w $56e4a
pub fn i_057b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B38;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57B38);
    cpu.pc = 0x56E4A;
    18
}
/// $057B38: bcs.w $57b44
pub fn i_057b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B3C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57B44; return 10; }
    8
}
/// $057B3C: bra.w $57fc6
pub fn i_057b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B40;
    cpu.ir = 0x6000;
    cpu.pc = 0x57FC6;
    10
}
/// $057B40: bra.w $57b66
pub fn i_057b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B44;
    cpu.ir = 0x6000;
    cpu.pc = 0x57B66;
    10
}
/// $057B44: movea.l $7a(a6), a0
pub fn i_057b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B48;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057B48: move.w $22(a6), d0
pub fn i_057b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B4C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B4C: move.w $24(a6), d1
pub fn i_057b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B50;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B50: sub.w $22(a0), d0
pub fn i_057b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B54;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057B54: sub.w $24(a0), d1
pub fn i_057b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B58;
    cpu.ir = 0x9268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057B58: move.w d0, $88(a6)
pub fn i_057b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B5C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057B5C: move.w d1, $8a(a6)
pub fn i_057b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B60;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057B60: lea.l $57b66(pc), a1
pub fn i_057b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57B66;
    16
}
/// $057B64: move.l a1, (a6)
pub fn i_057b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057B66: bsr.w $56e4a
pub fn i_057b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B6A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57B6A);
    cpu.pc = 0x56E4A;
    18
}
/// $057B6A: bcs.w $57b74
pub fn i_057b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B6E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57B74; return 10; }
    8
}
/// $057B6E: lea.l $57fc6(pc), a1
pub fn i_057b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057B72: move.l a1, (a6)
pub fn i_057b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057B74: movea.l $7a(a6), a0
pub fn i_057b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B78;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057B78: move.w $38(a0), d0
pub fn i_057b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B7C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B7C: andi.w #$ffe3, d0
pub fn i_057b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B80;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057B80: ori.w #$18, d0
pub fn i_057b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B84;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057B84: move.w d0, $38(a6)
pub fn i_057b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B88;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057B88: move.w $22(a0), d0
pub fn i_057b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B8C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B8C: move.w $24(a0), d1
pub fn i_057b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B90;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057B90: add.w $88(a6), d0
pub fn i_057b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B94;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057B94: add.w $8a(a6), d1
pub fn i_057b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B98;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057B98: move.w d0, $22(a6)
pub fn i_057b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57B9C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057B9C: move.w d1, $24(a6)
pub fn i_057b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BA0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057BA0: jsr $28d70.l
pub fn i_057ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57BA6);
    cpu.pc = 0x28D70;
    16
}
/// $057BA6: bcc.w $57bb0
pub fn i_057ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57BB0; return 10; }
    8
}
/// $057BAA: lea.l $57bb4(pc), a1
pub fn i_057baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57BB4;
    16
}
/// $057BAE: move.l a1, (a6)
pub fn i_057bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057BB0: jmp $57ae4(pc)
pub fn i_057bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BB4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabSlideToAnchor_057bb4  ($057BB4, 244 B, enemies/soldiers/helpers_056acc.s)
/// $057BB4: lea.l $2b6a60.l, a0
pub fn i_057bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6A60;
    20
}
/// $057BBA: jsr $28cd4.l
pub fn i_057bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57BC0);
    cpu.pc = 0x28CD4;
    16
}
/// $057BC0: bclr.b #$0, $3a(a6)
pub fn i_057bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BC6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057BC6: bsr.w $56e4a
pub fn i_057bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BCA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57BCA);
    cpu.pc = 0x56E4A;
    18
}
/// $057BCA: bcc.w $57fc6
pub fn i_057bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FC6; return 10; }
    8
}
/// $057BCE: move.b d0, $76(a6)
pub fn i_057bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BD2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057BD2: movea.l $7a(a6), a0
pub fn i_057bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BD6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057BD6: jsr $8f82a.l
pub fn i_057bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57BDC);
    cpu.pc = 0x8F82A;
    16
}
/// $057BDC: bcc.w $57fc6
pub fn i_057bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FC6; return 10; }
    8
}
/// $057BE0: sub.w $22(a6), d0
pub fn i_057be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BE4;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057BE4: sub.w $24(a6), d1
pub fn i_057be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BE8;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057BE8: neg.w d0
pub fn i_057be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BEA;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $057BEA: neg.w d1
pub fn i_057bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BEC;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $057BEC: move.w d0, $88(a6)
pub fn i_057bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BF0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057BF0: move.w d1, $8a(a6)
pub fn i_057bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BF4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057BF4: lea.l $57bfa(pc), a1
pub fn i_057bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57BFA;
    16
}
/// $057BF8: move.l a1, (a6)
pub fn i_057bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057BFA: move.w $88(a6), d0
pub fn i_057bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57BFE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057BFE: beq.w $57c0e
pub fn i_057bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57C0E; return 10; }
    8
}
/// $057C02: asr.w #$5, d0
pub fn i_057c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C04;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $057C04: bne.w $57c0a
pub fn i_057c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57C0A; return 10; }
    8
}
/// $057C08: moveq #$1, d0
pub fn i_057c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C0A;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $057C0A: sub.w d0, $88(a6)
pub fn i_057c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C0E;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $057C0E: move.w $8a(a6), d0
pub fn i_057c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C12;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057C12: beq.w $57c22
pub fn i_057c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57C22; return 10; }
    8
}
/// $057C16: asr.w #$5, d0
pub fn i_057c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C18;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $057C18: bne.w $57c1e
pub fn i_057c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C1C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57C1E; return 10; }
    8
}
/// $057C1C: moveq #$1, d0
pub fn i_057c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C1E;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $057C1E: sub.w d0, $8a(a6)
pub fn i_057c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C22;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $057C22: move.b $76(a6), d0
pub fn i_057c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C26;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $057C26: movea.l $7a(a6), a0
pub fn i_057c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C2A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057C2A: jsr $8f82a.l
pub fn i_057c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57C30);
    cpu.pc = 0x8F82A;
    16
}
/// $057C30: bcs.w $57c3e
pub fn i_057c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C34;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57C3E; return 10; }
    8
}
/// $057C34: lea.l $57fc6(pc), a1
pub fn i_057c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057C38: move.l a1, (a6)
pub fn i_057c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057C3A: bra.w $57c5e
pub fn i_057c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x57C5E;
    10
}
/// $057C3E: add.w $88(a6), d0
pub fn i_057c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C42;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057C42: add.w $8a(a6), d1
pub fn i_057c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C46;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057C46: move.w d0, $22(a6)
pub fn i_057c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C4A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057C4A: move.w d1, $24(a6)
pub fn i_057c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C4E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057C4E: move.w $38(a0), d0
pub fn i_057c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C52;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057C52: andi.w #$ffe3, d0
pub fn i_057c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C56;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057C56: ori.w #$18, d0
pub fn i_057c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C5A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057C5A: move.w d0, $38(a6)
pub fn i_057c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C5E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057C5E: jsr $28d70.l
pub fn i_057c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57C64);
    cpu.pc = 0x28D70;
    16
}
/// $057C64: move.w $88(a6), d0
pub fn i_057c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C68;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057C68: or.w $8a(a6), d0
pub fn i_057c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C6C;
    cpu.ir = 0x806E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $057C6C: bne.w $57ca4
pub fn i_057c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57CA4; return 10; }
    8
}
/// $057C70: lea.l $57fc6(pc), a1
pub fn i_057c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057C74: move.l a1, (a6)
pub fn i_057c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057C76: move.b $76(a6), d0
pub fn i_057c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C7A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $057C7A: cmpi.b #$0, d0
pub fn i_057c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C7E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $057C7E: bne.w $57c88
pub fn i_057c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C82;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57C88; return 10; }
    8
}
/// $057C82: lea.l $57d04(pc), a1
pub fn i_057c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57D04;
    16
}
/// $057C86: move.l a1, (a6)
pub fn i_057c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057C88: cmpi.b #$1, d0
pub fn i_057c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C8C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $057C8C: bne.w $57c96
pub fn i_057c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C90;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57C96; return 10; }
    8
}
/// $057C90: lea.l $57d6e(pc), a1
pub fn i_057c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57D6E;
    16
}
/// $057C94: move.l a1, (a6)
pub fn i_057c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057C96: cmpi.b #$2, d0
pub fn i_057c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C9A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $057C9A: bne.w $57ca4
pub fn i_057c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57C9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57CA4; return 10; }
    8
}
/// $057C9E: lea.l $57e0a(pc), a1
pub fn i_057c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57E0A;
    16
}
/// $057CA2: move.l a1, (a6)
pub fn i_057ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057CA4: jmp $57ae4(pc)
pub fn i_057ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CA8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabHoldFlag_057ca8  ($057CA8, 16 B, enemies/soldiers/helpers_056acc.s)
/// $057CA8: tst.b $106ed3.l
pub fn i_057ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CAE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $057CAE: bne.w $57cc0
pub fn i_057cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57CC0; return 10; }
    8
}
/// $057CB2: bset.b #$3, $13(a6)
pub fn i_057cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CB8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_057cb8  ($057CB8, 8 B, kernel/task_handlers.c)
/// $057CB8: lea.l $57f4e(pc), a1
pub fn i_057cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57F4E;
    16
}
/// $057CBC: move.l a1, (a6)
pub fn i_057cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057CBE: rts
pub fn i_057cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabFollowPlayer_057cc0  ($057CC0, 60 B, enemies/soldiers/helpers_056acc.s)
/// $057CC0: move.b $76(a6), d0
pub fn i_057cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CC4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $057CC4: movea.l $7a(a6), a0
pub fn i_057cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CC8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057CC8: jsr $8f884.l
pub fn i_057cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57CCE);
    cpu.pc = 0x8F884;
    16
}
/// $057CCE: bcc.w $57cee
pub fn i_057cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57CEE; return 10; }
    8
}
/// $057CD2: move.w d0, $22(a6)
pub fn i_057cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CD6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057CD6: move.w d1, $24(a6)
pub fn i_057cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CDA;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057CDA: move.w $38(a0), d0
pub fn i_057cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CDE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057CDE: andi.w #$ffe3, d0
pub fn i_057cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CE2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057CE2: ori.w #$18, d0
pub fn i_057ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CE6;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $057CE6: move.w d0, $38(a6)
pub fn i_057ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CEA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057CEA: bra.w $57cf4
pub fn i_057cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x57CF4;
    10
}
/// $057CEE: lea.l $57fc6(pc), a1
pub fn i_057cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057CF2: move.l a1, (a6)
pub fn i_057cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057CF4: bsr.w $56f10
pub fn i_057cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CF8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57CF8);
    cpu.pc = 0x56F10;
    18
}
/// $057CF8: bcc.w $57d02
pub fn i_057cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57CFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57D02; return 10; }
    8
}
// ===== SetTaskHandler_057cfc  ($057CFC, 8 B, kernel/task_handlers.c)
/// $057CFC: lea.l $57f4e(pc), a1
pub fn i_057cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57F4E;
    16
}
/// $057D00: move.l a1, (a6)
pub fn i_057d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057D02: rts
pub fn i_057d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabPlayer_057d04  ($057D04, 290 B, enemies/soldiers/states_057d04.s)
/// $057D04: bclr.b #$0, $3a(a6)
pub fn i_057d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D0A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057D0A: movea.l $7a(a6), a0
pub fn i_057d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D0E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057D0E: move.w #$0, d0
pub fn i_057d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D12;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057D12: jsr $8f85c.l
pub fn i_057d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D18);
    cpu.pc = 0x8F85C;
    16
}
/// $057D18: bcc.w $57fc6
pub fn i_057d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FC6; return 10; }
    8
}
/// $057D1C: lea.l $2b6d64.l, a0
pub fn i_057d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6D64;
    20
}
/// $057D22: jsr $28cd4.l
pub fn i_057d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D28);
    cpu.pc = 0x28CD4;
    16
}
/// $057D28: clr.b $20(a6)
pub fn i_057d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D2C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $057D2C: lea.l $57d32(pc), a1
pub fn i_057d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57D32;
    16
}
/// $057D30: move.l a1, (a6)
pub fn i_057d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057D32: bsr.w $57ca8
pub fn i_057d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D36;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57D36);
    cpu.pc = 0x57CA8;
    18
}
/// $057D36: jsr $56f64(pc)
pub fn i_057d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x57D3A);
    cpu.pc = 0x56F64;
    16
}
/// $057D3A: bcc.w $57d44
pub fn i_057d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57D44; return 10; }
    8
}
/// $057D3E: lea.l $57ece(pc), a1
pub fn i_057d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57ECE;
    16
}
/// $057D42: move.l a1, (a6)
pub fn i_057d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057D44: jsr $28d70.l
pub fn i_057d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D4A);
    cpu.pc = 0x28D70;
    16
}
/// $057D4A: bcc.w $57d54
pub fn i_057d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57D54; return 10; }
    8
}
/// $057D4E: lea.l $57fc6(pc), a1
pub fn i_057d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057D52: move.l a1, (a6)
pub fn i_057d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057D54: tst.b $20(a6)
pub fn i_057d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D58;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $057D58: beq.w $57d6a
pub fn i_057d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D5C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57D6A; return 10; }
    8
}
/// $057D5C: movea.l $7a(a6), a0
pub fn i_057d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057D60: move.w #$0, d0
pub fn i_057d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D64;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057D64: jsr $8f8c2.l
pub fn i_057d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D6A);
    cpu.pc = 0x8F8C2;
    16
}
/// $057D6A: jmp $57ae4(pc)
pub fn i_057d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D6E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
/// $057D6E: bclr.b #$0, $3a(a6)
pub fn i_057d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D74;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057D74: movea.l $7a(a6), a0
pub fn i_057d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D78;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057D78: move.w #$1, d0
pub fn i_057d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D7C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057D7C: jsr $8f85c.l
pub fn i_057d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D82);
    cpu.pc = 0x8F85C;
    16
}
/// $057D82: bcc.w $57fc6
pub fn i_057d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FC6; return 10; }
    8
}
/// $057D86: jsr $13600.l
pub fn i_057d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D8C);
    cpu.pc = 0x13600;
    16
}
/// $057D8C: lea.l $2b6aa4.l, a0
pub fn i_057d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D92;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6AA4;
    20
}
/// $057D92: jsr $28cd4.l
pub fn i_057d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57D98);
    cpu.pc = 0x28CD4;
    16
}
/// $057D98: move.l $7a(a6), d0
pub fn i_057d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57D9C;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057D9C: cmpi.l #$100440, d0
pub fn i_057d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DA2;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100440, Size::Long, false);
    8
}
/// $057DA2: bne.w $57dc4
pub fn i_057da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57DC4; return 10; }
    8
}
/// $057DA6: move.w #$20, d1
pub fn i_057da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DAA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057DAA: jsr $236e.l
pub fn i_057daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57DB0);
    cpu.pc = 0x236E;
    16
}
/// $057DB0: lea.l $2b7284.l, a0
pub fn i_057db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DB6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7284;
    20
}
/// $057DB6: move.l a0, $4c(a6)
pub fn i_057db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DBA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057DBA: jsr $283ca.l
pub fn i_057dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57DC0);
    cpu.pc = 0x283CA;
    16
}
/// $057DC0: bra.w $57dde
pub fn i_057dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x57DDE;
    10
}
/// $057DC4: move.w #$14b, d1
pub fn i_057dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DC8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057DC8: jsr $236e.l
pub fn i_057dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57DCE);
    cpu.pc = 0x236E;
    16
}
/// $057DCE: lea.l $2b72d8.l, a0
pub fn i_057dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B72D8;
    20
}
/// $057DD4: move.l a0, $4c(a6)
pub fn i_057dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DD8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057DD8: jsr $283ca.l
pub fn i_057dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57DDE);
    cpu.pc = 0x283CA;
    16
}
/// $057DDE: lea.l $57de4(pc), a1
pub fn i_057dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57DE4;
    16
}
/// $057DE2: move.l a1, (a6)
pub fn i_057de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057DE4: bsr.w $57ca8
pub fn i_057de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DE8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57DE8);
    cpu.pc = 0x57CA8;
    18
}
/// $057DE8: jsr $56f64(pc)
pub fn i_057de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x57DEC);
    cpu.pc = 0x56F64;
    16
}
/// $057DEC: bcc.w $57df6
pub fn i_057dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DF0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57DF6; return 10; }
    8
}
/// $057DF0: lea.l $57ee2(pc), a1
pub fn i_057df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57EE2;
    16
}
/// $057DF4: move.l a1, (a6)
pub fn i_057df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057DF6: jsr $28d70.l
pub fn i_057df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57DFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57DFC);
    cpu.pc = 0x28D70;
    16
}
/// $057DFC: bcc.w $57e06
pub fn i_057dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57E06; return 10; }
    8
}
/// $057E00: lea.l $57fc6(pc), a1
pub fn i_057e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FC6;
    16
}
/// $057E04: move.l a1, (a6)
pub fn i_057e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057E06: jmp $57ae4(pc)
pub fn i_057e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E0A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
/// $057E0A: bclr.b #$0, $3a(a6)
pub fn i_057e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E10;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057E10: movea.l $7a(a6), a0
pub fn i_057e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E14;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057E14: move.w #$2, d0
pub fn i_057e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E18;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057E18: jsr $8f85c.l
pub fn i_057e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57E1E);
    cpu.pc = 0x8F85C;
    16
}
/// $057E1E: bcc.w $57fc6
pub fn i_057e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FC6; return 10; }
    8
}
/// $057E22: clr.b $30(a6)
pub fn i_057e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E26;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== Soldier_GrabStruggle_057e26  ($057E26, 84 B, enemies/soldiers/states_057d04.s)
/// $057E26: lea.l $2b6832.l, a0
pub fn i_057e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6832;
    20
}
/// $057E2C: jsr $28cd4.l
pub fn i_057e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57E32);
    cpu.pc = 0x28CD4;
    16
}
/// $057E32: bclr.b #$3, $13(a6)
pub fn i_057e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E38;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057E38: bclr.b #$0, $13(a6)
pub fn i_057e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E3E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057E3E: lea.l $57e44(pc), a1
pub fn i_057e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57E44;
    16
}
/// $057E42: move.l a1, (a6)
pub fn i_057e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057E44: bsr.w $57ca8
pub fn i_057e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E48;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57E48);
    cpu.pc = 0x57CA8;
    18
}
/// $057E48: jsr $28d70.l
pub fn i_057e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57E4E);
    cpu.pc = 0x28D70;
    16
}
/// $057E4E: jsr $56f64(pc)
pub fn i_057e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x57E52);
    cpu.pc = 0x56F64;
    16
}
/// $057E52: bcc.w $57e5c
pub fn i_057e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57E5C; return 10; }
    8
}
/// $057E56: lea.l $57ef6(pc), a1
pub fn i_057e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57EF6;
    16
}
/// $057E5A: move.l a1, (a6)
pub fn i_057e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057E5C: jsr $2870a.l
pub fn i_057e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57E62);
    cpu.pc = 0x2870A;
    16
}
/// $057E62: bcc.w $57e76
pub fn i_057e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57E76; return 10; }
    8
}
/// $057E66: cmpi.b #$2, $58(a6)
pub fn i_057e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E6C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $057E6C: bne.w $57e76
pub fn i_057e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57E76; return 10; }
    8
}
/// $057E70: lea.l $57e7a(pc), a1
pub fn i_057e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57E7A;
    16
}
/// $057E74: move.l a1, (a6)
pub fn i_057e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057E76: jmp $57ae4(pc)
pub fn i_057e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E7A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabStruggleNext_057e7a  ($057E7A, 84 B, enemies/soldiers/states_057d04.s)
/// $057E7A: move.b $30(a6), d0
pub fn i_057e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E7E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $057E7E: addi.b #$1, d0
pub fn i_057e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E82;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $057E82: move.b d0, $30(a6)
pub fn i_057e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E86;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057E86: cmpi.b #$3, d0
pub fn i_057e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E8A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $057E8A: bcs.w $57e98
pub fn i_057e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E8E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57E98; return 10; }
    8
}
/// $057E8E: bset.b #$3, $13(a6)
pub fn i_057e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E94;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057E94: bra.w $57f4e
pub fn i_057e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E98;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F4E;
    10
}
/// $057E98: bclr.b #$3, $13(a6)
pub fn i_057e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E9E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057E9E: bclr.b #$0, $13(a6)
pub fn i_057e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EA4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057EA4: lea.l $2b6892.l, a0
pub fn i_057ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6892;
    20
}
/// $057EAA: jsr $28cd4.l
pub fn i_057eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57EB0);
    cpu.pc = 0x28CD4;
    16
}
/// $057EB0: lea.l $57eb6(pc), a1
pub fn i_057eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57EB6;
    16
}
/// $057EB4: move.l a1, (a6)
pub fn i_057eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057EB6: bsr.w $57ca8
pub fn i_057eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EBA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57EBA);
    cpu.pc = 0x57CA8;
    18
}
/// $057EBA: jsr $28d70.l
pub fn i_057eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57EC0);
    cpu.pc = 0x28D70;
    16
}
/// $057EC0: bcc.w $57eca
pub fn i_057ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57ECA; return 10; }
    8
}
/// $057EC4: lea.l $57e26(pc), a1
pub fn i_057ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57E26;
    16
}
/// $057EC8: move.l a1, (a6)
pub fn i_057ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ECA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057ECA: jmp $57ae4(pc)
pub fn i_057eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ECE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabBreakA_057ece  ($057ECE, 20 B, enemies/soldiers/states_057d04.s)
/// $057ECE: move.l #$57d1c, $5c(a6)
pub fn i_057ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57ED6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x57D1C;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057ED6: move.l #$2b6c32, $92(a6)
pub fn i_057ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EDE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6C32;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057EDE: bra.w $57f06
pub fn i_057ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F06;
    10
}
// ===== Soldier_GrabBreakB_057ee2  ($057EE2, 20 B, enemies/soldiers/states_057d04.s)
/// $057EE2: move.l #$57d86, $5c(a6)
pub fn i_057ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EEA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x57D86;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057EEA: move.l #$2b6c32, $92(a6)
pub fn i_057eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EF2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6C32;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057EF2: bra.w $57f06
pub fn i_057ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F06;
    10
}
// ===== Soldier_GrabBreakC_057ef6  ($057EF6, 16 B, enemies/soldiers/states_057d04.s)
/// $057EF6: move.l #$57e26, $5c(a6)
pub fn i_057ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57EFE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x57E26;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057EFE: move.l #$2b6bc0, $92(a6)
pub fn i_057efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F06;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2B6BC0;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Soldier_GrabBreak_057f06  ($057F06, 72 B, enemies/soldiers/states_057d04.s)
/// $057F06: movea.l $92(a6), a0
pub fn i_057f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F0A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057F0A: jsr $28cd4.l
pub fn i_057f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F10);
    cpu.pc = 0x28CD4;
    16
}
/// $057F10: lea.l $57f16(pc), a1
pub fn i_057f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57F16;
    16
}
/// $057F14: move.l a1, (a6)
pub fn i_057f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057F16: bsr.w $57ca8
pub fn i_057f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F1A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57F1A);
    cpu.pc = 0x57CA8;
    18
}
/// $057F1A: jsr $28d70.l
pub fn i_057f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F20);
    cpu.pc = 0x28D70;
    16
}
/// $057F20: bcc.w $57f3a
pub fn i_057f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57F3A; return 10; }
    8
}
/// $057F24: bsr.w $56f64
pub fn i_057f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F28;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57F28);
    cpu.pc = 0x56F64;
    18
}
/// $057F28: bcc.w $57f36
pub fn i_057f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57F36; return 10; }
    8
}
/// $057F2C: lea.l $57f06(pc), a1
pub fn i_057f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57F06;
    16
}
/// $057F30: move.l a1, (a6)
pub fn i_057f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057F32: bra.w $57f3a
pub fn i_057f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F36;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F3A;
    10
}
/// $057F36: move.l $5c(a6), (a6)
pub fn i_057f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F3A;
    cpu.ir = 0x2CAE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $057F3A: jsr $2870a.l
pub fn i_057f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F40);
    cpu.pc = 0x2870A;
    16
}
/// $057F40: bcc.w $57f4a
pub fn i_057f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57F4A; return 10; }
    8
}
/// $057F44: lea.l $57e7a(pc), a1
pub fn i_057f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57E7A;
    16
}
/// $057F48: move.l a1, (a6)
pub fn i_057f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057F4A: jmp $57ae4(pc)
pub fn i_057f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F4E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x57AE4;
    8
}
// ===== Soldier_GrabThrownA_057f4e  ($057F4E, 120 B, enemies/soldiers/states_057d04.s)
/// $057F4E: lea.l $2b6ca4.l, a0
pub fn i_057f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6CA4;
    20
}
/// $057F54: jsr $28cd4.l
pub fn i_057f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F5A);
    cpu.pc = 0x28CD4;
    16
}
/// $057F5A: movea.l $7a(a6), a0
pub fn i_057f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F5E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057F5E: move.w $22(a0), d0
pub fn i_057f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F62;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057F62: sub.w $22(a6), d0
pub fn i_057f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F66;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057F66: bpl.w $57f74
pub fn i_057f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F6A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57F74; return 10; }
    8
}
/// $057F6A: bclr.b #$0, $3a(a6)
pub fn i_057f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057F70: bra.w $57f7a
pub fn i_057f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F74;
    cpu.ir = 0x6000;
    cpu.pc = 0x57F7A;
    10
}
/// $057F74: bset.b #$0, $3a(a6)
pub fn i_057f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F7A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057F7A: move.w #$333, d0
pub fn i_057f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F7E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x333;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $057F7E: jsr $5dca4.l
pub fn i_057f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57F84);
    cpu.pc = 0x5DCA4;
    16
}
/// $057F84: move.w d0, $28(a6)
pub fn i_057f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F88;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057F88: move.w #$32a, $2a(a6)
pub fn i_057f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057F8E: move.w #$ffaf, $2e(a6)
pub fn i_057f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057F94: move.w #$0, $2c(a6)
pub fn i_057f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F9A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $057F9A: lea.l $57fa0(pc), a1
pub fn i_057f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57F9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57FA0;
    16
}
/// $057F9E: move.l a1, (a6)
pub fn i_057f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057FA0: jsr $2831e.l
pub fn i_057fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57FA6);
    cpu.pc = 0x2831E;
    16
}
/// $057FA6: scs.b $78(a6)
pub fn i_057fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FAA;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $057FAA: bcc.w $57fb4
pub fn i_057faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x57FB4; return 10; }
    8
}
/// $057FAE: lea.l $588ae(pc), a1
pub fn i_057fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $057FB2: move.l a1, (a6)
pub fn i_057fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057FB4: jsr $28d70.l
pub fn i_057fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57FBA);
    cpu.pc = 0x28D70;
    16
}
/// $057FBA: jsr $49fd0.l
pub fn i_057fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57FC0);
    cpu.pc = 0x49FD0;
    16
}
/// $057FC0: bsr.w $56e1e
pub fn i_057fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FC4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x57FC4);
    cpu.pc = 0x56E1E;
    18
}
/// $057FC4: rts
pub fn i_057fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabThrownB_057fc6  ($057FC6, 134 B, enemies/soldiers/states_057d04.s)
/// $057FC6: lea.l $ffff.w, a0
pub fn i_057fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FCA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $057FCA: move.l a0, $4c(a6)
pub fn i_057fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FCE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $057FCE: jsr $283ca.l
pub fn i_057fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57FD4);
    cpu.pc = 0x283CA;
    16
}
/// $057FD4: movea.l $7a(a6), a0
pub fn i_057fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FD8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $057FD8: move.w $22(a0), d0
pub fn i_057fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FDC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057FDC: sub.w $22(a6), d0
pub fn i_057fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FE0;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $057FE0: bpl.w $57fee
pub fn i_057fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FE4;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x57FEE; return 10; }
    8
}
/// $057FE4: bclr.b #$0, $3a(a6)
pub fn i_057fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FEA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057FEA: bra.w $57ff4
pub fn i_057fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x57FF4;
    10
}
/// $057FEE: bset.b #$0, $3a(a6)
pub fn i_057fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FF4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $057FF4: lea.l $2b6d00.l, a0
pub fn i_057ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57FFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B6D00;
    20
}
/// $057FFA: jsr $28cd4.l
pub fn i_057ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58000);
    cpu.pc = 0x28CD4;
    16
}
/// $058000: move.w #$266, d0
pub fn i_058000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58004;
    cpu.ir = 0x303C;
    let v1: u32 = 0x266;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $058004: jsr $5dca4.l
pub fn i_058004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5800A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5800A);
    cpu.pc = 0x5DCA4;
    16
}
/// $05800A: move.w d0, $28(a6)
pub fn i_05800a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5800E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05800E: move.w #$65e, $2a(a6)
pub fn i_05800e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58014;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x65E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058014: move.w #$ff5d, $2e(a6)
pub fn i_058014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5801A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF5D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05801A: move.w #$0, $2c(a6)
pub fn i_05801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58020;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058020: lea.l $58026(pc), a1
pub fn i_058020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58024;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58026;
    16
}
/// $058024: move.l a1, (a6)
pub fn i_058024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58026;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058026: jsr $2831e.l
pub fn i_058026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5802C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5802C);
    cpu.pc = 0x2831E;
    16
}
/// $05802C: scs.b $78(a6)
pub fn i_05802c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58030;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058030: bcc.w $5803a
pub fn i_058030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58034;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5803A; return 10; }
    8
}
/// $058034: lea.l $58464(pc), a1
pub fn i_058034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58038;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58464;
    16
}
/// $058038: move.l a1, (a6)
pub fn i_058038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5803A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05803A: jsr $28d70.l
pub fn i_05803a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58040);
    cpu.pc = 0x28D70;
    16
}
/// $058040: jsr $49fd0.l
pub fn i_058040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58046;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58046);
    cpu.pc = 0x49FD0;
    16
}
/// $058046: bsr.w $56e1e
pub fn i_058046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5804A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5804A);
    cpu.pc = 0x56E1E;
    18
}
/// $05804A: rts
pub fn i_05804a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5804C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_RunToward_05804c  ($05804C, 42 B, enemies/soldiers/states_057d04.s)
/// $05804C: btst.b #$4, $74(a6)
pub fn i_05804c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58052;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058052: bne.w $58aa8
pub fn i_058052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58056;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58AA8; return 10; }
    8
}
/// $058056: lea.l $29b7c8.l, a0
pub fn i_058056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5805C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B7C8;
    20
}
/// $05805C: jsr $28cd4.l
pub fn i_05805c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58062;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58062);
    cpu.pc = 0x28CD4;
    16
}
/// $058062: move.w #$100, $36(a6)
pub fn i_058062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58068;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058068: bsr.w $56f8a
pub fn i_058068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5806C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5806C);
    cpu.pc = 0x56F8A;
    18
}
/// $05806C: lea.l $5809e(pc), a1
pub fn i_05806c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58070;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5809E;
    16
}
/// $058070: move.l a1, (a6)
pub fn i_058070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58072;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058072: bra.w $5809e
pub fn i_058072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58076;
    cpu.ir = 0x6000;
    cpu.pc = 0x5809E;
    10
}
// ===== Soldier_RunByTable_058076  ($058076, 40 B, enemies/soldiers/states_057d04.s)
/// $058076: lea.l $2b5d14.l, a0
pub fn i_058076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5807C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5D14;
    20
}
/// $05807C: jsr $28cd4.l
pub fn i_05807c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58082);
    cpu.pc = 0x28CD4;
    16
}
/// $058082: lea.l $2b756e.l, a0
pub fn i_058082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58088;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B756E;
    20
}
/// $058088: jsr $799de.l
pub fn i_058088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5808E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5808E);
    cpu.pc = 0x799DE;
    16
}
/// $05808E: neg.w d0
pub fn i_05808e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58090;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058090: move.w d0, $36(a6)
pub fn i_058090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58094;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058094: bsr.w $56f8a
pub fn i_058094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58098;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58098);
    cpu.pc = 0x56F8A;
    18
}
/// $058098: lea.l $5809e(pc), a1
pub fn i_058098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5809C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5809E;
    16
}
/// $05809C: move.l a1, (a6)
pub fn i_05809c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5809E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Run_Loop_05809e  ($05809E, 166 B, enemies/soldiers/states_057d04.s)
/// $05809E: bsr.w $56acc
pub fn i_05809e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580A2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x580A2);
    cpu.pc = 0x56ACC;
    18
}
/// $0580A2: bsr.w $56b92
pub fn i_0580a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x580A6);
    cpu.pc = 0x56B92;
    18
}
/// $0580A6: move.w $28(a6), d0
pub fn i_0580a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580AA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0580AA: asr.w #$4, d0
pub fn i_0580aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580AC;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0580AC: sub.w d0, $28(a6)
pub fn i_0580ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580B0;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0580B0: jsr $28d70.l
pub fn i_0580b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x580B6);
    cpu.pc = 0x28D70;
    16
}
/// $0580B6: bcc.w $580c0
pub fn i_0580b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x580C0; return 10; }
    8
}
/// $0580BA: lea.l $58144(pc), a1
pub fn i_0580ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $0580BE: move.l a1, (a6)
pub fn i_0580be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0580C0: bsr.w $56fec
pub fn i_0580c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580C4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x580C4);
    cpu.pc = 0x56FEC;
    18
}
/// $0580C4: jsr $5e9b6.l
pub fn i_0580c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x580CA);
    cpu.pc = 0x5E9B6;
    16
}
/// $0580CA: move.w d0, -(a7)
pub fn i_0580ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580CC;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0580CC: andi.w #$ff, d0
pub fn i_0580cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580D0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0580D0: cmp.w $8a(a6), d0
pub fn i_0580d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580D4;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0580D4: bcc.w $580de
pub fn i_0580d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x580DE; return 10; }
    8
}
/// $0580D8: move.b (a7), d0
pub fn i_0580d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580DA;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0580DA: cmp.w $8a(a6), d0
pub fn i_0580da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580DE;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0580DE: addq.w #$2, a7
pub fn i_0580de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580E0;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0580E0: bcc.w $580ea
pub fn i_0580e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x580EA; return 10; }
    8
}
/// $0580E4: lea.l $58658(pc), a1
pub fn i_0580e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58658;
    16
}
/// $0580E8: move.l a1, (a6)
pub fn i_0580e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0580EA: jsr $56e36(pc)
pub fn i_0580ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x580EE);
    cpu.pc = 0x56E36;
    16
}
/// $0580EE: bcc.w $580f8
pub fn i_0580ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x580F8; return 10; }
    8
}
/// $0580F2: lea.l $59062(pc), a1
pub fn i_0580f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59062;
    16
}
/// $0580F6: move.l a1, (a6)
pub fn i_0580f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0580F8: tst.b $78(a6)
pub fn i_0580f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x580FC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0580FC: bne.w $58106
pub fn i_0580fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58100;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58106; return 10; }
    8
}
/// $058100: lea.l $58412(pc), a1
pub fn i_058100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58104;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058104: move.l a1, (a6)
pub fn i_058104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58106;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058106: jsr $5e9b6.l
pub fn i_058106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5810C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5810C);
    cpu.pc = 0x5E9B6;
    16
}
/// $05810C: move.w d0, -(a7)
pub fn i_05810c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5810E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05810E: andi.w #$ff, d0
pub fn i_05810e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58112;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058112: cmp.w $88(a6), d0
pub fn i_058112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58116;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058116: bcc.w $58120
pub fn i_058116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5811A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58120; return 10; }
    8
}
/// $05811A: move.b (a7), d0
pub fn i_05811a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5811C;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05811C: cmp.w $88(a6), d0
pub fn i_05811c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58120;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058120: addq.w #$2, a7
pub fn i_058120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58122;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058122: bcc.w $5812c
pub fn i_058122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58126;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5812C; return 10; }
    8
}
/// $058126: lea.l $588f6(pc), a1
pub fn i_058126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5812A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588F6;
    16
}
/// $05812A: move.l a1, (a6)
pub fn i_05812a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5812C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05812C: jsr $574e8(pc)
pub fn i_05812c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58130;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58130);
    cpu.pc = 0x574E8;
    16
}
/// $058130: bcc.w $5813a
pub fn i_058130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58134;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5813A; return 10; }
    8
}
/// $058134: lea.l $585ae(pc), a1
pub fn i_058134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58138;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585AE;
    16
}
/// $058138: move.l a1, (a6)
pub fn i_058138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5813A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05813A: jsr $49fd0.l
pub fn i_05813a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58140);
    cpu.pc = 0x49FD0;
    16
}
/// $058140: bra.w $56e1e
pub fn i_058140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58144;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_Idle_058144  ($058144, 648 B, enemies/soldiers/states_057d04.s)
/// $058144: tst.w $34(a6)
pub fn i_058144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58148;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $058148: bne.w $5815c
pub fn i_058148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5814C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5815C; return 10; }
    8
}
/// $05814C: lea.l $29b816.l, a0
pub fn i_05814c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58152;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B816;
    20
}
/// $058152: jsr $28cd4.l
pub fn i_058152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58158;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58158);
    cpu.pc = 0x28CD4;
    16
}
/// $058158: bra.w $58188
pub fn i_058158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5815C;
    cpu.ir = 0x6000;
    cpu.pc = 0x58188;
    10
}
/// $05815C: spl.b d0
pub fn i_05815c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5815E;
    cpu.ir = 0x5AC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05815E: btst.b #$0, $3a(a6)
pub fn i_05815e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58164;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058164: spl.b d1
pub fn i_058164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58166;
    cpu.ir = 0x5AC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $058166: eor.w d0, d1
pub fn i_058166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58168;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058168: bne.w $5817c
pub fn i_058168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5816C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5817C; return 10; }
    8
}
/// $05816C: lea.l $2b5f28.l, a0
pub fn i_05816c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58172;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5F28;
    20
}
/// $058172: jsr $28cd4.l
pub fn i_058172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58178;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58178);
    cpu.pc = 0x28CD4;
    16
}
/// $058178: bra.w $58188
pub fn i_058178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5817C;
    cpu.ir = 0x6000;
    cpu.pc = 0x58188;
    10
}
/// $05817C: lea.l $2b5f28.l, a0
pub fn i_05817c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58182;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5F28;
    20
}
/// $058182: jsr $28cd4.l
pub fn i_058182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58188);
    cpu.pc = 0x28CD4;
    16
}
/// $058188: clr.w $28(a6)
pub fn i_058188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5818C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05818C: clr.w $2a(a6)
pub fn i_05818c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58190;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058190: lea.l $58196(pc), a1
pub fn i_058190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58194;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58196;
    16
}
/// $058194: move.l a1, (a6)
pub fn i_058194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58196;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058196: bsr.w $56acc
pub fn i_058196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5819A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5819A);
    cpu.pc = 0x56ACC;
    18
}
/// $05819A: bsr.w $56b92
pub fn i_05819a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5819E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5819E);
    cpu.pc = 0x56B92;
    18
}
/// $05819E: jsr $28d70.l
pub fn i_05819e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x581A4);
    cpu.pc = 0x28D70;
    16
}
/// $0581A4: bsr.w $56fec
pub fn i_0581a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581A8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x581A8);
    cpu.pc = 0x56FEC;
    18
}
/// $0581A8: btst.b #$0, $72(a6)
pub fn i_0581a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581AE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0581AE: beq.w $5828c
pub fn i_0581ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5828C; return 10; }
    8
}
/// $0581B2: move.w $80(a6), d1
pub fn i_0581b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581B6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0581B6: cmp.w $86(a6), d1
pub fn i_0581b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581BA;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0581BA: bcc.w $581c2
pub fn i_0581ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x581C2; return 10; }
    8
}
/// $0581BE: move.w $86(a6), d1
pub fn i_0581be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581C2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0581C2: jsr $5e9b6.l
pub fn i_0581c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x581C8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0581C8: move.w d0, -(a7)
pub fn i_0581c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581CA;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0581CA: andi.w #$ff, d0
pub fn i_0581ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581CE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0581CE: cmp.w d1, d0
pub fn i_0581ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581D0;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0581D0: bcc.w $581d8
pub fn i_0581d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x581D8; return 10; }
    8
}
/// $0581D4: move.b (a7), d0
pub fn i_0581d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581D6;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0581D6: cmp.w d1, d0
pub fn i_0581d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581D8;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0581D8: addq.w #$2, a7
pub fn i_0581d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581DA;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0581DA: bcc.w $581e4
pub fn i_0581da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x581E4; return 10; }
    8
}
/// $0581DE: lea.l $58500(pc), a1
pub fn i_0581de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58500;
    16
}
/// $0581E2: move.l a1, (a6)
pub fn i_0581e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0581E4: mulu.w d1, d1
pub fn i_0581e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581E6;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0581E6: lsr.l #$8, d1
pub fn i_0581e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581E8;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $0581E8: jsr $5e9b6.l
pub fn i_0581e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x581EE);
    cpu.pc = 0x5E9B6;
    16
}
/// $0581EE: move.w d0, -(a7)
pub fn i_0581ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581F0;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0581F0: andi.w #$ff, d0
pub fn i_0581f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581F4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0581F4: cmp.w d1, d0
pub fn i_0581f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581F6;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0581F6: bcc.w $581fe
pub fn i_0581f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x581FE; return 10; }
    8
}
/// $0581FA: move.b (a7), d0
pub fn i_0581fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581FC;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0581FC: cmp.w d1, d0
pub fn i_0581fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x581FE;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0581FE: addq.w #$2, a7
pub fn i_0581fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58200;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058200: bcc.w $5820a
pub fn i_058200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58204;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5820A; return 10; }
    8
}
/// $058204: lea.l $58076(pc), a1
pub fn i_058204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58208;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58076;
    16
}
/// $058208: move.l a1, (a6)
pub fn i_058208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5820A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05820A: mulu.w d1, d1
pub fn i_05820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5820C;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05820C: lsr.l #$8, d1
pub fn i_05820c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5820E;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $05820E: jsr $5e9b6.l
pub fn i_05820e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58214);
    cpu.pc = 0x5E9B6;
    16
}
/// $058214: move.w d0, -(a7)
pub fn i_058214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58216;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058216: andi.w #$ff, d0
pub fn i_058216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5821A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05821A: cmp.w d1, d0
pub fn i_05821a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5821C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05821C: bcc.w $58224
pub fn i_05821c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58220;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58224; return 10; }
    8
}
/// $058220: move.b (a7), d0
pub fn i_058220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58222;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058222: cmp.w d1, d0
pub fn i_058222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58224;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058224: addq.w #$2, a7
pub fn i_058224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58226;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058226: bcc.w $58230
pub fn i_058226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5822A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58230; return 10; }
    8
}
/// $05822A: lea.l $585f6(pc), a1
pub fn i_05822a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5822E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $05822E: move.l a1, (a6)
pub fn i_05822e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58230;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058230: move.w $82(a6), d1
pub fn i_058230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58234;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058234: cmp.w $84(a6), d1
pub fn i_058234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58238;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058238: bcc.w $58240
pub fn i_058238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5823C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58240; return 10; }
    8
}
/// $05823C: move.w $84(a6), d1
pub fn i_05823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58240;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058240: jsr $5e9b6.l
pub fn i_058240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58246);
    cpu.pc = 0x5E9B6;
    16
}
/// $058246: move.w d0, -(a7)
pub fn i_058246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58248;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058248: andi.w #$ff, d0
pub fn i_058248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5824C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05824C: cmp.w d1, d0
pub fn i_05824c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5824E;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05824E: bcc.w $58256
pub fn i_05824e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58252;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58256; return 10; }
    8
}
/// $058252: move.b (a7), d0
pub fn i_058252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58254;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058254: cmp.w d1, d0
pub fn i_058254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58256;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058256: addq.w #$2, a7
pub fn i_058256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58258;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058258: bcc.w $58262
pub fn i_058258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5825C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58262; return 10; }
    8
}
/// $05825C: lea.l $584c4(pc), a1
pub fn i_05825c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58260;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x584C4;
    16
}
/// $058260: move.l a1, (a6)
pub fn i_058260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58262;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058262: mulu.w d1, d1
pub fn i_058262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58264;
    cpu.ir = 0xC2C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $058264: lsr.l #$8, d1
pub fn i_058264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58266;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $058266: jsr $5e9b6.l
pub fn i_058266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5826C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5826C);
    cpu.pc = 0x5E9B6;
    16
}
/// $05826C: move.w d0, -(a7)
pub fn i_05826c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5826E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05826E: andi.w #$ff, d0
pub fn i_05826e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58272;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058272: cmp.w d1, d0
pub fn i_058272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58274;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058274: bcc.w $5827c
pub fn i_058274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58278;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5827C; return 10; }
    8
}
/// $058278: move.b (a7), d0
pub fn i_058278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5827A;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05827A: cmp.w d1, d0
pub fn i_05827a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5827C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05827C: addq.w #$2, a7
pub fn i_05827c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5827E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05827E: bcc.w $58288
pub fn i_05827e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58282;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58288; return 10; }
    8
}
/// $058282: lea.l $57558(pc), a1
pub fn i_058282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58286;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $058286: move.l a1, (a6)
pub fn i_058286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58288;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058288: bra.w $582d8
pub fn i_058288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5828C;
    cpu.ir = 0x6000;
    cpu.pc = 0x582D8;
    10
}
/// $05828C: jsr $5e9b6.l
pub fn i_05828c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58292);
    cpu.pc = 0x5E9B6;
    16
}
/// $058292: move.w d0, -(a7)
pub fn i_058292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58294;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058294: andi.w #$ff, d0
pub fn i_058294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58298;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058298: cmp.w $80(a6), d0
pub fn i_058298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5829C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05829C: bcc.w $582a6
pub fn i_05829c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582A6; return 10; }
    8
}
/// $0582A0: move.b (a7), d0
pub fn i_0582a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582A2;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0582A2: cmp.w $80(a6), d0
pub fn i_0582a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582A6;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582A6: addq.w #$2, a7
pub fn i_0582a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582A8;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0582A8: bcc.w $582b2
pub fn i_0582a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582B2; return 10; }
    8
}
/// $0582AC: lea.l $57558(pc), a1
pub fn i_0582ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $0582B0: move.l a1, (a6)
pub fn i_0582b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0582B2: jsr $5e9b6.l
pub fn i_0582b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x582B8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0582B8: move.w d0, -(a7)
pub fn i_0582b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582BA;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0582BA: andi.w #$ff, d0
pub fn i_0582ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582BE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0582BE: cmp.w $82(a6), d0
pub fn i_0582be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582C2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582C2: bcc.w $582cc
pub fn i_0582c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582CC; return 10; }
    8
}
/// $0582C6: move.b (a7), d0
pub fn i_0582c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582C8;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0582C8: cmp.w $82(a6), d0
pub fn i_0582c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582CC;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582CC: addq.w #$2, a7
pub fn i_0582cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582CE;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0582CE: bcc.w $582d8
pub fn i_0582ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582D8; return 10; }
    8
}
/// $0582D2: lea.l $583cc(pc), a1
pub fn i_0582d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x583CC;
    16
}
/// $0582D6: move.l a1, (a6)
pub fn i_0582d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0582D8: jsr $5e9b6.l
pub fn i_0582d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x582DE);
    cpu.pc = 0x5E9B6;
    16
}
/// $0582DE: move.w d0, -(a7)
pub fn i_0582de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582E0;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0582E0: andi.w #$ff, d0
pub fn i_0582e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582E4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0582E4: cmp.w $86(a6), d0
pub fn i_0582e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582E8;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582E8: bcc.w $582f2
pub fn i_0582e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582F2; return 10; }
    8
}
/// $0582EC: move.b (a7), d0
pub fn i_0582ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582EE;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0582EE: cmp.w $86(a6), d0
pub fn i_0582ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582F2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0582F2: addq.w #$2, a7
pub fn i_0582f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582F4;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0582F4: bcc.w $582fe
pub fn i_0582f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x582FE; return 10; }
    8
}
/// $0582F8: lea.l $585f6(pc), a1
pub fn i_0582f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585F6;
    16
}
/// $0582FC: move.l a1, (a6)
pub fn i_0582fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x582FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0582FE: jsr $5e9b6.l
pub fn i_0582fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58304);
    cpu.pc = 0x5E9B6;
    16
}
/// $058304: move.w d0, -(a7)
pub fn i_058304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58306;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058306: andi.w #$ff, d0
pub fn i_058306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5830A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05830A: cmp.w $84(a6), d0
pub fn i_05830a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5830E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05830E: bcc.w $58318
pub fn i_05830e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58312;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58318; return 10; }
    8
}
/// $058312: move.b (a7), d0
pub fn i_058312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58314;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058314: cmp.w $84(a6), d0
pub fn i_058314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58318;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058318: addq.w #$2, a7
pub fn i_058318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5831A;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05831A: bcc.w $58324
pub fn i_05831a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5831E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58324; return 10; }
    8
}
/// $05831E: lea.l $57558(pc), a1
pub fn i_05831e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58322;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $058322: move.l a1, (a6)
pub fn i_058322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58324;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058324: move.w $34(a6), d0
pub fn i_058324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58328;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058328: bne.w $58352
pub fn i_058328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5832C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58352; return 10; }
    8
}
/// $05832C: jsr $5e9b6.l
pub fn i_05832c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58332);
    cpu.pc = 0x5E9B6;
    16
}
/// $058332: move.w d0, -(a7)
pub fn i_058332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58334;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058334: andi.w #$ff, d0
pub fn i_058334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58338;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058338: cmp.w $8a(a6), d0
pub fn i_058338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5833C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05833C: bcc.w $58346
pub fn i_05833c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58340;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58346; return 10; }
    8
}
/// $058340: move.b (a7), d0
pub fn i_058340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58342;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058342: cmp.w $8a(a6), d0
pub fn i_058342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58346;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058346: addq.w #$2, a7
pub fn i_058346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58348;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058348: bcc.w $58352
pub fn i_058348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5834C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58352; return 10; }
    8
}
/// $05834C: lea.l $58658(pc), a1
pub fn i_05834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58350;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58658;
    16
}
/// $058350: move.l a1, (a6)
pub fn i_058350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58352;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058352: jsr $56e36(pc)
pub fn i_058352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58356;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58356);
    cpu.pc = 0x56E36;
    16
}
/// $058356: bcc.w $58360
pub fn i_058356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5835A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58360; return 10; }
    8
}
/// $05835A: lea.l $59062(pc), a1
pub fn i_05835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5835E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59062;
    16
}
/// $05835E: move.l a1, (a6)
pub fn i_05835e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58360;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058360: jsr $56fa0(pc)
pub fn i_058360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58364;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x58364);
    cpu.pc = 0x56FA0;
    16
}
/// $058364: bcc.w $5836e
pub fn i_058364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58368;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5836E; return 10; }
    8
}
/// $058368: lea.l $58968(pc), a1
pub fn i_058368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5836C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58968;
    16
}
/// $05836C: move.l a1, (a6)
pub fn i_05836c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5836E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05836E: btst.b #$0, $73(a6)
pub fn i_05836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58374;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $058374: beq.w $5837e
pub fn i_058374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58378;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5837E; return 10; }
    8
}
/// $058378: lea.l $58e08(pc), a1
pub fn i_058378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5837C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58E08;
    16
}
/// $05837C: move.l a1, (a6)
pub fn i_05837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5837E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05837E: jsr $5e9b6.l
pub fn i_05837e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58384);
    cpu.pc = 0x5E9B6;
    16
}
/// $058384: move.w d0, -(a7)
pub fn i_058384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58386;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058386: andi.w #$ff, d0
pub fn i_058386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5838A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05838A: cmp.w $88(a6), d0
pub fn i_05838a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5838E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05838E: bcc.w $58398
pub fn i_05838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58392;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58398; return 10; }
    8
}
/// $058392: move.b (a7), d0
pub fn i_058392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58394;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058394: cmp.w $88(a6), d0
pub fn i_058394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58398;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058398: addq.w #$2, a7
pub fn i_058398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5839A;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05839A: bcc.w $583a4
pub fn i_05839a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5839E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x583A4; return 10; }
    8
}
/// $05839E: lea.l $588f6(pc), a1
pub fn i_05839e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588F6;
    16
}
/// $0583A2: move.l a1, (a6)
pub fn i_0583a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0583A4: jsr $574e8(pc)
pub fn i_0583a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x583A8);
    cpu.pc = 0x574E8;
    16
}
/// $0583A8: bcc.w $583b2
pub fn i_0583a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x583B2; return 10; }
    8
}
/// $0583AC: lea.l $585ae(pc), a1
pub fn i_0583ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585AE;
    16
}
/// $0583B0: move.l a1, (a6)
pub fn i_0583b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0583B2: tst.b $78(a6)
pub fn i_0583b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583B6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0583B6: bne.w $583c0
pub fn i_0583b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x583C0; return 10; }
    8
}
/// $0583BA: lea.l $58412(pc), a1
pub fn i_0583ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0583BE: move.l a1, (a6)
pub fn i_0583be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0583C0: jsr $49fd0.l
pub fn i_0583c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583C6);
    cpu.pc = 0x49FD0;
    16
}
/// $0583C6: bsr.w $56e1e
pub fn i_0583c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583CA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x583CA);
    cpu.pc = 0x56E1E;
    18
}
/// $0583CA: rts
pub fn i_0583ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_IdleFidget_0583cc  ($0583CC, 70 B, enemies/soldiers/states_057d04.s)
/// $0583CC: jsr $5e9b6.l
pub fn i_0583cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583D2);
    cpu.pc = 0x5E9B6;
    16
}
/// $0583D2: andi.b #$f, d0
pub fn i_0583d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583D6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0583D6: bne.w $585f6
pub fn i_0583d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x585F6; return 10; }
    8
}
/// $0583DA: lea.l $29bf34.l, a0
pub fn i_0583da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BF34;
    20
}
/// $0583E0: jsr $28cd4.l
pub fn i_0583e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0583E6: lea.l $583ec(pc), a1
pub fn i_0583e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x583EC;
    16
}
/// $0583EA: move.l a1, (a6)
pub fn i_0583ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0583EC: jsr $2783a.l
pub fn i_0583ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583F2);
    cpu.pc = 0x2783A;
    16
}
/// $0583F2: bsr.w $56b92
pub fn i_0583f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583F6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x583F6);
    cpu.pc = 0x56B92;
    18
}
/// $0583F6: jsr $28d70.l
pub fn i_0583f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x583FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x583FC);
    cpu.pc = 0x28D70;
    16
}
/// $0583FC: bcc.w $58406
pub fn i_0583fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58400;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58406; return 10; }
    8
}
/// $058400: lea.l $58144(pc), a1
pub fn i_058400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58404;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $058404: move.l a1, (a6)
pub fn i_058404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58406;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058406: jsr $49fd0.l
pub fn i_058406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5840C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5840C);
    cpu.pc = 0x49FD0;
    16
}
/// $05840C: bsr.w $56e1e
pub fn i_05840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58410;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58410);
    cpu.pc = 0x56E1E;
    18
}
/// $058410: rts
pub fn i_058410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58412;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Hurt_058412  ($058412, 18 B, enemies/soldiers/states_057d04.s)
/// $058412: lea.l $2b5b92.l, a0
pub fn i_058412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58418;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5B92;
    20
}
/// $058418: jsr $28cd4.l
pub fn i_058418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5841E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5841E);
    cpu.pc = 0x28CD4;
    16
}
/// $05841E: lea.l $58424(pc), a1
pub fn i_05841e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58422;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58424;
    16
}
/// $058422: move.l a1, (a6)
pub fn i_058422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58424;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Hurt_Loop_058424  ($058424, 64 B, enemies/soldiers/states_057d04.s)
/// $058424: move.w $28(a6), d0
pub fn i_058424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58428;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058428: asr.w #$6, d0
pub fn i_058428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5842A;
    cpu.ir = 0xEC40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05842A: sub.w d0, $28(a6)
pub fn i_05842a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5842E;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05842E: jsr $28364.l
pub fn i_05842e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58434);
    cpu.pc = 0x28364;
    16
}
/// $058434: scs.b $78(a6)
pub fn i_058434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58438;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $058438: bcc.w $58442
pub fn i_058438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5843C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58442; return 10; }
    8
}
/// $05843C: lea.l $58464(pc), a1
pub fn i_05843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58440;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58464;
    16
}
/// $058440: move.l a1, (a6)
pub fn i_058440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58442;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058442: jsr $28d70.l
pub fn i_058442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58448);
    cpu.pc = 0x28D70;
    16
}
/// $058448: jsr $49fd0.l
pub fn i_058448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5844E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5844E);
    cpu.pc = 0x49FD0;
    16
}
/// $05844E: bsr.w $56e1e
pub fn i_05844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58452;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58452);
    cpu.pc = 0x56E1E;
    18
}
/// $058452: rts
pub fn i_058452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $058454: lea.l $2b7030.l, a0
pub fn i_058454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5845A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7030;
    20
}
/// $05845A: jsr $28cd4.l
pub fn i_05845a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58460);
    cpu.pc = 0x28CD4;
    16
}
/// $058460: bra.w $5848a
pub fn i_058460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58464;
    cpu.ir = 0x6000;
    cpu.pc = 0x5848A;
    10
}
// ===== Soldier_Land_058464  ($058464, 96 B, enemies/soldiers/states_057d04.s)
/// $058464: move.w $28(a6), d0
pub fn i_058464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58468;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058468: bpl.w $5846e
pub fn i_058468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5846C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5846E; return 10; }
    8
}
/// $05846C: neg.w d0
pub fn i_05846c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5846E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05846E: move.w $2a(a6), d1
pub fn i_05846e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58472;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058472: bpl.w $58478
pub fn i_058472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58476;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x58478; return 10; }
    8
}
/// $058476: neg.w d1
pub fn i_058476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58478;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058478: cmp.w d0, d1
pub fn i_058478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5847A;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05847A: bcs.w $5804c
pub fn i_05847a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5847E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5804C; return 10; }
    8
}
/// $05847E: lea.l $2b5c22.l, a0
pub fn i_05847e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58484;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C22;
    20
}
/// $058484: jsr $28cd4.l
pub fn i_058484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5848A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5848A);
    cpu.pc = 0x28CD4;
    16
}
/// $05848A: clr.w $28(a6)
pub fn i_05848a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5848E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05848E: lea.l $58494(pc), a1
pub fn i_05848e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58492;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58494;
    16
}
/// $058492: move.l a1, (a6)
pub fn i_058492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58494;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058494: bsr.w $56acc
pub fn i_058494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58498;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58498);
    cpu.pc = 0x56ACC;
    18
}
/// $058498: jsr $28d70.l
pub fn i_058498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5849E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5849E);
    cpu.pc = 0x28D70;
    16
}
/// $05849E: bcc.w $584ba
pub fn i_05849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x584BA; return 10; }
    8
}
/// $0584A2: tst.b $78(a6)
pub fn i_0584a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584A6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0584A6: beq.w $584b4
pub fn i_0584a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x584B4; return 10; }
    8
}
/// $0584AA: lea.l $57558(pc), a1
pub fn i_0584aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $0584AE: move.l a1, (a6)
pub fn i_0584ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0584B0: bra.w $584ba
pub fn i_0584b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x584BA;
    10
}
/// $0584B4: lea.l $58412(pc), a1
pub fn i_0584b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0584B8: move.l a1, (a6)
pub fn i_0584b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0584BA: jsr $49fd0.l
pub fn i_0584ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x584C0);
    cpu.pc = 0x49FD0;
    16
}
/// $0584C0: bra.w $56e1e
pub fn i_0584c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_StepRight_0584c4  ($0584C4, 60 B, enemies/soldiers/states_057d04.s)
/// $0584C4: tst.w $34(a6)
pub fn i_0584c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584C8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0584C8: bne.w $584dc
pub fn i_0584c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x584DC; return 10; }
    8
}
/// $0584CC: lea.l $2b5c56.l, a0
pub fn i_0584cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C56;
    20
}
/// $0584D2: jsr $28cd4.l
pub fn i_0584d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x584D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0584D8: bra.w $584e8
pub fn i_0584d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x584E8;
    10
}
/// $0584DC: lea.l $2b5f56.l, a0
pub fn i_0584dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5F56;
    20
}
/// $0584E2: jsr $28cd4.l
pub fn i_0584e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x584E8);
    cpu.pc = 0x28CD4;
    16
}
/// $0584E8: move.w #$80, $36(a6)
pub fn i_0584e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0584EE: bsr.w $56f8a
pub fn i_0584ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584F2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x584F2);
    cpu.pc = 0x56F8A;
    18
}
/// $0584F2: clr.w $2a(a6)
pub fn i_0584f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584F6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0584F6: lea.l $58538(pc), a1
pub fn i_0584f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58538;
    16
}
/// $0584FA: move.l a1, (a6)
pub fn i_0584fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0584FC: bra.w $58538
pub fn i_0584fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58500;
    cpu.ir = 0x6000;
    cpu.pc = 0x58538;
    10
}
// ===== Soldier_StepLeft_058500  ($058500, 56 B, enemies/soldiers/states_057d04.s)
/// $058500: tst.w $34(a6)
pub fn i_058500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58504;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $058504: bne.w $58518
pub fn i_058504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58508;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58518; return 10; }
    8
}
/// $058508: lea.l $2b5ca8.l, a0
pub fn i_058508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5850E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5CA8;
    20
}
/// $05850E: jsr $28cd4.l
pub fn i_05850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58514);
    cpu.pc = 0x28CD4;
    16
}
/// $058514: bra.w $58524
pub fn i_058514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58518;
    cpu.ir = 0x6000;
    cpu.pc = 0x58524;
    10
}
/// $058518: lea.l $2b5fd0.l, a0
pub fn i_058518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5851E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5FD0;
    20
}
/// $05851E: jsr $28cd4.l
pub fn i_05851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58524);
    cpu.pc = 0x28CD4;
    16
}
/// $058524: move.w #$ff80, $36(a6)
pub fn i_058524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5852A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05852A: bsr.w $56f8a
pub fn i_05852a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5852E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5852E);
    cpu.pc = 0x56F8A;
    18
}
/// $05852E: clr.w $2a(a6)
pub fn i_05852e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58532;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $058532: lea.l $58538(pc), a1
pub fn i_058532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58536;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58538;
    16
}
/// $058536: move.l a1, (a6)
pub fn i_058536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58538;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Step_Loop_058538  ($058538, 118 B, enemies/soldiers/states_057d04.s)
/// $058538: bsr.w $56acc
pub fn i_058538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5853C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5853C);
    cpu.pc = 0x56ACC;
    18
}
/// $05853C: bsr.w $56b92
pub fn i_05853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58540;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58540);
    cpu.pc = 0x56B92;
    18
}
/// $058540: jsr $28d70.l
pub fn i_058540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58546;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58546);
    cpu.pc = 0x28D70;
    16
}
/// $058546: bcc.w $58550
pub fn i_058546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5854A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58550; return 10; }
    8
}
/// $05854A: lea.l $58144(pc), a1
pub fn i_05854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5854E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $05854E: move.l a1, (a6)
pub fn i_05854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58550;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058550: bsr.w $56fec
pub fn i_058550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58554;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58554);
    cpu.pc = 0x56FEC;
    18
}
/// $058554: jsr $5e9b6.l
pub fn i_058554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5855A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5855A);
    cpu.pc = 0x5E9B6;
    16
}
/// $05855A: move.w d0, -(a7)
pub fn i_05855a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5855C;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05855C: andi.w #$ff, d0
pub fn i_05855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58560;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058560: cmp.w $88(a6), d0
pub fn i_058560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58564;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058564: bcc.w $5856e
pub fn i_058564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58568;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5856E; return 10; }
    8
}
/// $058568: move.b (a7), d0
pub fn i_058568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5856A;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05856A: cmp.w $88(a6), d0
pub fn i_05856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5856E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05856E: addq.w #$2, a7
pub fn i_05856e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58570;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058570: bcc.w $5857a
pub fn i_058570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58574;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5857A; return 10; }
    8
}
/// $058574: lea.l $588f6(pc), a1
pub fn i_058574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58578;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588F6;
    16
}
/// $058578: move.l a1, (a6)
pub fn i_058578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5857A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05857A: tst.b $78(a6)
pub fn i_05857a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5857E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05857E: bne.w $58588
pub fn i_05857e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58582;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58588; return 10; }
    8
}
/// $058582: lea.l $58412(pc), a1
pub fn i_058582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58586;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058586: move.l a1, (a6)
pub fn i_058586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58588;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058588: jsr $56fa0(pc)
pub fn i_058588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5858C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5858C);
    cpu.pc = 0x56FA0;
    16
}
/// $05858C: bcc.w $58596
pub fn i_05858c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58590;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58596; return 10; }
    8
}
/// $058590: lea.l $58968(pc), a1
pub fn i_058590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58594;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58968;
    16
}
/// $058594: move.l a1, (a6)
pub fn i_058594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58596;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058596: jsr $574e8(pc)
pub fn i_058596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5859A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5859A);
    cpu.pc = 0x574E8;
    16
}
/// $05859A: bcc.w $585a4
pub fn i_05859a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5859E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x585A4; return 10; }
    8
}
/// $05859E: lea.l $585ae(pc), a1
pub fn i_05859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585AE;
    16
}
/// $0585A2: move.l a1, (a6)
pub fn i_0585a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0585A4: jsr $49fd0.l
pub fn i_0585a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585AA);
    cpu.pc = 0x49FD0;
    16
}
/// $0585AA: bra.w $56e1e
pub fn i_0585aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_MeleeAttack_0585ae  ($0585AE, 72 B, enemies/soldiers/states_057d04.s)
/// $0585AE: clr.w $28(a6)
pub fn i_0585ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585B2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0585B2: lea.l $2b70d2.l, a0
pub fn i_0585b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B70D2;
    20
}
/// $0585B8: jsr $28cd4.l
pub fn i_0585b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585BE);
    cpu.pc = 0x28CD4;
    16
}
/// $0585BE: lea.l $57494(pc), a0
pub fn i_0585be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x57494;
    16
}
/// $0585C2: move.l a0, $4c(a6)
pub fn i_0585c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585C6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0585C6: jsr $283ca.l
pub fn i_0585c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585CC);
    cpu.pc = 0x283CA;
    16
}
/// $0585CC: jsr $283ca.l
pub fn i_0585cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585D2);
    cpu.pc = 0x283CA;
    16
}
/// $0585D2: lea.l $585d8(pc), a1
pub fn i_0585d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x585D8;
    16
}
/// $0585D6: move.l a1, (a6)
pub fn i_0585d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0585D8: bsr.w $56acc
pub fn i_0585d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x585DC);
    cpu.pc = 0x56ACC;
    18
}
/// $0585DC: jsr $28d70.l
pub fn i_0585dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585E2);
    cpu.pc = 0x28D70;
    16
}
/// $0585E2: bcc.w $585ec
pub fn i_0585e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x585EC; return 10; }
    8
}
/// $0585E6: lea.l $58144(pc), a1
pub fn i_0585e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58144;
    16
}
/// $0585EA: move.l a1, (a6)
pub fn i_0585ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0585EC: jsr $49fd0.l
pub fn i_0585ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x585F2);
    cpu.pc = 0x49FD0;
    16
}
/// $0585F2: bra.w $56e1e
pub fn i_0585f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x56E1E;
    10
}
// ===== Soldier_Brake_0585f6  ($0585F6, 98 B, enemies/soldiers/states_057d04.s)
/// $0585F6: lea.l $2b5cfa.l, a0
pub fn i_0585f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x585FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5CFA;
    20
}
/// $0585FC: jsr $28cd4.l
pub fn i_0585fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58602);
    cpu.pc = 0x28CD4;
    16
}
/// $058602: lea.l $58608(pc), a1
pub fn i_058602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58606;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58608;
    16
}
/// $058606: move.l a1, (a6)
pub fn i_058606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58608;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058608: bsr.w $56b92
pub fn i_058608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5860C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5860C);
    cpu.pc = 0x56B92;
    18
}
/// $05860C: move.w $28(a6), d0
pub fn i_05860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58610;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058610: asr.w #$4, d0
pub fn i_058610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58612;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $058612: sub.w d0, $28(a6)
pub fn i_058612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58616;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $058616: bsr.w $56acc
pub fn i_058616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5861A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5861A);
    cpu.pc = 0x56ACC;
    18
}
/// $05861A: jsr $28d70.l
pub fn i_05861a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58620);
    cpu.pc = 0x28D70;
    16
}
/// $058620: bcc.w $5863e
pub fn i_058620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58624;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5863E; return 10; }
    8
}
/// $058624: btst.b #$0, $72(a6)
pub fn i_058624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5862A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05862A: beq.w $58638
pub fn i_05862a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5862E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58638; return 10; }
    8
}
/// $05862E: lea.l $5804c(pc), a1
pub fn i_05862e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58632;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5804C;
    16
}
/// $058632: move.l a1, (a6)
pub fn i_058632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58634;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058634: bra.w $5863e
pub fn i_058634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58638;
    cpu.ir = 0x6000;
    cpu.pc = 0x5863E;
    10
}
/// $058638: lea.l $57558(pc), a1
pub fn i_058638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5863C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $05863C: move.l a1, (a6)
pub fn i_05863c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5863E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05863E: tst.b $78(a6)
pub fn i_05863e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58642;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058642: bne.w $5864c
pub fn i_058642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58646;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5864C; return 10; }
    8
}
/// $058646: lea.l $58412(pc), a1
pub fn i_058646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5864A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $05864A: move.l a1, (a6)
pub fn i_05864a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5864C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05864C: jsr $49fd0.l
pub fn i_05864c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58652);
    cpu.pc = 0x49FD0;
    16
}
/// $058652: bsr.w $56e1e
pub fn i_058652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58656;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58656);
    cpu.pc = 0x56E1E;
    18
}
/// $058656: rts
pub fn i_058656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58658;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Flee_058658  ($058658, 96 B, enemies/soldiers/states_057d04.s)
/// $058658: btst.b #$4, $74(a6)
pub fn i_058658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5865E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05865E: bne.w $58aa8
pub fn i_05865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58662;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58AA8; return 10; }
    8
}
/// $058662: lea.l $29b956.l, a0
pub fn i_058662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58668;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B956;
    20
}
/// $058668: jsr $28cd4.l
pub fn i_058668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5866E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5866E);
    cpu.pc = 0x28CD4;
    16
}
/// $05866E: move.w $28(a6), d0
pub fn i_05866e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58672;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058672: add.w d0, d0
pub fn i_058672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58674;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $058674: move.w d0, $28(a6)
pub fn i_058674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58678;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058678: clr.w $2a(a6)
pub fn i_058678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5867C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05867C: lea.l $58682(pc), a1
pub fn i_05867c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58680;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58682;
    16
}
/// $058680: move.l a1, (a6)
pub fn i_058680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58682;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058682: move.w $28(a6), d0
pub fn i_058682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58686;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058686: asr.w #$5, d0
pub fn i_058686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58688;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $058688: sub.w d0, $28(a6)
pub fn i_058688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5868C;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05868C: bsr.w $56acc
pub fn i_05868c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58690;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58690);
    cpu.pc = 0x56ACC;
    18
}
/// $058690: bsr.w $56b92
pub fn i_058690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58694;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x58694);
    cpu.pc = 0x56B92;
    18
}
/// $058694: jsr $28d70.l
pub fn i_058694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5869A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5869A);
    cpu.pc = 0x28D70;
    16
}
/// $05869A: bcc.w $586ac
pub fn i_05869a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5869E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x586AC; return 10; }
    8
}
/// $05869E: tst.b $78(a6)
pub fn i_05869e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586A2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0586A2: beq.w $586ac
pub fn i_0586a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x586AC; return 10; }
    8
}
/// $0586A6: lea.l $586b8(pc), a1
pub fn i_0586a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x586B8;
    16
}
/// $0586AA: move.l a1, (a6)
pub fn i_0586aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0586AC: jsr $49fd0.l
pub fn i_0586ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x586B2);
    cpu.pc = 0x49FD0;
    16
}
/// $0586B2: bsr.w $56e1e
pub fn i_0586b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586B6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x586B6);
    cpu.pc = 0x56E1E;
    18
}
/// $0586B6: rts
pub fn i_0586b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_FleeStop_0586b8  ($0586B8, 230 B, enemies/soldiers/states_057d04.s)
/// $0586B8: lea.l $2b5c10.l, a0
pub fn i_0586b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C10;
    20
}
/// $0586BE: jsr $28cd4.l
pub fn i_0586be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x586C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0586C4: clr.w $28(a6)
pub fn i_0586c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586C8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0586C8: lea.l $586ce(pc), a1
pub fn i_0586c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x586CE;
    16
}
/// $0586CC: move.l a1, (a6)
pub fn i_0586cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0586CE: bsr.w $56acc
pub fn i_0586ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586D2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x586D2);
    cpu.pc = 0x56ACC;
    18
}
/// $0586D2: bsr.w $56b92
pub fn i_0586d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586D6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x586D6);
    cpu.pc = 0x56B92;
    18
}
/// $0586D6: jsr $28d70.l
pub fn i_0586d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x586DC);
    cpu.pc = 0x28D70;
    16
}
/// $0586DC: move.w $8a(a6), d1
pub fn i_0586dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586E0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0586E0: neg.w d1
pub fn i_0586e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586E2;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0586E2: addi.w #$100, d1
pub fn i_0586e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586E6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0586E6: jsr $5e9b6.l
pub fn i_0586e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x586EC);
    cpu.pc = 0x5E9B6;
    16
}
/// $0586EC: move.w d0, -(a7)
pub fn i_0586ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586EE;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0586EE: andi.w #$ff, d0
pub fn i_0586ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586F2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0586F2: cmp.w d1, d0
pub fn i_0586f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586F4;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0586F4: bcc.w $586fc
pub fn i_0586f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x586FC; return 10; }
    8
}
/// $0586F8: move.b (a7), d0
pub fn i_0586f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586FA;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0586FA: cmp.w d1, d0
pub fn i_0586fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586FC;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0586FC: addq.w #$2, a7
pub fn i_0586fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586FE;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $0586FE: bcc.w $58708
pub fn i_0586fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58702;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58708; return 10; }
    8
}
/// $058702: lea.l $588ae(pc), a1
pub fn i_058702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58706;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $058706: move.l a1, (a6)
pub fn i_058706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58708;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058708: btst.b #$0, $72(a6)
pub fn i_058708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5870E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05870E: beq.w $58762
pub fn i_05870e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58712;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58762; return 10; }
    8
}
/// $058712: jsr $5e9b6.l
pub fn i_058712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58718);
    cpu.pc = 0x5E9B6;
    16
}
/// $058718: move.w d0, -(a7)
pub fn i_058718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5871A;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05871A: andi.w #$ff, d0
pub fn i_05871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5871E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05871E: cmp.w $82(a6), d0
pub fn i_05871e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58722;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $058722: bcc.w $5872c
pub fn i_058722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58726;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5872C; return 10; }
    8
}
/// $058726: move.b (a7), d0
pub fn i_058726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58728;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058728: cmp.w $82(a6), d0
pub fn i_058728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5872C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05872C: addq.w #$2, a7
pub fn i_05872c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5872E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05872E: bcc.w $58738
pub fn i_05872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58732;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58738; return 10; }
    8
}
/// $058732: lea.l $5879e(pc), a1
pub fn i_058732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58736;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5879E;
    16
}
/// $058736: move.l a1, (a6)
pub fn i_058736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58738;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058738: move.w $80(a6), d1
pub fn i_058738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5873C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05873C: jsr $5e9b6.l
pub fn i_05873c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58742);
    cpu.pc = 0x5E9B6;
    16
}
/// $058742: move.w d0, -(a7)
pub fn i_058742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58744;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $058744: andi.w #$ff, d0
pub fn i_058744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58748;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058748: cmp.w d1, d0
pub fn i_058748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5874A;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05874A: bcc.w $58752
pub fn i_05874a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5874E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58752; return 10; }
    8
}
/// $05874E: move.b (a7), d0
pub fn i_05874e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58750;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $058750: cmp.w d1, d0
pub fn i_058750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58752;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058752: addq.w #$2, a7
pub fn i_058752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58754;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $058754: bcc.w $5875e
pub fn i_058754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58758;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5875E; return 10; }
    8
}
/// $058758: lea.l $587ac(pc), a1
pub fn i_058758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5875C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x587AC;
    16
}
/// $05875C: move.l a1, (a6)
pub fn i_05875c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5875E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05875E: bra.w $58768
pub fn i_05875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58762;
    cpu.ir = 0x6000;
    cpu.pc = 0x58768;
    10
}
/// $058762: lea.l $587ac(pc), a1
pub fn i_058762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x587AC;
    16
}
/// $058766: move.l a1, (a6)
pub fn i_058766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058768: jsr $56e36(pc)
pub fn i_058768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5876C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5876C);
    cpu.pc = 0x56E36;
    16
}
/// $05876C: bcc.w $58776
pub fn i_05876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58770;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58776; return 10; }
    8
}
/// $058770: lea.l $591c8(pc), a1
pub fn i_058770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58774;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x591C8;
    16
}
/// $058774: move.l a1, (a6)
pub fn i_058774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58776;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058776: jsr $56fa0(pc)
pub fn i_058776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5877A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5877A);
    cpu.pc = 0x56FA0;
    16
}
/// $05877A: bcc.w $58784
pub fn i_05877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5877E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58784; return 10; }
    8
}
/// $05877E: lea.l $589f8(pc), a1
pub fn i_05877e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58782;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x589F8;
    16
}
/// $058782: move.l a1, (a6)
pub fn i_058782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58784;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058784: tst.b $78(a6)
pub fn i_058784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58788;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058788: bne.w $58792
pub fn i_058788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5878C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x58792; return 10; }
    8
}
/// $05878C: lea.l $58412(pc), a1
pub fn i_05878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58790;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $058790: move.l a1, (a6)
pub fn i_058790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58792;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058792: jsr $49ff2.l
pub fn i_058792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58798);
    cpu.pc = 0x49FF2;
    16
}
/// $058798: bsr.w $56e1e
pub fn i_058798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5879C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5879C);
    cpu.pc = 0x56E1E;
    18
}
/// $05879C: rts
pub fn i_05879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5879E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_RetreatJmp_05879e  ($05879E, 14 B, enemies/soldiers/states_057d04.s)
/// $05879E: btst.b #$0, $72(a6)
pub fn i_05879e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587A4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0587A4: bne.w $587b6
pub fn i_0587a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x587B6; return 10; }
    8
}
/// $0587A8: bra.w $587e4
pub fn i_0587a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x587E4;
    10
}
// ===== Soldier_Retreat_0587ac  ($0587AC, 84 B, enemies/soldiers/states_057d04.s)
/// $0587AC: btst.b #$0, $72(a6)
pub fn i_0587ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0587B2: bne.w $587e4
pub fn i_0587b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x587E4; return 10; }
    8
}
/// $0587B6: lea.l $2b5bcc.l, a0
pub fn i_0587b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5BCC;
    20
}
/// $0587BC: jsr $28cd4.l
pub fn i_0587bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x587C2);
    cpu.pc = 0x28CD4;
    16
}
/// $0587C2: move.w #$100, d0
pub fn i_0587c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0587C6: btst.b #$0, $72(a6)
pub fn i_0587c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587CC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0587CC: bne.w $587d2
pub fn i_0587cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x587D2; return 10; }
    8
}
/// $0587D0: add.w d0, d0
pub fn i_0587d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587D2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0587D2: move.w d0, $36(a6)
pub fn i_0587d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587D6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0587D6: bsr.w $56f8a
pub fn i_0587d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587DA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x587DA);
    cpu.pc = 0x56F8A;
    18
}
/// $0587DA: lea.l $58800(pc), a1
pub fn i_0587da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58800;
    16
}
/// $0587DE: move.l a1, (a6)
pub fn i_0587de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0587E0: bra.w $58800
pub fn i_0587e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x58800;
    10
}
/// $0587E4: lea.l $2b5bcc.l, a0
pub fn i_0587e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5BCC;
    20
}
/// $0587EA: jsr $28cd4.l
pub fn i_0587ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x587F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0587F0: move.w #$ff00, $36(a6)
pub fn i_0587f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0587F6: bsr.w $56f8a
pub fn i_0587f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587FA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x587FA);
    cpu.pc = 0x56F8A;
    18
}
/// $0587FA: lea.l $58800(pc), a1
pub fn i_0587fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x587FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58800;
    16
}
/// $0587FE: move.l a1, (a6)
pub fn i_0587fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58800;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Soldier_Retreat_Loop_058800  ($058800, 174 B, enemies/soldiers/states_057d04.s)
/// $058800: jsr $282d8.l
pub fn i_058800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58806);
    cpu.pc = 0x282D8;
    16
}
/// $058806: bcc.w $58810
pub fn i_058806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5880A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58810; return 10; }
    8
}
/// $05880A: lea.l $588ae(pc), a1
pub fn i_05880a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5880E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $05880E: move.l a1, (a6)
pub fn i_05880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58810;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058810: jsr $27eba.l
pub fn i_058810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58816);
    cpu.pc = 0x27EBA;
    16
}
/// $058816: scc.b $78(a6)
pub fn i_058816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5881A;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $05881A: bsr.w $56b92
pub fn i_05881a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5881E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5881E);
    cpu.pc = 0x56B92;
    18
}
/// $05881E: move.w $34(a6), d0
pub fn i_05881e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58822;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058822: beq.w $5882c
pub fn i_058822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58826;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5882C; return 10; }
    8
}
/// $058826: lea.l $588ae(pc), a1
pub fn i_058826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5882A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $05882A: move.l a1, (a6)
pub fn i_05882a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5882C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05882C: jsr $28d70.l
pub fn i_05882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58832);
    cpu.pc = 0x28D70;
    16
}
/// $058832: bcc.w $5883c
pub fn i_058832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58836;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5883C; return 10; }
    8
}
/// $058836: lea.l $586b8(pc), a1
pub fn i_058836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5883A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x586B8;
    16
}
/// $05883A: move.l a1, (a6)
pub fn i_05883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5883C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05883C: move.w $8a(a6), d1
pub fn i_05883c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58840;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $058840: neg.w d1
pub fn i_058840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58842;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058842: addi.w #$100, d1
pub fn i_058842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58846;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058846: jsr $5e9b6.l
pub fn i_058846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5884C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5884C);
    cpu.pc = 0x5E9B6;
    16
}
/// $05884C: move.w d0, -(a7)
pub fn i_05884c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5884E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05884E: andi.w #$ff, d0
pub fn i_05884e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58852;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $058852: cmp.w d1, d0
pub fn i_058852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58854;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $058854: bcc.w $5885c
pub fn i_058854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58858;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5885C; return 10; }
    8
}
/// $058858: move.b (a7), d0
pub fn i_058858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5885A;
    cpu.ir = 0x1017;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05885A: cmp.w d1, d0
pub fn i_05885a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5885C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05885C: addq.w #$2, a7
pub fn i_05885c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5885E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05885E: bcc.w $58868
pub fn i_05885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58862;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58868; return 10; }
    8
}
/// $058862: lea.l $588ae(pc), a1
pub fn i_058862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58866;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $058866: move.l a1, (a6)
pub fn i_058866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58868;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058868: btst.b #$5, $5a(a6)
pub fn i_058868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5886E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05886E: beq.w $58878
pub fn i_05886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58872;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x58878; return 10; }
    8
}
/// $058872: lea.l $588ae(pc), a1
pub fn i_058872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58876;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588AE;
    16
}
/// $058876: move.l a1, (a6)
pub fn i_058876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58878;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058878: jsr $56e36(pc)
pub fn i_058878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5887C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5887C);
    cpu.pc = 0x56E36;
    16
}
/// $05887C: bcc.w $58886
pub fn i_05887c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58880;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58886; return 10; }
    8
}
/// $058880: lea.l $591c8(pc), a1
pub fn i_058880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58884;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x591C8;
    16
}
/// $058884: move.l a1, (a6)
pub fn i_058884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58886;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058886: jsr $56fa0(pc)
pub fn i_058886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5888A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5888A);
    cpu.pc = 0x56FA0;
    16
}
/// $05888A: bcc.w $58894
pub fn i_05888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5888E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x58894; return 10; }
    8
}
/// $05888E: lea.l $589f8(pc), a1
pub fn i_05888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58892;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x589F8;
    16
}
/// $058892: move.l a1, (a6)
pub fn i_058892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58894;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058894: tst.b $78(a6)
pub fn i_058894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58898;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $058898: bne.w $588a2
pub fn i_058898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5889C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x588A2; return 10; }
    8
}
/// $05889C: lea.l $58412(pc), a1
pub fn i_05889c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0588A0: move.l a1, (a6)
pub fn i_0588a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0588A2: jsr $49ff2.l
pub fn i_0588a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x588A8);
    cpu.pc = 0x49FF2;
    16
}
/// $0588A8: bsr.w $56e1e
pub fn i_0588a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588AC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x588AC);
    cpu.pc = 0x56E1E;
    18
}
/// $0588AC: rts
pub fn i_0588ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Stand_0588ae  ($0588AE, 72 B, enemies/soldiers/states_057d04.s)
/// $0588AE: lea.l $29bd84.l, a0
pub fn i_0588ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BD84;
    20
}
/// $0588B4: jsr $28cd4.l
pub fn i_0588b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x588BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0588BA: clr.w $28(a6)
pub fn i_0588ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588BE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0588BE: lea.l $588c4(pc), a1
pub fn i_0588be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x588C4;
    16
}
/// $0588C2: move.l a1, (a6)
pub fn i_0588c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0588C4: jsr $56acc(pc)
pub fn i_0588c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x588C8);
    cpu.pc = 0x56ACC;
    16
}
/// $0588C8: bsr.w $56b92
pub fn i_0588c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588CC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x588CC);
    cpu.pc = 0x56B92;
    18
}
/// $0588CC: jsr $28d70.l
pub fn i_0588cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x588D2);
    cpu.pc = 0x28D70;
    16
}
/// $0588D2: bcc.w $588dc
pub fn i_0588d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x588DC; return 10; }
    8
}
/// $0588D6: lea.l $57558(pc), a1
pub fn i_0588d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x57558;
    16
}
/// $0588DA: move.l a1, (a6)
pub fn i_0588da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0588DC: tst.b $78(a6)
pub fn i_0588dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588E0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0588E0: bne.w $588ea
pub fn i_0588e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x588EA; return 10; }
    8
}
/// $0588E4: lea.l $58412(pc), a1
pub fn i_0588e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58412;
    16
}
/// $0588E8: move.l a1, (a6)
pub fn i_0588e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0588EA: jsr $49fd0.l
pub fn i_0588ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x588F0);
    cpu.pc = 0x49FD0;
    16
}
/// $0588F0: bsr.w $56e1e
pub fn i_0588f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x588F4);
    cpu.pc = 0x56E1E;
    18
}
/// $0588F4: rts
pub fn i_0588f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Jump_0588f6  ($0588F6, 114 B, enemies/soldiers/states_057d04.s)
/// $0588F6: move.w $84(a6), d0
pub fn i_0588f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0588FA: add.w $82(a6), d0
pub fn i_0588fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588FE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0588FE: sub.w $86(a6), d0
pub fn i_0588fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58902;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058902: sub.w $80(a6), d0
pub fn i_058902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58906;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058906: asr.w #$1, d0
pub fn i_058906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58908;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $058908: btst.b #$0, $3a(a6)
pub fn i_058908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5890E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05890E: seq.b d1
pub fn i_05890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58910;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $058910: ext.w d1
pub fn i_058910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58912;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $058912: eor.w d0, d1
pub fn i_058912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58914;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $058914: move.w d1, $28(a6)
pub fn i_058914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58918;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058918: tst.w d0
pub fn i_058918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5891A;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05891A: bmi.w $5893e
pub fn i_05891a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5891E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5893E; return 10; }
    8
}
/// $05891E: lea.l $2b5d58.l, a0
pub fn i_05891e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58924;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5D58;
    20
}
/// $058924: jsr $28cd4.l
pub fn i_058924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5892A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5892A);
    cpu.pc = 0x28CD4;
    16
}
/// $05892A: move.w $88(a6), d0
pub fn i_05892a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5892E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05892E: lsl.w #$3, d0
pub fn i_05892e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58930;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058930: move.w d0, $2a(a6)
pub fn i_058930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58934;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058934: lea.l $58424(pc), a1
pub fn i_058934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58938;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58424;
    16
}
/// $058938: move.l a1, (a6)
pub fn i_058938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5893A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05893A: bra.w $58424
pub fn i_05893a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5893E;
    cpu.ir = 0x6000;
    cpu.pc = 0x58424;
    10
}
/// $05893E: lea.l $2b5e68.l, a0
pub fn i_05893e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58944;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5E68;
    20
}
/// $058944: jsr $28cd4.l
pub fn i_058944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5894A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5894A);
    cpu.pc = 0x28CD4;
    16
}
/// $05894A: move.w $88(a6), d0
pub fn i_05894a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5894E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05894E: lsl.w #$3, d0
pub fn i_05894e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58950;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $058950: move.w d0, $2a(a6)
pub fn i_058950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58954;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $058954: move.w #$ff00, $36(a6)
pub fn i_058954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5895A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05895A: bsr.w $56f8a
pub fn i_05895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5895E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5895E);
    cpu.pc = 0x56F8A;
    18
}
/// $05895E: lea.l $58424(pc), a1
pub fn i_05895e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58962;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58424;
    16
}
/// $058962: move.l a1, (a6)
pub fn i_058962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58964;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $058964: bra.w $58424
pub fn i_058964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58968;
    cpu.ir = 0x6000;
    cpu.pc = 0x58424;
    10
}
// ===== Soldier_Surrender_058968  ($058968, 144 B, enemies/soldiers/states_057d04.s)
/// $058968: move.b $75(a6), d0
pub fn i_058968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5896C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x057108, i_057108),
    (0x05710C, i_05710c),
    (0x057110, i_057110),
    (0x057114, i_057114),
    (0x057116, i_057116),
    (0x05711A, i_05711a),
    (0x057120, i_057120),
    (0x057124, i_057124),
    (0x05712A, i_05712a),
    (0x05712E, i_05712e),
    (0x057132, i_057132),
    (0x057136, i_057136),
    (0x05713C, i_05713c),
    (0x057142, i_057142),
    (0x057148, i_057148),
    (0x05714E, i_05714e),
    (0x057154, i_057154),
    (0x057158, i_057158),
    (0x05715C, i_05715c),
    (0x057160, i_057160),
    (0x057164, i_057164),
    (0x05716A, i_05716a),
    (0x05716E, i_05716e),
    (0x057174, i_057174),
    (0x057178, i_057178),
    (0x05717A, i_05717a),
    (0x057180, i_057180),
    (0x057184, i_057184),
    (0x05718A, i_05718a),
    (0x05718E, i_05718e),
    (0x057194, i_057194),
    (0x05719A, i_05719a),
    (0x05719C, i_05719c),
    (0x0571A2, i_0571a2),
    (0x0571A6, i_0571a6),
    (0x0571AC, i_0571ac),
    (0x0571B0, i_0571b0),
    (0x0571B6, i_0571b6),
    (0x0571BC, i_0571bc),
    (0x0571C2, i_0571c2),
    (0x0571C4, i_0571c4),
    (0x0571CA, i_0571ca),
    (0x0571D0, i_0571d0),
    (0x0571D6, i_0571d6),
    (0x0571D8, i_0571d8),
    (0x0571DE, i_0571de),
    (0x0571E4, i_0571e4),
    (0x0571EA, i_0571ea),
    (0x0571EC, i_0571ec),
    (0x0571F2, i_0571f2),
    (0x0571F8, i_0571f8),
    (0x0571FE, i_0571fe),
    (0x057200, i_057200),
    (0x057206, i_057206),
    (0x05720C, i_05720c),
    (0x05720E, i_05720e),
    (0x057214, i_057214),
    (0x057216, i_057216),
    (0x05721C, i_05721c),
    (0x05721E, i_05721e),
    (0x057224, i_057224),
    (0x057226, i_057226),
    (0x05722C, i_05722c),
    (0x057232, i_057232),
    (0x057238, i_057238),
    (0x05723E, i_05723e),
    (0x057244, i_057244),
    (0x05724A, i_05724a),
    (0x05724E, i_05724e),
    (0x057254, i_057254),
    (0x05725A, i_05725a),
    (0x057260, i_057260),
    (0x057266, i_057266),
    (0x05726C, i_05726c),
    (0x057272, i_057272),
    (0x057276, i_057276),
    (0x05727A, i_05727a),
    (0x05727E, i_05727e),
    (0x057282, i_057282),
    (0x057286, i_057286),
    (0x05728A, i_05728a),
    (0x05728E, i_05728e),
    (0x057292, i_057292),
    (0x057296, i_057296),
    (0x05729A, i_05729a),
    (0x05729E, i_05729e),
    (0x0572A2, i_0572a2),
    (0x0572A6, i_0572a6),
    (0x0572AA, i_0572aa),
    (0x0572AE, i_0572ae),
    (0x0572B2, i_0572b2),
    (0x0572B6, i_0572b6),
    (0x0572BA, i_0572ba),
    (0x0572BE, i_0572be),
    (0x0572C2, i_0572c2),
    (0x0572C6, i_0572c6),
    (0x0572CA, i_0572ca),
    (0x0572CE, i_0572ce),
    (0x0572D2, i_0572d2),
    (0x0572D6, i_0572d6),
    (0x0572DA, i_0572da),
    (0x0572DE, i_0572de),
    (0x0572E2, i_0572e2),
    (0x0572E6, i_0572e6),
    (0x0572EA, i_0572ea),
    (0x0572EE, i_0572ee),
    (0x0572F2, i_0572f2),
    (0x0572F6, i_0572f6),
    (0x0572FA, i_0572fa),
    (0x0572FE, i_0572fe),
    (0x057302, i_057302),
    (0x057306, i_057306),
    (0x05730A, i_05730a),
    (0x05730E, i_05730e),
    (0x057312, i_057312),
    (0x057316, i_057316),
    (0x05731A, i_05731a),
    (0x05731E, i_05731e),
    (0x057322, i_057322),
    (0x057326, i_057326),
    (0x05732A, i_05732a),
    (0x05732E, i_05732e),
    (0x057332, i_057332),
    (0x057336, i_057336),
    (0x05733A, i_05733a),
    (0x05733E, i_05733e),
    (0x057342, i_057342),
    (0x057346, i_057346),
    (0x05734A, i_05734a),
    (0x05734E, i_05734e),
    (0x057352, i_057352),
    (0x057356, i_057356),
    (0x05735A, i_05735a),
    (0x05735E, i_05735e),
    (0x057362, i_057362),
    (0x057366, i_057366),
    (0x05736A, i_05736a),
    (0x057370, i_057370),
    (0x057374, i_057374),
    (0x05737A, i_05737a),
    (0x05737E, i_05737e),
    (0x057382, i_057382),
    (0x057386, i_057386),
    (0x05738A, i_05738a),
    (0x05738E, i_05738e),
    (0x057392, i_057392),
    (0x057396, i_057396),
    (0x05739A, i_05739a),
    (0x05739E, i_05739e),
    (0x0573A2, i_0573a2),
    (0x0573A6, i_0573a6),
    (0x0573AA, i_0573aa),
    (0x0573AE, i_0573ae),
    (0x0573B2, i_0573b2),
    (0x0573B6, i_0573b6),
    (0x0573BC, i_0573bc),
    (0x0573BE, i_0573be),
    (0x0573C2, i_0573c2),
    (0x0573C8, i_0573c8),
    (0x0573CE, i_0573ce),
    (0x0573D2, i_0573d2),
    (0x0573D6, i_0573d6),
    (0x0573DA, i_0573da),
    (0x0573DC, i_0573dc),
    (0x0573DE, i_0573de),
    (0x0573E2, i_0573e2),
    (0x0573E6, i_0573e6),
    (0x0573EC, i_0573ec),
    (0x0573EE, i_0573ee),
    (0x0573F2, i_0573f2),
    (0x0573F8, i_0573f8),
    (0x0573FE, i_0573fe),
    (0x057402, i_057402),
    (0x057406, i_057406),
    (0x05740A, i_05740a),
    (0x05740C, i_05740c),
    (0x05740E, i_05740e),
    (0x057412, i_057412),
    (0x057416, i_057416),
    (0x057418, i_057418),
    (0x05741C, i_05741c),
    (0x057420, i_057420),
    (0x057422, i_057422),
    (0x057424, i_057424),
    (0x057428, i_057428),
    (0x05742E, i_05742e),
    (0x057432, i_057432),
    (0x057438, i_057438),
    (0x05743E, i_05743e),
    (0x0574E8, i_0574e8),
    (0x0574EC, i_0574ec),
    (0x0574F0, i_0574f0),
    (0x0574F4, i_0574f4),
    (0x0574F8, i_0574f8),
    (0x0574FC, i_0574fc),
    (0x057500, i_057500),
    (0x057504, i_057504),
    (0x05750A, i_05750a),
    (0x057510, i_057510),
    (0x057516, i_057516),
    (0x05751C, i_05751c),
    (0x057520, i_057520),
    (0x057524, i_057524),
    (0x057526, i_057526),
    (0x05752A, i_05752a),
    (0x05752C, i_05752c),
    (0x057530, i_057530),
    (0x057534, i_057534),
    (0x057538, i_057538),
    (0x05753E, i_05753e),
    (0x057542, i_057542),
    (0x057546, i_057546),
    (0x05754A, i_05754a),
    (0x05754C, i_05754c),
    (0x057550, i_057550),
    (0x057554, i_057554),
    (0x057556, i_057556),
    (0x057558, i_057558),
    (0x05755E, i_05755e),
    (0x057564, i_057564),
    (0x05756A, i_05756a),
    (0x057570, i_057570),
    (0x057574, i_057574),
    (0x057578, i_057578),
    (0x05757C, i_05757c),
    (0x057580, i_057580),
    (0x057582, i_057582),
    (0x057586, i_057586),
    (0x05758A, i_05758a),
    (0x05758E, i_05758e),
    (0x057592, i_057592),
    (0x057596, i_057596),
    (0x057598, i_057598),
    (0x05759C, i_05759c),
    (0x0575A0, i_0575a0),
    (0x0575A4, i_0575a4),
    (0x0575A8, i_0575a8),
    (0x0575AA, i_0575aa),
    (0x0575AE, i_0575ae),
    (0x0575B4, i_0575b4),
    (0x0575B8, i_0575b8),
    (0x0575BE, i_0575be),
    (0x0575C2, i_0575c2),
    (0x0575C6, i_0575c6),
    (0x0575CA, i_0575ca),
    (0x0575CE, i_0575ce),
    (0x0575D2, i_0575d2),
    (0x0575D4, i_0575d4),
    (0x0575D8, i_0575d8),
    (0x0575DE, i_0575de),
    (0x0575E0, i_0575e0),
    (0x0575E4, i_0575e4),
    (0x0575E6, i_0575e6),
    (0x0575EA, i_0575ea),
    (0x0575EC, i_0575ec),
    (0x0575EE, i_0575ee),
    (0x0575F0, i_0575f0),
    (0x0575F4, i_0575f4),
    (0x0575F8, i_0575f8),
    (0x0575FA, i_0575fa),
    (0x057600, i_057600),
    (0x057602, i_057602),
    (0x057606, i_057606),
    (0x05760A, i_05760a),
    (0x05760E, i_05760e),
    (0x057610, i_057610),
    (0x057614, i_057614),
    (0x057616, i_057616),
    (0x05761A, i_05761a),
    (0x05761E, i_05761e),
    (0x057620, i_057620),
    (0x057624, i_057624),
    (0x057628, i_057628),
    (0x05762C, i_05762c),
    (0x057630, i_057630),
    (0x057636, i_057636),
    (0x057638, i_057638),
    (0x05763C, i_05763c),
    (0x05763E, i_05763e),
    (0x057642, i_057642),
    (0x057644, i_057644),
    (0x057646, i_057646),
    (0x057648, i_057648),
    (0x05764C, i_05764c),
    (0x057650, i_057650),
    (0x057652, i_057652),
    (0x057658, i_057658),
    (0x05765A, i_05765a),
    (0x05765E, i_05765e),
    (0x057662, i_057662),
    (0x057666, i_057666),
    (0x057668, i_057668),
    (0x05766C, i_05766c),
    (0x05766E, i_05766e),
    (0x057672, i_057672),
    (0x057676, i_057676),
    (0x057678, i_057678),
    (0x05767C, i_05767c),
    (0x057680, i_057680),
    (0x057686, i_057686),
    (0x057688, i_057688),
    (0x05768C, i_05768c),
    (0x05768E, i_05768e),
    (0x057692, i_057692),
    (0x057694, i_057694),
    (0x057696, i_057696),
    (0x057698, i_057698),
    (0x05769C, i_05769c),
    (0x0576A0, i_0576a0),
    (0x0576A2, i_0576a2),
    (0x0576A8, i_0576a8),
    (0x0576AC, i_0576ac),
    (0x0576B0, i_0576b0),
    (0x0576B2, i_0576b2),
    (0x0576B8, i_0576b8),
    (0x0576BC, i_0576bc),
    (0x0576C0, i_0576c0),
    (0x0576C2, i_0576c2),
    (0x0576C6, i_0576c6),
    (0x0576CA, i_0576ca),
    (0x0576CE, i_0576ce),
    (0x0576D0, i_0576d0),
    (0x0576D4, i_0576d4),
    (0x0576D8, i_0576d8),
    (0x0576DC, i_0576dc),
    (0x0576DE, i_0576de),
    (0x0576E2, i_0576e2),
    (0x0576E6, i_0576e6),
    (0x0576EA, i_0576ea),
    (0x0576EC, i_0576ec),
    (0x0576F0, i_0576f0),
    (0x0576F4, i_0576f4),
    (0x0576F8, i_0576f8),
    (0x0576FA, i_0576fa),
    (0x057700, i_057700),
    (0x057704, i_057704),
    (0x057706, i_057706),
    (0x05770C, i_05770c),
    (0x057710, i_057710),
    (0x057714, i_057714),
    (0x057718, i_057718),
    (0x05771C, i_05771c),
    (0x057720, i_057720),
    (0x057724, i_057724),
    (0x057728, i_057728),
    (0x05772C, i_05772c),
    (0x057730, i_057730),
    (0x057736, i_057736),
    (0x05773A, i_05773a),
    (0x05773E, i_05773e),
    (0x057742, i_057742),
    (0x057744, i_057744),
    (0x057746, i_057746),
    (0x05774C, i_05774c),
    (0x05774E, i_05774e),
    (0x057752, i_057752),
    (0x057754, i_057754),
    (0x057758, i_057758),
    (0x05775A, i_05775a),
    (0x05775C, i_05775c),
    (0x05775E, i_05775e),
    (0x057762, i_057762),
    (0x057766, i_057766),
    (0x057768, i_057768),
    (0x05776E, i_05776e),
    (0x057772, i_057772),
    (0x057776, i_057776),
    (0x057778, i_057778),
    (0x05777C, i_05777c),
    (0x057780, i_057780),
    (0x057784, i_057784),
    (0x057786, i_057786),
    (0x05778C, i_05778c),
    (0x057790, i_057790),
    (0x057794, i_057794),
    (0x057796, i_057796),
    (0x05779C, i_05779c),
    (0x0577A0, i_0577a0),
    (0x0577A4, i_0577a4),
    (0x0577A6, i_0577a6),
    (0x0577AA, i_0577aa),
    (0x0577AE, i_0577ae),
    (0x0577B0, i_0577b0),
    (0x0577B6, i_0577b6),
    (0x0577BA, i_0577ba),
    (0x0577BC, i_0577bc),
    (0x0577C0, i_0577c0),
    (0x0577C4, i_0577c4),
    (0x0577CA, i_0577ca),
    (0x0577CE, i_0577ce),
    (0x0577D0, i_0577d0),
    (0x0577D2, i_0577d2),
    (0x0577D8, i_0577d8),
    (0x0577DA, i_0577da),
    (0x0577DE, i_0577de),
    (0x0577E0, i_0577e0),
    (0x0577E4, i_0577e4),
    (0x0577E6, i_0577e6),
    (0x0577E8, i_0577e8),
    (0x0577EA, i_0577ea),
    (0x0577EE, i_0577ee),
    (0x0577F2, i_0577f2),
    (0x0577F4, i_0577f4),
    (0x0577FA, i_0577fa),
    (0x0577FE, i_0577fe),
    (0x057802, i_057802),
    (0x057804, i_057804),
    (0x057808, i_057808),
    (0x05780C, i_05780c),
    (0x057810, i_057810),
    (0x057812, i_057812),
    (0x057818, i_057818),
    (0x05781C, i_05781c),
    (0x057820, i_057820),
    (0x057822, i_057822),
    (0x057828, i_057828),
    (0x05782C, i_05782c),
    (0x057830, i_057830),
    (0x057832, i_057832),
    (0x057838, i_057838),
    (0x05783C, i_05783c),
    (0x05783E, i_05783e),
    (0x057842, i_057842),
    (0x057846, i_057846),
    (0x05784C, i_05784c),
    (0x057852, i_057852),
    (0x057856, i_057856),
    (0x05785C, i_05785c),
    (0x057860, i_057860),
    (0x057866, i_057866),
    (0x05786C, i_05786c),
    (0x057872, i_057872),
    (0x057876, i_057876),
    (0x05787A, i_05787a),
    (0x05787C, i_05787c),
    (0x057880, i_057880),
    (0x057884, i_057884),
    (0x057888, i_057888),
    (0x05788C, i_05788c),
    (0x057890, i_057890),
    (0x057896, i_057896),
    (0x05789C, i_05789c),
    (0x0578A0, i_0578a0),
    (0x0578A6, i_0578a6),
    (0x0578AA, i_0578aa),
    (0x0578B0, i_0578b0),
    (0x0578B6, i_0578b6),
    (0x0578BC, i_0578bc),
    (0x0578C0, i_0578c0),
    (0x0578C4, i_0578c4),
    (0x0578C8, i_0578c8),
    (0x0578CC, i_0578cc),
    (0x0578D2, i_0578d2),
    (0x0578D8, i_0578d8),
    (0x0578DC, i_0578dc),
    (0x0578E2, i_0578e2),
    (0x0578E8, i_0578e8),
    (0x0578EC, i_0578ec),
    (0x0578F2, i_0578f2),
    (0x0578F6, i_0578f6),
    (0x0578FC, i_0578fc),
    (0x057902, i_057902),
    (0x057908, i_057908),
    (0x05790C, i_05790c),
    (0x057910, i_057910),
    (0x057916, i_057916),
    (0x05791A, i_05791a),
    (0x057920, i_057920),
    (0x057924, i_057924),
    (0x05792A, i_05792a),
    (0x05792E, i_05792e),
    (0x057934, i_057934),
    (0x057938, i_057938),
    (0x05793E, i_05793e),
    (0x057942, i_057942),
    (0x057944, i_057944),
    (0x05794A, i_05794a),
    (0x05794E, i_05794e),
    (0x057952, i_057952),
    (0x057954, i_057954),
    (0x05795A, i_05795a),
    (0x05795E, i_05795e),
    (0x057962, i_057962),
    (0x057966, i_057966),
    (0x05796A, i_05796a),
    (0x057970, i_057970),
    (0x057976, i_057976),
    (0x05797A, i_05797a),
    (0x05797E, i_05797e),
    (0x057982, i_057982),
    (0x057986, i_057986),
    (0x05798A, i_05798a),
    (0x05798E, i_05798e),
    (0x057992, i_057992),
    (0x057994, i_057994),
    (0x05799A, i_05799a),
    (0x05799E, i_05799e),
    (0x0579A4, i_0579a4),
    (0x0579AA, i_0579aa),
    (0x0579B0, i_0579b0),
    (0x0579B4, i_0579b4),
    (0x0579B8, i_0579b8),
    (0x0579BE, i_0579be),
    (0x0579C2, i_0579c2),
    (0x0579C8, i_0579c8),
    (0x0579CE, i_0579ce),
    (0x0579D4, i_0579d4),
    (0x0579D8, i_0579d8),
    (0x0579DE, i_0579de),
    (0x0579E2, i_0579e2),
    (0x0579E6, i_0579e6),
    (0x0579EC, i_0579ec),
    (0x0579F0, i_0579f0),
    (0x0579F6, i_0579f6),
    (0x0579FC, i_0579fc),
    (0x057A02, i_057a02),
    (0x057A06, i_057a06),
    (0x057A0C, i_057a0c),
    (0x057A10, i_057a10),
    (0x057A14, i_057a14),
    (0x057A1A, i_057a1a),
    (0x057A1E, i_057a1e),
    (0x057A24, i_057a24),
    (0x057A2A, i_057a2a),
    (0x057A30, i_057a30),
    (0x057A34, i_057a34),
    (0x057A38, i_057a38),
    (0x057A3E, i_057a3e),
    (0x057A42, i_057a42),
    (0x057A48, i_057a48),
    (0x057A4E, i_057a4e),
    (0x057A54, i_057a54),
    (0x057A5A, i_057a5a),
    (0x057A5E, i_057a5e),
    (0x057A60, i_057a60),
    (0x057A66, i_057a66),
    (0x057A6A, i_057a6a),
    (0x057A6E, i_057a6e),
    (0x057A70, i_057a70),
    (0x057A76, i_057a76),
    (0x057A7A, i_057a7a),
    (0x057A7E, i_057a7e),
    (0x057A84, i_057a84),
    (0x057A8A, i_057a8a),
    (0x057A8E, i_057a8e),
    (0x057A92, i_057a92),
    (0x057A94, i_057a94),
    (0x057A9A, i_057a9a),
    (0x057A9E, i_057a9e),
    (0x057AA4, i_057aa4),
    (0x057AAA, i_057aaa),
    (0x057AB0, i_057ab0),
    (0x057AB4, i_057ab4),
    (0x057AB6, i_057ab6),
    (0x057ABC, i_057abc),
    (0x057AC0, i_057ac0),
    (0x057AC6, i_057ac6),
    (0x057ACC, i_057acc),
    (0x057AD0, i_057ad0),
    (0x057AD4, i_057ad4),
    (0x057AD8, i_057ad8),
    (0x057ADA, i_057ada),
    (0x057AE0, i_057ae0),
    (0x057AE4, i_057ae4),
    (0x057AEA, i_057aea),
    (0x057AEE, i_057aee),
    (0x057AF4, i_057af4),
    (0x057AF8, i_057af8),
    (0x057AFE, i_057afe),
    (0x057B04, i_057b04),
    (0x057B06, i_057b06),
    (0x057B0C, i_057b0c),
    (0x057B12, i_057b12),
    (0x057B18, i_057b18),
    (0x057B1C, i_057b1c),
    (0x057B22, i_057b22),
    (0x057B26, i_057b26),
    (0x057B2A, i_057b2a),
    (0x057B30, i_057b30),
    (0x057B34, i_057b34),
    (0x057B38, i_057b38),
    (0x057B3C, i_057b3c),
    (0x057B40, i_057b40),
    (0x057B44, i_057b44),
    (0x057B48, i_057b48),
    (0x057B4C, i_057b4c),
    (0x057B50, i_057b50),
    (0x057B54, i_057b54),
    (0x057B58, i_057b58),
    (0x057B5C, i_057b5c),
    (0x057B60, i_057b60),
    (0x057B64, i_057b64),
    (0x057B66, i_057b66),
    (0x057B6A, i_057b6a),
    (0x057B6E, i_057b6e),
    (0x057B72, i_057b72),
    (0x057B74, i_057b74),
    (0x057B78, i_057b78),
    (0x057B7C, i_057b7c),
    (0x057B80, i_057b80),
    (0x057B84, i_057b84),
    (0x057B88, i_057b88),
    (0x057B8C, i_057b8c),
    (0x057B90, i_057b90),
    (0x057B94, i_057b94),
    (0x057B98, i_057b98),
    (0x057B9C, i_057b9c),
    (0x057BA0, i_057ba0),
    (0x057BA6, i_057ba6),
    (0x057BAA, i_057baa),
    (0x057BAE, i_057bae),
    (0x057BB0, i_057bb0),
    (0x057BB4, i_057bb4),
    (0x057BBA, i_057bba),
    (0x057BC0, i_057bc0),
    (0x057BC6, i_057bc6),
    (0x057BCA, i_057bca),
    (0x057BCE, i_057bce),
    (0x057BD2, i_057bd2),
    (0x057BD6, i_057bd6),
    (0x057BDC, i_057bdc),
    (0x057BE0, i_057be0),
    (0x057BE4, i_057be4),
    (0x057BE8, i_057be8),
    (0x057BEA, i_057bea),
    (0x057BEC, i_057bec),
    (0x057BF0, i_057bf0),
    (0x057BF4, i_057bf4),
    (0x057BF8, i_057bf8),
    (0x057BFA, i_057bfa),
    (0x057BFE, i_057bfe),
    (0x057C02, i_057c02),
    (0x057C04, i_057c04),
    (0x057C08, i_057c08),
    (0x057C0A, i_057c0a),
    (0x057C0E, i_057c0e),
    (0x057C12, i_057c12),
    (0x057C16, i_057c16),
    (0x057C18, i_057c18),
    (0x057C1C, i_057c1c),
    (0x057C1E, i_057c1e),
    (0x057C22, i_057c22),
    (0x057C26, i_057c26),
    (0x057C2A, i_057c2a),
    (0x057C30, i_057c30),
    (0x057C34, i_057c34),
    (0x057C38, i_057c38),
    (0x057C3A, i_057c3a),
    (0x057C3E, i_057c3e),
    (0x057C42, i_057c42),
    (0x057C46, i_057c46),
    (0x057C4A, i_057c4a),
    (0x057C4E, i_057c4e),
    (0x057C52, i_057c52),
    (0x057C56, i_057c56),
    (0x057C5A, i_057c5a),
    (0x057C5E, i_057c5e),
    (0x057C64, i_057c64),
    (0x057C68, i_057c68),
    (0x057C6C, i_057c6c),
    (0x057C70, i_057c70),
    (0x057C74, i_057c74),
    (0x057C76, i_057c76),
    (0x057C7A, i_057c7a),
    (0x057C7E, i_057c7e),
    (0x057C82, i_057c82),
    (0x057C86, i_057c86),
    (0x057C88, i_057c88),
    (0x057C8C, i_057c8c),
    (0x057C90, i_057c90),
    (0x057C94, i_057c94),
    (0x057C96, i_057c96),
    (0x057C9A, i_057c9a),
    (0x057C9E, i_057c9e),
    (0x057CA2, i_057ca2),
    (0x057CA4, i_057ca4),
    (0x057CA8, i_057ca8),
    (0x057CAE, i_057cae),
    (0x057CB2, i_057cb2),
    (0x057CB8, i_057cb8),
    (0x057CBC, i_057cbc),
    (0x057CBE, i_057cbe),
    (0x057CC0, i_057cc0),
    (0x057CC4, i_057cc4),
    (0x057CC8, i_057cc8),
    (0x057CCE, i_057cce),
    (0x057CD2, i_057cd2),
    (0x057CD6, i_057cd6),
    (0x057CDA, i_057cda),
    (0x057CDE, i_057cde),
    (0x057CE2, i_057ce2),
    (0x057CE6, i_057ce6),
    (0x057CEA, i_057cea),
    (0x057CEE, i_057cee),
    (0x057CF2, i_057cf2),
    (0x057CF4, i_057cf4),
    (0x057CF8, i_057cf8),
    (0x057CFC, i_057cfc),
    (0x057D00, i_057d00),
    (0x057D02, i_057d02),
    (0x057D04, i_057d04),
    (0x057D0A, i_057d0a),
    (0x057D0E, i_057d0e),
    (0x057D12, i_057d12),
    (0x057D18, i_057d18),
    (0x057D1C, i_057d1c),
    (0x057D22, i_057d22),
    (0x057D28, i_057d28),
    (0x057D2C, i_057d2c),
    (0x057D30, i_057d30),
    (0x057D32, i_057d32),
    (0x057D36, i_057d36),
    (0x057D3A, i_057d3a),
    (0x057D3E, i_057d3e),
    (0x057D42, i_057d42),
    (0x057D44, i_057d44),
    (0x057D4A, i_057d4a),
    (0x057D4E, i_057d4e),
    (0x057D52, i_057d52),
    (0x057D54, i_057d54),
    (0x057D58, i_057d58),
    (0x057D5C, i_057d5c),
    (0x057D60, i_057d60),
    (0x057D64, i_057d64),
    (0x057D6A, i_057d6a),
    (0x057D6E, i_057d6e),
    (0x057D74, i_057d74),
    (0x057D78, i_057d78),
    (0x057D7C, i_057d7c),
    (0x057D82, i_057d82),
    (0x057D86, i_057d86),
    (0x057D8C, i_057d8c),
    (0x057D92, i_057d92),
    (0x057D98, i_057d98),
    (0x057D9C, i_057d9c),
    (0x057DA2, i_057da2),
    (0x057DA6, i_057da6),
    (0x057DAA, i_057daa),
    (0x057DB0, i_057db0),
    (0x057DB6, i_057db6),
    (0x057DBA, i_057dba),
    (0x057DC0, i_057dc0),
    (0x057DC4, i_057dc4),
    (0x057DC8, i_057dc8),
    (0x057DCE, i_057dce),
    (0x057DD4, i_057dd4),
    (0x057DD8, i_057dd8),
    (0x057DDE, i_057dde),
    (0x057DE2, i_057de2),
    (0x057DE4, i_057de4),
    (0x057DE8, i_057de8),
    (0x057DEC, i_057dec),
    (0x057DF0, i_057df0),
    (0x057DF4, i_057df4),
    (0x057DF6, i_057df6),
    (0x057DFC, i_057dfc),
    (0x057E00, i_057e00),
    (0x057E04, i_057e04),
    (0x057E06, i_057e06),
    (0x057E0A, i_057e0a),
    (0x057E10, i_057e10),
    (0x057E14, i_057e14),
    (0x057E18, i_057e18),
    (0x057E1E, i_057e1e),
    (0x057E22, i_057e22),
    (0x057E26, i_057e26),
    (0x057E2C, i_057e2c),
    (0x057E32, i_057e32),
    (0x057E38, i_057e38),
    (0x057E3E, i_057e3e),
    (0x057E42, i_057e42),
    (0x057E44, i_057e44),
    (0x057E48, i_057e48),
    (0x057E4E, i_057e4e),
    (0x057E52, i_057e52),
    (0x057E56, i_057e56),
    (0x057E5A, i_057e5a),
    (0x057E5C, i_057e5c),
    (0x057E62, i_057e62),
    (0x057E66, i_057e66),
    (0x057E6C, i_057e6c),
    (0x057E70, i_057e70),
    (0x057E74, i_057e74),
    (0x057E76, i_057e76),
    (0x057E7A, i_057e7a),
    (0x057E7E, i_057e7e),
    (0x057E82, i_057e82),
    (0x057E86, i_057e86),
    (0x057E8A, i_057e8a),
    (0x057E8E, i_057e8e),
    (0x057E94, i_057e94),
    (0x057E98, i_057e98),
    (0x057E9E, i_057e9e),
    (0x057EA4, i_057ea4),
    (0x057EAA, i_057eaa),
    (0x057EB0, i_057eb0),
    (0x057EB4, i_057eb4),
    (0x057EB6, i_057eb6),
    (0x057EBA, i_057eba),
    (0x057EC0, i_057ec0),
    (0x057EC4, i_057ec4),
    (0x057EC8, i_057ec8),
    (0x057ECA, i_057eca),
    (0x057ECE, i_057ece),
    (0x057ED6, i_057ed6),
    (0x057EDE, i_057ede),
    (0x057EE2, i_057ee2),
    (0x057EEA, i_057eea),
    (0x057EF2, i_057ef2),
    (0x057EF6, i_057ef6),
    (0x057EFE, i_057efe),
    (0x057F06, i_057f06),
    (0x057F0A, i_057f0a),
    (0x057F10, i_057f10),
    (0x057F14, i_057f14),
    (0x057F16, i_057f16),
    (0x057F1A, i_057f1a),
    (0x057F20, i_057f20),
    (0x057F24, i_057f24),
    (0x057F28, i_057f28),
    (0x057F2C, i_057f2c),
    (0x057F30, i_057f30),
    (0x057F32, i_057f32),
    (0x057F36, i_057f36),
    (0x057F3A, i_057f3a),
    (0x057F40, i_057f40),
    (0x057F44, i_057f44),
    (0x057F48, i_057f48),
    (0x057F4A, i_057f4a),
    (0x057F4E, i_057f4e),
    (0x057F54, i_057f54),
    (0x057F5A, i_057f5a),
    (0x057F5E, i_057f5e),
    (0x057F62, i_057f62),
    (0x057F66, i_057f66),
    (0x057F6A, i_057f6a),
    (0x057F70, i_057f70),
    (0x057F74, i_057f74),
    (0x057F7A, i_057f7a),
    (0x057F7E, i_057f7e),
    (0x057F84, i_057f84),
    (0x057F88, i_057f88),
    (0x057F8E, i_057f8e),
    (0x057F94, i_057f94),
    (0x057F9A, i_057f9a),
    (0x057F9E, i_057f9e),
    (0x057FA0, i_057fa0),
    (0x057FA6, i_057fa6),
    (0x057FAA, i_057faa),
    (0x057FAE, i_057fae),
    (0x057FB2, i_057fb2),
    (0x057FB4, i_057fb4),
    (0x057FBA, i_057fba),
    (0x057FC0, i_057fc0),
    (0x057FC4, i_057fc4),
    (0x057FC6, i_057fc6),
    (0x057FCA, i_057fca),
    (0x057FCE, i_057fce),
    (0x057FD4, i_057fd4),
    (0x057FD8, i_057fd8),
    (0x057FDC, i_057fdc),
    (0x057FE0, i_057fe0),
    (0x057FE4, i_057fe4),
    (0x057FEA, i_057fea),
    (0x057FEE, i_057fee),
    (0x057FF4, i_057ff4),
    (0x057FFA, i_057ffa),
    (0x058000, i_058000),
    (0x058004, i_058004),
    (0x05800A, i_05800a),
    (0x05800E, i_05800e),
    (0x058014, i_058014),
    (0x05801A, i_05801a),
    (0x058020, i_058020),
    (0x058024, i_058024),
    (0x058026, i_058026),
    (0x05802C, i_05802c),
    (0x058030, i_058030),
    (0x058034, i_058034),
    (0x058038, i_058038),
    (0x05803A, i_05803a),
    (0x058040, i_058040),
    (0x058046, i_058046),
    (0x05804A, i_05804a),
    (0x05804C, i_05804c),
    (0x058052, i_058052),
    (0x058056, i_058056),
    (0x05805C, i_05805c),
    (0x058062, i_058062),
    (0x058068, i_058068),
    (0x05806C, i_05806c),
    (0x058070, i_058070),
    (0x058072, i_058072),
    (0x058076, i_058076),
    (0x05807C, i_05807c),
    (0x058082, i_058082),
    (0x058088, i_058088),
    (0x05808E, i_05808e),
    (0x058090, i_058090),
    (0x058094, i_058094),
    (0x058098, i_058098),
    (0x05809C, i_05809c),
    (0x05809E, i_05809e),
    (0x0580A2, i_0580a2),
    (0x0580A6, i_0580a6),
    (0x0580AA, i_0580aa),
    (0x0580AC, i_0580ac),
    (0x0580B0, i_0580b0),
    (0x0580B6, i_0580b6),
    (0x0580BA, i_0580ba),
    (0x0580BE, i_0580be),
    (0x0580C0, i_0580c0),
    (0x0580C4, i_0580c4),
    (0x0580CA, i_0580ca),
    (0x0580CC, i_0580cc),
    (0x0580D0, i_0580d0),
    (0x0580D4, i_0580d4),
    (0x0580D8, i_0580d8),
    (0x0580DA, i_0580da),
    (0x0580DE, i_0580de),
    (0x0580E0, i_0580e0),
    (0x0580E4, i_0580e4),
    (0x0580E8, i_0580e8),
    (0x0580EA, i_0580ea),
    (0x0580EE, i_0580ee),
    (0x0580F2, i_0580f2),
    (0x0580F6, i_0580f6),
    (0x0580F8, i_0580f8),
    (0x0580FC, i_0580fc),
    (0x058100, i_058100),
    (0x058104, i_058104),
    (0x058106, i_058106),
    (0x05810C, i_05810c),
    (0x05810E, i_05810e),
    (0x058112, i_058112),
    (0x058116, i_058116),
    (0x05811A, i_05811a),
    (0x05811C, i_05811c),
    (0x058120, i_058120),
    (0x058122, i_058122),
    (0x058126, i_058126),
    (0x05812A, i_05812a),
    (0x05812C, i_05812c),
    (0x058130, i_058130),
    (0x058134, i_058134),
    (0x058138, i_058138),
    (0x05813A, i_05813a),
    (0x058140, i_058140),
    (0x058144, i_058144),
    (0x058148, i_058148),
    (0x05814C, i_05814c),
    (0x058152, i_058152),
    (0x058158, i_058158),
    (0x05815C, i_05815c),
    (0x05815E, i_05815e),
    (0x058164, i_058164),
    (0x058166, i_058166),
    (0x058168, i_058168),
    (0x05816C, i_05816c),
    (0x058172, i_058172),
    (0x058178, i_058178),
    (0x05817C, i_05817c),
    (0x058182, i_058182),
    (0x058188, i_058188),
    (0x05818C, i_05818c),
    (0x058190, i_058190),
    (0x058194, i_058194),
    (0x058196, i_058196),
    (0x05819A, i_05819a),
    (0x05819E, i_05819e),
    (0x0581A4, i_0581a4),
    (0x0581A8, i_0581a8),
    (0x0581AE, i_0581ae),
    (0x0581B2, i_0581b2),
    (0x0581B6, i_0581b6),
    (0x0581BA, i_0581ba),
    (0x0581BE, i_0581be),
    (0x0581C2, i_0581c2),
    (0x0581C8, i_0581c8),
    (0x0581CA, i_0581ca),
    (0x0581CE, i_0581ce),
    (0x0581D0, i_0581d0),
    (0x0581D4, i_0581d4),
    (0x0581D6, i_0581d6),
    (0x0581D8, i_0581d8),
    (0x0581DA, i_0581da),
    (0x0581DE, i_0581de),
    (0x0581E2, i_0581e2),
    (0x0581E4, i_0581e4),
    (0x0581E6, i_0581e6),
    (0x0581E8, i_0581e8),
    (0x0581EE, i_0581ee),
    (0x0581F0, i_0581f0),
    (0x0581F4, i_0581f4),
    (0x0581F6, i_0581f6),
    (0x0581FA, i_0581fa),
    (0x0581FC, i_0581fc),
    (0x0581FE, i_0581fe),
    (0x058200, i_058200),
    (0x058204, i_058204),
    (0x058208, i_058208),
    (0x05820A, i_05820a),
    (0x05820C, i_05820c),
    (0x05820E, i_05820e),
    (0x058214, i_058214),
    (0x058216, i_058216),
    (0x05821A, i_05821a),
    (0x05821C, i_05821c),
    (0x058220, i_058220),
    (0x058222, i_058222),
    (0x058224, i_058224),
    (0x058226, i_058226),
    (0x05822A, i_05822a),
    (0x05822E, i_05822e),
    (0x058230, i_058230),
    (0x058234, i_058234),
    (0x058238, i_058238),
    (0x05823C, i_05823c),
    (0x058240, i_058240),
    (0x058246, i_058246),
    (0x058248, i_058248),
    (0x05824C, i_05824c),
    (0x05824E, i_05824e),
    (0x058252, i_058252),
    (0x058254, i_058254),
    (0x058256, i_058256),
    (0x058258, i_058258),
    (0x05825C, i_05825c),
    (0x058260, i_058260),
    (0x058262, i_058262),
    (0x058264, i_058264),
    (0x058266, i_058266),
    (0x05826C, i_05826c),
    (0x05826E, i_05826e),
    (0x058272, i_058272),
    (0x058274, i_058274),
    (0x058278, i_058278),
    (0x05827A, i_05827a),
    (0x05827C, i_05827c),
    (0x05827E, i_05827e),
    (0x058282, i_058282),
    (0x058286, i_058286),
    (0x058288, i_058288),
    (0x05828C, i_05828c),
    (0x058292, i_058292),
    (0x058294, i_058294),
    (0x058298, i_058298),
    (0x05829C, i_05829c),
    (0x0582A0, i_0582a0),
    (0x0582A2, i_0582a2),
    (0x0582A6, i_0582a6),
    (0x0582A8, i_0582a8),
    (0x0582AC, i_0582ac),
    (0x0582B0, i_0582b0),
    (0x0582B2, i_0582b2),
    (0x0582B8, i_0582b8),
    (0x0582BA, i_0582ba),
    (0x0582BE, i_0582be),
    (0x0582C2, i_0582c2),
    (0x0582C6, i_0582c6),
    (0x0582C8, i_0582c8),
    (0x0582CC, i_0582cc),
    (0x0582CE, i_0582ce),
    (0x0582D2, i_0582d2),
    (0x0582D6, i_0582d6),
    (0x0582D8, i_0582d8),
    (0x0582DE, i_0582de),
    (0x0582E0, i_0582e0),
    (0x0582E4, i_0582e4),
    (0x0582E8, i_0582e8),
    (0x0582EC, i_0582ec),
    (0x0582EE, i_0582ee),
    (0x0582F2, i_0582f2),
    (0x0582F4, i_0582f4),
    (0x0582F8, i_0582f8),
    (0x0582FC, i_0582fc),
    (0x0582FE, i_0582fe),
    (0x058304, i_058304),
    (0x058306, i_058306),
    (0x05830A, i_05830a),
    (0x05830E, i_05830e),
    (0x058312, i_058312),
    (0x058314, i_058314),
    (0x058318, i_058318),
    (0x05831A, i_05831a),
    (0x05831E, i_05831e),
    (0x058322, i_058322),
    (0x058324, i_058324),
    (0x058328, i_058328),
    (0x05832C, i_05832c),
    (0x058332, i_058332),
    (0x058334, i_058334),
    (0x058338, i_058338),
    (0x05833C, i_05833c),
    (0x058340, i_058340),
    (0x058342, i_058342),
    (0x058346, i_058346),
    (0x058348, i_058348),
    (0x05834C, i_05834c),
    (0x058350, i_058350),
    (0x058352, i_058352),
    (0x058356, i_058356),
    (0x05835A, i_05835a),
    (0x05835E, i_05835e),
    (0x058360, i_058360),
    (0x058364, i_058364),
    (0x058368, i_058368),
    (0x05836C, i_05836c),
    (0x05836E, i_05836e),
    (0x058374, i_058374),
    (0x058378, i_058378),
    (0x05837C, i_05837c),
    (0x05837E, i_05837e),
    (0x058384, i_058384),
    (0x058386, i_058386),
    (0x05838A, i_05838a),
    (0x05838E, i_05838e),
    (0x058392, i_058392),
    (0x058394, i_058394),
    (0x058398, i_058398),
    (0x05839A, i_05839a),
    (0x05839E, i_05839e),
    (0x0583A2, i_0583a2),
    (0x0583A4, i_0583a4),
    (0x0583A8, i_0583a8),
    (0x0583AC, i_0583ac),
    (0x0583B0, i_0583b0),
    (0x0583B2, i_0583b2),
    (0x0583B6, i_0583b6),
    (0x0583BA, i_0583ba),
    (0x0583BE, i_0583be),
    (0x0583C0, i_0583c0),
    (0x0583C6, i_0583c6),
    (0x0583CA, i_0583ca),
    (0x0583CC, i_0583cc),
    (0x0583D2, i_0583d2),
    (0x0583D6, i_0583d6),
    (0x0583DA, i_0583da),
    (0x0583E0, i_0583e0),
    (0x0583E6, i_0583e6),
    (0x0583EA, i_0583ea),
    (0x0583EC, i_0583ec),
    (0x0583F2, i_0583f2),
    (0x0583F6, i_0583f6),
    (0x0583FC, i_0583fc),
    (0x058400, i_058400),
    (0x058404, i_058404),
    (0x058406, i_058406),
    (0x05840C, i_05840c),
    (0x058410, i_058410),
    (0x058412, i_058412),
    (0x058418, i_058418),
    (0x05841E, i_05841e),
    (0x058422, i_058422),
    (0x058424, i_058424),
    (0x058428, i_058428),
    (0x05842A, i_05842a),
    (0x05842E, i_05842e),
    (0x058434, i_058434),
    (0x058438, i_058438),
    (0x05843C, i_05843c),
    (0x058440, i_058440),
    (0x058442, i_058442),
    (0x058448, i_058448),
    (0x05844E, i_05844e),
    (0x058452, i_058452),
    (0x058454, i_058454),
    (0x05845A, i_05845a),
    (0x058460, i_058460),
    (0x058464, i_058464),
    (0x058468, i_058468),
    (0x05846C, i_05846c),
    (0x05846E, i_05846e),
    (0x058472, i_058472),
    (0x058476, i_058476),
    (0x058478, i_058478),
    (0x05847A, i_05847a),
    (0x05847E, i_05847e),
    (0x058484, i_058484),
    (0x05848A, i_05848a),
    (0x05848E, i_05848e),
    (0x058492, i_058492),
    (0x058494, i_058494),
    (0x058498, i_058498),
    (0x05849E, i_05849e),
    (0x0584A2, i_0584a2),
    (0x0584A6, i_0584a6),
    (0x0584AA, i_0584aa),
    (0x0584AE, i_0584ae),
    (0x0584B0, i_0584b0),
    (0x0584B4, i_0584b4),
    (0x0584B8, i_0584b8),
    (0x0584BA, i_0584ba),
    (0x0584C0, i_0584c0),
    (0x0584C4, i_0584c4),
    (0x0584C8, i_0584c8),
    (0x0584CC, i_0584cc),
    (0x0584D2, i_0584d2),
    (0x0584D8, i_0584d8),
    (0x0584DC, i_0584dc),
    (0x0584E2, i_0584e2),
    (0x0584E8, i_0584e8),
    (0x0584EE, i_0584ee),
    (0x0584F2, i_0584f2),
    (0x0584F6, i_0584f6),
    (0x0584FA, i_0584fa),
    (0x0584FC, i_0584fc),
    (0x058500, i_058500),
    (0x058504, i_058504),
    (0x058508, i_058508),
    (0x05850E, i_05850e),
    (0x058514, i_058514),
    (0x058518, i_058518),
    (0x05851E, i_05851e),
    (0x058524, i_058524),
    (0x05852A, i_05852a),
    (0x05852E, i_05852e),
    (0x058532, i_058532),
    (0x058536, i_058536),
    (0x058538, i_058538),
    (0x05853C, i_05853c),
    (0x058540, i_058540),
    (0x058546, i_058546),
    (0x05854A, i_05854a),
    (0x05854E, i_05854e),
    (0x058550, i_058550),
    (0x058554, i_058554),
    (0x05855A, i_05855a),
    (0x05855C, i_05855c),
    (0x058560, i_058560),
    (0x058564, i_058564),
    (0x058568, i_058568),
    (0x05856A, i_05856a),
    (0x05856E, i_05856e),
    (0x058570, i_058570),
    (0x058574, i_058574),
    (0x058578, i_058578),
    (0x05857A, i_05857a),
    (0x05857E, i_05857e),
    (0x058582, i_058582),
    (0x058586, i_058586),
    (0x058588, i_058588),
    (0x05858C, i_05858c),
    (0x058590, i_058590),
    (0x058594, i_058594),
    (0x058596, i_058596),
    (0x05859A, i_05859a),
    (0x05859E, i_05859e),
    (0x0585A2, i_0585a2),
    (0x0585A4, i_0585a4),
    (0x0585AA, i_0585aa),
    (0x0585AE, i_0585ae),
    (0x0585B2, i_0585b2),
    (0x0585B8, i_0585b8),
    (0x0585BE, i_0585be),
    (0x0585C2, i_0585c2),
    (0x0585C6, i_0585c6),
    (0x0585CC, i_0585cc),
    (0x0585D2, i_0585d2),
    (0x0585D6, i_0585d6),
    (0x0585D8, i_0585d8),
    (0x0585DC, i_0585dc),
    (0x0585E2, i_0585e2),
    (0x0585E6, i_0585e6),
    (0x0585EA, i_0585ea),
    (0x0585EC, i_0585ec),
    (0x0585F2, i_0585f2),
    (0x0585F6, i_0585f6),
    (0x0585FC, i_0585fc),
    (0x058602, i_058602),
    (0x058606, i_058606),
    (0x058608, i_058608),
    (0x05860C, i_05860c),
    (0x058610, i_058610),
    (0x058612, i_058612),
    (0x058616, i_058616),
    (0x05861A, i_05861a),
    (0x058620, i_058620),
    (0x058624, i_058624),
    (0x05862A, i_05862a),
    (0x05862E, i_05862e),
    (0x058632, i_058632),
    (0x058634, i_058634),
    (0x058638, i_058638),
    (0x05863C, i_05863c),
    (0x05863E, i_05863e),
    (0x058642, i_058642),
    (0x058646, i_058646),
    (0x05864A, i_05864a),
    (0x05864C, i_05864c),
    (0x058652, i_058652),
    (0x058656, i_058656),
    (0x058658, i_058658),
    (0x05865E, i_05865e),
    (0x058662, i_058662),
    (0x058668, i_058668),
    (0x05866E, i_05866e),
    (0x058672, i_058672),
    (0x058674, i_058674),
    (0x058678, i_058678),
    (0x05867C, i_05867c),
    (0x058680, i_058680),
    (0x058682, i_058682),
    (0x058686, i_058686),
    (0x058688, i_058688),
    (0x05868C, i_05868c),
    (0x058690, i_058690),
    (0x058694, i_058694),
    (0x05869A, i_05869a),
    (0x05869E, i_05869e),
    (0x0586A2, i_0586a2),
    (0x0586A6, i_0586a6),
    (0x0586AA, i_0586aa),
    (0x0586AC, i_0586ac),
    (0x0586B2, i_0586b2),
    (0x0586B6, i_0586b6),
    (0x0586B8, i_0586b8),
    (0x0586BE, i_0586be),
    (0x0586C4, i_0586c4),
    (0x0586C8, i_0586c8),
    (0x0586CC, i_0586cc),
    (0x0586CE, i_0586ce),
    (0x0586D2, i_0586d2),
    (0x0586D6, i_0586d6),
    (0x0586DC, i_0586dc),
    (0x0586E0, i_0586e0),
    (0x0586E2, i_0586e2),
    (0x0586E6, i_0586e6),
    (0x0586EC, i_0586ec),
    (0x0586EE, i_0586ee),
    (0x0586F2, i_0586f2),
    (0x0586F4, i_0586f4),
    (0x0586F8, i_0586f8),
    (0x0586FA, i_0586fa),
    (0x0586FC, i_0586fc),
    (0x0586FE, i_0586fe),
    (0x058702, i_058702),
    (0x058706, i_058706),
    (0x058708, i_058708),
    (0x05870E, i_05870e),
    (0x058712, i_058712),
    (0x058718, i_058718),
    (0x05871A, i_05871a),
    (0x05871E, i_05871e),
    (0x058722, i_058722),
    (0x058726, i_058726),
    (0x058728, i_058728),
    (0x05872C, i_05872c),
    (0x05872E, i_05872e),
    (0x058732, i_058732),
    (0x058736, i_058736),
    (0x058738, i_058738),
    (0x05873C, i_05873c),
    (0x058742, i_058742),
    (0x058744, i_058744),
    (0x058748, i_058748),
    (0x05874A, i_05874a),
    (0x05874E, i_05874e),
    (0x058750, i_058750),
    (0x058752, i_058752),
    (0x058754, i_058754),
    (0x058758, i_058758),
    (0x05875C, i_05875c),
    (0x05875E, i_05875e),
    (0x058762, i_058762),
    (0x058766, i_058766),
    (0x058768, i_058768),
    (0x05876C, i_05876c),
    (0x058770, i_058770),
    (0x058774, i_058774),
    (0x058776, i_058776),
    (0x05877A, i_05877a),
    (0x05877E, i_05877e),
    (0x058782, i_058782),
    (0x058784, i_058784),
    (0x058788, i_058788),
    (0x05878C, i_05878c),
    (0x058790, i_058790),
    (0x058792, i_058792),
    (0x058798, i_058798),
    (0x05879C, i_05879c),
    (0x05879E, i_05879e),
    (0x0587A4, i_0587a4),
    (0x0587A8, i_0587a8),
    (0x0587AC, i_0587ac),
    (0x0587B2, i_0587b2),
    (0x0587B6, i_0587b6),
    (0x0587BC, i_0587bc),
    (0x0587C2, i_0587c2),
    (0x0587C6, i_0587c6),
    (0x0587CC, i_0587cc),
    (0x0587D0, i_0587d0),
    (0x0587D2, i_0587d2),
    (0x0587D6, i_0587d6),
    (0x0587DA, i_0587da),
    (0x0587DE, i_0587de),
    (0x0587E0, i_0587e0),
    (0x0587E4, i_0587e4),
    (0x0587EA, i_0587ea),
    (0x0587F0, i_0587f0),
    (0x0587F6, i_0587f6),
    (0x0587FA, i_0587fa),
    (0x0587FE, i_0587fe),
    (0x058800, i_058800),
    (0x058806, i_058806),
    (0x05880A, i_05880a),
    (0x05880E, i_05880e),
    (0x058810, i_058810),
    (0x058816, i_058816),
    (0x05881A, i_05881a),
    (0x05881E, i_05881e),
    (0x058822, i_058822),
    (0x058826, i_058826),
    (0x05882A, i_05882a),
    (0x05882C, i_05882c),
    (0x058832, i_058832),
    (0x058836, i_058836),
    (0x05883A, i_05883a),
    (0x05883C, i_05883c),
    (0x058840, i_058840),
    (0x058842, i_058842),
    (0x058846, i_058846),
    (0x05884C, i_05884c),
    (0x05884E, i_05884e),
    (0x058852, i_058852),
    (0x058854, i_058854),
    (0x058858, i_058858),
    (0x05885A, i_05885a),
    (0x05885C, i_05885c),
    (0x05885E, i_05885e),
    (0x058862, i_058862),
    (0x058866, i_058866),
    (0x058868, i_058868),
    (0x05886E, i_05886e),
    (0x058872, i_058872),
    (0x058876, i_058876),
    (0x058878, i_058878),
    (0x05887C, i_05887c),
    (0x058880, i_058880),
    (0x058884, i_058884),
    (0x058886, i_058886),
    (0x05888A, i_05888a),
    (0x05888E, i_05888e),
    (0x058892, i_058892),
    (0x058894, i_058894),
    (0x058898, i_058898),
    (0x05889C, i_05889c),
    (0x0588A0, i_0588a0),
    (0x0588A2, i_0588a2),
    (0x0588A8, i_0588a8),
    (0x0588AC, i_0588ac),
    (0x0588AE, i_0588ae),
    (0x0588B4, i_0588b4),
    (0x0588BA, i_0588ba),
    (0x0588BE, i_0588be),
    (0x0588C2, i_0588c2),
    (0x0588C4, i_0588c4),
    (0x0588C8, i_0588c8),
    (0x0588CC, i_0588cc),
    (0x0588D2, i_0588d2),
    (0x0588D6, i_0588d6),
    (0x0588DA, i_0588da),
    (0x0588DC, i_0588dc),
    (0x0588E0, i_0588e0),
    (0x0588E4, i_0588e4),
    (0x0588E8, i_0588e8),
    (0x0588EA, i_0588ea),
    (0x0588F0, i_0588f0),
    (0x0588F4, i_0588f4),
    (0x0588F6, i_0588f6),
    (0x0588FA, i_0588fa),
    (0x0588FE, i_0588fe),
    (0x058902, i_058902),
    (0x058906, i_058906),
    (0x058908, i_058908),
    (0x05890E, i_05890e),
    (0x058910, i_058910),
    (0x058912, i_058912),
    (0x058914, i_058914),
    (0x058918, i_058918),
    (0x05891A, i_05891a),
    (0x05891E, i_05891e),
    (0x058924, i_058924),
    (0x05892A, i_05892a),
    (0x05892E, i_05892e),
    (0x058930, i_058930),
    (0x058934, i_058934),
    (0x058938, i_058938),
    (0x05893A, i_05893a),
    (0x05893E, i_05893e),
    (0x058944, i_058944),
    (0x05894A, i_05894a),
    (0x05894E, i_05894e),
    (0x058950, i_058950),
    (0x058954, i_058954),
    (0x05895A, i_05895a),
    (0x05895E, i_05895e),
    (0x058962, i_058962),
    (0x058964, i_058964),
    (0x058968, i_058968),
];
