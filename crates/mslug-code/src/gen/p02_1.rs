// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $026818: add.w $28(a6), d0
pub fn i_026818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2681C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02681C: jsr $267f8(pc)
pub fn i_02681c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26820;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26820);
    cpu.pc = 0x267F8;
    16
}
/// $026820: move.w d0, $28(a6)
pub fn i_026820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26824;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026824: move.w $2e(a6), d0
pub fn i_026824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26828;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026828: add.w $2a(a6), d0
pub fn i_026828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2682C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02682C: jsr $267f8(pc)
pub fn i_02682c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26830;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26830);
    cpu.pc = 0x267F8;
    16
}
// ===== SetTaskW_026830  ($026830, 6 B, kernel/task_setters_w.c)
/// $026830: move.w d0, $2a(a6)
pub fn i_026830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26834;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026834: rts
pub fn i_026834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26836;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveX_WallStop_026836  ($026836, 342 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026836: move.b -$1144(a5), -$1150(a5)
pub fn i_026836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2683C;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    bus.write8(ad4, v3 as u8);
    20
}
/// $02683C: move.b -$1143(a5), -$114f(a5)
pub fn i_02683c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26842;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    bus.write8(ad4, v3 as u8);
    20
}
/// $026842: clr.w -$1154(a5)
pub fn i_026842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26846;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $026846: move.w $28(a6), d2
pub fn i_026846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2684A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02684A: btst.b #$0, $69(a6)
pub fn i_02684a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26850;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $026850: beq.w $26860
pub fn i_026850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26854;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26860; return 10; }
    8
}
/// $026854: tst.w d2
pub fn i_026854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26856;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $026856: ble.w $26860
pub fn i_026856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2685A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x26860; return 10; }
    8
}
/// $02685A: clr.w d2
pub fn i_02685a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2685C;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02685C: bra.w $26872
pub fn i_02685c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26860;
    cpu.ir = 0x6000;
    cpu.pc = 0x26872;
    10
}
/// $026860: btst.b #$1, $69(a6)
pub fn i_026860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26866;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $026866: beq.w $26872
pub fn i_026866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2686A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26872; return 10; }
    8
}
/// $02686A: tst.w d2
pub fn i_02686a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2686C;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02686C: bge.w $26872
pub fn i_02686c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26870;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x26872; return 10; }
    8
}
/// $026870: clr.w d2
pub fn i_026870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26872;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026872: moveq #$0, d3
pub fn i_026872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26874;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026874: cmpi.b #$1, $106ece.l
pub fn i_026874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2687C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $02687C: bne.w $268c2
pub fn i_02687c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26880;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x268C2; return 10; }
    8
}
/// $026880: bra.w $268c2
pub fn i_026880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26884;
    cpu.ir = 0x6000;
    cpu.pc = 0x268C2;
    10
}
/// $0268C2: cmpi.b #$3, $106ece.l
pub fn i_0268c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268CA;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0268CA: bne.w $268f2
pub fn i_0268ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x268F2; return 10; }
    8
}
/// $0268CE: cmpa.l #$100440, a6
pub fn i_0268ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268D4;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0268D4: beq.w $268e6
pub fn i_0268d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x268E6; return 10; }
    8
}
/// $0268D8: cmpa.l #$1004e0, a6
pub fn i_0268d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268DE;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0268DE: beq.w $268e6
pub fn i_0268de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x268E6; return 10; }
    8
}
/// $0268E2: bra.w $268f2
pub fn i_0268e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x268F2;
    10
}
/// $0268E6: cmpi.w #$1f0, $24(a6)
pub fn i_0268e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F0, Size::Word, false);
    16
}
/// $0268EC: blt.w $268f2
pub fn i_0268ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268F0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x268F2; return 10; }
    8
}
/// $0268F0: clr.w d2
pub fn i_0268f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268F2;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0268F2: cmpa.l #$100440, a6
pub fn i_0268f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268F8;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0268F8: beq.w $26922
pub fn i_0268f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26922; return 10; }
    8
}
/// $0268FC: cmpa.l #$1004e0, a6
pub fn i_0268fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26902;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $026902: beq.w $26922
pub fn i_026902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26906;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26922; return 10; }
    8
}
/// $026906: cmpa.l #$100580, a6
pub fn i_026906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2690C;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100580;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02690C: bne.w $2691e
pub fn i_02690c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26910;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2691E; return 10; }
    8
}
/// $026910: jsr $2a276.l
pub fn i_026910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26916;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26916);
    cpu.pc = 0x2A276;
    16
}
/// $026916: bcc.w $2691e
pub fn i_026916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2691A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2691E; return 10; }
    8
}
/// $02691A: bra.w $26922
pub fn i_02691a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2691E;
    cpu.ir = 0x6000;
    cpu.pc = 0x26922;
    10
}
/// $02691E: bra.w $26926
pub fn i_02691e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26922;
    cpu.ir = 0x6000;
    cpu.pc = 0x26926;
    10
}
/// $026922: jsr $28074(pc)
pub fn i_026922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26926;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26926);
    cpu.pc = 0x28074;
    16
}
/// $026926: add.w d3, d2
pub fn i_026926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26928;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026928: moveq #$0, d0
pub fn i_026928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2692A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02692A: move.b -$1150(a5), d0
pub fn i_02692a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2692E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02692E: add.w d2, d0
pub fn i_02692e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26930;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026930: move.w d0, d2
pub fn i_026930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26932;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026932: move.b d2, -$1150(a5)
pub fn i_026932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26936;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026936: asr.w #$8, d0
pub fn i_026936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26938;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $026938: move.w d0, -$1154(a5)
pub fn i_026938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2693C;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02693C: moveq #$0, d1
pub fn i_02693c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2693E;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02693E: move.b -$114f(a5), d1
pub fn i_02693e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26942;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026942: move.w $2a(a6), d2
pub fn i_026942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26946;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026946: cmpi.b #$3, $106ece.l
pub fn i_026946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2694E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02694E: bne.w $2697e
pub fn i_02694e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26952;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2697E; return 10; }
    8
}
/// $026952: cmpa.l #$100440, a6
pub fn i_026952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26958;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $026958: beq.w $2696a
pub fn i_026958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2695C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2696A; return 10; }
    8
}
/// $02695C: cmpa.l #$1004e0, a6
pub fn i_02695c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26962;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $026962: beq.w $2696a
pub fn i_026962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26966;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2696A; return 10; }
    8
}
/// $026966: bra.w $2697e
pub fn i_026966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2696A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2697E;
    10
}
/// $02696A: cmpi.w #$1f0, $24(a6)
pub fn i_02696a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26970;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F0, Size::Word, false);
    16
}
/// $026970: blt.w $2697e
pub fn i_026970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26974;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2697E; return 10; }
    8
}
/// $026974: cmpi.w #$0, d2
pub fn i_026974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26978;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $026978: ble.w $2697e
pub fn i_026978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2697C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2697E; return 10; }
    8
}
/// $02697C: clr.w d2
pub fn i_02697c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2697E;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02697E: add.w d2, d1
pub fn i_02697e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26980;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026980: move.w d1, d2
pub fn i_026980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26982;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026982: move.b d1, -$114f(a5)
pub fn i_026982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26986;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026986: asr.w #$8, d1
pub fn i_026986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26988;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $026988: move.w d1, -$1152(a5)
pub fn i_026988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2698C;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrPcThunk_02698c  ($02698C, 6 B, thunks/jsr_pc_thunks.c)
/// $02698C: jsr $281c8(pc)
pub fn i_02698c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26990;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26990);
    cpu.pc = 0x281C8;
    16
}
/// $026990: rts
pub fn i_026990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26992;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveXY_Probe_026992  ($026992, 452 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026992: move.b -$1144(a5), -$1150(a5)
pub fn i_026992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26998;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    bus.write8(ad4, v3 as u8);
    20
}
/// $026998: move.b -$1143(a5), -$114f(a5)
pub fn i_026998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2699E;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    bus.write8(ad4, v3 as u8);
    20
}
/// $02699E: clr.w -$1154(a5)
pub fn i_02699e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269A2;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0269A2: move.w $28(a6), d2
pub fn i_0269a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269A6;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0269A6: btst.b #$0, $69(a6)
pub fn i_0269a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269AC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0269AC: beq.w $269bc
pub fn i_0269ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x269BC; return 10; }
    8
}
/// $0269B0: tst.w d2
pub fn i_0269b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269B2;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0269B2: ble.w $269bc
pub fn i_0269b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269B6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x269BC; return 10; }
    8
}
/// $0269B6: clr.w d2
pub fn i_0269b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269B8;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0269B8: bra.w $269ce
pub fn i_0269b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x269CE;
    10
}
/// $0269BC: btst.b #$1, $69(a6)
pub fn i_0269bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269C2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0269C2: beq.w $269ce
pub fn i_0269c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x269CE; return 10; }
    8
}
/// $0269C6: tst.w d2
pub fn i_0269c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269C8;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0269C8: bge.w $269ce
pub fn i_0269c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269CC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x269CE; return 10; }
    8
}
/// $0269CC: clr.w d2
pub fn i_0269cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269CE;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0269CE: moveq #$0, d3
pub fn i_0269ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269D0;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0269D0: cmpi.b #$1, $106ece.l
pub fn i_0269d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269D8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0269D8: bne.w $26a1e
pub fn i_0269d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26A1E; return 10; }
    8
}
/// $0269DC: bra.w $26a1e
pub fn i_0269dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x26A1E;
    10
}
/// $026A1E: cmpa.l #$100440, a6
pub fn i_026a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A24;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $026A24: beq.w $26a4e
pub fn i_026a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26A4E; return 10; }
    8
}
/// $026A28: cmpa.l #$1004e0, a6
pub fn i_026a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A2E;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $026A2E: beq.w $26a4e
pub fn i_026a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26A4E; return 10; }
    8
}
/// $026A32: cmpa.l #$100580, a6
pub fn i_026a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A38;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100580;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $026A38: bne.w $26a4a
pub fn i_026a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26A4A; return 10; }
    8
}
/// $026A3C: jsr $2a276.l
pub fn i_026a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26A42);
    cpu.pc = 0x2A276;
    16
}
/// $026A42: bcc.w $26a4a
pub fn i_026a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A46;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x26A4A; return 10; }
    8
}
/// $026A46: bra.w $26a4e
pub fn i_026a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A4A;
    cpu.ir = 0x6000;
    cpu.pc = 0x26A4E;
    10
}
/// $026A4A: bra.w $26a52
pub fn i_026a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x26A52;
    10
}
/// $026A4E: jsr $28074(pc)
pub fn i_026a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26A52);
    cpu.pc = 0x28074;
    16
}
/// $026A52: add.w d3, d2
pub fn i_026a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A54;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026A54: jsr $2800e(pc)
pub fn i_026a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26A58);
    cpu.pc = 0x2800E;
    16
}
/// $026A58: moveq #$0, d0
pub fn i_026a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A5A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026A5A: move.b -$1150(a5), d0
pub fn i_026a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A5E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026A5E: add.w d2, d0
pub fn i_026a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A60;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026A60: move.w d0, d2
pub fn i_026a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A62;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026A62: move.b d2, -$1150(a5)
pub fn i_026a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A66;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026A66: asr.w #$8, d0
pub fn i_026a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A68;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $026A68: move.w d0, -$1154(a5)
pub fn i_026a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A6C;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026A6C: jsr $281c8(pc)
pub fn i_026a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26A70);
    cpu.pc = 0x281C8;
    16
}
/// $026A70: move.w -$1154(a5), d1
pub fn i_026a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A74;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026A74: moveq #$0, d0
pub fn i_026a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A76;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026A76: move.w $34(a6), d2
pub fn i_026a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A7A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026A7A: cmpi.w #$ffff, d2
pub fn i_026a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A7E;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $026A7E: beq.w $26ac2
pub fn i_026a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26AC2; return 10; }
    8
}
/// $026A82: tst.w d2
pub fn i_026a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A84;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $026A84: beq.w $26ae0
pub fn i_026a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A88;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26AE0; return 10; }
    8
}
/// $026A88: bge.w $26a8e
pub fn i_026a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A8C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x26A8E; return 10; }
    8
}
/// $026A8C: moveq #$ff, d0
pub fn i_026a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A8E;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026A8E: eor.w d0, d2
pub fn i_026a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A90;
    cpu.ir = 0xB142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026A90: sub.w d0, d2
pub fn i_026a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A92;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026A92: cmpi.w #$1000, d2
pub fn i_026a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A96;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1000, Size::Word, false);
    8
}
/// $026A96: beq.w $26ae8
pub fn i_026a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26AE8; return 10; }
    8
}
/// $026A9A: cmpi.w #$400, d2
pub fn i_026a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A9E;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x400, Size::Word, false);
    8
}
/// $026A9E: beq.w $26ad4
pub fn i_026a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26AD4; return 10; }
    8
}
/// $026AA2: cmpi.w #$80, d2
pub fn i_026aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AA6;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $026AA6: beq.w $26abe
pub fn i_026aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26ABE; return 10; }
    8
}
/// $026AAA: cmpi.w #$80, d2
pub fn i_026aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AAE;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $026AAE: beq.w $26abe
pub fn i_026aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26ABE; return 10; }
    8
}
/// $026AB2: nop
pub fn i_026ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AB4;
    cpu.ir = 0x4E71;
    4
}
/// $026AB4: nop
pub fn i_026ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AB6;
    cpu.ir = 0x4E71;
    4
}
/// $026AB6: cmpi.w #$80, d2
pub fn i_026ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ABA;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $026ABA: nop
pub fn i_026aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ABC;
    cpu.ir = 0x4E71;
    4
}
/// $026ABC: trap #$f
pub fn i_026abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ABE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $026ABE: clr.w -$1154(a5)
pub fn i_026abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AC2;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $026AC2: btst.b #$3, $5b(a6)
pub fn i_026ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AC8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $026AC8: beq.w $26ad0
pub fn i_026ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ACC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26AD0; return 10; }
    8
}
/// $026ACC: clr.w -$1152(a5)
pub fn i_026acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AD0;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $026AD0: bra.w $26b08
pub fn i_026ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AD4;
    cpu.ir = 0x6000;
    cpu.pc = 0x26B08;
    10
}
/// $026AD4: add.w d0, d1
pub fn i_026ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AD6;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026AD6: eor.w d0, d1
pub fn i_026ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AD8;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026AD8: move.w d1, -$1152(a5)
pub fn i_026ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ADC;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026ADC: bra.w $26b08
pub fn i_026adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x26B08;
    10
}
/// $026AE0: clr.w -$1152(a5)
pub fn i_026ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AE4;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $026AE4: bra.w $26b08
pub fn i_026ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x26B08;
    10
}
/// $026AE8: move.b -$114f(a5), d6
pub fn i_026ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AEC;
    cpu.ir = 0x1C2D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026AEC: andi.w #$ff, d6
pub fn i_026aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AF0;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026AF0: asl.w #$6, d1
pub fn i_026af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AF2;
    cpu.ir = 0xED41;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $026AF2: add.w d0, d1
pub fn i_026af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AF4;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026AF4: eor.w d0, d1
pub fn i_026af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AF6;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026AF6: add.w d1, d6
pub fn i_026af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AF8;
    cpu.ir = 0xDC41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026AF8: move.w d6, d1
pub fn i_026af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AFA;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026AFA: move.b d6, -$114f(a5)
pub fn i_026afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AFE;
    cpu.ir = 0x1B46;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026AFE: asr.w #$8, d1
pub fn i_026afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B00;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $026B00: move.w d1, -$1152(a5)
pub fn i_026b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B04;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026B04: bra.w $26b08
pub fn i_026b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B08;
    cpu.ir = 0x6000;
    cpu.pc = 0x26B08;
    10
}
/// $026B08: cmpi.b #$3, $106ece.l
pub fn i_026b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B10;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $026B10: bne.w $26b54
pub fn i_026b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26B54; return 10; }
    8
}
/// $026B14: cmpa.l #$100440, a6
pub fn i_026b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B1A;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $026B1A: beq.w $26b2c
pub fn i_026b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26B2C; return 10; }
    8
}
/// $026B1E: cmpa.l #$1004e0, a6
pub fn i_026b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B24;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $026B24: beq.w $26b2c
pub fn i_026b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26B2C; return 10; }
    8
}
/// $026B28: bra.w $26b54
pub fn i_026b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x26B54;
    10
}
/// $026B2C: cmpi.w #$1f0, $24(a6)
pub fn i_026b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B32;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F0, Size::Word, false);
    16
}
/// $026B32: blt.w $26b54
pub fn i_026b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B36;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x26B54; return 10; }
    8
}
/// $026B36: cmpi.w #$0, -$1152(a5)
pub fn i_026b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B3C;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $026B3C: ble.w $26b54
pub fn i_026b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B40;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x26B54; return 10; }
    8
}
/// $026B40: clr.w -$1154(a5)
pub fn i_026b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B44;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $026B44: clr.w -$1152(a5)
pub fn i_026b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B48;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $026B48: move.b -$1144(a5), -$1150(a5)
pub fn i_026b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B4E;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    bus.write8(ad4, v3 as u8);
    20
}
/// $026B4E: move.b -$1143(a5), -$114f(a5)
pub fn i_026b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B54;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    bus.write8(ad4, v3 as u8);
    20
}
/// $026B54: rts
pub fn i_026b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveAndCollide_A_026b56  ($026B56, 576 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026B56: jsr $26814(pc)
pub fn i_026b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26B5A);
    cpu.pc = 0x26814;
    16
}
/// $026B5A: jsr $26992(pc)
pub fn i_026b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26B5E);
    cpu.pc = 0x26992;
    16
}
/// $026B5E: jsr $44182.l
pub fn i_026b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26B64);
    cpu.pc = 0x44182;
    16
}
/// $026B64: move.w -$1148(a5), d1
pub fn i_026b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B68;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026B68: move.w -$1146(a5), d2
pub fn i_026b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B6C;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026B6C: subq.w #$1, d2
pub fn i_026b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B6E;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026B6E: jsr $27db2(pc)
pub fn i_026b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26B72);
    cpu.pc = 0x27DB2;
    16
}
/// $026B72: add.w -$1154(a5), d1
pub fn i_026b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B76;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $026B76: add.w -$1152(a5), d2
pub fn i_026b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B7A;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $026B7A: jsr $998ca.l
pub fn i_026b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26B80);
    cpu.pc = 0x998CA;
    16
}
/// $026B80: bcc.w $26ba6
pub fn i_026b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x26BA6; return 10; }
    8
}
/// $026B84: moveq #$0, d0
pub fn i_026b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B86;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026B86: moveq #$0, d3
pub fn i_026b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B88;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026B88: moveq #$0, d4
pub fn i_026b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B8A;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026B8A: moveq #$11, d0
pub fn i_026b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B8C;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026B8C: move.l d0, d7
pub fn i_026b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B8E;
    cpu.ir = 0x2E00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026B8E: lea.l $278ba8.l, a1
pub fn i_026b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B94;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $026B94: movea.l a1, a4
pub fn i_026b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B96;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $026B96: movea.l $4(a1), a3
pub fn i_026b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B9A;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $026B9A: move.b $12(a3, d3.w), $106f31.l
pub fn i_026b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BA2;
    cpu.ir = 0x13F3;
    let ad2 = cpu.a[3].wrapping_add(0x12).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F31, v3 as u8);
    26
}
/// $026BA2: bra.w $26c82
pub fn i_026ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x26C82;
    10
}
/// $026BA6: sub.w -$1154(a5), d1
pub fn i_026ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BAA;
    cpu.ir = 0x926D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $026BAA: sub.w -$1152(a5), d2
pub fn i_026baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BAE;
    cpu.ir = 0x946D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $026BAE: cmpi.b #$1, $a(a1)
pub fn i_026bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BB4;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $026BB4: bne.w $26bc4
pub fn i_026bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26BC4; return 10; }
    8
}
/// $026BB8: tst.w $34(a6)
pub fn i_026bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BBC;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026BBC: bne.w $26bc4
pub fn i_026bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26BC4; return 10; }
    8
}
/// $026BC0: adda.w #$10, a1
pub fn i_026bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BC4;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x10;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $026BC4: jsr $27e9c(pc)
pub fn i_026bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26BC8);
    cpu.pc = 0x27E9C;
    16
}
/// $026BC8: bcs.w $26bd4
pub fn i_026bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BCC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x26BD4; return 10; }
    8
}
/// $026BCC: jsr $27e7e(pc)
pub fn i_026bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26BD0);
    cpu.pc = 0x27E7E;
    16
}
/// $026BD0: bcs.w $26bd4
pub fn i_026bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BD4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x26BD4; return 10; }
    8
}
/// $026BD4: add.w -$1154(a5), d1
pub fn i_026bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BD8;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $026BD8: add.w -$1152(a5), d2
pub fn i_026bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BDC;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $026BDC: movem.w d3-d4, -(a7)
pub fn i_026bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BE0;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    cpu.a[7] = a;
    20
}
/// $026BE0: movea.l a1, a4
pub fn i_026be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BE2;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $026BE2: jsr $27db2(pc)
pub fn i_026be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26BE6);
    cpu.pc = 0x27DB2;
    16
}
/// $026BE6: cmpi.b #$1, $a(a1)
pub fn i_026be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BEC;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $026BEC: bne.w $26c74
pub fn i_026bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BF0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26C74; return 10; }
    8
}
/// $026BF0: tst.w $34(a6)
pub fn i_026bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BF4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026BF4: bne.w $26c74
pub fn i_026bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26C74; return 10; }
    8
}
/// $026BF8: tst.w -$1154(a5)
pub fn i_026bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BFC;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026BFC: blt.w $26c14
pub fn i_026bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C00;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x26C14; return 10; }
    8
}
/// $026C00: cmpi.b #$23, d0
pub fn i_026c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C04;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x23, Size::Byte, false);
    8
}
/// $026C04: beq.w $26c28
pub fn i_026c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26C28; return 10; }
    8
}
/// $026C08: cmpi.b #$33, d0
pub fn i_026c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C0C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x33, Size::Byte, false);
    8
}
/// $026C0C: beq.w $26c28
pub fn i_026c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C10;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26C28; return 10; }
    8
}
/// $026C10: bra.w $26c70
pub fn i_026c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C14;
    cpu.ir = 0x6000;
    cpu.pc = 0x26C70;
    10
}
/// $026C14: cmpi.b #$22, d0
pub fn i_026c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C18;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x22, Size::Byte, false);
    8
}
/// $026C18: beq.w $26c28
pub fn i_026c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C1C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26C28; return 10; }
    8
}
/// $026C1C: cmpi.b #$32, d0
pub fn i_026c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C20;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x32, Size::Byte, false);
    8
}
/// $026C20: beq.w $26c28
pub fn i_026c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26C28; return 10; }
    8
}
/// $026C24: bra.w $26c70
pub fn i_026c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C28;
    cpu.ir = 0x6000;
    cpu.pc = 0x26C70;
    10
}
/// $026C28: btst.b #$5, $13(a6)
pub fn i_026c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C2E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $026C2E: beq.w $26c70
pub fn i_026c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26C70; return 10; }
    8
}
/// $026C32: tst.w -$1154(a5)
pub fn i_026c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C36;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026C36: beq.w $26c70
pub fn i_026c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26C70; return 10; }
    8
}
/// $026C3A: btst.b #$5, $5b(a6)
pub fn i_026c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C40;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $026C40: bne.w $26c66
pub fn i_026c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C44;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26C66; return 10; }
    8
}
/// $026C44: cmpi.b #$0, $6a(a6)
pub fn i_026c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C4A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $026C4A: beq.w $26c66
pub fn i_026c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26C66; return 10; }
    8
}
/// $026C4E: move.b $6a(a6), d6
pub fn i_026c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C52;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026C52: ext.w d6
pub fn i_026c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C54;
    cpu.ir = 0x4886;
    let v = cpu.d[6] as i8 as i16 as u16 as u32; cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $026C54: andi.w #$8000, d6
pub fn i_026c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C58;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026C58: move.w $28(a6), d5
pub fn i_026c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C5C;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026C5C: andi.w #$8000, d5
pub fn i_026c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C60;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026C60: eor.w d5, d6
pub fn i_026c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C62;
    cpu.ir = 0xBB46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026C62: bne.w $26c70
