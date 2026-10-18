// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C173C0: move.w d2, (a4)
pub fn i_c173c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C2;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C173C2: bra.b $c173ba
pub fn i_c173c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C4;
    cpu.ir = 0x60F6;
    cpu.pc = 0xC173BA;
    10
}
/// $C173C4: move.b (a2)+, d2
pub fn i_c173c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C6;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C173C6: move.w d2, (a4)
pub fn i_c173c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C8;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C173C8: subq.b #$1, d7
pub fn i_c173c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173CA;
    cpu.ir = 0x5307;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C173CA: bne.b $c173c4
pub fn i_c173ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173CC;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173C4; return 10; }
    8
}
/// $C173CC: rts
pub fn i_c173cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C173CE: move.w d5, d7
pub fn i_c173ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173D0;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C173D0: btst.b #$10, d5
pub fn i_c173d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173D4;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((16) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C173D4: bne.b $c173e0
pub fn i_c173d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173D6;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173E0; return 10; }
    8
}
/// $C173D6: move.w (a2)+, d2
pub fn i_c173d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173D8;
    cpu.ir = 0x341A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C173D8: cmp.w d2, d5
pub fn i_c173d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173DA;
    cpu.ir = 0xBA42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C173DA: beq.b $c173e6
pub fn i_c173da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173DC;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC173E6; return 10; }
    8
}
/// $C173DC: move.w d2, (a4)
pub fn i_c173dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173DE;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C173DE: bra.b $c173d6
pub fn i_c173de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E0;
    cpu.ir = 0x60F6;
    cpu.pc = 0xC173D6;
    10
}
/// $C173E0: move.w (a2)+, (a4)
pub fn i_c173e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E2;
    cpu.ir = 0x389A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v3 as u16);
    12
}
/// $C173E2: subq.w #$1, d7
pub fn i_c173e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E4;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C173E4: bne.b $c173e0
pub fn i_c173e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E6;
    cpu.ir = 0x66FA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173E0; return 10; }
    8
}
/// $C173E6: rts
pub fn i_c173e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C173E8: move.w #$20, $2(a4)
pub fn i_c173e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173EE;
    cpu.ir = 0x397C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C173EE: move.b d1, d2
pub fn i_c173ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173F0;
    cpu.ir = 0x1401;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C173F0: lsl.w #$8, d2
pub fn i_c173f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173F2;
    cpu.ir = 0xE14A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C173F2: movea.l a1, a2
pub fn i_c173f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173F4;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C173F4: jsr $c17412(pc)
pub fn i_c173f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC173F8);
    cpu.pc = 0xC17412;
    16
}
/// $C173F8: addq.w #$1, d6
pub fn i_c173f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173FA;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C173FA: move.w d6, (a6)
pub fn i_c173fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173FC;
    cpu.ir = 0x3C86;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C173FC: movea.l a1, a2
pub fn i_c173fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173FE;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C173FE: addi.w #$100, d2
pub fn i_c173fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17402;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C17402: jsr $c17412(pc)
pub fn i_c17402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17406;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17406);
    cpu.pc = 0xC17412;
    16
}
/// $C17406: move.l a2, d0
pub fn i_c17406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17408;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17408: addq.l #$1, d0
pub fn i_c17408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1740A;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C1740A: andi.w #$fffe, d0
pub fn i_c1740a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1740E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1740E: movea.l d0, a1
pub fn i_c1740e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17410;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C17410: rts
pub fn i_c17410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17412;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17412: move.b (a2)+, d2
pub fn i_c17412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17414;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C17414: cmpi.b #$ff, d2
pub fn i_c17414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17418;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C17418: beq.b $c1744c
pub fn i_c17418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1741A;
    cpu.ir = 0x6732;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1744C; return 10; }
    8
}
/// $C1741A: cmpi.b #$20, d2
pub fn i_c1741a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1741E;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $C1741E: bcs.b $c17430
pub fn i_c1741e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17420;
    cpu.ir = 0x6510;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC17430; return 10; }
    8
}
/// $C17420: cmpi.b #$60, d2
pub fn i_c17420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17424;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x60, Size::Byte, false);
    8
}
/// $C17424: bcs.b $c17448
pub fn i_c17424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17426;
    cpu.ir = 0x6522;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC17448; return 10; }
    8
}
/// $C17426: tst.b d2
pub fn i_c17426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17428;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17428: bmi.w $c17448
pub fn i_c17428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1742C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC17448; return 10; }
    8
}
/// $C1742C: subi.b #$40, d2
pub fn i_c1742c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17430;
    cpu.ir = 0x402;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17430: addi.b #$80, d2
pub fn i_c17430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17434;
    cpu.ir = 0x602;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17434: move.w d2, d1
pub fn i_c17434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17436;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17436: andi.w #$f0ff, d1
pub fn i_c17436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1743A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF0FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1743A: btst.b #$8, d2
pub fn i_c1743a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1743E;
    cpu.ir = 0x802;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1743E: bne.b $c17444
pub fn i_c1743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17440;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17444; return 10; }
    8
}
/// $C17440: addi.b #$40, d1
pub fn i_c17440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17444;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17444: move.w d1, (a4)
pub fn i_c17444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17446;
    cpu.ir = 0x3881;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C17446: bra.b $c17412
pub fn i_c17446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17448;
    cpu.ir = 0x60CA;
    cpu.pc = 0xC17412;
    10
}
/// $C17448: move.w d2, (a4)
pub fn i_c17448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1744A;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C1744A: bra.b $c17412
pub fn i_c1744a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1744C;
    cpu.ir = 0x60C6;
    cpu.pc = 0xC17412;
    10
}
/// $C1744C: rts
pub fn i_c1744c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1744E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1744E: move.w #$20, $2(a4)
pub fn i_c1744e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17454;
    cpu.ir = 0x397C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17454: move.b d1, d2
pub fn i_c17454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17456;
    cpu.ir = 0x1401;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17456: lsl.w #$8, d2
pub fn i_c17456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17458;
    cpu.ir = 0xE14A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C17458: movea.l a1, a2
pub fn i_c17458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1745A;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C1745A: jsr $c17478(pc)
pub fn i_c1745a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1745E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1745E);
    cpu.pc = 0xC17478;
    16
}
/// $C1745E: addq.w #$1, d6
pub fn i_c1745e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17460;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C17460: move.w d6, (a6)
pub fn i_c17460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17462;
    cpu.ir = 0x3C86;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C17462: movea.l a1, a2
pub fn i_c17462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17464;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C17464: addi.w #$100, d2
pub fn i_c17464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17468;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C17468: jsr $c17478(pc)
pub fn i_c17468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1746C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1746C);
    cpu.pc = 0xC17478;
    16
}
/// $C1746C: move.l a2, d0
pub fn i_c1746c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1746E;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1746E: addq.l #$1, d0
pub fn i_c1746e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17470;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C17470: andi.w #$fffe, d0
pub fn i_c17470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17474;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C17474: movea.l d0, a1
pub fn i_c17474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17476;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C17476: rts
pub fn i_c17476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17478;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17478: move.b (a2)+, d2
pub fn i_c17478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1747A;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1747A: cmpi.b #$ff, d2
pub fn i_c1747a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1747E;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C1747E: beq.b $c17484
pub fn i_c1747e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17480;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17484; return 10; }
    8
}
/// $C17480: move.w d2, (a4)
pub fn i_c17480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17482;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C17482: bra.b $c17478
pub fn i_c17482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17484;
    cpu.ir = 0x60F4;
    cpu.pc = 0xC17478;
    10
}
/// $C17484: rts
pub fn i_c17484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17486;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17486: tst.b $10fd82.l
pub fn i_c17486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1748C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C1748C: beq.w $c1764a
pub fn i_c1748c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17490;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1764A; return 10; }
    8
}
/// $C17490: lea.l $10f300.l, a5
pub fn i_c17490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17496;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C17496: move.b #$1, $ac2(a5)
pub fn i_c17496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1749C;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1749C: bclr.b #$7, $a80(a5)
pub fn i_c1749c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174A2;
    cpu.ir = 0x8AD;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[5].wrapping_add(0xA80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C174A2: move.w #$7, $3c000c.l
pub fn i_c174a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174AA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C174AA: move.w #$2000, sr
pub fn i_c174aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174AE;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C174AE: jsr $c10fae(pc)
pub fn i_c174ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC174B2);
    cpu.pc = 0xC10FAE;
    16
}
/// $C174B2: jsr $c1727e(pc)
pub fn i_c174b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC174B6);
    cpu.pc = 0xC1727E;
    16
}
/// $C174B6: move.w #$2700, sr
pub fn i_c174b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174BA;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C174BA: move.b #$3, $320000.l
pub fn i_c174ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174C2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C174C2: move.b #$ff, $b8d(a5)
pub fn i_c174c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174C8;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C174C8: move.b d0, $3a0003.l
pub fn i_c174c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174CE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C174CE: move.b d0, $3a000b.l
pub fn i_c174ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174D4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C174D4: move.w #$2000, sr
pub fn i_c174d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174D8;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C174D8: lea.l $400000.l, a0
pub fn i_c174d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x400000;
    20
}
/// $C174DE: lea.l $c1fbca.l, a1
pub fn i_c174de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xC1FBCA;
    20
}
/// $C174E4: move.w #$6f, d7
pub fn i_c174e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174E8;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x6F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C174E8: move.l (a1)+, (a0)+
pub fn i_c174e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174EA;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C174EA: dbra d7, $c174e8
pub fn i_c174ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174EE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC174EE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC174EE; return 14; }
    cpu.pc = 0xC174E8;
    10
}
/// $C174EE: move.l #$eee0000, $2(a0)
pub fn i_c174ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174F6;
    cpu.ir = 0x217C;
    let v1: u32 = 0xEEE0000;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C174F6: clr.w $401ffe.l
pub fn i_c174f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC174FC;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x401FFE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x401FFE, r2 as u16);
    16
}
/// $C174FC: jsr $c16f0c(pc)
pub fn i_c174fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17500;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17500);
    cpu.pc = 0xC16F0C;
    16
}
/// $C17500: move.b #$1, $ac2(a5)
pub fn i_c17500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17506;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17506: lea.l $c00(a5), a0
pub fn i_c17506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1750A;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C1750A: move.l #$c1766c, (a0)+
pub fn i_c1750a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17510;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1766C;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17510: move.l #$c17978, (a0)+
pub fn i_c17510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17516;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17978;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17516: move.l #$c179c2, (a0)+
pub fn i_c17516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1751C;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC179C2;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1751C: move.l #$c179e6, (a0)+
pub fn i_c1751c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17522;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC179E6;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17522: move.l #$c17a08, (a0)+
pub fn i_c17522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17528;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17A08;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17528: move.l #$c17a2e, (a0)+
pub fn i_c17528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1752E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17A2E;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1752E: move.l a0, $abe(a5)
pub fn i_c1752e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17532;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17532: clr.b $ac2(a5)
pub fn i_c17532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17536;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17536: move.l a5, $9ee(a5)
pub fn i_c17536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1753A;
    cpu.ir = 0x2B4D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0x9EE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1753A: bra.w $c17558
pub fn i_c1753a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1753E;
    cpu.ir = 0x6000;
    cpu.pc = 0xC17558;
    10
}
/// $C1753E: jsr $c1727e(pc)
pub fn i_c1753e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17542;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17542);
    cpu.pc = 0xC1727E;
    16
}
/// $C17542: move.b $beb(a5), d0
pub fn i_c17542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17546;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBEB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17546: or.b $a9d(a5), d0
pub fn i_c17546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1754A;
    cpu.ir = 0x802D;
    let ad2 = cpu.a[5].wrapping_add(0xA9D);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1754A: andi.b #$f0, d0
pub fn i_c1754a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1754E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1754E: bne.w $c17616
pub fn i_c1754e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17552;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17616; return 10; }
    8
}
/// $C17552: subq.w #$1, $9f2(a5)
pub fn i_c17552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17556;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0x9F2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C17556: bne.b $c1753e
pub fn i_c17556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17558;
    cpu.ir = 0x66E6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1753E; return 10; }
    8
}
/// $C17558: movea.l $9ee(a5), a1
pub fn i_c17558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1755C;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0x9EE);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C1755C: move.l (a1)+, d0
pub fn i_c1755c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1755E;
    cpu.ir = 0x2019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1755E: beq.w $c17616
pub fn i_c1755e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17562;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17616; return 10; }
    8
}
/// $C17562: cmpi.l #$100, d0
pub fn i_c17562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17568;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Long, false);
    8
}
/// $C17568: bcs.w $c175f4
pub fn i_c17568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1756C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC175F4; return 10; }
    8
}
/// $C1756C: cmpi.l #$200, d0
pub fn i_c1756c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17572;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x200, Size::Long, false);
    8
}
/// $C17572: bcs.w $c1759c
pub fn i_c17572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17576;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1759C; return 10; }
    8
}
/// $C17576: cmpi.l #$300, d0
pub fn i_c17576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1757C;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Long, false);
    8
}
/// $C1757C: bcs.w $c175ca
pub fn i_c1757c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17580;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC175CA; return 10; }
    8
}
/// $C17580: move.w (a1)+, $9f2(a5)
pub fn i_c17580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17584;
    cpu.ir = 0x3B59;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9F2);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C17584: move.l a1, $9ee(a5)
pub fn i_c17584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17588;
    cpu.ir = 0x2B49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0x9EE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17588: addq.b #$1, $ac2(a5)
pub fn i_c17588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1758C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1758C: movea.l $abe(a5), a0
pub fn i_c1758c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17590;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17590: move.l d0, (a0)+
pub fn i_c17590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17592;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17592: move.l a0, $abe(a5)
pub fn i_c17592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17596;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17596: subq.b #$1, $ac2(a5)
pub fn i_c17596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1759A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1759A: bra.b $c1753e
pub fn i_c1759a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1759C;
    cpu.ir = 0x60A2;
    cpu.pc = 0xC1753E;
    10
}
/// $C1759C: lea.l $40000a.l, a6
pub fn i_c1759c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175A2;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x40000A;
    20
}
/// $C175A2: move.w $22(a6), d5
pub fn i_c175a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175A6;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C175A6: move.w $20(a6), d6
pub fn i_c175a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175AA;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C175AA: moveq #$7, d7
pub fn i_c175aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175AC;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C175AC: lsr.b #$1, d0
pub fn i_c175ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175AE;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C175AE: bcc.b $c175b4
pub fn i_c175ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175B0;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC175B4; return 10; }
    8
}
/// $C175B0: move.w d6, (a6)+
pub fn i_c175b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175B2;
    cpu.ir = 0x3CC6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C175B2: bra.b $c175b6
pub fn i_c175b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175B4;
    cpu.ir = 0x6002;
    cpu.pc = 0xC175B6;
    10
}
/// $C175B4: move.w d5, (a6)+
pub fn i_c175b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175B6;
    cpu.ir = 0x3CC5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C175B6: dbra d7, $c175ac
pub fn i_c175b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175BA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC175BA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC175BA; return 14; }
    cpu.pc = 0xC175AC;
    10
}
/// $C175BA: move.l (a1)+, d0
pub fn i_c175ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175BC;
    cpu.ir = 0x2019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C175BC: bne.b $c17562
pub fn i_c175bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175BE;
    cpu.ir = 0x66A4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17562; return 10; }
    8
}
/// $C175BE: move.w (a1)+, $9f2(a5)
pub fn i_c175be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175C2;
    cpu.ir = 0x3B59;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9F2);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C175C2: move.l a1, $9ee(a5)
pub fn i_c175c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175C6;
    cpu.ir = 0x2B49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0x9EE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C175C6: bra.w $c1753e
pub fn i_c175c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175CA;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1753E;
    10
}
/// $C175CA: addq.b #$1, $ac2(a5)
pub fn i_c175ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175CE;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C175CE: movea.l $abe(a5), a6
pub fn i_c175ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175D2;
    cpu.ir = 0x2C6D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $C175D2: lea.l $c1764c(pc), a3
pub fn i_c175d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175D6;
    cpu.ir = 0x47FA;
    cpu.a[3] = 0xC1764C;
    16
}
/// $C175D6: moveq #$3, d7
pub fn i_c175d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175D8;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C175D8: lsr.b #$1, d0
pub fn i_c175d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175DA;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C175DA: bcc.b $c175e2
pub fn i_c175da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175DC;
    cpu.ir = 0x6406;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC175E2; return 10; }
    8
}
/// $C175DC: move.l $4(a3), (a6)+
pub fn i_c175dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175E0;
    cpu.ir = 0x2CEB;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6]; cpu.a[6] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C175E0: bra.b $c175e4
pub fn i_c175e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175E2;
    cpu.ir = 0x6002;
    cpu.pc = 0xC175E4;
    10
}
/// $C175E2: move.l (a3), (a6)+
pub fn i_c175e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175E4;
    cpu.ir = 0x2CD3;
    let v2 = bus.read32(cpu.a[3]);
    let ad3 = cpu.a[6]; cpu.a[6] = ad3.wrapping_add(4);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C175E4: addq.l #$8, a3
pub fn i_c175e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175E6;
    cpu.ir = 0x508B;
    cpu.a[3] = cpu.a[3].wrapping_add(8);
    8
}
/// $C175E6: dbra d7, $c175d8
pub fn i_c175e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175EA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC175EA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC175EA; return 14; }
    cpu.pc = 0xC175D8;
    10
}
/// $C175EA: move.l a6, $abe(a5)
pub fn i_c175ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175EE;
    cpu.ir = 0x2B4E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C175EE: subq.b #$1, $ac2(a5)
pub fn i_c175ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175F2;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C175F2: bra.b $c175ba
pub fn i_c175f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175F4;
    cpu.ir = 0x60C6;
    cpu.pc = 0xC175BA;
    10
}
/// $C175F4: cmpi.b #$ff, d0
pub fn i_c175f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175F8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C175F8: beq.b $c17606
pub fn i_c175f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175FA;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17606; return 10; }
    8
}
/// $C175FA: move.b d0, $bc4(a5)
pub fn i_c175fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC175FE;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C175FE: move.l a1, $bc0(a5)
pub fn i_c175fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17602;
    cpu.ir = 0x2B49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xBC0);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17602: bra.w $c1755c
pub fn i_c17602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17606;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1755C;
    10
}
/// $C17606: subq.b #$1, $bc4(a5)
pub fn i_c17606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1760A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xBC4);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1760A: beq.w $c1755c
pub fn i_c1760a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1760E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1755C; return 10; }
    8
}
/// $C1760E: movea.l $bc0(a5), a1
pub fn i_c1760e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17612;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xBC0);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C17612: bra.w $c1755c
pub fn i_c17612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17616;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1755C;
    10
}
/// $C17616: clr.l $400002.l
pub fn i_c17616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1761C;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x400002);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x400002, r2);
    20
}
/// $C1761C: jsr $c12a0e(pc)
pub fn i_c1761c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17620;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17620);
    cpu.pc = 0xC12A0E;
    16
}
/// $C17620: jsr $c10fae(pc)
pub fn i_c17620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17624;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17624);
    cpu.pc = 0xC10FAE;
    16
}
/// $C17624: jsr $c1727e(pc)
pub fn i_c17624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17628;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17628);
    cpu.pc = 0xC1727E;
    16
}
/// $C17628: move.b #$3, $320000.l
pub fn i_c17628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17630;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C17630: clr.b $bbd(a5)
pub fn i_c17630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17634;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBD);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17634: move.b d0, $3a001b.l
pub fn i_c17634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1763A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001B, v1 as u8);
    16
}
/// $C1763A: move.b d0, $3a0013.l
pub fn i_c1763a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17640;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C17640: clr.b $b8d(a5)
pub fn i_c17640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17644;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xB8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17644: bset.b #$7, $a80(a5)
pub fn i_c17644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1764A;
    cpu.ir = 0x8ED;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[5].wrapping_add(0xA80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C1764A: rts
pub fn i_c1764a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1764C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1764C: ?
pub fn i_c1764c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1764E;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1766C: btst.l d0, d2
pub fn i_c1766c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1766E;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1766E: ori.b #$2, d3
pub fn i_c1766e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17672;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17672: ?
pub fn i_c17672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17674;
    cpu.ir = 0x1C0C;
    let v1 = cpu.a[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17674: ?
pub fn i_c17674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17676;
    cpu.ir = 0x7FF;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C17676: ori.b #$20, d5
pub fn i_c17676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1767A;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1767A: ?
pub fn i_c1767a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1767C;
    cpu.ir = 0x1C0C;
    let v1 = cpu.a[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1767C: ?
pub fn i_c1767c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1767E;
    cpu.ir = 0x7FF;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C1767E: ori.b #$e2, d3
pub fn i_c1767e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17682;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xE2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17682: ?
pub fn i_c17682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17684;
    cpu.ir = 0x1C0C;
    let v1 = cpu.a[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17684: ?
pub fn i_c17684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17686;
    cpu.ir = 0x7FF;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C17686: move.l d2, d0
pub fn i_c17686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17688;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17688: ori.b #$42, d3
pub fn i_c17688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1768C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x42) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1768C: move.l a4, d3
pub fn i_c1768c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1768E;
    cpu.ir = 0x260C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1768E: ?
pub fn i_c1768e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17690;
    cpu.ir = 0x7FF;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C17690: ori.b #$1, d5
pub fn i_c17690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17694;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17694: move.l a4, -(a3)
pub fn i_c17694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17696;
    cpu.ir = 0x270C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let o2 = cpu.a[3];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[3] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17696: ?
pub fn i_c17696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17698;
    cpu.ir = 0x7FF;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C17698: ori.b #$5c, d3
pub fn i_c17698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1769C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x5C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1769C: move.l a4, d3
pub fn i_c1769c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1769E;
    cpu.ir = 0x260C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1769E: ?
pub fn i_c1769e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176A0;
    cpu.ir = 0x7FF;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C176A0: ori.b #$1, d5
pub fn i_c176a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176A4;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176A4: move.l a4, -(a3)
pub fn i_c176a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176A6;
    cpu.ir = 0x270C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let o2 = cpu.a[3];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[3] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C176A6: ?
pub fn i_c176a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176A8;
    cpu.ir = 0x7FF;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C176A8: ori.b #$1, d5
pub fn i_c176a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176AC;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176AC: move.l a4, -(a3)
pub fn i_c176ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176AE;
    cpu.ir = 0x270C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let o2 = cpu.a[3];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[3] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C176AE: ?
pub fn i_c176ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176B0;
    cpu.ir = 0x7FF;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C176B0: ori.b #$44, d3
pub fn i_c176b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176B4;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x44) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176B4: addx.b -(a0), -(a3)
pub fn i_c176b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176B6;
    cpu.ir = 0xD708;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    let v1 = bus.read8(cpu.a[0]) as u32;
    cpu.a[3] = cpu.a[3].wrapping_sub(1);
    let v2 = bus.read8(cpu.a[3]) as u32;
    let r = addx_with_flags(cpu, v2, v1, x, Size::Byte);
    bus.write8(cpu.a[3], r as u8);
    4
}
/// $C176B6: ori.b #$ff, d1
pub fn i_c176b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176BA;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176BA: ori.b #$84, d3
pub fn i_c176ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176BE;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176BE: abcd.b -(a0), -(a3)
pub fn i_c176be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176C0;
    cpu.ir = 0xC708;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    let s = bus.read8(cpu.a[0]);
    cpu.a[3] = cpu.a[3].wrapping_sub(1);
    let d = bus.read8(cpu.a[3]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[3], r);
    18
}
/// $C176C0: subi.b #$5, d5
pub fn i_c176c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176C4;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176C4: btst.l d2, d5
pub fn i_c176c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176C6;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176C6: btst.l d2, d5
pub fn i_c176c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176C8;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176C8: btst.l d2, d5
pub fn i_c176c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176CA;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176CA: btst.l d2, d5
pub fn i_c176ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176CC;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176CC: btst.l d2, d5
pub fn i_c176cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176CE;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176CE: btst.l d2, d5
pub fn i_c176ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176D0;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176D0: ?
pub fn i_c176d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176D2;
    cpu.ir = 0x6FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C176D2: ori.b #$a4, d3
