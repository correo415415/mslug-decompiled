// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $049D72: move.b $10(a6), d0
pub fn i_049d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D76;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $049D76: cmp.b $10(a1), d0
pub fn i_049d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D7A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $049D7A: bcs.w $49d84
pub fn i_049d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D7E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49D84; return 10; }
    8
}
// ===== ClearXN_049d7e  ($049D7E, 6 B, thunks/ccr_helpers.c)
/// $049D7E: andi.b #$ee, ccr
pub fn i_049d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D82;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049D82: rts
pub fn i_049d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_049d84  ($049D84, 6 B, thunks/ccr_helpers.c)
/// $049D84: ori.b #$11, ccr
pub fn i_049d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D88;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049D88: rts
pub fn i_049d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_HitCheckUnlessCutscene_049fba  ($049FBA, 10 B, pow/hang_049430.s)
/// $049FBA: tst.b $106ed3.l
pub fn i_049fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FC0;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $049FC0: bne.w $49fca
pub fn i_049fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49FCA; return 10; }
    8
}
// ===== SetC_049fc4  ($049FC4, 6 B, thunks/ccr_helpers.c)
/// $049FC4: ori.b #$1, ccr
pub fn i_049fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FC8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $049FC8: rts
pub fn i_049fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_049fca  ($049FCA, 6 B, thunks/jmp_abs_thunks.c)
/// $049FCA: jmp $2870a.l
pub fn i_049fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FD0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x2870A;
    8
}
// ===== Entity_ProbeAndInstallHandler_049FD0  ($049FD0, 34 B, entity/probe_install_handler_049fd0.s)
/// $049FD0: bsr.b $49fba
pub fn i_049fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FD2;
    cpu.ir = 0x61E8;
    cpu.push32(bus, 0x49FD2);
    cpu.pc = 0x49FBA;
    18
}
/// $049FD2: bcc.w $49ff0
pub fn i_049fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49FF0; return 10; }
    8
}
/// $049FD6: jsr $27eba.l
pub fn i_049fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49FDC);
    cpu.pc = 0x27EBA;
    16
}
/// $049FDC: bcc.w $49fea
pub fn i_049fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49FEA; return 10; }
    8
}
/// $049FE0: lea.l $4a034(pc), a1
pub fn i_049fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A034;
    16
}
/// $049FE4: move.l a1, (a6)
pub fn i_049fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049FE6: bra.w $49ff0
pub fn i_049fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x49FF0;
    10
}
/// $049FEA: lea.l $4a014(pc), a1
pub fn i_049fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A014;
    16
}
/// $049FEE: move.l a1, (a6)
pub fn i_049fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049FF0: rts
pub fn i_049ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_Dispatch_049ff2  ($049FF2, 26 B, pow/human_death_049ff2.s)
/// $049FF2: bsr.b $49fba
pub fn i_049ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FF4;
    cpu.ir = 0x61C6;
    cpu.push32(bus, 0x49FF4);
    cpu.pc = 0x49FBA;
    18
}
/// $049FF4: bcc.w $4a012
pub fn i_049ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A012; return 10; }
    8
}
/// $049FF8: jsr $27eba.l
pub fn i_049ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49FFE);
    cpu.pc = 0x27EBA;
    16
}
/// $049FFE: bcc.w $4a00c
pub fn i_049ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A002;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A00C; return 10; }
    8
}
/// $04A002: lea.l $4a034(pc), a1
pub fn i_04a002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A006;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A034;
    16
}
/// $04A006: move.l a1, (a6)
pub fn i_04a006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A008;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A008: bra.w $4a012
pub fn i_04a008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A00C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A012;
    10
}
// ===== SetTaskHandler_04a00c  ($04A00C, 8 B, kernel/task_handlers.c)
/// $04A00C: lea.l $4a024(pc), a1
pub fn i_04a00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A010;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A024;
    16
}
/// $04A010: move.l a1, (a6)
pub fn i_04a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A012;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A012: rts
pub fn i_04a012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A014;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_EntryKind0_04a014  ($04A014, 16 B, pow/human_death_049ff2.s)
/// $04A014: jsr $27f60.l
pub fn i_04a014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A01A);
    cpu.pc = 0x27F60;
    16
}
/// $04A01A: scc.b $70(a6)
pub fn i_04a01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A01E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A01E: moveq #$0, d7
pub fn i_04a01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A020;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A020: bra.w $4a050
pub fn i_04a020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A024;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A050;
    10
}
// ===== HumanDeath_EntryKind1_04a024  ($04A024, 16 B, pow/human_death_049ff2.s)
/// $04A024: jsr $27f60.l
pub fn i_04a024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A02A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A02A);
    cpu.pc = 0x27F60;
    16
}
/// $04A02A: scc.b $70(a6)
pub fn i_04a02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A02E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A02E: moveq #$1, d7
pub fn i_04a02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A030;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A030: bra.w $4a050
pub fn i_04a030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A034;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A050;
    10
}
// ===== HumanDeath_EntryKind2_04a034  ($04A034, 104 B, pow/human_death_049ff2.s)
/// $04A034: jsr $27f60.l
pub fn i_04a034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A03A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A03A);
    cpu.pc = 0x27F60;
    16
}
/// $04A03A: scc.b $70(a6)
pub fn i_04a03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A03E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A03E: moveq #$2, d7
pub fn i_04a03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A040;
    cpu.ir = 0x7E02;
    cpu.d[7] = 0x2;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A040: bra.w $4a050
pub fn i_04a040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A044;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A050;
    10
}
/// $04A044: jsr $27f60.l
pub fn i_04a044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A04A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A04A);
    cpu.pc = 0x27F60;
    16
}
/// $04A04A: scc.b $70(a6)
pub fn i_04a04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A04E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A04E: moveq #$3, d7
pub fn i_04a04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A050;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A050: move.w $22(a6), d0
pub fn i_04a050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A054;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A054: sub.w $54(a6), d0
pub fn i_04a054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A058;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04A058: scc.b d0
pub fn i_04a058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A05A;
    cpu.ir = 0x54C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A05A: btst.b #$0, $3a(a6)
pub fn i_04a05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A060;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04A060: sne.b d1
pub fn i_04a060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A062;
    cpu.ir = 0x56C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A062: eor.b d0, d1
pub fn i_04a062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A064;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04A064: move.b d1, $71(a6)
pub fn i_04a064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A068;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A068: andi.w #$ff, d7
pub fn i_04a068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A06C;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A06C: lsl.w #$2, d7
pub fn i_04a06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A06E;
    cpu.ir = 0xE54F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04A06E: lea.l $49faa(pc), a0
pub fn i_04a06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A072;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x49FAA;
    16
}
/// $04A072: movea.l (a0, d7.w), a0
pub fn i_04a072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A076;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04A076: moveq #$0, d0
pub fn i_04a076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A078;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A078: move.b $58(a6), d0
pub fn i_04a078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A07C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04A07C: cmpi.w #$22, d0
pub fn i_04a07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A080;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x22, Size::Word, false);
    8
}
/// $04A080: bcs.w $4a088
pub fn i_04a080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A084;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4A088; return 10; }
    8
}
/// $04A084: move.w #$f, d0
pub fn i_04a084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A088;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A088: lsl.w #$2, d0
pub fn i_04a088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A08A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04A08A: move.l (a0, d0.w), -(a7)
pub fn i_04a08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A08E;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $04A08E: bsr.w $4a09c
pub fn i_04a08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A092;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A092);
    cpu.pc = 0x4A09C;
    18
}
/// $04A092: jsr $8f308.l
pub fn i_04a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A098);
    cpu.pc = 0x8F308;
    16
}
/// $04A098: movea.l (a7)+, a0
pub fn i_04a098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A09A;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $04A09A: jmp (a0)
pub fn i_04a09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A09C;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
// ===== HumanDeath_ResetBody_04a09c  ($04A09C, 56 B, pow/human_death_049ff2.s)
/// $04A09C: jsr $2783a.l
pub fn i_04a09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A0A2);
    cpu.pc = 0x2783A;
    16
}
/// $04A0A2: bsr.w $4a0d4
pub fn i_04a0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A0A6);
    cpu.pc = 0x4A0D4;
    18
}
/// $04A0A6: bclr.b #$1, $12(a6)
pub fn i_04a0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0AC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A0AC: move.w #$ffff, $34(a6)
pub fn i_04a0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A0B2: move.l #$ffffffff, $60(a6)
pub fn i_04a0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0BA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04A0BA: jsr $27f60.l
pub fn i_04a0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A0C0);
    cpu.pc = 0x27F60;
    16
}
/// $04A0C0: scc.b $70(a6)
pub fn i_04a0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0C4;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A0C4: clr.w $2c(a6)
pub fn i_04a0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0C8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A0C8: clr.w $2e(a6)
pub fn i_04a0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0CC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A0CC: move.w #$c000, $38(a6)
pub fn i_04a0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A0D2: rts
pub fn i_04a0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PlayCryByKind_04a0d4  ($04A0D4, 146 B, pow/human_death_049ff2.s)
/// $04A0D4: move.w #$e, d1
pub fn i_04a0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A0D8: tst.b $10fd8f.l
pub fn i_04a0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0DE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04A0DE: bne.w $4a0e6
pub fn i_04a0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A0E6; return 10; }
    8
}
/// $04A0E2: move.w #$2f, d1
pub fn i_04a0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0E6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A0E6: bra.w $4a0f4
pub fn i_04a0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A0F4;
    10
}
/// $04A0EA: move.w #$1a, d1
pub fn i_04a0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0EE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A0EE: bra.w $4a0f4
pub fn i_04a0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A0F4;
    10
}
/// $04A0F2: nop
pub fn i_04a0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0F4;
    cpu.ir = 0x4E71;
    4
}
/// $04A0F4: movem.w d1, -(a7)
pub fn i_04a0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0F8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $04A0F8: jsr $13600.l
pub fn i_04a0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A0FE);
    cpu.pc = 0x13600;
    16
}
/// $04A0FE: movem.w (a7)+, d1
pub fn i_04a0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A102;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04A102: jmp $236e.l
pub fn i_04a102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A108;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x236E;
    8
}
/// $04A108: move.w #$14, d1
pub fn i_04a108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A10C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A10C: bra.w $4a132
pub fn i_04a10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A110;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A132;
    10
}
/// $04A110: move.w #$15c, d1
pub fn i_04a110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A114;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A114: bra.w $4a132
pub fn i_04a114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A118;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A132;
    10
}
/// $04A118: move.w #$21, d1
pub fn i_04a118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A11C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x21;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A11C: bra.w $4a132
pub fn i_04a11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A120;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A132;
    10
}
/// $04A120: move.w #$e, d1
pub fn i_04a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A124;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A124: bra.w $4a132
pub fn i_04a124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A128;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A132;
    10
}
/// $04A128: move.w #$185, d1
pub fn i_04a128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A12C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x185;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A12C: bra.w $4a132
pub fn i_04a12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A130;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A132;
    10
}
/// $04A130: nop
pub fn i_04a130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A132;
    cpu.ir = 0x4E71;
    4
}
/// $04A132: movem.w d1, -(a7)
pub fn i_04a132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A136;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $04A136: jsr $13600.l
pub fn i_04a136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A13C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A13C);
    cpu.pc = 0x13600;
    16
}
/// $04A13C: movem.w (a7)+, d1
pub fn i_04a13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A140;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04A140: jmp $236e.l
pub fn i_04a140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A146;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x236E;
    8
}
/// $04A146: jsr $27f60.l
pub fn i_04a146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A14C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A14C);
    cpu.pc = 0x27F60;
    16
}
/// $04A14C: scc.b $70(a6)
pub fn i_04a14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A150;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A150: bra.w $4a4a2
pub fn i_04a150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A154;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A4A2;
    10
}
/// $04A154: jsr $27f60.l
pub fn i_04a154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A15A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A15A);
    cpu.pc = 0x27F60;
    16
}
/// $04A15A: scc.b $70(a6)
pub fn i_04a15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A15E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A15E: bsr.w $4a0d4
pub fn i_04a15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A162;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A162);
    cpu.pc = 0x4A0D4;
    18
}
/// $04A162: bra.w $4a5a6
pub fn i_04a162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A166;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A5A6;
    10
}
// ===== SetTaskHandler_04a166  ($04A166, 8 B, kernel/task_handlers.c)
/// $04A166: lea.l $4a18c(pc), a1
pub fn i_04a166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A16A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A18C;
    16
}
/// $04A16A: move.l a1, (a6)
pub fn i_04a16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A16C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A16C: rts
pub fn i_04a16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A16E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SpawnCorpseA_04a16e  ($04A16E, 30 B, pow/human_death_049ff2.s)
/// $04A16E: lea.l $4a18c(pc), a1
pub fn i_04a16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A172;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A18C;
    16
}
/// $04A172: jsr $4ae.l
pub fn i_04a172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A178;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A178);
    cpu.pc = 0x4AE;
    16
}
/// $04A178: jsr $5dd02.l
pub fn i_04a178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A17E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A17E);
    cpu.pc = 0x5DD02;
    16
}
/// $04A17E: move.w $22(a6), $54(a0)
pub fn i_04a17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A184;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04A184: move.w $24(a6), $54(a0)
pub fn i_04a184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A18A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04A18A: rts
pub fn i_04a18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A18C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_CorpseA_04a18c  ($04A18C, 8 B, pow/human_death_049ff2.s)
/// $04A18C: bsr.w $4a09c
pub fn i_04a18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A190;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A190);
    cpu.pc = 0x4A09C;
    18
}
/// $04A190: bra.w $4a72e
pub fn i_04a190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A194;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A72E;
    10
}
// ===== HumanDeath_SpawnCorpseB_04a194  ($04A194, 30 B, pow/human_death_049ff2.s)
/// $04A194: lea.l $4a1b2(pc), a1
pub fn i_04a194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A198;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A1B2;
    16
}
/// $04A198: jsr $4ae.l
pub fn i_04a198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A19E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A19E);
    cpu.pc = 0x4AE;
    16
}
/// $04A19E: jsr $5dd02.l
pub fn i_04a19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A1A4);
    cpu.pc = 0x5DD02;
    16
}
/// $04A1A4: move.w $22(a6), $54(a0)
pub fn i_04a1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1AA;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04A1AA: move.w $24(a6), $54(a0)
pub fn i_04a1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1B0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04A1B0: rts
pub fn i_04a1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_CorpseB_04a1b2  ($04A1B2, 8 B, pow/human_death_049ff2.s)
/// $04A1B2: bsr.w $4a09c
pub fn i_04a1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1B6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A1B6);
    cpu.pc = 0x4A09C;
    18
}
/// $04A1B6: bra.w $4a802
pub fn i_04a1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A802;
    10
}
// ===== HumanDeath_LoadTimer_04a1ba  ($04A1BA, 4 B, pow/human_death_049ff2.s)
/// $04A1BA: move.w $74(a6), d0
pub fn i_04a1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1BE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== JsrAbsThunk_04a1be  ($04A1BE, 8 B, thunks/jsr_abs_thunks.c)
/// $04A1BE: jsr $2352.l
pub fn i_04a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A1C4);
    cpu.pc = 0x2352;
    16
}
/// $04A1C4: rts
pub fn i_04a1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_DampVelocity_04a1c6  ($04A1C6, 42 B, pow/human_death_049ff2.s)
/// $04A1C6: move.w $28(a6), d0
pub fn i_04a1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1CA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A1CA: beq.w $4a1da
pub fn i_04a1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A1DA; return 10; }
    8
}
/// $04A1CE: asr.w #$5, d0
pub fn i_04a1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1D0;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04A1D0: seq.b d1
pub fn i_04a1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1D2;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A1D2: ext.w d1
pub fn i_04a1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1D4;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A1D4: sub.w d1, d0
pub fn i_04a1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1D6;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04A1D6: sub.w d0, $28(a6)
pub fn i_04a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1DA;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A1DA: move.w $2a(a6), d0
pub fn i_04a1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A1DE: beq.w $4a1ee
pub fn i_04a1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A1EE; return 10; }
    8
}
/// $04A1E2: asr.w #$5, d0
pub fn i_04a1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1E4;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04A1E4: seq.b d1
pub fn i_04a1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1E6;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A1E6: ext.w d1
pub fn i_04a1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1E8;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A1E8: sub.w d1, d0
pub fn i_04a1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1EA;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04A1EA: sub.w d0, $2a(a6)
pub fn i_04a1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1EE;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A1EE: rts
pub fn i_04a1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SetVelXByFacing_04a1f0  ($04A1F0, 12 B, pow/human_death_049ff2.s)
/// $04A1F0: btst.b #$0, $3a(a6)
pub fn i_04a1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1F6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04A1F6: bne.w $4a1fc
pub fn i_04a1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A1FC; return 10; }
    8
}
/// $04A1FA: neg.w d0
pub fn i_04a1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1FC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_04a1fc  ($04A1FC, 6 B, kernel/task_setters_w.c)
/// $04A1FC: move.w d0, $28(a6)
pub fn i_04a1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A200;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A200: rts
pub fn i_04a200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A202;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_RandBelowY_04a202  ($04A202, 14 B, pow/human_death_049ff2.s)
/// $04A202: jsr $5e9b6.l
pub fn i_04a202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A208;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A208);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A208: move.w $24(a6), d1
pub fn i_04a208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A20C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A20C: cmp.b d1, d0
pub fn i_04a20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A20E;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $04A20E: rts
pub fn i_04a20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A210;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_04a210  ($04A210, 8 B, kernel/jmp_to_scheduler.c)
/// $04A210: jmp $518.l
pub fn i_04a210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A216;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04A216: rts
pub fn i_04a216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A218;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PhysicsAir_04a218  ($04A218, 72 B, pow/human_death_049ff2.s)
/// $04A218: jsr $27f08.l
pub fn i_04a218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A21E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A21E);
    cpu.pc = 0x27F08;
    16
}
/// $04A21E: bcc.w $4a22a
pub fn i_04a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A222;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A22A; return 10; }
    8
}
/// $04A222: move.b d3, $70(a6)
pub fn i_04a222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A226;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A226: bra.w $4a240
pub fn i_04a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A22A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A240;
    10
}
/// $04A22A: cmp.b $70(a6), d0
pub fn i_04a22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A22E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04A22E: bne.w $4a23a
pub fn i_04a22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A232;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A23A; return 10; }
    8
}
/// $04A232: move.b d3, $70(a6)
pub fn i_04a232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A236;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A236: bra.w $4a240
pub fn i_04a236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A23A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A240;
    10
}
/// $04A23A: move.b #$ff, $70(a6)
pub fn i_04a23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A240;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A240: bsr.b $4a1c6
pub fn i_04a240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A242;
    cpu.ir = 0x6184;
    cpu.push32(bus, 0x4A242);
    cpu.pc = 0x4A1C6;
    18
}
/// $04A242: tst.b $70(a6)
pub fn i_04a242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A246;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A246: bne.w $4a25c
pub fn i_04a246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A24A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A25C; return 10; }
    8
}
/// $04A24A: move.w #$ff40, $2e(a6)
pub fn i_04a24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A250;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A250: jsr $27bc8.l
pub fn i_04a250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A256);
    cpu.pc = 0x27BC8;
    16
}
/// $04A256: scs.b $70(a6)
pub fn i_04a256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A25A;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A25A: rts
pub fn i_04a25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A25C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A25C: bsr.w $4a1c6
pub fn i_04a25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A260;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A260);
    cpu.pc = 0x4A1C6;
    18
}
// ===== JsrAbsThunk_04a260  ($04A260, 8 B, thunks/jsr_abs_thunks.c)
/// $04A260: jsr $27a92.l
pub fn i_04a260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A266);
    cpu.pc = 0x27A92;
    16
}
/// $04A266: rts
pub fn i_04a266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A268;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PhysicsGround_04a268  ($04A268, 86 B, pow/human_death_049ff2.s)
/// $04A268: jsr $27fd8.l
pub fn i_04a268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A26E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A26E);
    cpu.pc = 0x27FD8;
    16
}
/// $04A26E: bcc.w $4a27a
pub fn i_04a26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A272;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A27A; return 10; }
    8
}
/// $04A272: move.b d3, $70(a6)
pub fn i_04a272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A276;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A276: bra.w $4a290
pub fn i_04a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A27A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A290;
    10
}
/// $04A27A: cmp.b $70(a6), d0
pub fn i_04a27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A27E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04A27E: bne.w $4a28a
pub fn i_04a27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A282;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A28A; return 10; }
    8
}
/// $04A282: move.b d3, $70(a6)
pub fn i_04a282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A286;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A286: bra.w $4a290
pub fn i_04a286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A28A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A290;
    10
}
/// $04A28A: move.b #$ff, $70(a6)
pub fn i_04a28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A290;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A290: bsr.w $4a1c6
pub fn i_04a290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A294;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A294);
    cpu.pc = 0x4A1C6;
    18
}
/// $04A294: tst.b $70(a6)
pub fn i_04a294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A298;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A298: bne.w $4a2a8
pub fn i_04a298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A29C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A2A8; return 10; }
    8
}
/// $04A29C: jsr $27d50.l
pub fn i_04a29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2A2);
    cpu.pc = 0x27D50;
    16
}
/// $04A2A2: scs.b $70(a6)
pub fn i_04a2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2A6;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A2A6: rts
pub fn i_04a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A2A8: bsr.w $4a1c6
pub fn i_04a2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2AC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A2AC);
    cpu.pc = 0x4A1C6;
    18
}
/// $04A2AC: jsr $27a92.l
pub fn i_04a2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2B2);
    cpu.pc = 0x27A92;
    16
}
/// $04A2B2: jsr $27fac.l
pub fn i_04a2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2B8);
    cpu.pc = 0x27FAC;
    16
}
/// $04A2B8: scc.b $70(a6)
pub fn i_04a2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2BC;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A2BC: rts
pub fn i_04a2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PhysicsFall_04a2be  ($04A2BE, 224 B, pow/human_death_049ff2.s)
/// $04A2BE: jsr $27f08.l
pub fn i_04a2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2C4);
    cpu.pc = 0x27F08;
    16
}
/// $04A2C4: bcc.w $4a2d0
pub fn i_04a2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A2D0; return 10; }
    8
}
/// $04A2C8: move.b d3, $70(a6)
pub fn i_04a2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2CC;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A2CC: bra.w $4a2e6
pub fn i_04a2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A2E6;
    10
}
/// $04A2D0: cmp.b $70(a6), d0
pub fn i_04a2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2D4;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04A2D4: bne.w $4a2e0
pub fn i_04a2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A2E0; return 10; }
    8
}
/// $04A2D8: move.b d3, $70(a6)
pub fn i_04a2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2DC;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A2DC: bra.w $4a2e6
pub fn i_04a2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A2E6;
    10
}
/// $04A2E0: move.b #$ff, $70(a6)
pub fn i_04a2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A2E6: tst.b $70(a6)
pub fn i_04a2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2EA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A2EA: bne.w $4a2fe
pub fn i_04a2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A2FE; return 10; }
    8
}
/// $04A2EE: move.w #$ff40, $2e(a6)
pub fn i_04a2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A2F4: jsr $27bc8.l
pub fn i_04a2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2FA);
    cpu.pc = 0x27BC8;
    16
}
/// $04A2FA: scs.b $70(a6)
pub fn i_04a2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2FE;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A2FE: jsr $27a92.l
pub fn i_04a2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A304);
    cpu.pc = 0x27A92;
    16
}
/// $04A304: jsr $27eba.l
pub fn i_04a304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A30A);
    cpu.pc = 0x27EBA;
    16
}
/// $04A30A: scc.b $70(a6)
pub fn i_04a30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A30E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A30E: rts
pub fn i_04a30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A310;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A310: jsr $27fd8.l
pub fn i_04a310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A316);
    cpu.pc = 0x27FD8;
    16
}
/// $04A316: bcc.w $4a322
pub fn i_04a316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A31A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A322; return 10; }
    8
}
/// $04A31A: move.b d3, $70(a6)
pub fn i_04a31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A31E;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A31E: bra.w $4a338
pub fn i_04a31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A322;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A338;
    10
}
/// $04A322: cmp.b $70(a6), d0
pub fn i_04a322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A326;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04A326: bne.w $4a332
pub fn i_04a326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A32A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A332; return 10; }
    8
}
/// $04A32A: move.b d3, $70(a6)
pub fn i_04a32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A32E;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A32E: bra.w $4a338
pub fn i_04a32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A332;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A338;
    10
}
/// $04A332: move.b #$ff, $70(a6)
pub fn i_04a332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A338;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A338: tst.b $70(a6)
pub fn i_04a338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A33C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A33C: bne.w $4a34c
pub fn i_04a33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A340;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A34C; return 10; }
    8
}
/// $04A340: jsr $27d50.l
pub fn i_04a340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A346);
    cpu.pc = 0x27D50;
    16
}
/// $04A346: scs.b $70(a6)
pub fn i_04a346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A34A;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A34A: rts
pub fn i_04a34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A34C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A34C: jsr $27a92.l
pub fn i_04a34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A352);
    cpu.pc = 0x27A92;
    16
}
/// $04A352: jsr $27fac.l
pub fn i_04a352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A358);
    cpu.pc = 0x27FAC;
    16
}
/// $04A358: scc.b $70(a6)
pub fn i_04a358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A35C;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A35C: rts
pub fn i_04a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A35E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A35E: move.w $22(a6), d0
pub fn i_04a35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A362;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A362: addi.w #$20, d0
pub fn i_04a362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A366;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A366: cmpi.w #$180, d0
pub fn i_04a366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A36A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x180, Size::Word, false);
    8
}
/// $04A36A: bcs.w $4a372
pub fn i_04a36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A36E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4A372; return 10; }
    8
}
/// $04A36E: jmp $4a210(pc)
pub fn i_04a36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A372;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A210;
    8
}
/// $04A372: cmpi.w #$e0, $24(a6)
pub fn i_04a372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A378;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE0, Size::Word, false);
    16
}
/// $04A378: bcc.w $4a380
pub fn i_04a378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A37C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A380; return 10; }
    8
}
/// $04A37C: jmp $4a210(pc)
pub fn i_04a37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A380;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A210;
    8
}
/// $04A380: rts
pub fn i_04a380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A382;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A382: lea.l $4a388(pc), a1
pub fn i_04a382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A386;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A388;
    16
}
/// $04A386: move.l a1, (a6)
pub fn i_04a386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A388;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A388: bsr.w $4a218
pub fn i_04a388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A38C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A38C);
    cpu.pc = 0x4A218;
    18
}
/// $04A38C: jsr $28d70.l
pub fn i_04a38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A392;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A392);
    cpu.pc = 0x28D70;
    16
}
/// $04A392: bcc.w $4a39c
pub fn i_04a392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A396;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A39C; return 10; }
    8
}
/// $04A396: lea.l $4a39e(pc), a1
pub fn i_04a396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A39A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A39E;
    16
}
/// $04A39A: move.l a1, (a6)
pub fn i_04a39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A39C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A39C: bra.b $4a35e
pub fn i_04a39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A39E;
    cpu.ir = 0x60C0;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_FadeOut_04a39e  ($04A39E, 106 B, pow/human_death_049ff2.s)
