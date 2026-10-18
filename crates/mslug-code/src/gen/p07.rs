// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== Platform_FindNearestX_07700c  ($07700C, 112 B, props/fx_explosions_platforms_crew_076000.s)
/// $07700C: clr.w d7
pub fn i_07700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7700E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07700E: moveq #$ff, d6
pub fn i_07700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77010;
    cpu.ir = 0x7CFF;
    cpu.d[6] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077010: move.w d6, d5
pub fn i_077010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77012;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077012: lea.l (a0), a5
pub fn i_077012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77014;
    cpu.ir = 0x4BD0;
    cpu.a[5] = cpu.a[0];
    12
}
/// $077014: cmp.w $30(a0), d7
pub fn i_077014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77018;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $077018: bcc.w $77054
pub fn i_077018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7701C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77054; return 10; }
    8
}
/// $07701C: move.w $2(a5, d7.w), d2
pub fn i_07701c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77020;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $077020: sub.w d1, d2
pub fn i_077020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77022;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077022: clr.w d3
pub fn i_077022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77024;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077024: move.b $4(a5, d7.w), d3
pub fn i_077024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77028;
    cpu.ir = 0x1635;
    let ad2 = cpu.a[5].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    14
}
/// $077028: cmp.w d3, d2
pub fn i_077028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7702A;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $07702A: bhi.w $77050
pub fn i_07702a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7702E;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x77050; return 10; }
    8
}
/// $07702E: lsr.w #$3, d2
pub fn i_07702e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77030;
    cpu.ir = 0xE64A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077030: neg.w d2
pub fn i_077030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77032;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077032: add.w (a5, d7.w), d2
pub fn i_077032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77036;
    cpu.ir = 0xD475;
    let ad2 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $077036: move.w d2, d3
pub fn i_077036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77038;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077038: sub.w d0, d3
pub fn i_077038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703A;
    cpu.ir = 0x9640;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07703A: smi.b d4
pub fn i_07703a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703C;
    cpu.ir = 0x5BC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07703C: ext.w d4
pub fn i_07703c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703E;
    cpu.ir = 0x4884;
    let v = cpu.d[4] as i8 as i16 as u16 as u32; cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07703E: eor.w d4, d3
pub fn i_07703e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77040;
    cpu.ir = 0xB943;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077040: sub.w d4, d3
pub fn i_077040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77042;
    cpu.ir = 0x9644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077042: cmp.w d3, d5
pub fn i_077042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77044;
    cpu.ir = 0xBA43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $077044: bls.w $77050
pub fn i_077044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77048;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x77050; return 10; }
    8
}
/// $077048: move.w d3, d5
pub fn i_077048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7704A;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07704A: move.w d2, d6
pub fn i_07704a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7704C;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07704C: lea.l (a5, d7.w), a1
pub fn i_07704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77050;
    cpu.ir = 0x43F5;
    let ad1 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $077050: addq.w #$6, d7
pub fn i_077050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77052;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077052: bra.b $77014
pub fn i_077052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77054;
    cpu.ir = 0x60C0;
    cpu.pc = 0x77014;
    10
}
/// $077054: rts
pub fn i_077054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77056;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ListsInit_07707c  ($07707C, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $07707C: lea.l $10e27c.l, a0
pub fn i_07707c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77082;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $077082: clr.w $30(a0)
pub fn i_077082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77086;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $077086: lea.l $10e2ae.l, a0
pub fn i_077086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7708C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $07708C: clr.w $30(a0)
pub fn i_07708c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77090;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $077090: rts
pub fn i_077090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77092;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ClearListByScroll_077092  ($077092, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $077092: lea.l $10e27c.l, a0
pub fn i_077092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77098;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $077098: btst.b #$0, $106f28.l
pub fn i_077098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770A0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0770A0: beq.w $770aa
pub fn i_0770a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x770AA; return 10; }
    8
}
/// $0770A4: lea.l $10e2ae.l, a0
pub fn i_0770a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0770AA: clr.w $30(a0)
pub fn i_0770aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770AE;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0770AE: rts
pub fn i_0770ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ProbeUnderFeet_0770cc  ($0770CC, 120 B, props/fx_explosions_platforms_crew_076000.s)
/// $0770CC: lea.l $10e27c.l, a0
pub fn i_0770cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $0770D2: btst.b #$0, $106f28.l
pub fn i_0770d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770DA;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0770DA: bne.w $770e4
pub fn i_0770da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x770E4; return 10; }
    8
}
/// $0770DE: lea.l $10e2ae.l, a0
pub fn i_0770de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0770E4: move.w $22(a6), d0
pub fn i_0770e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770E8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0770E8: move.w $24(a6), d1
pub fn i_0770e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770EC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0770EC: add.w $106f50.l, d0
pub fn i_0770ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F2;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0770F2: subi.w #$200, d1
pub fn i_0770f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F6;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0770F6: neg.w d1
pub fn i_0770f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F8;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0770F8: add.w $106f54.l, d1
pub fn i_0770f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770FE;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0770FE: bsr.w $7700c
pub fn i_0770fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77102;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77102);
    cpu.pc = 0x7700C;
    18
}
/// $077102: addq.w #$1, d5
pub fn i_077102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77104;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077104: beq.w $77144
pub fn i_077104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77108;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77144; return 10; }
    8
}
/// $077108: clr.w d2
pub fn i_077108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7710A;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07710A: move.b $26(a6), d2
pub fn i_07710a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7710E;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07710E: add.w $28(a6), d2
pub fn i_07710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77112;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077112: asr.w #$8, d2
pub fn i_077112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77114;
    cpu.ir = 0xE042;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $077114: add.w d0, d2
pub fn i_077114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77116;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077116: sub.w d6, d2
pub fn i_077116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77118;
    cpu.ir = 0x9446;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077118: smi.b d2
pub fn i_077118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711A;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07711A: sub.w d6, d0
pub fn i_07711a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711C;
    cpu.ir = 0x9046;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07711C: smi.b d0
pub fn i_07711c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711E;
    cpu.ir = 0x5BC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07711E: eor.b d2, d0
pub fn i_07711e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77120;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $077120: beq.w $77144
pub fn i_077120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77124;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77144; return 10; }
    8
}
/// $077124: clr.w d4
pub fn i_077124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77126;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077126: clr.w d2
pub fn i_077126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77128;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077128: move.b $4(a1), d2
pub fn i_077128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7712C;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07712C: lsr.w #$1, d2
pub fn i_07712c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7712E;
    cpu.ir = 0xE24A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07712E: neg.w d2
pub fn i_07712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77130;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077130: add.w $2(a1), d2
pub fn i_077130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77134;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077134: cmp.w d2, d1
pub fn i_077134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77136;
    cpu.ir = 0xB242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $077136: spl.b d1
pub fn i_077136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77138;
    cpu.ir = 0x5AC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $077138: addi.b #$2, d1