pub fn i_c176d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176D6;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xA4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176D6: addx.b -(a0), -(a3)
pub fn i_c176d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176D8;
    cpu.ir = 0xD708;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    let v1 = bus.read8(cpu.a[0]) as u32;
    cpu.a[3] = cpu.a[3].wrapping_sub(1);
    let v2 = bus.read8(cpu.a[3]) as u32;
    let r = addx_with_flags(cpu, v2, v1, x, Size::Byte);
    bus.write8(cpu.a[3], r as u8);
    4
}
/// $C176D8: btst.l d3, d7
pub fn i_c176d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176DA;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176DA: btst.l d3, d7
pub fn i_c176da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176DC;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176DC: btst.l d3, d7
pub fn i_c176dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176DE;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176DE: btst.l d3, d7
pub fn i_c176de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176E0;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176E0: btst.l d3, d7
pub fn i_c176e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176E2;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176E2: btst.l d3, d7
pub fn i_c176e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176E4;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176E4: btst.l d3, d7
pub fn i_c176e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176E6;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176E6: btst.l d3, d7
pub fn i_c176e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176E8;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176E8: btst.l d3, d7
pub fn i_c176e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176EA;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C176EA: dc.w $ffff
pub fn i_c176ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176EC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C176EC: ori.b #$46, d3
pub fn i_c176ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176F0;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x46) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176F0: addx.b -(a0), -(a3)
pub fn i_c176f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176F2;
    cpu.ir = 0xD708;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    let v1 = bus.read8(cpu.a[0]) as u32;
    cpu.a[3] = cpu.a[3].wrapping_sub(1);
    let v2 = bus.read8(cpu.a[3]) as u32;
    let r = addx_with_flags(cpu, v2, v1, x, Size::Byte);
    bus.write8(cpu.a[3], r as u8);
    4
}
/// $C176F2: andi.b #$ff, d3
pub fn i_c176f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176F6;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176F6: ori.b #$86, d3
pub fn i_c176f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176FA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x86) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C176FA: abcd.b -(a0), -(a3)
pub fn i_c176fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC176FC;
    cpu.ir = 0xC708;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    let s = bus.read8(cpu.a[0]);
    cpu.a[3] = cpu.a[3].wrapping_sub(1);
    let d = bus.read8(cpu.a[3]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[3], r);
    18
}
/// $C176FC: ?
pub fn i_c176fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17700;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((9) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C17700: movep.w $909(a1), d4
pub fn i_c17700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17704;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17704: movep.w $909(a1), d4
pub fn i_c17704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17708;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17708: movep.w $909(a1), d4
pub fn i_c17708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1770C;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1770C: ?
pub fn i_c1770c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1770E;
    cpu.ir = 0xAFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1770E: ori.b #$a6, d3
pub fn i_c1770e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17712;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xA6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17712: addx.b -(a0), -(a3)
pub fn i_c17712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17714;
    cpu.ir = 0xD708;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    let v1 = bus.read8(cpu.a[0]) as u32;
    cpu.a[3] = cpu.a[3].wrapping_sub(1);
    let v2 = bus.read8(cpu.a[3]) as u32;
    let r = addx_with_flags(cpu, v2, v1, x, Size::Byte);
    bus.write8(cpu.a[3], r as u8);
    4
}
/// $C17714: movep.w $b0b(a3), d5
pub fn i_c17714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17718;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17718: movep.w $b0b(a3), d5
pub fn i_c17718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1771C;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1771C: movep.w $b0b(a3), d5
pub fn i_c1771c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17720;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17720: movep.w $b0b(a3), d5
pub fn i_c17720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17724;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17724: movep.w -$1(a3), d5
pub fn i_c17724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17728;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xFFFFFFFF);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17728: ori.b #$48, d3
pub fn i_c17728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1772C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x48) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1772C: addx.b -(a0), -(a3)
pub fn i_c1772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1772E;
    cpu.ir = 0xD708;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    let v1 = bus.read8(cpu.a[0]) as u32;
    cpu.a[3] = cpu.a[3].wrapping_sub(1);
    let v2 = bus.read8(cpu.a[3]) as u32;
    let r = addx_with_flags(cpu, v2, v1, x, Size::Byte);
    bus.write8(cpu.a[3], r as u8);
    4
}
/// $C1772E: btst.l d1, d3
pub fn i_c1772e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17730;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C17730: ?
pub fn i_c17730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17734;
    cpu.ir = 0xC0D;
    let v1 = cpu.a[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $C17734: dc.w $ffff
pub fn i_c17734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17736;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17736: ori.b #$c8, d3
pub fn i_c17736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1773A;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xC8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1773A: addx.b -(a0), -(a3)
pub fn i_c1773a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1773C;
    cpu.ir = 0xD708;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    let v1 = bus.read8(cpu.a[0]) as u32;
    cpu.a[3] = cpu.a[3].wrapping_sub(1);
    let v2 = bus.read8(cpu.a[3]) as u32;
    let r = addx_with_flags(cpu, v2, v1, x, Size::Byte);
    bus.write8(cpu.a[3], r as u8);
    4
}
/// $C1773C: movep.w $f0f(a7), d7
pub fn i_c1773c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17740;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17740: movep.w $f0f(a7), d7
pub fn i_c17740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17744;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17744: movep.w $f0f(a7), d7
pub fn i_c17744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17748;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17748: movep.w $f0f(a7), d7
pub fn i_c17748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1774C;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1774C: movep.w $f0f(a7), d7
pub fn i_c1774c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17750;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17750: movep.w $6364(a7), d7
pub fn i_c17750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17754;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0x6364);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17978: move.l d2, d0
pub fn i_c17978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1797A;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1797A: ori.b #$8, d3
pub fn i_c1797a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1797E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1797E: ?
pub fn i_c1797e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17982;
    cpu.ir = 0x60D;
    let v1 = cpu.a[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Byte, true);
    cpu.a[5] = r2;
    8
}
/// $C17982: ori.b #$1, d5
pub fn i_c17982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17986;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17986: ?
pub fn i_c17986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1798A;
    cpu.ir = 0x60D;
    let v1 = cpu.a[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Byte, true);
    cpu.a[5] = r2;
    8
}
/// $C1798A: ori.b #$1, d5
pub fn i_c1798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1798E;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1798E: movep.w $1617(a0), d3
pub fn i_c1798e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17992;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x1617);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17992: move.b (a1)+, d4
pub fn i_c17992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17994;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C17994: move.b (a3)+, d5
pub fn i_c17994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17996;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C17996: dc.w $ffff
pub fn i_c17996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17998;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17998: ori.b #$1, d5
pub fn i_c17998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1799C;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1799C: movep.w $1c1d(a0), d3
pub fn i_c1799c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179A0;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x1C1D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C179A0: move.b (a7)+, d7
pub fn i_c179a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179A2;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C179A2: move.l -(a1), d0
pub fn i_c179a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179A4;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C179A4: dc.w $ffff
pub fn i_c179a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179A6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C179A6: ori.b #$1, d5
pub fn i_c179a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179AA;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C179AA: movep.w $2223(a0), d3
pub fn i_c179aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179AE;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x2223);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C179AE: move.l -(a5), d2
pub fn i_c179ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179B0;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C179B0: move.l -(a7), d3
pub fn i_c179b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179B2;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C179B2: dc.w $ffff
pub fn i_c179b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179B4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C179B4: ori.b #$21, d5
pub fn i_c179b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179B8;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x21) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C179B8: movep.w $2829(a0), d3
pub fn i_c179b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179BC;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x2829);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C179BC: move.l -$1(a3), d5
pub fn i_c179bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179C0;
    cpu.ir = 0x2A2B;
    let ad2 = cpu.a[3].wrapping_add(0xFFFFFFFF);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C179C0: ori.b #$3, d0
pub fn i_c179c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179C4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C179C2: ori.b #$6c, d3
pub fn i_c179c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179C6;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x6C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C179C4: moveq #$6c, d1
pub fn i_c179c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179C6;
    cpu.ir = 0x726C;
    cpu.d[1] = 0x6C;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C179C6: dc.w $a708
pub fn i_c179c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179C8;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C179C8: move.w $3a3bffff.l, d4
pub fn i_c179c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179CE;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3BFFFF) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C179CE: ori.b #$1, d5
pub fn i_c179ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179D2;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C179D2: dc.w $a708
pub fn i_c179d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179D4;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C179D4: ?
pub fn i_c179d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179D6;
    cpu.ir = 0x4748;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C179D6: ?
pub fn i_c179d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179D8;
    cpu.ir = 0x494A;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C179E6: ori.b #$ca, d3
pub fn i_c179e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179EA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xCA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C179EA: dc.w $a708
pub fn i_c179ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179EC;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C179EC: move.l $2e2f(a5), d6
pub fn i_c179ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179F0;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C179F0: dc.w $ffff
pub fn i_c179f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179F2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C179F2: ori.b #$21, d5
pub fn i_c179f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179F6;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x21) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C179F6: dc.w $a708
pub fn i_c179f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179F8;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C179F8: ?
pub fn i_c179f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179FA;
    cpu.ir = 0x3C3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C179FA: ?
pub fn i_c179fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC179FC;
    cpu.ir = 0x3EFF;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C179FC: ori.b #$21, d5
pub fn i_c179fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A00;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x21) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A00: dc.w $a708
pub fn i_c17a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A02;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C17A02: ?
pub fn i_c17a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A04;
    cpu.ir = 0x4C4D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17A04: dc.w $ffff
pub fn i_c17a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A06;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A06: ori.b #$3, d0
pub fn i_c17a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A0A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A08: ori.b #$4a, d3
pub fn i_c17a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A0C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A0C: dc.w $a708
pub fn i_c17a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A0E;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C17A0E: move.w $33(a1, d3.w), d0
pub fn i_c17a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A12;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C17A12: dc.w $ffff
pub fn i_c17a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A14;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A14: ori.b #$1, d5
pub fn i_c17a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A18;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A18: dc.w $a708
pub fn i_c17a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A1A;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C17A1A: move.w d0, $4142(a7)
pub fn i_c17a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A1E;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4142);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17A1E: dc.w $ffff
pub fn i_c17a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A20;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A20: ori.b #$1, d5
pub fn i_c17a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A24;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A24: dc.w $a708
pub fn i_c17a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A26;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C17A26: trap #$f
pub fn i_c17a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A28;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $C17A28: addq.w #$8, (a1)
pub fn i_c17a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A2A;
    cpu.ir = 0x5051;
    let ad1 = cpu.a[1];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17A2A: dc.w $ffff
pub fn i_c17a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A2C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A2C: ori.b #$3, d0
pub fn i_c17a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A30;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A2E: ori.b #$ca, d3
pub fn i_c17a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A32;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xCA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A32: dc.w $a708
pub fn i_c17a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A34;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C17A34: move.w $37(a5, d3.w), d2
pub fn i_c17a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A38;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C17A38: dc.w $ffff
pub fn i_c17a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A3A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A3A: ori.b #$1, d5
pub fn i_c17a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A3E;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A3E: dc.w $a708
pub fn i_c17a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A40;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C17A40: ?
pub fn i_c17a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A42;
    cpu.ir = 0x4344;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17A42: ?
pub fn i_c17a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A44;
    cpu.ir = 0x4546;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17A44: dc.w $ffff
pub fn i_c17a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A46;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A46: ori.b #$1, d5
pub fn i_c17a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A4A;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A4A: dc.w $a708
pub fn i_c17a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A4C;
    cpu.ir = 0xA708;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C17A4C: addq.w #$1, (a3)
pub fn i_c17a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A4E;
    cpu.ir = 0x5253;
    let ad1 = cpu.a[3];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17A4E: addq.w #$2, (a5)
pub fn i_c17a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A50;
    cpu.ir = 0x5455;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17A50: dc.w $ffff
pub fn i_c17a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A52;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A52: ori.b #$3, d0
pub fn i_c17a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A56;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A54: ori.b #$6c, d3
pub fn i_c17a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A58;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x6C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A56: moveq #$6c, d1
pub fn i_c17a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A58;
    cpu.ir = 0x726C;
    cpu.d[1] = 0x6C;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17A58: cmpm.b (a0)+, (a3)+
pub fn i_c17a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A5A;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17A5A: move.w $3a3bffff.l, d4
pub fn i_c17a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A60;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3BFFFF) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C17A60: ori.b #$1, d5
pub fn i_c17a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A64;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A64: cmpm.b (a0)+, (a3)+
pub fn i_c17a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A66;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17A66: ?
pub fn i_c17a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A68;
    cpu.ir = 0x4748;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17A68: ?
pub fn i_c17a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A6A;
    cpu.ir = 0x494A;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17A78: ori.b #$ca, d3
pub fn i_c17a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A7C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xCA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A7C: cmpm.b (a0)+, (a3)+
pub fn i_c17a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A7E;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17A7E: move.l $2e2f(a5), d6
pub fn i_c17a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A82;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17A82: dc.w $ffff
pub fn i_c17a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A84;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A84: ori.b #$21, d5
pub fn i_c17a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A88;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x21) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A88: cmpm.b (a0)+, (a3)+
pub fn i_c17a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A8A;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17A8A: ?
pub fn i_c17a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A8C;
    cpu.ir = 0x3C3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17A8C: ?
pub fn i_c17a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A8E;
    cpu.ir = 0x3EFF;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17A8E: ori.b #$21, d5
pub fn i_c17a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A92;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x21) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A92: cmpm.b (a0)+, (a3)+
pub fn i_c17a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A94;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17A94: ?
pub fn i_c17a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A96;
    cpu.ir = 0x4C4D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17A96: dc.w $ffff
pub fn i_c17a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A98;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17A98: ori.b #$3, d0
pub fn i_c17a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A9C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A9A: ori.b #$4a, d3
pub fn i_c17a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17A9E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17A9E: cmpm.b (a0)+, (a3)+
pub fn i_c17a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AA0;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17AA0: move.w $33(a1, d3.w), d0
pub fn i_c17aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AA4;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C17AA4: dc.w $ffff
pub fn i_c17aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AA6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17AA6: ori.b #$1, d5
pub fn i_c17aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AAA;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17AAA: cmpm.b (a0)+, (a3)+
pub fn i_c17aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AAC;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17AAC: move.w d0, $4142(a7)
pub fn i_c17aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AB0;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4142);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17AB0: dc.w $ffff
pub fn i_c17ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AB2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17AB2: ori.b #$1, d5
pub fn i_c17ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AB6;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17AB6: cmpm.b (a0)+, (a3)+
pub fn i_c17ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AB8;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17AB8: trap #$f
pub fn i_c17ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ABA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $C17ABA: addq.w #$8, (a1)
pub fn i_c17aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ABC;
    cpu.ir = 0x5051;
    let ad1 = cpu.a[1];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17ABC: dc.w $ffff
pub fn i_c17abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ABE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17ABE: ori.b #$3, d0
pub fn i_c17abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AC2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17AC0: ori.b #$ca, d3
pub fn i_c17ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AC4;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xCA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17AC4: cmpm.b (a0)+, (a3)+
pub fn i_c17ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AC6;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17AC6: move.w $37(a5, d3.w), d2
pub fn i_c17ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ACA;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C17ACA: dc.w $ffff
pub fn i_c17aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ACC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17ACC: ori.b #$1, d5
pub fn i_c17acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AD0;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17AD0: cmpm.b (a0)+, (a3)+
pub fn i_c17ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AD2;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17AD2: ?
pub fn i_c17ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AD4;
    cpu.ir = 0x4344;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17AD4: ?
pub fn i_c17ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AD6;
    cpu.ir = 0x4546;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17AD6: dc.w $ffff
pub fn i_c17ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AD8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17AD8: ori.b #$1, d5
pub fn i_c17ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ADC;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17ADC: cmpm.b (a0)+, (a3)+
pub fn i_c17adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ADE;
    cpu.ir = 0xB708;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[3];
    let v2 = bus.read8(ax) as u32;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C17ADE: addq.w #$1, (a3)
pub fn i_c17ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AE0;
    cpu.ir = 0x5253;
    let ad1 = cpu.a[3];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17AE0: addq.w #$2, (a5)
pub fn i_c17ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AE2;
    cpu.ir = 0x5455;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17AE2: dc.w $ffff
pub fn i_c17ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AE4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17AE4: ori.b #$f9, d0
pub fn i_c17ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AE8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17AE6: lea.l $10f300.l, a5
pub fn i_c17ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AEC;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C17AE8: ori.b #$0, (a0)
pub fn i_c17ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AEC;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17AEC: move.w #$2700, sr
pub fn i_c17aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AF0;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C17AF0: clr.w $a80(a5)
pub fn i_c17af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AF4;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xA80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C17AF4: jsr $c10fae(pc)
pub fn i_c17af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17AF8);
    cpu.pc = 0xC10FAE;
    16
}
/// $C17AF8: move.b d0, $3a0003.l
pub fn i_c17af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AFE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C17AFE: move.b d0, $3a000b.l
pub fn i_c17afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B04;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C17B04: moveq #$0, d0
pub fn i_c17b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B06;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17B06: move.l d0, d1
pub fn i_c17b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B08;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17B08: move.l d0, d2
pub fn i_c17b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B0A;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17B0A: move.l d0, d3
pub fn i_c17b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B0C;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17B0C: lea.l $110000.l, a0
pub fn i_c17b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x110000;
    20
}
/// $C17B12: move.w #$1ff, d7
pub fn i_c17b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B16;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17B16: move.b d0, $300001.l
pub fn i_c17b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B1C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C17B1C: movem.l d0-d3, -(a0)
pub fn i_c17b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B20;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B20: movem.l d0-d3, -(a0)
pub fn i_c17b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B24;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B24: movem.l d0-d3, -(a0)
pub fn i_c17b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B28;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B28: movem.l d0-d3, -(a0)
pub fn i_c17b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B2C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B2C: movem.l d0-d3, -(a0)
pub fn i_c17b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B30;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B30: movem.l d0-d3, -(a0)
pub fn i_c17b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B34;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B34: movem.l d0-d3, -(a0)
pub fn i_c17b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B38;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B38: movem.l d0-d3, -(a0)
pub fn i_c17b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B3C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B3C: dbra d7, $c17b16
pub fn i_c17b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B40;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC17B40; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC17B40; return 14; }
    cpu.pc = 0xC17B16;
    10
}
/// $C17B40: move.b d0, $3a000f.l
pub fn i_c17b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B46;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C17B46: lea.l $402000.l, a0
pub fn i_c17b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C17B4C: move.w #$7f, d7
pub fn i_c17b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B50;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17B50: movem.l d0-d3, -(a0)
pub fn i_c17b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B54;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B54: movem.l d0-d3, -(a0)
pub fn i_c17b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B58;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B58: movem.l d0-d3, -(a0)
pub fn i_c17b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B5C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B5C: movem.l d0-d3, -(a0)
pub fn i_c17b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B60;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B60: dbra d7, $c17b50
pub fn i_c17b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B64;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC17B64; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC17B64; return 14; }
    cpu.pc = 0xC17B50;
    10
}
/// $C17B64: move.b d0, $300001.l
pub fn i_c17b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B6A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C17B6A: move.b d0, $3a001f.l
pub fn i_c17b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B70;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C17B70: lea.l $402000.l, a0
pub fn i_c17b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C17B76: move.w #$7f, d7
pub fn i_c17b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B7A;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17B7A: movem.l d0-d3, -(a0)
pub fn i_c17b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B7E;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B7E: movem.l d0-d3, -(a0)
pub fn i_c17b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B82;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B82: movem.l d0-d3, -(a0)
pub fn i_c17b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B86;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B86: movem.l d0-d3, -(a0)
pub fn i_c17b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B8A;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B8A: dbra d7, $c17b7a
pub fn i_c17b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B8E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC17B8E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC17B8E; return 14; }
    cpu.pc = 0xC17B7A;
    10
}
/// $C17B8E: move.l #$eee0000, $400002.l
pub fn i_c17b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B98;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C17B98: move.l #$e000000, $400022.l
pub fn i_c17b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BA2;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C17BA2: move.l #$100f0000, $4001ea.l
pub fn i_c17ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BAC;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x100F0000;
    bus.write32(0x4001EA, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C17BAC: lea.l $10f300.l, a5
pub fn i_c17bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BB2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C17BB2: move.b #$ff, $b8d(a5)
pub fn i_c17bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BB8;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17BB8: move.b #$1, $bbc(a5)
pub fn i_c17bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BBE;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17BBE: move.b $c00401.l, $a83(a5)
pub fn i_c17bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BC6;
    cpu.ir = 0x1B79;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA83);
    bus.write8(ad3, v2 as u8);
    24
}
/// $C17BC6: move.b #$80, $10fd82.l
pub fn i_c17bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BCE;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD82, v1 as u8);
    16
}
/// $C17BCE: move.l #$10ff00, $abe(a5)
pub fn i_c17bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BD6;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17BD6: move.b d0, $3a0001.l
pub fn i_c17bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BDC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C17BDC: move.w #$7, $3c000c.l
pub fn i_c17bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BE4;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C17BE4: move.w #$2000, sr
pub fn i_c17be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BE8;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C17BE8: move.w #$0, $9d8(a5)
pub fn i_c17be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BEE;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17BEE: jsr $c12a0e(pc)
pub fn i_c17bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17BF2);
    cpu.pc = 0xC12A0E;
    16
}
/// $C17BF2: jsr $c11c42(pc)
pub fn i_c17bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17BF6);
    cpu.pc = 0xC11C42;
    16
}
/// $C17BF6: addq.b #$1, $ac2(a5)
pub fn i_c17bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BFA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17BFA: movea.l $abe(a5), a0
pub fn i_c17bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BFE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17BFE: move.l #$c17d44, (a0)+
pub fn i_c17bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C04;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17D44;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17C04: move.l #$c17e20, (a0)+
pub fn i_c17c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C0A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17C0A: jsr $c17eaa(pc)
pub fn i_c17c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C0E);
    cpu.pc = 0xC17EAA;
    16
}
/// $C17C0E: nop
pub fn i_c17c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C10;
    cpu.ir = 0x4E71;
    4
}
/// $C17C10: clr.l (a0)+
pub fn i_c17c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C12;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C17C12: move.w #$7318, d2
pub fn i_c17c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C16;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7318;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C16: move.b $ad2(a5), d0
pub fn i_c17c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C1A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C1A: jsr $c17d12(pc)
pub fn i_c17c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C1E);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C1E: move.w #$7258, d2
pub fn i_c17c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C22;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C22: move.b $ad3(a5), d0
pub fn i_c17c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C26;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C26: jsr $c17d12(pc)
pub fn i_c17c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C2A);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C2A: move.w #$72b8, d2
pub fn i_c17c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C2E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x72B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C2E: move.b $ad4(a5), d0
pub fn i_c17c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C32;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C32: jsr $c17d12(pc)
pub fn i_c17c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C36);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C36: move.w #$3, (a0)+
pub fn i_c17c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C3A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17C3A: move.w #$7398, (a0)+
pub fn i_c17c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C3E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7398;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17C3E: jsr $c17e7a(pc)
pub fn i_c17c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C42);
    cpu.pc = 0xC17E7A;
    16
}
/// $C17C42: clr.w (a0)+
pub fn i_c17c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C44;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17C44: move.l a0, $abe(a5)
pub fn i_c17c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C48;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17C48: subq.b #$1, $ac2(a5)
pub fn i_c17c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C4C;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17C4C: move.w $9d8(a5), $9da(a5)
pub fn i_c17c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C52;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C17C52: jsr $c17cca(pc)
pub fn i_c17c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C56);
    cpu.pc = 0xC17CCA;
    16
}
/// $C17C56: jsr $c11c42.l
pub fn i_c17c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC17C5C);
    cpu.pc = 0xC11C42;
    16
}
/// $C17C5C: addq.b #$1, $ac2(a5)
pub fn i_c17c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C60;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17C60: movea.l $abe(a5), a0
pub fn i_c17c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C64;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17C64: clr.l (a0)+
pub fn i_c17c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C66;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C17C66: move.w #$725a, d2
pub fn i_c17c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C6A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x725A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C6A: move.b $ad6(a5), d0
pub fn i_c17c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C6E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C6E: jsr $c17d12(pc)
pub fn i_c17c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C72);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C72: move.w #$72ba, d2
pub fn i_c17c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C76;
    cpu.ir = 0x343C;
    let v1: u32 = 0x72BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C76: move.b $ad7(a5), d0
pub fn i_c17c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C7A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C7A: jsr $c17d12(pc)
pub fn i_c17c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C7E);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C7E: move.w #$731a, d2
pub fn i_c17c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C82;
    cpu.ir = 0x343C;
    let v1: u32 = 0x731A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C82: move.b $ad8(a5), d0
pub fn i_c17c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C86;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C86: jsr $c17d12(pc)
pub fn i_c17c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C8A);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C8A: clr.w (a0)+
pub fn i_c17c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C8C;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17C8C: move.l a0, $abe(a5)
pub fn i_c17c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C90;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17C90: subq.b #$1, $ac2(a5)
pub fn i_c17c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C94;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17C94: jsr $c1727e(pc)
pub fn i_c17c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C98);
    cpu.pc = 0xC1727E;
    16
}
/// $C17C98: move.b $a97(a5), d0
pub fn i_c17c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C9C;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C9C: btst.b #$4, d0
pub fn i_c17c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CA0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C17CA0: bne.w $c17fca
pub fn i_c17ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17FCA; return 10; }
    8
}
/// $C17CA4: andi.b #$3, d0
pub fn i_c17ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CA8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17CA8: beq.b $c17c56
pub fn i_c17ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CAA;
    cpu.ir = 0x67AC;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17C56; return 10; }
    8
}
/// $C17CAA: moveq #$1, d1
pub fn i_c17caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CAC;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17CAC: andi.b #$2, d0
pub fn i_c17cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CB0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17CB0: bne.b $c17cb4
pub fn i_c17cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CB2;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17CB4; return 10; }
    8
}
/// $C17CB2: moveq #$ff, d1
pub fn i_c17cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CB4;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17CB4: add.w $9d8(a5), d1
pub fn i_c17cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CB8;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C17CB8: bmi.b $c17c56
pub fn i_c17cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CBA;
    cpu.ir = 0x6B9C;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC17C56; return 10; }
    8
}
/// $C17CBA: cmpi.w #$6, d1
pub fn i_c17cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CBE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $C17CBE: bcc.b $c17c56
pub fn i_c17cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CC0;
    cpu.ir = 0x6496;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC17C56; return 10; }
    8
}
/// $C17CC0: move.w d1, $9da(a5)
pub fn i_c17cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CC4;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17CC4: jsr $c17cca(pc)
pub fn i_c17cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17CC8);
    cpu.pc = 0xC17CCA;
    16
}
/// $C17CC8: bra.b $c17c56
pub fn i_c17cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CCA;
    cpu.ir = 0x608C;
    cpu.pc = 0xC17C56;
    10
}
/// $C17CCA: addq.b #$1, $ac2(a5)
pub fn i_c17cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CCE;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17CCE: movea.l $abe(a5), a0
pub fn i_c17cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CD2;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17CD2: clr.l (a0)+
pub fn i_c17cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CD4;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C17CD4: move.w #$20ff, d1
pub fn i_c17cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17CD8: jsr $c17cf6(pc)
pub fn i_c17cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17CDC);
    cpu.pc = 0xC17CF6;
    16
}
/// $C17CDC: move.w $9da(a5), $9d8(a5)
pub fn i_c17cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CE2;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C17CE2: move.w #$11ff, d1
pub fn i_c17ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17CE6: jsr $c17cf6(pc)
pub fn i_c17ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17CEA);
    cpu.pc = 0xC17CF6;
    16
}
/// $C17CEA: clr.w (a0)+
pub fn i_c17cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CEC;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17CEC: move.l a0, $abe(a5)
pub fn i_c17cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CF0;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17CF0: subq.b #$1, $ac2(a5)
pub fn i_c17cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CF4;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17CF4: rts
pub fn i_c17cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17CF6: move.w #$3, (a0)+
pub fn i_c17cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CFA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17CFA: move.w $9d8(a5), d0
pub fn i_c17cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CFE;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C17CFE: move.w d0, d2
pub fn i_c17cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D00;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17D00: add.w d0, d0
pub fn i_c17d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D02;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17D02: add.w d0, d2
pub fn i_c17d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D04;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17D04: addi.w #$70e5, d2
pub fn i_c17d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D08;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x70E5, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C17D08: move.w d2, (a0)+
pub fn i_c17d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D0A;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D0A: move.w #$1108, (a0)+
pub fn i_c17d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D0E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D0E: move.w d1, (a0)+
pub fn i_c17d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D10;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D10: rts
pub fn i_c17d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17D12: move.w #$3, (a0)+
pub fn i_c17d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D16;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D16: move.w d2, (a0)+
pub fn i_c17d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D18;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D18: move.w #$108, (a0)+
pub fn i_c17d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D1C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D1C: move.b #$20, (a0)+
pub fn i_c17d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D20;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C17D20: move.b d0, d1
pub fn i_c17d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D22;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17D22: lsr.b #$4, d0
pub fn i_c17d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D24;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C17D24: andi.b #$f, d0
pub fn i_c17d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D28;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D28: beq.b $c17d32
pub fn i_c17d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D2A;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17D32; return 10; }
    8
}
/// $C17D2A: addi.b #$30, d0
pub fn i_c17d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D2E;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D2E: move.b d0, -$1(a0)
pub fn i_c17d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D32;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17D32: andi.b #$f, d1
pub fn i_c17d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D36;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D36: addi.b #$30, d1
pub fn i_c17d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D3A;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D3A: move.b d1, (a0)+
pub fn i_c17d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D3C;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C17D3C: move.w #$ff00, (a0)+
pub fn i_c17d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D40;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D40: addq.w #$1, d6
pub fn i_c17d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D42;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C17D42: rts
pub fn i_c17d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17D44: ori.b #$45, d3
pub fn i_c17d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D48;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x45) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D48: movep.w $4841(a0), d0
pub fn i_c17d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D4C;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4841);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C17D4C: addq.w #$1, d4
pub fn i_c17d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D4E;
    cpu.ir = 0x5244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C17D4E: subq.w #$3, d1