/// $04A39E: move.b #$a, $5c(a6)
pub fn i_04a39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A3A4: lea.l $4a3aa(pc), a1
pub fn i_04a3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A3AA;
    16
}
/// $04A3A8: move.l a1, (a6)
pub fn i_04a3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A3AA: bsr.w $4a218
pub fn i_04a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3AE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A3AE);
    cpu.pc = 0x4A218;
    18
}
/// $04A3AE: jsr $28d70.l
pub fn i_04a3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A3B4);
    cpu.pc = 0x28D70;
    16
}
/// $04A3B4: subq.b #$1, $5c(a6)
pub fn i_04a3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3B8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A3B8: bne.w $4a3c2
pub fn i_04a3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A3C2; return 10; }
    8
}
/// $04A3BC: lea.l $4a3c4(pc), a1
pub fn i_04a3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A3C4;
    16
}
/// $04A3C0: move.l a1, (a6)
pub fn i_04a3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A3C2: bra.b $4a35e
pub fn i_04a3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3C4;
    cpu.ir = 0x609A;
    cpu.pc = 0x4A35E;
    10
}
/// $04A3C4: move.b #$a, $5c(a6)
pub fn i_04a3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A3CA: lea.l $4a3d0(pc), a1
pub fn i_04a3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A3D0;
    16
}
/// $04A3CE: move.l a1, (a6)
pub fn i_04a3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A3D0: jsr $4a218(pc)
pub fn i_04a3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A3D4);
    cpu.pc = 0x4A218;
    16
}
/// $04A3D4: subq.b #$1, $5c(a6)
pub fn i_04a3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3D8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A3D8: bne.w $4a3e4
pub fn i_04a3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A3E4; return 10; }
    8
}
/// $04A3DC: jmp $4a210(pc)
pub fn i_04a3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3E0;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A210;
    8
}
/// $04A3E0: bra.w $4a3f4
pub fn i_04a3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A3F4;
    10
}
/// $04A3E4: btst.b #$0, $5c(a6)
pub fn i_04a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04A3EA: bne.w $4a3f4
pub fn i_04a3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A3F4; return 10; }
    8
}
/// $04A3EE: jsr $28d70.l
pub fn i_04a3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A3F4);
    cpu.pc = 0x28D70;
    16
}
/// $04A3F4: bra.w $4a35e
pub fn i_04a3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
/// $04A3F8: bsr.w $4aae0
pub fn i_04a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3FC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A3FC);
    cpu.pc = 0x4AAE0;
    18
}
/// $04A3FC: tst.b $71(a6)
pub fn i_04a3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A400;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A400: bne.w $4a420
pub fn i_04a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A404;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A420; return 10; }
    8
}
/// $04A404: bra.w $4a4c8
pub fn i_04a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A408;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A4C8;
    10
}
// ===== HumanDeath_TumbleBackStart_04a420  ($04A420, 24 B, pow/human_death_049ff2.s)
/// $04A420: bsr.w $4a202
pub fn i_04a420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A424;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A424);
    cpu.pc = 0x4A202;
    18
}
/// $04A424: bcs.w $4a620
pub fn i_04a424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A428;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4A620; return 10; }
    8
}
/// $04A428: jsr $5e9b6.l
pub fn i_04a428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A42E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A42E);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A42E: tst.w d0
pub fn i_04a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A430;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A430: bmi.w $4a568
pub fn i_04a430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A434;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4A568; return 10; }
    8
}
/// $04A434: clr.b $72(a6)
pub fn i_04a434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A438;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== HumanDeath_TumbleBack_04a438  ($04A438, 106 B, pow/human_death_049ff2.s)
/// $04A438: lea.l $4a408(pc), a0
pub fn i_04a438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A43C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4A408;
    16
}
/// $04A43C: moveq #$0, d0
pub fn i_04a43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A43E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A43E: move.b $72(a6), d0
pub fn i_04a43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A442;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04A442: lsl.w #$2, d0
pub fn i_04a442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A444;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04A444: move.l (a0, d0.w), d0
pub fn i_04a444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A448;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $04A448: beq.w $4a4a2
pub fn i_04a448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A44C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A4A2; return 10; }
    8
}
/// $04A44C: movea.l d0, a0
pub fn i_04a44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A44E;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04A44E: jsr $28cd4.l
pub fn i_04a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A454);
    cpu.pc = 0x28CD4;
    16
}
/// $04A454: addq.b #$1, $72(a6)
pub fn i_04a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A458;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A458: bclr.b #$3, $13(a6)
pub fn i_04a458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A45E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A45E: move.w #$ff80, d0
pub fn i_04a45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A462;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A462: bsr.w $4a1f0
pub fn i_04a462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A466;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A466);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A466: addi.w #$80, $2a(a6)
pub fn i_04a466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A46C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A46C: addq.w #$2, $24(a6)
pub fn i_04a46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A470;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A470: lea.l $4a476(pc), a1
pub fn i_04a470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A474;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A476;
    16
}
/// $04A474: move.l a1, (a6)
pub fn i_04a474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A476;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A476: bsr.w $4a218
pub fn i_04a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A47A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A47A);
    cpu.pc = 0x4A218;
    18
}
/// $04A47A: jsr $28d70.l
pub fn i_04a47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A480;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A480);
    cpu.pc = 0x28D70;
    16
}
/// $04A480: bcc.w $4a49e
pub fn i_04a480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A484;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A49E; return 10; }
    8
}
/// $04A484: jsr $2870a.l
pub fn i_04a484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A48A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A48A);
    cpu.pc = 0x2870A;
    16
}
/// $04A48A: bcc.w $4a498
pub fn i_04a48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A48E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A498; return 10; }
    8
}
/// $04A48E: lea.l $4a438(pc), a1
pub fn i_04a48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A492;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A438;
    16
}
/// $04A492: move.l a1, (a6)
pub fn i_04a492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A494;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A494: bra.w $4a49e
pub fn i_04a494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A498;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A49E;
    10
}
/// $04A498: lea.l $4a4a2(pc), a1
pub fn i_04a498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A49C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A4A2;
    16
}
/// $04A49C: move.l a1, (a6)
pub fn i_04a49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A49E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A49E: bra.w $4a35e
pub fn i_04a49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_TumbleBackLand_04a4a2  ($04A4A2, 14 B, pow/human_death_049ff2.s)
/// $04A4A2: lea.l $4b6b8(pc), a0
pub fn i_04a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4A6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B6B8;
    16
}
/// $04A4A6: jsr $28cd4.l
pub fn i_04a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A4AC);
    cpu.pc = 0x28CD4;
    16
}
/// $04A4AC: bra.w $4a382
pub fn i_04a4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
// ===== HumanDeath_TumbleFwdStart_04a4c8  ($04A4C8, 24 B, pow/human_death_049ff2.s)
/// $04A4C8: bsr.w $4a202
pub fn i_04a4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4CC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A4CC);
    cpu.pc = 0x4A202;
    18
}
/// $04A4CC: bcs.w $4a620
pub fn i_04a4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4D0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4A620; return 10; }
    8
}
/// $04A4D0: jsr $5e9b6.l
pub fn i_04a4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A4D6);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A4D6: tst.w d0
pub fn i_04a4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4D8;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A4D8: bmi.w $4a568
pub fn i_04a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4DC;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4A568; return 10; }
    8
}
/// $04A4DC: clr.b $72(a6)
pub fn i_04a4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4E0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== HumanDeath_TumbleFwd_04a4e0  ($04A4E0, 106 B, pow/human_death_049ff2.s)
/// $04A4E0: lea.l $4a4b0(pc), a0
pub fn i_04a4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4E4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4A4B0;
    16
}
/// $04A4E4: moveq #$0, d0
pub fn i_04a4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4E6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A4E6: move.b $72(a6), d0
pub fn i_04a4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04A4EA: lsl.w #$2, d0
pub fn i_04a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4EC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04A4EC: move.l (a0, d0.w), d0
pub fn i_04a4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4F0;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $04A4F0: beq.w $4a54a
pub fn i_04a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A54A; return 10; }
    8
}
/// $04A4F4: movea.l d0, a0
pub fn i_04a4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4F6;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04A4F6: jsr $28cd4.l
pub fn i_04a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A4FC);
    cpu.pc = 0x28CD4;
    16
}
/// $04A4FC: addq.b #$1, $72(a6)
pub fn i_04a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A500;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A500: bclr.b #$3, $13(a6)
pub fn i_04a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A506;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A506: move.w #$80, d0
pub fn i_04a506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A50A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A50A: bsr.w $4a1f0
pub fn i_04a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A50E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A50E);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A50E: addi.w #$80, $2a(a6)
pub fn i_04a50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A514;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A514: addq.w #$2, $24(a6)
pub fn i_04a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A518;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A518: lea.l $4a51e(pc), a1
pub fn i_04a518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A51C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A51E;
    16
}
/// $04A51C: move.l a1, (a6)
pub fn i_04a51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A51E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A51E: bsr.w $4a218
pub fn i_04a51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A522;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A522);
    cpu.pc = 0x4A218;
    18
}
/// $04A522: jsr $28d70.l
pub fn i_04a522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A528;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A528);
    cpu.pc = 0x28D70;
    16
}
/// $04A528: bcc.w $4a546
pub fn i_04a528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A52C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A546; return 10; }
    8
}
/// $04A52C: jsr $2870a.l
pub fn i_04a52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A532);
    cpu.pc = 0x2870A;
    16
}
/// $04A532: bcc.w $4a540
pub fn i_04a532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A536;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A540; return 10; }
    8
}
/// $04A536: lea.l $4a4e0(pc), a1
pub fn i_04a536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A53A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A4E0;
    16
}
/// $04A53A: move.l a1, (a6)
pub fn i_04a53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A53C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A53C: bra.w $4a546
pub fn i_04a53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A540;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A546;
    10
}
/// $04A540: lea.l $4a54a(pc), a1
pub fn i_04a540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A544;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A54A;
    16
}
/// $04A544: move.l a1, (a6)
pub fn i_04a544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A546;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A546: bra.w $4a35e
pub fn i_04a546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A54A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_Collapse_04a54a  ($04A54A, 296 B, pow/human_death_049ff2.s)
/// $04A54A: jsr $5e9b6.l
pub fn i_04a54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A550);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A550: lea.l $4b57c(pc), a0
pub fn i_04a550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A554;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B57C;
    16
}
/// $04A554: tst.w d0
pub fn i_04a554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A556;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A556: bpl.w $4a55e
pub fn i_04a556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A55A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4A55E; return 10; }
    8
}
/// $04A55A: lea.l $4b61a(pc), a0
pub fn i_04a55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A55E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B61A;
    16
}
/// $04A55E: jsr $28cd4.l
pub fn i_04a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A564);
    cpu.pc = 0x28CD4;
    16
}
/// $04A564: bra.w $4a382
pub fn i_04a564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A568;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A568: tst.b $71(a6)
pub fn i_04a568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A56C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A56C: beq.w $4a586
pub fn i_04a56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A570;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A586; return 10; }
    8
}
/// $04A570: lea.l $4b1f6(pc), a0
pub fn i_04a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A574;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B1F6;
    16
}
/// $04A574: jsr $28cd4.l
pub fn i_04a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A57A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A57A);
    cpu.pc = 0x28CD4;
    16
}
/// $04A57A: move.w #$fc00, d0
pub fn i_04a57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A57E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A57E: bsr.w $4a1f0
pub fn i_04a57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A582;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A582);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A582: bra.w $4a598
pub fn i_04a582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A586;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A598;
    10
}
/// $04A586: lea.l $4b28c(pc), a0
pub fn i_04a586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A58A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B28C;
    16
}
/// $04A58A: jsr $28cd4.l
pub fn i_04a58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A590);
    cpu.pc = 0x28CD4;
    16
}
/// $04A590: move.w #$400, d0
pub fn i_04a590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A594;
    cpu.ir = 0x303C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A594: bsr.w $4a1f0
pub fn i_04a594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A598;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A598);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A598: move.w #$100, $2a(a6)
pub fn i_04a598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A59E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A59E: addq.w #$2, $24(a6)
pub fn i_04a59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5A2;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A5A2: bra.w $4a382
pub fn i_04a5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A5A6: lea.l $4b34a(pc), a0
pub fn i_04a5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5AA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B34A;
    16
}
/// $04A5AA: jsr $28cd4.l
pub fn i_04a5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A5B0);
    cpu.pc = 0x28CD4;
    16
}
/// $04A5B0: bra.w $4a382
pub fn i_04a5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A5B4: lea.l $4ad90(pc), a0
pub fn i_04a5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5B8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AD90;
    16
}
/// $04A5B8: tst.b $71(a6)
pub fn i_04a5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5BC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A5BC: beq.w $4a5c4
pub fn i_04a5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A5C4; return 10; }
    8
}
/// $04A5C0: lea.l $4ae22(pc), a0
pub fn i_04a5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5C4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AE22;
    16
}
/// $04A5C4: jsr $28cd4.l
pub fn i_04a5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A5CA);
    cpu.pc = 0x28CD4;
    16
}
/// $04A5CA: bra.w $4a382
pub fn i_04a5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A5CE: jsr $27f60.l
pub fn i_04a5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A5D4);
    cpu.pc = 0x27F60;
    16
}
/// $04A5D4: scc.b $70(a6)
pub fn i_04a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5D8;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A5D8: jsr $13600.l
pub fn i_04a5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A5DE);
    cpu.pc = 0x13600;
    16
}
/// $04A5DE: move.w #$154, d1
pub fn i_04a5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x154;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A5E2: tst.b $10fd8f.l
pub fn i_04a5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5E8;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04A5E8: bne.w $4a5f0
pub fn i_04a5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A5F0; return 10; }
    8
}
/// $04A5EC: move.w #$155, d1
pub fn i_04a5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x155;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A5F0: jsr $236e.l
pub fn i_04a5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A5F6);
    cpu.pc = 0x236E;
    16
}
/// $04A5F6: lea.l $4b13c(pc), a0
pub fn i_04a5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5FA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B13C;
    16
}
/// $04A5FA: jsr $28cd4.l
pub fn i_04a5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A600);
    cpu.pc = 0x28CD4;
    16
}
/// $04A600: bra.w $4a382
pub fn i_04a600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A604;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A604: lea.l $4b136(pc), a0
pub fn i_04a604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A608;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B136;
    16
}
/// $04A608: jsr $28cd4.l
pub fn i_04a608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A60E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A60E);
    cpu.pc = 0x28CD4;
    16
}
/// $04A60E: bra.w $4a382
pub fn i_04a60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A612;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A612: lea.l $4b78c(pc), a0
pub fn i_04a612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A616;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B78C;
    16
}
/// $04A616: jsr $28cd4.l
pub fn i_04a616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A61C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A61C);
    cpu.pc = 0x28CD4;
    16
}
/// $04A61C: bra.w $4a62a
pub fn i_04a61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A620;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A62A;
    10
}
/// $04A620: lea.l $4b74c(pc), a0
pub fn i_04a620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A624;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B74C;
    16
}
/// $04A624: jsr $28cd4.l
pub fn i_04a624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A62A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A62A);
    cpu.pc = 0x28CD4;
    16
}
/// $04A62A: tst.b $71(a6)
pub fn i_04a62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A62E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A62E: bne.w $4a638
pub fn i_04a62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A632;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A638; return 10; }
    8
}
/// $04A632: bchg.b #$0, $3a(a6)
pub fn i_04a632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A638;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A638: move.w #$ff38, d0
pub fn i_04a638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A63C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A63C: bsr.w $4a1f0
pub fn i_04a63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A640;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A640);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A640: move.w #$ff40, $2e(a6)
pub fn i_04a640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A646;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A646: move.w #$0, $2a(a6)
pub fn i_04a646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A64C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A64C: lea.l $4a652(pc), a1
pub fn i_04a64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A650;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A652;
    16
}
/// $04A650: move.l a1, (a6)
pub fn i_04a650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A652;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A652: bsr.w $4a268
pub fn i_04a652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A656;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A656);
    cpu.pc = 0x4A268;
    18
}
/// $04A656: jsr $28d70.l
pub fn i_04a656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A65C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A65C);
    cpu.pc = 0x28D70;
    16
}
/// $04A65C: bcc.w $4a66e
pub fn i_04a65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A660;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A66E; return 10; }
    8
}
/// $04A660: tst.b $70(a6)
pub fn i_04a660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A664;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A664: beq.w $4a66e
pub fn i_04a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A668;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A66E; return 10; }
    8
}
/// $04A668: lea.l $4a672(pc), a1
pub fn i_04a668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A66C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A672;
    16
}
/// $04A66C: move.l a1, (a6)
pub fn i_04a66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A66E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A66E: bra.w $4a35e
pub fn i_04a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A672;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_Knockdown_04a672  ($04A672, 64 B, pow/human_death_049ff2.s)
/// $04A672: lea.l $4b7b8(pc), a0
pub fn i_04a672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A676;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B7B8;
    16
}
/// $04A676: jsr $28cd4.l
pub fn i_04a676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A67C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A67C);
    cpu.pc = 0x28CD4;
    16
}
/// $04A67C: bra.w $4a382
pub fn i_04a67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A680;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A680: jsr $5e9b6.l
pub fn i_04a680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A686;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A686);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A686: tst.w d0
pub fn i_04a686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A688;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A688: bmi.w $4a612
pub fn i_04a688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A68C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4A612; return 10; }
    8
}
/// $04A68C: lea.l $4acfe(pc), a0
pub fn i_04a68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A690;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4ACFE;
    16
}
/// $04A690: jsr $28cd4.l
pub fn i_04a690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A696);
    cpu.pc = 0x28CD4;
    16
}
/// $04A696: lea.l $4a69c(pc), a1
pub fn i_04a696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A69A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A69C;
    16
}
/// $04A69A: move.l a1, (a6)
pub fn i_04a69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A69C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A69C: bsr.w $4a268
pub fn i_04a69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6A0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A6A0);
    cpu.pc = 0x4A268;
    18
}
/// $04A6A0: jsr $28d70.l
pub fn i_04a6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A6A6);
    cpu.pc = 0x28D70;
    16
}
/// $04A6A6: bcc.w $4a6ae
pub fn i_04a6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A6AE; return 10; }
    8
}
/// $04A6AA: jmp $4a210(pc)
pub fn i_04a6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6AE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A210;
    8
}
/// $04A6AE: bra.w $4a35e
pub fn i_04a6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_InitBurst_04a6b2  ($04A6B2, 230 B, pow/human_death_049ff2.s)
/// $04A6B2: move.w #$0, d0
pub fn i_04a6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A6B6: jsr $5dca4.l
pub fn i_04a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A6BC);
    cpu.pc = 0x5DCA4;
    16
}
/// $04A6BC: move.w d0, $28(a6)
pub fn i_04a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6C0: move.w #$cc6, $2a(a6)
pub fn i_04a6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xCC6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6C6: move.w #$ff93, $2e(a6)
pub fn i_04a6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF93;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6CC: move.w #$0, $2c(a6)
pub fn i_04a6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6D2: jsr $5e9b6.l
pub fn i_04a6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A6D8);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A6D8: andi.w #$3ff, d0
pub fn i_04a6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6DC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A6DC: move.w $22(a6), d1
pub fn i_04a6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6E0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A6E0: sub.w $54(a6), d1
pub fn i_04a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6E4;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04A6E4: smi.b d1
pub fn i_04a6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6E6;
    cpu.ir = 0x5BC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A6E6: ext.w d1
pub fn i_04a6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6E8;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A6E8: eor.w d1, d0
pub fn i_04a6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6EA;
    cpu.ir = 0xB340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04A6EA: move.w d0, $28(a6)
pub fn i_04a6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6EE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6EE: jsr $5e9b6.l
pub fn i_04a6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A6F4);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A6F4: andi.w #$3ff, d0
pub fn i_04a6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A6F8: add.w d0, $2a(a6)
pub fn i_04a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6FC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A6FC: clr.b $70(a6)
pub fn i_04a6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A700;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A700: rts
pub fn i_04a700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A702;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A702: jsr $5e9b6.l
pub fn i_04a702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A708);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A708: andi.b #$7f, d0
pub fn i_04a708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A70C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04A70C: beq.w $4a802
pub fn i_04a70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A710;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A802; return 10; }
    8
}
/// $04A710: jsr $5e9b6.l
pub fn i_04a710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A716);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A716: tst.w d0
pub fn i_04a716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A718;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A718: bmi.w $4a3f8
pub fn i_04a718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A71C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4A3F8; return 10; }
    8
}
/// $04A71C: jsr $5e9b6.l
pub fn i_04a71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A722;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A722);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A722: andi.w #$3, d0
pub fn i_04a722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A726;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A726: beq.w $4a7b0
pub fn i_04a726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A72A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A7B0; return 10; }
    8
}
/// $04A72A: bra.w $4a732
pub fn i_04a72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A72E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A732;
    10
}
/// $04A72E: addq.w #$4, $24(a6)
pub fn i_04a72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A732;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A732: lea.l $4af08(pc), a0
pub fn i_04a732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A736;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AF08;
    16
}
/// $04A736: jsr $28cd4.l
pub fn i_04a736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A73C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A73C);
    cpu.pc = 0x28CD4;
    16
}
/// $04A73C: bsr.w $4a6b2
pub fn i_04a73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A740;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A740);
    cpu.pc = 0x4A6B2;
    18
}
/// $04A740: lea.l $ffff.w, a0
pub fn i_04a740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A744;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04A744: move.l a0, $48(a6)
pub fn i_04a744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A748;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04A748: bclr.b #$3, $13(a6)
pub fn i_04a748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A74E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A74E: lea.l $4a754(pc), a1
pub fn i_04a74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A752;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A754;
    16
}
/// $04A752: move.l a1, (a6)
pub fn i_04a752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A754;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A754: move.w $2a(a6), d0
pub fn i_04a754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A758;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A758: bpl.w $4a76a
pub fn i_04a758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A75C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4A76A; return 10; }
    8
}
/// $04A75C: lea.l $4aeb4(pc), a0
pub fn i_04a75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A760;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AEB4;
    16
}
/// $04A760: move.l a0, $48(a6)
pub fn i_04a760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A764;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04A764: lea.l $4a76a(pc), a1
pub fn i_04a764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A76A;
    16
}
/// $04A768: move.l a1, (a6)
pub fn i_04a768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A76A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A76A: jsr $4a268(pc)
pub fn i_04a76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A76E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A76E);
    cpu.pc = 0x4A268;
    16
}
/// $04A76E: jsr $5e9b6.l
pub fn i_04a76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A774);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A774: jsr $4aba8(pc)
pub fn i_04a774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A778;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A778);
    cpu.pc = 0x4ABA8;
    16
}
/// $04A778: jsr $2870a.l
pub fn i_04a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A77E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A77E);
    cpu.pc = 0x2870A;
    16
}
/// $04A77E: bcc.w $4a786
pub fn i_04a77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A782;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A786; return 10; }
    8
}
/// $04A782: bsr.w $4a044
pub fn i_04a782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A786;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A786);
    cpu.pc = 0x4A044;
    18
}
/// $04A786: tst.b $70(a6)
pub fn i_04a786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A78A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A78A: beq.w $4a794
pub fn i_04a78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A78E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A794; return 10; }
    8
}
/// $04A78E: lea.l $4a798(pc), a1
pub fn i_04a78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A792;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A798;
    16
}
/// $04A792: move.l a1, (a6)
pub fn i_04a792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A794;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A794: bra.w $4a35e
pub fn i_04a794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A798;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_BurstLand_04a798  ($04A798, 92 B, pow/human_death_049ff2.s)
/// $04A798: lea.l $4af9e(pc), a0
pub fn i_04a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A79C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AF9E;
    16
}
/// $04A79C: jsr $28cd4.l
pub fn i_04a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7A2);
    cpu.pc = 0x28CD4;
    16
}
/// $04A7A2: bra.w $4a382
pub fn i_04a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A7A6: jsr $27f60.l
pub fn i_04a7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7AC);
    cpu.pc = 0x27F60;
    16
}
/// $04A7AC: scc.b $70(a6)
pub fn i_04a7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7B0;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A7B0: addq.w #$4, $24(a6)
pub fn i_04a7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7B4;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A7B4: lea.l $4b078(pc), a0
pub fn i_04a7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7B8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B078;
    16
}
/// $04A7B8: jsr $28cd4.l
pub fn i_04a7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7BE);
    cpu.pc = 0x28CD4;
    16
}
/// $04A7BE: bsr.w $4a6b2
pub fn i_04a7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7C2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A7C2);
    cpu.pc = 0x4A6B2;
    18
}
/// $04A7C2: jsr $13600.l
pub fn i_04a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7C8);
    cpu.pc = 0x13600;
    16
}
/// $04A7C8: move.w #$19, d1
pub fn i_04a7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7CC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A7CC: jsr $236e.l
pub fn i_04a7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7D2);
    cpu.pc = 0x236E;
    16
}
/// $04A7D2: lea.l $4a7d8(pc), a1
pub fn i_04a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A7D8;
    16
}
/// $04A7D6: move.l a1, (a6)
pub fn i_04a7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A7D8: jsr $4a268(pc)
pub fn i_04a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A7DC);
    cpu.pc = 0x4A268;
    16
}
/// $04A7DC: jsr $28d70.l
pub fn i_04a7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7E2);
    cpu.pc = 0x28D70;
    16
}
/// $04A7E2: tst.b $70(a6)
pub fn i_04a7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7E6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A7E6: beq.w $4a7f0
pub fn i_04a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A7F0; return 10; }
    8
}
/// $04A7EA: lea.l $4a7f4(pc), a1
pub fn i_04a7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A7F4;
    16
}
/// $04A7EE: move.l a1, (a6)
pub fn i_04a7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A7F0: bra.w $4a35e
pub fn i_04a7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_Launched_04a7f4  ($04A7F4, 444 B, pow/human_death_049ff2.s)
/// $04A7F4: lea.l $4b0c6(pc), a0
pub fn i_04a7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7F8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B0C6;
    16
}
/// $04A7F8: jsr $28cd4.l
pub fn i_04a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7FE);
    cpu.pc = 0x28CD4;
    16
}
/// $04A7FE: bra.w $4a382
pub fn i_04a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A802;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A802: jsr $27f60.l
pub fn i_04a802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A808);
    cpu.pc = 0x27F60;
    16
}
/// $04A808: scc.b $70(a6)
pub fn i_04a808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A80C;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A80C: jsr $13600.l
pub fn i_04a80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A812);
    cpu.pc = 0x13600;
    16
}
/// $04A812: move.w #$8a, d1
pub fn i_04a812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A816;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A816: jsr $236e.l
pub fn i_04a816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A81C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A81C);
    cpu.pc = 0x236E;
    16
}
/// $04A81C: move.w #$fffc, d0
pub fn i_04a81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A820;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A820: jsr $5dca4.l
pub fn i_04a820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A826);
    cpu.pc = 0x5DCA4;
    16
}
/// $04A826: move.w d0, $28(a6)
pub fn i_04a826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A82A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A82A: move.w #$870, $2a(a6)
pub fn i_04a82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A830;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x870;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A830: move.w #$ffb8, $2e(a6)
pub fn i_04a830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A836;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A836: move.w #$0, $2c(a6)
pub fn i_04a836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A83C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A83C: lea.l $4b7ec(pc), a0
pub fn i_04a83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A840;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B7EC;
    16
}
/// $04A840: jsr $28cd4.l
pub fn i_04a840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A846);
    cpu.pc = 0x28CD4;
    16
}
/// $04A846: move.w #$2800, $5c(a6)
pub fn i_04a846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A84C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A84C: move.w #$ffff, $38(a6)
pub fn i_04a84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A852;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A852: bset.b #$6, $12(a6)
pub fn i_04a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A858;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A858: lea.l $4a85e(pc), a1
pub fn i_04a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A85C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A85E;
    16
}
/// $04A85C: move.l a1, (a6)
pub fn i_04a85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A85E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A85E: move.w $5c(a6), d0
pub fn i_04a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A862;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A862: move.w d0, d1
pub fn i_04a862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A864;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A864: lsr.w #$5, d0
pub fn i_04a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A866;
    cpu.ir = 0xEA48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04A866: add.w d0, d1
