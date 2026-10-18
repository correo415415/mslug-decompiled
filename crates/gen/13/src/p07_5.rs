// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $07A6FC: lsl.w #$1, d1
pub fn i_07a6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6FE;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A6FE: lea.l $7a7f4(pc), a1
pub fn i_07a6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A702;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A702: move.w (a1, d1.w), d1
pub fn i_07a702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A706;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A706: add.w d1, d0
pub fn i_07a706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A708;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A708: jsr $219c.l
pub fn i_07a708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A70E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A70E);
    cpu.pc = 0x219C;
    16
}
/// $07A70E: move.l (a7)+, d0
pub fn i_07a70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A710;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A710: btst.b #$7, d0
pub fn i_07a710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A714;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A714: beq.w $7a736
pub fn i_07a714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A718;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A736; return 10; }
    8
}
/// $07A718: move.l d0, -(a7)
pub fn i_07a718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A71A;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07A71A: move.w $76(a6), d0
pub fn i_07a71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A71E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A71E: move.w $7e(a6), d1
pub fn i_07a71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A722;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A722: lsl.w #$1, d1
pub fn i_07a722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A724;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A724: lea.l $7a7f4(pc), a1
pub fn i_07a724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A728;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A728: move.w (a1, d1.w), d1
pub fn i_07a728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A72C;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A72C: add.w d1, d0
pub fn i_07a72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A72E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A72E: jsr $219c.l
pub fn i_07a72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A734);
    cpu.pc = 0x219C;
    16
}
/// $07A734: move.l (a7)+, d0
pub fn i_07a734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A736;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A736: move.w $70(a6), d0
pub fn i_07a736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A73A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A73A: move.w $78(a6), d1
pub fn i_07a73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A73E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A73E: lsl.w #$1, d1
pub fn i_07a73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A740;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A740: lea.l $7a7f4(pc), a1
pub fn i_07a740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A744;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A744: move.w (a1, d1.w), d1
pub fn i_07a744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A748;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A748: add.w d1, d0
pub fn i_07a748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A74A;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A74A: movea.l #$724a, a1
pub fn i_07a74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A750;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724A;
    cpu.a[1] = v1;
    4
}
/// $07A750: move.w #$2300, d1
pub fn i_07a750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A754;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A754: jsr $5d6c2.l
pub fn i_07a754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A75A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A75A);
    cpu.pc = 0x5D6C2;
    16
}
/// $07A75A: move.w $72(a6), d0
pub fn i_07a75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A75E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A75E: move.w $7a(a6), d1
pub fn i_07a75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A762;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A762: lsl.w #$1, d1
pub fn i_07a762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A764;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A764: lea.l $7a7f4(pc), a1
pub fn i_07a764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A768: move.w (a1, d1.w), d1
pub fn i_07a768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A76C;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A76C: add.w d1, d0
pub fn i_07a76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A76E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A76E: movea.l #$724c, a1
pub fn i_07a76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A774;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724C;
    cpu.a[1] = v1;
    4
}
/// $07A774: move.w #$2300, d1
pub fn i_07a774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A778;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A778: jsr $5d6c2.l
pub fn i_07a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A77E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A77E);
    cpu.pc = 0x5D6C2;
    16
}
/// $07A77E: move.w $74(a6), d0
pub fn i_07a77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A782;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A782: move.w $7c(a6), d1
pub fn i_07a782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A786;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A786: lsl.w #$1, d1
pub fn i_07a786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A788;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A788: lea.l $7a7f4(pc), a1
pub fn i_07a788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A78C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A78C: move.w (a1, d1.w), d1
pub fn i_07a78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A790;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A790: add.w d1, d0
pub fn i_07a790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A792;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A792: movea.l #$724e, a1
pub fn i_07a792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A798;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724E;
    cpu.a[1] = v1;
    4
}
/// $07A798: move.w #$2300, d1
pub fn i_07a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A79C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A79C: jsr $5d6c2.l
pub fn i_07a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A7A2);
    cpu.pc = 0x5D6C2;
    16
}
/// $07A7A2: move.w $76(a6), d0
pub fn i_07a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7A6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A7A6: move.w $7e(a6), d1
pub fn i_07a7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7AA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A7AA: lsl.w #$1, d1
pub fn i_07a7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7AC;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A7AC: lea.l $7a7f4(pc), a1
pub fn i_07a7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A7B0: move.w (a1, d1.w), d1
pub fn i_07a7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7B4;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A7B4: add.w d1, d0
pub fn i_07a7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7B6;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A7B6: movea.l #$7250, a1
pub fn i_07a7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7BC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7250;
    cpu.a[1] = v1;
    4
}
/// $07A7BC: move.w #$2300, d1
pub fn i_07a7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A7C0: jsr $5d6c2.l
pub fn i_07a7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A7C6);
    cpu.pc = 0x5D6C2;
    16
}
/// $07A7C6: lea.l $7a914(pc), a0
pub fn i_07a7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7CA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7A914;
    16
}
/// $07A7CA: move.w $80(a6), d0
pub fn i_07a7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A7CE: add.w d0, d0
pub fn i_07a7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7D0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A7D0: add.w d0, d0
pub fn i_07a7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7D2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A7D2: movea.l (a0, d0.w), a2
pub fn i_07a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7D6;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $07A7D6: movea.l #$720a, a1
pub fn i_07a7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7DC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x720A;
    cpu.a[1] = v1;
    4
}
/// $07A7DC: move.w #$2300, d0
pub fn i_07a7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_07a7e0  ($07A7E0, 8 B, thunks/jsr_abs_thunks.c)
/// $07A7E0: jsr $5dad8.l
pub fn i_07a7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A7E6);
    cpu.pc = 0x5DAD8;
    16
}
/// $07A7E6: rts
pub fn i_07a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_RetGate_07a954  ($07A954, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A954: movea.l $8(a6), a1
pub fn i_07a954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A958;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07A958: move.b $10(a6), d0
pub fn i_07a958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A95C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07A95C: cmp.b $10(a1), d0
pub fn i_07a95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A960;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07A960: bcs.w $7a96a
pub fn i_07a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A964;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7A96A; return 10; }
    8
}
// ===== ClearXN_07a964  ($07A964, 6 B, thunks/ccr_helpers.c)
/// $07A964: andi.b #$ee, ccr
pub fn i_07a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A968;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07A968: rts
pub fn i_07a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A96A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07a96a  ($07A96A, 6 B, thunks/ccr_helpers.c)
/// $07A96A: ori.b #$11, ccr
pub fn i_07a96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A96E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07A96E: rts
pub fn i_07a96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A970;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerStart_Boot_07A970  ($07A970, 120 B, mission/banner_07a970.s)
/// $07A970: jsr $4707e.l
pub fn i_07a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A976);
    cpu.pc = 0x4707E;
    16
}
/// $07A976: move.b #$0, $7b(a6)
pub fn i_07a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A97C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A97C: move.w #$0, $22(a6)
pub fn i_07a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A982;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A982: move.w #$1ff, $24(a6)
pub fn i_07a982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A988;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A988: move.w #$ffff, $38(a6)
pub fn i_07a988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A98E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A98E: move.w #$0, $74(a6)
pub fn i_07a98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A994;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A994: move.b $106ece.l, d0
pub fn i_07a994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A99A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A99A: cmpi.b #$5, d0
pub fn i_07a99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A99E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07A99E: bne.w $7a9aa
pub fn i_07a99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A9AA; return 10; }
    8
}
/// $07A9A2: jsr $7af50(pc)
pub fn i_07a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7A9A6);
    cpu.pc = 0x7AF50;
    16
}
/// $07A9A6: bra.w $7a9ae
pub fn i_07a9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A9AE;
    10
}
/// $07A9AA: jsr $7acd0(pc)
pub fn i_07a9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7A9AE);
    cpu.pc = 0x7ACD0;
    16
}
/// $07A9AE: move.b #$0, $21(a6)
pub fn i_07a9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A9B4: move.b #$0, $20(a6)
pub fn i_07a9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9BA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A9BA: lea.l $7a9c0(pc), a1
pub fn i_07a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A9C0;
    16
}
/// $07A9BE: move.l a1, (a6)
pub fn i_07a9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A9C0: move.b $106ece.l, d0
pub fn i_07a9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9C6;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A9C6: cmpi.b #$5, d0
pub fn i_07a9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9CA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07A9CA: bne.w $7a9d6
pub fn i_07a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A9D6; return 10; }
    8
}
/// $07A9CE: move.b #$12, d0
pub fn i_07a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9D2;
    cpu.ir = 0x103C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07A9D2: bra.w $7a9da
pub fn i_07a9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A9DA;
    10
}
/// $07A9D6: move.b #$e, d0
pub fn i_07a9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9DA;
    cpu.ir = 0x103C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07A9DA: cmp.b $21(a6), d0
pub fn i_07a9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9DE;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07A9DE: bne.w $7a9ee
pub fn i_07a9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A9EE; return 10; }
    8
}
/// $07A9E2: jsr $4709e.l
pub fn i_07a9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A9E8);
    cpu.pc = 0x4709E;
    16
}
// ===== SetTaskHandler_07a9e8  ($07A9E8, 8 B, kernel/task_handlers.c)
/// $07A9E8: lea.l $7aa78(pc), a1
pub fn i_07a9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA78;
    16
}
/// $07A9EC: move.l a1, (a6)
pub fn i_07a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A9EE: rts
pub fn i_07a9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerComplete_Boot_07A9F0  ($07A9F0, 128 B, mission/banner_07a970.s)
/// $07A9F0: move.w #$20, d0
pub fn i_07a9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A9F4: jsr $2352.l
pub fn i_07a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A9FA);
    cpu.pc = 0x2352;
    16
}
/// $07A9FA: move.b #$1, $7b(a6)
pub fn i_07a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AA00: move.w #$0, $22(a6)
pub fn i_07aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AA06: move.w #$1ff, $24(a6)
pub fn i_07aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA0C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AA0C: move.w #$ffff, $38(a6)
pub fn i_07aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA12;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AA12: move.w #$0, $74(a6)
pub fn i_07aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA18;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AA18: move.b $106ece.l, d0
pub fn i_07aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA1E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07AA1E: cmpi.b #$5, d0
pub fn i_07aa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA22;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07AA22: bne.w $7aa2e
pub fn i_07aa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AA2E; return 10; }
    8
}
/// $07AA26: jsr $7b598(pc)
pub fn i_07aa26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7AA2A);
    cpu.pc = 0x7B598;
    16
}
/// $07AA2A: bra.w $7aa32
pub fn i_07aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7AA32;
    10
}
/// $07AA2E: jsr $7b28e(pc)
pub fn i_07aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7AA32);
    cpu.pc = 0x7B28E;
    16
}
/// $07AA32: move.b #$0, $21(a6)
pub fn i_07aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA38;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AA38: move.b #$0, $20(a6)
pub fn i_07aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AA3E: lea.l $7aa44(pc), a1
pub fn i_07aa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA44;
    16
}
/// $07AA42: move.l a1, (a6)
pub fn i_07aa42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AA44: move.b $106ece.l, d0
pub fn i_07aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA4A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07AA4A: cmpi.b #$5, d0
pub fn i_07aa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA4E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07AA4E: bne.w $7aa5a
pub fn i_07aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AA5A; return 10; }
    8
}
/// $07AA52: move.b #$15, d0
pub fn i_07aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA56;
    cpu.ir = 0x103C;
    let v1: u32 = 0x15;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07AA56: bra.w $7aa5e
pub fn i_07aa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x7AA5E;
    10
}
/// $07AA5A: move.b #$11, d0
pub fn i_07aa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA5E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07AA5E: cmp.b $21(a6), d0
pub fn i_07aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA62;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07AA62: bne.w $7aa76
pub fn i_07aa62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AA76; return 10; }
    8
}
/// $07AA66: move.w #$1137, d0
pub fn i_07aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1137;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AA6A: jsr $2352.l
pub fn i_07aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AA70);
    cpu.pc = 0x2352;
    16
}
// ===== SetTaskHandler_07aa70  ($07AA70, 8 B, kernel/task_handlers.c)
/// $07AA70: lea.l $7aa78(pc), a1
pub fn i_07aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA78;
    16
}
/// $07AA74: move.l a1, (a6)
pub fn i_07aa74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AA76: rts
pub fn i_07aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_Wait26_07AA78  ($07AA78, 20 B, mission/banner_07a970.s)
/// $07AA78: lea.l $7aa7e(pc), a1
pub fn i_07aa78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA7E;
    16
}
/// $07AA7C: move.l a1, (a6)
pub fn i_07aa7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AA7E: addq.w #$1, $74(a6)
pub fn i_07aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA82;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07AA82: cmpi.w #$26, $74(a6)
pub fn i_07aa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Word, false);
    16
}
/// $07AA88: blt.w $7aa92
pub fn i_07aa88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA8C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7AA92; return 10; }
    8
}
// ===== SetTaskHandler_07aa8c  ($07AA8C, 8 B, kernel/task_handlers.c)
/// $07AA8C: lea.l $7aa94(pc), a1
pub fn i_07aa8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA94;
    16
}
/// $07AA90: move.l a1, (a6)
pub fn i_07aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AA92: rts
pub fn i_07aa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_Close_07AA94  ($07AA94, 76 B, mission/banner_07a970.s)
/// $07AA94: move.b #$ff, $20(a6)
pub fn i_07aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA9A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AA9A: cmpi.b #$0, $7b(a6)
pub fn i_07aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAA0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07AAA0: bne.w $7aab4
pub fn i_07aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AAB4; return 10; }
    8
}
/// $07AAA4: movea.l $c(a6), a1
pub fn i_07aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAA8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07AAA8: clr.b $21(a1)
pub fn i_07aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAAC;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07AAAC: jmp $518.l
pub fn i_07aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAB2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07AAB4: move.w #$2, d0
pub fn i_07aab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAB8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AAB8: jsr $5239e.l
pub fn i_07aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AABE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AABE);
    cpu.pc = 0x5239E;
    16
}
/// $07AABE: move.w #$28, $74(a6)
pub fn i_07aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AAC4: lea.l $7aaca(pc), a1
pub fn i_07aac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AACA;
    16
}
/// $07AAC8: move.l a1, (a6)
pub fn i_07aac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AACA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AACA: subq.w #$1, $74(a6)
pub fn i_07aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AACE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07AACE: bpl.w $7aade
pub fn i_07aace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAD2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7AADE; return 10; }
    8
}
/// $07AAD2: clr.b $106ed2.l
pub fn i_07aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAD8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $07AAD8: jmp $518.l
pub fn i_07aad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AADE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07AADE: rts
pub fn i_07aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_Drop_07AAE0  ($07AAE0, 124 B, mission/banner_07a970.s)
/// $07AAE0: move.w $74(a6), $76(a6)
pub fn i_07aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAE6;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07AAE6: lea.l $7aaec(pc), a1
pub fn i_07aae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAEC;
    16
}
/// $07AAEA: move.l a1, (a6)
pub fn i_07aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AAEC: subq.w #$1, $74(a6)
pub fn i_07aaec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAF0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07AAF0: bmi.w $7aaf6
pub fn i_07aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAF4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x7AAF6; return 10; }
    8
}
/// $07AAF4: rts
pub fn i_07aaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07AAF6: move.w #$1be, d1
pub fn i_07aaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAFA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AAFA: jsr $236e.l
pub fn i_07aafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB00);
    cpu.pc = 0x236E;
    16
}
/// $07AB00: ori.b #$1, $3a(a6)
pub fn i_07ab00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB06;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07AB06: move.w #$1ff, $78(a6)
pub fn i_07ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB0C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AB0C: move.b #$ff, $33(a6)
pub fn i_07ab0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AB12: move.b #$0, $7a(a6)
pub fn i_07ab12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AB18: move.w #$ffff, $38(a6)
pub fn i_07ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AB1E: jsr $267e2.l
pub fn i_07ab1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB24);
    cpu.pc = 0x267E2;
    16
}
/// $07AB24: clr.l d0
pub fn i_07ab24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB26;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $07AB26: move.b $20(a6), d0
pub fn i_07ab26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB2A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07AB2A: cmpi.b #$d, d0
pub fn i_07ab2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB2E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $07AB2E: bls.w $7ab36
pub fn i_07ab2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB32;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x7AB36; return 10; }
    8
}
/// $07AB32: move.b #$0, d0
pub fn i_07ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB36;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07AB36: movea.l #$2df684, a0
pub fn i_07ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB3C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DF684;
    cpu.a[0] = v1;
    4
}
/// $07AB3C: lsl.w #$2, d0
pub fn i_07ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB3E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07AB3E: movea.l (a0, d0.w), a0
pub fn i_07ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB42;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07AB42: cmpa.l #$ffffffff, a0
pub fn i_07ab42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB48;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07AB48: beq.w $7ab52
pub fn i_07ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7AB52; return 10; }
    8
}
/// $07AB4C: jsr $28cd4.l
pub fn i_07ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB52);
    cpu.pc = 0x28CD4;
    16
}
/// $07AB52: lea.l $7abd0(pc), a1
pub fn i_07ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ABD0;
    16
}
/// $07AB56: move.l a1, (a6)
pub fn i_07ab56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AB58: bra.w $7abd0
pub fn i_07ab58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7ABD0;
    10
}
// ===== BannerDigit_Drop_07AB5C  ($07AB5C, 116 B, mission/banner_07a970.s)
/// $07AB5C: move.w $74(a6), $76(a6)
pub fn i_07ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB62;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07AB62: lea.l $7ab68(pc), a1
pub fn i_07ab62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AB68;
    16
}
/// $07AB66: move.l a1, (a6)
pub fn i_07ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AB68: subq.w #$1, $74(a6)
pub fn i_07ab68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB6C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07AB6C: bmi.w $7ab72
pub fn i_07ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB70;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x7AB72; return 10; }
    8
}
/// $07AB70: rts
pub fn i_07ab70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07AB72: move.w #$1be, d1
pub fn i_07ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB76;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AB76: jsr $236e.l
pub fn i_07ab76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB7C);
    cpu.pc = 0x236E;
    16
}
/// $07AB7C: ori.b #$1, $3a(a6)
pub fn i_07ab7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB82;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07AB82: move.w #$1ff, $78(a6)
pub fn i_07ab82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB88;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AB88: move.b #$ff, $33(a6)
pub fn i_07ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB8E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AB8E: move.b #$0, $7a(a6)
pub fn i_07ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB94;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AB94: jsr $267e2.l
pub fn i_07ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB9A);
    cpu.pc = 0x267E2;
    16
}
/// $07AB9A: clr.l d0
pub fn i_07ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB9C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $07AB9C: move.b $106ece.l, d0
pub fn i_07ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABA2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07ABA2: cmpi.b #$5, d0
pub fn i_07aba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABA6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07ABA6: bls.w $7abae
pub fn i_07aba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABAA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x7ABAE; return 10; }
    8
}
/// $07ABAA: move.b #$0, d0
pub fn i_07abaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABAE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07ABAE: movea.l #$2df71c, a0
pub fn i_07abae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABB4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DF71C;
    cpu.a[0] = v1;
    4
}
/// $07ABB4: lsl.w #$2, d0
pub fn i_07abb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABB6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07ABB6: movea.l (a0, d0.w), a0
pub fn i_07abb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABBA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07ABBA: cmpa.l #$ffffffff, a0
pub fn i_07abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABC0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07ABC0: beq.w $7abca
pub fn i_07abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7ABCA; return 10; }
    8
}
/// $07ABC4: jsr $28cd4.l
pub fn i_07abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ABCA);
    cpu.pc = 0x28CD4;
    16
}
/// $07ABCA: lea.l $7abd0(pc), a1
pub fn i_07abca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ABD0;
    16
}
/// $07ABCE: move.l a1, (a6)
pub fn i_07abce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== BannerLetter_Tick_07ABD0  ($07ABD0, 28 B, mission/banner_07a970.s)
/// $07ABD0: jsr $7b960(pc)
pub fn i_07abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7ABD4);
    cpu.pc = 0x7B960;
    16
}
/// $07ABD4: jsr $28d70.l
pub fn i_07abd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ABDA);
    cpu.pc = 0x28D70;
    16
}
/// $07ABDA: jsr $7b9a4(pc)
pub fn i_07abda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7ABDE);
    cpu.pc = 0x7B9A4;
    16
}
/// $07ABDE: bcc.w $7abf2
pub fn i_07abde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7ABF2; return 10; }
    8
}
/// $07ABE2: move.w #$0, d1
pub fn i_07abe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07ABE6: jsr $236e.l
pub fn i_07abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ABEC);
    cpu.pc = 0x236E;
    16
}
// ===== SetTaskHandler_07abec  ($07ABEC, 8 B, kernel/task_handlers.c)
/// $07ABEC: lea.l $7abf4(pc), a1
pub fn i_07abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ABF4;
    16
}
/// $07ABF0: move.l a1, (a6)
pub fn i_07abf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07ABF2: rts
pub fn i_07abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_Attach_07ABF4  ($07ABF4, 78 B, mission/banner_07a970.s)
/// $07ABF4: move.w $18(a6), $14(a6)
pub fn i_07abf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABFA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07ABFA: movea.l $c(a6), a0
pub fn i_07abfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABFE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07ABFE: addq.b #$1, $21(a0)
pub fn i_07abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC02;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $07AC02: move.b #$ff, $33(a6)
pub fn i_07ac02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC08;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AC08: move.b #$ff, $32(a6)
pub fn i_07ac08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC0E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AC0E: lea.l $7ac14(pc), a1
pub fn i_07ac0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AC14;
    16
}
/// $07AC12: move.l a1, (a6)
pub fn i_07ac12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AC14: cmpi.b #$0, $21(a6)
pub fn i_07ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC1A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07AC1A: beq.w $7ac2e
pub fn i_07ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7AC2E; return 10; }
    8
}
/// $07AC1E: movea.l $c(a6), a0
pub fn i_07ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC22;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07AC22: move.w $74(a0), d0
pub fn i_07ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC26;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07AC26: btst.b #$3, d0
pub fn i_07ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC2A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07AC2A: bne.w $7ac34
pub fn i_07ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AC34; return 10; }
    8
}
/// $07AC2E: jsr $28d70.l
pub fn i_07ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AC34);
    cpu.pc = 0x28D70;
    16
}
/// $07AC34: movea.l $c(a6), a0
pub fn i_07ac34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC38;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07AC38: cmpi.b #$ff, $20(a0)
pub fn i_07ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC3E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07AC3E: bne.w $7ac48
pub fn i_07ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC42;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AC48; return 10; }
    8
}
// ===== SetTaskHandler_07ac42  ($07AC42, 8 B, kernel/task_handlers.c)
/// $07AC42: lea.l $7ac4a(pc), a1
pub fn i_07ac42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AC4A;
    16
}
/// $07AC46: move.l a1, (a6)
pub fn i_07ac46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AC48: rts
pub fn i_07ac48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_FlyOff_07AC4A  ($07AC4A, 134 B, mission/banner_07a970.s)
/// $07AC4A: move.w #$0, $36(a6)
pub fn i_07ac4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AC50: lea.l $7ac56(pc), a1
pub fn i_07ac50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AC56;
    16
}
/// $07AC54: move.l a1, (a6)
pub fn i_07ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AC56: addi.w #$1ff, $36(a6)
pub fn i_07ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC5C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1FF, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07AC5C: clr.l d0
pub fn i_07ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC5E;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $07AC5E: move.w $36(a6), d0
pub fn i_07ac5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC62;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07AC62: btst.b #$f, d0
pub fn i_07ac62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC66;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07AC66: beq.w $7ac70
pub fn i_07ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC6A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7AC70; return 10; }
    8
}
/// $07AC6A: move.w #$7fff, $36(a6)
pub fn i_07ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AC70: move.w #$a0, d0
pub fn i_07ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC74;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AC74: move.w #$190, d1
pub fn i_07ac74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC78;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AC78: sub.w $22(a6), d0
pub fn i_07ac78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC7C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07AC7C: sub.w $24(a6), d1
pub fn i_07ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC80;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07AC80: jsr $5e018.l
pub fn i_07ac80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AC86);
    cpu.pc = 0x5E018;
    16
}
/// $07AC86: addi.b #$7f, d0
pub fn i_07ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC8A;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x7F, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07AC8A: andi.w #$ff, d0
pub fn i_07ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC8E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07AC8E: move.w $36(a6), d1
pub fn i_07ac8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC92;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07AC92: jsr $13c0e.l
pub fn i_07ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AC98);
    cpu.pc = 0x13C0E;
    16
}
/// $07AC98: move.w d1, $28(a6)
pub fn i_07ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC9C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AC9C: move.w d2, $2a(a6)
pub fn i_07ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACA0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ACA0: bset.b #$6, $13(a6)
pub fn i_07aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACA6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07ACA6: jsr $27cee.l
pub fn i_07aca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACAC);
    cpu.pc = 0x27CEE;
    16
}
/// $07ACAC: jsr $28d70.l
pub fn i_07acac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACB2);
    cpu.pc = 0x28D70;
    16
}
/// $07ACB2: movea.l #$ffffffff, a0
pub fn i_07acb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACB8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07ACB8: lea.l $2df734.l, a0
pub fn i_07acb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF734;
    20
}
/// $07ACBE: jsr $5dd5c.l
pub fn i_07acbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACC4);
    cpu.pc = 0x5DD5C;
    16
}
/// $07ACC4: bcc.w $7acce
pub fn i_07acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7ACCE; return 10; }
    8
}
/// $07ACC8: jmp $518.l
pub fn i_07acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACCE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07ACCE: rts
pub fn i_07acce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLayout_Start_07ACD0  ($07ACD0, 640 B, mission/banner_07a970.s)
/// $07ACD0: lea.l $7aae0(pc), a1
pub fn i_07acd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07ACD4: jsr $4ae.l
pub fn i_07acd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACDA);
    cpu.pc = 0x4AE;
    16
}
/// $07ACDA: jsr $5dd02.l
pub fn i_07acda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACE0);
    cpu.pc = 0x5DD02;
    16
}
/// $07ACE0: move.b #$c, $20(a0)
pub fn i_07ace0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACE6;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ACE6: move.b #$1, $21(a0)
pub fn i_07ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACEC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ACEC: move.w #$18, $74(a0)
pub fn i_07acec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACF2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ACF2: move.w #$da, $70(a0)
pub fn i_07acf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACF8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xDA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ACF8: move.w #$178, $72(a0)
pub fn i_07acf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACFE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ACFE: lea.l $7aae0(pc), a1
pub fn i_07acfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AD02: jsr $4ae.l
pub fn i_07ad02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD08);
    cpu.pc = 0x4AE;
    16
}
/// $07AD08: jsr $5dd02.l
pub fn i_07ad08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD0E);
    cpu.pc = 0x5DD02;
    16
}
/// $07AD0E: move.b #$b, $20(a0)
pub fn i_07ad0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD14;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD14: move.b #$1, $21(a0)
pub fn i_07ad14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD1A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD1A: move.w #$16, $74(a0)
pub fn i_07ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD20;
    cpu.ir = 0x317C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD20: move.w #$c4, $70(a0)