pub fn i_026c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26C70; return 10; }
    8
}
/// $026C66: bset.b #$3, $5a(a6)
pub fn i_026c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C6C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $026C6C: bra.w $26c74
pub fn i_026c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C70;
    cpu.ir = 0x6000;
    cpu.pc = 0x26C74;
    10
}
/// $026C70: adda.w #$10, a1
pub fn i_026c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C74;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x10;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $026C74: movea.l $4(a1), a3
pub fn i_026c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C78;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $026C78: move.b $12(a3, d3.w), -$114f(a5)
pub fn i_026c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C7E;
    cpu.ir = 0x1B73;
    let ad2 = cpu.a[3].wrapping_add(0x12).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    bus.write8(ad4, v3 as u8);
    22
}
/// $026C7E: movem.w (a7)+, d3-d4
pub fn i_026c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C82;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $026C82: tst.w -$1154(a5)
pub fn i_026c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C86;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026C86: blt.w $26cae
pub fn i_026c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C8A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x26CAE; return 10; }
    8
}
/// $026C8A: cmpi.b #$20, d0
pub fn i_026c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C8E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $026C8E: beq.w $26cd2
pub fn i_026c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26CD2; return 10; }
    8
}
/// $026C92: cmpi.b #$30, d0
pub fn i_026c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C96;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $026C96: beq.w $26cd2
pub fn i_026c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26CD2; return 10; }
    8
}
/// $026C9A: cmpi.b #$21, d0
pub fn i_026c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C9E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $026C9E: beq.w $26d1a
pub fn i_026c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D1A; return 10; }
    8
}
/// $026CA2: cmpi.b #$31, d0
pub fn i_026ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CA6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $026CA6: beq.w $26d1a
pub fn i_026ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D1A; return 10; }
    8
}
/// $026CAA: bra.w $26d24
pub fn i_026caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x26D24;
    10
}
/// $026CAE: cmpi.b #$20, d0
pub fn i_026cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CB2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $026CB2: beq.w $26d1a
pub fn i_026cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D1A; return 10; }
    8
}
/// $026CB6: cmpi.b #$30, d0
pub fn i_026cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CBA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $026CBA: beq.w $26d1a
pub fn i_026cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CBE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D1A; return 10; }
    8
}
/// $026CBE: cmpi.b #$21, d0
pub fn i_026cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CC2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $026CC2: beq.w $26cd2
pub fn i_026cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CC6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26CD2; return 10; }
    8
}
/// $026CC6: cmpi.b #$31, d0
pub fn i_026cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CCA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $026CCA: beq.w $26cd2
pub fn i_026cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CCE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26CD2; return 10; }
    8
}
/// $026CCE: bra.w $26d24
pub fn i_026cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x26D24;
    10
}
/// $026CD2: btst.b #$4, $13(a6)
pub fn i_026cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CD8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $026CD8: beq.w $26d1a
pub fn i_026cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CDC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D1A; return 10; }
    8
}
/// $026CDC: tst.w -$1154(a5)
pub fn i_026cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CE0;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026CE0: beq.w $26d1a
pub fn i_026ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CE4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D1A; return 10; }
    8
}
/// $026CE4: btst.b #$4, $5b(a6)
pub fn i_026ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CEA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $026CEA: bne.w $26d10
pub fn i_026cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26D10; return 10; }
    8
}
/// $026CEE: cmpi.b #$0, $6a(a6)
pub fn i_026cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CF4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $026CF4: beq.w $26d10
pub fn i_026cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CF8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D10; return 10; }
    8
}
/// $026CF8: move.b $6a(a6), d6
pub fn i_026cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CFC;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026CFC: ext.w d6
pub fn i_026cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CFE;
    cpu.ir = 0x4886;
    let v = cpu.d[6] as i8 as i16 as u16 as u32; cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $026CFE: andi.w #$8000, d6
pub fn i_026cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D02;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026D02: move.w $28(a6), d5
pub fn i_026d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D06;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026D06: andi.w #$8000, d5
pub fn i_026d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D0A;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026D0A: eor.w d5, d6
pub fn i_026d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D0C;
    cpu.ir = 0xBB46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026D0C: bne.w $26d1a
pub fn i_026d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26D1A; return 10; }
    8
}
/// $026D10: bset.b #$3, $5a(a6)
pub fn i_026d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D16;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $026D16: bra.w $26d24
pub fn i_026d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x26D24;
    10
}
/// $026D1A: lea.l $278704.l, a3
pub fn i_026d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D20;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x278704;
    20
}
/// $026D20: bra.w $26d24
pub fn i_026d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D24;
    cpu.ir = 0x6000;
    cpu.pc = 0x26D24;
    10
}
/// $026D24: cmpi.b #$0, $6a(a6)
pub fn i_026d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D2A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $026D2A: beq.w $26d46
pub fn i_026d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D46; return 10; }
    8
}
/// $026D2E: move.b $6a(a6), d6
pub fn i_026d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D32;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026D32: ext.w d6
pub fn i_026d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D34;
    cpu.ir = 0x4886;
    let v = cpu.d[6] as i8 as i16 as u16 as u32; cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $026D34: andi.w #$8000, d6
pub fn i_026d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D38;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026D38: move.w $28(a6), d5
pub fn i_026d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D3C;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026D3C: andi.w #$8000, d5
pub fn i_026d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D40;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $026D40: eor.w d5, d6
pub fn i_026d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D42;
    cpu.ir = 0xBB46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026D42: bne.w $26d60
pub fn i_026d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26D60; return 10; }
    8
}
/// $026D46: cmpi.b #$10, d0
pub fn i_026d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D4A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $026D4A: beq.w $26d56
pub fn i_026d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D56; return 10; }
    8
}
/// $026D4E: cmpi.b #$27, d0
pub fn i_026d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D52;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x27, Size::Byte, false);
    8
}
/// $026D52: bne.w $26d60
pub fn i_026d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26D60; return 10; }
    8
}
/// $026D56: bset.b #$5, $5a(a6)
pub fn i_026d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D5C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $026D5C: bra.w $26dbc
pub fn i_026d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D60;
    cpu.ir = 0x6000;
    cpu.pc = 0x26DBC;
    10
}
/// $026D60: move.w $64(a6), d6
pub fn i_026d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D64;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x64);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026D64: cmp.w $1a(a3), d6
pub fn i_026d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D68;
    cpu.ir = 0xBC6B;
    let ad2 = cpu.a[3].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $026D68: bne.w $26d76
pub fn i_026d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26D76; return 10; }
    8
}
/// $026D6C: move.w (a3), d6
pub fn i_026d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D6E;
    cpu.ir = 0x3C13;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $026D6E: cmp.w $34(a6), d6
pub fn i_026d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D72;
    cpu.ir = 0xBC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $026D72: beq.w $26d9c
pub fn i_026d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26D9C; return 10; }
    8
}
/// $026D76: movea.l a4, a1
pub fn i_026d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D78;
    cpu.ir = 0x224C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $026D78: jsr $26e0a(pc)
pub fn i_026d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26D7C);
    cpu.pc = 0x26E0A;
    16
}
/// $026D7C: jsr $27020(pc)
pub fn i_026d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26D80);
    cpu.pc = 0x27020;
    16
}
/// $026D80: clr.b -$1144(a5)
pub fn i_026d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D84;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $026D84: move.b $12(a2, d3.w), d5
pub fn i_026d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D88;
    cpu.ir = 0x1A32;
    let ad2 = cpu.a[2].wrapping_add(0x12).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $026D88: move.b d5, -$1143(a5)
pub fn i_026d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D8C;
    cpu.ir = 0x1B45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026D8C: addq.w #$1, d2
pub fn i_026d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D8E;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026D8E: move.w d1, -$1148(a5)
pub fn i_026d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D92;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026D92: move.w d2, -$1146(a5)
pub fn i_026d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D96;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_026d96  ($026D96, 6 B, thunks/ccr_helpers.c)
/// $026D96: ori.b #$11, ccr
pub fn i_026d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D9A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $026D9A: rts
pub fn i_026d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CommitMove_026d9c  ($026D9C, 32 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026D9C: bclr.b #$3, $5a(a6)
pub fn i_026d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DA2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $026DA2: addq.w #$1, d2
pub fn i_026da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DA4;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026DA4: move.w d1, -$1148(a5)
pub fn i_026da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DA8;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026DA8: move.w d2, -$1146(a5)
pub fn i_026da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DAC;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $026DAC: move.b -$1150(a5), d1
pub fn i_026dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DB0;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026DB0: move.b -$114f(a5), d2
pub fn i_026db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DB4;
    cpu.ir = 0x142D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $026DB4: move.b d1, -$1144(a5)
pub fn i_026db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DB8;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $026DB8: move.b d2, -$1143(a5)
pub fn i_026db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DBC;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_026dbc  ($026DBC, 6 B, thunks/ccr_helpers.c)
/// $026DBC: andi.b #$ee, ccr
pub fn i_026dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DC0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $026DC0: rts
pub fn i_026dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeOffsets_Right_026dc2  ($026DC2, 24 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026DC2: ?
pub fn i_026dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DC6;
    cpu.ir = 0x8;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[0] = r2;
    8
}
/// $026DC6: ?
pub fn i_026dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DCA;
    cpu.ir = 0x8;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[0] = r2;
    8
}
/// $026DCA: ?
pub fn i_026dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DCE;
    cpu.ir = 0x8;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[0] = r2;
    8
}
/// $026DCE: ?
pub fn i_026dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DD2;
    cpu.ir = 0x8;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[0] = r2;
    8
}
/// $026DD2: ?
pub fn i_026dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DD6;
    cpu.ir = 0x8;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = (v1 | 0xF8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[0] = r2;
    8
}
/// $026DD6: ?
pub fn i_026dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DDA;
    cpu.ir = 0x8;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = (v1 | 0xF8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[0] = r2;
    8
}
// ===== Entity_ProbeOffsets_Left_026dda  ($026DDA, 24 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026DDA: dc.w $ffff
pub fn i_026dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DDC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Entity_ProbeOffsets_None_026df2  ($026DF2, 24 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026DF2: ori.b #$0, d0
pub fn i_026df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DF6;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026DF6: ori.b #$0, d0
pub fn i_026df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DFA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026DFA: ori.b #$0, d0
pub fn i_026dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DFE;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026DFE: ori.b #$0, d0
pub fn i_026dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E02;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026E02: ori.b #$0, d0
pub fn i_026e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E06;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $026E06: ori.b #$0, d0
pub fn i_026e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E0A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
// ===== Entity_FloorProbe_026e0a  ($026E0A, 478 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026E0A: lea.l $26dc2(pc), a2
pub fn i_026e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E0E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x26DC2;
    16
}
/// $026E0E: tst.w $106f2c.l
pub fn i_026e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E14;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F2C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $026E14: beq.w $26e22
pub fn i_026e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E18;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26E22; return 10; }
    8
}
/// $026E18: tst.w $106f2c.l
pub fn i_026e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E1E;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F2C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $026E1E: bra.w $26e26
pub fn i_026e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E22;
    cpu.ir = 0x6000;
    cpu.pc = 0x26E26;
    10
}
/// $026E22: tst.w $28(a6)
pub fn i_026e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E26;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026E26: beq.w $26e36
pub fn i_026e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26E36; return 10; }
    8
}
/// $026E2A: bge.w $26e3a
pub fn i_026e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E2E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x26E3A; return 10; }
    8
}
/// $026E2E: lea.l $26dda(pc), a2
pub fn i_026e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E32;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x26DDA;
    16
}
/// $026E32: bra.w $26e3a
pub fn i_026e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E36;
    cpu.ir = 0x6000;
    cpu.pc = 0x26E3A;
    10
}
/// $026E36: lea.l $26df2(pc), a2
pub fn i_026e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E3A;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x26DF2;
    16
}
/// $026E3A: clr.w d5
pub fn i_026e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E3C;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026E3C: cmpi.w #$ffff, $34(a6)
pub fn i_026e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E42;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $026E42: beq.w $26e5a
pub fn i_026e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26E5A; return 10; }
    8
}
/// $026E46: tst.w $34(a6)
pub fn i_026e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E4A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026E4A: beq.w $26e5a
pub fn i_026e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26E5A; return 10; }
    8
}
/// $026E4E: bge.w $26e5e
pub fn i_026e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E52;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x26E5E; return 10; }
    8
}
/// $026E52: move.w #$10, d5
pub fn i_026e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E56;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026E56: bra.w $26e5e
pub fn i_026e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x26E5E;
    10
}
/// $026E5A: move.w #$8, d5
pub fn i_026e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E5E;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026E5E: move.w (a3), d6
pub fn i_026e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E60;
    cpu.ir = 0x3C13;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $026E60: cmpi.w #$ffff, d6
pub fn i_026e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E64;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $026E64: beq.w $26e70
pub fn i_026e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26E70; return 10; }
    8
}
/// $026E68: cmp.w $34(a6), d6
pub fn i_026e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E6C;
    cpu.ir = 0xBC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $026E6C: bge.w $26e72
pub fn i_026e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E70;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x26E72; return 10; }
    8
}
/// $026E70: addq.w #$4, d5
pub fn i_026e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E72;
    cpu.ir = 0x5845;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026E72: move.w $2(a2, d5.w), d6
pub fn i_026e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E76;
    cpu.ir = 0x3C32;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $026E76: move.w (a2, d5.w), d5
pub fn i_026e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E7A;
    cpu.ir = 0x3A32;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $026E7A: move.w -$1148(a5), d1
pub fn i_026e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E7E;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026E7E: move.w -$1146(a5), d2
pub fn i_026e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E82;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $026E82: subq.w #$1, d2
pub fn i_026e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E84;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026E84: sub.w d3, d1
pub fn i_026e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E86;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026E86: add.w d4, d2
pub fn i_026e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E88;
    cpu.ir = 0xD444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026E88: add.w d5, d1
pub fn i_026e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E8A;
    cpu.ir = 0xD245;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026E8A: add.w d6, d2
pub fn i_026e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E8C;
    cpu.ir = 0xD446;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026E8C: jsr $27db2(pc)
pub fn i_026e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26E90);
    cpu.pc = 0x27DB2;
    16
}
/// $026E90: jsr $9993c.l
pub fn i_026e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26E96);
    cpu.pc = 0x9993C;
    16
}
/// $026E96: move.b d6, $106f44.l
pub fn i_026e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E9C;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $026E9C: cmpi.b #$f, d6
pub fn i_026e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EA0;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $026EA0: beq.w $26eb6
pub fn i_026ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26EB6; return 10; }
    8
}
/// $026EA4: moveq #$0, d0
pub fn i_026ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EA6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026EA6: moveq #$0, d3
pub fn i_026ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EA8;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026EA8: move.w d5, d4
pub fn i_026ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EAA;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026EAA: moveq #$11, d0
pub fn i_026eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EAC;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026EAC: lea.l $278ba8.l, a1
pub fn i_026eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EB2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $026EB2: bra.w $26eb6
pub fn i_026eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x26EB6;
    10
}
/// $026EB6: cmpi.b #$c, d0
pub fn i_026eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EBA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $026EBA: beq.w $26f72
pub fn i_026eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EBE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026EBE: cmpi.b #$d, d0
pub fn i_026ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EC2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $026EC2: beq.w $26f72
pub fn i_026ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EC6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026EC6: cmpi.b #$e, d0
pub fn i_026ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ECA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $026ECA: beq.w $26f72
pub fn i_026eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ECE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026ECE: cmpi.b #$f, d0
pub fn i_026ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ED2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $026ED2: beq.w $26f72
pub fn i_026ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26ED6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026ED6: cmpi.b #$1c, d0
pub fn i_026ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EDA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1C, Size::Byte, false);
    8
}
/// $026EDA: beq.w $26f72
pub fn i_026eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EDE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026EDE: cmpi.b #$1d, d0
pub fn i_026ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EE2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1D, Size::Byte, false);
    8
}
/// $026EE2: beq.w $26f72
pub fn i_026ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EE6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026EE6: cmpi.b #$1e, d0
pub fn i_026ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EEA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1E, Size::Byte, false);
    8
}
/// $026EEA: beq.w $26f72
pub fn i_026eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026EEE: cmpi.b #$1f, d0
pub fn i_026eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EF2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1F, Size::Byte, false);
    8
}
/// $026EF2: beq.w $26f72
pub fn i_026ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026EF6: cmpi.b #$34, d0
pub fn i_026ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EFA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x34, Size::Byte, false);
    8
}
/// $026EFA: beq.w $26f72
pub fn i_026efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F72; return 10; }
    8
}
/// $026EFE: cmpi.b #$2c, d0
pub fn i_026efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F02;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2C, Size::Byte, false);
    8
}
/// $026F02: beq.w $26f70
pub fn i_026f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F06;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F70; return 10; }
    8
}
/// $026F06: cmpi.b #$2d, d0
pub fn i_026f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F0A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2D, Size::Byte, false);
    8
}
/// $026F0A: beq.w $26f70
pub fn i_026f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F70; return 10; }
    8
}
/// $026F0E: cmpi.b #$2e, d0
pub fn i_026f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F12;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2E, Size::Byte, false);
    8
}
/// $026F12: beq.w $26f70
pub fn i_026f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F70; return 10; }
    8
}
/// $026F16: cmpi.b #$2f, d0
pub fn i_026f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F1A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2F, Size::Byte, false);
    8
}
/// $026F1A: beq.w $26f70
pub fn i_026f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F70; return 10; }
    8
}
/// $026F1E: cmpi.b #$3c, d0
pub fn i_026f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F22;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3C, Size::Byte, false);
    8
}
/// $026F22: beq.w $26f70
pub fn i_026f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F26;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F70; return 10; }
    8
}
/// $026F26: cmpi.b #$3d, d0
pub fn i_026f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F2A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3D, Size::Byte, false);
    8
}
/// $026F2A: beq.w $26f70
pub fn i_026f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F70; return 10; }
    8
}
/// $026F2E: cmpi.b #$3e, d0
pub fn i_026f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F32;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3E, Size::Byte, false);
    8
}
/// $026F32: beq.w $26f70
pub fn i_026f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F36;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F70; return 10; }
    8
}
/// $026F36: cmpi.b #$3f, d0
pub fn i_026f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F3A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3F, Size::Byte, false);
    8
}
/// $026F3A: beq.w $26f70
pub fn i_026f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F70; return 10; }
    8
}
/// $026F3E: cmpi.b #$20, d0
pub fn i_026f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F42;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $026F42: beq.w $26f62
pub fn i_026f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F62; return 10; }
    8
}
/// $026F46: cmpi.b #$21, d0
pub fn i_026f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F4A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $026F4A: beq.w $26f62
pub fn i_026f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F62; return 10; }
    8
}
/// $026F4E: cmpi.b #$30, d0
pub fn i_026f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F52;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $026F52: beq.w $26f62
pub fn i_026f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F62; return 10; }
    8
}
/// $026F56: cmpi.b #$31, d0
pub fn i_026f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F5A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $026F5A: beq.w $26f62
pub fn i_026f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F62; return 10; }
    8
}
/// $026F5E: bra.w $26f9e
pub fn i_026f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F62;
    cpu.ir = 0x6000;
    cpu.pc = 0x26F9E;
    10
}
/// $026F62: btst.b #$4, $13(a6)
pub fn i_026f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F68;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $026F68: beq.w $26f9e
pub fn i_026f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F9E; return 10; }
    8
}
/// $026F6C: bra.w $26f72
pub fn i_026f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F70;
    cpu.ir = 0x6000;
    cpu.pc = 0x26F72;
    10
}
/// $026F70: addq.w #$8, d2
pub fn i_026f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F72;
    cpu.ir = 0x5042;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026F72: addq.w #$8, d2
pub fn i_026f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F74;
    cpu.ir = 0x5042;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026F74: jsr $27db2(pc)
pub fn i_026f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x26F78);
    cpu.pc = 0x27DB2;
    16
}
/// $026F78: jsr $9993c.l
pub fn i_026f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x26F7E);
    cpu.pc = 0x9993C;
    16
}
/// $026F7E: move.b d6, $106f44.l
pub fn i_026f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F84;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $026F84: cmpi.b #$f, d6
pub fn i_026f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F88;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $026F88: beq.w $26f9e
pub fn i_026f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x26F9E; return 10; }
    8
}
/// $026F8C: moveq #$0, d0
pub fn i_026f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F8E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026F8E: moveq #$0, d3
pub fn i_026f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F90;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026F90: move.w d5, d4
pub fn i_026f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F92;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $026F92: moveq #$11, d0
pub fn i_026f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F94;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $026F94: lea.l $278ba8.l, a1
pub fn i_026f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F9A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $026F9A: bra.w $26f9e
pub fn i_026f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x26F9E;
    10
}
/// $026F9E: movea.l $4(a1), a2
pub fn i_026f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FA2;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $026FA2: move.w (a2), $34(a6)
pub fn i_026fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FA6;
    cpu.ir = 0x3D52;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad3, v2 as u16);
    16
}
/// $026FA6: move.w $1a(a2), $64(a6)
pub fn i_026fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FAC;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x64);
    bus.write16(ad4, v3 as u16);
    20
}
/// $026FAC: cmpi.w #$80, $34(a6)
pub fn i_026fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FB2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Word, false);
    16
}
/// $026FB2: bne.w $26fc4
pub fn i_026fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26FC4; return 10; }
    8
}
/// $026FB6: nop
pub fn i_026fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FB8;
    cpu.ir = 0x4E71;
    4
}
/// $026FB8: nop
pub fn i_026fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FBA;
    cpu.ir = 0x4E71;
    4
}
/// $026FBA: cmpi.w #$80, $34(a6)
pub fn i_026fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FC0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Word, false);
    16
}
/// $026FC0: nop
pub fn i_026fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FC2;
    cpu.ir = 0x4E71;
    4
}
/// $026FC2: trap #$f
pub fn i_026fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FC4;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $026FC4: cmpi.w #$ff80, $34(a6)
pub fn i_026fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FCA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $026FCA: bne.w $26fdc
pub fn i_026fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FCE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26FDC; return 10; }
    8
}
/// $026FCE: nop
pub fn i_026fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FD0;
    cpu.ir = 0x4E71;
    4
}
/// $026FD0: nop
pub fn i_026fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FD2;
    cpu.ir = 0x4E71;
    4
}
/// $026FD2: cmpi.w #$ff80, $34(a6)
pub fn i_026fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FD8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $026FD8: nop
pub fn i_026fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FDA;
    cpu.ir = 0x4E71;
    4
}
/// $026FDA: trap #$f
pub fn i_026fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FDC;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $026FDC: asl.w #$1, d3
pub fn i_026fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FDE;
    cpu.ir = 0xE343;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $026FDE: move.w $2(a2, d3.w), d4
pub fn i_026fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FE2;
    cpu.ir = 0x3832;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $026FE2: asr.w #$1, d3
pub fn i_026fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FE4;
    cpu.ir = 0xE243;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $026FE4: sub.w d4, d2
pub fn i_026fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FE6;
    cpu.ir = 0x9444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $026FE6: rts
pub fn i_026fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_FloorProbe_Reenter_026fe8  ($026FE8, 56 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $026FE8: lea.l $26dc2(pc), a2
pub fn i_026fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FEC;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x26DC2;
    16
}
/// $026FEC: tst.w $28(a6)
pub fn i_026fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FF0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $026FF0: bge.w $26ff8
pub fn i_026ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FF4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x26FF8; return 10; }
    8
}
/// $026FF4: lea.l $26dda(pc), a2
pub fn i_026ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FF8;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x26DDA;
    16
}
/// $026FF8: clr.w d5
pub fn i_026ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FFA;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $026FFA: cmpi.w #$ffff, -$114e(a5)
pub fn i_026ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27000;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB2);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $027000: beq.w $27018
pub fn i_027000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27004;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27018; return 10; }
    8
}
/// $027004: tst.w -$114e(a5)
pub fn i_027004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27008;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $027008: beq.w $27018
pub fn i_027008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2700C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27018; return 10; }
    8
}
/// $02700C: bge.w $2701c
pub fn i_02700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27010;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2701C; return 10; }
    8
}
/// $027010: move.w #$10, d5
pub fn i_027010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27014;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027014: bra.w $2701c
pub fn i_027014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27018;
    cpu.ir = 0x6000;
    cpu.pc = 0x2701C;
    10
}
/// $027018: move.w #$8, d5
pub fn i_027018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2701C;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02701C: jmp $26e5e(pc)
pub fn i_02701c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27020;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x26E5E;
    8
}
// ===== Entity_WallProbeBoth_027020  ($027020, 22 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $027020: jsr $27e9c(pc)
pub fn i_027020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27024;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27024);
    cpu.pc = 0x27E9C;
    16
}
/// $027024: bcs.w $27034
pub fn i_027024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27028;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27034; return 10; }
    8
}
/// $027028: jsr $27e7e(pc)
pub fn i_027028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2702C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2702C);
    cpu.pc = 0x27E7E;
    16
}
/// $02702C: bcs.w $27034
pub fn i_02702c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27030;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27034; return 10; }
    8
}
/// $027030: bra.w $27034
pub fn i_027030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27034;
    cpu.ir = 0x6000;
    cpu.pc = 0x27034;
    10
}
/// $027034: rts
pub fn i_027034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27036;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveAndCollide_B_027036  ($027036, 580 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $027036: jsr $26814(pc)
pub fn i_027036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2703A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2703A);
    cpu.pc = 0x26814;
    16
}
/// $02703A: jsr $26992(pc)
pub fn i_02703a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2703E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2703E);
    cpu.pc = 0x26992;
    16
}
/// $02703E: jsr $44182.l
pub fn i_02703e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27044);
    cpu.pc = 0x44182;
    16
}
/// $027044: move.w -$1148(a5), d1
pub fn i_027044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27048;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027048: move.w -$1146(a5), d2
pub fn i_027048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2704C;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02704C: subq.w #$1, d2
pub fn i_02704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2704E;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02704E: jsr $27db2(pc)
pub fn i_02704e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27052;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27052);
    cpu.pc = 0x27DB2;
    16
}
/// $027052: add.w -$1154(a5), d1
pub fn i_027052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27056;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $027056: add.w -$1152(a5), d2
pub fn i_027056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2705A;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02705A: jsr $998ca.l
pub fn i_02705a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27060;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27060);
    cpu.pc = 0x998CA;
    16
}
/// $027060: bcc.w $27086
pub fn i_027060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27064;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x27086; return 10; }
    8
}
/// $027064: moveq #$0, d0
pub fn i_027064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27066;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027066: moveq #$0, d3
pub fn i_027066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27068;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027068: moveq #$0, d4
pub fn i_027068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2706A;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02706A: moveq #$11, d0
pub fn i_02706a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2706C;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02706C: move.l d0, d7
pub fn i_02706c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2706E;
    cpu.ir = 0x2E00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02706E: lea.l $278ba8.l, a1
pub fn i_02706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27074;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $027074: movea.l a1, a4
pub fn i_027074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27076;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $027076: movea.l $4(a1), a3
pub fn i_027076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2707A;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $02707A: move.b $12(a3, d3.w), $106f31.l
pub fn i_02707a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27082;
    cpu.ir = 0x13F3;
    let ad2 = cpu.a[3].wrapping_add(0x12).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F31, v3 as u8);
    26
}
/// $027082: bra.w $27166
pub fn i_027082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27086;
    cpu.ir = 0x6000;
    cpu.pc = 0x27166;
    10
}
/// $027086: sub.w -$1154(a5), d1
pub fn i_027086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2708A;
    cpu.ir = 0x926D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02708A: sub.w -$1152(a5), d2
pub fn i_02708a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2708E;
    cpu.ir = 0x946D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02708E: cmpi.b #$1, $a(a1)