pub fn i_04a866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A868;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04A868: bcc.w $4a86e
pub fn i_04a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A86C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A86E; return 10; }
    8
}
/// $04A86C: moveq #$ff, d1
pub fn i_04a86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A86E;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A86E: move.w d1, $5c(a6)
pub fn i_04a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A872;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A872: lsr.w #$8, d1
pub fn i_04a872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A874;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $04A874: move.b d1, $32(a6)
pub fn i_04a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A878;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A878: move.b d1, $33(a6)
pub fn i_04a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A87C;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A87C: jsr $27cee.l
pub fn i_04a87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A882);
    cpu.pc = 0x27CEE;
    16
}
/// $04A882: jsr $28d70.l
pub fn i_04a882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A888);
    cpu.pc = 0x28D70;
    16
}
/// $04A888: bra.w $4a35e
pub fn i_04a888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A88C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
/// $04A88C: jsr $27f60.l
pub fn i_04a88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A892);
    cpu.pc = 0x27F60;
    16
}
/// $04A892: scc.b $70(a6)
pub fn i_04a892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A896;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A896: jmp $4a7b0(pc)
pub fn i_04a896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A89A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A7B0;
    8
}
/// $04A89A: lea.l $4b1f6(pc), a0
pub fn i_04a89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A89E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B1F6;
    16
}
/// $04A89E: jsr $28cd4.l
pub fn i_04a89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A8A4);
    cpu.pc = 0x28CD4;
    16
}
/// $04A8A4: movea.l $50(a6), a0
pub fn i_04a8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8A8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04A8A8: cmpi.w #$0, $5c(a0)
pub fn i_04a8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8AE;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04A8AE: beq.w $4a8bc
pub fn i_04a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A8BC; return 10; }
    8
}
/// $04A8B2: bset.b #$0, $3a(a6)
pub fn i_04a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8B8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A8B8: bra.w $4a8c2
pub fn i_04a8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A8C2;
    10
}
/// $04A8BC: bclr.b #$0, $3a(a6)
pub fn i_04a8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8C2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A8C2: move.w #$f800, d0
pub fn i_04a8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A8C6: bsr.w $4a1f0
pub fn i_04a8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8CA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A8CA);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A8CA: move.w #$100, $2a(a6)
pub fn i_04a8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A8D0: addq.w #$2, $24(a6)
pub fn i_04a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8D4;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A8D4: bsr.w $4aad2
pub fn i_04a8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8D8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A8D8);
    cpu.pc = 0x4AAD2;
    18
}
/// $04A8D8: lea.l $4a8de(pc), a1
pub fn i_04a8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A8DE;
    16
}
/// $04A8DC: move.l a1, (a6)
pub fn i_04a8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A8DE: bsr.w $4a218
pub fn i_04a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A8E2);
    cpu.pc = 0x4A218;
    18
}
/// $04A8E2: bsr.w $4a2be
pub fn i_04a8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8E6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A8E6);
    cpu.pc = 0x4A2BE;
    18
}
/// $04A8E6: jsr $28d70.l
pub fn i_04a8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A8EC);
    cpu.pc = 0x28D70;
    16
}
/// $04A8EC: bcc.w $4a8f6
pub fn i_04a8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A8F6; return 10; }
    8
}
/// $04A8F0: lea.l $4a39e(pc), a1
pub fn i_04a8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A39E;
    16
}
/// $04A8F4: move.l a1, (a6)
pub fn i_04a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A8F6: bra.w $4a35e
pub fn i_04a8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
/// $04A8FA: clr.w $28(a6)
pub fn i_04a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A8FE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A8FE: lea.l $4af08(pc), a0
pub fn i_04a8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A902;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AF08;
    16
}
/// $04A902: jsr $28cd4.l
pub fn i_04a902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A908;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A908);
    cpu.pc = 0x28CD4;
    16
}
/// $04A908: move.w #$800, $2a(a6)
pub fn i_04a908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A90E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A90E: addq.w #$2, $24(a6)
pub fn i_04a90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A912;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A912: clr.w $28(a6)
pub fn i_04a912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A916;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A916: bsr.w $4aad2
pub fn i_04a916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A91A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A91A);
    cpu.pc = 0x4AAD2;
    18
}
/// $04A91A: lea.l $4a920(pc), a1
pub fn i_04a91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A91E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A920;
    16
}
/// $04A91E: move.l a1, (a6)
pub fn i_04a91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A920;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A920: jsr $4a218(pc)
pub fn i_04a920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A924;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A924);
    cpu.pc = 0x4A218;
    16
}
/// $04A924: clr.w $2e(a6)
pub fn i_04a924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A928;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A928: bsr.w $4a2be
pub fn i_04a928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A92C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A92C);
    cpu.pc = 0x4A2BE;
    18
}
/// $04A92C: jsr $5e9b6.l
pub fn i_04a92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A932);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A932: jsr $4aba8(pc)
pub fn i_04a932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A936;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A936);
    cpu.pc = 0x4ABA8;
    16
}
/// $04A936: tst.b $70(a6)
pub fn i_04a936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A93A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A93A: beq.w $4a944
pub fn i_04a93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A93E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A944; return 10; }
    8
}
/// $04A93E: lea.l $4a798(pc), a1
pub fn i_04a93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A798;
    16
}
/// $04A942: move.l a1, (a6)
pub fn i_04a942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A944: bra.w $4a35e
pub fn i_04a944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A948;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
/// $04A948: move.w #$8000, d0
pub fn i_04a948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A94C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A94C: jsr $28134.l
pub fn i_04a94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A952);
    cpu.pc = 0x28134;
    16
}
/// $04A952: andi.w #$ffe3, $38(a6)
pub fn i_04a952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A958;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A958: ori.w #$0, $38(a6)
pub fn i_04a958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A95E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A95E: jsr $5e9b6.l
pub fn i_04a95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A964);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A964: andi.b #$3, d0
pub fn i_04a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A968;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04A968: beq.w $4a9be
pub fn i_04a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A96C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A9BE; return 10; }
    8
}
/// $04A96C: lea.l $4b82a(pc), a0
pub fn i_04a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A970;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B82A;
    16
}
/// $04A970: jsr $28cd4.l
pub fn i_04a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A976);
    cpu.pc = 0x28CD4;
    16
}
/// $04A976: move.w #$1e, $30(a6)
pub fn i_04a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A97C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A97C: lea.l $4a982(pc), a1
pub fn i_04a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A980;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A982;
    16
}
/// $04A980: move.l a1, (a6)
pub fn i_04a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A982;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A982: bsr.w $4a218
pub fn i_04a982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A986;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A986);
    cpu.pc = 0x4A218;
    18
}
/// $04A986: jsr $28d70.l
pub fn i_04a986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A98C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A98C);
    cpu.pc = 0x28D70;
    16
}
/// $04A98C: move.w $30(a6), d0
pub fn i_04a98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A990;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A990: beq.w $4a99e
pub fn i_04a990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A994;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A99E; return 10; }
    8
}
/// $04A994: subq.w #$1, d0
pub fn i_04a994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A996;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04A996: move.w d0, $30(a6)
pub fn i_04a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A99A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A99A: bra.w $4a9ac
pub fn i_04a99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A99E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A9AC;
    10
}
/// $04A99E: tst.b $70(a6)
pub fn i_04a99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9A2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A9A2: beq.w $4a9ac
pub fn i_04a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A9AC; return 10; }
    8
}
/// $04A9A6: lea.l $4a9b0(pc), a1
pub fn i_04a9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A9B0;
    16
}
/// $04A9AA: move.l a1, (a6)
pub fn i_04a9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A9AC: bra.w $4a35e
pub fn i_04a9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_Burning_04a9b0  ($04A9B0, 178 B, pow/human_death_049ff2.s)
/// $04A9B0: lea.l $4b898(pc), a0
pub fn i_04a9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9B4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B898;
    16
}
/// $04A9B4: jsr $28cd4.l
pub fn i_04a9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9BA);
    cpu.pc = 0x28CD4;
    16
}
/// $04A9BA: bra.w $4a382
pub fn i_04a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A9BE: lea.l $2b604a.l, a0
pub fn i_04a9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B604A;
    20
}
/// $04A9C4: jsr $28cd4.l
pub fn i_04a9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9CA);
    cpu.pc = 0x28CD4;
    16
}
/// $04A9CA: move.w #$1042, d0
pub fn i_04a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1042;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A9CE: jsr $2352.l
pub fn i_04a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9D4);
    cpu.pc = 0x2352;
    16
}
/// $04A9D4: clr.b $21(a6)
pub fn i_04a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9D8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A9D8: lea.l $4aa76(pc), a1
pub fn i_04a9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA76;
    16
}
/// $04A9DC: jsr $4ae.l
pub fn i_04a9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9E2);
    cpu.pc = 0x4AE;
    16
}
/// $04A9E2: move.w #$200, d0
pub fn i_04a9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A9E6: bsr.w $4a1f0
pub fn i_04a9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A9EA);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A9EA: move.w #$28, $30(a6)
pub fn i_04a9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A9F0: jsr $5e9b6.l
pub fn i_04a9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9F6);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A9F6: tst.w d0
pub fn i_04a9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9F8;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A9F8: bpl.w $4aa06
pub fn i_04a9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9FC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4AA06; return 10; }
    8
}
/// $04A9FC: move.b #$a, $72(a6)
pub fn i_04a9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA02;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04AA02: bra.w $4aa0c
pub fn i_04aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA06;
    cpu.ir = 0x6000;
    cpu.pc = 0x4AA0C;
    10
}
/// $04AA06: move.b #$ff, $72(a6)
pub fn i_04aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04AA0C: move.b $72(a6), $73(a6)
pub fn i_04aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA12;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04AA12: move.w #$ffc0, $2e(a6)
pub fn i_04aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA18;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04AA18: lea.l $4aa1e(pc), a1
pub fn i_04aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA1E;
    16
}
/// $04AA1C: move.l a1, (a6)
pub fn i_04aa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AA1E: jsr $4a268(pc)
pub fn i_04aa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4AA22);
    cpu.pc = 0x4A268;
    16
}
/// $04AA22: jsr $28d70.l
pub fn i_04aa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA28);
    cpu.pc = 0x28D70;
    16
}
/// $04AA28: btst.b #$5, $5a(a6)
pub fn i_04aa28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA2E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04AA2E: beq.w $4aa38
pub fn i_04aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4AA38; return 10; }
    8
}
/// $04AA32: lea.l $4aa62(pc), a1
pub fn i_04aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA62;
    16
}
/// $04AA36: move.l a1, (a6)
pub fn i_04aa36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AA38: subq.w #$1, $30(a6)
pub fn i_04aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA3C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04AA3C: bne.w $4aa46
pub fn i_04aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AA46; return 10; }
    8
}
/// $04AA40: lea.l $4aa62(pc), a1
pub fn i_04aa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA62;
    16
}
/// $04AA44: move.l a1, (a6)
pub fn i_04aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AA46: subq.b #$1, $73(a6)
pub fn i_04aa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA4A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04AA4A: bne.w $4aa5e
pub fn i_04aa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AA5E; return 10; }
    8
}
/// $04AA4E: bchg.b #$0, $3a(a6)
pub fn i_04aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA54;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04AA54: move.b $72(a6), $73(a6)
pub fn i_04aa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA5A;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04AA5A: neg.w $28(a6)
pub fn i_04aa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA5E;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04AA5E: jmp $4a35e(pc)
pub fn i_04aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA62;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A35E;
    8
}
// ===== HumanDeath_BurnedDown_04aa62  ($04AA62, 20 B, pow/human_death_049ff2.s)
/// $04AA62: lea.l $4b0c6(pc), a0
pub fn i_04aa62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA66;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B0C6;
    16
}
/// $04AA66: jsr $28cd4.l
pub fn i_04aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA6C);
    cpu.pc = 0x28CD4;
    16
}
/// $04AA6C: move.b #$ff, $21(a6)
pub fn i_04aa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA72;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04AA72: bra.w $4a382
pub fn i_04aa72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA76;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
// ===== HumanDeath_FlameChild_04aa76  ($04AA76, 84 B, pow/human_death_049ff2.s)
/// $04AA76: move.w #$4, d1
pub fn i_04aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA7A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AA7A: jsr $236e.l
pub fn i_04aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA80);
    cpu.pc = 0x236E;
    16
}
/// $04AA80: lea.l $4b926(pc), a0
pub fn i_04aa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA84;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B926;
    16
}
/// $04AA84: jsr $28cd4.l
pub fn i_04aa84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA8A);
    cpu.pc = 0x28CD4;
    16
}
/// $04AA8A: lea.l $4aa90(pc), a1
pub fn i_04aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA90;
    16
}
/// $04AA8E: move.l a1, (a6)
pub fn i_04aa8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AA90: jsr $7b2.l
pub fn i_04aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA96);
    cpu.pc = 0x7B2;
    16
}
/// $04AA96: bcc.w $4aaa0
pub fn i_04aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4AAA0; return 10; }
    8
}
/// $04AA9A: jmp $518.l
pub fn i_04aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAA0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04AAA0: movea.l $c(a6), a0
pub fn i_04aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAA4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04AAA4: tst.b $21(a0)
pub fn i_04aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAA8;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04AAA8: beq.w $4aab2
pub fn i_04aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4AAB2; return 10; }
    8
}
/// $04AAAC: jmp $518.l
pub fn i_04aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAB2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04AAB2: move.w $22(a0), $22(a6)
pub fn i_04aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAB8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04AAB8: move.w $24(a0), d0
pub fn i_04aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AABC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04AABC: addi.w #$1e, d0
pub fn i_04aabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAC0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04AAC0: move.w d0, $24(a6)
pub fn i_04aac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAC4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04AAC4: move.w $38(a0), $38(a6)
pub fn i_04aac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AACA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== JsrAbsThunk_04aaca  ($04AACA, 8 B, thunks/jsr_abs_thunks.c)
/// $04AACA: jsr $28d70.l
pub fn i_04aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AAD0);
    cpu.pc = 0x28D70;
    16
}
/// $04AAD0: rts
pub fn i_04aad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SpawnBloodSplash_04aad2  ($04AAD2, 62 B, pow/human_death_049ff2.s)
/// $04AAD2: lea.l $4ab10(pc), a1
pub fn i_04aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AB10;
    16
}
/// $04AAD6: jsr $4ae.l
pub fn i_04aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AADC);
    cpu.pc = 0x4AE;
    16
}
/// $04AADC: bra.w $4aaea
pub fn i_04aadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4AAEA;
    10
}
/// $04AAE0: lea.l $4ab52(pc), a1
pub fn i_04aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AB52;
    16
}
/// $04AAE4: jsr $4ae.l
pub fn i_04aae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AAEA);
    cpu.pc = 0x4AE;
    16
}
/// $04AAEA: move.w $24(a6), $24(a0)
pub fn i_04aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAF0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04AAF0: move.w #$8, d0
pub fn i_04aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAF4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AAF4: btst.b #$0, $3a(a6)
pub fn i_04aaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAFA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04AAFA: bne.w $4ab06
pub fn i_04aafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AB06; return 10; }
    8
}
/// $04AAFE: bset.b #$0, $3a(a0)
pub fn i_04aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB04;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04AB04: neg.w d0
pub fn i_04ab04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB06;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04AB06: add.w $22(a6), d0
pub fn i_04ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB0A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04AB0A: move.w d0, $22(a0)
pub fn i_04ab0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB0E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04AB0E: rts
pub fn i_04ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_BloodSplashA_04ab10  ($04AB10, 66 B, pow/human_death_049ff2.s)
/// $04AB10: move.w #$17a, d1
pub fn i_04ab10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB14: tst.b $10fd8f.l
pub fn i_04ab14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB1A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04AB1A: bne.w $4ab22
pub fn i_04ab1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AB22; return 10; }
    8
}
/// $04AB1E: move.w #$1ba, d1
pub fn i_04ab1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB22;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB22: jsr $236e.l
pub fn i_04ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB28);
    cpu.pc = 0x236E;
    16
}
/// $04AB28: move.w #$c000, d0
pub fn i_04ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB2C: jsr $28134.l
pub fn i_04ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB32);
    cpu.pc = 0x28134;
    16
}
/// $04AB32: andi.w #$ffe3, $38(a6)
pub fn i_04ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB38;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04AB38: ori.w #$10, $38(a6)
pub fn i_04ab38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB3E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04AB3E: lea.l $4bad2(pc), a0
pub fn i_04ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB42;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4BAD2;
    16
}
/// $04AB42: jsr $28cd4.l
pub fn i_04ab42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB48);
    cpu.pc = 0x28CD4;
    16
}
/// $04AB48: lea.l $4ab90(pc), a1
pub fn i_04ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AB90;
    16
}
/// $04AB4C: move.l a1, (a6)
pub fn i_04ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AB4E: bra.w $4ab90
pub fn i_04ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB52;
    cpu.ir = 0x6000;
    cpu.pc = 0x4AB90;
    10
}
// ===== HumanDeath_BloodSplashB_04ab52  ($04AB52, 62 B, pow/human_death_049ff2.s)
/// $04AB52: move.w #$17a, d1
pub fn i_04ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB56: tst.b $10fd8f.l
pub fn i_04ab56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB5C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04AB5C: bne.w $4ab64
pub fn i_04ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB60;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AB64; return 10; }
    8
}
/// $04AB60: move.w #$1ba, d1
pub fn i_04ab60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB64;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB64: jsr $236e.l
pub fn i_04ab64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB6A);
    cpu.pc = 0x236E;
    16
}
/// $04AB6A: move.w #$c000, d0
pub fn i_04ab6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB6E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB6E: jsr $28134.l
pub fn i_04ab6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB74);
    cpu.pc = 0x28134;
    16
}
/// $04AB74: andi.w #$ffe3, $38(a6)
pub fn i_04ab74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB7A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04AB7A: ori.w #$10, $38(a6)
pub fn i_04ab7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB80;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04AB80: lea.l $4b9f0(pc), a0
pub fn i_04ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB84;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B9F0;
    16
}
/// $04AB84: jsr $28cd4.l
pub fn i_04ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB8A);
    cpu.pc = 0x28CD4;
    16
}
/// $04AB8A: lea.l $4ab90(pc), a1
pub fn i_04ab8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AB90;
    16
}
/// $04AB8E: move.l a1, (a6)
pub fn i_04ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== HumanDeath_BloodSplash_Loop_04ab90  ($04AB90, 24 B, pow/human_death_049ff2.s)
/// $04AB90: jsr $2783a.l
pub fn i_04ab90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB96);
    cpu.pc = 0x2783A;
    16
}
/// $04AB96: jsr $28d70.l
pub fn i_04ab96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB9C);
    cpu.pc = 0x28D70;
    16
}
/// $04AB9C: bcc.w $4aba6
pub fn i_04ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4ABA6; return 10; }
    8
}
/// $04ABA0: jmp $518.l
pub fn i_04aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABA6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04ABA6: rts
pub fn i_04aba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PickGibPtr_04aba8  ($04ABA8, 16 B, pow/human_death_049ff2.s)
/// $04ABA8: andi.w #$7, d0
pub fn i_04aba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABAC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04ABAC: lea.l $4af5a(pc), a0
pub fn i_04abac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABB0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AF5A;
    16
}
/// $04ABB0: asl.w #$2, d0
pub fn i_04abb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABB2;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04ABB2: move.l (a0, d0.w), $76(a6)
pub fn i_04abb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABB8;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
// ===== JsrAbsThunk_04abb8  ($04ABB8, 8 B, thunks/jsr_abs_thunks.c)
/// $04ABB8: jsr $28d70.l
pub fn i_04abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABBE);
    cpu.pc = 0x28D70;
    16
}
/// $04ABBE: rts
pub fn i_04abbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SpawnSmokePair_04abc0  ($04ABC0, 106 B, pow/human_death_049ff2.s)
/// $04ABC0: lea.l $4ac3a(pc), a1
pub fn i_04abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC3A;
    16
}
/// $04ABC4: jsr $4ae.l
pub fn i_04abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABCA);
    cpu.pc = 0x4AE;
    16
}
/// $04ABCA: jsr $5dd02.l
pub fn i_04abca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABD0);
    cpu.pc = 0x5DD02;
    16
}
/// $04ABD0: addi.w #$18, $24(a0)
pub fn i_04abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABD6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04ABD6: jsr $13600.l
pub fn i_04abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABDC);
    cpu.pc = 0x13600;
    16
}
/// $04ABDC: move.w #$19f, d1
pub fn i_04abdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABE0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04ABE0: tst.b $10fd8f.l
pub fn i_04abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABE6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04ABE6: bne.w $4abee
pub fn i_04abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4ABEE; return 10; }
    8
}
/// $04ABEA: move.w #$1a3, d1
pub fn i_04abea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABEE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04ABEE: jsr $236e.l
pub fn i_04abee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABF4);
    cpu.pc = 0x236E;
    16
}
/// $04ABF4: lea.l $4bb44(pc), a0
pub fn i_04abf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABF8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4BB44;
    16
}
/// $04ABF8: jsr $28cd4.l
pub fn i_04abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABFE);
    cpu.pc = 0x28CD4;
    16
}
/// $04ABFE: lea.l $4ac04(pc), a1
pub fn i_04abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC04;
    16
}
/// $04AC02: move.l a1, (a6)
pub fn i_04ac02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AC04: jsr $2783a.l
pub fn i_04ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC0A);
    cpu.pc = 0x2783A;
    16
}
/// $04AC0A: jsr $28d70.l
pub fn i_04ac0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC10);
    cpu.pc = 0x28D70;
    16
}
/// $04AC10: bcc.w $4ac1a
pub fn i_04ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4AC1A; return 10; }
    8
}
/// $04AC14: lea.l $4ac32(pc), a1
pub fn i_04ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC32;
    16
}
/// $04AC18: move.l a1, (a6)
pub fn i_04ac18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AC1A: movea.l #$ffffffff, a0
pub fn i_04ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC20;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04AC20: jsr $5dd56.l
pub fn i_04ac20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC26);
    cpu.pc = 0x5DD56;
    16
}
/// $04AC26: bcc.w $4ac30
pub fn i_04ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4AC30; return 10; }
    8
}
// ===== SetTaskHandler_04ac2a  ($04AC2A, 8 B, kernel/task_handlers.c)
/// $04AC2A: lea.l $4ac32(pc), a1
pub fn i_04ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC32;
    16
}
/// $04AC2E: move.l a1, (a6)
pub fn i_04ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AC30: rts
pub fn i_04ac30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_04ac32  ($04AC32, 8 B, kernel/jmp_to_scheduler.c)
/// $04AC32: jmp $518.l
pub fn i_04ac32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC38;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04AC38: rts
pub fn i_04ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SmokeChild_04ac3a  ($04AC3A, 28 B, pow/human_death_049ff2.s)
/// $04AC3A: move.w #$d, d1
pub fn i_04ac3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC3E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AC3E: jsr $236e.l
pub fn i_04ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC44);
    cpu.pc = 0x236E;
    16
}
/// $04AC44: lea.l $4bb4a(pc), a0
pub fn i_04ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC48;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4BB4A;
    16
}
/// $04AC48: jsr $28cd4.l
pub fn i_04ac48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC4E);
    cpu.pc = 0x28CD4;
    16
}
/// $04AC4E: lea.l $4ac04(pc), a1
pub fn i_04ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC04;
    16
}
/// $04AC52: move.l a1, (a6)
pub fn i_04ac52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AC54: bra.b $4ac04
pub fn i_04ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC56;
    cpu.ir = 0x60AE;
    cpu.pc = 0x4AC04;
    10
}
/// $04B052: addq.w #$2, $24(a6)
pub fn i_04b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B056;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04B056: move.w #$fff8, d0
pub fn i_04b056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B05A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04B05A: jsr $5dca4.l
pub fn i_04b05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B060;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4B060);
    cpu.pc = 0x5DCA4;
    16
}
/// $04B060: move.w d0, $28(a6)
pub fn i_04b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B064;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04B064: move.w #$180, $2a(a6)
pub fn i_04b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B06A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04B06A: move.w #$ffe8, $2e(a6)
pub fn i_04b06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B070;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04B070: move.w #$0, $2c(a6)
pub fn i_04b070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B076;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04B076: rts
pub fn i_04b076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B078;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04bb8e  ($04BB8E, 6 B, thunks/ccr_helpers.c)
/// $04BB8E: andi.b #$ee, ccr
pub fn i_04bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BB92;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04BB92: rts
pub fn i_04bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BB94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04bb94  ($04BB94, 6 B, thunks/ccr_helpers.c)
/// $04BB94: ori.b #$11, ccr
pub fn i_04bb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BB98;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04BB98: rts
pub fn i_04bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BB9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Spawn_04bb9a  ($04BB9A, 166 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BB9A: move.w #$0, $70(a6)
pub fn i_04bb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBA0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BBA0: bra.w $4bbbe
pub fn i_04bba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BBBE;
    10
}
/// $04BBA4: move.w #$1, $70(a6)
pub fn i_04bba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BBAA: bra.w $4bbbe
pub fn i_04bbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BBBE;
    10
}
/// $04BBAE: move.w #$2, $70(a6)
pub fn i_04bbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BBB4: bra.w $4bbbe
pub fn i_04bbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BBBE;
    10
}
/// $04BBB8: move.w #$3, $70(a6)
pub fn i_04bbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBBE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BBBE: move.w #$84, d1
pub fn i_04bbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBC2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04BBC2: jsr $236e.l
pub fn i_04bbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BBC8);
    cpu.pc = 0x236E;
    16
}
/// $04BBC8: move.w #$19, $1c(a6)
pub fn i_04bbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBCE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BBCE: jsr $138fe.l
pub fn i_04bbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BBD4);
    cpu.pc = 0x138FE;
    16
}
/// $04BBD4: move.w #$4000, d0
pub fn i_04bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBD8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04BBD8: jsr $28134.l
pub fn i_04bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BBDE);
    cpu.pc = 0x28134;
    16
}
/// $04BBDE: andi.w #$ffe3, $38(a6)
pub fn i_04bbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBE4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BBE4: ori.w #$8, $38(a6)
pub fn i_04bbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBEA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BBEA: moveq #$0, d0
pub fn i_04bbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBEC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04BBEC: move.b d0, $20(a6)
pub fn i_04bbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBF0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BBF0: move.b d0, $21(a6)
pub fn i_04bbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBF4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BBF4: lea.l $2bd0da.l, a0
pub fn i_04bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD0DA;
    20
}
/// $04BBFA: jsr $799de.l
pub fn i_04bbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC00);
    cpu.pc = 0x799DE;
    16
}
/// $04BC00: move.w d0, $66(a6)
pub fn i_04bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC04;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BC04: jsr $267e2.l
pub fn i_04bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC0A);
    cpu.pc = 0x267E2;
    16
}
/// $04BC0A: jsr $5e7c0.l
pub fn i_04bc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC10);
    cpu.pc = 0x5E7C0;
    16
}
/// $04BC10: lea.l $2902d8.l, a0
pub fn i_04bc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902D8;
    20
}
/// $04BC16: move.w $70(a6), d0
pub fn i_04bc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC1A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BC1A: andi.w #$3, d0
pub fn i_04bc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC1E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04BC1E: lsl.w #$2, d0
pub fn i_04bc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC20;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04BC20: movea.l (a0, d0.w), a1
pub fn i_04bc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC24;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $04BC24: jsr $43fac.l
pub fn i_04bc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC2A);
    cpu.pc = 0x43FAC;
    16
}
/// $04BC2A: lea.l $4c776(pc), a1
pub fn i_04bc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C776;
    16
}
/// $04BC2E: jsr $4ae.l
pub fn i_04bc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC34);
    cpu.pc = 0x4AE;
    16
}
/// $04BC34: jsr $5dd02.l
pub fn i_04bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC3A);
    cpu.pc = 0x5DD02;
    16
}
/// $04BC3A: move.w $70(a6), $70(a0)
pub fn i_04bc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC40;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== SetTaskHandler_04bc40  ($04BC40, 8 B, kernel/task_handlers.c)
/// $04BC40: lea.l $4bc48(pc), a1
pub fn i_04bc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BC48;
    16
}
/// $04BC44: move.l a1, (a6)
pub fn i_04bc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BC46: rts
pub fn i_04bc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpawnCrewAndIdle_04bc48  ($04BC48, 270 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BC48: lea.l $2902f0.l, a0
pub fn i_04bc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902F0;
    20
}
/// $04BC4E: move.w $70(a6), d0
pub fn i_04bc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC52;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BC52: cmpi.b #$1, (a0, d0.w)
pub fn i_04bc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC58;
    cpu.ir = 0xC30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    18
}
/// $04BC58: bne.w $4bc76
pub fn i_04bc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC5C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BC76; return 10; }
    8
}
/// $04BC5C: lea.l $4c44a(pc), a1
pub fn i_04bc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C44A;
    16
}
/// $04BC60: jsr $4ae.l
pub fn i_04bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC66);
    cpu.pc = 0x4AE;
    16
}
/// $04BC66: jsr $5dd02.l
pub fn i_04bc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC6C);
    cpu.pc = 0x5DD02;
    16
}
/// $04BC6C: addi.w #$18, $24(a0)
pub fn i_04bc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC72;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BC72: bra.w $4bc96
pub fn i_04bc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC76;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BC96;
    10
}
/// $04BC76: cmpi.b #$2, (a0, d0.w)
pub fn i_04bc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC7C;
    cpu.ir = 0xC30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    18
}
/// $04BC7C: bne.w $4bc96
pub fn i_04bc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC80;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BC96; return 10; }
    8
}
/// $04BC80: lea.l $4c2e4(pc), a1
pub fn i_04bc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C2E4;
    16
}
/// $04BC84: jsr $4ae.l
pub fn i_04bc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC8A);
    cpu.pc = 0x4AE;
    16
}
/// $04BC8A: jsr $5dd02.l
pub fn i_04bc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC90);
    cpu.pc = 0x5DD02;
    16
}
/// $04BC90: addi.w #$18, $24(a0)
pub fn i_04bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC96;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BC96: lea.l $2902e8.l, a0
pub fn i_04bc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902E8;
    20
}
/// $04BC9C: move.w $70(a6), d0
pub fn i_04bc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCA0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BCA0: tst.b (a0, d0.w)
pub fn i_04bca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCA4;
    cpu.ir = 0x4A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $04BCA4: beq.w $4bcd0