pub fn i_077138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7713C;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07713C: clr.w d0
pub fn i_07713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7713E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07713E: move.b $5(a1), d0
pub fn i_07713e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77142;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $077142: rts
pub fn i_077142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77144;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RetMinus1_00077144  ($077144, 4 B, thunks/trivial_returns.c)
/// $077144: moveq #$ff, d0
pub fn i_077144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77146;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077146: rts
pub fn i_077146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SceneGate_077640  ($077640, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $077640: cmpi.b #$0, $106ece.l
pub fn i_077640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77648;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $077648: beq.w $7765e
pub fn i_077648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7764C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7765E; return 10; }
    8
}
/// $07764C: cmpi.b #$9, $106ece.l
pub fn i_07764c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77654;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x9, Size::Byte, false);
    20
}
/// $077654: beq.w $7765e
pub fn i_077654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77658;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7765E; return 10; }
    8
}
/// $077658: jmp $518.l
pub fn i_077658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7765E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07765E: jsr $77670(pc)
pub fn i_07765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77662;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77662);
    cpu.pc = 0x77670;
    16
}
/// $077662: jsr $236e.l
pub fn i_077662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77668;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77668);
    cpu.pc = 0x236E;
    16
}
/// $077668: move.w #$e000, $38(a6)
pub fn i_077668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7766E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07766E: rts
pub fn i_07766e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77670;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SndByScene_077670  ($077670, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $077670: move.w #$10, d1
pub fn i_077670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77674;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077674: cmpi.b #$5, $106ece.l
pub fn i_077674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7767C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x5, Size::Byte, false);
    20
}
/// $07767C: bne.w $77684
pub fn i_07767c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77680;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77684; return 10; }
    8
}
/// $077680: move.w #$138, d1
pub fn i_077680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77684;
    cpu.ir = 0x323C;
    let v1: u32 = 0x138;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077684: rts