pub fn i_02708e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27094;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $027094: bne.w $270a4
pub fn i_027094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27098;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x270A4; return 10; }
    8
}
/// $027098: tst.w $34(a6)
pub fn i_027098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2709C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02709C: bne.w $270a4
pub fn i_02709c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x270A4; return 10; }
    8
}
/// $0270A0: adda.w #$10, a1
pub fn i_0270a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270A4;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x10;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $0270A4: jsr $27e9c(pc)
pub fn i_0270a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x270A8);
    cpu.pc = 0x27E9C;
    16
}
/// $0270A8: bcs.w $270b8
pub fn i_0270a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270AC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x270B8; return 10; }
    8
}
/// $0270AC: jsr $27e7e(pc)
pub fn i_0270ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x270B0);
    cpu.pc = 0x27E7E;
    16
}
/// $0270B0: bcs.w $270b8
pub fn i_0270b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270B4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x270B8; return 10; }
    8
}
/// $0270B4: bra.w $270b8
pub fn i_0270b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x270B8;
    10
}
/// $0270B8: add.w -$1154(a5), d1
pub fn i_0270b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270BC;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0270BC: add.w -$1152(a5), d2
pub fn i_0270bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270C0;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0270C0: movem.w d3-d4, -(a7)
pub fn i_0270c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270C4;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    cpu.a[7] = a;
    20
}
/// $0270C4: movea.l a1, a4
pub fn i_0270c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270C6;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $0270C6: jsr $27db2(pc)
pub fn i_0270c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x270CA);
    cpu.pc = 0x27DB2;
    16
}
/// $0270CA: cmpi.b #$1, $a(a1)
pub fn i_0270ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270D0;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0270D0: bne.w $27158
pub fn i_0270d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27158; return 10; }
    8
}
/// $0270D4: tst.w $34(a6)
pub fn i_0270d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270D8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0270D8: bne.w $27158
pub fn i_0270d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27158; return 10; }
    8
}
/// $0270DC: tst.w -$1154(a5)
pub fn i_0270dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270E0;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0270E0: blt.w $270f8
pub fn i_0270e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270E4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x270F8; return 10; }
    8
}
/// $0270E4: cmpi.b #$23, d0
pub fn i_0270e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270E8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x23, Size::Byte, false);
    8
}
/// $0270E8: beq.w $2710c
pub fn i_0270e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2710C; return 10; }
    8
}
/// $0270EC: cmpi.b #$33, d0
pub fn i_0270ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270F0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x33, Size::Byte, false);
    8
}
/// $0270F0: beq.w $2710c
pub fn i_0270f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2710C; return 10; }
    8
}
/// $0270F4: bra.w $27154
pub fn i_0270f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x27154;
    10
}
/// $0270F8: cmpi.b #$22, d0
pub fn i_0270f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270FC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x22, Size::Byte, false);
    8
}
/// $0270FC: beq.w $2710c
pub fn i_0270fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27100;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2710C; return 10; }
    8
}
/// $027100: cmpi.b #$32, d0
pub fn i_027100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27104;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x32, Size::Byte, false);
    8
}
/// $027104: beq.w $2710c
pub fn i_027104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27108;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2710C; return 10; }
    8
}
/// $027108: bra.w $27154
pub fn i_027108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2710C;
    cpu.ir = 0x6000;
    cpu.pc = 0x27154;
    10
}
/// $02710C: btst.b #$5, $13(a6)
pub fn i_02710c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27112;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $027112: beq.w $27154
pub fn i_027112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27116;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27154; return 10; }
    8
}
/// $027116: tst.w -$1154(a5)
pub fn i_027116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2711A;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02711A: beq.w $27154
pub fn i_02711a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2711E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27154; return 10; }
    8
}
/// $02711E: btst.b #$5, $5b(a6)
pub fn i_02711e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27124;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $027124: bne.w $2714a
pub fn i_027124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27128;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2714A; return 10; }
    8
}
/// $027128: cmpi.b #$0, $6a(a6)
pub fn i_027128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2712E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02712E: beq.w $2714a
pub fn i_02712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27132;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2714A; return 10; }
    8
}
/// $027132: move.b $6a(a6), d6
pub fn i_027132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27136;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $027136: ext.w d6
pub fn i_027136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27138;
    cpu.ir = 0x4886;
    let v = cpu.d[6] as i8 as i16 as u16 as u32; cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $027138: andi.w #$8000, d6
pub fn i_027138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2713C;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02713C: move.w $28(a6), d5
pub fn i_02713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27140;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027140: andi.w #$8000, d5
pub fn i_027140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27144;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $027144: eor.w d5, d6
pub fn i_027144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27146;
    cpu.ir = 0xBB46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $027146: bne.w $27154
pub fn i_027146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2714A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27154; return 10; }
    8
}
/// $02714A: bset.b #$3, $5a(a6)
pub fn i_02714a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27150;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $027150: bra.w $27158
pub fn i_027150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27154;
    cpu.ir = 0x6000;
    cpu.pc = 0x27158;
    10
}
/// $027154: adda.w #$10, a1
pub fn i_027154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27158;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x10;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $027158: movea.l $4(a1), a3
pub fn i_027158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2715C;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $02715C: move.b $12(a3, d3.w), -$114f(a5)
pub fn i_02715c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27162;
    cpu.ir = 0x1B73;
    let ad2 = cpu.a[3].wrapping_add(0x12).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    bus.write8(ad4, v3 as u8);
    22
}
/// $027162: movem.w (a7)+, d3-d4
pub fn i_027162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27166;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $027166: tst.w -$1154(a5)
pub fn i_027166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2716A;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02716A: blt.w $27192
pub fn i_02716a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2716E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x27192; return 10; }
    8
}
/// $02716E: cmpi.b #$20, d0
pub fn i_02716e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27172;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $027172: beq.w $271b6
pub fn i_027172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27176;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271B6; return 10; }
    8
}
/// $027176: cmpi.b #$30, d0
pub fn i_027176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2717A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $02717A: beq.w $271b6
pub fn i_02717a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2717E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271B6; return 10; }
    8
}
/// $02717E: cmpi.b #$21, d0
pub fn i_02717e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27182;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $027182: beq.w $271fe
pub fn i_027182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27186;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271FE; return 10; }
    8
}
/// $027186: cmpi.b #$31, d0
pub fn i_027186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2718A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $02718A: beq.w $271fe
pub fn i_02718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2718E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271FE; return 10; }
    8
}
/// $02718E: bra.w $27208
pub fn i_02718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27192;
    cpu.ir = 0x6000;
    cpu.pc = 0x27208;
    10
}
/// $027192: cmpi.b #$20, d0
pub fn i_027192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27196;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $027196: beq.w $271fe
pub fn i_027196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2719A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271FE; return 10; }
    8
}
/// $02719A: cmpi.b #$30, d0
pub fn i_02719a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2719E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $02719E: beq.w $271fe
pub fn i_02719e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271FE; return 10; }
    8
}
/// $0271A2: cmpi.b #$21, d0
pub fn i_0271a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271A6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $0271A6: beq.w $271b6
pub fn i_0271a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271B6; return 10; }
    8
}
/// $0271AA: cmpi.b #$31, d0
pub fn i_0271aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271AE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $0271AE: beq.w $271b6
pub fn i_0271ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271B6; return 10; }
    8
}
/// $0271B2: bra.w $27208
pub fn i_0271b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x27208;
    10
}
/// $0271B6: btst.b #$4, $13(a6)
pub fn i_0271b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271BC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0271BC: beq.w $271fe
pub fn i_0271bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271FE; return 10; }
    8
}
/// $0271C0: tst.w -$1154(a5)
pub fn i_0271c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271C4;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0271C4: beq.w $271fe
pub fn i_0271c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271FE; return 10; }
    8
}
/// $0271C8: btst.b #$4, $5b(a6)
pub fn i_0271c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271CE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0271CE: bne.w $271f4
pub fn i_0271ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x271F4; return 10; }
    8
}
/// $0271D2: cmpi.b #$0, $6a(a6)
pub fn i_0271d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271D8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0271D8: beq.w $271f4
pub fn i_0271d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x271F4; return 10; }
    8
}
/// $0271DC: move.b $6a(a6), d6
pub fn i_0271dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271E0;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0271E0: ext.w d6
pub fn i_0271e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271E2;
    cpu.ir = 0x4886;
    let v = cpu.d[6] as i8 as i16 as u16 as u32; cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0271E2: andi.w #$8000, d6
pub fn i_0271e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271E6;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0271E6: move.w $28(a6), d5
pub fn i_0271e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271EA;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0271EA: andi.w #$8000, d5
pub fn i_0271ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271EE;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0271EE: eor.w d5, d6
pub fn i_0271ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271F0;
    cpu.ir = 0xBB46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0271F0: bne.w $271fe
pub fn i_0271f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x271FE; return 10; }
    8
}
/// $0271F4: bset.b #$3, $5a(a6)
pub fn i_0271f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271FA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0271FA: bra.w $27208
pub fn i_0271fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x27208;
    10
}
/// $0271FE: lea.l $278704.l, a3
pub fn i_0271fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27204;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x278704;
    20
}
/// $027204: bra.w $27208
pub fn i_027204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27208;
    cpu.ir = 0x6000;
    cpu.pc = 0x27208;
    10
}
/// $027208: cmpi.b #$0, $6a(a6)
pub fn i_027208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2720E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02720E: beq.w $2722a
pub fn i_02720e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27212;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2722A; return 10; }
    8
}
/// $027212: move.b $6a(a6), d6
pub fn i_027212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27216;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $027216: ext.w d6
pub fn i_027216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27218;
    cpu.ir = 0x4886;
    let v = cpu.d[6] as i8 as i16 as u16 as u32; cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $027218: andi.w #$8000, d6
pub fn i_027218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2721C;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02721C: move.w $28(a6), d5
pub fn i_02721c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27220;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027220: andi.w #$8000, d5
pub fn i_027220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27224;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $027224: eor.w d5, d6
pub fn i_027224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27226;
    cpu.ir = 0xBB46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $027226: bne.w $27244
pub fn i_027226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2722A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27244; return 10; }
    8
}
/// $02722A: cmpi.b #$10, d0
pub fn i_02722a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2722E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $02722E: beq.w $2723a
pub fn i_02722e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27232;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2723A; return 10; }
    8
}
/// $027232: cmpi.b #$27, d0
pub fn i_027232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27236;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x27, Size::Byte, false);
    8
}
/// $027236: bne.w $27244
pub fn i_027236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2723A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27244; return 10; }
    8
}
/// $02723A: bset.b #$5, $5a(a6)
pub fn i_02723a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27240;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $027240: bra.w $272a2
pub fn i_027240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27244;
    cpu.ir = 0x6000;
    cpu.pc = 0x272A2;
    10
}
/// $027244: move.w $64(a6), d6
pub fn i_027244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27248;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x64);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027248: cmp.w $1a(a3), d6
pub fn i_027248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2724C;
    cpu.ir = 0xBC6B;
    let ad2 = cpu.a[3].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02724C: bne.w $2725a
pub fn i_02724c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27250;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2725A; return 10; }
    8
}
/// $027250: move.w (a3), d6
pub fn i_027250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27252;
    cpu.ir = 0x3C13;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $027252: cmp.w $34(a6), d6
pub fn i_027252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27256;
    cpu.ir = 0xBC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $027256: beq.w $27282
pub fn i_027256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2725A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27282; return 10; }
    8
}
/// $02725A: movea.l a4, a1
pub fn i_02725a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2725C;
    cpu.ir = 0x224C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02725C: jsr $26e0a(pc)
pub fn i_02725c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27260;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27260);
    cpu.pc = 0x26E0A;
    16
}
/// $027260: jsr $27020(pc)
pub fn i_027260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27264;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27264);
    cpu.pc = 0x27020;
    16
}
/// $027264: clr.b -$1144(a5)
pub fn i_027264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27268;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $027268: move.b $12(a2, d3.w), d5
pub fn i_027268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2726C;
    cpu.ir = 0x1A32;
    let ad2 = cpu.a[2].wrapping_add(0x12).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02726C: move.b d5, -$1143(a5)
pub fn i_02726c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27270;
    cpu.ir = 0x1B45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $027270: addq.w #$1, d2
pub fn i_027270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27272;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $027272: move.w d1, -$1148(a5)
pub fn i_027272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27276;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027276: move.w d2, -$1146(a5)
pub fn i_027276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2727A;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_02727a  ($02727A, 6 B, thunks/ccr_helpers.c)
/// $02727A: ori.b #$11, ccr
pub fn i_02727a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2727E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02727E: rts
pub fn i_02727e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27280;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CommitMove_ClearBit3_027280  ($027280, 34 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $027280: nop
pub fn i_027280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27282;
    cpu.ir = 0x4E71;
    4
}
/// $027282: bclr.b #$3, $5a(a6)
pub fn i_027282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27288;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $027288: addq.w #$1, d2
pub fn i_027288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2728A;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02728A: move.w d1, -$1148(a5)
pub fn i_02728a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2728E;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02728E: move.w d2, -$1146(a5)
pub fn i_02728e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27292;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027292: move.b -$1150(a5), d1
pub fn i_027292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27296;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $027296: move.b -$114f(a5), d2
pub fn i_027296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2729A;
    cpu.ir = 0x142D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02729A: move.b d1, -$1144(a5)
pub fn i_02729a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2729E;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02729E: move.b d2, -$1143(a5)
pub fn i_02729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272A2;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_0272a2  ($0272A2, 6 B, thunks/ccr_helpers.c)
/// $0272A2: andi.b #$ee, ccr
pub fn i_0272a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272A6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0272A6: rts
pub fn i_0272a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveAndCollide_C_0272a8  ($0272A8, 302 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0272A8: jsr $26814(pc)
pub fn i_0272a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x272AC);
    cpu.pc = 0x26814;
    16
}
/// $0272AC: jsr $26992(pc)
pub fn i_0272ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x272B0);
    cpu.pc = 0x26992;
    16
}
/// $0272B0: jsr $44182.l
pub fn i_0272b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x272B6);
    cpu.pc = 0x44182;
    16
}
/// $0272B6: move.w -$1148(a5), d1
pub fn i_0272b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272BA;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0272BA: move.w -$1146(a5), d2
pub fn i_0272ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272BE;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0272BE: subq.w #$1, d2
pub fn i_0272be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272C0;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0272C0: jsr $27db2(pc)
pub fn i_0272c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x272C4);
    cpu.pc = 0x27DB2;
    16
}
/// $0272C4: add.w -$1154(a5), d1
pub fn i_0272c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272C8;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0272C8: add.w -$1152(a5), d2
pub fn i_0272c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272CC;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0272CC: jsr $998ca.l
pub fn i_0272cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x272D2);
    cpu.pc = 0x998CA;
    16
}
/// $0272D2: bcc.w $272f8
pub fn i_0272d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x272F8; return 10; }
    8
}
/// $0272D6: moveq #$0, d0
pub fn i_0272d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272D8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0272D8: moveq #$0, d3
pub fn i_0272d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272DA;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0272DA: moveq #$0, d4
pub fn i_0272da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272DC;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0272DC: moveq #$11, d0
pub fn i_0272dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272DE;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0272DE: move.l d0, d7
pub fn i_0272de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272E0;
    cpu.ir = 0x2E00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0272E0: lea.l $278ba8.l, a1
pub fn i_0272e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $0272E6: movea.l a1, a4
pub fn i_0272e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272E8;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $0272E8: movea.l $4(a1), a3
pub fn i_0272e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272EC;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $0272EC: move.b $12(a3, d3.w), $106f31.l
pub fn i_0272ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272F4;
    cpu.ir = 0x13F3;
    let ad2 = cpu.a[3].wrapping_add(0x12).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F31, v3 as u8);
    26
}
/// $0272F4: bra.w $27372
pub fn i_0272f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x27372;
    10
}
/// $0272F8: sub.w -$1154(a5), d1
pub fn i_0272f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272FC;
    cpu.ir = 0x926D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0272FC: sub.w -$1152(a5), d2
pub fn i_0272fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27300;
    cpu.ir = 0x946D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $027300: cmpi.b #$1, $a(a1)
pub fn i_027300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27306;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $027306: bne.w $27316
pub fn i_027306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2730A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27316; return 10; }
    8
}
/// $02730A: tst.w $64(a6)
pub fn i_02730a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2730E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x64);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02730E: bne.w $27316
pub fn i_02730e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27312;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27316; return 10; }
    8
}
/// $027312: adda.w #$10, a1
pub fn i_027312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27316;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x10;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $027316: jsr $27e9c(pc)
pub fn i_027316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2731A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2731A);
    cpu.pc = 0x27E9C;
    16
}
/// $02731A: bcs.w $27342
pub fn i_02731a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2731E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27342; return 10; }
    8
}
/// $02731E: jsr $27e7e(pc)
pub fn i_02731e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27322;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27322);
    cpu.pc = 0x27E7E;
    16
}
/// $027322: bcs.w $27336
pub fn i_027322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27326;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27336; return 10; }
    8
}
/// $027326: btst.b #$0, $100000.l
pub fn i_027326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2732E;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $02732E: beq.w $27332
pub fn i_02732e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27332;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27332; return 10; }
    8
}
/// $027332: bra.w $27342
pub fn i_027332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27336;
    cpu.ir = 0x6000;
    cpu.pc = 0x27342;
    10
}
/// $027336: btst.b #$0, $100000.l
pub fn i_027336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2733E;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $02733E: beq.w $27342
pub fn i_02733e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27342;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27342; return 10; }
    8
}
/// $027342: add.w -$1154(a5), d1
pub fn i_027342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27346;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $027346: add.w -$1152(a5), d2
pub fn i_027346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2734A;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02734A: movem.w d3-d4, -(a7)
pub fn i_02734a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2734E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    cpu.a[7] = a;
    20
}
/// $02734E: movea.l a1, a4
pub fn i_02734e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27350;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $027350: jsr $27db2(pc)
pub fn i_027350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27354;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27354);
    cpu.pc = 0x27DB2;
    16
}
/// $027354: cmpi.b #$1, $a(a1)
pub fn i_027354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2735A;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $02735A: bne.w $2736a
pub fn i_02735a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2735E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2736A; return 10; }
    8
}
/// $02735E: tst.w $34(a6)
pub fn i_02735e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27362;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $027362: bne.w $2736a
pub fn i_027362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27366;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2736A; return 10; }
    8
}
/// $027366: adda.w #$10, a1
pub fn i_027366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2736A;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x10;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $02736A: movem.w (a7)+, d3-d4
pub fn i_02736a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2736E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02736E: movea.l $4(a1), a3
pub fn i_02736e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27372;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $027372: cmpi.b #$20, d0
pub fn i_027372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27376;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $027376: beq.w $27396
pub fn i_027376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2737A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27396; return 10; }
    8
}
/// $02737A: cmpi.b #$21, d0
pub fn i_02737a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2737E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $02737E: beq.w $27396
pub fn i_02737e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27382;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27396; return 10; }
    8
}
/// $027382: cmpi.b #$30, d0
pub fn i_027382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27386;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $027386: beq.w $27396
pub fn i_027386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2738A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27396; return 10; }
    8
}
/// $02738A: cmpi.b #$31, d0
pub fn i_02738a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2738E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $02738E: beq.w $27396
pub fn i_02738e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27392;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27396; return 10; }
    8
}
/// $027392: bra.w $273a6
pub fn i_027392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27396;
    cpu.ir = 0x6000;
    cpu.pc = 0x273A6;
    10
}
/// $027396: btst.b #$4, $13(a6)
pub fn i_027396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2739C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02739C: bne.w $273a6
pub fn i_02739c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x273A6; return 10; }
    8
}
/// $0273A0: lea.l $278704.l, a3
pub fn i_0273a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273A6;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x278704;
    20
}
/// $0273A6: cmpi.b #$10, d0
pub fn i_0273a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273AA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $0273AA: beq.w $273b6
pub fn i_0273aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x273B6; return 10; }
    8
}
/// $0273AE: cmpi.b #$27, d0
pub fn i_0273ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273B2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x27, Size::Byte, false);
    8
}
/// $0273B2: bne.w $273c0
pub fn i_0273b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x273C0; return 10; }
    8
}
/// $0273B6: bset.b #$5, $5a(a6)
pub fn i_0273b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273BC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0273BC: bra.w $273d6
pub fn i_0273bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x273D6;
    10
}
/// $0273C0: move.w $64(a6), d6
pub fn i_0273c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273C4;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x64);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0273C4: cmp.w $1a(a3), d6
pub fn i_0273c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273C8;
    cpu.ir = 0xBC6B;
    let ad2 = cpu.a[3].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0273C8: bne.w $273d6
pub fn i_0273c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x273D6; return 10; }
    8
}
/// $0273CC: move.w $34(a6), d6
pub fn i_0273cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273D0;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0273D0: cmp.w (a3), d6
pub fn i_0273d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273D2;
    cpu.ir = 0xBC53;
    let v2 = bus.read16(cpu.a[3]) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    8
}
/// $0273D2: beq.w $273dc
pub fn i_0273d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x273DC; return 10; }
    8
}
// ===== SetXN_0273d6  ($0273D6, 6 B, thunks/ccr_helpers.c)
/// $0273D6: ori.b #$11, ccr
pub fn i_0273d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273DA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0273DA: rts
pub fn i_0273da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CommitMove_C_0273dc  ($0273DC, 26 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0273DC: addq.w #$1, d2
pub fn i_0273dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273DE;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0273DE: move.w d1, -$1148(a5)
pub fn i_0273de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273E2;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0273E2: move.w d2, -$1146(a5)
pub fn i_0273e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273E6;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0273E6: move.b -$1150(a5), d1
pub fn i_0273e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273EA;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0273EA: move.b -$114f(a5), d2
pub fn i_0273ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273EE;
    cpu.ir = 0x142D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0273EE: move.b d1, -$1144(a5)
pub fn i_0273ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273F2;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0273F2: move.b d2, -$1143(a5)
pub fn i_0273f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273F6;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_0273f6  ($0273F6, 6 B, thunks/ccr_helpers.c)
/// $0273F6: andi.b #$ee, ccr
pub fn i_0273f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273FA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0273FA: rts
pub fn i_0273fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SaveRegs_0273fc  ($0273FC, 4 B, entity/hud_inputrec_entity_move_024e1c.s)
/// $0273FC: movem.l d3-d6/a1, -(a7)
pub fn i_0273fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27400;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
// ===== Entity_MoveC_Tail_027400  ($027400, 36 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027400: cmpi.w #$ffff, $34(a6)
pub fn i_027400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27406;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $027406: beq.w $2742a
pub fn i_027406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2740A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2742A; return 10; }
    8
}
/// $02740A: jsr $44182.l
pub fn i_02740a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27410);
    cpu.pc = 0x44182;
    16
}
/// $027410: move.w -$1148(a5), d1
pub fn i_027410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27414;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027414: move.w -$1146(a5), d2
pub fn i_027414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27418;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027418: jsr $27ec2(pc)
pub fn i_027418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2741C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2741C);
    cpu.pc = 0x27EC2;
    16
}
/// $02741C: bcs.w $2742a
pub fn i_02741c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27420;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2742A; return 10; }
    8
}
/// $027420: movem.l (a7)+, d3-d6/a1
pub fn i_027420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27424;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_027424  ($027424, 6 B, thunks/ccr_helpers.c)
/// $027424: ori.b #$11, ccr
pub fn i_027424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27428;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027428: rts
pub fn i_027428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2742A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveC_Land_02742a  ($02742A, 26 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02742A: movem.l (a7)+, d3-d6/a1
pub fn i_02742a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2742E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $02742E: move.w $2a(a6), d0
pub fn i_02742e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27432;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027432: move.w #$400, d1
pub fn i_027432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27436;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027436: jsr $267f4(pc)
pub fn i_027436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2743A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2743A);
    cpu.pc = 0x267F4;
    16
}
/// $02743A: move.w d0, $2a(a6)
pub fn i_02743a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2743E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02743E: move.w #$ffc0, $2e(a6)
pub fn i_02743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27444;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Entity_FallStep_027444  ($027444, 20 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027444: jsr $274b8(pc)
pub fn i_027444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27448;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27448);
    cpu.pc = 0x274B8;
    16
}
/// $027448: bcc.w $2745e
pub fn i_027448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2744C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2745E; return 10; }
    8
}
/// $02744C: move.w #$0, $2e(a6)
pub fn i_02744c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27452;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027452: move.w #$0, $2a(a6)
pub fn i_027452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27458;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_027458  ($027458, 6 B, thunks/ccr_helpers.c)
/// $027458: ori.b #$11, ccr
pub fn i_027458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2745C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02745C: rts
pub fn i_02745c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2745E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02745e  ($02745E, 6 B, thunks/ccr_helpers.c)
/// $02745E: andi.b #$ee, ccr
pub fn i_02745e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27462;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027462: rts
pub fn i_027462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27464;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveFall_027464  ($027464, 72 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027464: move.w #$ffff, $34(a6)
pub fn i_027464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2746A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02746A: move.w #$ffff, $64(a6)
pub fn i_02746a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27470;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x64);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027470: move.w $2c(a6), d0
pub fn i_027470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27474;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027474: add.w $28(a6), d0
pub fn i_027474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27478;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $027478: jsr $267f8(pc)
pub fn i_027478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2747C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2747C);
    cpu.pc = 0x267F8;
    16
}
/// $02747C: move.w d0, $28(a6)
pub fn i_02747c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27480;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027480: move.w $2e(a6), d0
pub fn i_027480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27484;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027484: add.w $2a(a6), d0
pub fn i_027484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27488;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $027488: bgt.w $27490
pub fn i_027488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2748C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x27490; return 10; }
    8
}
/// $02748C: jsr $267f8(pc)
pub fn i_02748c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27490;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27490);
    cpu.pc = 0x267F8;
    16
}
/// $027490: move.w d0, $2a(a6)
pub fn i_027490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27494;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027494: jsr $26836(pc)
pub fn i_027494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27498;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27498);
    cpu.pc = 0x26836;
    16
}
/// $027498: jsr $274cc(pc)
pub fn i_027498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2749C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2749C);
    cpu.pc = 0x274CC;
    16
}
/// $02749C: bcc.w $274b2
pub fn i_02749c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x274B2; return 10; }
    8
}
/// $0274A0: move.w #$0, $2e(a6)
pub fn i_0274a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274A6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0274A6: move.w #$0, $2a(a6)
pub fn i_0274a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_0274ac  ($0274AC, 6 B, thunks/ccr_helpers.c)
/// $0274AC: ori.b #$11, ccr
pub fn i_0274ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274B0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0274B0: rts
pub fn i_0274b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0274b2  ($0274B2, 6 B, thunks/ccr_helpers.c)
/// $0274B2: andi.b #$ee, ccr
pub fn i_0274b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274B6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0274B6: rts
pub fn i_0274b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveAndCollide_D_0274b8  ($0274B8, 250 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0274B8: move.w #$ffff, $34(a6)
pub fn i_0274b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0274BE: move.w #$ffff, $64(a6)
pub fn i_0274be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x64);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0274C4: jsr $26814(pc)
pub fn i_0274c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x274C8);
    cpu.pc = 0x26814;
    16
}
/// $0274C8: jsr $26836(pc)
pub fn i_0274c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x274CC);
    cpu.pc = 0x26836;
    16
}
/// $0274CC: jsr $44182.l
pub fn i_0274cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x274D2);
    cpu.pc = 0x44182;
    16
}
/// $0274D2: move.w -$1148(a5), d1
pub fn i_0274d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274D6;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0274D6: move.w -$1146(a5), d2
pub fn i_0274d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274DA;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0274DA: add.w -$1154(a5), d1
pub fn i_0274da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274DE;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0274DE: jsr $27db2(pc)
pub fn i_0274de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x274E2);
    cpu.pc = 0x27DB2;
    16
}
/// $0274E2: cmpi.b #$1, d0
pub fn i_0274e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274E6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0274E6: beq.w $27526
pub fn i_0274e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27526; return 10; }
    8
}
/// $0274EA: cmpi.b #$10, d0
pub fn i_0274ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274EE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $0274EE: beq.w $27526
pub fn i_0274ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27526; return 10; }
    8
}
/// $0274F2: cmpi.b #$26, d0
pub fn i_0274f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274F6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x26, Size::Byte, false);
    8
}
/// $0274F6: beq.w $27526
pub fn i_0274f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27526; return 10; }
    8
}
/// $0274FA: cmpi.b #$27, d0
pub fn i_0274fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274FE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x27, Size::Byte, false);
    8
}
/// $0274FE: beq.w $27526
pub fn i_0274fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27502;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27526; return 10; }
    8
}
/// $027502: cmpi.b #$c, d0
pub fn i_027502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27506;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $027506: beq.w $27526
pub fn i_027506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2750A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27526; return 10; }
    8
}
/// $02750A: cmpi.b #$d, d0
pub fn i_02750a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2750E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $02750E: beq.w $27526
pub fn i_02750e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27512;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27526; return 10; }
    8
}
/// $027512: cmpi.b #$e, d0
pub fn i_027512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27516;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $027516: beq.w $27526
pub fn i_027516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2751A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27526; return 10; }
    8
}
/// $02751A: cmpi.b #$f, d0
pub fn i_02751a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2751E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $02751E: beq.w $27526
pub fn i_02751e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27522;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27526; return 10; }
    8
}
/// $027522: bra.w $27534
pub fn i_027522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27526;
    cpu.ir = 0x6000;
    cpu.pc = 0x27534;
    10
}
/// $027526: bset.b #$5, $5a(a6)
pub fn i_027526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2752C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02752C: clr.w -$1154(a5)
pub fn i_02752c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27530;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $027530: clr.b -$1150(a5)
pub fn i_027530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27534;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $027534: move.w -$1148(a5), d1
pub fn i_027534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27538;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027538: move.w -$1146(a5), d2
pub fn i_027538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2753C;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02753C: add.w -$1154(a5), d1
pub fn i_02753c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27540;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $027540: add.w -$1152(a5), d2
pub fn i_027540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27544;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $027544: subi.w #$1, d2
pub fn i_027544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27548;
    cpu.ir = 0x442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $027548: jsr $27db2(pc)