pub fn i_07ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD26;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD26: move.w #$178, $72(a0)
pub fn i_07ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD2C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD2C: lea.l $7aae0(pc), a1
pub fn i_07ad2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AD30: jsr $4ae.l
pub fn i_07ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD36);
    cpu.pc = 0x4AE;
    16
}
/// $07AD36: jsr $5dd02.l
pub fn i_07ad36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD3C);
    cpu.pc = 0x5DD02;
    16
}
/// $07AD3C: move.b #$6, $20(a0)
pub fn i_07ad3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD42;
    cpu.ir = 0x117C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD42: move.b #$1, $21(a0)
pub fn i_07ad42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD48;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD48: move.w #$14, $74(a0)
pub fn i_07ad48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD4E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD4E: move.w #$ad, $70(a0)
pub fn i_07ad4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD54;
    cpu.ir = 0x317C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD54: move.w #$178, $72(a0)
pub fn i_07ad54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD5A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD5A: lea.l $7aae0(pc), a1
pub fn i_07ad5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AD5E: jsr $4ae.l
pub fn i_07ad5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD64);
    cpu.pc = 0x4AE;
    16
}
/// $07AD64: jsr $5dd02.l
pub fn i_07ad64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD6A);
    cpu.pc = 0x5DD02;
    16
}
/// $07AD6A: move.b #$5, $20(a0)
pub fn i_07ad6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD70;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD70: move.b #$1, $21(a0)
pub fn i_07ad70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD76;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD76: move.w #$12, $74(a0)
pub fn i_07ad76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD7C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD7C: move.w #$94, $70(a0)
pub fn i_07ad7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD82;
    cpu.ir = 0x317C;
    let v1: u32 = 0x94;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD82: move.w #$178, $72(a0)
pub fn i_07ad82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD88;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD88: lea.l $7aae0(pc), a1
pub fn i_07ad88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AD8C: jsr $4ae.l
pub fn i_07ad8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD92);
    cpu.pc = 0x4AE;
    16
}
/// $07AD92: jsr $5dd02.l
pub fn i_07ad92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD98);
    cpu.pc = 0x5DD02;
    16
}
/// $07AD98: move.b #$b, $20(a0)
pub fn i_07ad98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD9E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD9E: move.b #$1, $21(a0)
pub fn i_07ad9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADA4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ADA4: move.w #$10, $74(a0)
pub fn i_07ada4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADAA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADAA: move.w #$7e, $70(a0)
pub fn i_07adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADB0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADB0: move.w #$178, $72(a0)
pub fn i_07adb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADB6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADB6: lea.l $7aae0(pc), a1
pub fn i_07adb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07ADBA: jsr $4ae.l
pub fn i_07adba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ADC0);
    cpu.pc = 0x4AE;
    16
}
/// $07ADC0: jsr $5dd02.l
pub fn i_07adc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ADC6);
    cpu.pc = 0x5DD02;
    16
}
/// $07ADC6: move.b #$2, $20(a0)
pub fn i_07adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADCC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ADCC: move.b #$1, $21(a0)
pub fn i_07adcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADD2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ADD2: move.w #$e, $74(a0)
pub fn i_07add2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADD8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADD8: move.w #$64, $70(a0)
pub fn i_07add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADDE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADDE: move.w #$178, $72(a0)
pub fn i_07adde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADE4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADE4: lea.l $7ab5c(pc), a1
pub fn i_07ade4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AB5C;
    16
}
/// $07ADE8: jsr $4ae.l
pub fn i_07ade8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ADEE);
    cpu.pc = 0x4AE;
    16
}
/// $07ADEE: jsr $5dd02.l
pub fn i_07adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ADF4);
    cpu.pc = 0x5DD02;
    16
}
/// $07ADF4: move.b #$0, $21(a0)
pub fn i_07adf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADFA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ADFA: move.w #$e, $74(a0)
pub fn i_07adfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE00;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE00: move.w #$f0, $70(a0)
pub fn i_07ae00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE06;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE06: move.w #$1a8, $72(a0)
pub fn i_07ae06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE0C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE0C: lea.l $7aae0(pc), a1
pub fn i_07ae0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AE10: jsr $4ae.l
pub fn i_07ae10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE16);
    cpu.pc = 0x4AE;
    16
}
/// $07AE16: jsr $5dd02.l
pub fn i_07ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE1C);
    cpu.pc = 0x5DD02;
    16
}
/// $07AE1C: move.b #$4, $20(a0)
pub fn i_07ae1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE22;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE22: move.b #$0, $21(a0)
pub fn i_07ae22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE28;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE28: move.w #$c, $74(a0)
pub fn i_07ae28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE2E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE2E: move.w #$d0, $70(a0)
pub fn i_07ae2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE34;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE34: move.w #$1a8, $72(a0)
pub fn i_07ae34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE3A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE3A: lea.l $7aae0(pc), a1
pub fn i_07ae3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AE3E: jsr $4ae.l
pub fn i_07ae3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE44);
    cpu.pc = 0x4AE;
    16
}
/// $07AE44: jsr $5dd02.l
pub fn i_07ae44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE4A);
    cpu.pc = 0x5DD02;
    16
}
/// $07AE4A: move.b #$3, $20(a0)
pub fn i_07ae4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE50;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE50: move.b #$0, $21(a0)
pub fn i_07ae50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE56;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE56: move.w #$a, $74(a0)
pub fn i_07ae56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE5C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE5C: move.w #$b7, $70(a0)
pub fn i_07ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE62;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE62: move.w #$1a8, $72(a0)
pub fn i_07ae62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE68;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE68: lea.l $7aae0(pc), a1
pub fn i_07ae68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AE6C: jsr $4ae.l
pub fn i_07ae6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE72);
    cpu.pc = 0x4AE;
    16
}
/// $07AE72: jsr $5dd02.l
pub fn i_07ae72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE78);
    cpu.pc = 0x5DD02;
    16
}
/// $07AE78: move.b #$1, $20(a0)
pub fn i_07ae78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE7E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE7E: move.b #$0, $21(a0)
pub fn i_07ae7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE84;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE84: move.w #$8, $74(a0)
pub fn i_07ae84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE8A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE8A: move.w #$a4, $70(a0)
pub fn i_07ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE90;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE90: move.w #$1a8, $72(a0)
pub fn i_07ae90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE96;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE96: lea.l $7aae0(pc), a1
pub fn i_07ae96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AE9A: jsr $4ae.l
pub fn i_07ae9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AEA0);
    cpu.pc = 0x4AE;
    16
}
/// $07AEA0: jsr $5dd02.l
pub fn i_07aea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AEA6);
    cpu.pc = 0x5DD02;
    16
}
/// $07AEA6: move.b #$2, $20(a0)
pub fn i_07aea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEAC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AEAC: move.b #$0, $21(a0)
pub fn i_07aeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEB2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AEB2: move.w #$6, $74(a0)
pub fn i_07aeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEB8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEB8: move.w #$90, $70(a0)
pub fn i_07aeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEBE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEBE: move.w #$1a8, $72(a0)
pub fn i_07aebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEC4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEC4: lea.l $7aae0(pc), a1
pub fn i_07aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AEC8: jsr $4ae.l
pub fn i_07aec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AECE);
    cpu.pc = 0x4AE;
    16
}
/// $07AECE: jsr $5dd02.l
pub fn i_07aece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AED4);
    cpu.pc = 0x5DD02;
    16
}
/// $07AED4: move.b #$2, $20(a0)
pub fn i_07aed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEDA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AEDA: move.b #$0, $21(a0)
pub fn i_07aeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEE0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AEE0: move.w #$4, $74(a0)
pub fn i_07aee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEE6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEE6: move.w #$77, $70(a0)
pub fn i_07aee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEEC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEEC: move.w #$1a8, $72(a0)
pub fn i_07aeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEF2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEF2: lea.l $7aae0(pc), a1
pub fn i_07aef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AEF6: jsr $4ae.l
pub fn i_07aef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AEFC);
    cpu.pc = 0x4AE;
    16
}
/// $07AEFC: jsr $5dd02.l
pub fn i_07aefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF02);
    cpu.pc = 0x5DD02;
    16
}
/// $07AF02: move.b #$1, $20(a0)
pub fn i_07af02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF08;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF08: move.b #$0, $21(a0)
pub fn i_07af08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF0E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF0E: move.w #$2, $74(a0)
pub fn i_07af0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF14;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF14: move.w #$64, $70(a0)
pub fn i_07af14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF1A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF1A: move.w #$1a8, $72(a0)
pub fn i_07af1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF20;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF20: lea.l $7aae0(pc), a1
pub fn i_07af20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AF24: jsr $4ae.l
pub fn i_07af24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF2A);
    cpu.pc = 0x4AE;
    16
}
/// $07AF2A: jsr $5dd02.l
pub fn i_07af2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF30);
    cpu.pc = 0x5DD02;
    16
}
/// $07AF30: move.b #$0, $20(a0)
pub fn i_07af30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF36;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF36: move.b #$0, $21(a0)
pub fn i_07af36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF3C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF3C: move.w #$0, $74(a0)
pub fn i_07af3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF42;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF42: move.w #$50, $70(a0)
pub fn i_07af42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF48;
    cpu.ir = 0x317C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF48: move.w #$1a8, $72(a0)
pub fn i_07af48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF4E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF4E: rts
pub fn i_07af4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLayout_StartFinal_07AF50  ($07AF50, 830 B, mission/banner_07a970.s)
/// $07AF50: lea.l $7aae0(pc), a1
pub fn i_07af50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AF54: jsr $4ae.l
pub fn i_07af54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF5A);
    cpu.pc = 0x4AE;
    16
}
/// $07AF5A: jsr $5dd02.l
pub fn i_07af5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF60);
    cpu.pc = 0x5DD02;
    16
}
/// $07AF60: move.b #$c, $20(a0)
pub fn i_07af60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF66;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF66: move.b #$1, $21(a0)
pub fn i_07af66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF6C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF6C: move.w #$22, $74(a0)
pub fn i_07af6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF72;
    cpu.ir = 0x317C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF72: move.w #$da, $70(a0)
pub fn i_07af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF78;
    cpu.ir = 0x317C;
    let v1: u32 = 0xDA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF78: move.w #$168, $72(a0)
pub fn i_07af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF7E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF7E: lea.l $7aae0(pc), a1
pub fn i_07af7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AF82: jsr $4ae.l
pub fn i_07af82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF88);
    cpu.pc = 0x4AE;
    16
}
/// $07AF88: jsr $5dd02.l
pub fn i_07af88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF8E);
    cpu.pc = 0x5DD02;
    16
}
/// $07AF8E: move.b #$b, $20(a0)
pub fn i_07af8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF94;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF94: move.b #$1, $21(a0)
pub fn i_07af94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF9A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF9A: move.w #$20, $74(a0)
pub fn i_07af9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFA0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFA0: move.w #$c6, $70(a0)
pub fn i_07afa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFA6;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFA6: move.w #$168, $72(a0)
pub fn i_07afa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFAC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFAC: lea.l $7aae0(pc), a1
pub fn i_07afac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AFB0: jsr $4ae.l
pub fn i_07afb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AFB6);
    cpu.pc = 0x4AE;
    16
}
/// $07AFB6: jsr $5dd02.l
pub fn i_07afb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AFBC);
    cpu.pc = 0x5DD02;
    16
}
/// $07AFBC: move.b #$6, $20(a0)
pub fn i_07afbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFC2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AFC2: move.b #$1, $21(a0)
pub fn i_07afc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFC8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AFC8: move.w #$1e, $74(a0)
pub fn i_07afc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFCE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFCE: move.w #$af, $70(a0)
pub fn i_07afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFD4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFD4: move.w #$168, $72(a0)
pub fn i_07afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFDA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFDA: lea.l $7aae0(pc), a1
pub fn i_07afda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AFDE: jsr $4ae.l
pub fn i_07afde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AFE4);
    cpu.pc = 0x4AE;
    16
}
/// $07AFE4: jsr $5dd02.l
pub fn i_07afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AFEA);
    cpu.pc = 0x5DD02;
    16
}
/// $07AFEA: move.b #$5, $20(a0)
pub fn i_07afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFF0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AFF0: move.b #$1, $21(a0)
pub fn i_07aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFF6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AFF6: move.w #$1c, $74(a0)
pub fn i_07aff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFFC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFFC: move.w #$96, $70(a0)
pub fn i_07affc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B002;
    cpu.ir = 0x317C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B002: move.w #$168, $72(a0)
pub fn i_07b002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B008;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B008: lea.l $7aae0(pc), a1
pub fn i_07b008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B00C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B00C: jsr $4ae.l
pub fn i_07b00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B012);
    cpu.pc = 0x4AE;
    16
}
/// $07B012: jsr $5dd02.l
pub fn i_07b012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B018);
    cpu.pc = 0x5DD02;
    16
}
/// $07B018: move.b #$b, $20(a0)
pub fn i_07b018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B01E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B01E: move.b #$1, $21(a0)
pub fn i_07b01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B024;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B024: move.w #$1a, $74(a0)
pub fn i_07b024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B02A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B02A: move.w #$80, $70(a0)
pub fn i_07b02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B030;
    cpu.ir = 0x317C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B030: move.w #$168, $72(a0)
pub fn i_07b030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B036;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B036: lea.l $7aae0(pc), a1
pub fn i_07b036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B03A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B03A: jsr $4ae.l
pub fn i_07b03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B040);
    cpu.pc = 0x4AE;
    16
}
/// $07B040: jsr $5dd02.l
pub fn i_07b040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B046;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B046);
    cpu.pc = 0x5DD02;
    16
}
/// $07B046: move.b #$2, $20(a0)
pub fn i_07b046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B04C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B04C: move.b #$1, $21(a0)
pub fn i_07b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B052;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B052: move.w #$18, $74(a0)
pub fn i_07b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B058;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B058: move.w #$66, $70(a0)
pub fn i_07b058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B05E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x66;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B05E: move.w #$168, $72(a0)
pub fn i_07b05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B064;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B064: lea.l $7aae0(pc), a1
pub fn i_07b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B068;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B068: jsr $4ae.l
pub fn i_07b068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B06E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B06E);
    cpu.pc = 0x4AE;
    16
}
/// $07B06E: jsr $5dd02.l
pub fn i_07b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B074);
    cpu.pc = 0x5DD02;
    16
}
/// $07B074: move.b #$4, $20(a0)
pub fn i_07b074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B07A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B07A: move.b #$0, $21(a0)
pub fn i_07b07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B080;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B080: move.w #$16, $74(a0)
pub fn i_07b080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B086;
    cpu.ir = 0x317C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B086: move.w #$e0, $70(a0)
pub fn i_07b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B08C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B08C: move.w #$198, $72(a0)
pub fn i_07b08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B092;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B092: lea.l $7aae0(pc), a1
pub fn i_07b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B096;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B096: jsr $4ae.l
pub fn i_07b096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B09C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B09C);
    cpu.pc = 0x4AE;
    16
}
/// $07B09C: jsr $5dd02.l
pub fn i_07b09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0A2);
    cpu.pc = 0x5DD02;
    16
}
/// $07B0A2: move.b #$3, $20(a0)
pub fn i_07b0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0A8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B0A8: move.b #$0, $21(a0)
pub fn i_07b0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0AE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B0AE: move.w #$14, $74(a0)
pub fn i_07b0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0B4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0B4: move.w #$c7, $70(a0)
pub fn i_07b0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0BA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0BA: move.w #$198, $72(a0)
pub fn i_07b0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0C0: lea.l $7aae0(pc), a1
pub fn i_07b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B0C4: jsr $4ae.l
pub fn i_07b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0CA);
    cpu.pc = 0x4AE;
    16
}
/// $07B0CA: jsr $5dd02.l
pub fn i_07b0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0D0);
    cpu.pc = 0x5DD02;
    16
}
/// $07B0D0: move.b #$1, $20(a0)
pub fn i_07b0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0D6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B0D6: move.b #$0, $21(a0)
pub fn i_07b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0DC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B0DC: move.w #$12, $74(a0)
pub fn i_07b0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0E2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0E2: move.w #$b4, $70(a0)
pub fn i_07b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0E8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0E8: move.w #$198, $72(a0)
pub fn i_07b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0EE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0EE: lea.l $7aae0(pc), a1
pub fn i_07b0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B0F2: jsr $4ae.l
pub fn i_07b0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0F8);
    cpu.pc = 0x4AE;
    16
}
/// $07B0F8: jsr $5dd02.l
pub fn i_07b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0FE);
    cpu.pc = 0x5DD02;
    16
}
/// $07B0FE: move.b #$2, $20(a0)
pub fn i_07b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B104;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B104: move.b #$0, $21(a0)
pub fn i_07b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B10A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B10A: move.w #$10, $74(a0)
pub fn i_07b10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B110;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B110: move.w #$a0, $70(a0)
pub fn i_07b110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B116;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B116: move.w #$198, $72(a0)
pub fn i_07b116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B11C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B11C: lea.l $7aae0(pc), a1
pub fn i_07b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B120;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B120: jsr $4ae.l
pub fn i_07b120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B126;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B126);
    cpu.pc = 0x4AE;
    16
}
/// $07B126: jsr $5dd02.l
pub fn i_07b126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B12C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B12C);
    cpu.pc = 0x5DD02;
    16
}
/// $07B12C: move.b #$2, $20(a0)
pub fn i_07b12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B132;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B132: move.b #$0, $21(a0)
pub fn i_07b132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B138;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B138: move.w #$e, $74(a0)
pub fn i_07b138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B13E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B13E: move.w #$87, $70(a0)
pub fn i_07b13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B144;
    cpu.ir = 0x317C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B144: move.w #$198, $72(a0)
pub fn i_07b144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B14A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B14A: lea.l $7aae0(pc), a1
pub fn i_07b14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B14E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B14E: jsr $4ae.l
pub fn i_07b14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B154);
    cpu.pc = 0x4AE;
    16
}
/// $07B154: jsr $5dd02.l
pub fn i_07b154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B15A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B15A);
    cpu.pc = 0x5DD02;
    16
}
/// $07B15A: move.b #$1, $20(a0)
pub fn i_07b15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B160;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B160: move.b #$0, $21(a0)
pub fn i_07b160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B166;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B166: move.w #$c, $74(a0)
pub fn i_07b166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B16C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B16C: move.w #$74, $70(a0)
pub fn i_07b16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B172;
    cpu.ir = 0x317C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B172: move.w #$198, $72(a0)
pub fn i_07b172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B178;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B178: lea.l $7aae0(pc), a1
pub fn i_07b178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B17C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B17C: jsr $4ae.l
pub fn i_07b17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B182);
    cpu.pc = 0x4AE;
    16
}
/// $07B182: jsr $5dd02.l
pub fn i_07b182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B188);
    cpu.pc = 0x5DD02;
    16
}
/// $07B188: move.b #$0, $20(a0)
pub fn i_07b188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B18E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B18E: move.b #$0, $21(a0)
pub fn i_07b18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B194;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B194: move.w #$a, $74(a0)
pub fn i_07b194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B19A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B19A: move.w #$60, $70(a0)
pub fn i_07b19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1A0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1A0: move.w #$198, $72(a0)
pub fn i_07b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1A6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1A6: lea.l $7aae0(pc), a1
pub fn i_07b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B1AA: jsr $4ae.l
pub fn i_07b1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B1B0);
    cpu.pc = 0x4AE;
    16
}
/// $07B1B0: jsr $5dd02.l
pub fn i_07b1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B1B6);
    cpu.pc = 0x5DD02;
    16
}
/// $07B1B6: move.b #$9, $20(a0)
pub fn i_07b1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1BC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B1BC: move.b #$0, $21(a0)
pub fn i_07b1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1C2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B1C2: move.w #$8, $74(a0)
pub fn i_07b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1C8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1C8: move.w #$cd, $70(a0)
pub fn i_07b1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1CE: move.w #$1b8, $72(a0)
pub fn i_07b1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1D4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1D4: lea.l $7aae0(pc), a1
pub fn i_07b1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B1D8: jsr $4ae.l
pub fn i_07b1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B1DE);
    cpu.pc = 0x4AE;
    16
}
/// $07B1DE: jsr $5dd02.l
pub fn i_07b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B1E4);
    cpu.pc = 0x5DD02;
    16
}
/// $07B1E4: move.b #$5, $20(a0)
pub fn i_07b1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1EA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B1EA: move.b #$0, $21(a0)
pub fn i_07b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1F0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B1F0: move.w #$6, $74(a0)
pub fn i_07b1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1F6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1F6: move.w #$b4, $70(a0)
pub fn i_07b1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1FC;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1FC: move.w #$1b8, $72(a0)
pub fn i_07b1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B202;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B202: lea.l $7aae0(pc), a1
pub fn i_07b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B206;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B206: jsr $4ae.l
pub fn i_07b206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B20C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B20C);
    cpu.pc = 0x4AE;
    16
}
/// $07B20C: jsr $5dd02.l
pub fn i_07b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B212);
    cpu.pc = 0x5DD02;
    16
}
/// $07B212: move.b #$4, $20(a0)
pub fn i_07b212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B218;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B218: move.b #$0, $21(a0)
pub fn i_07b218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B21E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B21E: move.w #$4, $74(a0)
pub fn i_07b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B224;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B224: move.w #$9c, $70(a0)
pub fn i_07b224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B22A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B22A: move.w #$1b8, $72(a0)
pub fn i_07b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B230;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B230: lea.l $7aae0(pc), a1
pub fn i_07b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B234;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B234: jsr $4ae.l
pub fn i_07b234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B23A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B23A);
    cpu.pc = 0x4AE;
    16
}
/// $07B23A: jsr $5dd02.l
pub fn i_07b23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B240);
    cpu.pc = 0x5DD02;
    16
}
/// $07B240: move.b #$1, $20(a0)
pub fn i_07b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B246;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B246: move.b #$0, $21(a0)
pub fn i_07b246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B24C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B24C: move.w #$2, $74(a0)
pub fn i_07b24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B252;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B252: move.w #$89, $70(a0)
pub fn i_07b252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B258;
    cpu.ir = 0x317C;
    let v1: u32 = 0x89;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B258: move.w #$1b8, $72(a0)
pub fn i_07b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B25E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B25E: lea.l $7aae0(pc), a1
pub fn i_07b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B262;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B262: jsr $4ae.l
pub fn i_07b262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B268;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B268);
    cpu.pc = 0x4AE;
    16
}
/// $07B268: jsr $5dd02.l
pub fn i_07b268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B26E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B26E);
    cpu.pc = 0x5DD02;
    16
}
/// $07B26E: move.b #$d, $20(a0)
pub fn i_07b26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B274;
    cpu.ir = 0x117C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B274: move.b #$0, $21(a0)
pub fn i_07b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B27A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B27A: move.w #$0, $74(a0)
pub fn i_07b27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B280;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B280: move.w #$76, $70(a0)
pub fn i_07b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B286;
    cpu.ir = 0x317C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B286: move.w #$1b8, $72(a0)
pub fn i_07b286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B28C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B28C: rts
pub fn i_07b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B28E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLayout_Complete_07B28E  ($07B28E, 778 B, mission/banner_07a970.s)
/// $07B28E: lea.l $7aae0(pc), a1
pub fn i_07b28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B292;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B292: jsr $4ae.l
pub fn i_07b292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B298);
    cpu.pc = 0x4AE;
    16
}
/// $07B298: jsr $5dd02.l
pub fn i_07b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B29E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B29E);
    cpu.pc = 0x5DD02;
    16
}
/// $07B29E: move.b #$c, $20(a0)
pub fn i_07b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2A4;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B2A4: move.b #$1, $21(a0)
pub fn i_07b2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2AA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B2AA: move.w #$20, $74(a0)
pub fn i_07b2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2B0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2B0: move.w #$102, $70(a0)
pub fn i_07b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2B6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x102;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2B6: move.w #$178, $72(a0)
pub fn i_07b2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2BC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2BC: lea.l $7aae0(pc), a1
pub fn i_07b2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B2C0: jsr $4ae.l
pub fn i_07b2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B2C6);
    cpu.pc = 0x4AE;
    16
}
/// $07B2C6: jsr $5dd02.l
pub fn i_07b2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B2CC);
    cpu.pc = 0x5DD02;
    16
}
/// $07B2CC: move.b #$a, $20(a0)
pub fn i_07b2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2D2;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B2D2: move.b #$1, $21(a0)
pub fn i_07b2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2D8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B2D8: move.w #$1e, $74(a0)
pub fn i_07b2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2DE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2DE: move.w #$ec, $70(a0)
pub fn i_07b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2E4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2E4: move.w #$178, $72(a0)
pub fn i_07b2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2EA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2EA: lea.l $7aae0(pc), a1
pub fn i_07b2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B2EE: jsr $4ae.l
pub fn i_07b2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B2F4);
    cpu.pc = 0x4AE;
    16
}
/// $07B2F4: jsr $5dd02.l
pub fn i_07b2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B2FA);
    cpu.pc = 0x5DD02;
    16
}
/// $07B2FA: move.b #$b, $20(a0)
pub fn i_07b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B300;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B300: move.b #$1, $21(a0)
pub fn i_07b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B306;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B306: move.w #$1c, $74(a0)
pub fn i_07b306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B30C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B30C: move.w #$d4, $70(a0)
pub fn i_07b30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B312;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B312: move.w #$178, $72(a0)
pub fn i_07b312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B318;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B318: lea.l $7aae0(pc), a1
pub fn i_07b318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B31C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B31C: jsr $4ae.l
pub fn i_07b31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B322);
    cpu.pc = 0x4AE;
    16
}
/// $07B322: jsr $5dd02.l
pub fn i_07b322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B328);
    cpu.pc = 0x5DD02;
    16
}
/// $07B328: move.b #$a, $20(a0)
pub fn i_07b328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B32E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B32E: move.b #$1, $21(a0)
pub fn i_07b32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B334;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B334: move.w #$1a, $74(a0)
pub fn i_07b334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B33A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B33A: move.w #$bc, $70(a0)
pub fn i_07b33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B340;
    cpu.ir = 0x317C;
    let v1: u32 = 0xBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B340: move.w #$178, $72(a0)