pub fn i_c17d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D50;
    cpu.ir = 0x5741;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C17D50: addq.w #$1, d5
pub fn i_c17d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D52;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C17D52: movea.l (a4), a0
pub fn i_c17d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D54;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C17D54: ?
pub fn i_c17d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D56;
    cpu.ir = 0x4553;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17DEC: ori.b #$ff, d1
pub fn i_c17dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17DF0;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17DF0: move.l d2, d0
pub fn i_c17df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17DF2;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17DF2: ori.b #$43, d3
pub fn i_c17df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17DF6;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x43) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17DF6: ori.b #$45, d7
pub fn i_c17df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17DFA;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x45) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17DFA: dc.w $4f2d
pub fn i_c17dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17DFC;
    cpu.ir = 0x4F2D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17DFC: ?
pub fn i_c17dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17DFE;
    cpu.ir = 0x4745;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17DFE: dc.w $4f20
pub fn i_c17dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E00;
    cpu.ir = 0x4F20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E00: ?
pub fn i_c17e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E02;
    cpu.ir = 0x4D56;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E02: subq.b #$1, -(a0)
pub fn i_c17e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E04;
    cpu.ir = 0x5320;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C17E04: subq.w #$1, (a1)+
pub fn i_c17e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E06;
    cpu.ir = 0x5359;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17E06: subq.w #$1, (a4)
pub fn i_c17e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E08;
    cpu.ir = 0x5354;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17E08: ?
pub fn i_c17e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E0A;
    cpu.ir = 0x454D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E0A: move.l $4153(a0), d0
pub fn i_c17e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E0E;
    cpu.ir = 0x2028;
    let ad2 = cpu.a[0].wrapping_add(0x4153);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17E0E: ?
pub fn i_c17e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E10;
    cpu.ir = 0x4941;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E10: move.l $5333(a7), d0
pub fn i_c17e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E14;
    cpu.ir = 0x202F;
    let ad2 = cpu.a[7].wrapping_add(0x5333);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17E14: move.l -(a0), -(a4)
pub fn i_c17e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E16;
    cpu.ir = 0x2920;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C17E16: addq.w #$3, -(a5)
pub fn i_c17e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E18;
    cpu.ir = 0x5665;
    let ad1 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad1;
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    10
}
/// $C17E18: moveq #$2e, d1
pub fn i_c17e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E1A;
    cpu.ir = 0x722E;
    cpu.d[1] = 0x2E;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17E1A: move.l (a6, a7.l * 8), d0
pub fn i_c17e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E1E;
    cpu.ir = 0x2036;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add(cpu.a[7]);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C17E1E: ori.b #$1, d0
pub fn i_c17e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E22;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17E20: ori.b #$ff, d1
pub fn i_c17e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E24;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17E22: ?
pub fn i_c17e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E24;
    cpu.ir = 0xFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E24: move.l d2, d0
pub fn i_c17e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E26;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17E26: ori.b #$fd, d3
pub fn i_c17e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E2A;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xFD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17E2A: ori.b #$31, d7
pub fn i_c17e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E2E;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x31) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17E2E: move.w $3520534e.l, -(a4)
pub fn i_c17e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E34;
    cpu.ir = 0x3939;
    let v2 = bus.read16(0x3520534E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    22
}
/// $C17E34: dc.w $4b20
pub fn i_c17e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E36;
    cpu.ir = 0x4B20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E36: ?
pub fn i_c17e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E38;
    cpu.ir = 0x434F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E38: addq.w #$1, (a0)
pub fn i_c17e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E3A;
    cpu.ir = 0x5250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17E3A: ?
pub fn i_c17e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E3C;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E3C: ?
pub fn i_c17e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E3E;
    cpu.ir = 0x4154;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E3E: ?
pub fn i_c17e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E40;
    cpu.ir = 0x494F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E40: ?
pub fn i_c17e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E42;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E42: ori.b #$bb, d3
pub fn i_c17e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E46;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xBB) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17E46: dc.w $f208
pub fn i_c17e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E48;
    cpu.ir = 0xF208;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17E48: ori.b #$3, d1
pub fn i_c17e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E4C;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17E4C: subi.b #$7, d5
pub fn i_c17e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E50;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17E50: ?
pub fn i_c17e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E54;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((255) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C17E54: ori.b #$bc, d3
pub fn i_c17e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E58;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xBC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17E58: dc.w $f208
pub fn i_c17e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E5A;
    cpu.ir = 0xF208;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17E5A: ?
pub fn i_c17e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E5E;
    cpu.ir = 0xA0B;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 ^ 0xD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C17E5E: ?
pub fn i_c17e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E62;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E62: move.b (a7), d3
pub fn i_c17e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E64;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C17E64: dc.w $ffff
pub fn i_c17e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E66;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17E7A: move.w #$108, (a0)+
pub fn i_c17e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E7E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17E7E: moveq #$0, d0
pub fn i_c17e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E80;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17E80: move.b $ad5(a5), d0
pub fn i_c17e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E84;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17E84: add.w d0, d0
pub fn i_c17e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E86;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17E86: add.w d0, d0
pub fn i_c17e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E88;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17E88: move.l $c17e8e(pc, d0.w), (a0)+
pub fn i_c17e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E8C;
    cpu.ir = 0x20FB;
    let ad2 = 0xC17E8E_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C17E8C: rts
pub fn i_c17e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17E8E: subq.w #$1, (a5)
pub fn i_c17e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E90;
    cpu.ir = 0x5355;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17E90: ?
pub fn i_c17e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E92;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E92: ?
pub fn i_c17e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E94;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E94: ?
pub fn i_c17e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E96;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17E96: addq.w #$2, (a5)
pub fn i_c17e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E98;
    cpu.ir = 0x5455;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17EAA: move.l #$c17dec, (a0)+
pub fn i_c17eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EB0;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17DEC;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17EB0: rts
pub fn i_c17eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17EB2: subq.b #$1, $ae5(a5)
pub fn i_c17eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EB6;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAE5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17EB6: bcc.w $c17eee
pub fn i_c17eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC17EEE; return 10; }
    8
}
/// $C17EBA: addq.b #$1, $ac2(a5)
pub fn i_c17eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EBE;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17EBE: movea.l $abe(a5), a0
pub fn i_c17ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EC2;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17EC2: not.b $ae4(a5)
pub fn i_c17ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EC6;
    cpu.ir = 0x462D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17EC6: bpl.w $c17eda
pub fn i_c17ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ECA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC17EDA; return 10; }
    8
}
/// $C17ECA: move.b #$32, $ae5(a5)
pub fn i_c17eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ED0;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAE5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17ED0: move.l #$c17ef0, (a0)+
pub fn i_c17ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17ED6;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17EF0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17ED6: bra.w $c17ee6
pub fn i_c17ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EDA;
    cpu.ir = 0x6000;
    cpu.pc = 0xC17EE6;
    10
}
/// $C17EDA: move.b #$a, $ae5(a5)
pub fn i_c17eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EE0;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAE5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17EE0: move.l #$c17f18, (a0)+
pub fn i_c17ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EE6;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17EE6: move.l a0, $abe(a5)
pub fn i_c17ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EEA;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17EEA: subq.b #$1, $ac2(a5)
pub fn i_c17eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EEE;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17EEE: rts
pub fn i_c17eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17EF0: ori.b #$ff, d1
pub fn i_c17ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EF4;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17EF4: move.l d2, d0
pub fn i_c17ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EF6;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17EF6: ori.b #$bc, d3
pub fn i_c17ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EFA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xBC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17EFA: ori.b #$55, d7
pub fn i_c17efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EFE;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x55) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17EFE: subq.w #$1, a0
pub fn i_c17efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F00;
    cpu.ir = 0x5348;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    8
}
/// $C17F00: move.l $2d(a1, d5.w), d0
pub fn i_c17f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F04;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x2D).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C17F04: dc.w $4320
pub fn i_c17f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F06;
    cpu.ir = 0x4320;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17F06: clr.w (a5)
pub fn i_c17f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F08;
    cpu.ir = 0x4255;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17F08: addq.w #$2, (a4)
pub fn i_c17f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F0A;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17F0A: ?
pub fn i_c17f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F0C;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17F0C: movea.l (a4), a0
pub fn i_c17f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F0E;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C17F0E: dc.w $4f20
pub fn i_c17f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F10;
    cpu.ir = 0x4F20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17F10: ?
pub fn i_c17f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F12;
    cpu.ir = 0x4D45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17F12: link.w a5, #$ffff
pub fn i_c17f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F16;
    cpu.ir = 0x4E55;
    cpu.push32(bus, cpu.a[5]); cpu.a[5] = cpu.a[7]; cpu.a[7] = cpu.a[7].wrapping_add(0xFFFFFFFF);
    16
}
/// $C17F16: ori.b #$1, d0
pub fn i_c17f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F1A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17F18: ori.b #$ff, d1
pub fn i_c17f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F1C;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17F1A: ?
pub fn i_c17f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F1C;
    cpu.ir = 0xFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17F1C: move.l d2, d0
pub fn i_c17f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F1E;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17F1E: ori.b #$bc, d3
pub fn i_c17f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F22;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xBC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17F22: ori.b #$20, d7
pub fn i_c17f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F26;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17F26: move.l -(a0), d0
pub fn i_c17f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F28;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F28: move.l -(a0), d0
pub fn i_c17f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F2A;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F2A: move.l -(a0), d0
pub fn i_c17f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F2C;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F2C: move.l -(a0), d0
pub fn i_c17f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F2E;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F2E: move.l -(a0), d0
pub fn i_c17f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F30;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F30: move.l -(a0), d0
pub fn i_c17f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F32;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F32: move.l -(a0), d0
pub fn i_c17f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F34;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F34: move.l -(a0), d0
pub fn i_c17f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F36;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F36: move.l -(a0), d0
pub fn i_c17f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F38;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F38: move.l -(a0), d0
pub fn i_c17f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F3A;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F3A: move.l -(a0), d0
pub fn i_c17f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F3C;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F3C: dc.w $ffff
pub fn i_c17f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F3E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17F3E: ori.b #$8, d0
pub fn i_c17f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F42;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17F40: move.l a0, -(a7)
pub fn i_c17f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F42;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F42: subq.b #$1, $ae5(a5)
pub fn i_c17f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F46;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAE5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17F46: bcc.w $c17f7e
pub fn i_c17f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC17F7E; return 10; }
    8
}
/// $C17F4A: addq.b #$1, $ac2(a5)
pub fn i_c17f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F4E;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17F4E: movea.l $abe(a5), a0
pub fn i_c17f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F52;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17F52: not.b $ae4(a5)
pub fn i_c17f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F56;
    cpu.ir = 0x462D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17F56: bpl.w $c17f6a
pub fn i_c17f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F5A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC17F6A; return 10; }
    8
}
/// $C17F5A: move.b #$32, $ae5(a5)
pub fn i_c17f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F60;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAE5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17F60: move.l #$c17f82, (a0)+
pub fn i_c17f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F66;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17F82;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17F66: bra.w $c17f76
pub fn i_c17f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F6A;
    cpu.ir = 0x6000;
    cpu.pc = 0xC17F76;
    10
}
/// $C17F6A: move.b #$a, $ae5(a5)
pub fn i_c17f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F70;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAE5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17F70: move.l #$c17fa6, (a0)+
pub fn i_c17f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F76;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17FA6;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17F76: move.l a0, $abe(a5)
pub fn i_c17f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F7A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17F7A: subq.b #$1, $ac2(a5)
pub fn i_c17f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F7E;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17F7E: movea.l (a7)+, a0
pub fn i_c17f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F80;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $C17F80: rts
pub fn i_c17f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17F82: ori.b #$ff, d1
pub fn i_c17f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F86;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17F86: move.l d2, d0
pub fn i_c17f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F88;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17F88: ori.b #$fc, d3
pub fn i_c17f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F8C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xFC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17F8C: ori.b #$55, d7
pub fn i_c17f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F90;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x55) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17F90: subq.w #$1, a0
pub fn i_c17f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F92;
    cpu.ir = 0x5348;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    8
}
/// $C17F92: move.l $2d(a1, d5.w), d0
pub fn i_c17f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F96;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x2D).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C17F96: subq.w #$1, (a4)
pub fn i_c17f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F98;
    cpu.ir = 0x5354;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17F98: ?
pub fn i_c17f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F9A;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17F9A: addq.b #$2, -(a0)
pub fn i_c17f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F9C;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C17F9C: addq.w #$2, a7
pub fn i_c17f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F9E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $C17F9E: movea.l a6, a0
pub fn i_c17f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FA0;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C17FA0: ?
pub fn i_c17fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FA2;
    cpu.ir = 0x4558;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17FA6: ori.b #$ff, d1
pub fn i_c17fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FAA;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17FAA: move.l d2, d0
pub fn i_c17faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FAC;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17FAC: ori.b #$fc, d3
pub fn i_c17fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FB0;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xFC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17FB0: ori.b #$20, d7
pub fn i_c17fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FB4;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17FB4: move.l -(a0), d0
pub fn i_c17fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FB6;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FB6: move.l -(a0), d0
pub fn i_c17fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FB8;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FB8: move.l -(a0), d0
pub fn i_c17fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FBA;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FBA: move.l -(a0), d0
pub fn i_c17fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FBC;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FBC: move.l -(a0), d0
pub fn i_c17fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FBE;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FBE: move.l -(a0), d0
pub fn i_c17fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FC0;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FC0: move.l -(a0), d0
pub fn i_c17fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FC2;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FC2: move.l -(a0), d0
pub fn i_c17fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FC4;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FC4: move.l -(a0), d0
pub fn i_c17fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FC6;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17FC6: ?
pub fn i_c17fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FC8;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17FC8: ori.b #$7a, d0
pub fn i_c17fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FCC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x7A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17FCA: pea.l $c17bee(pc)
pub fn i_c17fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FCE;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC17BEE);
    24
}
/// $C17FCC: dc.w $fc22
pub fn i_c17fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FCE;
    cpu.ir = 0xFC22;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C17FCE: move.w $9d8(a5), d0
pub fn i_c17fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FD2;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C17FD2: add.w d0, d0
pub fn i_c17fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FD4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17FD4: add.w d0, d0
pub fn i_c17fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FD6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17FD6: movea.l $c17fdc(pc, d0.w), a0
pub fn i_c17fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FDA;
    cpu.ir = 0x207B;
    let ad2 = 0xC17FDC_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C17FDA: jmp (a0)
pub fn i_c17fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FDC;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C17FDC: ?
pub fn i_c17fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FDE;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17FDE: bra.b $c18014
pub fn i_c17fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FE0;
    cpu.ir = 0x6034;
    cpu.pc = 0xC18014;
    10
}
/// $C17FE0: ?
pub fn i_c17fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FE2;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17FE2: or.w (a4), d2
pub fn i_c17fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FE4;
    cpu.ir = 0x8454;
    let v2 = bus.read16(cpu.a[4]) as u32;
    let r = (v2 | cpu.d[2]) & 0xFFFF;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $C17FE4: ?
pub fn i_c17fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FE6;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17FE6: dc.w $898a
pub fn i_c17fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FE8;
    cpu.ir = 0x898A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let r2 = (v1 | cpu.d[4]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    4
}
/// $C17FE8: ?
pub fn i_c17fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FEA;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17FEA: sub.w d2, -$3f(a0, d0.w)
pub fn i_c17fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FEE;
    cpu.ir = 0x9570;
    let ad1 = cpu.a[0].wrapping_add(0xFFFFFFC1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $C17FEE: or.l d6, d3
pub fn i_c17fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FF0;
    cpu.ir = 0x8686;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[3]) & 0xFFFFFFFF;
    cpu.d[3] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17FF0: ?
pub fn i_c17ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FF2;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17FF2: or.b d2, d2
pub fn i_c17ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FF4;
    cpu.ir = 0x8402;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17FF4: jsr $c12a0e(pc)
pub fn i_c17ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17FF8);
    cpu.pc = 0xC12A0E;
    16
}
/// $C17FF8: move.l #$e000000, $400022.l
pub fn i_c17ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18002;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18002: addq.b #$1, $ac2(a5)
pub fn i_c18002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18006;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18006: movea.l $abe(a5), a0
pub fn i_c18006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1800A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1800A: move.l #$c17e20, (a0)+
pub fn i_c1800a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18010;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18010: jsr $c17eaa(pc)
pub fn i_c18010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18014;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18014);
    cpu.pc = 0xC17EAA;
    16
}
/// $C18014: lea.l $c182ba(pc), a1
pub fn i_c18014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18018;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC182BA;
    16
}
/// $C18018: tst.b $c00401.l
pub fn i_c18018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1801E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C1801E: beq.b $c18024
pub fn i_c1801e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18020;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18024; return 10; }
    8
}
/// $C18020: lea.l $c18334(pc), a1
pub fn i_c18020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18024;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC18334;
    16
}
/// $C18024: move.l a1, (a0)+
pub fn i_c18024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18026;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18026: move.l a0, $abe(a5)
pub fn i_c18026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1802A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1802A: subq.b #$1, $ac2(a5)
pub fn i_c1802a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1802E;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1802E: move.l $d001ac.l, $900(a5)
pub fn i_c1802e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18036;
    cpu.ir = 0x2B79;
    let v2 = bus.read32(0xD001AC);
    let ad3 = cpu.a[5].wrapping_add(0x900);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C18036: clr.l $904(a5)
pub fn i_c18036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1803A;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0x904);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C1803A: jsr $c180b2(pc)
pub fn i_c1803a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1803E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1803E);
    cpu.pc = 0xC180B2;
    16
}
/// $C1803E: clr.w $ae4(a5)
pub fn i_c1803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18042;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C18042: jsr $c1727e(pc)
pub fn i_c18042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18046;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18046);
    cpu.pc = 0xC1727E;
    16
}
/// $C18046: jsr $c17eb2(pc)
pub fn i_c18046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1804A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1804A);
    cpu.pc = 0xC17EB2;
    16
}
/// $C1804A: move.b $beb(a5), d0
pub fn i_c1804a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1804E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBEB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1804E: btst.b #$6, d0
pub fn i_c1804e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18052;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C18052: beq.b $c18056
pub fn i_c18052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18054;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18056; return 10; }
    8
}
/// $C18054: rts
pub fn i_c18054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18056;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18056: btst.b #$7, d0
pub fn i_c18056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1805A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1805A: bne.w $c18106
pub fn i_c1805a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1805E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18106; return 10; }
    8
}
/// $C1805E: pea.l $c18042(pc)
pub fn i_c1805e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18062;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC18042);
    24
}
/// $C18062: move.b d0, d1
pub fn i_c18062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18064;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C18064: andi.b #$30, d0
pub fn i_c18064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18068;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18068: beq.b $c18092
pub fn i_c18068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1806A;
    cpu.ir = 0x6728;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18092; return 10; }
    8
}
/// $C1806A: moveq #$1, d1
pub fn i_c1806a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1806C;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1806C: btst.b #$4, d0
pub fn i_c1806c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18070;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C18070: bne.b $c18076
pub fn i_c18070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18072;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18076; return 10; }
    8
}
/// $C18072: move.b #$99, d1
pub fn i_c18072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18076;
    cpu.ir = 0x123C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C18076: lea.l $900(a5), a0
pub fn i_c18076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1807A;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x900);
    cpu.a[0] = ad1;
    16
}
/// $C1807A: move.w $904(a5), d0
pub fn i_c1807a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1807E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x904);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1807E: move.b (a0, d0.w), d2
pub fn i_c1807e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18082;
    cpu.ir = 0x1430;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C18082: sub.w d3, d3
pub fn i_c18082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18084;
    cpu.ir = 0x9643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18084: abcd.b d1, d2
pub fn i_c18084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18086;
    cpu.ir = 0xC501;
    let s = cpu.d[1] as u8; let d = cpu.d[2] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[2] = (cpu.d[2] & !0xFF) | r as u32;
    6
}
/// $C18086: andi.b #$f, d2
pub fn i_c18086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1808A;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1808A: move.b d2, (a0, d0.w)
pub fn i_c1808a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1808E;
    cpu.ir = 0x1182;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C1808E: bra.w $c180b2
pub fn i_c1808e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18092;
    cpu.ir = 0x6000;
    cpu.pc = 0xC180B2;
    10
}
/// $C18092: andi.b #$c, d1
pub fn i_c18092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18096;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18096: bne.b $c1809a
pub fn i_c18096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18098;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1809A; return 10; }
    8
}
/// $C18098: rts
pub fn i_c18098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1809A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1809A: moveq #$1, d0
pub fn i_c1809a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1809C;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1809C: andi.b #$8, d1
pub fn i_c1809c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180A0;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C180A0: bne.b $c180a4
pub fn i_c180a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180A2;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC180A4; return 10; }
    8
}
/// $C180A2: moveq #$ff, d0
pub fn i_c180a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180A4;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C180A4: move.w $904(a5), d1
pub fn i_c180a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180A8;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x904);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C180A8: add.w d0, d1
pub fn i_c180a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180AA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C180AA: andi.w #$3, d1
pub fn i_c180aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180AE;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C180AE: move.w d1, $904(a5)
pub fn i_c180ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180B2;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x904);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C180B2: addq.b #$1, $ac2(a5)
pub fn i_c180b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180B6;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C180B6: movea.l $abe(a5), a0
pub fn i_c180b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180BA;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C180BA: clr.l (a0)+
pub fn i_c180ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180BC;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C180BC: move.w #$720a, d6
pub fn i_c180bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180C0;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x720A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C180C0: moveq #$0, d7
pub fn i_c180c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180C2;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C180C2: lea.l $900(a5), a1
pub fn i_c180c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180C6;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x900);
    cpu.a[1] = ad1;
    16
}
/// $C180C6: move.w #$3, (a0)+
pub fn i_c180c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180CA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C180CA: move.w d6, (a0)+
pub fn i_c180ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180CC;
    cpu.ir = 0x30C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C180CC: move.w #$108, (a0)+
pub fn i_c180cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180D0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C180D0: cmp.w $904(a5), d7
pub fn i_c180d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180D4;
    cpu.ir = 0xBE6D;
    let ad2 = cpu.a[5].wrapping_add(0x904);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C180D4: bne.b $c180dc
pub fn i_c180d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180D6;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC180DC; return 10; }
    8
}
/// $C180D6: move.w #$1108, -$2(a0)
pub fn i_c180d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180DC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C180DC: move.b (a1, d7.w), d0
pub fn i_c180dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180E0;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C180E0: andi.b #$f, d0
pub fn i_c180e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180E4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C180E4: addi.b #$30, d0
pub fn i_c180e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180E8;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C180E8: move.b d0, (a0)+
pub fn i_c180e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180EA;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C180EA: move.b #$ff, (a0)+
pub fn i_c180ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180EE;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C180EE: addi.w #$20, d6
pub fn i_c180ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180F2;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C180F2: addq.w #$1, d7
pub fn i_c180f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180F4;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C180F4: cmpi.w #$4, d7
pub fn i_c180f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180F8;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x4, Size::Word, false);
    8
}
/// $C180F8: bne.b $c180c6
pub fn i_c180f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180FA;
    cpu.ir = 0x66CC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC180C6; return 10; }
    8
}
/// $C180FA: clr.w (a0)+
pub fn i_c180fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC180FC;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C180FC: move.l a0, $abe(a5)
pub fn i_c180fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18100;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18100: subq.b #$1, $ac2(a5)
pub fn i_c18100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18104;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18104: rts
pub fn i_c18104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18106;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18106: move.b $380000.l, d0
pub fn i_c18106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1810C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1810C: andi.b #$30, d0
pub fn i_c1810c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18110;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18110: bne.w $c1824c
pub fn i_c18110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18114;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1824C; return 10; }
    8
}
/// $C18114: move.b #$5, $ac4(a5)
pub fn i_c18114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1811A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1811A: move.l #$10f300, $ac8(a5)
pub fn i_c1811a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18122;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10F300;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18122: clr.w $acc(a5)
pub fn i_c18122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18126;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xACC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C18126: jsr $c1231e(pc)
pub fn i_c18126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1812A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1812A);
    cpu.pc = 0xC1231E;
    16
}
/// $C1812A: tst.b $ac6(a5)
pub fn i_c1812a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1812E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1812E: beq.w $c182a0
pub fn i_c1812e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18132;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC182A0; return 10; }
    8
}
/// $C18132: move.b $9f4(a5), d0
pub fn i_c18132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18136;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0x9F4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18136: andi.b #$f0, d0
pub fn i_c18136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1813A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1813A: beq.w $c182a0
pub fn i_c1813a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1813E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC182A0; return 10; }
    8
}
/// $C1813E: cmpi.b #$20, d0
pub fn i_c1813e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18142;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $C18142: beq.w $c18218
pub fn i_c18142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18146;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18218; return 10; }
    8
}
/// $C18146: moveq #$0, d0
pub fn i_c18146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18148;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18148: move.b d0, $3a0019.l
pub fn i_c18148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1814E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C1814E: move.b d0, $380011.l
pub fn i_c1814e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18154;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $C18154: move.b d0, $3a0005.l
pub fn i_c18154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1815A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $C1815A: move.b d0, $3a0017.l
pub fn i_c1815a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18160;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $C18160: lea.l $800000.l, a6
pub fn i_c18160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18166;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C18166: movea.l a6, a0
pub fn i_c18166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18168;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C18168: move.w #$5555, (a0)
pub fn i_c18168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1816C;
    cpu.ir = 0x30BC;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C1816C: move.w #$aaaa, $2(a0)
pub fn i_c1816c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18172;
    cpu.ir = 0x317C;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C18172: cmpi.w #$5555, (a0)
pub fn i_c18172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18176;
    cpu.ir = 0xC50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x5555, Size::Word, false);
    12
}
/// $C18176: bne.w $c182a0
pub fn i_c18176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1817A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC182A0; return 10; }
    8
}
/// $C1817A: move.w #$ffff, d6
pub fn i_c1817a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1817E;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1817E: lea.l $300001.l, a3
pub fn i_c1817e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18184;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x300001;
    20
}
/// $C18184: move.w #$5555, d0
pub fn i_c18184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18188;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18188: move.w d6, d7
pub fn i_c18188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1818A;
    cpu.ir = 0x3E06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1818A: movea.l a6, a0