pub fn i_04bca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4BCD0; return 10; }
    8
}
/// $04BCA8: lea.l $4bee0(pc), a1
pub fn i_04bca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BEE0;
    16
}
/// $04BCAC: jsr $4ae.l
pub fn i_04bcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BCB2);
    cpu.pc = 0x4AE;
    16
}
/// $04BCB2: jsr $5dd02.l
pub fn i_04bcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BCB8);
    cpu.pc = 0x5DD02;
    16
}
/// $04BCB8: lea.l $2902f4.l, a1
pub fn i_04bcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCBE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2902F4;
    20
}
/// $04BCBE: move.w $70(a6), d1
pub fn i_04bcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCC2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BCC2: move.w d1, $70(a0)
pub fn i_04bcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCC6;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BCC6: add.w d1, d1
pub fn i_04bcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCC8;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04BCC8: add.w d1, d1
pub fn i_04bcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCCA;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04BCCA: move.w $2(a1, d1.w), $72(a0)
pub fn i_04bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCD0;
    cpu.ir = 0x3171;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    22
}
/// $04BCD0: lea.l $2902ec.l, a0
pub fn i_04bcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902EC;
    20
}
/// $04BCD6: move.w $70(a6), d0
pub fn i_04bcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCDA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BCDA: tst.b (a0, d0.w)
pub fn i_04bcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCDE;
    cpu.ir = 0x4A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $04BCDE: beq.w $4bcf8