pub fn i_07b340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B346;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B346: lea.l $7aae0(pc), a1
pub fn i_07b346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B34A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B34A: jsr $4ae.l
pub fn i_07b34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B350);
    cpu.pc = 0x4AE;
    16
}
/// $07B350: jsr $5dd02.l
pub fn i_07b350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B356;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B356);
    cpu.pc = 0x5DD02;
    16
}
/// $07B356: move.b #$9, $20(a0)
pub fn i_07b356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B35C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B35C: move.b #$1, $21(a0)
pub fn i_07b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B362;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B362: move.w #$18, $74(a0)
pub fn i_07b362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B368;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B368: move.w #$a5, $70(a0)
pub fn i_07b368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B36E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B36E: move.w #$178, $72(a0)
pub fn i_07b36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B374;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B374: lea.l $7aae0(pc), a1
pub fn i_07b374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B378;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B378: jsr $4ae.l
pub fn i_07b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B37E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B37E);
    cpu.pc = 0x4AE;
    16
}
/// $07B37E: jsr $5dd02.l
pub fn i_07b37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B384);
    cpu.pc = 0x5DD02;
    16
}
/// $07B384: move.b #$8, $20(a0)
pub fn i_07b384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B38A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B38A: move.b #$1, $21(a0)
pub fn i_07b38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B390;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B390: move.w #$16, $74(a0)
pub fn i_07b390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B396;
    cpu.ir = 0x317C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B396: move.w #$8d, $70(a0)
pub fn i_07b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B39C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B39C: move.w #$178, $72(a0)
pub fn i_07b39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3A2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3A2: lea.l $7aae0(pc), a1
pub fn i_07b3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B3A6: jsr $4ae.l
pub fn i_07b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B3AC);
    cpu.pc = 0x4AE;
    16
}
/// $07B3AC: jsr $5dd02.l
pub fn i_07b3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B3B2);
    cpu.pc = 0x5DD02;
    16
}
/// $07B3B2: move.b #$0, $20(a0)
pub fn i_07b3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3B8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B3B8: move.b #$1, $21(a0)
pub fn i_07b3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3BE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B3BE: move.w #$14, $74(a0)
pub fn i_07b3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3C4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3C4: move.w #$74, $70(a0)
pub fn i_07b3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3CA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3CA: move.w #$178, $72(a0)
pub fn i_07b3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3D0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3D0: lea.l $7aae0(pc), a1
pub fn i_07b3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B3D4: jsr $4ae.l
pub fn i_07b3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B3DA);
    cpu.pc = 0x4AE;
    16
}
/// $07B3DA: jsr $5dd02.l
pub fn i_07b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B3E0);
    cpu.pc = 0x5DD02;
    16
}
/// $07B3E0: move.b #$3, $20(a0)
pub fn i_07b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3E6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B3E6: move.b #$1, $21(a0)
pub fn i_07b3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3EC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B3EC: move.w #$12, $74(a0)
pub fn i_07b3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3F2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3F2: move.w #$5b, $70(a0)
pub fn i_07b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3F8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3F8: move.w #$178, $72(a0)
pub fn i_07b3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3FE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3FE: lea.l $7aae0(pc), a1
pub fn i_07b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B402;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B402: jsr $4ae.l
pub fn i_07b402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B408);
    cpu.pc = 0x4AE;
    16
}
/// $07B408: jsr $5dd02.l
pub fn i_07b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B40E);
    cpu.pc = 0x5DD02;
    16
}
/// $07B40E: move.b #$7, $20(a0)
pub fn i_07b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B414;
    cpu.ir = 0x117C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B414: move.b #$1, $21(a0)
pub fn i_07b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B41A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B41A: move.w #$10, $74(a0)
pub fn i_07b41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B420;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B420: move.w #$42, $70(a0)
pub fn i_07b420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B426;
    cpu.ir = 0x317C;
    let v1: u32 = 0x42;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B426: move.w #$178, $72(a0)
pub fn i_07b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B42C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B42C: lea.l $7ab5c(pc), a1
pub fn i_07b42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B430;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AB5C;
    16
}
/// $07B430: jsr $4ae.l
pub fn i_07b430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B436);
    cpu.pc = 0x4AE;
    16
}
/// $07B436: jsr $5dd02.l
pub fn i_07b436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B43C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B43C);
    cpu.pc = 0x5DD02;
    16
}
/// $07B43C: move.b #$0, $21(a0)
pub fn i_07b43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B442;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B442: move.w #$e, $74(a0)
pub fn i_07b442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B448;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B448: move.w #$f0, $70(a0)
pub fn i_07b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B44E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B44E: move.w #$1a8, $72(a0)
pub fn i_07b44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B454;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B454: lea.l $7aae0(pc), a1
pub fn i_07b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B458;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B458: jsr $4ae.l
pub fn i_07b458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B45E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B45E);
    cpu.pc = 0x4AE;
    16
}
/// $07B45E: jsr $5dd02.l
pub fn i_07b45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B464);
    cpu.pc = 0x5DD02;
    16
}
/// $07B464: move.b #$4, $20(a0)
pub fn i_07b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B46A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B46A: move.b #$0, $21(a0)
pub fn i_07b46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B470;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B470: move.w #$c, $74(a0)
pub fn i_07b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B476;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B476: move.w #$d0, $70(a0)
pub fn i_07b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B47C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B47C: move.w #$1a8, $72(a0)
pub fn i_07b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B482;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B482: lea.l $7aae0(pc), a1
pub fn i_07b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B486;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B486: jsr $4ae.l
pub fn i_07b486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B48C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B48C);
    cpu.pc = 0x4AE;
    16
}
/// $07B48C: jsr $5dd02.l
pub fn i_07b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B492;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B492);
    cpu.pc = 0x5DD02;
    16
}
/// $07B492: move.b #$3, $20(a0)
pub fn i_07b492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B498;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B498: move.b #$0, $21(a0)
pub fn i_07b498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B49E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B49E: move.w #$a, $74(a0)
pub fn i_07b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4A4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4A4: move.w #$b7, $70(a0)
pub fn i_07b4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4AA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4AA: move.w #$1a8, $72(a0)
pub fn i_07b4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4B0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4B0: lea.l $7aae0(pc), a1
pub fn i_07b4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B4B4: jsr $4ae.l
pub fn i_07b4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B4BA);
    cpu.pc = 0x4AE;
    16
}
/// $07B4BA: jsr $5dd02.l
pub fn i_07b4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B4C0);
    cpu.pc = 0x5DD02;
    16
}
/// $07B4C0: move.b #$1, $20(a0)
pub fn i_07b4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4C6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B4C6: move.b #$0, $21(a0)
pub fn i_07b4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4CC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B4CC: move.w #$8, $74(a0)
pub fn i_07b4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4D2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4D2: move.w #$a4, $70(a0)
pub fn i_07b4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4D8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4D8: move.w #$1a8, $72(a0)
pub fn i_07b4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4DE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4DE: lea.l $7aae0(pc), a1
pub fn i_07b4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B4E2: jsr $4ae.l
pub fn i_07b4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B4E8);
    cpu.pc = 0x4AE;
    16
}
/// $07B4E8: jsr $5dd02.l
pub fn i_07b4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B4EE);
    cpu.pc = 0x5DD02;
    16
}
/// $07B4EE: move.b #$2, $20(a0)
pub fn i_07b4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4F4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B4F4: move.b #$0, $21(a0)
pub fn i_07b4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4FA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B4FA: move.w #$6, $74(a0)
pub fn i_07b4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B500;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B500: move.w #$90, $70(a0)
pub fn i_07b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B506;
    cpu.ir = 0x317C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B506: move.w #$1a8, $72(a0)
pub fn i_07b506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B50C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B50C: lea.l $7aae0(pc), a1
pub fn i_07b50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B510;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B510: jsr $4ae.l
pub fn i_07b510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B516);
    cpu.pc = 0x4AE;
    16
}
/// $07B516: jsr $5dd02.l
pub fn i_07b516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B51C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B51C);
    cpu.pc = 0x5DD02;
    16
}
/// $07B51C: move.b #$2, $20(a0)
pub fn i_07b51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B522;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B522: move.b #$0, $21(a0)
pub fn i_07b522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B528;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B528: move.w #$4, $74(a0)
pub fn i_07b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B52E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B52E: move.w #$77, $70(a0)
pub fn i_07b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B534;
    cpu.ir = 0x317C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B534: move.w #$1a8, $72(a0)
pub fn i_07b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B53A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B53A: lea.l $7aae0(pc), a1
pub fn i_07b53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B53E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B53E: jsr $4ae.l
pub fn i_07b53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B544;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B544);
    cpu.pc = 0x4AE;
    16
}
/// $07B544: jsr $5dd02.l
pub fn i_07b544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B54A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B54A);
    cpu.pc = 0x5DD02;
    16
}
/// $07B54A: move.b #$1, $20(a0)
pub fn i_07b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B550;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B550: move.b #$0, $21(a0)
pub fn i_07b550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B556;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B556: move.w #$2, $74(a0)
pub fn i_07b556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B55C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B55C: move.w #$64, $70(a0)
pub fn i_07b55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B562;
    cpu.ir = 0x317C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B562: move.w #$1a8, $72(a0)
pub fn i_07b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B568;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B568: lea.l $7aae0(pc), a1
pub fn i_07b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B56C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B56C: jsr $4ae.l
pub fn i_07b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B572;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B572);
    cpu.pc = 0x4AE;
    16
}
/// $07B572: jsr $5dd02.l
pub fn i_07b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B578);
    cpu.pc = 0x5DD02;
    16
}
/// $07B578: move.b #$0, $20(a0)
pub fn i_07b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B57E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B57E: move.b #$0, $21(a0)
pub fn i_07b57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B584;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B584: move.w #$0, $74(a0)
pub fn i_07b584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B58A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B58A: move.w #$50, $70(a0)
pub fn i_07b58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B590;
    cpu.ir = 0x317C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B590: move.w #$1a8, $72(a0)
pub fn i_07b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B596;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B596: rts
pub fn i_07b596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B598;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLayout_CompleteFinal_07B598  ($07B598, 968 B, mission/banner_07a970.s)
/// $07B598: lea.l $7aae0(pc), a1
pub fn i_07b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B59C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B59C: jsr $4ae.l
pub fn i_07b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5A2);
    cpu.pc = 0x4AE;
    16
}
/// $07B5A2: jsr $5dd02.l
pub fn i_07b5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5A8);
    cpu.pc = 0x5DD02;
    16
}
/// $07B5A8: move.b #$c, $20(a0)
pub fn i_07b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5AE;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B5AE: move.b #$1, $21(a0)
pub fn i_07b5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5B4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B5B4: move.w #$28, $74(a0)
pub fn i_07b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5BA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5BA: move.w #$104, $70(a0)
pub fn i_07b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x104;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5C0: move.w #$168, $72(a0)
pub fn i_07b5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5C6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5C6: lea.l $7aae0(pc), a1
pub fn i_07b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B5CA: jsr $4ae.l
pub fn i_07b5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5D0);
    cpu.pc = 0x4AE;
    16
}
/// $07B5D0: jsr $5dd02.l
pub fn i_07b5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5D6);
    cpu.pc = 0x5DD02;
    16
}
/// $07B5D6: move.b #$a, $20(a0)
pub fn i_07b5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5DC;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B5DC: move.b #$1, $21(a0)
pub fn i_07b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5E2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B5E2: move.w #$26, $74(a0)
pub fn i_07b5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5E8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5E8: move.w #$ee, $70(a0)
pub fn i_07b5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5EE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5EE: move.w #$168, $72(a0)
pub fn i_07b5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5F4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5F4: lea.l $7aae0(pc), a1
pub fn i_07b5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B5F8: jsr $4ae.l
pub fn i_07b5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5FE);
    cpu.pc = 0x4AE;
    16
}
/// $07B5FE: jsr $5dd02.l
pub fn i_07b5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B604);
    cpu.pc = 0x5DD02;
    16
}
/// $07B604: move.b #$b, $20(a0)
pub fn i_07b604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B60A;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B60A: move.b #$1, $21(a0)
pub fn i_07b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B610;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B610: move.w #$24, $74(a0)
pub fn i_07b610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B616;
    cpu.ir = 0x317C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B616: move.w #$d6, $70(a0)
pub fn i_07b616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B61C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B61C: move.w #$168, $72(a0)
pub fn i_07b61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B622;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B622: lea.l $7aae0(pc), a1
pub fn i_07b622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B626;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B626: jsr $4ae.l
pub fn i_07b626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B62C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B62C);
    cpu.pc = 0x4AE;
    16
}
/// $07B62C: jsr $5dd02.l
pub fn i_07b62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B632;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B632);
    cpu.pc = 0x5DD02;
    16
}
/// $07B632: move.b #$a, $20(a0)
pub fn i_07b632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B638;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B638: move.b #$1, $21(a0)
pub fn i_07b638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B63E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B63E: move.w #$22, $74(a0)
pub fn i_07b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B644;
    cpu.ir = 0x317C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B644: move.w #$be, $70(a0)
pub fn i_07b644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B64A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xBE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B64A: move.w #$168, $72(a0)
pub fn i_07b64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B650;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B650: lea.l $7aae0(pc), a1
pub fn i_07b650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B654;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B654: jsr $4ae.l
pub fn i_07b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B65A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B65A);
    cpu.pc = 0x4AE;
    16
}
/// $07B65A: jsr $5dd02.l
pub fn i_07b65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B660;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B660);
    cpu.pc = 0x5DD02;
    16
}
/// $07B660: move.b #$9, $20(a0)
pub fn i_07b660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B666;
    cpu.ir = 0x117C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B666: move.b #$1, $21(a0)
pub fn i_07b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B66C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B66C: move.w #$20, $74(a0)
pub fn i_07b66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B672;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B672: move.w #$a7, $70(a0)
pub fn i_07b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B678;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B678: move.w #$168, $72(a0)
pub fn i_07b678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B67E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B67E: lea.l $7aae0(pc), a1
pub fn i_07b67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B682;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B682: jsr $4ae.l
pub fn i_07b682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B688);
    cpu.pc = 0x4AE;
    16
}
/// $07B688: jsr $5dd02.l
pub fn i_07b688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B68E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B68E);
    cpu.pc = 0x5DD02;
    16
}
/// $07B68E: move.b #$8, $20(a0)
pub fn i_07b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B694;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B694: move.b #$1, $21(a0)
pub fn i_07b694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B69A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B69A: move.w #$1e, $74(a0)
pub fn i_07b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6A0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6A0: move.w #$8f, $70(a0)
pub fn i_07b6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6A6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6A6: move.w #$168, $72(a0)
pub fn i_07b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6AC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6AC: lea.l $7aae0(pc), a1
pub fn i_07b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B6B0: jsr $4ae.l
pub fn i_07b6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B6B6);
    cpu.pc = 0x4AE;
    16
}
/// $07B6B6: jsr $5dd02.l
pub fn i_07b6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B6BC);
    cpu.pc = 0x5DD02;
    16
}
/// $07B6BC: move.b #$0, $20(a0)
pub fn i_07b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6C2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B6C2: move.b #$1, $21(a0)
pub fn i_07b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6C8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B6C8: move.w #$1c, $74(a0)
pub fn i_07b6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6CE: move.w #$76, $70(a0)
pub fn i_07b6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6D4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6D4: move.w #$168, $72(a0)
pub fn i_07b6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6DA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6DA: lea.l $7aae0(pc), a1
pub fn i_07b6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B6DE: jsr $4ae.l
pub fn i_07b6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B6E4);
    cpu.pc = 0x4AE;
    16
}
/// $07B6E4: jsr $5dd02.l
pub fn i_07b6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B6EA);
    cpu.pc = 0x5DD02;
    16
}
/// $07B6EA: move.b #$3, $20(a0)
pub fn i_07b6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6F0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B6F0: move.b #$1, $21(a0)
pub fn i_07b6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6F6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B6F6: move.w #$1a, $74(a0)
pub fn i_07b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6FC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6FC: move.w #$5d, $70(a0)
pub fn i_07b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B702;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B702: move.w #$168, $72(a0)
pub fn i_07b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B708;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B708: lea.l $7aae0(pc), a1
pub fn i_07b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B70C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B70C: jsr $4ae.l
pub fn i_07b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B712;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B712);
    cpu.pc = 0x4AE;
    16
}
/// $07B712: jsr $5dd02.l
pub fn i_07b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B718);
    cpu.pc = 0x5DD02;
    16
}
/// $07B718: move.b #$7, $20(a0)
pub fn i_07b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B71E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B71E: move.b #$1, $21(a0)
pub fn i_07b71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B724;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B724: move.w #$18, $74(a0)
pub fn i_07b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B72A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B72A: move.w #$44, $70(a0)
pub fn i_07b72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B730;
    cpu.ir = 0x317C;
    let v1: u32 = 0x44;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B730: move.w #$168, $72(a0)
pub fn i_07b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B736;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B736: lea.l $7aae0(pc), a1
pub fn i_07b736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B73A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B73A: jsr $4ae.l
pub fn i_07b73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B740);
    cpu.pc = 0x4AE;
    16
}
/// $07B740: jsr $5dd02.l
pub fn i_07b740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B746);
    cpu.pc = 0x5DD02;
    16
}
/// $07B746: move.b #$4, $20(a0)
pub fn i_07b746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B74C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B74C: move.b #$0, $21(a0)
pub fn i_07b74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B752;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B752: move.w #$16, $74(a0)
pub fn i_07b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B758;
    cpu.ir = 0x317C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B758: move.w #$e0, $70(a0)
pub fn i_07b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B75E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B75E: move.w #$198, $72(a0)
pub fn i_07b75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B764;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B764: lea.l $7aae0(pc), a1
pub fn i_07b764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B768: jsr $4ae.l
pub fn i_07b768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B76E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B76E);
    cpu.pc = 0x4AE;
    16
}
/// $07B76E: jsr $5dd02.l
pub fn i_07b76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B774);
    cpu.pc = 0x5DD02;
    16
}
/// $07B774: move.b #$3, $20(a0)
pub fn i_07b774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B77A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B77A: move.b #$0, $21(a0)
pub fn i_07b77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B780;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B780: move.w #$14, $74(a0)
pub fn i_07b780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B786;
    cpu.ir = 0x317C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B786: move.w #$c7, $70(a0)
pub fn i_07b786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B78C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B78C: move.w #$198, $72(a0)
pub fn i_07b78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B792;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B792: lea.l $7aae0(pc), a1
pub fn i_07b792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B796;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B796: jsr $4ae.l
pub fn i_07b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B79C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B79C);
    cpu.pc = 0x4AE;
    16
}
/// $07B79C: jsr $5dd02.l
pub fn i_07b79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7A2);
    cpu.pc = 0x5DD02;
    16
}
/// $07B7A2: move.b #$1, $20(a0)
pub fn i_07b7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7A8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B7A8: move.b #$0, $21(a0)
pub fn i_07b7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7AE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B7AE: move.w #$12, $74(a0)
pub fn i_07b7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7B4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7B4: move.w #$b4, $70(a0)
pub fn i_07b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7BA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7BA: move.w #$198, $72(a0)
pub fn i_07b7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7C0: lea.l $7aae0(pc), a1
pub fn i_07b7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B7C4: jsr $4ae.l
pub fn i_07b7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7CA);
    cpu.pc = 0x4AE;
    16
}
/// $07B7CA: jsr $5dd02.l
pub fn i_07b7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7D0);
    cpu.pc = 0x5DD02;
    16
}
/// $07B7D0: move.b #$2, $20(a0)
pub fn i_07b7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7D6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B7D6: move.b #$0, $21(a0)
pub fn i_07b7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7DC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B7DC: move.w #$10, $74(a0)
pub fn i_07b7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7E2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7E2: move.w #$a0, $70(a0)
pub fn i_07b7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7E8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7E8: move.w #$198, $72(a0)
pub fn i_07b7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7EE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7EE: lea.l $7aae0(pc), a1
pub fn i_07b7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B7F2: jsr $4ae.l
pub fn i_07b7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7F8);
    cpu.pc = 0x4AE;
    16
}
/// $07B7F8: jsr $5dd02.l
pub fn i_07b7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7FE);
    cpu.pc = 0x5DD02;
    16
}
/// $07B7FE: move.b #$2, $20(a0)
pub fn i_07b7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B804;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B804: move.b #$0, $21(a0)
pub fn i_07b804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B80A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B80A: move.w #$e, $74(a0)
pub fn i_07b80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B810;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B810: move.w #$87, $70(a0)
pub fn i_07b810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B816;
    cpu.ir = 0x317C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B816: move.w #$198, $72(a0)
pub fn i_07b816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B81C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B81C: lea.l $7aae0(pc), a1
pub fn i_07b81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B820;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B820: jsr $4ae.l
pub fn i_07b820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B826);
    cpu.pc = 0x4AE;
    16
}
/// $07B826: jsr $5dd02.l
pub fn i_07b826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B82C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B82C);
    cpu.pc = 0x5DD02;
    16
}
/// $07B82C: move.b #$1, $20(a0)
pub fn i_07b82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B832;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B832: move.b #$0, $21(a0)
pub fn i_07b832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B838;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B838: move.w #$c, $74(a0)
pub fn i_07b838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B83E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B83E: move.w #$74, $70(a0)
pub fn i_07b83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B844;
    cpu.ir = 0x317C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B844: move.w #$198, $72(a0)
pub fn i_07b844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B84A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B84A: lea.l $7aae0(pc), a1
pub fn i_07b84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B84E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B84E: jsr $4ae.l
pub fn i_07b84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B854);
    cpu.pc = 0x4AE;
    16
}
/// $07B854: jsr $5dd02.l
pub fn i_07b854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B85A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B85A);
    cpu.pc = 0x5DD02;
    16
}
/// $07B85A: move.b #$0, $20(a0)
pub fn i_07b85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B860;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B860: move.b #$0, $21(a0)
pub fn i_07b860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B866;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B866: move.w #$a, $74(a0)
pub fn i_07b866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B86C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B86C: move.w #$60, $70(a0)
pub fn i_07b86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B872;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B872: move.w #$198, $72(a0)
pub fn i_07b872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B878;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B878: lea.l $7aae0(pc), a1
pub fn i_07b878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B87C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B87C: jsr $4ae.l
pub fn i_07b87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B882);
    cpu.pc = 0x4AE;
    16
}
/// $07B882: jsr $5dd02.l
pub fn i_07b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B888);
    cpu.pc = 0x5DD02;
    16
}
/// $07B888: move.b #$9, $20(a0)
pub fn i_07b888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B88E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B88E: move.b #$0, $21(a0)
pub fn i_07b88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B894;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B894: move.w #$8, $74(a0)
pub fn i_07b894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B89A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B89A: move.w #$cd, $70(a0)
pub fn i_07b89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8A0;
    cpu.ir = 0x317C;
    let v1: u32 = 0xCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8A0: move.w #$1b8, $72(a0)
pub fn i_07b8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8A6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8A6: lea.l $7aae0(pc), a1
pub fn i_07b8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B8AA: jsr $4ae.l
pub fn i_07b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B8B0);
    cpu.pc = 0x4AE;
    16
}
/// $07B8B0: jsr $5dd02.l
pub fn i_07b8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B8B6);
    cpu.pc = 0x5DD02;
    16
}
/// $07B8B6: move.b #$5, $20(a0)
pub fn i_07b8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8BC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B8BC: move.b #$0, $21(a0)
pub fn i_07b8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8C2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B8C2: move.w #$6, $74(a0)
pub fn i_07b8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8C8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8C8: move.w #$b4, $70(a0)
pub fn i_07b8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8CE: move.w #$1b8, $72(a0)
pub fn i_07b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8D4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8D4: lea.l $7aae0(pc), a1
pub fn i_07b8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B8D8: jsr $4ae.l
pub fn i_07b8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B8DE);
    cpu.pc = 0x4AE;
    16
}
/// $07B8DE: jsr $5dd02.l
pub fn i_07b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B8E4);
    cpu.pc = 0x5DD02;
    16
}
/// $07B8E4: move.b #$4, $20(a0)
pub fn i_07b8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8EA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B8EA: move.b #$0, $21(a0)
pub fn i_07b8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8F0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B8F0: move.w #$4, $74(a0)
pub fn i_07b8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8F6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8F6: move.w #$9c, $70(a0)
pub fn i_07b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8FC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8FC: move.w #$1b8, $72(a0)
pub fn i_07b8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B902;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B902: lea.l $7aae0(pc), a1
pub fn i_07b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B906;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B906: jsr $4ae.l
pub fn i_07b906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B90C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B90C);
    cpu.pc = 0x4AE;
    16
}
/// $07B90C: jsr $5dd02.l
pub fn i_07b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B912);
    cpu.pc = 0x5DD02;
    16
}
/// $07B912: move.b #$1, $20(a0)
pub fn i_07b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B918;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B918: move.b #$0, $21(a0)
pub fn i_07b918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B91E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B91E: move.w #$2, $74(a0)
pub fn i_07b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B924;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B924: move.w #$89, $70(a0)
pub fn i_07b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B92A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x89;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B92A: move.w #$1b8, $72(a0)
pub fn i_07b92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B930;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B930: lea.l $7aae0(pc), a1
pub fn i_07b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B934;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B934: jsr $4ae.l
pub fn i_07b934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B93A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B93A);
    cpu.pc = 0x4AE;
    16
}
/// $07B93A: jsr $5dd02.l
pub fn i_07b93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B940);
    cpu.pc = 0x5DD02;
    16
}
/// $07B940: move.b #$d, $20(a0)
pub fn i_07b940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B946;
    cpu.ir = 0x117C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B946: move.b #$0, $21(a0)