pub fn i_c1818a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1818C;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1818C: move.b d0, (a3)
pub fn i_c1818c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1818E;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C1818E: move.w d0, (a0)+
pub fn i_c1818e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18190;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18190: dbra d7, $c1818c
pub fn i_c18190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18194;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC18194; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC18194; return 14; }
    cpu.pc = 0xC1818C;
    10
}
/// $C18194: move.w d6, d7
pub fn i_c18194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18196;
    cpu.ir = 0x3E06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18196: movea.l a6, a0
pub fn i_c18196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18198;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C18198: move.b d0, (a3)
pub fn i_c18198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1819A;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C1819A: move.w (a0)+, d1
pub fn i_c1819a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1819C;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C1819C: cmp.w d0, d1
pub fn i_c1819c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1819E;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C1819E: bne.w $c182a0
pub fn i_c1819e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC182A0; return 10; }
    8
}
/// $C181A2: dbra d7, $c18198
pub fn i_c181a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181A6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC181A6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC181A6; return 14; }
    cpu.pc = 0xC18198;
    10
}
/// $C181A6: move.w #$aaaa, d0
pub fn i_c181a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C181AA: move.w d6, d7
pub fn i_c181aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181AC;
    cpu.ir = 0x3E06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C181AC: movea.l a6, a0
pub fn i_c181ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181AE;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C181AE: move.b d0, (a3)
pub fn i_c181ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181B0;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C181B0: move.w d0, (a0)+
pub fn i_c181b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181B2;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C181B2: dbra d7, $c181ae
pub fn i_c181b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181B6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC181B6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC181B6; return 14; }
    cpu.pc = 0xC181AE;
    10
}
/// $C181B6: move.w d6, d7
pub fn i_c181b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181B8;
    cpu.ir = 0x3E06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C181B8: movea.l a6, a0
pub fn i_c181b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181BA;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C181BA: move.b d0, (a3)
pub fn i_c181ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181BC;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C181BC: move.w (a0)+, d1
pub fn i_c181bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181BE;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C181BE: cmp.w d0, d1
pub fn i_c181be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181C0;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C181C0: bne.w $c182a0
pub fn i_c181c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC182A0; return 10; }
    8
}
/// $C181C4: dbra d7, $c181ba
pub fn i_c181c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181C8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC181C8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC181C8; return 14; }
    cpu.pc = 0xC181BA;
    10
}
/// $C181C8: move.w #$7fff, d7
pub fn i_c181c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181CC;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C181CC: movea.l a6, a0
pub fn i_c181cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181CE;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C181CE: move.b d0, (a3)
pub fn i_c181ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181D0;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C181D0: move.l a0, d0
pub fn i_c181d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181D2;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C181D2: move.l d0, (a0)+
pub fn i_c181d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181D4;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C181D4: dbra d7, $c181ce
pub fn i_c181d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181D8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC181D8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC181D8; return 14; }
    cpu.pc = 0xC181CE;
    10
}
/// $C181D8: move.w #$7fff, d7
pub fn i_c181d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181DC;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C181DC: movea.l a6, a0
pub fn i_c181dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181DE;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C181DE: move.b d0, (a3)
pub fn i_c181de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181E0;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C181E0: move.l a0, d1
pub fn i_c181e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181E2;
    cpu.ir = 0x2208;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C181E2: move.l (a0)+, d0
pub fn i_c181e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181E4;
    cpu.ir = 0x2018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C181E4: cmp.l d0, d1
pub fn i_c181e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181E6;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $C181E6: bne.w $c182a0
pub fn i_c181e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC182A0; return 10; }
    8
}
/// $C181EA: dbra d7, $c181de
pub fn i_c181ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181EE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC181EE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC181EE; return 14; }
    cpu.pc = 0xC181DE;
    10
}
/// $C181EE: lea.l $40(a6), a0
pub fn i_c181ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181F2;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x40);
    cpu.a[0] = ad1;
    16
}
/// $C181F2: lea.l $c182b2(pc), a1
pub fn i_c181f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC182B2;
    16
}
/// $C181F6: moveq #$0, d0
pub fn i_c181f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181F8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C181F8: moveq #$7, d7
pub fn i_c181f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181FA;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C181FA: move.b (a1)+, d0
pub fn i_c181fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181FC;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C181FC: move.w d0, (a0)
pub fn i_c181fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC181FE;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C181FE: addq.l #$4, a0
pub fn i_c181fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18200;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C18200: dbra d7, $c181fa
pub fn i_c18200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18204;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC18204; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC18204; return 14; }
    cpu.pc = 0xC181FA;
    10
}
/// $C18204: move.b $c00401.l, d0
pub fn i_c18204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1820A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1820A: move.w d0, (a0)
pub fn i_c1820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1820C;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C1820C: move.w #$20, $10(a6)
pub fn i_c1820c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18212;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C18212: move.w #$0, $18(a6)
pub fn i_c18212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18218;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C18218: moveq #$0, d0
pub fn i_c18218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1821A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1821A: move.b d0, $3a0019.l
pub fn i_c1821a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18220;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C18220: move.b d0, $380011.l
pub fn i_c18220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18226;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $C18226: move.b d0, $3a0005.l
pub fn i_c18226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1822C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $C1822C: move.b d0, $3a0017.l
pub fn i_c1822c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18232;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $C18232: lea.l $800000.l, a6
pub fn i_c18232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18238;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C18238: move.l $900(a5), d0
pub fn i_c18238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1823C;
    cpu.ir = 0x202D;
    let ad2 = cpu.a[5].wrapping_add(0x900);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1823C: andi.l #$f0f0f0f, d0
pub fn i_c1823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18242;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF0F0F0F) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C18242: move.l d0, $70(a6)
pub fn i_c18242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18246;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18246: move.w #$0, $74(a6)
pub fn i_c18246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1824C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1824C: move.b #$1, $bbf(a5)
pub fn i_c1824c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18252;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18252: move.b d0, $3a001d.l
pub fn i_c18252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18258;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C18258: move.l $900(a5), d0
pub fn i_c18258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1825C;
    cpu.ir = 0x202D;
    let ad2 = cpu.a[5].wrapping_add(0x900);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1825C: andi.l #$f0f0f0f, d0
pub fn i_c1825c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18262;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF0F0F0F) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C18262: move.l d0, $d001ac.l
pub fn i_c18262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18268;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0xD001AC, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18268: move.w #$ffff, $d001b0.l
pub fn i_c18268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18270;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0xD001B0, v1 as u16);
    16
}
/// $C18270: lea.l $c1830c(pc), a1
pub fn i_c18270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1830C;
    16
}
/// $C18274: tst.b $c00401.l
pub fn i_c18274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1827A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C1827A: beq.b $c18280
pub fn i_c1827a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1827C;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18280; return 10; }
    8
}
/// $C1827C: lea.l $c183c2(pc), a1
pub fn i_c1827c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18280;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC183C2;
    16
}
/// $C18280: addq.b #$1, $ac2(a5)
pub fn i_c18280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18284;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18284: movea.l $abe(a5), a0
pub fn i_c18284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18288;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18288: move.l a1, (a0)+
pub fn i_c18288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1828A;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1828A: move.l a0, $abe(a5)
pub fn i_c1828a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1828E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1828E: subq.b #$1, $ac2(a5)
pub fn i_c1828e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18292;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18292: move.w #$b4, d7
pub fn i_c18292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18296;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18296: jsr $c1727e(pc)
pub fn i_c18296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1829A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1829A);
    cpu.pc = 0xC1727E;
    16
}
/// $C1829A: dbra d7, $c18296
pub fn i_c1829a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1829E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1829E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1829E; return 14; }
    cpu.pc = 0xC18296;
    10
}
/// $C1829E: rts
pub fn i_c1829e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C182A0: lea.l $c1831e(pc), a1
pub fn i_c182a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1831E;
    16
}
/// $C182A4: tst.b $c00401.l
pub fn i_c182a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182AA;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C182AA: beq.b $c18280
pub fn i_c182aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182AC;
    cpu.ir = 0x67D4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18280; return 10; }
    8
}
/// $C182AC: lea.l $c183e4(pc), a1
pub fn i_c182ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC183E4;
    16
}
/// $C182B0: bra.b $c18280
pub fn i_c182b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182B2;
    cpu.ir = 0x60CE;
    cpu.pc = 0xC18280;
    10
}
/// $C182B2: trap #$5
pub fn i_c182b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182B4;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C182B4: dc.w $4f2d
pub fn i_c182b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182B6;
    cpu.ir = 0x4F2D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C182B6: ?
pub fn i_c182b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182B8;
    cpu.ir = 0x4745;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C182B8: chk.w d0, d7
pub fn i_c182b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182BA;
    cpu.ir = 0x4F80;
    let v1 = cpu.d[0] & 0xFFFF;
    let bd = v1 as u16 as i16 as i32; let vv = cpu.d[7] as u16 as i16 as i32;
    cpu.sr.set(StatusRegister::Z, false); cpu.sr.set(StatusRegister::V, false); cpu.sr.set(StatusRegister::C, false);
    if vv < 0 { cpu.sr.set(StatusRegister::N, true); cpu.enter_exception(bus, Exception::Chk); return 40; }
    if vv > bd { cpu.sr.set(StatusRegister::N, false); cpu.enter_exception(bus, Exception::Chk); return 40; }
    cpu.sr.set(StatusRegister::N, false);
    10
}
/// $C182BA: ori.b #$e7, d3
pub fn i_c182ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182BE;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xE7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C182BE: movep.w -$3907(a1), d0
pub fn i_c182be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182C2;
    cpu.ir = 0x109;
    let a = cpu.a[1].wrapping_add(0xFFFFC6F9);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C182C2: dc.w $ebf9
pub fn i_c182c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182C8;
    cpu.ir = 0xEBF9;
    let v1 = bus.read16(0x6EFF0003) as u32;
    let r2 = do_shift_rotate(cpu, v1, 1, Size::Word, 1, true);
    bus.write16(0x6EFF0003, r2 as u16);
    20
}
/// $C182C8: moveq #$87, d1
pub fn i_c182c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182CA;
    cpu.ir = 0x7287;
    cpu.d[1] = 0xFFFFFF87;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C182CA: movep.w $1617(a0), d1
pub fn i_c182ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182CE;
    cpu.ir = 0x308;
    let a = cpu.a[0].wrapping_add(0x1617);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C182CE: move.b (a1)+, d4
pub fn i_c182ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182D0;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C182D0: dc.w $ffff
pub fn i_c182d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182D2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C182D2: ori.b #$ee, d3
pub fn i_c182d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182D6;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xEE) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C182D6: movep.w $4473(a1), d0
pub fn i_c182d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182DA;
    cpu.ir = 0x109;
    let a = cpu.a[1].wrapping_add(0x4473);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C182DA: muls.w $3dcd(a5), d7
pub fn i_c182da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182DE;
    cpu.ir = 0xCFED;
    let ad2 = cpu.a[5].wrapping_add(0x3DCD);
    let v3 = bus.read16(ad2) as u32;
    let r = ((v3 as u16 as i16 as i32).wrapping_mul(cpu.d[7] as u16 as i16 as i32)) as u32;
    cpu.d[7] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    78
}
/// $C182DE: dc.w $f6d3
pub fn i_c182de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182E0;
    cpu.ir = 0xF6D3;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C182E0: dc.w $ffff
pub fn i_c182e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182E2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C182E2: ori.b #$d2, d3
pub fn i_c182e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182E6;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xD2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C182E6: movep.w -$1e1e(a1), d0
pub fn i_c182e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182EA;
    cpu.ir = 0x109;
    let a = cpu.a[1].wrapping_add(0xFFFFE1E2);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C182EA: rol.w $c5f96e00.l
pub fn i_c182ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182F0;
    cpu.ir = 0xE7F9;
    let v1 = bus.read16(0xC5F96E00) as u32;
    let r2 = do_shift_rotate(cpu, v1, 1, Size::Word, 3, true);
    bus.write16(0xC5F96E00, r2 as u16);
    20
}
/// $C182F0: or.l -$6c7a(a0), d0
pub fn i_c182f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182F4;
    cpu.ir = 0x80A8;
    let ad2 = cpu.a[0].wrapping_add(0xFFFF9386);
    let v3 = bus.read32(ad2);
    let r = (v3 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C182F4: suba.l $c182f5(pc), a4
pub fn i_c182f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182F8;
    cpu.ir = 0x99FA;
    let v2 = bus.read32(0xC182F5);
    cpu.a[4] = cpu.a[4].wrapping_sub(v2);
    20
}
/// $C182F8: ori.b #$2, d5
pub fn i_c182f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC182FC;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C182FC: movep.w -$3a07(a1), d0
pub fn i_c182fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18300;
    cpu.ir = 0x109;
    let a = cpu.a[1].wrapping_add(0xFFFFC5F9);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18402: move.b $300001.l, d0
pub fn i_c18402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18408;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C18408: andi.b #$1, d0
pub fn i_c18408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1840C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1840C: bne.w $c18436
pub fn i_c1840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18410;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18436; return 10; }
    8
}
/// $C18410: jsr $c12a0e(pc)
pub fn i_c18410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18414;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18414);
    cpu.pc = 0xC12A0E;
    16
}
/// $C18414: addq.b #$1, $ac2(a5)
pub fn i_c18414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18418;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18418: movea.l $abe(a5), a0
pub fn i_c18418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1841C;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1841C: move.l #$c1843c, (a0)+
pub fn i_c1841c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18422;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1843C;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18422: move.l a0, $abe(a5)
pub fn i_c18422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18426;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18426: subq.b #$1, $ac2(a5)
pub fn i_c18426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1842A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1842A: move.b $300001.l, d0
pub fn i_c1842a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18430;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C18430: andi.b #$1, d0
pub fn i_c18430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18434;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18434: beq.b $c1842a
pub fn i_c18434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18436;
    cpu.ir = 0x67F4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1842A; return 10; }
    8
}
/// $C18436: jmp $c10002.l
pub fn i_c18436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1843C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C1843C: ori.b #$4c, d3
pub fn i_c1843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18440;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18440: movep.w $4449(a0), d0
pub fn i_c18440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18444;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4449);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18444: addq.b #$8, $5357(a5)
pub fn i_c18444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18448;
    cpu.ir = 0x502D;
    let ad1 = cpu.a[5].wrapping_add(0x5357);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18448: move.l $4f(a1, d2.w), d0
pub fn i_c18448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1844C;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x4F).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C1844C: not.w d6
pub fn i_c1844c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1844E;
    cpu.ir = 0x4646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1844E: move.l -(a1), d0
pub fn i_c1844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18450;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C18450: ?
pub fn i_c18450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18454;
    cpu.ir = 0x21FF;
    let v1: u32 = 0x0;
    bus.write32(0x0, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18454: jsr $c12a0e(pc)
pub fn i_c18454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18458;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18458);
    cpu.pc = 0xC12A0E;
    16
}
/// $C18458: addq.b #$1, $ac2(a5)
pub fn i_c18458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1845C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1845C: movea.l $abe(a5), a0
pub fn i_c1845c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18460;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18460: move.l #$c17e20, (a0)+
pub fn i_c18460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18466;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18466: jsr $c17eaa(pc)
pub fn i_c18466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1846A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1846A);
    cpu.pc = 0xC17EAA;
    16
}
/// $C1846A: move.l #$c185fc, (a0)+
pub fn i_c1846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18470;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC185FC;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18470: move.l a0, $abe(a5)
pub fn i_c18470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18474;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18474: subq.b #$1, $ac2(a5)
pub fn i_c18474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18478;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18478: move.b $300001.l, d0
pub fn i_c18478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1847E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1847E: not.b d0
pub fn i_c1847e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18480;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C18480: move.b d0, $9de(a5)
pub fn i_c18480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18484;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18484: jsr $c184b4(pc)
pub fn i_c18484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18488;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18488);
    cpu.pc = 0xC184B4;
    16
}
/// $C18488: clr.w $ae4(a5)
pub fn i_c18488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1848C;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1848C: jsr $c1727e(pc)
pub fn i_c1848c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18490;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18490);
    cpu.pc = 0xC1727E;
    16
}
/// $C18490: jsr $c17eb2(pc)
pub fn i_c18490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18494;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18494);
    cpu.pc = 0xC17EB2;
    16
}
/// $C18494: btst.b #$6, $a97(a5)
pub fn i_c18494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1849A;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C1849A: beq.b $c1849e
pub fn i_c1849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1849C;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1849E; return 10; }
    8
}
/// $C1849C: rts
pub fn i_c1849c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1849E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1849E: move.b $300001.l, d0
pub fn i_c1849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184A4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C184A4: not.b d0
pub fn i_c184a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184A6;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C184A6: cmp.b $9de(a5), d0
pub fn i_c184a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184AA;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C184AA: beq.b $c1848c
pub fn i_c184aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184AC;
    cpu.ir = 0x67E0;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1848C; return 10; }
    8
}
/// $C184AC: move.b d0, $9de(a5)
pub fn i_c184ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184B0;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C184B0: pea.l $c1848c(pc)
pub fn i_c184b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184B4;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC1848C);
    24
}
/// $C184B4: addq.b #$1, $ac2(a5)
pub fn i_c184b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184B8;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C184B8: movea.l $abe(a5), a0
pub fn i_c184b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184BC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C184BC: clr.l (a0)+
pub fn i_c184bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184BE;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C184BE: move.b $9de(a5), d7
pub fn i_c184be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184C2;
    cpu.ir = 0x1E2D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C184C2: move.w #$700c, d6
pub fn i_c184c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184C6;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x700C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C184C6: lea.l $c18534(pc), a1
pub fn i_c184c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC18534;
    16
}
/// $C184CA: move.w (a1)+, d0
pub fn i_c184ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184CC;
    cpu.ir = 0x3019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C184CC: beq.w $c18528
pub fn i_c184cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184D0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18528; return 10; }
    8
}
/// $C184D0: move.w d0, d1
pub fn i_c184d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184D2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C184D2: and.b d7, d1
pub fn i_c184d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184D4;
    cpu.ir = 0xC207;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C184D4: move.w #$3, (a0)+
pub fn i_c184d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184D8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C184D8: move.w d6, d2
pub fn i_c184d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184DA;
    cpu.ir = 0x3406;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C184DA: addi.w #$240, d2
pub fn i_c184da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184DE;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x240, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C184DE: move.w d2, (a0)+
pub fn i_c184de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184E0;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C184E0: move.w #$308, (a0)+
pub fn i_c184e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184E4;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x308;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C184E4: moveq #$7, d5
pub fn i_c184e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184E6;
    cpu.ir = 0x7A07;
    cpu.d[5] = 0x7;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C184E6: lsr.b #$1, d0
pub fn i_c184e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184E8;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C184E8: bcc.b $c184f8
pub fn i_c184e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184EA;
    cpu.ir = 0x640E;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC184F8; return 10; }
    8
}
/// $C184EA: move.b d1, d2
pub fn i_c184ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184EC;
    cpu.ir = 0x1401;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C184EC: andi.b #$1, d2
pub fn i_c184ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184F0;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C184F0: addi.b #$30, d2
pub fn i_c184f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184F4;
    cpu.ir = 0x602;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C184F4: bra.w $c184fc
pub fn i_c184f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184F8;
    cpu.ir = 0x6000;
    cpu.pc = 0xC184FC;
    10
}
/// $C184F8: move.b #$2e, d2
pub fn i_c184f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184FC;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C184FC: move.b d2, (a0)+
pub fn i_c184fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC184FE;
    cpu.ir = 0x10C2;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C184FE: ror.b #$1, d1
pub fn i_c184fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18500;
    cpu.ir = 0xE219;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 3, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $C18500: dbra d5, $c184e6
pub fn i_c18500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18504;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC18504; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC18504; return 14; }
    cpu.pc = 0xC184E6;
    10
}
/// $C18504: move.w #$ff00, (a0)+
pub fn i_c18504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18508;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18508: move.w #$3, (a0)+
pub fn i_c18508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1850C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1850C: move.w d6, d2
pub fn i_c1850c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1850E;
    cpu.ir = 0x3406;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1850E: addi.w #$380, d2
pub fn i_c1850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18512;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x380, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C18512: move.w d2, (a0)+
pub fn i_c18512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18514;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18514: move.w #$a, (a0)+
pub fn i_c18514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18518;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18518: move.w (a1)+, d0
pub fn i_c18518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1851A;
    cpu.ir = 0x3019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C1851A: bmi.b $c184ca
pub fn i_c1851a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1851C;
    cpu.ir = 0x6BAE;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC184CA; return 10; }
    8
}
/// $C1851C: cmp.b d0, d1
pub fn i_c1851c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1851E;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C1851E: bne.b $c18524
pub fn i_c1851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18520;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18524; return 10; }
    8
}
/// $C18520: addq.w #$2, d6
pub fn i_c18520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18522;
    cpu.ir = 0x5446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C18522: move.l (a1), (a0)+
pub fn i_c18522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18524;
    cpu.ir = 0x20D1;
    let v2 = bus.read32(cpu.a[1]);
    let ad3 = cpu.a[0]; cpu.a[0] = ad3.wrapping_add(4);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18524: addq.l #$4, a1
pub fn i_c18524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18526;
    cpu.ir = 0x5889;
    cpu.a[1] = cpu.a[1].wrapping_add(4);
    8
}
/// $C18526: bra.b $c18518
pub fn i_c18526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18528;
    cpu.ir = 0x60F0;
    cpu.pc = 0xC18518;
    10
}
/// $C18528: clr.w (a0)+
pub fn i_c18528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1852A;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1852A: move.l a0, $abe(a5)
pub fn i_c1852a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1852E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1852E: subq.b #$1, $ac2(a5)
pub fn i_c1852e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18532;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18532: rts
pub fn i_c18532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18534;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18534: ori.b #$0, d1
pub fn i_c18534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18538;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18538: ?
pub fn i_c18538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1853A;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1853A: or.l d2, (a2)+
pub fn i_c1853a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1853C;
    cpu.ir = 0x859A;
    let ad1 = cpu.a[2]; cpu.a[2] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    12
}
/// $C1853C: ori.b #$c1, d1
pub fn i_c1853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18540;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18540: or.l d2, -(a2)
pub fn i_c18540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18542;
    cpu.ir = 0x85A2;
    let ad1 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad1;
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    14
}
/// $C18542: dc.w $ffff
pub fn i_c18542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18544;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C18544: ori.b #$0, $c1.w
pub fn i_c18544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1854A;
    cpu.ir = 0x38;
    let v1 = bus.read8(0xC1) as u32;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0xC1, r2 as u8);
    16
}
/// $C1854A: or.l d2, $8(a2)
pub fn i_c1854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1854E;
    cpu.ir = 0x85AA;
    let ad1 = cpu.a[2].wrapping_add(0x8);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    16
}
/// $C1854E: ?
pub fn i_c1854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18550;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18550: or.l d2, $10(a2)
pub fn i_c18550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18554;
    cpu.ir = 0x85AA;
    let ad1 = cpu.a[2].wrapping_add(0x10);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    16
}
/// $C18554: ?
pub fn i_c18554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18556;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18556: or.l d2, $18(a2)
pub fn i_c18556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1855A;
    cpu.ir = 0x85AA;
    let ad1 = cpu.a[2].wrapping_add(0x18);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    16
}
/// $C1855A: ?
pub fn i_c1855a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1855C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1855C: or.l d2, $20(a2)
pub fn i_c1855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18560;
    cpu.ir = 0x85AA;
    let ad1 = cpu.a[2].wrapping_add(0x20);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    16
}
/// $C18560: ?
pub fn i_c18560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18562;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18562: or.l d2, $28.w
pub fn i_c18562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18566;
    cpu.ir = 0x85B8;
    let v1 = bus.read32(0x28);
    let r2 = (v1 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(0x28, r2);
    16
}
/// $C18566: ?
pub fn i_c18566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18568;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18568: divs.w d6, d2
pub fn i_c18568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1856A;
    cpu.ir = 0x85C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[2] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 158; }
    cpu.d[2] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    158
}
/// $C1856A: ori.b #$c1, (invalid.w)
pub fn i_c1856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18570;
    cpu.ir = 0x30;
    let ad1 = cpu.a[0].wrapping_add(0xFFFFFFD4).wrapping_add((cpu.a[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $C18570: ori.b #$c1, $85e2.w
pub fn i_c18570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18576;
    cpu.ir = 0x38;
    let v1 = bus.read8(0xFFFF85E2) as u32;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0xFFFF85E2, r2 as u8);
    16
}
/// $C18576: dc.w $ffff
pub fn i_c18576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18578;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C18578: ori.w #$0, d0
pub fn i_c18578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1857C;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1857C: ?
pub fn i_c1857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1857E;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1857E: or.l d2, (a2)+
pub fn i_c1857e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18580;
    cpu.ir = 0x859A;
    let ad1 = cpu.a[2]; cpu.a[2] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    12
}
/// $C18580: ori.w #$c1, d0
pub fn i_c18580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18584;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0xC1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C18584: or.l d2, -(a2)
pub fn i_c18584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18586;
    cpu.ir = 0x85A2;
    let ad1 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad1;
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    14
}
/// $C18586: dc.w $ffff
pub fn i_c18586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18588;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C18588: ori.l #$c1, d0
pub fn i_c18588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1858E;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0xC1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C1858E: or.l d2, (a2)+
pub fn i_c1858e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18590;
    cpu.ir = 0x859A;
    let ad1 = cpu.a[2]; cpu.a[2] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    let r3 = (v2 | cpu.d[2]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r3, Size::Long); cpu.sr.0 &= !0x3;
    bus.write32(ad1, r3);
    12
}
/// $C18590: ori.l #$c185a2, d0
pub fn i_c18590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18596;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0xC185A2) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C18596: dc.w $ffff
pub fn i_c18596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18598;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C18598: ori.b #$8, d0
pub fn i_c18598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1859C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1859A: movep.w $4f46(a0), d0
pub fn i_c1859a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1859E;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F46);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1859C: ?
pub fn i_c1859c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1859E;
    cpu.ir = 0x4F46;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1859E: ?
pub fn i_c1859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185A0;
    cpu.ir = 0x46FF;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x0;
    write_sr(cpu, v1 as u16);
    12
}
/// $C185A0: ?
pub fn i_c185a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185A4;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185A2: movep.w $4f4e(a0), d0
pub fn i_c185a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185A6;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F4E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C185A4: ?
pub fn i_c185a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185A6;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C185A6: ?
pub fn i_c185a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185A8;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C185A8: ?
pub fn i_c185a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185AC;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185AA: movep.w $4f46(a0), d0
pub fn i_c185aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185AE;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F46);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C185AC: ?
pub fn i_c185ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185AE;
    cpu.ir = 0x4F46;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C185AE: not.b -(a0)