pub fn i_027548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2754C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2754C);
    cpu.pc = 0x27DB2;
    16
}
/// $02754C: jsr $9993c.l
pub fn i_02754c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27552);
    cpu.pc = 0x9993C;
    16
}
/// $027552: move.b d6, $106f44.l
pub fn i_027552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27558;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $027558: cmpi.b #$f, d6
pub fn i_027558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2755C;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $02755C: beq.w $27572
pub fn i_02755c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27560;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27572; return 10; }
    8
}
/// $027560: moveq #$0, d0
pub fn i_027560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27562;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027562: moveq #$0, d3
pub fn i_027562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27564;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027564: move.w d5, d4
pub fn i_027564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27566;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027566: moveq #$11, d0
pub fn i_027566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27568;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027568: lea.l $278ba8.l, a1
pub fn i_027568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2756E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $02756E: bra.w $27572
pub fn i_02756e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27572;
    cpu.ir = 0x6000;
    cpu.pc = 0x27572;
    10
}
/// $027572: cmpi.b #$28, d0
pub fn i_027572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27576;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x28, Size::Byte, false);
    8
}
/// $027576: bne.w $27586
pub fn i_027576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2757A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27586; return 10; }
    8
}
/// $02757A: tst.w $2a(a6)
pub fn i_02757a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2757E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02757E: ble.w $27586
pub fn i_02757e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27582;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x27586; return 10; }
    8
}
/// $027582: clr.w $2a(a6)
pub fn i_027582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27586;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $027586: jsr $27e28(pc)
pub fn i_027586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2758A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2758A);
    cpu.pc = 0x27E28;
    16
}
/// $02758A: bcc.w $2759a
pub fn i_02758a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2758E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2759A; return 10; }
    8
}
/// $02758E: movea.l $4(a1), a3
pub fn i_02758e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27592;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $027592: bsr.w $2822c
pub fn i_027592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27596;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x27596);
    cpu.pc = 0x2822C;
    18
}
/// $027596: bcc.w $275b8
pub fn i_027596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2759A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x275B8; return 10; }
    8
}
/// $02759A: move.w d1, -$1148(a5)
pub fn i_02759a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2759E;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02759E: addi.w #$1, d2
pub fn i_02759e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275A2;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0275A2: move.w d2, -$1146(a5)
pub fn i_0275a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275A6;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0275A6: move.b -$1150(a5), -$1144(a5)
pub fn i_0275a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275AC;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0275AC: move.b -$114f(a5), -$1143(a5)
pub fn i_0275ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275B2;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== ClearXN_0275b2  ($0275B2, 6 B, thunks/ccr_helpers.c)
/// $0275B2: andi.b #$ee, ccr
pub fn i_0275b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275B6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0275B6: rts
pub fn i_0275b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SlopeCommit_0275b8  ($0275B8, 56 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0275B8: movea.l $4(a1), a3
pub fn i_0275b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275BC;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $0275BC: jsr $275f6(pc)
pub fn i_0275bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x275C0);
    cpu.pc = 0x275F6;
    16
}
/// $0275C0: bcs.b $2759a
pub fn i_0275c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275C2;
    cpu.ir = 0x65D8;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2759A; return 10; }
    8
}
/// $0275C2: addi.w #$1, d2
pub fn i_0275c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275C6;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0275C6: move.w d1, -$1148(a5)
pub fn i_0275c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275CA;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0275CA: move.w d2, -$1146(a5)
pub fn i_0275ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275CE;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0275CE: move.b #$0, -$1144(a5)
pub fn i_0275ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275D4;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0275D4: asr.w #$1, d3
pub fn i_0275d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275D6;
    cpu.ir = 0xE243;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0275D6: move.b $12(a3, d3.w), d2
pub fn i_0275d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275DA;
    cpu.ir = 0x1433;
    let ad2 = cpu.a[3].wrapping_add(0x12).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0275DA: move.b d2, -$1143(a5)
pub fn i_0275da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275DE;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0275DE: bset.b #$6, $5a(a6)
pub fn i_0275de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275E4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0275E4: move.b $106f44.l, d0
pub fn i_0275e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275EA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F44) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0275EA: jsr $999ca.l
pub fn i_0275ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x275F0);
    cpu.pc = 0x999CA;
    16
}
// ===== SetXN_0275f0  ($0275F0, 6 B, thunks/ccr_helpers.c)
/// $0275F0: ori.b #$11, ccr
pub fn i_0275f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275F4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0275F4: rts
pub fn i_0275f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SlopeResolve_0275f6  ($0275F6, 304 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0275F6: cmpi.b #$c, d0
pub fn i_0275f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275FA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0275FA: beq.w $276a8
pub fn i_0275fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $0275FE: cmpi.b #$d, d0
pub fn i_0275fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27602;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $027602: beq.w $276a8
pub fn i_027602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27606;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $027606: cmpi.b #$e, d0
pub fn i_027606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2760A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $02760A: beq.w $276a8
pub fn i_02760a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2760E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $02760E: cmpi.b #$f, d0
pub fn i_02760e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27612;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $027612: beq.w $276a8
pub fn i_027612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27616;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $027616: cmpi.b #$1c, d0
pub fn i_027616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2761A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1C, Size::Byte, false);
    8
}
/// $02761A: beq.w $276a8
pub fn i_02761a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2761E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $02761E: cmpi.b #$1d, d0
pub fn i_02761e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27622;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1D, Size::Byte, false);
    8
}
/// $027622: beq.w $276a8
pub fn i_027622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27626;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $027626: cmpi.b #$1e, d0
pub fn i_027626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2762A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1E, Size::Byte, false);
    8
}
/// $02762A: beq.w $276a8
pub fn i_02762a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2762E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $02762E: cmpi.b #$1f, d0
pub fn i_02762e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27632;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1F, Size::Byte, false);
    8
}
/// $027632: beq.w $276a8
pub fn i_027632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27636;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $027636: cmpi.b #$34, d0
pub fn i_027636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2763A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x34, Size::Byte, false);
    8
}
/// $02763A: beq.w $276a8
pub fn i_02763a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2763E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A8; return 10; }
    8
}
/// $02763E: cmpi.b #$2c, d0
pub fn i_02763e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27642;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2C, Size::Byte, false);
    8
}
/// $027642: beq.w $276a6
pub fn i_027642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27646;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A6; return 10; }
    8
}
/// $027646: cmpi.b #$2d, d0
pub fn i_027646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2764A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2D, Size::Byte, false);
    8
}
/// $02764A: beq.w $276a6
pub fn i_02764a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2764E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A6; return 10; }
    8
}
/// $02764E: cmpi.b #$2e, d0
pub fn i_02764e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27652;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2E, Size::Byte, false);
    8
}
/// $027652: beq.w $276a6
pub fn i_027652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27656;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A6; return 10; }
    8
}
/// $027656: cmpi.b #$2f, d0
pub fn i_027656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2765A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2F, Size::Byte, false);
    8
}
/// $02765A: beq.w $276a6
pub fn i_02765a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2765E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A6; return 10; }
    8
}
/// $02765E: cmpi.b #$3c, d0
pub fn i_02765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27662;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3C, Size::Byte, false);
    8
}
/// $027662: beq.w $276a6
pub fn i_027662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27666;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A6; return 10; }
    8
}
/// $027666: cmpi.b #$3d, d0
pub fn i_027666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2766A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3D, Size::Byte, false);
    8
}
/// $02766A: beq.w $276a6
pub fn i_02766a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2766E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A6; return 10; }
    8
}
/// $02766E: cmpi.b #$3e, d0
pub fn i_02766e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27672;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3E, Size::Byte, false);
    8
}
/// $027672: beq.w $276a6
pub fn i_027672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27676;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A6; return 10; }
    8
}
/// $027676: cmpi.b #$3f, d0
pub fn i_027676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2767A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3F, Size::Byte, false);
    8
}
/// $02767A: beq.w $276a6
pub fn i_02767a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2767E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A6; return 10; }
    8
}
/// $02767E: cmpi.b #$20, d0
pub fn i_02767e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27682;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $027682: beq.w $276a2
pub fn i_027682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27686;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A2; return 10; }
    8
}
/// $027686: cmpi.b #$21, d0
pub fn i_027686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2768A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $02768A: beq.w $276a2
pub fn i_02768a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2768E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A2; return 10; }
    8
}
/// $02768E: cmpi.b #$30, d0
pub fn i_02768e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27692;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $027692: beq.w $276a2
pub fn i_027692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27696;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A2; return 10; }
    8
}
/// $027696: cmpi.b #$31, d0
pub fn i_027696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2769A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $02769A: beq.w $276a2
pub fn i_02769a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2769E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276A2; return 10; }
    8
}
/// $02769E: bra.w $276d4
pub fn i_02769e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x276D4;
    10
}
/// $0276A2: bra.w $276a8
pub fn i_0276a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x276A8;
    10
}
/// $0276A6: addq.w #$8, d2
pub fn i_0276a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276A8;
    cpu.ir = 0x5042;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0276A8: addq.w #$8, d2
pub fn i_0276a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276AA;
    cpu.ir = 0x5042;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0276AA: jsr $27db2(pc)