pub fn i_07b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B94C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B94C: move.w #$0, $74(a0)
pub fn i_07b94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B952;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B952: move.w #$76, $70(a0)
pub fn i_07b952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B958;
    cpu.ir = 0x317C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B958: move.w #$1b8, $72(a0)
pub fn i_07b958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B95E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B95E: rts
pub fn i_07b95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B960;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_Seek_07B960  ($07B960, 56 B, mission/banner_07a970.s)
/// $07B960: move.w $70(a6), d0
pub fn i_07b960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B964;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B964: move.w $72(a6), d1
pub fn i_07b964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B968;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B968: sub.w $22(a6), d0
pub fn i_07b968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B96C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07B96C: sub.w $24(a6), d1
pub fn i_07b96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B970;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07B970: cmpi.w #$0, d0
pub fn i_07b970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B974;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $07B974: bne.w $7b980
pub fn i_07b974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B978;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7B980; return 10; }
    8
}
/// $07B978: cmpi.w #$0, d1
pub fn i_07b978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B97C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $07B97C: beq.w $7b99e
pub fn i_07b97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B980;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7B99E; return 10; }
    8
}
/// $07B980: asl.w #$6, d0
pub fn i_07b980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B982;
    cpu.ir = 0xED40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $07B982: asl.w #$6, d1
pub fn i_07b982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B984;
    cpu.ir = 0xED41;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $07B984: move.w d0, $28(a6)
pub fn i_07b984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B988;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B988: move.w d1, $2a(a6)
pub fn i_07b988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B98C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B98C: bset.b #$6, $13(a6)
pub fn i_07b98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B992;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07B992: jsr $27cee.l
pub fn i_07b992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B998;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B998);
    cpu.pc = 0x27CEE;
    16
}
// ===== ClearXN_07b998  ($07B998, 6 B, thunks/ccr_helpers.c)
/// $07B998: andi.b #$ee, ccr
pub fn i_07b998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B99C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07B99C: rts
pub fn i_07b99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B99E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07b99e  ($07B99E, 6 B, thunks/ccr_helpers.c)
/// $07B99E: ori.b #$11, ccr
pub fn i_07b99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9A2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07B9A2: rts
pub fn i_07b9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_CheckSettle_07B9A4  ($07B9A4, 10 B, mission/banner_07a970.s)
/// $07B9A4: cmpi.w #$0, $78(a6)
pub fn i_07b9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9AA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $07B9AA: bne.w $7b9b4
pub fn i_07b9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7B9B4; return 10; }
    8
}
// ===== SetXN_07b9ae  ($07B9AE, 6 B, thunks/ccr_helpers.c)
/// $07B9AE: ori.b #$11, ccr
pub fn i_07b9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9B2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07B9B2: rts
pub fn i_07b9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_Approach_07B9B4  ($07B9B4, 48 B, mission/banner_07a970.s)
/// $07B9B4: move.w $70(a6), d2
pub fn i_07b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9B8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9B8: move.w $72(a6), d3
pub fn i_07b9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9BC;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9BC: move.w $22(a6), d0
pub fn i_07b9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9C0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9C0: move.w $24(a6), d1
pub fn i_07b9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9C4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9C4: jsr $5e23a.l
pub fn i_07b9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B9CA);
    cpu.pc = 0x5E23A;
    16
}
/// $07B9CA: cmpi.w #$30, d0
pub fn i_07b9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9CE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $07B9CE: bgt.w $7b9ea
pub fn i_07b9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7B9EA; return 10; }
    8
}
/// $07B9D2: move.w #$18, d0
pub fn i_07b9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9D6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07B9D6: sub.w d0, $78(a6)
pub fn i_07b9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9DA;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07B9DA: bpl.w $7b9ea
pub fn i_07b9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9DE;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7B9EA; return 10; }
    8
}
/// $07B9DE: move.w #$0, $78(a6)
pub fn i_07b9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_07b9e4  ($07B9E4, 6 B, thunks/ccr_helpers.c)
/// $07B9E4: ori.b #$11, ccr
pub fn i_07b9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9E8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07B9E8: rts
pub fn i_07b9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_Blink_07B9EA  ($07B9EA, 62 B, mission/banner_07a970.s)
/// $07B9EA: move.w $78(a6), d0
pub fn i_07b9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9EE: btst.b #$8, d0
pub fn i_07b9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9F2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07B9F2: bne.w $7b9f8
pub fn i_07b9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7B9F8; return 10; }
    8
}
/// $07B9F6: neg.b d0
pub fn i_07b9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9F8;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07B9F8: move.b d0, $32(a6)
pub fn i_07b9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9FC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B9FC: move.w $78(a6), d0
pub fn i_07b9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA00;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA00: btst.b #$8, d0
pub fn i_07ba00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA04;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07BA04: bne.w $7ba28
pub fn i_07ba04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BA28; return 10; }
    8
}
/// $07BA08: cmpi.b #$0, $7a(a6)
pub fn i_07ba08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA0E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07BA0E: bne.w $7ba28
pub fn i_07ba0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA12;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BA28; return 10; }
    8
}
/// $07BA12: move.w #$1aa, d1
pub fn i_07ba12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA16;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BA16: jsr $236e.l
pub fn i_07ba16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BA1C);
    cpu.pc = 0x236E;
    16
}
/// $07BA1C: move.b #$ff, $7a(a6)
pub fn i_07ba1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA22;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BA22: move.b #$0, $3a(a6)
pub fn i_07ba22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA28;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_07ba28  ($07BA28, 6 B, thunks/ccr_helpers.c)
/// $07BA28: andi.b #$ee, ccr
pub fn i_07ba28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA2C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07BA2C: rts
pub fn i_07ba2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_LetterTimingFromDist_07ba2e  ($07BA2E, 50 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BA2E: move.w $70(a6), d2
pub fn i_07ba2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA32;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA32: move.w $72(a6), d3
pub fn i_07ba32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA36;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA36: move.w $22(a6), d0
pub fn i_07ba36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA3A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA3A: move.w $24(a6), d1
pub fn i_07ba3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA3E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA3E: jsr $5e23a.l
pub fn i_07ba3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BA44);
    cpu.pc = 0x5E23A;
    16
}
/// $07BA44: asr.w #$4, d0
pub fn i_07ba44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA46;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07BA46: clr.w d1
pub fn i_07ba46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA48;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07BA48: move.b $32(a6), d1
pub fn i_07ba48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA4C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07BA4C: sub.w d0, d1
pub fn i_07ba4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA4E;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07BA4E: move.b d1, $32(a6)
pub fn i_07ba4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA52;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BA52: clr.w d1
pub fn i_07ba52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA54;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07BA54: move.b $33(a6), d1
pub fn i_07ba54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA58;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07BA58: sub.w d0, d1
pub fn i_07ba58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA5A;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07BA5A: move.b d1, $33(a6)
pub fn i_07ba5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA5E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BA5E: rts
pub fn i_07ba5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss2Intro_RetGate_07ba60  ($07BA60, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BA60: movea.l $8(a6), a1
pub fn i_07ba60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA64;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07BA64: move.b $10(a6), d0
pub fn i_07ba64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA68;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07BA68: cmp.b $10(a1), d0
pub fn i_07ba68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA6C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07BA6C: bcs.w $7ba76
pub fn i_07ba6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA70;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7BA76; return 10; }
    8
}
// ===== ClearXN_07ba70  ($07BA70, 6 B, thunks/ccr_helpers.c)
/// $07BA70: andi.b #$ee, ccr
pub fn i_07ba70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA74;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07BA74: rts
pub fn i_07ba74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07ba76  ($07BA76, 6 B, thunks/ccr_helpers.c)
/// $07BA76: ori.b #$11, ccr
pub fn i_07ba76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA7A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07BA7A: rts
pub fn i_07ba7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Tmpl132_07ba7c  ($07BA7C, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BA7C: move.w #$34, d1
pub fn i_07ba7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA80;
    cpu.ir = 0x323C;
    let v1: u32 = 0x34;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BA80: jsr $236e.l
pub fn i_07ba80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BA86);
    cpu.pc = 0x236E;
    16
}
/// $07BA86: move.w #$a90, d0
pub fn i_07ba86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA8A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BA8A: move.l #$100, d1
pub fn i_07ba8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA90;
    cpu.ir = 0x223C;
    let v1: u32 = 0x100;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07BA90: jsr $96a5a.l
pub fn i_07ba90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BA96);
    cpu.pc = 0x96A5A;
    16
}
/// $07BA96: subq.w #$1, d0
pub fn i_07ba96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA98;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07BA98: move.w d0, $22(a6)
pub fn i_07ba98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA9C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BA9C: move.w d1, $24(a6)
pub fn i_07ba9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAA0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BAA0: move.w #$0, d0
pub fn i_07baa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BAA4: jsr $28134.l
pub fn i_07baa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAAA);
    cpu.pc = 0x28134;
    16
}
/// $07BAAA: andi.w #$ffe3, $38(a6)
pub fn i_07baaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAB0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BAB0: ori.w #$1c, $38(a6)
pub fn i_07bab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAB6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BAB6: move.b #$1, $20(a6)
pub fn i_07bab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BABC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BABC: clr.b $21(a6)
pub fn i_07babc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAC0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07BAC0: jsr $267e2.l
pub fn i_07bac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAC6);
    cpu.pc = 0x267E2;
    16
}
// ===== SetTaskHandler_07bac6  ($07BAC6, 8 B, kernel/task_handlers.c)
/// $07BAC6: lea.l $7bace(pc), a1
pub fn i_07bac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BACA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BACE;
    16
}
/// $07BACA: move.l a1, (a6)
pub fn i_07baca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BACC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BACC: rts
pub fn i_07bacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BACE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_WaitScroll_07bace  ($07BACE, 68 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BACE: lea.l $7bc8e(pc), a1
pub fn i_07bace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BC8E;
    16
}
/// $07BAD2: jsr $4ae.l
pub fn i_07bad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAD8);
    cpu.pc = 0x4AE;
    16
}
/// $07BAD8: jsr $5dd02.l
pub fn i_07bad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BADE);
    cpu.pc = 0x5DD02;
    16
}
/// $07BADE: lea.l $2e0148.l, a0
pub fn i_07bade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0148;
    20
}
/// $07BAE4: jsr $28cd4.l
pub fn i_07bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAEA);
    cpu.pc = 0x28CD4;
    16
}
/// $07BAEA: lea.l $7baf0(pc), a1
pub fn i_07baea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BAF0;
    16
}
/// $07BAEE: move.l a1, (a6)
pub fn i_07baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BAF0: jsr $2783a.l
pub fn i_07baf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAF6);
    cpu.pc = 0x2783A;
    16
}
/// $07BAF6: jsr $28d70.l
pub fn i_07baf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAFC);
    cpu.pc = 0x28D70;
    16
}
/// $07BAFC: move.w $2bcf52.l, d0
pub fn i_07bafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB02;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x2BCF52) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $07BB02: cmp.w $106f50.l, d0
pub fn i_07bb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB08;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $07BB08: bgt.w $7bb18
pub fn i_07bb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB0C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7BB18; return 10; }
    8
}
/// $07BB0C: clr.b $10e39a.l
pub fn i_07bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB12;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
// ===== SetTaskHandler_07bb12  ($07BB12, 8 B, kernel/task_handlers.c)
/// $07BB12: lea.l $7bb1a(pc), a1
pub fn i_07bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BB1A;
    16
}
/// $07BB16: move.l a1, (a6)
pub fn i_07bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BB18: rts
pub fn i_07bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrRamWord_07bb4c  ($07BB4C, 8 B, thunks/clr_ram_word.c)
/// $07BB4C: clr.b $10e39a.l
pub fn i_07bb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB52;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $07BB52: rts
pub fn i_07bb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_WaitDeath_07bb54  ($07BB54, 26 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BB54: jsr $2783a.l
pub fn i_07bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB5A);
    cpu.pc = 0x2783A;
    16
}
/// $07BB5A: tst.b $21(a6)
pub fn i_07bb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB5E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07BB5E: beq.w $7bb70
pub fn i_07bb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BB70; return 10; }
    8
}
/// $07BB62: bclr.b #$1, $12(a6)
pub fn i_07bb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB68;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07BB68: jmp $518.l
pub fn i_07bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB6E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M2Boss_Rts_07bb6e  ($07BB6E, 2 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BB6E: rts
pub fn i_07bb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrRamWord_07bb70  ($07BB70, 8 B, thunks/clr_ram_word.c)
/// $07BB70: clr.b $10e39a.l
pub fn i_07bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB76;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $07BB76: rts
pub fn i_07bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Body_Init_07bb78  ($07BB78, 270 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BB78: move.w #$2a, d1
pub fn i_07bb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB7C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BB7C: jsr $236e.l
pub fn i_07bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB82);
    cpu.pc = 0x236E;
    16
}
/// $07BB82: move.w #$4, $1c(a6)
pub fn i_07bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB88;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BB88: jsr $138fe.l
pub fn i_07bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB8E);
    cpu.pc = 0x138FE;
    16
}
/// $07BB8E: move.w #$0, d0
pub fn i_07bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB92;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BB92: jsr $28134.l
pub fn i_07bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB98);
    cpu.pc = 0x28134;
    16
}
/// $07BB98: andi.w #$ffe3, $38(a6)
pub fn i_07bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB9E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BB9E: ori.w #$8, $38(a6)
pub fn i_07bb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBA4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BBA4: move.w #$1, $66(a6)
pub fn i_07bba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BBAA: jsr $267e2.l
pub fn i_07bbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BBB0);
    cpu.pc = 0x267E2;
    16
}
/// $07BBB0: clr.b $21(a6)
pub fn i_07bbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBB4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07BBB4: move.w #$ffff, $7c(a6)
pub fn i_07bbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBBA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BBBA: lea.l $7bbc0(pc), a1
pub fn i_07bbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BBC0;
    16
}
/// $07BBBE: move.l a1, (a6)
pub fn i_07bbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BBC0: movea.l $c(a6), a0
pub fn i_07bbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBC4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BBC4: move.w $22(a0), $22(a6)
pub fn i_07bbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBCA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07BBCA: move.w $24(a0), $24(a6)
pub fn i_07bbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBD0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07BBD0: movea.l $c(a6), a0
pub fn i_07bbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BBD4: move.w $7c(a0), d0
pub fn i_07bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBD8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BBD8: cmp.w $7c(a6), d0
pub fn i_07bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBDC;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07BBDC: beq.w $7bc00
pub fn i_07bbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBE0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BC00; return 10; }
    8
}
/// $07BBE0: move.l d0, -(a7)
pub fn i_07bbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBE2;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07BBE2: movea.l #$2df7d6, a0
pub fn i_07bbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBE8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DF7D6;
    cpu.a[0] = v1;
    4
}
/// $07BBE8: lsl.w #$2, d0
pub fn i_07bbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBEA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07BBEA: movea.l (a0, d0.w), a0
pub fn i_07bbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBEE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07BBEE: cmpa.l #$ffffffff, a0
pub fn i_07bbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBF4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07BBF4: beq.w $7bbfe
pub fn i_07bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBF8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BBFE; return 10; }
    8
}
/// $07BBF8: jsr $28cd4.l
pub fn i_07bbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BBFE);
    cpu.pc = 0x28CD4;
    16
}
/// $07BBFE: move.l (a7)+, d0
pub fn i_07bbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC00;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BC00: move.w d0, $7c(a6)
pub fn i_07bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC04;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BC04: movea.l $c(a6), a0
pub fn i_07bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC08;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BC08: btst.b #$7, $5a(a0)
pub fn i_07bc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC0E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07BC0E: beq.w $7bc18
pub fn i_07bc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC12;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BC18; return 10; }
    8
}
/// $07BC12: bset.b #$0, $5a(a6)
pub fn i_07bc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC18;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07BC18: jsr $28d70.l
pub fn i_07bc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BC1E);
    cpu.pc = 0x28D70;
    16
}
/// $07BC1E: movea.l $c(a6), a0
pub fn i_07bc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC22;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BC22: cmpi.b #$ff, $20(a0)
pub fn i_07bc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC28;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07BC28: bne.w $7bc78
pub fn i_07bc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC2C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BC78; return 10; }
    8
}
/// $07BC2C: cmpi.w #$140, $22(a6)
pub fn i_07bc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC32;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $07BC32: ble.w $7bc78
pub fn i_07bc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC36;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7BC78; return 10; }
    8
}
/// $07BC36: tst.b $21(a6)
pub fn i_07bc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC3A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07BC3A: bne.w $7bc78
pub fn i_07bc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC3E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BC78; return 10; }
    8
}
/// $07BC3E: move.b #$1, $10a2d1.l
pub fn i_07bc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC46;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $07BC46: jsr $434ce.l
pub fn i_07bc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BC4C);
    cpu.pc = 0x434CE;
    16
}
/// $07BC4C: move.b #$ff, $21(a6)
pub fn i_07bc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BC52: lea.l $2df7bc.l, a1
pub fn i_07bc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC58;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF7BC;
    20
}
/// $07BC58: jsr $77c7e.l
pub fn i_07bc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BC5E);
    cpu.pc = 0x77C7E;
    16
}
/// $07BC5E: move.w #$d000, $38(a0)
pub fn i_07bc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC64;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BC64: move.w $22(a6), -(a7)
pub fn i_07bc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC68;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $07BC68: subi.w #$20, $22(a6)
pub fn i_07bc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC6E;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BC6E: jsr $628ea.l
pub fn i_07bc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BC74);
    cpu.pc = 0x628EA;
    16
}
/// $07BC74: move.w (a7)+, $22(a6)
pub fn i_07bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC78;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    16
}
/// $07BC78: movea.l $c(a6), a0
pub fn i_07bc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC7C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BC7C: cmpi.b #$ff, $21(a0)
pub fn i_07bc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC82;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07BC82: bne.w $7bc8c
pub fn i_07bc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BC8C; return 10; }
    8
}
// ===== SetTaskHandler_07bc86  ($07BC86, 8 B, kernel/task_handlers.c)
/// $07BC86: lea.l $7c65c(pc), a1
pub fn i_07bc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07BC8A: move.l a1, (a6)
pub fn i_07bc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BC8C: rts
pub fn i_07bc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Body_SpawnTurretAndArm_07bc8e  ($07BC8E, 140 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BC8E: moveq #$0, d0
pub fn i_07bc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC90;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07BC90: move.b d0, $20(a6)
pub fn i_07bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC94;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BC94: move.b d0, $21(a6)
pub fn i_07bc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC98;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BC98: move.w d0, $70(a6)
pub fn i_07bc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC9C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BC9C: move.w d0, $72(a6)
pub fn i_07bc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCA0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BCA0: move.w d0, $74(a6)
pub fn i_07bca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCA4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BCA4: move.w d0, $76(a6)
pub fn i_07bca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCA8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BCA8: move.w d0, $78(a6)
pub fn i_07bca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCAC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BCAC: move.w d0, $7a(a6)
pub fn i_07bcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCB0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BCB0: lea.l $7bb78(pc), a1
pub fn i_07bcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BB78;
    16
}
/// $07BCB4: jsr $4ae.l
pub fn i_07bcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BCBA);
    cpu.pc = 0x4AE;
    16
}
/// $07BCBA: jsr $5dd02.l
pub fn i_07bcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BCC0);
    cpu.pc = 0x5DD02;
    16
}
/// $07BCC0: lea.l $7bf36(pc), a1
pub fn i_07bcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BF36;
    16
}
/// $07BCC4: jsr $4ae.l
pub fn i_07bcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BCCA);
    cpu.pc = 0x4AE;
    16
}
/// $07BCCA: jsr $5dd02.l
pub fn i_07bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BCD0);
    cpu.pc = 0x5DD02;
    16
}
/// $07BCD0: move.w #$0, $7c(a6)
pub fn i_07bcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BCD6: lea.l $7bcdc(pc), a1
pub fn i_07bcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BCDC;
    16
}
/// $07BCDA: move.l a1, (a6)
pub fn i_07bcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BCDC: jsr $7bef4(pc)
pub fn i_07bcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7BCE0);
    cpu.pc = 0x7BEF4;
    16
}
/// $07BCE0: cmpi.w #$ffff, $70(a6)
pub fn i_07bce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCE6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $07BCE6: bne.w $7bcf0
pub fn i_07bce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BCF0; return 10; }
    8
}
/// $07BCEA: lea.l $7bd22(pc), a1
pub fn i_07bcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BD22;
    16
}
/// $07BCEE: move.l a1, (a6)
pub fn i_07bcee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BCF0: cmpi.w #$1, $70(a6)
pub fn i_07bcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCF6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07BCF6: bne.w $7bd00
pub fn i_07bcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCFA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BD00; return 10; }
    8
}
/// $07BCFA: lea.l $7bdcc(pc), a1
pub fn i_07bcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BCFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BDCC;
    16
}
/// $07BCFE: move.l a1, (a6)
pub fn i_07bcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BD00: cmpi.w #$2, $70(a6)
pub fn i_07bd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD06;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $07BD06: bne.w $7bd10
pub fn i_07bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BD10; return 10; }
    8
}
/// $07BD0A: lea.l $7be7c(pc), a1
pub fn i_07bd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BE7C;
    16
}
/// $07BD0E: move.l a1, (a6)
pub fn i_07bd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BD10: cmpi.b #$ff, $20(a6)
pub fn i_07bd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD16;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07BD16: bne.w $7bd20
pub fn i_07bd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BD20; return 10; }
    8
}
// ===== SetTaskHandler_07bd1a  ($07BD1A, 8 B, kernel/task_handlers.c)
/// $07BD1A: lea.l $7c374(pc), a1
pub fn i_07bd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C374;
    16
}
/// $07BD1E: move.l a1, (a6)
pub fn i_07bd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BD20: rts
pub fn i_07bd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Arm_SwingLeft_07bd22  ($07BD22, 162 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BD22: move.w #$1, $7c(a6)
pub fn i_07bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD28;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BD28: lea.l $7bd2e(pc), a1
pub fn i_07bd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BD2E;
    16
}
/// $07BD2C: move.l a1, (a6)
pub fn i_07bd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BD2E: cmpi.w #$ffff, $70(a6)
pub fn i_07bd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD34;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $07BD34: beq.w $7bd58
pub fn i_07bd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BD58; return 10; }
    8
}
/// $07BD38: tst.w $78(a6)
pub fn i_07bd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD3C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07BD3C: bne.w $7bd5c
pub fn i_07bd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BD5C; return 10; }
    8
}
/// $07BD40: subi.w #$a0, $74(a6)
pub fn i_07bd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD46;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BD46: bpl.w $7bd72
pub fn i_07bd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD4A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7BD72; return 10; }
    8
}
/// $07BD4A: clr.w $74(a6)
pub fn i_07bd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD4E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BD4E: lea.l $7bcd0(pc), a1
pub fn i_07bd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BCD0;
    16
}
/// $07BD52: move.l a1, (a6)
pub fn i_07bd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BD54: bra.w $7bd72
pub fn i_07bd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD58;
    cpu.ir = 0x6000;
    cpu.pc = 0x7BD72;
    10
}
/// $07BD58: clr.w $78(a6)
pub fn i_07bd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD5C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BD5C: addi.w #$40, $74(a6)
pub fn i_07bd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD62;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BD62: cmpi.w #$180, $74(a6)
pub fn i_07bd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD68;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $07BD68: blt.w $7bd72
pub fn i_07bd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD6C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7BD72; return 10; }
    8
}
/// $07BD6C: move.w #$180, $74(a6)
pub fn i_07bd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD72;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BD72: move.w $74(a6), d0
pub fn i_07bd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD76;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BD76: sub.w d0, $72(a6)
pub fn i_07bd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD7A;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BD7A: bpl.w $7bdac
pub fn i_07bd7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD7E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7BDAC; return 10; }
    8
}
/// $07BD7E: move.w #$0, $72(a6)
pub fn i_07bd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD84;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BD84: addq.w #$1, $76(a6)
pub fn i_07bd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD88;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BD88: cmpi.w #$14, $76(a6)
pub fn i_07bd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD8E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $07BD8E: blt.w $7bdb0
pub fn i_07bd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD92;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7BDB0; return 10; }
    8
}
/// $07BD92: moveq #$0, d0
pub fn i_07bd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD94;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07BD94: move.w d0, $74(a6)
pub fn i_07bd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD98;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BD98: move.w d0, $76(a6)
pub fn i_07bd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BD9C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BD9C: move.w #$ffff, $78(a6)
pub fn i_07bd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BDA2: lea.l $7bdcc(pc), a1
pub fn i_07bda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BDCC;
    16
}
/// $07BDA6: move.l a1, (a6)
pub fn i_07bda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BDA8: bra.w $7bdb0
pub fn i_07bda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x7BDB0;
    10
}
/// $07BDAC: clr.w $76(a6)
pub fn i_07bdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDB0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BDB0: jsr $7bef4(pc)
pub fn i_07bdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7BDB4);
    cpu.pc = 0x7BEF4;
    16
}
/// $07BDB4: jsr $28d70.l
pub fn i_07bdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BDBA);
    cpu.pc = 0x28D70;
    16
}
/// $07BDBA: cmpi.b #$ff, $20(a6)
pub fn i_07bdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDC0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07BDC0: bne.w $7bdca
pub fn i_07bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BDCA; return 10; }
    8
}
// ===== SetTaskHandler_07bdc4  ($07BDC4, 8 B, kernel/task_handlers.c)
/// $07BDC4: lea.l $7c374(pc), a1
pub fn i_07bdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C374;
    16
}
/// $07BDC8: move.l a1, (a6)
pub fn i_07bdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BDCA: rts
pub fn i_07bdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDCC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Arm_SwingRight_07bdcc  ($07BDCC, 168 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BDCC: move.w #$2, $7c(a6)
pub fn i_07bdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDD2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BDD2: lea.l $7bdd8(pc), a1
pub fn i_07bdd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BDD8;
    16
}
/// $07BDD6: move.l a1, (a6)
pub fn i_07bdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BDD8: cmpi.w #$1, $70(a6)
pub fn i_07bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDDE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07BDDE: beq.w $7be02
pub fn i_07bdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDE2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BE02; return 10; }
    8
}
/// $07BDE2: tst.w $78(a6)
pub fn i_07bde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDE6;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07BDE6: bne.w $7be06
pub fn i_07bde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BE06; return 10; }
    8
}
/// $07BDEA: subi.w #$a0, $74(a6)
pub fn i_07bdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDF0;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BDF0: bpl.w $7be1c
pub fn i_07bdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDF4;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7BE1C; return 10; }
    8
}
/// $07BDF4: clr.w $74(a6)
pub fn i_07bdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDF8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BDF8: lea.l $7bcd0(pc), a1
pub fn i_07bdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BCD0;
    16
}
/// $07BDFC: move.l a1, (a6)
pub fn i_07bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BDFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BDFE: bra.w $7be1c
pub fn i_07bdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE02;
    cpu.ir = 0x6000;
    cpu.pc = 0x7BE1C;
    10
}
/// $07BE02: clr.w $78(a6)
pub fn i_07be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE06;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BE06: addi.w #$40, $74(a6)
pub fn i_07be06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE0C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BE0C: cmpi.w #$180, $74(a6)
pub fn i_07be0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE12;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $07BE12: blt.w $7be1c
pub fn i_07be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE16;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7BE1C; return 10; }
    8
}
/// $07BE16: move.w #$180, $74(a6)
pub fn i_07be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE1C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BE1C: move.w $74(a6), d0
pub fn i_07be1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE20;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BE20: add.w d0, $72(a6)
pub fn i_07be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE24;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BE24: cmpi.w #$7e00, $72(a6)
pub fn i_07be24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE2A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7E00, Size::Word, false);
    16
}
/// $07BE2A: ble.w $7be5c
pub fn i_07be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE2E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7BE5C; return 10; }
    8
}
/// $07BE2E: move.w #$7e00, $72(a6)
pub fn i_07be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE34;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7E00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BE34: addq.w #$1, $76(a6)
pub fn i_07be34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE38;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BE38: cmpi.w #$14, $76(a6)
pub fn i_07be38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE3E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $07BE3E: blt.w $7be60
pub fn i_07be3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE42;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7BE60; return 10; }
    8
}
/// $07BE42: moveq #$0, d0
pub fn i_07be42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE44;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07BE44: move.w d0, $74(a6)
pub fn i_07be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE48;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BE48: move.w d0, $76(a6)
pub fn i_07be48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE4C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BE4C: move.w #$ffff, $78(a6)
pub fn i_07be4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE52;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BE52: lea.l $7bd22(pc), a1
pub fn i_07be52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BD22;
    16
}
/// $07BE56: move.l a1, (a6)
pub fn i_07be56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BE58: bra.w $7be60
pub fn i_07be58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7BE60;
    10
}
/// $07BE5C: clr.w $76(a6)
pub fn i_07be5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE60;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BE60: jsr $7bef4(pc)
pub fn i_07be60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7BE64);
    cpu.pc = 0x7BEF4;
    16
}
/// $07BE64: jsr $28d70.l
pub fn i_07be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BE6A);
    cpu.pc = 0x28D70;
    16
}
/// $07BE6A: cmpi.b #$ff, $20(a6)
pub fn i_07be6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE70;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07BE70: bne.w $7be7a
pub fn i_07be70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BE7A; return 10; }
    8
}
// ===== SetTaskHandler_07be74  ($07BE74, 8 B, kernel/task_handlers.c)
/// $07BE74: lea.l $7c374(pc), a1
pub fn i_07be74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C374;
    16
}
/// $07BE78: move.l a1, (a6)
pub fn i_07be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BE7A: rts
pub fn i_07be7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Arm_SwingFar_07be7c  ($07BE7C, 112 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BE7C: move.w #$2, $7c(a6)
pub fn i_07be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BE82: lea.l $7be88(pc), a1
pub fn i_07be82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BE88;
    16
}
/// $07BE86: move.l a1, (a6)
pub fn i_07be86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BE88: cmpi.w #$2, $70(a6)
pub fn i_07be88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE8E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $07BE8E: beq.w $7beaa
pub fn i_07be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BEAA; return 10; }
    8
}
/// $07BE92: subi.w #$a0, $74(a6)
pub fn i_07be92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE98;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BE98: bpl.w $7bec0
pub fn i_07be98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BE9C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7BEC0; return 10; }
    8
}
/// $07BE9C: clr.w $74(a6)
pub fn i_07be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEA0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BEA0: lea.l $7bcd0(pc), a1
pub fn i_07bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BCD0;
    16
}
/// $07BEA4: move.l a1, (a6)
pub fn i_07bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BEA6: bra.w $7bec0
pub fn i_07bea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x7BEC0;
    10
}
/// $07BEAA: addi.w #$40, $74(a6)
pub fn i_07beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEB0;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BEB0: cmpi.w #$140, $74(a6)
pub fn i_07beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEB6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $07BEB6: blt.w $7bec0
pub fn i_07beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEBA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7BEC0; return 10; }
    8
}
/// $07BEBA: move.w #$140, $74(a6)
pub fn i_07beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BEC0: move.w $74(a6), d0
pub fn i_07bec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEC4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BEC4: add.w d0, $72(a6)
pub fn i_07bec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEC8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BEC8: cmpi.w #$7e00, $72(a6)
pub fn i_07bec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BECE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7E00, Size::Word, false);
    16
}
/// $07BECE: ble.w $7bed8
pub fn i_07bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BED2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7BED8; return 10; }
    8
}
/// $07BED2: move.w #$7e00, $72(a6)
pub fn i_07bed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BED8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7E00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BED8: jsr $7bef4(pc)
pub fn i_07bed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7BEDC);
    cpu.pc = 0x7BEF4;
    16
}
/// $07BEDC: jsr $28d70.l
pub fn i_07bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BEE2);
    cpu.pc = 0x28D70;
    16
}
/// $07BEE2: cmpi.b #$ff, $20(a6)
pub fn i_07bee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEE8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07BEE8: bne.w $7bef2
pub fn i_07bee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BEF2; return 10; }
    8
}
// ===== SetTaskHandler_07beec  ($07BEEC, 8 B, kernel/task_handlers.c)
/// $07BEEC: lea.l $7c374(pc), a1
pub fn i_07beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C374;
    16
}
/// $07BEF0: move.l a1, (a6)
pub fn i_07bef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BEF2: rts
pub fn i_07bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Arm_PosFromAngle_07bef4  ($07BEF4, 66 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BEF4: movea.l $c(a6), a0
pub fn i_07bef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEF8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BEF8: move.w $22(a0), $22(a6)
pub fn i_07bef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BEFE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07BEFE: move.w $24(a0), $24(a6)
pub fn i_07befe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF04;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07BF04: subi.w #$60, $22(a6)
pub fn i_07bf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF0A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BF0A: lea.l $2df8a4.l, a0
pub fn i_07bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF8A4;
    20
}
/// $07BF10: move.w $72(a6), d0
pub fn i_07bf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF14;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BF14: lsr.w #$8, d0
pub fn i_07bf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF16;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07BF16: andi.w #$ff, d0
pub fn i_07bf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF1A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07BF1A: add.w d0, d0
pub fn i_07bf1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF1C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07BF1C: move.b (a0, d0.w), d1
pub fn i_07bf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF20;
    cpu.ir = 0x1230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $07BF20: andi.w #$ff, d1