pub fn i_c185ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185B0;
    cpu.ir = 0x4620;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    10
}
/// $C185B0: move.l -(a0), d0
pub fn i_c185b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185B2;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C185B2: move.l -(a0), d0
pub fn i_c185b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185B4;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C185B4: ?
pub fn i_c185b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185B6;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C185B6: ?
pub fn i_c185b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185BA;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185B8: movep.w $4f4e(a0), d0
pub fn i_c185b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185BC;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F4E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C185BA: ?
pub fn i_c185ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185BC;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C185BC: movea.l (a3)+, a0
pub fn i_c185bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185BE;
    cpu.ir = 0x205B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $C185BE: move usp, a7
pub fn i_c185be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185C0;
    cpu.ir = 0x4E6F;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[7] = cpu.usp;
    4
}
/// $C185C0: move.l ([$b0108]), d0
pub fn i_c185c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185C4;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add(cpu.d[5]);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C185C4: ?
pub fn i_c185c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185C8;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185C6: movep.w $4f4e(a0), d0
pub fn i_c185c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185CA;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F4E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C185C8: ?
pub fn i_c185c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185CA;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C185CA: movea.l (a3)+, a0
pub fn i_c185ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185CC;
    cpu.ir = 0x205B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $C185CC: move usp, a7
pub fn i_c185cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185CE;
    cpu.ir = 0x4E6F;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[7] = cpu.usp;
    4
}
/// $C185CE: move.l ([$b0108]), d0
pub fn i_c185ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185D2;
    cpu.ir = 0x2032;
    let ad2 = cpu.a[2].wrapping_add(0xFFFFFFFF).wrapping_add(cpu.d[5]);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C185D2: ?
pub fn i_c185d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185D6;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185D4: movep.w $4f4e(a0), d0
pub fn i_c185d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185D8;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F4E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C185D6: ?
pub fn i_c185d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185D8;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C185D8: movea.l (a3)+, a0
pub fn i_c185d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185DA;
    cpu.ir = 0x205B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $C185DA: move usp, a7
pub fn i_c185da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185DC;
    cpu.ir = 0x4E6F;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[7] = cpu.usp;
    4
}
/// $C185DC: move.l ([$b0108]), d0
pub fn i_c185dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185E0;
    cpu.ir = 0x2033;
    let ad2 = cpu.a[3].wrapping_add(0xFFFFFFFF).wrapping_add(cpu.d[5]);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C185E0: ?
pub fn i_c185e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185E4;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185E2: movep.w $4f4e(a0), d0
pub fn i_c185e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185E6;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F4E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C185E4: ?
pub fn i_c185e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185E6;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C185E6: movea.l (a3)+, a0
pub fn i_c185e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185E8;
    cpu.ir = 0x205B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $C185E8: move usp, a7
pub fn i_c185e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185EA;
    cpu.ir = 0x4E6F;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[7] = cpu.usp;
    4
}
/// $C185EA: move.l ([$b0108]), d0
pub fn i_c185ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185EE;
    cpu.ir = 0x2034;
    let ad2 = cpu.a[4].wrapping_add(0xFFFFFFFF).wrapping_add(cpu.d[5]);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C185EE: ?
pub fn i_c185ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185F2;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185F2: ?
pub fn i_c185f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185F4;
    cpu.ir = 0x32FF;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C185F4: ?
pub fn i_c185f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185F8;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185F8: ?
pub fn i_c185f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185FA;
    cpu.ir = 0x34FF;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C185FA: ?
pub fn i_c185fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC185FE;
    cpu.ir = 0xB;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C185FC: ori.b #$45, d3
pub fn i_c185fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18600;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x45) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18600: movep.w $5345(a0), d0
pub fn i_c18600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18604;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18604: addq.w #$2, (a4)
pub fn i_c18604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18606;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18606: ?
pub fn i_c18606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18608;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18608: dc.w $4720
pub fn i_c18608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1860A;
    cpu.ir = 0x4720;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1860A: subq.w #$2, (a0)
pub fn i_c1860a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1860C;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1860C: movea.l (a4), a0
pub fn i_c1860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1860E;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C1860E: swap d5
pub fn i_c1860e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18610;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18610: movea.l a0, a0
pub fn i_c18610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18612;
    cpu.ir = 0x2048;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C18612: ?
pub fn i_c18612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18614;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18614: neg.b -(a0)
pub fn i_c18614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18616;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C18616: ?
pub fn i_c18616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18618;
    cpu.ir = 0x4449;
    let v1 = cpu.a[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C18686: jsr $c12a0e(pc)
pub fn i_c18686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1868A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1868A);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1868A: move.l #$e000000, $400022.l
pub fn i_c1868a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18694;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18694: jsr $c11c42(pc)
pub fn i_c18694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18698;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18698);
    cpu.pc = 0xC11C42;
    16
}
/// $C18698: clr.w $ae4(a5)
pub fn i_c18698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1869C;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1869C: moveq #$7, d6
pub fn i_c1869c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1869E;
    cpu.ir = 0x7C07;
    cpu.d[6] = 0x7;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1869E: addq.b #$1, $ac2(a5)
pub fn i_c1869e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186A2;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C186A2: movea.l $abe(a5), a0
pub fn i_c186a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186A6;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C186A6: move.l #$c17e20, (a0)+
pub fn i_c186a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186AC;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C186AC: jsr $c17eaa(pc)
pub fn i_c186ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC186B0);
    cpu.pc = 0xC17EAA;
    16
}
/// $C186B0: move.l #$c188d6, (a0)+
pub fn i_c186b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186B6;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC188D6;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C186B6: clr.l (a0)+
pub fn i_c186b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186B8;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C186B8: move.w #$720a, d2
pub fn i_c186b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186BC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x720A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C186BC: move.b $ad2(a5), d0
pub fn i_c186bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186C0;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C186C0: jsr $c168be(pc)
pub fn i_c186c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC186C4);
    cpu.pc = 0xC168BE;
    16
}
/// $C186C4: move.w #$714a, d2
pub fn i_c186c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186C8;
    cpu.ir = 0x343C;
    let v1: u32 = 0x714A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C186C8: move.b $ad3(a5), d0
pub fn i_c186c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186CC;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C186CC: jsr $c168be(pc)
pub fn i_c186cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC186D0);
    cpu.pc = 0xC168BE;
    16
}
/// $C186D0: move.w #$71aa, d2
pub fn i_c186d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186D4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C186D4: move.b $ad4(a5), d0
pub fn i_c186d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186D8;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C186D8: jsr $c168be(pc)
pub fn i_c186d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC186DC);
    cpu.pc = 0xC168BE;
    16
}
/// $C186DC: move.w #$714d, d2
pub fn i_c186dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186E0;
    cpu.ir = 0x343C;
    let v1: u32 = 0x714D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C186E0: move.b $ad6(a5), d0
pub fn i_c186e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186E4;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C186E4: jsr $c168be(pc)
pub fn i_c186e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC186E8);
    cpu.pc = 0xC168BE;
    16
}
/// $C186E8: move.w #$71ad, d2
pub fn i_c186e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186EC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71AD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C186EC: move.b $ad7(a5), d0
pub fn i_c186ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186F0;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C186F0: jsr $c168be(pc)
pub fn i_c186f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC186F4);
    cpu.pc = 0xC168BE;
    16
}
/// $C186F4: move.w #$720d, d2
pub fn i_c186f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186F8;
    cpu.ir = 0x343C;
    let v1: u32 = 0x720D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C186F8: move.b $ad8(a5), d0
pub fn i_c186f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC186FC;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C186FC: jsr $c168be(pc)
pub fn i_c186fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18700;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18700);
    cpu.pc = 0xC168BE;
    16
}
/// $C18700: move.w #$3, (a0)+
pub fn i_c18700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18704;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18704: move.w #$728a, (a0)+
pub fn i_c18704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18708;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x728A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18708: jsr $c17e7a(pc)
pub fn i_c18708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1870C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1870C);
    cpu.pc = 0xC17E7A;
    16
}
/// $C1870C: clr.w (a0)+
pub fn i_c1870c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1870E;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1870E: move.l a0, $abe(a5)
pub fn i_c1870e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18712;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18712: subq.b #$1, $ac2(a5)
pub fn i_c18712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18716;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18716: jsr $c18876(pc)
pub fn i_c18716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1871A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1871A);
    cpu.pc = 0xC18876;
    16
}
/// $C1871A: clr.w $9da(a5)
pub fn i_c1871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1871E;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x9DA);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1871E: bra.w $c187c0
pub fn i_c1871e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18722;
    cpu.ir = 0x6000;
    cpu.pc = 0xC187C0;
    10
}
/// $C1872E: movem.l d0-d7/a0-a6, -(a7)
pub fn i_c1872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18732;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C18732: move.l $ad2(a5), $adc(a5)
pub fn i_c18732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18738;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xADC);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C18738: move.l $ad6(a5), $ae0(a5)
pub fn i_c18738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1873E;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xAE0);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C1873E: jsr $c11c42.l
pub fn i_c1873e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18744;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC18744);
    cpu.pc = 0xC11C42;
    16
}
/// $C18744: moveq #$7, d6
pub fn i_c18744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18746;
    cpu.ir = 0x7C07;
    cpu.d[6] = 0x7;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18746: addq.b #$1, $ac2(a5)
pub fn i_c18746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1874A;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1874A: movea.l $abe(a5), a0
pub fn i_c1874a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1874E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1874E: clr.l (a0)+
pub fn i_c1874e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18750;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C18750: move.w #$720a, d2
pub fn i_c18750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18754;
    cpu.ir = 0x343C;
    let v1: u32 = 0x720A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18754: move.b $ad2(a5), d0
pub fn i_c18754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18758;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18758: bsr.w $c168be
pub fn i_c18758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1875C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1875C);
    cpu.pc = 0xC168BE;
    18
}
/// $C1875C: move.w #$714a, d2
pub fn i_c1875c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18760;
    cpu.ir = 0x343C;
    let v1: u32 = 0x714A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18760: move.b $ad3(a5), d0
pub fn i_c18760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18764;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18764: bsr.w $c168be
pub fn i_c18764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18768;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC18768);
    cpu.pc = 0xC168BE;
    18
}
/// $C18768: move.w #$71aa, d2
pub fn i_c18768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1876C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1876C: move.b $ad4(a5), d0
pub fn i_c1876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18770;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18770: bsr.w $c168be
pub fn i_c18770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18774;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC18774);
    cpu.pc = 0xC168BE;
    18
}
/// $C18774: move.w #$714d, d2
pub fn i_c18774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18778;
    cpu.ir = 0x343C;
    let v1: u32 = 0x714D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18778: move.b $ad6(a5), d0
pub fn i_c18778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1877C;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1877C: bsr.w $c168be
pub fn i_c1877c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18780;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC18780);
    cpu.pc = 0xC168BE;
    18
}
/// $C18780: move.w #$71ad, d2
pub fn i_c18780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18784;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71AD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18784: move.b $ad7(a5), d0
pub fn i_c18784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18788;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18788: bsr.w $c168be
pub fn i_c18788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1878C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1878C);
    cpu.pc = 0xC168BE;
    18
}
/// $C1878C: move.w #$720d, d2
pub fn i_c1878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18790;
    cpu.ir = 0x343C;
    let v1: u32 = 0x720D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18790: move.b $ad8(a5), d0
pub fn i_c18790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18794;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18794: bsr.w $c168be
pub fn i_c18794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18798;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC18798);
    cpu.pc = 0xC168BE;
    18
}
/// $C18798: move.w #$3, (a0)+
pub fn i_c18798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1879C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1879C: move.w #$728a, (a0)+
pub fn i_c1879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187A0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x728A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C187A0: bsr.w $c17e7a
pub fn i_c187a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187A4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC187A4);
    cpu.pc = 0xC17E7A;
    18
}
/// $C187A4: clr.w (a0)+
pub fn i_c187a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187A6;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C187A6: move.l a0, $abe(a5)
pub fn i_c187a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187AA;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C187AA: subq.b #$1, $ac2(a5)
pub fn i_c187aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187AE;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C187AE: move.l $adc(a5), $ad2(a5)
pub fn i_c187ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187B4;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xADC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xAD2);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C187B4: move.l $ae0(a5), $ad6(a5)
pub fn i_c187b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187BA;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAE0);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xAD6);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C187BA: movem.l (a7)+, d0-d7/a0-a6
pub fn i_c187ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187BE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C187BE: rts
pub fn i_c187be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C187C0: jsr $c1727e(pc)
pub fn i_c187c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC187C4);
    cpu.pc = 0xC1727E;
    16
}
/// $C187C4: jsr $c17eb2(pc)
pub fn i_c187c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC187C8);
    cpu.pc = 0xC17EB2;
    16
}
/// $C187C8: btst.b #$6, $a97(a5)
pub fn i_c187c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187CE;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C187CE: beq.b $c187d2
pub fn i_c187ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187D0;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC187D2; return 10; }
    8
}
/// $C187D0: rts
pub fn i_c187d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C187D2: btst.b #$7, $a97(a5)
pub fn i_c187d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187D8;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C187D8: bne.w $c18868
pub fn i_c187d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18868; return 10; }
    8
}
/// $C187DC: pea.l $c187c0(pc)
pub fn i_c187dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187E0;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC187C0);
    24
}
/// $C187E0: jsr $c18876(pc)
pub fn i_c187e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC187E4);
    cpu.pc = 0xC18876;
    16
}
/// $C187E4: move.b $a97(a5), d0
pub fn i_c187e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187E8;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C187E8: andi.b #$c, d0
pub fn i_c187e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187EC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C187EC: beq.w $c18816
pub fn i_c187ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18816; return 10; }
    8
}
/// $C187F0: moveq #$1, d1
pub fn i_c187f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187F2;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C187F2: andi.b #$8, d0
pub fn i_c187f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187F6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C187F6: bne.b $c187fa
pub fn i_c187f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187F8;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC187FA; return 10; }
    8
}
/// $C187F8: moveq #$ff, d1
pub fn i_c187f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187FA;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C187FA: move.w $9da(a5), d0
pub fn i_c187fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC187FE;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C187FE: add.b d1, d0
pub fn i_c187fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18800;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C18800: cmpi.b #$6, d0
pub fn i_c18800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18804;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $C18804: bcc.b $c187fe
pub fn i_c18804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18806;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC187FE; return 10; }
    8
}
/// $C18806: move.w d0, $9da(a5)
pub fn i_c18806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1880A;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1880A: bsr.w $c1872e
pub fn i_c1880a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1880E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1880E);
    cpu.pc = 0xC1872E;
    18
}
/// $C1880E: rts
pub fn i_c1880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18810;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18810: btst.l d0, d2
pub fn i_c18810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18812;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C18812: ori.b #$6, d4
pub fn i_c18812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18816;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18816: move.b $a98(a5), d0
pub fn i_c18816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1881A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1881A: andi.b #$30, d0
pub fn i_c1881a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1881E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1881E: beq.b $c1880a
pub fn i_c1881e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18820;
    cpu.ir = 0x67EA;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1880A; return 10; }
    8
}
/// $C18820: moveq #$1, d1
pub fn i_c18820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18822;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18822: andi.b #$10, d0
pub fn i_c18822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18826;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18826: bne.b $c1882c
pub fn i_c18826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18828;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1882C; return 10; }
    8
}
/// $C18828: move.b #$99, d1
pub fn i_c18828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1882C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1882C: moveq #$0, d2
pub fn i_c1882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1882E;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1882E: move.w $9da(a5), d0
pub fn i_c1882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18832;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C18832: lea.l $ad2(a5), a1
pub fn i_c18832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18836;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xAD2);
    cpu.a[1] = ad1;
    16
}
/// $C18836: move.b $c18810(pc, d0.w), d2
pub fn i_c18836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1883A;
    cpu.ir = 0x143B;
    let ad2 = 0xC18810_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C1883A: move.b (a1, d2.w), d3
pub fn i_c1883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1883E;
    cpu.ir = 0x1631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C1883E: sub.b d4, d4
pub fn i_c1883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18840;
    cpu.ir = 0x9804;
    let v1 = cpu.d[4] & 0xFF;
    let h1 = cpu.d[4] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    4
}
/// $C18840: abcd.b d1, d3
pub fn i_c18840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18842;
    cpu.ir = 0xC701;
    let s = cpu.d[1] as u8; let d = cpu.d[3] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[3] = (cpu.d[3] & !0xFF) | r as u32;
    6
}
/// $C18842: tst.b d2
pub fn i_c18842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18844;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C18844: beq.b $c18852
pub fn i_c18844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18846;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18852; return 10; }
    8
}
/// $C18846: cmp.b $c18858(pc, d2.w), d3
pub fn i_c18846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1884A;
    cpu.ir = 0xB63B;
    let ad2 = 0xC18858_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $C1884A: bhi.b $c1883e
pub fn i_c1884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1884C;
    cpu.ir = 0x62F2;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC1883E; return 10; }
    8
}
/// $C1884C: cmp.b $c18860(pc, d2.w), d3
pub fn i_c1884c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18850;
    cpu.ir = 0xB63B;
    let ad2 = 0xC18860_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $C18850: bcs.b $c1883e
pub fn i_c18850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18852;
    cpu.ir = 0x65EC;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1883E; return 10; }
    8
}
/// $C18852: move.b d3, (a1, d2.w)
pub fn i_c18852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18856;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C18856: rts
pub fn i_c18856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18858;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18858: ori.b #$0, (a2)
pub fn i_c18858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1885C;
    cpu.ir = 0x12;
    let ad1 = cpu.a[2];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1885C: move.l (a1)+, $5900(a1)
pub fn i_c1885c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18860;
    cpu.ir = 0x2359;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1].wrapping_add(0x5900);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C18860: ori.b #$0, d1
pub fn i_c18860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18864;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18864: ori.b #$0, d0
pub fn i_c18864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18868;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18868: jsr $c11cc0(pc)
pub fn i_c18868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1886C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1886C);
    cpu.pc = 0xC11CC0;
    16
}
/// $C1886C: tst.b d7
pub fn i_c1886c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1886E;
    cpu.ir = 0x4A07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1886E: beq.w $c18686
pub fn i_c1886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18872;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18686; return 10; }
    8
}
/// $C18872: bra.w $c187c0
pub fn i_c18872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18876;
    cpu.ir = 0x6000;
    cpu.pc = 0xC187C0;
    10
}
/// $C18876: moveq #$0, d6
pub fn i_c18876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18878;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18878: addq.b #$1, $ac2(a5)
pub fn i_c18878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1887C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1887C: movea.l $abe(a5), a0
pub fn i_c1887c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18880;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18880: clr.l (a0)+
pub fn i_c18880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18882;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C18882: move.w #$7119, d2
pub fn i_c18882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18886;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7119;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18886: move.b $ad3(a5), d0
pub fn i_c18886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1888A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1888A: jsr $c168be(pc)
pub fn i_c1888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1888E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1888E);
    cpu.pc = 0xC168BE;
    16
}
/// $C1888E: move.w #$7179, d2
pub fn i_c1888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18892;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7179;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18892: move.b $ad4(a5), d0
pub fn i_c18892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18896;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18896: jsr $c168be(pc)
pub fn i_c18896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1889A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1889A);
    cpu.pc = 0xC168BE;
    16
}
/// $C1889A: move.w #$71d9, d2
pub fn i_c1889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1889E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71D9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1889E: move.b $ad2(a5), d0
pub fn i_c1889e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188A2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C188A2: jsr $c168be(pc)
pub fn i_c188a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC188A6);
    cpu.pc = 0xC168BE;
    16
}
/// $C188A6: move.w #$7239, d2
pub fn i_c188a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188AA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7239;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C188AA: move.b $ad6(a5), d0
pub fn i_c188aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188AE;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C188AE: jsr $c168be(pc)
pub fn i_c188ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC188B2);
    cpu.pc = 0xC168BE;
    16
}
/// $C188B2: move.w #$7299, d2
pub fn i_c188b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188B6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7299;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C188B6: move.b $ad7(a5), d0
pub fn i_c188b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188BA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C188BA: jsr $c168be(pc)
pub fn i_c188ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC188BE);
    cpu.pc = 0xC168BE;
    16
}
/// $C188BE: move.w #$72f9, d2
pub fn i_c188be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188C2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x72F9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C188C2: move.b $ad8(a5), d0
pub fn i_c188c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188C6;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C188C6: jsr $c168be(pc)
pub fn i_c188c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC188CA);
    cpu.pc = 0xC168BE;
    16
}
/// $C188CA: clr.w (a0)+
pub fn i_c188ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188CC;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C188CC: move.l a0, $abe(a5)
pub fn i_c188cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188D0;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C188D0: subq.b #$1, $ac2(a5)
pub fn i_c188d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188D4;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C188D4: rts
pub fn i_c188d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C188D6: ori.b #$4, d3
pub fn i_c188d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188DA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C188DA: movep.w $5345(a0), d0
pub fn i_c188da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188DE;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C188DE: addq.w #$2, (a4)
pub fn i_c188de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188E0;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C188E0: ?
pub fn i_c188e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188E2;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C188E2: dc.w $4720
pub fn i_c188e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188E4;
    cpu.ir = 0x4720;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C188E4: subq.w #$2, (a0)
pub fn i_c188e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188E6;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C188E6: movea.l (a4), a0
pub fn i_c188e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188E8;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C188E8: swap d5
pub fn i_c188e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188EA;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C188EA: movea.l d3, a0
pub fn i_c188ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188EC;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C188EC: ?
pub fn i_c188ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188EE;
    cpu.ir = 0x414C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C188EE: ?
pub fn i_c188ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188F0;
    cpu.ir = 0x454E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C188F0: neg.w d1
pub fn i_c188f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC188F2;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1898A: jsr $c10fae(pc)
pub fn i_c1898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1898E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1898E);
    cpu.pc = 0xC10FAE;
    16
}
/// $C1898E: move.b #$1, $bbf(a5)
pub fn i_c1898e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18994;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18994: moveq #$0, d0
pub fn i_c18994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18996;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18996: move.w #$1d, d7
pub fn i_c18996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1899A;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1899A: movea.l a5, a0
pub fn i_c1899a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1899C;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1899C: move.l d0, (a0)+
pub fn i_c1899c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1899E;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1899E: dbra d7, $c1899c
pub fn i_c1899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189A2;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC189A2; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC189A2; return 14; }
    cpu.pc = 0xC1899C;
    10
}
/// $C189A2: lea.l $d08000.l, a4
pub fn i_c189a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189A8;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C189A8: moveq #$0, d6
pub fn i_c189a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189AA;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C189AA: move.w d6, d5
pub fn i_c189aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189AC;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C189AC: lea.l -$7edc(a4), a2
pub fn i_c189ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189B0;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[2] = ad1;
    16
}
/// $C189B0: lea.l -$7d60(a4), a1
pub fn i_c189b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189B4;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF82A0);
    cpu.a[1] = ad1;
    16
}
/// $C189B4: movea.l a5, a0
pub fn i_c189b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189B6;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C189B6: tst.w (a2)
pub fn i_c189b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189B8;
    cpu.ir = 0x4A52;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $C189B8: beq.b $c189d6
pub fn i_c189b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189BA;
    cpu.ir = 0x671C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC189D6; return 10; }
    8
}
/// $C189BA: addq.b #$1, d5
pub fn i_c189ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189BC;
    cpu.ir = 0x5205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C189BC: move.w d6, (a0)+
pub fn i_c189bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189BE;
    cpu.ir = 0x30C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C189BE: move.w $2(a2), d0
pub fn i_c189be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189C2;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C189C2: move.w d0, (a0)+
pub fn i_c189c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189C4;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C189C4: lsl.w #$4, d0
pub fn i_c189c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189C6;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C189C6: move.l (a1, d0.w), (a0)+
pub fn i_c189c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189CA;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C189CA: move.l $4(a1, d0.w), (a0)+
pub fn i_c189ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189CE;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C189CE: move.l $8(a1, d0.w), (a0)+
pub fn i_c189ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189D2;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C189D2: move.l $c(a1, d0.w), (a0)+
pub fn i_c189d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189D6;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0xC).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C189D6: addq.l #$4, a2
pub fn i_c189d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189D8;
    cpu.ir = 0x588A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $C189D8: addq.b #$1, d6
pub fn i_c189d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189DA;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C189DA: cmp.b -$7fb9(a4), d6
pub fn i_c189da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189DE;
    cpu.ir = 0xBC2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8047);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C189DE: bcs.b $c189b6
pub fn i_c189de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189E0;
    cpu.ir = 0x65D6;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC189B6; return 10; }
    8
}
/// $C189E0: move.b d5, $9e6(a5)
pub fn i_c189e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189E4;
    cpu.ir = 0x1B45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C189E4: addq.b #$1, d5
pub fn i_c189e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189E6;
    cpu.ir = 0x5205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C189E6: move.b d5, $9e7(a5)
pub fn i_c189e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189EA;
    cpu.ir = 0x1B45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E7);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C189EA: clr.w $9de(a5)
pub fn i_c189ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189EE;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x9DE);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C189EE: jsr $c12a0e(pc)
pub fn i_c189ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC189F2);
    cpu.pc = 0xC12A0E;
    16
}
/// $C189F2: move.b $9e6(a5), d5
pub fn i_c189f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189F6;
    cpu.ir = 0x1A2D;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C189F6: addq.b #$1, $ac2(a5)
pub fn i_c189f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189FA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C189FA: movea.l $abe(a5), a0
pub fn i_c189fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC189FE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C189FE: move.l #$c17e20, (a0)+
pub fn i_c189fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A04;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18A04: jsr $c17eaa(pc)
pub fn i_c18a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18A08);
    cpu.pc = 0xC17EAA;
    16
}
/// $C18A08: move.l #$c18aec, (a0)+
pub fn i_c18a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A0E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC18AEC;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18A0E: clr.l (a0)+
pub fn i_c18a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A10;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C18A10: movea.l a5, a1
pub fn i_c18a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A12;
    cpu.ir = 0x224D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C18A12: move.l #$50002, (a0)+