pub fn i_077684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77686;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_InitC_0776e2  ($0776E2, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $0776E2: jsr $77640(pc)
pub fn i_0776e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x776E6);
    cpu.pc = 0x77640;
    16
}
/// $0776E6: move.l #$2dcabc, $74(a6)
pub fn i_0776e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776EE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCABC;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776EE: move.l #$2dca52, $78(a6)
pub fn i_0776ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776F6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA52;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776F6: move.l #$2dca72, $7c(a6)
pub fn i_0776f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776FE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA72;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776FE: move.l #$2dc9ba, $80(a6)
pub fn i_0776fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77706;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DC9BA;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077706: lea.l $778a2(pc), a1
pub fn i_077706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7770A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $07770A: move.l a1, (a6)
pub fn i_07770a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7770C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07770C: bra.w $778a2
pub fn i_07770c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77710;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitE_07773e  ($07773E, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $07773E: jsr $77640(pc)
pub fn i_07773e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77742);
    cpu.pc = 0x77640;
    16
}
/// $077742: move.l #$2dcb84, $74(a6)
pub fn i_077742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7774A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCB84;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07774A: move.l #$2dca62, $78(a6)
pub fn i_07774a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77752;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA62;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077752: move.l #$2dca82, $7c(a6)
pub fn i_077752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7775A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA82;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07775A: move.l #$2dca1a, $80(a6)
pub fn i_07775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77762;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA1A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077762: lea.l $778a2(pc), a1
pub fn i_077762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $077766: move.l a1, (a6)
pub fn i_077766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077768: bra.w $778a2
pub fn i_077768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7776C;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_Track_0778a2  ($0778A2, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $0778A2: jsr $77c0c(pc)
pub fn i_0778a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778A6);
    cpu.pc = 0x77C0C;
    16
}
/// $0778A6: movea.l $c(a6), a2
pub fn i_0778a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778AA;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0778AA: move.w $22(a2), d1
pub fn i_0778aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778AE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0778AE: move.w $24(a2), d2
pub fn i_0778ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778B2;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0778B2: move.w d1, $22(a6)
pub fn i_0778b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778B6;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0778B6: move.w d2, $24(a6)
pub fn i_0778b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778BA;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0778BA: jsr $77bc6(pc)
pub fn i_0778ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778BE);
    cpu.pc = 0x77BC6;
    16
}
/// $0778BE: bcc.w $778e0
pub fn i_0778be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x778E0; return 10; }
    8
}
/// $0778C2: cmpi.b #$40, $72(a6)
pub fn i_0778c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778C8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $0778C8: beq.w $778ea
pub fn i_0778c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x778EA; return 10; }
    8
}
/// $0778CC: cmpi.b #$80, $72(a6)
pub fn i_0778cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778D2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $0778D2: beq.w $778ea
pub fn i_0778d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x778EA; return 10; }
    8
}
/// $0778D6: lea.l $778fc(pc), a1
pub fn i_0778d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778FC;
    16
}
/// $0778DA: move.l a1, (a6)
pub fn i_0778da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778DC: bra.w $778ea
pub fn i_0778dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x778EA;
    10
}
/// $0778E0: jsr $77bae(pc)
pub fn i_0778e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778E4);
    cpu.pc = 0x77BAE;
    16
}
/// $0778E4: lea.l $77912(pc), a1
pub fn i_0778e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77912;
    16
}
/// $0778E8: move.l a1, (a6)
pub fn i_0778e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778EA: jsr $7b2.l
pub fn i_0778ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x778F0);
    cpu.pc = 0x7B2;
    16
}
/// $0778F0: bcc.w $778fa
pub fn i_0778f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x778FA; return 10; }
    8
}
// ===== SetTaskHandler_0778f4  ($0778F4, 8 B, kernel/task_handlers.c)
/// $0778F4: lea.l $77a8e(pc), a1
pub fn i_0778f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $0778F8: move.l a1, (a6)
pub fn i_0778f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778FA: rts
pub fn i_0778fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_WaitAim_0778fc  ($0778FC, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $0778FC: jsr $77bc6(pc)
pub fn i_0778fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77900;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77900);
    cpu.pc = 0x77BC6;
    16
}
/// $077900: bcs.w $7790e
pub fn i_077900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77904;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7790E; return 10; }
    8
}
/// $077904: jsr $77bae(pc)
pub fn i_077904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77908;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77908);
    cpu.pc = 0x77BAE;
    16
}
/// $077908: lea.l $77912(pc), a1
pub fn i_077908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7790C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77912;
    16
}
/// $07790C: move.l a1, (a6)
pub fn i_07790c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7790E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07790E: jmp $778ea(pc)
pub fn i_07790e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77912;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x778EA;
    8
}
// ===== MuzzleFx_Flash_077912  ($077912, 252 B, props/fx_explosions_platforms_crew_076000.s)
/// $077912: jsr $77bc6(pc)
pub fn i_077912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77916;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77916);
    cpu.pc = 0x77BC6;
    16
}
/// $077916: clr.w d0
pub fn i_077916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77918;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077918: cmpi.b #$80, $72(a6)
pub fn i_077918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7791E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $07791E: bne.w $77938
pub fn i_07791e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77922;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77938; return 10; }
    8
}
/// $077922: cmpi.b #$82, $88(a6)
pub fn i_077922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77928;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x82, Size::Byte, false);
    16
}
/// $077928: beq.w $77936
pub fn i_077928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7792C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77936; return 10; }
    8
}
/// $07792C: cmpi.b #$8c, $88(a6)
pub fn i_07792c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77932;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8C, Size::Byte, false);
    16
}
/// $077932: bne.w $77938
pub fn i_077932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77936;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77938; return 10; }
    8
}
/// $077936: addq.w #$1, d0
pub fn i_077936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77938;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077938: movea.l $7c(a6), a0
pub fn i_077938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7793C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07793C: lsl.w #$2, d0
pub fn i_07793c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7793E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07793E: movea.l (a0, d0.w), a0
pub fn i_07793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77942;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $077942: cmpa.l #$ffffffff, a0
pub fn i_077942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77948;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $077948: beq.w $77952
pub fn i_077948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7794C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77952; return 10; }
    8
}
/// $07794C: jsr $28cd4.l
pub fn i_07794c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77952);
    cpu.pc = 0x28CD4;
    16
}
/// $077952: lea.l $77958(pc), a1
pub fn i_077952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77956;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77958;
    16
}
/// $077956: move.l a1, (a6)
pub fn i_077956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77958;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077958: jsr $77c0c(pc)
pub fn i_077958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7795C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7795C);
    cpu.pc = 0x77C0C;
    16
}
/// $07795C: movea.l $c(a6), a2
pub fn i_07795c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77960;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077960: move.w $22(a2), d1
pub fn i_077960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77964;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077964: move.w $24(a2), d2
pub fn i_077964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77968;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077968: move.w d1, $22(a6)
pub fn i_077968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7796C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07796C: move.w d2, $24(a6)
pub fn i_07796c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77970;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077970: jsr $77bc6(pc)
pub fn i_077970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77974;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77974);
    cpu.pc = 0x77BC6;
    16
}
/// $077974: bcc.w $779d8
pub fn i_077974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77978;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x779D8; return 10; }
    8
}
/// $077978: jsr $77c28(pc)
pub fn i_077978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7797C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7797C);
    cpu.pc = 0x77C28;
    16
}
/// $07797C: lea.l $778a2(pc), a1
pub fn i_07797c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77980;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $077980: move.l a1, (a6)
pub fn i_077980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77982;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077982: move.w $84(a6), d1
pub fn i_077982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77986;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077986: jsr $77bc6(pc)
pub fn i_077986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7798A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7798A);
    cpu.pc = 0x77BC6;
    16
}
/// $07798A: cmpi.b #$0, $72(a6)
pub fn i_07798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77990;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $077990: beq.w $77a04
pub fn i_077990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77994;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $077994: cmpi.b #$c0, $72(a6)
pub fn i_077994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7799A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $07799A: beq.w $77a04
pub fn i_07799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7799E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $07799E: lea.l $77a16(pc), a1
pub fn i_07799e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A16;
    16
}
/// $0779A2: jsr $6fe.l
pub fn i_0779a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x779A8);
    cpu.pc = 0x6FE;
    16
}
/// $0779A8: jsr $5dd02.l
pub fn i_0779a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x779AE);
    cpu.pc = 0x5DD02;
    16
}
/// $0779AE: movea.l $c(a6), a2
pub fn i_0779ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779B2;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0779B2: move.w $34(a2), $34(a0)
pub fn i_0779b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779B8;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0779B8: move.w $88(a6), $88(a0)
pub fn i_0779b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779BE;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0779BE: bclr.b #$6, $12(a0)
pub fn i_0779be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779C4;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0779C4: btst.b #$6, $12(a6)
pub fn i_0779c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0779CA: beq.w $77a04
pub fn i_0779ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $0779CE: bset.b #$6, $12(a0)
pub fn i_0779ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779D4;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0779D4: bra.w $77a04
pub fn i_0779d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x77A04;
    10
}
/// $0779D8: cmpi.b #$0, $72(a6)
pub fn i_0779d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0779DE: beq.w $77a04
pub fn i_0779de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $0779E2: cmpi.b #$c0, $72(a6)
pub fn i_0779e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779E8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $0779E8: beq.w $77a04
pub fn i_0779e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $0779EC: jsr $77c28(pc)
pub fn i_0779ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x779F0);
    cpu.pc = 0x77C28;
    16
}
/// $0779F0: jsr $77bae(pc)
pub fn i_0779f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x779F4);
    cpu.pc = 0x77BAE;
    16
}
/// $0779F4: jsr $28d70.l
pub fn i_0779f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x779FA);
    cpu.pc = 0x28D70;
    16
}
/// $0779FA: bcc.w $77a04
pub fn i_0779fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $0779FE: lea.l $7776c(pc), a1
pub fn i_0779fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7776C;
    16
}
/// $077A02: move.l a1, (a6)
pub fn i_077a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077A04: jsr $7b2.l
pub fn i_077a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77A0A);
    cpu.pc = 0x7B2;
    16
}
/// $077A0A: bcc.w $77a14
pub fn i_077a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77A14; return 10; }
    8
}
// ===== SetTaskHandler_077a0e  ($077A0E, 8 B, kernel/task_handlers.c)
/// $077A0E: lea.l $77a8e(pc), a1
pub fn i_077a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077A12: move.l a1, (a6)
pub fn i_077a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077A14: rts
pub fn i_077a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_077a8e  ($077A8E, 8 B, kernel/jmp_to_scheduler.c)
/// $077A8E: jmp $518.l
pub fn i_077a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A94;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== MuzzleFx_ShotA_077a96  ($077A96, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $077A96: jsr $77640(pc)
pub fn i_077a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77A9A);
    cpu.pc = 0x77640;
    16
}
/// $077A9A: move.l #$2dca6a, $78(a6)
pub fn i_077a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AA2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA6A;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077AA2: jmp $77ad6(pc)
pub fn i_077aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AA6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x77AD6;
    8
}
// ===== MuzzleFx_Shot_Run_077ad6  ($077AD6, 138 B, props/fx_explosions_platforms_crew_076000.s)
/// $077AD6: jsr $77c0c(pc)
pub fn i_077ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ADA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77ADA);
    cpu.pc = 0x77C0C;
    16
}
/// $077ADA: movea.l $c(a6), a2
pub fn i_077ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ADE;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077ADE: move.w $22(a2), d1
pub fn i_077ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AE2;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077AE2: move.w $24(a2), d2
pub fn i_077ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AE6;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077AE6: move.w d1, $22(a6)
pub fn i_077ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AEA;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077AEA: move.w d2, $24(a6)
pub fn i_077aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AEE;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077AEE: jsr $77bc6(pc)
pub fn i_077aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77AF2);
    cpu.pc = 0x77BC6;
    16
}
/// $077AF2: bcs.w $77b14
pub fn i_077af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x77B14; return 10; }
    8
}
/// $077AF6: cmpi.b #$40, $72(a6)
pub fn i_077af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AFC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $077AFC: beq.w $77b28
pub fn i_077afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77B28; return 10; }
    8
}
/// $077B00: cmpi.b #$80, $72(a6)
pub fn i_077b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B06;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $077B06: beq.w $77b2c
pub fn i_077b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B0A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77B2C; return 10; }
    8
}
/// $077B0A: lea.l $77a8e(pc), a1
pub fn i_077b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077B0E: move.l a1, (a6)
pub fn i_077b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077B10: bra.w $77b40
pub fn i_077b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B14;
    cpu.ir = 0x6000;
    cpu.pc = 0x77B40;
    10
}
/// $077B14: cmpi.b #$80, $72(a6)
pub fn i_077b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B1A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $077B1A: beq.w $77b2c
pub fn i_077b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77B2C; return 10; }
    8
}
/// $077B1E: cmpi.b #$40, $72(a6)
pub fn i_077b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B24;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $077B24: bne.w $77b40
pub fn i_077b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77B40; return 10; }
    8
}
/// $077B28: jsr $77c28(pc)
pub fn i_077b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77B2C);
    cpu.pc = 0x77C28;
    16
}
/// $077B2C: lea.l $77b68(pc), a1
pub fn i_077b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77B68;
    16
}
/// $077B30: move.l a1, (a6)
pub fn i_077b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077B32: movea.l $c(a6), a2
pub fn i_077b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B36;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077B36: move.w $34(a2), $34(a6)
pub fn i_077b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B3C;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077B3C: jsr $77bae(pc)
pub fn i_077b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77B40);
    cpu.pc = 0x77BAE;
    16
}
/// $077B40: movea.l #$ffffffff, a0
pub fn i_077b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B46;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $077B46: jsr $5dd56.l
pub fn i_077b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77B4C);
    cpu.pc = 0x5DD56;
    16
}
/// $077B4C: bcc.w $77b56
pub fn i_077b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77B56; return 10; }
    8
}
/// $077B50: lea.l $77a8e(pc), a1
pub fn i_077b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077B54: move.l a1, (a6)
pub fn i_077b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077B56: jsr $7b2.l
pub fn i_077b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77B5C);
    cpu.pc = 0x7B2;
    16
}
/// $077B5C: bcc.w $77b66
pub fn i_077b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77B66; return 10; }
    8
}
// ===== SetTaskHandler_077b60  ($077B60, 8 B, kernel/task_handlers.c)
/// $077B60: lea.l $77a8e(pc), a1
pub fn i_077b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077B64: move.l a1, (a6)
pub fn i_077b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077B66: rts
pub fn i_077b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SetFlipByDir_077bae  ($077BAE, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $077BAE: bset.b #$6, $12(a6)
pub fn i_077bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BB4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $077BB4: cmpi.b #$80, $72(a6)
pub fn i_077bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BBA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $077BBA: bne.w $77bc4
pub fn i_077bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77BC4; return 10; }
    8
}
/// $077BBE: bclr.b #$6, $12(a6)
pub fn i_077bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BC4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $077BC4: rts
pub fn i_077bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_ReadParentAim_077bc6  ($077BC6, 38 B, props/fx_explosions_platforms_crew_076000.s)
/// $077BC6: movem.l a6, -(a7)
pub fn i_077bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BCA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $077BCA: movea.l $c(a6), a6
pub fn i_077bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BCE;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $077BCE: jsr $27fac.l
pub fn i_077bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77BD4);
    cpu.pc = 0x27FAC;
    16
}
/// $077BD4: movem.l (a7)+, a6
pub fn i_077bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BD8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $077BD8: bcc.w $77bf2
pub fn i_077bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77BF2; return 10; }
    8
}
/// $077BDC: move.b d7, $72(a6)
pub fn i_077bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BE0;
    cpu.ir = 0x1D47;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077BE0: andi.b #$c0, $72(a6)