pub fn i_07bf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF24;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07BF24: add.w d1, $22(a6)
pub fn i_07bf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF28;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BF28: move.b $1(a0, d0.w), d1
pub fn i_07bf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF2C;
    cpu.ir = 0x1230;
    let ad2 = cpu.a[0].wrapping_add(0x1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $07BF2C: andi.w #$ff, d1
pub fn i_07bf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF30;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07BF30: sub.w d1, $24(a6)
pub fn i_07bf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF34;
    cpu.ir = 0x936E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07BF34: rts
pub fn i_07bf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_Init_07bf36  ($07BF36, 306 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BF36: move.w #$2a, d1
pub fn i_07bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF3A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BF3A: jsr $236e.l
pub fn i_07bf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BF40);
    cpu.pc = 0x236E;
    16
}
/// $07BF40: move.w #$4, $1c(a6)
pub fn i_07bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF46;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BF46: jsr $138fe.l
pub fn i_07bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BF4C);
    cpu.pc = 0x138FE;
    16
}
/// $07BF4C: bset.b #$1, $12(a6)
pub fn i_07bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF52;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07BF52: move.w #$0, d0
pub fn i_07bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF56;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BF56: jsr $28134.l
pub fn i_07bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BF5C);
    cpu.pc = 0x28134;
    16
}
/// $07BF5C: andi.w #$ffe3, $38(a6)
pub fn i_07bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF62;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BF62: ori.w #$14, $38(a6)
pub fn i_07bf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF68;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BF68: lea.l $2bcf54.l, a0
pub fn i_07bf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCF54;
    20
}
/// $07BF6E: jsr $799de.l
pub fn i_07bf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BF74);
    cpu.pc = 0x799DE;
    16
}
/// $07BF74: move.w d0, $66(a6)
pub fn i_07bf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF78;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BF78: move.w d0, $7c(a6)
pub fn i_07bf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF7C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BF7C: jsr $267e2.l
pub fn i_07bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BF82);
    cpu.pc = 0x267E2;
    16
}
/// $07BF82: moveq #$0, d0
pub fn i_07bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF84;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07BF84: move.b d0, $20(a6)
pub fn i_07bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF88;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BF88: move.b d0, $21(a6)
pub fn i_07bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF8C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BF8C: move.w #$3, $70(a6)
pub fn i_07bf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF92;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BF92: move.w d0, $72(a6)
pub fn i_07bf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF96;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BF96: move.w d0, $74(a6)
pub fn i_07bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF9A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BF9A: move.w d0, $76(a6)
pub fn i_07bf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BF9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BF9E: move.w d0, $78(a6)
pub fn i_07bf9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFA2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BFA2: move.w d0, $7a(a6)
pub fn i_07bfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFA6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BFA6: move.w d0, $7e(a6)
pub fn i_07bfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFAA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BFAA: move.w d0, $80(a6)
pub fn i_07bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFAE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BFAE: move.w d0, $82(a6)
pub fn i_07bfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFB2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BFB2: lea.l $7c1b6(pc), a1
pub fn i_07bfb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C1B6;
    16
}
/// $07BFB6: jsr $4ae.l
pub fn i_07bfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BFBC);
    cpu.pc = 0x4AE;
    16
}
/// $07BFBC: jsr $5dd02.l
pub fn i_07bfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BFC2);
    cpu.pc = 0x5DD02;
    16
}
/// $07BFC2: bset.b #$3, $13(a6)
pub fn i_07bfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFC8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07BFC8: lea.l $7bfce(pc), a1
pub fn i_07bfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BFCE;
    16
}
/// $07BFCC: move.l a1, (a6)
pub fn i_07bfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BFCE: movea.l $c(a6), a0
pub fn i_07bfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFD2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BFD2: move.w $22(a0), $22(a6)
pub fn i_07bfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFD8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07BFD8: move.w $24(a0), $24(a6)
pub fn i_07bfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFDE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07BFDE: lea.l $2df850.l, a0
pub fn i_07bfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF850;
    20
}
/// $07BFE4: move.w $70(a6), d0
pub fn i_07bfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFE8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BFE8: add.w d0, d0
pub fn i_07bfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFEA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07BFEA: add.w d0, d0
pub fn i_07bfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFEC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07BFEC: movea.l (a0, d0.w), a0
pub fn i_07bfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFF0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07BFF0: jsr $28cd4.l
pub fn i_07bff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BFF6);
    cpu.pc = 0x28CD4;
    16
}
/// $07BFF6: jsr $28d70.l
pub fn i_07bff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BFFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BFFC);
    cpu.pc = 0x28D70;
    16
}
/// $07BFFC: btst.b #$7, $5a(a6)
pub fn i_07bffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C002;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07C002: beq.w $7c014
pub fn i_07c002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C006;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C014; return 10; }
    8
}
/// $07C006: move.l a6, -(a7)
pub fn i_07c006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C008;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07C008: movea.l $c(a6), a6
pub fn i_07c008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C00C;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $07C00C: bset.b #$7, $5a(a6)
pub fn i_07c00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C012;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C012: movea.l (a7)+, a6
pub fn i_07c012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C014;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $07C014: movea.l $c(a6), a0
pub fn i_07c014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C018;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C018: movea.l $c(a0), a0
pub fn i_07c018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C01C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C01C: tst.b $20(a0)
pub fn i_07c01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C020;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07C020: bne.w $7c070
pub fn i_07c020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C024;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C070; return 10; }
    8
}
/// $07C024: bclr.b #$3, $13(a6)
pub fn i_07c024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C02A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C02A: jsr $7c512(pc)
pub fn i_07c02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C02E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7C02E);
    cpu.pc = 0x7C512;
    16
}
/// $07C02E: bcc.w $7c038
pub fn i_07c02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C032;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C038; return 10; }
    8
}
/// $07C032: lea.l $7c106(pc), a1
pub fn i_07c032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C036;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C106;
    16
}
/// $07C036: move.l a1, (a6)
pub fn i_07c036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C038;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C038: jsr $2870a.l
pub fn i_07c038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C03E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C03E);
    cpu.pc = 0x2870A;
    16
}
/// $07C03E: lea.l $5e766.l, a0
pub fn i_07c03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C044;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07C044: jsr $5e770.l
pub fn i_07c044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C04A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C04A);
    cpu.pc = 0x5E770;
    16
}
/// $07C04A: jsr $28758.l
pub fn i_07c04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C050;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C050);
    cpu.pc = 0x28758;
    16
}
/// $07C050: bcc.w $7c05a
pub fn i_07c050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C054;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C05A; return 10; }
    8
}
/// $07C054: lea.l $7c47a(pc), a1
pub fn i_07c054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C058;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C47A;
    16
}
/// $07C058: move.l a1, (a6)
pub fn i_07c058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C05A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C05A: movea.l $c(a6), a0
pub fn i_07c05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C05E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C05E: cmpi.b #$ff, $21(a0)
pub fn i_07c05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C064;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07C064: bne.w $7c06e
pub fn i_07c064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C068;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C06E; return 10; }
    8
}
// ===== SetTaskHandler_07c068  ($07C068, 8 B, kernel/task_handlers.c)
/// $07C068: lea.l $7c65c(pc), a1
pub fn i_07c068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C06C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C06C: move.l a1, (a6)
pub fn i_07c06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C06E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C06E: rts
pub fn i_07c06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C070;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_ParentPhase_07c070  ($07C070, 142 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C070: cmpi.b #$1, $20(a0)
pub fn i_07c070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C076;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07C076: beq.b $7c06e
pub fn i_07c076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C078;
    cpu.ir = 0x67F6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C06E; return 10; }
    8
}
/// $07C078: bclr.b #$3, $13(a6)
pub fn i_07c078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C07E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C07E: move.w $7c(a6), $66(a6)
pub fn i_07c07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C084;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C084: move.l a0, -(a7)
pub fn i_07c084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C086;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07C086: jsr $2870a.l
pub fn i_07c086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C08C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C08C);
    cpu.pc = 0x2870A;
    16
}
/// $07C08C: lea.l $5e766.l, a0
pub fn i_07c08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C092;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07C092: jsr $5e770.l
pub fn i_07c092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C098);
    cpu.pc = 0x5E770;
    16
}
/// $07C098: movea.l (a7)+, a0
pub fn i_07c098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C09A;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $07C09A: cmpi.b #$2, $20(a0)
pub fn i_07c09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0A0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $07C0A0: bne.w $7c0b0
pub fn i_07c0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C0B0; return 10; }
    8
}
/// $07C0A4: movea.l $c(a6), a0
pub fn i_07c0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0A8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C0A8: move.w #$2, $70(a0)
pub fn i_07c0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0AE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C0AE: rts
pub fn i_07c0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07C0B0: cmpi.b #$3, $20(a0)
pub fn i_07c0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0B6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07C0B6: bne.b $7c06e
pub fn i_07c0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0B8;
    cpu.ir = 0x66B6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C06E; return 10; }
    8
}
/// $07C0B8: movea.l $c(a6), a0
pub fn i_07c0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0BC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C0BC: clr.w $70(a0)
pub fn i_07c0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0C0;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C0C0: tst.w $80(a6)
pub fn i_07c0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0C4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07C0C4: bne.w $7c0d8
pub fn i_07c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C0D8; return 10; }
    8
}
/// $07C0C8: move.w #$ffff, $80(a6)
pub fn i_07c0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C0CE: move.w #$1074, d0
pub fn i_07c0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1074;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C0D2: jsr $2352.l
pub fn i_07c0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C0D8);
    cpu.pc = 0x2352;
    16
}
/// $07C0D8: addi.w #$1, $7a(a6)
pub fn i_07c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0DE;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C0DE: cmpi.w #$5, $7a(a6)
pub fn i_07c0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0E4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Word, false);
    16
}
/// $07C0E4: blt.b $7c06e
pub fn i_07c0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0E6;
    cpu.ir = 0x6D88;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C06E; return 10; }
    8
}
/// $07C0E6: clr.w $7a(a6)
pub fn i_07c0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C0EA: addq.w #$1, $70(a6)
pub fn i_07c0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0EE;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C0EE: cmpi.w #$7, $70(a6)
pub fn i_07c0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0F4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Word, false);
    16
}
/// $07C0F4: blt.w $7c06e
pub fn i_07c0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0F8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C06E; return 10; }
    8
}
/// $07C0F8: move.w #$6, $70(a6)
pub fn i_07c0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0FE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_07c0fe  ($07C0FE, 8 B, kernel/task_handlers.c)
/// $07C0FE: lea.l $7c106(pc), a1
pub fn i_07c0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C102;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C106;
    16
}
/// $07C102: move.l a1, (a6)
pub fn i_07c102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C104;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C104: rts
pub fn i_07c104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C106;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_Recoil_07c106  ($07C106, 168 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C106: lea.l $2df86c.l, a0
pub fn i_07c106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C10C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF86C;
    20
}
/// $07C10C: move.w $70(a6), d0
pub fn i_07c10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C110;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C110: add.w d0, d0
pub fn i_07c110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C112;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C112: add.w d0, d0
pub fn i_07c112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C114;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C114: movea.l (a0, d0.w), a0
pub fn i_07c114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C118;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07C118: jsr $28cd4.l
pub fn i_07c118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C11E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C11E);
    cpu.pc = 0x28CD4;
    16
}
/// $07C11E: lea.l $2bcfd6.l, a0
pub fn i_07c11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C124;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCFD6;
    20
}
/// $07C124: jsr $799de.l
pub fn i_07c124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C12A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C12A);
    cpu.pc = 0x799DE;
    16
}
/// $07C12A: move.w d0, $82(a6)
pub fn i_07c12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C12E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C12E: bclr.b #$3, $13(a6)
pub fn i_07c12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C134;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C134: lea.l $7c13a(pc), a1
pub fn i_07c134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C138;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C13A;
    16
}
/// $07C138: move.l a1, (a6)
pub fn i_07c138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C13A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C13A: movea.l $c(a6), a0
pub fn i_07c13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C13E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C13E: move.w $22(a0), $22(a6)
pub fn i_07c13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C144;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C144: move.w $24(a0), $24(a6)
pub fn i_07c144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C14A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C14A: jsr $28d70.l
pub fn i_07c14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C150);
    cpu.pc = 0x28D70;
    16
}
/// $07C150: bcc.w $7c15a
pub fn i_07c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C154;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C15A; return 10; }
    8
}
/// $07C154: lea.l $7bfce(pc), a1
pub fn i_07c154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C158;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BFCE;
    16
}
/// $07C158: move.l a1, (a6)
pub fn i_07c158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C15A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C15A: btst.b #$7, $5a(a6)
pub fn i_07c15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C160;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07C160: beq.w $7c172
pub fn i_07c160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C164;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C172; return 10; }
    8
}
/// $07C164: move.l a6, -(a7)
pub fn i_07c164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C166;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07C166: movea.l $c(a6), a6
pub fn i_07c166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C16A;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $07C16A: bset.b #$7, $5a(a6)
pub fn i_07c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C170;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C170: movea.l (a7)+, a6
pub fn i_07c170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C172;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $07C172: bclr.b #$3, $13(a6)
pub fn i_07c172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C178;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C178: movea.l $c(a6), a0
pub fn i_07c178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C17C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C17C: movea.l $c(a0), a0
pub fn i_07c17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C180;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C180: tst.b $20(a0)
pub fn i_07c180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C184;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07C184: beq.w $7c18e
pub fn i_07c184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C188;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C18E; return 10; }
    8
}
/// $07C188: move.w $7c(a6), $66(a6)
pub fn i_07c188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C18E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C18E: jsr $2870a.l
pub fn i_07c18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C194);
    cpu.pc = 0x2870A;
    16
}
/// $07C194: lea.l $5e766.l, a0
pub fn i_07c194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C19A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07C19A: jsr $5e770.l
pub fn i_07c19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C1A0);
    cpu.pc = 0x5E770;
    16
}
/// $07C1A0: movea.l $c(a6), a0
pub fn i_07c1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1A4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C1A4: cmpi.b #$ff, $21(a0)
pub fn i_07c1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1AA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07C1AA: bne.w $7c1b4
pub fn i_07c1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C1B4; return 10; }
    8
}
// ===== SetTaskHandler_07c1ae  ($07C1AE, 8 B, kernel/task_handlers.c)
/// $07C1AE: lea.l $7c65c(pc), a1
pub fn i_07c1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C1B2: move.l a1, (a6)
pub fn i_07c1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C1B4: rts
pub fn i_07c1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Smoke_Init_07c1b6  ($07C1B6, 206 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C1B6: moveq #$0, d0
pub fn i_07c1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1B8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07C1B8: move.w d0, $70(a6)
pub fn i_07c1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1BC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C1BC: move.w #$5, $72(a6)
pub fn i_07c1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C1C2: move.w d0, $74(a6)
pub fn i_07c1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C1C6: lea.l $7c1cc(pc), a1
pub fn i_07c1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C1CC;
    16
}
/// $07C1CA: move.l a1, (a6)
pub fn i_07c1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C1CC: movea.l $c(a6), a0
pub fn i_07c1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1D0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C1D0: move.w $22(a0), $22(a6)
pub fn i_07c1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1D6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C1D6: move.w $24(a0), $24(a6)
pub fn i_07c1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1DC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C1DC: move.w #$0, d0
pub fn i_07c1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C1E0: jsr $28134.l
pub fn i_07c1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C1E6);
    cpu.pc = 0x28134;
    16
}
/// $07C1E6: andi.w #$ffe3, $38(a6)
pub fn i_07c1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1EC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C1EC: ori.w #$c, $38(a6)
pub fn i_07c1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1F2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C1F2: movea.l $c(a0), a0
pub fn i_07c1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1F6;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C1F6: cmpi.b #$ff, $20(a0)
pub fn i_07c1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C1FC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07C1FC: bne.w $7c206
pub fn i_07c1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C200;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C206; return 10; }
    8
}
/// $07C200: lea.l $7c65c(pc), a1
pub fn i_07c200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C204;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C204: move.l a1, (a6)
pub fn i_07c204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C206;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C206: movea.l $c(a6), a0
pub fn i_07c206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C20A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C20A: move.w $66(a0), d0
pub fn i_07c20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C20E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C20E: mulu.w #$a, d0
pub fn i_07c20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C212;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0xA;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $07C212: move.w $7c(a0), d1
pub fn i_07c212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C216;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C216: divu.w d1, d0
pub fn i_07c216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C218;
    cpu.ir = 0x80C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $07C218: add.w d0, d0
pub fn i_07c218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C21A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C21A: lea.l $2df802.l, a0
pub fn i_07c21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C220;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF802;
    20
}
/// $07C220: tst.w (a0, d0.w)
pub fn i_07c220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C224;
    cpu.ir = 0x4A70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $07C224: beq.w $7c282