pub fn i_c18a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A18;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x50002;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18A18: move.w #$108, (a0)+
pub fn i_c18a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A1C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18A1C: move.b #$53, (a0)+
pub fn i_c18a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A20;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x53;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18A20: move.b #$4c, (a0)+
pub fn i_c18a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A24;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x4C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18A24: move.b #$4f, (a0)+
pub fn i_c18a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A28;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18A28: move.b #$54, (a0)+
pub fn i_c18a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A2C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x54;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18A2C: move.b $1(a1), d0
pub fn i_c18a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A30;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18A30: addi.b #$31, d0
pub fn i_c18a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A34;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x31, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18A34: move.b d0, (a0)+
pub fn i_c18a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A36;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18A36: move.b #$20, (a0)+
pub fn i_c18a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A3A;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18A3A: addq.l #$4, a1
pub fn i_c18a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A3C;
    cpu.ir = 0x5889;
    cpu.a[1] = cpu.a[1].wrapping_add(4);
    8
}
/// $C18A3C: move.l (a1)+, (a0)+
pub fn i_c18a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A3E;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18A3E: move.l (a1)+, (a0)+
pub fn i_c18a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A40;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18A40: move.l (a1)+, (a0)+
pub fn i_c18a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A42;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18A42: move.l (a1)+, (a0)+
pub fn i_c18a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A44;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18A44: move.w #$ff00, (a0)+
pub fn i_c18a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A48;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18A48: subq.b #$1, d5
pub fn i_c18a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A4A;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C18A4A: bne.b $c18a12
pub fn i_c18a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A4C;
    cpu.ir = 0x66C6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18A12; return 10; }
    8
}
/// $C18A4C: clr.w (a0)+
pub fn i_c18a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A4E;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18A4E: move.l a0, $abe(a5)
pub fn i_c18a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A52;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18A52: subq.b #$1, $ac2(a5)
pub fn i_c18a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A56;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18A56: move.w $9de(a5), $9e0(a5)
pub fn i_c18a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A5C;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9E0);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C18A5C: jsr $c18aa4(pc)
pub fn i_c18a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18A60);
    cpu.pc = 0xC18AA4;
    16
}
/// $C18A60: clr.w $ae4(a5)
pub fn i_c18a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A64;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C18A64: jsr $c1727e(pc)
pub fn i_c18a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18A68);
    cpu.pc = 0xC1727E;
    16
}
/// $C18A68: jsr $c17eb2(pc)
pub fn i_c18a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18A6C);
    cpu.pc = 0xC17EB2;
    16
}
/// $C18A6C: move.b $a97(a5), d0
pub fn i_c18a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A70;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18A70: btst.b #$6, d0
pub fn i_c18a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A74;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C18A74: beq.b $c18a7c
pub fn i_c18a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A76;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18A7C; return 10; }
    8
}
/// $C18A76: clr.b $bbf(a5)
pub fn i_c18a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A7A;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18A7A: rts
pub fn i_c18a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18A7C: btst.b #$4, d0
pub fn i_c18a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A80;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C18A80: bne.w $c18b0c
pub fn i_c18a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18B0C; return 10; }
    8
}
/// $C18A84: andi.b #$3, d0
pub fn i_c18a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A88;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18A88: beq.b $c18a64
pub fn i_c18a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A8A;
    cpu.ir = 0x67DA;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18A64; return 10; }
    8
}
/// $C18A8A: moveq #$1, d1
pub fn i_c18a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A8C;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18A8C: andi.b #$2, d0
pub fn i_c18a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A90;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18A90: bne.b $c18a94
pub fn i_c18a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A92;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18A94; return 10; }
    8
}
/// $C18A92: moveq #$ff, d1
pub fn i_c18a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A94;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18A94: add.w $9de(a5), d1
pub fn i_c18a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A98;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C18A98: cmp.b $9e7(a5), d1
pub fn i_c18a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A9C;
    cpu.ir = 0xB22D;
    let ad2 = cpu.a[5].wrapping_add(0x9E7);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C18A9C: bcc.b $c18a64
pub fn i_c18a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18A9E;
    cpu.ir = 0x64C6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC18A64; return 10; }
    8
}
/// $C18A9E: move.w d1, $9e0(a5)
pub fn i_c18a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AA2;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C18AA2: bra.b $c18a5c
pub fn i_c18aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AA4;
    cpu.ir = 0x60B8;
    cpu.pc = 0xC18A5C;
    10
}
/// $C18AA4: addq.b #$1, $ac2(a5)
pub fn i_c18aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AA8;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18AA8: movea.l $abe(a5), a0
pub fn i_c18aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AAC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18AAC: clr.l (a0)+
pub fn i_c18aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AAE;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C18AAE: move.w #$20ff, d1
pub fn i_c18aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18AB2: jsr $c18ad0(pc)
pub fn i_c18ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18AB6);
    cpu.pc = 0xC18AD0;
    16
}
/// $C18AB6: move.w $9e0(a5), $9de(a5)
pub fn i_c18ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ABC;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9DE);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C18ABC: move.w #$11ff, d1
pub fn i_c18abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AC0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18AC0: jsr $c18ad0(pc)
pub fn i_c18ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18AC4);
    cpu.pc = 0xC18AD0;
    16
}
/// $C18AC4: clr.w (a0)+
pub fn i_c18ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AC6;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18AC6: move.l a0, $abe(a5)
pub fn i_c18ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ACA;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18ACA: subq.b #$1, $ac2(a5)
pub fn i_c18aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ACE;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18ACE: rts
pub fn i_c18ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18AD0: move.w #$3, (a0)+
pub fn i_c18ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AD4;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18AD4: move.w $9de(a5), d0
pub fn i_c18ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AD8;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C18AD8: move.w d0, d2
pub fn i_c18ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ADA;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18ADA: add.w d0, d0
pub fn i_c18ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ADC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18ADC: add.w d0, d2
pub fn i_c18adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ADE;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18ADE: addi.w #$7085, d2
pub fn i_c18ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AE2;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7085, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C18AE2: move.w d2, (a0)+
pub fn i_c18ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AE4;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18AE4: move.w #$1108, (a0)+
pub fn i_c18ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AE8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18AE8: move.w d1, (a0)+
pub fn i_c18ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AEA;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18AEA: rts
pub fn i_c18aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18AEC: ori.b #$c5, d3
pub fn i_c18aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AF0;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xC5) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18AF0: movep.w $5345(a0), d0
pub fn i_c18af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AF4;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18AF4: addq.w #$2, (a4)
pub fn i_c18af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AF6;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18AF6: ?
pub fn i_c18af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AF8;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18AF8: dc.w $4720
pub fn i_c18af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AFA;
    cpu.ir = 0x4720;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18AFA: subq.w #$2, (a0)
pub fn i_c18afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AFC;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18AFC: movea.l (a4), a0
pub fn i_c18afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18AFE;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C18AFE: swap d5
pub fn i_c18afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B00;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18B00: movea.l d3, a0
pub fn i_c18b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B02;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C18B02: ?
pub fn i_c18b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B04;
    cpu.ir = 0x4142;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18B04: ?
pub fn i_c18b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B06;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18B06: ?
pub fn i_c18b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B08;
    cpu.ir = 0x4554;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18B08: dc.w $ffff
pub fn i_c18b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B0A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C18B0A: ori.b #$7a, d0
pub fn i_c18b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B0E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x7A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18B0C: pea.l $c189ee(pc)
pub fn i_c18b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B10;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC189EE);
    24
}
/// $C18B0E: dc.w $fee0
pub fn i_c18b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B10;
    cpu.ir = 0xFEE0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C18B10: tst.w $9de(a5)
pub fn i_c18b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B14;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C18B14: bne.w $c18f2c
pub fn i_c18b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B18;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18F2C; return 10; }
    8
}
/// $C18B18: jmp $c19edc(pc)
pub fn i_c18b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B1C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC19EDC;
    8
}
/// $C18B1C: jsr $c12a0e(pc)
pub fn i_c18b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18B20);
    cpu.pc = 0xC12A0E;
    16
}
/// $C18B20: addq.b #$1, $ac2(a5)
pub fn i_c18b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B24;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18B24: movea.l $abe(a5), a0
pub fn i_c18b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B28;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18B28: move.l #$c18eaa, (a0)+
pub fn i_c18b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B2E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC18EAA;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18B2E: move.l a0, $abe(a5)
pub fn i_c18b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B32;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18B32: subq.b #$1, $ac2(a5)
pub fn i_c18b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B36;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18B36: clr.l $9e2(a5)
pub fn i_c18b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B3A;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C18B3A: move.b #$2, $9e8(a5)
pub fn i_c18b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B40;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18B40: jsr $c18e44(pc)
pub fn i_c18b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18B44);
    cpu.pc = 0xC18E44;
    16
}
/// $C18B44: jsr $c17290(pc)
pub fn i_c18b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18B48);
    cpu.pc = 0xC17290;
    16
}
/// $C18B48: lea.l $d08000.l, a4
pub fn i_c18b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B4E;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C18B4E: addq.w #$1, $9e2(a5)
pub fn i_c18b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B52;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C18B52: cmpi.w #$4, $9e2(a5)
pub fn i_c18b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B58;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $C18B58: bne.b $c18b40
pub fn i_c18b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B5A;
    cpu.ir = 0x66E6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18B40; return 10; }
    8
}
/// $C18B5A: clr.l $9e2(a5)
pub fn i_c18b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B5E;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C18B5E: clr.b $9e8(a5)
pub fn i_c18b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B62;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0x9E8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18B62: jsr $c18e62(pc)
pub fn i_c18b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18B66);
    cpu.pc = 0xC18E62;
    16
}
/// $C18B66: jsr $c18c18(pc)
pub fn i_c18b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18B6A);
    cpu.pc = 0xC18C18;
    16
}
/// $C18B6A: jsr $c1727e(pc)
pub fn i_c18b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18B6E);
    cpu.pc = 0xC1727E;
    16
}
/// $C18B6E: move.b $a97(a5), d0
pub fn i_c18b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B72;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18B72: btst.b #$6, d0
pub fn i_c18b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B76;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C18B76: beq.b $c18b7a
pub fn i_c18b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B78;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18B7A; return 10; }
    8
}
/// $C18B78: rts
pub fn i_c18b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18B7A: andi.b #$3, d0
pub fn i_c18b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B7E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18B7E: beq.w $c18b9c
pub fn i_c18b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18B9C; return 10; }
    8
}
/// $C18B82: moveq #$1, d1
pub fn i_c18b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B84;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18B84: andi.b #$2, d0
pub fn i_c18b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B88;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18B88: bne.b $c18b8c
pub fn i_c18b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B8A;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18B8C; return 10; }
    8
}
/// $C18B8A: moveq #$ff, d1
pub fn i_c18b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B8C;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18B8C: add.w $9e2(a5), d1
pub fn i_c18b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B90;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C18B90: cmpi.w #$4, d1
pub fn i_c18b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B94;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x4, Size::Word, false);
    8
}
/// $C18B94: bcc.b $c18b6a
pub fn i_c18b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B96;
    cpu.ir = 0x64D4;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC18B6A; return 10; }
    8
}
/// $C18B96: move.w d1, $9e4(a5)
pub fn i_c18b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B9A;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C18B9A: bra.b $c18b62
pub fn i_c18b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18B9C;
    cpu.ir = 0x60C6;
    cpu.pc = 0xC18B62;
    10
}
/// $C18B9C: pea.l $c18b6a(pc)
pub fn i_c18b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BA0;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC18B6A);
    24
}
/// $C18BA0: move.w $9e2(a5), d0
pub fn i_c18ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BA4;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C18BA4: add.w d0, d0
pub fn i_c18ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BA6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18BA6: add.w d0, d0
pub fn i_c18ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BA8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18BA8: movea.l $c18bae(pc, d0.w), a0
pub fn i_c18ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BAC;
    cpu.ir = 0x207B;
    let ad2 = 0xC18BAE_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C18BAC: jmp (a0)
pub fn i_c18bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BAE;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C18BAE: ?
pub fn i_c18bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BB0;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18BB0: ?
pub fn i_c18bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BB2;
    cpu.ir = 0x8BBE;
    let v1: u32 = 0x0;
    let r2 = (v1 | cpu.d[5]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18BB2: ?
pub fn i_c18bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BB4;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18BB4: or.l -(a0), d6
pub fn i_c18bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BB6;
    cpu.ir = 0x8CA0;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let r = (v3 | cpu.d[6]) & 0xFFFFFFFF;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C18BB6: ?
pub fn i_c18bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BB8;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18BB8: or.b d6, (a4)+
pub fn i_c18bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BBA;
    cpu.ir = 0x8D1C;
    let ad1 = cpu.a[4]; cpu.a[4] = ad1.wrapping_add(1);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[6]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    8
}
/// $C18BBA: ?
pub fn i_c18bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BBC;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18BBC: ?
pub fn i_c18bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BBE;
    cpu.ir = 0x8DC8;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[6] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 158; }
    cpu.d[6] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    158
}
/// $C18BBE: move.b $a98(a5), d0
pub fn i_c18bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BC2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18BC2: andi.b #$30, d0
pub fn i_c18bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BC6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18BC6: bne.w $c18be0
pub fn i_c18bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18BE0; return 10; }
    8
}
/// $C18BCA: move.b $a97(a5), d0
pub fn i_c18bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BCE;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18BCE: andi.b #$c, d0
pub fn i_c18bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BD2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18BD2: beq.w $c18c52
pub fn i_c18bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BD6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18C52; return 10; }
    8
}
/// $C18BD6: bchg.b #$0, $9e8(a5)
pub fn i_c18bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BDC;
    cpu.ir = 0x86D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[5].wrapping_add(0x9E8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C18BDC: bra.w $c18c18
pub fn i_c18bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BE0;
    cpu.ir = 0x6000;
    cpu.pc = 0xC18C18;
    10
}
/// $C18BE0: move.b d0, $3a001d.l
pub fn i_c18be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BE6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C18BE6: moveq #$1, d1
pub fn i_c18be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BE8;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18BE8: andi.b #$10, d0
pub fn i_c18be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BEC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18BEC: bne.b $c18bf0
pub fn i_c18bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BEE;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18BF0; return 10; }
    8
}
/// $C18BEE: moveq #$ff, d1
pub fn i_c18bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BF0;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18BF0: lea.l -$7fc6(a4), a3
pub fn i_c18bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BF4;
    cpu.ir = 0x47EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF803A);
    cpu.a[3] = ad1;
    16
}
/// $C18BF4: move.w $9e2(a5), d0
pub fn i_c18bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BF8;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C18BF8: add.w d0, d0
pub fn i_c18bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BFA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18BFA: adda.l d0, a3
pub fn i_c18bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18BFC;
    cpu.ir = 0xD7C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $C18BFC: btst.b #$0, $9e8(a5)
pub fn i_c18bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C02;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C18C02: beq.b $c18c06
pub fn i_c18c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C04;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18C06; return 10; }
    8
}
/// $C18C04: addq.l #$1, a3
pub fn i_c18c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C06;
    cpu.ir = 0x528B;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    8
}
/// $C18C06: move.b (a3), d0
pub fn i_c18c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C08;
    cpu.ir = 0x1013;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C18C08: add.b d1, d0
pub fn i_c18c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C0A;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C18C0A: bne.b $c18c0e
pub fn i_c18c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C0C;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18C0E; return 10; }
    8
}
/// $C18C0C: moveq #$9, d0
pub fn i_c18c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C0E;
    cpu.ir = 0x7009;
    cpu.d[0] = 0x9;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18C0E: cmpi.b #$a, d0
pub fn i_c18c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C12;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C18C12: bcs.b $c18c16
pub fn i_c18c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C14;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC18C16; return 10; }
    8
}
/// $C18C14: moveq #$1, d0
pub fn i_c18c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C16;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18C16: move.b d0, (a3)
pub fn i_c18c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C18;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C18C18: lea.l -$7fc6(a4), a3
pub fn i_c18c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C1C;
    cpu.ir = 0x47EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF803A);
    cpu.a[3] = ad1;
    16
}
/// $C18C1C: moveq #$0, d2
pub fn i_c18c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C1E;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18C1E: move.w #$7008, d6
pub fn i_c18c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C22;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x7008;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18C22: addq.b #$1, $ac2(a5)
pub fn i_c18c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C26;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18C26: movea.l $abe(a5), a0
pub fn i_c18c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C2A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18C2A: clr.l (a0)+
pub fn i_c18c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C2C;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C18C2C: move.w #$220, d4
pub fn i_c18c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C30;
    cpu.ir = 0x383C;
    let v1: u32 = 0x220;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18C30: move.w #$2c0, d3
pub fn i_c18c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C34;
    cpu.ir = 0x363C;
    let v1: u32 = 0x2C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18C34: moveq #$0, d7
pub fn i_c18c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C36;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18C36: jsr $c18c54(pc)
pub fn i_c18c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18C3A);
    cpu.pc = 0xC18C54;
    16
}
/// $C18C3A: move.w #$300, d4
pub fn i_c18c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C3E;
    cpu.ir = 0x383C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18C3E: move.w #$3e0, d3
pub fn i_c18c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C42;
    cpu.ir = 0x363C;
    let v1: u32 = 0x3E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18C42: moveq #$1, d7
pub fn i_c18c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C44;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18C44: jsr $c18c54(pc)
pub fn i_c18c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18C48);
    cpu.pc = 0xC18C54;
    16
}
/// $C18C48: clr.w (a0)+
pub fn i_c18c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C4A;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18C4A: move.l a0, $abe(a5)
pub fn i_c18c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C4E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18C4E: subq.b #$1, $ac2(a5)
pub fn i_c18c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C52;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18C52: rts
pub fn i_c18c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18C54: move.w #$3, (a0)+
pub fn i_c18c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C58;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18C58: add.w d6, d4
pub fn i_c18c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C5A;
    cpu.ir = 0xD846;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18C5A: move.w d4, (a0)+
pub fn i_c18c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C5C;
    cpu.ir = 0x30C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18C5C: move.w #$108, (a0)+
pub fn i_c18c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C60;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18C60: cmp.b $9e8(a5), d7
pub fn i_c18c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C64;
    cpu.ir = 0xBE2D;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C18C64: bne.b $c18c72
pub fn i_c18c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C66;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18C72; return 10; }
    8
}
/// $C18C66: cmp.w $9e2(a5), d2
pub fn i_c18c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C6A;
    cpu.ir = 0xB46D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C18C6A: bne.b $c18c72
pub fn i_c18c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C6C;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18C72; return 10; }
    8
}
/// $C18C6C: move.w #$1108, -$2(a0)
pub fn i_c18c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C72;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C18C72: move.b (a3)+, d1
pub fn i_c18c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C74;
    cpu.ir = 0x121B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C18C74: addi.b #$30, d1
