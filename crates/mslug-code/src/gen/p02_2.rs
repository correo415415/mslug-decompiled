// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $02875E: bne.w $28768
pub fn i_02875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28762;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28768; return 10; }
    8
}
// ===== ClearXN_028762  ($028762, 6 B, thunks/ccr_helpers.c)
/// $028762: andi.b #$ee, ccr
pub fn i_028762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28766;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028766: rts
pub fn i_028766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28768;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_028768  ($028768, 6 B, thunks/ccr_helpers.c)
/// $028768: ori.b #$11, ccr
pub fn i_028768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2876C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02876C: rts
pub fn i_02876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2876E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapRect_02876e  ($02876E, 104 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02876E: move.w (a0), d0
pub fn i_02876e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28770;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028770: move.w $2(a0), d1
pub fn i_028770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28774;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028774: btst.b #$0, $3a(a6)
pub fn i_028774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2877A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02877A: beq.w $28784
pub fn i_02877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2877E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28784; return 10; }
    8
}
/// $02877E: neg.w d0
pub fn i_02877e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28780;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028780: neg.w d1
pub fn i_028780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28782;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028782: exg.l d0, d1
pub fn i_028782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28784;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $028784: cmp.w d1, d0
pub fn i_028784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28786;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028786: blt.w $28794
pub fn i_028786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2878A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28794; return 10; }
    8
}
/// $02878A: nop
pub fn i_02878a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2878C;
    cpu.ir = 0x4E71;
    4
}
/// $02878C: nop
pub fn i_02878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2878E;
    cpu.ir = 0x4E71;
    4
}
/// $02878E: cmp.w d1, d0
pub fn i_02878e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28790;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028790: nop
pub fn i_028790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28792;
    cpu.ir = 0x4E71;
    4
}
/// $028792: trap #$f
pub fn i_028792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28794;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028794: move.w (a2), d2
pub fn i_028794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28796;
    cpu.ir = 0x3412;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028796: move.w $2(a2), d3
pub fn i_028796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2879A;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02879A: btst.b #$0, $3a(a1)
pub fn i_02879a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A0;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0287A0: beq.w $287aa
pub fn i_0287a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x287AA; return 10; }
    8
}
/// $0287A4: neg.w d2
pub fn i_0287a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A6;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0287A6: neg.w d3
pub fn i_0287a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A8;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0287A8: exg.l d2, d3
pub fn i_0287a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287AA;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $0287AA: cmp.w d3, d2
pub fn i_0287aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287AC;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287AC: blt.w $287ba
pub fn i_0287ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x287BA; return 10; }
    8
}
/// $0287B0: nop
pub fn i_0287b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B2;
    cpu.ir = 0x4E71;
    4
}
/// $0287B2: nop
pub fn i_0287b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B4;
    cpu.ir = 0x4E71;
    4
}
/// $0287B4: cmp.w d3, d2
pub fn i_0287b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B6;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287B6: nop
pub fn i_0287b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B8;
    cpu.ir = 0x4E71;
    4
}
/// $0287B8: trap #$f
pub fn i_0287b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287BA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0287BA: add.w $22(a6), d0
pub fn i_0287ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287BE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0287BE: add.w $22(a6), d1
pub fn i_0287be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287C2;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0287C2: add.w $22(a1), d2
pub fn i_0287c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287C6;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0287C6: add.w $22(a1), d3
pub fn i_0287c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287CA;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0287CA: cmp.w d1, d2
pub fn i_0287ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287CC;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287CC: bgt.w $287d6
pub fn i_0287cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287D0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x287D6; return 10; }
    8
}
/// $0287D0: cmp.w d3, d0
pub fn i_0287d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287D2;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287D2: ble.w $287dc
pub fn i_0287d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287D6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x287DC; return 10; }
    8
}
// ===== ClearXN_0287d6  ($0287D6, 6 B, thunks/ccr_helpers.c)
/// $0287D6: andi.b #$ee, ccr
pub fn i_0287d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287DA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0287DA: rts
pub fn i_0287da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapRect_Y_0287dc  ($0287DC, 130 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0287DC: cmp.w d2, d0
pub fn i_0287dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287DE;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287DE: bcc.w $287e4
pub fn i_0287de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x287E4; return 10; }
    8
}
/// $0287E2: move.w d2, d0
pub fn i_0287e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287E4;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0287E4: cmp.w d3, d1
pub fn i_0287e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287E6;
    cpu.ir = 0xB243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287E6: bcs.w $287ec
pub fn i_0287e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287EA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x287EC; return 10; }
    8
}
/// $0287EA: move.w d3, d1
pub fn i_0287ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287EC;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0287EC: add.w d0, d1
pub fn i_0287ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287EE;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0287EE: asr.w #$1, d1
pub fn i_0287ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287F0;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0287F0: move.w d1, d7
pub fn i_0287f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287F2;
    cpu.ir = 0x3E01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0287F2: move.w $4(a0), d0
pub fn i_0287f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287F6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0287F6: move.w $6(a0), d1
pub fn i_0287f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287FA;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0287FA: btst.b #$1, $3a(a6)
pub fn i_0287fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28800;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028800: beq.w $2880a
pub fn i_028800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28804;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2880A; return 10; }
    8
}
/// $028804: neg.w d0
pub fn i_028804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28806;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028806: neg.w d1
pub fn i_028806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28808;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028808: exg.l d0, d1
pub fn i_028808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2880A;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $02880A: cmp.w d1, d0
pub fn i_02880a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2880C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02880C: blt.w $2881a
pub fn i_02880c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28810;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2881A; return 10; }
    8
}
/// $028810: nop
pub fn i_028810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28812;
    cpu.ir = 0x4E71;
    4
}
/// $028812: nop
pub fn i_028812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28814;
    cpu.ir = 0x4E71;
    4
}
/// $028814: cmp.w d1, d0
pub fn i_028814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28816;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028816: nop
pub fn i_028816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28818;
    cpu.ir = 0x4E71;
    4
}
/// $028818: trap #$f
pub fn i_028818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2881A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02881A: move.w $4(a2), d2
pub fn i_02881a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2881E;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02881E: move.w $6(a2), d3
pub fn i_02881e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28822;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028822: btst.b #$1, $3a(a1)
pub fn i_028822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28828;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028828: beq.w $28832
pub fn i_028828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2882C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28832; return 10; }
    8
}
/// $02882C: neg.w d2
pub fn i_02882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2882E;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02882E: neg.w d3
pub fn i_02882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28830;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028830: exg.l d2, d3
pub fn i_028830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28832;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $028832: cmp.w d3, d2
pub fn i_028832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28834;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028834: blt.w $28842
pub fn i_028834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28838;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28842; return 10; }
    8
}
/// $028838: nop
pub fn i_028838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2883A;
    cpu.ir = 0x4E71;
    4
}
/// $02883A: nop
pub fn i_02883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2883C;
    cpu.ir = 0x4E71;
    4
}
/// $02883C: cmp.w d3, d2
pub fn i_02883c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2883E;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02883E: nop
pub fn i_02883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28840;
    cpu.ir = 0x4E71;
    4
}
/// $028840: trap #$f
pub fn i_028840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28842;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028842: add.w $24(a6), d0
pub fn i_028842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28846;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028846: add.w $24(a6), d1
pub fn i_028846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2884A;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02884A: add.w $24(a1), d2
pub fn i_02884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2884E;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02884E: add.w $24(a1), d3
pub fn i_02884e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28852;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028852: cmp.w d1, d2
pub fn i_028852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28854;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028854: bgt.w $2885e
pub fn i_028854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28858;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2885E; return 10; }
    8
}
/// $028858: cmp.w d3, d0
pub fn i_028858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2885A;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02885A: ble.w $28864
pub fn i_02885a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2885E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28864; return 10; }
    8
}
// ===== ClearXN_02885e  ($02885E, 6 B, thunks/ccr_helpers.c)
/// $02885E: andi.b #$ee, ccr
pub fn i_02885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28862;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028862: rts
pub fn i_028862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28864;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapRect_Store_028864  ($028864, 28 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028864: cmp.w d2, d0
pub fn i_028864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28866;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028866: bcc.w $2886c
pub fn i_028866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2886A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2886C; return 10; }
    8
}
/// $02886A: move.w d2, d0
pub fn i_02886a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2886C;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02886C: cmp.w d3, d1
pub fn i_02886c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2886E;
    cpu.ir = 0xB243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02886E: bcs.w $28874
pub fn i_02886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28872;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28874; return 10; }
    8
}
/// $028872: move.w d3, d1
pub fn i_028872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28874;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028874: add.w d0, d1
pub fn i_028874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28876;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028876: asr.w #$1, d1
pub fn i_028876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28878;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $028878: move.w d1, $56(a1)
pub fn i_028878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2887C;
    cpu.ir = 0x3341;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x56);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02887C: move.w d7, $54(a1)
pub fn i_02887c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28880;
    cpu.ir = 0x3347;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x54);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_028880  ($028880, 6 B, thunks/ccr_helpers.c)
/// $028880: ori.b #$11, ccr
pub fn i_028880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28884;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028884: rts
pub fn i_028884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $028930: cmpi.b #$0, $6(a0)
pub fn i_028930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28936;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $028936: bne.w $28986
pub fn i_028936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2893A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28986; return 10; }
    8
}
/// $02893A: cmpa.l #$ffffffff, a4
pub fn i_02893a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28940;
    cpu.ir = 0xB9FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[4];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028940: bne.w $28948
pub fn i_028940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28944;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28948; return 10; }
    8
}
/// $028944: lea.l $28926(pc), a4
pub fn i_028944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28948;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x28926;
    16
}
/// $028948: move.w #$0, d0
pub fn i_028948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2894C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02894C: move.w #$140, d1
pub fn i_02894c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28950;
    cpu.ir = 0x323C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028950: move.w #$100, d2
pub fn i_028950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28954;
    cpu.ir = 0x343C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028954: move.w #$1f0, d3
pub fn i_028954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28958;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1F0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028958: add.w (a4), d0
pub fn i_028958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2895A;
    cpu.ir = 0xD054;
    let v2 = bus.read16(cpu.a[4]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02895A: add.w $2(a4), d1
pub fn i_02895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2895E;
    cpu.ir = 0xD26C;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02895E: sub.w $6(a4), d2
pub fn i_02895e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28962;
    cpu.ir = 0x946C;
    let ad2 = cpu.a[4].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028962: sub.w $4(a4), d3
pub fn i_028962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28966;
    cpu.ir = 0x966C;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028966: cmp.w $22(a6), d0
pub fn i_028966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2896A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02896A: bgt.w $2898c
pub fn i_02896a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2896E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2898C; return 10; }
    8
}
/// $02896E: cmp.w $22(a6), d1
pub fn i_02896e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28972;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $028972: blt.w $2898c
pub fn i_028972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28976;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2898C; return 10; }
    8
}
/// $028976: cmp.w $24(a6), d2
pub fn i_028976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2897A;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02897A: bgt.w $2898c
pub fn i_02897a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2897E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2898C; return 10; }
    8
}
/// $02897E: cmp.w $24(a6), d3
pub fn i_02897e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28982;
    cpu.ir = 0xB66E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $028982: blt.w $2898c
pub fn i_028982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28986;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2898C; return 10; }
    8
}
// ===== ClearC_028986  ($028986, 6 B, thunks/ccr_helpers.c)
/// $028986: andi.b #$fe, ccr
pub fn i_028986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2898A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $02898A: rts
pub fn i_02898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2898C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_02898c  ($02898C, 6 B, thunks/ccr_helpers.c)
/// $02898C: ori.b #$1, ccr
pub fn i_02898c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28990;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $028990: rts
pub fn i_028990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28992;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_028992  ($028992, 6 B, thunks/ccr_helpers.c)
/// $028992: andi.b #$ee, ccr
pub fn i_028992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28996;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028996: rts
pub fn i_028996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28998;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Dispatch3Slots_028998  ($028998, 94 B, player/dispatch_3slots_028998.s)
/// $028998: cmpi.w #$ffff, $60(a6)
pub fn i_028998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2899E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $02899E: beq.w $289f0
pub fn i_02899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x289F0; return 10; }
    8
}
/// $0289A2: movea.l $60(a6), a0
pub fn i_0289a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289A6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0289A6: cmpi.b #$80, (a0)
pub fn i_0289a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289AA;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x80, Size::Byte, false);
    12
}
/// $0289AA: bne.w $289f0
pub fn i_0289aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x289F0; return 10; }
    8
}
/// $0289AE: lea.l $100580.l, a1
pub fn i_0289ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289B4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $0289B4: jsr $28a96(pc)
pub fn i_0289b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x289B8);
    cpu.pc = 0x28A96;
    16
}
/// $0289B8: lea.l $100440.l, a1
pub fn i_0289b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $0289BE: jsr $28a96(pc)
pub fn i_0289be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x289C2);
    cpu.pc = 0x28A96;
    16
}
/// $0289C2: bcs.w $289d8
pub fn i_0289c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x289D8; return 10; }
    8
}
/// $0289C6: lea.l $1004e0.l, a1
pub fn i_0289c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $0289CC: jsr $28a96(pc)
pub fn i_0289cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x289D0);
    cpu.pc = 0x28A96;
    16
}
/// $0289D0: bcc.w $289f0
pub fn i_0289d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x289F0; return 10; }
    8
}
/// $0289D4: bra.w $289ea
pub fn i_0289d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x289EA;
    10
}
/// $0289D8: movem.l a1, -(a7)
pub fn i_0289d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289DC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0289DC: lea.l $1004e0.l, a1
pub fn i_0289dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $0289E2: jsr $28a96(pc)
pub fn i_0289e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x289E6);
    cpu.pc = 0x28A96;
    16
}
/// $0289E6: movem.l (a7)+, a1
pub fn i_0289e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289EA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0289EA: ori.b #$11, ccr
pub fn i_0289ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289EE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0289EE: rts
pub fn i_0289ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0289F0: andi.b #$ee, ccr
pub fn i_0289f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289F4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0289F4: rts
pub fn i_0289f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_HitboxCollide_028A96  ($028A96, 114 B, entity/hitbox_collide_028a96.s)
/// $028A96: cmpa.l a6, a1
pub fn i_028a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28A98;
    cpu.ir = 0xB3CE;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028A98: beq.w $28b08
pub fn i_028a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28A9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28B08; return 10; }
    8
}
/// $028A9C: movea.l $60(a1), a2
pub fn i_028a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AA0;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x60);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $028AA0: cmpa.l #$ffffffff, a2
pub fn i_028aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AA6;
    cpu.ir = 0xB5FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028AA6: beq.w $28b08
pub fn i_028aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28B08; return 10; }
    8
}
/// $028AAA: cmpi.b #$80, (a2)
pub fn i_028aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AAE;
    cpu.ir = 0xC12;
    let v2 = bus.read8(cpu.a[2]) as u32;
    sub_with_flags(cpu, v2, 0x80, Size::Byte, false);
    12
}
/// $028AAE: bne.w $28b08
pub fn i_028aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28B08; return 10; }
    8
}
/// $028AB2: btst.b #$3, $69(a1)
pub fn i_028ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AB8;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028AB8: bne.w $28b08
pub fn i_028ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ABC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28B08; return 10; }
    8
}
/// $028ABC: movea.l a2, a3
pub fn i_028abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ABE;
    cpu.ir = 0x264A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $028ABE: adda.l #$6, a3
pub fn i_028abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AC4;
    cpu.ir = 0xD7FC;
    let v1: u32 = 0x6;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $028AC4: movea.l a0, a4
pub fn i_028ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AC6;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $028AC6: adda.l #$6, a4
pub fn i_028ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ACC;
    cpu.ir = 0xD9FC;
    let v1: u32 = 0x6;
    cpu.a[4] = cpu.a[4].wrapping_add(v1);
    8
}
/// $028ACC: jsr $28b14(pc)
pub fn i_028acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28AD0);
    cpu.pc = 0x28B14;
    16
}
/// $028AD0: bcs.w $28ad8
pub fn i_028ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AD4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28AD8; return 10; }
    8
}
/// $028AD4: bra.w $28b08
pub fn i_028ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x28B08;
    10
}
/// $028AD8: addq.b #$1, d7
pub fn i_028ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ADA;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028ADA: btst.b #$6, $6b(a1)
pub fn i_028ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AE0;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028AE0: bne.w $28ae8
pub fn i_028ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28AE8; return 10; }
    8
}
/// $028AE4: bset.b d7, $69(a6)
pub fn i_028ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AE8;
    cpu.ir = 0xFEE;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028AE8: addq.b #$1, d6
pub fn i_028ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AEA;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028AEA: bset.b d6, $69(a1)
pub fn i_028aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AEE;
    cpu.ir = 0xDE9;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028AEE: bclr.b #$2, $69(a1)
pub fn i_028aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AF4;
    cpu.ir = 0x8A9;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028AF4: cmpi.b #$2, $1(a0)
pub fn i_028af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AFA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $028AFA: bne.w $28b04
pub fn i_028afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28B04; return 10; }
    8
}
/// $028AFE: bset.b #$2, $69(a1)
pub fn i_028afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B04;
    cpu.ir = 0x8E9;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028B04: bra.w $28b0e
pub fn i_028b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B08;
    cpu.ir = 0x6000;
    cpu.pc = 0x28B0E;
    10
}
// ===== ClearXN_028b08  ($028B08, 6 B, thunks/ccr_helpers.c)
/// $028B08: andi.b #$ee, ccr
pub fn i_028b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B0C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028B0C: rts
pub fn i_028b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_028b0e  ($028B0E, 6 B, thunks/ccr_helpers.c)
/// $028B0E: ori.b #$11, ccr
pub fn i_028b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B12;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028B12: rts
pub fn i_028b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapTestXY_028B14  ($028B14, 268 B, entity/hitbox_collide_028a96.s)
/// $028B14: move.w (a4), d0
pub fn i_028b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B16;
    cpu.ir = 0x3014;
    let v2 = bus.read16(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028B16: move.w $2(a4), d1
pub fn i_028b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B1A;
    cpu.ir = 0x322C;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028B1A: btst.b #$0, $3a(a6)
pub fn i_028b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B20;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028B20: beq.w $28b2a
pub fn i_028b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28B2A; return 10; }
    8
}
/// $028B24: neg.w d0
pub fn i_028b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B26;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028B26: neg.w d1
pub fn i_028b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B28;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028B28: exg.l d0, d1
pub fn i_028b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B2A;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $028B2A: cmp.w d1, d0
pub fn i_028b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B2C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B2C: blt.w $28b3a
pub fn i_028b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B30;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28B3A; return 10; }
    8
}
/// $028B30: nop
pub fn i_028b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B32;
    cpu.ir = 0x4E71;
    4
}
/// $028B32: nop
pub fn i_028b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B34;
    cpu.ir = 0x4E71;
    4
}
/// $028B34: cmp.w d1, d0
pub fn i_028b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B36;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B36: nop
pub fn i_028b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B38;
    cpu.ir = 0x4E71;
    4
}
/// $028B38: trap #$f
pub fn i_028b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B3A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028B3A: move.w (a3), d2
pub fn i_028b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B3C;
    cpu.ir = 0x3413;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028B3C: move.w $2(a3), d3
pub fn i_028b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B40;
    cpu.ir = 0x362B;
    let ad2 = cpu.a[3].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028B40: btst.b #$0, $3a(a1)
pub fn i_028b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B46;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028B46: beq.w $28b50
pub fn i_028b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28B50; return 10; }
    8
}
/// $028B4A: neg.w d2
pub fn i_028b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B4C;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028B4C: neg.w d3
pub fn i_028b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B4E;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028B4E: exg.l d2, d3
pub fn i_028b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B50;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $028B50: cmp.w d3, d2
pub fn i_028b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B52;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B52: blt.w $28b60
pub fn i_028b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B56;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28B60; return 10; }
    8
}
/// $028B56: nop
pub fn i_028b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B58;
    cpu.ir = 0x4E71;
    4
}
/// $028B58: nop
pub fn i_028b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B5A;
    cpu.ir = 0x4E71;
    4
}
/// $028B5A: cmp.w d3, d2
pub fn i_028b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B5C;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B5C: nop
pub fn i_028b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B5E;
    cpu.ir = 0x4E71;
    4
}
/// $028B5E: trap #$f
pub fn i_028b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B60;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028B60: add.w $22(a6), d0
pub fn i_028b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B64;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B64: add.w $22(a6), d1
pub fn i_028b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B68;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B68: add.w $22(a1), d2
pub fn i_028b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B6C;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B6C: add.w $22(a1), d3
pub fn i_028b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B70;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B70: cmp.w d1, d2
pub fn i_028b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B72;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B72: bgt.w $28b7c
pub fn i_028b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B76;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28B7C; return 10; }
    8
}
/// $028B76: cmp.w d3, d0
pub fn i_028b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B78;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B78: ble.w $28b82
pub fn i_028b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B7C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28B82; return 10; }
    8
}
/// $028B7C: andi.b #$ee, ccr
pub fn i_028b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B80;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028B80: rts
pub fn i_028b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $028B82: cmp.w d2, d0
pub fn i_028b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B84;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B84: bcc.w $28b8a
pub fn i_028b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28B8A; return 10; }
    8
}
/// $028B88: move.w d2, d0
pub fn i_028b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B8A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028B8A: cmp.w d3, d1
pub fn i_028b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B8C;
    cpu.ir = 0xB243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B8C: bcs.w $28b92
pub fn i_028b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B90;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28B92; return 10; }
    8
}
/// $028B90: move.w d3, d1
pub fn i_028b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B92;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028B92: move.w $22(a1), d4
pub fn i_028b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B96;
    cpu.ir = 0x3829;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028B96: move.w $22(a6), d5
pub fn i_028b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B9A;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028B9A: add.w $2(a2), d4
pub fn i_028b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B9E;
    cpu.ir = 0xD86A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B9E: add.w $2(a0), d5
pub fn i_028b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA2;
    cpu.ir = 0xDA68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028BA2: cmp.w d4, d5
pub fn i_028ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA4;
    cpu.ir = 0xBA44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BA4: slt.b d7
pub fn i_028ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA6;
    cpu.ir = 0x5DC7;
    let c1: u32 = if cpu.sr.evaluate(Condition::Lt) { 0xFF } else { 0 };
    let v2 = cpu.d[7] & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (c1 & 0xFF);
    4
}
/// $028BA6: sge.b d6
pub fn i_028ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA8;
    cpu.ir = 0x5CC6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ge) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $028BA8: move.w $4(a4), d0
pub fn i_028ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BAC;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028BAC: move.w $6(a4), d1
pub fn i_028bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BB0;
    cpu.ir = 0x322C;
    let ad2 = cpu.a[4].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028BB0: btst.b #$1, $3a(a6)
pub fn i_028bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BB6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028BB6: beq.w $28bc0
pub fn i_028bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28BC0; return 10; }
    8
}
/// $028BBA: neg.w d0
pub fn i_028bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BBC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028BBC: neg.w d1
pub fn i_028bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BBE;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028BBE: exg.l d0, d1
pub fn i_028bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC0;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $028BC0: cmp.w d1, d0
pub fn i_028bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC2;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BC2: blt.w $28bd0
pub fn i_028bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28BD0; return 10; }
    8
}
/// $028BC6: nop
pub fn i_028bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC8;
    cpu.ir = 0x4E71;
    4
}
/// $028BC8: nop
pub fn i_028bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BCA;
    cpu.ir = 0x4E71;
    4
}
/// $028BCA: cmp.w d1, d0
pub fn i_028bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BCC;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BCC: nop
pub fn i_028bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BCE;
    cpu.ir = 0x4E71;
    4
}
/// $028BCE: trap #$f
pub fn i_028bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BD0;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028BD0: move.w $4(a3), d2
pub fn i_028bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BD4;
    cpu.ir = 0x342B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028BD4: move.w $6(a3), d3
pub fn i_028bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BD8;
    cpu.ir = 0x362B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028BD8: btst.b #$1, $3a(a1)
pub fn i_028bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BDE;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028BDE: beq.w $28be8
pub fn i_028bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BE2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28BE8; return 10; }
    8
}
/// $028BE2: neg.w d2
pub fn i_028be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BE4;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028BE4: neg.w d3
pub fn i_028be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BE6;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028BE6: exg.l d2, d3
pub fn i_028be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BE8;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $028BE8: cmp.w d3, d2
pub fn i_028be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BEA;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BEA: blt.w $28bf8
pub fn i_028bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BEE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28BF8; return 10; }
    8
}
/// $028BEE: nop
pub fn i_028bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF0;
    cpu.ir = 0x4E71;
    4
}
/// $028BF0: nop
pub fn i_028bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF2;
    cpu.ir = 0x4E71;
    4
}
/// $028BF2: cmp.w d3, d2
pub fn i_028bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF4;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BF4: nop
pub fn i_028bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF6;
    cpu.ir = 0x4E71;
    4
}
/// $028BF6: trap #$f
pub fn i_028bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028BF8: add.w $24(a6), d0
pub fn i_028bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BFC;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028BFC: add.w $24(a6), d1
pub fn i_028bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C00;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C00: add.w $24(a1), d2
pub fn i_028c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C04;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C04: add.w $24(a1), d3
pub fn i_028c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C08;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C08: cmp.w d1, d2
pub fn i_028c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C0A;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C0A: bgt.w $28c14
pub fn i_028c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C0E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28C14; return 10; }
    8
}
/// $028C0E: cmp.w d3, d0
pub fn i_028c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C10;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C10: ble.w $28c1a
pub fn i_028c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C14;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28C1A; return 10; }
    8
}
/// $028C14: andi.b #$ee, ccr
pub fn i_028c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C18;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028C18: rts
pub fn i_028c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $028C1A: ori.b #$11, ccr
pub fn i_028c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C1E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028C1E: rts
pub fn i_028c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntitySetSpriteMap  ($028CD4, 28 B, entity/entity_setters.c)
/// $028CD4: move.l a0, $3c(a6)
pub fn i_028cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CD8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028CD8: move.l a0, $40(a6)
pub fn i_028cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CDC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028CDC: clr.b $3b(a6)
pub fn i_028cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CE0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $028CE0: clr.b $46(a6)
pub fn i_028ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CE4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $028CE4: clr.b $47(a6)
pub fn i_028ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CE8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x47);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $028CE8: bset.b #$5, $69(a6)
pub fn i_028ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CEE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028CEE: rts
pub fn i_028cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_HasLinkedSlots  ($028D70, 30 B, entity/has_linked_028d70.s)
/// $028D70: movea.l $40(a6), a1
pub fn i_028d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D74;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028D74: cmpa.l #$ffffffff, a1
pub fn i_028d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7A;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028D7A: beq.w $28d8c
pub fn i_028d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28D8C; return 10; }
    8
}
/// $028D7E: movea.l $3c(a6), a1
pub fn i_028d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D82;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028D82: cmpa.l #$ffffffff, a1
pub fn i_028d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D88;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028D88: bne.w $28d8e
pub fn i_028d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D8E; return 10; }
    8
}
/// $028D8C: rts
pub fn i_028d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Script_DispatchOpcode  ($028D8E, 70 B, entity/script_dispatch_028d8e.s)
/// $028D8E: movea.l $3c(a6), a1
pub fn i_028d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D92;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028D92: cmpa.l $40(a6), a1
pub fn i_028d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D96;
    cpu.ir = 0xB3EE;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    18
}
/// $028D96: beq.w $28da8
pub fn i_028d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28DA8; return 10; }
    8
}
/// $028D9A: move.b $3b(a6), d1
pub fn i_028d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028D9E: addq.b #$1, d1
pub fn i_028d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA0;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028DA0: beq.w $28da8
pub fn i_028da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28DA8; return 10; }
    8
}
/// $028DA4: move.b d1, $3b(a6)
pub fn i_028da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA8;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $028DA8: moveq #$0, d2
pub fn i_028da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAA;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028DAA: move.b (a1), d2
pub fn i_028daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAC;
    cpu.ir = 0x1411;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $028DAC: cmpi.b #$20, d2