pub fn i_04bcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCE2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4BCF8; return 10; }
    8
}
/// $04BCE2: lea.l $4c1b0(pc), a1
pub fn i_04bce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C1B0;
    16
}
/// $04BCE6: jsr $4ae.l
pub fn i_04bce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BCEC);
    cpu.pc = 0x4AE;
    16
}
/// $04BCEC: jsr $5dd02.l
pub fn i_04bcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BCF2);
    cpu.pc = 0x5DD02;
    16
}
/// $04BCF2: move.w $70(a6), $70(a0)
pub fn i_04bcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCF8;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04BCF8: jsr $4ca0e(pc)
pub fn i_04bcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BCFC);
    cpu.pc = 0x4CA0E;
    16
}
/// $04BCFC: moveq #$0, d0
pub fn i_04bcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCFE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04BCFE: move.b d0, $20(a6)
pub fn i_04bcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD02;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BD02: move.w d0, $88(a6)
pub fn i_04bd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD06;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BD06: lea.l $4bd0c(pc), a1
pub fn i_04bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD0C;
    16
}
/// $04BD0A: move.l a1, (a6)
pub fn i_04bd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD0C: jsr $2783a.l
pub fn i_04bd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BD12);
    cpu.pc = 0x2783A;
    16
}
/// $04BD12: lea.l $290304.l, a1
pub fn i_04bd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD18;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290304;
    20
}
/// $04BD18: jsr $4c98a(pc)
pub fn i_04bd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BD1C);
    cpu.pc = 0x4C98A;
    16
}
/// $04BD1C: jsr $28d70.l
pub fn i_04bd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BD22);
    cpu.pc = 0x28D70;
    16
}
/// $04BD22: jsr $4c9d6(pc)
pub fn i_04bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BD26);
    cpu.pc = 0x4C9D6;
    16
}
/// $04BD26: bcc.w $4bd36
pub fn i_04bd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BD36; return 10; }
    8
}
/// $04BD2A: move.b #$1, $20(a6)
pub fn i_04bd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BD30: lea.l $4bd6c(pc), a1
pub fn i_04bd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD6C;
    16
}
/// $04BD34: move.l a1, (a6)
pub fn i_04bd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD36: clr.b $10e39a.l
pub fn i_04bd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD3C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04BD3C: jsr $4c6d4(pc)
pub fn i_04bd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BD40);
    cpu.pc = 0x4C6D4;
    16
}
/// $04BD40: movea.l #$ffffffff, a0
pub fn i_04bd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD46;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BD46: lea.l $290912.l, a0
pub fn i_04bd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BD4C: jsr $5dd5c.l
pub fn i_04bd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BD52);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BD52: bcc.w $4bd5c
pub fn i_04bd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BD5C; return 10; }
    8
}
// ===== SetTaskHandler_04bd56  ($04BD56, 8 B, kernel/task_handlers.c)
/// $04BD56: lea.l $4c578(pc), a1
pub fn i_04bd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C578;
    16
}
/// $04BD5A: move.l a1, (a6)
pub fn i_04bd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD5C: rts
pub fn i_04bd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Rearm_04bd5e  ($04BD5E, 50 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BD5E: clr.w $88(a6)
pub fn i_04bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD62;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BD62: lea.l $4bd90(pc), a1
pub fn i_04bd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD90;
    16
}
/// $04BD66: move.l a1, (a6)
pub fn i_04bd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD68: bra.w $4bd90
pub fn i_04bd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BD90;
    10
}
/// $04BD6C: jsr $4ca24(pc)
pub fn i_04bd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BD70);
    cpu.pc = 0x4CA24;
    16
}
/// $04BD70: move.w #$1e, $88(a6)
pub fn i_04bd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD76;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BD76: move.b #$1, $20(a6)
pub fn i_04bd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD7C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BD7C: lea.l $4bd82(pc), a1
pub fn i_04bd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD82;
    16
}
/// $04BD80: move.l a1, (a6)
pub fn i_04bd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD82: subq.w #$1, $88(a6)
pub fn i_04bd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD86;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BD86: bne.w $4bd90
pub fn i_04bd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD8A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BD90; return 10; }
    8
}
/// $04BD8A: lea.l $4bd90(pc), a1
pub fn i_04bd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD90;
    16
}
/// $04BD8E: move.l a1, (a6)
pub fn i_04bd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== GunPlatform_Aim_04bd90  ($04BD90, 108 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BD90: jsr $2783a.l
pub fn i_04bd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BD96);
    cpu.pc = 0x2783A;
    16
}
/// $04BD96: lea.l $290304.l, a1
pub fn i_04bd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290304;
    20
}
/// $04BD9C: jsr $4c98a(pc)
pub fn i_04bd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BDA0);
    cpu.pc = 0x4C98A;
    16
}
/// $04BDA0: jsr $28d70.l
pub fn i_04bda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BDA6);
    cpu.pc = 0x28D70;
    16
}
/// $04BDA6: tst.w $88(a6)
pub fn i_04bda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDAA;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $04BDAA: bne.w $4bddc
pub fn i_04bdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BDDC; return 10; }
    8
}
/// $04BDAE: jsr $4c9ee(pc)
pub fn i_04bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BDB2);
    cpu.pc = 0x4C9EE;
    16
}
/// $04BDB2: bcc.w $4bddc
pub fn i_04bdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BDDC; return 10; }
    8
}
/// $04BDB6: btst.b #$0, $8a(a6)
pub fn i_04bdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDBC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04BDBC: bne.w $4bdd0
pub fn i_04bdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BDD0; return 10; }
    8
}
/// $04BDC0: move.b #$2, $20(a6)
pub fn i_04bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BDC6: lea.l $4be04(pc), a1
pub fn i_04bdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BE04;
    16
}
/// $04BDCA: move.l a1, (a6)
pub fn i_04bdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BDCC: bra.w $4bddc
pub fn i_04bdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BDDC;
    10
}
/// $04BDD0: move.b #$3, $20(a6)
pub fn i_04bdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDD6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BDD6: lea.l $4be72(pc), a1
pub fn i_04bdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BE72;
    16
}
/// $04BDDA: move.l a1, (a6)
pub fn i_04bdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BDDC: clr.b $10e39a.l
pub fn i_04bddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDE2;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04BDE2: jsr $4c6d4(pc)
pub fn i_04bde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BDE6);
    cpu.pc = 0x4C6D4;
    16
}
/// $04BDE6: movea.l #$ffffffff, a0
pub fn i_04bde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDEC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BDEC: lea.l $290912.l, a0
pub fn i_04bdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BDF2: jsr $5dd5c.l
pub fn i_04bdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BDF8);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BDF8: bcc.w $4be02
pub fn i_04bdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BE02; return 10; }
    8
}
// ===== SetTaskHandler_04bdfc  ($04BDFC, 8 B, kernel/task_handlers.c)
/// $04BDFC: lea.l $4c578(pc), a1
pub fn i_04bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C578;
    16
}
/// $04BE00: move.l a1, (a6)
pub fn i_04be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE02: rts
pub fn i_04be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_FireA_04be04  ($04BE04, 102 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BE04: move.w #$1b, $88(a6)
pub fn i_04be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BE0A: lea.l $4be10(pc), a1
pub fn i_04be0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BE10;
    16
}
/// $04BE0E: move.l a1, (a6)
pub fn i_04be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE10: jsr $2783a.l
pub fn i_04be10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE16);
    cpu.pc = 0x2783A;
    16
}
/// $04BE16: lea.l $290304.l, a1
pub fn i_04be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE1C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290304;
    20
}
/// $04BE1C: jsr $4c98a(pc)
pub fn i_04be1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BE20);
    cpu.pc = 0x4C98A;
    16
}
/// $04BE20: jsr $28d70.l
pub fn i_04be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE26);
    cpu.pc = 0x28D70;
    16
}
/// $04BE26: subq.w #$1, $88(a6)
pub fn i_04be26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE2A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BE2A: bne.w $4be4a
pub fn i_04be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BE4A; return 10; }
    8
}
/// $04BE2E: addq.b #$1, $8a(a6)
pub fn i_04be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE32;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04BE32: subq.b #$1, $82(a6)
pub fn i_04be32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE36;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04BE36: bne.w $4be44
pub fn i_04be36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BE44; return 10; }
    8
}
/// $04BE3A: lea.l $4bcf8(pc), a1
pub fn i_04be3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BCF8;
    16
}
/// $04BE3E: move.l a1, (a6)
pub fn i_04be3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE40: bra.w $4be4a
pub fn i_04be40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE44;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BE4A;
    10
}
/// $04BE44: lea.l $4bd5e(pc), a1
pub fn i_04be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD5E;
    16
}
/// $04BE48: move.l a1, (a6)
pub fn i_04be48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE4A: clr.b $10e39a.l
pub fn i_04be4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE50;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04BE50: jsr $4c6d4(pc)
pub fn i_04be50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BE54);
    cpu.pc = 0x4C6D4;
    16
}
/// $04BE54: movea.l #$ffffffff, a0
pub fn i_04be54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE5A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BE5A: lea.l $290912.l, a0
pub fn i_04be5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BE60: jsr $5dd5c.l
pub fn i_04be60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE66);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BE66: bcc.w $4be70
pub fn i_04be66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BE70; return 10; }
    8
}
// ===== SetTaskHandler_04be6a  ($04BE6A, 8 B, kernel/task_handlers.c)
/// $04BE6A: lea.l $4c578(pc), a1
pub fn i_04be6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C578;
    16
}
/// $04BE6E: move.l a1, (a6)
pub fn i_04be6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE70: rts
pub fn i_04be70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_FireB_04be72  ($04BE72, 102 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BE72: move.w #$1b, $88(a6)
pub fn i_04be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE78;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BE78: lea.l $4be7e(pc), a1
pub fn i_04be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BE7E;
    16
}
/// $04BE7C: move.l a1, (a6)
pub fn i_04be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE7E: jsr $2783a.l
pub fn i_04be7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE84);
    cpu.pc = 0x2783A;
    16
}
/// $04BE84: lea.l $290304.l, a1
pub fn i_04be84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE8A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290304;
    20
}
/// $04BE8A: jsr $4c98a(pc)
pub fn i_04be8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BE8E);
    cpu.pc = 0x4C98A;
    16
}
/// $04BE8E: jsr $28d70.l
pub fn i_04be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE94);
    cpu.pc = 0x28D70;
    16
}
/// $04BE94: subq.w #$1, $88(a6)
pub fn i_04be94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE98;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BE98: bne.w $4beb8
pub fn i_04be98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BEB8; return 10; }
    8
}
/// $04BE9C: addq.b #$1, $8a(a6)
pub fn i_04be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEA0;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04BEA0: subq.b #$1, $82(a6)
pub fn i_04bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEA4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04BEA4: bne.w $4beb2
pub fn i_04bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEA8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BEB2; return 10; }
    8
}
/// $04BEA8: lea.l $4bcf8(pc), a1
pub fn i_04bea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BCF8;
    16
}
/// $04BEAC: move.l a1, (a6)
pub fn i_04beac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BEAE: bra.w $4beb8
pub fn i_04beae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BEB8;
    10
}
/// $04BEB2: lea.l $4bd5e(pc), a1
pub fn i_04beb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD5E;
    16
}
/// $04BEB6: move.l a1, (a6)
pub fn i_04beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BEB8: clr.b $10e39a.l
pub fn i_04beb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEBE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04BEBE: jsr $4c6d4(pc)
pub fn i_04bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BEC2);
    cpu.pc = 0x4C6D4;
    16
}
/// $04BEC2: movea.l #$ffffffff, a0
pub fn i_04bec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEC8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BEC8: lea.l $290912.l, a0
pub fn i_04bec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BECE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BECE: jsr $5dd5c.l
pub fn i_04bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BED4);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BED4: bcc.w $4bede
pub fn i_04bed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BED8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BEDE; return 10; }
    8
}
// ===== SetTaskHandler_04bed8  ($04BED8, 8 B, kernel/task_handlers.c)
/// $04BED8: lea.l $4c578(pc), a1
pub fn i_04bed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C578;
    16
}
/// $04BEDC: move.l a1, (a6)
pub fn i_04bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BEDE: rts
pub fn i_04bede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Hatch_04bee0  ($04BEE0, 112 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BEE0: move.w #$84, d1
pub fn i_04bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEE4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04BEE4: jsr $236e.l
pub fn i_04bee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BEEA);
    cpu.pc = 0x236E;
    16
}
/// $04BEEA: move.w #$19, $1c(a6)
pub fn i_04beea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEF0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BEF0: jsr $138fe.l
pub fn i_04bef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BEF6);
    cpu.pc = 0x138FE;
    16
}
/// $04BEF6: move.w #$8000, d0
pub fn i_04bef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEFA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04BEFA: jsr $28134.l
pub fn i_04befa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BF00);
    cpu.pc = 0x28134;
    16
}
/// $04BF00: andi.w #$ffe3, $38(a6)
pub fn i_04bf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF06;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BF06: ori.w #$8, $38(a6)
pub fn i_04bf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF0C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BF0C: moveq #$0, d0
pub fn i_04bf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF0E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04BF0E: move.b d0, $20(a6)
pub fn i_04bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF12;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BF12: move.b d0, $21(a6)
pub fn i_04bf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF16;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BF16: jsr $267e2.l
pub fn i_04bf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BF1C);
    cpu.pc = 0x267E2;
    16
}
/// $04BF1C: lea.l $4bf22(pc), a1
pub fn i_04bf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BF22;
    16
}
/// $04BF20: move.l a1, (a6)
pub fn i_04bf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BF22: movea.l $c(a6), a0
pub fn i_04bf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF26;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04BF26: cmpi.b #$1, $20(a0)
pub fn i_04bf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF2C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04BF2C: bne.w $4bf36
pub fn i_04bf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BF36; return 10; }
    8
}
/// $04BF30: lea.l $4bf58(pc), a1
pub fn i_04bf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BF58;
    16
}
/// $04BF34: move.l a1, (a6)
pub fn i_04bf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BF36: jsr $4c942(pc)
pub fn i_04bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BF3A);
    cpu.pc = 0x4C942;
    16
}
/// $04BF3A: movea.l #$ffffffff, a0
pub fn i_04bf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF40;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BF40: lea.l $290912.l, a0
pub fn i_04bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BF46: jsr $5dd5c.l
pub fn i_04bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BF4C);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BF4C: bcc.w $4bf56
pub fn i_04bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BF56; return 10; }
    8
}
// ===== SetTaskHandler_04bf50  ($04BF50, 8 B, kernel/task_handlers.c)
/// $04BF50: lea.l $4c58c(pc), a1
pub fn i_04bf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04BF54: move.l a1, (a6)
pub fn i_04bf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BF56: rts
pub fn i_04bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HatchOpen_04bf58  ($04BF58, 144 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BF58: move.b $21(a6), d0
pub fn i_04bf58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF5C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04BF5C: andi.w #$ff, d0
pub fn i_04bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF60;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04BF60: movea.l #$290344, a0
pub fn i_04bf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF66;
    cpu.ir = 0x207C;
    let v1: u32 = 0x290344;
    cpu.a[0] = v1;
    4
}
/// $04BF66: lsl.w #$2, d0
pub fn i_04bf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF68;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04BF68: movea.l (a0, d0.w), a0
pub fn i_04bf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF6C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04BF6C: cmpa.l #$ffffffff, a0
pub fn i_04bf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF72;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04BF72: beq.w $4bf7c
pub fn i_04bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4BF7C; return 10; }
    8
}
/// $04BF76: jsr $28cd4.l
pub fn i_04bf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BF7C);
    cpu.pc = 0x28CD4;
    16
}
/// $04BF7C: lea.l $4bf82(pc), a1
pub fn i_04bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BF82;
    16
}
/// $04BF80: move.l a1, (a6)
pub fn i_04bf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BF82: movea.l $c(a6), a0
pub fn i_04bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF86;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04BF86: move.w $22(a0), $22(a6)
pub fn i_04bf86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF8C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04BF8C: move.w $24(a0), $24(a6)
pub fn i_04bf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF92;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04BF92: move.w $72(a6), d0
pub fn i_04bf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF96;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BF96: add.w d0, $24(a6)
pub fn i_04bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF9A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BF9A: jsr $28108.l
pub fn i_04bf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BFA0);
    cpu.pc = 0x28108;
    16
}
/// $04BFA0: movea.l $c(a6), a0
pub fn i_04bfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFA4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04BFA4: btst.b #$7, $5a(a0)
pub fn i_04bfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFAA;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04BFAA: beq.w $4bfb4
pub fn i_04bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4BFB4; return 10; }
    8
}
/// $04BFAE: bset.b #$0, $5a(a6)
pub fn i_04bfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFB4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04BFB4: jsr $28d70.l
pub fn i_04bfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BFBA);
    cpu.pc = 0x28D70;
    16
}
/// $04BFBA: movea.l $c(a6), a0
pub fn i_04bfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFBE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04BFBE: cmpi.b #$2, $20(a0)
pub fn i_04bfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFC4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04BFC4: bne.w $4bfce
pub fn i_04bfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFC8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BFCE; return 10; }
    8
}
/// $04BFC8: lea.l $4bff0(pc), a1
pub fn i_04bfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BFF0;
    16
}
/// $04BFCC: move.l a1, (a6)
pub fn i_04bfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BFCE: jsr $4c942(pc)
pub fn i_04bfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BFD2);
    cpu.pc = 0x4C942;
    16
}
/// $04BFD2: movea.l #$ffffffff, a0
pub fn i_04bfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFD8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BFD8: lea.l $290912.l, a0
pub fn i_04bfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFDE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BFDE: jsr $5dd5c.l
pub fn i_04bfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BFE4);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BFE4: bcc.w $4bfee
pub fn i_04bfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFE8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BFEE; return 10; }
    8
}
// ===== SetTaskHandler_04bfe8  ($04BFE8, 8 B, kernel/task_handlers.c)
/// $04BFE8: lea.l $4c58c(pc), a1
pub fn i_04bfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04BFEC: move.l a1, (a6)
pub fn i_04bfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BFEE: rts
pub fn i_04bfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HatchFire_04bff0  ($04BFF0, 144 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BFF0: move.b $21(a6), d0
pub fn i_04bff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFF4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04BFF4: andi.w #$ff, d0
pub fn i_04bff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04BFF8: movea.l #$29033c, a0
pub fn i_04bff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFFE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29033C;
    cpu.a[0] = v1;
    4
}
/// $04BFFE: lsl.w #$2, d0
pub fn i_04bffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C000;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04C000: movea.l (a0, d0.w), a0
pub fn i_04c000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C004;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C004: cmpa.l #$ffffffff, a0
pub fn i_04c004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C00A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04C00A: beq.w $4c014
pub fn i_04c00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C00E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C014; return 10; }
    8
}
/// $04C00E: jsr $28cd4.l
pub fn i_04c00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C014);
    cpu.pc = 0x28CD4;
    16
}
/// $04C014: lea.l $4c01a(pc), a1
pub fn i_04c014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C018;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C01A;
    16
}
/// $04C018: move.l a1, (a6)
pub fn i_04c018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C01A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C01A: movea.l $c(a6), a0
pub fn i_04c01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C01E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C01E: move.w $22(a0), $22(a6)
pub fn i_04c01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C024;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C024: move.w $24(a0), $24(a6)
pub fn i_04c024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C02A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C02A: move.w $72(a6), d0
pub fn i_04c02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C02E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C02E: add.w d0, $24(a6)
pub fn i_04c02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C032;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C032: jsr $28108.l
pub fn i_04c032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C038;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C038);
    cpu.pc = 0x28108;
    16
}
/// $04C038: movea.l $c(a6), a0
pub fn i_04c038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C03C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C03C: btst.b #$7, $5a(a0)
pub fn i_04c03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C042;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C042: beq.w $4c04c
pub fn i_04c042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C046;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C04C; return 10; }
    8
}
/// $04C046: bset.b #$0, $5a(a6)
pub fn i_04c046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C04C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C04C: jsr $28d70.l
pub fn i_04c04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C052);
    cpu.pc = 0x28D70;
    16
}
/// $04C052: movea.l $c(a6), a0
pub fn i_04c052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C056;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C056: cmpi.b #$2, $20(a0)
pub fn i_04c056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C05C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04C05C: beq.w $4c066
pub fn i_04c05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C060;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C066; return 10; }
    8
}
/// $04C060: lea.l $4c088(pc), a1
pub fn i_04c060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C064;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C088;
    16
}
/// $04C064: move.l a1, (a6)
pub fn i_04c064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C066;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C066: jsr $4c942(pc)
pub fn i_04c066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C06A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C06A);
    cpu.pc = 0x4C942;
    16
}
/// $04C06A: movea.l #$ffffffff, a0
pub fn i_04c06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C070;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C070: lea.l $290912.l, a0
pub fn i_04c070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C076;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C076: jsr $5dd5c.l
pub fn i_04c076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C07C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C07C);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C07C: bcc.w $4c086
pub fn i_04c07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C080;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C086; return 10; }
    8
}
// ===== SetTaskHandler_04c080  ($04C080, 8 B, kernel/task_handlers.c)
/// $04C080: lea.l $4c58c(pc), a1
pub fn i_04c080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C084;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C084: move.l a1, (a6)
pub fn i_04c084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C086;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C086: rts
pub fn i_04c086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C088;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HatchHold_04c088  ($04C088, 152 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C088: movea.l $c(a6), a0
pub fn i_04c088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C08C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C08C: move.w $22(a0), $22(a6)
pub fn i_04c08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C092;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C092: move.w $24(a0), $24(a6)
pub fn i_04c092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C098;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C098: move.w $72(a6), d0
pub fn i_04c098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C09C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C09C: add.w d0, $24(a6)
pub fn i_04c09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0A0;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C0A0: jsr $28108.l
pub fn i_04c0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C0A6);
    cpu.pc = 0x28108;
    16
}
/// $04C0A6: move.b $21(a6), d0
pub fn i_04c0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0AA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04C0AA: andi.w #$ff, d0
pub fn i_04c0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0AE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C0AE: movea.l #$290334, a0
pub fn i_04c0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x290334;
    cpu.a[0] = v1;
    4
}
/// $04C0B4: lsl.w #$2, d0
pub fn i_04c0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0B6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04C0B6: movea.l (a0, d0.w), a0
pub fn i_04c0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0BA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C0BA: cmpa.l #$ffffffff, a0
pub fn i_04c0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0C0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04C0C0: beq.w $4c0ca
pub fn i_04c0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C0CA; return 10; }
    8
}
/// $04C0C4: jsr $28cd4.l
pub fn i_04c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C0CA);
    cpu.pc = 0x28CD4;
    16
}
/// $04C0CA: movea.l $c(a6), a0
pub fn i_04c0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0CE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C0CE: btst.b #$7, $5a(a0)
pub fn i_04c0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0D4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C0D4: beq.w $4c0de
pub fn i_04c0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C0DE; return 10; }
    8
}
/// $04C0D8: bset.b #$0, $5a(a6)
pub fn i_04c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C0DE: jsr $28d70.l
pub fn i_04c0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C0E4);
    cpu.pc = 0x28D70;
    16
}
/// $04C0E4: movea.l $c(a6), a0
pub fn i_04c0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C0E8: tst.b $20(a0)
pub fn i_04c0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0EC;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C0EC: bne.w $4c0f6
pub fn i_04c0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C0F6; return 10; }
    8
}
/// $04C0F0: lea.l $4c128(pc), a1
pub fn i_04c0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C128;
    16
}
/// $04C0F4: move.l a1, (a6)
pub fn i_04c0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C0F6: cmpi.b #$2, $20(a0)
pub fn i_04c0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0FC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04C0FC: bne.w $4c106
pub fn i_04c0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C100;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C106; return 10; }
    8
}
/// $04C100: lea.l $4bff0(pc), a1
pub fn i_04c100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C104;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BFF0;
    16
}
/// $04C104: move.l a1, (a6)
pub fn i_04c104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C106;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C106: jsr $4c942(pc)
pub fn i_04c106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C10A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C10A);
    cpu.pc = 0x4C942;
    16
}
/// $04C10A: movea.l #$ffffffff, a0
pub fn i_04c10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C110;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C110: lea.l $290912.l, a0
pub fn i_04c110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C116;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C116: jsr $5dd5c.l
pub fn i_04c116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C11C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C11C);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C11C: bcc.w $4c126
pub fn i_04c11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C120;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C126; return 10; }
    8
}
// ===== SetTaskHandler_04c120  ($04C120, 8 B, kernel/task_handlers.c)
/// $04C120: lea.l $4c58c(pc), a1
pub fn i_04c120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C124;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C124: move.l a1, (a6)
pub fn i_04c124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C126;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C126: rts
pub fn i_04c126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C128;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HatchClose_04c128  ($04C128, 128 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C128: move.b $21(a6), d0
pub fn i_04c128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C12C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04C12C: andi.w #$ff, d0
pub fn i_04c12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C130;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C130: movea.l #$29034c, a0
pub fn i_04c130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C136;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29034C;
    cpu.a[0] = v1;
    4
}
/// $04C136: lsl.w #$2, d0
pub fn i_04c136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C138;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04C138: movea.l (a0, d0.w), a0
pub fn i_04c138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C13C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C13C: cmpa.l #$ffffffff, a0
pub fn i_04c13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C142;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04C142: beq.w $4c14c
pub fn i_04c142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C146;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C14C; return 10; }
    8
}
/// $04C146: jsr $28cd4.l
pub fn i_04c146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C14C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C14C);
    cpu.pc = 0x28CD4;
    16
}
/// $04C14C: lea.l $4c152(pc), a1
pub fn i_04c14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C150;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C152;
    16
}
/// $04C150: move.l a1, (a6)
pub fn i_04c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C152;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C152: movea.l $c(a6), a0
pub fn i_04c152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C156;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C156: move.w $22(a0), $22(a6)
pub fn i_04c156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C15C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C15C: move.w $24(a0), $24(a6)
pub fn i_04c15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C162;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C162: move.w $72(a6), d0
pub fn i_04c162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C166;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C166: add.w d0, $24(a6)
pub fn i_04c166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C16A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C16A: movea.l $c(a6), a0
pub fn i_04c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C16E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C16E: btst.b #$7, $5a(a0)
pub fn i_04c16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C174;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C174: beq.w $4c17e
pub fn i_04c174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C178;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C17E; return 10; }
    8
}
/// $04C178: bset.b #$0, $5a(a6)
pub fn i_04c178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C17E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C17E: jsr $28d70.l
pub fn i_04c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C184);
    cpu.pc = 0x28D70;
    16
}
/// $04C184: bcc.w $4c18e
pub fn i_04c184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C188;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C18E; return 10; }
    8
}
/// $04C188: lea.l $4bf1c(pc), a1
pub fn i_04c188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C18C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BF1C;
    16
}
/// $04C18C: move.l a1, (a6)
pub fn i_04c18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C18E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C18E: jsr $4c942(pc)
pub fn i_04c18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C192;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C192);
    cpu.pc = 0x4C942;
    16
}
/// $04C192: movea.l #$ffffffff, a0
pub fn i_04c192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C198;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C198: lea.l $290912.l, a0
pub fn i_04c198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C19E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C19E: jsr $5dd5c.l
pub fn i_04c19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1A4);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C1A4: bcc.w $4c1ae
pub fn i_04c1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C1AE; return 10; }
    8
}
// ===== SetTaskHandler_04c1a8  ($04C1A8, 8 B, kernel/task_handlers.c)
/// $04C1A8: lea.l $4c58c(pc), a1
pub fn i_04c1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C1AC: move.l a1, (a6)
pub fn i_04c1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C1AE: rts
pub fn i_04c1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Shield_04c1b0  ($04C1B0, 190 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C1B0: move.w #$84, d1
pub fn i_04c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C1B4: jsr $236e.l
pub fn i_04c1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1BA);
    cpu.pc = 0x236E;
    16
}
/// $04C1BA: move.w #$19, $1c(a6)
pub fn i_04c1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C1C0: jsr $138fe.l
pub fn i_04c1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1C6);
    cpu.pc = 0x138FE;
    16
}
/// $04C1C6: bset.b #$1, $12(a6)
pub fn i_04c1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1CC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C1CC: move.w #$0, d0
pub fn i_04c1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C1D0: jsr $28134.l
pub fn i_04c1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1D6);
    cpu.pc = 0x28134;
    16
}
/// $04C1D6: andi.w #$ffe3, $38(a6)
pub fn i_04c1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1DC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C1DC: ori.w #$8, $38(a6)
pub fn i_04c1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1E2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C1E2: moveq #$0, d0
pub fn i_04c1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1E4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04C1E4: move.b d0, $20(a6)
pub fn i_04c1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1E8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C1E8: move.b d0, $21(a6)
pub fn i_04c1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1EC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C1EC: lea.l $2bd15c.l, a0
pub fn i_04c1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD15C;
    20
}
/// $04C1F2: jsr $799de.l
pub fn i_04c1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1F8);
    cpu.pc = 0x799DE;
    16
}
/// $04C1F8: move.w d0, $66(a6)
pub fn i_04c1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C1FC: jsr $267e2.l
pub fn i_04c1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C202;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C202);
    cpu.pc = 0x267E2;
    16
}
/// $04C202: lea.l $4c208(pc), a1
pub fn i_04c202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C206;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C208;
    16
}
/// $04C206: move.l a1, (a6)
pub fn i_04c206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C208;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C208: movea.l $c(a6), a0
pub fn i_04c208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C20C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C20C: move.w $22(a0), $22(a6)
pub fn i_04c20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C212;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C212: move.w $24(a0), $24(a6)
pub fn i_04c212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C218;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C218: lea.l $29030c.l, a1
pub fn i_04c218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C21E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29030C;
    20
}
/// $04C21E: jsr $4c9ae(pc)
pub fn i_04c21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C222;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C222);
    cpu.pc = 0x4C9AE;
    16
}
/// $04C222: movea.l $c(a6), a0
pub fn i_04c222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C226;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C226: btst.b #$7, $5a(a0)
pub fn i_04c226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C22C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C22C: beq.w $4c236
pub fn i_04c22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C230;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C236; return 10; }
    8
}
/// $04C230: bset.b #$0, $5a(a6)
pub fn i_04c230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C236;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C236: jsr $28d70.l
pub fn i_04c236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C23C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C23C);
    cpu.pc = 0x28D70;
    16
}
/// $04C23C: movea.l $c(a6), a0
pub fn i_04c23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C240;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C240: cmpi.b #$2, $20(a0)
pub fn i_04c240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C246;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04C246: bne.w $4c250
pub fn i_04c246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C24A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C250; return 10; }
    8
}
/// $04C24A: lea.l $4c276(pc), a1
pub fn i_04c24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C24E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C276;
    16
}
/// $04C24E: move.l a1, (a6)
pub fn i_04c24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C250;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C250: jsr $4c942(pc)
pub fn i_04c250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C254;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C254);
    cpu.pc = 0x4C942;
    16
}
/// $04C254: jsr $4c958(pc)
pub fn i_04c254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C258;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C258);
    cpu.pc = 0x4C958;
    16
}
/// $04C258: movea.l #$ffffffff, a0
pub fn i_04c258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C25E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C25E: lea.l $290912.l, a0
pub fn i_04c25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C264;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C264: jsr $5dd5c.l
pub fn i_04c264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C26A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C26A);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C26A: bcc.w $4c274
pub fn i_04c26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C26E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C274; return 10; }
    8
}
// ===== SetTaskHandler_04c26e  ($04C26E, 8 B, kernel/task_handlers.c)
/// $04C26E: lea.l $4c58c(pc), a1
pub fn i_04c26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C272;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C272: move.l a1, (a6)
pub fn i_04c272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C274;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C274: rts
pub fn i_04c274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C276;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_ShieldFire_04c276  ($04C276, 102 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C276: movea.l $c(a6), a0
pub fn i_04c276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C27A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C27A: move.w $22(a0), $22(a6)
pub fn i_04c27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C280;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C280: move.w $24(a0), $24(a6)
pub fn i_04c280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C286;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C286: movea.l $c(a6), a0
pub fn i_04c286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C28A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C28A: btst.b #$7, $5a(a0)
pub fn i_04c28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C290;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C290: beq.w $4c29a
pub fn i_04c290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C294;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C29A; return 10; }
    8
}
/// $04C294: bset.b #$0, $5a(a6)
pub fn i_04c294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C29A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C29A: lea.l $29030c.l, a1
pub fn i_04c29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29030C;
    20
}
/// $04C2A0: jsr $4c9ae(pc)
pub fn i_04c2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C2A4);
    cpu.pc = 0x4C9AE;
    16
}
/// $04C2A4: jsr $28d70.l
pub fn i_04c2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C2AA);
    cpu.pc = 0x28D70;
    16
}
/// $04C2AA: movea.l $c(a6), a0
pub fn i_04c2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2AE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C2AE: cmpi.b #$2, $20(a0)
pub fn i_04c2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2B4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04C2B4: beq.w $4c2be
pub fn i_04c2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C2BE; return 10; }
    8
}
/// $04C2B8: lea.l $4c208(pc), a1
pub fn i_04c2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C208;
    16
}
/// $04C2BC: move.l a1, (a6)
pub fn i_04c2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C2BE: jsr $4c942(pc)
pub fn i_04c2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C2C2);
    cpu.pc = 0x4C942;
    16
}
/// $04C2C2: jsr $4c958(pc)
pub fn i_04c2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C2C6);
    cpu.pc = 0x4C958;
    16
}
/// $04C2C6: movea.l #$ffffffff, a0
pub fn i_04c2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2CC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C2CC: lea.l $290912.l, a0
pub fn i_04c2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C2D2: jsr $5dd5c.l
pub fn i_04c2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C2D8);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C2D8: bcc.w $4c2e2
pub fn i_04c2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C2E2; return 10; }
    8
}
// ===== SetTaskHandler_04c2dc  ($04C2DC, 8 B, kernel/task_handlers.c)
/// $04C2DC: lea.l $4c58c(pc), a1
pub fn i_04c2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C2E0: move.l a1, (a6)
pub fn i_04c2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C2E2: rts
pub fn i_04c2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_RiderB_Init_04c2e4  ($04C2E4, 80 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C2E4: clr.b $20(a6)
pub fn i_04c2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2E8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C2E8: clr.b $21(a6)
pub fn i_04c2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2EC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C2EC: move.w #$1, $66(a6)
pub fn i_04c2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C2F2: move.w #$38, d1
pub fn i_04c2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2F6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C2F6: jsr $236e.l
pub fn i_04c2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C2FC);
    cpu.pc = 0x236E;
    16
}
/// $04C2FC: jsr $5e7c0.l
pub fn i_04c2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C302);
    cpu.pc = 0x5E7C0;
    16
}
/// $04C302: jsr $267e2.l
pub fn i_04c302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C308);
    cpu.pc = 0x267E2;
    16
}
/// $04C308: addi.w #$a, $22(a6)
pub fn i_04c308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C30E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C30E: subi.w #$b, $24(a6)
pub fn i_04c30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C314;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xB, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C314: lea.l $290c74.l, a0
pub fn i_04c314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C31A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290C74;
    20
}
/// $04C31A: move.l a0, $48(a6)
pub fn i_04c31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C31E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04C31E: move.w #$2000, d0
pub fn i_04c31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C322;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C322: jsr $28134.l
pub fn i_04c322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C328);
    cpu.pc = 0x28134;
    16
}
/// $04C328: andi.w #$ffe3, $38(a6)
pub fn i_04c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C32E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C32E: ori.w #$8, $38(a6)
pub fn i_04c32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C334;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== GunPlatform_RiderB_Idle_04c334  ($04C334, 52 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C334: lea.l $28e312.l, a0
pub fn i_04c334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C33A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E312;
    20
}
/// $04C33A: jsr $28cd4.l
pub fn i_04c33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C340);
    cpu.pc = 0x28CD4;
    16
}
/// $04C340: lea.l $4c346(pc), a1
pub fn i_04c340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C344;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C346;
    16
}
/// $04C344: move.l a1, (a6)
pub fn i_04c344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C346;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C346: jsr $2783a.l
pub fn i_04c346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C34C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C34C);
    cpu.pc = 0x2783A;
    16
}
/// $04C34C: jsr $28d70.l
pub fn i_04c34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C352);
    cpu.pc = 0x28D70;
    16
}
/// $04C352: movea.l $c(a6), a0
pub fn i_04c352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C356;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C356: tst.b $20(a0)
pub fn i_04c356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C35A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C35A: beq.w $4c364
pub fn i_04c35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C35E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C364; return 10; }
    8
}
/// $04C35E: lea.l $4c368(pc), a1
pub fn i_04c35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C362;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C368;
    16
}
/// $04C362: move.l a1, (a6)
pub fn i_04c362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C364;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C364: bra.w $4c416
pub fn i_04c364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C368;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C416;
    10
}
// ===== GunPlatform_RiderB_Alert_04c368  ($04C368, 12 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C368: lea.l $28e8f4.l, a0
pub fn i_04c368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C36E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E8F4;
    20
}
/// $04C36E: jsr $28cd4.l
pub fn i_04c36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C374);
    cpu.pc = 0x28CD4;
    16
}
// ===== GunPlatform_RiderB_Anim_04c374  ($04C374, 64 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C374: lea.l $4c37a(pc), a1
pub fn i_04c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C378;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C37A;
    16
}
/// $04C378: move.l a1, (a6)
pub fn i_04c378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C37A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C37A: jsr $2783a.l
pub fn i_04c37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C380;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C380);
    cpu.pc = 0x2783A;
    16
}
/// $04C380: jsr $28d70.l
pub fn i_04c380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C386;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C386);
    cpu.pc = 0x28D70;
    16
}
/// $04C386: bcc.w $4c3b0
pub fn i_04c386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C38A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C3B0; return 10; }
    8
}
/// $04C38A: movea.l $c(a6), a0
pub fn i_04c38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C38E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C38E: tst.b $20(a0)
pub fn i_04c38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C392;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C392: bne.w $4c39c
pub fn i_04c392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C396;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C39C; return 10; }
    8
}
/// $04C396: lea.l $4c3ea(pc), a1
pub fn i_04c396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C39A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C3EA;
    16
}
/// $04C39A: move.l a1, (a6)
pub fn i_04c39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C39C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C39C: movea.l $c(a6), a0
pub fn i_04c39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3A0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C3A0: cmpi.b #$3, $20(a0)
pub fn i_04c3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3A6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $04C3A6: bne.w $4c3b0
pub fn i_04c3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3AA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C3B0; return 10; }
    8
}
/// $04C3AA: lea.l $4c3b4(pc), a1
pub fn i_04c3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C3B4;
    16
}
/// $04C3AE: move.l a1, (a6)
pub fn i_04c3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C3B0: bra.w $4c416
pub fn i_04c3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C416;
    10
}
// ===== GunPlatform_RiderB_Fire_04c3b4  ($04C3B4, 54 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C3B4: lea.l $28ec54.l, a0
pub fn i_04c3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28EC54;
    20
}
/// $04C3BA: jsr $28cd4.l
pub fn i_04c3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C3C0);
    cpu.pc = 0x28CD4;
    16
}
/// $04C3C0: lea.l $4c3c6(pc), a1
pub fn i_04c3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C3C6;
    16
}
/// $04C3C4: move.l a1, (a6)
pub fn i_04c3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C3C6: jsr $2783a.l
pub fn i_04c3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C3CC);
    cpu.pc = 0x2783A;
    16
}
/// $04C3CC: jsr $28d70.l
pub fn i_04c3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C3D2);
    cpu.pc = 0x28D70;
    16
}
/// $04C3D2: movea.l $c(a6), a0
pub fn i_04c3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3D6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C3D6: cmpi.b #$3, $20(a0)
pub fn i_04c3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3DC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $04C3DC: beq.w $4c3e6
pub fn i_04c3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C3E6; return 10; }
    8
}
/// $04C3E0: lea.l $4c374(pc), a1
pub fn i_04c3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C374;
    16
}
/// $04C3E4: move.l a1, (a6)
pub fn i_04c3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C3E6: bra.w $4c416
pub fn i_04c3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C416;
    10
}
// ===== GunPlatform_RiderB_Return_04c3ea  ($04C3EA, 88 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C3EA: lea.l $28e934.l, a0
pub fn i_04c3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E934;
    20
}
/// $04C3F0: jsr $28cd4.l
pub fn i_04c3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C3F6);
    cpu.pc = 0x28CD4;
    16
}
/// $04C3F6: lea.l $4c3fc(pc), a1
pub fn i_04c3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C3FC;
    16
}
/// $04C3FA: move.l a1, (a6)
pub fn i_04c3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C3FC: jsr $2783a.l
pub fn i_04c3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C402;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C402);
    cpu.pc = 0x2783A;
    16
}
/// $04C402: jsr $28d70.l
pub fn i_04c402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C408);
    cpu.pc = 0x28D70;
    16
}
/// $04C408: bcc.w $4c412
pub fn i_04c408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C40C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C412; return 10; }
    8
}
/// $04C40C: lea.l $4c334(pc), a1
pub fn i_04c40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C410;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C334;
    16
}
/// $04C410: move.l a1, (a6)
pub fn i_04c410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C412;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C412: bra.w $4c416
pub fn i_04c412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C416;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C416;
    10
}
/// $04C416: jsr $2870a.l
pub fn i_04c416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C41C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C41C);
    cpu.pc = 0x2870A;
    16
}
/// $04C41C: bcc.w $4c428
pub fn i_04c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C420;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C428; return 10; }
    8
}
/// $04C420: lea.l $4a146.l, a1
pub fn i_04c420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C426;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4A146;
    20
}
/// $04C426: move.l a1, (a6)
pub fn i_04c426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C428;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C428: jsr $4c942(pc)
pub fn i_04c428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C42C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C42C);
    cpu.pc = 0x4C942;
    16
}
/// $04C42C: movea.l #$ffffffff, a0
pub fn i_04c42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C432;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C432: lea.l $290912.l, a0
pub fn i_04c432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C438;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C438: jsr $5dd5c.l
pub fn i_04c438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C43E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C43E);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C43E: bcc.w $4c448
pub fn i_04c43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C442;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C448; return 10; }
    8
}
// ===== SetTaskHandler_04c442  ($04C442, 8 B, kernel/task_handlers.c)
/// $04C442: lea.l $4c58c(pc), a1
pub fn i_04c442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C446;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C446: move.l a1, (a6)
pub fn i_04c446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C448;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C448: rts
pub fn i_04c448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C44A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_RiderA_Init_04c44a  ($04C44A, 68 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C44A: clr.b $20(a6)
pub fn i_04c44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C44E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C44E: clr.b $21(a6)
pub fn i_04c44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C452;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C452: move.w #$1, $66(a6)
pub fn i_04c452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C458;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C458: move.w #$38, d1
pub fn i_04c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C45C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C45C: jsr $236e.l
pub fn i_04c45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C462);
    cpu.pc = 0x236E;
    16
}
/// $04C462: jsr $5e7c0.l
pub fn i_04c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C468);
    cpu.pc = 0x5E7C0;
    16
}
/// $04C468: jsr $267e2.l
pub fn i_04c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C46E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C46E);
    cpu.pc = 0x267E2;
    16
}
/// $04C46E: lea.l $290c20.l, a0
pub fn i_04c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C474;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290C20;
    20
}
/// $04C474: move.l a0, $48(a6)
pub fn i_04c474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C478;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04C478: move.w #$2000, d0
pub fn i_04c478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C47C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C47C: jsr $28134.l
pub fn i_04c47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C482);
    cpu.pc = 0x28134;
    16
}
/// $04C482: andi.w #$ffe3, $38(a6)
pub fn i_04c482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C488;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C488: ori.w #$8, $38(a6)
pub fn i_04c488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C48E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== GunPlatform_RiderA_Idle_04c48e  ($04C48E, 52 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C48E: lea.l $28e6b0.l, a0
pub fn i_04c48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C494;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6B0;
    20
}
/// $04C494: jsr $28cd4.l
pub fn i_04c494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C49A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C49A);
    cpu.pc = 0x28CD4;
    16
}
/// $04C49A: lea.l $4c4a0(pc), a1
pub fn i_04c49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C49E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C4A0;
    16
}
/// $04C49E: move.l a1, (a6)
pub fn i_04c49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C4A0: jsr $2783a.l
pub fn i_04c4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4A6);
    cpu.pc = 0x2783A;
    16
}
/// $04C4A6: jsr $28d70.l
pub fn i_04c4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4AC);
    cpu.pc = 0x28D70;
    16
}
/// $04C4AC: movea.l $c(a6), a0
pub fn i_04c4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4B0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C4B0: tst.b $20(a0)
pub fn i_04c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4B4;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C4B4: beq.w $4c4be
pub fn i_04c4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C4BE; return 10; }
    8
}
/// $04C4B8: lea.l $4c4c2(pc), a1
pub fn i_04c4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C4C2;
    16
}
/// $04C4BC: move.l a1, (a6)
pub fn i_04c4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C4BE: bra.w $4c544
pub fn i_04c4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C544;
    10
}
// ===== GunPlatform_RiderA_Alert_04c4c2  ($04C4C2, 12 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C4C2: lea.l $28e6bc.l, a0
pub fn i_04c4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6BC;
    20
}
/// $04C4C8: jsr $28cd4.l
pub fn i_04c4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4CE);
    cpu.pc = 0x28CD4;
    16
}
// ===== GunPlatform_RiderA_Anim_04c4ce  ($04C4CE, 64 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C4CE: lea.l $4c4d4(pc), a1
pub fn i_04c4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C4D4;
    16
}
/// $04C4D2: move.l a1, (a6)
pub fn i_04c4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C4D4: jsr $2783a.l
pub fn i_04c4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4DA);
    cpu.pc = 0x2783A;
    16
}
/// $04C4DA: jsr $28d70.l
pub fn i_04c4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4E0);
    cpu.pc = 0x28D70;
    16
}
/// $04C4E0: bcc.w $4c50a
pub fn i_04c4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C50A; return 10; }
    8
}
/// $04C4E4: movea.l $c(a6), a0
pub fn i_04c4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C4E8: tst.b $20(a0)
pub fn i_04c4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4EC;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C4EC: bne.w $4c4f6
pub fn i_04c4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C4F6; return 10; }
    8
}
/// $04C4F0: lea.l $4c48e(pc), a1
pub fn i_04c4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C48E;
    16
}
/// $04C4F4: move.l a1, (a6)
pub fn i_04c4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C4F6: movea.l $c(a6), a0
pub fn i_04c4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4FA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C4FA: cmpi.b #$3, $20(a0)
pub fn i_04c4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C500;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $04C500: bne.w $4c50a
pub fn i_04c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C504;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C50A; return 10; }
    8
}
/// $04C504: lea.l $4c50e(pc), a1
pub fn i_04c504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C508;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C50E;
    16
}
/// $04C508: move.l a1, (a6)
pub fn i_04c508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C50A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C50A: bra.w $4c544
pub fn i_04c50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C50E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C544;
    10
}
// ===== GunPlatform_RiderA_Fire_04c50e  ($04C50E, 98 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C50E: lea.l $28e6c8.l, a0
pub fn i_04c50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C514;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6C8;
    20
}
/// $04C514: jsr $28cd4.l
pub fn i_04c514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C51A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C51A);
    cpu.pc = 0x28CD4;
    16
}
/// $04C51A: lea.l $4c520(pc), a1
pub fn i_04c51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C51E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C520;
    16
}
/// $04C51E: move.l a1, (a6)
pub fn i_04c51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C520;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C520: jsr $2783a.l
pub fn i_04c520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C526);
    cpu.pc = 0x2783A;
    16
}
/// $04C526: jsr $28d70.l
pub fn i_04c526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C52C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C52C);
    cpu.pc = 0x28D70;
    16
}
/// $04C52C: movea.l $c(a6), a0
pub fn i_04c52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C530;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C530: cmpi.b #$3, $20(a0)
pub fn i_04c530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C536;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $04C536: beq.w $4c540
pub fn i_04c536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C53A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C540; return 10; }
    8
}
/// $04C53A: lea.l $4c4ce(pc), a1
pub fn i_04c53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C53E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C4CE;
    16
}
/// $04C53E: move.l a1, (a6)
pub fn i_04c53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C540;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C540: bra.w $4c544
pub fn i_04c540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C544;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C544;
    10
}
/// $04C544: jsr $2870a.l
pub fn i_04c544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C54A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C54A);
    cpu.pc = 0x2870A;
    16
}
/// $04C54A: bcc.w $4c556
pub fn i_04c54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C54E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C556; return 10; }
    8
}
/// $04C54E: lea.l $4a154.l, a1
pub fn i_04c54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C554;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4A154;
    20
}
/// $04C554: move.l a1, (a6)
pub fn i_04c554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C556;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C556: jsr $4c942(pc)
pub fn i_04c556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C55A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C55A);
    cpu.pc = 0x4C942;
    16
}
/// $04C55A: movea.l #$ffffffff, a0
pub fn i_04c55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C560;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C560: lea.l $290912.l, a0
pub fn i_04c560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C566;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C566: jsr $5dd5c.l
pub fn i_04c566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C56C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C56C);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C56C: bcc.w $4c576
pub fn i_04c56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C570;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C576; return 10; }
    8
}
// ===== SetTaskHandler_04c570  ($04C570, 8 B, kernel/task_handlers.c)
/// $04C570: lea.l $4c58c(pc), a1
pub fn i_04c570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C574;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C574: move.l a1, (a6)
pub fn i_04c574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C576;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C576: rts
pub fn i_04c576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C578;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_MarkDead_04c578  ($04C578, 6 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C578: move.b #$ff, $20(a6)
pub fn i_04c578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C57E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Jsr5B6ThenJmpScheduler_04c57e  ($04C57E, 14 B, kernel/jsr5b6_jmp518.c)
/// $04C57E: jsr $5b6.l
pub fn i_04c57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C584);
    cpu.pc = 0x5B6;
    16
}
/// $04C584: jmp $518.l
pub fn i_04c584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C58A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04C58A: rts
pub fn i_04c58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C58C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_FreeClearBit1_04c58c  ($04C58C, 22 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C58C: btst.b #$1, $12(a6)
pub fn i_04c58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C592;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C592: beq.w $4c59c
pub fn i_04c592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C596;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C59C; return 10; }
    8
}
/// $04C596: bclr.b #$1, $12(a6)
pub fn i_04c596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C59C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C59C: jmp $518.l
pub fn i_04c59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== GunPlatform_Nop_04c5a2  ($04C5A2, 2 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C5A2: rts
pub fn i_04c5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Destroyed_04c5a4  ($04C5A4, 90 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C5A4: clr.b $78(a6)
pub fn i_04c5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5A8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C5A8: jsr $4ca98(pc)
pub fn i_04c5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C5AC);
    cpu.pc = 0x4CA98;
    16
}
/// $04C5AC: move.w #$1023, d0
pub fn i_04c5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C5B0: jsr $2352.l
pub fn i_04c5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5B6);
    cpu.pc = 0x2352;
    16
}
/// $04C5B6: lea.l $290ea4.l, a1
pub fn i_04c5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EA4;
    20
}
/// $04C5BC: jsr $77c7e.l
pub fn i_04c5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5C2);
    cpu.pc = 0x77C7E;
    16
}
/// $04C5C2: lea.l $290eb6.l, a1
pub fn i_04c5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5C8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EB6;
    20
}
/// $04C5C8: jsr $77c7e.l
pub fn i_04c5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5CE);
    cpu.pc = 0x77C7E;
    16
}
/// $04C5CE: move.w #$a, $88(a6)
pub fn i_04c5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C5D4: lea.l $4c5da(pc), a1
pub fn i_04c5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C5DA;
    16
}
/// $04C5D8: move.l a1, (a6)
pub fn i_04c5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C5DA: jsr $2783a.l
pub fn i_04c5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5E0);
    cpu.pc = 0x2783A;
    16
}
/// $04C5E0: jsr $28d70.l
pub fn i_04c5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5E6);
    cpu.pc = 0x28D70;
    16
}
/// $04C5E6: subq.w #$1, $88(a6)
pub fn i_04c5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5EA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C5EA: bne.w $4c604
pub fn i_04c5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C604; return 10; }
    8
}
/// $04C5EE: move.b #$ff, $20(a6)
pub fn i_04c5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C5F4: jsr $4ca4a(pc)
pub fn i_04c5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C5F8);
    cpu.pc = 0x4CA4A;
    16
}
/// $04C5F8: bclr.b #$1, $12(a6)
pub fn i_04c5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5FE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_04c5fe  ($04C5FE, 8 B, kernel/task_handlers.c)
/// $04C5FE: lea.l $4c68a(pc), a1
pub fn i_04c5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C602;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C68A;
    16
}
/// $04C602: move.l a1, (a6)
pub fn i_04c602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C604;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C604: rts
pub fn i_04c604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C606;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_DestroyedWithWreck_04c606  ($04C606, 124 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C606: clr.b $78(a6)
pub fn i_04c606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C60A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C60A: jsr $4ca98(pc)
pub fn i_04c60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C60E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C60E);
    cpu.pc = 0x4CA98;
    16
}
/// $04C60E: bclr.b #$1, $12(a6)
pub fn i_04c60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C614;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C614: lea.l $290374.l, a1
pub fn i_04c614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C61A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290374;
    20
}
/// $04C61A: move.w $70(a6), d1
pub fn i_04c61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C61E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C61E: add.w d1, d1
pub fn i_04c61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C620;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C620: add.w d1, d1
pub fn i_04c620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C622;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C622: movea.l (a1, d1.w), a0
pub fn i_04c622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C626;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C626: cmpa.l #$ffffffff, a0
pub fn i_04c626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C62C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04C62C: beq.w $4c5a4
pub fn i_04c62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C630;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C5A4; return 10; }
    8
}
/// $04C630: jsr $28cd4.l
pub fn i_04c630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C636);
    cpu.pc = 0x28CD4;
    16
}
/// $04C636: lea.l $4c68a(pc), a1
pub fn i_04c636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C63A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C68A;
    16
}
/// $04C63A: jsr $4ae.l
pub fn i_04c63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C640);
    cpu.pc = 0x4AE;
    16
}
/// $04C640: jsr $5dd02.l
pub fn i_04c640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C646);
    cpu.pc = 0x5DD02;
    16
}
/// $04C646: move.w $38(a6), d0
pub fn i_04c646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C64A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C64A: subq.w #$8, d0
pub fn i_04c64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C64C;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04C64C: move.w d0, $38(a0)
pub fn i_04c64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C650;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C650: lea.l $4c656(pc), a1
pub fn i_04c650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C654;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C656;
    16
}
/// $04C654: move.l a1, (a6)
pub fn i_04c654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C656;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C656: jsr $2783a.l
pub fn i_04c656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C65C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C65C);
    cpu.pc = 0x2783A;
    16
}
/// $04C65C: jsr $28d70.l
pub fn i_04c65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C662);
    cpu.pc = 0x28D70;
    16
}
/// $04C662: bcc.w $4c66c
pub fn i_04c662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C666;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C66C; return 10; }
    8
}
/// $04C666: lea.l $4c68a(pc), a1
pub fn i_04c666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C66A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C68A;
    16
}
/// $04C66A: move.l a1, (a6)
pub fn i_04c66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C66C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C66C: movea.l #$ffffffff, a0
pub fn i_04c66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C672;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C672: lea.l $290912.l, a0
pub fn i_04c672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C678;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C678: jsr $5dd5c.l
pub fn i_04c678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C67E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C67E);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C67E: bcc.w $4c688
pub fn i_04c67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C682;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C688; return 10; }
    8
}
// ===== SetTaskHandler_04c682  ($04C682, 8 B, kernel/task_handlers.c)
/// $04C682: lea.l $4c58c(pc), a1
pub fn i_04c682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C686;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C686: move.l a1, (a6)
pub fn i_04c686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C688;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C688: rts
pub fn i_04c688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C68A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Wreck_04c68a  ($04C68A, 66 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C68A: clr.b $78(a6)
pub fn i_04c68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C68E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C68E: move.w #$84, d1
pub fn i_04c68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C692;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C692: jsr $236e.l
pub fn i_04c692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C698);
    cpu.pc = 0x236E;
    16
}
/// $04C698: lea.l $290864.l, a0
pub fn i_04c698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C69E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290864;
    20
}
/// $04C69E: jsr $28cd4.l
pub fn i_04c69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6A4);
    cpu.pc = 0x28CD4;
    16
}
/// $04C6A4: lea.l $4c6aa(pc), a1
pub fn i_04c6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C6AA;
    16
}
/// $04C6A8: move.l a1, (a6)
pub fn i_04c6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C6AA: jsr $2783a.l
pub fn i_04c6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6B0);
    cpu.pc = 0x2783A;
    16
}
/// $04C6B0: jsr $28d70.l
pub fn i_04c6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6B6);
    cpu.pc = 0x28D70;
    16
}
/// $04C6B6: movea.l #$ffffffff, a0
pub fn i_04c6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6BC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C6BC: lea.l $290912.l, a0
pub fn i_04c6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C6C2: jsr $5dd5c.l
pub fn i_04c6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6C8);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C6C8: bcc.w $4c6d2
pub fn i_04c6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C6D2; return 10; }
    8
}
// ===== SetTaskHandler_04c6cc  ($04C6CC, 8 B, kernel/task_handlers.c)
/// $04C6CC: lea.l $4c58c(pc), a1
pub fn i_04c6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C6D0: move.l a1, (a6)
pub fn i_04c6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C6D2: rts
pub fn i_04c6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HitCheckUnlessDead_04c6d4  ($04C6D4, 14 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C6D4: cmpi.b #$ff, $20(a6)
pub fn i_04c6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6DA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04C6DA: bne.w $4c6ea
pub fn i_04c6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C6EA; return 10; }
    8
}
/// $04C6DE: jsr $4ca4a(pc)
pub fn i_04c6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C6E2);
    cpu.pc = 0x4CA4A;
    16
}
// ===== SetTaskHandler_04c6e2  ($04C6E2, 8 B, kernel/task_handlers.c)
/// $04C6E2: lea.l $4c606(pc), a1
pub fn i_04c6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C606;
    16
}
/// $04C6E6: move.l a1, (a6)
pub fn i_04c6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C6E8: rts
pub fn i_04c6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HitCheck_04c6ea  ($04C6EA, 128 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C6EA: clr.b $78(a6)
pub fn i_04c6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6EE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C6EE: jsr $2870a.l
pub fn i_04c6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6F4);
    cpu.pc = 0x2870A;
    16
}
/// $04C6F4: bcc.w $4c75a
pub fn i_04c6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C75A; return 10; }
    8
}
/// $04C6F8: move.b #$1, $78(a6)
pub fn i_04c6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C6FE: bclr.b #$3, $13(a6)
pub fn i_04c6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C704;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C704: cmpi.b #$1, $21(a6)
pub fn i_04c704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C70A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04C70A: beq.w $4c75a
pub fn i_04c70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C70E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C75A; return 10; }
    8
}
/// $04C70E: cmpi.w #$32, $66(a6)
pub fn i_04c70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C714;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x32, Size::Word, false);
    16
}
/// $04C714: bge.w $4c75a
pub fn i_04c714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C718;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x4C75A; return 10; }
    8
}
/// $04C718: move.b #$1, $21(a6)
pub fn i_04c718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C71E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C71E: move.w #$1027, d0
pub fn i_04c71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C722;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C722: jsr $2352.l
pub fn i_04c722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C728);
    cpu.pc = 0x2352;
    16
}
/// $04C728: cmpi.b #$1, $58(a6)
pub fn i_04c728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C72E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04C72E: beq.w $4c742
pub fn i_04c72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C732;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C742; return 10; }
    8
}
/// $04C732: lea.l $290eec.l, a1
pub fn i_04c732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C738;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EEC;
    20
}
/// $04C738: jsr $77c7e.l
pub fn i_04c738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C73E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C73E);
    cpu.pc = 0x77C7E;
    16
}
/// $04C73E: bra.w $4c74e
pub fn i_04c73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C742;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C74E;
    10
}
/// $04C742: lea.l $290ec8.l, a1
pub fn i_04c742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C748;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EC8;
    20
}
/// $04C748: jsr $77c7e.l
pub fn i_04c748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C74E);
    cpu.pc = 0x77C7E;
    16
}
/// $04C74E: lea.l $290eda.l, a1
pub fn i_04c74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C754;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EDA;
    20
}
/// $04C754: jsr $77c7e.l
pub fn i_04c754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C75A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C75A);
    cpu.pc = 0x77C7E;
    16
}
/// $04C75A: jsr $28758.l
pub fn i_04c75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C760);
    cpu.pc = 0x28758;
    16
}
/// $04C760: bcc.w $4c770
pub fn i_04c760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C764;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C770; return 10; }
    8
}
/// $04C764: lea.l $4c5a4(pc), a1
pub fn i_04c764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C5A4;
    16
}
/// $04C768: move.l a1, (a6)
pub fn i_04c768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C76A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== SetXN_04c76a  ($04C76A, 6 B, thunks/ccr_helpers.c)
/// $04C76A: ori.b #$11, ccr
pub fn i_04c76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C76E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04C76E: rts
pub fn i_04c76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C770;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04c770  ($04C770, 6 B, thunks/ccr_helpers.c)
/// $04C770: andi.b #$ee, ccr
pub fn i_04c770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C774;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04C774: rts
pub fn i_04c774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C776;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Gun_04c776  ($04C776, 180 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C776: move.w #$84, d1
pub fn i_04c776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C77A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C77A: jsr $236e.l
pub fn i_04c77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C780);
    cpu.pc = 0x236E;
    16
}
/// $04C780: move.w #$19, $1c(a6)
pub fn i_04c780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C786;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C786: jsr $138fe.l
pub fn i_04c786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C78C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C78C);
    cpu.pc = 0x138FE;
    16
}
/// $04C78C: move.w $70(a6), d0
pub fn i_04c78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C790;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C790: andi.w #$3, d0
pub fn i_04c790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C794;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C794: lsl.w #$2, d0
pub fn i_04c794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C796;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04C796: lea.l $290efe.l, a1
pub fn i_04c796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C79C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EFE;
    20
}
/// $04C79C: move.w (a1, d0.w), $74(a6)
pub fn i_04c79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7A2;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    22
}
/// $04C7A2: move.w $2(a1, d0.w), $76(a6)
pub fn i_04c7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7A8;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    22
}
/// $04C7A8: move.l #$290d70, $4c(a6)
pub fn i_04c7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7B0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x290D70;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04C7B0: lea.l $290876.l, a0
pub fn i_04c7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290876;
    20
}
/// $04C7B6: jsr $28cd4.l
pub fn i_04c7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C7BC);
    cpu.pc = 0x28CD4;
    16
}
/// $04C7BC: lea.l $4c7c2(pc), a1
pub fn i_04c7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C7C2;
    16
}
/// $04C7C0: move.l a1, (a6)
pub fn i_04c7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C7C2: movea.l $c(a6), a0
pub fn i_04c7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7C6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C7C6: cmpi.b #$1, $21(a0)
pub fn i_04c7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7CC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04C7CC: bne.w $4c7d6
pub fn i_04c7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C7D6; return 10; }
    8
}
/// $04C7D0: lea.l $4c832(pc), a1
pub fn i_04c7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C832;
    16
}
/// $04C7D4: move.l a1, (a6)
pub fn i_04c7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C7D6: jsr $5e506.l
pub fn i_04c7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C7DC);
    cpu.pc = 0x5E506;
    16
}
/// $04C7DC: tst.b $78(a0)
pub fn i_04c7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7E0;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C7E0: beq.w $4c7ea
pub fn i_04c7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C7EA; return 10; }
    8
}
/// $04C7E4: bset.b #$0, $5a(a6)
pub fn i_04c7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7EA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C7EA: move.w $74(a6), d0
pub fn i_04c7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C7EE: add.w d0, $22(a6)
pub fn i_04c7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7F2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C7F2: move.w $76(a6), d0
pub fn i_04c7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7F6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C7F6: add.w d0, $24(a6)
pub fn i_04c7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7FA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C7FA: jsr $28d70.l
pub fn i_04c7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C800;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C800);
    cpu.pc = 0x28D70;
    16
}
/// $04C800: jsr $283ca.l
pub fn i_04c800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C806);
    cpu.pc = 0x283CA;
    16
}
/// $04C806: jsr $283d8.l
pub fn i_04c806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C80C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C80C);
    cpu.pc = 0x283D8;
    16
}
/// $04C80C: movea.l $c(a6), a0
pub fn i_04c80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C810;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C810: cmpi.b #$ff, $20(a0)
pub fn i_04c810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C816;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04C816: bne.w $4c820
pub fn i_04c816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C81A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C820; return 10; }
    8
}
/// $04C81A: lea.l $4c934(pc), a1
pub fn i_04c81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C81E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C934;
    16
}
/// $04C81E: move.l a1, (a6)
pub fn i_04c81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C820: jsr $5e45a.l
pub fn i_04c820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C826);
    cpu.pc = 0x5E45A;
    16
}
/// $04C826: bcc.w $4c830
pub fn i_04c826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C82A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C830; return 10; }
    8
}
// ===== SetTaskHandler_04c82a  ($04C82A, 8 B, kernel/task_handlers.c)
/// $04C82A: lea.l $4c934(pc), a1
pub fn i_04c82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C82E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C934;
    16
}
/// $04C82E: move.l a1, (a6)
pub fn i_04c82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C830;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C830: rts
pub fn i_04c830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C832;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_GunDamaged_04c832  ($04C832, 20 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C832: lea.l $290882.l, a0
pub fn i_04c832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C838;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290882;
    20
}
/// $04C838: jsr $28cd4.l
pub fn i_04c838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C83E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C83E);
    cpu.pc = 0x28CD4;
    16
}
/// $04C83E: lea.l $4c844(pc), a1
pub fn i_04c83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C842;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C844;
    16
}
/// $04C842: move.l a1, (a6)
pub fn i_04c842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C844;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C844: bra.b $4c7d6
pub fn i_04c844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C846;
    cpu.ir = 0x6090;
    cpu.pc = 0x4C7D6;
    10
}
// ===== GunPlatform_FlyingPart_04c846  ($04C846, 194 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C846: lea.l $2908c8.l, a0
pub fn i_04c846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C84C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2908C8;
    20
}
/// $04C84C: jsr $28cd4.l
pub fn i_04c84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C852);
    cpu.pc = 0x28CD4;
    16
}
/// $04C852: subq.w #$8, $24(a6)
pub fn i_04c852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C856;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C856: move.w $36(a6), d0
pub fn i_04c856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C85A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C85A: cmpi.w #$0, d0
pub fn i_04c85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C85E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $04C85E: bgt.w $4c886
pub fn i_04c85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C862;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4C886; return 10; }
    8
}
/// $04C862: move.w $36(a6), d0
pub fn i_04c862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C866;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C866: cmpi.w #$0, d0
pub fn i_04c866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C86A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $04C86A: bgt.w $4c87a
pub fn i_04c86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C86E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4C87A; return 10; }
    8
}
/// $04C86E: lea.l $2bd1de.l, a0
pub fn i_04c86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C874;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD1DE;
    20
}
/// $04C874: jsr $799de.l
pub fn i_04c874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C87A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C87A);
    cpu.pc = 0x799DE;
    16
}
/// $04C87A: lea.l $29088e.l, a0
pub fn i_04c87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C880;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29088E;
    20
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x049D72, i_049d72),
    (0x049D76, i_049d76),
    (0x049D7A, i_049d7a),
    (0x049D7E, i_049d7e),
    (0x049D82, i_049d82),
    (0x049D84, i_049d84),
    (0x049D88, i_049d88),
    (0x049FBA, i_049fba),
    (0x049FC0, i_049fc0),
    (0x049FC4, i_049fc4),
    (0x049FC8, i_049fc8),
    (0x049FCA, i_049fca),
    (0x049FD0, i_049fd0),
    (0x049FD2, i_049fd2),
    (0x049FD6, i_049fd6),
    (0x049FDC, i_049fdc),
    (0x049FE0, i_049fe0),
    (0x049FE4, i_049fe4),
    (0x049FE6, i_049fe6),
    (0x049FEA, i_049fea),
    (0x049FEE, i_049fee),
    (0x049FF0, i_049ff0),
    (0x049FF2, i_049ff2),
    (0x049FF4, i_049ff4),
    (0x049FF8, i_049ff8),
    (0x049FFE, i_049ffe),
    (0x04A002, i_04a002),
    (0x04A006, i_04a006),
    (0x04A008, i_04a008),
    (0x04A00C, i_04a00c),
    (0x04A010, i_04a010),
    (0x04A012, i_04a012),
    (0x04A014, i_04a014),
    (0x04A01A, i_04a01a),
    (0x04A01E, i_04a01e),
    (0x04A020, i_04a020),
    (0x04A024, i_04a024),
    (0x04A02A, i_04a02a),
    (0x04A02E, i_04a02e),
    (0x04A030, i_04a030),
    (0x04A034, i_04a034),
    (0x04A03A, i_04a03a),
    (0x04A03E, i_04a03e),
    (0x04A040, i_04a040),
    (0x04A044, i_04a044),
    (0x04A04A, i_04a04a),
    (0x04A04E, i_04a04e),
    (0x04A050, i_04a050),
    (0x04A054, i_04a054),
    (0x04A058, i_04a058),
    (0x04A05A, i_04a05a),
    (0x04A060, i_04a060),
    (0x04A062, i_04a062),
    (0x04A064, i_04a064),
    (0x04A068, i_04a068),
    (0x04A06C, i_04a06c),
    (0x04A06E, i_04a06e),
    (0x04A072, i_04a072),
    (0x04A076, i_04a076),
    (0x04A078, i_04a078),
    (0x04A07C, i_04a07c),
    (0x04A080, i_04a080),
    (0x04A084, i_04a084),
    (0x04A088, i_04a088),
    (0x04A08A, i_04a08a),
    (0x04A08E, i_04a08e),
    (0x04A092, i_04a092),
    (0x04A098, i_04a098),
    (0x04A09A, i_04a09a),
    (0x04A09C, i_04a09c),
    (0x04A0A2, i_04a0a2),
    (0x04A0A6, i_04a0a6),
    (0x04A0AC, i_04a0ac),
    (0x04A0B2, i_04a0b2),
    (0x04A0BA, i_04a0ba),
    (0x04A0C0, i_04a0c0),
    (0x04A0C4, i_04a0c4),
    (0x04A0C8, i_04a0c8),
    (0x04A0CC, i_04a0cc),
    (0x04A0D2, i_04a0d2),
    (0x04A0D4, i_04a0d4),
    (0x04A0D8, i_04a0d8),
    (0x04A0DE, i_04a0de),
    (0x04A0E2, i_04a0e2),
    (0x04A0E6, i_04a0e6),
    (0x04A0EA, i_04a0ea),
    (0x04A0EE, i_04a0ee),
    (0x04A0F2, i_04a0f2),
    (0x04A0F4, i_04a0f4),
    (0x04A0F8, i_04a0f8),
    (0x04A0FE, i_04a0fe),
    (0x04A102, i_04a102),
    (0x04A108, i_04a108),
    (0x04A10C, i_04a10c),
    (0x04A110, i_04a110),
    (0x04A114, i_04a114),
    (0x04A118, i_04a118),
    (0x04A11C, i_04a11c),
    (0x04A120, i_04a120),
    (0x04A124, i_04a124),
    (0x04A128, i_04a128),
    (0x04A12C, i_04a12c),
    (0x04A130, i_04a130),
    (0x04A132, i_04a132),
    (0x04A136, i_04a136),
    (0x04A13C, i_04a13c),
    (0x04A140, i_04a140),
    (0x04A146, i_04a146),
    (0x04A14C, i_04a14c),
    (0x04A150, i_04a150),
    (0x04A154, i_04a154),
    (0x04A15A, i_04a15a),
    (0x04A15E, i_04a15e),
    (0x04A162, i_04a162),
    (0x04A166, i_04a166),
    (0x04A16A, i_04a16a),
    (0x04A16C, i_04a16c),
    (0x04A16E, i_04a16e),
    (0x04A172, i_04a172),
    (0x04A178, i_04a178),
    (0x04A17E, i_04a17e),
    (0x04A184, i_04a184),
    (0x04A18A, i_04a18a),
    (0x04A18C, i_04a18c),
    (0x04A190, i_04a190),
    (0x04A194, i_04a194),
    (0x04A198, i_04a198),
    (0x04A19E, i_04a19e),
    (0x04A1A4, i_04a1a4),
    (0x04A1AA, i_04a1aa),
    (0x04A1B0, i_04a1b0),
    (0x04A1B2, i_04a1b2),
    (0x04A1B6, i_04a1b6),
    (0x04A1BA, i_04a1ba),
    (0x04A1BE, i_04a1be),
    (0x04A1C4, i_04a1c4),
    (0x04A1C6, i_04a1c6),
    (0x04A1CA, i_04a1ca),
    (0x04A1CE, i_04a1ce),
    (0x04A1D0, i_04a1d0),
    (0x04A1D2, i_04a1d2),
    (0x04A1D4, i_04a1d4),
    (0x04A1D6, i_04a1d6),
    (0x04A1DA, i_04a1da),
    (0x04A1DE, i_04a1de),
    (0x04A1E2, i_04a1e2),
    (0x04A1E4, i_04a1e4),
    (0x04A1E6, i_04a1e6),
    (0x04A1E8, i_04a1e8),
    (0x04A1EA, i_04a1ea),
    (0x04A1EE, i_04a1ee),
    (0x04A1F0, i_04a1f0),
    (0x04A1F6, i_04a1f6),
    (0x04A1FA, i_04a1fa),
    (0x04A1FC, i_04a1fc),
    (0x04A200, i_04a200),
    (0x04A202, i_04a202),
    (0x04A208, i_04a208),
    (0x04A20C, i_04a20c),
    (0x04A20E, i_04a20e),
    (0x04A210, i_04a210),
    (0x04A216, i_04a216),
    (0x04A218, i_04a218),
    (0x04A21E, i_04a21e),
    (0x04A222, i_04a222),
    (0x04A226, i_04a226),
    (0x04A22A, i_04a22a),
    (0x04A22E, i_04a22e),
    (0x04A232, i_04a232),
    (0x04A236, i_04a236),
    (0x04A23A, i_04a23a),
    (0x04A240, i_04a240),
    (0x04A242, i_04a242),
    (0x04A246, i_04a246),
    (0x04A24A, i_04a24a),
    (0x04A250, i_04a250),
    (0x04A256, i_04a256),
    (0x04A25A, i_04a25a),
    (0x04A25C, i_04a25c),
    (0x04A260, i_04a260),
    (0x04A266, i_04a266),
    (0x04A268, i_04a268),
    (0x04A26E, i_04a26e),
    (0x04A272, i_04a272),
    (0x04A276, i_04a276),
    (0x04A27A, i_04a27a),
    (0x04A27E, i_04a27e),
    (0x04A282, i_04a282),
    (0x04A286, i_04a286),
    (0x04A28A, i_04a28a),
    (0x04A290, i_04a290),
    (0x04A294, i_04a294),
    (0x04A298, i_04a298),
    (0x04A29C, i_04a29c),
    (0x04A2A2, i_04a2a2),
    (0x04A2A6, i_04a2a6),
    (0x04A2A8, i_04a2a8),
    (0x04A2AC, i_04a2ac),
    (0x04A2B2, i_04a2b2),
    (0x04A2B8, i_04a2b8),
    (0x04A2BC, i_04a2bc),
    (0x04A2BE, i_04a2be),
    (0x04A2C4, i_04a2c4),
    (0x04A2C8, i_04a2c8),
    (0x04A2CC, i_04a2cc),
    (0x04A2D0, i_04a2d0),
    (0x04A2D4, i_04a2d4),
    (0x04A2D8, i_04a2d8),
    (0x04A2DC, i_04a2dc),
    (0x04A2E0, i_04a2e0),
    (0x04A2E6, i_04a2e6),
    (0x04A2EA, i_04a2ea),
    (0x04A2EE, i_04a2ee),
    (0x04A2F4, i_04a2f4),
    (0x04A2FA, i_04a2fa),
    (0x04A2FE, i_04a2fe),
    (0x04A304, i_04a304),
    (0x04A30A, i_04a30a),
    (0x04A30E, i_04a30e),
    (0x04A310, i_04a310),
    (0x04A316, i_04a316),
    (0x04A31A, i_04a31a),
    (0x04A31E, i_04a31e),
    (0x04A322, i_04a322),
    (0x04A326, i_04a326),
    (0x04A32A, i_04a32a),
    (0x04A32E, i_04a32e),
    (0x04A332, i_04a332),
    (0x04A338, i_04a338),
    (0x04A33C, i_04a33c),
    (0x04A340, i_04a340),
    (0x04A346, i_04a346),
    (0x04A34A, i_04a34a),
    (0x04A34C, i_04a34c),
    (0x04A352, i_04a352),
    (0x04A358, i_04a358),
    (0x04A35C, i_04a35c),
    (0x04A35E, i_04a35e),
    (0x04A362, i_04a362),
    (0x04A366, i_04a366),
    (0x04A36A, i_04a36a),
    (0x04A36E, i_04a36e),
    (0x04A372, i_04a372),
    (0x04A378, i_04a378),
    (0x04A37C, i_04a37c),
    (0x04A380, i_04a380),
    (0x04A382, i_04a382),
    (0x04A386, i_04a386),
    (0x04A388, i_04a388),
    (0x04A38C, i_04a38c),
    (0x04A392, i_04a392),
    (0x04A396, i_04a396),
    (0x04A39A, i_04a39a),
    (0x04A39C, i_04a39c),
    (0x04A39E, i_04a39e),
    (0x04A3A4, i_04a3a4),
    (0x04A3A8, i_04a3a8),
    (0x04A3AA, i_04a3aa),
    (0x04A3AE, i_04a3ae),
    (0x04A3B4, i_04a3b4),
    (0x04A3B8, i_04a3b8),
    (0x04A3BC, i_04a3bc),
    (0x04A3C0, i_04a3c0),
    (0x04A3C2, i_04a3c2),
    (0x04A3C4, i_04a3c4),
    (0x04A3CA, i_04a3ca),
    (0x04A3CE, i_04a3ce),
    (0x04A3D0, i_04a3d0),
    (0x04A3D4, i_04a3d4),
    (0x04A3D8, i_04a3d8),
    (0x04A3DC, i_04a3dc),
    (0x04A3E0, i_04a3e0),
    (0x04A3E4, i_04a3e4),
    (0x04A3EA, i_04a3ea),
    (0x04A3EE, i_04a3ee),
    (0x04A3F4, i_04a3f4),
    (0x04A3F8, i_04a3f8),
    (0x04A3FC, i_04a3fc),
    (0x04A400, i_04a400),
    (0x04A404, i_04a404),
    (0x04A420, i_04a420),
    (0x04A424, i_04a424),
    (0x04A428, i_04a428),
    (0x04A42E, i_04a42e),
    (0x04A430, i_04a430),
    (0x04A434, i_04a434),
    (0x04A438, i_04a438),
    (0x04A43C, i_04a43c),
    (0x04A43E, i_04a43e),
    (0x04A442, i_04a442),
    (0x04A444, i_04a444),
    (0x04A448, i_04a448),
    (0x04A44C, i_04a44c),
    (0x04A44E, i_04a44e),
    (0x04A454, i_04a454),
    (0x04A458, i_04a458),
    (0x04A45E, i_04a45e),
    (0x04A462, i_04a462),
    (0x04A466, i_04a466),
    (0x04A46C, i_04a46c),
    (0x04A470, i_04a470),
    (0x04A474, i_04a474),
    (0x04A476, i_04a476),
    (0x04A47A, i_04a47a),
    (0x04A480, i_04a480),
    (0x04A484, i_04a484),
    (0x04A48A, i_04a48a),
    (0x04A48E, i_04a48e),
    (0x04A492, i_04a492),
    (0x04A494, i_04a494),
    (0x04A498, i_04a498),
    (0x04A49C, i_04a49c),
    (0x04A49E, i_04a49e),
    (0x04A4A2, i_04a4a2),
    (0x04A4A6, i_04a4a6),
    (0x04A4AC, i_04a4ac),
    (0x04A4C8, i_04a4c8),
    (0x04A4CC, i_04a4cc),
    (0x04A4D0, i_04a4d0),
    (0x04A4D6, i_04a4d6),
    (0x04A4D8, i_04a4d8),
    (0x04A4DC, i_04a4dc),
    (0x04A4E0, i_04a4e0),
    (0x04A4E4, i_04a4e4),
    (0x04A4E6, i_04a4e6),
    (0x04A4EA, i_04a4ea),
    (0x04A4EC, i_04a4ec),
    (0x04A4F0, i_04a4f0),
    (0x04A4F4, i_04a4f4),
    (0x04A4F6, i_04a4f6),
    (0x04A4FC, i_04a4fc),
    (0x04A500, i_04a500),
    (0x04A506, i_04a506),
    (0x04A50A, i_04a50a),
    (0x04A50E, i_04a50e),
    (0x04A514, i_04a514),
    (0x04A518, i_04a518),
    (0x04A51C, i_04a51c),
    (0x04A51E, i_04a51e),
    (0x04A522, i_04a522),
    (0x04A528, i_04a528),
    (0x04A52C, i_04a52c),
    (0x04A532, i_04a532),
    (0x04A536, i_04a536),
    (0x04A53A, i_04a53a),
    (0x04A53C, i_04a53c),
    (0x04A540, i_04a540),
    (0x04A544, i_04a544),
    (0x04A546, i_04a546),
    (0x04A54A, i_04a54a),
    (0x04A550, i_04a550),
    (0x04A554, i_04a554),
    (0x04A556, i_04a556),
    (0x04A55A, i_04a55a),
    (0x04A55E, i_04a55e),
    (0x04A564, i_04a564),
    (0x04A568, i_04a568),
    (0x04A56C, i_04a56c),
    (0x04A570, i_04a570),
    (0x04A574, i_04a574),
    (0x04A57A, i_04a57a),
    (0x04A57E, i_04a57e),
    (0x04A582, i_04a582),
    (0x04A586, i_04a586),
    (0x04A58A, i_04a58a),
    (0x04A590, i_04a590),
    (0x04A594, i_04a594),
    (0x04A598, i_04a598),
    (0x04A59E, i_04a59e),
    (0x04A5A2, i_04a5a2),
    (0x04A5A6, i_04a5a6),
    (0x04A5AA, i_04a5aa),
    (0x04A5B0, i_04a5b0),
    (0x04A5B4, i_04a5b4),
    (0x04A5B8, i_04a5b8),
    (0x04A5BC, i_04a5bc),
    (0x04A5C0, i_04a5c0),
    (0x04A5C4, i_04a5c4),
    (0x04A5CA, i_04a5ca),
    (0x04A5CE, i_04a5ce),
    (0x04A5D4, i_04a5d4),
    (0x04A5D8, i_04a5d8),
    (0x04A5DE, i_04a5de),
    (0x04A5E2, i_04a5e2),
    (0x04A5E8, i_04a5e8),
    (0x04A5EC, i_04a5ec),
    (0x04A5F0, i_04a5f0),
    (0x04A5F6, i_04a5f6),
    (0x04A5FA, i_04a5fa),
    (0x04A600, i_04a600),
    (0x04A604, i_04a604),
    (0x04A608, i_04a608),
    (0x04A60E, i_04a60e),
    (0x04A612, i_04a612),
    (0x04A616, i_04a616),
    (0x04A61C, i_04a61c),
    (0x04A620, i_04a620),
    (0x04A624, i_04a624),
    (0x04A62A, i_04a62a),
    (0x04A62E, i_04a62e),
    (0x04A632, i_04a632),
    (0x04A638, i_04a638),
    (0x04A63C, i_04a63c),
    (0x04A640, i_04a640),
    (0x04A646, i_04a646),
    (0x04A64C, i_04a64c),
    (0x04A650, i_04a650),
    (0x04A652, i_04a652),
    (0x04A656, i_04a656),
    (0x04A65C, i_04a65c),
    (0x04A660, i_04a660),
    (0x04A664, i_04a664),
    (0x04A668, i_04a668),
    (0x04A66C, i_04a66c),
    (0x04A66E, i_04a66e),
    (0x04A672, i_04a672),
    (0x04A676, i_04a676),
    (0x04A67C, i_04a67c),
    (0x04A680, i_04a680),
    (0x04A686, i_04a686),
    (0x04A688, i_04a688),
    (0x04A68C, i_04a68c),
    (0x04A690, i_04a690),
    (0x04A696, i_04a696),
    (0x04A69A, i_04a69a),
    (0x04A69C, i_04a69c),
    (0x04A6A0, i_04a6a0),
    (0x04A6A6, i_04a6a6),
    (0x04A6AA, i_04a6aa),
    (0x04A6AE, i_04a6ae),
    (0x04A6B2, i_04a6b2),
    (0x04A6B6, i_04a6b6),
    (0x04A6BC, i_04a6bc),
    (0x04A6C0, i_04a6c0),
    (0x04A6C6, i_04a6c6),
    (0x04A6CC, i_04a6cc),
    (0x04A6D2, i_04a6d2),
    (0x04A6D8, i_04a6d8),
    (0x04A6DC, i_04a6dc),
    (0x04A6E0, i_04a6e0),
    (0x04A6E4, i_04a6e4),
    (0x04A6E6, i_04a6e6),
    (0x04A6E8, i_04a6e8),
    (0x04A6EA, i_04a6ea),
    (0x04A6EE, i_04a6ee),
    (0x04A6F4, i_04a6f4),
    (0x04A6F8, i_04a6f8),
    (0x04A6FC, i_04a6fc),
    (0x04A700, i_04a700),
    (0x04A702, i_04a702),
    (0x04A708, i_04a708),
    (0x04A70C, i_04a70c),
    (0x04A710, i_04a710),
    (0x04A716, i_04a716),
    (0x04A718, i_04a718),
    (0x04A71C, i_04a71c),
    (0x04A722, i_04a722),
    (0x04A726, i_04a726),
    (0x04A72A, i_04a72a),
    (0x04A72E, i_04a72e),
    (0x04A732, i_04a732),
    (0x04A736, i_04a736),
    (0x04A73C, i_04a73c),
    (0x04A740, i_04a740),
    (0x04A744, i_04a744),
    (0x04A748, i_04a748),
    (0x04A74E, i_04a74e),
    (0x04A752, i_04a752),
    (0x04A754, i_04a754),
    (0x04A758, i_04a758),
    (0x04A75C, i_04a75c),
    (0x04A760, i_04a760),
    (0x04A764, i_04a764),
    (0x04A768, i_04a768),
    (0x04A76A, i_04a76a),
    (0x04A76E, i_04a76e),
    (0x04A774, i_04a774),
    (0x04A778, i_04a778),
    (0x04A77E, i_04a77e),
    (0x04A782, i_04a782),
    (0x04A786, i_04a786),
    (0x04A78A, i_04a78a),
    (0x04A78E, i_04a78e),
    (0x04A792, i_04a792),
    (0x04A794, i_04a794),
    (0x04A798, i_04a798),
    (0x04A79C, i_04a79c),
    (0x04A7A2, i_04a7a2),
    (0x04A7A6, i_04a7a6),
    (0x04A7AC, i_04a7ac),
    (0x04A7B0, i_04a7b0),
    (0x04A7B4, i_04a7b4),
    (0x04A7B8, i_04a7b8),
    (0x04A7BE, i_04a7be),
    (0x04A7C2, i_04a7c2),
    (0x04A7C8, i_04a7c8),
    (0x04A7CC, i_04a7cc),
    (0x04A7D2, i_04a7d2),
    (0x04A7D6, i_04a7d6),
    (0x04A7D8, i_04a7d8),
    (0x04A7DC, i_04a7dc),
    (0x04A7E2, i_04a7e2),
    (0x04A7E6, i_04a7e6),
    (0x04A7EA, i_04a7ea),
    (0x04A7EE, i_04a7ee),
    (0x04A7F0, i_04a7f0),
    (0x04A7F4, i_04a7f4),
    (0x04A7F8, i_04a7f8),
    (0x04A7FE, i_04a7fe),
    (0x04A802, i_04a802),
    (0x04A808, i_04a808),
    (0x04A80C, i_04a80c),
    (0x04A812, i_04a812),
    (0x04A816, i_04a816),
    (0x04A81C, i_04a81c),
    (0x04A820, i_04a820),
    (0x04A826, i_04a826),
    (0x04A82A, i_04a82a),
    (0x04A830, i_04a830),
    (0x04A836, i_04a836),
    (0x04A83C, i_04a83c),
    (0x04A840, i_04a840),
    (0x04A846, i_04a846),
    (0x04A84C, i_04a84c),
    (0x04A852, i_04a852),
    (0x04A858, i_04a858),
    (0x04A85C, i_04a85c),
    (0x04A85E, i_04a85e),
    (0x04A862, i_04a862),
    (0x04A864, i_04a864),
    (0x04A866, i_04a866),
    (0x04A868, i_04a868),
    (0x04A86C, i_04a86c),
    (0x04A86E, i_04a86e),
    (0x04A872, i_04a872),
    (0x04A874, i_04a874),
    (0x04A878, i_04a878),
    (0x04A87C, i_04a87c),
    (0x04A882, i_04a882),
    (0x04A888, i_04a888),
    (0x04A88C, i_04a88c),
    (0x04A892, i_04a892),
    (0x04A896, i_04a896),
    (0x04A89A, i_04a89a),
    (0x04A89E, i_04a89e),
    (0x04A8A4, i_04a8a4),
    (0x04A8A8, i_04a8a8),
    (0x04A8AE, i_04a8ae),
    (0x04A8B2, i_04a8b2),
    (0x04A8B8, i_04a8b8),
    (0x04A8BC, i_04a8bc),
    (0x04A8C2, i_04a8c2),
    (0x04A8C6, i_04a8c6),
    (0x04A8CA, i_04a8ca),
    (0x04A8D0, i_04a8d0),
    (0x04A8D4, i_04a8d4),
    (0x04A8D8, i_04a8d8),
    (0x04A8DC, i_04a8dc),
    (0x04A8DE, i_04a8de),
    (0x04A8E2, i_04a8e2),
    (0x04A8E6, i_04a8e6),
    (0x04A8EC, i_04a8ec),
    (0x04A8F0, i_04a8f0),
    (0x04A8F4, i_04a8f4),
    (0x04A8F6, i_04a8f6),
    (0x04A8FA, i_04a8fa),
    (0x04A8FE, i_04a8fe),
    (0x04A902, i_04a902),
    (0x04A908, i_04a908),
    (0x04A90E, i_04a90e),
    (0x04A912, i_04a912),
    (0x04A916, i_04a916),
    (0x04A91A, i_04a91a),
    (0x04A91E, i_04a91e),
    (0x04A920, i_04a920),
    (0x04A924, i_04a924),
    (0x04A928, i_04a928),
    (0x04A92C, i_04a92c),
    (0x04A932, i_04a932),
    (0x04A936, i_04a936),
    (0x04A93A, i_04a93a),
    (0x04A93E, i_04a93e),
    (0x04A942, i_04a942),
    (0x04A944, i_04a944),
    (0x04A948, i_04a948),
    (0x04A94C, i_04a94c),
    (0x04A952, i_04a952),
    (0x04A958, i_04a958),
    (0x04A95E, i_04a95e),
    (0x04A964, i_04a964),
    (0x04A968, i_04a968),
    (0x04A96C, i_04a96c),
    (0x04A970, i_04a970),
    (0x04A976, i_04a976),
    (0x04A97C, i_04a97c),
    (0x04A980, i_04a980),
    (0x04A982, i_04a982),
    (0x04A986, i_04a986),
    (0x04A98C, i_04a98c),
    (0x04A990, i_04a990),
    (0x04A994, i_04a994),
    (0x04A996, i_04a996),
    (0x04A99A, i_04a99a),
    (0x04A99E, i_04a99e),
    (0x04A9A2, i_04a9a2),
    (0x04A9A6, i_04a9a6),
    (0x04A9AA, i_04a9aa),
    (0x04A9AC, i_04a9ac),
    (0x04A9B0, i_04a9b0),
    (0x04A9B4, i_04a9b4),
    (0x04A9BA, i_04a9ba),
    (0x04A9BE, i_04a9be),
    (0x04A9C4, i_04a9c4),
    (0x04A9CA, i_04a9ca),
    (0x04A9CE, i_04a9ce),
    (0x04A9D4, i_04a9d4),
    (0x04A9D8, i_04a9d8),
    (0x04A9DC, i_04a9dc),
    (0x04A9E2, i_04a9e2),
    (0x04A9E6, i_04a9e6),
    (0x04A9EA, i_04a9ea),
    (0x04A9F0, i_04a9f0),
    (0x04A9F6, i_04a9f6),
    (0x04A9F8, i_04a9f8),
    (0x04A9FC, i_04a9fc),
    (0x04AA02, i_04aa02),
    (0x04AA06, i_04aa06),
    (0x04AA0C, i_04aa0c),
    (0x04AA12, i_04aa12),
    (0x04AA18, i_04aa18),
    (0x04AA1C, i_04aa1c),
    (0x04AA1E, i_04aa1e),
    (0x04AA22, i_04aa22),
    (0x04AA28, i_04aa28),
    (0x04AA2E, i_04aa2e),
    (0x04AA32, i_04aa32),
    (0x04AA36, i_04aa36),
    (0x04AA38, i_04aa38),
    (0x04AA3C, i_04aa3c),
    (0x04AA40, i_04aa40),
    (0x04AA44, i_04aa44),
    (0x04AA46, i_04aa46),
    (0x04AA4A, i_04aa4a),
    (0x04AA4E, i_04aa4e),
    (0x04AA54, i_04aa54),
    (0x04AA5A, i_04aa5a),
    (0x04AA5E, i_04aa5e),
    (0x04AA62, i_04aa62),
    (0x04AA66, i_04aa66),
    (0x04AA6C, i_04aa6c),
    (0x04AA72, i_04aa72),
    (0x04AA76, i_04aa76),
    (0x04AA7A, i_04aa7a),
    (0x04AA80, i_04aa80),
    (0x04AA84, i_04aa84),
    (0x04AA8A, i_04aa8a),
    (0x04AA8E, i_04aa8e),
    (0x04AA90, i_04aa90),
    (0x04AA96, i_04aa96),
    (0x04AA9A, i_04aa9a),
    (0x04AAA0, i_04aaa0),
    (0x04AAA4, i_04aaa4),
    (0x04AAA8, i_04aaa8),
    (0x04AAAC, i_04aaac),
    (0x04AAB2, i_04aab2),
    (0x04AAB8, i_04aab8),
    (0x04AABC, i_04aabc),
    (0x04AAC0, i_04aac0),
    (0x04AAC4, i_04aac4),
    (0x04AACA, i_04aaca),
    (0x04AAD0, i_04aad0),
    (0x04AAD2, i_04aad2),
    (0x04AAD6, i_04aad6),
    (0x04AADC, i_04aadc),
    (0x04AAE0, i_04aae0),
    (0x04AAE4, i_04aae4),
    (0x04AAEA, i_04aaea),
    (0x04AAF0, i_04aaf0),
    (0x04AAF4, i_04aaf4),
    (0x04AAFA, i_04aafa),
    (0x04AAFE, i_04aafe),
    (0x04AB04, i_04ab04),
    (0x04AB06, i_04ab06),
    (0x04AB0A, i_04ab0a),
    (0x04AB0E, i_04ab0e),
    (0x04AB10, i_04ab10),
    (0x04AB14, i_04ab14),
    (0x04AB1A, i_04ab1a),
    (0x04AB1E, i_04ab1e),
    (0x04AB22, i_04ab22),
    (0x04AB28, i_04ab28),
    (0x04AB2C, i_04ab2c),
    (0x04AB32, i_04ab32),
    (0x04AB38, i_04ab38),
    (0x04AB3E, i_04ab3e),
    (0x04AB42, i_04ab42),
    (0x04AB48, i_04ab48),
    (0x04AB4C, i_04ab4c),
    (0x04AB4E, i_04ab4e),
    (0x04AB52, i_04ab52),
    (0x04AB56, i_04ab56),
    (0x04AB5C, i_04ab5c),
    (0x04AB60, i_04ab60),
    (0x04AB64, i_04ab64),
    (0x04AB6A, i_04ab6a),
    (0x04AB6E, i_04ab6e),
    (0x04AB74, i_04ab74),
    (0x04AB7A, i_04ab7a),
    (0x04AB80, i_04ab80),
    (0x04AB84, i_04ab84),
    (0x04AB8A, i_04ab8a),
    (0x04AB8E, i_04ab8e),
    (0x04AB90, i_04ab90),
    (0x04AB96, i_04ab96),
    (0x04AB9C, i_04ab9c),
    (0x04ABA0, i_04aba0),
    (0x04ABA6, i_04aba6),
    (0x04ABA8, i_04aba8),
    (0x04ABAC, i_04abac),
    (0x04ABB0, i_04abb0),
    (0x04ABB2, i_04abb2),
    (0x04ABB8, i_04abb8),
    (0x04ABBE, i_04abbe),
    (0x04ABC0, i_04abc0),
    (0x04ABC4, i_04abc4),
    (0x04ABCA, i_04abca),
    (0x04ABD0, i_04abd0),
    (0x04ABD6, i_04abd6),
    (0x04ABDC, i_04abdc),
    (0x04ABE0, i_04abe0),
    (0x04ABE6, i_04abe6),
    (0x04ABEA, i_04abea),
    (0x04ABEE, i_04abee),
    (0x04ABF4, i_04abf4),
    (0x04ABF8, i_04abf8),
    (0x04ABFE, i_04abfe),
    (0x04AC02, i_04ac02),
    (0x04AC04, i_04ac04),
    (0x04AC0A, i_04ac0a),
    (0x04AC10, i_04ac10),
    (0x04AC14, i_04ac14),
    (0x04AC18, i_04ac18),
    (0x04AC1A, i_04ac1a),
    (0x04AC20, i_04ac20),
    (0x04AC26, i_04ac26),
    (0x04AC2A, i_04ac2a),
    (0x04AC2E, i_04ac2e),
    (0x04AC30, i_04ac30),
    (0x04AC32, i_04ac32),
    (0x04AC38, i_04ac38),
    (0x04AC3A, i_04ac3a),
    (0x04AC3E, i_04ac3e),
    (0x04AC44, i_04ac44),
    (0x04AC48, i_04ac48),
    (0x04AC4E, i_04ac4e),
    (0x04AC52, i_04ac52),
    (0x04AC54, i_04ac54),
    (0x04B052, i_04b052),
    (0x04B056, i_04b056),
    (0x04B05A, i_04b05a),
    (0x04B060, i_04b060),
    (0x04B064, i_04b064),
    (0x04B06A, i_04b06a),
    (0x04B070, i_04b070),
    (0x04B076, i_04b076),
    (0x04BB8E, i_04bb8e),
    (0x04BB92, i_04bb92),
    (0x04BB94, i_04bb94),
    (0x04BB98, i_04bb98),
    (0x04BB9A, i_04bb9a),
    (0x04BBA0, i_04bba0),
    (0x04BBA4, i_04bba4),
    (0x04BBAA, i_04bbaa),
    (0x04BBAE, i_04bbae),
    (0x04BBB4, i_04bbb4),
    (0x04BBB8, i_04bbb8),
    (0x04BBBE, i_04bbbe),
    (0x04BBC2, i_04bbc2),
    (0x04BBC8, i_04bbc8),
    (0x04BBCE, i_04bbce),
    (0x04BBD4, i_04bbd4),
    (0x04BBD8, i_04bbd8),
    (0x04BBDE, i_04bbde),
    (0x04BBE4, i_04bbe4),
    (0x04BBEA, i_04bbea),
    (0x04BBEC, i_04bbec),
    (0x04BBF0, i_04bbf0),
    (0x04BBF4, i_04bbf4),
    (0x04BBFA, i_04bbfa),
    (0x04BC00, i_04bc00),
    (0x04BC04, i_04bc04),
    (0x04BC0A, i_04bc0a),
    (0x04BC10, i_04bc10),
    (0x04BC16, i_04bc16),
    (0x04BC1A, i_04bc1a),
    (0x04BC1E, i_04bc1e),
    (0x04BC20, i_04bc20),
    (0x04BC24, i_04bc24),
    (0x04BC2A, i_04bc2a),
    (0x04BC2E, i_04bc2e),
    (0x04BC34, i_04bc34),
    (0x04BC3A, i_04bc3a),
    (0x04BC40, i_04bc40),
    (0x04BC44, i_04bc44),
    (0x04BC46, i_04bc46),
    (0x04BC48, i_04bc48),
    (0x04BC4E, i_04bc4e),
    (0x04BC52, i_04bc52),
    (0x04BC58, i_04bc58),
    (0x04BC5C, i_04bc5c),
    (0x04BC60, i_04bc60),
    (0x04BC66, i_04bc66),
    (0x04BC6C, i_04bc6c),
    (0x04BC72, i_04bc72),
    (0x04BC76, i_04bc76),
    (0x04BC7C, i_04bc7c),
    (0x04BC80, i_04bc80),
    (0x04BC84, i_04bc84),
    (0x04BC8A, i_04bc8a),
    (0x04BC90, i_04bc90),
    (0x04BC96, i_04bc96),
    (0x04BC9C, i_04bc9c),
    (0x04BCA0, i_04bca0),
    (0x04BCA4, i_04bca4),
    (0x04BCA8, i_04bca8),
    (0x04BCAC, i_04bcac),
    (0x04BCB2, i_04bcb2),
    (0x04BCB8, i_04bcb8),
    (0x04BCBE, i_04bcbe),
    (0x04BCC2, i_04bcc2),
    (0x04BCC6, i_04bcc6),
    (0x04BCC8, i_04bcc8),
    (0x04BCCA, i_04bcca),
    (0x04BCD0, i_04bcd0),
    (0x04BCD6, i_04bcd6),
    (0x04BCDA, i_04bcda),
    (0x04BCDE, i_04bcde),
    (0x04BCE2, i_04bce2),
    (0x04BCE6, i_04bce6),
    (0x04BCEC, i_04bcec),
    (0x04BCF2, i_04bcf2),
    (0x04BCF8, i_04bcf8),
    (0x04BCFC, i_04bcfc),
    (0x04BCFE, i_04bcfe),
    (0x04BD02, i_04bd02),
    (0x04BD06, i_04bd06),
    (0x04BD0A, i_04bd0a),
    (0x04BD0C, i_04bd0c),
    (0x04BD12, i_04bd12),
    (0x04BD18, i_04bd18),
    (0x04BD1C, i_04bd1c),
    (0x04BD22, i_04bd22),
    (0x04BD26, i_04bd26),
    (0x04BD2A, i_04bd2a),
    (0x04BD30, i_04bd30),
    (0x04BD34, i_04bd34),
    (0x04BD36, i_04bd36),
    (0x04BD3C, i_04bd3c),
    (0x04BD40, i_04bd40),
    (0x04BD46, i_04bd46),
    (0x04BD4C, i_04bd4c),
    (0x04BD52, i_04bd52),
    (0x04BD56, i_04bd56),
    (0x04BD5A, i_04bd5a),
    (0x04BD5C, i_04bd5c),
    (0x04BD5E, i_04bd5e),
    (0x04BD62, i_04bd62),
    (0x04BD66, i_04bd66),
    (0x04BD68, i_04bd68),
    (0x04BD6C, i_04bd6c),
    (0x04BD70, i_04bd70),
    (0x04BD76, i_04bd76),
    (0x04BD7C, i_04bd7c),
    (0x04BD80, i_04bd80),
    (0x04BD82, i_04bd82),
    (0x04BD86, i_04bd86),
    (0x04BD8A, i_04bd8a),
    (0x04BD8E, i_04bd8e),
    (0x04BD90, i_04bd90),
    (0x04BD96, i_04bd96),
    (0x04BD9C, i_04bd9c),
    (0x04BDA0, i_04bda0),
    (0x04BDA6, i_04bda6),
    (0x04BDAA, i_04bdaa),
    (0x04BDAE, i_04bdae),
    (0x04BDB2, i_04bdb2),
    (0x04BDB6, i_04bdb6),
    (0x04BDBC, i_04bdbc),
    (0x04BDC0, i_04bdc0),
    (0x04BDC6, i_04bdc6),
    (0x04BDCA, i_04bdca),
    (0x04BDCC, i_04bdcc),
    (0x04BDD0, i_04bdd0),
    (0x04BDD6, i_04bdd6),
    (0x04BDDA, i_04bdda),
    (0x04BDDC, i_04bddc),
    (0x04BDE2, i_04bde2),
    (0x04BDE6, i_04bde6),
    (0x04BDEC, i_04bdec),
    (0x04BDF2, i_04bdf2),
    (0x04BDF8, i_04bdf8),
    (0x04BDFC, i_04bdfc),
    (0x04BE00, i_04be00),
    (0x04BE02, i_04be02),
    (0x04BE04, i_04be04),
    (0x04BE0A, i_04be0a),
    (0x04BE0E, i_04be0e),
    (0x04BE10, i_04be10),
    (0x04BE16, i_04be16),
    (0x04BE1C, i_04be1c),
    (0x04BE20, i_04be20),
    (0x04BE26, i_04be26),
    (0x04BE2A, i_04be2a),
    (0x04BE2E, i_04be2e),
    (0x04BE32, i_04be32),
    (0x04BE36, i_04be36),
    (0x04BE3A, i_04be3a),
    (0x04BE3E, i_04be3e),
    (0x04BE40, i_04be40),
    (0x04BE44, i_04be44),
    (0x04BE48, i_04be48),
    (0x04BE4A, i_04be4a),
    (0x04BE50, i_04be50),
    (0x04BE54, i_04be54),
    (0x04BE5A, i_04be5a),
    (0x04BE60, i_04be60),
    (0x04BE66, i_04be66),
    (0x04BE6A, i_04be6a),
    (0x04BE6E, i_04be6e),
    (0x04BE70, i_04be70),
    (0x04BE72, i_04be72),
    (0x04BE78, i_04be78),
    (0x04BE7C, i_04be7c),
    (0x04BE7E, i_04be7e),
    (0x04BE84, i_04be84),
    (0x04BE8A, i_04be8a),
    (0x04BE8E, i_04be8e),
    (0x04BE94, i_04be94),
    (0x04BE98, i_04be98),
    (0x04BE9C, i_04be9c),
    (0x04BEA0, i_04bea0),
    (0x04BEA4, i_04bea4),
    (0x04BEA8, i_04bea8),
    (0x04BEAC, i_04beac),
    (0x04BEAE, i_04beae),
    (0x04BEB2, i_04beb2),
    (0x04BEB6, i_04beb6),
    (0x04BEB8, i_04beb8),
    (0x04BEBE, i_04bebe),
    (0x04BEC2, i_04bec2),
    (0x04BEC8, i_04bec8),
    (0x04BECE, i_04bece),
    (0x04BED4, i_04bed4),
    (0x04BED8, i_04bed8),
    (0x04BEDC, i_04bedc),
    (0x04BEDE, i_04bede),
    (0x04BEE0, i_04bee0),
    (0x04BEE4, i_04bee4),
    (0x04BEEA, i_04beea),
    (0x04BEF0, i_04bef0),
    (0x04BEF6, i_04bef6),
    (0x04BEFA, i_04befa),
    (0x04BF00, i_04bf00),
    (0x04BF06, i_04bf06),
    (0x04BF0C, i_04bf0c),
    (0x04BF0E, i_04bf0e),
    (0x04BF12, i_04bf12),
    (0x04BF16, i_04bf16),
    (0x04BF1C, i_04bf1c),
    (0x04BF20, i_04bf20),
    (0x04BF22, i_04bf22),
    (0x04BF26, i_04bf26),
    (0x04BF2C, i_04bf2c),
    (0x04BF30, i_04bf30),
    (0x04BF34, i_04bf34),
    (0x04BF36, i_04bf36),
    (0x04BF3A, i_04bf3a),
    (0x04BF40, i_04bf40),
    (0x04BF46, i_04bf46),
    (0x04BF4C, i_04bf4c),
    (0x04BF50, i_04bf50),
    (0x04BF54, i_04bf54),
    (0x04BF56, i_04bf56),
    (0x04BF58, i_04bf58),
    (0x04BF5C, i_04bf5c),
    (0x04BF60, i_04bf60),
    (0x04BF66, i_04bf66),
    (0x04BF68, i_04bf68),
    (0x04BF6C, i_04bf6c),
    (0x04BF72, i_04bf72),
    (0x04BF76, i_04bf76),
    (0x04BF7C, i_04bf7c),
    (0x04BF80, i_04bf80),
    (0x04BF82, i_04bf82),
    (0x04BF86, i_04bf86),
    (0x04BF8C, i_04bf8c),
    (0x04BF92, i_04bf92),
    (0x04BF96, i_04bf96),
    (0x04BF9A, i_04bf9a),
    (0x04BFA0, i_04bfa0),
    (0x04BFA4, i_04bfa4),
    (0x04BFAA, i_04bfaa),
    (0x04BFAE, i_04bfae),
    (0x04BFB4, i_04bfb4),
    (0x04BFBA, i_04bfba),
    (0x04BFBE, i_04bfbe),
    (0x04BFC4, i_04bfc4),
    (0x04BFC8, i_04bfc8),
    (0x04BFCC, i_04bfcc),
    (0x04BFCE, i_04bfce),
    (0x04BFD2, i_04bfd2),
    (0x04BFD8, i_04bfd8),
    (0x04BFDE, i_04bfde),
    (0x04BFE4, i_04bfe4),
    (0x04BFE8, i_04bfe8),
    (0x04BFEC, i_04bfec),
    (0x04BFEE, i_04bfee),
    (0x04BFF0, i_04bff0),
    (0x04BFF4, i_04bff4),
    (0x04BFF8, i_04bff8),
    (0x04BFFE, i_04bffe),
    (0x04C000, i_04c000),
    (0x04C004, i_04c004),
    (0x04C00A, i_04c00a),
    (0x04C00E, i_04c00e),
    (0x04C014, i_04c014),
    (0x04C018, i_04c018),
    (0x04C01A, i_04c01a),
    (0x04C01E, i_04c01e),
    (0x04C024, i_04c024),
    (0x04C02A, i_04c02a),
    (0x04C02E, i_04c02e),
    (0x04C032, i_04c032),
    (0x04C038, i_04c038),
    (0x04C03C, i_04c03c),
    (0x04C042, i_04c042),
    (0x04C046, i_04c046),
    (0x04C04C, i_04c04c),
    (0x04C052, i_04c052),
    (0x04C056, i_04c056),
    (0x04C05C, i_04c05c),
    (0x04C060, i_04c060),
    (0x04C064, i_04c064),
    (0x04C066, i_04c066),
    (0x04C06A, i_04c06a),
    (0x04C070, i_04c070),
    (0x04C076, i_04c076),
    (0x04C07C, i_04c07c),
    (0x04C080, i_04c080),
    (0x04C084, i_04c084),
    (0x04C086, i_04c086),
    (0x04C088, i_04c088),
    (0x04C08C, i_04c08c),
    (0x04C092, i_04c092),
    (0x04C098, i_04c098),
    (0x04C09C, i_04c09c),
    (0x04C0A0, i_04c0a0),
    (0x04C0A6, i_04c0a6),
    (0x04C0AA, i_04c0aa),
    (0x04C0AE, i_04c0ae),
    (0x04C0B4, i_04c0b4),
    (0x04C0B6, i_04c0b6),
    (0x04C0BA, i_04c0ba),
    (0x04C0C0, i_04c0c0),
    (0x04C0C4, i_04c0c4),
    (0x04C0CA, i_04c0ca),
    (0x04C0CE, i_04c0ce),
    (0x04C0D4, i_04c0d4),
    (0x04C0D8, i_04c0d8),
    (0x04C0DE, i_04c0de),
    (0x04C0E4, i_04c0e4),
    (0x04C0E8, i_04c0e8),
    (0x04C0EC, i_04c0ec),
    (0x04C0F0, i_04c0f0),
    (0x04C0F4, i_04c0f4),
    (0x04C0F6, i_04c0f6),
    (0x04C0FC, i_04c0fc),
    (0x04C100, i_04c100),
    (0x04C104, i_04c104),
    (0x04C106, i_04c106),
    (0x04C10A, i_04c10a),
    (0x04C110, i_04c110),
    (0x04C116, i_04c116),
    (0x04C11C, i_04c11c),
    (0x04C120, i_04c120),
    (0x04C124, i_04c124),
    (0x04C126, i_04c126),
    (0x04C128, i_04c128),
    (0x04C12C, i_04c12c),
    (0x04C130, i_04c130),
    (0x04C136, i_04c136),
    (0x04C138, i_04c138),
    (0x04C13C, i_04c13c),
    (0x04C142, i_04c142),
    (0x04C146, i_04c146),
    (0x04C14C, i_04c14c),
    (0x04C150, i_04c150),
    (0x04C152, i_04c152),
    (0x04C156, i_04c156),
    (0x04C15C, i_04c15c),
    (0x04C162, i_04c162),
    (0x04C166, i_04c166),
    (0x04C16A, i_04c16a),
    (0x04C16E, i_04c16e),
    (0x04C174, i_04c174),
    (0x04C178, i_04c178),
    (0x04C17E, i_04c17e),
    (0x04C184, i_04c184),
    (0x04C188, i_04c188),
    (0x04C18C, i_04c18c),
    (0x04C18E, i_04c18e),
    (0x04C192, i_04c192),
    (0x04C198, i_04c198),
    (0x04C19E, i_04c19e),
    (0x04C1A4, i_04c1a4),
    (0x04C1A8, i_04c1a8),
    (0x04C1AC, i_04c1ac),
    (0x04C1AE, i_04c1ae),
    (0x04C1B0, i_04c1b0),
    (0x04C1B4, i_04c1b4),
    (0x04C1BA, i_04c1ba),
    (0x04C1C0, i_04c1c0),
    (0x04C1C6, i_04c1c6),
    (0x04C1CC, i_04c1cc),
    (0x04C1D0, i_04c1d0),
    (0x04C1D6, i_04c1d6),
    (0x04C1DC, i_04c1dc),
    (0x04C1E2, i_04c1e2),
    (0x04C1E4, i_04c1e4),
    (0x04C1E8, i_04c1e8),
    (0x04C1EC, i_04c1ec),
    (0x04C1F2, i_04c1f2),
    (0x04C1F8, i_04c1f8),
    (0x04C1FC, i_04c1fc),
    (0x04C202, i_04c202),
    (0x04C206, i_04c206),
    (0x04C208, i_04c208),
    (0x04C20C, i_04c20c),
    (0x04C212, i_04c212),
    (0x04C218, i_04c218),
    (0x04C21E, i_04c21e),
    (0x04C222, i_04c222),
    (0x04C226, i_04c226),
    (0x04C22C, i_04c22c),
    (0x04C230, i_04c230),
    (0x04C236, i_04c236),
    (0x04C23C, i_04c23c),
    (0x04C240, i_04c240),
    (0x04C246, i_04c246),
    (0x04C24A, i_04c24a),
    (0x04C24E, i_04c24e),
    (0x04C250, i_04c250),
    (0x04C254, i_04c254),
    (0x04C258, i_04c258),
    (0x04C25E, i_04c25e),
    (0x04C264, i_04c264),
    (0x04C26A, i_04c26a),
    (0x04C26E, i_04c26e),
    (0x04C272, i_04c272),
    (0x04C274, i_04c274),
    (0x04C276, i_04c276),
    (0x04C27A, i_04c27a),
    (0x04C280, i_04c280),
    (0x04C286, i_04c286),
    (0x04C28A, i_04c28a),
    (0x04C290, i_04c290),
    (0x04C294, i_04c294),
    (0x04C29A, i_04c29a),
    (0x04C2A0, i_04c2a0),
    (0x04C2A4, i_04c2a4),
    (0x04C2AA, i_04c2aa),
    (0x04C2AE, i_04c2ae),
    (0x04C2B4, i_04c2b4),
    (0x04C2B8, i_04c2b8),
    (0x04C2BC, i_04c2bc),
    (0x04C2BE, i_04c2be),
    (0x04C2C2, i_04c2c2),
    (0x04C2C6, i_04c2c6),
    (0x04C2CC, i_04c2cc),
    (0x04C2D2, i_04c2d2),
    (0x04C2D8, i_04c2d8),
    (0x04C2DC, i_04c2dc),
    (0x04C2E0, i_04c2e0),
    (0x04C2E2, i_04c2e2),
    (0x04C2E4, i_04c2e4),
    (0x04C2E8, i_04c2e8),
    (0x04C2EC, i_04c2ec),
    (0x04C2F2, i_04c2f2),
    (0x04C2F6, i_04c2f6),
    (0x04C2FC, i_04c2fc),
    (0x04C302, i_04c302),
    (0x04C308, i_04c308),
    (0x04C30E, i_04c30e),
    (0x04C314, i_04c314),
    (0x04C31A, i_04c31a),
    (0x04C31E, i_04c31e),
    (0x04C322, i_04c322),
    (0x04C328, i_04c328),
    (0x04C32E, i_04c32e),
    (0x04C334, i_04c334),
    (0x04C33A, i_04c33a),
    (0x04C340, i_04c340),
    (0x04C344, i_04c344),
    (0x04C346, i_04c346),
    (0x04C34C, i_04c34c),
    (0x04C352, i_04c352),
    (0x04C356, i_04c356),
    (0x04C35A, i_04c35a),
    (0x04C35E, i_04c35e),
    (0x04C362, i_04c362),
    (0x04C364, i_04c364),
    (0x04C368, i_04c368),
    (0x04C36E, i_04c36e),
    (0x04C374, i_04c374),
    (0x04C378, i_04c378),
    (0x04C37A, i_04c37a),
    (0x04C380, i_04c380),
    (0x04C386, i_04c386),
    (0x04C38A, i_04c38a),
    (0x04C38E, i_04c38e),
    (0x04C392, i_04c392),
    (0x04C396, i_04c396),
    (0x04C39A, i_04c39a),
    (0x04C39C, i_04c39c),
    (0x04C3A0, i_04c3a0),
    (0x04C3A6, i_04c3a6),
    (0x04C3AA, i_04c3aa),
    (0x04C3AE, i_04c3ae),
    (0x04C3B0, i_04c3b0),
    (0x04C3B4, i_04c3b4),
    (0x04C3BA, i_04c3ba),
    (0x04C3C0, i_04c3c0),
    (0x04C3C4, i_04c3c4),
    (0x04C3C6, i_04c3c6),
    (0x04C3CC, i_04c3cc),
    (0x04C3D2, i_04c3d2),
    (0x04C3D6, i_04c3d6),
    (0x04C3DC, i_04c3dc),
    (0x04C3E0, i_04c3e0),
    (0x04C3E4, i_04c3e4),
    (0x04C3E6, i_04c3e6),
    (0x04C3EA, i_04c3ea),
    (0x04C3F0, i_04c3f0),
    (0x04C3F6, i_04c3f6),
    (0x04C3FA, i_04c3fa),
    (0x04C3FC, i_04c3fc),
    (0x04C402, i_04c402),
    (0x04C408, i_04c408),
    (0x04C40C, i_04c40c),
    (0x04C410, i_04c410),
    (0x04C412, i_04c412),
    (0x04C416, i_04c416),
    (0x04C41C, i_04c41c),
    (0x04C420, i_04c420),
    (0x04C426, i_04c426),
    (0x04C428, i_04c428),
    (0x04C42C, i_04c42c),
    (0x04C432, i_04c432),
    (0x04C438, i_04c438),
    (0x04C43E, i_04c43e),
    (0x04C442, i_04c442),
    (0x04C446, i_04c446),
    (0x04C448, i_04c448),
    (0x04C44A, i_04c44a),
    (0x04C44E, i_04c44e),
    (0x04C452, i_04c452),
    (0x04C458, i_04c458),
    (0x04C45C, i_04c45c),
    (0x04C462, i_04c462),
    (0x04C468, i_04c468),
    (0x04C46E, i_04c46e),
    (0x04C474, i_04c474),
    (0x04C478, i_04c478),
    (0x04C47C, i_04c47c),
    (0x04C482, i_04c482),
    (0x04C488, i_04c488),
    (0x04C48E, i_04c48e),
    (0x04C494, i_04c494),
    (0x04C49A, i_04c49a),
    (0x04C49E, i_04c49e),
    (0x04C4A0, i_04c4a0),
    (0x04C4A6, i_04c4a6),
    (0x04C4AC, i_04c4ac),
    (0x04C4B0, i_04c4b0),
    (0x04C4B4, i_04c4b4),
    (0x04C4B8, i_04c4b8),
    (0x04C4BC, i_04c4bc),
    (0x04C4BE, i_04c4be),
    (0x04C4C2, i_04c4c2),
    (0x04C4C8, i_04c4c8),
    (0x04C4CE, i_04c4ce),
    (0x04C4D2, i_04c4d2),
    (0x04C4D4, i_04c4d4),
    (0x04C4DA, i_04c4da),
    (0x04C4E0, i_04c4e0),
    (0x04C4E4, i_04c4e4),
    (0x04C4E8, i_04c4e8),
    (0x04C4EC, i_04c4ec),
    (0x04C4F0, i_04c4f0),
    (0x04C4F4, i_04c4f4),
    (0x04C4F6, i_04c4f6),
    (0x04C4FA, i_04c4fa),
    (0x04C500, i_04c500),
    (0x04C504, i_04c504),
    (0x04C508, i_04c508),
    (0x04C50A, i_04c50a),
    (0x04C50E, i_04c50e),
    (0x04C514, i_04c514),
    (0x04C51A, i_04c51a),
    (0x04C51E, i_04c51e),
    (0x04C520, i_04c520),
    (0x04C526, i_04c526),
    (0x04C52C, i_04c52c),
    (0x04C530, i_04c530),
    (0x04C536, i_04c536),
    (0x04C53A, i_04c53a),
    (0x04C53E, i_04c53e),
    (0x04C540, i_04c540),
    (0x04C544, i_04c544),
    (0x04C54A, i_04c54a),
    (0x04C54E, i_04c54e),
    (0x04C554, i_04c554),
    (0x04C556, i_04c556),
    (0x04C55A, i_04c55a),
    (0x04C560, i_04c560),
    (0x04C566, i_04c566),
    (0x04C56C, i_04c56c),
    (0x04C570, i_04c570),
    (0x04C574, i_04c574),
    (0x04C576, i_04c576),
    (0x04C578, i_04c578),
    (0x04C57E, i_04c57e),
    (0x04C584, i_04c584),
    (0x04C58A, i_04c58a),
    (0x04C58C, i_04c58c),
    (0x04C592, i_04c592),
    (0x04C596, i_04c596),
    (0x04C59C, i_04c59c),
    (0x04C5A2, i_04c5a2),
    (0x04C5A4, i_04c5a4),
    (0x04C5A8, i_04c5a8),
    (0x04C5AC, i_04c5ac),
    (0x04C5B0, i_04c5b0),
    (0x04C5B6, i_04c5b6),
    (0x04C5BC, i_04c5bc),
    (0x04C5C2, i_04c5c2),
    (0x04C5C8, i_04c5c8),
    (0x04C5CE, i_04c5ce),
    (0x04C5D4, i_04c5d4),
    (0x04C5D8, i_04c5d8),
    (0x04C5DA, i_04c5da),
    (0x04C5E0, i_04c5e0),
    (0x04C5E6, i_04c5e6),
    (0x04C5EA, i_04c5ea),
    (0x04C5EE, i_04c5ee),
    (0x04C5F4, i_04c5f4),
    (0x04C5F8, i_04c5f8),
    (0x04C5FE, i_04c5fe),
    (0x04C602, i_04c602),
    (0x04C604, i_04c604),
    (0x04C606, i_04c606),
    (0x04C60A, i_04c60a),
    (0x04C60E, i_04c60e),
    (0x04C614, i_04c614),
    (0x04C61A, i_04c61a),
    (0x04C61E, i_04c61e),
    (0x04C620, i_04c620),
    (0x04C622, i_04c622),
    (0x04C626, i_04c626),
    (0x04C62C, i_04c62c),
    (0x04C630, i_04c630),
    (0x04C636, i_04c636),
    (0x04C63A, i_04c63a),
    (0x04C640, i_04c640),
    (0x04C646, i_04c646),
    (0x04C64A, i_04c64a),
    (0x04C64C, i_04c64c),
    (0x04C650, i_04c650),
    (0x04C654, i_04c654),
    (0x04C656, i_04c656),
    (0x04C65C, i_04c65c),
    (0x04C662, i_04c662),
    (0x04C666, i_04c666),
    (0x04C66A, i_04c66a),
    (0x04C66C, i_04c66c),
    (0x04C672, i_04c672),
    (0x04C678, i_04c678),
    (0x04C67E, i_04c67e),
    (0x04C682, i_04c682),
    (0x04C686, i_04c686),
    (0x04C688, i_04c688),
    (0x04C68A, i_04c68a),
    (0x04C68E, i_04c68e),
    (0x04C692, i_04c692),
    (0x04C698, i_04c698),
    (0x04C69E, i_04c69e),
    (0x04C6A4, i_04c6a4),
    (0x04C6A8, i_04c6a8),
    (0x04C6AA, i_04c6aa),
    (0x04C6B0, i_04c6b0),
    (0x04C6B6, i_04c6b6),
    (0x04C6BC, i_04c6bc),
    (0x04C6C2, i_04c6c2),
    (0x04C6C8, i_04c6c8),
    (0x04C6CC, i_04c6cc),
    (0x04C6D0, i_04c6d0),
    (0x04C6D2, i_04c6d2),
    (0x04C6D4, i_04c6d4),
    (0x04C6DA, i_04c6da),
    (0x04C6DE, i_04c6de),
    (0x04C6E2, i_04c6e2),
    (0x04C6E6, i_04c6e6),
    (0x04C6E8, i_04c6e8),
    (0x04C6EA, i_04c6ea),
    (0x04C6EE, i_04c6ee),
    (0x04C6F4, i_04c6f4),
    (0x04C6F8, i_04c6f8),
    (0x04C6FE, i_04c6fe),
    (0x04C704, i_04c704),
    (0x04C70A, i_04c70a),
    (0x04C70E, i_04c70e),
    (0x04C714, i_04c714),
    (0x04C718, i_04c718),
    (0x04C71E, i_04c71e),
    (0x04C722, i_04c722),
    (0x04C728, i_04c728),
    (0x04C72E, i_04c72e),
    (0x04C732, i_04c732),
    (0x04C738, i_04c738),
    (0x04C73E, i_04c73e),
    (0x04C742, i_04c742),
    (0x04C748, i_04c748),
    (0x04C74E, i_04c74e),
    (0x04C754, i_04c754),
    (0x04C75A, i_04c75a),
    (0x04C760, i_04c760),
    (0x04C764, i_04c764),
    (0x04C768, i_04c768),
    (0x04C76A, i_04c76a),
    (0x04C76E, i_04c76e),
    (0x04C770, i_04c770),
    (0x04C774, i_04c774),
    (0x04C776, i_04c776),
    (0x04C77A, i_04c77a),
    (0x04C780, i_04c780),
    (0x04C786, i_04c786),
    (0x04C78C, i_04c78c),
    (0x04C790, i_04c790),
    (0x04C794, i_04c794),
    (0x04C796, i_04c796),
    (0x04C79C, i_04c79c),
    (0x04C7A2, i_04c7a2),
    (0x04C7A8, i_04c7a8),
    (0x04C7B0, i_04c7b0),
    (0x04C7B6, i_04c7b6),
    (0x04C7BC, i_04c7bc),
    (0x04C7C0, i_04c7c0),
    (0x04C7C2, i_04c7c2),
    (0x04C7C6, i_04c7c6),
    (0x04C7CC, i_04c7cc),
    (0x04C7D0, i_04c7d0),
    (0x04C7D4, i_04c7d4),
    (0x04C7D6, i_04c7d6),
    (0x04C7DC, i_04c7dc),
    (0x04C7E0, i_04c7e0),
    (0x04C7E4, i_04c7e4),
    (0x04C7EA, i_04c7ea),
    (0x04C7EE, i_04c7ee),
    (0x04C7F2, i_04c7f2),
    (0x04C7F6, i_04c7f6),
    (0x04C7FA, i_04c7fa),
    (0x04C800, i_04c800),
    (0x04C806, i_04c806),
    (0x04C80C, i_04c80c),
    (0x04C810, i_04c810),
    (0x04C816, i_04c816),
    (0x04C81A, i_04c81a),
    (0x04C81E, i_04c81e),
    (0x04C820, i_04c820),
    (0x04C826, i_04c826),
    (0x04C82A, i_04c82a),
    (0x04C82E, i_04c82e),
    (0x04C830, i_04c830),
    (0x04C832, i_04c832),
    (0x04C838, i_04c838),
    (0x04C83E, i_04c83e),
    (0x04C842, i_04c842),
    (0x04C844, i_04c844),
    (0x04C846, i_04c846),
    (0x04C84C, i_04c84c),
    (0x04C852, i_04c852),
    (0x04C856, i_04c856),
    (0x04C85A, i_04c85a),
    (0x04C85E, i_04c85e),
    (0x04C862, i_04c862),
    (0x04C866, i_04c866),
    (0x04C86A, i_04c86a),
    (0x04C86E, i_04c86e),
    (0x04C874, i_04c874),
    (0x04C87A, i_04c87a),
];