pub fn i_c18c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C78;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18C78: move.b d1, (a0)+
pub fn i_c18c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C7A;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC173C0, i_c173c0),
    (0xC173C2, i_c173c2),
    (0xC173C4, i_c173c4),
    (0xC173C6, i_c173c6),
    (0xC173C8, i_c173c8),
    (0xC173CA, i_c173ca),
    (0xC173CC, i_c173cc),
    (0xC173CE, i_c173ce),
    (0xC173D0, i_c173d0),
    (0xC173D4, i_c173d4),
    (0xC173D6, i_c173d6),
    (0xC173D8, i_c173d8),
    (0xC173DA, i_c173da),
    (0xC173DC, i_c173dc),
    (0xC173DE, i_c173de),
    (0xC173E0, i_c173e0),
    (0xC173E2, i_c173e2),
    (0xC173E4, i_c173e4),
    (0xC173E6, i_c173e6),
    (0xC173E8, i_c173e8),
    (0xC173EE, i_c173ee),
    (0xC173F0, i_c173f0),
    (0xC173F2, i_c173f2),
    (0xC173F4, i_c173f4),
    (0xC173F8, i_c173f8),
    (0xC173FA, i_c173fa),
    (0xC173FC, i_c173fc),
    (0xC173FE, i_c173fe),
    (0xC17402, i_c17402),
    (0xC17406, i_c17406),
    (0xC17408, i_c17408),
    (0xC1740A, i_c1740a),
    (0xC1740E, i_c1740e),
    (0xC17410, i_c17410),
    (0xC17412, i_c17412),
    (0xC17414, i_c17414),
    (0xC17418, i_c17418),
    (0xC1741A, i_c1741a),
    (0xC1741E, i_c1741e),
    (0xC17420, i_c17420),
    (0xC17424, i_c17424),
    (0xC17426, i_c17426),
    (0xC17428, i_c17428),
    (0xC1742C, i_c1742c),
    (0xC17430, i_c17430),
    (0xC17434, i_c17434),
    (0xC17436, i_c17436),
    (0xC1743A, i_c1743a),
    (0xC1743E, i_c1743e),
    (0xC17440, i_c17440),
    (0xC17444, i_c17444),
    (0xC17446, i_c17446),
    (0xC17448, i_c17448),
    (0xC1744A, i_c1744a),
    (0xC1744C, i_c1744c),
    (0xC1744E, i_c1744e),
    (0xC17454, i_c17454),
    (0xC17456, i_c17456),
    (0xC17458, i_c17458),
    (0xC1745A, i_c1745a),
    (0xC1745E, i_c1745e),
    (0xC17460, i_c17460),
    (0xC17462, i_c17462),
    (0xC17464, i_c17464),
    (0xC17468, i_c17468),
    (0xC1746C, i_c1746c),
    (0xC1746E, i_c1746e),
    (0xC17470, i_c17470),
    (0xC17474, i_c17474),
    (0xC17476, i_c17476),
    (0xC17478, i_c17478),
    (0xC1747A, i_c1747a),
    (0xC1747E, i_c1747e),
    (0xC17480, i_c17480),
    (0xC17482, i_c17482),
    (0xC17484, i_c17484),
    (0xC17486, i_c17486),
    (0xC1748C, i_c1748c),
    (0xC17490, i_c17490),
    (0xC17496, i_c17496),
    (0xC1749C, i_c1749c),
    (0xC174A2, i_c174a2),
    (0xC174AA, i_c174aa),
    (0xC174AE, i_c174ae),
    (0xC174B2, i_c174b2),
    (0xC174B6, i_c174b6),
    (0xC174BA, i_c174ba),
    (0xC174C2, i_c174c2),
    (0xC174C8, i_c174c8),
    (0xC174CE, i_c174ce),
    (0xC174D4, i_c174d4),
    (0xC174D8, i_c174d8),
    (0xC174DE, i_c174de),
    (0xC174E4, i_c174e4),
    (0xC174E8, i_c174e8),
    (0xC174EA, i_c174ea),
    (0xC174EE, i_c174ee),
    (0xC174F6, i_c174f6),
    (0xC174FC, i_c174fc),
    (0xC17500, i_c17500),
    (0xC17506, i_c17506),
    (0xC1750A, i_c1750a),
    (0xC17510, i_c17510),
    (0xC17516, i_c17516),
    (0xC1751C, i_c1751c),
    (0xC17522, i_c17522),
    (0xC17528, i_c17528),
    (0xC1752E, i_c1752e),
    (0xC17532, i_c17532),
    (0xC17536, i_c17536),
    (0xC1753A, i_c1753a),
    (0xC1753E, i_c1753e),
    (0xC17542, i_c17542),
    (0xC17546, i_c17546),
    (0xC1754A, i_c1754a),
    (0xC1754E, i_c1754e),
    (0xC17552, i_c17552),
    (0xC17556, i_c17556),
    (0xC17558, i_c17558),
    (0xC1755C, i_c1755c),
    (0xC1755E, i_c1755e),
    (0xC17562, i_c17562),
    (0xC17568, i_c17568),
    (0xC1756C, i_c1756c),
    (0xC17572, i_c17572),
    (0xC17576, i_c17576),
    (0xC1757C, i_c1757c),
    (0xC17580, i_c17580),
    (0xC17584, i_c17584),
    (0xC17588, i_c17588),
    (0xC1758C, i_c1758c),
    (0xC17590, i_c17590),
    (0xC17592, i_c17592),
    (0xC17596, i_c17596),
    (0xC1759A, i_c1759a),
    (0xC1759C, i_c1759c),
    (0xC175A2, i_c175a2),
    (0xC175A6, i_c175a6),
    (0xC175AA, i_c175aa),
    (0xC175AC, i_c175ac),
    (0xC175AE, i_c175ae),
    (0xC175B0, i_c175b0),
    (0xC175B2, i_c175b2),
    (0xC175B4, i_c175b4),
    (0xC175B6, i_c175b6),
    (0xC175BA, i_c175ba),
    (0xC175BC, i_c175bc),
    (0xC175BE, i_c175be),
    (0xC175C2, i_c175c2),
    (0xC175C6, i_c175c6),
    (0xC175CA, i_c175ca),
    (0xC175CE, i_c175ce),
    (0xC175D2, i_c175d2),
    (0xC175D6, i_c175d6),
    (0xC175D8, i_c175d8),
    (0xC175DA, i_c175da),
    (0xC175DC, i_c175dc),
    (0xC175E0, i_c175e0),
    (0xC175E2, i_c175e2),
    (0xC175E4, i_c175e4),
    (0xC175E6, i_c175e6),
    (0xC175EA, i_c175ea),
    (0xC175EE, i_c175ee),
    (0xC175F2, i_c175f2),
    (0xC175F4, i_c175f4),
    (0xC175F8, i_c175f8),
    (0xC175FA, i_c175fa),
    (0xC175FE, i_c175fe),
    (0xC17602, i_c17602),
    (0xC17606, i_c17606),
    (0xC1760A, i_c1760a),
    (0xC1760E, i_c1760e),
    (0xC17612, i_c17612),
    (0xC17616, i_c17616),
    (0xC1761C, i_c1761c),
    (0xC17620, i_c17620),
    (0xC17624, i_c17624),
    (0xC17628, i_c17628),
    (0xC17630, i_c17630),
    (0xC17634, i_c17634),
    (0xC1763A, i_c1763a),
    (0xC17640, i_c17640),
    (0xC17644, i_c17644),
    (0xC1764A, i_c1764a),
    (0xC1764C, i_c1764c),
    (0xC1766C, i_c1766c),
    (0xC1766E, i_c1766e),
    (0xC17672, i_c17672),
    (0xC17674, i_c17674),
    (0xC17676, i_c17676),
    (0xC1767A, i_c1767a),
    (0xC1767C, i_c1767c),
    (0xC1767E, i_c1767e),
    (0xC17682, i_c17682),
    (0xC17684, i_c17684),
    (0xC17686, i_c17686),
    (0xC17688, i_c17688),
    (0xC1768C, i_c1768c),
    (0xC1768E, i_c1768e),
    (0xC17690, i_c17690),
    (0xC17694, i_c17694),
    (0xC17696, i_c17696),
    (0xC17698, i_c17698),
    (0xC1769C, i_c1769c),
    (0xC1769E, i_c1769e),
    (0xC176A0, i_c176a0),
    (0xC176A4, i_c176a4),
    (0xC176A6, i_c176a6),
    (0xC176A8, i_c176a8),
    (0xC176AC, i_c176ac),
    (0xC176AE, i_c176ae),
    (0xC176B0, i_c176b0),
    (0xC176B4, i_c176b4),
    (0xC176B6, i_c176b6),
    (0xC176BA, i_c176ba),
    (0xC176BE, i_c176be),
    (0xC176C0, i_c176c0),
    (0xC176C4, i_c176c4),
    (0xC176C6, i_c176c6),
    (0xC176C8, i_c176c8),
    (0xC176CA, i_c176ca),
    (0xC176CC, i_c176cc),
    (0xC176CE, i_c176ce),
    (0xC176D0, i_c176d0),
    (0xC176D2, i_c176d2),
    (0xC176D6, i_c176d6),
    (0xC176D8, i_c176d8),
    (0xC176DA, i_c176da),
    (0xC176DC, i_c176dc),
    (0xC176DE, i_c176de),
    (0xC176E0, i_c176e0),
    (0xC176E2, i_c176e2),
    (0xC176E4, i_c176e4),
    (0xC176E6, i_c176e6),
    (0xC176E8, i_c176e8),
    (0xC176EA, i_c176ea),
    (0xC176EC, i_c176ec),
    (0xC176F0, i_c176f0),
    (0xC176F2, i_c176f2),
    (0xC176F6, i_c176f6),
    (0xC176FA, i_c176fa),
    (0xC176FC, i_c176fc),
    (0xC17700, i_c17700),
    (0xC17704, i_c17704),
    (0xC17708, i_c17708),
    (0xC1770C, i_c1770c),
    (0xC1770E, i_c1770e),
    (0xC17712, i_c17712),
    (0xC17714, i_c17714),
    (0xC17718, i_c17718),
    (0xC1771C, i_c1771c),
    (0xC17720, i_c17720),
    (0xC17724, i_c17724),
    (0xC17728, i_c17728),
    (0xC1772C, i_c1772c),
    (0xC1772E, i_c1772e),
    (0xC17730, i_c17730),
    (0xC17734, i_c17734),
    (0xC17736, i_c17736),
    (0xC1773A, i_c1773a),
    (0xC1773C, i_c1773c),
    (0xC17740, i_c17740),
    (0xC17744, i_c17744),
    (0xC17748, i_c17748),
    (0xC1774C, i_c1774c),
    (0xC17750, i_c17750),
    (0xC17978, i_c17978),
    (0xC1797A, i_c1797a),
    (0xC1797E, i_c1797e),
    (0xC17982, i_c17982),
    (0xC17986, i_c17986),
    (0xC1798A, i_c1798a),
    (0xC1798E, i_c1798e),
    (0xC17992, i_c17992),
    (0xC17994, i_c17994),
    (0xC17996, i_c17996),
    (0xC17998, i_c17998),
    (0xC1799C, i_c1799c),
    (0xC179A0, i_c179a0),
    (0xC179A2, i_c179a2),
    (0xC179A4, i_c179a4),
    (0xC179A6, i_c179a6),
    (0xC179AA, i_c179aa),
    (0xC179AE, i_c179ae),
    (0xC179B0, i_c179b0),
    (0xC179B2, i_c179b2),
    (0xC179B4, i_c179b4),
    (0xC179B8, i_c179b8),
    (0xC179BC, i_c179bc),
    (0xC179C0, i_c179c0),
    (0xC179C2, i_c179c2),
    (0xC179C4, i_c179c4),
    (0xC179C6, i_c179c6),
    (0xC179C8, i_c179c8),
    (0xC179CE, i_c179ce),
    (0xC179D2, i_c179d2),
    (0xC179D4, i_c179d4),
    (0xC179D6, i_c179d6),
    (0xC179E6, i_c179e6),
    (0xC179EA, i_c179ea),
    (0xC179EC, i_c179ec),
    (0xC179F0, i_c179f0),
    (0xC179F2, i_c179f2),
    (0xC179F6, i_c179f6),
    (0xC179F8, i_c179f8),
    (0xC179FA, i_c179fa),
    (0xC179FC, i_c179fc),
    (0xC17A00, i_c17a00),
    (0xC17A02, i_c17a02),
    (0xC17A04, i_c17a04),
    (0xC17A06, i_c17a06),
    (0xC17A08, i_c17a08),
    (0xC17A0C, i_c17a0c),
    (0xC17A0E, i_c17a0e),
    (0xC17A12, i_c17a12),
    (0xC17A14, i_c17a14),
    (0xC17A18, i_c17a18),
    (0xC17A1A, i_c17a1a),
    (0xC17A1E, i_c17a1e),
    (0xC17A20, i_c17a20),
    (0xC17A24, i_c17a24),
    (0xC17A26, i_c17a26),
    (0xC17A28, i_c17a28),
    (0xC17A2A, i_c17a2a),
    (0xC17A2C, i_c17a2c),
    (0xC17A2E, i_c17a2e),
    (0xC17A32, i_c17a32),
    (0xC17A34, i_c17a34),
    (0xC17A38, i_c17a38),
    (0xC17A3A, i_c17a3a),
    (0xC17A3E, i_c17a3e),
    (0xC17A40, i_c17a40),
    (0xC17A42, i_c17a42),
    (0xC17A44, i_c17a44),
    (0xC17A46, i_c17a46),
    (0xC17A4A, i_c17a4a),
    (0xC17A4C, i_c17a4c),
    (0xC17A4E, i_c17a4e),
    (0xC17A50, i_c17a50),
    (0xC17A52, i_c17a52),
    (0xC17A54, i_c17a54),
    (0xC17A56, i_c17a56),
    (0xC17A58, i_c17a58),
    (0xC17A5A, i_c17a5a),
    (0xC17A60, i_c17a60),
    (0xC17A64, i_c17a64),
    (0xC17A66, i_c17a66),
    (0xC17A68, i_c17a68),
    (0xC17A78, i_c17a78),
    (0xC17A7C, i_c17a7c),
    (0xC17A7E, i_c17a7e),
    (0xC17A82, i_c17a82),
    (0xC17A84, i_c17a84),
    (0xC17A88, i_c17a88),
    (0xC17A8A, i_c17a8a),
    (0xC17A8C, i_c17a8c),
    (0xC17A8E, i_c17a8e),
    (0xC17A92, i_c17a92),
    (0xC17A94, i_c17a94),
    (0xC17A96, i_c17a96),
    (0xC17A98, i_c17a98),
    (0xC17A9A, i_c17a9a),
    (0xC17A9E, i_c17a9e),
    (0xC17AA0, i_c17aa0),
    (0xC17AA4, i_c17aa4),
    (0xC17AA6, i_c17aa6),
    (0xC17AAA, i_c17aaa),
    (0xC17AAC, i_c17aac),
    (0xC17AB0, i_c17ab0),
    (0xC17AB2, i_c17ab2),
    (0xC17AB6, i_c17ab6),
    (0xC17AB8, i_c17ab8),
    (0xC17ABA, i_c17aba),
    (0xC17ABC, i_c17abc),
    (0xC17ABE, i_c17abe),
    (0xC17AC0, i_c17ac0),
    (0xC17AC4, i_c17ac4),
    (0xC17AC6, i_c17ac6),
    (0xC17ACA, i_c17aca),
    (0xC17ACC, i_c17acc),
    (0xC17AD0, i_c17ad0),
    (0xC17AD2, i_c17ad2),
    (0xC17AD4, i_c17ad4),
    (0xC17AD6, i_c17ad6),
    (0xC17AD8, i_c17ad8),
    (0xC17ADC, i_c17adc),
    (0xC17ADE, i_c17ade),
    (0xC17AE0, i_c17ae0),
    (0xC17AE2, i_c17ae2),
    (0xC17AE4, i_c17ae4),
    (0xC17AE6, i_c17ae6),
    (0xC17AE8, i_c17ae8),
    (0xC17AEC, i_c17aec),
    (0xC17AF0, i_c17af0),
    (0xC17AF4, i_c17af4),
    (0xC17AF8, i_c17af8),
    (0xC17AFE, i_c17afe),
    (0xC17B04, i_c17b04),
    (0xC17B06, i_c17b06),
    (0xC17B08, i_c17b08),
    (0xC17B0A, i_c17b0a),
    (0xC17B0C, i_c17b0c),
    (0xC17B12, i_c17b12),
    (0xC17B16, i_c17b16),
    (0xC17B1C, i_c17b1c),
    (0xC17B20, i_c17b20),
    (0xC17B24, i_c17b24),
    (0xC17B28, i_c17b28),
    (0xC17B2C, i_c17b2c),
    (0xC17B30, i_c17b30),
    (0xC17B34, i_c17b34),
    (0xC17B38, i_c17b38),
    (0xC17B3C, i_c17b3c),
    (0xC17B40, i_c17b40),
    (0xC17B46, i_c17b46),
    (0xC17B4C, i_c17b4c),
    (0xC17B50, i_c17b50),
    (0xC17B54, i_c17b54),
    (0xC17B58, i_c17b58),
    (0xC17B5C, i_c17b5c),
    (0xC17B60, i_c17b60),
    (0xC17B64, i_c17b64),
    (0xC17B6A, i_c17b6a),
    (0xC17B70, i_c17b70),
    (0xC17B76, i_c17b76),
    (0xC17B7A, i_c17b7a),
    (0xC17B7E, i_c17b7e),
    (0xC17B82, i_c17b82),
    (0xC17B86, i_c17b86),
    (0xC17B8A, i_c17b8a),
    (0xC17B8E, i_c17b8e),
    (0xC17B98, i_c17b98),
    (0xC17BA2, i_c17ba2),
    (0xC17BAC, i_c17bac),
    (0xC17BB2, i_c17bb2),
    (0xC17BB8, i_c17bb8),
    (0xC17BBE, i_c17bbe),
    (0xC17BC6, i_c17bc6),
    (0xC17BCE, i_c17bce),
    (0xC17BD6, i_c17bd6),
    (0xC17BDC, i_c17bdc),
    (0xC17BE4, i_c17be4),
    (0xC17BE8, i_c17be8),
    (0xC17BEE, i_c17bee),
    (0xC17BF2, i_c17bf2),
    (0xC17BF6, i_c17bf6),
    (0xC17BFA, i_c17bfa),
    (0xC17BFE, i_c17bfe),
    (0xC17C04, i_c17c04),
    (0xC17C0A, i_c17c0a),
    (0xC17C0E, i_c17c0e),
    (0xC17C10, i_c17c10),
    (0xC17C12, i_c17c12),
    (0xC17C16, i_c17c16),
    (0xC17C1A, i_c17c1a),
    (0xC17C1E, i_c17c1e),
    (0xC17C22, i_c17c22),
    (0xC17C26, i_c17c26),
    (0xC17C2A, i_c17c2a),
    (0xC17C2E, i_c17c2e),
    (0xC17C32, i_c17c32),
    (0xC17C36, i_c17c36),
    (0xC17C3A, i_c17c3a),
    (0xC17C3E, i_c17c3e),
    (0xC17C42, i_c17c42),
    (0xC17C44, i_c17c44),
    (0xC17C48, i_c17c48),
    (0xC17C4C, i_c17c4c),
    (0xC17C52, i_c17c52),
    (0xC17C56, i_c17c56),
    (0xC17C5C, i_c17c5c),
    (0xC17C60, i_c17c60),
    (0xC17C64, i_c17c64),
    (0xC17C66, i_c17c66),
    (0xC17C6A, i_c17c6a),
    (0xC17C6E, i_c17c6e),
    (0xC17C72, i_c17c72),
    (0xC17C76, i_c17c76),
    (0xC17C7A, i_c17c7a),
    (0xC17C7E, i_c17c7e),
    (0xC17C82, i_c17c82),
    (0xC17C86, i_c17c86),
    (0xC17C8A, i_c17c8a),
    (0xC17C8C, i_c17c8c),
    (0xC17C90, i_c17c90),
    (0xC17C94, i_c17c94),
    (0xC17C98, i_c17c98),
    (0xC17C9C, i_c17c9c),
    (0xC17CA0, i_c17ca0),
    (0xC17CA4, i_c17ca4),
    (0xC17CA8, i_c17ca8),
    (0xC17CAA, i_c17caa),
    (0xC17CAC, i_c17cac),
    (0xC17CB0, i_c17cb0),
    (0xC17CB2, i_c17cb2),
    (0xC17CB4, i_c17cb4),
    (0xC17CB8, i_c17cb8),
    (0xC17CBA, i_c17cba),
    (0xC17CBE, i_c17cbe),
    (0xC17CC0, i_c17cc0),
    (0xC17CC4, i_c17cc4),
    (0xC17CC8, i_c17cc8),
    (0xC17CCA, i_c17cca),
    (0xC17CCE, i_c17cce),
    (0xC17CD2, i_c17cd2),
    (0xC17CD4, i_c17cd4),
    (0xC17CD8, i_c17cd8),
    (0xC17CDC, i_c17cdc),
    (0xC17CE2, i_c17ce2),
    (0xC17CE6, i_c17ce6),
    (0xC17CEA, i_c17cea),
    (0xC17CEC, i_c17cec),
    (0xC17CF0, i_c17cf0),
    (0xC17CF4, i_c17cf4),
    (0xC17CF6, i_c17cf6),
    (0xC17CFA, i_c17cfa),
    (0xC17CFE, i_c17cfe),
    (0xC17D00, i_c17d00),
    (0xC17D02, i_c17d02),
    (0xC17D04, i_c17d04),
    (0xC17D08, i_c17d08),
    (0xC17D0A, i_c17d0a),
    (0xC17D0E, i_c17d0e),
    (0xC17D10, i_c17d10),
    (0xC17D12, i_c17d12),
    (0xC17D16, i_c17d16),
    (0xC17D18, i_c17d18),
    (0xC17D1C, i_c17d1c),
    (0xC17D20, i_c17d20),
    (0xC17D22, i_c17d22),
    (0xC17D24, i_c17d24),
    (0xC17D28, i_c17d28),
    (0xC17D2A, i_c17d2a),
    (0xC17D2E, i_c17d2e),
    (0xC17D32, i_c17d32),
    (0xC17D36, i_c17d36),
    (0xC17D3A, i_c17d3a),
    (0xC17D3C, i_c17d3c),
    (0xC17D40, i_c17d40),
    (0xC17D42, i_c17d42),
    (0xC17D44, i_c17d44),
    (0xC17D48, i_c17d48),
    (0xC17D4C, i_c17d4c),
    (0xC17D4E, i_c17d4e),
    (0xC17D50, i_c17d50),
    (0xC17D52, i_c17d52),
    (0xC17D54, i_c17d54),
    (0xC17DEC, i_c17dec),
    (0xC17DF0, i_c17df0),
    (0xC17DF2, i_c17df2),
    (0xC17DF6, i_c17df6),
    (0xC17DFA, i_c17dfa),
    (0xC17DFC, i_c17dfc),
    (0xC17DFE, i_c17dfe),
    (0xC17E00, i_c17e00),
    (0xC17E02, i_c17e02),
    (0xC17E04, i_c17e04),
    (0xC17E06, i_c17e06),
    (0xC17E08, i_c17e08),
    (0xC17E0A, i_c17e0a),
    (0xC17E0E, i_c17e0e),
    (0xC17E10, i_c17e10),
    (0xC17E14, i_c17e14),
    (0xC17E16, i_c17e16),
    (0xC17E18, i_c17e18),
    (0xC17E1A, i_c17e1a),
    (0xC17E1E, i_c17e1e),
    (0xC17E20, i_c17e20),
    (0xC17E22, i_c17e22),
    (0xC17E24, i_c17e24),
    (0xC17E26, i_c17e26),
    (0xC17E2A, i_c17e2a),
    (0xC17E2E, i_c17e2e),
    (0xC17E34, i_c17e34),
    (0xC17E36, i_c17e36),
    (0xC17E38, i_c17e38),
    (0xC17E3A, i_c17e3a),
    (0xC17E3C, i_c17e3c),
    (0xC17E3E, i_c17e3e),
    (0xC17E40, i_c17e40),
    (0xC17E42, i_c17e42),
    (0xC17E46, i_c17e46),
    (0xC17E48, i_c17e48),
    (0xC17E4C, i_c17e4c),
    (0xC17E50, i_c17e50),
    (0xC17E54, i_c17e54),
    (0xC17E58, i_c17e58),
    (0xC17E5A, i_c17e5a),
    (0xC17E5E, i_c17e5e),
    (0xC17E62, i_c17e62),
    (0xC17E64, i_c17e64),
    (0xC17E7A, i_c17e7a),
    (0xC17E7E, i_c17e7e),
    (0xC17E80, i_c17e80),
    (0xC17E84, i_c17e84),
    (0xC17E86, i_c17e86),
    (0xC17E88, i_c17e88),
    (0xC17E8C, i_c17e8c),
    (0xC17E8E, i_c17e8e),
    (0xC17E90, i_c17e90),
    (0xC17E92, i_c17e92),
    (0xC17E94, i_c17e94),
    (0xC17E96, i_c17e96),
    (0xC17EAA, i_c17eaa),
    (0xC17EB0, i_c17eb0),
    (0xC17EB2, i_c17eb2),
    (0xC17EB6, i_c17eb6),
    (0xC17EBA, i_c17eba),
    (0xC17EBE, i_c17ebe),
    (0xC17EC2, i_c17ec2),
    (0xC17EC6, i_c17ec6),
    (0xC17ECA, i_c17eca),
    (0xC17ED0, i_c17ed0),
    (0xC17ED6, i_c17ed6),
    (0xC17EDA, i_c17eda),
    (0xC17EE0, i_c17ee0),
    (0xC17EE6, i_c17ee6),
    (0xC17EEA, i_c17eea),
    (0xC17EEE, i_c17eee),
    (0xC17EF0, i_c17ef0),
    (0xC17EF4, i_c17ef4),
    (0xC17EF6, i_c17ef6),
    (0xC17EFA, i_c17efa),
    (0xC17EFE, i_c17efe),
    (0xC17F00, i_c17f00),
    (0xC17F04, i_c17f04),
    (0xC17F06, i_c17f06),
    (0xC17F08, i_c17f08),
    (0xC17F0A, i_c17f0a),
    (0xC17F0C, i_c17f0c),
    (0xC17F0E, i_c17f0e),
    (0xC17F10, i_c17f10),
    (0xC17F12, i_c17f12),
    (0xC17F16, i_c17f16),
    (0xC17F18, i_c17f18),
    (0xC17F1A, i_c17f1a),
    (0xC17F1C, i_c17f1c),
    (0xC17F1E, i_c17f1e),
    (0xC17F22, i_c17f22),
    (0xC17F26, i_c17f26),
    (0xC17F28, i_c17f28),
    (0xC17F2A, i_c17f2a),
    (0xC17F2C, i_c17f2c),
    (0xC17F2E, i_c17f2e),
    (0xC17F30, i_c17f30),
    (0xC17F32, i_c17f32),
    (0xC17F34, i_c17f34),
    (0xC17F36, i_c17f36),
    (0xC17F38, i_c17f38),
    (0xC17F3A, i_c17f3a),
    (0xC17F3C, i_c17f3c),
    (0xC17F3E, i_c17f3e),
    (0xC17F40, i_c17f40),
    (0xC17F42, i_c17f42),
    (0xC17F46, i_c17f46),
    (0xC17F4A, i_c17f4a),
    (0xC17F4E, i_c17f4e),
    (0xC17F52, i_c17f52),
    (0xC17F56, i_c17f56),
    (0xC17F5A, i_c17f5a),
    (0xC17F60, i_c17f60),
    (0xC17F66, i_c17f66),
    (0xC17F6A, i_c17f6a),
    (0xC17F70, i_c17f70),
    (0xC17F76, i_c17f76),
    (0xC17F7A, i_c17f7a),
    (0xC17F7E, i_c17f7e),
    (0xC17F80, i_c17f80),
    (0xC17F82, i_c17f82),
    (0xC17F86, i_c17f86),
    (0xC17F88, i_c17f88),
    (0xC17F8C, i_c17f8c),
    (0xC17F90, i_c17f90),
    (0xC17F92, i_c17f92),
    (0xC17F96, i_c17f96),
    (0xC17F98, i_c17f98),
    (0xC17F9A, i_c17f9a),
    (0xC17F9C, i_c17f9c),
    (0xC17F9E, i_c17f9e),
    (0xC17FA0, i_c17fa0),
    (0xC17FA6, i_c17fa6),
    (0xC17FAA, i_c17faa),
    (0xC17FAC, i_c17fac),
    (0xC17FB0, i_c17fb0),
    (0xC17FB4, i_c17fb4),
    (0xC17FB6, i_c17fb6),
    (0xC17FB8, i_c17fb8),
    (0xC17FBA, i_c17fba),
    (0xC17FBC, i_c17fbc),
    (0xC17FBE, i_c17fbe),
    (0xC17FC0, i_c17fc0),
    (0xC17FC2, i_c17fc2),
    (0xC17FC4, i_c17fc4),
    (0xC17FC6, i_c17fc6),
    (0xC17FC8, i_c17fc8),
    (0xC17FCA, i_c17fca),
    (0xC17FCC, i_c17fcc),
    (0xC17FCE, i_c17fce),
    (0xC17FD2, i_c17fd2),
    (0xC17FD4, i_c17fd4),
    (0xC17FD6, i_c17fd6),
    (0xC17FDA, i_c17fda),
    (0xC17FDC, i_c17fdc),
    (0xC17FDE, i_c17fde),
    (0xC17FE0, i_c17fe0),
    (0xC17FE2, i_c17fe2),
    (0xC17FE4, i_c17fe4),
    (0xC17FE6, i_c17fe6),
    (0xC17FE8, i_c17fe8),
    (0xC17FEA, i_c17fea),
    (0xC17FEE, i_c17fee),
    (0xC17FF0, i_c17ff0),
    (0xC17FF2, i_c17ff2),
    (0xC17FF4, i_c17ff4),
    (0xC17FF8, i_c17ff8),
    (0xC18002, i_c18002),
    (0xC18006, i_c18006),
    (0xC1800A, i_c1800a),
    (0xC18010, i_c18010),
    (0xC18014, i_c18014),
    (0xC18018, i_c18018),
    (0xC1801E, i_c1801e),
    (0xC18020, i_c18020),
    (0xC18024, i_c18024),
    (0xC18026, i_c18026),
    (0xC1802A, i_c1802a),
    (0xC1802E, i_c1802e),
    (0xC18036, i_c18036),
    (0xC1803A, i_c1803a),
    (0xC1803E, i_c1803e),
    (0xC18042, i_c18042),
    (0xC18046, i_c18046),
    (0xC1804A, i_c1804a),
    (0xC1804E, i_c1804e),
    (0xC18052, i_c18052),
    (0xC18054, i_c18054),
    (0xC18056, i_c18056),
    (0xC1805A, i_c1805a),
    (0xC1805E, i_c1805e),
    (0xC18062, i_c18062),
    (0xC18064, i_c18064),
    (0xC18068, i_c18068),
    (0xC1806A, i_c1806a),
    (0xC1806C, i_c1806c),
    (0xC18070, i_c18070),
    (0xC18072, i_c18072),
    (0xC18076, i_c18076),
    (0xC1807A, i_c1807a),
    (0xC1807E, i_c1807e),
    (0xC18082, i_c18082),
    (0xC18084, i_c18084),
    (0xC18086, i_c18086),
    (0xC1808A, i_c1808a),
    (0xC1808E, i_c1808e),
    (0xC18092, i_c18092),
    (0xC18096, i_c18096),
    (0xC18098, i_c18098),
    (0xC1809A, i_c1809a),
    (0xC1809C, i_c1809c),
    (0xC180A0, i_c180a0),
    (0xC180A2, i_c180a2),
    (0xC180A4, i_c180a4),
    (0xC180A8, i_c180a8),
    (0xC180AA, i_c180aa),
    (0xC180AE, i_c180ae),
    (0xC180B2, i_c180b2),
    (0xC180B6, i_c180b6),
    (0xC180BA, i_c180ba),
    (0xC180BC, i_c180bc),
    (0xC180C0, i_c180c0),
    (0xC180C2, i_c180c2),
    (0xC180C6, i_c180c6),
    (0xC180CA, i_c180ca),
    (0xC180CC, i_c180cc),
    (0xC180D0, i_c180d0),
    (0xC180D4, i_c180d4),
    (0xC180D6, i_c180d6),
    (0xC180DC, i_c180dc),
    (0xC180E0, i_c180e0),
    (0xC180E4, i_c180e4),
    (0xC180E8, i_c180e8),
    (0xC180EA, i_c180ea),
    (0xC180EE, i_c180ee),
    (0xC180F2, i_c180f2),
    (0xC180F4, i_c180f4),
    (0xC180F8, i_c180f8),
    (0xC180FA, i_c180fa),
    (0xC180FC, i_c180fc),
    (0xC18100, i_c18100),
    (0xC18104, i_c18104),
    (0xC18106, i_c18106),
    (0xC1810C, i_c1810c),
    (0xC18110, i_c18110),
    (0xC18114, i_c18114),
    (0xC1811A, i_c1811a),
    (0xC18122, i_c18122),
    (0xC18126, i_c18126),
    (0xC1812A, i_c1812a),
    (0xC1812E, i_c1812e),
    (0xC18132, i_c18132),
    (0xC18136, i_c18136),
    (0xC1813A, i_c1813a),
    (0xC1813E, i_c1813e),
    (0xC18142, i_c18142),
    (0xC18146, i_c18146),
    (0xC18148, i_c18148),
    (0xC1814E, i_c1814e),
    (0xC18154, i_c18154),
    (0xC1815A, i_c1815a),
    (0xC18160, i_c18160),
    (0xC18166, i_c18166),
    (0xC18168, i_c18168),
    (0xC1816C, i_c1816c),
    (0xC18172, i_c18172),
    (0xC18176, i_c18176),
    (0xC1817A, i_c1817a),
    (0xC1817E, i_c1817e),
    (0xC18184, i_c18184),
    (0xC18188, i_c18188),
    (0xC1818A, i_c1818a),
    (0xC1818C, i_c1818c),
    (0xC1818E, i_c1818e),
    (0xC18190, i_c18190),
    (0xC18194, i_c18194),
    (0xC18196, i_c18196),
    (0xC18198, i_c18198),
    (0xC1819A, i_c1819a),
    (0xC1819C, i_c1819c),
    (0xC1819E, i_c1819e),
    (0xC181A2, i_c181a2),
    (0xC181A6, i_c181a6),
    (0xC181AA, i_c181aa),
    (0xC181AC, i_c181ac),
    (0xC181AE, i_c181ae),
    (0xC181B0, i_c181b0),
    (0xC181B2, i_c181b2),
    (0xC181B6, i_c181b6),
    (0xC181B8, i_c181b8),
    (0xC181BA, i_c181ba),
    (0xC181BC, i_c181bc),
    (0xC181BE, i_c181be),
    (0xC181C0, i_c181c0),
    (0xC181C4, i_c181c4),
    (0xC181C8, i_c181c8),
    (0xC181CC, i_c181cc),
    (0xC181CE, i_c181ce),
    (0xC181D0, i_c181d0),
    (0xC181D2, i_c181d2),
    (0xC181D4, i_c181d4),
    (0xC181D8, i_c181d8),
    (0xC181DC, i_c181dc),
    (0xC181DE, i_c181de),
    (0xC181E0, i_c181e0),
    (0xC181E2, i_c181e2),
    (0xC181E4, i_c181e4),
    (0xC181E6, i_c181e6),
    (0xC181EA, i_c181ea),
    (0xC181EE, i_c181ee),
    (0xC181F2, i_c181f2),
    (0xC181F6, i_c181f6),
    (0xC181F8, i_c181f8),
    (0xC181FA, i_c181fa),
    (0xC181FC, i_c181fc),
    (0xC181FE, i_c181fe),
    (0xC18200, i_c18200),
    (0xC18204, i_c18204),
    (0xC1820A, i_c1820a),
    (0xC1820C, i_c1820c),
    (0xC18212, i_c18212),
    (0xC18218, i_c18218),
    (0xC1821A, i_c1821a),
    (0xC18220, i_c18220),
    (0xC18226, i_c18226),
    (0xC1822C, i_c1822c),
    (0xC18232, i_c18232),
    (0xC18238, i_c18238),
    (0xC1823C, i_c1823c),
    (0xC18242, i_c18242),
    (0xC18246, i_c18246),
    (0xC1824C, i_c1824c),
    (0xC18252, i_c18252),
    (0xC18258, i_c18258),
    (0xC1825C, i_c1825c),
    (0xC18262, i_c18262),
    (0xC18268, i_c18268),
    (0xC18270, i_c18270),
    (0xC18274, i_c18274),
    (0xC1827A, i_c1827a),
    (0xC1827C, i_c1827c),
    (0xC18280, i_c18280),
    (0xC18284, i_c18284),
    (0xC18288, i_c18288),
    (0xC1828A, i_c1828a),
    (0xC1828E, i_c1828e),
    (0xC18292, i_c18292),
    (0xC18296, i_c18296),
    (0xC1829A, i_c1829a),
    (0xC1829E, i_c1829e),
    (0xC182A0, i_c182a0),
    (0xC182A4, i_c182a4),
    (0xC182AA, i_c182aa),
    (0xC182AC, i_c182ac),
    (0xC182B0, i_c182b0),
    (0xC182B2, i_c182b2),
    (0xC182B4, i_c182b4),
    (0xC182B6, i_c182b6),
    (0xC182B8, i_c182b8),
    (0xC182BA, i_c182ba),
    (0xC182BE, i_c182be),
    (0xC182C2, i_c182c2),
    (0xC182C8, i_c182c8),
    (0xC182CA, i_c182ca),
    (0xC182CE, i_c182ce),
    (0xC182D0, i_c182d0),
    (0xC182D2, i_c182d2),
    (0xC182D6, i_c182d6),
    (0xC182DA, i_c182da),
    (0xC182DE, i_c182de),
    (0xC182E0, i_c182e0),
    (0xC182E2, i_c182e2),
    (0xC182E6, i_c182e6),
    (0xC182EA, i_c182ea),
    (0xC182F0, i_c182f0),
    (0xC182F4, i_c182f4),
    (0xC182F8, i_c182f8),
    (0xC182FC, i_c182fc),
    (0xC18402, i_c18402),
    (0xC18408, i_c18408),
    (0xC1840C, i_c1840c),
    (0xC18410, i_c18410),
    (0xC18414, i_c18414),
    (0xC18418, i_c18418),
    (0xC1841C, i_c1841c),
    (0xC18422, i_c18422),
    (0xC18426, i_c18426),
    (0xC1842A, i_c1842a),
    (0xC18430, i_c18430),
    (0xC18434, i_c18434),
    (0xC18436, i_c18436),
    (0xC1843C, i_c1843c),
    (0xC18440, i_c18440),
    (0xC18444, i_c18444),
    (0xC18448, i_c18448),
    (0xC1844C, i_c1844c),
    (0xC1844E, i_c1844e),
    (0xC18450, i_c18450),
    (0xC18454, i_c18454),
    (0xC18458, i_c18458),
    (0xC1845C, i_c1845c),
    (0xC18460, i_c18460),
    (0xC18466, i_c18466),
    (0xC1846A, i_c1846a),
    (0xC18470, i_c18470),
    (0xC18474, i_c18474),
    (0xC18478, i_c18478),
    (0xC1847E, i_c1847e),
    (0xC18480, i_c18480),
    (0xC18484, i_c18484),
    (0xC18488, i_c18488),
    (0xC1848C, i_c1848c),
    (0xC18490, i_c18490),
    (0xC18494, i_c18494),
    (0xC1849A, i_c1849a),
    (0xC1849C, i_c1849c),
    (0xC1849E, i_c1849e),
    (0xC184A4, i_c184a4),
    (0xC184A6, i_c184a6),
    (0xC184AA, i_c184aa),
    (0xC184AC, i_c184ac),
    (0xC184B0, i_c184b0),
    (0xC184B4, i_c184b4),
    (0xC184B8, i_c184b8),
    (0xC184BC, i_c184bc),
    (0xC184BE, i_c184be),
    (0xC184C2, i_c184c2),
    (0xC184C6, i_c184c6),
    (0xC184CA, i_c184ca),
    (0xC184CC, i_c184cc),
    (0xC184D0, i_c184d0),
    (0xC184D2, i_c184d2),
    (0xC184D4, i_c184d4),
    (0xC184D8, i_c184d8),
    (0xC184DA, i_c184da),
    (0xC184DE, i_c184de),
    (0xC184E0, i_c184e0),
    (0xC184E4, i_c184e4),
    (0xC184E6, i_c184e6),
    (0xC184E8, i_c184e8),
    (0xC184EA, i_c184ea),
    (0xC184EC, i_c184ec),
    (0xC184F0, i_c184f0),
    (0xC184F4, i_c184f4),
    (0xC184F8, i_c184f8),
    (0xC184FC, i_c184fc),
    (0xC184FE, i_c184fe),
    (0xC18500, i_c18500),
    (0xC18504, i_c18504),
    (0xC18508, i_c18508),
    (0xC1850C, i_c1850c),
    (0xC1850E, i_c1850e),
    (0xC18512, i_c18512),
    (0xC18514, i_c18514),
    (0xC18518, i_c18518),
    (0xC1851A, i_c1851a),
    (0xC1851C, i_c1851c),
    (0xC1851E, i_c1851e),
    (0xC18520, i_c18520),
    (0xC18522, i_c18522),
    (0xC18524, i_c18524),
    (0xC18526, i_c18526),
    (0xC18528, i_c18528),
    (0xC1852A, i_c1852a),
    (0xC1852E, i_c1852e),
    (0xC18532, i_c18532),
    (0xC18534, i_c18534),
    (0xC18538, i_c18538),
    (0xC1853A, i_c1853a),
    (0xC1853C, i_c1853c),
    (0xC18540, i_c18540),
    (0xC18542, i_c18542),
    (0xC18544, i_c18544),
    (0xC1854A, i_c1854a),
    (0xC1854E, i_c1854e),
    (0xC18550, i_c18550),
    (0xC18554, i_c18554),
    (0xC18556, i_c18556),
    (0xC1855A, i_c1855a),
    (0xC1855C, i_c1855c),
    (0xC18560, i_c18560),
    (0xC18562, i_c18562),
    (0xC18566, i_c18566),
    (0xC18568, i_c18568),
    (0xC1856A, i_c1856a),
    (0xC18570, i_c18570),
    (0xC18576, i_c18576),
    (0xC18578, i_c18578),
    (0xC1857C, i_c1857c),
    (0xC1857E, i_c1857e),
    (0xC18580, i_c18580),
    (0xC18584, i_c18584),
    (0xC18586, i_c18586),
    (0xC18588, i_c18588),
    (0xC1858E, i_c1858e),
    (0xC18590, i_c18590),
    (0xC18596, i_c18596),
    (0xC18598, i_c18598),
    (0xC1859A, i_c1859a),
    (0xC1859C, i_c1859c),
    (0xC1859E, i_c1859e),
    (0xC185A0, i_c185a0),
    (0xC185A2, i_c185a2),
    (0xC185A4, i_c185a4),
    (0xC185A6, i_c185a6),
    (0xC185A8, i_c185a8),
    (0xC185AA, i_c185aa),
    (0xC185AC, i_c185ac),
    (0xC185AE, i_c185ae),
    (0xC185B0, i_c185b0),
    (0xC185B2, i_c185b2),
    (0xC185B4, i_c185b4),
    (0xC185B6, i_c185b6),
    (0xC185B8, i_c185b8),
    (0xC185BA, i_c185ba),
    (0xC185BC, i_c185bc),
    (0xC185BE, i_c185be),
    (0xC185C0, i_c185c0),
    (0xC185C4, i_c185c4),
    (0xC185C6, i_c185c6),
    (0xC185C8, i_c185c8),
    (0xC185CA, i_c185ca),
    (0xC185CC, i_c185cc),
    (0xC185CE, i_c185ce),
    (0xC185D2, i_c185d2),
    (0xC185D4, i_c185d4),
    (0xC185D6, i_c185d6),
    (0xC185D8, i_c185d8),
    (0xC185DA, i_c185da),
    (0xC185DC, i_c185dc),
    (0xC185E0, i_c185e0),
    (0xC185E2, i_c185e2),
    (0xC185E4, i_c185e4),
    (0xC185E6, i_c185e6),
    (0xC185E8, i_c185e8),
    (0xC185EA, i_c185ea),
    (0xC185EE, i_c185ee),
    (0xC185F2, i_c185f2),
    (0xC185F4, i_c185f4),
    (0xC185F8, i_c185f8),
    (0xC185FA, i_c185fa),
    (0xC185FC, i_c185fc),
    (0xC18600, i_c18600),
    (0xC18604, i_c18604),
    (0xC18606, i_c18606),
    (0xC18608, i_c18608),
    (0xC1860A, i_c1860a),
    (0xC1860C, i_c1860c),
    (0xC1860E, i_c1860e),
    (0xC18610, i_c18610),
    (0xC18612, i_c18612),
    (0xC18614, i_c18614),
    (0xC18616, i_c18616),
    (0xC18686, i_c18686),
    (0xC1868A, i_c1868a),
    (0xC18694, i_c18694),
    (0xC18698, i_c18698),
    (0xC1869C, i_c1869c),
    (0xC1869E, i_c1869e),
    (0xC186A2, i_c186a2),
    (0xC186A6, i_c186a6),
    (0xC186AC, i_c186ac),
    (0xC186B0, i_c186b0),
    (0xC186B6, i_c186b6),
    (0xC186B8, i_c186b8),
    (0xC186BC, i_c186bc),
    (0xC186C0, i_c186c0),
    (0xC186C4, i_c186c4),
    (0xC186C8, i_c186c8),
    (0xC186CC, i_c186cc),
    (0xC186D0, i_c186d0),
    (0xC186D4, i_c186d4),
    (0xC186D8, i_c186d8),
    (0xC186DC, i_c186dc),
    (0xC186E0, i_c186e0),
    (0xC186E4, i_c186e4),
    (0xC186E8, i_c186e8),
    (0xC186EC, i_c186ec),
    (0xC186F0, i_c186f0),
    (0xC186F4, i_c186f4),
    (0xC186F8, i_c186f8),
    (0xC186FC, i_c186fc),
    (0xC18700, i_c18700),
    (0xC18704, i_c18704),
    (0xC18708, i_c18708),
    (0xC1870C, i_c1870c),
    (0xC1870E, i_c1870e),
    (0xC18712, i_c18712),
    (0xC18716, i_c18716),
    (0xC1871A, i_c1871a),
    (0xC1871E, i_c1871e),
    (0xC1872E, i_c1872e),
    (0xC18732, i_c18732),
    (0xC18738, i_c18738),
    (0xC1873E, i_c1873e),
    (0xC18744, i_c18744),
    (0xC18746, i_c18746),
    (0xC1874A, i_c1874a),
    (0xC1874E, i_c1874e),
    (0xC18750, i_c18750),
    (0xC18754, i_c18754),
    (0xC18758, i_c18758),
    (0xC1875C, i_c1875c),
    (0xC18760, i_c18760),
    (0xC18764, i_c18764),
    (0xC18768, i_c18768),
    (0xC1876C, i_c1876c),
    (0xC18770, i_c18770),
    (0xC18774, i_c18774),
    (0xC18778, i_c18778),
    (0xC1877C, i_c1877c),
    (0xC18780, i_c18780),
    (0xC18784, i_c18784),
    (0xC18788, i_c18788),
    (0xC1878C, i_c1878c),
    (0xC18790, i_c18790),
    (0xC18794, i_c18794),
    (0xC18798, i_c18798),
    (0xC1879C, i_c1879c),
    (0xC187A0, i_c187a0),
    (0xC187A4, i_c187a4),
    (0xC187A6, i_c187a6),
    (0xC187AA, i_c187aa),
    (0xC187AE, i_c187ae),
    (0xC187B4, i_c187b4),
    (0xC187BA, i_c187ba),
    (0xC187BE, i_c187be),
    (0xC187C0, i_c187c0),
    (0xC187C4, i_c187c4),
    (0xC187C8, i_c187c8),
    (0xC187CE, i_c187ce),
    (0xC187D0, i_c187d0),
    (0xC187D2, i_c187d2),
    (0xC187D8, i_c187d8),
    (0xC187DC, i_c187dc),
    (0xC187E0, i_c187e0),
    (0xC187E4, i_c187e4),
    (0xC187E8, i_c187e8),
    (0xC187EC, i_c187ec),
    (0xC187F0, i_c187f0),
    (0xC187F2, i_c187f2),
    (0xC187F6, i_c187f6),
    (0xC187F8, i_c187f8),
    (0xC187FA, i_c187fa),
    (0xC187FE, i_c187fe),
    (0xC18800, i_c18800),
    (0xC18804, i_c18804),
    (0xC18806, i_c18806),
    (0xC1880A, i_c1880a),
    (0xC1880E, i_c1880e),
    (0xC18810, i_c18810),
    (0xC18812, i_c18812),
    (0xC18816, i_c18816),
    (0xC1881A, i_c1881a),
    (0xC1881E, i_c1881e),
    (0xC18820, i_c18820),
    (0xC18822, i_c18822),
    (0xC18826, i_c18826),
    (0xC18828, i_c18828),
    (0xC1882C, i_c1882c),
    (0xC1882E, i_c1882e),
    (0xC18832, i_c18832),
    (0xC18836, i_c18836),
    (0xC1883A, i_c1883a),
    (0xC1883E, i_c1883e),
    (0xC18840, i_c18840),
    (0xC18842, i_c18842),
    (0xC18844, i_c18844),
    (0xC18846, i_c18846),
    (0xC1884A, i_c1884a),
    (0xC1884C, i_c1884c),
    (0xC18850, i_c18850),
    (0xC18852, i_c18852),
    (0xC18856, i_c18856),
    (0xC18858, i_c18858),
    (0xC1885C, i_c1885c),
    (0xC18860, i_c18860),
    (0xC18864, i_c18864),
    (0xC18868, i_c18868),
    (0xC1886C, i_c1886c),
    (0xC1886E, i_c1886e),
    (0xC18872, i_c18872),
    (0xC18876, i_c18876),
    (0xC18878, i_c18878),
    (0xC1887C, i_c1887c),
    (0xC18880, i_c18880),
    (0xC18882, i_c18882),
    (0xC18886, i_c18886),
    (0xC1888A, i_c1888a),
    (0xC1888E, i_c1888e),
    (0xC18892, i_c18892),
    (0xC18896, i_c18896),
    (0xC1889A, i_c1889a),
    (0xC1889E, i_c1889e),
    (0xC188A2, i_c188a2),
    (0xC188A6, i_c188a6),
    (0xC188AA, i_c188aa),
    (0xC188AE, i_c188ae),
    (0xC188B2, i_c188b2),
    (0xC188B6, i_c188b6),
    (0xC188BA, i_c188ba),
    (0xC188BE, i_c188be),
    (0xC188C2, i_c188c2),
    (0xC188C6, i_c188c6),
    (0xC188CA, i_c188ca),
    (0xC188CC, i_c188cc),
    (0xC188D0, i_c188d0),
    (0xC188D4, i_c188d4),
    (0xC188D6, i_c188d6),
    (0xC188DA, i_c188da),
    (0xC188DE, i_c188de),
    (0xC188E0, i_c188e0),
    (0xC188E2, i_c188e2),
    (0xC188E4, i_c188e4),
    (0xC188E6, i_c188e6),
    (0xC188E8, i_c188e8),
    (0xC188EA, i_c188ea),
    (0xC188EC, i_c188ec),
    (0xC188EE, i_c188ee),
    (0xC188F0, i_c188f0),
    (0xC1898A, i_c1898a),
    (0xC1898E, i_c1898e),
    (0xC18994, i_c18994),
    (0xC18996, i_c18996),
    (0xC1899A, i_c1899a),
    (0xC1899C, i_c1899c),
    (0xC1899E, i_c1899e),
    (0xC189A2, i_c189a2),
    (0xC189A8, i_c189a8),
    (0xC189AA, i_c189aa),
    (0xC189AC, i_c189ac),
    (0xC189B0, i_c189b0),
    (0xC189B4, i_c189b4),
    (0xC189B6, i_c189b6),
    (0xC189B8, i_c189b8),
    (0xC189BA, i_c189ba),
    (0xC189BC, i_c189bc),
    (0xC189BE, i_c189be),
    (0xC189C2, i_c189c2),
    (0xC189C4, i_c189c4),
    (0xC189C6, i_c189c6),
    (0xC189CA, i_c189ca),
    (0xC189CE, i_c189ce),
    (0xC189D2, i_c189d2),
    (0xC189D6, i_c189d6),
    (0xC189D8, i_c189d8),
    (0xC189DA, i_c189da),
    (0xC189DE, i_c189de),
    (0xC189E0, i_c189e0),
    (0xC189E4, i_c189e4),
    (0xC189E6, i_c189e6),
    (0xC189EA, i_c189ea),
    (0xC189EE, i_c189ee),
    (0xC189F2, i_c189f2),
    (0xC189F6, i_c189f6),
    (0xC189FA, i_c189fa),
    (0xC189FE, i_c189fe),
    (0xC18A04, i_c18a04),
    (0xC18A08, i_c18a08),
    (0xC18A0E, i_c18a0e),
    (0xC18A10, i_c18a10),
    (0xC18A12, i_c18a12),
    (0xC18A18, i_c18a18),
    (0xC18A1C, i_c18a1c),
    (0xC18A20, i_c18a20),
    (0xC18A24, i_c18a24),
    (0xC18A28, i_c18a28),
    (0xC18A2C, i_c18a2c),
    (0xC18A30, i_c18a30),
    (0xC18A34, i_c18a34),
    (0xC18A36, i_c18a36),
    (0xC18A3A, i_c18a3a),
    (0xC18A3C, i_c18a3c),
    (0xC18A3E, i_c18a3e),
    (0xC18A40, i_c18a40),
    (0xC18A42, i_c18a42),
    (0xC18A44, i_c18a44),
    (0xC18A48, i_c18a48),
    (0xC18A4A, i_c18a4a),
    (0xC18A4C, i_c18a4c),
    (0xC18A4E, i_c18a4e),
    (0xC18A52, i_c18a52),
    (0xC18A56, i_c18a56),
    (0xC18A5C, i_c18a5c),
    (0xC18A60, i_c18a60),
    (0xC18A64, i_c18a64),
    (0xC18A68, i_c18a68),
    (0xC18A6C, i_c18a6c),
    (0xC18A70, i_c18a70),
    (0xC18A74, i_c18a74),
    (0xC18A76, i_c18a76),
    (0xC18A7A, i_c18a7a),
    (0xC18A7C, i_c18a7c),
    (0xC18A80, i_c18a80),
    (0xC18A84, i_c18a84),
    (0xC18A88, i_c18a88),
    (0xC18A8A, i_c18a8a),
    (0xC18A8C, i_c18a8c),
    (0xC18A90, i_c18a90),
    (0xC18A92, i_c18a92),
    (0xC18A94, i_c18a94),
    (0xC18A98, i_c18a98),
    (0xC18A9C, i_c18a9c),
    (0xC18A9E, i_c18a9e),
    (0xC18AA2, i_c18aa2),
    (0xC18AA4, i_c18aa4),
    (0xC18AA8, i_c18aa8),
    (0xC18AAC, i_c18aac),
    (0xC18AAE, i_c18aae),
    (0xC18AB2, i_c18ab2),
    (0xC18AB6, i_c18ab6),
    (0xC18ABC, i_c18abc),
    (0xC18AC0, i_c18ac0),
    (0xC18AC4, i_c18ac4),
    (0xC18AC6, i_c18ac6),
    (0xC18ACA, i_c18aca),
    (0xC18ACE, i_c18ace),
    (0xC18AD0, i_c18ad0),
    (0xC18AD4, i_c18ad4),
    (0xC18AD8, i_c18ad8),
    (0xC18ADA, i_c18ada),
    (0xC18ADC, i_c18adc),
    (0xC18ADE, i_c18ade),
    (0xC18AE2, i_c18ae2),
    (0xC18AE4, i_c18ae4),
    (0xC18AE8, i_c18ae8),
    (0xC18AEA, i_c18aea),
    (0xC18AEC, i_c18aec),
    (0xC18AF0, i_c18af0),
    (0xC18AF4, i_c18af4),
    (0xC18AF6, i_c18af6),
    (0xC18AF8, i_c18af8),
    (0xC18AFA, i_c18afa),
    (0xC18AFC, i_c18afc),
    (0xC18AFE, i_c18afe),
    (0xC18B00, i_c18b00),
    (0xC18B02, i_c18b02),
    (0xC18B04, i_c18b04),
    (0xC18B06, i_c18b06),
    (0xC18B08, i_c18b08),
    (0xC18B0A, i_c18b0a),
    (0xC18B0C, i_c18b0c),
    (0xC18B0E, i_c18b0e),
    (0xC18B10, i_c18b10),
    (0xC18B14, i_c18b14),
    (0xC18B18, i_c18b18),
    (0xC18B1C, i_c18b1c),
    (0xC18B20, i_c18b20),
    (0xC18B24, i_c18b24),
    (0xC18B28, i_c18b28),
    (0xC18B2E, i_c18b2e),
    (0xC18B32, i_c18b32),
    (0xC18B36, i_c18b36),
    (0xC18B3A, i_c18b3a),
    (0xC18B40, i_c18b40),
    (0xC18B44, i_c18b44),
    (0xC18B48, i_c18b48),
    (0xC18B4E, i_c18b4e),
    (0xC18B52, i_c18b52),
    (0xC18B58, i_c18b58),
    (0xC18B5A, i_c18b5a),
    (0xC18B5E, i_c18b5e),
    (0xC18B62, i_c18b62),
    (0xC18B66, i_c18b66),
    (0xC18B6A, i_c18b6a),
    (0xC18B6E, i_c18b6e),
    (0xC18B72, i_c18b72),
    (0xC18B76, i_c18b76),
    (0xC18B78, i_c18b78),
    (0xC18B7A, i_c18b7a),
    (0xC18B7E, i_c18b7e),
    (0xC18B82, i_c18b82),
    (0xC18B84, i_c18b84),
    (0xC18B88, i_c18b88),
    (0xC18B8A, i_c18b8a),
    (0xC18B8C, i_c18b8c),
    (0xC18B90, i_c18b90),
    (0xC18B94, i_c18b94),
    (0xC18B96, i_c18b96),
    (0xC18B9A, i_c18b9a),
    (0xC18B9C, i_c18b9c),
    (0xC18BA0, i_c18ba0),
    (0xC18BA4, i_c18ba4),
    (0xC18BA6, i_c18ba6),
    (0xC18BA8, i_c18ba8),
    (0xC18BAC, i_c18bac),
    (0xC18BAE, i_c18bae),
    (0xC18BB0, i_c18bb0),
    (0xC18BB2, i_c18bb2),
    (0xC18BB4, i_c18bb4),
    (0xC18BB6, i_c18bb6),
    (0xC18BB8, i_c18bb8),
    (0xC18BBA, i_c18bba),
    (0xC18BBC, i_c18bbc),
    (0xC18BBE, i_c18bbe),
    (0xC18BC2, i_c18bc2),
    (0xC18BC6, i_c18bc6),
    (0xC18BCA, i_c18bca),
    (0xC18BCE, i_c18bce),
    (0xC18BD2, i_c18bd2),
    (0xC18BD6, i_c18bd6),
    (0xC18BDC, i_c18bdc),
    (0xC18BE0, i_c18be0),
    (0xC18BE6, i_c18be6),
    (0xC18BE8, i_c18be8),
    (0xC18BEC, i_c18bec),
    (0xC18BEE, i_c18bee),
    (0xC18BF0, i_c18bf0),
    (0xC18BF4, i_c18bf4),
    (0xC18BF8, i_c18bf8),
    (0xC18BFA, i_c18bfa),
    (0xC18BFC, i_c18bfc),
    (0xC18C02, i_c18c02),
    (0xC18C04, i_c18c04),
    (0xC18C06, i_c18c06),
    (0xC18C08, i_c18c08),
    (0xC18C0A, i_c18c0a),
    (0xC18C0C, i_c18c0c),
    (0xC18C0E, i_c18c0e),
    (0xC18C12, i_c18c12),
    (0xC18C14, i_c18c14),
    (0xC18C16, i_c18c16),
    (0xC18C18, i_c18c18),
    (0xC18C1C, i_c18c1c),
    (0xC18C1E, i_c18c1e),
    (0xC18C22, i_c18c22),
    (0xC18C26, i_c18c26),
    (0xC18C2A, i_c18c2a),
    (0xC18C2C, i_c18c2c),
    (0xC18C30, i_c18c30),
    (0xC18C34, i_c18c34),
    (0xC18C36, i_c18c36),
    (0xC18C3A, i_c18c3a),
    (0xC18C3E, i_c18c3e),
    (0xC18C42, i_c18c42),
    (0xC18C44, i_c18c44),
    (0xC18C48, i_c18c48),
    (0xC18C4A, i_c18c4a),
    (0xC18C4E, i_c18c4e),
    (0xC18C52, i_c18c52),
    (0xC18C54, i_c18c54),
    (0xC18C58, i_c18c58),
    (0xC18C5A, i_c18c5a),
    (0xC18C5C, i_c18c5c),
    (0xC18C60, i_c18c60),
    (0xC18C64, i_c18c64),
    (0xC18C66, i_c18c66),
    (0xC18C6A, i_c18c6a),
    (0xC18C6C, i_c18c6c),
    (0xC18C72, i_c18c72),
    (0xC18C74, i_c18c74),
    (0xC18C78, i_c18c78),
];