pub fn i_077be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BE6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $077BE6: move.b #$1, $73(a6)
pub fn i_077be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BEC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_077bec  ($077BEC, 6 B, thunks/ccr_helpers.c)
/// $077BEC: ori.b #$11, ccr
pub fn i_077bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BF0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $077BF0: rts
pub fn i_077bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_StoreAim_077bf2  ($077BF2, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $077BF2: move.b d7, $72(a6)
pub fn i_077bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BF6;
    cpu.ir = 0x1D47;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077BF6: move.b d7, $88(a6)
pub fn i_077bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BFA;
    cpu.ir = 0x1D47;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077BFA: andi.b #$c0, $72(a6)
pub fn i_077bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C00;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $077C00: move.b #$0, $73(a6)
pub fn i_077c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C06;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_077c06  ($077C06, 6 B, thunks/ccr_helpers.c)
/// $077C06: andi.b #$ee, ccr
pub fn i_077c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C0A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $077C0A: rts
pub fn i_077c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SavePos_077c0c  ($077C0C, 14 B, props/fx_explosions_platforms_crew_076000.s)
/// $077C0C: move.w $22(a6), $84(a6)
pub fn i_077c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C12;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077C12: move.w $24(a6), $86(a6)
pub fn i_077c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C18;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077C18: rts
pub fn i_077c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_ClampGroundY_077c28  ($077C28, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $077C28: cmpi.b #$40, $72(a6)
pub fn i_077c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C2E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $077C2E: bne.w $77c60
pub fn i_077c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C32;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77C60; return 10; }
    8
}
/// $077C32: move.w #$0, d0
pub fn i_077c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C36;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077C36: move.w #$167, d1
pub fn i_077c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C3A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x167;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077C3A: cmpi.b #$6, $106ece.l
pub fn i_077c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C42;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x6, Size::Byte, false);
    20
}
/// $077C42: beq.w $77c52
pub fn i_077c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77C52; return 10; }
    8
}
/// $077C46: cmpi.b #$9, $106ece.l
pub fn i_077c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C4E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x9, Size::Byte, false);
    20
}
/// $077C4E: bne.w $77c56
pub fn i_077c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77C56; return 10; }
    8
}
/// $077C52: move.w #$1ef, d1
pub fn i_077c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1EF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077C56: jsr $440d0.l
pub fn i_077c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77C5C);
    cpu.pc = 0x440D0;
    16
}
/// $077C5C: move.w d1, $24(a6)
pub fn i_077c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C60;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077C60: rts
pub fn i_077c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fx_RandFlipFacing_077e00  ($077E00, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $077E00: jsr $5e9b6.l
pub fn i_077e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E06);
    cpu.pc = 0x5E9B6;
    16
}
/// $077E06: andi.b #$1, d0
pub fn i_077e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E0A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $077E0A: or.b d0, $3a(a6)
pub fn i_077e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E0E;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $077E0E: rts
pub fn i_077e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Explosion_Fire_077f6a  ($077F6A, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $077F6A: lea.l $2dd594.l, a0
pub fn i_077f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD594;
    20
}
/// $077F70: jsr $28cd4.l
pub fn i_077f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F76);
    cpu.pc = 0x28CD4;
    16
}
/// $077F76: lea.l $2df206.l, a0
pub fn i_077f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF206;
    20
}
/// $077F7C: move.l a0, $70(a6)
pub fn i_077f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F80;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077F80: move.w #$4, d1
pub fn i_077f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F84;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077F84: jsr $236e.l
pub fn i_077f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F8A);
    cpu.pc = 0x236E;
    16
}
/// $077F8A: bra.w $780de
pub fn i_077f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
/// $0780DE: jsr $77e00(pc)
pub fn i_0780de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x780E2);
    cpu.pc = 0x77E00;
    16
}
/// $0780E2: move.b #$ff, $32(a6)
pub fn i_0780e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0780E8: move.b #$ff, $33(a6)
pub fn i_0780e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0780EE: movea.l $70(a6), a0
pub fn i_0780ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0780F2: jsr $5dd56.l
pub fn i_0780f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x780F8);
    cpu.pc = 0x5DD56;
    16
}
/// $0780F8: bcs.w $78128
pub fn i_0780f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78128; return 10; }
    8
}
/// $0780FC: lea.l $ffff.w, a0
pub fn i_0780fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78100;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $078100: move.l a0, $48(a6)
pub fn i_078100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78104;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078104: lea.l $7810a(pc), a1
pub fn i_078104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78108;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7810A;
    16
}
/// $078108: move.l a1, (a6)
pub fn i_078108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7810A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07810A: jsr $44048.l
pub fn i_07810a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78110;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78110);
    cpu.pc = 0x44048;
    16
}
/// $078110: jsr $28d70.l
pub fn i_078110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78116);
    cpu.pc = 0x28D70;
    16
}
/// $078116: bcs.w $78128
pub fn i_078116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7811A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78128; return 10; }
    8
}
/// $07811A: movea.l $70(a6), a0
pub fn i_07811a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7811E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07811E: jsr $5dd56.l
pub fn i_07811e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78124);
    cpu.pc = 0x5DD56;
    16
}
/// $078124: bcc.w $7812e
pub fn i_078124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78128;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7812E; return 10; }
    8
}
/// $078128: jmp $518.l
pub fn i_078128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7812E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07812E: rts
pub fn i_07812e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78130;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_Tmpl_Driver_079298  ($079298, 142 B, props/fx_explosions_platforms_crew_076000.s)
/// $079298: clr.b d4
pub fn i_079298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7929A;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07929A: move.b $10e2e4.l, $10e2e3.l
pub fn i_07929a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792A4;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $0792A4: move.b $10e2ea.l, $10e2e9.l
pub fn i_0792a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792AE;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $0792AE: move.b d4, $10e2e3.l
pub fn i_0792ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792B4;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $0792B4: move.b d4, $10e2e9.l
pub fn i_0792b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792BA;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $0792BA: move.b d4, $10e2e4.l
pub fn i_0792ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792C0;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $0792C0: move.b d4, $10e2ea.l
pub fn i_0792c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792C6;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $0792C6: move.b d4, $10e2e5.l
pub fn i_0792c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792CC;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $0792CC: move.b d4, $10e2eb.l
pub fn i_0792cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792D2;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $0792D2: move.b d4, $10e2e6.l
pub fn i_0792d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792D8;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $0792D8: move.b d4, $10e2ec.l
pub fn i_0792d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792DE;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $0792DE: clr.b $10e39c.l
pub fn i_0792de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792E4;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $0792E4: clr.b $10e39d.l
pub fn i_0792e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792EA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39D) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39D, r2 as u8);
    16
}
/// $0792EA: lea.l $792f0(pc), a1
pub fn i_0792ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x792F0;
    16
}
/// $0792EE: move.l a1, (a6)
pub fn i_0792ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0792F0: jsr $7962c(pc)
pub fn i_0792f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x792F4);
    cpu.pc = 0x7962C;
    16
}
/// $0792F4: tst.b $10e39c.l
pub fn i_0792f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792FA;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0792FA: beq.w $79324
pub fn i_0792fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79324; return 10; }
    8
}
/// $0792FE: clr.w d0
pub fn i_0792fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79300;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079300: jsr $5e3a2.l
pub fn i_079300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79306;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79306);
    cpu.pc = 0x5E3A2;
    16
}
/// $079306: bcc.w $79310
pub fn i_079306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7930A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79310; return 10; }
    8
}
/// $07930A: move.b #$2, $45(a0)
pub fn i_07930a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79310;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079310: move.w #$1, d0
pub fn i_079310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79314;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079314: jsr $5e3a2.l
pub fn i_079314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7931A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7931A);
    cpu.pc = 0x5E3A2;
    16
}
/// $07931A: bcc.w $79324
pub fn i_07931a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7931E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79324; return 10; }
    8
}
/// $07931E: move.b #$2, $45(a0)
pub fn i_07931e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79324;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079324: rts
pub fn i_079324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79326;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_StartLevelScript_07962c  ($07962C, 184 B, props/fx_explosions_platforms_crew_076000.s)
/// $07962C: move.b $10e39c.l, d0
pub fn i_07962c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79632;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E39C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $079632: cmpi.b #$0, d0
pub fn i_079632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79636;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $079636: beq.w $796ea
pub fn i_079636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7963A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x796EA; return 10; }
    8
}
/// $07963A: cmpi.b #$5, d0
pub fn i_07963a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7963E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07963E: bcs.w $7964e
pub fn i_07963e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79642;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7964E; return 10; }
    8
}
/// $079642: nop
pub fn i_079642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79644;
    cpu.ir = 0x4E71;
    4
}
/// $079644: nop
pub fn i_079644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79646;
    cpu.ir = 0x4E71;
    4
}
/// $079646: cmpi.b #$5, d0
pub fn i_079646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7964A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07964A: nop
pub fn i_07964a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7964C;
    cpu.ir = 0x4E71;
    4
}
/// $07964C: trap #$f
pub fn i_07964c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7964E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $07964E: cmp.b $10e39d.l, d0
pub fn i_07964e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79654;
    cpu.ir = 0xB039;
    let v2 = bus.read8(0x10E39D) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $079654: beq.w $796f0