pub fn i_028dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB0;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $028DB0: bcs.w $28dc0
pub fn i_028db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28DC0; return 10; }
    8
}
/// $028DB4: nop
pub fn i_028db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB6;
    cpu.ir = 0x4E71;
    4
}
/// $028DB6: nop
pub fn i_028db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB8;
    cpu.ir = 0x4E71;
    4
}
/// $028DB8: cmpi.b #$20, d2
pub fn i_028db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DBC;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $028DBC: nop
pub fn i_028dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DBE;
    cpu.ir = 0x4E71;
    4
}
/// $028DBE: trap #$f
pub fn i_028dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DC0;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028DC0: andi.l #$ff, d2
pub fn i_028dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DC6;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $028DC6: lsl.l #$2, d2
pub fn i_028dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DC8;
    cpu.ir = 0xE58A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 1, true);
    cpu.d[2] = r;
    10
}
/// $028DC8: movea.l #$28cf0, a2
pub fn i_028dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DCE;
    cpu.ir = 0x247C;
    let v1: u32 = 0x28CF0;
    cpu.a[2] = v1;
    4
}
/// $028DCE: movea.l (a2, d2.w), a3
pub fn i_028dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD2;
    cpu.ir = 0x2672;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $028DD2: jmp (a3)
pub fn i_028dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD4;
    cpu.ir = 0x4ED3;
    cpu.jump_to(cpu.a[3]);
    8
}
// ===== ScriptOp00_Wait_028dd4  ($028DD4, 30 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028DD4: move.b $46(a6), d2
pub fn i_028dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD8;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028DD8: bne.w $28de4
pub fn i_028dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28DE4; return 10; }
    8
}
/// $028DDC: move.b $1(a1), d2
pub fn i_028ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DE0;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028DE0: move.b d2, $46(a6)
pub fn i_028de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DE4;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write8(ad2, v1 as u8);
    12
}
/// $028DE4: adda.l #$2, a1
pub fn i_028de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DEA;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $028DEA: move.l a1, $3c(a6)
pub fn i_028dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DEE;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028DEE: jmp $28da8(pc)
pub fn i_028dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DF2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp01_Jump_028df2  ($028DF2, 12 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028DF2: movea.l $2(a1), a1
pub fn i_028df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DF6;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028DF6: move.l a1, $3c(a6)
pub fn i_028df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DFA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028DFA: jmp $28da8(pc)
pub fn i_028dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DFE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp02_Sprite_028dfe  ($028DFE, 546 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028DFE: move.l a1, $40(a6)
pub fn i_028dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E02;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028E02: move.b $1(a1), d1
pub fn i_028e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E06;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028E06: move.b $3a(a6), d2
pub fn i_028e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E0A;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028E0A: eor.b d1, d2
pub fn i_028e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E0C;
    cpu.ir = 0xB302;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028E0C: btst.b #$3, $1(a1)
pub fn i_028e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E12;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028E12: beq.w $28e22
pub fn i_028e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E22; return 10; }
    8
}
/// $028E16: move.w $22(a6), d0
pub fn i_028e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E1A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028E1A: move.w $24(a6), d1
pub fn i_028e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E1E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028E1E: bra.w $28e46
pub fn i_028e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E22;
    cpu.ir = 0x6000;
    cpu.pc = 0x28E46;
    10
}
/// $028E22: move.w $8(a1), d0
pub fn i_028e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E26;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028E26: btst.b #$0, d2
pub fn i_028e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E2A;
    cpu.ir = 0x802;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $028E2A: beq.w $28e30
pub fn i_028e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E30; return 10; }
    8
}
/// $028E2E: neg.w d0
pub fn i_028e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E30;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028E30: add.w $22(a6), d0
pub fn i_028e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E34;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028E34: move.w $a(a1), d1
pub fn i_028e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E38;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028E38: btst.b #$1, d2
pub fn i_028e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E3C;
    cpu.ir = 0x802;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $028E3C: beq.w $28e42
pub fn i_028e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E42; return 10; }
    8
}
/// $028E40: neg.w d1
pub fn i_028e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E42;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028E42: add.w $24(a6), d1
pub fn i_028e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E46;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028E46: swap d0
pub fn i_028e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E48;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028E48: subi.w #$1, d1
pub fn i_028e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E4C;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $028E4C: move.w d1, d0
pub fn i_028e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E4E;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028E4E: move.b $59(a6), d1
pub fn i_028e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E52;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028E52: andi.b #$1, d1
pub fn i_028e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E56;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028E56: beq.w $28e9c
pub fn i_028e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E9C; return 10; }
    8
}
/// $028E5A: btst.b #$6, $6b(a6)
pub fn i_028e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E60;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028E60: beq.w $28e6e
pub fn i_028e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E64;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E6E; return 10; }
    8
}
/// $028E64: bset.b #$7, $5a(a6)
pub fn i_028e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E6A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028E6A: bra.w $28e9c
pub fn i_028e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x28E9C;
    10
}
/// $028E6E: btst.b #$0, $6b(a6)
pub fn i_028e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E74;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028E74: beq.w $28e82
pub fn i_028e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E78;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E82; return 10; }
    8
}
/// $028E78: bset.b #$7, $5a(a6)
pub fn i_028e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E7E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028E7E: bra.w $28e9c
pub fn i_028e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E82;
    cpu.ir = 0x6000;
    cpu.pc = 0x28E9C;
    10
}
/// $028E82: btst.b #$1, $100001.l
pub fn i_028e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E8A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((1) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $028E8A: bne.w $28e96
pub fn i_028e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28E96; return 10; }
    8
}
/// $028E8E: jmp $28fca(pc)
pub fn i_028e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E92;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28FCA;
    8
}
/// $028E96: bset.b #$7, $5a(a6)
pub fn i_028e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E9C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028E9C: btst.b #$5, $12(a6)
pub fn i_028e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EA2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028EA2: beq.w $28ec6
pub fn i_028ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28EC6; return 10; }
    8
}
/// $028EA6: bclr.b #$5, $12(a6)
pub fn i_028ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EAC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028EAC: move.b $12(a6), d1
pub fn i_028eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EB0;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028EB0: andi.b #$10, d1
pub fn i_028eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EB4;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028EB4: ror.b #$4, d1
pub fn i_028eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EB6;
    cpu.ir = 0xE819;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 3, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $028EB6: move.b $1(a1), d2
pub fn i_028eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EBA;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028EBA: andi.b #$10, d2
pub fn i_028eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EBE;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028EBE: ror.b #$4, d2
pub fn i_028ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EC0;
    cpu.ir = 0xE81A;
    let h1 = cpu.d[2] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 3, false);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    14
}
/// $028EC0: eor.b d2, d1
pub fn i_028ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EC2;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028EC2: bne.w $28fca
pub fn i_028ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FCA; return 10; }
    8
}
/// $028EC6: btst.b #$2, $1(a1)
pub fn i_028ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ECC;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028ECC: beq.w $28eda
pub fn i_028ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ED0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28EDA; return 10; }
    8
}
/// $028ED0: btst.b #$3, $12(a6)
pub fn i_028ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ED6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028ED6: bne.w $28fca
pub fn i_028ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EDA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FCA; return 10; }
    8
}
/// $028EDA: move.w $38(a6), d1
pub fn i_028eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EDE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028EDE: cmpi.w #$ffff, $6(a1)
pub fn i_028ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EE4;
    cpu.ir = 0xC69;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $028EE4: beq.w $28eec
pub fn i_028ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28EEC; return 10; }
    8
}
/// $028EE8: move.w $6(a1), d1
pub fn i_028ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EEC;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028EEC: move.w $14(a6), d2
pub fn i_028eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EF0;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028EF0: btst.b #$7, $5a(a6)
pub fn i_028ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EF6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028EF6: bne.w $28f04
pub fn i_028ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EFA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28F04; return 10; }
    8
}
/// $028EFA: btst.b #$0, $5a(a6)
pub fn i_028efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F00;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028F00: beq.w $28f22
pub fn i_028f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28F22; return 10; }
    8
}
/// $028F04: btst.b #$0, $13(a6)
pub fn i_028f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F0A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028F0A: bne.w $28f22
pub fn i_028f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F0E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28F22; return 10; }
    8
}
/// $028F0E: cmpi.w #$ffff, $1c(a6)
pub fn i_028f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $028F14: beq.w $28f22
pub fn i_028f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F18;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28F22; return 10; }
    8
}
/// $028F18: move.w $1c(a6), d2
pub fn i_028f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F1C;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028F1C: bset.b #$7, $5a(a6)
pub fn i_028f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F22;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028F22: btst.b #$4, $100001.l
pub fn i_028f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F2A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $028F2A: beq.w $28f3e
pub fn i_028f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28F3E; return 10; }
    8
}
/// $028F2E: btst.b #$0, $6b(a6)
pub fn i_028f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F34;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028F34: beq.w $28f3e
pub fn i_028f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28F3E; return 10; }
    8
}
/// $028F38: jsr $2f84a.l
pub fn i_028f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28F3E);
    cpu.pc = 0x2F84A;
    16
}
/// $028F3E: move.b $3a(a6), d5
pub fn i_028f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F42;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F42: move.b $12(a6), d4
pub fn i_028f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F46;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F46: andi.b #$4, d4
pub fn i_028f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F4A;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028F4A: or.b d4, d5
pub fn i_028f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F4C;
    cpu.ir = 0x8A04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $028F4C: move.b $1(a1), d4
pub fn i_028f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F50;
    cpu.ir = 0x1829;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F50: andi.b #$3, d4
pub fn i_028f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F54;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028F54: eor.b d4, d5
pub fn i_028f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F56;
    cpu.ir = 0xB905;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028F56: move.b $33(a6), d3
pub fn i_028f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F5A;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F5A: move.b $32(a6), d4
pub fn i_028f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F5E;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F5E: movea.l $2(a1), a0
pub fn i_028f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F62;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $028F62: movem.l d0, -(a7)
pub fn i_028f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F66;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $028F66: movem.l d1, -(a7)
pub fn i_028f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F6A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $028F6A: move.w d0, d1
pub fn i_028f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F6C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028F6C: swap d0
pub fn i_028f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F6E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028F6E: cmpi.w #$ff00, d0
pub fn i_028f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F72;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF00, Size::Word, false);
    8
}
/// $028F72: bgt.w $28f7a
pub fn i_028f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F76;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28F7A; return 10; }
    8
}
/// $028F76: bra.w $28fa2
pub fn i_028f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA2;
    10
}
/// $028F7A: cmpi.w #$200, d0
pub fn i_028f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F7E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x200, Size::Word, false);
    8
}
/// $028F7E: blt.w $28f86
pub fn i_028f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F82;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28F86; return 10; }
    8
}
/// $028F82: bra.w $28fa2
pub fn i_028f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F86;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA2;
    10
}
/// $028F86: cmpi.w #$0, d1
pub fn i_028f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F8A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $028F8A: bgt.w $28f92
pub fn i_028f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F8E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28F92; return 10; }
    8
}
/// $028F8E: bra.w $28fa2
pub fn i_028f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F92;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA2;
    10
}
/// $028F92: cmpi.w #$300, d1
pub fn i_028f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F96;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $028F96: blt.w $28f9e
pub fn i_028f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F9A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28F9E; return 10; }
    8
}
/// $028F9A: bra.w $28fa2
pub fn i_028f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA2;
    10
}
/// $028F9E: bra.w $28fa8
pub fn i_028f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA8;
    10
}
/// $028FA2: movea.l #$ffffffff, a0
pub fn i_028fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FA8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $028FA8: movem.l (a7)+, d1
pub fn i_028fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FAC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $028FAC: movem.l (a7)+, d0
pub fn i_028fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FB0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $028FB0: btst.b #$6, $12(a6)
pub fn i_028fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FB6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028FB6: bne.w $28fc4
pub fn i_028fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FBA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FC4; return 10; }
    8
}
/// $028FBA: jsr $5a9e2.l
pub fn i_028fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28FC0);
    cpu.pc = 0x5A9E2;
    16
}
/// $028FC0: bra.w $28fca
pub fn i_028fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FCA;
    10
}
/// $028FC4: jsr $5a9d6.l
pub fn i_028fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28FCA);
    cpu.pc = 0x5A9D6;
    16
}
/// $028FCA: move.b $1(a1), d0
pub fn i_028fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FCE;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028FCE: adda.l #$8, a1
pub fn i_028fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FD4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x8;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $028FD4: btst.b #$3, d0
pub fn i_028fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FD8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $028FD8: bne.w $28fe2
pub fn i_028fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FE2; return 10; }
    8
}
/// $028FDC: adda.l #$4, a1
pub fn i_028fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FE2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $028FE2: cmpi.b #$18, (a1)
pub fn i_028fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FE6;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x18, Size::Byte, false);
    12
}
/// $028FE6: bne.w $28fee
pub fn i_028fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FEE; return 10; }
    8
}
/// $028FEA: jmp $2934c(pc)
pub fn i_028fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FEE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2934C;
    8
}
/// $028FEE: cmpi.b #$2, (a1)
pub fn i_028fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FF2;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $028FF2: bne.w $28ffa
pub fn i_028ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FFA; return 10; }
    8
}
/// $028FF6: jmp $28e02(pc)
pub fn i_028ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FFA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28E02;
    8
}
/// $028FFA: tst.b $46(a6)
pub fn i_028ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FFE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $028FFE: beq.w $29026
pub fn i_028ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29002;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29026; return 10; }
    8
}
/// $029002: tst.b $44(a6)
pub fn i_029002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29006;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $029006: bne.w $29020
pub fn i_029006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2900A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29020; return 10; }
    8
}
/// $02900A: subi.b #$1, $46(a6)
pub fn i_02900a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29010;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $029010: bne.w $29020
pub fn i_029010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29014;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29020; return 10; }
    8
}
/// $029014: cmpi.b #$16, (a1)
pub fn i_029014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29018;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x16, Size::Byte, false);
    12
}
/// $029018: beq.w $29026
pub fn i_029018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2901C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29026; return 10; }
    8
}
/// $02901C: move.l a1, $3c(a6)
pub fn i_02901c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29020;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== ClearXN_029020  ($029020, 6 B, thunks/ccr_helpers.c)
/// $029020: andi.b #$ee, ccr
pub fn i_029020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29024;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $029024: rts
pub fn i_029024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29026;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_029026  ($029026, 6 B, thunks/ccr_helpers.c)
/// $029026: ori.b #$11, ccr
pub fn i_029026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2902A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02902A: rts
pub fn i_02902a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2902C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptOp03_Loop_02902c  ($02902C, 66 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02902C: tst.b $47(a6)
pub fn i_02902c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29030;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $029030: bne.w $2903e
pub fn i_029030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29034;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2903E; return 10; }
    8
}
/// $029034: move.b $1(a1), d1
pub fn i_029034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29038;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $029038: addq.b #$1, d1
pub fn i_029038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2903A;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02903A: move.b d1, $47(a6)
pub fn i_02903a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2903E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02903E: move.b $47(a6), d1
pub fn i_02903e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29042;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $029042: clr.b d0
pub fn i_029042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29044;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $029044: subi.b #$1, d1
pub fn i_029044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29048;
    cpu.ir = 0x401;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029048: addx.b d0, d1
pub fn i_029048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2904A;
    cpu.ir = 0xD300;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let h2 = cpu.d[0] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $02904A: move.b d1, $47(a6)
pub fn i_02904a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2904E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02904E: tst.b d1
pub fn i_02904e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29050;
    cpu.ir = 0x4A01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $029050: beq.w $29060
pub fn i_029050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29054;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29060; return 10; }
    8
}
/// $029054: movea.l $2(a1), a1
pub fn i_029054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29058;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $029058: move.l a1, $3c(a6)
pub fn i_029058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2905C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02905C: jmp $28da8(pc)
pub fn i_02905c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29060;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
/// $029060: adda.l #$6, a1
pub fn i_029060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29066;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029066: move.l a1, $3c(a6)
pub fn i_029066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2906A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02906A: jmp $28da8(pc)
pub fn i_02906a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2906E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp04_Sound_02906e  ($02906E, 24 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02906E: move.w $2(a1), d0
pub fn i_02906e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29072;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $029072: jsr $2352.l
pub fn i_029072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29078);
    cpu.pc = 0x2352;
    16
}
/// $029078: adda.l #$4, a1
pub fn i_029078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2907E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02907E: move.l a1, $3c(a6)
pub fn i_02907e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29082;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $029082: jmp $28da8(pc)
pub fn i_029082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29086;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp05_SetByte_02909e  ($02909E, 44 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02909E: move.w $2(a1), d1
pub fn i_02909e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290A2;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0290A2: cmpi.w #$a0, d1
pub fn i_0290a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290A6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290A6: blt.w $290b6
pub fn i_0290a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290AA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x290B6; return 10; }
    8
}
/// $0290AA: nop
pub fn i_0290aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290AC;
    cpu.ir = 0x4E71;
    4
}
/// $0290AC: nop
pub fn i_0290ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290AE;
    cpu.ir = 0x4E71;
    4
}
/// $0290AE: cmpi.w #$a0, d1
pub fn i_0290ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290B2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290B2: nop
pub fn i_0290b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290B4;
    cpu.ir = 0x4E71;
    4
}
/// $0290B4: trap #$f
pub fn i_0290b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290B6;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0290B6: move.b $1(a1), (a6, d1.w)
pub fn i_0290b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290BC;
    cpu.ir = 0x1DA9;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $0290BC: adda.l #$4, a1
pub fn i_0290bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290C2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0290C2: move.l a1, $3c(a6)
pub fn i_0290c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290C6;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0290C6: jmp $28da8(pc)
pub fn i_0290c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290CA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp06_SetWord_0290ca  ($0290CA, 44 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0290CA: move.w $4(a1), d1
pub fn i_0290ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290CE;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0290CE: cmpi.w #$a0, d1
pub fn i_0290ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290D2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290D2: blt.w $290e2
pub fn i_0290d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290D6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x290E2; return 10; }
    8
}
/// $0290D6: nop
pub fn i_0290d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290D8;
    cpu.ir = 0x4E71;
    4
}
/// $0290D8: nop
pub fn i_0290d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290DA;
    cpu.ir = 0x4E71;
    4
}
/// $0290DA: cmpi.w #$a0, d1
pub fn i_0290da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290DE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290DE: nop
pub fn i_0290de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290E0;
    cpu.ir = 0x4E71;
    4
}
/// $0290E0: trap #$f
pub fn i_0290e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290E2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0290E2: move.w $2(a1), (a6, d1.w)
pub fn i_0290e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290E8;
    cpu.ir = 0x3DA9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    22
}
/// $0290E8: adda.l #$6, a1
pub fn i_0290e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290EE;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0290EE: move.l a1, $3c(a6)
pub fn i_0290ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290F2;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0290F2: jmp $28da8(pc)
pub fn i_0290f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290F6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp08_Call_029122  ($029122, 28 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029122: movem.l a1, -(a7)
pub fn i_029122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29126;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $029126: movea.l $2(a1), a1
pub fn i_029126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2912A;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $02912A: jsr (a1)
pub fn i_02912a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2912C;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x2912C);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $02912C: movem.l (a7)+, a1
pub fn i_02912c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29130;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $029130: adda.l #$6, a1
pub fn i_029130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29136;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029136: move.l a1, $3c(a6)
pub fn i_029136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2913A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02913A: jmp $28da8(pc)
pub fn i_02913a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2913E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp09_SetHitbox48_02913e  ($02913E, 20 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02913E: move.l $2(a1), $48(a6)
pub fn i_02913e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29144;
    cpu.ir = 0x2D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $029144: adda.l #$6, a1
pub fn i_029144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2914A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02914A: move.l a1, $3c(a6)
pub fn i_02914a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2914E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02914E: jmp $28da8(pc)
pub fn i_02914e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29152;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp0A_SetField4C_029152  ($029152, 20 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029152: move.l $2(a1), $4c(a6)
pub fn i_029152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29158;
    cpu.ir = 0x2D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $029158: adda.l #$6, a1
pub fn i_029158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2915E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02915E: move.l a1, $3c(a6)
pub fn i_02915e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29162;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $029162: jmp $28da8(pc)
pub fn i_029162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29166;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp0D_AndByte_029196  ($029196, 46 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029196: move.w $2(a1), d1
pub fn i_029196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2919A;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02919A: cmpi.w #$a0, d1
pub fn i_02919a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2919E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $02919E: blt.w $291ae
pub fn i_02919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291A2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x291AE; return 10; }
    8
}
/// $0291A2: nop
pub fn i_0291a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291A4;
    cpu.ir = 0x4E71;
    4
}
/// $0291A4: nop
pub fn i_0291a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291A6;
    cpu.ir = 0x4E71;
    4
}
/// $0291A6: cmpi.w #$a0, d1
pub fn i_0291a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291AA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0291AA: nop
pub fn i_0291aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291AC;
    cpu.ir = 0x4E71;
    4
}
/// $0291AC: trap #$f
pub fn i_0291ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291AE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0291AE: move.b $1(a1), d0
pub fn i_0291ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291B2;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0291B2: and.b d0, (a6, d1.w)
pub fn i_0291b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291B6;
    cpu.ir = 0xC136;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $0291B6: adda.l #$4, a1
pub fn i_0291b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291BC;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0291BC: move.l a1, $3c(a6)
pub fn i_0291bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291C0;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0291C0: jmp $28da8(pc)
pub fn i_0291c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291C4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp10_OrByte_029220  ($029220, 46 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029220: move.w $2(a1), d1
pub fn i_029220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29224;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $029224: cmpi.w #$a0, d1
pub fn i_029224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29228;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $029228: blt.w $29238
pub fn i_029228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2922C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x29238; return 10; }
    8
}
/// $02922C: nop
pub fn i_02922c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2922E;
    cpu.ir = 0x4E71;
    4
}
/// $02922E: nop
pub fn i_02922e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29230;
    cpu.ir = 0x4E71;
    4
}
/// $029230: cmpi.w #$a0, d1
pub fn i_029230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29234;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $029234: nop
pub fn i_029234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29236;
    cpu.ir = 0x4E71;
    4
}
/// $029236: trap #$f
pub fn i_029236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29238;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $029238: move.b $1(a1), d0
pub fn i_029238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2923C;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02923C: or.b d0, (a6, d1.w)
pub fn i_02923c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29240;
    cpu.ir = 0x8136;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $029240: adda.l #$4, a1
pub fn i_029240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29246;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029246: move.l a1, $3c(a6)
pub fn i_029246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2924A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02924A: jmp $28da8(pc)
pub fn i_02924a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2924E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp13_EorByte_0292aa  ($0292AA, 46 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0292AA: move.w $2(a1), d1
pub fn i_0292aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292AE;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0292AE: cmpi.w #$a0, d1
pub fn i_0292ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292B2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0292B2: blt.w $292c2
pub fn i_0292b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292B6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x292C2; return 10; }
    8
}
/// $0292B6: nop
pub fn i_0292b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292B8;
    cpu.ir = 0x4E71;
    4
}
/// $0292B8: nop
pub fn i_0292b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292BA;
    cpu.ir = 0x4E71;
    4
}
/// $0292BA: cmpi.w #$a0, d1
pub fn i_0292ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292BE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0292BE: nop
pub fn i_0292be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292C0;
    cpu.ir = 0x4E71;
    4
}
/// $0292C0: trap #$f
pub fn i_0292c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292C2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0292C2: move.b $1(a1), d0
pub fn i_0292c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292C6;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0292C6: eor.b d0, (a6, d1.w)
pub fn i_0292c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292CA;
    cpu.ir = 0xB136;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $0292CA: adda.l #$4, a1
pub fn i_0292ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292D0;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0292D0: move.l a1, $3c(a6)
pub fn i_0292d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292D4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0292D4: jmp $28da8(pc)
pub fn i_0292d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292D8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp16_RepeatSprite_029334  ($029334, 8 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029334: movea.l $40(a6), a1
pub fn i_029334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29338;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $029338: jmp $28dfe(pc)
pub fn i_029338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2933C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DFE;
    8
}
// ===== ScriptOp17_SpriteFlag5_02933c  ($02933C, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02933C: bset.b #$5, $12(a6)
pub fn i_02933c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29342;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $029342: adda.l #$2, a1
pub fn i_029342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29348;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029348: jmp $28dfe(pc)
pub fn i_029348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2934C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DFE;
    8
}
// ===== ScriptOp18_SpriteFlag5Cont_02934c  ($02934C, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02934C: bset.b #$5, $12(a6)
pub fn i_02934c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29352;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $029352: adda.l #$2, a1
pub fn i_029352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29358;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029358: jmp $28e02(pc)
pub fn i_029358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2935C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28E02;
    8
}
// ===== ScriptOp19_SetField70_02935c  ($02935C, 26 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02935C: move.l a1, $70(a6)
pub fn i_02935c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29360;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $029360: addi.l #$2, $70(a6)
pub fn i_029360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29368;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $029368: adda.l #$a, a1
pub fn i_029368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2936E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xA;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02936E: move.l a1, $3c(a6)
pub fn i_02936e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29372;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $029372: jmp $28da8(pc)
pub fn i_029372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29376;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp1E_SpawnChild_02942a  ($02942A, 128 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02942A: move.w $2(a1), d0
pub fn i_02942a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2942E;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02942E: cmpi.b #$b, $106ece.l
pub fn i_02942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29436;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xB, Size::Byte, false);
    20
}
/// $029436: beq.w $29446
pub fn i_029436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2943A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29446; return 10; }
    8
}
/// $02943A: cmpi.b #$c, $106ece.l
pub fn i_02943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29442;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $029442: bne.w $2944a
pub fn i_029442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29446;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2944A; return 10; }
    8
}
/// $029446: andi.w #$fff, d0
pub fn i_029446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2944A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02944A: move.w $4(a1), d1
pub fn i_02944a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2944E;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02944E: jsr $440d0.l
pub fn i_02944e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29454);
    cpu.pc = 0x440D0;
    16
}
/// $029454: move.w d0, $22(a6)
pub fn i_029454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29458;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $029458: move.w d1, $24(a6)
pub fn i_029458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2945C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02945C: move.w $c(a1), $38(a6)
pub fn i_02945c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29462;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $029462: move.w $a(a1), $70(a6)
pub fn i_029462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29468;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $029468: move.b $1(a1), d0
pub fn i_029468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2946C;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02946C: or.b d0, $12(a6)
pub fn i_02946c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29470;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $029470: tst.b (a1)
pub fn i_029470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29472;
    cpu.ir = 0x4A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $029472: bne.w $294ae