pub fn i_0276aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x276AE);
    cpu.pc = 0x27DB2;
    16
}
/// $0276AE: jsr $9993c.l
pub fn i_0276ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x276B4);
    cpu.pc = 0x9993C;
    16
}
/// $0276B4: move.b d6, $106f44.l
pub fn i_0276b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276BA;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $0276BA: cmpi.b #$f, d6
pub fn i_0276ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276BE;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $0276BE: beq.w $276d4
pub fn i_0276be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x276D4; return 10; }
    8
}
/// $0276C2: moveq #$0, d0
pub fn i_0276c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276C4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0276C4: moveq #$0, d3
pub fn i_0276c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276C6;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0276C6: move.w d5, d4
pub fn i_0276c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276C8;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0276C8: moveq #$11, d0
pub fn i_0276c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276CA;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0276CA: lea.l $278ba8.l, a1
pub fn i_0276ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276D0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $0276D0: bra.w $276d4
pub fn i_0276d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x276D4;
    10
}
/// $0276D4: movea.l $4(a1), a2
pub fn i_0276d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276D8;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0276D8: move.w (a2), $34(a6)
pub fn i_0276d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276DC;
    cpu.ir = 0x3D52;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad3, v2 as u16);
    16
}
/// $0276DC: move.w $1a(a2), $64(a6)
pub fn i_0276dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276E2;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x64);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0276E2: cmpi.w #$80, $34(a6)
pub fn i_0276e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276E8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Word, false);
    16
}
/// $0276E8: bne.w $276fa
pub fn i_0276e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x276FA; return 10; }
    8
}
/// $0276EC: nop
pub fn i_0276ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276EE;
    cpu.ir = 0x4E71;
    4
}
/// $0276EE: nop
pub fn i_0276ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276F0;
    cpu.ir = 0x4E71;
    4
}
/// $0276F0: cmpi.w #$80, $34(a6)
pub fn i_0276f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276F6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Word, false);
    16
}
/// $0276F6: nop
pub fn i_0276f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276F8;
    cpu.ir = 0x4E71;
    4
}
/// $0276F8: trap #$f
pub fn i_0276f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276FA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0276FA: cmpi.w #$ff80, $34(a6)
pub fn i_0276fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27700;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $027700: bne.w $27712
pub fn i_027700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27704;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27712; return 10; }
    8
}
/// $027704: nop
pub fn i_027704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27706;
    cpu.ir = 0x4E71;
    4
}
/// $027706: nop
pub fn i_027706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27708;
    cpu.ir = 0x4E71;
    4
}
/// $027708: cmpi.w #$ff80, $34(a6)
pub fn i_027708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2770E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $02770E: nop
pub fn i_02770e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27710;
    cpu.ir = 0x4E71;
    4
}
/// $027710: trap #$f
pub fn i_027710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27712;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $027712: btst.b #$7, $13(a6)
pub fn i_027712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27718;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $027718: beq.w $2772c
pub fn i_027718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2771C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2772C; return 10; }
    8
}
/// $02771C: cmpi.w #$ffff, $34(a6)
pub fn i_02771c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27722;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $027722: bne.w $2772c
pub fn i_027722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27726;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2772C; return 10; }
    8
}
// ===== SetXN_027726  ($027726, 6 B, thunks/ccr_helpers.c)
/// $027726: ori.b #$11, ccr
pub fn i_027726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2772A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02772A: rts
pub fn i_02772a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2772C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SlopeTail_02772c  ($02772C, 10 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02772C: add.w d4, d2
pub fn i_02772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2772E;
    cpu.ir = 0xD444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02772E: asl.w #$1, d3
pub fn i_02772e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27730;
    cpu.ir = 0xE343;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $027730: move.w $2(a2, d3.w), d4
pub fn i_027730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27734;
    cpu.ir = 0x3832;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $027734: sub.w d4, d2
pub fn i_027734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27736;
    cpu.ir = 0x9444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
// ===== ClearXN_027736  ($027736, 6 B, thunks/ccr_helpers.c)
/// $027736: andi.b #$ee, ccr
pub fn i_027736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2773A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02773A: rts
pub fn i_02773a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2773C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MoveAndCollide_E_02773c  ($02773C, 136 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02773C: jsr $44182.l
pub fn i_02773c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27742);
    cpu.pc = 0x44182;
    16
}
/// $027742: jsr $26814(pc)
pub fn i_027742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27746;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27746);
    cpu.pc = 0x26814;
    16
}
/// $027746: jsr $26836(pc)
pub fn i_027746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2774A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2774A);
    cpu.pc = 0x26836;
    16
}
/// $02774A: move.w -$1148(a5), d1
pub fn i_02774a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2774E;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02774E: move.w -$1146(a5), d2
pub fn i_02774e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27752;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027752: add.w -$1154(a5), d1
pub fn i_027752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27756;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $027756: add.w -$1152(a5), d2
pub fn i_027756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2775A;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02775A: subq.w #$1, d2
pub fn i_02775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2775C;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02775C: jsr $27db2(pc)
pub fn i_02775c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27760;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27760);
    cpu.pc = 0x27DB2;
    16
}
/// $027760: jsr $9993c.l
pub fn i_027760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27766;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27766);
    cpu.pc = 0x9993C;
    16
}
/// $027766: move.b d6, $106f44.l
pub fn i_027766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2776C;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $02776C: cmpi.b #$f, d6
pub fn i_02776c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27770;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $027770: beq.w $27786
pub fn i_027770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27774;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27786; return 10; }
    8
}
/// $027774: moveq #$0, d0
pub fn i_027774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27776;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027776: moveq #$0, d3
pub fn i_027776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27778;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027778: move.w d5, d4
pub fn i_027778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2777A;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02777A: moveq #$11, d0
pub fn i_02777a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2777C;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02777C: lea.l $278ba8.l, a1
pub fn i_02777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27782;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $027782: bra.w $27786
pub fn i_027782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27786;
    cpu.ir = 0x6000;
    cpu.pc = 0x27786;
    10
}
/// $027786: cmpi.b #$0, d0
pub fn i_027786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2778A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02778A: beq.w $277c0
pub fn i_02778a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2778E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x277C0; return 10; }
    8
}
/// $02778E: jsr $27e62(pc)
pub fn i_02778e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27792;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27792);
    cpu.pc = 0x27E62;
    16
}
/// $027792: bcc.w $277c0
pub fn i_027792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27796;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x277C0; return 10; }
    8
}
/// $027796: tst.w $106f2e.l
pub fn i_027796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2779C;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F2E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $02779C: beq.w $277aa
pub fn i_02779c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x277AA; return 10; }
    8
}
/// $0277A0: tst.w $106f2e.l
pub fn i_0277a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277A6;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F2E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $0277A6: bra.w $277ae
pub fn i_0277a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x277AE;
    10
}
/// $0277AA: tst.w $2a(a6)
pub fn i_0277aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277AE;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0277AE: bge.w $277c0
pub fn i_0277ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277B2;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x277C0; return 10; }
    8
}
/// $0277B2: cmpi.b #$0, $9(a1)
pub fn i_0277b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277B8;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x9);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0277B8: bne.w $277c0
pub fn i_0277b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x277C0; return 10; }
    8
}
/// $0277BC: jmp $275b8(pc)
pub fn i_0277bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277C0;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x275B8;
    8
}
/// $0277C0: jmp $2759a(pc)
pub fn i_0277c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277C4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2759A;
    8
}
// ===== Entity_MoveAndCollide_F_0277c4  ($0277C4, 108 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0277C4: jsr $44182.l
pub fn i_0277c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x277CA);
    cpu.pc = 0x44182;
    16
}
/// $0277CA: jsr $26814(pc)
pub fn i_0277ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x277CE);
    cpu.pc = 0x26814;
    16
}
/// $0277CE: jsr $26836(pc)
pub fn i_0277ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x277D2);
    cpu.pc = 0x26836;
    16
}
/// $0277D2: move.w -$1148(a5), d1
pub fn i_0277d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277D6;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0277D6: move.w -$1146(a5), d2
pub fn i_0277d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277DA;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0277DA: add.w -$1154(a5), d1
pub fn i_0277da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277DE;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0277DE: add.w -$1152(a5), d2
pub fn i_0277de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277E2;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEAE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0277E2: subq.w #$1, d2
pub fn i_0277e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277E4;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0277E4: jsr $27db2(pc)
pub fn i_0277e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x277E8);
    cpu.pc = 0x27DB2;
    16
}
/// $0277E8: jsr $9993c.l
pub fn i_0277e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x277EE);
    cpu.pc = 0x9993C;
    16
}
/// $0277EE: move.b d6, $106f44.l
pub fn i_0277ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277F4;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $0277F4: cmpi.b #$f, d6
pub fn i_0277f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277F8;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $0277F8: beq.w $2780e
pub fn i_0277f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2780E; return 10; }
    8
}
/// $0277FC: moveq #$0, d0
pub fn i_0277fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277FE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0277FE: moveq #$0, d3
pub fn i_0277fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27800;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027800: move.w d5, d4
pub fn i_027800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27802;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027802: moveq #$11, d0
pub fn i_027802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27804;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027804: lea.l $278ba8.l, a1
pub fn i_027804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2780A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $02780A: bra.w $2780e
pub fn i_02780a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2780E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2780E;
    10
}
/// $02780E: cmpi.b #$0, d0
pub fn i_02780e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27812;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $027812: beq.w $27836
pub fn i_027812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27816;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27836; return 10; }
    8
}
/// $027816: jsr $27e62(pc)
pub fn i_027816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2781A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2781A);
    cpu.pc = 0x27E62;
    16
}
/// $02781A: bcc.w $27836
pub fn i_02781a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2781E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x27836; return 10; }
    8
}
/// $02781E: cmpi.b #$0, $9(a1)
pub fn i_02781e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27824;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x9);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $027824: bne.w $27836
pub fn i_027824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27828;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27836; return 10; }
    8
}
/// $027828: jsr $2759a(pc)
pub fn i_027828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2782C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2782C);
    cpu.pc = 0x2759A;
    16
}
/// $02782C: movea.l $4(a1), a3
pub fn i_02782c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27830;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
// ===== SetXN_027830  ($027830, 6 B, thunks/ccr_helpers.c)
/// $027830: ori.b #$11, ccr
pub fn i_027830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27834;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027834: rts
pub fn i_027834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27836;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CommitMove_Jmp_027836  ($027836, 4 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027836: jmp $2759a(pc)
pub fn i_027836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2783A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2759A;
    8
}
// ===== ActorCtxWrapper_02783a  ($02783A, 34 B, entity/actor_ctx_wrapper_02783a.s)
/// $02783A: movea.l #$108080, a5
pub fn i_02783a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27840;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027840: jsr $440e4.l
pub fn i_027840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27846);
    cpu.pc = 0x440E4;
    16
}
/// $027846: movem.w d5, -(a7)
pub fn i_027846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2784A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    cpu.a[7] = a;
    20
}
/// $02784A: movem.w d6, -(a7)
pub fn i_02784a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2784E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    cpu.a[7] = a;
    20
}
/// $02784E: jsr $28108(pc)
pub fn i_02784e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27852;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27852);
    cpu.pc = 0x28108;
    16
}
/// $027852: movem.w (a7)+, d6
pub fn i_027852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27856;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $027856: movem.w (a7)+, d5
pub fn i_027856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2785A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02785A: rts
pub fn i_02785a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2785C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_Probe_Scratch_02785C  ($02785C, 48 B, entity/probe_scratch_02785c.s)
/// $02785C: movea.l #$108080, a5
pub fn i_02785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27862;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027862: move.w $22(a6), -$1148(a5)
pub fn i_027862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27868;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027868: move.w $82(a6), -$1146(a5)
pub fn i_027868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2786E;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02786E: jsr $44182.l
pub fn i_02786e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27874);
    cpu.pc = 0x44182;
    16
}
/// $027874: move.w -$1148(a5), $22(a6)
pub fn i_027874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2787A;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02787A: move.w -$1146(a5), $82(a6)
pub fn i_02787a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27880;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027880: andi.w #$3ff, $82(a6)
pub fn i_027880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27886;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $027886: jsr $28108(pc)
pub fn i_027886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2788A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2788A);
    cpu.pc = 0x28108;
    16
}
/// $02788A: rts
pub fn i_02788a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2788C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeRevertCcr_02788C  ($02788C, 118 B, entity/probe_revert_ccr_02788c.s)
/// $02788C: movea.l #$108080, a5
pub fn i_02788c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27892;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027892: move.w $22(a6), -$1148(a5)
pub fn i_027892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27898;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027898: move.w $82(a6), -$1146(a5)
pub fn i_027898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2789E;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02789E: move.b $26(a6), -$1144(a5)
pub fn i_02789e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278A4;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0278A4: move.b $27(a6), -$1143(a5)
pub fn i_0278a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278AA;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0278AA: jsr $27036(pc)
pub fn i_0278aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x278AE);
    cpu.pc = 0x27036;
    16
}
/// $0278AE: bcc.w $278da
pub fn i_0278ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x278DA; return 10; }
    8
}
/// $0278B2: move.w -$1148(a5), $22(a6)
pub fn i_0278b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278B8;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0278B8: move.w -$1146(a5), $82(a6)
pub fn i_0278b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278BE;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0278BE: move.b -$1144(a5), $26(a6)
pub fn i_0278be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278C4;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0278C4: move.b -$1143(a5), $27(a6)
pub fn i_0278c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278CA;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0278CA: andi.w #$3ff, $82(a6)
pub fn i_0278ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278D0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0278D0: jsr $28108(pc)
pub fn i_0278d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x278D4);
    cpu.pc = 0x28108;
    16
}
/// $0278D4: ori.b #$11, ccr
pub fn i_0278d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278D8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0278D8: rts
pub fn i_0278d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0278DA: move.w -$1148(a5), $22(a6)
pub fn i_0278da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278E0;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0278E0: move.b -$1144(a5), $26(a6)
pub fn i_0278e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278E6;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0278E6: move.w -$1146(a5), $82(a6)
pub fn i_0278e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278EC;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0278EC: move.b -$1143(a5), $27(a6)
pub fn i_0278ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278F2;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0278F2: andi.w #$3ff, $82(a6)
pub fn i_0278f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278F8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0278F8: jsr $28108(pc)
pub fn i_0278f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x278FC);
    cpu.pc = 0x28108;
    16
}
/// $0278FC: andi.b #$ee, ccr
pub fn i_0278fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27900;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027900: rts
pub fn i_027900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27902;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepB_UpdatePos_027902  ($027902, 66 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027902: movea.l #$108080, a5
pub fn i_027902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27908;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027908: move.w $22(a6), -$1148(a5)
pub fn i_027908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2790E;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02790E: move.w $24(a6), -$1146(a5)
pub fn i_02790e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27914;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027914: move.b $26(a6), -$1144(a5)
pub fn i_027914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2791A;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $02791A: move.b $27(a6), -$1143(a5)
pub fn i_02791a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27920;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027920: jsr $27036(pc)
pub fn i_027920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27924;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27924);
    cpu.pc = 0x27036;
    16
}
/// $027924: bcs.w $2794a
pub fn i_027924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27928;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2794A; return 10; }
    8
}
/// $027928: move.w -$1148(a5), $22(a6)
pub fn i_027928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2792E;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02792E: move.w -$1146(a5), $24(a6)
pub fn i_02792e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27934;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027934: move.b -$1144(a5), $26(a6)
pub fn i_027934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2793A;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $02793A: move.b -$1143(a5), $27(a6)
pub fn i_02793a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27940;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027940: jsr $28108(pc)
pub fn i_027940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27944;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27944);
    cpu.pc = 0x28108;
    16
}
// ===== ClearXN_027944  ($027944, 6 B, thunks/ccr_helpers.c)
/// $027944: andi.b #$ee, ccr
pub fn i_027944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27948;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027948: rts
pub fn i_027948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2794A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepB_UpdatePos_Tail_02794a  ($02794A, 28 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02794A: move.w -$1148(a5), $22(a6)
pub fn i_02794a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27950;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027950: move.w -$1146(a5), $24(a6)
pub fn i_027950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27956;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027956: move.b -$1144(a5), $26(a6)
pub fn i_027956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2795C;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $02795C: move.b -$1143(a5), $27(a6)
pub fn i_02795c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27962;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027962: jsr $28108(pc)
pub fn i_027962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27966;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27966);
    cpu.pc = 0x28108;
    16
}
// ===== SetXN_027966  ($027966, 6 B, thunks/ccr_helpers.c)
/// $027966: ori.b #$11, ccr
pub fn i_027966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2796A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02796A: rts
pub fn i_02796a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2796C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepC_UpdatePosY82_02796c  ($02796C, 72 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02796C: movea.l #$108080, a5
pub fn i_02796c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27972;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027972: move.w $22(a6), -$1148(a5)
pub fn i_027972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27978;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027978: move.w $82(a6), -$1146(a5)
pub fn i_027978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2797E;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02797E: move.b $26(a6), -$1144(a5)
pub fn i_02797e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27984;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027984: move.b $27(a6), -$1143(a5)
pub fn i_027984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2798A;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $02798A: jsr $272a8(pc)
pub fn i_02798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2798E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2798E);
    cpu.pc = 0x272A8;
    16
}
/// $02798E: bcc.w $279ba
pub fn i_02798e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27992;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x279BA; return 10; }
    8
}
/// $027992: move.w -$1148(a5), $22(a6)
pub fn i_027992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27998;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027998: move.w -$1146(a5), $82(a6)
pub fn i_027998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2799E;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02799E: move.b -$1144(a5), $26(a6)
pub fn i_02799e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279A4;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0279A4: move.b -$1143(a5), $27(a6)
pub fn i_0279a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279AA;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0279AA: andi.w #$3ff, $82(a6)
pub fn i_0279aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279B0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0279B0: jsr $28108(pc)
pub fn i_0279b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x279B4);
    cpu.pc = 0x28108;
    16
}
// ===== SetXN_0279b4  ($0279B4, 6 B, thunks/ccr_helpers.c)
/// $0279B4: ori.b #$11, ccr
pub fn i_0279b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279B8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0279B8: rts
pub fn i_0279b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepC_UpdatePosY82_Tail_0279ba  ($0279BA, 34 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0279BA: move.w -$1148(a5), $22(a6)
pub fn i_0279ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279C0;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0279C0: move.b -$1144(a5), $26(a6)
pub fn i_0279c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279C6;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0279C6: move.w -$1146(a5), $82(a6)
pub fn i_0279c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279CC;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0279CC: move.b -$1143(a5), $27(a6)
pub fn i_0279cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279D2;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0279D2: andi.w #$3ff, $82(a6)
pub fn i_0279d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279D8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0279D8: jsr $28108(pc)
pub fn i_0279d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x279DC);
    cpu.pc = 0x28108;
    16
}
// ===== ClearXN_0279dc  ($0279DC, 6 B, thunks/ccr_helpers.c)
/// $0279DC: andi.b #$ee, ccr
pub fn i_0279dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279E0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0279E0: rts
pub fn i_0279e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepD_UpdatePosY82_0279e2  ($0279E2, 134 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0279E2: movea.l #$108080, a5
pub fn i_0279e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279E8;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $0279E8: move.w #$ffff, $34(a6)
pub fn i_0279e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0279EE: move.w #$ffff, $64(a6)
pub fn i_0279ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x64);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0279F4: move.w $22(a6), -$1148(a5)
pub fn i_0279f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279FA;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0279FA: move.w $82(a6), -$1146(a5)
pub fn i_0279fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A00;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027A00: move.b $26(a6), -$1144(a5)
pub fn i_027a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A06;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027A06: move.b $27(a6), -$1143(a5)
pub fn i_027a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A0C;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027A0C: jsr $274cc(pc)
pub fn i_027a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27A10);
    cpu.pc = 0x274CC;
    16
}
/// $027A10: bcc.w $27a6e
pub fn i_027a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x27A6E; return 10; }
    8
}
/// $027A14: bra.w $27a3e
pub fn i_027a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A18;
    cpu.ir = 0x6000;
    cpu.pc = 0x27A3E;
    10
}
/// $027A18: movea.l #$108080, a5
pub fn i_027a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A1E;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027A1E: move.w $22(a6), -$1148(a5)
pub fn i_027a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A24;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027A24: move.w $82(a6), -$1146(a5)
pub fn i_027a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A2A;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027A2A: move.b $26(a6), -$1144(a5)
pub fn i_027a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A30;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027A30: move.b $27(a6), -$1143(a5)
pub fn i_027a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A36;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027A36: jsr $27444(pc)
pub fn i_027a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27A3A);
    cpu.pc = 0x27444;
    16
}
/// $027A3A: bcc.w $27a6e
pub fn i_027a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x27A6E; return 10; }
    8
}
/// $027A3E: move.w #$0, $2e(a6)
pub fn i_027a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A44;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027A44: move.w #$0, $2a(a6)
pub fn i_027a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A4A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027A4A: move.w -$1148(a5), $22(a6)
pub fn i_027a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A50;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027A50: move.w -$1146(a5), $82(a6)
pub fn i_027a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A56;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027A56: move.b -$1144(a5), $26(a6)
pub fn i_027a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A5C;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027A5C: move.b -$1143(a5), $27(a6)
pub fn i_027a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A62;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027A62: andi.w #$3ff, $82(a6)
pub fn i_027a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A68;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetXN_027a68  ($027A68, 6 B, thunks/ccr_helpers.c)
/// $027A68: ori.b #$11, ccr
pub fn i_027a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A6C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027A6C: rts
pub fn i_027a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepD_UpdatePosY82_Tail_027a6e  ($027A6E, 30 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027A6E: move.w -$1148(a5), $22(a6)
pub fn i_027a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A74;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027A74: move.w -$1146(a5), $82(a6)
pub fn i_027a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A7A;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027A7A: move.b -$1144(a5), $26(a6)
pub fn i_027a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A80;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027A80: move.b -$1143(a5), $27(a6)
pub fn i_027a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A86;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027A86: andi.w #$3ff, $82(a6)
pub fn i_027a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A8C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== ClearXN_027a8c  ($027A8C, 6 B, thunks/ccr_helpers.c)
/// $027A8C: andi.b #$ee, ccr
pub fn i_027a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A90;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027A90: rts
pub fn i_027a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeRevertCcr_027A92  ($027A92, 106 B, entity/probe_revert_ccr_027a92.s)
/// $027A92: movea.l #$108080, a5
pub fn i_027a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A98;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027A98: move.w $22(a6), -$1148(a5)
pub fn i_027a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A9E;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027A9E: move.w $24(a6), -$1146(a5)
pub fn i_027a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AA4;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027AA4: move.b $26(a6), -$1144(a5)
pub fn i_027aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AAA;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027AAA: move.b $27(a6), -$1143(a5)
pub fn i_027aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AB0;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027AB0: jsr $26b56(pc)
pub fn i_027ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27AB4);
    cpu.pc = 0x26B56;
    16
}
/// $027AB4: bcs.w $27ada
pub fn i_027ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AB8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27ADA; return 10; }
    8
}
/// $027AB8: move.w -$1148(a5), $22(a6)
pub fn i_027ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27ABE;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027ABE: move.w -$1146(a5), $24(a6)
pub fn i_027abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AC4;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027AC4: move.b -$1144(a5), $26(a6)
pub fn i_027ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27ACA;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027ACA: move.b -$1143(a5), $27(a6)
pub fn i_027aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AD0;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027AD0: jsr $28108(pc)
pub fn i_027ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27AD4);
    cpu.pc = 0x28108;
    16
}
/// $027AD4: andi.b #$ee, ccr
pub fn i_027ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AD8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027AD8: rts
pub fn i_027ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27ADA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $027ADA: move.w -$1148(a5), $22(a6)
pub fn i_027ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AE0;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027AE0: move.w -$1146(a5), $24(a6)
pub fn i_027ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AE6;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027AE6: move.b -$1144(a5), $26(a6)
pub fn i_027ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AEC;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027AEC: move.b -$1143(a5), $27(a6)
pub fn i_027aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AF2;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027AF2: jsr $28108(pc)
pub fn i_027af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27AF6);
    cpu.pc = 0x28108;
    16
}
/// $027AF6: ori.b #$11, ccr
pub fn i_027af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AFA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027AFA: rts
pub fn i_027afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeRevertCcr_027AFC  ($027AFC, 106 B, entity/probe_revert_ccr_027afc.s)
/// $027AFC: movea.l #$108080, a5
pub fn i_027afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B02;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027B02: move.w $22(a6), -$1148(a5)
pub fn i_027b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B08;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B08: move.w $24(a6), -$1146(a5)
pub fn i_027b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B0E;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B0E: move.b $26(a6), -$1144(a5)
pub fn i_027b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B14;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B14: move.b $27(a6), -$1143(a5)
pub fn i_027b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B1A;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B1A: jsr $272a8(pc)
pub fn i_027b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27B1E);
    cpu.pc = 0x272A8;
    16
}
/// $027B1E: bcs.w $27b44
pub fn i_027b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B22;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27B44; return 10; }
    8
}
/// $027B22: move.w -$1148(a5), $22(a6)
pub fn i_027b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B28;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B28: move.w -$1146(a5), $24(a6)
pub fn i_027b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B2E;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B2E: move.b -$1144(a5), $26(a6)
pub fn i_027b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B34;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B34: move.b -$1143(a5), $27(a6)
pub fn i_027b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B3A;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B3A: jsr $28108(pc)
pub fn i_027b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27B3E);
    cpu.pc = 0x28108;
    16
}
/// $027B3E: andi.b #$ee, ccr
pub fn i_027b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B42;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027B42: rts
pub fn i_027b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $027B44: move.w -$1148(a5), $22(a6)
pub fn i_027b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B4A;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B4A: move.w -$1146(a5), $24(a6)
pub fn i_027b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B50;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B50: move.b -$1144(a5), $26(a6)
pub fn i_027b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B56;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B56: move.b -$1143(a5), $27(a6)
pub fn i_027b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B5C;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B5C: jsr $28108(pc)
pub fn i_027b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27B60);
    cpu.pc = 0x28108;
    16
}
/// $027B60: ori.b #$11, ccr
pub fn i_027b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B64;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027B64: rts
pub fn i_027b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepFall_UpdatePos_027b66  ($027B66, 62 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027B66: movea.l #$108080, a5
pub fn i_027b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B6C;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027B6C: move.w $22(a6), -$1148(a5)
pub fn i_027b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B72;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B72: move.w $24(a6), -$1146(a5)
pub fn i_027b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B78;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B78: move.b $26(a6), -$1144(a5)
pub fn i_027b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B7E;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B7E: move.b $27(a6), -$1143(a5)
pub fn i_027b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B84;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B84: jsr $27464(pc)
pub fn i_027b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B88;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27B88);
    cpu.pc = 0x27464;
    16
}
/// $027B88: bcs.w $27baa
pub fn i_027b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B8C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27BAA; return 10; }
    8
}
/// $027B8C: move.w -$1148(a5), $22(a6)
pub fn i_027b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B92;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B92: move.w -$1146(a5), $24(a6)
pub fn i_027b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B98;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027B98: move.b -$1144(a5), $26(a6)
pub fn i_027b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B9E;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027B9E: move.b -$1143(a5), $27(a6)
pub fn i_027b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BA4;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== ClearXN_027ba4  ($027BA4, 6 B, thunks/ccr_helpers.c)
/// $027BA4: andi.b #$ee, ccr
pub fn i_027ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BA8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027BA8: rts
pub fn i_027ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepFall_UpdatePos_Tail_027baa  ($027BAA, 24 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027BAA: move.w -$1148(a5), $22(a6)
pub fn i_027baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BB0;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027BB0: move.w -$1146(a5), $24(a6)
pub fn i_027bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BB6;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027BB6: move.b -$1144(a5), $26(a6)
pub fn i_027bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BBC;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027BBC: move.b -$1143(a5), $27(a6)
pub fn i_027bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BC2;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetXN_027bc2  ($027BC2, 6 B, thunks/ccr_helpers.c)
/// $027BC2: ori.b #$11, ccr
pub fn i_027bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BC6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027BC6: rts
pub fn i_027bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeTransformFreeCcr_027bc8  ($027BC8, 68 B, entity/probe_transform_027bc8.s)
/// $027BC8: movea.l #$108080, a5
pub fn i_027bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BCE;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027BCE: move.w $22(a6), -$1148(a5)
pub fn i_027bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BD4;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027BD4: move.w $24(a6), -$1146(a5)
pub fn i_027bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BDA;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027BDA: move.b $26(a6), -$1144(a5)
pub fn i_027bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BE0;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027BE0: move.b $27(a6), -$1143(a5)
pub fn i_027be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BE6;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027BE6: jsr $27444(pc)
pub fn i_027be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27BEA);
    cpu.pc = 0x27444;
    16
}
/// $027BEA: bcs.w $27c0c
pub fn i_027bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BEE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27C0C; return 10; }
    8
}
/// $027BEE: move.w -$1148(a5), $22(a6)
pub fn i_027bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BF4;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027BF4: move.w -$1146(a5), $24(a6)
pub fn i_027bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BFA;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027BFA: move.b -$1144(a5), $26(a6)
pub fn i_027bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C00;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C00: move.b -$1143(a5), $27(a6)
pub fn i_027c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C06;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C06: andi.b #$ee, ccr
pub fn i_027c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C0A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027C0A: rts
pub fn i_027c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_RestoreTransformSetC_027c0c  ($027C0C, 30 B, entity/restore_transform_027c0c.s)
/// $027C0C: move.w -$1148(a5), $22(a6)
pub fn i_027c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C12;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C12: move.w -$1146(a5), $24(a6)
pub fn i_027c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C18;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C18: move.b -$1144(a5), $26(a6)
pub fn i_027c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C1E;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C1E: move.b -$1143(a5), $27(a6)
pub fn i_027c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C24;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C24: ori.b #$11, ccr
pub fn i_027c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C28;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027C28: rts
pub fn i_027c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeTransformFreeCcr_027c2a  ($027C2A, 68 B, entity/probe_transform_027c2a.s)
/// $027C2A: movea.l #$108080, a5
pub fn i_027c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C30;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027C30: move.w $22(a6), -$1148(a5)
pub fn i_027c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C36;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C36: move.w $24(a6), -$1146(a5)
pub fn i_027c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C3C;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C3C: move.b $26(a6), -$1144(a5)
pub fn i_027c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C42;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C42: move.b $27(a6), -$1143(a5)
pub fn i_027c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C48;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C48: jsr $27444(pc)
pub fn i_027c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27C4C);
    cpu.pc = 0x27444;
    16
}
/// $027C4C: bcs.w $27c6e
pub fn i_027c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C50;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27C6E; return 10; }
    8
}
/// $027C50: move.w -$1148(a5), $22(a6)
pub fn i_027c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C56;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C56: move.w -$1146(a5), $24(a6)
pub fn i_027c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C5C;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C5C: move.b -$1144(a5), $26(a6)
pub fn i_027c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C62;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C62: move.b -$1143(a5), $27(a6)
pub fn i_027c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C68;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C68: andi.b #$ee, ccr
pub fn i_027c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C6C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027C6C: rts
pub fn i_027c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_RestoreTransformSetC_027c6e  ($027C6E, 30 B, entity/restore_transform_027c6e.s)
/// $027C6E: move.w -$1148(a5), $22(a6)
pub fn i_027c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C74;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C74: move.w -$1146(a5), $24(a6)
pub fn i_027c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C7A;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C7A: move.b -$1144(a5), $26(a6)
pub fn i_027c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C80;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C80: move.b -$1143(a5), $27(a6)
pub fn i_027c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C86;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027C86: ori.b #$11, ccr
pub fn i_027c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C8A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027C8A: rts
pub fn i_027c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeTransformFreeCcr_027c8c  ($027C8C, 68 B, entity/probe_transform_027c8c.s)
/// $027C8C: movea.l #$108080, a5
pub fn i_027c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C92;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027C92: move.w $22(a6), -$1148(a5)
pub fn i_027c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C98;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C98: move.w $24(a6), -$1146(a5)
pub fn i_027c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C9E;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027C9E: move.b $26(a6), -$1144(a5)
pub fn i_027c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CA4;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027CA4: move.b $27(a6), -$1143(a5)
pub fn i_027ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CAA;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027CAA: jsr $273fc(pc)
pub fn i_027caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27CAE);
    cpu.pc = 0x273FC;
    16
}
/// $027CAE: bcs.w $27cd0
pub fn i_027cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CB2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27CD0; return 10; }
    8
}
/// $027CB2: move.w -$1148(a5), $22(a6)
pub fn i_027cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CB8;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027CB8: move.w -$1146(a5), $24(a6)
pub fn i_027cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CBE;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027CBE: move.b -$1144(a5), $26(a6)
pub fn i_027cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CC4;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027CC4: move.b -$1143(a5), $27(a6)
pub fn i_027cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CCA;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027CCA: andi.b #$ee, ccr
pub fn i_027cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CCE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027CCE: rts
pub fn i_027cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_RestoreTransformSetC_027cd0  ($027CD0, 30 B, entity/restore_transform_027cd0.s)
/// $027CD0: move.w -$1148(a5), $22(a6)
pub fn i_027cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CD6;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027CD6: move.w -$1146(a5), $24(a6)
pub fn i_027cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CDC;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027CDC: move.b -$1144(a5), $26(a6)
pub fn i_027cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CE2;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027CE2: move.b -$1143(a5), $27(a6)
pub fn i_027ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CE8;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027CE8: ori.b #$11, ccr
pub fn i_027ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CEC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027CEC: rts
pub fn i_027cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeTransformFreeCcr  ($027CEE, 68 B, entity/probe_transform_027cee.s)
/// $027CEE: movea.l #$108080, a5
pub fn i_027cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CF4;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027CF4: move.w $22(a6), -$1148(a5)
pub fn i_027cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CFA;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027CFA: move.w $24(a6), -$1146(a5)
pub fn i_027cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D00;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D00: move.b $26(a6), -$1144(a5)
pub fn i_027d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D06;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D06: move.b $27(a6), -$1143(a5)
pub fn i_027d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D0C;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D0C: jsr $277c4(pc)
pub fn i_027d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27D10);
    cpu.pc = 0x277C4;
    16
}
/// $027D10: bcs.w $27d32
pub fn i_027d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D14;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27D32; return 10; }
    8
}
/// $027D14: move.w -$1148(a5), $22(a6)
pub fn i_027d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D1A;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D1A: move.w -$1146(a5), $24(a6)
pub fn i_027d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D20;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D20: move.b -$1144(a5), $26(a6)
pub fn i_027d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D26;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D26: move.b -$1143(a5), $27(a6)
pub fn i_027d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D2C;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D2C: andi.b #$ee, ccr
pub fn i_027d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D30;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027D30: rts
pub fn i_027d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_RestoreTransformSetC_027d32  ($027D32, 30 B, entity/restore_transform_027d32.s)
/// $027D32: move.w -$1148(a5), $22(a6)
pub fn i_027d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D38;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D38: move.w -$1146(a5), $24(a6)
pub fn i_027d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D3E;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D3E: move.b -$1144(a5), $26(a6)
pub fn i_027d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D44;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D44: move.b -$1143(a5), $27(a6)
pub fn i_027d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D4A;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D4A: ori.b #$11, ccr
pub fn i_027d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D4E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027D4E: rts
pub fn i_027d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeTransformFreeCcr_027d50  ($027D50, 68 B, entity/probe_transform_027d50.s)
/// $027D50: movea.l #$108080, a5
pub fn i_027d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D56;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $027D56: move.w $22(a6), -$1148(a5)
pub fn i_027d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D5C;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D5C: move.w $24(a6), -$1146(a5)
pub fn i_027d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D62;
    cpu.ir = 0x3B6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D62: move.b $26(a6), -$1144(a5)
pub fn i_027d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D68;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D68: move.b $27(a6), -$1143(a5)
pub fn i_027d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D6E;
    cpu.ir = 0x1B6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D6E: jsr $2773c(pc)
pub fn i_027d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27D72);
    cpu.pc = 0x2773C;
    16
}
/// $027D72: bcs.w $27d94
pub fn i_027d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D76;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27D94; return 10; }
    8
}
/// $027D76: move.w -$1148(a5), $22(a6)
pub fn i_027d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D7C;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D7C: move.w -$1146(a5), $24(a6)
pub fn i_027d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D82;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D82: move.b -$1144(a5), $26(a6)
pub fn i_027d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D88;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D88: move.b -$1143(a5), $27(a6)
pub fn i_027d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D8E;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027D8E: andi.b #$ee, ccr
pub fn i_027d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D92;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027D92: rts
pub fn i_027d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_RestoreTransformSetC_027d94  ($027D94, 30 B, entity/restore_transform_027d94.s)
/// $027D94: move.w -$1148(a5), $22(a6)
pub fn i_027d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D9A;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027D9A: move.w -$1146(a5), $24(a6)
pub fn i_027d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DA0;
    cpu.ir = 0x3D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $027DA0: move.b -$1144(a5), $26(a6)
pub fn i_027da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DA6;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027DA6: move.b -$1143(a5), $27(a6)
pub fn i_027da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DAC;
    cpu.ir = 0x1D6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $027DAC: ori.b #$11, ccr
pub fn i_027dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DB0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027DB0: rts
pub fn i_027db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CollMap_LookupTile_027db2  ($027DB2, 118 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027DB2: move.w d1, d5
pub fn i_027db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DB4;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027DB4: move.w d2, d6
pub fn i_027db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DB6;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027DB6: move.w d1, d0
pub fn i_027db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DB8;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027DB8: move.w d2, d1
pub fn i_027db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DBA;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027DBA: cmpi.w #$1ff, d1
pub fn i_027dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DBE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1FF, Size::Word, false);
    8
}
/// $027DBE: bgt.w $27dcc
pub fn i_027dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DC2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x27DCC; return 10; }
    8
}
/// $027DC2: jsr $43f02.l
pub fn i_027dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27DC8);
    cpu.pc = 0x43F02;
    16
}
/// $027DC8: bra.w $27dce
pub fn i_027dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x27DCE;
    10
}
/// $027DCC: moveq #$0, d0
pub fn i_027dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DCE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027DCE: move.b $6b(a6), d2
pub fn i_027dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DD2;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $027DD2: btst.b #$2, $5b(a6)
pub fn i_027dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DD8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $027DD8: beq.w $27dee
pub fn i_027dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DDC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27DEE; return 10; }
    8
}
/// $027DDC: move.b d0, d7
pub fn i_027ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DDE;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $027DDE: andi.b #$c0, d7
pub fn i_027dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DE2;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $027DE2: cmpi.b #$c0, d7
pub fn i_027de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DE6;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Byte, false);
    8
}
/// $027DE6: bne.w $27dee
pub fn i_027de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27DEE; return 10; }
    8
}
/// $027DEA: move.b #$0, d0
pub fn i_027dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DEE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $027DEE: move.b d0, d7
pub fn i_027dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DF0;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $027DF0: andi.b #$3f, d0
pub fn i_027df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DF4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $027DF4: lea.l $278988.l, a1
pub fn i_027df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DFA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278988;
    20
}
/// $027DFA: move.w d0, d1
pub fn i_027dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DFC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027DFC: andi.l #$3f, d1
pub fn i_027dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E02;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $027E02: asl.l #$5, d1
pub fn i_027e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E04;
    cpu.ir = 0xEB81;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Long, 0, true);
    cpu.d[1] = r;
    16
}
/// $027E04: adda.l d1, a1
pub fn i_027e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E06;
    cpu.ir = 0xD3C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $027E06: and.b $c(a1), d2