pub fn i_07c224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C228;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C282; return 10; }
    8
}
/// $07C228: move.w (a0, d0.w), $74(a6)
pub fn i_07c228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C22E;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    22
}
/// $07C22E: addq.w #$1, $70(a6)
pub fn i_07c22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C232;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C232: move.w $70(a6), d0
pub fn i_07c232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C236;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C236: cmp.w $74(a6), d0
pub fn i_07c236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C23A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07C23A: blt.w $7c258
pub fn i_07c23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C23E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C258; return 10; }
    8
}
/// $07C23E: clr.w $70(a6)
pub fn i_07c23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C242;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C242: lea.l $7c284(pc), a1
pub fn i_07c242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C246;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C284;
    16
}
/// $07C246: jsr $4ae.l
pub fn i_07c246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C24C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C24C);
    cpu.pc = 0x4AE;
    16
}
/// $07C24C: jsr $5dd02.l
pub fn i_07c24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C252);
    cpu.pc = 0x5DD02;
    16
}
/// $07C252: move.w $38(a6), $38(a0)
pub fn i_07c252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C258;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C258: addq.w #$1, $72(a6)
pub fn i_07c258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C25C;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C25C: move.w $72(a6), d0
pub fn i_07c25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C260;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C260: cmp.w $74(a6), d0
pub fn i_07c260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C264;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07C264: blt.w $7c282
pub fn i_07c264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C268;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C282; return 10; }
    8
}
/// $07C268: clr.w $72(a6)
pub fn i_07c268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C26C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C26C: lea.l $7c2ea(pc), a1
pub fn i_07c26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C270;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C2EA;
    16
}
/// $07C270: jsr $4ae.l
pub fn i_07c270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C276;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C276);
    cpu.pc = 0x4AE;
    16
}
/// $07C276: jsr $5dd02.l
pub fn i_07c276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C27C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C27C);
    cpu.pc = 0x5DD02;
    16
}
/// $07C27C: move.w $38(a6), $38(a0)
pub fn i_07c27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C282;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C282: rts
pub fn i_07c282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C284;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_SmokePuff_Left_07c284  ($07C284, 102 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C284: move.w #$d, d1
pub fn i_07c284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C288;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C288: jsr $236e.l
pub fn i_07c288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C28E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C28E);
    cpu.pc = 0x236E;
    16
}
/// $07C28E: subi.w #$20, $22(a6)
pub fn i_07c28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C294;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C294: addi.w #$10, $24(a6)
pub fn i_07c294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C29A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C29A: move.w #$fd9a, d0
pub fn i_07c29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C29E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFD9A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C29E: jsr $5dca4.l
pub fn i_07c29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C2A4);
    cpu.pc = 0x5DCA4;
    16
}
/// $07C2A4: move.w d0, $28(a6)
pub fn i_07c2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2A8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C2A8: move.w #$3fc, $2a(a6)
pub fn i_07c2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C2AE: move.w #$ff34, $2e(a6)
pub fn i_07c2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2B4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF34;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C2B4: move.w #$0, $2c(a6)
pub fn i_07c2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C2BA: neg.w $2a(a6)
pub fn i_07c2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2BE;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C2BE: neg.w $2e(a6)
pub fn i_07c2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2C2;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C2C2: jsr $5e9b6.l
pub fn i_07c2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C2C8);
    cpu.pc = 0x5E9B6;
    16
}
/// $07C2C8: andi.w #$3f, d0
pub fn i_07c2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2CC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C2CC: subi.w #$1f, d0
pub fn i_07c2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2D0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C2D0: add.w d0, $2e(a6)
pub fn i_07c2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2D4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C2D4: lea.l $2dfe06.l, a0
pub fn i_07c2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DFE06;
    20
}
/// $07C2DA: jsr $28cd4.l
pub fn i_07c2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C2E0);
    cpu.pc = 0x28CD4;
    16
}
/// $07C2E0: lea.l $7c346(pc), a1
pub fn i_07c2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C346;
    16
}
/// $07C2E4: move.l a1, (a6)
pub fn i_07c2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C2E6: bra.w $7c346
pub fn i_07c2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x7C346;
    10
}
// ===== M2Boss_SmokePuff_Right_07c2ea  ($07C2EA, 92 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C2EA: move.w #$d, d1
pub fn i_07c2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2EE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C2EE: jsr $236e.l
pub fn i_07c2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C2F4);
    cpu.pc = 0x236E;
    16
}
/// $07C2F4: addi.w #$20, $22(a6)
pub fn i_07c2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2FA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C2FA: move.w #$333, d0
pub fn i_07c2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C2FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x333;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C2FE: jsr $5dca4.l
pub fn i_07c2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C304);
    cpu.pc = 0x5DCA4;
    16
}
/// $07C304: move.w d0, $28(a6)
pub fn i_07c304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C308;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C308: move.w #$3fc, $2a(a6)
pub fn i_07c308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C30E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C30E: move.w #$ff34, $2e(a6)
pub fn i_07c30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C314;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF34;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C314: move.w #$0, $2c(a6)
pub fn i_07c314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C31A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C31A: neg.w $2a(a6)
pub fn i_07c31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C31E;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C31E: neg.w $2e(a6)
pub fn i_07c31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C322;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C322: jsr $5e9b6.l
pub fn i_07c322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C328);
    cpu.pc = 0x5E9B6;
    16
}
/// $07C328: andi.w #$3f, d0
pub fn i_07c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C32C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C32C: subi.w #$1f, d0
pub fn i_07c32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C330;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C330: add.w d0, $2e(a6)
pub fn i_07c330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C334;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C334: lea.l $2dfe06.l, a0
pub fn i_07c334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C33A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DFE06;
    20
}
/// $07C33A: jsr $28cd4.l
pub fn i_07c33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C340);
    cpu.pc = 0x28CD4;
    16
}
/// $07C340: lea.l $7c346(pc), a1
pub fn i_07c340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C344;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C346;
    16
}
/// $07C344: move.l a1, (a6)
pub fn i_07c344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C346;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== M2Boss_SmokePuff_Run_07c346  ($07C346, 38 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C346: jsr $27bc8.l
pub fn i_07c346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C34C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C34C);
    cpu.pc = 0x27BC8;
    16
}
/// $07C34C: jsr $28d70.l
pub fn i_07c34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C352);
    cpu.pc = 0x28D70;
    16
}
/// $07C352: bcc.w $7c35c
pub fn i_07c352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C356;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C35C; return 10; }
    8
}
/// $07C356: lea.l $7c65c(pc), a1
pub fn i_07c356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C35A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C35A: move.l a1, (a6)
pub fn i_07c35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C35C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C35C: movea.l #$ffffffff, a0
pub fn i_07c35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C362;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07C362: jsr $5dd56.l
pub fn i_07c362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C368);
    cpu.pc = 0x5DD56;
    16
}
/// $07C368: bcc.w $7c372
pub fn i_07c368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C36C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C372; return 10; }
    8
}
// ===== SetTaskHandler_07c36c  ($07C36C, 8 B, kernel/task_handlers.c)
/// $07C36C: lea.l $7c65c(pc), a1
pub fn i_07c36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C370;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C370: move.l a1, (a6)
pub fn i_07c370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C372;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C372: rts
pub fn i_07c372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C374;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_DeathSwing_07c374  ($07C374, 168 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C374: jsr $4a16e.l
pub fn i_07c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C37A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C37A);
    cpu.pc = 0x4A16E;
    16
}
/// $07C37A: subi.w #$10, $22(a0)
pub fn i_07c37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C380;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C380: addi.w #$10, $24(a0)
pub fn i_07c380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C386;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C386: jsr $4a16e.l
pub fn i_07c386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C38C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C38C);
    cpu.pc = 0x4A16E;
    16
}
/// $07C38C: addi.w #$10, $22(a0)
pub fn i_07c38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C392;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C392: addi.w #$10, $24(a0)
pub fn i_07c392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C398;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C398: move.w #$0, $80(a6)
pub fn i_07c398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C39E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C39E: move.w #$102d, d0
pub fn i_07c39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3A2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C3A2: jsr $2352.l
pub fn i_07c3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C3A8);
    cpu.pc = 0x2352;
    16
}
/// $07C3A8: move.w #$2, $7c(a6)
pub fn i_07c3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C3AE: lea.l $7c3b4(pc), a1
pub fn i_07c3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C3B4;
    16
}
/// $07C3B2: move.l a1, (a6)
pub fn i_07c3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C3B4: addi.w #$40, $74(a6)
pub fn i_07c3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3BA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C3BA: cmpi.w #$180, $74(a6)
pub fn i_07c3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3C0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $07C3C0: blt.w $7c3ca
pub fn i_07c3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3C4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C3CA; return 10; }
    8
}
/// $07C3C4: move.w #$180, $74(a6)
pub fn i_07c3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C3CA: move.w $74(a6), d0
pub fn i_07c3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C3CE: add.w d0, $72(a6)
pub fn i_07c3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3D2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C3D2: cmpi.w #$af00, $72(a6)
pub fn i_07c3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3D8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xAF00, Size::Word, false);
    16
}
/// $07C3D8: bls.w $7c3e2
pub fn i_07c3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3DC;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x7C3E2; return 10; }
    8
}
/// $07C3DC: move.w #$af00, $72(a6)
pub fn i_07c3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3E2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xAF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C3E2: jsr $7bef4(pc)
pub fn i_07c3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7C3E6);
    cpu.pc = 0x7BEF4;
    16
}
/// $07C3E6: addq.w #$1, $80(a6)
pub fn i_07c3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3EA;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C3EA: cmpi.w #$19, $80(a6)
pub fn i_07c3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3F0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x19, Size::Word, false);
    16
}
/// $07C3F0: bne.w $7c3fe
pub fn i_07c3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C3FE; return 10; }
    8
}
/// $07C3F4: move.w #$1032, d0
pub fn i_07c3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1032;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C3F8: jsr $2352.l
pub fn i_07c3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C3FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C3FE);
    cpu.pc = 0x2352;
    16
}
/// $07C3FE: cmpi.w #$5a, $80(a6)
pub fn i_07c3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C404;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5A, Size::Word, false);
    16
}
/// $07C404: bne.w $7c412
pub fn i_07c404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C408;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C412; return 10; }
    8
}
/// $07C408: move.w #$1032, d0
pub fn i_07c408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C40C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1032;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C40C: jsr $2352.l
pub fn i_07c40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C412;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C412);
    cpu.pc = 0x2352;
    16
}
/// $07C412: cmpi.w #$af00, $72(a6)
pub fn i_07c412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C418;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xAF00, Size::Word, false);
    16
}
/// $07C418: bcs.w $7c422
pub fn i_07c418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C41C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7C422; return 10; }
    8
}
// ===== SetTaskHandler_07c41c  ($07C41C, 8 B, kernel/task_handlers.c)
/// $07C41C: lea.l $7c424(pc), a1
pub fn i_07c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C420;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C424;
    16
}
/// $07C420: move.l a1, (a6)
pub fn i_07c420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C422;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C422: rts
pub fn i_07c422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C424;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_DeathFall_07c424  ($07C424, 78 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C424: move.w #$266, d0
pub fn i_07c424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C428;
    cpu.ir = 0x303C;
    let v1: u32 = 0x266;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C428: jsr $5dca4.l
pub fn i_07c428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C42E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C42E);
    cpu.pc = 0x5DCA4;
    16
}
/// $07C42E: move.w d0, $28(a6)
pub fn i_07c42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C432;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C432: move.w #$fa, $2a(a6)
pub fn i_07c432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C438;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C438: move.w #$ffe7, $2e(a6)
pub fn i_07c438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C43E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C43E: move.w #$0, $2c(a6)
pub fn i_07c43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C444;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C444: move.w #$23, d0
pub fn i_07c444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C448;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C448: jsr $2352.l
pub fn i_07c448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C44E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C44E);
    cpu.pc = 0x2352;
    16
}
/// $07C44E: move.w #$1033, d0
pub fn i_07c44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C452;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C452: jsr $2352.l
pub fn i_07c452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C458);
    cpu.pc = 0x2352;
    16
}
/// $07C458: move.w #$2, $7c(a6)
pub fn i_07c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C45E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C45E: lea.l $7c464(pc), a1
pub fn i_07c45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C462;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C464;
    16
}
/// $07C462: move.l a1, (a6)
pub fn i_07c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C464;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C464: jsr $27cee.l
pub fn i_07c464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C46A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C46A);
    cpu.pc = 0x27CEE;
    16
}
/// $07C46A: jsr $7c628(pc)
pub fn i_07c46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C46E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7C46E);
    cpu.pc = 0x7C628;
    16
}
/// $07C46E: bcc.w $7c478
pub fn i_07c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C472;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C478; return 10; }
    8
}
// ===== SetTaskHandler_07c472  ($07C472, 8 B, kernel/task_handlers.c)
/// $07C472: lea.l $7c644(pc), a1
pub fn i_07c472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C476;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C644;
    16
}
/// $07C476: move.l a1, (a6)
pub fn i_07c476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C478;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C478: rts
pub fn i_07c478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C47A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_Destroyed_07c47a  ($07C47A, 144 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C47A: lea.l $2e0124.l, a1
pub fn i_07c47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C480;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E0124;
    20
}
/// $07C480: jsr $77c7e.l
pub fn i_07c480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C486);
    cpu.pc = 0x77C7E;
    16
}
/// $07C486: lea.l $2e0136.l, a1
pub fn i_07c486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C48C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E0136;
    20
}
/// $07C48C: jsr $77c7e.l
pub fn i_07c48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C492;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C492);
    cpu.pc = 0x77C7E;
    16
}
/// $07C492: move.w #$d000, $38(a0)
pub fn i_07c492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C498;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C498: movea.l $c(a6), a0
pub fn i_07c498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C49C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C49C: move.b #$ff, $20(a0)
pub fn i_07c49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4A2;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07C4A2: bclr.b #$1, $12(a6)
pub fn i_07c4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C4A8: move.b #$1, $10a2d1.l
pub fn i_07c4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4B0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $07C4B0: lea.l $7c4b6(pc), a1
pub fn i_07c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C4B6;
    16
}
/// $07C4B4: move.l a1, (a6)
pub fn i_07c4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C4B6: movea.l $c(a6), a0
pub fn i_07c4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4BA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C4BA: move.w $22(a0), $22(a6)
pub fn i_07c4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4C0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C4C0: move.w $24(a0), $24(a6)
pub fn i_07c4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4C6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C4C6: lea.l $2df850.l, a0
pub fn i_07c4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF850;
    20
}
/// $07C4CC: move.w $70(a6), d0
pub fn i_07c4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C4D0: add.w d0, d0
pub fn i_07c4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4D2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C4D2: add.w d0, d0
pub fn i_07c4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4D4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C4D4: movea.l (a0, d0.w), a0
pub fn i_07c4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4D8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07C4D8: jsr $28cd4.l
pub fn i_07c4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C4DE);
    cpu.pc = 0x28CD4;
    16
}
/// $07C4DE: jsr $28d70.l
pub fn i_07c4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C4E4);
    cpu.pc = 0x28D70;
    16
}
/// $07C4E4: btst.b #$7, $5a(a6)
pub fn i_07c4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07C4EA: beq.w $7c4fc
pub fn i_07c4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C4FC; return 10; }
    8
}
/// $07C4EE: move.l a6, -(a7)
pub fn i_07c4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4F0;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07C4F0: movea.l $c(a6), a6
pub fn i_07c4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4F4;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $07C4F4: bset.b #$7, $5a(a6)
pub fn i_07c4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4FA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C4FA: movea.l (a7)+, a6
pub fn i_07c4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4FC;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $07C4FC: movea.l $c(a6), a0
pub fn i_07c4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C500;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C500: cmpi.b #$ff, $21(a0)
pub fn i_07c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C506;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07C506: bne.w $7c510
pub fn i_07c506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C50A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C510; return 10; }
    8
}
// ===== SetTaskHandler_07c50a  ($07C50A, 8 B, kernel/task_handlers.c)
/// $07C50A: lea.l $7c65c(pc), a1
pub fn i_07c50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C50E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C50E: move.l a1, (a6)
pub fn i_07c50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C510;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C510: rts
pub fn i_07c510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C512;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_AimAtPlayer_07c512  ($07C512, 256 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C512: jsr $5e1aa.l
pub fn i_07c512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C518;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C518);
    cpu.pc = 0x5E1AA;
    16
}
/// $07C518: lea.l $100440.l, a0
pub fn i_07c518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C51E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $07C51E: cmpi.w #$0, d0
pub fn i_07c51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C522;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $07C522: beq.w $7c56a
pub fn i_07c522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C526;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C56A; return 10; }
    8
}
/// $07C526: cmpi.w #$1, d0
pub fn i_07c526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C52A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $07C52A: beq.w $7c56a
pub fn i_07c52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C52E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C56A; return 10; }
    8
}
/// $07C52E: lea.l $1004e0.l, a0
pub fn i_07c52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C534;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $07C534: cmpi.w #$2, d0
pub fn i_07c534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C538;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x07A6FC, i_07a6fc),
    (0x07A6FE, i_07a6fe),
    (0x07A702, i_07a702),
    (0x07A706, i_07a706),
    (0x07A708, i_07a708),
    (0x07A70E, i_07a70e),
    (0x07A710, i_07a710),
    (0x07A714, i_07a714),
    (0x07A718, i_07a718),
    (0x07A71A, i_07a71a),
    (0x07A71E, i_07a71e),
    (0x07A722, i_07a722),
    (0x07A724, i_07a724),
    (0x07A728, i_07a728),
    (0x07A72C, i_07a72c),
    (0x07A72E, i_07a72e),
    (0x07A734, i_07a734),
    (0x07A736, i_07a736),
    (0x07A73A, i_07a73a),
    (0x07A73E, i_07a73e),
    (0x07A740, i_07a740),
    (0x07A744, i_07a744),
    (0x07A748, i_07a748),
    (0x07A74A, i_07a74a),
    (0x07A750, i_07a750),
    (0x07A754, i_07a754),
    (0x07A75A, i_07a75a),
    (0x07A75E, i_07a75e),
    (0x07A762, i_07a762),
    (0x07A764, i_07a764),
    (0x07A768, i_07a768),
    (0x07A76C, i_07a76c),
    (0x07A76E, i_07a76e),
    (0x07A774, i_07a774),
    (0x07A778, i_07a778),
    (0x07A77E, i_07a77e),
    (0x07A782, i_07a782),
    (0x07A786, i_07a786),
    (0x07A788, i_07a788),
    (0x07A78C, i_07a78c),
    (0x07A790, i_07a790),
    (0x07A792, i_07a792),
    (0x07A798, i_07a798),
    (0x07A79C, i_07a79c),
    (0x07A7A2, i_07a7a2),
    (0x07A7A6, i_07a7a6),
    (0x07A7AA, i_07a7aa),
    (0x07A7AC, i_07a7ac),
    (0x07A7B0, i_07a7b0),
    (0x07A7B4, i_07a7b4),
    (0x07A7B6, i_07a7b6),
    (0x07A7BC, i_07a7bc),
    (0x07A7C0, i_07a7c0),
    (0x07A7C6, i_07a7c6),
    (0x07A7CA, i_07a7ca),
    (0x07A7CE, i_07a7ce),
    (0x07A7D0, i_07a7d0),
    (0x07A7D2, i_07a7d2),
    (0x07A7D6, i_07a7d6),
    (0x07A7DC, i_07a7dc),
    (0x07A7E0, i_07a7e0),
    (0x07A7E6, i_07a7e6),
    (0x07A954, i_07a954),
    (0x07A958, i_07a958),
    (0x07A95C, i_07a95c),
    (0x07A960, i_07a960),
    (0x07A964, i_07a964),
    (0x07A968, i_07a968),
    (0x07A96A, i_07a96a),
    (0x07A96E, i_07a96e),
    (0x07A970, i_07a970),
    (0x07A976, i_07a976),
    (0x07A97C, i_07a97c),
    (0x07A982, i_07a982),
    (0x07A988, i_07a988),
    (0x07A98E, i_07a98e),
    (0x07A994, i_07a994),
    (0x07A99A, i_07a99a),
    (0x07A99E, i_07a99e),
    (0x07A9A2, i_07a9a2),
    (0x07A9A6, i_07a9a6),
    (0x07A9AA, i_07a9aa),
    (0x07A9AE, i_07a9ae),
    (0x07A9B4, i_07a9b4),
    (0x07A9BA, i_07a9ba),
    (0x07A9BE, i_07a9be),
    (0x07A9C0, i_07a9c0),
    (0x07A9C6, i_07a9c6),
    (0x07A9CA, i_07a9ca),
    (0x07A9CE, i_07a9ce),
    (0x07A9D2, i_07a9d2),
    (0x07A9D6, i_07a9d6),
    (0x07A9DA, i_07a9da),
    (0x07A9DE, i_07a9de),
    (0x07A9E2, i_07a9e2),
    (0x07A9E8, i_07a9e8),
    (0x07A9EC, i_07a9ec),
    (0x07A9EE, i_07a9ee),
    (0x07A9F0, i_07a9f0),
    (0x07A9F4, i_07a9f4),
    (0x07A9FA, i_07a9fa),
    (0x07AA00, i_07aa00),
    (0x07AA06, i_07aa06),
    (0x07AA0C, i_07aa0c),
    (0x07AA12, i_07aa12),
    (0x07AA18, i_07aa18),
    (0x07AA1E, i_07aa1e),
    (0x07AA22, i_07aa22),
    (0x07AA26, i_07aa26),
    (0x07AA2A, i_07aa2a),
    (0x07AA2E, i_07aa2e),
    (0x07AA32, i_07aa32),
    (0x07AA38, i_07aa38),
    (0x07AA3E, i_07aa3e),
    (0x07AA42, i_07aa42),
    (0x07AA44, i_07aa44),
    (0x07AA4A, i_07aa4a),
    (0x07AA4E, i_07aa4e),
    (0x07AA52, i_07aa52),
    (0x07AA56, i_07aa56),
    (0x07AA5A, i_07aa5a),
    (0x07AA5E, i_07aa5e),
    (0x07AA62, i_07aa62),
    (0x07AA66, i_07aa66),
    (0x07AA6A, i_07aa6a),
    (0x07AA70, i_07aa70),
    (0x07AA74, i_07aa74),
    (0x07AA76, i_07aa76),
    (0x07AA78, i_07aa78),
    (0x07AA7C, i_07aa7c),
    (0x07AA7E, i_07aa7e),
    (0x07AA82, i_07aa82),
    (0x07AA88, i_07aa88),
    (0x07AA8C, i_07aa8c),
    (0x07AA90, i_07aa90),
    (0x07AA92, i_07aa92),
    (0x07AA94, i_07aa94),
    (0x07AA9A, i_07aa9a),
    (0x07AAA0, i_07aaa0),
    (0x07AAA4, i_07aaa4),
    (0x07AAA8, i_07aaa8),
    (0x07AAAC, i_07aaac),
    (0x07AAB4, i_07aab4),
    (0x07AAB8, i_07aab8),
    (0x07AABE, i_07aabe),
    (0x07AAC4, i_07aac4),
    (0x07AAC8, i_07aac8),
    (0x07AACA, i_07aaca),
    (0x07AACE, i_07aace),
    (0x07AAD2, i_07aad2),
    (0x07AAD8, i_07aad8),
    (0x07AADE, i_07aade),
    (0x07AAE0, i_07aae0),
    (0x07AAE6, i_07aae6),
    (0x07AAEA, i_07aaea),
    (0x07AAEC, i_07aaec),
    (0x07AAF0, i_07aaf0),
    (0x07AAF4, i_07aaf4),
    (0x07AAF6, i_07aaf6),
    (0x07AAFA, i_07aafa),
    (0x07AB00, i_07ab00),
    (0x07AB06, i_07ab06),
    (0x07AB0C, i_07ab0c),
    (0x07AB12, i_07ab12),
    (0x07AB18, i_07ab18),
    (0x07AB1E, i_07ab1e),
    (0x07AB24, i_07ab24),
    (0x07AB26, i_07ab26),
    (0x07AB2A, i_07ab2a),
    (0x07AB2E, i_07ab2e),
    (0x07AB32, i_07ab32),
    (0x07AB36, i_07ab36),
    (0x07AB3C, i_07ab3c),
    (0x07AB3E, i_07ab3e),
    (0x07AB42, i_07ab42),
    (0x07AB48, i_07ab48),
    (0x07AB4C, i_07ab4c),
    (0x07AB52, i_07ab52),
    (0x07AB56, i_07ab56),
    (0x07AB58, i_07ab58),
    (0x07AB5C, i_07ab5c),
    (0x07AB62, i_07ab62),
    (0x07AB66, i_07ab66),
    (0x07AB68, i_07ab68),
    (0x07AB6C, i_07ab6c),
    (0x07AB70, i_07ab70),
    (0x07AB72, i_07ab72),
    (0x07AB76, i_07ab76),
    (0x07AB7C, i_07ab7c),
    (0x07AB82, i_07ab82),
    (0x07AB88, i_07ab88),
    (0x07AB8E, i_07ab8e),
    (0x07AB94, i_07ab94),
    (0x07AB9A, i_07ab9a),
    (0x07AB9C, i_07ab9c),
    (0x07ABA2, i_07aba2),
    (0x07ABA6, i_07aba6),
    (0x07ABAA, i_07abaa),
    (0x07ABAE, i_07abae),
    (0x07ABB4, i_07abb4),
    (0x07ABB6, i_07abb6),
    (0x07ABBA, i_07abba),
    (0x07ABC0, i_07abc0),
    (0x07ABC4, i_07abc4),
    (0x07ABCA, i_07abca),
    (0x07ABCE, i_07abce),
    (0x07ABD0, i_07abd0),
    (0x07ABD4, i_07abd4),
    (0x07ABDA, i_07abda),
    (0x07ABDE, i_07abde),
    (0x07ABE2, i_07abe2),
    (0x07ABE6, i_07abe6),
    (0x07ABEC, i_07abec),
    (0x07ABF0, i_07abf0),
    (0x07ABF2, i_07abf2),
    (0x07ABF4, i_07abf4),
    (0x07ABFA, i_07abfa),
    (0x07ABFE, i_07abfe),
    (0x07AC02, i_07ac02),
    (0x07AC08, i_07ac08),
    (0x07AC0E, i_07ac0e),
    (0x07AC12, i_07ac12),
    (0x07AC14, i_07ac14),
    (0x07AC1A, i_07ac1a),
    (0x07AC1E, i_07ac1e),
    (0x07AC22, i_07ac22),
    (0x07AC26, i_07ac26),
    (0x07AC2A, i_07ac2a),
    (0x07AC2E, i_07ac2e),
    (0x07AC34, i_07ac34),
    (0x07AC38, i_07ac38),
    (0x07AC3E, i_07ac3e),
    (0x07AC42, i_07ac42),
    (0x07AC46, i_07ac46),
    (0x07AC48, i_07ac48),
    (0x07AC4A, i_07ac4a),
    (0x07AC50, i_07ac50),
    (0x07AC54, i_07ac54),
    (0x07AC56, i_07ac56),
    (0x07AC5C, i_07ac5c),
    (0x07AC5E, i_07ac5e),
    (0x07AC62, i_07ac62),
    (0x07AC66, i_07ac66),
    (0x07AC6A, i_07ac6a),
    (0x07AC70, i_07ac70),
    (0x07AC74, i_07ac74),
    (0x07AC78, i_07ac78),
    (0x07AC7C, i_07ac7c),
    (0x07AC80, i_07ac80),
    (0x07AC86, i_07ac86),
    (0x07AC8A, i_07ac8a),
    (0x07AC8E, i_07ac8e),
    (0x07AC92, i_07ac92),
    (0x07AC98, i_07ac98),
    (0x07AC9C, i_07ac9c),
    (0x07ACA0, i_07aca0),
    (0x07ACA6, i_07aca6),
    (0x07ACAC, i_07acac),
    (0x07ACB2, i_07acb2),
    (0x07ACB8, i_07acb8),
    (0x07ACBE, i_07acbe),
    (0x07ACC4, i_07acc4),
    (0x07ACC8, i_07acc8),
    (0x07ACCE, i_07acce),
    (0x07ACD0, i_07acd0),
    (0x07ACD4, i_07acd4),
    (0x07ACDA, i_07acda),
    (0x07ACE0, i_07ace0),
    (0x07ACE6, i_07ace6),
    (0x07ACEC, i_07acec),
    (0x07ACF2, i_07acf2),
    (0x07ACF8, i_07acf8),
    (0x07ACFE, i_07acfe),
    (0x07AD02, i_07ad02),
    (0x07AD08, i_07ad08),
    (0x07AD0E, i_07ad0e),
    (0x07AD14, i_07ad14),
    (0x07AD1A, i_07ad1a),
    (0x07AD20, i_07ad20),
    (0x07AD26, i_07ad26),
    (0x07AD2C, i_07ad2c),
    (0x07AD30, i_07ad30),
    (0x07AD36, i_07ad36),
    (0x07AD3C, i_07ad3c),
    (0x07AD42, i_07ad42),
    (0x07AD48, i_07ad48),
    (0x07AD4E, i_07ad4e),
    (0x07AD54, i_07ad54),
    (0x07AD5A, i_07ad5a),
    (0x07AD5E, i_07ad5e),
    (0x07AD64, i_07ad64),
    (0x07AD6A, i_07ad6a),
    (0x07AD70, i_07ad70),
    (0x07AD76, i_07ad76),
    (0x07AD7C, i_07ad7c),
    (0x07AD82, i_07ad82),
    (0x07AD88, i_07ad88),
    (0x07AD8C, i_07ad8c),
    (0x07AD92, i_07ad92),
    (0x07AD98, i_07ad98),
    (0x07AD9E, i_07ad9e),
    (0x07ADA4, i_07ada4),
    (0x07ADAA, i_07adaa),
    (0x07ADB0, i_07adb0),
    (0x07ADB6, i_07adb6),
    (0x07ADBA, i_07adba),
    (0x07ADC0, i_07adc0),
    (0x07ADC6, i_07adc6),
    (0x07ADCC, i_07adcc),
    (0x07ADD2, i_07add2),
    (0x07ADD8, i_07add8),
    (0x07ADDE, i_07adde),
    (0x07ADE4, i_07ade4),
    (0x07ADE8, i_07ade8),
    (0x07ADEE, i_07adee),
    (0x07ADF4, i_07adf4),
    (0x07ADFA, i_07adfa),
    (0x07AE00, i_07ae00),
    (0x07AE06, i_07ae06),
    (0x07AE0C, i_07ae0c),
    (0x07AE10, i_07ae10),
    (0x07AE16, i_07ae16),
    (0x07AE1C, i_07ae1c),
    (0x07AE22, i_07ae22),
    (0x07AE28, i_07ae28),
    (0x07AE2E, i_07ae2e),
    (0x07AE34, i_07ae34),
    (0x07AE3A, i_07ae3a),
    (0x07AE3E, i_07ae3e),
    (0x07AE44, i_07ae44),
    (0x07AE4A, i_07ae4a),
    (0x07AE50, i_07ae50),
    (0x07AE56, i_07ae56),
    (0x07AE5C, i_07ae5c),
    (0x07AE62, i_07ae62),
    (0x07AE68, i_07ae68),
    (0x07AE6C, i_07ae6c),
    (0x07AE72, i_07ae72),
    (0x07AE78, i_07ae78),
    (0x07AE7E, i_07ae7e),
    (0x07AE84, i_07ae84),
    (0x07AE8A, i_07ae8a),
    (0x07AE90, i_07ae90),
    (0x07AE96, i_07ae96),
    (0x07AE9A, i_07ae9a),
    (0x07AEA0, i_07aea0),
    (0x07AEA6, i_07aea6),
    (0x07AEAC, i_07aeac),
    (0x07AEB2, i_07aeb2),
    (0x07AEB8, i_07aeb8),
    (0x07AEBE, i_07aebe),
    (0x07AEC4, i_07aec4),
    (0x07AEC8, i_07aec8),
    (0x07AECE, i_07aece),
    (0x07AED4, i_07aed4),
    (0x07AEDA, i_07aeda),
    (0x07AEE0, i_07aee0),
    (0x07AEE6, i_07aee6),
    (0x07AEEC, i_07aeec),
    (0x07AEF2, i_07aef2),
    (0x07AEF6, i_07aef6),
    (0x07AEFC, i_07aefc),
    (0x07AF02, i_07af02),
    (0x07AF08, i_07af08),
    (0x07AF0E, i_07af0e),
    (0x07AF14, i_07af14),
    (0x07AF1A, i_07af1a),
    (0x07AF20, i_07af20),
    (0x07AF24, i_07af24),
    (0x07AF2A, i_07af2a),
    (0x07AF30, i_07af30),
    (0x07AF36, i_07af36),
    (0x07AF3C, i_07af3c),
    (0x07AF42, i_07af42),
    (0x07AF48, i_07af48),
    (0x07AF4E, i_07af4e),
    (0x07AF50, i_07af50),
    (0x07AF54, i_07af54),
    (0x07AF5A, i_07af5a),
    (0x07AF60, i_07af60),
    (0x07AF66, i_07af66),
    (0x07AF6C, i_07af6c),
    (0x07AF72, i_07af72),
    (0x07AF78, i_07af78),
    (0x07AF7E, i_07af7e),
    (0x07AF82, i_07af82),
    (0x07AF88, i_07af88),
    (0x07AF8E, i_07af8e),
    (0x07AF94, i_07af94),
    (0x07AF9A, i_07af9a),
    (0x07AFA0, i_07afa0),
    (0x07AFA6, i_07afa6),
    (0x07AFAC, i_07afac),
    (0x07AFB0, i_07afb0),
    (0x07AFB6, i_07afb6),
    (0x07AFBC, i_07afbc),
    (0x07AFC2, i_07afc2),
    (0x07AFC8, i_07afc8),
    (0x07AFCE, i_07afce),
    (0x07AFD4, i_07afd4),
    (0x07AFDA, i_07afda),
    (0x07AFDE, i_07afde),
    (0x07AFE4, i_07afe4),
    (0x07AFEA, i_07afea),
    (0x07AFF0, i_07aff0),
    (0x07AFF6, i_07aff6),
    (0x07AFFC, i_07affc),
    (0x07B002, i_07b002),
    (0x07B008, i_07b008),
    (0x07B00C, i_07b00c),
    (0x07B012, i_07b012),
    (0x07B018, i_07b018),
    (0x07B01E, i_07b01e),
    (0x07B024, i_07b024),
    (0x07B02A, i_07b02a),
    (0x07B030, i_07b030),
    (0x07B036, i_07b036),
    (0x07B03A, i_07b03a),
    (0x07B040, i_07b040),
    (0x07B046, i_07b046),
    (0x07B04C, i_07b04c),
    (0x07B052, i_07b052),
    (0x07B058, i_07b058),
    (0x07B05E, i_07b05e),
    (0x07B064, i_07b064),
    (0x07B068, i_07b068),
    (0x07B06E, i_07b06e),
    (0x07B074, i_07b074),
    (0x07B07A, i_07b07a),
    (0x07B080, i_07b080),
    (0x07B086, i_07b086),
    (0x07B08C, i_07b08c),
    (0x07B092, i_07b092),
    (0x07B096, i_07b096),
    (0x07B09C, i_07b09c),
    (0x07B0A2, i_07b0a2),
    (0x07B0A8, i_07b0a8),
    (0x07B0AE, i_07b0ae),
    (0x07B0B4, i_07b0b4),
    (0x07B0BA, i_07b0ba),
    (0x07B0C0, i_07b0c0),
    (0x07B0C4, i_07b0c4),
    (0x07B0CA, i_07b0ca),
    (0x07B0D0, i_07b0d0),
    (0x07B0D6, i_07b0d6),
    (0x07B0DC, i_07b0dc),
    (0x07B0E2, i_07b0e2),
    (0x07B0E8, i_07b0e8),
    (0x07B0EE, i_07b0ee),
    (0x07B0F2, i_07b0f2),
    (0x07B0F8, i_07b0f8),
    (0x07B0FE, i_07b0fe),
    (0x07B104, i_07b104),
    (0x07B10A, i_07b10a),
    (0x07B110, i_07b110),
    (0x07B116, i_07b116),
    (0x07B11C, i_07b11c),
    (0x07B120, i_07b120),
    (0x07B126, i_07b126),
    (0x07B12C, i_07b12c),
    (0x07B132, i_07b132),
    (0x07B138, i_07b138),
    (0x07B13E, i_07b13e),
    (0x07B144, i_07b144),
    (0x07B14A, i_07b14a),
    (0x07B14E, i_07b14e),
    (0x07B154, i_07b154),
    (0x07B15A, i_07b15a),
    (0x07B160, i_07b160),
    (0x07B166, i_07b166),
    (0x07B16C, i_07b16c),
    (0x07B172, i_07b172),
    (0x07B178, i_07b178),
    (0x07B17C, i_07b17c),
    (0x07B182, i_07b182),
    (0x07B188, i_07b188),
    (0x07B18E, i_07b18e),
    (0x07B194, i_07b194),
    (0x07B19A, i_07b19a),
    (0x07B1A0, i_07b1a0),
    (0x07B1A6, i_07b1a6),
    (0x07B1AA, i_07b1aa),
    (0x07B1B0, i_07b1b0),
    (0x07B1B6, i_07b1b6),
    (0x07B1BC, i_07b1bc),
    (0x07B1C2, i_07b1c2),
    (0x07B1C8, i_07b1c8),
    (0x07B1CE, i_07b1ce),
    (0x07B1D4, i_07b1d4),
    (0x07B1D8, i_07b1d8),
    (0x07B1DE, i_07b1de),
    (0x07B1E4, i_07b1e4),
    (0x07B1EA, i_07b1ea),
    (0x07B1F0, i_07b1f0),
    (0x07B1F6, i_07b1f6),
    (0x07B1FC, i_07b1fc),
    (0x07B202, i_07b202),
    (0x07B206, i_07b206),
    (0x07B20C, i_07b20c),
    (0x07B212, i_07b212),
    (0x07B218, i_07b218),
    (0x07B21E, i_07b21e),
    (0x07B224, i_07b224),
    (0x07B22A, i_07b22a),
    (0x07B230, i_07b230),
    (0x07B234, i_07b234),
    (0x07B23A, i_07b23a),
    (0x07B240, i_07b240),
    (0x07B246, i_07b246),
    (0x07B24C, i_07b24c),
    (0x07B252, i_07b252),
    (0x07B258, i_07b258),
    (0x07B25E, i_07b25e),
    (0x07B262, i_07b262),
    (0x07B268, i_07b268),
    (0x07B26E, i_07b26e),
    (0x07B274, i_07b274),
    (0x07B27A, i_07b27a),
    (0x07B280, i_07b280),
    (0x07B286, i_07b286),
    (0x07B28C, i_07b28c),
    (0x07B28E, i_07b28e),
    (0x07B292, i_07b292),
    (0x07B298, i_07b298),
    (0x07B29E, i_07b29e),
    (0x07B2A4, i_07b2a4),
    (0x07B2AA, i_07b2aa),
    (0x07B2B0, i_07b2b0),
    (0x07B2B6, i_07b2b6),
    (0x07B2BC, i_07b2bc),
    (0x07B2C0, i_07b2c0),
    (0x07B2C6, i_07b2c6),
    (0x07B2CC, i_07b2cc),
    (0x07B2D2, i_07b2d2),
    (0x07B2D8, i_07b2d8),
    (0x07B2DE, i_07b2de),
    (0x07B2E4, i_07b2e4),
    (0x07B2EA, i_07b2ea),
    (0x07B2EE, i_07b2ee),
    (0x07B2F4, i_07b2f4),
    (0x07B2FA, i_07b2fa),
    (0x07B300, i_07b300),
    (0x07B306, i_07b306),
    (0x07B30C, i_07b30c),
    (0x07B312, i_07b312),
    (0x07B318, i_07b318),
    (0x07B31C, i_07b31c),
    (0x07B322, i_07b322),
    (0x07B328, i_07b328),
    (0x07B32E, i_07b32e),
    (0x07B334, i_07b334),
    (0x07B33A, i_07b33a),
    (0x07B340, i_07b340),
    (0x07B346, i_07b346),
    (0x07B34A, i_07b34a),
    (0x07B350, i_07b350),
    (0x07B356, i_07b356),
    (0x07B35C, i_07b35c),
    (0x07B362, i_07b362),
    (0x07B368, i_07b368),
    (0x07B36E, i_07b36e),
    (0x07B374, i_07b374),
    (0x07B378, i_07b378),
    (0x07B37E, i_07b37e),
    (0x07B384, i_07b384),
    (0x07B38A, i_07b38a),
    (0x07B390, i_07b390),
    (0x07B396, i_07b396),
    (0x07B39C, i_07b39c),
    (0x07B3A2, i_07b3a2),
    (0x07B3A6, i_07b3a6),
    (0x07B3AC, i_07b3ac),
    (0x07B3B2, i_07b3b2),
    (0x07B3B8, i_07b3b8),
    (0x07B3BE, i_07b3be),
    (0x07B3C4, i_07b3c4),
    (0x07B3CA, i_07b3ca),
    (0x07B3D0, i_07b3d0),
    (0x07B3D4, i_07b3d4),
    (0x07B3DA, i_07b3da),
    (0x07B3E0, i_07b3e0),
    (0x07B3E6, i_07b3e6),
    (0x07B3EC, i_07b3ec),
    (0x07B3F2, i_07b3f2),
    (0x07B3F8, i_07b3f8),
    (0x07B3FE, i_07b3fe),
    (0x07B402, i_07b402),
    (0x07B408, i_07b408),
    (0x07B40E, i_07b40e),
    (0x07B414, i_07b414),
    (0x07B41A, i_07b41a),
    (0x07B420, i_07b420),
    (0x07B426, i_07b426),
    (0x07B42C, i_07b42c),
    (0x07B430, i_07b430),
    (0x07B436, i_07b436),
    (0x07B43C, i_07b43c),
    (0x07B442, i_07b442),
    (0x07B448, i_07b448),
    (0x07B44E, i_07b44e),
    (0x07B454, i_07b454),
    (0x07B458, i_07b458),
    (0x07B45E, i_07b45e),
    (0x07B464, i_07b464),
    (0x07B46A, i_07b46a),
    (0x07B470, i_07b470),
    (0x07B476, i_07b476),
    (0x07B47C, i_07b47c),
    (0x07B482, i_07b482),
    (0x07B486, i_07b486),
    (0x07B48C, i_07b48c),
    (0x07B492, i_07b492),
    (0x07B498, i_07b498),
    (0x07B49E, i_07b49e),
    (0x07B4A4, i_07b4a4),
    (0x07B4AA, i_07b4aa),
    (0x07B4B0, i_07b4b0),
    (0x07B4B4, i_07b4b4),
    (0x07B4BA, i_07b4ba),
    (0x07B4C0, i_07b4c0),
    (0x07B4C6, i_07b4c6),
    (0x07B4CC, i_07b4cc),
    (0x07B4D2, i_07b4d2),
    (0x07B4D8, i_07b4d8),
    (0x07B4DE, i_07b4de),
    (0x07B4E2, i_07b4e2),
    (0x07B4E8, i_07b4e8),
    (0x07B4EE, i_07b4ee),
    (0x07B4F4, i_07b4f4),
    (0x07B4FA, i_07b4fa),
    (0x07B500, i_07b500),
    (0x07B506, i_07b506),
    (0x07B50C, i_07b50c),
    (0x07B510, i_07b510),
    (0x07B516, i_07b516),
    (0x07B51C, i_07b51c),
    (0x07B522, i_07b522),
    (0x07B528, i_07b528),
    (0x07B52E, i_07b52e),
    (0x07B534, i_07b534),
    (0x07B53A, i_07b53a),
    (0x07B53E, i_07b53e),
    (0x07B544, i_07b544),
    (0x07B54A, i_07b54a),
    (0x07B550, i_07b550),
    (0x07B556, i_07b556),
    (0x07B55C, i_07b55c),
    (0x07B562, i_07b562),
    (0x07B568, i_07b568),
    (0x07B56C, i_07b56c),
    (0x07B572, i_07b572),
    (0x07B578, i_07b578),
    (0x07B57E, i_07b57e),
    (0x07B584, i_07b584),
    (0x07B58A, i_07b58a),
    (0x07B590, i_07b590),
    (0x07B596, i_07b596),
    (0x07B598, i_07b598),
    (0x07B59C, i_07b59c),
    (0x07B5A2, i_07b5a2),
    (0x07B5A8, i_07b5a8),
    (0x07B5AE, i_07b5ae),
    (0x07B5B4, i_07b5b4),
    (0x07B5BA, i_07b5ba),
    (0x07B5C0, i_07b5c0),
    (0x07B5C6, i_07b5c6),
    (0x07B5CA, i_07b5ca),
    (0x07B5D0, i_07b5d0),
    (0x07B5D6, i_07b5d6),
    (0x07B5DC, i_07b5dc),
    (0x07B5E2, i_07b5e2),
    (0x07B5E8, i_07b5e8),
    (0x07B5EE, i_07b5ee),
    (0x07B5F4, i_07b5f4),
    (0x07B5F8, i_07b5f8),
    (0x07B5FE, i_07b5fe),
    (0x07B604, i_07b604),
    (0x07B60A, i_07b60a),
    (0x07B610, i_07b610),
    (0x07B616, i_07b616),
    (0x07B61C, i_07b61c),
    (0x07B622, i_07b622),
    (0x07B626, i_07b626),
    (0x07B62C, i_07b62c),
    (0x07B632, i_07b632),
    (0x07B638, i_07b638),
    (0x07B63E, i_07b63e),
    (0x07B644, i_07b644),
    (0x07B64A, i_07b64a),
    (0x07B650, i_07b650),
    (0x07B654, i_07b654),
    (0x07B65A, i_07b65a),
    (0x07B660, i_07b660),
    (0x07B666, i_07b666),
    (0x07B66C, i_07b66c),
    (0x07B672, i_07b672),
    (0x07B678, i_07b678),
    (0x07B67E, i_07b67e),
    (0x07B682, i_07b682),
    (0x07B688, i_07b688),
    (0x07B68E, i_07b68e),
    (0x07B694, i_07b694),
    (0x07B69A, i_07b69a),
    (0x07B6A0, i_07b6a0),
    (0x07B6A6, i_07b6a6),
    (0x07B6AC, i_07b6ac),
    (0x07B6B0, i_07b6b0),
    (0x07B6B6, i_07b6b6),
    (0x07B6BC, i_07b6bc),
    (0x07B6C2, i_07b6c2),
    (0x07B6C8, i_07b6c8),
    (0x07B6CE, i_07b6ce),
    (0x07B6D4, i_07b6d4),
    (0x07B6DA, i_07b6da),
    (0x07B6DE, i_07b6de),
    (0x07B6E4, i_07b6e4),
    (0x07B6EA, i_07b6ea),
    (0x07B6F0, i_07b6f0),
    (0x07B6F6, i_07b6f6),
    (0x07B6FC, i_07b6fc),
    (0x07B702, i_07b702),
    (0x07B708, i_07b708),
    (0x07B70C, i_07b70c),
    (0x07B712, i_07b712),
    (0x07B718, i_07b718),
    (0x07B71E, i_07b71e),
    (0x07B724, i_07b724),
    (0x07B72A, i_07b72a),
    (0x07B730, i_07b730),
    (0x07B736, i_07b736),
    (0x07B73A, i_07b73a),
    (0x07B740, i_07b740),
    (0x07B746, i_07b746),
    (0x07B74C, i_07b74c),
    (0x07B752, i_07b752),
    (0x07B758, i_07b758),
    (0x07B75E, i_07b75e),
    (0x07B764, i_07b764),
    (0x07B768, i_07b768),
    (0x07B76E, i_07b76e),
    (0x07B774, i_07b774),
    (0x07B77A, i_07b77a),
    (0x07B780, i_07b780),
    (0x07B786, i_07b786),
    (0x07B78C, i_07b78c),
    (0x07B792, i_07b792),
    (0x07B796, i_07b796),
    (0x07B79C, i_07b79c),
    (0x07B7A2, i_07b7a2),
    (0x07B7A8, i_07b7a8),
    (0x07B7AE, i_07b7ae),
    (0x07B7B4, i_07b7b4),
    (0x07B7BA, i_07b7ba),
    (0x07B7C0, i_07b7c0),
    (0x07B7C4, i_07b7c4),
    (0x07B7CA, i_07b7ca),
    (0x07B7D0, i_07b7d0),
    (0x07B7D6, i_07b7d6),
    (0x07B7DC, i_07b7dc),
    (0x07B7E2, i_07b7e2),
    (0x07B7E8, i_07b7e8),
    (0x07B7EE, i_07b7ee),
    (0x07B7F2, i_07b7f2),
    (0x07B7F8, i_07b7f8),
    (0x07B7FE, i_07b7fe),
    (0x07B804, i_07b804),
    (0x07B80A, i_07b80a),
    (0x07B810, i_07b810),
    (0x07B816, i_07b816),
    (0x07B81C, i_07b81c),
    (0x07B820, i_07b820),
    (0x07B826, i_07b826),
    (0x07B82C, i_07b82c),
    (0x07B832, i_07b832),
    (0x07B838, i_07b838),
    (0x07B83E, i_07b83e),
    (0x07B844, i_07b844),
    (0x07B84A, i_07b84a),
    (0x07B84E, i_07b84e),
    (0x07B854, i_07b854),
    (0x07B85A, i_07b85a),
    (0x07B860, i_07b860),
    (0x07B866, i_07b866),
    (0x07B86C, i_07b86c),
    (0x07B872, i_07b872),
    (0x07B878, i_07b878),
    (0x07B87C, i_07b87c),
    (0x07B882, i_07b882),
    (0x07B888, i_07b888),
    (0x07B88E, i_07b88e),
    (0x07B894, i_07b894),
    (0x07B89A, i_07b89a),
    (0x07B8A0, i_07b8a0),
    (0x07B8A6, i_07b8a6),
    (0x07B8AA, i_07b8aa),
    (0x07B8B0, i_07b8b0),
    (0x07B8B6, i_07b8b6),
    (0x07B8BC, i_07b8bc),
    (0x07B8C2, i_07b8c2),
    (0x07B8C8, i_07b8c8),
    (0x07B8CE, i_07b8ce),
    (0x07B8D4, i_07b8d4),
    (0x07B8D8, i_07b8d8),
    (0x07B8DE, i_07b8de),
    (0x07B8E4, i_07b8e4),
    (0x07B8EA, i_07b8ea),
    (0x07B8F0, i_07b8f0),
    (0x07B8F6, i_07b8f6),
    (0x07B8FC, i_07b8fc),
    (0x07B902, i_07b902),
    (0x07B906, i_07b906),
    (0x07B90C, i_07b90c),
    (0x07B912, i_07b912),
    (0x07B918, i_07b918),
    (0x07B91E, i_07b91e),
    (0x07B924, i_07b924),
    (0x07B92A, i_07b92a),
    (0x07B930, i_07b930),
    (0x07B934, i_07b934),
    (0x07B93A, i_07b93a),
    (0x07B940, i_07b940),
    (0x07B946, i_07b946),
    (0x07B94C, i_07b94c),
    (0x07B952, i_07b952),
    (0x07B958, i_07b958),
    (0x07B95E, i_07b95e),
    (0x07B960, i_07b960),
    (0x07B964, i_07b964),
    (0x07B968, i_07b968),
    (0x07B96C, i_07b96c),
    (0x07B970, i_07b970),
    (0x07B974, i_07b974),
    (0x07B978, i_07b978),
    (0x07B97C, i_07b97c),
    (0x07B980, i_07b980),
    (0x07B982, i_07b982),
    (0x07B984, i_07b984),
    (0x07B988, i_07b988),
    (0x07B98C, i_07b98c),
    (0x07B992, i_07b992),
    (0x07B998, i_07b998),
    (0x07B99C, i_07b99c),
    (0x07B99E, i_07b99e),
    (0x07B9A2, i_07b9a2),
    (0x07B9A4, i_07b9a4),
    (0x07B9AA, i_07b9aa),
    (0x07B9AE, i_07b9ae),
    (0x07B9B2, i_07b9b2),
    (0x07B9B4, i_07b9b4),
    (0x07B9B8, i_07b9b8),
    (0x07B9BC, i_07b9bc),
    (0x07B9C0, i_07b9c0),
    (0x07B9C4, i_07b9c4),
    (0x07B9CA, i_07b9ca),
    (0x07B9CE, i_07b9ce),
    (0x07B9D2, i_07b9d2),
    (0x07B9D6, i_07b9d6),
    (0x07B9DA, i_07b9da),
    (0x07B9DE, i_07b9de),
    (0x07B9E4, i_07b9e4),
    (0x07B9E8, i_07b9e8),
    (0x07B9EA, i_07b9ea),
    (0x07B9EE, i_07b9ee),
    (0x07B9F2, i_07b9f2),
    (0x07B9F6, i_07b9f6),
    (0x07B9F8, i_07b9f8),
    (0x07B9FC, i_07b9fc),
    (0x07BA00, i_07ba00),
    (0x07BA04, i_07ba04),
    (0x07BA08, i_07ba08),
    (0x07BA0E, i_07ba0e),
    (0x07BA12, i_07ba12),
    (0x07BA16, i_07ba16),
    (0x07BA1C, i_07ba1c),
    (0x07BA22, i_07ba22),
    (0x07BA28, i_07ba28),
    (0x07BA2C, i_07ba2c),
    (0x07BA2E, i_07ba2e),
    (0x07BA32, i_07ba32),
    (0x07BA36, i_07ba36),
    (0x07BA3A, i_07ba3a),
    (0x07BA3E, i_07ba3e),
    (0x07BA44, i_07ba44),
    (0x07BA46, i_07ba46),
    (0x07BA48, i_07ba48),
    (0x07BA4C, i_07ba4c),
    (0x07BA4E, i_07ba4e),
    (0x07BA52, i_07ba52),
    (0x07BA54, i_07ba54),
    (0x07BA58, i_07ba58),
    (0x07BA5A, i_07ba5a),
    (0x07BA5E, i_07ba5e),
    (0x07BA60, i_07ba60),
    (0x07BA64, i_07ba64),
    (0x07BA68, i_07ba68),
    (0x07BA6C, i_07ba6c),
    (0x07BA70, i_07ba70),
    (0x07BA74, i_07ba74),
    (0x07BA76, i_07ba76),
    (0x07BA7A, i_07ba7a),
    (0x07BA7C, i_07ba7c),
    (0x07BA80, i_07ba80),
    (0x07BA86, i_07ba86),
    (0x07BA8A, i_07ba8a),
    (0x07BA90, i_07ba90),
    (0x07BA96, i_07ba96),
    (0x07BA98, i_07ba98),
    (0x07BA9C, i_07ba9c),
    (0x07BAA0, i_07baa0),
    (0x07BAA4, i_07baa4),
    (0x07BAAA, i_07baaa),
    (0x07BAB0, i_07bab0),
    (0x07BAB6, i_07bab6),
    (0x07BABC, i_07babc),
    (0x07BAC0, i_07bac0),
    (0x07BAC6, i_07bac6),
    (0x07BACA, i_07baca),
    (0x07BACC, i_07bacc),
    (0x07BACE, i_07bace),
    (0x07BAD2, i_07bad2),
    (0x07BAD8, i_07bad8),
    (0x07BADE, i_07bade),
    (0x07BAE4, i_07bae4),
    (0x07BAEA, i_07baea),
    (0x07BAEE, i_07baee),
    (0x07BAF0, i_07baf0),
    (0x07BAF6, i_07baf6),
    (0x07BAFC, i_07bafc),
    (0x07BB02, i_07bb02),
    (0x07BB08, i_07bb08),
    (0x07BB0C, i_07bb0c),
    (0x07BB12, i_07bb12),
    (0x07BB16, i_07bb16),
    (0x07BB18, i_07bb18),
    (0x07BB4C, i_07bb4c),
    (0x07BB52, i_07bb52),
    (0x07BB54, i_07bb54),
    (0x07BB5A, i_07bb5a),
    (0x07BB5E, i_07bb5e),
    (0x07BB62, i_07bb62),
    (0x07BB68, i_07bb68),
    (0x07BB6E, i_07bb6e),
    (0x07BB70, i_07bb70),
    (0x07BB76, i_07bb76),
    (0x07BB78, i_07bb78),
    (0x07BB7C, i_07bb7c),
    (0x07BB82, i_07bb82),
    (0x07BB88, i_07bb88),
    (0x07BB8E, i_07bb8e),
    (0x07BB92, i_07bb92),
    (0x07BB98, i_07bb98),
    (0x07BB9E, i_07bb9e),
    (0x07BBA4, i_07bba4),
    (0x07BBAA, i_07bbaa),
    (0x07BBB0, i_07bbb0),
    (0x07BBB4, i_07bbb4),
    (0x07BBBA, i_07bbba),
    (0x07BBBE, i_07bbbe),
    (0x07BBC0, i_07bbc0),
    (0x07BBC4, i_07bbc4),
    (0x07BBCA, i_07bbca),
    (0x07BBD0, i_07bbd0),
    (0x07BBD4, i_07bbd4),
    (0x07BBD8, i_07bbd8),
    (0x07BBDC, i_07bbdc),
    (0x07BBE0, i_07bbe0),
    (0x07BBE2, i_07bbe2),
    (0x07BBE8, i_07bbe8),
    (0x07BBEA, i_07bbea),
    (0x07BBEE, i_07bbee),
    (0x07BBF4, i_07bbf4),
    (0x07BBF8, i_07bbf8),
    (0x07BBFE, i_07bbfe),
    (0x07BC00, i_07bc00),
    (0x07BC04, i_07bc04),
    (0x07BC08, i_07bc08),
    (0x07BC0E, i_07bc0e),
    (0x07BC12, i_07bc12),
    (0x07BC18, i_07bc18),
    (0x07BC1E, i_07bc1e),
    (0x07BC22, i_07bc22),
    (0x07BC28, i_07bc28),
    (0x07BC2C, i_07bc2c),
    (0x07BC32, i_07bc32),
    (0x07BC36, i_07bc36),
    (0x07BC3A, i_07bc3a),
    (0x07BC3E, i_07bc3e),
    (0x07BC46, i_07bc46),
    (0x07BC4C, i_07bc4c),
    (0x07BC52, i_07bc52),
    (0x07BC58, i_07bc58),
    (0x07BC5E, i_07bc5e),
    (0x07BC64, i_07bc64),
    (0x07BC68, i_07bc68),
    (0x07BC6E, i_07bc6e),
    (0x07BC74, i_07bc74),
    (0x07BC78, i_07bc78),
    (0x07BC7C, i_07bc7c),
    (0x07BC82, i_07bc82),
    (0x07BC86, i_07bc86),
    (0x07BC8A, i_07bc8a),
    (0x07BC8C, i_07bc8c),
    (0x07BC8E, i_07bc8e),
    (0x07BC90, i_07bc90),
    (0x07BC94, i_07bc94),
    (0x07BC98, i_07bc98),
    (0x07BC9C, i_07bc9c),
    (0x07BCA0, i_07bca0),
    (0x07BCA4, i_07bca4),
    (0x07BCA8, i_07bca8),
    (0x07BCAC, i_07bcac),
    (0x07BCB0, i_07bcb0),
    (0x07BCB4, i_07bcb4),
    (0x07BCBA, i_07bcba),
    (0x07BCC0, i_07bcc0),
    (0x07BCC4, i_07bcc4),
    (0x07BCCA, i_07bcca),
    (0x07BCD0, i_07bcd0),
    (0x07BCD6, i_07bcd6),
    (0x07BCDA, i_07bcda),
    (0x07BCDC, i_07bcdc),
    (0x07BCE0, i_07bce0),
    (0x07BCE6, i_07bce6),
    (0x07BCEA, i_07bcea),
    (0x07BCEE, i_07bcee),
    (0x07BCF0, i_07bcf0),
    (0x07BCF6, i_07bcf6),
    (0x07BCFA, i_07bcfa),
    (0x07BCFE, i_07bcfe),
    (0x07BD00, i_07bd00),
    (0x07BD06, i_07bd06),
    (0x07BD0A, i_07bd0a),
    (0x07BD0E, i_07bd0e),
    (0x07BD10, i_07bd10),
    (0x07BD16, i_07bd16),
    (0x07BD1A, i_07bd1a),
    (0x07BD1E, i_07bd1e),
    (0x07BD20, i_07bd20),
    (0x07BD22, i_07bd22),
    (0x07BD28, i_07bd28),
    (0x07BD2C, i_07bd2c),
    (0x07BD2E, i_07bd2e),
    (0x07BD34, i_07bd34),
    (0x07BD38, i_07bd38),
    (0x07BD3C, i_07bd3c),
    (0x07BD40, i_07bd40),
    (0x07BD46, i_07bd46),
    (0x07BD4A, i_07bd4a),
    (0x07BD4E, i_07bd4e),
    (0x07BD52, i_07bd52),
    (0x07BD54, i_07bd54),
    (0x07BD58, i_07bd58),
    (0x07BD5C, i_07bd5c),
    (0x07BD62, i_07bd62),
    (0x07BD68, i_07bd68),
    (0x07BD6C, i_07bd6c),
    (0x07BD72, i_07bd72),
    (0x07BD76, i_07bd76),
    (0x07BD7A, i_07bd7a),
    (0x07BD7E, i_07bd7e),
    (0x07BD84, i_07bd84),
    (0x07BD88, i_07bd88),
    (0x07BD8E, i_07bd8e),
    (0x07BD92, i_07bd92),
    (0x07BD94, i_07bd94),
    (0x07BD98, i_07bd98),
    (0x07BD9C, i_07bd9c),
    (0x07BDA2, i_07bda2),
    (0x07BDA6, i_07bda6),
    (0x07BDA8, i_07bda8),
    (0x07BDAC, i_07bdac),
    (0x07BDB0, i_07bdb0),
    (0x07BDB4, i_07bdb4),
    (0x07BDBA, i_07bdba),
    (0x07BDC0, i_07bdc0),
    (0x07BDC4, i_07bdc4),
    (0x07BDC8, i_07bdc8),
    (0x07BDCA, i_07bdca),
    (0x07BDCC, i_07bdcc),
    (0x07BDD2, i_07bdd2),
    (0x07BDD6, i_07bdd6),
    (0x07BDD8, i_07bdd8),
    (0x07BDDE, i_07bdde),
    (0x07BDE2, i_07bde2),
    (0x07BDE6, i_07bde6),
    (0x07BDEA, i_07bdea),
    (0x07BDF0, i_07bdf0),
    (0x07BDF4, i_07bdf4),
    (0x07BDF8, i_07bdf8),
    (0x07BDFC, i_07bdfc),
    (0x07BDFE, i_07bdfe),
    (0x07BE02, i_07be02),
    (0x07BE06, i_07be06),
    (0x07BE0C, i_07be0c),
    (0x07BE12, i_07be12),
    (0x07BE16, i_07be16),
    (0x07BE1C, i_07be1c),
    (0x07BE20, i_07be20),
    (0x07BE24, i_07be24),
    (0x07BE2A, i_07be2a),
    (0x07BE2E, i_07be2e),
    (0x07BE34, i_07be34),
    (0x07BE38, i_07be38),
    (0x07BE3E, i_07be3e),
    (0x07BE42, i_07be42),
    (0x07BE44, i_07be44),
    (0x07BE48, i_07be48),
    (0x07BE4C, i_07be4c),
    (0x07BE52, i_07be52),
    (0x07BE56, i_07be56),
    (0x07BE58, i_07be58),
    (0x07BE5C, i_07be5c),
    (0x07BE60, i_07be60),
    (0x07BE64, i_07be64),
    (0x07BE6A, i_07be6a),
    (0x07BE70, i_07be70),
    (0x07BE74, i_07be74),
    (0x07BE78, i_07be78),
    (0x07BE7A, i_07be7a),
    (0x07BE7C, i_07be7c),
    (0x07BE82, i_07be82),
    (0x07BE86, i_07be86),
    (0x07BE88, i_07be88),
    (0x07BE8E, i_07be8e),
    (0x07BE92, i_07be92),
    (0x07BE98, i_07be98),
    (0x07BE9C, i_07be9c),
    (0x07BEA0, i_07bea0),
    (0x07BEA4, i_07bea4),
    (0x07BEA6, i_07bea6),
    (0x07BEAA, i_07beaa),
    (0x07BEB0, i_07beb0),
    (0x07BEB6, i_07beb6),
    (0x07BEBA, i_07beba),
    (0x07BEC0, i_07bec0),
    (0x07BEC4, i_07bec4),
    (0x07BEC8, i_07bec8),
    (0x07BECE, i_07bece),
    (0x07BED2, i_07bed2),
    (0x07BED8, i_07bed8),
    (0x07BEDC, i_07bedc),
    (0x07BEE2, i_07bee2),
    (0x07BEE8, i_07bee8),
    (0x07BEEC, i_07beec),
    (0x07BEF0, i_07bef0),
    (0x07BEF2, i_07bef2),
    (0x07BEF4, i_07bef4),
    (0x07BEF8, i_07bef8),
    (0x07BEFE, i_07befe),
    (0x07BF04, i_07bf04),
    (0x07BF0A, i_07bf0a),
    (0x07BF10, i_07bf10),
    (0x07BF14, i_07bf14),
    (0x07BF16, i_07bf16),
    (0x07BF1A, i_07bf1a),
    (0x07BF1C, i_07bf1c),
    (0x07BF20, i_07bf20),
    (0x07BF24, i_07bf24),
    (0x07BF28, i_07bf28),
    (0x07BF2C, i_07bf2c),
    (0x07BF30, i_07bf30),
    (0x07BF34, i_07bf34),
    (0x07BF36, i_07bf36),
    (0x07BF3A, i_07bf3a),
    (0x07BF40, i_07bf40),
    (0x07BF46, i_07bf46),
    (0x07BF4C, i_07bf4c),
    (0x07BF52, i_07bf52),
    (0x07BF56, i_07bf56),
    (0x07BF5C, i_07bf5c),
    (0x07BF62, i_07bf62),
    (0x07BF68, i_07bf68),
    (0x07BF6E, i_07bf6e),
    (0x07BF74, i_07bf74),
    (0x07BF78, i_07bf78),
    (0x07BF7C, i_07bf7c),
    (0x07BF82, i_07bf82),
    (0x07BF84, i_07bf84),
    (0x07BF88, i_07bf88),
    (0x07BF8C, i_07bf8c),
    (0x07BF92, i_07bf92),
    (0x07BF96, i_07bf96),
    (0x07BF9A, i_07bf9a),
    (0x07BF9E, i_07bf9e),
    (0x07BFA2, i_07bfa2),
    (0x07BFA6, i_07bfa6),
    (0x07BFAA, i_07bfaa),
    (0x07BFAE, i_07bfae),
    (0x07BFB2, i_07bfb2),
    (0x07BFB6, i_07bfb6),
    (0x07BFBC, i_07bfbc),
    (0x07BFC2, i_07bfc2),
    (0x07BFC8, i_07bfc8),
    (0x07BFCC, i_07bfcc),
    (0x07BFCE, i_07bfce),
    (0x07BFD2, i_07bfd2),
    (0x07BFD8, i_07bfd8),
    (0x07BFDE, i_07bfde),
    (0x07BFE4, i_07bfe4),
    (0x07BFE8, i_07bfe8),
    (0x07BFEA, i_07bfea),
    (0x07BFEC, i_07bfec),
    (0x07BFF0, i_07bff0),
    (0x07BFF6, i_07bff6),
    (0x07BFFC, i_07bffc),
    (0x07C002, i_07c002),
    (0x07C006, i_07c006),
    (0x07C008, i_07c008),
    (0x07C00C, i_07c00c),
    (0x07C012, i_07c012),
    (0x07C014, i_07c014),
    (0x07C018, i_07c018),
    (0x07C01C, i_07c01c),
    (0x07C020, i_07c020),
    (0x07C024, i_07c024),
    (0x07C02A, i_07c02a),
    (0x07C02E, i_07c02e),
    (0x07C032, i_07c032),
    (0x07C036, i_07c036),
    (0x07C038, i_07c038),
    (0x07C03E, i_07c03e),
    (0x07C044, i_07c044),
    (0x07C04A, i_07c04a),
    (0x07C050, i_07c050),
    (0x07C054, i_07c054),
    (0x07C058, i_07c058),
    (0x07C05A, i_07c05a),
    (0x07C05E, i_07c05e),
    (0x07C064, i_07c064),
    (0x07C068, i_07c068),
    (0x07C06C, i_07c06c),
    (0x07C06E, i_07c06e),
    (0x07C070, i_07c070),
    (0x07C076, i_07c076),
    (0x07C078, i_07c078),
    (0x07C07E, i_07c07e),
    (0x07C084, i_07c084),
    (0x07C086, i_07c086),
    (0x07C08C, i_07c08c),
    (0x07C092, i_07c092),
    (0x07C098, i_07c098),
    (0x07C09A, i_07c09a),
    (0x07C0A0, i_07c0a0),
    (0x07C0A4, i_07c0a4),
    (0x07C0A8, i_07c0a8),
    (0x07C0AE, i_07c0ae),
    (0x07C0B0, i_07c0b0),
    (0x07C0B6, i_07c0b6),
    (0x07C0B8, i_07c0b8),
    (0x07C0BC, i_07c0bc),
    (0x07C0C0, i_07c0c0),
    (0x07C0C4, i_07c0c4),
    (0x07C0C8, i_07c0c8),
    (0x07C0CE, i_07c0ce),
    (0x07C0D2, i_07c0d2),
    (0x07C0D8, i_07c0d8),
    (0x07C0DE, i_07c0de),
    (0x07C0E4, i_07c0e4),
    (0x07C0E6, i_07c0e6),
    (0x07C0EA, i_07c0ea),
    (0x07C0EE, i_07c0ee),
    (0x07C0F4, i_07c0f4),
    (0x07C0F8, i_07c0f8),
    (0x07C0FE, i_07c0fe),
    (0x07C102, i_07c102),
    (0x07C104, i_07c104),
    (0x07C106, i_07c106),
    (0x07C10C, i_07c10c),
    (0x07C110, i_07c110),
    (0x07C112, i_07c112),
    (0x07C114, i_07c114),
    (0x07C118, i_07c118),
    (0x07C11E, i_07c11e),
    (0x07C124, i_07c124),
    (0x07C12A, i_07c12a),
    (0x07C12E, i_07c12e),
    (0x07C134, i_07c134),
    (0x07C138, i_07c138),
    (0x07C13A, i_07c13a),
    (0x07C13E, i_07c13e),
    (0x07C144, i_07c144),
    (0x07C14A, i_07c14a),
    (0x07C150, i_07c150),
    (0x07C154, i_07c154),
    (0x07C158, i_07c158),
    (0x07C15A, i_07c15a),
    (0x07C160, i_07c160),
    (0x07C164, i_07c164),
    (0x07C166, i_07c166),
    (0x07C16A, i_07c16a),
    (0x07C170, i_07c170),
    (0x07C172, i_07c172),
    (0x07C178, i_07c178),
    (0x07C17C, i_07c17c),
    (0x07C180, i_07c180),
    (0x07C184, i_07c184),
    (0x07C188, i_07c188),
    (0x07C18E, i_07c18e),
    (0x07C194, i_07c194),
    (0x07C19A, i_07c19a),
    (0x07C1A0, i_07c1a0),
    (0x07C1A4, i_07c1a4),
    (0x07C1AA, i_07c1aa),
    (0x07C1AE, i_07c1ae),
    (0x07C1B2, i_07c1b2),
    (0x07C1B4, i_07c1b4),
    (0x07C1B6, i_07c1b6),
    (0x07C1B8, i_07c1b8),
    (0x07C1BC, i_07c1bc),
    (0x07C1C2, i_07c1c2),
    (0x07C1C6, i_07c1c6),
    (0x07C1CA, i_07c1ca),
    (0x07C1CC, i_07c1cc),
    (0x07C1D0, i_07c1d0),
    (0x07C1D6, i_07c1d6),
    (0x07C1DC, i_07c1dc),
    (0x07C1E0, i_07c1e0),
    (0x07C1E6, i_07c1e6),
    (0x07C1EC, i_07c1ec),
    (0x07C1F2, i_07c1f2),
    (0x07C1F6, i_07c1f6),
    (0x07C1FC, i_07c1fc),
    (0x07C200, i_07c200),
    (0x07C204, i_07c204),
    (0x07C206, i_07c206),
    (0x07C20A, i_07c20a),
    (0x07C20E, i_07c20e),
    (0x07C212, i_07c212),
    (0x07C216, i_07c216),
    (0x07C218, i_07c218),
    (0x07C21A, i_07c21a),
    (0x07C220, i_07c220),
    (0x07C224, i_07c224),
    (0x07C228, i_07c228),
    (0x07C22E, i_07c22e),
    (0x07C232, i_07c232),
    (0x07C236, i_07c236),
    (0x07C23A, i_07c23a),
    (0x07C23E, i_07c23e),
    (0x07C242, i_07c242),
    (0x07C246, i_07c246),
    (0x07C24C, i_07c24c),
    (0x07C252, i_07c252),
    (0x07C258, i_07c258),
    (0x07C25C, i_07c25c),
    (0x07C260, i_07c260),
    (0x07C264, i_07c264),
    (0x07C268, i_07c268),
    (0x07C26C, i_07c26c),
    (0x07C270, i_07c270),
    (0x07C276, i_07c276),
    (0x07C27C, i_07c27c),
    (0x07C282, i_07c282),
    (0x07C284, i_07c284),
    (0x07C288, i_07c288),
    (0x07C28E, i_07c28e),
    (0x07C294, i_07c294),
    (0x07C29A, i_07c29a),
    (0x07C29E, i_07c29e),
    (0x07C2A4, i_07c2a4),
    (0x07C2A8, i_07c2a8),
    (0x07C2AE, i_07c2ae),
    (0x07C2B4, i_07c2b4),
    (0x07C2BA, i_07c2ba),
    (0x07C2BE, i_07c2be),
    (0x07C2C2, i_07c2c2),
    (0x07C2C8, i_07c2c8),
    (0x07C2CC, i_07c2cc),
    (0x07C2D0, i_07c2d0),
    (0x07C2D4, i_07c2d4),
    (0x07C2DA, i_07c2da),
    (0x07C2E0, i_07c2e0),
    (0x07C2E4, i_07c2e4),
    (0x07C2E6, i_07c2e6),
    (0x07C2EA, i_07c2ea),
    (0x07C2EE, i_07c2ee),
    (0x07C2F4, i_07c2f4),
    (0x07C2FA, i_07c2fa),
    (0x07C2FE, i_07c2fe),
    (0x07C304, i_07c304),
    (0x07C308, i_07c308),
    (0x07C30E, i_07c30e),
    (0x07C314, i_07c314),
    (0x07C31A, i_07c31a),
    (0x07C31E, i_07c31e),
    (0x07C322, i_07c322),
    (0x07C328, i_07c328),
    (0x07C32C, i_07c32c),
    (0x07C330, i_07c330),
    (0x07C334, i_07c334),
    (0x07C33A, i_07c33a),
    (0x07C340, i_07c340),
    (0x07C344, i_07c344),
    (0x07C346, i_07c346),
    (0x07C34C, i_07c34c),
    (0x07C352, i_07c352),
    (0x07C356, i_07c356),
    (0x07C35A, i_07c35a),
    (0x07C35C, i_07c35c),
    (0x07C362, i_07c362),
    (0x07C368, i_07c368),
    (0x07C36C, i_07c36c),
    (0x07C370, i_07c370),
    (0x07C372, i_07c372),
    (0x07C374, i_07c374),
    (0x07C37A, i_07c37a),
    (0x07C380, i_07c380),
    (0x07C386, i_07c386),
    (0x07C38C, i_07c38c),
    (0x07C392, i_07c392),
    (0x07C398, i_07c398),
    (0x07C39E, i_07c39e),
    (0x07C3A2, i_07c3a2),
    (0x07C3A8, i_07c3a8),
    (0x07C3AE, i_07c3ae),
    (0x07C3B2, i_07c3b2),
    (0x07C3B4, i_07c3b4),
    (0x07C3BA, i_07c3ba),
    (0x07C3C0, i_07c3c0),
    (0x07C3C4, i_07c3c4),
    (0x07C3CA, i_07c3ca),
    (0x07C3CE, i_07c3ce),
    (0x07C3D2, i_07c3d2),
    (0x07C3D8, i_07c3d8),
    (0x07C3DC, i_07c3dc),
    (0x07C3E2, i_07c3e2),
    (0x07C3E6, i_07c3e6),
    (0x07C3EA, i_07c3ea),
    (0x07C3F0, i_07c3f0),
    (0x07C3F4, i_07c3f4),
    (0x07C3F8, i_07c3f8),
    (0x07C3FE, i_07c3fe),
    (0x07C404, i_07c404),
    (0x07C408, i_07c408),
    (0x07C40C, i_07c40c),
    (0x07C412, i_07c412),
    (0x07C418, i_07c418),
    (0x07C41C, i_07c41c),
    (0x07C420, i_07c420),
    (0x07C422, i_07c422),
    (0x07C424, i_07c424),
    (0x07C428, i_07c428),
    (0x07C42E, i_07c42e),
    (0x07C432, i_07c432),
    (0x07C438, i_07c438),
    (0x07C43E, i_07c43e),
    (0x07C444, i_07c444),
    (0x07C448, i_07c448),
    (0x07C44E, i_07c44e),
    (0x07C452, i_07c452),
    (0x07C458, i_07c458),
    (0x07C45E, i_07c45e),
    (0x07C462, i_07c462),
    (0x07C464, i_07c464),
    (0x07C46A, i_07c46a),
    (0x07C46E, i_07c46e),
    (0x07C472, i_07c472),
    (0x07C476, i_07c476),
    (0x07C478, i_07c478),
    (0x07C47A, i_07c47a),
    (0x07C480, i_07c480),
    (0x07C486, i_07c486),
    (0x07C48C, i_07c48c),
    (0x07C492, i_07c492),
    (0x07C498, i_07c498),
    (0x07C49C, i_07c49c),
    (0x07C4A2, i_07c4a2),
    (0x07C4A8, i_07c4a8),
    (0x07C4B0, i_07c4b0),
    (0x07C4B4, i_07c4b4),
    (0x07C4B6, i_07c4b6),
    (0x07C4BA, i_07c4ba),
    (0x07C4C0, i_07c4c0),
    (0x07C4C6, i_07c4c6),
    (0x07C4CC, i_07c4cc),
    (0x07C4D0, i_07c4d0),
    (0x07C4D2, i_07c4d2),
    (0x07C4D4, i_07c4d4),
    (0x07C4D8, i_07c4d8),
    (0x07C4DE, i_07c4de),
    (0x07C4E4, i_07c4e4),
    (0x07C4EA, i_07c4ea),
    (0x07C4EE, i_07c4ee),
    (0x07C4F0, i_07c4f0),
    (0x07C4F4, i_07c4f4),
    (0x07C4FA, i_07c4fa),
    (0x07C4FC, i_07c4fc),
    (0x07C500, i_07c500),
    (0x07C506, i_07c506),
    (0x07C50A, i_07c50a),
    (0x07C50E, i_07c50e),
    (0x07C510, i_07c510),
    (0x07C512, i_07c512),
    (0x07C518, i_07c518),
    (0x07C51E, i_07c51e),
    (0x07C522, i_07c522),
    (0x07C526, i_07c526),
    (0x07C52A, i_07c52a),
    (0x07C52E, i_07c52e),
    (0x07C534, i_07c534),
];