pub fn i_029472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29476;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x294AE; return 10; }
    8
}
/// $029476: move.w $e(a1), d1
pub fn i_029476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2947A;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0xE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02947A: cmpi.w #$ffff, d1
pub fn i_02947a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2947E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02947E: bne.w $2948e
pub fn i_02947e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29482;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2948E; return 10; }
    8
}
/// $029482: nop
pub fn i_029482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29484;
    cpu.ir = 0x4E71;
    4
}
/// $029484: nop
pub fn i_029484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29486;
    cpu.ir = 0x4E71;
    4
}
/// $029486: cmpi.w #$ffff, d1
pub fn i_029486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2948A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02948A: nop
pub fn i_02948a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2948C;
    cpu.ir = 0x4E71;
    4
}
/// $02948C: trap #$f
pub fn i_02948c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2948E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02948E: movem.l a1, -(a7)
pub fn i_02948e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29492;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $029492: jsr $236e.l
pub fn i_029492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29498);
    cpu.pc = 0x236E;
    16
}
/// $029498: movem.l (a7)+, a1
pub fn i_029498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2949C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $02949C: movea.l $10(a1), a0
pub fn i_02949c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294A0;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0294A0: cmpa.l #$ffffffff, a0
pub fn i_0294a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294A6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0294A6: beq.w $294ae
pub fn i_0294a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x294AE; return 10; }
    8
}
// ===== JsrPcThunk_0294aa  ($0294AA, 6 B, thunks/jsr_pc_thunks.c)
/// $0294AA: jsr $28cd4(pc)
pub fn i_0294aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x294AE);
    cpu.pc = 0x28CD4;
    16
}
/// $0294AE: rts
pub fn i_0294ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptOp1E_SpawnChild_Sprite_0294b0  ($0294B0, 170 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0294B0: move.l a1, $40(a6)
pub fn i_0294b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294B4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0294B4: move.w $22(a6), d0
pub fn i_0294b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294B8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0294B8: move.w $24(a6), d1
pub fn i_0294b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294BC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0294BC: bra.w $294c8
pub fn i_0294bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x294C8;
    10
}
/// $0294C8: swap d0
pub fn i_0294c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294CA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0294CA: subi.w #$1, d1
pub fn i_0294ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294CE;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0294CE: move.w d1, d0
pub fn i_0294ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294D0;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0294D0: move.w $38(a6), d1
pub fn i_0294d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0294D4: move.w $14(a6), d2
pub fn i_0294d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294D8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0294D8: move.b $3a(a6), d5
pub fn i_0294d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294DC;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0294DC: move.b $12(a6), d4
pub fn i_0294dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294E0;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0294E0: andi.b #$4, d4
pub fn i_0294e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294E4;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0294E4: or.b d4, d5
pub fn i_0294e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294E6;
    cpu.ir = 0x8A04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0294E6: move.b $33(a6), d3
pub fn i_0294e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294EA;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0294EA: move.b $32(a6), d4
pub fn i_0294ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294EE;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0294EE: movea.l $2(a1), a0
pub fn i_0294ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294F2;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0294F2: btst.b #$6, $12(a6)
pub fn i_0294f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294F8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0294F8: bne.w $29506
pub fn i_0294f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29506; return 10; }
    8
}
/// $0294FC: jsr $5a9e2.l
pub fn i_0294fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29502);
    cpu.pc = 0x5A9E2;
    16
}
/// $029502: bra.w $2950c
pub fn i_029502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29506;
    cpu.ir = 0x6000;
    cpu.pc = 0x2950C;
    10
}
/// $029506: jsr $5a9d6.l
pub fn i_029506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2950C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2950C);
    cpu.pc = 0x5A9D6;
    16
}
/// $02950C: move.b $1(a1), d0
pub fn i_02950c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29510;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $029510: adda.l #$8, a1
pub fn i_029510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29516;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x8;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029516: btst.b #$3, d0
pub fn i_029516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2951A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $02951A: bne.w $29524
pub fn i_02951a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2951E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29524; return 10; }
    8
}
/// $02951E: adda.l #$4, a1
pub fn i_02951e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29524;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029524: cmpi.b #$18, (a1)
pub fn i_029524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29528;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x18, Size::Byte, false);
    12
}
/// $029528: bne.w $29530
pub fn i_029528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2952C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29530; return 10; }
    8
}
/// $02952C: jmp $2934c(pc)
pub fn i_02952c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29530;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2934C;
    8
}
/// $029530: cmpi.b #$2, (a1)
pub fn i_029530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29534;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $029534: bne.w $2953c
pub fn i_029534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29538;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2953C; return 10; }
    8
}
/// $029538: jmp $294b4(pc)
pub fn i_029538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2953C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x294B4;
    8
}
/// $02953C: tst.b $46(a6)
pub fn i_02953c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29540;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $029540: beq.w $29560
pub fn i_029540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29544;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29560; return 10; }
    8
}
/// $029544: subi.b #$1, $46(a6)
pub fn i_029544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2954A;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $02954A: bne.w $2955a
pub fn i_02954a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2954E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2955A; return 10; }
    8
}
/// $02954E: cmpi.b #$16, (a1)
pub fn i_02954e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29552;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x16, Size::Byte, false);
    12
}
/// $029552: beq.w $29560
pub fn i_029552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29556;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29560; return 10; }
    8
}
/// $029556: move.l a1, $3c(a6)
pub fn i_029556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2955A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== ClearXN_02955a  ($02955A, 6 B, thunks/ccr_helpers.c)
/// $02955A: andi.b #$ee, ccr
pub fn i_02955a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2955E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02955E: rts
pub fn i_02955e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29560;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_029560  ($029560, 6 B, thunks/ccr_helpers.c)
/// $029560: ori.b #$11, ccr
pub fn i_029560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29564;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $029564: rts
pub fn i_029564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29566;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_029582  ($029582, 6 B, thunks/jsr_pc_thunks.c)
/// $029582: jsr $2a46c(pc)
pub fn i_029582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29586;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x29586);
    cpu.pc = 0x2A46C;
    16
}
/// $029586: rts
pub fn i_029586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29588;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetGlobalFlagFF_029588  ($029588, 16 B, kernel/scheduler_flag_setters.c)
/// $029588: move.b #$ff, $106f4a.l
pub fn i_029588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29590;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F4A, v1 as u8);
    16
}
/// $029590: jmp $518.l
pub fn i_029590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29596;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Slug_AngleToSpriteIdx_0295a6  ($0295A6, 14 B, slug/helpers_0295a6.s)
/// $0295A6: lea.l $2b0bee.l, a0
pub fn i_0295a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B0BEE;
    20
}
/// $0295AC: move.b (a0, d0.w), d0
pub fn i_0295ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295B0;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0295B0: ext.w d0
pub fn i_0295b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295B2;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0295B2: rts
pub fn i_0295b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Init_02a0f8  ($02A0F8, 178 B, slug/helpers_0295a6.s)
/// $02A0F8: jsr $236e.l
pub fn i_02a0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A0FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A0FE);
    cpu.pc = 0x236E;
    16
}
/// $02A0FE: move.w #$1, d1
pub fn i_02a0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A102;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A102: jsr $236e.l
pub fn i_02a102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A108);
    cpu.pc = 0x236E;
    16
}
/// $02A108: move.w #$1e, $1c(a6)
pub fn i_02a108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A10E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A10E: jsr $138fe.l
pub fn i_02a10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A114;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A114);
    cpu.pc = 0x138FE;
    16
}
/// $02A114: move.w $16(a6), $14(a6)
pub fn i_02a114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A11A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02A11A: bset.b #$4, $12(a6)
pub fn i_02a11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A120;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A120: bset.b #$2, $5b(a6)
pub fn i_02a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A126;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A126: bset.b #$0, $6b(a6)
pub fn i_02a126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A12C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A12C: move.b #$ff, $32(a6)
pub fn i_02a12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A132;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A132: move.b #$ff, $33(a6)
pub fn i_02a132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A138;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A138: move.b #$0, $3a(a6)
pub fn i_02a138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A13E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A13E: move.w #$8000, $38(a6)
pub fn i_02a13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A144;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A144: ori.w #$c, $38(a6)
pub fn i_02a144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A14A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A14A: jsr $5e98a.l
pub fn i_02a14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A150);
    cpu.pc = 0x5E98A;
    16
}
/// $02A150: clr.b $8c(a6)
pub fn i_02a150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A154;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A154: move.l #$295b4, $60(a6)
pub fn i_02a154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A15C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02A15C: jsr $8f6d2.l
pub fn i_02a15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A162);
    cpu.pc = 0x8F6D2;
    16
}
/// $02A162: move.w #$30, $66(a6)
pub fn i_02a162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A168;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A168: jsr $2fada(pc)
pub fn i_02a168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A16C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A16C);
    cpu.pc = 0x2FADA;
    16
}
/// $02A16C: clr.w $94(a6)
pub fn i_02a16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A170;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02A170: clr.w $36(a6)
pub fn i_02a170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A174;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02A174: clr.b $92(a6)
pub fn i_02a174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A178;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A178: clr.b $8f(a6)
pub fn i_02a178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A17C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A17C: move.b #$a, $90(a6)
pub fn i_02a17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A182;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A182: bclr.b #$3, $13(a6)
pub fn i_02a182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A188;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A188: lea.l $7773e.l, a1
pub fn i_02a188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A18E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7773E;
    20
}
/// $02A18E: jsr $4ae.l
pub fn i_02a18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A194);
    cpu.pc = 0x4AE;
    16
}
/// $02A194: jsr $5dd02.l
pub fn i_02a194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A19A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A19A);
    cpu.pc = 0x5DD02;
    16
}
/// $02A19A: jsr $3060a.l
pub fn i_02a19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A1A0);
    cpu.pc = 0x3060A;
    16
}
/// $02A1A0: move.l #$10000, $98(a6)
pub fn i_02a1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1A8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x10000;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02A1A8: rts
pub fn i_02a1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrRamWord_02a24a  ($02A24A, 8 B, thunks/clr_ram_word.c)
/// $02A24A: clr.b $106f4b.l
pub fn i_02a24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A250;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106F4B) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106F4B, r2 as u8);
    16
}
/// $02A250: rts
pub fn i_02a250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A252;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IsIdleFlagClear_02a25c  ($02A25C, 20 B, slug/helpers_0295a6.s)
/// $02A25C: cmpi.b #$0, $106ed3.l
pub fn i_02a25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A264;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED3) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02A264: bne.w $2a270
pub fn i_02a264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A268;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A270; return 10; }
    8
}
/// $02A268: ori.b #$11, ccr
pub fn i_02a268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A26C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A26C: bra.w $2a274
pub fn i_02a26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A270;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A274;
    10
}
// ===== ClearXN_02a270  ($02A270, 6 B, thunks/ccr_helpers.c)
/// $02A270: andi.b #$ee, ccr
pub fn i_02a270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A274;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A274: rts
pub fn i_02a274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A276;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestBit5Field8D_02a276  ($02A276, 18 B, slug/helpers_0295a6.s)
/// $02A276: btst.b #$5, $8d(a6)
pub fn i_02a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A27C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A27C: beq.w $2a288
pub fn i_02a27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A280;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A288; return 10; }
    8
}
/// $02A280: andi.b #$ee, ccr
pub fn i_02a280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A284;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A284: bra.w $2a28c
pub fn i_02a284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A288;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A28C;
    10
}
// ===== SetXN_02a288  ($02A288, 6 B, thunks/ccr_helpers.c)
/// $02A288: ori.b #$11, ccr
pub fn i_02a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A28C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A28C: rts
pub fn i_02a28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A28E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_GetGauge_02a2ac  ($02A2AC, 14 B, slug/helpers_0295a6.s)
/// $02A2AC: lea.l $100580.l, a0
pub fn i_02a2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02A2B2: moveq #$0, d0
pub fn i_02a2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2B4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02A2B4: move.b $90(a0), d0
pub fn i_02a2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2B8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02A2B8: rts
pub fn i_02a2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_StopMusic1092_02a310  ($02A310, 24 B, slug/helpers_0295a6.s)
/// $02A310: tst.b $8b(a6)
pub fn i_02a310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A314;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02A314: beq.w $2a326
pub fn i_02a314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A318;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A326; return 10; }
    8
}
/// $02A318: move.w #$1092, d0
pub fn i_02a318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A31C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1092;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A31C: jsr $2352.l
pub fn i_02a31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A322);
    cpu.pc = 0x2352;
    16
}
/// $02A322: clr.b $8b(a6)
pub fn i_02a322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A326;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A326: rts
pub fn i_02a326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A328;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CallGroundProbeA_02a328  ($02A328, 18 B, slug/helpers_0295a6.s)
/// $02A328: move.w $94(a6), d0
pub fn i_02a328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A32C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A32C: asl.w #$2, d0
pub fn i_02a32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A32E;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02A32E: lea.l $2a33a(pc), a0
pub fn i_02a32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A332;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A33A;
    16
}
/// $02A332: movea.l (a0, d0.w), a0
pub fn i_02a332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A336;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02A336: jsr (a0)
pub fn i_02a336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A338;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x2A338);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $02A338: rts
pub fn i_02a338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A33A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CallGroundProbeB_02a34e  ($02A34E, 18 B, slug/helpers_0295a6.s)
/// $02A34E: move.w $94(a6), d0
pub fn i_02a34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A352;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A352: asl.w #$2, d0
pub fn i_02a352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A354;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02A354: lea.l $2a360(pc), a0
pub fn i_02a354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A358;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A360;
    16
}
/// $02A358: movea.l (a0, d0.w), a0
pub fn i_02a358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A35C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02A35C: jsr (a0)
pub fn i_02a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A35E;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x2A35E);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $02A35E: rts
pub fn i_02a35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A360;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ClearFlags8D_02a46c  ($02A46C, 12 B, slug/helpers_0295a6.s)
/// $02A46C: lea.l $100580.l, a0
pub fn i_02a46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A472;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02A472: clr.b $8d(a0)
pub fn i_02a472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A476;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A476: rts
pub fn i_02a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A478;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateAirFlag_02a478  ($02A478, 42 B, slug/helpers_0295a6.s)
/// $02A478: jsr $5cf1c.l
pub fn i_02a478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A47E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A47E);
    cpu.pc = 0x5CF1C;
    16
}
/// $02A47E: bcc.w $2a48c
pub fn i_02a47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A482;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A48C; return 10; }
    8
}
/// $02A482: bset.b #$4, $12(a6)
pub fn i_02a482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A488;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A488: bra.w $2a4a0
pub fn i_02a488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A48C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A4A0;
    10
}
/// $02A48C: bclr.b #$4, $12(a6)
pub fn i_02a48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A492;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A492: jsr $2abcc(pc)
pub fn i_02a492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A496;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A496);
    cpu.pc = 0x2ABCC;
    16
}
/// $02A496: bcc.w $2a4a0
pub fn i_02a496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A49A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A4A0; return 10; }
    8
}
/// $02A49A: bset.b #$4, $12(a6)
pub fn i_02a49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A4A0: rts
pub fn i_02a4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ResetTurnTimer_02a4a2  ($02A4A2, 6 B, slug/helpers_0295a6.s)
/// $02A4A2: clr.b $84(a6)
pub fn i_02a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A4A6: rts
pub fn i_02a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateAngleIsSlope_02a4f0  ($02A4F0, 12 B, slug/helpers_0295a6.s)
/// $02A4F0: jsr $2a958(pc)
pub fn i_02a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A4F4);
    cpu.pc = 0x2A958;
    16
}
/// $02A4F4: cmp.w $80(a6), d2
pub fn i_02a4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4F8;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02A4F8: bne.w $2a502
pub fn i_02a4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A502; return 10; }
    8
}
// ===== ClearXN_02a4fc  ($02A4FC, 6 B, thunks/ccr_helpers.c)
/// $02A4FC: andi.b #$ee, ccr
pub fn i_02a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A500;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A500: rts
pub fn i_02a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A502;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SetAngleIsSlope_02a502  ($02A502, 22 B, slug/helpers_0295a6.s)
/// $02A502: move.w d2, $80(a6)
pub fn i_02a502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A506;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A506: move.w $80(a6), d7
pub fn i_02a506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A50A;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A50A: lea.l $2a558(pc), a0
pub fn i_02a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A50E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A558;
    16
}
/// $02A50E: move.b (a0, d7.w), d0
pub fn i_02a50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A512;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02A512: cmpi.b #$1, d0
pub fn i_02a512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A516;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $02A516: rts
pub fn i_02a516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A518;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CheckFreeThenC_02a59a  ($02A59A, 8 B, slug/helpers_0295a6.s)
/// $02A59A: jsr $2abc6(pc)
pub fn i_02a59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A59E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A59E);
    cpu.pc = 0x2ABC6;
    16
}
/// $02A59E: bcc.w $2a5a8
pub fn i_02a59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A5A8; return 10; }
    8
}
// ===== SetXN_02a5a2  ($02A5A2, 6 B, thunks/ccr_helpers.c)
/// $02A5A2: ori.b #$11, ccr
pub fn i_02a5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5A6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A5A6: rts
pub fn i_02a5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a5a8  ($02A5A8, 6 B, thunks/ccr_helpers.c)
/// $02A5A8: andi.b #$ee, ccr
pub fn i_02a5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5AC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A5AC: rts
pub fn i_02a5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_02a5cc  ($02A5CC, 8 B, thunks/jsr_abs_thunks.c)
/// $02A5CC: jsr $5d00e.l
pub fn i_02a5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A5D2);
    cpu.pc = 0x5D00E;
    16
}
/// $02A5D2: rts
pub fn i_02a5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_MusicByTerrainA_02a5d4  ($02A5D4, 30 B, slug/helpers_0295a6.s)
/// $02A5D4: jsr $27eba.l
pub fn i_02a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A5DA);
    cpu.pc = 0x27EBA;
    16
}
/// $02A5DA: andi.b #$c0, d0
pub fn i_02a5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5DE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02A5DE: cmpi.b #$40, d0
pub fn i_02a5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5E2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $02A5E2: beq.w $2a5fa
pub fn i_02a5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A5FA; return 10; }
    8
}
/// $02A5E6: cmpi.b #$80, d0
pub fn i_02a5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5EA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $02A5EA: beq.w $2a5fa
pub fn i_02a5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A5FA; return 10; }
    8
}
/// $02A5EE: move.w #$1090, d0
pub fn i_02a5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5F2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1090;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_02a5f2  ($02A5F2, 8 B, thunks/jsr_abs_thunks.c)
/// $02A5F2: jsr $2352.l
pub fn i_02a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A5F8);
    cpu.pc = 0x2352;
    16
}
/// $02A5F8: rts
pub fn i_02a5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Music1052_02a5fa  ($02A5FA, 4 B, slug/helpers_0295a6.s)
/// $02A5FA: move.w #$1052, d0
pub fn i_02a5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1052;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_02a5fe  ($02A5FE, 8 B, thunks/jsr_abs_thunks.c)
/// $02A5FE: jsr $2352.l
pub fn i_02a5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A604);
    cpu.pc = 0x2352;
    16
}
/// $02A604: rts
pub fn i_02a604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A606;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TryStartDestroyed_02a664  ($02A664, 32 B, slug/helpers_0295a6.s)
/// $02A664: btst.b #$5, $8d(a6)
pub fn i_02a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A66A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A66A: bne.w $2a68a
pub fn i_02a66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A66E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A68A; return 10; }
    8
}
/// $02A66E: tst.w $106e92.l
pub fn i_02a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A674;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106E92) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $02A674: bne.w $2a68a
pub fn i_02a674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A678;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A68A; return 10; }
    8
}
/// $02A678: bset.b #$0, $13(a6)
pub fn i_02a678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A67E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A67E: lea.l $2dcc0(pc), a1
pub fn i_02a67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A682;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DCC0;
    16
}
/// $02A682: move.l a1, (a6)
pub fn i_02a682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A684;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== ClearXN_02a684  ($02A684, 6 B, thunks/ccr_helpers.c)
/// $02A684: andi.b #$ee, ccr
pub fn i_02a684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A688;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A688: rts
pub fn i_02a688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A68A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_02a68a  ($02A68A, 6 B, thunks/jmp_abs_thunks.c)
/// $02A68A: jmp $28758.l
pub fn i_02a68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A690;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x28758;
    8
}
// ===== Slug_TryStartDestroyedB_02a690  ($02A690, 40 B, slug/helpers_0295a6.s)
/// $02A690: btst.b #$5, $8d(a6)
pub fn i_02a690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A696;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A696: bne.w $2a6be
pub fn i_02a696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A69A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A6BE; return 10; }
    8
}
/// $02A69A: tst.w $106e92.l
pub fn i_02a69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6A0;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106E92) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $02A6A0: bne.w $2a6be
pub fn i_02a6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A6BE; return 10; }
    8
}
/// $02A6A4: bset.b #$3, $13(a6)
pub fn i_02a6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6AA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A6AA: bset.b #$0, $13(a6)
pub fn i_02a6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6B0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A6B0: lea.l $2dcc0(pc), a1
pub fn i_02a6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DCC0;
    16
}
/// $02A6B4: move.l a1, (a6)
pub fn i_02a6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02A6B6: clr.w d0
pub fn i_02a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6B8;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== ClearXN_02a6b8  ($02A6B8, 6 B, thunks/ccr_helpers.c)
/// $02A6B8: andi.b #$ee, ccr
pub fn i_02a6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6BC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A6BC: rts
pub fn i_02a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DamageTick_02a6be  ($02A6BE, 72 B, slug/helpers_0295a6.s)
/// $02A6BE: tst.b $45(a6)
pub fn i_02a6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6C2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02A6C2: beq.w $2a6cc
pub fn i_02a6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A6CC; return 10; }
    8
}
/// $02A6C6: jsr $8f6da.l
pub fn i_02a6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A6CC);
    cpu.pc = 0x8F6DA;
    16
}
/// $02A6CC: btst.b #$3, $100001.l
pub fn i_02a6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6D4;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((3) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $02A6D4: bne.w $2a712
pub fn i_02a6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A712; return 10; }
    8
}
/// $02A6D8: btst.b #$5, $100001.l
pub fn i_02a6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6E0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((5) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $02A6E0: beq.w $2a6ea
pub fn i_02a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A6EA; return 10; }
    8
}
/// $02A6E4: move.w #$300, $66(a6)
pub fn i_02a6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A6EA: jsr $2870a.l
pub fn i_02a6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A6F0);
    cpu.pc = 0x2870A;
    16
}
/// $02A6F0: bcc.w $2a70c
pub fn i_02a6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A70C; return 10; }
    8
}
/// $02A6F4: movem.w d0, -(a7)
pub fn i_02a6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6F8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02A6F8: move.w #$1089, d0
pub fn i_02a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6FC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1089;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A6FC: jsr $2352.l
pub fn i_02a6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A702);
    cpu.pc = 0x2352;
    16
}
/// $02A702: movem.w (a7)+, d0
pub fn i_02a702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A706;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== SetXN_02a706  ($02A706, 6 B, thunks/ccr_helpers.c)
/// $02A706: ori.b #$11, ccr
pub fn i_02a706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A70A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A70A: rts
pub fn i_02a70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A70C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a70c  ($02A70C, 6 B, thunks/ccr_helpers.c)
/// $02A70C: andi.b #$ee, ccr
pub fn i_02a70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A710;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A710: rts
pub fn i_02a710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A712;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ResetHP_02a712  ($02A712, 8 B, slug/helpers_0295a6.s)
/// $02A712: move.w #$30, $66(a6)
pub fn i_02a712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A718;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A718: clr.w d0
pub fn i_02a718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A71A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== ClearXN_02a71a  ($02A71A, 6 B, thunks/ccr_helpers.c)
/// $02A71A: andi.b #$ee, ccr
pub fn i_02a71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A71E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A71E: rts
pub fn i_02a71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A720;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateInputFlags_02a720  ($02A720, 50 B, slug/helpers_0295a6.s)
/// $02A720: bclr.b #$4, $13(a6)
pub fn i_02a720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A726;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A726: bclr.b #$5, $13(a6)
pub fn i_02a726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A72C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A72C: jsr $5ceec.l
pub fn i_02a72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A732;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A732);
    cpu.pc = 0x5CEEC;
    16
}
/// $02A732: bcc.w $2a740
pub fn i_02a732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A736;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A740; return 10; }
    8
}
/// $02A736: bset.b #$4, $13(a6)
pub fn i_02a736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A73C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A73C: bra.w $2a740
pub fn i_02a73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A740;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A740;
    10
}
/// $02A740: jsr $5cef8.l
pub fn i_02a740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A746);
    cpu.pc = 0x5CEF8;
    16
}
/// $02A746: bcc.w $2a750
pub fn i_02a746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A74A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A750; return 10; }
    8
}
/// $02A74A: bset.b #$5, $13(a6)
pub fn i_02a74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A750;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A750: rts
pub fn i_02a750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A752;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsE_02a7d8  ($02A7D8, 64 B, slug/helpers_0295a6.s)
/// $02A7D8: jsr $28992.l
pub fn i_02a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7DE);
    cpu.pc = 0x28992;
    16
}
/// $02A7DE: jsr $30704.l
pub fn i_02a7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7E4);
    cpu.pc = 0x30704;
    16
}
/// $02A7E4: beq.w $2a7ee
pub fn i_02a7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A7EE; return 10; }
    8
}
/// $02A7E8: jsr $281b0.l
pub fn i_02a7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7EE);
    cpu.pc = 0x281B0;
    16
}
/// $02A7EE: jsr $304c4.l
pub fn i_02a7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7F4);
    cpu.pc = 0x304C4;
    16
}
/// $02A7F4: jsr $2788c.l
pub fn i_02a7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7FA);
    cpu.pc = 0x2788C;
    16
}
/// $02A7FA: jsr $2a720(pc)
pub fn i_02a7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A7FE);
    cpu.pc = 0x2A720;
    16
}
/// $02A7FE: btst.b #$5, $5a(a6)
pub fn i_02a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A804;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A804: beq.w $2a80e
pub fn i_02a804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A808;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A80E; return 10; }
    8
}
/// $02A808: addi.w #$10, $82(a6)
pub fn i_02a808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A80E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A80E: jsr $3076a.l
pub fn i_02a80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A814);
    cpu.pc = 0x3076A;
    16
}
/// $02A814: beq.w $2a81e
pub fn i_02a814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A818;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A81E; return 10; }
    8
}
// ===== SetXN_02a818  ($02A818, 6 B, thunks/ccr_helpers.c)
/// $02A818: ori.b #$11, ccr
pub fn i_02a818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A81C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A81C: rts
pub fn i_02a81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A81E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a81e  ($02A81E, 6 B, thunks/ccr_helpers.c)
/// $02A81E: andi.b #$ee, ccr
pub fn i_02a81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A822;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A822: rts
pub fn i_02a822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A824;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsF_02a824  ($02A824, 42 B, slug/helpers_0295a6.s)
/// $02A824: jsr $304c4.l
pub fn i_02a824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A82A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A82A);
    cpu.pc = 0x304C4;
    16
}
/// $02A82A: jsr $30704.l
pub fn i_02a82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A830);
    cpu.pc = 0x30704;
    16
}
/// $02A830: beq.w $2a83a
pub fn i_02a830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A834;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A83A; return 10; }
    8
}
/// $02A834: jsr $281b0.l
pub fn i_02a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A83A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A83A);
    cpu.pc = 0x281B0;
    16
}
/// $02A83A: jsr $2788c.l
pub fn i_02a83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A840);
    cpu.pc = 0x2788C;
    16
}
/// $02A840: jsr $2a720(pc)
pub fn i_02a840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A844;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A844);
    cpu.pc = 0x2A720;
    16
}
/// $02A844: jsr $3076a.l
pub fn i_02a844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A84A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A84A);
    cpu.pc = 0x3076A;
    16
}
/// $02A84A: beq.w $2a854
pub fn i_02a84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A84E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A854; return 10; }
    8
}
// ===== SetXN_02a84e  ($02A84E, 6 B, thunks/ccr_helpers.c)
/// $02A84E: ori.b #$11, ccr
pub fn i_02a84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A852;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A852: rts
pub fn i_02a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A854;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a854  ($02A854, 6 B, thunks/ccr_helpers.c)
/// $02A854: andi.b #$ee, ccr
pub fn i_02a854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A858;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A858: rts
pub fn i_02a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A85A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsG_02a85a  ($02A85A, 22 B, slug/helpers_0295a6.s)
/// $02A85A: jsr $28992.l
pub fn i_02a85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A860);
    cpu.pc = 0x28992;
    16
}
/// $02A860: jsr $30704.l
pub fn i_02a860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A866);
    cpu.pc = 0x30704;
    16
}
/// $02A866: beq.w $2a870
pub fn i_02a866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A86A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A870; return 10; }
    8
}
/// $02A86A: jsr $281b0.l
pub fn i_02a86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A870);
    cpu.pc = 0x281B0;
    16
}
// ===== JsrAbsThunk_02a870  ($02A870, 8 B, thunks/jsr_abs_thunks.c)
/// $02A870: jsr $2788c.l
pub fn i_02a870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A876);
    cpu.pc = 0x2788C;
    16
}
/// $02A876: rts
pub fn i_02a876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A878;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsAir_02a878  ($02A878, 54 B, slug/helpers_0295a6.s)
/// $02A878: jsr $28992.l
pub fn i_02a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A87E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A87E);
    cpu.pc = 0x28992;
    16
}
/// $02A87E: jsr $30704.l
pub fn i_02a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A884);
    cpu.pc = 0x30704;
    16
}
/// $02A884: beq.w $2a88e
pub fn i_02a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A888;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A88E; return 10; }
    8
}
/// $02A888: jsr $281b0.l
pub fn i_02a888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A88E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A88E);
    cpu.pc = 0x281B0;
    16
}
/// $02A88E: jsr $27a18.l
pub fn i_02a88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A894);
    cpu.pc = 0x27A18;
    16
}
/// $02A894: bcs.w $2a8b4
pub fn i_02a894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A898;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A8B4; return 10; }
    8
}
/// $02A898: tst.w $2a(a6)
pub fn i_02a898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A89C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02A89C: bge.w $2a8a8
pub fn i_02a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8A0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A8A8; return 10; }
    8
}
/// $02A8A0: jsr $2a8c0(pc)
pub fn i_02a8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A8A4);
    cpu.pc = 0x2A8C0;
    16
}
/// $02A8A4: bcc.w $2a8b4
pub fn i_02a8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A8B4; return 10; }
    8
}
/// $02A8A8: bset.b #$1, $8d(a6)
pub fn i_02a8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8AE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearXN_02a8ae  ($02A8AE, 6 B, thunks/ccr_helpers.c)
/// $02A8AE: andi.b #$ee, ccr
pub fn i_02a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8B2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A8B2: rts
pub fn i_02a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ClearBit1Field8D_02a8b4  ($02A8B4, 6 B, slug/helpers_0295a6.s)
/// $02A8B4: bclr.b #$1, $8d(a6)
pub fn i_02a8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetXN_02a8ba  ($02A8BA, 6 B, thunks/ccr_helpers.c)
/// $02A8BA: ori.b #$11, ccr
pub fn i_02a8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8BE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A8BE: rts
pub fn i_02a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_GroundContact_02a8c0  ($02A8C0, 110 B, slug/helpers_0295a6.s)
/// $02A8C0: btst.b #$6, $6b(a6)
pub fn i_02a8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8C6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A8C6: beq.w $2a8d0
pub fn i_02a8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A8D0; return 10; }
    8
}
/// $02A8CA: jmp $27eba.l
pub fn i_02a8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8D0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x27EBA;
    8
}
/// $02A8D0: bclr.b #$3, $5b(a6)
pub fn i_02a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8D6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A8D6: move.w $22(a6), d1
pub fn i_02a8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8DA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A8DA: move.w $82(a6), d2
pub fn i_02a8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8DE;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A8DE: jsr $27ec2.l
pub fn i_02a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A8E4);
    cpu.pc = 0x27EC2;
    16
}
/// $02A8E4: bcs.w $2a8fa
pub fn i_02a8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A8FA; return 10; }
    8
}
/// $02A8E8: movea.l $74(a6), a2
pub fn i_02a8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8EC;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02A8EC: movea.l $78(a6), a3
pub fn i_02a8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8F0;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $02A8F0: movea.l $7c(a6), a4
pub fn i_02a8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8F4;
    cpu.ir = 0x286E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $02A8F4: moveq #$3, d1