pub fn i_027e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E0A;
    cpu.ir = 0xC429;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $027E0A: beq.w $27e22
pub fn i_027e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27E22; return 10; }
    8
}
/// $027E0E: moveq #$0, d2
pub fn i_027e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E10;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027E10: moveq #$0, d0
pub fn i_027e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E12;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027E12: move.b $d(a1), d1
pub fn i_027e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E16;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0xD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $027E16: cmpi.b #$ff, d1
pub fn i_027e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E1A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $027E1A: beq.w $27e22
pub fn i_027e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27E22; return 10; }
    8
}
/// $027E1E: move.b d1, d0
pub fn i_027e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E20;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $027E20: bra.b $27dd2
pub fn i_027e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E22;
    cpu.ir = 0x60B0;
    cpu.pc = 0x27DD2;
    10
}
/// $027E22: move.w d5, d1
pub fn i_027e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E24;
    cpu.ir = 0x3205;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027E24: move.w d6, d2
pub fn i_027e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E26;
    cpu.ir = 0x3406;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027E26: rts
pub fn i_027e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CollMap_TestSolidOrPlatform_027e28  ($027E28, 46 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027E28: movea.l (a1), a0
pub fn i_027e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E2A;
    cpu.ir = 0x2051;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[0] = v2;
    12
}
/// $027E2A: move.b (a0, d4.w), d5
pub fn i_027e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E2E;
    cpu.ir = 0x1A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $027E2E: moveq #$7, d6
pub fn i_027e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E30;
    cpu.ir = 0x7C07;
    cpu.d[6] = 0x7;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027E30: sub.w d3, d6
pub fn i_027e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E32;
    cpu.ir = 0x9C43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $027E32: btst.l d6, d5
pub fn i_027e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E34;
    cpu.ir = 0xD05;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $027E34: beq.w $27e3c
pub fn i_027e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27E3C; return 10; }
    8
}
/// $027E38: bra.w $27e56
pub fn i_027e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x27E56;
    10
}
/// $027E3C: movea.l $10(a1), a0
pub fn i_027e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E40;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $027E40: move.b (a0, d4.w), d5
pub fn i_027e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E44;
    cpu.ir = 0x1A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $027E44: move.w #$7, d6
pub fn i_027e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E48;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027E48: sub.w d3, d6
pub fn i_027e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E4A;
    cpu.ir = 0x9C43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $027E4A: btst.l d6, d5
pub fn i_027e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E4C;
    cpu.ir = 0xD05;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $027E4C: beq.w $27e5c
pub fn i_027e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27E5C; return 10; }
    8
}
/// $027E50: adda.l #$10, a1
pub fn i_027e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E56;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x10;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
// ===== SetXN_027e56  ($027E56, 6 B, thunks/ccr_helpers.c)
/// $027E56: ori.b #$11, ccr
pub fn i_027e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E5A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027E5A: rts
pub fn i_027e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_027e5c  ($027E5C, 6 B, thunks/ccr_helpers.c)
/// $027E5C: andi.b #$ee, ccr
pub fn i_027e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E60;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027E60: rts
pub fn i_027e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CollMap_TestSolidBit_027e62  ($027E62, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027E62: movea.l (a1), a0
pub fn i_027e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E64;
    cpu.ir = 0x2051;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[0] = v2;
    12
}
/// $027E64: move.b (a0, d4.w), d5
pub fn i_027e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E68;
    cpu.ir = 0x1A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $027E68: moveq #$7, d6
pub fn i_027e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E6A;
    cpu.ir = 0x7C07;
    cpu.d[6] = 0x7;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027E6A: sub.w d3, d6
pub fn i_027e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E6C;
    cpu.ir = 0x9C43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $027E6C: btst.l d6, d5
pub fn i_027e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E6E;
    cpu.ir = 0xD05;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $027E6E: beq.w $27e78
pub fn i_027e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E72;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27E78; return 10; }
    8
}
// ===== SetXN_027e72  ($027E72, 6 B, thunks/ccr_helpers.c)
/// $027E72: ori.b #$11, ccr
pub fn i_027e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E76;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027E76: rts
pub fn i_027e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_027e78  ($027E78, 6 B, thunks/ccr_helpers.c)
/// $027E78: andi.b #$ee, ccr
pub fn i_027e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E7C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027E7C: rts
pub fn i_027e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CollMap_TestSolidBitC_027e7e  ($027E7E, 18 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027E7E: movea.l (a1), a0
pub fn i_027e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E80;
    cpu.ir = 0x2051;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[0] = v2;
    12
}
/// $027E80: move.b (a0, d4.w), d0
pub fn i_027e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E84;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $027E84: move.w #$7, d6
pub fn i_027e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E88;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027E88: sub.w d3, d6
pub fn i_027e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E8A;
    cpu.ir = 0x9C43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $027E8A: btst.l d6, d0
pub fn i_027e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E8C;
    cpu.ir = 0xD00;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $027E8C: beq.w $27e96
pub fn i_027e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27E96; return 10; }
    8
}
// ===== SetXN_027e90  ($027E90, 6 B, thunks/ccr_helpers.c)
/// $027E90: ori.b #$11, ccr
pub fn i_027e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E94;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027E94: rts
pub fn i_027e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_027e96  ($027E96, 6 B, thunks/ccr_helpers.c)
/// $027E96: andi.b #$ee, ccr
pub fn i_027e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E9A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027E9A: rts
pub fn i_027e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CollMap_TestPlatformBitC_027e9c  ($027E9C, 18 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027E9C: movea.l (a1), a0
pub fn i_027e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E9E;
    cpu.ir = 0x2051;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[0] = v2;
    12
}
/// $027E9E: move.b $8(a0, d4.w), d0
pub fn i_027e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EA2;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x8).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $027EA2: move.w #$7, d6
pub fn i_027ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EA6;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027EA6: sub.w d3, d6
pub fn i_027ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EA8;
    cpu.ir = 0x9C43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $027EA8: btst.l d6, d0
pub fn i_027ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EAA;
    cpu.ir = 0xD00;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $027EAA: beq.w $27eb4
pub fn i_027eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27EB4; return 10; }
    8
}
// ===== SetXN_027eae  ($027EAE, 6 B, thunks/ccr_helpers.c)
/// $027EAE: ori.b #$11, ccr
pub fn i_027eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EB2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027EB2: rts
pub fn i_027eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_027eb4  ($027EB4, 6 B, thunks/ccr_helpers.c)
/// $027EB4: andi.b #$ee, ccr
pub fn i_027eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EB8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027EB8: rts
pub fn i_027eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpritePubEffect_027EBA  ($027EBA, 78 B, player/route_publish_027eba.s)
/// $027EBA: move.w $22(a6), d1
pub fn i_027eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EBE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027EBE: move.w $24(a6), d2
pub fn i_027ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EC2;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027EC2: subq.w #$1, d2
pub fn i_027ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EC4;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $027EC4: jsr $27db2(pc)
pub fn i_027ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27EC8);
    cpu.pc = 0x27DB2;
    16
}
/// $027EC8: jsr $9993c.l
pub fn i_027ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27ECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27ECE);
    cpu.pc = 0x9993C;
    16
}
/// $027ECE: move.b d6, $106f44.l
pub fn i_027ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27ED4;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $027ED4: cmpi.b #$f, d6
pub fn i_027ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27ED8;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $027ED8: beq.w $27eee
pub fn i_027ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EDC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27EEE; return 10; }
    8
}
/// $027EDC: moveq #$0, d0
pub fn i_027edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EDE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027EDE: moveq #$0, d3
pub fn i_027ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EE0;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027EE0: move.w d5, d4
pub fn i_027ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EE2;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027EE2: moveq #$11, d0
pub fn i_027ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EE4;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027EE4: lea.l $278ba8.l, a1
pub fn i_027ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EEA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $027EEA: bra.w $27eee
pub fn i_027eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x27EEE;
    10
}
/// $027EEE: jsr $27e28(pc)
pub fn i_027eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27EF2);
    cpu.pc = 0x27E28;
    16
}
/// $027EF2: bcc.w $27f02
pub fn i_027ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x27F02; return 10; }
    8
}
/// $027EF6: bset.b #$6, $5a(a6)
pub fn i_027ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EFC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $027EFC: andi.b #$ee, ccr
pub fn i_027efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F00;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027F00: rts
pub fn i_027f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $027F02: ori.b #$11, ccr
pub fn i_027f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F06;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027F06: rts
pub fn i_027f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_FloorBelowProbe_027f08  ($027F08, 74 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027F08: jsr $2783a(pc)
pub fn i_027f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27F0C);
    cpu.pc = 0x2783A;
    16
}
/// $027F0C: move.w $22(a6), d1
pub fn i_027f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F10;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027F10: move.w $24(a6), d2
pub fn i_027f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F14;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027F14: subq.w #$1, d2
pub fn i_027f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F16;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $027F16: jsr $27db2(pc)
pub fn i_027f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27F1A);
    cpu.pc = 0x27DB2;
    16
}
/// $027F1A: jsr $9993c.l
pub fn i_027f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x27F20);
    cpu.pc = 0x9993C;
    16
}
/// $027F20: move.b d6, $106f44.l
pub fn i_027f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F26;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $027F26: cmpi.b #$f, d6
pub fn i_027f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F2A;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $027F2A: beq.w $27f40
pub fn i_027f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x27F40; return 10; }
    8
}
/// $027F2E: moveq #$0, d0
pub fn i_027f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F30;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027F30: moveq #$0, d3
pub fn i_027f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F32;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027F32: move.w d5, d4
pub fn i_027f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F34;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $027F34: moveq #$11, d0
pub fn i_027f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F36;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $027F36: lea.l $278ba8.l, a1
pub fn i_027f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F3C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $027F3C: bra.w $27f40
pub fn i_027f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F40;
    cpu.ir = 0x6000;
    cpu.pc = 0x27F40;
    10
}
/// $027F40: jsr $27e28(pc)
pub fn i_027f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27F44);
    cpu.pc = 0x27E28;
    16
}
/// $027F44: bcc.w $27f58
pub fn i_027f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x27F58; return 10; }
    8
}
/// $027F48: bset.b #$6, $5a(a6)
pub fn i_027f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F4E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $027F4E: move.b #$ff, d3
pub fn i_027f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F52;
    cpu.ir = 0x163C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== ClearXN_027f52  ($027F52, 6 B, thunks/ccr_helpers.c)
/// $027F52: andi.b #$ee, ccr
pub fn i_027f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F56;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027F56: rts
pub fn i_027f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_FloorBelowProbe_Tail_027f58  ($027F58, 2 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027F58: clr.b d3
pub fn i_027f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F5A;
    cpu.ir = 0x4203;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== SetXN_027f5a  ($027F5A, 6 B, thunks/ccr_helpers.c)
/// $027F5A: ori.b #$11, ccr
pub fn i_027f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F5E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027F5E: rts
pub fn i_027f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TestGroundUnder_027f60  ($027F60, 76 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027F60: move.w $28(a6), -(a7)
pub fn i_027f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F64;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $027F64: move.w $2a(a6), -(a7)
pub fn i_027f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F68;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $027F68: move.w $22(a6), -(a7)
pub fn i_027f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F6C;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $027F6C: move.w $24(a6), -(a7)
pub fn i_027f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F70;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $027F70: addi.w #$1, $24(a6)
pub fn i_027f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F76;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $027F76: clr.w $28(a6)
pub fn i_027f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F7A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $027F7A: move.w #$ff00, $2a(a6)
pub fn i_027f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $027F80: jsr $27bc8(pc)
pub fn i_027f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27F84);
    cpu.pc = 0x27BC8;
    16
}
/// $027F84: bcs.w $27f92
pub fn i_027f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F88;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x27F92; return 10; }
    8
}
/// $027F88: clr.b d3
pub fn i_027f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F8A;
    cpu.ir = 0x4203;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $027F8A: ori.b #$11, ccr
pub fn i_027f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F8E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027F8E: bra.w $27f9a
pub fn i_027f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F92;
    cpu.ir = 0x6000;
    cpu.pc = 0x27F9A;
    10
}
/// $027F92: move.b #$ff, d3
pub fn i_027f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F96;
    cpu.ir = 0x163C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $027F96: andi.b #$ee, ccr
pub fn i_027f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F9A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027F9A: move.w (a7)+, $24(a6)
pub fn i_027f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F9E;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    16
}
/// $027F9E: move.w (a7)+, $22(a6)
pub fn i_027f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FA2;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    16
}
/// $027FA2: move.w (a7)+, $2a(a6)
pub fn i_027fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FA6;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    16
}
/// $027FA6: move.w (a7)+, $28(a6)
pub fn i_027fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FAA;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    16
}
/// $027FAA: rts
pub fn i_027faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TestSolidAtFeet_027fac  ($027FAC, 32 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027FAC: move.w $22(a6), d1
pub fn i_027fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FB0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027FB0: move.w $24(a6), d2
pub fn i_027fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FB4;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027FB4: subq.w #$1, d2
pub fn i_027fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FB6;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $027FB6: jsr $27db2(pc)
pub fn i_027fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FBA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27FBA);
    cpu.pc = 0x27DB2;
    16
}
/// $027FBA: jsr $27e62(pc)
pub fn i_027fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27FBE);
    cpu.pc = 0x27E62;
    16
}
/// $027FBE: bcc.w $27fd2
pub fn i_027fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x27FD2; return 10; }
    8
}
/// $027FC2: cmpi.b #$0, $9(a1)
pub fn i_027fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FC8;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x9);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $027FC8: bne.w $27fd2
pub fn i_027fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FCC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27FD2; return 10; }
    8
}
// ===== ClearXN_027fcc  ($027FCC, 6 B, thunks/ccr_helpers.c)
/// $027FCC: andi.b #$ee, ccr
pub fn i_027fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FD0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $027FD0: rts
pub fn i_027fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_027fd2  ($027FD2, 6 B, thunks/ccr_helpers.c)
/// $027FD2: ori.b #$11, ccr
pub fn i_027fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FD6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $027FD6: rts
pub fn i_027fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TestSolidAtFeetCtx_027fd8  ($027FD8, 40 B, entity/collmap_hitbox_script_ops_027400.s)
/// $027FD8: jsr $2783a(pc)
pub fn i_027fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27FDC);
    cpu.pc = 0x2783A;
    16
}
/// $027FDC: move.w $22(a6), d1
pub fn i_027fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FE0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027FE0: move.w $24(a6), d2
pub fn i_027fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FE4;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $027FE4: subq.w #$1, d2
pub fn i_027fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FE6;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $027FE6: jsr $27db2(pc)
pub fn i_027fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27FEA);
    cpu.pc = 0x27DB2;
    16
}
/// $027FEA: jsr $27e62(pc)
pub fn i_027fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x27FEE);
    cpu.pc = 0x27E62;
    16
}
/// $027FEE: bcc.w $28006
pub fn i_027fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28006; return 10; }
    8
}
/// $027FF2: cmpi.b #$0, $9(a1)
pub fn i_027ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FF8;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x9);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $027FF8: bne.w $28006
pub fn i_027ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FFC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28006; return 10; }
    8
}
/// $027FFC: move.b #$ff, d3
pub fn i_027ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28000;
    cpu.ir = 0x163C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== ClearXN_028000  ($028000, 6 B, thunks/ccr_helpers.c)
/// $028000: andi.b #$ee, ccr
pub fn i_028000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28004;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028004: rts
pub fn i_028004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28006;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TestSolidAtFeet_Tail_028006  ($028006, 2 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028006: clr.b d3
pub fn i_028006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28008;
    cpu.ir = 0x4203;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== SetXN_028008  ($028008, 6 B, thunks/ccr_helpers.c)
/// $028008: ori.b #$11, ccr
pub fn i_028008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2800C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02800C: rts
pub fn i_02800c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2800E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ScaleDyForGravity_02800e  ($02800E, 54 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02800E: cmpi.w #$400, $34(a6)
pub fn i_02800e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28014;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x400, Size::Word, false);
    16
}
/// $028014: beq.w $28024
pub fn i_028014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28018;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28024; return 10; }
    8
}
/// $028018: cmpi.w #$fc00, $34(a6)
pub fn i_028018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2801E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFC00, Size::Word, false);
    16
}
/// $02801E: beq.w $28024
pub fn i_02801e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28022;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28024; return 10; }
    8
}
/// $028022: rts
pub fn i_028022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28024;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $028024: cmpi.b #$1, $106ece.l
pub fn i_028024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2802C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $02802C: bne.w $28038
pub fn i_02802c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28030;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28038; return 10; }
    8
}
/// $028030: tst.w $106f5e.l
pub fn i_028030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28036;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $028036: bne.b $28022
pub fn i_028036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28038;
    cpu.ir = 0x66EA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28022; return 10; }
    8
}
/// $028038: move.w d2, d0
pub fn i_028038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2803A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02803A: muls.w #$aa, d0
pub fn i_02803a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2803E;
    cpu.ir = 0xC1FC;
    let v1: u32 = 0xAA;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $02803E: asr.l #$8, d0