pub fn i_079654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79658;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x796F0; return 10; }
    8
}
/// $079658: move.b d0, $10e39d.l
pub fn i_079658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7965E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39D, v1 as u8);
    16
}
/// $07965E: move.b $10e200.l, d1
pub fn i_07965e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79664;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10E200) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $079664: move.b d1, $10e2e2.l
pub fn i_079664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7966A;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E2, v1 as u8);
    16
}
/// $07966A: move.b d1, $10e2e8.l
pub fn i_07966a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79670;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E8, v1 as u8);
    16
}
/// $079670: clr.b d4
pub fn i_079670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79672;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $079672: move.b $10e2e4.l, $10e2e3.l
pub fn i_079672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7967C;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $07967C: move.b $10e2ea.l, $10e2e9.l
pub fn i_07967c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79686;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $079686: move.b d4, $10e2e3.l
pub fn i_079686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7968C;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $07968C: move.b d4, $10e2e9.l
pub fn i_07968c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79692;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $079692: move.b d4, $10e2e4.l
pub fn i_079692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79698;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $079698: move.b d4, $10e2ea.l
pub fn i_079698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7969E;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $07969E: move.b d4, $10e2e5.l
pub fn i_07969e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796A4;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $0796A4: move.b d4, $10e2eb.l
pub fn i_0796a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796AA;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $0796AA: move.b d4, $10e2e6.l
pub fn i_0796aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796B0;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $0796B0: move.b d4, $10e2ec.l
pub fn i_0796b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796B6;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $0796B6: lea.l $1001c0.l, a4
pub fn i_0796b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796BC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $0796BC: move.b #$2, $44(a4)
pub fn i_0796bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796C2;
    cpu.ir = 0x197C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0796C2: andi.w #$ff, d0
pub fn i_0796c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0796C6: lsl.w #$2, d0
pub fn i_0796c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796C8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0796C8: lea.l $2df442.l, a1
pub fn i_0796c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF442;
    20
}
/// $0796CE: move.l (a1, d0.w), d0
pub fn i_0796ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796D2;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $0796D2: cmpi.l #$ffffffff, d0
pub fn i_0796d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796D8;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $0796D8: beq.w $796f0
pub fn i_0796d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x796F0; return 10; }
    8
}
/// $0796DC: movea.l d0, a1
pub fn i_0796dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796DE;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $0796DE: jsr $4ae.l
pub fn i_0796de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x796E4);
    cpu.pc = 0x4AE;
    16
}
// ===== SetXN_0796e4  ($0796E4, 6 B, thunks/ccr_helpers.c)
/// $0796E4: ori.b #$11, ccr
pub fn i_0796e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796E8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0796E8: rts
pub fn i_0796e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_ClearLevel_0796ea  ($0796EA, 6 B, props/fx_explosions_platforms_crew_076000.s)
/// $0796EA: clr.b $10e39d.l
pub fn i_0796ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796F0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39D) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39D, r2 as u8);
    16
}
// ===== ClearXN_0796f0  ($0796F0, 6 B, thunks/ccr_helpers.c)
/// $0796F0: andi.b #$ee, ccr
pub fn i_0796f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796F4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0796F4: rts
pub fn i_0796f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rank_DelayByDifficulty_079970  ($079970, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $079970: lea.l $10fd84.l, a3
pub fn i_079970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79976;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10FD84;
    20
}
/// $079976: moveq #$0, d0
pub fn i_079976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79978;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079978: move.b $8(a3), d0
pub fn i_079978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7997C;
    cpu.ir = 0x102B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07997C: lea.l $79968(pc), a0