pub fn i_02a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8F6;
    cpu.ir = 0x7203;
    cpu.d[1] = 0x3;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02A8F6: bra.w $2a940
pub fn i_02a8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A940;
    10
}
/// $02A8FA: movea.l $74(a6), a2
pub fn i_02a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8FE;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02A8FE: movea.l $78(a6), a3
pub fn i_02a8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A902;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $02A902: movea.l $7c(a6), a4
pub fn i_02a902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A906;
    cpu.ir = 0x286E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $02A906: moveq #$0, d1
pub fn i_02a906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A908;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02A908: add.b $84(a4), d1
pub fn i_02a908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A90C;
    cpu.ir = 0xD22C;
    let ad2 = cpu.a[4].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    12
}
/// $02A90C: add.b $84(a2), d1
pub fn i_02a90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A910;
    cpu.ir = 0xD22A;
    let ad2 = cpu.a[2].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    12
}
/// $02A910: add.b $84(a3), d1
pub fn i_02a910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A914;
    cpu.ir = 0xD22B;
    let ad2 = cpu.a[3].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    12
}
/// $02A914: cmpi.b #$2, d1
pub fn i_02a914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A918;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $02A918: bge.w $2a934
pub fn i_02a918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A91C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A934; return 10; }
    8
}
/// $02A91C: andi.w #$1ff, $24(a2)
pub fn i_02a91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A922;
    cpu.ir = 0x26A;
    let ad1 = cpu.a[2].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A922: andi.w #$1ff, $24(a3)
pub fn i_02a922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A928;
    cpu.ir = 0x26B;
    let ad1 = cpu.a[3].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A928: andi.w #$1ff, $24(a4)
pub fn i_02a928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A92E;
    cpu.ir = 0x26C;
    let ad1 = cpu.a[4].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetXN_02a92e  ($02A92E, 6 B, thunks/ccr_helpers.c)
/// $02A92E: ori.b #$11, ccr
pub fn i_02a92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A932;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A932: rts
pub fn i_02a932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A934;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_GroundContactWrapY_02a934  ($02A934, 30 B, slug/helpers_0295a6.s)
/// $02A934: bset.b #$3, $5b(a6)
pub fn i_02a934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A93A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A93A: move.w $24(a6), $82(a6)
pub fn i_02a93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A940;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02A940: andi.w #$1ff, $24(a2)
pub fn i_02a940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A946;
    cpu.ir = 0x26A;
    let ad1 = cpu.a[2].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A946: andi.w #$1ff, $24(a3)
pub fn i_02a946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A94C;
    cpu.ir = 0x26B;
    let ad1 = cpu.a[3].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A94C: andi.w #$1ff, $24(a4)
pub fn i_02a94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A952;
    cpu.ir = 0x26C;
    let ad1 = cpu.a[4].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== ClearXN_02a952  ($02A952, 6 B, thunks/ccr_helpers.c)
/// $02A952: andi.b #$ee, ccr
pub fn i_02a952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A956;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A956: rts
pub fn i_02a956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A958;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TerrainSlope_02a958  ($02A958, 72 B, slug/helpers_0295a6.s)
/// $02A958: movea.l $74(a6), a1
pub fn i_02a958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A95C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $02A95C: move.w $22(a1), d1
pub fn i_02a95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A960;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A960: move.w $9a(a1), d2
pub fn i_02a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A964;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A964: movea.l $78(a6), a2
pub fn i_02a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A968;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02A968: move.w $22(a2), d3
pub fn i_02a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A96C;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A96C: move.w $9a(a2), d4
pub fn i_02a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A970;
    cpu.ir = 0x382A;
    let ad2 = cpu.a[2].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A970: sub.w d3, d1
pub fn i_02a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A972;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02A972: move.w d1, d0
pub fn i_02a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A974;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A974: sub.w d4, d2
pub fn i_02a974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A976;
    cpu.ir = 0x9444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02A976: bne.w $2a980
pub fn i_02a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A97A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A980; return 10; }
    8
}
/// $02A97A: clr.w d1
pub fn i_02a97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A97C;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02A97C: bra.w $2a982
pub fn i_02a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A980;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A982;
    10
}
/// $02A980: muls.w d2, d1
pub fn i_02a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A982;
    cpu.ir = 0xC3C2;
    let v1 = cpu.d[2] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $02A982: cmpi.w #$ffc0, d2
pub fn i_02a982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A986;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFC0, Size::Word, false);
    8
}
/// $02A986: bge.w $2a992
pub fn i_02a986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A98A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A992; return 10; }
    8
}
/// $02A98A: move.w #$ffc0, d2
pub fn i_02a98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A98E;
    cpu.ir = 0x343C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A98E: bra.w $2a99e
pub fn i_02a98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A992;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A99E;
    10
}
/// $02A992: cmpi.w #$40, d2
pub fn i_02a992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A996;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02A996: ble.w $2a99e
pub fn i_02a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A99A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2A99E; return 10; }
    8
}
/// $02A99A: move.w #$40, d2
pub fn i_02a99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A99E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A99E: rts
pub fn i_02a99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SlopeToAnimIdx_02a9a0  ($02A9A0, 110 B, slug/helpers_0295a6.s)
/// $02A9A0: bra.w $2a9ce
pub fn i_02a9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A9CE;
    10
}
/// $02A9CE: jsr $2a958(pc)
pub fn i_02a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A9D2);
    cpu.pc = 0x2A958;
    16
}
/// $02A9D2: clr.w d0
pub fn i_02a9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9D4;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02A9D4: cmpi.w #$384, d1
pub fn i_02a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9D8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x384, Size::Word, false);
    8
}
/// $02A9D8: bge.w $2a9f6
pub fn i_02a9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9DC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A9F6; return 10; }
    8
}
/// $02A9DC: cmpi.w #$fc7c, d1
pub fn i_02a9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9E0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFC7C, Size::Word, false);
    8
}
/// $02A9E0: ble.w $2a9fc
pub fn i_02a9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9E4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2A9FC; return 10; }
    8
}
/// $02A9E4: cmpi.w #$5a, d1
pub fn i_02a9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9E8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5A, Size::Word, false);
    8
}
/// $02A9E8: bge.w $2aa02
pub fn i_02a9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9EC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2AA02; return 10; }
    8
}
/// $02A9EC: cmpi.w #$ffa6, d1
pub fn i_02a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9F0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFA6, Size::Word, false);
    8
}
/// $02A9F0: ble.w $2aa08
pub fn i_02a9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9F4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2AA08; return 10; }
    8
}
/// $02A9F4: rts
pub fn i_02a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02A9F6: move.w #$3, d0
pub fn i_02a9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9FA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A9FA: bra.b $2a9f4
pub fn i_02a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9FC;
    cpu.ir = 0x60F8;
    cpu.pc = 0x2A9F4;
    10
}
/// $02A9FC: move.w #$4, d0
pub fn i_02a9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA00;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AA00: bra.b $2a9f4
pub fn i_02aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA02;
    cpu.ir = 0x60F2;
    cpu.pc = 0x2A9F4;
    10
}
/// $02AA02: move.w #$1, d0
pub fn i_02aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA06;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AA06: bra.b $2a9f4
pub fn i_02aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA08;
    cpu.ir = 0x60EC;
    cpu.pc = 0x2A9F4;
    10
}
/// $02AA08: move.w #$2, d0
pub fn i_02aa08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA0C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AA0C: bra.b $2a9f4
pub fn i_02aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA0E;
    cpu.ir = 0x60E6;
    cpu.pc = 0x2A9F4;
    10
}
// ===== Slug_UpdateAnimKeepIdx_02aa0e  ($02AA0E, 16 B, slug/helpers_0295a6.s)
/// $02AA0E: move.w $94(a6), d0
pub fn i_02aa0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA12;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA12: movem.w d0, -(a7)
pub fn i_02aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA16;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02AA16: jsr $2aa24(pc)
pub fn i_02aa16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AA1A);
    cpu.pc = 0x2AA24;
    16
}
/// $02AA1A: movem.w (a7)+, d0
pub fn i_02aa1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA1E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== SetTaskW_02aa1e  ($02AA1E, 6 B, kernel/task_setters_w.c)
/// $02AA1E: move.w d0, $94(a6)
pub fn i_02aa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA22;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02AA22: rts
pub fn i_02aa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateAnimAndChassis_02aa24  ($02AA24, 140 B, slug/helpers_0295a6.s)
/// $02AA24: jsr $283ca.l
pub fn i_02aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AA2A);
    cpu.pc = 0x283CA;
    16
}
/// $02AA2A: cmpi.b #$0, $68(a6)
pub fn i_02aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA30;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02AA30: beq.b $2aa3c
pub fn i_02aa30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA32;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AA3C; return 10; }
    8
}
/// $02AA32: cmpi.b #$1, $68(a6)
pub fn i_02aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA38;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $02AA38: bne.w $2aa42
pub fn i_02aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AA42; return 10; }
    8
}
/// $02AA3C: move.b $68(a6), $85(a6)
pub fn i_02aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA42;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad4, v3 as u8);
    20
}
/// $02AA42: jsr $2a4a2(pc)
pub fn i_02aa42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AA46);
    cpu.pc = 0x2A4A2;
    16
}
/// $02AA46: btst.b #$0, $13(a6)
pub fn i_02aa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA4C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02AA4C: bne.w $2aa54
pub fn i_02aa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AA54; return 10; }
    8
}
/// $02AA50: jsr $2fae4(pc)
pub fn i_02aa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AA54);
    cpu.pc = 0x2FAE4;
    16
}
/// $02AA54: jsr $2a9a0(pc)
pub fn i_02aa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AA58);
    cpu.pc = 0x2A9A0;
    16
}
/// $02AA58: move.w d0, $94(a6)
pub fn i_02aa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA5C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02AA5C: move.b $8e(a6), d0
pub fn i_02aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA60;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AA60: clr.b d1
pub fn i_02aa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA62;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02AA62: subi.b #$1, d0
pub fn i_02aa62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA66;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02AA66: addx.b d1, d0
pub fn i_02aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA68;
    cpu.ir = 0xD101;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let h2 = cpu.d[1] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $02AA68: move.b d0, $8e(a6)
pub fn i_02aa68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA6C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02AA6C: movea.l $74(a6), a2
pub fn i_02aa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA70;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02AA70: movea.l $78(a6), a3
pub fn i_02aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA74;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $02AA74: move.w $9a(a2), d1
pub fn i_02aa74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA78;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA78: move.w $9a(a3), d0
pub fn i_02aa78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA7C;
    cpu.ir = 0x302B;
    let ad2 = cpu.a[3].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA7C: clr.w d3
pub fn i_02aa7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA7E;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02AA7E: sub.w d0, d1
pub fn i_02aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA80;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02AA80: bge.w $2aa8a
pub fn i_02aa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA84;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2AA8A; return 10; }
    8
}
/// $02AA84: move.w $9a(a2), d0
pub fn i_02aa84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA88;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA88: moveq #$ff, d3
pub fn i_02aa88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA8A;
    cpu.ir = 0x76FF;
    cpu.d[3] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AA8A: asl.w #$5, d1
pub fn i_02aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA8C;
    cpu.ir = 0xEB41;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $02AA8C: move.w $22(a2), d2
pub fn i_02aa8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA90;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA90: sub.w $22(a3), d2
pub fn i_02aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA94;
    cpu.ir = 0x946B;
    let ad2 = cpu.a[3].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02AA94: beq.w $2aaa0
pub fn i_02aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AAA0; return 10; }
    8
}
/// $02AA98: ext.l d1
pub fn i_02aa98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA9A;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AA9A: divs.w d2, d1
pub fn i_02aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA9C;
    cpu.ir = 0x83C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[1] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 158; }
    cpu.d[1] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    158
}
/// $02AA9C: bra.w $2aaa2
pub fn i_02aa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AAA2;
    10
}
/// $02AAA0: clr.w d1
pub fn i_02aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA2;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02AAA2: asr.w #$1, d1
pub fn i_02aaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA4;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02AAA4: eor.w d3, d1
pub fn i_02aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA6;
    cpu.ir = 0xB741;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[3]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02AAA6: sub.w d3, d1
pub fn i_02aaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA8;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02AAA8: add.w d0, d1
pub fn i_02aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAAA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02AAAA: move.w d1, $24(a6)
pub fn i_02aaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAAE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02AAAE: rts
pub fn i_02aaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CanFire_02aac0  ($02AAC0, 30 B, slug/helpers_0295a6.s)
/// $02AAC0: cmpi.b #$0, $90(a6)
pub fn i_02aac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAC6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02AAC6: beq.w $2aade
pub fn i_02aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AACA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AADE; return 10; }
    8
}
/// $02AACA: move.b #$2, d0
pub fn i_02aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AACE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02AACE: cmp.b $8f(a6), d0
pub fn i_02aace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAD2;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x8F);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02AAD2: bls.w $2aade
pub fn i_02aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAD6;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2AADE; return 10; }
    8
}
/// $02AAD6: tst.b $8e(a6)
pub fn i_02aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AADA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02AADA: beq.w $2aae4
pub fn i_02aada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AADE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AAE4; return 10; }
    8
}
// ===== ClearXN_02aade  ($02AADE, 6 B, thunks/ccr_helpers.c)
/// $02AADE: andi.b #$ee, ccr
pub fn i_02aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAE2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AAE2: rts
pub fn i_02aae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_02aae4  ($02AAE4, 6 B, thunks/jsr_pc_thunks.c)
/// $02AAE4: jsr $2ab86(pc)
pub fn i_02aae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AAE8);
    cpu.pc = 0x2AB86;
    16
}
/// $02AAE8: rts
pub fn i_02aae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_InputDirByLayoutA_02aaf0  ($02AAF0, 56 B, slug/helpers_0295a6.s)
/// $02AAF0: cmpi.b #$0, $106f2a.l
pub fn i_02aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAF8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02AAF8: bne.w $2ab06
pub fn i_02aaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAFC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AB06; return 10; }
    8
}
/// $02AAFC: jsr $5cdb4.l
pub fn i_02aafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB02);
    cpu.pc = 0x5CDB4;
    16
}
/// $02AB02: bra.w $2ab22
pub fn i_02ab02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB06;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AB22;
    10
}
/// $02AB06: cmpi.b #$4, $106f2a.l
pub fn i_02ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB0E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x4, Size::Byte, false);
    20
}
/// $02AB0E: bne.w $2ab1c
pub fn i_02ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB12;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AB1C; return 10; }
    8
}
/// $02AB12: jsr $5cdb4.l
pub fn i_02ab12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB18);
    cpu.pc = 0x5CDB4;
    16
}
/// $02AB18: bra.w $2ab22
pub fn i_02ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AB22;
    10
}
/// $02AB1C: jsr $5ceec.l
pub fn i_02ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB22);
    cpu.pc = 0x5CEEC;
    16
}
/// $02AB22: bcs.w $2ab2e
pub fn i_02ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB26;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AB2E; return 10; }
    8
}
/// $02AB26: moveq #$0, d0
pub fn i_02ab26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB28;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== ClearXN_02ab28  ($02AB28, 6 B, thunks/ccr_helpers.c)
/// $02AB28: andi.b #$ee, ccr
pub fn i_02ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB2C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AB2C: rts
pub fn i_02ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_InputDirA_02ab2e  ($02AB2E, 8 B, slug/helpers_0295a6.s)
/// $02AB2E: jsr $5d5b6.l
pub fn i_02ab2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB34);
    cpu.pc = 0x5D5B6;
    16
}
/// $02AB34: addq.w #$1, d0
pub fn i_02ab34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB36;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_02ab36  ($02AB36, 6 B, thunks/ccr_helpers.c)
/// $02AB36: ori.b #$11, ccr
pub fn i_02ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB3A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AB3A: rts
pub fn i_02ab3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_InputFireByLayout_02ab86  ($02AB86, 44 B, slug/helpers_0295a6.s)
/// $02AB86: cmpi.b #$4, $106f2a.l
pub fn i_02ab86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB8E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x4, Size::Byte, false);
    20
}
/// $02AB8E: bne.w $2ab9c
pub fn i_02ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AB9C; return 10; }
    8
}
/// $02AB92: jsr $5cda8.l
pub fn i_02ab92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB98);
    cpu.pc = 0x5CDA8;
    16
}
/// $02AB98: bra.w $2abb8
pub fn i_02ab98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ABB8;
    10
}
/// $02AB9C: cmpi.b #$0, $106f2a.l
pub fn i_02ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABA4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02ABA4: bne.w $2abb2
pub fn i_02aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABA8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ABB2; return 10; }
    8
}
/// $02ABA8: jsr $5cdc0.l
pub fn i_02aba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ABAE);
    cpu.pc = 0x5CDC0;
    16
}
/// $02ABAE: bra.w $2abb8
pub fn i_02abae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ABB8;
    10
}
// ===== JsrAbsThunk_02abb2  ($02ABB2, 8 B, thunks/jsr_abs_thunks.c)
/// $02ABB2: jsr $5cdfc.l
pub fn i_02abb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ABB8);
    cpu.pc = 0x5CDFC;
    16
}
/// $02ABB8: rts
pub fn i_02abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02abc0  ($02ABC0, 6 B, thunks/ccr_helpers.c)
/// $02ABC0: andi.b #$ee, ccr
pub fn i_02abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABC4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ABC4: rts
pub fn i_02abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_02abc6  ($02ABC6, 6 B, thunks/jmp_abs_thunks.c)
/// $02ABC6: jmp $5cef8.l
pub fn i_02abc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABCC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CEF8;
    8
}
// ===== ClearXN_02abcc  ($02ABCC, 6 B, thunks/ccr_helpers.c)
/// $02ABCC: andi.b #$ee, ccr
pub fn i_02abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABD0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ABD0: rts
pub fn i_02abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestBit4Field8D_02abd2  ($02ABD2, 24 B, slug/helpers_0295a6.s)
/// $02ABD2: lea.l $100580.l, a0
pub fn i_02abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABD8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02ABD8: btst.b #$4, $8d(a0)
pub fn i_02abd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABDE;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02ABDE: bne.w $2abea
pub fn i_02abde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABE2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ABEA; return 10; }
    8
}
/// $02ABE2: andi.b #$ee, ccr
pub fn i_02abe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABE6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ABE6: bra.w $2abee
pub fn i_02abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ABEE;
    10
}
// ===== SetXN_02abea  ($02ABEA, 6 B, thunks/ccr_helpers.c)
/// $02ABEA: ori.b #$11, ccr
pub fn i_02abea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABEE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02ABEE: rts
pub fn i_02abee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestBit1Field8D_02abf0  ($02ABF0, 24 B, slug/helpers_0295a6.s)
/// $02ABF0: lea.l $100580.l, a0
pub fn i_02abf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABF6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02ABF6: btst.b #$1, $8d(a0)
pub fn i_02abf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABFC;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02ABFC: bne.w $2ac08
pub fn i_02abfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC08; return 10; }
    8
}
/// $02AC00: andi.b #$ee, ccr
pub fn i_02ac00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC04;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AC04: bra.w $2ac0c
pub fn i_02ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC08;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AC0C;
    10
}
// ===== SetXN_02ac08  ($02AC08, 6 B, thunks/ccr_helpers.c)
/// $02AC08: ori.b #$11, ccr
pub fn i_02ac08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC0C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AC0C: rts
pub fn i_02ac0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IsRiddenByPlayer_02ac0e  ($02AC0E, 50 B, slug/helpers_0295a6.s)
/// $02AC0E: cmpa.l #$100440, a0
pub fn i_02ac0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC14;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02AC14: bne.w $2ac1e
pub fn i_02ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC18;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC1E; return 10; }
    8
}
/// $02AC18: moveq #$1, d0
pub fn i_02ac18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC1A;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AC1A: bra.w $2ac32
pub fn i_02ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AC32;
    10
}
/// $02AC1E: cmpa.l #$1004e0, a0
pub fn i_02ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC24;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02AC24: bne.w $2ac2e
pub fn i_02ac24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC2E; return 10; }
    8
}
/// $02AC28: moveq #$2, d0
pub fn i_02ac28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC2A;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AC2A: bra.w $2ac32
pub fn i_02ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AC32;
    10
}
/// $02AC2E: bra.w $2ac46
pub fn i_02ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC32;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AC46;
    10
}
/// $02AC32: lea.l $100580.l, a1
pub fn i_02ac32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC38;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $02AC38: cmp.b $6d(a1), d0
pub fn i_02ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC3C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02AC3C: bne.w $2ac46
pub fn i_02ac3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC46; return 10; }
    8
}
// ===== JsrPcThunk_02ac40  ($02AC40, 6 B, thunks/jsr_pc_thunks.c)
/// $02AC40: jsr $2ac4c(pc)
pub fn i_02ac40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AC44);
    cpu.pc = 0x2AC4C;
    16
}
/// $02AC44: rts
pub fn i_02ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02ac46  ($02AC46, 6 B, thunks/ccr_helpers.c)
/// $02AC46: andi.b #$ee, ccr
pub fn i_02ac46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC4A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AC4A: rts
pub fn i_02ac4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestField100609_02ac4c  ($02AC4C, 10 B, slug/helpers_0295a6.s)
/// $02AC4C: move.b $100609.l, d0
pub fn i_02ac4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC52;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x100609) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02AC52: bne.w $2ac5c
pub fn i_02ac52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC5C; return 10; }
    8
}
// ===== ClearXN_02ac56  ($02AC56, 6 B, thunks/ccr_helpers.c)
/// $02AC56: andi.b #$ee, ccr
pub fn i_02ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC5A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AC5A: rts
pub fn i_02ac5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CopyField68_02ac5c  ($02AC5C, 8 B, slug/helpers_0295a6.s)
/// $02AC5C: move.b $68(a0), d2
pub fn i_02ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC60;
    cpu.ir = 0x1428;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AC60: move.b d2, $68(a6)