pub fn i_02803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28040;
    cpu.ir = 0xE080;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[0] = r;
    22
}
/// $028040: move.w d0, d2
pub fn i_028040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28042;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028042: rts
pub fn i_028042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28044;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TestD1Any_028044  ($028044, 6 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028044: tst.w d1
pub fn i_028044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28046;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $028046: bra.w $2806e
pub fn i_028046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2804A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2806E;
    10
}
// ===== Entity_TestD1Pos_02804a  ($02804A, 10 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02804A: tst.w d1
pub fn i_02804a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2804C;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02804C: ble.w $2806e
pub fn i_02804c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28050;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2806E; return 10; }
    8
}
/// $028050: bra.w $28068
pub fn i_028050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28054;
    cpu.ir = 0x6000;
    cpu.pc = 0x28068;
    10
}
// ===== Entity_TestD1Neg_028054  ($028054, 10 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028054: tst.w d1
pub fn i_028054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28056;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $028056: bge.w $2806e
pub fn i_028056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2805A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2806E; return 10; }
    8
}
/// $02805A: bra.w $28068
pub fn i_02805a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2805E;
    cpu.ir = 0x6000;
    cpu.pc = 0x28068;
    10
}
// ===== SetXN_028068  ($028068, 6 B, thunks/ccr_helpers.c)
/// $028068: ori.b #$11, ccr
pub fn i_028068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2806C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02806C: rts
pub fn i_02806c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2806E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02806e  ($02806E, 6 B, thunks/ccr_helpers.c)
/// $02806E: andi.b #$ee, ccr
pub fn i_02806e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28072;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028072: rts
pub fn i_028072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28074;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ClampXToScreenEdge_028074  ($028074, 62 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028074: cmpi.b #$9, $106ece.l
pub fn i_028074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2807C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x9, Size::Byte, false);
    20
}
/// $02807C: beq.w $280ac
pub fn i_02807c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28080;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x280AC; return 10; }
    8
}
/// $028080: move.w #$1e, d1
pub fn i_028080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28084;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028084: sub.w $22(a6), d1
pub fn i_028084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28088;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028088: ble.w $28096
pub fn i_028088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2808C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28096; return 10; }
    8
}
/// $02808C: tst.w d2
pub fn i_02808c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2808E;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02808E: blt.w $280b8
pub fn i_02808e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28092;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x280B8; return 10; }
    8
}
/// $028092: bra.w $280ba
pub fn i_028092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28096;
    cpu.ir = 0x6000;
    cpu.pc = 0x280BA;
    10
}
/// $028096: move.w #$117, d1
pub fn i_028096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2809A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x117;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02809A: sub.w $22(a6), d1
pub fn i_02809a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2809E;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02809E: bge.w $280ac
pub fn i_02809e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280A2;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x280AC; return 10; }
    8
}
/// $0280A2: tst.w d2
pub fn i_0280a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280A4;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0280A4: bgt.w $280b8
pub fn i_0280a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280A8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x280B8; return 10; }
    8
}
/// $0280A8: bra.w $280ba
pub fn i_0280a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x280BA;
    10
}
/// $0280AC: bset.b #$7, $13(a6)
pub fn i_0280ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280B2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearXN_0280b2  ($0280B2, 6 B, thunks/ccr_helpers.c)
/// $0280B2: andi.b #$ee, ccr
pub fn i_0280b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280B6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0280B6: rts
pub fn i_0280b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ClampX_ZeroDx_0280b8  ($0280B8, 8 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0280B8: moveq #$0, d2
pub fn i_0280b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280BA;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0280BA: bclr.b #$7, $13(a6)
pub fn i_0280ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280C0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x026818, i_026818),
    (0x02681C, i_02681c),
    (0x026820, i_026820),
    (0x026824, i_026824),
    (0x026828, i_026828),
    (0x02682C, i_02682c),
    (0x026830, i_026830),
    (0x026834, i_026834),
    (0x026836, i_026836),
    (0x02683C, i_02683c),
    (0x026842, i_026842),
    (0x026846, i_026846),
    (0x02684A, i_02684a),
    (0x026850, i_026850),
    (0x026854, i_026854),
    (0x026856, i_026856),
    (0x02685A, i_02685a),
    (0x02685C, i_02685c),
    (0x026860, i_026860),
    (0x026866, i_026866),
    (0x02686A, i_02686a),
    (0x02686C, i_02686c),
    (0x026870, i_026870),
    (0x026872, i_026872),
    (0x026874, i_026874),
    (0x02687C, i_02687c),
    (0x026880, i_026880),
    (0x0268C2, i_0268c2),
    (0x0268CA, i_0268ca),
    (0x0268CE, i_0268ce),
    (0x0268D4, i_0268d4),
    (0x0268D8, i_0268d8),
    (0x0268DE, i_0268de),
    (0x0268E2, i_0268e2),
    (0x0268E6, i_0268e6),
    (0x0268EC, i_0268ec),
    (0x0268F0, i_0268f0),
    (0x0268F2, i_0268f2),
    (0x0268F8, i_0268f8),
    (0x0268FC, i_0268fc),
    (0x026902, i_026902),
    (0x026906, i_026906),
    (0x02690C, i_02690c),
    (0x026910, i_026910),
    (0x026916, i_026916),
    (0x02691A, i_02691a),
    (0x02691E, i_02691e),
    (0x026922, i_026922),
    (0x026926, i_026926),
    (0x026928, i_026928),
    (0x02692A, i_02692a),
    (0x02692E, i_02692e),
    (0x026930, i_026930),
    (0x026932, i_026932),
    (0x026936, i_026936),
    (0x026938, i_026938),
    (0x02693C, i_02693c),
    (0x02693E, i_02693e),
    (0x026942, i_026942),
    (0x026946, i_026946),
    (0x02694E, i_02694e),
    (0x026952, i_026952),
    (0x026958, i_026958),
    (0x02695C, i_02695c),
    (0x026962, i_026962),
    (0x026966, i_026966),
    (0x02696A, i_02696a),
    (0x026970, i_026970),
    (0x026974, i_026974),
    (0x026978, i_026978),
    (0x02697C, i_02697c),
    (0x02697E, i_02697e),
    (0x026980, i_026980),
    (0x026982, i_026982),
    (0x026986, i_026986),
    (0x026988, i_026988),
    (0x02698C, i_02698c),
    (0x026990, i_026990),
    (0x026992, i_026992),
    (0x026998, i_026998),
    (0x02699E, i_02699e),
    (0x0269A2, i_0269a2),
    (0x0269A6, i_0269a6),
    (0x0269AC, i_0269ac),
    (0x0269B0, i_0269b0),
    (0x0269B2, i_0269b2),
    (0x0269B6, i_0269b6),
    (0x0269B8, i_0269b8),
    (0x0269BC, i_0269bc),
    (0x0269C2, i_0269c2),
    (0x0269C6, i_0269c6),
    (0x0269C8, i_0269c8),
    (0x0269CC, i_0269cc),
    (0x0269CE, i_0269ce),
    (0x0269D0, i_0269d0),
    (0x0269D8, i_0269d8),
    (0x0269DC, i_0269dc),
    (0x026A1E, i_026a1e),
    (0x026A24, i_026a24),
    (0x026A28, i_026a28),
    (0x026A2E, i_026a2e),
    (0x026A32, i_026a32),
    (0x026A38, i_026a38),
    (0x026A3C, i_026a3c),
    (0x026A42, i_026a42),
    (0x026A46, i_026a46),
    (0x026A4A, i_026a4a),
    (0x026A4E, i_026a4e),
    (0x026A52, i_026a52),
    (0x026A54, i_026a54),
    (0x026A58, i_026a58),
    (0x026A5A, i_026a5a),
    (0x026A5E, i_026a5e),
    (0x026A60, i_026a60),
    (0x026A62, i_026a62),
    (0x026A66, i_026a66),
    (0x026A68, i_026a68),
    (0x026A6C, i_026a6c),
    (0x026A70, i_026a70),
    (0x026A74, i_026a74),
    (0x026A76, i_026a76),
    (0x026A7A, i_026a7a),
    (0x026A7E, i_026a7e),
    (0x026A82, i_026a82),
    (0x026A84, i_026a84),
    (0x026A88, i_026a88),
    (0x026A8C, i_026a8c),
    (0x026A8E, i_026a8e),
    (0x026A90, i_026a90),
    (0x026A92, i_026a92),
    (0x026A96, i_026a96),
    (0x026A9A, i_026a9a),
    (0x026A9E, i_026a9e),
    (0x026AA2, i_026aa2),
    (0x026AA6, i_026aa6),
    (0x026AAA, i_026aaa),
    (0x026AAE, i_026aae),
    (0x026AB2, i_026ab2),
    (0x026AB4, i_026ab4),
    (0x026AB6, i_026ab6),
    (0x026ABA, i_026aba),
    (0x026ABC, i_026abc),
    (0x026ABE, i_026abe),
    (0x026AC2, i_026ac2),
    (0x026AC8, i_026ac8),
    (0x026ACC, i_026acc),
    (0x026AD0, i_026ad0),
    (0x026AD4, i_026ad4),
    (0x026AD6, i_026ad6),
    (0x026AD8, i_026ad8),
    (0x026ADC, i_026adc),
    (0x026AE0, i_026ae0),
    (0x026AE4, i_026ae4),
    (0x026AE8, i_026ae8),
    (0x026AEC, i_026aec),
    (0x026AF0, i_026af0),
    (0x026AF2, i_026af2),
    (0x026AF4, i_026af4),
    (0x026AF6, i_026af6),
    (0x026AF8, i_026af8),
    (0x026AFA, i_026afa),
    (0x026AFE, i_026afe),
    (0x026B00, i_026b00),
    (0x026B04, i_026b04),
    (0x026B08, i_026b08),
    (0x026B10, i_026b10),
    (0x026B14, i_026b14),
    (0x026B1A, i_026b1a),
    (0x026B1E, i_026b1e),
    (0x026B24, i_026b24),
    (0x026B28, i_026b28),
    (0x026B2C, i_026b2c),
    (0x026B32, i_026b32),
    (0x026B36, i_026b36),
    (0x026B3C, i_026b3c),
    (0x026B40, i_026b40),
    (0x026B44, i_026b44),
    (0x026B48, i_026b48),
    (0x026B4E, i_026b4e),
    (0x026B54, i_026b54),
    (0x026B56, i_026b56),
    (0x026B5A, i_026b5a),
    (0x026B5E, i_026b5e),
    (0x026B64, i_026b64),
    (0x026B68, i_026b68),
    (0x026B6C, i_026b6c),
    (0x026B6E, i_026b6e),
    (0x026B72, i_026b72),
    (0x026B76, i_026b76),
    (0x026B7A, i_026b7a),
    (0x026B80, i_026b80),
    (0x026B84, i_026b84),
    (0x026B86, i_026b86),
    (0x026B88, i_026b88),
    (0x026B8A, i_026b8a),
    (0x026B8C, i_026b8c),
    (0x026B8E, i_026b8e),
    (0x026B94, i_026b94),
    (0x026B96, i_026b96),
    (0x026B9A, i_026b9a),
    (0x026BA2, i_026ba2),
    (0x026BA6, i_026ba6),
    (0x026BAA, i_026baa),
    (0x026BAE, i_026bae),
    (0x026BB4, i_026bb4),
    (0x026BB8, i_026bb8),
    (0x026BBC, i_026bbc),
    (0x026BC0, i_026bc0),
    (0x026BC4, i_026bc4),
    (0x026BC8, i_026bc8),
    (0x026BCC, i_026bcc),
    (0x026BD0, i_026bd0),
    (0x026BD4, i_026bd4),
    (0x026BD8, i_026bd8),
    (0x026BDC, i_026bdc),
    (0x026BE0, i_026be0),
    (0x026BE2, i_026be2),
    (0x026BE6, i_026be6),
    (0x026BEC, i_026bec),
    (0x026BF0, i_026bf0),
    (0x026BF4, i_026bf4),
    (0x026BF8, i_026bf8),
    (0x026BFC, i_026bfc),
    (0x026C00, i_026c00),
    (0x026C04, i_026c04),
    (0x026C08, i_026c08),
    (0x026C0C, i_026c0c),
    (0x026C10, i_026c10),
    (0x026C14, i_026c14),
    (0x026C18, i_026c18),
    (0x026C1C, i_026c1c),
    (0x026C20, i_026c20),
    (0x026C24, i_026c24),
    (0x026C28, i_026c28),
    (0x026C2E, i_026c2e),
    (0x026C32, i_026c32),
    (0x026C36, i_026c36),
    (0x026C3A, i_026c3a),
    (0x026C40, i_026c40),
    (0x026C44, i_026c44),
    (0x026C4A, i_026c4a),
    (0x026C4E, i_026c4e),
    (0x026C52, i_026c52),
    (0x026C54, i_026c54),
    (0x026C58, i_026c58),
    (0x026C5C, i_026c5c),
    (0x026C60, i_026c60),
    (0x026C62, i_026c62),
    (0x026C66, i_026c66),
    (0x026C6C, i_026c6c),
    (0x026C70, i_026c70),
    (0x026C74, i_026c74),
    (0x026C78, i_026c78),
    (0x026C7E, i_026c7e),
    (0x026C82, i_026c82),
    (0x026C86, i_026c86),
    (0x026C8A, i_026c8a),
    (0x026C8E, i_026c8e),
    (0x026C92, i_026c92),
    (0x026C96, i_026c96),
    (0x026C9A, i_026c9a),
    (0x026C9E, i_026c9e),
    (0x026CA2, i_026ca2),
    (0x026CA6, i_026ca6),
    (0x026CAA, i_026caa),
    (0x026CAE, i_026cae),
    (0x026CB2, i_026cb2),
    (0x026CB6, i_026cb6),
    (0x026CBA, i_026cba),
    (0x026CBE, i_026cbe),
    (0x026CC2, i_026cc2),
    (0x026CC6, i_026cc6),
    (0x026CCA, i_026cca),
    (0x026CCE, i_026cce),
    (0x026CD2, i_026cd2),
    (0x026CD8, i_026cd8),
    (0x026CDC, i_026cdc),
    (0x026CE0, i_026ce0),
    (0x026CE4, i_026ce4),
    (0x026CEA, i_026cea),
    (0x026CEE, i_026cee),
    (0x026CF4, i_026cf4),
    (0x026CF8, i_026cf8),
    (0x026CFC, i_026cfc),
    (0x026CFE, i_026cfe),
    (0x026D02, i_026d02),
    (0x026D06, i_026d06),
    (0x026D0A, i_026d0a),
    (0x026D0C, i_026d0c),
    (0x026D10, i_026d10),
    (0x026D16, i_026d16),
    (0x026D1A, i_026d1a),
    (0x026D20, i_026d20),
    (0x026D24, i_026d24),
    (0x026D2A, i_026d2a),
    (0x026D2E, i_026d2e),
    (0x026D32, i_026d32),
    (0x026D34, i_026d34),
    (0x026D38, i_026d38),
    (0x026D3C, i_026d3c),
    (0x026D40, i_026d40),
    (0x026D42, i_026d42),
    (0x026D46, i_026d46),
    (0x026D4A, i_026d4a),
    (0x026D4E, i_026d4e),
    (0x026D52, i_026d52),
    (0x026D56, i_026d56),
    (0x026D5C, i_026d5c),
    (0x026D60, i_026d60),
    (0x026D64, i_026d64),
    (0x026D68, i_026d68),
    (0x026D6C, i_026d6c),
    (0x026D6E, i_026d6e),
    (0x026D72, i_026d72),
    (0x026D76, i_026d76),
    (0x026D78, i_026d78),
    (0x026D7C, i_026d7c),
    (0x026D80, i_026d80),
    (0x026D84, i_026d84),
    (0x026D88, i_026d88),
    (0x026D8C, i_026d8c),
    (0x026D8E, i_026d8e),
    (0x026D92, i_026d92),
    (0x026D96, i_026d96),
    (0x026D9A, i_026d9a),
    (0x026D9C, i_026d9c),
    (0x026DA2, i_026da2),
    (0x026DA4, i_026da4),
    (0x026DA8, i_026da8),
    (0x026DAC, i_026dac),
    (0x026DB0, i_026db0),
    (0x026DB4, i_026db4),
    (0x026DB8, i_026db8),
    (0x026DBC, i_026dbc),
    (0x026DC0, i_026dc0),
    (0x026DC2, i_026dc2),
    (0x026DC6, i_026dc6),
    (0x026DCA, i_026dca),
    (0x026DCE, i_026dce),
    (0x026DD2, i_026dd2),
    (0x026DD6, i_026dd6),
    (0x026DDA, i_026dda),
    (0x026DF2, i_026df2),
    (0x026DF6, i_026df6),
    (0x026DFA, i_026dfa),
    (0x026DFE, i_026dfe),
    (0x026E02, i_026e02),
    (0x026E06, i_026e06),
    (0x026E0A, i_026e0a),
    (0x026E0E, i_026e0e),
    (0x026E14, i_026e14),
    (0x026E18, i_026e18),
    (0x026E1E, i_026e1e),
    (0x026E22, i_026e22),
    (0x026E26, i_026e26),
    (0x026E2A, i_026e2a),
    (0x026E2E, i_026e2e),
    (0x026E32, i_026e32),
    (0x026E36, i_026e36),
    (0x026E3A, i_026e3a),
    (0x026E3C, i_026e3c),
    (0x026E42, i_026e42),
    (0x026E46, i_026e46),
    (0x026E4A, i_026e4a),
    (0x026E4E, i_026e4e),
    (0x026E52, i_026e52),
    (0x026E56, i_026e56),
    (0x026E5A, i_026e5a),
    (0x026E5E, i_026e5e),
    (0x026E60, i_026e60),
    (0x026E64, i_026e64),
    (0x026E68, i_026e68),
    (0x026E6C, i_026e6c),
    (0x026E70, i_026e70),
    (0x026E72, i_026e72),
    (0x026E76, i_026e76),
    (0x026E7A, i_026e7a),
    (0x026E7E, i_026e7e),
    (0x026E82, i_026e82),
    (0x026E84, i_026e84),
    (0x026E86, i_026e86),
    (0x026E88, i_026e88),
    (0x026E8A, i_026e8a),
    (0x026E8C, i_026e8c),
    (0x026E90, i_026e90),
    (0x026E96, i_026e96),
    (0x026E9C, i_026e9c),
    (0x026EA0, i_026ea0),
    (0x026EA4, i_026ea4),
    (0x026EA6, i_026ea6),
    (0x026EA8, i_026ea8),
    (0x026EAA, i_026eaa),
    (0x026EAC, i_026eac),
    (0x026EB2, i_026eb2),
    (0x026EB6, i_026eb6),
    (0x026EBA, i_026eba),
    (0x026EBE, i_026ebe),
    (0x026EC2, i_026ec2),
    (0x026EC6, i_026ec6),
    (0x026ECA, i_026eca),
    (0x026ECE, i_026ece),
    (0x026ED2, i_026ed2),
    (0x026ED6, i_026ed6),
    (0x026EDA, i_026eda),
    (0x026EDE, i_026ede),
    (0x026EE2, i_026ee2),
    (0x026EE6, i_026ee6),
    (0x026EEA, i_026eea),
    (0x026EEE, i_026eee),
    (0x026EF2, i_026ef2),
    (0x026EF6, i_026ef6),
    (0x026EFA, i_026efa),
    (0x026EFE, i_026efe),
    (0x026F02, i_026f02),
    (0x026F06, i_026f06),
    (0x026F0A, i_026f0a),
    (0x026F0E, i_026f0e),
    (0x026F12, i_026f12),
    (0x026F16, i_026f16),
    (0x026F1A, i_026f1a),
    (0x026F1E, i_026f1e),
    (0x026F22, i_026f22),
    (0x026F26, i_026f26),
    (0x026F2A, i_026f2a),
    (0x026F2E, i_026f2e),
    (0x026F32, i_026f32),
    (0x026F36, i_026f36),
    (0x026F3A, i_026f3a),
    (0x026F3E, i_026f3e),
    (0x026F42, i_026f42),
    (0x026F46, i_026f46),
    (0x026F4A, i_026f4a),
    (0x026F4E, i_026f4e),
    (0x026F52, i_026f52),
    (0x026F56, i_026f56),
    (0x026F5A, i_026f5a),
    (0x026F5E, i_026f5e),
    (0x026F62, i_026f62),
    (0x026F68, i_026f68),
    (0x026F6C, i_026f6c),
    (0x026F70, i_026f70),
    (0x026F72, i_026f72),
    (0x026F74, i_026f74),
    (0x026F78, i_026f78),
    (0x026F7E, i_026f7e),
    (0x026F84, i_026f84),
    (0x026F88, i_026f88),
    (0x026F8C, i_026f8c),
    (0x026F8E, i_026f8e),
    (0x026F90, i_026f90),
    (0x026F92, i_026f92),
    (0x026F94, i_026f94),
    (0x026F9A, i_026f9a),
    (0x026F9E, i_026f9e),
    (0x026FA2, i_026fa2),
    (0x026FA6, i_026fa6),
    (0x026FAC, i_026fac),
    (0x026FB2, i_026fb2),
    (0x026FB6, i_026fb6),
    (0x026FB8, i_026fb8),
    (0x026FBA, i_026fba),
    (0x026FC0, i_026fc0),
    (0x026FC2, i_026fc2),
    (0x026FC4, i_026fc4),
    (0x026FCA, i_026fca),
    (0x026FCE, i_026fce),
    (0x026FD0, i_026fd0),
    (0x026FD2, i_026fd2),
    (0x026FD8, i_026fd8),
    (0x026FDA, i_026fda),
    (0x026FDC, i_026fdc),
    (0x026FDE, i_026fde),
    (0x026FE2, i_026fe2),
    (0x026FE4, i_026fe4),
    (0x026FE6, i_026fe6),
    (0x026FE8, i_026fe8),
    (0x026FEC, i_026fec),
    (0x026FF0, i_026ff0),
    (0x026FF4, i_026ff4),
    (0x026FF8, i_026ff8),
    (0x026FFA, i_026ffa),
    (0x027000, i_027000),
    (0x027004, i_027004),
    (0x027008, i_027008),
    (0x02700C, i_02700c),
    (0x027010, i_027010),
    (0x027014, i_027014),
    (0x027018, i_027018),
    (0x02701C, i_02701c),
    (0x027020, i_027020),
    (0x027024, i_027024),
    (0x027028, i_027028),
    (0x02702C, i_02702c),
    (0x027030, i_027030),
    (0x027034, i_027034),
    (0x027036, i_027036),
    (0x02703A, i_02703a),
    (0x02703E, i_02703e),
    (0x027044, i_027044),
    (0x027048, i_027048),
    (0x02704C, i_02704c),
    (0x02704E, i_02704e),
    (0x027052, i_027052),
    (0x027056, i_027056),
    (0x02705A, i_02705a),
    (0x027060, i_027060),
    (0x027064, i_027064),
    (0x027066, i_027066),
    (0x027068, i_027068),
    (0x02706A, i_02706a),
    (0x02706C, i_02706c),
    (0x02706E, i_02706e),
    (0x027074, i_027074),
    (0x027076, i_027076),
    (0x02707A, i_02707a),
    (0x027082, i_027082),
    (0x027086, i_027086),
    (0x02708A, i_02708a),
    (0x02708E, i_02708e),
    (0x027094, i_027094),
    (0x027098, i_027098),
    (0x02709C, i_02709c),
    (0x0270A0, i_0270a0),
    (0x0270A4, i_0270a4),
    (0x0270A8, i_0270a8),
    (0x0270AC, i_0270ac),
    (0x0270B0, i_0270b0),
    (0x0270B4, i_0270b4),
    (0x0270B8, i_0270b8),
    (0x0270BC, i_0270bc),
    (0x0270C0, i_0270c0),
    (0x0270C4, i_0270c4),
    (0x0270C6, i_0270c6),
    (0x0270CA, i_0270ca),
    (0x0270D0, i_0270d0),
    (0x0270D4, i_0270d4),
    (0x0270D8, i_0270d8),
    (0x0270DC, i_0270dc),
    (0x0270E0, i_0270e0),
    (0x0270E4, i_0270e4),
    (0x0270E8, i_0270e8),
    (0x0270EC, i_0270ec),
    (0x0270F0, i_0270f0),
    (0x0270F4, i_0270f4),
    (0x0270F8, i_0270f8),
    (0x0270FC, i_0270fc),
    (0x027100, i_027100),
    (0x027104, i_027104),
    (0x027108, i_027108),
    (0x02710C, i_02710c),
    (0x027112, i_027112),
    (0x027116, i_027116),
    (0x02711A, i_02711a),
    (0x02711E, i_02711e),
    (0x027124, i_027124),
    (0x027128, i_027128),
    (0x02712E, i_02712e),
    (0x027132, i_027132),
    (0x027136, i_027136),
    (0x027138, i_027138),
    (0x02713C, i_02713c),
    (0x027140, i_027140),
    (0x027144, i_027144),
    (0x027146, i_027146),
    (0x02714A, i_02714a),
    (0x027150, i_027150),
    (0x027154, i_027154),
    (0x027158, i_027158),
    (0x02715C, i_02715c),
    (0x027162, i_027162),
    (0x027166, i_027166),
    (0x02716A, i_02716a),
    (0x02716E, i_02716e),
    (0x027172, i_027172),
    (0x027176, i_027176),
    (0x02717A, i_02717a),
    (0x02717E, i_02717e),
    (0x027182, i_027182),
    (0x027186, i_027186),
    (0x02718A, i_02718a),
    (0x02718E, i_02718e),
    (0x027192, i_027192),
    (0x027196, i_027196),
    (0x02719A, i_02719a),
    (0x02719E, i_02719e),
    (0x0271A2, i_0271a2),
    (0x0271A6, i_0271a6),
    (0x0271AA, i_0271aa),
    (0x0271AE, i_0271ae),
    (0x0271B2, i_0271b2),
    (0x0271B6, i_0271b6),
    (0x0271BC, i_0271bc),
    (0x0271C0, i_0271c0),
    (0x0271C4, i_0271c4),
    (0x0271C8, i_0271c8),
    (0x0271CE, i_0271ce),
    (0x0271D2, i_0271d2),
    (0x0271D8, i_0271d8),
    (0x0271DC, i_0271dc),
    (0x0271E0, i_0271e0),
    (0x0271E2, i_0271e2),
    (0x0271E6, i_0271e6),
    (0x0271EA, i_0271ea),
    (0x0271EE, i_0271ee),
    (0x0271F0, i_0271f0),
    (0x0271F4, i_0271f4),
    (0x0271FA, i_0271fa),
    (0x0271FE, i_0271fe),
    (0x027204, i_027204),
    (0x027208, i_027208),
    (0x02720E, i_02720e),
    (0x027212, i_027212),
    (0x027216, i_027216),
    (0x027218, i_027218),
    (0x02721C, i_02721c),
    (0x027220, i_027220),
    (0x027224, i_027224),
    (0x027226, i_027226),
    (0x02722A, i_02722a),
    (0x02722E, i_02722e),
    (0x027232, i_027232),
    (0x027236, i_027236),
    (0x02723A, i_02723a),
    (0x027240, i_027240),
    (0x027244, i_027244),
    (0x027248, i_027248),
    (0x02724C, i_02724c),
    (0x027250, i_027250),
    (0x027252, i_027252),
    (0x027256, i_027256),
    (0x02725A, i_02725a),
    (0x02725C, i_02725c),
    (0x027260, i_027260),
    (0x027264, i_027264),
    (0x027268, i_027268),
    (0x02726C, i_02726c),
    (0x027270, i_027270),
    (0x027272, i_027272),
    (0x027276, i_027276),
    (0x02727A, i_02727a),
    (0x02727E, i_02727e),
    (0x027280, i_027280),
    (0x027282, i_027282),
    (0x027288, i_027288),
    (0x02728A, i_02728a),
    (0x02728E, i_02728e),
    (0x027292, i_027292),
    (0x027296, i_027296),
    (0x02729A, i_02729a),
    (0x02729E, i_02729e),
    (0x0272A2, i_0272a2),
    (0x0272A6, i_0272a6),
    (0x0272A8, i_0272a8),
    (0x0272AC, i_0272ac),
    (0x0272B0, i_0272b0),
    (0x0272B6, i_0272b6),
    (0x0272BA, i_0272ba),
    (0x0272BE, i_0272be),
    (0x0272C0, i_0272c0),
    (0x0272C4, i_0272c4),
    (0x0272C8, i_0272c8),
    (0x0272CC, i_0272cc),
    (0x0272D2, i_0272d2),
    (0x0272D6, i_0272d6),
    (0x0272D8, i_0272d8),
    (0x0272DA, i_0272da),
    (0x0272DC, i_0272dc),
    (0x0272DE, i_0272de),
    (0x0272E0, i_0272e0),
    (0x0272E6, i_0272e6),
    (0x0272E8, i_0272e8),
    (0x0272EC, i_0272ec),
    (0x0272F4, i_0272f4),
    (0x0272F8, i_0272f8),
    (0x0272FC, i_0272fc),
    (0x027300, i_027300),
    (0x027306, i_027306),
    (0x02730A, i_02730a),
    (0x02730E, i_02730e),
    (0x027312, i_027312),
    (0x027316, i_027316),
    (0x02731A, i_02731a),
    (0x02731E, i_02731e),
    (0x027322, i_027322),
    (0x027326, i_027326),
    (0x02732E, i_02732e),
    (0x027332, i_027332),
    (0x027336, i_027336),
    (0x02733E, i_02733e),
    (0x027342, i_027342),
    (0x027346, i_027346),
    (0x02734A, i_02734a),
    (0x02734E, i_02734e),
    (0x027350, i_027350),
    (0x027354, i_027354),
    (0x02735A, i_02735a),
    (0x02735E, i_02735e),
    (0x027362, i_027362),
    (0x027366, i_027366),
    (0x02736A, i_02736a),
    (0x02736E, i_02736e),
    (0x027372, i_027372),
    (0x027376, i_027376),
    (0x02737A, i_02737a),
    (0x02737E, i_02737e),
    (0x027382, i_027382),
    (0x027386, i_027386),
    (0x02738A, i_02738a),
    (0x02738E, i_02738e),
    (0x027392, i_027392),
    (0x027396, i_027396),
    (0x02739C, i_02739c),
    (0x0273A0, i_0273a0),
    (0x0273A6, i_0273a6),
    (0x0273AA, i_0273aa),
    (0x0273AE, i_0273ae),
    (0x0273B2, i_0273b2),
    (0x0273B6, i_0273b6),
    (0x0273BC, i_0273bc),
    (0x0273C0, i_0273c0),
    (0x0273C4, i_0273c4),
    (0x0273C8, i_0273c8),
    (0x0273CC, i_0273cc),
    (0x0273D0, i_0273d0),
    (0x0273D2, i_0273d2),
    (0x0273D6, i_0273d6),
    (0x0273DA, i_0273da),
    (0x0273DC, i_0273dc),
    (0x0273DE, i_0273de),
    (0x0273E2, i_0273e2),
    (0x0273E6, i_0273e6),
    (0x0273EA, i_0273ea),
    (0x0273EE, i_0273ee),
    (0x0273F2, i_0273f2),
    (0x0273F6, i_0273f6),
    (0x0273FA, i_0273fa),
    (0x0273FC, i_0273fc),
    (0x027400, i_027400),
    (0x027406, i_027406),
    (0x02740A, i_02740a),
    (0x027410, i_027410),
    (0x027414, i_027414),
    (0x027418, i_027418),
    (0x02741C, i_02741c),
    (0x027420, i_027420),
    (0x027424, i_027424),
    (0x027428, i_027428),
    (0x02742A, i_02742a),
    (0x02742E, i_02742e),
    (0x027432, i_027432),
    (0x027436, i_027436),
    (0x02743A, i_02743a),
    (0x02743E, i_02743e),
    (0x027444, i_027444),
    (0x027448, i_027448),
    (0x02744C, i_02744c),
    (0x027452, i_027452),
    (0x027458, i_027458),
    (0x02745C, i_02745c),
    (0x02745E, i_02745e),
    (0x027462, i_027462),
    (0x027464, i_027464),
    (0x02746A, i_02746a),
    (0x027470, i_027470),
    (0x027474, i_027474),
    (0x027478, i_027478),
    (0x02747C, i_02747c),
    (0x027480, i_027480),
    (0x027484, i_027484),
    (0x027488, i_027488),
    (0x02748C, i_02748c),
    (0x027490, i_027490),
    (0x027494, i_027494),
    (0x027498, i_027498),
    (0x02749C, i_02749c),
    (0x0274A0, i_0274a0),
    (0x0274A6, i_0274a6),
    (0x0274AC, i_0274ac),
    (0x0274B0, i_0274b0),
    (0x0274B2, i_0274b2),
    (0x0274B6, i_0274b6),
    (0x0274B8, i_0274b8),
    (0x0274BE, i_0274be),
    (0x0274C4, i_0274c4),
    (0x0274C8, i_0274c8),
    (0x0274CC, i_0274cc),
    (0x0274D2, i_0274d2),
    (0x0274D6, i_0274d6),
    (0x0274DA, i_0274da),
    (0x0274DE, i_0274de),
    (0x0274E2, i_0274e2),
    (0x0274E6, i_0274e6),
    (0x0274EA, i_0274ea),
    (0x0274EE, i_0274ee),
    (0x0274F2, i_0274f2),
    (0x0274F6, i_0274f6),
    (0x0274FA, i_0274fa),
    (0x0274FE, i_0274fe),
    (0x027502, i_027502),
    (0x027506, i_027506),
    (0x02750A, i_02750a),
    (0x02750E, i_02750e),
    (0x027512, i_027512),
    (0x027516, i_027516),
    (0x02751A, i_02751a),
    (0x02751E, i_02751e),
    (0x027522, i_027522),
    (0x027526, i_027526),
    (0x02752C, i_02752c),
    (0x027530, i_027530),
    (0x027534, i_027534),
    (0x027538, i_027538),
    (0x02753C, i_02753c),
    (0x027540, i_027540),
    (0x027544, i_027544),
    (0x027548, i_027548),
    (0x02754C, i_02754c),
    (0x027552, i_027552),
    (0x027558, i_027558),
    (0x02755C, i_02755c),
    (0x027560, i_027560),
    (0x027562, i_027562),
    (0x027564, i_027564),
    (0x027566, i_027566),
    (0x027568, i_027568),
    (0x02756E, i_02756e),
    (0x027572, i_027572),
    (0x027576, i_027576),
    (0x02757A, i_02757a),
    (0x02757E, i_02757e),
    (0x027582, i_027582),
    (0x027586, i_027586),
    (0x02758A, i_02758a),
    (0x02758E, i_02758e),
    (0x027592, i_027592),
    (0x027596, i_027596),
    (0x02759A, i_02759a),
    (0x02759E, i_02759e),
    (0x0275A2, i_0275a2),
    (0x0275A6, i_0275a6),
    (0x0275AC, i_0275ac),
    (0x0275B2, i_0275b2),
    (0x0275B6, i_0275b6),
    (0x0275B8, i_0275b8),
    (0x0275BC, i_0275bc),
    (0x0275C0, i_0275c0),
    (0x0275C2, i_0275c2),
    (0x0275C6, i_0275c6),
    (0x0275CA, i_0275ca),
    (0x0275CE, i_0275ce),
    (0x0275D4, i_0275d4),
    (0x0275D6, i_0275d6),
    (0x0275DA, i_0275da),
    (0x0275DE, i_0275de),
    (0x0275E4, i_0275e4),
    (0x0275EA, i_0275ea),
    (0x0275F0, i_0275f0),
    (0x0275F4, i_0275f4),
    (0x0275F6, i_0275f6),
    (0x0275FA, i_0275fa),
    (0x0275FE, i_0275fe),
    (0x027602, i_027602),
    (0x027606, i_027606),
    (0x02760A, i_02760a),
    (0x02760E, i_02760e),
    (0x027612, i_027612),
    (0x027616, i_027616),
    (0x02761A, i_02761a),
    (0x02761E, i_02761e),
    (0x027622, i_027622),
    (0x027626, i_027626),
    (0x02762A, i_02762a),
    (0x02762E, i_02762e),
    (0x027632, i_027632),
    (0x027636, i_027636),
    (0x02763A, i_02763a),
    (0x02763E, i_02763e),
    (0x027642, i_027642),
    (0x027646, i_027646),
    (0x02764A, i_02764a),
    (0x02764E, i_02764e),
    (0x027652, i_027652),
    (0x027656, i_027656),
    (0x02765A, i_02765a),
    (0x02765E, i_02765e),
    (0x027662, i_027662),
    (0x027666, i_027666),
    (0x02766A, i_02766a),
    (0x02766E, i_02766e),
    (0x027672, i_027672),
    (0x027676, i_027676),
    (0x02767A, i_02767a),
    (0x02767E, i_02767e),
    (0x027682, i_027682),
    (0x027686, i_027686),
    (0x02768A, i_02768a),
    (0x02768E, i_02768e),
    (0x027692, i_027692),
    (0x027696, i_027696),
    (0x02769A, i_02769a),
    (0x02769E, i_02769e),
    (0x0276A2, i_0276a2),
    (0x0276A6, i_0276a6),
    (0x0276A8, i_0276a8),
    (0x0276AA, i_0276aa),
    (0x0276AE, i_0276ae),
    (0x0276B4, i_0276b4),
    (0x0276BA, i_0276ba),
    (0x0276BE, i_0276be),
    (0x0276C2, i_0276c2),
    (0x0276C4, i_0276c4),
    (0x0276C6, i_0276c6),
    (0x0276C8, i_0276c8),
    (0x0276CA, i_0276ca),
    (0x0276D0, i_0276d0),
    (0x0276D4, i_0276d4),
    (0x0276D8, i_0276d8),
    (0x0276DC, i_0276dc),
    (0x0276E2, i_0276e2),
    (0x0276E8, i_0276e8),
    (0x0276EC, i_0276ec),
    (0x0276EE, i_0276ee),
    (0x0276F0, i_0276f0),
    (0x0276F6, i_0276f6),
    (0x0276F8, i_0276f8),
    (0x0276FA, i_0276fa),
    (0x027700, i_027700),
    (0x027704, i_027704),
    (0x027706, i_027706),
    (0x027708, i_027708),
    (0x02770E, i_02770e),
    (0x027710, i_027710),
    (0x027712, i_027712),
    (0x027718, i_027718),
    (0x02771C, i_02771c),
    (0x027722, i_027722),
    (0x027726, i_027726),
    (0x02772A, i_02772a),
    (0x02772C, i_02772c),
    (0x02772E, i_02772e),
    (0x027730, i_027730),
    (0x027734, i_027734),
    (0x027736, i_027736),
    (0x02773A, i_02773a),
    (0x02773C, i_02773c),
    (0x027742, i_027742),
    (0x027746, i_027746),
    (0x02774A, i_02774a),
    (0x02774E, i_02774e),
    (0x027752, i_027752),
    (0x027756, i_027756),
    (0x02775A, i_02775a),
    (0x02775C, i_02775c),
    (0x027760, i_027760),
    (0x027766, i_027766),
    (0x02776C, i_02776c),
    (0x027770, i_027770),
    (0x027774, i_027774),
    (0x027776, i_027776),
    (0x027778, i_027778),
    (0x02777A, i_02777a),
    (0x02777C, i_02777c),
    (0x027782, i_027782),
    (0x027786, i_027786),
    (0x02778A, i_02778a),
    (0x02778E, i_02778e),
    (0x027792, i_027792),
    (0x027796, i_027796),
    (0x02779C, i_02779c),
    (0x0277A0, i_0277a0),
    (0x0277A6, i_0277a6),
    (0x0277AA, i_0277aa),
    (0x0277AE, i_0277ae),
    (0x0277B2, i_0277b2),
    (0x0277B8, i_0277b8),
    (0x0277BC, i_0277bc),
    (0x0277C0, i_0277c0),
    (0x0277C4, i_0277c4),
    (0x0277CA, i_0277ca),
    (0x0277CE, i_0277ce),
    (0x0277D2, i_0277d2),
    (0x0277D6, i_0277d6),
    (0x0277DA, i_0277da),
    (0x0277DE, i_0277de),
    (0x0277E2, i_0277e2),
    (0x0277E4, i_0277e4),
    (0x0277E8, i_0277e8),
    (0x0277EE, i_0277ee),
    (0x0277F4, i_0277f4),
    (0x0277F8, i_0277f8),
    (0x0277FC, i_0277fc),
    (0x0277FE, i_0277fe),
    (0x027800, i_027800),
    (0x027802, i_027802),
    (0x027804, i_027804),
    (0x02780A, i_02780a),
    (0x02780E, i_02780e),
    (0x027812, i_027812),
    (0x027816, i_027816),
    (0x02781A, i_02781a),
    (0x02781E, i_02781e),
    (0x027824, i_027824),
    (0x027828, i_027828),
    (0x02782C, i_02782c),
    (0x027830, i_027830),
    (0x027834, i_027834),
    (0x027836, i_027836),
    (0x02783A, i_02783a),
    (0x027840, i_027840),
    (0x027846, i_027846),
    (0x02784A, i_02784a),
    (0x02784E, i_02784e),
    (0x027852, i_027852),
    (0x027856, i_027856),
    (0x02785A, i_02785a),
    (0x02785C, i_02785c),
    (0x027862, i_027862),
    (0x027868, i_027868),
    (0x02786E, i_02786e),
    (0x027874, i_027874),
    (0x02787A, i_02787a),
    (0x027880, i_027880),
    (0x027886, i_027886),
    (0x02788A, i_02788a),
    (0x02788C, i_02788c),
    (0x027892, i_027892),
    (0x027898, i_027898),
    (0x02789E, i_02789e),
    (0x0278A4, i_0278a4),
    (0x0278AA, i_0278aa),
    (0x0278AE, i_0278ae),
    (0x0278B2, i_0278b2),
    (0x0278B8, i_0278b8),
    (0x0278BE, i_0278be),
    (0x0278C4, i_0278c4),
    (0x0278CA, i_0278ca),
    (0x0278D0, i_0278d0),
    (0x0278D4, i_0278d4),
    (0x0278D8, i_0278d8),
    (0x0278DA, i_0278da),
    (0x0278E0, i_0278e0),
    (0x0278E6, i_0278e6),
    (0x0278EC, i_0278ec),
    (0x0278F2, i_0278f2),
    (0x0278F8, i_0278f8),
    (0x0278FC, i_0278fc),
    (0x027900, i_027900),
    (0x027902, i_027902),
    (0x027908, i_027908),
    (0x02790E, i_02790e),
    (0x027914, i_027914),
    (0x02791A, i_02791a),
    (0x027920, i_027920),
    (0x027924, i_027924),
    (0x027928, i_027928),
    (0x02792E, i_02792e),
    (0x027934, i_027934),
    (0x02793A, i_02793a),
    (0x027940, i_027940),
    (0x027944, i_027944),
    (0x027948, i_027948),
    (0x02794A, i_02794a),
    (0x027950, i_027950),
    (0x027956, i_027956),
    (0x02795C, i_02795c),
    (0x027962, i_027962),
    (0x027966, i_027966),
    (0x02796A, i_02796a),
    (0x02796C, i_02796c),
    (0x027972, i_027972),
    (0x027978, i_027978),
    (0x02797E, i_02797e),
    (0x027984, i_027984),
    (0x02798A, i_02798a),
    (0x02798E, i_02798e),
    (0x027992, i_027992),
    (0x027998, i_027998),
    (0x02799E, i_02799e),
    (0x0279A4, i_0279a4),
    (0x0279AA, i_0279aa),
    (0x0279B0, i_0279b0),
    (0x0279B4, i_0279b4),
    (0x0279B8, i_0279b8),
    (0x0279BA, i_0279ba),
    (0x0279C0, i_0279c0),
    (0x0279C6, i_0279c6),
    (0x0279CC, i_0279cc),
    (0x0279D2, i_0279d2),
    (0x0279D8, i_0279d8),
    (0x0279DC, i_0279dc),
    (0x0279E0, i_0279e0),
    (0x0279E2, i_0279e2),
    (0x0279E8, i_0279e8),
    (0x0279EE, i_0279ee),
    (0x0279F4, i_0279f4),
    (0x0279FA, i_0279fa),
    (0x027A00, i_027a00),
    (0x027A06, i_027a06),
    (0x027A0C, i_027a0c),
    (0x027A10, i_027a10),
    (0x027A14, i_027a14),
    (0x027A18, i_027a18),
    (0x027A1E, i_027a1e),
    (0x027A24, i_027a24),
    (0x027A2A, i_027a2a),
    (0x027A30, i_027a30),
    (0x027A36, i_027a36),
    (0x027A3A, i_027a3a),
    (0x027A3E, i_027a3e),
    (0x027A44, i_027a44),
    (0x027A4A, i_027a4a),
    (0x027A50, i_027a50),
    (0x027A56, i_027a56),
    (0x027A5C, i_027a5c),
    (0x027A62, i_027a62),
    (0x027A68, i_027a68),
    (0x027A6C, i_027a6c),
    (0x027A6E, i_027a6e),
    (0x027A74, i_027a74),
    (0x027A7A, i_027a7a),
    (0x027A80, i_027a80),
    (0x027A86, i_027a86),
    (0x027A8C, i_027a8c),
    (0x027A90, i_027a90),
    (0x027A92, i_027a92),
    (0x027A98, i_027a98),
    (0x027A9E, i_027a9e),
    (0x027AA4, i_027aa4),
    (0x027AAA, i_027aaa),
    (0x027AB0, i_027ab0),
    (0x027AB4, i_027ab4),
    (0x027AB8, i_027ab8),
    (0x027ABE, i_027abe),
    (0x027AC4, i_027ac4),
    (0x027ACA, i_027aca),
    (0x027AD0, i_027ad0),
    (0x027AD4, i_027ad4),
    (0x027AD8, i_027ad8),
    (0x027ADA, i_027ada),
    (0x027AE0, i_027ae0),
    (0x027AE6, i_027ae6),
    (0x027AEC, i_027aec),
    (0x027AF2, i_027af2),
    (0x027AF6, i_027af6),
    (0x027AFA, i_027afa),
    (0x027AFC, i_027afc),
    (0x027B02, i_027b02),
    (0x027B08, i_027b08),
    (0x027B0E, i_027b0e),
    (0x027B14, i_027b14),
    (0x027B1A, i_027b1a),
    (0x027B1E, i_027b1e),
    (0x027B22, i_027b22),
    (0x027B28, i_027b28),
    (0x027B2E, i_027b2e),
    (0x027B34, i_027b34),
    (0x027B3A, i_027b3a),
    (0x027B3E, i_027b3e),
    (0x027B42, i_027b42),
    (0x027B44, i_027b44),
    (0x027B4A, i_027b4a),
    (0x027B50, i_027b50),
    (0x027B56, i_027b56),
    (0x027B5C, i_027b5c),
    (0x027B60, i_027b60),
    (0x027B64, i_027b64),
    (0x027B66, i_027b66),
    (0x027B6C, i_027b6c),
    (0x027B72, i_027b72),
    (0x027B78, i_027b78),
    (0x027B7E, i_027b7e),
    (0x027B84, i_027b84),
    (0x027B88, i_027b88),
    (0x027B8C, i_027b8c),
    (0x027B92, i_027b92),
    (0x027B98, i_027b98),
    (0x027B9E, i_027b9e),
    (0x027BA4, i_027ba4),
    (0x027BA8, i_027ba8),
    (0x027BAA, i_027baa),
    (0x027BB0, i_027bb0),
    (0x027BB6, i_027bb6),
    (0x027BBC, i_027bbc),
    (0x027BC2, i_027bc2),
    (0x027BC6, i_027bc6),
    (0x027BC8, i_027bc8),
    (0x027BCE, i_027bce),
    (0x027BD4, i_027bd4),
    (0x027BDA, i_027bda),
    (0x027BE0, i_027be0),
    (0x027BE6, i_027be6),
    (0x027BEA, i_027bea),
    (0x027BEE, i_027bee),
    (0x027BF4, i_027bf4),
    (0x027BFA, i_027bfa),
    (0x027C00, i_027c00),
    (0x027C06, i_027c06),
    (0x027C0A, i_027c0a),
    (0x027C0C, i_027c0c),
    (0x027C12, i_027c12),
    (0x027C18, i_027c18),
    (0x027C1E, i_027c1e),
    (0x027C24, i_027c24),
    (0x027C28, i_027c28),
    (0x027C2A, i_027c2a),
    (0x027C30, i_027c30),
    (0x027C36, i_027c36),
    (0x027C3C, i_027c3c),
    (0x027C42, i_027c42),
    (0x027C48, i_027c48),
    (0x027C4C, i_027c4c),
    (0x027C50, i_027c50),
    (0x027C56, i_027c56),
    (0x027C5C, i_027c5c),
    (0x027C62, i_027c62),
    (0x027C68, i_027c68),
    (0x027C6C, i_027c6c),
    (0x027C6E, i_027c6e),
    (0x027C74, i_027c74),
    (0x027C7A, i_027c7a),
    (0x027C80, i_027c80),
    (0x027C86, i_027c86),
    (0x027C8A, i_027c8a),
    (0x027C8C, i_027c8c),
    (0x027C92, i_027c92),
    (0x027C98, i_027c98),
    (0x027C9E, i_027c9e),
    (0x027CA4, i_027ca4),
    (0x027CAA, i_027caa),
    (0x027CAE, i_027cae),
    (0x027CB2, i_027cb2),
    (0x027CB8, i_027cb8),
    (0x027CBE, i_027cbe),
    (0x027CC4, i_027cc4),
    (0x027CCA, i_027cca),
    (0x027CCE, i_027cce),
    (0x027CD0, i_027cd0),
    (0x027CD6, i_027cd6),
    (0x027CDC, i_027cdc),
    (0x027CE2, i_027ce2),
    (0x027CE8, i_027ce8),
    (0x027CEC, i_027cec),
    (0x027CEE, i_027cee),
    (0x027CF4, i_027cf4),
    (0x027CFA, i_027cfa),
    (0x027D00, i_027d00),
    (0x027D06, i_027d06),
    (0x027D0C, i_027d0c),
    (0x027D10, i_027d10),
    (0x027D14, i_027d14),
    (0x027D1A, i_027d1a),
    (0x027D20, i_027d20),
    (0x027D26, i_027d26),
    (0x027D2C, i_027d2c),
    (0x027D30, i_027d30),
    (0x027D32, i_027d32),
    (0x027D38, i_027d38),
    (0x027D3E, i_027d3e),
    (0x027D44, i_027d44),
    (0x027D4A, i_027d4a),
    (0x027D4E, i_027d4e),
    (0x027D50, i_027d50),
    (0x027D56, i_027d56),
    (0x027D5C, i_027d5c),
    (0x027D62, i_027d62),
    (0x027D68, i_027d68),
    (0x027D6E, i_027d6e),
    (0x027D72, i_027d72),
    (0x027D76, i_027d76),
    (0x027D7C, i_027d7c),
    (0x027D82, i_027d82),
    (0x027D88, i_027d88),
    (0x027D8E, i_027d8e),
    (0x027D92, i_027d92),
    (0x027D94, i_027d94),
    (0x027D9A, i_027d9a),
    (0x027DA0, i_027da0),
    (0x027DA6, i_027da6),
    (0x027DAC, i_027dac),
    (0x027DB0, i_027db0),
    (0x027DB2, i_027db2),
    (0x027DB4, i_027db4),
    (0x027DB6, i_027db6),
    (0x027DB8, i_027db8),
    (0x027DBA, i_027dba),
    (0x027DBE, i_027dbe),
    (0x027DC2, i_027dc2),
    (0x027DC8, i_027dc8),
    (0x027DCC, i_027dcc),
    (0x027DCE, i_027dce),
    (0x027DD2, i_027dd2),
    (0x027DD8, i_027dd8),
    (0x027DDC, i_027ddc),
    (0x027DDE, i_027dde),
    (0x027DE2, i_027de2),
    (0x027DE6, i_027de6),
    (0x027DEA, i_027dea),
    (0x027DEE, i_027dee),
    (0x027DF0, i_027df0),
    (0x027DF4, i_027df4),
    (0x027DFA, i_027dfa),
    (0x027DFC, i_027dfc),
    (0x027E02, i_027e02),
    (0x027E04, i_027e04),
    (0x027E06, i_027e06),
    (0x027E0A, i_027e0a),
    (0x027E0E, i_027e0e),
    (0x027E10, i_027e10),
    (0x027E12, i_027e12),
    (0x027E16, i_027e16),
    (0x027E1A, i_027e1a),
    (0x027E1E, i_027e1e),
    (0x027E20, i_027e20),
    (0x027E22, i_027e22),
    (0x027E24, i_027e24),
    (0x027E26, i_027e26),
    (0x027E28, i_027e28),
    (0x027E2A, i_027e2a),
    (0x027E2E, i_027e2e),
    (0x027E30, i_027e30),
    (0x027E32, i_027e32),
    (0x027E34, i_027e34),
    (0x027E38, i_027e38),
    (0x027E3C, i_027e3c),
    (0x027E40, i_027e40),
    (0x027E44, i_027e44),
    (0x027E48, i_027e48),
    (0x027E4A, i_027e4a),
    (0x027E4C, i_027e4c),
    (0x027E50, i_027e50),
    (0x027E56, i_027e56),
    (0x027E5A, i_027e5a),
    (0x027E5C, i_027e5c),
    (0x027E60, i_027e60),
    (0x027E62, i_027e62),
    (0x027E64, i_027e64),
    (0x027E68, i_027e68),
    (0x027E6A, i_027e6a),
    (0x027E6C, i_027e6c),
    (0x027E6E, i_027e6e),
    (0x027E72, i_027e72),
    (0x027E76, i_027e76),
    (0x027E78, i_027e78),
    (0x027E7C, i_027e7c),
    (0x027E7E, i_027e7e),
    (0x027E80, i_027e80),
    (0x027E84, i_027e84),
    (0x027E88, i_027e88),
    (0x027E8A, i_027e8a),
    (0x027E8C, i_027e8c),
    (0x027E90, i_027e90),
    (0x027E94, i_027e94),
    (0x027E96, i_027e96),
    (0x027E9A, i_027e9a),
    (0x027E9C, i_027e9c),
    (0x027E9E, i_027e9e),
    (0x027EA2, i_027ea2),
    (0x027EA6, i_027ea6),
    (0x027EA8, i_027ea8),
    (0x027EAA, i_027eaa),
    (0x027EAE, i_027eae),
    (0x027EB2, i_027eb2),
    (0x027EB4, i_027eb4),
    (0x027EB8, i_027eb8),
    (0x027EBA, i_027eba),
    (0x027EBE, i_027ebe),
    (0x027EC2, i_027ec2),
    (0x027EC4, i_027ec4),
    (0x027EC8, i_027ec8),
    (0x027ECE, i_027ece),
    (0x027ED4, i_027ed4),
    (0x027ED8, i_027ed8),
    (0x027EDC, i_027edc),
    (0x027EDE, i_027ede),
    (0x027EE0, i_027ee0),
    (0x027EE2, i_027ee2),
    (0x027EE4, i_027ee4),
    (0x027EEA, i_027eea),
    (0x027EEE, i_027eee),
    (0x027EF2, i_027ef2),
    (0x027EF6, i_027ef6),
    (0x027EFC, i_027efc),
    (0x027F00, i_027f00),
    (0x027F02, i_027f02),
    (0x027F06, i_027f06),
    (0x027F08, i_027f08),
    (0x027F0C, i_027f0c),
    (0x027F10, i_027f10),
    (0x027F14, i_027f14),
    (0x027F16, i_027f16),
    (0x027F1A, i_027f1a),
    (0x027F20, i_027f20),
    (0x027F26, i_027f26),
    (0x027F2A, i_027f2a),
    (0x027F2E, i_027f2e),
    (0x027F30, i_027f30),
    (0x027F32, i_027f32),
    (0x027F34, i_027f34),
    (0x027F36, i_027f36),
    (0x027F3C, i_027f3c),
    (0x027F40, i_027f40),
    (0x027F44, i_027f44),
    (0x027F48, i_027f48),
    (0x027F4E, i_027f4e),
    (0x027F52, i_027f52),
    (0x027F56, i_027f56),
    (0x027F58, i_027f58),
    (0x027F5A, i_027f5a),
    (0x027F5E, i_027f5e),
    (0x027F60, i_027f60),
    (0x027F64, i_027f64),
    (0x027F68, i_027f68),
    (0x027F6C, i_027f6c),
    (0x027F70, i_027f70),
    (0x027F76, i_027f76),
    (0x027F7A, i_027f7a),
    (0x027F80, i_027f80),
    (0x027F84, i_027f84),
    (0x027F88, i_027f88),
    (0x027F8A, i_027f8a),
    (0x027F8E, i_027f8e),
    (0x027F92, i_027f92),
    (0x027F96, i_027f96),
    (0x027F9A, i_027f9a),
    (0x027F9E, i_027f9e),
    (0x027FA2, i_027fa2),
    (0x027FA6, i_027fa6),
    (0x027FAA, i_027faa),
    (0x027FAC, i_027fac),
    (0x027FB0, i_027fb0),
    (0x027FB4, i_027fb4),
    (0x027FB6, i_027fb6),
    (0x027FBA, i_027fba),
    (0x027FBE, i_027fbe),
    (0x027FC2, i_027fc2),
    (0x027FC8, i_027fc8),
    (0x027FCC, i_027fcc),
    (0x027FD0, i_027fd0),
    (0x027FD2, i_027fd2),
    (0x027FD6, i_027fd6),
    (0x027FD8, i_027fd8),
    (0x027FDC, i_027fdc),
    (0x027FE0, i_027fe0),
    (0x027FE4, i_027fe4),
    (0x027FE6, i_027fe6),
    (0x027FEA, i_027fea),
    (0x027FEE, i_027fee),
    (0x027FF2, i_027ff2),
    (0x027FF8, i_027ff8),
    (0x027FFC, i_027ffc),
    (0x028000, i_028000),
    (0x028004, i_028004),
    (0x028006, i_028006),
    (0x028008, i_028008),
    (0x02800C, i_02800c),
    (0x02800E, i_02800e),
    (0x028014, i_028014),
    (0x028018, i_028018),
    (0x02801E, i_02801e),
    (0x028022, i_028022),
    (0x028024, i_028024),
    (0x02802C, i_02802c),
    (0x028030, i_028030),
    (0x028036, i_028036),
    (0x028038, i_028038),
    (0x02803A, i_02803a),
    (0x02803E, i_02803e),
    (0x028040, i_028040),
    (0x028042, i_028042),
    (0x028044, i_028044),
    (0x028046, i_028046),
    (0x02804A, i_02804a),
    (0x02804C, i_02804c),
    (0x028050, i_028050),
    (0x028054, i_028054),
    (0x028056, i_028056),
    (0x02805A, i_02805a),
    (0x028068, i_028068),
    (0x02806C, i_02806c),
    (0x02806E, i_02806e),
    (0x028072, i_028072),
    (0x028074, i_028074),
    (0x02807C, i_02807c),
    (0x028080, i_028080),
    (0x028084, i_028084),
    (0x028088, i_028088),
    (0x02808C, i_02808c),
    (0x02808E, i_02808e),
    (0x028092, i_028092),
    (0x028096, i_028096),
    (0x02809A, i_02809a),
    (0x02809E, i_02809e),
    (0x0280A2, i_0280a2),
    (0x0280A4, i_0280a4),
    (0x0280A8, i_0280a8),
    (0x0280AC, i_0280ac),
    (0x0280B2, i_0280b2),
    (0x0280B6, i_0280b6),
    (0x0280B8, i_0280b8),
    (0x0280BA, i_0280ba),
];