pub fn i_07997c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79980;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x79968;
    16
}
/// $079980: lsl.w #$1, d0
pub fn i_079980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79982;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $079982: move.w (a0, d0.w), d0
pub fn i_079982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79986;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $079986: rts
pub fn i_079986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79988;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rank_SubIndex_0799a4  ($0799A4, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $0799A4: move.b $10fd8b.l, d1
pub fn i_0799a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799AA;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10FD8B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0799AA: cmpi.b #$1, $10fdaf.l
pub fn i_0799aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799B2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0799B2: bne.w $799ba
pub fn i_0799b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x799BA; return 10; }
    8
}
/// $0799B6: move.b #$4, d1
pub fn i_0799b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799BA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0799BA: move.l a0, -(a7)
pub fn i_0799ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799BC;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0799BC: move.w d0, -(a7)
pub fn i_0799bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799BE;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0799BE: move.b d1, -(a7)
pub fn i_0799be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799C0;
    cpu.ir = 0x1F01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write8(ad2, v1 as u8);
    10
}
/// $0799C0: jsr $2ac4c.l
pub fn i_0799c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x799C6);
    cpu.pc = 0x2AC4C;
    16
}
/// $0799C6: scs.b d1
pub fn i_0799c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799C8;
    cpu.ir = 0x55C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0799C8: neg.b d1
pub fn i_0799c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799CA;
    cpu.ir = 0x4401;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0799CA: add.b (a7)+, d1
pub fn i_0799ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799CC;
    cpu.ir = 0xD21F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $0799CC: move.w (a7)+, d0
pub fn i_0799cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799CE;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $0799CE: movea.l (a7)+, a0
pub fn i_0799ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799D0;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $0799D0: cmpi.b #$8, d1
pub fn i_0799d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799D4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $0799D4: bcs.w $799dc
pub fn i_0799d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x799DC; return 10; }
    8
}
/// $0799D8: move.b #$7, d1
pub fn i_0799d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799DC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0799DC: rts
pub fn i_0799dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tbl_Decode2D_0799DE  ($0799DE, 48 B, kernel/table_decode_2d_0799de.s)
/// $0799DE: cmpi.w #$2, (a0)+
pub fn i_0799de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799E2;
    cpu.ir = 0xC58;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    12
}
/// $0799E2: beq.w $79a0e
pub fn i_0799e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79A0E; return 10; }
    8
}
/// $0799E6: move.b $11(a6), d0
pub fn i_0799e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0799EA: andi.w #$3, d0
pub fn i_0799ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799EE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0799EE: add.w d0, d0
pub fn i_0799ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799F0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0799F0: add.w d0, d0
pub fn i_0799f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799F2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0799F2: move.b $106ed1.l, d1
pub fn i_0799f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799F8;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106ED1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0799F8: andi.w #$2, d1
pub fn i_0799f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799FC;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x2) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0799FC: add.w d1, d0
pub fn i_0799fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799FE;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0799FE: bsr.b $799a4
pub fn i_0799fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A00;
    cpu.ir = 0x61A4;
    cpu.push32(bus, 0x79A00);
    cpu.pc = 0x799A4;
    18
}
/// $079A00: andi.w #$7, d1
pub fn i_079a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A04;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A04: lsl.w #$4, d1
pub fn i_079a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A06;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $079A06: add.w d1, d0
pub fn i_079a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A08;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079A08: move.w (a0, d0.w), d0
pub fn i_079a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A0C;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $079A0C: rts
pub fn i_079a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tbl_DecodeShort_079A0E  ($079A0E, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $079A0E: move.b $11(a6), d0
pub fn i_079a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A12;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079A12: andi.w #$3, d0
pub fn i_079a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A16;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A16: add.w d0, d0
pub fn i_079a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A18;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079A18: move.b $106ed1.l, d1
pub fn i_079a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A1E;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106ED1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $079A1E: subq.b #$1, d1
pub fn i_079a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A20;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $079A20: andi.w #$1, d1
pub fn i_079a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A24;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A24: add.w d1, d0
pub fn i_079a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A26;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079A26: jsr $799a4(pc)
pub fn i_079a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79A2A);
    cpu.pc = 0x799A4;
    16
}
/// $079A2A: andi.w #$7, d1
pub fn i_079a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A2E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A2E: lsl.w #$3, d1
pub fn i_079a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A30;
    cpu.ir = 0xE749;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $079A30: add.w d1, d0