pub fn i_02ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC64;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_02ac64  ($02AC64, 6 B, thunks/ccr_helpers.c)
/// $02AC64: ori.b #$11, ccr
pub fn i_02ac64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC68;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AC68: rts
pub fn i_02ac68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestField100609B_02ac6a  ($02AC6A, 10 B, slug/helpers_0295a6.s)
/// $02AC6A: move.b $100609.l, d0
pub fn i_02ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC70;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x100609) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02AC70: bne.w $2ac7a
pub fn i_02ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC7A; return 10; }
    8
}
// ===== ClearXN_02ac74  ($02AC74, 6 B, thunks/ccr_helpers.c)
/// $02AC74: andi.b #$ee, ccr
pub fn i_02ac74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC78;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AC78: rts
pub fn i_02ac78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_02ac7a  ($02AC7A, 6 B, thunks/ccr_helpers.c)
/// $02AC7A: ori.b #$11, ccr
pub fn i_02ac7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC7E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AC7E: rts
pub fn i_02ac7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ConsumeField89_02ac80  ($02AC80, 28 B, slug/helpers_0295a6.s)
/// $02AC80: cmpi.b #$0, $89(a6)
pub fn i_02ac80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC86;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02AC86: bne.w $2ac98
pub fn i_02ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC8A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC98; return 10; }
    8
}
/// $02AC8A: move.b #$ff, $6d(a6)
pub fn i_02ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02AC90: ori.b #$11, ccr
pub fn i_02ac90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC94;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AC94: bra.w $2aca0
pub fn i_02ac94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC98;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ACA0;
    10
}
/// $02AC98: clr.b $89(a6)
pub fn i_02ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC9C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x89);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== ClearXN_02ac9c  ($02AC9C, 6 B, thunks/ccr_helpers.c)
/// $02AC9C: andi.b #$ee, ccr
pub fn i_02ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACA0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ACA0: rts
pub fn i_02aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SetField89_02aca2  ($02ACA2, 22 B, slug/helpers_0295a6.s)
/// $02ACA2: movem.l a0, -(a7)
pub fn i_02aca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACA6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $02ACA6: lea.l $100580.l, a0
pub fn i_02aca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02ACAC: move.b #$ff, $89(a0)
pub fn i_02acac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACB2;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02ACB2: movem.l (a7)+, a0
pub fn i_02acb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACB6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $02ACB6: rts
pub fn i_02acb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CheckPlayersNear_02acb8  ($02ACB8, 56 B, slug/helpers_0295a6.s)
/// $02ACB8: cmpi.b #$0, $106ed3.l
pub fn i_02acb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACC0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED3) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02ACC0: bne.w $2acc8
pub fn i_02acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ACC8; return 10; }
    8
}
/// $02ACC4: bra.w $2acf0
pub fn i_02acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ACF0;
    10
}
/// $02ACC8: jsr $3eecc.l
pub fn i_02acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ACCE);
    cpu.pc = 0x3EECC;
    16
}
/// $02ACCE: bcc.w $2acdc
pub fn i_02acce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2ACDC; return 10; }
    8
}
/// $02ACD2: jsr $32d6c.l
pub fn i_02acd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ACD8);
    cpu.pc = 0x32D6C;
    16
}
/// $02ACD8: bcs.w $2acf6
pub fn i_02acd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACDC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2ACF6; return 10; }
    8
}
/// $02ACDC: jsr $3ef14.l
pub fn i_02acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ACE2);
    cpu.pc = 0x3EF14;
    16
}
/// $02ACE2: bcc.w $2acf0
pub fn i_02ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2ACF0; return 10; }
    8
}
/// $02ACE6: jsr $32d6c.l
pub fn i_02ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ACEC);
    cpu.pc = 0x32D6C;
    16
}
/// $02ACEC: bcs.w $2acf6
pub fn i_02acec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACF0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2ACF6; return 10; }
    8
}
// ===== ClearXN_02acf0  ($02ACF0, 6 B, thunks/ccr_helpers.c)
/// $02ACF0: andi.b #$ee, ccr
pub fn i_02acf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACF4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ACF4: rts
pub fn i_02acf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_02acf6  ($02ACF6, 6 B, thunks/ccr_helpers.c)
/// $02ACF6: ori.b #$11, ccr
pub fn i_02acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACFA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02ACFA: rts
pub fn i_02acfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IsAlive_02acfc  ($02ACFC, 46 B, slug/helpers_0295a6.s)
/// $02ACFC: lea.l $100580.l, a0
pub fn i_02acfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02AD02: cmpi.l #$ffffffff, (a0)
pub fn i_02ad02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD08;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $02AD08: beq.w $2ad30
pub fn i_02ad08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD0C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AD30; return 10; }
    8
}
/// $02AD0C: cmpi.l #$52a, (a0)
pub fn i_02ad0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD12;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $02AD12: beq.w $2ad30
pub fn i_02ad12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AD30; return 10; }
    8
}
/// $02AD16: cmpi.l #$400, (a0)
pub fn i_02ad16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD1C;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $02AD1C: beq.w $2ad30
pub fn i_02ad1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD20;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AD30; return 10; }
    8
}
/// $02AD20: cmpi.l #$2ae3e, (a0)
pub fn i_02ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD26;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x2AE3E, Size::Long, false);
    16
}
/// $02AD26: beq.w $2ad30
pub fn i_02ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AD30; return 10; }
    8
}
// ===== SetXN_02ad2a  ($02AD2A, 6 B, thunks/ccr_helpers.c)
/// $02AD2A: ori.b #$11, ccr
pub fn i_02ad2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD2E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AD2E: rts
pub fn i_02ad2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02ad30  ($02AD30, 6 B, thunks/ccr_helpers.c)
/// $02AD30: andi.b #$ee, ccr
pub fn i_02ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD34;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AD34: rts
pub fn i_02ad34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SpawnDrop_02ae50  ($02AE50, 64 B, slug/states_02ae3e.s)
/// $02AE50: jsr $2acfc(pc)
pub fn i_02ae50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AE54);
    cpu.pc = 0x2ACFC;
    16
}
/// $02AE54: bcs.w $2ae8e
pub fn i_02ae54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE58;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AE8E; return 10; }
    8
}
/// $02AE58: lea.l $2ae90(pc), a0
pub fn i_02ae58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE5C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2AE90;
    16
}
/// $02AE5C: moveq #$0, d0
pub fn i_02ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE5E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AE5E: move.b $98(a6), d0
pub fn i_02ae5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE62;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AE62: cmpi.b #$5, d0
pub fn i_02ae62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE66;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $02AE66: bcs.w $2ae6e
pub fn i_02ae66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE6A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AE6E; return 10; }
    8
}
/// $02AE6A: move.b #$4, d0
pub fn i_02ae6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE6E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02AE6E: lsl.w #$2, d0
pub fn i_02ae6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE70;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02AE70: movea.l (a0, d0.w), a1
pub fn i_02ae70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE74;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $02AE74: lea.l $100580.l, a0
pub fn i_02ae74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02AE7A: move.l a1, (a0)
pub fn i_02ae7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE7C;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02AE7C: jsr $5dd02.l
pub fn i_02ae7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AE82);
    cpu.pc = 0x5DD02;
    16
}
/// $02AE82: move.w $24(a6), $82(a0)
pub fn i_02ae82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE88;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02AE88: jmp $518.l
pub fn i_02ae88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE8E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $02AE8E: rts
pub fn i_02ae8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DropVariant3_02af64  ($02AF64, 36 B, slug/states_02ae3e.s)
/// $02AF64: move.w #$3, d1
pub fn i_02af64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF68;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AF68: jsr $2a0f8(pc)
pub fn i_02af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AF6C);
    cpu.pc = 0x2A0F8;
    16
}
/// $02AF6C: ori.w #$2, $38(a6)
pub fn i_02af6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF72;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02AF72: lea.l $459f0.l, a1
pub fn i_02af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x459F0;
    20
}
/// $02AF78: jsr $4ae.l
pub fn i_02af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF7E);
    cpu.pc = 0x4AE;
    16
}
/// $02AF7E: jsr $5dd02.l
pub fn i_02af7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF84);
    cpu.pc = 0x5DD02;
    16
}
/// $02AF84: bra.w $2afac
pub fn i_02af84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF88;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AFAC;
    10
}
/// $02AFAC: lea.l $31fca.l, a1
pub fn i_02afac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFB2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31FCA;
    20
}
/// $02AFB2: jsr $4ae.l
pub fn i_02afb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFB8);
    cpu.pc = 0x4AE;
    16
}
/// $02AFB8: lea.l $2fb9a(pc), a1
pub fn i_02afb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FB9A;
    16
}
/// $02AFBC: jsr $4ae.l
pub fn i_02afbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFC2);
    cpu.pc = 0x4AE;
    16
}
/// $02AFC2: jsr $5dd02.l
pub fn i_02afc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFC8);
    cpu.pc = 0x5DD02;
    16
}
/// $02AFC8: lea.l $2792b0.l, a0
pub fn i_02afc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFCE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2792B0;
    20
}
/// $02AFCE: jsr $28cd4.l
pub fn i_02afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFD4);
    cpu.pc = 0x28CD4;
    16
}
/// $02AFD4: clr.w $28(a6)
pub fn i_02afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFD8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AFD8: clr.w $2a(a6)
pub fn i_02afd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFDC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AFDC: clr.w $2c(a6)
pub fn i_02afdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFE0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AFE0: clr.w $2e(a6)
pub fn i_02afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFE4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AFE4: clr.b $26(a6)
pub fn i_02afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFE8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AFE8: clr.b $27(a6)
pub fn i_02afe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFEC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AFEC: clr.b $8e(a6)
pub fn i_02afec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFF0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AFF0: lea.l $29698(pc), a0
pub fn i_02aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFF4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29698;
    16
}
/// $02AFF4: move.l a0, $4c(a6)
pub fn i_02aff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFF8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02AFF8: jsr $283ca.l
pub fn i_02aff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFFE);
    cpu.pc = 0x283CA;
    16
}
/// $02AFFE: lea.l $2b004(pc), a1
pub fn i_02affe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B002;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B004;
    16
}
/// $02B002: move.l a1, (a6)
pub fn i_02b002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B004;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B004: move.b #$5a, $45(a6)
pub fn i_02b004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B00A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B00A: jsr $2aa0e(pc)
pub fn i_02b00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B00E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B00E);
    cpu.pc = 0x2AA0E;
    16
}
/// $02B00E: move.w #$fc00, $2a(a6)
pub fn i_02b00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B014;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B014: jsr $2a878(pc)
pub fn i_02b014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B018;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B018);
    cpu.pc = 0x2A878;
    16
}
/// $02B018: jsr $2a478(pc)
pub fn i_02b018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B01C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B01C);
    cpu.pc = 0x2A478;
    16
}
/// $02B01C: move.w $38(a6), d1
pub fn i_02b01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B020;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B020: move.w #$9fe0, d0
pub fn i_02b020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B024;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B024: andi.w #$ffe3, d1
pub fn i_02b024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B028;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B028: ori.w #$c, d0
pub fn i_02b028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B02C;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B02C: or.w d0, d1
pub fn i_02b02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B02E;
    cpu.ir = 0x8240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02B02E: move.w d1, $38(a6)
pub fn i_02b02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B032;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B032: jsr $28d70.l
pub fn i_02b032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B038;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B038);
    cpu.pc = 0x28D70;
    16
}
/// $02B038: jsr $283d8.l
pub fn i_02b038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B03E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B03E);
    cpu.pc = 0x283D8;
    16
}
/// $02B03E: cmpi.w #$180, $24(a6)
pub fn i_02b03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B044;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $02B044: bgt.w $2b04e
pub fn i_02b044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B048;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2B04E; return 10; }
    8
}
/// $02B048: lea.l $2b05e(pc), a1
pub fn i_02b048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B04C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B05E;
    16
}
/// $02B04C: move.l a1, (a6)
pub fn i_02b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B04E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B04E: jsr $2a8c0(pc)
pub fn i_02b04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B052;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B052);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B052: bcs.w $2b262
pub fn i_02b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B056;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B262; return 10; }
    8
}
// ===== SetTaskHandler_02b056  ($02B056, 8 B, kernel/task_handlers.c)
/// $02B056: lea.l $2b05e(pc), a1
pub fn i_02b056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B05A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B05E;
    16
}
/// $02B05A: move.l a1, (a6)
pub fn i_02b05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B05C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B05C: rts
pub fn i_02b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B05E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DropDescend_02b05e  ($02B05E, 264 B, slug/states_02ae3e.s)
/// $02B05E: lea.l $2792b0.l, a0
pub fn i_02b05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B064;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2792B0;
    20
}
/// $02B064: jsr $28cd4.l
pub fn i_02b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B06A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B06A);
    cpu.pc = 0x28CD4;
    16
}
/// $02B06A: addq.w #$8, $82(a6)
pub fn i_02b06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B06E;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B06E: move.w $24(a6), d0
pub fn i_02b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B072;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B072: cmp.w $82(a6), d0
pub fn i_02b072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B076;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B076: ble.w $2b07e
pub fn i_02b076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B07A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2B07E; return 10; }
    8
}
/// $02B07A: move.w d0, $82(a6)
pub fn i_02b07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B07E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B07E: jsr $2a958(pc)
pub fn i_02b07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B082;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B082);
    cpu.pc = 0x2A958;
    16
}
/// $02B082: cmp.w $80(a6), d2
pub fn i_02b082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B086;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B086: beq.w $2b08e
pub fn i_02b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B08A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B08E; return 10; }
    8
}
/// $02B08A: move.w d2, $80(a6)
pub fn i_02b08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B08E;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B08E: jsr $2a478(pc)
pub fn i_02b08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B092;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B092);
    cpu.pc = 0x2A478;
    16
}
/// $02B092: move.w #$fc00, $2a(a6)
pub fn i_02b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B098;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B098: lea.l $2b09e(pc), a1
pub fn i_02b098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B09C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B09E;
    16
}
/// $02B09C: move.l a1, (a6)
pub fn i_02b09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B09E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B09E: move.b #$46, $45(a6)
pub fn i_02b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B0A4: bset.b #$1, $8d(a6)
pub fn i_02b0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0AA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B0AA: jsr $2aa0e(pc)
pub fn i_02b0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B0AE);
    cpu.pc = 0x2AA0E;
    16
}
/// $02B0AE: move.w $2a(a6), d0
pub fn i_02b0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0B2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B0B2: move.w #$400, d1
pub fn i_02b0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B0B6: jsr $267f4.l
pub fn i_02b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B0BC);
    cpu.pc = 0x267F4;
    16
}
/// $02B0BC: move.w d0, $2a(a6)
pub fn i_02b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B0C0: jsr $2a878(pc)
pub fn i_02b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B0C4);
    cpu.pc = 0x2A878;
    16
}
/// $02B0C4: bcc.w $2b0ce
pub fn i_02b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B0CE; return 10; }
    8
}
/// $02B0C8: lea.l $2dd20(pc), a1
pub fn i_02b0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD20;
    16
}
/// $02B0CC: move.l a1, (a6)
pub fn i_02b0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B0CE: jsr $2a478(pc)
pub fn i_02b0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B0D2);
    cpu.pc = 0x2A478;
    16
}
/// $02B0D2: move.w $38(a6), d1
pub fn i_02b0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0D6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B0D6: move.w #$9fe0, d0
pub fn i_02b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0DA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B0DA: andi.w #$ffe3, d1
pub fn i_02b0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0DE;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B0DE: ori.w #$c, d0
pub fn i_02b0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0E2;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B0E2: or.w d0, d1
pub fn i_02b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0E4;
    cpu.ir = 0x8240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02B0E4: move.w d1, $38(a6)
pub fn i_02b0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0E8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B0E8: jsr $28d70.l
pub fn i_02b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B0EE);
    cpu.pc = 0x28D70;
    16
}
/// $02B0EE: jsr $283d8.l
pub fn i_02b0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B0F4);
    cpu.pc = 0x283D8;
    16
}
/// $02B0F4: tst.w $2a(a6)
pub fn i_02b0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0F8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02B0F8: bgt.w $2b164
pub fn i_02b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2B164; return 10; }
    8
}
/// $02B0FC: jsr $2a8c0(pc)
pub fn i_02b0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B100;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B100);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B100: cmpi.b #$0, d1
pub fn i_02b100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B104;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02B104: ble.w $2b110
pub fn i_02b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B108;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2B110; return 10; }
    8
}
/// $02B108: clr.w $28(a6)
pub fn i_02b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B10C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B10C: clr.w $2c(a6)
pub fn i_02b10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B110;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B110: jsr $2a958(pc)
pub fn i_02b110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B114;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B114);
    cpu.pc = 0x2A958;
    16
}
/// $02B114: cmp.w $80(a6), d2
pub fn i_02b114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B118;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B118: beq.w $2b164
pub fn i_02b118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B11C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B164; return 10; }
    8
}
/// $02B11C: move.w d2, $80(a6)
pub fn i_02b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B120;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B120: move.w d2, d0
pub fn i_02b120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B122;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B122: asr.w #$1, d0
pub fn i_02b122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B124;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B124: neg.w d0
pub fn i_02b124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B126;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02B126: addi.w #$20, d0
pub fn i_02b126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B12A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B12A: cmpi.w #$40, d0
pub fn i_02b12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B12E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02B12E: bls.w $2b136
pub fn i_02b12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B132;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2B136; return 10; }
    8
}
/// $02B132: move.w #$40, d0
pub fn i_02b132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B136;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B136: movem.w d0, -(a7)
pub fn i_02b136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B13A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02B13A: jsr $295a6(pc)
pub fn i_02b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B13E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B13E);
    cpu.pc = 0x295A6;
    16
}
/// $02B13E: movea.l #$2b0c30, a0
pub fn i_02b13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B144;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02B144: lsl.w #$2, d0
pub fn i_02b144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B146;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B146: movea.l (a0, d0.w), a0
pub fn i_02b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B14A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B14A: cmpa.l #$ffffffff, a0
pub fn i_02b14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B150;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B150: beq.w $2b15a
pub fn i_02b150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B154;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B15A; return 10; }
    8
}
/// $02B154: jsr $28cd4.l
pub fn i_02b154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B15A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B15A);
    cpu.pc = 0x28CD4;
    16
}
/// $02B15A: movem.w (a7)+, d0
pub fn i_02b15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B15E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02B15E: move.b #$14, $20(a6)
pub fn i_02b15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B164;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B164: rts
pub fn i_02b164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B166;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02B262: rts
pub fn i_02b262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B264;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IdleFlat_02b38c  ($02B38C, 318 B, slug/states_02ae3e.s)
/// $02B38C: jsr $267e6.l
pub fn i_02b38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B392;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B392);
    cpu.pc = 0x267E6;
    16
}
/// $02B392: clr.w $28(a6)
pub fn i_02b392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B396;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B396: jsr $2a9a0(pc)
pub fn i_02b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B39A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B39A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02B39A: movea.l #$2792f4, a0
pub fn i_02b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3A0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2792F4;
    cpu.a[0] = v1;
    4
}
/// $02B3A0: lsl.w #$2, d0
pub fn i_02b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3A2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B3A2: movea.l (a0, d0.w), a0
pub fn i_02b3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3A6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B3A6: cmpa.l #$ffffffff, a0
pub fn i_02b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3AC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B3AC: beq.w $2b3b6
pub fn i_02b3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B3B6; return 10; }
    8
}
/// $02B3B0: jsr $28cd4.l
pub fn i_02b3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B3B6);
    cpu.pc = 0x28CD4;
    16
}
/// $02B3B6: lea.l $29ffc(pc), a1
pub fn i_02b3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x29FFC;
    16
}
/// $02B3BA: movea.l (a1, d0.w), a0
pub fn i_02b3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3BE;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B3BE: move.l a0, $48(a6)
pub fn i_02b3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3C2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B3C2: move.b #$14, $20(a6)
pub fn i_02b3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3C8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B3C8: andi.w #$ff, d0
pub fn i_02b3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3CC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B3CC: lsr.w #$2, d0
pub fn i_02b3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3CE;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B3CE: add.b d0, $20(a6)
pub fn i_02b3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3D2;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B3D2: move.l #$295b4, $60(a6)
pub fn i_02b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3DA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B3DA: lea.l $2b3e0(pc), a1
pub fn i_02b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B3E0;
    16
}
/// $02B3DE: move.l a1, (a6)
pub fn i_02b3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B3E0: lea.l $29698(pc), a0
pub fn i_02b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3E4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29698;
    16
}
/// $02B3E4: move.l a0, $4c(a6)
pub fn i_02b3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3E8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B3E8: jsr $283ca.l
pub fn i_02b3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B3EE);
    cpu.pc = 0x283CA;
    16
}
/// $02B3EE: jsr $2aa24(pc)
pub fn i_02b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B3F2);
    cpu.pc = 0x2AA24;
    16
}
/// $02B3F2: jsr $2a7d8(pc)
pub fn i_02b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B3F6);
    cpu.pc = 0x2A7D8;
    16
}
/// $02B3F6: jsr $2a4f0(pc)
pub fn i_02b3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B3FA);
    cpu.pc = 0x2A4F0;
    16
}
/// $02B3FA: bcc.w $2b404
pub fn i_02b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B404; return 10; }
    8
}
/// $02B3FE: lea.l $2b4d2(pc), a1
pub fn i_02b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B402;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02B402: move.l a1, (a6)
pub fn i_02b402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B404;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B404: jsr $2abc0(pc)
pub fn i_02b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B408;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B408);
    cpu.pc = 0x2ABC0;
    16
}
/// $02B408: bcs.w $2b428
pub fn i_02b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B40C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B428; return 10; }
    8
}
/// $02B40C: jsr $2a328(pc)
pub fn i_02b40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B410;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B410);
    cpu.pc = 0x2A328;
    16
}
/// $02B410: bcc.w $2b41a
pub fn i_02b410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B414;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B41A; return 10; }
    8
}
/// $02B414: lea.l $2b8de(pc), a1
pub fn i_02b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B418;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B8DE;
    16
}
/// $02B418: move.l a1, (a6)
pub fn i_02b418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B41A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B41A: jsr $2a34e(pc)
pub fn i_02b41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B41E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B41E);
    cpu.pc = 0x2A34E;
    16
}
/// $02B41E: bcc.w $2b428
pub fn i_02b41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B422;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B428; return 10; }
    8
}
/// $02B422: lea.l $2ba44(pc), a1
pub fn i_02b422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B426;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BA44;
    16
}
/// $02B426: move.l a1, (a6)
pub fn i_02b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B428;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B428: jsr $2a59a(pc)
pub fn i_02b428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B42C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B42C);
    cpu.pc = 0x2A59A;
    16
}
/// $02B42C: bcc.w $2b448
pub fn i_02b42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B430;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B448; return 10; }
    8
}
/// $02B430: jsr $2a9a0(pc)
pub fn i_02b430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B434;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B434);
    cpu.pc = 0x2A9A0;
    16
}
/// $02B434: andi.w #$ff, d0
pub fn i_02b434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B438;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B438: lea.l $2a078(pc), a0
pub fn i_02b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B43C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02B43C: movea.l #$ffffffff, a1
pub fn i_02b43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B442;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B442: jsr $772.l
pub fn i_02b442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B448);
    cpu.pc = 0x772;
    16
}
/// $02B448: jsr $28d70.l
pub fn i_02b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B44E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B44E);
    cpu.pc = 0x28D70;
    16
}
/// $02B44E: jsr $2a8c0(pc)
pub fn i_02b44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B452;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B452);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B452: bcc.w $2b45c
pub fn i_02b452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B456;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B45C; return 10; }
    8
}
/// $02B456: lea.l $2d02e(pc), a1
pub fn i_02b456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B45A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02B45A: move.l a1, (a6)
pub fn i_02b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B45C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B45C: jsr $2aaf0(pc)
pub fn i_02b45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B460;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B460);
    cpu.pc = 0x2AAF0;
    16
}
/// $02B460: lea.l $2c9b0(pc), a0
pub fn i_02b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B464;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02B464: movea.l #$ffffffff, a1
pub fn i_02b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B46A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B46A: jsr $772.l
pub fn i_02b46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B470;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B470);
    cpu.pc = 0x772;
    16
}
/// $02B470: jsr $2aac0(pc)
pub fn i_02b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B474;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B474);
    cpu.pc = 0x2AAC0;
    16
}
/// $02B474: bcc.w $2b47e
pub fn i_02b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B478;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B47E; return 10; }
    8
}
/// $02B478: lea.l $2c24a(pc), a1
pub fn i_02b478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B47C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02B47C: move.l a1, (a6)
pub fn i_02b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B47E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B47E: jsr $2ac80(pc)
pub fn i_02b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B482;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B482);
    cpu.pc = 0x2AC80;
    16
}
/// $02B482: bcc.w $2b48c
pub fn i_02b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B486;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B48C; return 10; }
    8
}
/// $02B486: lea.l $2dd74(pc), a1
pub fn i_02b486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B48A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02B48A: move.l a1, (a6)
pub fn i_02b48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B48C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B48C: jsr $2a5cc(pc)
pub fn i_02b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B490;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B490);
    cpu.pc = 0x2A5CC;
    16
}
/// $02B490: bcc.w $2b49a
pub fn i_02b490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B494;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B49A; return 10; }
    8
}
/// $02B494: lea.l $2bbf2(pc), a1
pub fn i_02b494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B498;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02B498: move.l a1, (a6)
pub fn i_02b498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B49A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B49A: jsr $2a690(pc)
pub fn i_02b49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B49E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B49E);
    cpu.pc = 0x2A690;
    16
}
/// $02B49E: lea.l $2a060(pc), a0
pub fn i_02b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4A2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02B4A2: movea.l #$ffffffff, a1
pub fn i_02b4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4A8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B4A8: jsr $772.l
pub fn i_02b4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B4AE);
    cpu.pc = 0x772;
    16
}
/// $02B4AE: movea.l #$ffffffff, a0
pub fn i_02b4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02B4B4: lea.l $27964e.l, a0
pub fn i_02b4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02B4BA: jsr $5dd56.l
pub fn i_02b4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B4C0);
    cpu.pc = 0x5DD56;
    16
}
/// $02B4C0: bcc.w $2b4c8
pub fn i_02b4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B4C8; return 10; }
    8
}
/// $02B4C4: jmp $2dcbc(pc)
pub fn i_02b4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4C8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02B4C8: rts
pub fn i_02b4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_JumpCrouch_02c908  ($02C908, 84 B, slug/states_02ae3e.s)
/// $02C908: bclr.b #$5, $8c(a6)
pub fn i_02c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C90E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C90E: clr.w $2c(a6)
pub fn i_02c90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C912;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C912: jsr $2a9a0(pc)
pub fn i_02c912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C916;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C916);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C916: movea.l #$279308, a0
pub fn i_02c916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C91C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279308;
    cpu.a[0] = v1;
    4
}
/// $02C91C: lsl.w #$2, d0
pub fn i_02c91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C91E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C91E: movea.l (a0, d0.w), a0
pub fn i_02c91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C922;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C922: cmpa.l #$ffffffff, a0
pub fn i_02c922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C928;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02C928: beq.w $2c932
pub fn i_02c928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C92C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C932; return 10; }
    8
}
/// $02C92C: jsr $28cd4.l
pub fn i_02c92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C932);
    cpu.pc = 0x28CD4;
    16
}
/// $02C932: lea.l $2a038(pc), a1
pub fn i_02c932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C936;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A038;
    16
}
/// $02C936: movea.l (a1, d0.w), a0
pub fn i_02c936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C93A;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C93A: move.l a0, $48(a6)
pub fn i_02c93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C93E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C93E: move.b #$2d, $20(a6)
pub fn i_02c93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C944;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C944: andi.w #$ff, d0
pub fn i_02c944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C948;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C948: lsr.w #$2, d0
pub fn i_02c948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C94A;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C94A: add.b d0, $20(a6)
pub fn i_02c94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C94E;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02C94E: move.l #$295b4, $60(a6)
pub fn i_02c94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C956;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C956: lea.l $2c95c(pc), a1
pub fn i_02c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C95A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C95C;
    16
}
/// $02C95A: move.l a1, (a6)
pub fn i_02c95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C95C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Slug_JumpCrouch_Loop_02c95c  ($02C95C, 76 B, slug/states_02ae3e.s)
/// $02C95C: jsr $2aa24(pc)
pub fn i_02c95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C960;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C960);
    cpu.pc = 0x2AA24;
    16
}
/// $02C960: jsr $2a85a(pc)
pub fn i_02c960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C964;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C964);
    cpu.pc = 0x2A85A;
    16
}
/// $02C964: jsr $2a478(pc)
pub fn i_02c964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C968;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C968);
    cpu.pc = 0x2A478;
    16
}
/// $02C968: jsr $28d70.l
pub fn i_02c968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C96E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C96E);
    cpu.pc = 0x28D70;
    16
}
/// $02C96E: bcc.w $2c978
pub fn i_02c96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C972;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C978; return 10; }
    8
}
/// $02C972: lea.l $2ca0c(pc), a1
pub fn i_02c972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C976;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CA0C;
    16
}
/// $02C976: move.l a1, (a6)
pub fn i_02c976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C978;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C978: jsr $283d8.l
pub fn i_02c978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C97E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C97E);
    cpu.pc = 0x283D8;
    16
}
/// $02C97E: jsr $2aac0(pc)
pub fn i_02c97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C982;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C982);
    cpu.pc = 0x2AAC0;
    16
}
/// $02C982: bcc.w $2c98c
pub fn i_02c982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C986;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C98C; return 10; }
    8
}
/// $02C986: lea.l $2d286(pc), a1
pub fn i_02c986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C98A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D286;
    16
}
/// $02C98A: move.l a1, (a6)
pub fn i_02c98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C98C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C98C: jsr $2ac80(pc)
pub fn i_02c98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C990;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C990);
    cpu.pc = 0x2AC80;
    16
}
/// $02C990: bcc.w $2c99a
pub fn i_02c990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C994;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C99A; return 10; }
    8
}
/// $02C994: lea.l $2dd74(pc), a1
pub fn i_02c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C998;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C998: move.l a1, (a6)
pub fn i_02c998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C99A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C99A: jsr $2a690(pc)
pub fn i_02c99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C99E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C99E);
    cpu.pc = 0x2A690;
    16
}
/// $02C99E: lea.l $2a060(pc), a0
pub fn i_02c99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9A2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C9A2: movea.l #$ffffffff, a1
pub fn i_02c9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9A8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c9a8  ($02C9A8, 8 B, thunks/jsr_abs_thunks.c)
/// $02C9A8: jsr $772.l
pub fn i_02c9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C9AE);
    cpu.pc = 0x772;
    16
}
/// $02C9AE: rts
pub fn i_02c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_JumpNeutral_02c9c4  ($02C9C4, 16 B, slug/states_02ae3e.s)
/// $02C9C4: bclr.b #$5, $8c(a6)
pub fn i_02c9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9CA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C9CA: move.w #$0, $28(a6)
pub fn i_02c9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C9D0: bra.w $2c908
pub fn i_02c9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C908;
    10
}
// ===== Slug_JumpLaunch_02ca0c  ($02CA0C, 126 B, slug/states_02ae3e.s)
/// $02CA0C: bclr.b #$3, $5b(a6)
pub fn i_02ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA12;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CA12: bclr.b #$5, $8c(a6)
pub fn i_02ca12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA18;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CA18: move.w #$4a4, $2a(a6)
pub fn i_02ca18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4A4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA1E: move.w #$ff9d, $2e(a6)
pub fn i_02ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA24: move.w $24(a6), d0
pub fn i_02ca24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA28;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CA28: cmp.w $82(a6), d0
pub fn i_02ca28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA2C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02CA2C: ble.w $2ca34
pub fn i_02ca2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA30;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2CA34; return 10; }
    8
}
/// $02CA30: move.w d0, $82(a6)
pub fn i_02ca30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA34;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA34: jsr $2a9a0(pc)
pub fn i_02ca34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CA38);
    cpu.pc = 0x2A9A0;
    16
}
/// $02CA38: movea.l #$27931c, a0
pub fn i_02ca38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA3E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27931C;
    cpu.a[0] = v1;
    4
}
/// $02CA3E: lsl.w #$2, d0
pub fn i_02ca3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA40;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CA40: movea.l (a0, d0.w), a0
pub fn i_02ca40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA44;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CA44: cmpa.l #$ffffffff, a0
pub fn i_02ca44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA4A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02CA4A: beq.w $2ca54
pub fn i_02ca4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CA54; return 10; }
    8
}
/// $02CA4E: jsr $28cd4.l
pub fn i_02ca4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CA54);
    cpu.pc = 0x28CD4;
    16
}
/// $02CA54: lea.l $2a04c(pc), a1
pub fn i_02ca54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A04C;
    16
}
/// $02CA58: movea.l (a1, d0.w), a0
pub fn i_02ca58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA5C;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CA5C: move.l a0, $48(a6)
pub fn i_02ca5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA60;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02CA60: move.b #$32, $20(a6)
pub fn i_02ca60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA66;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CA66: andi.w #$ff, d0
pub fn i_02ca66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA6A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CA6A: lsr.w #$2, d0
pub fn i_02ca6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA6C;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CA6C: add.b d0, $20(a6)
pub fn i_02ca6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA70;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02CA70: lea.l $2ca8a(pc), a1
pub fn i_02ca70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CA8A;
    16
}
/// $02CA74: move.l a1, (a6)
pub fn i_02ca74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CA76: jsr $2a958(pc)
pub fn i_02ca76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CA7A);
    cpu.pc = 0x2A958;
    16
}
/// $02CA7A: cmp.w $80(a6), d2
pub fn i_02ca7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA7E;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02CA7E: beq.w $2ca86
pub fn i_02ca7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CA86; return 10; }
    8
}
/// $02CA82: move.w d2, $80(a6)
pub fn i_02ca82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA86;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA86: jsr $2a478(pc)
pub fn i_02ca86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CA8A);
    cpu.pc = 0x2A478;
    16
}
// ===== Slug_JumpAir_02ca8a  ($02CA8A, 376 B, slug/states_02ae3e.s)
/// $02CA8A: jsr $2aac0(pc)
pub fn i_02ca8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CA8E);
    cpu.pc = 0x2AAC0;
    16
}
/// $02CA8E: bcc.w $2ca98
pub fn i_02ca8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CA98; return 10; }
    8
}
/// $02CA92: lea.l $2d298(pc), a1
pub fn i_02ca92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D298;
    16
}
/// $02CA96: move.l a1, (a6)
pub fn i_02ca96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CA98: jsr $5d5b6.l
pub fn i_02ca98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CA9E);
    cpu.pc = 0x5D5B6;
    16
}
/// $02CA9E: asl.w #$1, d0
pub fn i_02ca9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAA0;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CAA0: lea.l $2c900(pc), a0
pub fn i_02caa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C900;
    16
}
/// $02CAA4: move.w (a0, d0.w), $2c(a6)
pub fn i_02caa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAAA;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02CAAA: move.w $28(a6), d0
pub fn i_02caaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAAE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CAAE: move.w #$2a0, d1
pub fn i_02caae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CAB2: jsr $267f4.l
pub fn i_02cab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CAB8);
    cpu.pc = 0x267F4;
    16
}
/// $02CAB8: move.w d0, $28(a6)
pub fn i_02cab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CABC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CABC: move.w $2a(a6), d0
pub fn i_02cabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAC0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CAC0: bgt.w $2cad2
pub fn i_02cac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAC4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2CAD2; return 10; }
    8
}
/// $02CAC4: move.w #$400, d1
pub fn i_02cac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAC8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CAC8: jsr $267f4.l
pub fn i_02cac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CACE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CACE);
    cpu.pc = 0x267F4;
    16
}
/// $02CACE: move.w d0, $2a(a6)
pub fn i_02cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAD2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CAD2: bset.b #$1, $8d(a6)
pub fn i_02cad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAD8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CAD8: jsr $2aa0e(pc)
pub fn i_02cad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CADC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CADC);
    cpu.pc = 0x2AA0E;
    16
}
/// $02CADC: jsr $2a878(pc)
pub fn i_02cadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CAE0);
    cpu.pc = 0x2A878;
    16
}
/// $02CAE0: bcc.w $2cb1a
pub fn i_02cae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CB1A; return 10; }
    8
}
/// $02CAE4: lea.l $2cc1a(pc), a1
pub fn i_02cae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CC1A;
    16
}
/// $02CAE8: move.l a1, (a6)
pub fn i_02cae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CAEA: jsr $5cef8.l
pub fn i_02caea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CAF0);
    cpu.pc = 0x5CEF8;
    16
}
/// $02CAF0: bcc.w $2cb0c
pub fn i_02caf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CB0C; return 10; }
    8
}
/// $02CAF4: jsr $2a9a0(pc)
pub fn i_02caf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CAF8);
    cpu.pc = 0x2A9A0;
    16
}
/// $02CAF8: andi.w #$ff, d0
pub fn i_02caf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAFC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CAFC: lea.l $2a08c(pc), a0
pub fn i_02cafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB00;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A08C;
    16
}
/// $02CB00: movea.l #$ffffffff, a1
pub fn i_02cb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB06;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CB06: jsr $772.l
pub fn i_02cb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CB0C);
    cpu.pc = 0x772;
    16
}
/// $02CB0C: jsr $2aac0(pc)
pub fn i_02cb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CB10);
    cpu.pc = 0x2AAC0;
    16
}
/// $02CB10: bcc.w $2cb1a
pub fn i_02cb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CB1A; return 10; }
    8
}
/// $02CB14: lea.l $2c24a(pc), a1
pub fn i_02cb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02CB18: move.l a1, (a6)
pub fn i_02cb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CB1A: jsr $28d70.l
pub fn i_02cb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CB20);
    cpu.pc = 0x28D70;
    16
}
/// $02CB20: tst.w $2a(a6)
pub fn i_02cb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB24;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02CB24: bgt.w $2cbb0
pub fn i_02cb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB28;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2CBB0; return 10; }
    8
}
/// $02CB28: jsr $2a8c0(pc)
pub fn i_02cb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CB2C);
    cpu.pc = 0x2A8C0;
    16
}
/// $02CB2C: cmpi.b #$0, d1
pub fn i_02cb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB30;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02CB30: ble.w $2cbb0
pub fn i_02cb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB34;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2CBB0; return 10; }
    8
}
/// $02CB34: clr.w $28(a6)
pub fn i_02cb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB38;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CB38: clr.w $2c(a6)
pub fn i_02cb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB3C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CB3C: jsr $2a958(pc)
pub fn i_02cb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CB40);
    cpu.pc = 0x2A958;
    16
}
/// $02CB40: cmp.w $80(a6), d2
pub fn i_02cb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB44;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02CB44: beq.w $2cbb0
pub fn i_02cb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CBB0; return 10; }
    8
}
/// $02CB48: move.w d2, $80(a6)
pub fn i_02cb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB4C;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CB4C: move.w d2, d0
pub fn i_02cb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB4E;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CB4E: asr.w #$1, d0
pub fn i_02cb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB50;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CB50: neg.w d0
pub fn i_02cb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB52;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02CB52: addi.w #$20, d0
pub fn i_02cb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB56;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CB56: cmpi.w #$40, d0
pub fn i_02cb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB5A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02CB5A: bls.w $2cb62
pub fn i_02cb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB5E;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2CB62; return 10; }
    8
}
/// $02CB5E: move.w #$40, d0
pub fn i_02cb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB62;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CB62: movem.w d0, -(a7)
pub fn i_02cb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB66;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02CB66: jsr $295a6(pc)
pub fn i_02cb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CB6A);
    cpu.pc = 0x295A6;
    16
}
/// $02CB6A: movea.l #$2b0c30, a0
pub fn i_02cb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB70;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02CB70: lsl.w #$2, d0
pub fn i_02cb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB72;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CB72: movea.l (a0, d0.w), a0
pub fn i_02cb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB76;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CB76: cmpa.l #$ffffffff, a0
pub fn i_02cb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB7C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02CB7C: beq.w $2cb86
pub fn i_02cb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CB86; return 10; }
    8
}
/// $02CB80: jsr $28cd4.l
pub fn i_02cb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CB86);
    cpu.pc = 0x28CD4;
    16
}
/// $02CB86: movem.w (a7)+, d0
pub fn i_02cb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB8A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02CB8A: move.b #$0, $20(a6)
pub fn i_02cb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CB90: lsr.w #$2, d0
pub fn i_02cb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB92;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CB92: andi.w #$ff, d0
pub fn i_02cb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB96;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CB96: add.b d0, $20(a6)
pub fn i_02cb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB9A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02CB9A: bset.b #$2, $8d(a6)
pub fn i_02cb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBA0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CBA0: lea.l $2ca8a(pc), a0
pub fn i_02cba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2CA8A;
    16
}
/// $02CBA4: cmpa.l (a6), a0
pub fn i_02cba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBA6;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $02CBA6: bne.w $2cbb0
pub fn i_02cba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBAA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2CBB0; return 10; }
    8
}
/// $02CBAA: lea.l $2cc02(pc), a1
pub fn i_02cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CC02;
    16
}
/// $02CBAE: move.l a1, (a6)
pub fn i_02cbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CBB0: btst.b #$1, $8d(a6)
pub fn i_02cbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBB6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02CBB6: beq.w $2cbd2
pub fn i_02cbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CBD2; return 10; }
    8
}
/// $02CBBA: jsr $2a690(pc)
pub fn i_02cbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CBBE);
    cpu.pc = 0x2A690;
    16
}
/// $02CBBE: lea.l $2a06c(pc), a0
pub fn i_02cbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBC2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A06C;
    16
}
/// $02CBC2: movea.l #$ffffffff, a1
pub fn i_02cbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBC8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CBC8: jsr $772.l
pub fn i_02cbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CBCE);
    cpu.pc = 0x772;
    16
}
/// $02CBCE: bra.w $2cbe6
pub fn i_02cbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2CBE6;
    10
}
/// $02CBD2: jsr $2a690(pc)
pub fn i_02cbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CBD6);
    cpu.pc = 0x2A690;
    16
}
/// $02CBD6: lea.l $2a060(pc), a0
pub fn i_02cbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBDA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02CBDA: movea.l #$ffffffff, a1
pub fn i_02cbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBE0;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CBE0: jsr $772.l
pub fn i_02cbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CBE6);
    cpu.pc = 0x772;
    16
}
/// $02CBE6: movea.l #$ffffffff, a0
pub fn i_02cbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBEC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02CBEC: lea.l $27964e.l, a0
pub fn i_02cbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02CBF2: jsr $5dd56.l
pub fn i_02cbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CBF8);
    cpu.pc = 0x5DD56;
    16
}
/// $02CBF8: bcc.w $2cc00
pub fn i_02cbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CC00; return 10; }
    8
}
/// $02CBFC: jmp $2dcbc(pc)
pub fn i_02cbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC00;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02CC00: rts
pub fn i_02cc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02DC8C: jsr $5b6.l
pub fn i_02dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DC92);
    cpu.pc = 0x5B6;
    16
}
/// $02DC92: bclr.b #$7, $5b(a6)
pub fn i_02dc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC98;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DC98: jsr $13600.l
pub fn i_02dc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DC9E);
    cpu.pc = 0x13600;
    16
}
/// $02DC9E: lea.l $2dcaa(pc), a1
pub fn i_02dc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DCAA;
    16
}
/// $02DCA2: move.l a1, (a6)
pub fn i_02dca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_02dca4  ($02DCA4, 6 B, thunks/jsr_pc_thunks.c)
/// $02DCA4: jsr $2ac80(pc)
pub fn i_02dca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DCA8);
    cpu.pc = 0x2AC80;
    16
}
/// $02DCA8: rts
pub fn i_02dca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DeathFinishJmp_02dcbc  ($02DCBC, 4 B, slug/states_02ae3e.s)
/// $02DCBC: jmp $2dc8c(pc)
pub fn i_02dcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCC0;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DC8C;
    8
}
// ===== Slug_IdleEnterA_02dd20  ($02DD20, 84 B, slug/vehicle_02dd20.s)
/// $02DD20: bclr.b #$5, $8c(a6)
pub fn i_02dd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD26;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DD26: jsr $2a9a0(pc)
pub fn i_02dd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DD2A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02DD2A: movea.l #$279614, a0
pub fn i_02dd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD30;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279614;
    cpu.a[0] = v1;
    4
}
/// $02DD30: lsl.w #$2, d0
pub fn i_02dd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD32;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DD32: movea.l (a0, d0.w), a0
pub fn i_02dd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD36;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02DD36: cmpa.l #$ffffffff, a0
pub fn i_02dd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD3C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02DD3C: beq.w $2dd46
pub fn i_02dd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DD46; return 10; }
    8
}
/// $02DD40: jsr $28cd4.l
pub fn i_02dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DD46);
    cpu.pc = 0x28CD4;
    16
}
/// $02DD46: move.b #$41, $20(a6)
pub fn i_02dd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02DD4C: andi.w #$ff, d0
pub fn i_02dd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD50;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02DD50: lsr.w #$2, d0
pub fn i_02dd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD52;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02DD52: add.b d0, $20(a6)
pub fn i_02dd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD56;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02DD56: clr.w $2c(a6)
pub fn i_02dd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD5A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02DD5A: move.l #$ffffffff, $60(a6)
pub fn i_02dd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD62;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DD62: lea.l $2999e(pc), a0
pub fn i_02dd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD66;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x02875E, i_02875e),
    (0x028762, i_028762),
    (0x028766, i_028766),
    (0x028768, i_028768),
    (0x02876C, i_02876c),
    (0x02876E, i_02876e),
    (0x028770, i_028770),
    (0x028774, i_028774),
    (0x02877A, i_02877a),
    (0x02877E, i_02877e),
    (0x028780, i_028780),
    (0x028782, i_028782),
    (0x028784, i_028784),
    (0x028786, i_028786),
    (0x02878A, i_02878a),
    (0x02878C, i_02878c),
    (0x02878E, i_02878e),
    (0x028790, i_028790),
    (0x028792, i_028792),
    (0x028794, i_028794),
    (0x028796, i_028796),
    (0x02879A, i_02879a),
    (0x0287A0, i_0287a0),
    (0x0287A4, i_0287a4),
    (0x0287A6, i_0287a6),
    (0x0287A8, i_0287a8),
    (0x0287AA, i_0287aa),
    (0x0287AC, i_0287ac),
    (0x0287B0, i_0287b0),
    (0x0287B2, i_0287b2),
    (0x0287B4, i_0287b4),
    (0x0287B6, i_0287b6),
    (0x0287B8, i_0287b8),
    (0x0287BA, i_0287ba),
    (0x0287BE, i_0287be),
    (0x0287C2, i_0287c2),
    (0x0287C6, i_0287c6),
    (0x0287CA, i_0287ca),
    (0x0287CC, i_0287cc),
    (0x0287D0, i_0287d0),
    (0x0287D2, i_0287d2),
    (0x0287D6, i_0287d6),
    (0x0287DA, i_0287da),
    (0x0287DC, i_0287dc),
    (0x0287DE, i_0287de),
    (0x0287E2, i_0287e2),
    (0x0287E4, i_0287e4),
    (0x0287E6, i_0287e6),
    (0x0287EA, i_0287ea),
    (0x0287EC, i_0287ec),
    (0x0287EE, i_0287ee),
    (0x0287F0, i_0287f0),
    (0x0287F2, i_0287f2),
    (0x0287F6, i_0287f6),
    (0x0287FA, i_0287fa),
    (0x028800, i_028800),
    (0x028804, i_028804),
    (0x028806, i_028806),
    (0x028808, i_028808),
    (0x02880A, i_02880a),
    (0x02880C, i_02880c),
    (0x028810, i_028810),
    (0x028812, i_028812),
    (0x028814, i_028814),
    (0x028816, i_028816),
    (0x028818, i_028818),
    (0x02881A, i_02881a),
    (0x02881E, i_02881e),
    (0x028822, i_028822),
    (0x028828, i_028828),
    (0x02882C, i_02882c),
    (0x02882E, i_02882e),
    (0x028830, i_028830),
    (0x028832, i_028832),
    (0x028834, i_028834),
    (0x028838, i_028838),
    (0x02883A, i_02883a),
    (0x02883C, i_02883c),
    (0x02883E, i_02883e),
    (0x028840, i_028840),
    (0x028842, i_028842),
    (0x028846, i_028846),
    (0x02884A, i_02884a),
    (0x02884E, i_02884e),
    (0x028852, i_028852),
    (0x028854, i_028854),
    (0x028858, i_028858),
    (0x02885A, i_02885a),
    (0x02885E, i_02885e),
    (0x028862, i_028862),
    (0x028864, i_028864),
    (0x028866, i_028866),
    (0x02886A, i_02886a),
    (0x02886C, i_02886c),
    (0x02886E, i_02886e),
    (0x028872, i_028872),
    (0x028874, i_028874),
    (0x028876, i_028876),
    (0x028878, i_028878),
    (0x02887C, i_02887c),
    (0x028880, i_028880),
    (0x028884, i_028884),
    (0x028930, i_028930),
    (0x028936, i_028936),
    (0x02893A, i_02893a),
    (0x028940, i_028940),
    (0x028944, i_028944),
    (0x028948, i_028948),
    (0x02894C, i_02894c),
    (0x028950, i_028950),
    (0x028954, i_028954),
    (0x028958, i_028958),
    (0x02895A, i_02895a),
    (0x02895E, i_02895e),
    (0x028962, i_028962),
    (0x028966, i_028966),
    (0x02896A, i_02896a),
    (0x02896E, i_02896e),
    (0x028972, i_028972),
    (0x028976, i_028976),
    (0x02897A, i_02897a),
    (0x02897E, i_02897e),
    (0x028982, i_028982),
    (0x028986, i_028986),
    (0x02898A, i_02898a),
    (0x02898C, i_02898c),
    (0x028990, i_028990),
    (0x028992, i_028992),
    (0x028996, i_028996),
    (0x028998, i_028998),
    (0x02899E, i_02899e),
    (0x0289A2, i_0289a2),
    (0x0289A6, i_0289a6),
    (0x0289AA, i_0289aa),
    (0x0289AE, i_0289ae),
    (0x0289B4, i_0289b4),
    (0x0289B8, i_0289b8),
    (0x0289BE, i_0289be),
    (0x0289C2, i_0289c2),
    (0x0289C6, i_0289c6),
    (0x0289CC, i_0289cc),
    (0x0289D0, i_0289d0),
    (0x0289D4, i_0289d4),
    (0x0289D8, i_0289d8),
    (0x0289DC, i_0289dc),
    (0x0289E2, i_0289e2),
    (0x0289E6, i_0289e6),
    (0x0289EA, i_0289ea),
    (0x0289EE, i_0289ee),
    (0x0289F0, i_0289f0),
    (0x0289F4, i_0289f4),
    (0x028A96, i_028a96),
    (0x028A98, i_028a98),
    (0x028A9C, i_028a9c),
    (0x028AA0, i_028aa0),
    (0x028AA6, i_028aa6),
    (0x028AAA, i_028aaa),
    (0x028AAE, i_028aae),
    (0x028AB2, i_028ab2),
    (0x028AB8, i_028ab8),
    (0x028ABC, i_028abc),
    (0x028ABE, i_028abe),
    (0x028AC4, i_028ac4),
    (0x028AC6, i_028ac6),
    (0x028ACC, i_028acc),
    (0x028AD0, i_028ad0),
    (0x028AD4, i_028ad4),
    (0x028AD8, i_028ad8),
    (0x028ADA, i_028ada),
    (0x028AE0, i_028ae0),
    (0x028AE4, i_028ae4),
    (0x028AE8, i_028ae8),
    (0x028AEA, i_028aea),
    (0x028AEE, i_028aee),
    (0x028AF4, i_028af4),
    (0x028AFA, i_028afa),
    (0x028AFE, i_028afe),
    (0x028B04, i_028b04),
    (0x028B08, i_028b08),
    (0x028B0C, i_028b0c),
    (0x028B0E, i_028b0e),
    (0x028B12, i_028b12),
    (0x028B14, i_028b14),
    (0x028B16, i_028b16),
    (0x028B1A, i_028b1a),
    (0x028B20, i_028b20),
    (0x028B24, i_028b24),
    (0x028B26, i_028b26),
    (0x028B28, i_028b28),
    (0x028B2A, i_028b2a),
    (0x028B2C, i_028b2c),
    (0x028B30, i_028b30),
    (0x028B32, i_028b32),
    (0x028B34, i_028b34),
    (0x028B36, i_028b36),
    (0x028B38, i_028b38),
    (0x028B3A, i_028b3a),
    (0x028B3C, i_028b3c),
    (0x028B40, i_028b40),
    (0x028B46, i_028b46),
    (0x028B4A, i_028b4a),
    (0x028B4C, i_028b4c),
    (0x028B4E, i_028b4e),
    (0x028B50, i_028b50),
    (0x028B52, i_028b52),
    (0x028B56, i_028b56),
    (0x028B58, i_028b58),
    (0x028B5A, i_028b5a),
    (0x028B5C, i_028b5c),
    (0x028B5E, i_028b5e),
    (0x028B60, i_028b60),
    (0x028B64, i_028b64),
    (0x028B68, i_028b68),
    (0x028B6C, i_028b6c),
    (0x028B70, i_028b70),
    (0x028B72, i_028b72),
    (0x028B76, i_028b76),
    (0x028B78, i_028b78),
    (0x028B7C, i_028b7c),
    (0x028B80, i_028b80),
    (0x028B82, i_028b82),
    (0x028B84, i_028b84),
    (0x028B88, i_028b88),
    (0x028B8A, i_028b8a),
    (0x028B8C, i_028b8c),
    (0x028B90, i_028b90),
    (0x028B92, i_028b92),
    (0x028B96, i_028b96),
    (0x028B9A, i_028b9a),
    (0x028B9E, i_028b9e),
    (0x028BA2, i_028ba2),
    (0x028BA4, i_028ba4),
    (0x028BA6, i_028ba6),
    (0x028BA8, i_028ba8),
    (0x028BAC, i_028bac),
    (0x028BB0, i_028bb0),
    (0x028BB6, i_028bb6),
    (0x028BBA, i_028bba),
    (0x028BBC, i_028bbc),
    (0x028BBE, i_028bbe),
    (0x028BC0, i_028bc0),
    (0x028BC2, i_028bc2),
    (0x028BC6, i_028bc6),
    (0x028BC8, i_028bc8),
    (0x028BCA, i_028bca),
    (0x028BCC, i_028bcc),
    (0x028BCE, i_028bce),
    (0x028BD0, i_028bd0),
    (0x028BD4, i_028bd4),
    (0x028BD8, i_028bd8),
    (0x028BDE, i_028bde),
    (0x028BE2, i_028be2),
    (0x028BE4, i_028be4),
    (0x028BE6, i_028be6),
    (0x028BE8, i_028be8),
    (0x028BEA, i_028bea),
    (0x028BEE, i_028bee),
    (0x028BF0, i_028bf0),
    (0x028BF2, i_028bf2),
    (0x028BF4, i_028bf4),
    (0x028BF6, i_028bf6),
    (0x028BF8, i_028bf8),
    (0x028BFC, i_028bfc),
    (0x028C00, i_028c00),
    (0x028C04, i_028c04),
    (0x028C08, i_028c08),
    (0x028C0A, i_028c0a),
    (0x028C0E, i_028c0e),
    (0x028C10, i_028c10),
    (0x028C14, i_028c14),
    (0x028C18, i_028c18),
    (0x028C1A, i_028c1a),
    (0x028C1E, i_028c1e),
    (0x028CD4, i_028cd4),
    (0x028CD8, i_028cd8),
    (0x028CDC, i_028cdc),
    (0x028CE0, i_028ce0),
    (0x028CE4, i_028ce4),
    (0x028CE8, i_028ce8),
    (0x028CEE, i_028cee),
    (0x028D70, i_028d70),
    (0x028D74, i_028d74),
    (0x028D7A, i_028d7a),
    (0x028D7E, i_028d7e),
    (0x028D82, i_028d82),
    (0x028D88, i_028d88),
    (0x028D8C, i_028d8c),
    (0x028D8E, i_028d8e),
    (0x028D92, i_028d92),
    (0x028D96, i_028d96),
    (0x028D9A, i_028d9a),
    (0x028D9E, i_028d9e),
    (0x028DA0, i_028da0),
    (0x028DA4, i_028da4),
    (0x028DA8, i_028da8),
    (0x028DAA, i_028daa),
    (0x028DAC, i_028dac),
    (0x028DB0, i_028db0),
    (0x028DB4, i_028db4),
    (0x028DB6, i_028db6),
    (0x028DB8, i_028db8),
    (0x028DBC, i_028dbc),
    (0x028DBE, i_028dbe),
    (0x028DC0, i_028dc0),
    (0x028DC6, i_028dc6),
    (0x028DC8, i_028dc8),
    (0x028DCE, i_028dce),
    (0x028DD2, i_028dd2),
    (0x028DD4, i_028dd4),
    (0x028DD8, i_028dd8),
    (0x028DDC, i_028ddc),
    (0x028DE0, i_028de0),
    (0x028DE4, i_028de4),
    (0x028DEA, i_028dea),
    (0x028DEE, i_028dee),
    (0x028DF2, i_028df2),
    (0x028DF6, i_028df6),
    (0x028DFA, i_028dfa),
    (0x028DFE, i_028dfe),
    (0x028E02, i_028e02),
    (0x028E06, i_028e06),
    (0x028E0A, i_028e0a),
    (0x028E0C, i_028e0c),
    (0x028E12, i_028e12),
    (0x028E16, i_028e16),
    (0x028E1A, i_028e1a),
    (0x028E1E, i_028e1e),
    (0x028E22, i_028e22),
    (0x028E26, i_028e26),
    (0x028E2A, i_028e2a),
    (0x028E2E, i_028e2e),
    (0x028E30, i_028e30),
    (0x028E34, i_028e34),
    (0x028E38, i_028e38),
    (0x028E3C, i_028e3c),
    (0x028E40, i_028e40),
    (0x028E42, i_028e42),
    (0x028E46, i_028e46),
    (0x028E48, i_028e48),
    (0x028E4C, i_028e4c),
    (0x028E4E, i_028e4e),
    (0x028E52, i_028e52),
    (0x028E56, i_028e56),
    (0x028E5A, i_028e5a),
    (0x028E60, i_028e60),
    (0x028E64, i_028e64),
    (0x028E6A, i_028e6a),
    (0x028E6E, i_028e6e),
    (0x028E74, i_028e74),
    (0x028E78, i_028e78),
    (0x028E7E, i_028e7e),
    (0x028E82, i_028e82),
    (0x028E8A, i_028e8a),
    (0x028E8E, i_028e8e),
    (0x028E96, i_028e96),
    (0x028E9C, i_028e9c),
    (0x028EA2, i_028ea2),
    (0x028EA6, i_028ea6),
    (0x028EAC, i_028eac),
    (0x028EB0, i_028eb0),
    (0x028EB4, i_028eb4),
    (0x028EB6, i_028eb6),
    (0x028EBA, i_028eba),
    (0x028EBE, i_028ebe),
    (0x028EC0, i_028ec0),
    (0x028EC2, i_028ec2),
    (0x028EC6, i_028ec6),
    (0x028ECC, i_028ecc),
    (0x028ED0, i_028ed0),
    (0x028ED6, i_028ed6),
    (0x028EDA, i_028eda),
    (0x028EDE, i_028ede),
    (0x028EE4, i_028ee4),
    (0x028EE8, i_028ee8),
    (0x028EEC, i_028eec),
    (0x028EF0, i_028ef0),
    (0x028EF6, i_028ef6),
    (0x028EFA, i_028efa),
    (0x028F00, i_028f00),
    (0x028F04, i_028f04),
    (0x028F0A, i_028f0a),
    (0x028F0E, i_028f0e),
    (0x028F14, i_028f14),
    (0x028F18, i_028f18),
    (0x028F1C, i_028f1c),
    (0x028F22, i_028f22),
    (0x028F2A, i_028f2a),
    (0x028F2E, i_028f2e),
    (0x028F34, i_028f34),
    (0x028F38, i_028f38),
    (0x028F3E, i_028f3e),
    (0x028F42, i_028f42),
    (0x028F46, i_028f46),
    (0x028F4A, i_028f4a),
    (0x028F4C, i_028f4c),
    (0x028F50, i_028f50),
    (0x028F54, i_028f54),
    (0x028F56, i_028f56),
    (0x028F5A, i_028f5a),
    (0x028F5E, i_028f5e),
    (0x028F62, i_028f62),
    (0x028F66, i_028f66),
    (0x028F6A, i_028f6a),
    (0x028F6C, i_028f6c),
    (0x028F6E, i_028f6e),
    (0x028F72, i_028f72),
    (0x028F76, i_028f76),
    (0x028F7A, i_028f7a),
    (0x028F7E, i_028f7e),
    (0x028F82, i_028f82),
    (0x028F86, i_028f86),
    (0x028F8A, i_028f8a),
    (0x028F8E, i_028f8e),
    (0x028F92, i_028f92),
    (0x028F96, i_028f96),
    (0x028F9A, i_028f9a),
    (0x028F9E, i_028f9e),
    (0x028FA2, i_028fa2),
    (0x028FA8, i_028fa8),
    (0x028FAC, i_028fac),
    (0x028FB0, i_028fb0),
    (0x028FB6, i_028fb6),
    (0x028FBA, i_028fba),
    (0x028FC0, i_028fc0),
    (0x028FC4, i_028fc4),
    (0x028FCA, i_028fca),
    (0x028FCE, i_028fce),
    (0x028FD4, i_028fd4),
    (0x028FD8, i_028fd8),
    (0x028FDC, i_028fdc),
    (0x028FE2, i_028fe2),
    (0x028FE6, i_028fe6),
    (0x028FEA, i_028fea),
    (0x028FEE, i_028fee),
    (0x028FF2, i_028ff2),
    (0x028FF6, i_028ff6),
    (0x028FFA, i_028ffa),
    (0x028FFE, i_028ffe),
    (0x029002, i_029002),
    (0x029006, i_029006),
    (0x02900A, i_02900a),
    (0x029010, i_029010),
    (0x029014, i_029014),
    (0x029018, i_029018),
    (0x02901C, i_02901c),
    (0x029020, i_029020),
    (0x029024, i_029024),
    (0x029026, i_029026),
    (0x02902A, i_02902a),
    (0x02902C, i_02902c),
    (0x029030, i_029030),
    (0x029034, i_029034),
    (0x029038, i_029038),
    (0x02903A, i_02903a),
    (0x02903E, i_02903e),
    (0x029042, i_029042),
    (0x029044, i_029044),
    (0x029048, i_029048),
    (0x02904A, i_02904a),
    (0x02904E, i_02904e),
    (0x029050, i_029050),
    (0x029054, i_029054),
    (0x029058, i_029058),
    (0x02905C, i_02905c),
    (0x029060, i_029060),
    (0x029066, i_029066),
    (0x02906A, i_02906a),
    (0x02906E, i_02906e),
    (0x029072, i_029072),
    (0x029078, i_029078),
    (0x02907E, i_02907e),
    (0x029082, i_029082),
    (0x02909E, i_02909e),
    (0x0290A2, i_0290a2),
    (0x0290A6, i_0290a6),
    (0x0290AA, i_0290aa),
    (0x0290AC, i_0290ac),
    (0x0290AE, i_0290ae),
    (0x0290B2, i_0290b2),
    (0x0290B4, i_0290b4),
    (0x0290B6, i_0290b6),
    (0x0290BC, i_0290bc),
    (0x0290C2, i_0290c2),
    (0x0290C6, i_0290c6),
    (0x0290CA, i_0290ca),
    (0x0290CE, i_0290ce),
    (0x0290D2, i_0290d2),
    (0x0290D6, i_0290d6),
    (0x0290D8, i_0290d8),
    (0x0290DA, i_0290da),
    (0x0290DE, i_0290de),
    (0x0290E0, i_0290e0),
    (0x0290E2, i_0290e2),
    (0x0290E8, i_0290e8),
    (0x0290EE, i_0290ee),
    (0x0290F2, i_0290f2),
    (0x029122, i_029122),
    (0x029126, i_029126),
    (0x02912A, i_02912a),
    (0x02912C, i_02912c),
    (0x029130, i_029130),
    (0x029136, i_029136),
    (0x02913A, i_02913a),
    (0x02913E, i_02913e),
    (0x029144, i_029144),
    (0x02914A, i_02914a),
    (0x02914E, i_02914e),
    (0x029152, i_029152),
    (0x029158, i_029158),
    (0x02915E, i_02915e),
    (0x029162, i_029162),
    (0x029196, i_029196),
    (0x02919A, i_02919a),
    (0x02919E, i_02919e),
    (0x0291A2, i_0291a2),
    (0x0291A4, i_0291a4),
    (0x0291A6, i_0291a6),
    (0x0291AA, i_0291aa),
    (0x0291AC, i_0291ac),
    (0x0291AE, i_0291ae),
    (0x0291B2, i_0291b2),
    (0x0291B6, i_0291b6),
    (0x0291BC, i_0291bc),
    (0x0291C0, i_0291c0),
    (0x029220, i_029220),
    (0x029224, i_029224),
    (0x029228, i_029228),
    (0x02922C, i_02922c),
    (0x02922E, i_02922e),
    (0x029230, i_029230),
    (0x029234, i_029234),
    (0x029236, i_029236),
    (0x029238, i_029238),
    (0x02923C, i_02923c),
    (0x029240, i_029240),
    (0x029246, i_029246),
    (0x02924A, i_02924a),
    (0x0292AA, i_0292aa),
    (0x0292AE, i_0292ae),
    (0x0292B2, i_0292b2),
    (0x0292B6, i_0292b6),
    (0x0292B8, i_0292b8),
    (0x0292BA, i_0292ba),
    (0x0292BE, i_0292be),
    (0x0292C0, i_0292c0),
    (0x0292C2, i_0292c2),
    (0x0292C6, i_0292c6),
    (0x0292CA, i_0292ca),
    (0x0292D0, i_0292d0),
    (0x0292D4, i_0292d4),
    (0x029334, i_029334),
    (0x029338, i_029338),
    (0x02933C, i_02933c),
    (0x029342, i_029342),
    (0x029348, i_029348),
    (0x02934C, i_02934c),
    (0x029352, i_029352),
    (0x029358, i_029358),
    (0x02935C, i_02935c),
    (0x029360, i_029360),
    (0x029368, i_029368),
    (0x02936E, i_02936e),
    (0x029372, i_029372),
    (0x02942A, i_02942a),
    (0x02942E, i_02942e),
    (0x029436, i_029436),
    (0x02943A, i_02943a),
    (0x029442, i_029442),
    (0x029446, i_029446),
    (0x02944A, i_02944a),
    (0x02944E, i_02944e),
    (0x029454, i_029454),
    (0x029458, i_029458),
    (0x02945C, i_02945c),
    (0x029462, i_029462),
    (0x029468, i_029468),
    (0x02946C, i_02946c),
    (0x029470, i_029470),
    (0x029472, i_029472),
    (0x029476, i_029476),
    (0x02947A, i_02947a),
    (0x02947E, i_02947e),
    (0x029482, i_029482),
    (0x029484, i_029484),
    (0x029486, i_029486),
    (0x02948A, i_02948a),
    (0x02948C, i_02948c),
    (0x02948E, i_02948e),
    (0x029492, i_029492),
    (0x029498, i_029498),
    (0x02949C, i_02949c),
    (0x0294A0, i_0294a0),
    (0x0294A6, i_0294a6),
    (0x0294AA, i_0294aa),
    (0x0294AE, i_0294ae),
    (0x0294B0, i_0294b0),
    (0x0294B4, i_0294b4),
    (0x0294B8, i_0294b8),
    (0x0294BC, i_0294bc),
    (0x0294C8, i_0294c8),
    (0x0294CA, i_0294ca),
    (0x0294CE, i_0294ce),
    (0x0294D0, i_0294d0),
    (0x0294D4, i_0294d4),
    (0x0294D8, i_0294d8),
    (0x0294DC, i_0294dc),
    (0x0294E0, i_0294e0),
    (0x0294E4, i_0294e4),
    (0x0294E6, i_0294e6),
    (0x0294EA, i_0294ea),
    (0x0294EE, i_0294ee),
    (0x0294F2, i_0294f2),
    (0x0294F8, i_0294f8),
    (0x0294FC, i_0294fc),
    (0x029502, i_029502),
    (0x029506, i_029506),
    (0x02950C, i_02950c),
    (0x029510, i_029510),
    (0x029516, i_029516),
    (0x02951A, i_02951a),
    (0x02951E, i_02951e),
    (0x029524, i_029524),
    (0x029528, i_029528),
    (0x02952C, i_02952c),
    (0x029530, i_029530),
    (0x029534, i_029534),
    (0x029538, i_029538),
    (0x02953C, i_02953c),
    (0x029540, i_029540),
    (0x029544, i_029544),
    (0x02954A, i_02954a),
    (0x02954E, i_02954e),
    (0x029552, i_029552),
    (0x029556, i_029556),
    (0x02955A, i_02955a),
    (0x02955E, i_02955e),
    (0x029560, i_029560),
    (0x029564, i_029564),
    (0x029582, i_029582),
    (0x029586, i_029586),
    (0x029588, i_029588),
    (0x029590, i_029590),
    (0x0295A6, i_0295a6),
    (0x0295AC, i_0295ac),
    (0x0295B0, i_0295b0),
    (0x0295B2, i_0295b2),
    (0x02A0F8, i_02a0f8),
    (0x02A0FE, i_02a0fe),
    (0x02A102, i_02a102),
    (0x02A108, i_02a108),
    (0x02A10E, i_02a10e),
    (0x02A114, i_02a114),
    (0x02A11A, i_02a11a),
    (0x02A120, i_02a120),
    (0x02A126, i_02a126),
    (0x02A12C, i_02a12c),
    (0x02A132, i_02a132),
    (0x02A138, i_02a138),
    (0x02A13E, i_02a13e),
    (0x02A144, i_02a144),
    (0x02A14A, i_02a14a),
    (0x02A150, i_02a150),
    (0x02A154, i_02a154),
    (0x02A15C, i_02a15c),
    (0x02A162, i_02a162),
    (0x02A168, i_02a168),
    (0x02A16C, i_02a16c),
    (0x02A170, i_02a170),
    (0x02A174, i_02a174),
    (0x02A178, i_02a178),
    (0x02A17C, i_02a17c),
    (0x02A182, i_02a182),
    (0x02A188, i_02a188),
    (0x02A18E, i_02a18e),
    (0x02A194, i_02a194),
    (0x02A19A, i_02a19a),
    (0x02A1A0, i_02a1a0),
    (0x02A1A8, i_02a1a8),
    (0x02A24A, i_02a24a),
    (0x02A250, i_02a250),
    (0x02A25C, i_02a25c),
    (0x02A264, i_02a264),
    (0x02A268, i_02a268),
    (0x02A26C, i_02a26c),
    (0x02A270, i_02a270),
    (0x02A274, i_02a274),
    (0x02A276, i_02a276),
    (0x02A27C, i_02a27c),
    (0x02A280, i_02a280),
    (0x02A284, i_02a284),
    (0x02A288, i_02a288),
    (0x02A28C, i_02a28c),
    (0x02A2AC, i_02a2ac),
    (0x02A2B2, i_02a2b2),
    (0x02A2B4, i_02a2b4),
    (0x02A2B8, i_02a2b8),
    (0x02A310, i_02a310),
    (0x02A314, i_02a314),
    (0x02A318, i_02a318),
    (0x02A31C, i_02a31c),
    (0x02A322, i_02a322),
    (0x02A326, i_02a326),
    (0x02A328, i_02a328),
    (0x02A32C, i_02a32c),
    (0x02A32E, i_02a32e),
    (0x02A332, i_02a332),
    (0x02A336, i_02a336),
    (0x02A338, i_02a338),
    (0x02A34E, i_02a34e),
    (0x02A352, i_02a352),
    (0x02A354, i_02a354),
    (0x02A358, i_02a358),
    (0x02A35C, i_02a35c),
    (0x02A35E, i_02a35e),
    (0x02A46C, i_02a46c),
    (0x02A472, i_02a472),
    (0x02A476, i_02a476),
    (0x02A478, i_02a478),
    (0x02A47E, i_02a47e),
    (0x02A482, i_02a482),
    (0x02A488, i_02a488),
    (0x02A48C, i_02a48c),
    (0x02A492, i_02a492),
    (0x02A496, i_02a496),
    (0x02A49A, i_02a49a),
    (0x02A4A0, i_02a4a0),
    (0x02A4A2, i_02a4a2),
    (0x02A4A6, i_02a4a6),
    (0x02A4F0, i_02a4f0),
    (0x02A4F4, i_02a4f4),
    (0x02A4F8, i_02a4f8),
    (0x02A4FC, i_02a4fc),
    (0x02A500, i_02a500),
    (0x02A502, i_02a502),
    (0x02A506, i_02a506),
    (0x02A50A, i_02a50a),
    (0x02A50E, i_02a50e),
    (0x02A512, i_02a512),
    (0x02A516, i_02a516),
    (0x02A59A, i_02a59a),
    (0x02A59E, i_02a59e),
    (0x02A5A2, i_02a5a2),
    (0x02A5A6, i_02a5a6),
    (0x02A5A8, i_02a5a8),
    (0x02A5AC, i_02a5ac),
    (0x02A5CC, i_02a5cc),
    (0x02A5D2, i_02a5d2),
    (0x02A5D4, i_02a5d4),
    (0x02A5DA, i_02a5da),
    (0x02A5DE, i_02a5de),
    (0x02A5E2, i_02a5e2),
    (0x02A5E6, i_02a5e6),
    (0x02A5EA, i_02a5ea),
    (0x02A5EE, i_02a5ee),
    (0x02A5F2, i_02a5f2),
    (0x02A5F8, i_02a5f8),
    (0x02A5FA, i_02a5fa),
    (0x02A5FE, i_02a5fe),
    (0x02A604, i_02a604),
    (0x02A664, i_02a664),
    (0x02A66A, i_02a66a),
    (0x02A66E, i_02a66e),
    (0x02A674, i_02a674),
    (0x02A678, i_02a678),
    (0x02A67E, i_02a67e),
    (0x02A682, i_02a682),
    (0x02A684, i_02a684),
    (0x02A688, i_02a688),
    (0x02A68A, i_02a68a),
    (0x02A690, i_02a690),
    (0x02A696, i_02a696),
    (0x02A69A, i_02a69a),
    (0x02A6A0, i_02a6a0),
    (0x02A6A4, i_02a6a4),
    (0x02A6AA, i_02a6aa),
    (0x02A6B0, i_02a6b0),
    (0x02A6B4, i_02a6b4),
    (0x02A6B6, i_02a6b6),
    (0x02A6B8, i_02a6b8),
    (0x02A6BC, i_02a6bc),
    (0x02A6BE, i_02a6be),
    (0x02A6C2, i_02a6c2),
    (0x02A6C6, i_02a6c6),
    (0x02A6CC, i_02a6cc),
    (0x02A6D4, i_02a6d4),
    (0x02A6D8, i_02a6d8),
    (0x02A6E0, i_02a6e0),
    (0x02A6E4, i_02a6e4),
    (0x02A6EA, i_02a6ea),
    (0x02A6F0, i_02a6f0),
    (0x02A6F4, i_02a6f4),
    (0x02A6F8, i_02a6f8),
    (0x02A6FC, i_02a6fc),
    (0x02A702, i_02a702),
    (0x02A706, i_02a706),
    (0x02A70A, i_02a70a),
    (0x02A70C, i_02a70c),
    (0x02A710, i_02a710),
    (0x02A712, i_02a712),
    (0x02A718, i_02a718),
    (0x02A71A, i_02a71a),
    (0x02A71E, i_02a71e),
    (0x02A720, i_02a720),
    (0x02A726, i_02a726),
    (0x02A72C, i_02a72c),
    (0x02A732, i_02a732),
    (0x02A736, i_02a736),
    (0x02A73C, i_02a73c),
    (0x02A740, i_02a740),
    (0x02A746, i_02a746),
    (0x02A74A, i_02a74a),
    (0x02A750, i_02a750),
    (0x02A7D8, i_02a7d8),
    (0x02A7DE, i_02a7de),
    (0x02A7E4, i_02a7e4),
    (0x02A7E8, i_02a7e8),
    (0x02A7EE, i_02a7ee),
    (0x02A7F4, i_02a7f4),
    (0x02A7FA, i_02a7fa),
    (0x02A7FE, i_02a7fe),
    (0x02A804, i_02a804),
    (0x02A808, i_02a808),
    (0x02A80E, i_02a80e),
    (0x02A814, i_02a814),
    (0x02A818, i_02a818),
    (0x02A81C, i_02a81c),
    (0x02A81E, i_02a81e),
    (0x02A822, i_02a822),
    (0x02A824, i_02a824),
    (0x02A82A, i_02a82a),
    (0x02A830, i_02a830),
    (0x02A834, i_02a834),
    (0x02A83A, i_02a83a),
    (0x02A840, i_02a840),
    (0x02A844, i_02a844),
    (0x02A84A, i_02a84a),
    (0x02A84E, i_02a84e),
    (0x02A852, i_02a852),
    (0x02A854, i_02a854),
    (0x02A858, i_02a858),
    (0x02A85A, i_02a85a),
    (0x02A860, i_02a860),
    (0x02A866, i_02a866),
    (0x02A86A, i_02a86a),
    (0x02A870, i_02a870),
    (0x02A876, i_02a876),
    (0x02A878, i_02a878),
    (0x02A87E, i_02a87e),
    (0x02A884, i_02a884),
    (0x02A888, i_02a888),
    (0x02A88E, i_02a88e),
    (0x02A894, i_02a894),
    (0x02A898, i_02a898),
    (0x02A89C, i_02a89c),
    (0x02A8A0, i_02a8a0),
    (0x02A8A4, i_02a8a4),
    (0x02A8A8, i_02a8a8),
    (0x02A8AE, i_02a8ae),
    (0x02A8B2, i_02a8b2),
    (0x02A8B4, i_02a8b4),
    (0x02A8BA, i_02a8ba),
    (0x02A8BE, i_02a8be),
    (0x02A8C0, i_02a8c0),
    (0x02A8C6, i_02a8c6),
    (0x02A8CA, i_02a8ca),
    (0x02A8D0, i_02a8d0),
    (0x02A8D6, i_02a8d6),
    (0x02A8DA, i_02a8da),
    (0x02A8DE, i_02a8de),
    (0x02A8E4, i_02a8e4),
    (0x02A8E8, i_02a8e8),
    (0x02A8EC, i_02a8ec),
    (0x02A8F0, i_02a8f0),
    (0x02A8F4, i_02a8f4),
    (0x02A8F6, i_02a8f6),
    (0x02A8FA, i_02a8fa),
    (0x02A8FE, i_02a8fe),
    (0x02A902, i_02a902),
    (0x02A906, i_02a906),
    (0x02A908, i_02a908),
    (0x02A90C, i_02a90c),
    (0x02A910, i_02a910),
    (0x02A914, i_02a914),
    (0x02A918, i_02a918),
    (0x02A91C, i_02a91c),
    (0x02A922, i_02a922),
    (0x02A928, i_02a928),
    (0x02A92E, i_02a92e),
    (0x02A932, i_02a932),
    (0x02A934, i_02a934),
    (0x02A93A, i_02a93a),
    (0x02A940, i_02a940),
    (0x02A946, i_02a946),
    (0x02A94C, i_02a94c),
    (0x02A952, i_02a952),
    (0x02A956, i_02a956),
    (0x02A958, i_02a958),
    (0x02A95C, i_02a95c),
    (0x02A960, i_02a960),
    (0x02A964, i_02a964),
    (0x02A968, i_02a968),
    (0x02A96C, i_02a96c),
    (0x02A970, i_02a970),
    (0x02A972, i_02a972),
    (0x02A974, i_02a974),
    (0x02A976, i_02a976),
    (0x02A97A, i_02a97a),
    (0x02A97C, i_02a97c),
    (0x02A980, i_02a980),
    (0x02A982, i_02a982),
    (0x02A986, i_02a986),
    (0x02A98A, i_02a98a),
    (0x02A98E, i_02a98e),
    (0x02A992, i_02a992),
    (0x02A996, i_02a996),
    (0x02A99A, i_02a99a),
    (0x02A99E, i_02a99e),
    (0x02A9A0, i_02a9a0),
    (0x02A9CE, i_02a9ce),
    (0x02A9D2, i_02a9d2),
    (0x02A9D4, i_02a9d4),
    (0x02A9D8, i_02a9d8),
    (0x02A9DC, i_02a9dc),
    (0x02A9E0, i_02a9e0),
    (0x02A9E4, i_02a9e4),
    (0x02A9E8, i_02a9e8),
    (0x02A9EC, i_02a9ec),
    (0x02A9F0, i_02a9f0),
    (0x02A9F4, i_02a9f4),
    (0x02A9F6, i_02a9f6),
    (0x02A9FA, i_02a9fa),
    (0x02A9FC, i_02a9fc),
    (0x02AA00, i_02aa00),
    (0x02AA02, i_02aa02),
    (0x02AA06, i_02aa06),
    (0x02AA08, i_02aa08),
    (0x02AA0C, i_02aa0c),
    (0x02AA0E, i_02aa0e),
    (0x02AA12, i_02aa12),
    (0x02AA16, i_02aa16),
    (0x02AA1A, i_02aa1a),
    (0x02AA1E, i_02aa1e),
    (0x02AA22, i_02aa22),
    (0x02AA24, i_02aa24),
    (0x02AA2A, i_02aa2a),
    (0x02AA30, i_02aa30),
    (0x02AA32, i_02aa32),
    (0x02AA38, i_02aa38),
    (0x02AA3C, i_02aa3c),
    (0x02AA42, i_02aa42),
    (0x02AA46, i_02aa46),
    (0x02AA4C, i_02aa4c),
    (0x02AA50, i_02aa50),
    (0x02AA54, i_02aa54),
    (0x02AA58, i_02aa58),
    (0x02AA5C, i_02aa5c),
    (0x02AA60, i_02aa60),
    (0x02AA62, i_02aa62),
    (0x02AA66, i_02aa66),
    (0x02AA68, i_02aa68),
    (0x02AA6C, i_02aa6c),
    (0x02AA70, i_02aa70),
    (0x02AA74, i_02aa74),
    (0x02AA78, i_02aa78),
    (0x02AA7C, i_02aa7c),
    (0x02AA7E, i_02aa7e),
    (0x02AA80, i_02aa80),
    (0x02AA84, i_02aa84),
    (0x02AA88, i_02aa88),
    (0x02AA8A, i_02aa8a),
    (0x02AA8C, i_02aa8c),
    (0x02AA90, i_02aa90),
    (0x02AA94, i_02aa94),
    (0x02AA98, i_02aa98),
    (0x02AA9A, i_02aa9a),
    (0x02AA9C, i_02aa9c),
    (0x02AAA0, i_02aaa0),
    (0x02AAA2, i_02aaa2),
    (0x02AAA4, i_02aaa4),
    (0x02AAA6, i_02aaa6),
    (0x02AAA8, i_02aaa8),
    (0x02AAAA, i_02aaaa),
    (0x02AAAE, i_02aaae),
    (0x02AAC0, i_02aac0),
    (0x02AAC6, i_02aac6),
    (0x02AACA, i_02aaca),
    (0x02AACE, i_02aace),
    (0x02AAD2, i_02aad2),
    (0x02AAD6, i_02aad6),
    (0x02AADA, i_02aada),
    (0x02AADE, i_02aade),
    (0x02AAE2, i_02aae2),
    (0x02AAE4, i_02aae4),
    (0x02AAE8, i_02aae8),
    (0x02AAF0, i_02aaf0),
    (0x02AAF8, i_02aaf8),
    (0x02AAFC, i_02aafc),
    (0x02AB02, i_02ab02),
    (0x02AB06, i_02ab06),
    (0x02AB0E, i_02ab0e),
    (0x02AB12, i_02ab12),
    (0x02AB18, i_02ab18),
    (0x02AB1C, i_02ab1c),
    (0x02AB22, i_02ab22),
    (0x02AB26, i_02ab26),
    (0x02AB28, i_02ab28),
    (0x02AB2C, i_02ab2c),
    (0x02AB2E, i_02ab2e),
    (0x02AB34, i_02ab34),
    (0x02AB36, i_02ab36),
    (0x02AB3A, i_02ab3a),
    (0x02AB86, i_02ab86),
    (0x02AB8E, i_02ab8e),
    (0x02AB92, i_02ab92),
    (0x02AB98, i_02ab98),
    (0x02AB9C, i_02ab9c),
    (0x02ABA4, i_02aba4),
    (0x02ABA8, i_02aba8),
    (0x02ABAE, i_02abae),
    (0x02ABB2, i_02abb2),
    (0x02ABB8, i_02abb8),
    (0x02ABC0, i_02abc0),
    (0x02ABC4, i_02abc4),
    (0x02ABC6, i_02abc6),
    (0x02ABCC, i_02abcc),
    (0x02ABD0, i_02abd0),
    (0x02ABD2, i_02abd2),
    (0x02ABD8, i_02abd8),
    (0x02ABDE, i_02abde),
    (0x02ABE2, i_02abe2),
    (0x02ABE6, i_02abe6),
    (0x02ABEA, i_02abea),
    (0x02ABEE, i_02abee),
    (0x02ABF0, i_02abf0),
    (0x02ABF6, i_02abf6),
    (0x02ABFC, i_02abfc),
    (0x02AC00, i_02ac00),
    (0x02AC04, i_02ac04),
    (0x02AC08, i_02ac08),
    (0x02AC0C, i_02ac0c),
    (0x02AC0E, i_02ac0e),
    (0x02AC14, i_02ac14),
    (0x02AC18, i_02ac18),
    (0x02AC1A, i_02ac1a),
    (0x02AC1E, i_02ac1e),
    (0x02AC24, i_02ac24),
    (0x02AC28, i_02ac28),
    (0x02AC2A, i_02ac2a),
    (0x02AC2E, i_02ac2e),
    (0x02AC32, i_02ac32),
    (0x02AC38, i_02ac38),
    (0x02AC3C, i_02ac3c),
    (0x02AC40, i_02ac40),
    (0x02AC44, i_02ac44),
    (0x02AC46, i_02ac46),
    (0x02AC4A, i_02ac4a),
    (0x02AC4C, i_02ac4c),
    (0x02AC52, i_02ac52),
    (0x02AC56, i_02ac56),
    (0x02AC5A, i_02ac5a),
    (0x02AC5C, i_02ac5c),
    (0x02AC60, i_02ac60),
    (0x02AC64, i_02ac64),
    (0x02AC68, i_02ac68),
    (0x02AC6A, i_02ac6a),
    (0x02AC70, i_02ac70),
    (0x02AC74, i_02ac74),
    (0x02AC78, i_02ac78),
    (0x02AC7A, i_02ac7a),
    (0x02AC7E, i_02ac7e),
    (0x02AC80, i_02ac80),
    (0x02AC86, i_02ac86),
    (0x02AC8A, i_02ac8a),
    (0x02AC90, i_02ac90),
    (0x02AC94, i_02ac94),
    (0x02AC98, i_02ac98),
    (0x02AC9C, i_02ac9c),
    (0x02ACA0, i_02aca0),
    (0x02ACA2, i_02aca2),
    (0x02ACA6, i_02aca6),
    (0x02ACAC, i_02acac),
    (0x02ACB2, i_02acb2),
    (0x02ACB6, i_02acb6),
    (0x02ACB8, i_02acb8),
    (0x02ACC0, i_02acc0),
    (0x02ACC4, i_02acc4),
    (0x02ACC8, i_02acc8),
    (0x02ACCE, i_02acce),
    (0x02ACD2, i_02acd2),
    (0x02ACD8, i_02acd8),
    (0x02ACDC, i_02acdc),
    (0x02ACE2, i_02ace2),
    (0x02ACE6, i_02ace6),
    (0x02ACEC, i_02acec),
    (0x02ACF0, i_02acf0),
    (0x02ACF4, i_02acf4),
    (0x02ACF6, i_02acf6),
    (0x02ACFA, i_02acfa),
    (0x02ACFC, i_02acfc),
    (0x02AD02, i_02ad02),
    (0x02AD08, i_02ad08),
    (0x02AD0C, i_02ad0c),
    (0x02AD12, i_02ad12),
    (0x02AD16, i_02ad16),
    (0x02AD1C, i_02ad1c),
    (0x02AD20, i_02ad20),
    (0x02AD26, i_02ad26),
    (0x02AD2A, i_02ad2a),
    (0x02AD2E, i_02ad2e),
    (0x02AD30, i_02ad30),
    (0x02AD34, i_02ad34),
    (0x02AE50, i_02ae50),
    (0x02AE54, i_02ae54),
    (0x02AE58, i_02ae58),
    (0x02AE5C, i_02ae5c),
    (0x02AE5E, i_02ae5e),
    (0x02AE62, i_02ae62),
    (0x02AE66, i_02ae66),
    (0x02AE6A, i_02ae6a),
    (0x02AE6E, i_02ae6e),
    (0x02AE70, i_02ae70),
    (0x02AE74, i_02ae74),
    (0x02AE7A, i_02ae7a),
    (0x02AE7C, i_02ae7c),
    (0x02AE82, i_02ae82),
    (0x02AE88, i_02ae88),
    (0x02AE8E, i_02ae8e),
    (0x02AF64, i_02af64),
    (0x02AF68, i_02af68),
    (0x02AF6C, i_02af6c),
    (0x02AF72, i_02af72),
    (0x02AF78, i_02af78),
    (0x02AF7E, i_02af7e),
    (0x02AF84, i_02af84),
    (0x02AFAC, i_02afac),
    (0x02AFB2, i_02afb2),
    (0x02AFB8, i_02afb8),
    (0x02AFBC, i_02afbc),
    (0x02AFC2, i_02afc2),
    (0x02AFC8, i_02afc8),
    (0x02AFCE, i_02afce),
    (0x02AFD4, i_02afd4),
    (0x02AFD8, i_02afd8),
    (0x02AFDC, i_02afdc),
    (0x02AFE0, i_02afe0),
    (0x02AFE4, i_02afe4),
    (0x02AFE8, i_02afe8),
    (0x02AFEC, i_02afec),
    (0x02AFF0, i_02aff0),
    (0x02AFF4, i_02aff4),
    (0x02AFF8, i_02aff8),
    (0x02AFFE, i_02affe),
    (0x02B002, i_02b002),
    (0x02B004, i_02b004),
    (0x02B00A, i_02b00a),
    (0x02B00E, i_02b00e),
    (0x02B014, i_02b014),
    (0x02B018, i_02b018),
    (0x02B01C, i_02b01c),
    (0x02B020, i_02b020),
    (0x02B024, i_02b024),
    (0x02B028, i_02b028),
    (0x02B02C, i_02b02c),
    (0x02B02E, i_02b02e),
    (0x02B032, i_02b032),
    (0x02B038, i_02b038),
    (0x02B03E, i_02b03e),
    (0x02B044, i_02b044),
    (0x02B048, i_02b048),
    (0x02B04C, i_02b04c),
    (0x02B04E, i_02b04e),
    (0x02B052, i_02b052),
    (0x02B056, i_02b056),
    (0x02B05A, i_02b05a),
    (0x02B05C, i_02b05c),
    (0x02B05E, i_02b05e),
    (0x02B064, i_02b064),
    (0x02B06A, i_02b06a),
    (0x02B06E, i_02b06e),
    (0x02B072, i_02b072),
    (0x02B076, i_02b076),
    (0x02B07A, i_02b07a),
    (0x02B07E, i_02b07e),
    (0x02B082, i_02b082),
    (0x02B086, i_02b086),
    (0x02B08A, i_02b08a),
    (0x02B08E, i_02b08e),
    (0x02B092, i_02b092),
    (0x02B098, i_02b098),
    (0x02B09C, i_02b09c),
    (0x02B09E, i_02b09e),
    (0x02B0A4, i_02b0a4),
    (0x02B0AA, i_02b0aa),
    (0x02B0AE, i_02b0ae),
    (0x02B0B2, i_02b0b2),
    (0x02B0B6, i_02b0b6),
    (0x02B0BC, i_02b0bc),
    (0x02B0C0, i_02b0c0),
    (0x02B0C4, i_02b0c4),
    (0x02B0C8, i_02b0c8),
    (0x02B0CC, i_02b0cc),
    (0x02B0CE, i_02b0ce),
    (0x02B0D2, i_02b0d2),
    (0x02B0D6, i_02b0d6),
    (0x02B0DA, i_02b0da),
    (0x02B0DE, i_02b0de),
    (0x02B0E2, i_02b0e2),
    (0x02B0E4, i_02b0e4),
    (0x02B0E8, i_02b0e8),
    (0x02B0EE, i_02b0ee),
    (0x02B0F4, i_02b0f4),
    (0x02B0F8, i_02b0f8),
    (0x02B0FC, i_02b0fc),
    (0x02B100, i_02b100),
    (0x02B104, i_02b104),
    (0x02B108, i_02b108),
    (0x02B10C, i_02b10c),
    (0x02B110, i_02b110),
    (0x02B114, i_02b114),
    (0x02B118, i_02b118),
    (0x02B11C, i_02b11c),
    (0x02B120, i_02b120),
    (0x02B122, i_02b122),
    (0x02B124, i_02b124),
    (0x02B126, i_02b126),
    (0x02B12A, i_02b12a),
    (0x02B12E, i_02b12e),
    (0x02B132, i_02b132),
    (0x02B136, i_02b136),
    (0x02B13A, i_02b13a),
    (0x02B13E, i_02b13e),
    (0x02B144, i_02b144),
    (0x02B146, i_02b146),
    (0x02B14A, i_02b14a),
    (0x02B150, i_02b150),
    (0x02B154, i_02b154),
    (0x02B15A, i_02b15a),
    (0x02B15E, i_02b15e),
    (0x02B164, i_02b164),
    (0x02B262, i_02b262),
    (0x02B38C, i_02b38c),
    (0x02B392, i_02b392),
    (0x02B396, i_02b396),
    (0x02B39A, i_02b39a),
    (0x02B3A0, i_02b3a0),
    (0x02B3A2, i_02b3a2),
    (0x02B3A6, i_02b3a6),
    (0x02B3AC, i_02b3ac),
    (0x02B3B0, i_02b3b0),
    (0x02B3B6, i_02b3b6),
    (0x02B3BA, i_02b3ba),
    (0x02B3BE, i_02b3be),
    (0x02B3C2, i_02b3c2),
    (0x02B3C8, i_02b3c8),
    (0x02B3CC, i_02b3cc),
    (0x02B3CE, i_02b3ce),
    (0x02B3D2, i_02b3d2),
    (0x02B3DA, i_02b3da),
    (0x02B3DE, i_02b3de),
    (0x02B3E0, i_02b3e0),
    (0x02B3E4, i_02b3e4),
    (0x02B3E8, i_02b3e8),
    (0x02B3EE, i_02b3ee),
    (0x02B3F2, i_02b3f2),
    (0x02B3F6, i_02b3f6),
    (0x02B3FA, i_02b3fa),
    (0x02B3FE, i_02b3fe),
    (0x02B402, i_02b402),
    (0x02B404, i_02b404),
    (0x02B408, i_02b408),
    (0x02B40C, i_02b40c),
    (0x02B410, i_02b410),
    (0x02B414, i_02b414),
    (0x02B418, i_02b418),
    (0x02B41A, i_02b41a),
    (0x02B41E, i_02b41e),
    (0x02B422, i_02b422),
    (0x02B426, i_02b426),
    (0x02B428, i_02b428),
    (0x02B42C, i_02b42c),
    (0x02B430, i_02b430),
    (0x02B434, i_02b434),
    (0x02B438, i_02b438),
    (0x02B43C, i_02b43c),
    (0x02B442, i_02b442),
    (0x02B448, i_02b448),
    (0x02B44E, i_02b44e),
    (0x02B452, i_02b452),
    (0x02B456, i_02b456),
    (0x02B45A, i_02b45a),
    (0x02B45C, i_02b45c),
    (0x02B460, i_02b460),
    (0x02B464, i_02b464),
    (0x02B46A, i_02b46a),
    (0x02B470, i_02b470),
    (0x02B474, i_02b474),
    (0x02B478, i_02b478),
    (0x02B47C, i_02b47c),
    (0x02B47E, i_02b47e),
    (0x02B482, i_02b482),
    (0x02B486, i_02b486),
    (0x02B48A, i_02b48a),
    (0x02B48C, i_02b48c),
    (0x02B490, i_02b490),
    (0x02B494, i_02b494),
    (0x02B498, i_02b498),
    (0x02B49A, i_02b49a),
    (0x02B49E, i_02b49e),
    (0x02B4A2, i_02b4a2),
    (0x02B4A8, i_02b4a8),
    (0x02B4AE, i_02b4ae),
    (0x02B4B4, i_02b4b4),
    (0x02B4BA, i_02b4ba),
    (0x02B4C0, i_02b4c0),
    (0x02B4C4, i_02b4c4),
    (0x02B4C8, i_02b4c8),
    (0x02C908, i_02c908),
    (0x02C90E, i_02c90e),
    (0x02C912, i_02c912),
    (0x02C916, i_02c916),
    (0x02C91C, i_02c91c),
    (0x02C91E, i_02c91e),
    (0x02C922, i_02c922),
    (0x02C928, i_02c928),
    (0x02C92C, i_02c92c),
    (0x02C932, i_02c932),
    (0x02C936, i_02c936),
    (0x02C93A, i_02c93a),
    (0x02C93E, i_02c93e),
    (0x02C944, i_02c944),
    (0x02C948, i_02c948),
    (0x02C94A, i_02c94a),
    (0x02C94E, i_02c94e),
    (0x02C956, i_02c956),
    (0x02C95A, i_02c95a),
    (0x02C95C, i_02c95c),
    (0x02C960, i_02c960),
    (0x02C964, i_02c964),
    (0x02C968, i_02c968),
    (0x02C96E, i_02c96e),
    (0x02C972, i_02c972),
    (0x02C976, i_02c976),
    (0x02C978, i_02c978),
    (0x02C97E, i_02c97e),
    (0x02C982, i_02c982),
    (0x02C986, i_02c986),
    (0x02C98A, i_02c98a),
    (0x02C98C, i_02c98c),
    (0x02C990, i_02c990),
    (0x02C994, i_02c994),
    (0x02C998, i_02c998),
    (0x02C99A, i_02c99a),
    (0x02C99E, i_02c99e),
    (0x02C9A2, i_02c9a2),
    (0x02C9A8, i_02c9a8),
    (0x02C9AE, i_02c9ae),
    (0x02C9C4, i_02c9c4),
    (0x02C9CA, i_02c9ca),
    (0x02C9D0, i_02c9d0),
    (0x02CA0C, i_02ca0c),
    (0x02CA12, i_02ca12),
    (0x02CA18, i_02ca18),
    (0x02CA1E, i_02ca1e),
    (0x02CA24, i_02ca24),
    (0x02CA28, i_02ca28),
    (0x02CA2C, i_02ca2c),
    (0x02CA30, i_02ca30),
    (0x02CA34, i_02ca34),
    (0x02CA38, i_02ca38),
    (0x02CA3E, i_02ca3e),
    (0x02CA40, i_02ca40),
    (0x02CA44, i_02ca44),
    (0x02CA4A, i_02ca4a),
    (0x02CA4E, i_02ca4e),
    (0x02CA54, i_02ca54),
    (0x02CA58, i_02ca58),
    (0x02CA5C, i_02ca5c),
    (0x02CA60, i_02ca60),
    (0x02CA66, i_02ca66),
    (0x02CA6A, i_02ca6a),
    (0x02CA6C, i_02ca6c),
    (0x02CA70, i_02ca70),
    (0x02CA74, i_02ca74),
    (0x02CA76, i_02ca76),
    (0x02CA7A, i_02ca7a),
    (0x02CA7E, i_02ca7e),
    (0x02CA82, i_02ca82),
    (0x02CA86, i_02ca86),
    (0x02CA8A, i_02ca8a),
    (0x02CA8E, i_02ca8e),
    (0x02CA92, i_02ca92),
    (0x02CA96, i_02ca96),
    (0x02CA98, i_02ca98),
    (0x02CA9E, i_02ca9e),
    (0x02CAA0, i_02caa0),
    (0x02CAA4, i_02caa4),
    (0x02CAAA, i_02caaa),
    (0x02CAAE, i_02caae),
    (0x02CAB2, i_02cab2),
    (0x02CAB8, i_02cab8),
    (0x02CABC, i_02cabc),
    (0x02CAC0, i_02cac0),
    (0x02CAC4, i_02cac4),
    (0x02CAC8, i_02cac8),
    (0x02CACE, i_02cace),
    (0x02CAD2, i_02cad2),
    (0x02CAD8, i_02cad8),
    (0x02CADC, i_02cadc),
    (0x02CAE0, i_02cae0),
    (0x02CAE4, i_02cae4),
    (0x02CAE8, i_02cae8),
    (0x02CAEA, i_02caea),
    (0x02CAF0, i_02caf0),
    (0x02CAF4, i_02caf4),
    (0x02CAF8, i_02caf8),
    (0x02CAFC, i_02cafc),
    (0x02CB00, i_02cb00),
    (0x02CB06, i_02cb06),
    (0x02CB0C, i_02cb0c),
    (0x02CB10, i_02cb10),
    (0x02CB14, i_02cb14),
    (0x02CB18, i_02cb18),
    (0x02CB1A, i_02cb1a),
    (0x02CB20, i_02cb20),
    (0x02CB24, i_02cb24),
    (0x02CB28, i_02cb28),
    (0x02CB2C, i_02cb2c),
    (0x02CB30, i_02cb30),
    (0x02CB34, i_02cb34),
    (0x02CB38, i_02cb38),
    (0x02CB3C, i_02cb3c),
    (0x02CB40, i_02cb40),
    (0x02CB44, i_02cb44),
    (0x02CB48, i_02cb48),
    (0x02CB4C, i_02cb4c),
    (0x02CB4E, i_02cb4e),
    (0x02CB50, i_02cb50),
    (0x02CB52, i_02cb52),
    (0x02CB56, i_02cb56),
    (0x02CB5A, i_02cb5a),
    (0x02CB5E, i_02cb5e),
    (0x02CB62, i_02cb62),
    (0x02CB66, i_02cb66),
    (0x02CB6A, i_02cb6a),
    (0x02CB70, i_02cb70),
    (0x02CB72, i_02cb72),
    (0x02CB76, i_02cb76),
    (0x02CB7C, i_02cb7c),
    (0x02CB80, i_02cb80),
    (0x02CB86, i_02cb86),
    (0x02CB8A, i_02cb8a),
    (0x02CB90, i_02cb90),
    (0x02CB92, i_02cb92),
    (0x02CB96, i_02cb96),
    (0x02CB9A, i_02cb9a),
    (0x02CBA0, i_02cba0),
    (0x02CBA4, i_02cba4),
    (0x02CBA6, i_02cba6),
    (0x02CBAA, i_02cbaa),
    (0x02CBAE, i_02cbae),
    (0x02CBB0, i_02cbb0),
    (0x02CBB6, i_02cbb6),
    (0x02CBBA, i_02cbba),
    (0x02CBBE, i_02cbbe),
    (0x02CBC2, i_02cbc2),
    (0x02CBC8, i_02cbc8),
    (0x02CBCE, i_02cbce),
    (0x02CBD2, i_02cbd2),
    (0x02CBD6, i_02cbd6),
    (0x02CBDA, i_02cbda),
    (0x02CBE0, i_02cbe0),
    (0x02CBE6, i_02cbe6),
    (0x02CBEC, i_02cbec),
    (0x02CBF2, i_02cbf2),
    (0x02CBF8, i_02cbf8),
    (0x02CBFC, i_02cbfc),
    (0x02CC00, i_02cc00),
    (0x02DC8C, i_02dc8c),
    (0x02DC92, i_02dc92),
    (0x02DC98, i_02dc98),
    (0x02DC9E, i_02dc9e),
    (0x02DCA2, i_02dca2),
    (0x02DCA4, i_02dca4),
    (0x02DCA8, i_02dca8),
    (0x02DCBC, i_02dcbc),
    (0x02DD20, i_02dd20),
    (0x02DD26, i_02dd26),
    (0x02DD2A, i_02dd2a),
    (0x02DD30, i_02dd30),
    (0x02DD32, i_02dd32),
    (0x02DD36, i_02dd36),
    (0x02DD3C, i_02dd3c),
    (0x02DD40, i_02dd40),
    (0x02DD46, i_02dd46),
    (0x02DD4C, i_02dd4c),
    (0x02DD50, i_02dd50),
    (0x02DD52, i_02dd52),
    (0x02DD56, i_02dd56),
    (0x02DD5A, i_02dd5a),
    (0x02DD62, i_02dd62),
];