pub fn i_079a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A32;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079A32: move.b (a0, d0.w), d0
pub fn i_079a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A36;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $079A36: andi.w #$ff, d0
pub fn i_079a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A3A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A3A: lsl.w #$4, d0
pub fn i_079a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A3C;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $079A3C: lea.l $40(a0, d0.w), a0
pub fn i_079a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A40;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x40).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $079A40: jsr $5e9b6.l
pub fn i_079a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A46);
    cpu.pc = 0x5E9B6;
    16
}
/// $079A46: andi.w #$e, d0
pub fn i_079a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A4A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A4A: move.w (a0, d0.w), d0
pub fn i_079a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A4E;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $079A4E: rts
pub fn i_079a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A50;
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x07700C, i_07700c),
    (0x07700E, i_07700e),
    (0x077010, i_077010),
    (0x077012, i_077012),
    (0x077014, i_077014),
    (0x077018, i_077018),
    (0x07701C, i_07701c),
    (0x077020, i_077020),
    (0x077022, i_077022),
    (0x077024, i_077024),
    (0x077028, i_077028),
    (0x07702A, i_07702a),
    (0x07702E, i_07702e),
    (0x077030, i_077030),
    (0x077032, i_077032),
    (0x077036, i_077036),
    (0x077038, i_077038),
    (0x07703A, i_07703a),
    (0x07703C, i_07703c),
    (0x07703E, i_07703e),
    (0x077040, i_077040),
    (0x077042, i_077042),
    (0x077044, i_077044),
    (0x077048, i_077048),
    (0x07704A, i_07704a),
    (0x07704C, i_07704c),
    (0x077050, i_077050),
    (0x077052, i_077052),
    (0x077054, i_077054),
    (0x07707C, i_07707c),
    (0x077082, i_077082),
    (0x077086, i_077086),
    (0x07708C, i_07708c),
    (0x077090, i_077090),
    (0x077092, i_077092),
    (0x077098, i_077098),
    (0x0770A0, i_0770a0),
    (0x0770A4, i_0770a4),
    (0x0770AA, i_0770aa),
    (0x0770AE, i_0770ae),
    (0x0770CC, i_0770cc),
    (0x0770D2, i_0770d2),
    (0x0770DA, i_0770da),
    (0x0770DE, i_0770de),
    (0x0770E4, i_0770e4),
    (0x0770E8, i_0770e8),
    (0x0770EC, i_0770ec),
    (0x0770F2, i_0770f2),
    (0x0770F6, i_0770f6),
    (0x0770F8, i_0770f8),
    (0x0770FE, i_0770fe),
    (0x077102, i_077102),
    (0x077104, i_077104),
    (0x077108, i_077108),
    (0x07710A, i_07710a),
    (0x07710E, i_07710e),
    (0x077112, i_077112),
    (0x077114, i_077114),
    (0x077116, i_077116),
    (0x077118, i_077118),
    (0x07711A, i_07711a),
    (0x07711C, i_07711c),
    (0x07711E, i_07711e),
    (0x077120, i_077120),
    (0x077124, i_077124),
    (0x077126, i_077126),
    (0x077128, i_077128),
    (0x07712C, i_07712c),
    (0x07712E, i_07712e),
    (0x077130, i_077130),
    (0x077134, i_077134),
    (0x077136, i_077136),
    (0x077138, i_077138),
    (0x07713C, i_07713c),
    (0x07713E, i_07713e),
    (0x077142, i_077142),
    (0x077144, i_077144),
    (0x077146, i_077146),
    (0x077640, i_077640),
    (0x077648, i_077648),
    (0x07764C, i_07764c),
    (0x077654, i_077654),
    (0x077658, i_077658),
    (0x07765E, i_07765e),
    (0x077662, i_077662),
    (0x077668, i_077668),
    (0x07766E, i_07766e),
    (0x077670, i_077670),
    (0x077674, i_077674),
    (0x07767C, i_07767c),
    (0x077680, i_077680),
    (0x077684, i_077684),
    (0x0776E2, i_0776e2),
    (0x0776E6, i_0776e6),
    (0x0776EE, i_0776ee),
    (0x0776F6, i_0776f6),
    (0x0776FE, i_0776fe),
    (0x077706, i_077706),
    (0x07770A, i_07770a),
    (0x07770C, i_07770c),
    (0x07773E, i_07773e),
    (0x077742, i_077742),
    (0x07774A, i_07774a),
    (0x077752, i_077752),
    (0x07775A, i_07775a),
    (0x077762, i_077762),
    (0x077766, i_077766),
    (0x077768, i_077768),
    (0x0778A2, i_0778a2),
    (0x0778A6, i_0778a6),
    (0x0778AA, i_0778aa),
    (0x0778AE, i_0778ae),
    (0x0778B2, i_0778b2),
    (0x0778B6, i_0778b6),
    (0x0778BA, i_0778ba),
    (0x0778BE, i_0778be),
    (0x0778C2, i_0778c2),
    (0x0778C8, i_0778c8),
    (0x0778CC, i_0778cc),
    (0x0778D2, i_0778d2),
    (0x0778D6, i_0778d6),
    (0x0778DA, i_0778da),
    (0x0778DC, i_0778dc),
    (0x0778E0, i_0778e0),
    (0x0778E4, i_0778e4),
    (0x0778E8, i_0778e8),
    (0x0778EA, i_0778ea),
    (0x0778F0, i_0778f0),
    (0x0778F4, i_0778f4),
    (0x0778F8, i_0778f8),
    (0x0778FA, i_0778fa),
    (0x0778FC, i_0778fc),
    (0x077900, i_077900),
    (0x077904, i_077904),
    (0x077908, i_077908),
    (0x07790C, i_07790c),
    (0x07790E, i_07790e),
    (0x077912, i_077912),
    (0x077916, i_077916),
    (0x077918, i_077918),
    (0x07791E, i_07791e),
    (0x077922, i_077922),
    (0x077928, i_077928),
    (0x07792C, i_07792c),
    (0x077932, i_077932),
    (0x077936, i_077936),
    (0x077938, i_077938),
    (0x07793C, i_07793c),
    (0x07793E, i_07793e),
    (0x077942, i_077942),
    (0x077948, i_077948),
    (0x07794C, i_07794c),
    (0x077952, i_077952),
    (0x077956, i_077956),
    (0x077958, i_077958),
    (0x07795C, i_07795c),
    (0x077960, i_077960),
    (0x077964, i_077964),
    (0x077968, i_077968),
    (0x07796C, i_07796c),
    (0x077970, i_077970),
    (0x077974, i_077974),
    (0x077978, i_077978),
    (0x07797C, i_07797c),
    (0x077980, i_077980),
    (0x077982, i_077982),
    (0x077986, i_077986),
    (0x07798A, i_07798a),
    (0x077990, i_077990),
    (0x077994, i_077994),
    (0x07799A, i_07799a),
    (0x07799E, i_07799e),
    (0x0779A2, i_0779a2),
    (0x0779A8, i_0779a8),
    (0x0779AE, i_0779ae),
    (0x0779B2, i_0779b2),
    (0x0779B8, i_0779b8),
    (0x0779BE, i_0779be),
    (0x0779C4, i_0779c4),
    (0x0779CA, i_0779ca),
    (0x0779CE, i_0779ce),
    (0x0779D4, i_0779d4),
    (0x0779D8, i_0779d8),
    (0x0779DE, i_0779de),
    (0x0779E2, i_0779e2),
    (0x0779E8, i_0779e8),
    (0x0779EC, i_0779ec),
    (0x0779F0, i_0779f0),
    (0x0779F4, i_0779f4),
    (0x0779FA, i_0779fa),
    (0x0779FE, i_0779fe),
    (0x077A02, i_077a02),
    (0x077A04, i_077a04),
    (0x077A0A, i_077a0a),
    (0x077A0E, i_077a0e),
    (0x077A12, i_077a12),
    (0x077A14, i_077a14),
    (0x077A8E, i_077a8e),
    (0x077A96, i_077a96),
    (0x077A9A, i_077a9a),
    (0x077AA2, i_077aa2),
    (0x077AD6, i_077ad6),
    (0x077ADA, i_077ada),
    (0x077ADE, i_077ade),
    (0x077AE2, i_077ae2),
    (0x077AE6, i_077ae6),
    (0x077AEA, i_077aea),
    (0x077AEE, i_077aee),
    (0x077AF2, i_077af2),
    (0x077AF6, i_077af6),
    (0x077AFC, i_077afc),
    (0x077B00, i_077b00),
    (0x077B06, i_077b06),
    (0x077B0A, i_077b0a),
    (0x077B0E, i_077b0e),
    (0x077B10, i_077b10),
    (0x077B14, i_077b14),
    (0x077B1A, i_077b1a),
    (0x077B1E, i_077b1e),
    (0x077B24, i_077b24),
    (0x077B28, i_077b28),
    (0x077B2C, i_077b2c),
    (0x077B30, i_077b30),
    (0x077B32, i_077b32),
    (0x077B36, i_077b36),
    (0x077B3C, i_077b3c),
    (0x077B40, i_077b40),
    (0x077B46, i_077b46),
    (0x077B4C, i_077b4c),
    (0x077B50, i_077b50),
    (0x077B54, i_077b54),
    (0x077B56, i_077b56),
    (0x077B5C, i_077b5c),
    (0x077B60, i_077b60),
    (0x077B64, i_077b64),
    (0x077B66, i_077b66),
    (0x077BAE, i_077bae),
    (0x077BB4, i_077bb4),
    (0x077BBA, i_077bba),
    (0x077BBE, i_077bbe),
    (0x077BC4, i_077bc4),
    (0x077BC6, i_077bc6),
    (0x077BCA, i_077bca),
    (0x077BCE, i_077bce),
    (0x077BD4, i_077bd4),
    (0x077BD8, i_077bd8),
    (0x077BDC, i_077bdc),
    (0x077BE0, i_077be0),
    (0x077BE6, i_077be6),
    (0x077BEC, i_077bec),
    (0x077BF0, i_077bf0),
    (0x077BF2, i_077bf2),
    (0x077BF6, i_077bf6),
    (0x077BFA, i_077bfa),
    (0x077C00, i_077c00),
    (0x077C06, i_077c06),
    (0x077C0A, i_077c0a),
    (0x077C0C, i_077c0c),
    (0x077C12, i_077c12),
    (0x077C18, i_077c18),
    (0x077C28, i_077c28),
    (0x077C2E, i_077c2e),
    (0x077C32, i_077c32),
    (0x077C36, i_077c36),
    (0x077C3A, i_077c3a),
    (0x077C42, i_077c42),
    (0x077C46, i_077c46),
    (0x077C4E, i_077c4e),
    (0x077C52, i_077c52),
    (0x077C56, i_077c56),
    (0x077C5C, i_077c5c),
    (0x077C60, i_077c60),
    (0x077E00, i_077e00),
    (0x077E06, i_077e06),
    (0x077E0A, i_077e0a),
    (0x077E0E, i_077e0e),
    (0x077F6A, i_077f6a),
    (0x077F70, i_077f70),
    (0x077F76, i_077f76),
    (0x077F7C, i_077f7c),
    (0x077F80, i_077f80),
    (0x077F84, i_077f84),
    (0x077F8A, i_077f8a),
    (0x0780DE, i_0780de),
    (0x0780E2, i_0780e2),
    (0x0780E8, i_0780e8),
    (0x0780EE, i_0780ee),
    (0x0780F2, i_0780f2),
    (0x0780F8, i_0780f8),
    (0x0780FC, i_0780fc),
    (0x078100, i_078100),
    (0x078104, i_078104),
    (0x078108, i_078108),
    (0x07810A, i_07810a),
    (0x078110, i_078110),
    (0x078116, i_078116),
    (0x07811A, i_07811a),
    (0x07811E, i_07811e),
    (0x078124, i_078124),
    (0x078128, i_078128),
    (0x07812E, i_07812e),
    (0x079298, i_079298),
    (0x07929A, i_07929a),
    (0x0792A4, i_0792a4),
    (0x0792AE, i_0792ae),
    (0x0792B4, i_0792b4),
    (0x0792BA, i_0792ba),
    (0x0792C0, i_0792c0),
    (0x0792C6, i_0792c6),
    (0x0792CC, i_0792cc),
    (0x0792D2, i_0792d2),
    (0x0792D8, i_0792d8),
    (0x0792DE, i_0792de),
    (0x0792E4, i_0792e4),
    (0x0792EA, i_0792ea),
    (0x0792EE, i_0792ee),
    (0x0792F0, i_0792f0),
    (0x0792F4, i_0792f4),
    (0x0792FA, i_0792fa),
    (0x0792FE, i_0792fe),
    (0x079300, i_079300),
    (0x079306, i_079306),
    (0x07930A, i_07930a),
    (0x079310, i_079310),
    (0x079314, i_079314),
    (0x07931A, i_07931a),
    (0x07931E, i_07931e),
    (0x079324, i_079324),
    (0x07962C, i_07962c),
    (0x079632, i_079632),
    (0x079636, i_079636),
    (0x07963A, i_07963a),
    (0x07963E, i_07963e),
    (0x079642, i_079642),
    (0x079644, i_079644),
    (0x079646, i_079646),
    (0x07964A, i_07964a),
    (0x07964C, i_07964c),
    (0x07964E, i_07964e),
    (0x079654, i_079654),
    (0x079658, i_079658),
    (0x07965E, i_07965e),
    (0x079664, i_079664),
    (0x07966A, i_07966a),
    (0x079670, i_079670),
    (0x079672, i_079672),
    (0x07967C, i_07967c),
    (0x079686, i_079686),
    (0x07968C, i_07968c),
    (0x079692, i_079692),
    (0x079698, i_079698),
    (0x07969E, i_07969e),
    (0x0796A4, i_0796a4),
    (0x0796AA, i_0796aa),
    (0x0796B0, i_0796b0),
    (0x0796B6, i_0796b6),
    (0x0796BC, i_0796bc),
    (0x0796C2, i_0796c2),
    (0x0796C6, i_0796c6),
    (0x0796C8, i_0796c8),
    (0x0796CE, i_0796ce),
    (0x0796D2, i_0796d2),
    (0x0796D8, i_0796d8),
    (0x0796DC, i_0796dc),
    (0x0796DE, i_0796de),
    (0x0796E4, i_0796e4),
    (0x0796E8, i_0796e8),
    (0x0796EA, i_0796ea),
    (0x0796F0, i_0796f0),
    (0x0796F4, i_0796f4),
    (0x079970, i_079970),
    (0x079976, i_079976),
    (0x079978, i_079978),
    (0x07997C, i_07997c),
    (0x079980, i_079980),
    (0x079982, i_079982),
    (0x079986, i_079986),
    (0x0799A4, i_0799a4),
    (0x0799AA, i_0799aa),
    (0x0799B2, i_0799b2),
    (0x0799B6, i_0799b6),
    (0x0799BA, i_0799ba),
    (0x0799BC, i_0799bc),
    (0x0799BE, i_0799be),
    (0x0799C0, i_0799c0),
    (0x0799C6, i_0799c6),
    (0x0799C8, i_0799c8),
    (0x0799CA, i_0799ca),
    (0x0799CC, i_0799cc),
    (0x0799CE, i_0799ce),
    (0x0799D0, i_0799d0),
    (0x0799D4, i_0799d4),
    (0x0799D8, i_0799d8),
    (0x0799DC, i_0799dc),
    (0x0799DE, i_0799de),
    (0x0799E2, i_0799e2),
    (0x0799E6, i_0799e6),
    (0x0799EA, i_0799ea),
    (0x0799EE, i_0799ee),
    (0x0799F0, i_0799f0),
    (0x0799F2, i_0799f2),
    (0x0799F8, i_0799f8),
    (0x0799FC, i_0799fc),
    (0x0799FE, i_0799fe),
    (0x079A00, i_079a00),
    (0x079A04, i_079a04),
    (0x079A06, i_079a06),
    (0x079A08, i_079a08),
    (0x079A0C, i_079a0c),
    (0x079A0E, i_079a0e),
    (0x079A12, i_079a12),
    (0x079A16, i_079a16),
    (0x079A18, i_079a18),
    (0x079A1E, i_079a1e),
    (0x079A20, i_079a20),
    (0x079A24, i_079a24),
    (0x079A26, i_079a26),
    (0x079A2A, i_079a2a),
    (0x079A2E, i_079a2e),
    (0x079A30, i_079a30),
    (0x079A32, i_079a32),
    (0x079A36, i_079a36),
    (0x079A3A, i_079a3a),
    (0x079A3C, i_079a3c),
    (0x079A40, i_079a40),
    (0x079A46, i_079a46),
    (0x079A4A, i_079a4a),
    (0x079A4E, i_079a4e),
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
];
