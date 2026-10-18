// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $05162C: move.b #$1, $106ed2.l
pub fn i_05162c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51634;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $051634: lea.l $98288.l, a1
pub fn i_051634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5163A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x98288;
    20
}
/// $05163A: jsr $4ae.l
pub fn i_05163a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51640);
    cpu.pc = 0x4AE;
    16
}
/// $051640: move.l #$5164a, $98(a0)
pub fn i_051640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51648;
    cpu.ir = 0x217C;
    let v1: u32 = 0x5164A;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051648: rts
pub fn i_051648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5164A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_LoadScene_05164a  ($05164A, 22 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05164A: lea.l $2970f8.l, a0
pub fn i_05164a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51650;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2970F8;
    20
}
/// $051650: jsr $43562.l
pub fn i_051650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51656);
    cpu.pc = 0x43562;
    16
}
/// $051656: moveq #$0, d0
pub fn i_051656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51658;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051658: moveq #$0, d1
pub fn i_051658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5165A;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05165A: jmp $437da.l
pub fn i_05165a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51660;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x437DA;
    8
}
// ===== MemCard_DetectAndSave_051660  ($051660, 46 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051660: jsr $9826e.l
pub fn i_051660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51666);
    cpu.pc = 0x9826E;
    16
}
/// $051666: bcs.w $51674
pub fn i_051666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5166A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x51674; return 10; }
    8
}
/// $05166A: clr.b $106ed2.l
pub fn i_05166a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51670;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $051670: bra.w $51694
pub fn i_051670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51674;
    cpu.ir = 0x6000;
    cpu.pc = 0x51694;
    10
}
/// $051674: move.b #$1, $106ed2.l
pub fn i_051674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5167C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $05167C: jsr $5164a(pc)
pub fn i_05167c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51680;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x51680);
    cpu.pc = 0x5164A;
    16
}
/// $051680: move.b #$ff, $10e3b6.l
pub fn i_051680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51688;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E3B6, v1 as u8);
    16
}
/// $051688: lea.l $9840c.l, a1
pub fn i_051688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5168E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x9840C;
    20
}
// ===== JsrAbsThunk_05168e  ($05168E, 8 B, thunks/jsr_abs_thunks.c)
/// $05168E: jsr $4ae.l
pub fn i_05168e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51694;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51694);
    cpu.pc = 0x4AE;
    16
}
/// $051694: rts
pub fn i_051694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51696;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_CopySlotFlag_051696  ($051696, 14 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051696: move.b $10e3b7.l, d0
pub fn i_051696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5169C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E3B7) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $05169C: move.b d0, $106ed0.l
pub fn i_05169c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516A2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED0, v1 as u8);
    16
}
/// $0516A2: rts
pub fn i_0516a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CounterSaturateByte_05170C  ($05170C, 64 B, player/counter_saturate_05170c.s)
/// $05170C: lea.l $10e3a2.l, a4
pub fn i_05170c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51712;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E3A2;
    20
}
/// $051712: cmpa.l #$106e94, a1
pub fn i_051712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51718;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x106E94;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $051718: bne.w $51726
pub fn i_051718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5171C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51726; return 10; }
    8
}
/// $05171C: adda.l #$18, a4
pub fn i_05171c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51722;
    cpu.ir = 0xD9FC;
    let v1: u32 = 0x18;
    cpu.a[4] = cpu.a[4].wrapping_add(v1);
    8
}
/// $051722: bra.w $5173c
pub fn i_051722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51726;
    cpu.ir = 0x6000;
    cpu.pc = 0x5173C;
    10
}
/// $051726: cmpa.l #$106e9c, a1
pub fn i_051726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5172C;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x106E9C;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05172C: bne.w $5173a
pub fn i_05172c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51730;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5173A; return 10; }
    8
}
/// $051730: adda.l #$19, a4
pub fn i_051730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51736;
    cpu.ir = 0xD9FC;
    let v1: u32 = 0x19;
    cpu.a[4] = cpu.a[4].wrapping_add(v1);
    8
}
/// $051736: bra.w $5173c
pub fn i_051736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5173A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5173C;
    10
}
/// $05173A: rts
pub fn i_05173a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5173C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05173C: move.b (a4), d0
pub fn i_05173c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5173E;
    cpu.ir = 0x1014;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05173E: addq.b #$1, d0
pub fn i_05173e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51740;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051740: bcc.w $51748
pub fn i_051740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51744;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51748; return 10; }
    8
}
/// $051744: move.b #$ff, d0
pub fn i_051744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51748;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051748: move.b d0, (a4)
pub fn i_051748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5174A;
    cpu.ir = 0x1880;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[4], v1 as u8);
    8
}
/// $05174A: rts
pub fn i_05174a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5174C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SetIndexFromParent_0517aa  ($0517AA, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0517AA: movea.l $c(a6), a0
pub fn i_0517aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517AE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0517AE: cmpa.l #$100300, a0
pub fn i_0517ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517B4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x100300;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0517B4: bne.w $517c6
pub fn i_0517b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x517C6; return 10; }
    8
}
/// $0517B8: move.b #$0, $68(a6)
pub fn i_0517b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517BE: move.b #$1, $6d(a6)
pub fn i_0517be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517C4: rts
pub fn i_0517c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0517C6: cmpa.l #$1003a0, a0
pub fn i_0517c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517CC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1003A0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0517CC: bne.w $517de
pub fn i_0517cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x517DE; return 10; }
    8
}
/// $0517D0: move.b #$1, $68(a6)
pub fn i_0517d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517D6: move.b #$2, $6d(a6)
pub fn i_0517d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517DC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517DC: rts
pub fn i_0517dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0517DE: move.b #$ff, $6d(a6)
pub fn i_0517de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517E4: move.b $68(a6), d0
pub fn i_0517e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517E8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0517E8: cmpi.b #$ff, d0
pub fn i_0517e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517EC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0517EC: beq.w $517fc
pub fn i_0517ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x517FC; return 10; }
    8
}
/// $0517F0: nop
pub fn i_0517f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517F2;
    cpu.ir = 0x4E71;
    4
}
/// $0517F2: nop
pub fn i_0517f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517F4;
    cpu.ir = 0x4E71;
    4
}
/// $0517F4: cmpi.b #$ff, d0
pub fn i_0517f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517F8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0517F8: nop
pub fn i_0517f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517FA;
    cpu.ir = 0x4E71;
    4
}
/// $0517FA: trap #$f
pub fn i_0517fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517FC;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0517FC: rts
pub fn i_0517fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyField68AndCall_0517FE  ($0517FE, 14 B, entity/copy_field68_call_0517fe.s)
/// $0517FE: move.b $68(a6), $68(a0)
pub fn i_0517fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51804;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $051804: jsr $5ccc8.l
pub fn i_051804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5180A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5180A);
    cpu.pc = 0x5CCC8;
    16
}
/// $05180A: rts
pub fn i_05180a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5180C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Clear8Bytes_05180c  ($05180C, 8 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05180C: clr.l (a1)
pub fn i_05180c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5180E;
    cpu.ir = 0x4291;
    let ad1 = cpu.a[1];
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $05180E: clr.l $4(a1)
pub fn i_05180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51812;
    cpu.ir = 0x42A9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051812: rts
pub fn i_051812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51814;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nibbles_Unpack4_051828  ($051828, 58 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051828: move.b (a1)+, d0
pub fn i_051828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5182A;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05182A: move.b d0, d1
pub fn i_05182a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5182C;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05182C: lsr.b #$4, d1
pub fn i_05182c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5182E;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $05182E: andi.b #$f, d0
pub fn i_05182e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51832;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $051832: move.b d1, (a0)+
pub fn i_051832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51834;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051834: move.b d0, (a0)+
pub fn i_051834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51836;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051836: move.b (a1)+, d0
pub fn i_051836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51838;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051838: move.b d0, d1
pub fn i_051838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5183A;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05183A: lsr.b #$4, d1
pub fn i_05183a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5183C;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $05183C: andi.b #$f, d0
pub fn i_05183c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51840;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $051840: move.b d1, (a0)+
pub fn i_051840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51842;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051842: move.b d0, (a0)+
pub fn i_051842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51844;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051844: move.b (a1)+, d0
pub fn i_051844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51846;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051846: move.b d0, d1
pub fn i_051846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51848;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051848: lsr.b #$4, d1
pub fn i_051848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5184A;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $05184A: andi.b #$f, d0
pub fn i_05184a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5184E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05184E: move.b d1, (a0)+
pub fn i_05184e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51850;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051850: move.b d0, (a0)+
pub fn i_051850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51852;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051852: move.b (a1)+, d0
pub fn i_051852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51854;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051854: move.b d0, d1
pub fn i_051854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51856;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051856: lsr.b #$4, d1
pub fn i_051856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51858;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $051858: andi.b #$f, d0
pub fn i_051858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5185C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05185C: move.b d1, (a0)+
pub fn i_05185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5185E;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $05185E: move.b d0, (a0)+
pub fn i_05185e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51860;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051860: rts
pub fn i_051860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51862;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nibbles_Pack8_051862  ($051862, 42 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051862: move.b (a0)+, d0
pub fn i_051862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51864;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051864: move.b (a0)+, d1
pub fn i_051864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51866;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051866: lsl.b #$4, d0
pub fn i_051866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51868;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $051868: or.b d1, d0
pub fn i_051868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5186A;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05186A: move.b d0, (a1)+
pub fn i_05186a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5186C;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $05186C: move.b (a0)+, d0
pub fn i_05186c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5186E;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05186E: move.b (a0)+, d1
pub fn i_05186e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51870;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051870: lsl.b #$4, d0
pub fn i_051870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51872;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $051872: or.b d1, d0
pub fn i_051872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51874;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $051874: move.b d0, (a1)+
pub fn i_051874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51876;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051876: move.b (a0)+, d0
pub fn i_051876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51878;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051878: move.b (a0)+, d1
pub fn i_051878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5187A;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05187A: lsl.b #$4, d0
pub fn i_05187a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5187C;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $05187C: or.b d1, d0
pub fn i_05187c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5187E;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05187E: move.b d0, (a1)+
pub fn i_05187e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51880;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051880: move.b (a0)+, d0
pub fn i_051880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51882;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051882: move.b (a0)+, d1
pub fn i_051882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51884;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051884: lsl.b #$4, d0
pub fn i_051884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51886;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $051886: or.b d1, d0
pub fn i_051886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51888;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $051888: move.b d0, (a1)+
pub fn i_051888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5188A;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $05188A: rts
pub fn i_05188a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5188C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_DispatchStateBySlot_051914  ($051914, 134 B, player/dispatch_cluster_051914.s)
/// $051914: cmpa.l #$100440, a1
pub fn i_051914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5191A;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05191A: beq.w $51998
pub fn i_05191a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5191E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $05191E: cmpa.l #$1004e0, a1
pub fn i_05191e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51924;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $051924: beq.w $51998
pub fn i_051924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51928;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $051928: cmpa.l #$100580, a1
pub fn i_051928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5192E;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x100580;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05192E: beq.w $51998
pub fn i_05192e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51932;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $051932: move.b $58(a1), d1
pub fn i_051932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51936;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $051936: beq.w $51998
pub fn i_051936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5193A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $05193A: move.b $68(a6), d0
pub fn i_05193a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5193E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05193E: bmi.w $51998
pub fn i_05193e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51942;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51998; return 10; }
    8
}
/// $051942: move.b d0, $6e(a1)
pub fn i_051942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51946;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x6E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051946: btst.b #$1, $12(a1)
pub fn i_051946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5194C;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05194C: beq.w $51998
pub fn i_05194c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51950;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $051950: movem.l a0-a2, -(a7)
pub fn i_051950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51954;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051954: tst.b d0
pub fn i_051954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51956;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $051956: bne.w $51964
pub fn i_051956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5195A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51964; return 10; }
    8
}
/// $05195A: lea.l $106e94.l, a0
pub fn i_05195a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51960;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E94;
    20
}
/// $051960: bra.w $5196a
pub fn i_051960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51964;
    cpu.ir = 0x6000;
    cpu.pc = 0x5196A;
    10
}
/// $051964: lea.l $106e9c.l, a0
pub fn i_051964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5196A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E9C;
    20
}
/// $05196A: lea.l $1081b6.l, a1
pub fn i_05196a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51970;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081B6;
    20
}
/// $051970: move.w d1, -(a7)
pub fn i_051970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51972;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $051972: bsr.w $51862
pub fn i_051972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51976;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51976);
    cpu.pc = 0x51862;
    18
}
/// $051976: move.w (a7)+, d1
pub fn i_051976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51978;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $051978: lea.l $5188c(pc), a2
pub fn i_051978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5197C;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5188C;
    16
}
/// $05197C: andi.w #$1f, d1
pub fn i_05197c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51980;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051980: add.w d1, d1
pub fn i_051980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51982;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051982: add.w d1, d1
pub fn i_051982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51984;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051984: addq.w #$4, d1
pub fn i_051984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51986;
    cpu.ir = 0x5841;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051986: adda.w d1, a2
pub fn i_051986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51988;
    cpu.ir = 0xD4C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051988: bsr.w $51a10
pub fn i_051988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5198C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5198C);
    cpu.pc = 0x51A10;
    18
}
/// $05198C: suba.w #$8, a0
pub fn i_05198c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51990;
    cpu.ir = 0x90FC;
    let v1: u32 = 0x8;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051990: bsr.w $51828
pub fn i_051990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51994;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51994);
    cpu.pc = 0x51828;
    18
}
/// $051994: movem.l (a7)+, a0-a2
pub fn i_051994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51998;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051998: rts
pub fn i_051998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5199A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BCD_AddClamp99999999_051A10  ($051A10, 24 B, kernel/math_bcd_add_clamp_051a10.s)
/// $051A10: andi.b #$ef, ccr
pub fn i_051a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A14;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xF);
    20
}
/// $051A14: abcd.b -(a2), -(a1)
pub fn i_051a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A16;
    cpu.ir = 0xC30A;
    cpu.a[2] = cpu.a[2].wrapping_sub(1);
    let s = bus.read8(cpu.a[2]);
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    let d = bus.read8(cpu.a[1]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[1], r);
    18
}
/// $051A16: abcd.b -(a2), -(a1)
pub fn i_051a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A18;
    cpu.ir = 0xC30A;
    cpu.a[2] = cpu.a[2].wrapping_sub(1);
    let s = bus.read8(cpu.a[2]);
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    let d = bus.read8(cpu.a[1]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[1], r);
    18
}
/// $051A18: abcd.b -(a2), -(a1)
pub fn i_051a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A1A;
    cpu.ir = 0xC30A;
    cpu.a[2] = cpu.a[2].wrapping_sub(1);
    let s = bus.read8(cpu.a[2]);
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    let d = bus.read8(cpu.a[1]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[1], r);
    18
}
/// $051A1A: abcd.b -(a2), -(a1)
pub fn i_051a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A1C;
    cpu.ir = 0xC30A;
    cpu.a[2] = cpu.a[2].wrapping_sub(1);
    let s = bus.read8(cpu.a[2]);
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    let d = bus.read8(cpu.a[1]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[1], r);
    18
}
/// $051A1C: bcc.w $51a26
pub fn i_051a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51A26; return 10; }
    8
}
/// $051A20: move.l #$99999999, (a1)
pub fn i_051a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A26;
    cpu.ir = 0x22BC;
    let v1: u32 = 0x99999999;
    bus.write32(cpu.a[1], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051A26: rts
pub fn i_051a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051A44: movem.l d0/a0-a2, -(a7)
pub fn i_051a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A48;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051A48: tst.b $6e(a6)
pub fn i_051a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A4C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $051A4C: bmi.w $51a80
pub fn i_051a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A50;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51A80; return 10; }
    8
}
/// $051A50: bne.w $51a5e
pub fn i_051a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51A5E; return 10; }
    8
}
/// $051A54: lea.l $106e94.l, a0
pub fn i_051a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A5A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E94;
    20
}
/// $051A5A: bra.w $51a64
pub fn i_051a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x51A64;
    10
}
/// $051A5E: lea.l $106e9c.l, a0
pub fn i_051a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E9C;
    20
}
/// $051A64: lea.l $1081b6.l, a1
pub fn i_051a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A6A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081B6;
    20
}
/// $051A6A: lea.l $1081ba.l, a2
pub fn i_051a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A70;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x1081BA;
    20
}
/// $051A70: move.l d0, (a2)
pub fn i_051a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A72;
    cpu.ir = 0x2480;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[2], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051A72: bsr.w $51862
pub fn i_051a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A76;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51A76);
    cpu.pc = 0x51862;
    18
}
/// $051A76: addq.w #$4, a2
pub fn i_051a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A78;
    cpu.ir = 0x584A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $051A78: bsr.b $51a10
pub fn i_051a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A7A;
    cpu.ir = 0x6196;
    cpu.push32(bus, 0x51A7A);
    cpu.pc = 0x51A10;
    18
}
/// $051A7A: subq.w #$8, a0
pub fn i_051a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A7C;
    cpu.ir = 0x5148;
    cpu.a[0] = cpu.a[0].wrapping_sub(8);
    8
}
/// $051A7C: bsr.w $51828
pub fn i_051a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A80;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51A80);
    cpu.pc = 0x51828;
    18
}
/// $051A80: movem.l (a7)+, d0/a0-a2
pub fn i_051a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A84;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051A84: rts
pub fn i_051a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_IncCounterAt7_051A86  ($051A86, 30 B, player/dispatch_cluster_051914.s)
/// $051A86: move.b $7(a1), d0
pub fn i_051a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A8A;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $051A8A: addq.b #$1, d0
pub fn i_051a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A8C;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051A8C: cmpi.b #$a, d0
pub fn i_051a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A90;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $051A90: bcs.w $51a98
pub fn i_051a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A94;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x51A98; return 10; }
    8
}
/// $051A94: move.b #$9, d0
pub fn i_051a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A98;
    cpu.ir = 0x103C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051A98: move.b d0, $7(a1)
pub fn i_051a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A9C;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051A9C: jsr $5170c.l
pub fn i_051a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51AA2);
    cpu.pc = 0x5170C;
    16
}
/// $051AA2: rts
pub fn i_051aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_ReadPacked32_051aa4  ($051AA4, 26 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051AA4: movem.l a0-a1, -(a7)
pub fn i_051aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AA8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051AA8: lea.l $1081b6.l, a1
pub fn i_051aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AAE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081B6;
    20
}
/// $051AAE: bsr.w $51862
pub fn i_051aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AB2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51AB2);
    cpu.pc = 0x51862;
    18
}
/// $051AB2: suba.w #$4, a1
pub fn i_051ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AB6;
    cpu.ir = 0x92FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051AB6: move.l (a1), d0
pub fn i_051ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AB8;
    cpu.ir = 0x2011;
    let v2 = bus.read32(cpu.a[1]);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051AB8: movem.l (a7)+, a0-a1
pub fn i_051ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ABC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051ABC: rts
pub fn i_051abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ABE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_AllocAndInit_051ABE  ($051ABE, 94 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051ABE: move.w d0, $28(a0)
pub fn i_051abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AC2;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051AC2: clr.l $e(a0)
pub fn i_051ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AC6;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0xE);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051AC6: move.w (a1), $16(a0)
pub fn i_051ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ACA;
    cpu.ir = 0x3151;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x16);
    bus.write16(ad3, v2 as u16);
    16
}
/// $051ACA: move.w $2(a1), $18(a0)
pub fn i_051aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AD0;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x18);
    bus.write16(ad4, v3 as u16);
    20
}
/// $051AD0: move.w $4(a1), $1a(a0)
pub fn i_051ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AD6;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x1A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $051AD6: move.w $6(a1), $1c(a0)
pub fn i_051ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ADC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x1C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $051ADC: clr.w $2a(a0)
pub fn i_051adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AE0;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AE0: clr.w $2c(a0)
pub fn i_051ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AE4;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AE4: clr.w $22(a0)
pub fn i_051ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AE8;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AE8: clr.w $24(a0)
pub fn i_051ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AEC;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AEC: clr.w $c(a0)
pub fn i_051aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AF0;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0xC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AF0: clr.l $4(a0)
pub fn i_051af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AF4;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051AF4: clr.l $8(a0)
pub fn i_051af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AF8;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051AF8: clr.w (a0)
pub fn i_051af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AFA;
    cpu.ir = 0x4250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $051AFA: clr.w $2(a0)
pub fn i_051afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AFE;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AFE: clr.w $1e(a0)
pub fn i_051afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B02;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051B02: clr.w $20(a0)
pub fn i_051b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B06;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051B06: bsr.w $51ed6
pub fn i_051b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B0A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51B0A);
    cpu.pc = 0x51ED6;
    18
}
/// $051B0A: bsr.w $51f02
pub fn i_051b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B0E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51B0E);
    cpu.pc = 0x51F02;
    18
}
/// $051B0E: lea.l $32(a0), a1
pub fn i_051b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B12;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x32);
    cpu.a[1] = ad1;
    16
}
/// $051B12: moveq #$1f, d7
pub fn i_051b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B14;
    cpu.ir = 0x7E1F;
    cpu.d[7] = 0x1F;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B14: clr.b (a1)+
pub fn i_051b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B16;
    cpu.ir = 0x4219;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(1);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    8
}
/// $051B16: dbra d7, $51b14
pub fn i_051b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B1A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51B1A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51B1A; return 14; }
    cpu.pc = 0x51B14;
    10
}
/// $051B1A: rts
pub fn i_051b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_SetCursorAndClear_051b1c  ($051B1C, 100 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051B1C: andi.b #$3f, d0
pub fn i_051b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B20;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $051B20: move.b d0, $27(a0)
pub fn i_051b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B24;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051B24: add.w $24(a0), d1
pub fn i_051b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B28;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051B28: neg.w d1
pub fn i_051b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B2A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051B2A: andi.w #$1f, d1
pub fn i_051b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B2E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051B2E: move.b d1, $26(a0)
pub fn i_051b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B32;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051B32: clr.l $12(a0)
pub fn i_051b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B36;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051B36: rts
pub fn i_051b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051B38: move.l a1, $12(a0)
pub fn i_051b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B3C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x12);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B3C: rts
pub fn i_051b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051B3E: move.l a1, $e(a0)
pub fn i_051b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B42;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B42: move.w $4(a0), d2
pub fn i_051b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B46;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051B46: sub.w d0, d2
pub fn i_051b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B48;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051B48: lsr.w #$4, d2
pub fn i_051b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B4A;
    cpu.ir = 0xE84A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051B4A: add.w $22(a0), d2
pub fn i_051b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B4E;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051B4E: andi.w #$1f, d2
pub fn i_051b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B52;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051B52: move.w d2, $22(a0)
pub fn i_051b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B56;
    cpu.ir = 0x3142;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051B56: move.w $8(a0), d2
pub fn i_051b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B5A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051B5A: sub.w d1, d2
pub fn i_051b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B5C;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051B5C: lsr.w #$4, d2
pub fn i_051b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B5E;
    cpu.ir = 0xE84A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051B5E: add.w $24(a0), d2
pub fn i_051b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B62;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051B62: andi.w #$1f, d2
pub fn i_051b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B66;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051B66: move.w d2, $24(a0)
pub fn i_051b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B6A;
    cpu.ir = 0x3142;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051B6A: move.w d0, $4(a0)
pub fn i_051b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B6E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051B6E: move.w d1, $8(a0)
pub fn i_051b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B72;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051B72: clr.w $1e(a0)
pub fn i_051b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B76;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051B76: clr.w $20(a0)
pub fn i_051b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B7A;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051B7A: moveq #$20, d0
pub fn i_051b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B7C;
    cpu.ir = 0x7020;
    cpu.d[0] = 0x20;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B7C: moveq #$0, d1
pub fn i_051b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B7E;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B7E: bra.b $51b1c
pub fn i_051b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B80;
    cpu.ir = 0x609C;
    cpu.pc = 0x51B1C;
    10
}
// ===== Integrator_XY_051B80  ($051B80, 40 B, video/camera_sprite_helpers_043e8c.s)
/// $051B80: move.l $4(a0), d2
pub fn i_051b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B84;
    cpu.ir = 0x2428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B84: add.l d2, d0
pub fn i_051b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B86;
    cpu.ir = 0xD082;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $051B86: move.l d0, $4(a0)
pub fn i_051b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B8A;
    cpu.ir = 0x2140;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B8A: swap d0
pub fn i_051b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B8C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B8C: swap d2
pub fn i_051b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B8E;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B8E: sub.w d2, d0
pub fn i_051b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B90;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051B90: move.w d0, (a0)
pub fn i_051b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B92;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $051B92: move.l $8(a0), d2
pub fn i_051b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B96;
    cpu.ir = 0x2428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B96: add.l d2, d1
pub fn i_051b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B98;
    cpu.ir = 0xD282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[1] = r;
    4
}
/// $051B98: move.l d1, $8(a0)
pub fn i_051b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B9C;
    cpu.ir = 0x2141;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B9C: swap d1
pub fn i_051b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B9E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B9E: swap d2
pub fn i_051b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BA0;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051BA0: sub.w d2, d1
pub fn i_051ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BA2;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051BA2: move.w d1, $2(a0)
pub fn i_051ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BA6;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051BA6: rts
pub fn i_051ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellApply_BidirScan_051BA8  ($051BA8, 96 B, entity/collision_cell_apply_051ba8.s)
/// $051BA8: movem.l d2-d3/a1, -(a7)
pub fn i_051ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BAC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051BAC: movea.l $12(a0), a1
pub fn i_051bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BB0;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x12);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051BB0: move.l a1, d2
pub fn i_051bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BB2;
    cpu.ir = 0x2409;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051BB2: beq.w $51bfe
pub fn i_051bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51BFE; return 10; }
    8
}
/// $051BB6: move.w (a1), d2
pub fn i_051bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BB8;
    cpu.ir = 0x3411;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $051BB8: move.w $4(a1), d3
pub fn i_051bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BBC;
    cpu.ir = 0x3629;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051BBC: cmp.w d2, d0
pub fn i_051bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BBE;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051BBE: bcc.w $51bd0
pub fn i_051bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51BD0; return 10; }
    8
}
/// $051BC2: cmpi.w #$0, d2
pub fn i_051bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BC6;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $051BC6: beq.w $51bfe
pub fn i_051bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51BFE; return 10; }
    8
}
/// $051BCA: lea.l -$4(a1), a1
pub fn i_051bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BCE;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0xFFFFFFFC);
    cpu.a[1] = ad1;
    16
}
/// $051BCE: bra.b $51bb6
pub fn i_051bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BD0;
    cpu.ir = 0x60E6;
    cpu.pc = 0x51BB6;
    10
}
/// $051BD0: cmp.w d3, d0
pub fn i_051bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BD2;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051BD2: bcs.w $51be6
pub fn i_051bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BD6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x51BE6; return 10; }
    8
}
/// $051BD6: cmpi.w #$ffff, $4(a1)
pub fn i_051bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BDC;
    cpu.ir = 0xC69;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $051BDC: beq.w $51bfe
pub fn i_051bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BE0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51BFE; return 10; }
    8
}
/// $051BE0: lea.l $4(a1), a1
pub fn i_051be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BE4;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $051BE4: bra.b $51bb6
pub fn i_051be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BE6;
    cpu.ir = 0x60D0;
    cpu.pc = 0x51BB6;
    10
}
/// $051BE6: move.b $2(a1), $27(a0)
pub fn i_051be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BEC;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $051BEC: move.b $3(a1), d2
pub fn i_051bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BF0;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $051BF0: add.w $24(a0), d2
pub fn i_051bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BF4;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051BF4: neg.w d2
pub fn i_051bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BF6;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051BF6: andi.w #$1f, d2
pub fn i_051bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BFA;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051BFA: move.b d2, $26(a0)
pub fn i_051bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BFE;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051BFE: move.l a1, $12(a0)
pub fn i_051bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C02;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x12);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051C02: movem.l (a7)+, d2-d3/a1
pub fn i_051c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C06;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051C06: rts
pub fn i_051c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Collision_ProbeRange_051C08  ($051C08, 122 B, entity/collision_probes_051c08.s)
/// $051C08: movea.l $e(a0), a1
pub fn i_051c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C0C;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051C0C: move.w $4(a0), d0
pub fn i_051c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C10;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C10: move.w $8(a0), d1
pub fn i_051c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C14;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C14: add.w $16(a0), d0
pub fn i_051c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C18;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C18: asr.w #$4, d0
pub fn i_051c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C1A;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051C1A: add.w $18(a0), d1
pub fn i_051c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C1E;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C1E: asr.w #$4, d1
pub fn i_051c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C20;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051C20: move.w d0, $1e(a0)
pub fn i_051c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C24;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051C24: move.w d1, $20(a0)
pub fn i_051c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C28;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051C28: move.w d0, d2
pub fn i_051c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C2A;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051C2A: move.w d1, d3
pub fn i_051c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C2C;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051C2C: move.w $1a(a0), d4
pub fn i_051c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C30;
    cpu.ir = 0x3828;
    let ad2 = cpu.a[0].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C30: move.w $1c(a0), d5
pub fn i_051c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C34;
    cpu.ir = 0x3A28;
    let ad2 = cpu.a[0].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C34: bsr.w $51d84
pub fn i_051c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C38;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51C38);
    cpu.pc = 0x51D84;
    18
}
/// $051C38: bcc.w $51c80
pub fn i_051c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51C80; return 10; }
    8
}
/// $051C3C: movem.w d0-d6, -(a7)
pub fn i_051c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C40;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051C40: movem.w d0/d4, -(a7)
pub fn i_051c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C44;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051C44: subq.w #$1, d4
pub fn i_051c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C46;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051C46: lea.l $52(a0), a2
pub fn i_051c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C4A;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x52);
    cpu.a[2] = ad1;
    16
}
/// $051C4A: lea.l $32(a0), a3
pub fn i_051c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C4E;
    cpu.ir = 0x47E8;
    let ad1 = cpu.a[0].wrapping_add(0x32);
    cpu.a[3] = ad1;
    16
}
/// $051C4E: bsr.w $51ba8
pub fn i_051c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C52;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51C52);
    cpu.pc = 0x51BA8;
    18
}
/// $051C52: move.w d0, d7
pub fn i_051c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C54;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051C54: add.w $22(a0), d7
pub fn i_051c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C58;
    cpu.ir = 0xDE68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C58: andi.w #$1f, d7
pub fn i_051c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C5C;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051C5C: move.b $26(a0), (a2, d7.w)
pub fn i_051c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C62;
    cpu.ir = 0x15A8;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051C62: move.b $27(a0), (a3, d7.w)
pub fn i_051c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C68;
    cpu.ir = 0x17A8;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051C68: addq.w #$1, d0
pub fn i_051c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C6A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051C6A: dbra d4, $51c4e
pub fn i_051c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C6E;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51C6E; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51C6E; return 14; }
    cpu.pc = 0x51C4E;
    10
}
/// $051C6E: movem.w (a7)+, d0/d4
pub fn i_051c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C72;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051C72: bsr.w $51de2
pub fn i_051c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C76;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51C76);
    cpu.pc = 0x51DE2;
    18
}
/// $051C76: movem.w (a7)+, d0-d6
pub fn i_051c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C7A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051C7A: ori.b #$1, ccr
pub fn i_051c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C7E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051C7E: rts
pub fn i_051c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051C80: rts
pub fn i_051c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Collision_ProbeX_051C82  ($051C82, 116 B, entity/collision_probes_051c08.s)
/// $051C82: movea.l $e(a0), a1
pub fn i_051c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C86;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051C86: move.w $4(a0), d0
pub fn i_051c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C8A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C8A: move.w $8(a0), d1
pub fn i_051c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C8E;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C8E: add.w $16(a0), d0
pub fn i_051c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C92;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C92: asr.w #$4, d0
pub fn i_051c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C94;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051C94: add.w $18(a0), d1
pub fn i_051c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C98;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C98: asr.w #$4, d1
pub fn i_051c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C9A;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051C9A: cmp.w $1e(a0), d0
pub fn i_051c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C9E;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $051C9E: beq.w $51cf0
pub fn i_051c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51CF0; return 10; }
    8
}
/// $051CA2: move.w d0, $1e(a0)
pub fn i_051ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CA6;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051CA6: move.w (a0), d2
pub fn i_051ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CA8;
    cpu.ir = 0x3410;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $051CA8: bmi.w $51cb2
pub fn i_051ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CAC;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51CB2; return 10; }
    8
}
/// $051CAC: add.w $1a(a0), d0
pub fn i_051cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB0;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051CB0: subq.w #$1, d0
pub fn i_051cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB2;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051CB2: move.w d0, d2
pub fn i_051cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB4;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051CB4: move.w d1, d3
pub fn i_051cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB6;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051CB6: moveq #$1, d4
pub fn i_051cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB8;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051CB8: move.w $1c(a0), d5
pub fn i_051cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CBC;
    cpu.ir = 0x3A28;
    let ad2 = cpu.a[0].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051CBC: bsr.w $51d84
pub fn i_051cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CC0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51CC0);
    cpu.pc = 0x51D84;
    18
}
/// $051CC0: bcc.w $51cf0
pub fn i_051cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51CF0; return 10; }
    8
}
/// $051CC4: movem.w d0-d6, -(a7)
pub fn i_051cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CC8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051CC8: bsr.w $51ba8
pub fn i_051cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CCC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51CCC);
    cpu.pc = 0x51BA8;
    18
}
/// $051CCC: move.w d0, d7
pub fn i_051ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CCE;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051CCE: add.w $22(a0), d7
pub fn i_051cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CD2;
    cpu.ir = 0xDE68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051CD2: andi.w #$1f, d7
pub fn i_051cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CD6;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051CD6: move.b $26(a0), $52(a0, d7.w)
pub fn i_051cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CDC;
    cpu.ir = 0x11A8;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x52).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051CDC: move.b $27(a0), $32(a0, d7.w)
pub fn i_051cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CE2;
    cpu.ir = 0x11A8;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051CE2: bsr.w $51de2
pub fn i_051ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CE6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51CE6);
    cpu.pc = 0x51DE2;
    18
}
/// $051CE6: movem.w (a7)+, d0-d6
pub fn i_051ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CEA;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051CEA: ori.b #$1, ccr
pub fn i_051cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CEE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051CEE: rts
pub fn i_051cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051CF0: andi.b #$fe, ccr
pub fn i_051cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CF4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $051CF4: rts
pub fn i_051cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Collision_ProbeY_051CF6  ($051CF6, 142 B, entity/collision_probes_051c08.s)
/// $051CF6: movea.l $e(a0), a1
pub fn i_051cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CFA;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051CFA: move.w $4(a0), d0
pub fn i_051cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CFE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051CFE: move.w $8(a0), d1
pub fn i_051cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D02;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051D02: add.w $16(a0), d0
pub fn i_051d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D06;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051D06: asr.w #$4, d0
pub fn i_051d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D08;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051D08: add.w $18(a0), d1
pub fn i_051d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D0C;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051D0C: asr.w #$4, d1
pub fn i_051d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D0E;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051D0E: cmp.w $20(a0), d1
pub fn i_051d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D12;
    cpu.ir = 0xB268;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $051D12: beq.w $51d7e
pub fn i_051d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51D7E; return 10; }
    8
}
/// $051D16: move.w d1, $20(a0)
pub fn i_051d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D1A;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051D1A: move.w $2(a0), d2
pub fn i_051d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D1E;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051D1E: bmi.w $51d28
pub fn i_051d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D22;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51D28; return 10; }
    8
}
/// $051D22: add.w $1c(a0), d1
pub fn i_051d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D26;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051D26: subq.w #$1, d1
pub fn i_051d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D28;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051D28: move.w d0, d2
pub fn i_051d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D2A;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051D2A: move.w d1, d3
pub fn i_051d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D2C;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051D2C: move.w $1a(a0), d4
pub fn i_051d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D30;
    cpu.ir = 0x3828;
    let ad2 = cpu.a[0].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051D30: moveq #$1, d5
pub fn i_051d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D32;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051D32: bsr.w $51d84
pub fn i_051d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D36;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51D36);
    cpu.pc = 0x51D84;
    18
}
/// $051D36: bcc.w $51d7e
pub fn i_051d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51D7E; return 10; }
    8
}
/// $051D3A: movem.w d0-d6, -(a7)
pub fn i_051d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D3E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051D3E: movem.w d0/d4, -(a7)
pub fn i_051d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D42;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051D42: subq.w #$1, d4
pub fn i_051d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D44;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051D44: lea.l $52(a0), a2
pub fn i_051d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D48;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x52);
    cpu.a[2] = ad1;
    16
}
/// $051D48: lea.l $32(a0), a3
pub fn i_051d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D4C;
    cpu.ir = 0x47E8;
    let ad1 = cpu.a[0].wrapping_add(0x32);
    cpu.a[3] = ad1;
    16
}
/// $051D4C: bsr.w $51ba8
pub fn i_051d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D50;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51D50);
    cpu.pc = 0x51BA8;
    18
}
/// $051D50: move.w d0, d7
pub fn i_051d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D52;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051D52: add.w $22(a0), d7
pub fn i_051d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D56;
    cpu.ir = 0xDE68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051D56: andi.w #$1f, d7
pub fn i_051d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D5A;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051D5A: move.b $26(a0), (a2, d7.w)
pub fn i_051d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D60;
    cpu.ir = 0x15A8;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051D60: move.b $27(a0), (a3, d7.w)
pub fn i_051d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D66;
    cpu.ir = 0x17A8;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051D66: addq.w #$1, d0
pub fn i_051d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D68;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051D68: dbra d4, $51d4c
pub fn i_051d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D6C;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51D6C; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51D6C; return 14; }
    cpu.pc = 0x51D4C;
    10
}
/// $051D6C: movem.w (a7)+, d0/d4
pub fn i_051d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D70;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051D70: bsr.w $51de2
pub fn i_051d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D74;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51D74);
    cpu.pc = 0x51DE2;
    18
}
/// $051D74: movem.w (a7)+, d0-d6
pub fn i_051d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D78;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051D78: ori.b #$1, ccr
pub fn i_051d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D7C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051D7C: rts
pub fn i_051d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051D7E: andi.b #$fe, ccr
pub fn i_051d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D82;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $051D82: rts
pub fn i_051d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_ClipRectToWindow_051d84  ($051D84, 82 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051D84: add.w d2, d4
pub fn i_051d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D86;
    cpu.ir = 0xD842;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051D86: move.w (a1), d6
pub fn i_051d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D88;
    cpu.ir = 0x3C11;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $051D88: cmp.w d2, d6
pub fn i_051d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D8A;
    cpu.ir = 0xBC42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051D8A: bcc.w $51d98
pub fn i_051d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51D98; return 10; }
    8
}
/// $051D8E: cmp.w d2, d4
pub fn i_051d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D90;
    cpu.ir = 0xB842;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051D90: bhi.w $51ddc
pub fn i_051d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D94;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x51DDC; return 10; }
    8
}
/// $051D94: sub.w d2, d0
pub fn i_051d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D96;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051D96: clr.w d2
pub fn i_051d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D98;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051D98: cmp.w d4, d6
pub fn i_051d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D9A;
    cpu.ir = 0xBC44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051D9A: bcc.w $51da0
pub fn i_051d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D9E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51DA0; return 10; }
    8
}
/// $051D9E: move.w d6, d4
pub fn i_051d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DA0;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051DA0: sub.w d2, d4
pub fn i_051da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DA2;
    cpu.ir = 0x9842;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DA2: beq.w $51ddc
pub fn i_051da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51DDC; return 10; }
    8
}
/// $051DA6: add.w d3, d5
pub fn i_051da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DA8;
    cpu.ir = 0xDA43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DA8: move.w $2(a1), d6
pub fn i_051da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DAC;
    cpu.ir = 0x3C29;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051DAC: cmp.w d3, d6
pub fn i_051dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DAE;
    cpu.ir = 0xBC43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051DAE: bcc.w $51dbc
pub fn i_051dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51DBC; return 10; }
    8
}
/// $051DB2: cmp.w d3, d5
pub fn i_051db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DB4;
    cpu.ir = 0xBA43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051DB4: bhi.w $51ddc
pub fn i_051db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DB8;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x51DDC; return 10; }
    8
}
/// $051DB8: sub.w d3, d1
pub fn i_051db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DBA;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DBA: clr.w d3
pub fn i_051dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DBC;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051DBC: cmp.w d5, d6
pub fn i_051dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DBE;
    cpu.ir = 0xBC45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051DBE: bcc.w $51dc4
pub fn i_051dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51DC4; return 10; }
    8
}
/// $051DC2: move.w d6, d5
pub fn i_051dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DC4;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051DC4: sub.w d3, d5
pub fn i_051dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DC6;
    cpu.ir = 0x9A43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DC6: beq.w $51ddc
pub fn i_051dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51DDC; return 10; }
    8
}
/// $051DCA: andi.l #$ffff, d4
pub fn i_051dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DD0;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $051DD0: andi.l #$ffff, d5
pub fn i_051dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DD6;
    cpu.ir = 0x285;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[5] = r2;
    8
}
// ===== SetC_051dd6  ($051DD6, 6 B, thunks/ccr_helpers.c)
/// $051DD6: ori.b #$1, ccr
pub fn i_051dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DDA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051DDA: rts
pub fn i_051dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_051ddc  ($051DDC, 6 B, thunks/ccr_helpers.c)
/// $051DDC: andi.b #$fe, ccr
pub fn i_051ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DE0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $051DE0: rts
pub fn i_051de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellCommit_MMIO_051DE2  ($051DE2, 146 B, entity/collision_cell_apply_051ba8.s)
/// $051DE2: andi.l #$ffff, d4
pub fn i_051de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DE8;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $051DE8: andi.l #$ffff, d5
pub fn i_051de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DEE;
    cpu.ir = 0x285;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[5] = r2;
    8
}
/// $051DEE: move.w $2(a1), d6
pub fn i_051dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DF2;
    cpu.ir = 0x3C29;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051DF2: mulu.w d6, d2
pub fn i_051df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DF4;
    cpu.ir = 0xC4C6;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[2] & 0xFFFF);
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $051DF4: add.w d3, d2
pub fn i_051df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DF6;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DF6: add.l d2, d2
pub fn i_051df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DF8;
    cpu.ir = 0xD482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $051DF8: add.l d2, d2
pub fn i_051df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DFA;
    cpu.ir = 0xD482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $051DFA: movea.l $4(a1), a3
pub fn i_051dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DFE;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $051DFE: lea.l (a3, d2.l), a3
pub fn i_051dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E02;
    cpu.ir = 0x47F3;
    let ad1 = cpu.a[3].wrapping_add(0x0).wrapping_add(cpu.d[2]);
    cpu.a[3] = ad1;
    18
}
/// $051E02: sub.w d5, d6
pub fn i_051e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E04;
    cpu.ir = 0x9C45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051E04: add.w d6, d6
pub fn i_051e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E06;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051E06: add.w d6, d6
pub fn i_051e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E08;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051E08: subq.w #$1, d4
pub fn i_051e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E0A;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E0A: subq.w #$1, d5
pub fn i_051e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E0C;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E0C: add.w $22(a0), d0
pub fn i_051e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E10;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051E10: add.w $24(a0), d1
pub fn i_051e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E14;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051E14: movem.w d0-d1/d4-d5, -(a7)
pub fn i_051e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E18;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051E18: andi.w #$1f, d0
pub fn i_051e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E1C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E1C: add.b $52(a0, d0.w), d1
pub fn i_051e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E20;
    cpu.ir = 0xD230;
    let ad2 = cpu.a[0].wrapping_add(0x52).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $051E20: andi.w #$1f, d1
pub fn i_051e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E24;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E24: add.w $28(a0), d0
pub fn i_051e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E28;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051E28: lsl.w #$6, d0
pub fn i_051e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E2A;
    cpu.ir = 0xED48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $051E2A: addi.w #$40, d0
pub fn i_051e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E2E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E2E: movea.w d0, a1
pub fn i_051e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E30;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $051E30: add.w d1, d1
pub fn i_051e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E32;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051E32: lea.l (a1, d1.w), a2
pub fn i_051e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E36;
    cpu.ir = 0x45F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $051E36: move.w a2, $106ee4.l
pub fn i_051e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E3C;
    cpu.ir = 0x33CA;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051E3C: move.w a2, $3c0000.l
pub fn i_051e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E42;
    cpu.ir = 0x33CA;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051E42: move.w (a3)+, $3c0002.l
pub fn i_051e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E48;
    cpu.ir = 0x33DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v3 as u16);
    20
}
/// $051E48: addq.w #$1, a2
pub fn i_051e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E4A;
    cpu.ir = 0x524A;
    cpu.a[2] = cpu.a[2].wrapping_add(1);
    8
}
/// $051E4A: move.w a2, $106ee4.l
pub fn i_051e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E50;
    cpu.ir = 0x33CA;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051E50: move.w a2, $3c0000.l
pub fn i_051e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E56;
    cpu.ir = 0x33CA;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051E56: move.w (a3)+, $3c0002.l
pub fn i_051e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E5C;
    cpu.ir = 0x33DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v3 as u16);
    20
}
/// $051E5C: addq.w #$2, d1
pub fn i_051e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E5E;
    cpu.ir = 0x5441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E5E: andi.w #$3f, d1
pub fn i_051e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E62;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E62: dbra d5, $51e32
pub fn i_051e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E66;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51E66; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51E66; return 14; }
    cpu.pc = 0x51E32;
    10
}
/// $051E66: movem.w (a7)+, d0-d1/d4-d5
pub fn i_051e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E6A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051E6A: addq.w #$1, d0
pub fn i_051e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E6C;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E6C: adda.w d6, a3
pub fn i_051e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E6E;
    cpu.ir = 0xD6C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051E6E: dbra d4, $51e14
pub fn i_051e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E72;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51E72; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51E72; return 14; }
    cpu.pc = 0x51E14;
    10
}
/// $051E72: rts
pub fn i_051e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_SetDirty_051ece  ($051ECE, 8 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051ECE: bset.b #$0, $c(a0)
pub fn i_051ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ED4;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0xC);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $051ED4: rts
pub fn i_051ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ED6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_ClearSprites32_051ed6  ($051ED6, 44 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051ED6: bclr.b #$0, $c(a0)
pub fn i_051ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EDC;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0xC);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $051EDC: moveq #$1f, d1
pub fn i_051edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EDE;
    cpu.ir = 0x721F;
    cpu.d[1] = 0x1F;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051EDE: move.w $28(a0), d0
pub fn i_051ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EE2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051EE2: addi.w #$8201, d0
pub fn i_051ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EE6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8201, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051EE6: move.w d0, $106ee4.l
pub fn i_051ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EEC;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051EEC: move.w d0, $3c0000.l
pub fn i_051eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EF2;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051EF2: move.w #$0, $3c0002.l
pub fn i_051ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EFA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $051EFA: addq.w #$1, d0
pub fn i_051efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EFC;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051EFC: dbra d1, $51ee6
pub fn i_051efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F00;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51F00; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51F00; return 14; }
    cpu.pc = 0x51EE6;
    10
}
/// $051F00: rts
pub fn i_051f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_ClearVramBlock_051f02  ($051F02, 46 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051F02: move.l #$3ff, d7
pub fn i_051f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F08;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x3FF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051F08: move.w $28(a0), d0
pub fn i_051f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F0C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F0C: lsl.w #$6, d0
pub fn i_051f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F0E;
    cpu.ir = 0xED48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $051F0E: addi.w #$40, d0
pub fn i_051f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F12;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051F12: swap d0
pub fn i_051f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F14;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051F14: clr.w d0
pub fn i_051f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F16;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051F16: moveq #$1, d1
pub fn i_051f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F18;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051F18: swap d1
pub fn i_051f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F1A;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051F1A: move.l d0, $3c0000.l
pub fn i_051f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F20;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x3C0000, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $051F20: add.l d1, d0
pub fn i_051f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F22;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $051F22: move.l d0, $3c0000.l
pub fn i_051f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F28;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x3C0000, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $051F28: add.l d1, d0
pub fn i_051f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F2A;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $051F2A: dbra d7, $51f1a
pub fn i_051f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F2E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51F2E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51F2E; return 14; }
    cpu.pc = 0x51F1A;
    10
}
/// $051F2E: rts
pub fn i_051f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TransformCommit_MMIO_051F30  ($051F30, 100 B, video/camera_sprite_helpers_043e8c.s)
/// $051F30: btst.b #$0, $c(a0)
pub fn i_051f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F36;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $051F36: bne.w $51f3c
pub fn i_051f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51F3C; return 10; }
    8
}
/// $051F3A: rts
pub fn i_051f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051F3C: move.w $4(a0), d0
pub fn i_051f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F40;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F40: move.w $22(a0), d1
pub fn i_051f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F44;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F44: lsl.w #$4, d1
pub fn i_051f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F46;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051F46: add.w d1, d0
pub fn i_051f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F48;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051F48: move.w d0, d1
pub fn i_051f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F4A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051F4A: lsr.w #$4, d1
pub fn i_051f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F4C;
    cpu.ir = 0xE849;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051F4C: subq.b #$1, d1
pub fn i_051f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F4E;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051F4E: andi.w #$1f, d1
pub fn i_051f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F52;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051F52: move.w d1, $2a(a0)
pub fn i_051f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F56;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051F56: addi.w #$16, d1
pub fn i_051f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F5A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x16, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051F5A: andi.w #$1f, d1
pub fn i_051f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F5E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051F5E: move.w d1, $2c(a0)
pub fn i_051f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F62;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051F62: neg.w d0
pub fn i_051f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F64;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051F64: lsl.w #$7, d0
pub fn i_051f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F66;
    cpu.ir = 0xEF48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $051F66: move.w d0, $2e(a0)
pub fn i_051f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F6A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051F6A: move.w $8(a0), d0
pub fn i_051f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F6E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F6E: move.w $24(a0), d1
pub fn i_051f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F72;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F72: lsl.w #$4, d1
pub fn i_051f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F74;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051F74: add.w d1, d0
pub fn i_051f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F76;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051F76: lsl.w #$7, d0
pub fn i_051f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F78;
    cpu.ir = 0xEF48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $051F78: move.w d0, $30(a0)
pub fn i_051f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F7C;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051F7C: movem.l a0/a6, -(a7)
pub fn i_051f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F80;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051F80: movea.l a0, a6
pub fn i_051f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F82;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $051F82: lea.l $51f94(pc), a0
pub fn i_051f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F86;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x51F94;
    16
}
/// $051F86: jsr $1f4a.l
pub fn i_051f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51F8C);
    cpu.pc = 0x1F4A;
    16
}
/// $051F8C: movea.l a6, a0
pub fn i_051f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F8E;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $051F8E: movem.l (a7)+, a0/a6
pub fn i_051f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F92;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051F92: rts
pub fn i_051f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TileMap_HandlerInline_051F94  ($051F94, 158 B, entity/collision_probes_051c08.s)
/// $051F94: move.w #$8201, d0
pub fn i_051f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051F98: add.w $28(a6), d0
pub fn i_051f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F9C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051F9C: move.w $2a(a6), d1
pub fn i_051f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FA0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051FA0: move.w $30(a6), d2
pub fn i_051fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FA4;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051FA4: move.w d0, d5
pub fn i_051fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FA6;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051FA6: add.w d1, d5
pub fn i_051fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FA8;
    cpu.ir = 0xDA41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051FA8: move.w d5, $106ee4.l
pub fn i_051fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FAE;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051FAE: move.w d5, $3c0000.l
pub fn i_051fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FB4;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051FB4: move.w #$0, $3c0002.l
pub fn i_051fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FBC;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $051FBC: addq.b #$1, d1
pub fn i_051fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FBE;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051FBE: andi.b #$1f, d1
pub fn i_051fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FC2;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $051FC2: moveq #$b, d4
pub fn i_051fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FC4;
    cpu.ir = 0x780B;
    cpu.d[4] = 0xB;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051FC4: move.w d1, d3
pub fn i_051fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FC6;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051FC6: lsl.w d4, d3
pub fn i_051fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FC8;
    cpu.ir = 0xE96B;
    let cnt = cpu.d[4] & 0x3F;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    6 + 2 * cnt
}
/// $051FC8: add.w $2e(a6), d3
pub fn i_051fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FCC;
    cpu.ir = 0xD66E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051FCC: cmp.w $2c(a6), d1
pub fn i_051fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FD0;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $051FD0: beq.w $52018
pub fn i_051fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52018; return 10; }
    8
}
/// $051FD4: move.w d0, d5
pub fn i_051fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FD6;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051FD6: add.w d1, d5
pub fn i_051fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FD8;
    cpu.ir = 0xDA41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051FD8: move.w d5, $106ee4.l
pub fn i_051fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FDE;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051FDE: move.w d5, $3c0000.l
pub fn i_051fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FE4;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051FE4: move.b $52(a6, d1.w), d6
pub fn i_051fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FE8;
    cpu.ir = 0x1C36;
    let ad2 = cpu.a[6].wrapping_add(0x52).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $051FE8: lsl.w d4, d6
pub fn i_051fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FEA;
    cpu.ir = 0xE96E;
    let cnt = cpu.d[4] & 0x3F;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    6 + 2 * cnt
}
/// $051FEA: add.w d2, d6
pub fn i_051fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FEC;
    cpu.ir = 0xDC42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051FEC: or.b $32(a6, d1.w), d6
pub fn i_051fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FF0;
    cpu.ir = 0x8C36;
    let ad2 = cpu.a[6].wrapping_add(0x32).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $051FF0: move.w d6, $3c0002.l
pub fn i_051ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FF6;
    cpu.ir = 0x33C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $051FF6: addi.w #$200, d5
pub fn i_051ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FFA;
    cpu.ir = 0x645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051FFA: move.w d5, $106ee4.l
pub fn i_051ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52000;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $052000: move.w d5, $3c0000.l
pub fn i_052000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52006;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $052006: addq.b #$1, d1
pub fn i_052006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52008;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $052008: andi.w #$1f, d1
pub fn i_052008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5200C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05200C: move.w d3, $3c0002.l
pub fn i_05200c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52012;
    cpu.ir = 0x33C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $052012: addi.w #$800, d3
pub fn i_052012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52016;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x800, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052016: bra.b $51fcc
pub fn i_052016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52018;
    cpu.ir = 0x60B4;
    cpu.pc = 0x51FCC;
    10
}
/// $052018: move.w d0, d4
pub fn i_052018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5201A;
    cpu.ir = 0x3800;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05201A: add.w d1, d4
pub fn i_05201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5201C;
    cpu.ir = 0xD841;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05201C: move.w d4, $106ee4.l
pub fn i_05201c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52022;
    cpu.ir = 0x33C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $052022: move.w d4, $3c0000.l
pub fn i_052022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52028;
    cpu.ir = 0x33C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $052028: move.w #$0, $3c0002.l
pub fn i_052028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52030;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $052030: rts
pub fn i_052030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52032;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixOverlay_DrawCreditsOrFree_052050  ($052050, 146 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052050: tst.b $10fd82.l
pub fn i_052050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52056;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $052056: beq.w $52068
pub fn i_052056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5205A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52068; return 10; }
    8
}
/// $05205A: move.b $10fd91.l, d0
pub fn i_05205a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52060;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD91) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $052060: andi.b #$2, d0
pub fn i_052060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52064;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $052064: bne.w $52142
pub fn i_052064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52068;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52142; return 10; }
    8
}
/// $052068: btst.b #$1, $1081be.l
pub fn i_052068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52070;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((1) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $052070: bne.w $52144
pub fn i_052070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52074;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52144; return 10; }
    8
}
/// $052074: btst.b #$0, $1081be.l
pub fn i_052074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5207C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05207C: beq.w $52142
pub fn i_05207c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52080;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52142; return 10; }
    8
}
/// $052080: bsr.w $521b2
pub fn i_052080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52084;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x52084);
    cpu.pc = 0x521B2;
    18
}
/// $052084: tst.b $10fd82.l
pub fn i_052084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5208A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $05208A: beq.w $520a6
pub fn i_05208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5208E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x520A6; return 10; }
    8
}
/// $05208E: lea.l $10fe00.l, a1
pub fn i_05208e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52094;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10FE00;
    20
}
/// $052094: tst.w $10fe80.l
pub fn i_052094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5209A;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x10FE80) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $05209A: bne.b $520ac
pub fn i_05209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5209C;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x520AC; return 10; }
    8
}
/// $05209C: lea.l $d00034.l, a1
pub fn i_05209c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520A2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xD00034;
    20
}
/// $0520A2: bra.w $520ac
pub fn i_0520a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x520AC;
    10
}
/// $0520A6: lea.l $1081bf.l, a1
pub fn i_0520a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081BF;
    20
}
/// $0520AC: move.w #$2300, d1
pub fn i_0520ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0520B0: btst.b #$4, $1081be.l
pub fn i_0520b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520B8;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0520B8: beq.w $520c0
pub fn i_0520b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x520C0; return 10; }
    8
}
/// $0520BC: move.w #$2300, d1
pub fn i_0520bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0520C0: tst.b $10fd82.l
pub fn i_0520c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520C6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0520C6: beq.w $520d4
pub fn i_0520c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x520D4; return 10; }
    8
}
/// $0520CA: cmpi.b #$1, $10fd83.l
pub fn i_0520ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520D2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0520D2: bne.b $520de
pub fn i_0520d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520D4;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x520DE; return 10; }
    8
}
/// $0520D4: move.w #$707d, d0
pub fn i_0520d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x707D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0520D8: bsr.w $520e2
pub fn i_0520d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x520DC);
    cpu.pc = 0x520E2;
    18
}
/// $0520DC: addq.l #$1, a1
pub fn i_0520dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520DE;
    cpu.ir = 0x5289;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    8
}
/// $0520DE: move.w #$737d, d0
pub fn i_0520de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520E2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x737D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== FixOverlay_PutCreditsLine_0520e2  ($0520E2, 80 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0520E2: lea.l $52230(pc), a2
pub fn i_0520e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520E6;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x52230;
    16
}
/// $0520E6: moveq #$5, d7
pub fn i_0520e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520E8;
    cpu.ir = 0x7E05;
    cpu.d[7] = 0x5;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0520E8: move.b (a2)+, d1
pub fn i_0520e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520EA;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0520EA: movem.w d0-d1, $3c0000.l
pub fn i_0520ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520F2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0520F2: addi.w #$20, d0
pub fn i_0520f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520F6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0520F6: dbra d7, $520e8
pub fn i_0520f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520FA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x520FA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x520FA; return 14; }
    cpu.pc = 0x520E8;
    10
}
/// $0520FA: move.b (a2)+, d1
pub fn i_0520fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520FC;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0520FC: move.b (a1), d2
pub fn i_0520fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520FE;
    cpu.ir = 0x1411;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0520FE: cmpi.b #$2, d2
pub fn i_0520fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52102;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $052102: bcc.w $5210a
pub fn i_052102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52106;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5210A; return 10; }
    8
}
/// $052106: move.b #$20, d1
pub fn i_052106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5210A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05210A: movem.w d0-d1, $3c0000.l
pub fn i_05210a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52112;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $052112: addi.w #$20, d0
pub fn i_052112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52116;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052116: move.b (a2)+, d1
pub fn i_052116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52118;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $052118: movem.w d0-d1, $3c0000.l
pub fn i_052118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52120;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $052120: addi.w #$20, d0
pub fn i_052120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52124;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052124: move.b d2, d1
pub fn i_052124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52126;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $052126: lsr.b #$4, d1
pub fn i_052126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52128;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $052128: bsr.w $52132
pub fn i_052128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5212C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5212C);
    cpu.pc = 0x52132;
    18
}
/// $05212C: andi.b #$f, d2
pub fn i_05212c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52130;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $052130: move.b d2, d1
pub fn i_052130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52132;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== FixOverlay_PutDigit_052132  ($052132, 128 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052132: addi.b #$30, d1
pub fn i_052132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52136;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $052136: movem.w d0-d1, $3c0000.l
pub fn i_052136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5213E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05213E: addi.w #$20, d0
pub fn i_05213e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52142;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052142: rts
pub fn i_052142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52144;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052144: move.w #$701d, d0
pub fn i_052144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52148;
    cpu.ir = 0x303C;
    let v1: u32 = 0x701D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052148: moveq #$f, d7
pub fn i_052148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5214A;
    cpu.ir = 0x7E0F;
    cpu.d[7] = 0xF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05214A: move.w #$20, d1
pub fn i_05214a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5214E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05214E: ori.w #$2300, d1
pub fn i_05214e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52152;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052152: btst.b #$4, $1081be.l
pub fn i_052152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5215A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05215A: beq.w $52166
pub fn i_05215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5215E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52166; return 10; }
    8
}
/// $05215E: move.w #$20, d1
pub fn i_05215e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52162;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052162: ori.w #$2300, d1
pub fn i_052162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52166;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052166: movem.w d0-d1, $3c0000.l
pub fn i_052166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5216E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05216E: addi.w #$20, d0
pub fn i_05216e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52172;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052172: dbra d7, $52166
pub fn i_052172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52176;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x52176; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x52176; return 14; }
    cpu.pc = 0x52166;
    10
}
/// $052176: move.w #$72fd, d0
pub fn i_052176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5217A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72FD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05217A: moveq #$10, d7
pub fn i_05217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5217C;
    cpu.ir = 0x7E10;
    cpu.d[7] = 0x10;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05217C: move.w #$20, d1
pub fn i_05217c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52180;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052180: ori.w #$2300, d1
pub fn i_052180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52184;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052184: btst.b #$4, $1081be.l
pub fn i_052184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5218C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05218C: beq.w $52198
pub fn i_05218c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52190;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52198; return 10; }
    8
}
/// $052190: move.w #$20, d1
pub fn i_052190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52194;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052194: ori.w #$2300, d1
pub fn i_052194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52198;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052198: movem.w d0-d1, $3c0000.l
pub fn i_052198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521A0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0521A0: addi.w #$20, d0
pub fn i_0521a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521A4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521A4: dbra d7, $52198
pub fn i_0521a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521A8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x521A8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x521A8; return 14; }
    cpu.pc = 0x52198;
    10
}
/// $0521A8: andi.b #$fc, $1081be.l
pub fn i_0521a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521B0;
    cpu.ir = 0x239;
    let v1 = bus.read8(0x1081BE) as u32;
    let r2 = (v1 & 0xFC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081BE, r2 as u8);
    20
}
/// $0521B0: rts
pub fn i_0521b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixOverlay_ClearCreditsArea_0521b2  ($0521B2, 126 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0521B2: move.w #$701d, d0
pub fn i_0521b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x701D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521B6: move.w #$2, d7
pub fn i_0521b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521BA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521BA: move.w #$20, d1
pub fn i_0521ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521BE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521BE: ori.w #$2300, d1
pub fn i_0521be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521C2;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521C2: btst.b #$4, $1081be.l
pub fn i_0521c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521CA;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0521CA: beq.w $521d6
pub fn i_0521ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x521D6; return 10; }
    8
}
/// $0521CE: move.w #$20, d1
pub fn i_0521ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521D2: ori.w #$2300, d1
pub fn i_0521d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521D6;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521D6: movem.w d0-d1, $3c0000.l
pub fn i_0521d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521DE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0521DE: addi.w #$20, d0
pub fn i_0521de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521E2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521E2: dbra d7, $521d6
pub fn i_0521e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521E6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x521E6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x521E6; return 14; }
    cpu.pc = 0x521D6;
    10
}
/// $0521E6: move.w #$719d, d0
pub fn i_0521e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521EA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x719D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521EA: move.w #$3, d7
pub fn i_0521ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521EE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521EE: movem.w d0-d1, $3c0000.l
pub fn i_0521ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521F6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0521F6: addi.w #$20, d0
pub fn i_0521f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521FA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521FA: dbra d7, $521ee
pub fn i_0521fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521FE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x521FE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x521FE; return 14; }
    cpu.pc = 0x521EE;
    10
}
/// $0521FE: move.w #$72fd, d0
pub fn i_0521fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52202;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72FD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052202: move.w #$3, d7
pub fn i_052202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52206;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052206: movem.w d0-d1, $3c0000.l
pub fn i_052206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5220E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05220E: addi.w #$20, d0
pub fn i_05220e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52212;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052212: dbra d7, $52206
pub fn i_052212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52216;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x52216; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x52216; return 14; }
    cpu.pc = 0x52206;
    10
}
/// $052216: move.w #$749d, d0
pub fn i_052216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5221A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x749D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05221A: move.w #$3, d7
pub fn i_05221a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5221E;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05221E: movem.w d0-d1, $3c0000.l
pub fn i_05221e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52226;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $052226: addi.w #$20, d0
pub fn i_052226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5222A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05222A: dbra d7, $5221e
pub fn i_05222a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5222E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5222E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5222E; return 14; }
    cpu.pc = 0x5221E;
    10
}
/// $05222E: rts
pub fn i_05222e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52230;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05223A: tst.b $10fd82.l
pub fn i_05223a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52240;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $052240: beq.w $522ac
pub fn i_052240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52244;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x522AC; return 10; }
    8
}
/// $052244: move.b $10fd91.l, d0
pub fn i_052244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5224A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD91) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $05224A: andi.b #$1, d0
pub fn i_05224a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5224E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05224E: bne.w $522ac
pub fn i_05224e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52252;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x522AC; return 10; }
    8
}
/// $052252: btst.b #$3, $1081be.l
pub fn i_052252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5225A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((3) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05225A: bne.w $522ae
pub fn i_05225a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5225E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x522AE; return 10; }
    8
}
/// $05225E: btst.b #$2, $1081be.l
pub fn i_05225e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52266;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $052266: beq.w $522ac
pub fn i_052266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5226A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x522AC; return 10; }
    8
}
/// $05226A: move.w #$721d, d0
pub fn i_05226a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5226E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x721D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05226E: lea.l $522ea(pc), a2
pub fn i_05226e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52272;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x522EA;
    16
}
/// $052272: moveq #$5, d7
pub fn i_052272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52274;
    cpu.ir = 0x7E05;
    cpu.d[7] = 0x5;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052274: move.w #$2300, d1
pub fn i_052274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52278;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052278: btst.b #$4, $1081be.l
pub fn i_052278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52280;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $052280: beq.w $52288
pub fn i_052280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52284;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52288; return 10; }
    8
}
/// $052284: move.w #$2300, d1
pub fn i_052284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52288;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052288: move.b (a2)+, d1
pub fn i_052288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5228A;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05228A: movem.w d0-d1, $3c0000.l
pub fn i_05228a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52292;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $052292: addi.w #$20, d0
pub fn i_052292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52296;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052296: dbra d7, $52288
pub fn i_052296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5229A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5229A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5229A; return 14; }
    cpu.pc = 0x52288;
    10
}
/// $05229A: move.b $10fd8b.l, d1
pub fn i_05229a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522A0;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10FD8B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0522A0: addi.b #$31, d1
pub fn i_0522a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522A4;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x31, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0522A4: movem.w d0-d1, $3c0000.l
pub fn i_0522a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522AC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0522AC: rts
pub fn i_0522ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0522AE: move.w #$721d, d0
pub fn i_0522ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x721D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0522B2: moveq #$7, d7
pub fn i_0522b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522B4;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0522B4: move.w #$20, d1
pub fn i_0522b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0522B8: ori.w #$2300, d1
pub fn i_0522b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522BC;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0522BC: btst.b #$4, $1081be.l
pub fn i_0522bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522C4;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0522C4: beq.w $522d0
pub fn i_0522c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x522D0; return 10; }
    8
}
/// $0522C8: move.w #$20, d1
pub fn i_0522c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522CC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0522CC: ori.w #$2300, d1
pub fn i_0522cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522D0;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0522D0: movem.w d0-d1, $3c0000.l
pub fn i_0522d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522D8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0522D8: addi.w #$20, d0
pub fn i_0522d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522DC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0522DC: dbra d7, $522d0
pub fn i_0522dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522E0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x522E0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x522E0; return 14; }
    cpu.pc = 0x522D0;
    10
}
/// $0522E0: andi.b #$f3, $1081be.l
pub fn i_0522e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522E8;
    cpu.ir = 0x239;
    let v1 = bus.read8(0x1081BE) as u32;
    let r2 = (v1 & 0xF3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081BE, r2 as u8);
    20
}
/// $0522E8: rts
pub fn i_0522e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052328: moveq #$0, d0
pub fn i_052328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5232A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05232A: tst.b $10fd82.l
pub fn i_05232a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52330;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $052330: bne.w $5233a
pub fn i_052330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52334;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5233A; return 10; }
    8
}
/// $052334: bset.b #$2, d0
pub fn i_052334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52338;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $052338: rts
pub fn i_052338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5233A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05233A: move.b #$1, $10fdb0.l
pub fn i_05233a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52342;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDB0, v1 as u8);
    16
}
/// $052342: move.b #$1, $10fdb1.l
pub fn i_052342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5234A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDB1, v1 as u8);
    16
}
/// $05234A: move.b #$0, $10fdb2.l
pub fn i_05234a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52352;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDB2, v1 as u8);
    16
}
/// $052352: move.b #$0, $10fdb3.l
pub fn i_052352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5235A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDB3, v1 as u8);
    16
}
/// $05235A: move.l d0, -(a7)
pub fn i_05235a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5235C;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05235C: jsr $c00450.l
pub fn i_05235c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52362;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52362);
    cpu.pc = 0xC00450;
    16
}
/// $052362: move.l (a7)+, d0
pub fn i_052362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52364;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052364: tst.b $10fdb0.l
pub fn i_052364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5236A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDB0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $05236A: beq.w $52372
pub fn i_05236a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5236E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52372; return 10; }
    8
}
/// $05236E: bset.b #$0, d0
pub fn i_05236e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52372;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $052372: tst.b $10fdb1.l
pub fn i_052372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52378;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDB1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $052378: beq.w $52380
pub fn i_052378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5237C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52380; return 10; }
    8
}
/// $05237C: bset.b #$1, d0
pub fn i_05237c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52380;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $052380: rts
pub fn i_052380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52382;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnAndPublishD0At70_05239E  ($05239E, 20 B, entity/spawn_publish_d0_at70_05239e.s)
/// $05239E: move.l d0, -(a7)
pub fn i_05239e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523A0;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0523A0: lea.l $523ee(pc), a1
pub fn i_0523a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x523EE;
    16
}
/// $0523A4: jsr $4ae.l
pub fn i_0523a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x523AA);
    cpu.pc = 0x4AE;
    16
}
/// $0523AA: move.l (a7)+, d0
pub fn i_0523aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523AC;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0523AC: move.w d0, $70(a0)
pub fn i_0523ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523B0;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0523B0: rts
pub fn i_0523b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnAndPublishD0At70_0523B2  ($0523B2, 20 B, entity/spawn_publish_d0_at70_0523b2.s)
/// $0523B2: move.l d0, -(a7)
pub fn i_0523b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523B4;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0523B4: lea.l $524aa(pc), a1
pub fn i_0523b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x524AA;
    16
}
/// $0523B8: jsr $4ae.l
pub fn i_0523b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x523BE);
    cpu.pc = 0x4AE;
    16
}
/// $0523BE: move.l (a7)+, d0
pub fn i_0523be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523C0;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0523C0: move.w d0, $70(a0)
pub fn i_0523c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523C4;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0523C4: rts
pub fn i_0523c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_SpawnIn_0523c6  ($0523C6, 20 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0523C6: move.l d0, -(a7)
pub fn i_0523c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523C8;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0523C8: lea.l $523fa(pc), a1
pub fn i_0523c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x523FA;
    16
}
/// $0523CC: jsr $4ae.l
pub fn i_0523cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x523D2);
    cpu.pc = 0x4AE;
    16
}
/// $0523D2: move.l (a7)+, d0
pub fn i_0523d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523D4;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0523D4: move.w d0, $70(a0)
pub fn i_0523d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523D8;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0523D8: rts
pub fn i_0523d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_SpawnOut_0523da  ($0523DA, 20 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0523DA: move.l d0, -(a7)
pub fn i_0523da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523DC;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0523DC: lea.l $524b6(pc), a1
pub fn i_0523dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x524B6;
    16
}
/// $0523E0: jsr $4ae.l
pub fn i_0523e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x523E6);
    cpu.pc = 0x4AE;
    16
}
/// $0523E6: move.l (a7)+, d0
pub fn i_0523e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523E8;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0523E8: move.w d0, $70(a0)
pub fn i_0523e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523EC;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0523EC: rts
pub fn i_0523ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Template_0523EE  ($0523EE, 12 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0523EE: move.b #$1, $10a2c8.l
pub fn i_0523ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523F6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $0523F6: bra.w $52402
pub fn i_0523f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x52402;
    10
}
// ===== PalFade_In_Task_0523fa  ($0523FA, 176 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0523FA: move.b #$ff, $10a2c8.l
pub fn i_0523fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52402;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $052402: moveq #$0, d0
pub fn i_052402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52404;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052404: move.b d0, $10a2ca.l
pub fn i_052404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5240A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $05240A: move.b d0, $10a2cb.l
pub fn i_05240a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52410;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052410: move.b d0, $10a2cc.l
pub fn i_052410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52416;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $052416: move.b d0, $10a2cf.l
pub fn i_052416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5241C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $05241C: moveq #$1, d1
pub fn i_05241c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5241E;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05241E: cmpi.w #$3, $70(a6)
pub fn i_05241e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52424;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $052424: bhi.w $5243e
pub fn i_052424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52428;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x5243E; return 10; }
    8
}
/// $052428: move.w $70(a6), d0
pub fn i_052428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5242C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05242C: add.w d0, d0
pub fn i_05242c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5242E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05242E: add.w d0, d0
pub fn i_05242e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52430;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $052430: lea.l $52570(pc), a0
pub fn i_052430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52434;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x52570;
    16
}
/// $052434: move.w (a0, d0.w), d1
pub fn i_052434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52438;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $052438: move.w $2(a0, d0.w), $70(a6)
pub fn i_052438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5243E;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    22
}
/// $05243E: move.w $70(a6), $72(a6)
pub fn i_05243e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52444;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052444: move.w #$0, $74(a6)
pub fn i_052444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5244A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05244A: move.w d1, $8c(a6)
pub fn i_05244a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5244E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05244E: lea.l $52454(pc), a1
pub fn i_05244e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52452;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52454;
    16
}
/// $052452: move.l a1, (a6)
pub fn i_052452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52454;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052454: subq.w #$1, $72(a6)
pub fn i_052454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52458;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052458: bne.w $524a8
pub fn i_052458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5245C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x524A8; return 10; }
    8
}
/// $05245C: move.w $70(a6), $72(a6)
pub fn i_05245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52462;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052462: move.w $8c(a6), d0
pub fn i_052462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52466;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052466: add.b d0, $10a2ca.l
pub fn i_052466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5246C;
    cpu.ir = 0xD139;
    let v1 = bus.read8(0x10A2CA) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CA, r2 as u8);
    16
}
/// $05246C: add.b d0, $10a2cb.l
pub fn i_05246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52472;
    cpu.ir = 0xD139;
    let v1 = bus.read8(0x10A2CB) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CB, r2 as u8);
    16
}
/// $052472: add.b d0, $10a2cc.l
pub fn i_052472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52478;
    cpu.ir = 0xD139;
    let v1 = bus.read8(0x10A2CC) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CC, r2 as u8);
    16
}
/// $052478: add.w d0, $74(a6)
pub fn i_052478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5247C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05247C: cmpi.w #$1f, $74(a6)
pub fn i_05247c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52482;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F, Size::Word, false);
    16
}
/// $052482: blt.w $524a8
pub fn i_052482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52486;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x524A8; return 10; }
    8
}
/// $052486: moveq #$1f, d0
pub fn i_052486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52488;
    cpu.ir = 0x701F;
    cpu.d[0] = 0x1F;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052488: move.b d0, $10a2ca.l
pub fn i_052488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5248E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $05248E: move.b d0, $10a2cb.l
pub fn i_05248e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52494;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052494: move.b d0, $10a2cc.l
pub fn i_052494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5249A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $05249A: move.b #$ff, $10a2cf.l
pub fn i_05249a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524A2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $0524A2: jmp $518.l
pub fn i_0524a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0524A8: rts
pub fn i_0524a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Template_0524AA  ($0524AA, 12 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0524AA: move.b #$1, $10a2c8.l
pub fn i_0524aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524B2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $0524B2: bra.w $524be
pub fn i_0524b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x524BE;
    10
}
// ===== PalFade_Out_Task_0524b6  ($0524B6, 86 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0524B6: move.b #$ff, $10a2c8.l
pub fn i_0524b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524BE;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $0524BE: moveq #$1f, d0
pub fn i_0524be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524C0;
    cpu.ir = 0x701F;
    cpu.d[0] = 0x1F;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0524C0: move.b d0, $10a2ca.l
pub fn i_0524c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524C6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $0524C6: move.b d0, $10a2cb.l
pub fn i_0524c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524CC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $0524CC: move.b d0, $10a2cc.l
pub fn i_0524cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524D2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $0524D2: move.b #$0, $10a2cf.l
pub fn i_0524d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524DA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $0524DA: moveq #$1, d1
pub fn i_0524da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524DC;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0524DC: cmpi.w #$3, $70(a6)
pub fn i_0524dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524E2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $0524E2: bhi.w $524fc
pub fn i_0524e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524E6;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x524FC; return 10; }
    8
}
/// $0524E6: move.w $70(a6), d0
pub fn i_0524e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524EA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0524EA: add.w d0, d0
pub fn i_0524ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524EC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0524EC: add.w d0, d0
pub fn i_0524ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524EE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0524EE: lea.l $52570(pc), a0
pub fn i_0524ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524F2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x52570;
    16
}
/// $0524F2: move.w (a0, d0.w), d1
pub fn i_0524f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524F6;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0524F6: move.w $2(a0, d0.w), $70(a6)
pub fn i_0524f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524FC;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0524FC: move.w $70(a6), $72(a6)
pub fn i_0524fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52502;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052502: move.w #$0, $74(a6)
pub fn i_052502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52508;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052508: move.w d1, $8c(a6)
pub fn i_052508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5250C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_05250c  ($05250C, 8 B, kernel/task_handlers.c)
/// $05250C: lea.l $52514(pc), a1
pub fn i_05250c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52510;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52514;
    16
}
/// $052510: move.l a1, (a6)
pub fn i_052510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52512;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052512: rts
pub fn i_052512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_Out_Step_052514  ($052514, 92 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052514: subq.w #$1, $72(a6)
pub fn i_052514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52518;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052518: bne.w $5256e
pub fn i_052518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5251C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5256E; return 10; }
    8
}
/// $05251C: move.w $70(a6), $72(a6)
pub fn i_05251c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52522;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052522: move.w $8c(a6), d0
pub fn i_052522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52526;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052526: sub.b d0, $10a2ca.l
pub fn i_052526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5252C;
    cpu.ir = 0x9139;
    let v1 = bus.read8(0x10A2CA) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CA, r2 as u8);
    16
}
/// $05252C: sub.b d0, $10a2cb.l
pub fn i_05252c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52532;
    cpu.ir = 0x9139;
    let v1 = bus.read8(0x10A2CB) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CB, r2 as u8);
    16
}
/// $052532: sub.b d0, $10a2cc.l
pub fn i_052532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52538;
    cpu.ir = 0x9139;
    let v1 = bus.read8(0x10A2CC) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CC, r2 as u8);
    16
}
/// $052538: add.w d0, $74(a6)
pub fn i_052538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5253C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05253C: cmpi.w #$1f, $74(a6)
pub fn i_05253c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52542;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F, Size::Word, false);
    16
}
/// $052542: blt.w $5256e
pub fn i_052542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52546;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5256E; return 10; }
    8
}
/// $052546: moveq #$0, d0
pub fn i_052546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52548;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052548: move.b d0, $10a2c8.l
pub fn i_052548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5254E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $05254E: move.b d0, $10a2ca.l
pub fn i_05254e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52554;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $052554: move.b d0, $10a2cb.l
pub fn i_052554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5255A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $05255A: move.b d0, $10a2cc.l
pub fn i_05255a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52560;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $052560: move.b #$ff, $10a2cf.l
pub fn i_052560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52568;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $052568: jmp $518.l
pub fn i_052568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5256E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05256E: rts
pub fn i_05256e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52570;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052580: movem.l d0-d3, -(a7)
pub fn i_052580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52584;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $052584: lea.l $525a4(pc), a1
pub fn i_052584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52588;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x525A4;
    16
}
/// $052588: jsr $4ae.l
pub fn i_052588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5258E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5258E);
    cpu.pc = 0x4AE;
    16
}
/// $05258E: movem.l (a7)+, d0-d3
pub fn i_05258e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52592;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $052592: move.w d0, $7c(a0)
pub fn i_052592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52596;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052596: move.w d1, $7e(a0)
pub fn i_052596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5259A;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05259A: move.w d2, $80(a0)
pub fn i_05259a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5259E;
    cpu.ir = 0x3142;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05259E: move.w d3, $70(a0)
pub fn i_05259e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525A2;
    cpu.ir = 0x3143;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525A2: rts
pub fn i_0525a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_ToColor_Task_0525a4  ($0525A4, 254 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0525A4: move.b #$80, $10a2c8.l
pub fn i_0525a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525AC;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $0525AC: move.b $10a2ca.l, $88(a6)
pub fn i_0525ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525B4;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10A2CA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0525B4: move.b $10a2cb.l, $89(a6)
pub fn i_0525b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525BC;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10A2CB) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0525BC: move.b $10a2cc.l, $8a(a6)
pub fn i_0525bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525C4;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10A2CC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0525C4: moveq #$0, d0
pub fn i_0525c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525C6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0525C6: move.w d0, $82(a6)
pub fn i_0525c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525CA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525CA: move.w d0, $84(a6)
pub fn i_0525ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525CE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525CE: move.w d0, $86(a6)
pub fn i_0525ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525D2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525D2: moveq #$0, d0
pub fn i_0525d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525D4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0525D4: move.b $10a2ca.l, d0
pub fn i_0525d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525DA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2CA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0525DA: asl.w #$8, d0
pub fn i_0525da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525DC;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0525DC: move.w $7c(a6), d1
pub fn i_0525dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525E0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0525E0: asl.w #$8, d1
pub fn i_0525e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525E2;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0525E2: sub.w d1, d0
pub fn i_0525e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525E4;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0525E4: ext.l d0
pub fn i_0525e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525E6;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0525E6: divs.w $70(a6), d0
pub fn i_0525e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525EA;
    cpu.ir = 0x81EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v3 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[0] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 166; }
    cpu.d[0] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    166
}
/// $0525EA: move.w d0, $76(a6)
pub fn i_0525ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525EE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525EE: moveq #$0, d0
pub fn i_0525ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525F0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0525F0: move.b $10a2cb.l, d0
pub fn i_0525f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525F6;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2CB) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0525F6: asl.w #$8, d0
pub fn i_0525f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525F8;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0525F8: move.w $7e(a6), d1
pub fn i_0525f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525FC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0525FC: asl.w #$8, d1
pub fn i_0525fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525FE;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0525FE: sub.w d1, d0
pub fn i_0525fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52600;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $052600: ext.l d0
pub fn i_052600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52602;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052602: divs.w $70(a6), d0
pub fn i_052602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52606;
    cpu.ir = 0x81EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v3 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[0] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 166; }
    cpu.d[0] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    166
}
/// $052606: move.w d0, $78(a6)
pub fn i_052606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5260A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05260A: moveq #$0, d0
pub fn i_05260a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5260C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05260C: move.b $10a2cc.l, d0
pub fn i_05260c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52612;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2CC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $052612: asl.w #$8, d0
pub fn i_052612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52614;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $052614: move.w $80(a6), d1
pub fn i_052614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52618;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052618: asl.w #$8, d1
pub fn i_052618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5261A;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05261A: sub.w d1, d0
pub fn i_05261a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5261C;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05261C: ext.l d0
pub fn i_05261c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5261E;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05261E: divs.w $70(a6), d0
pub fn i_05261e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52622;
    cpu.ir = 0x81EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v3 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[0] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 166; }
    cpu.d[0] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    166
}
/// $052622: move.w d0, $7a(a6)
pub fn i_052622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52626;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052626: lea.l $5262c(pc), a1
pub fn i_052626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5262A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5262C;
    16
}
/// $05262A: move.l a1, (a6)
pub fn i_05262a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5262C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05262C: subq.w #$1, $70(a6)
pub fn i_05262c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52630;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052630: beq.w $52684
pub fn i_052630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52634;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52684; return 10; }
    8
}
/// $052634: move.w $76(a6), d0
pub fn i_052634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52638;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052638: add.w d0, $82(a6)
pub fn i_052638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5263C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05263C: move.w $78(a6), d0
pub fn i_05263c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52640;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052640: add.w d0, $84(a6)
pub fn i_052640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52644;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052644: move.w $7a(a6), d0
pub fn i_052644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52648;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052648: add.w d0, $86(a6)
pub fn i_052648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5264C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05264C: move.w $82(a6), d0
pub fn i_05264c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52650;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052650: asr.w #$8, d0
pub fn i_052650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52652;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $052652: move.b $88(a6), d1
pub fn i_052652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52656;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $052656: sub.b d0, d1
pub fn i_052656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52658;
    cpu.ir = 0x9200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $052658: move.b d1, $10a2ca.l
pub fn i_052658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5265E;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $05265E: move.w $84(a6), d0
pub fn i_05265e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52662;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052662: asr.w #$8, d0
pub fn i_052662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52664;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $052664: move.b $89(a6), d1
pub fn i_052664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52668;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $052668: sub.b d0, d1
pub fn i_052668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5266A;
    cpu.ir = 0x9200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $05266A: move.b d1, $10a2cb.l
pub fn i_05266a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52670;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052670: move.w $86(a6), d0
pub fn i_052670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52674;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052674: asr.w #$8, d0
pub fn i_052674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52676;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $052676: move.b $8a(a6), d1
pub fn i_052676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5267A;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05267A: sub.b d0, d1
pub fn i_05267a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5267C;
    cpu.ir = 0x9200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $05267C: move.b d1, $10a2cc.l
pub fn i_05267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52682;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $052682: rts
pub fn i_052682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52684;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052684: move.w $7c(a6), d0
pub fn i_052684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52688;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052688: move.b d0, $10a2ca.l
pub fn i_052688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5268E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $05268E: move.w $7e(a6), d0
pub fn i_05268e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52692;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052692: move.b d0, $10a2cb.l
pub fn i_052692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52698;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052698: move.w $80(a6), d0
pub fn i_052698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5269C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05269C: move.b d0, $10a2cc.l
pub fn i_05269c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526A2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
// ===== SetTaskHandler_0526a2  ($0526A2, 8 B, kernel/task_handlers.c)
/// $0526A2: lea.l $526aa(pc), a1
pub fn i_0526a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x526AA;
    16
}
/// $0526A6: move.l a1, (a6)
pub fn i_0526a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0526A8: rts
pub fn i_0526a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearGlobalFlag_0526aa  ($0526AA, 14 B, kernel/scheduler_flag_setters.c)
/// $0526AA: clr.b $10a2c8.l
pub fn i_0526aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526B0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10A2C8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10A2C8, r2 as u8);
    16
}
/// $0526B0: jmp $518.l
pub fn i_0526b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== SpriteTable_Init256_0526b8  ($0526B8, 90 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0526B8: lea.l $1082c8.l, a1
pub fn i_0526b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $0526BE: move.w #$ff, d2
pub fn i_0526be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526C2;
    cpu.ir = 0x343C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0526C2: move.w #$8800, d0
pub fn i_0526c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0526C6: move.w d0, (a1)+
pub fn i_0526c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526C8;
    cpu.ir = 0x32C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $0526C8: clr.w (a1)+
pub fn i_0526c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526CA;
    cpu.ir = 0x4259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $0526CA: clr.l (a1)+
pub fn i_0526ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526CC;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526CC: clr.l (a1)+
pub fn i_0526cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526CE;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526CE: clr.l (a1)+
pub fn i_0526ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D0;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D0: clr.l (a1)+
pub fn i_0526d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D2;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D2: clr.l (a1)+
pub fn i_0526d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D4;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D4: clr.l (a1)+
pub fn i_0526d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D6;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D6: clr.l (a1)+
pub fn i_0526d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D8;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D8: dbra d2, $526c6
pub fn i_0526d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526DC;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x526DC; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x526DC; return 14; }
    cpu.pc = 0x526C6;
    10
}
/// $0526DC: lea.l $1081c2.l, a1
pub fn i_0526dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081C2;
    20
}
/// $0526E2: move.w #$30, d1
pub fn i_0526e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526E6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0526E6: move.w d1, $1082c2.l
pub fn i_0526e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526EC;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x1082C2, v1 as u16);
    16
}
/// $0526EC: subq.w #$1, d1
pub fn i_0526ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526EE;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0526EE: clr.b d2
pub fn i_0526ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526F0;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0526F0: move.b d2, (a1)+
pub fn i_0526f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526F2;
    cpu.ir = 0x12C2;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $0526F2: addq.b #$1, d2
pub fn i_0526f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526F4;
    cpu.ir = 0x5202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0526F4: dbra d1, $526f0
pub fn i_0526f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526F8;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x526F8; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x526F8; return 14; }
    cpu.pc = 0x526F0;
    10
}
/// $0526F8: moveq #$0, d0
pub fn i_0526f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526FA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0526FA: move.w d0, $1082c4.l
pub fn i_0526fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52700;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x1082C4, v1 as u16);
    16
}
/// $052700: move.w d0, $1082c6.l
pub fn i_052700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52706;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x1082C6, v1 as u16);
    16
}
/// $052706: move.b d0, $10a2c9.l
pub fn i_052706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5270C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C9, v1 as u8);
    16
}
/// $05270C: move.b d0, $10a2d1.l
pub fn i_05270c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52712;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
// ===== Pubcleaner_10A2Cx_052712  ($052712, 40 B, kernel/publish_cleaner_052712.s)
/// $052712: moveq #$0, d0
pub fn i_052712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52714;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052714: move.b d0, $10a2c8.l
pub fn i_052714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5271A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $05271A: move.b d0, $10a2ca.l
pub fn i_05271a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52720;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $052720: move.b d0, $10a2cb.l
pub fn i_052720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52726;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052726: move.b d0, $10a2cc.l
pub fn i_052726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5272C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $05272C: move.b d0, $10a2cf.l
pub fn i_05272c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52732;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $052732: move.b d0, $10a2d0.l
pub fn i_052732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52738;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D0, v1 as u8);
    16
}
/// $052738: rts
pub fn i_052738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5273A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_PhysicsStep_056acc  ($056ACC, 70 B, enemies/soldiers/helpers_056acc.s)
/// $056ACC: jsr $27f08.l
pub fn i_056acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56AD2);
    cpu.pc = 0x27F08;
    16
}
/// $056AD2: bcc.w $56ade
pub fn i_056ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56ADE; return 10; }
    8
}
/// $056AD6: move.b d3, $78(a6)
pub fn i_056ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ADA;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $056ADA: bra.w $56af4
pub fn i_056ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ADE;
    cpu.ir = 0x6000;
    cpu.pc = 0x56AF4;
    10
}
/// $056ADE: cmp.b $78(a6), d0
pub fn i_056ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AE2;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $056AE2: bne.w $56aee
pub fn i_056ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AE6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56AEE; return 10; }
    8
}
/// $056AE6: move.b d3, $78(a6)
pub fn i_056ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AEA;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $056AEA: bra.w $56af4
pub fn i_056aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x56AF4;
    10
}
/// $056AEE: move.b #$ff, $78(a6)
pub fn i_056aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $056AF4: tst.b $78(a6)
pub fn i_056af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AF8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $056AF8: beq.w $56b06
pub fn i_056af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56B06; return 10; }
    8
}
/// $056AFC: jsr $28292.l
pub fn i_056afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56B02);
    cpu.pc = 0x28292;
    16
}
/// $056B02: bra.w $56b10
pub fn i_056b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B06;
    cpu.ir = 0x6000;
    cpu.pc = 0x56B10;
    10
}
/// $056B06: jsr $28364.l
pub fn i_056b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56B0C);
    cpu.pc = 0x28364;
    16
}
/// $056B0C: scs.b $78(a6)
pub fn i_056b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B10;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $056B10: rts
pub fn i_056b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ApproxDist_056b12  ($056B12, 38 B, enemies/soldiers/helpers_056acc.s)
/// $056B12: ext.l d0
pub fn i_056b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B14;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B14: move.l d0, d2
pub fn i_056b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B16;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B16: swap d2
pub fn i_056b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B18;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B18: eor.w d2, d0
pub fn i_056b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B1A;
    cpu.ir = 0xB540;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056B1A: ext.l d1
pub fn i_056b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B1C;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B1C: move.l d1, d2
pub fn i_056b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B1E;
    cpu.ir = 0x2401;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B1E: swap d2
pub fn i_056b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B20;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B20: eor.w d2, d1
pub fn i_056b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B22;
    cpu.ir = 0xB541;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056B22: cmp.w d0, d1
pub fn i_056b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B24;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $056B24: bls.w $56b32
pub fn i_056b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B28;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x56B32; return 10; }
    8
}
/// $056B28: lsr.w #$1, d0
pub fn i_056b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B2A;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $056B2A: add.w d1, d0
pub fn i_056b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B2C;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056B2C: rts
pub fn i_056b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $056B32: lsr.w #$1, d1
pub fn i_056b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B34;
    cpu.ir = 0xE249;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $056B34: add.w d1, d0
pub fn i_056b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B36;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056B36: rts
pub fn i_056b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_FindNearestPlayer_056b38  ($056B38, 90 B, enemies/soldiers/helpers_056acc.s)
/// $056B38: clr.l -(a7)
pub fn i_056b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B3A;
    cpu.ir = 0x42A7;
    let ad1 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad1;
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    14
}
/// $056B3A: move.w #$ffff, -(a7)
pub fn i_056b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B3E;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $056B3E: moveq #$0, d0
pub fn i_056b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B40;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B40: jsr $5e3a2.l
pub fn i_056b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56B46);
    cpu.pc = 0x5E3A2;
    16
}
/// $056B46: bcc.w $56b62
pub fn i_056b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56B62; return 10; }
    8
}
/// $056B4A: move.w $22(a6), d0
pub fn i_056b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B4E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B4E: sub.w $22(a0), d0
pub fn i_056b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B52;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056B52: move.w $24(a6), d1
pub fn i_056b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B56;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B56: sub.w $24(a6), d1
pub fn i_056b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B5A;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056B5A: bsr.b $56b12
pub fn i_056b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B5C;
    cpu.ir = 0x61B6;
    cpu.push32(bus, 0x56B5C);
    cpu.pc = 0x56B12;
    18
}
/// $056B5C: move.l a0, $2(a7)
pub fn i_056b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B60;
    cpu.ir = 0x2F48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[7].wrapping_add(0x2);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056B60: move.w d0, (a7)
pub fn i_056b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B62;
    cpu.ir = 0x3E80;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $056B62: moveq #$1, d0
pub fn i_056b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B64;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B64: jsr $5e3a2.l
pub fn i_056b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56B6A);
    cpu.pc = 0x5E3A2;
    16
}
/// $056B6A: bcc.w $56b8c
pub fn i_056b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56B8C; return 10; }
    8
}
/// $056B6E: move.w $22(a6), d0
pub fn i_056b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B72;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B72: sub.w $22(a0), d0
pub fn i_056b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B76;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056B76: move.w $24(a6), d1
pub fn i_056b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B7A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B7A: sub.w $24(a0), d1
pub fn i_056b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B7E;
    cpu.ir = 0x9268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056B7E: bsr.b $56b12
pub fn i_056b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B80;
    cpu.ir = 0x6192;
    cpu.push32(bus, 0x56B80);
    cpu.pc = 0x56B12;
    18
}
/// $056B80: cmp.w (a7), d0
pub fn i_056b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B82;
    cpu.ir = 0xB057;
    let v2 = bus.read16(cpu.a[7]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    8
}
/// $056B82: bhi.w $56b8c
pub fn i_056b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B86;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x56B8C; return 10; }
    8
}
/// $056B86: move.w d0, (a7)
pub fn i_056b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B88;
    cpu.ir = 0x3E80;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $056B88: move.l a0, $2(a7)
pub fn i_056b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B8C;
    cpu.ir = 0x2F48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[7].wrapping_add(0x2);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056B8C: move.w (a7)+, d0
pub fn i_056b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B8E;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $056B8E: movea.l (a7)+, a0
pub fn i_056b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B90;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $056B90: rts
pub fn i_056b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Think_056b92  ($056B92, 646 B, enemies/soldiers/helpers_056acc.s)
/// $056B92: move.w $8c(a6), d0
pub fn i_056b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B96;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B96: beq.w $56ba0
pub fn i_056b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56BA0; return 10; }
    8
}
/// $056B9A: subq.w #$1, d0
pub fn i_056b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B9C;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056B9C: move.w d0, $8c(a6)
pub fn i_056b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BA0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056BA0: move.w $8e(a6), d0
pub fn i_056ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BA4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056BA4: beq.w $56bae
pub fn i_056ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56BAE; return 10; }
    8
}
/// $056BA8: subq.w #$1, d0
pub fn i_056ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BAA;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056BAA: move.w d0, $8e(a6)
pub fn i_056baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BAE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056BAE: bclr.b #$5, $72(a6)
pub fn i_056bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BB4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056BB4: bclr.b #$6, $72(a6)
pub fn i_056bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BBA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056BBA: btst.b #$5, $5a(a6)
pub fn i_056bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BC0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056BC0: beq.w $56bfc
pub fn i_056bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56BFC; return 10; }
    8
}
/// $056BC4: move.w $22(a6), d1
pub fn i_056bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BC8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056BC8: move.w $24(a6), d2
pub fn i_056bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BCC;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056BCC: btst.b #$0, $3a(a6)
pub fn i_056bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BD2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056BD2: bne.w $56bdc
pub fn i_056bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56BDC; return 10; }
    8
}
/// $056BD6: subq.w #$3, d1
pub fn i_056bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BD8;
    cpu.ir = 0x5741;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056BD8: bra.w $56bde
pub fn i_056bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x56BDE;
    10
}
/// $056BDC: addq.w #$3, d1
pub fn i_056bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BDE;
    cpu.ir = 0x5641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056BDE: addi.w #$18, d2
pub fn i_056bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BE2;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056BE2: jsr $280c6.l
pub fn i_056be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56BE8);
    cpu.pc = 0x280C6;
    16
}
/// $056BE8: bcc.w $56bf6
pub fn i_056be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56BF6; return 10; }
    8
}
/// $056BEC: bset.b #$5, $72(a6)
pub fn i_056bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BF2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056BF2: bra.w $56bfc
pub fn i_056bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x56BFC;
    10
}
/// $056BF6: bset.b #$6, $72(a6)
pub fn i_056bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BFC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056BFC: bsr.w $56b38
pub fn i_056bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C00;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x56C00);
    cpu.pc = 0x56B38;
    18
}
/// $056C00: move.w d0, $7e(a6)
pub fn i_056c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C04;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056C04: move.l a0, d0
pub fn i_056c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C06;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056C06: beq.w $56df0
pub fn i_056c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C0A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56DF0; return 10; }
    8
}
/// $056C0A: bclr.b #$0, $73(a6)
pub fn i_056c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C10;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C10: move.l a0, $7a(a6)
pub fn i_056c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C14;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056C14: move.w $22(a6), d0
pub fn i_056c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C18;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056C18: sub.w $22(a0), d0
pub fn i_056c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C1C;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056C1C: bmi.w $56c44
pub fn i_056c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C20;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56C44; return 10; }
    8
}
/// $056C20: bclr.b #$1, $72(a6)
pub fn i_056c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C26;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C26: btst.b #$0, $3a(a6)
pub fn i_056c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C2C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056C2C: bne.w $56c3a
pub fn i_056c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56C3A; return 10; }
    8
}
/// $056C30: bset.b #$0, $72(a6)
pub fn i_056c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C36;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C36: bra.w $56c40
pub fn i_056c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x56C40;
    10
}
/// $056C3A: bclr.b #$0, $72(a6)
pub fn i_056c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C40;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C40: bra.w $56c64
pub fn i_056c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C44;
    cpu.ir = 0x6000;
    cpu.pc = 0x56C64;
    10
}
/// $056C44: bset.b #$1, $72(a6)
pub fn i_056c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C4A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C4A: btst.b #$0, $3a(a6)
pub fn i_056c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C50;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056C50: bne.w $56c5e
pub fn i_056c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56C5E; return 10; }
    8
}
/// $056C54: bclr.b #$0, $72(a6)
pub fn i_056c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C5A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C5A: bra.w $56c64
pub fn i_056c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x56C64;
    10
}
/// $056C5E: bset.b #$0, $72(a6)
pub fn i_056c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C64;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C64: clr.l $88(a6)
pub fn i_056c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C68;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $056C68: clr.l $84(a6)
pub fn i_056c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C6C;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $056C6C: jsr $8f344.l
pub fn i_056c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56C72);
    cpu.pc = 0x8F344;
    16
}
/// $056C72: bcc.w $56ce0
pub fn i_056c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56CE0; return 10; }
    8
}
/// $056C76: tst.b d0
pub fn i_056c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C78;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $056C78: bpl.w $56c92
pub fn i_056c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C7C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56C92; return 10; }
    8
}
/// $056C7C: move.w #$400, $84(a6)
pub fn i_056c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056C82: bset.b #$2, $73(a6)
pub fn i_056c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C88;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C88: bclr.b #$3, $73(a6)
pub fn i_056c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C8E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C8E: bra.w $56cd6
pub fn i_056c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C92;
    cpu.ir = 0x6000;
    cpu.pc = 0x56CD6;
    10
}
/// $056C92: jsr $5e9b6.l
pub fn i_056c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56C98);
    cpu.pc = 0x5E9B6;
    16
}
/// $056C98: tst.b d0
pub fn i_056c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C9A;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $056C9A: bmi.w $56cb4
pub fn i_056c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C9E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56CB4; return 10; }
    8
}
/// $056C9E: move.w #$400, $86(a6)
pub fn i_056c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CA4: bclr.b #$2, $73(a6)
pub fn i_056ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CAA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056CAA: bset.b #$3, $73(a6)
pub fn i_056caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CB0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056CB0: bra.w $56cd6
pub fn i_056cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x56CD6;
    10
}
/// $056CB4: andi.w #$3ff, d0
pub fn i_056cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CB8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056CB8: move.w d0, $84(a6)
pub fn i_056cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CBC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CBC: jsr $5e9b6.l
pub fn i_056cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56CC2);
    cpu.pc = 0x5E9B6;
    16
}
/// $056CC2: andi.w #$3ff, d0
pub fn i_056cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CC6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056CC6: move.w d0, $86(a6)
pub fn i_056cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CCA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CCA: move.w #$80, $88(a6)
pub fn i_056cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CD0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CD0: move.w #$80, $8a(a6)
pub fn i_056cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CD6: bset.b #$1, $73(a6)
pub fn i_056cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CDC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056CDC: bra.w $56ce6
pub fn i_056cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x56CE6;
    10
}
/// $056CE0: andi.b #$f1, $73(a6)
pub fn i_056ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CE6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056CE6: move.b $98(a6), d0
pub fn i_056ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CEA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056CEA: beq.w $56d16
pub fn i_056cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56D16; return 10; }
    8
}
/// $056CEE: cmpi.b #$1, d0
pub fn i_056cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CF2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $056CF2: beq.w $56d0c
pub fn i_056cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56D0C; return 10; }
    8
}
/// $056CF6: cmpi.b #$2, d0
pub fn i_056cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CFA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $056CFA: beq.w $56d20
pub fn i_056cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56D20; return 10; }
    8
}
/// $056CFE: jsr $5e9b6.l
pub fn i_056cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56D04);
    cpu.pc = 0x5E9B6;
    16
}
/// $056D04: btst.b #$0, d0
pub fn i_056d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D08;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $056D08: beq.w $56d16
pub fn i_056d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D0C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56D16; return 10; }
    8
}
/// $056D0C: ori.b #$30, $13(a6)
pub fn i_056d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D12;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x30) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D12: bra.w $56d44
pub fn i_056d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D16;
    cpu.ir = 0x6000;
    cpu.pc = 0x56D44;
    10
}
/// $056D16: andi.b #$cf, $13(a6)
pub fn i_056d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D1C;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xCF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D1C: bra.w $56d44
pub fn i_056d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D20;
    cpu.ir = 0x6000;
    cpu.pc = 0x56D44;
    10
}
/// $056D20: move.b $70(a6), d0
pub fn i_056d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D24;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056D24: bmi.w $56d38
pub fn i_056d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D28;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56D38; return 10; }
    8
}
/// $056D28: bset.b #$4, $13(a6)
pub fn i_056d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D2E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D2E: bclr.b #$5, $13(a6)
pub fn i_056d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D34;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D34: bra.w $56d44
pub fn i_056d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D38;
    cpu.ir = 0x6000;
    cpu.pc = 0x56D44;
    10
}
/// $056D38: bset.b #$5, $13(a6)
pub fn i_056d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D3E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D3E: bclr.b #$4, $13(a6)
pub fn i_056d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D44;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D44: move.w $7e(a6), d0
pub fn i_056d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D48;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056D48: move.w d0, d1
pub fn i_056d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D4A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056D4A: move.w $90(a6), d2
pub fn i_056d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D4E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056D4E: move.b $90(a6), d3
pub fn i_056d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D52;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056D52: andi.w #$1f, d3
pub fn i_056d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D56;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D56: andi.w #$1f, d2
pub fn i_056d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D5A;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D5A: add.w d3, d2
pub fn i_056d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D5C;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056D5C: addi.w #$10, d2
pub fn i_056d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D60;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D60: sub.w d2, d0
pub fn i_056d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D62;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056D62: bpl.w $56d68
pub fn i_056d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D66;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56D68; return 10; }
    8
}
/// $056D66: clr.w d0
pub fn i_056d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D68;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056D68: lsl.w #$2, d0
pub fn i_056d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D6A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $056D6A: cmpi.w #$100, d0
pub fn i_056d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D6E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $056D6E: bcs.w $56d76
pub fn i_056d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D72;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56D76; return 10; }
    8
}
/// $056D72: move.w #$100, d0
pub fn i_056d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D76;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056D76: neg.w d0
pub fn i_056d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D78;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056D78: addi.w #$100, d0
pub fn i_056d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D7C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D7C: move.w d0, $80(a6)
pub fn i_056d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D80;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056D80: moveq #$0, d0
pub fn i_056d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D82;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056D82: move.w d1, d0
pub fn i_056d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D84;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056D84: move.w $90(a6), d2
pub fn i_056d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D88;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056D88: move.b $90(a6), d3
pub fn i_056d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D8C;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056D8C: andi.w #$1f, d3
pub fn i_056d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D90;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D90: andi.w #$1f, d2
pub fn i_056d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D94;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D94: add.w d3, d2
pub fn i_056d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D96;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056D96: addi.w #$10, d2
pub fn i_056d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D9A;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D9A: sub.w d2, d0
pub fn i_056d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D9C;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056D9C: bpl.w $56da2
pub fn i_056d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DA0;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56DA2; return 10; }
    8
}
/// $056DA0: clr.l d0
pub fn i_056da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DA2;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $056DA2: move.b $9c(a6), d7
pub fn i_056da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DA6;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056DA6: andi.l #$7, d7
pub fn i_056da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DAC;
    cpu.ir = 0x287;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = (v1 & 0x7) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[7] = r2;
    8
}
/// $056DAC: beq.w $56db2
pub fn i_056dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56DB2; return 10; }
    8
}
/// $056DB0: lsl.l d7, d0
pub fn i_056db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DB2;
    cpu.ir = 0xEFA8;
    let cnt = cpu.d[7] & 0x3F;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 1, true);
    cpu.d[0] = r;
    6 + 2 * cnt
}
/// $056DB2: lsl.l #$1, d0
pub fn i_056db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DB4;
    cpu.ir = 0xE388;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 1, true);
    cpu.d[0] = r;
    8
}
/// $056DB4: cmpi.l #$100, d0
pub fn i_056db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DBA;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Long, false);
    8
}
/// $056DBA: bcs.w $56dc2
pub fn i_056dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DBE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56DC2; return 10; }
    8
}
/// $056DBE: move.w #$100, d0
pub fn i_056dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DC2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056DC2: move.w d0, $82(a6)
pub fn i_056dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DC6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056DC6: move.w $22(a6), d0
pub fn i_056dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DCA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056DCA: cmpi.w #$30, d0
pub fn i_056dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DCE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $056DCE: bmi.w $56ddc
pub fn i_056dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DD2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56DDC; return 10; }
    8
}
/// $056DD2: cmpi.w #$120, d0
pub fn i_056dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DD6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $056DD6: bpl.w $56ddc
pub fn i_056dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DDA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56DDC; return 10; }
    8
}
/// $056DDA: rts
pub fn i_056dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $056DDC: move.w #$100, $82(a6)
pub fn i_056ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056DE2: clr.w $80(a6)
pub fn i_056de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DE6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056DE6: clr.w $84(a6)
pub fn i_056de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DEA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056DEA: clr.w $86(a6)
pub fn i_056dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DEE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056DEE: rts
pub fn i_056dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $056DF0: bset.b #$0, $73(a6)
pub fn i_056df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DF6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056DF6: clr.l $7a(a6)
pub fn i_056df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DFA;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $056DFA: bset.b #$0, $72(a6)
pub fn i_056dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E00;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056E00: move.w #$20, d0
pub fn i_056e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056E04: move.w d0, $80(a6)
pub fn i_056e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E08;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E08: move.w d0, $82(a6)
pub fn i_056e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E0C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E0C: move.w d0, $86(a6)
pub fn i_056e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E10;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E10: move.w d0, $84(a6)
pub fn i_056e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E14;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E14: move.w d0, $88(a6)
pub fn i_056e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E18;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskW_056e18  ($056E18, 6 B, kernel/task_setters_w.c)
/// $056E18: move.w d0, $8a(a6)
pub fn i_056e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E1C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E1C: rts
pub fn i_056e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_DespawnIfOffscreen_056e1e  ($056E1E, 24 B, enemies/soldiers/helpers_056acc.s)
/// $056E1E: lea.l $2b71d4.l, a0
pub fn i_056e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B71D4;
    20
}
/// $056E24: jsr $5dd56.l
pub fn i_056e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56E2A);
    cpu.pc = 0x5DD56;
    16
}
/// $056E2A: bcc.w $56e34
pub fn i_056e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56E34; return 10; }
    8
}
/// $056E2E: jmp $518.l
pub fn i_056e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E34;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $056E34: rts
pub fn i_056e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_LeaveTimerExpired_056e36  ($056E36, 8 B, enemies/soldiers/helpers_056acc.s)
/// $056E36: tst.w $8e(a6)
pub fn i_056e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E3A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $056E3A: beq.w $56e44
pub fn i_056e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56E44; return 10; }
    8
}
// ===== ClearC_056e3e  ($056E3E, 6 B, thunks/ccr_helpers.c)
/// $056E3E: andi.b #$fe, ccr
pub fn i_056e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E42;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056E42: rts
pub fn i_056e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_056e44  ($056E44, 6 B, thunks/ccr_helpers.c)
/// $056E44: ori.b #$1, ccr
pub fn i_056e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E48;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056E48: rts
pub fn i_056e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_PickGrabAnchor_056e4a  ($056E4A, 124 B, enemies/soldiers/helpers_056acc.s)
/// $056E4A: move.b $74(a6), d0
pub fn i_056e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E4E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056E4E: andi.b #$7, d0
pub fn i_056e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E52;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $056E52: beq.w $56ecc
pub fn i_056e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56ECC; return 10; }
    8
}
/// $056E56: btst.b #$0, $72(a6)
pub fn i_056e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E5C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056E5C: beq.w $56ecc
pub fn i_056e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56ECC; return 10; }
    8
}
/// $056E60: move.l $7a(a6), d0
pub fn i_056e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E64;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056E64: beq.w $56ecc
pub fn i_056e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56ECC; return 10; }
    8
}
/// $056E68: movea.l d0, a0
pub fn i_056e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E6A;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $056E6A: jsr $8f7ce.l
pub fn i_056e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56E70);
    cpu.pc = 0x8F7CE;
    16
}
/// $056E70: not.b d0
pub fn i_056e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E72;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $056E72: move.b $74(a6), d1
pub fn i_056e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E76;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056E76: andi.b #$7, d1
pub fn i_056e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E7A;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $056E7A: and.b d1, d0
pub fn i_056e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E7C;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $056E7C: beq.w $56ecc
pub fn i_056e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56ECC; return 10; }
    8
}
/// $056E80: clr.w d1
pub fn i_056e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E82;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056E82: btst.b #$0, d0
pub fn i_056e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E86;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $056E86: beq.w $56e90
pub fn i_056e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56E90; return 10; }
    8
}
/// $056E8A: move.w #$0, -(a7)
pub fn i_056e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E8E;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $056E8E: addq.w #$1, d1
pub fn i_056e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E90;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056E90: btst.b #$1, d0
pub fn i_056e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E94;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $056E94: beq.w $56e9e
pub fn i_056e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56E9E; return 10; }
    8
}
/// $056E98: move.w #$1, -(a7)
pub fn i_056e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E9C;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $056E9C: addq.w #$1, d1
pub fn i_056e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E9E;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056E9E: btst.b #$2, d0
pub fn i_056e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EA2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $056EA2: beq.w $56eac
pub fn i_056ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56EAC; return 10; }
    8
}
/// $056EA6: move.w #$2, -(a7)
pub fn i_056ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EAA;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $056EAA: addq.w #$1, d1
pub fn i_056eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EAC;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056EAC: move.w d1, d0
pub fn i_056eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EAE;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056EAE: movem.w d1, -(a7)
pub fn i_056eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EB2;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $056EB2: jsr $5e9e4.l
pub fn i_056eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56EB8);
    cpu.pc = 0x5E9E4;
    16
}
/// $056EB8: movem.w (a7)+, d1
pub fn i_056eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EBC;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $056EBC: add.w d0, d0
pub fn i_056ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EBE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056EBE: move.w (a7, d0.w), d0
pub fn i_056ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EC2;
    cpu.ir = 0x3037;
    let ad2 = cpu.a[7].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $056EC2: add.w d1, d1
pub fn i_056ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EC4;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056EC4: adda.w d1, a7
pub fn i_056ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EC6;
    cpu.ir = 0xDEC1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[7] = cpu.a[7].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
// ===== SetC_056ec6  ($056EC6, 6 B, thunks/ccr_helpers.c)
/// $056EC6: ori.b #$1, ccr
pub fn i_056ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ECA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056ECA: rts
pub fn i_056eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ECC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_056ecc  ($056ECC, 6 B, thunks/ccr_helpers.c)
/// $056ECC: andi.b #$fe, ccr
pub fn i_056ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ED0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056ED0: rts
pub fn i_056ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ED2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_TestPlayerInSlug_056eda  ($056EDA, 42 B, enemies/soldiers/helpers_056acc.s)
/// $056EDA: move.b $74(a6), d0
pub fn i_056eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EDE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056EDE: andi.b #$7, d0
pub fn i_056ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EE2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $056EE2: beq.w $56f0a
pub fn i_056ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EE6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F0A; return 10; }
    8
}
/// $056EE6: btst.b #$0, $72(a6)
pub fn i_056ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EEC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056EEC: beq.w $56f0a
pub fn i_056eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F0A; return 10; }
    8
}
/// $056EF0: move.l $7a(a6), d0
pub fn i_056ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EF4;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056EF4: beq.w $56f0a
pub fn i_056ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EF8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F0A; return 10; }
    8
}
/// $056EF8: movea.l d0, a0
pub fn i_056ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EFA;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $056EFA: jsr $2ac0e.l
pub fn i_056efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56F00);
    cpu.pc = 0x2AC0E;
    16
}
/// $056F00: bcs.w $56f0a
pub fn i_056f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F04;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56F0A; return 10; }
    8
}
// ===== SetC_056f04  ($056F04, 6 B, thunks/ccr_helpers.c)
/// $056F04: ori.b #$1, ccr
pub fn i_056f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F08;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056F08: rts
pub fn i_056f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F0A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_056f0a  ($056F0A, 6 B, thunks/ccr_helpers.c)
/// $056F0A: andi.b #$fe, ccr
pub fn i_056f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F0E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056F0E: rts
pub fn i_056f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SetVelXByFacing_056f8a  ($056F8A, 16 B, enemies/soldiers/helpers_056acc.s)
/// $056F8A: move.w $36(a6), d0
pub fn i_056f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F8E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056F8E: btst.b #$0, $3a(a6)
pub fn i_056f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F94;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056F94: bne.w $56f9a
pub fn i_056f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56F9A; return 10; }
    8
}
/// $056F98: neg.w d0
pub fn i_056f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F9A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_056f9a  ($056F9A, 6 B, kernel/task_setters_w.c)
/// $056F9A: move.w d0, $28(a6)
pub fn i_056f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056F9E: rts
pub fn i_056f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_TestSurrender_056fa0  ($056FA0, 64 B, enemies/soldiers/helpers_056acc.s)
/// $056FA0: btst.b #$3, $74(a6)
pub fn i_056fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FA6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056FA6: beq.w $56fe6
pub fn i_056fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FAA: move.w $22(a6), d0
pub fn i_056faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FAE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056FAE: cmpi.w #$30, d0
pub fn i_056fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FB2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $056FB2: bmi.w $56fe6
pub fn i_056fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FB6;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FB6: cmpi.w #$120, d0
pub fn i_056fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FBA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $056FBA: bpl.w $56fe6
pub fn i_056fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FBE;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FBE: tst.w $8c(a6)
pub fn i_056fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FC2;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $056FC2: bne.w $56fe6
pub fn i_056fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FC6: btst.b #$0, $72(a6)
pub fn i_056fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FCC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056FCC: beq.w $56fe6
pub fn i_056fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FD0: jsr $8f2a0.l
pub fn i_056fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56FD6);
    cpu.pc = 0x8F2A0;
    16
}
/// $056FD6: bcs.w $56fe6
pub fn i_056fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FDA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FDA: move.b $9a(a6), $8d(a6)
pub fn i_056fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FE0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8D);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetC_056fe0  ($056FE0, 6 B, thunks/ccr_helpers.c)
/// $056FE0: ori.b #$1, ccr
pub fn i_056fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FE4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056FE4: rts
pub fn i_056fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_056fe6  ($056FE6, 6 B, thunks/ccr_helpers.c)
/// $056FE6: andi.b #$fe, ccr
pub fn i_056fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FEA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056FEA: rts
pub fn i_056fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ProbeWalkEdge_056fec  ($056FEC, 54 B, enemies/soldiers/helpers_056acc.s)
/// $056FEC: btst.b #$0, $3a(a6)
pub fn i_056fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FF2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056FF2: seq.b d1
pub fn i_056ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FF4;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $056FF4: move.b $28(a6), d2
pub fn i_056ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FF8;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056FF8: smi.b d2
pub fn i_056ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FFA;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $056FFA: eor.b d2, d1
pub fn i_056ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FFC;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $056FFC: bne.w $57042
pub fn i_056ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57000;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57042; return 10; }
    8
}
/// $057000: jsr $770cc.l
pub fn i_057000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57006);
    cpu.pc = 0x770CC;
    16
}
/// $057006: tst.w d0
pub fn i_057006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57008;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $057008: bmi.w $57042
pub fn i_057008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5700C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x57042; return 10; }
    8
}
/// $05700C: move.b d0, $77(a6)
pub fn i_05700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57010;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057010: btst.b #$4, $13(a6)
pub fn i_057010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57016;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057016: sne.b d2
pub fn i_057016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57018;
    cpu.ir = 0x56C2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $057018: and.b d1, d2
pub fn i_057018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5701A;
    cpu.ir = 0xC401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05701A: andi.b #$1, d2
pub fn i_05701a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5701E;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05701E: beq.w $5702a
pub fn i_05701e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57022;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5702A; return 10; }
    8
}
// ===== SetTaskHandler_057022  ($057022, 8 B, kernel/task_handlers.c)
/// $057022: lea.l $58b1e(pc), a1
pub fn i_057022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57026;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58B1E;
    16
}
/// $057026: move.l a1, (a6)
pub fn i_057026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57028;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057028: rts
pub fn i_057028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5702A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ProbeWalkEdge_Bit5_05702a  ($05702A, 18 B, enemies/soldiers/helpers_056acc.s)
/// $05702A: btst.b #$5, $13(a6)
pub fn i_05702a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57030;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057030: sne.b d2
pub fn i_057030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57032;
    cpu.ir = 0x56C2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $057032: and.b d1, d2
pub fn i_057032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57034;
    cpu.ir = 0xC401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $057034: andi.b #$2, d2
pub fn i_057034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57038;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $057038: beq.w $57042
pub fn i_057038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5703C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57042; return 10; }
    8
}
// ===== SetTaskHandler_05703c  ($05703C, 8 B, kernel/task_handlers.c)
/// $05703C: lea.l $58c8e(pc), a1
pub fn i_05703c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57040;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58C8E;
    16
}
/// $057040: move.l a1, (a6)
pub fn i_057040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57042;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057042: rts
pub fn i_057042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57044;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_InitCommon_0570a8  ($0570A8, 204 B, enemies/soldiers/helpers_056acc.s)
/// $0570A8: jsr $27f60.l
pub fn i_0570a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x570AE);
    cpu.pc = 0x27F60;
    16
}
/// $0570AE: scc.b $78(a6)
pub fn i_0570ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570B2;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $0570B2: bset.b #$3, $6b(a6)
pub fn i_0570b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570B8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0570B8: lea.l $2b74ec.l, a0
pub fn i_0570b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B74EC;
    20
}
/// $0570BE: jsr $799de.l
pub fn i_0570be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x570C4);
    cpu.pc = 0x799DE;
    16
}
/// $0570C4: clr.w d1
pub fn i_0570c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570C6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0570C6: move.b $99(a6), d1
pub fn i_0570c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570CA;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0570CA: bne.w $570d0
pub fn i_0570ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x570D0; return 10; }
    8
}
/// $0570CE: moveq #$ff, d0
pub fn i_0570ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570D0;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0570D0: mulu.w d1, d0
pub fn i_0570d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570D2;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0570D2: lsr.l #$8, d0
pub fn i_0570d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570D4;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $0570D4: move.w d0, $8c(a6)
pub fn i_0570d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570D8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0570D8: lea.l $2b746a.l, a0
pub fn i_0570d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B746A;
    20
}
/// $0570DE: jsr $799de.l
pub fn i_0570de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x570E4);
    cpu.pc = 0x799DE;
    16
}
/// $0570E4: clr.w d1
pub fn i_0570e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570E6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0570E6: move.b $9a(a6), d1
pub fn i_0570e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570EA;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0570EA: bne.w $570f2
pub fn i_0570ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x570F2; return 10; }
    8
}
/// $0570EE: move.b #$1e, d1
pub fn i_0570ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570F2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0570F2: mulu.w d1, d0
pub fn i_0570f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570F4;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0570F4: lsr.l #$8, d0
pub fn i_0570f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570F6;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $0570F6: cmpi.w #$100, d0
pub fn i_0570f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570FA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $0570FA: bcs.w $57102
pub fn i_0570fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570FE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57102; return 10; }
    8
}
/// $0570FE: move.b #$ff, d0
pub fn i_0570fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57102;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $057102: move.b d0, $9a(a6)
pub fn i_057102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57106;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057106: moveq #$0, d0
pub fn i_057106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57108;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x05162C, i_05162c),
    (0x051634, i_051634),
    (0x05163A, i_05163a),
    (0x051640, i_051640),
    (0x051648, i_051648),
    (0x05164A, i_05164a),
    (0x051650, i_051650),
    (0x051656, i_051656),
    (0x051658, i_051658),
    (0x05165A, i_05165a),
    (0x051660, i_051660),
    (0x051666, i_051666),
    (0x05166A, i_05166a),
    (0x051670, i_051670),
    (0x051674, i_051674),
    (0x05167C, i_05167c),
    (0x051680, i_051680),
    (0x051688, i_051688),
    (0x05168E, i_05168e),
    (0x051694, i_051694),
    (0x051696, i_051696),
    (0x05169C, i_05169c),
    (0x0516A2, i_0516a2),
    (0x05170C, i_05170c),
    (0x051712, i_051712),
    (0x051718, i_051718),
    (0x05171C, i_05171c),
    (0x051722, i_051722),
    (0x051726, i_051726),
    (0x05172C, i_05172c),
    (0x051730, i_051730),
    (0x051736, i_051736),
    (0x05173A, i_05173a),
    (0x05173C, i_05173c),
    (0x05173E, i_05173e),
    (0x051740, i_051740),
    (0x051744, i_051744),
    (0x051748, i_051748),
    (0x05174A, i_05174a),
    (0x0517AA, i_0517aa),
    (0x0517AE, i_0517ae),
    (0x0517B4, i_0517b4),
    (0x0517B8, i_0517b8),
    (0x0517BE, i_0517be),
    (0x0517C4, i_0517c4),
    (0x0517C6, i_0517c6),
    (0x0517CC, i_0517cc),
    (0x0517D0, i_0517d0),
    (0x0517D6, i_0517d6),
    (0x0517DC, i_0517dc),
    (0x0517DE, i_0517de),
    (0x0517E4, i_0517e4),
    (0x0517E8, i_0517e8),
    (0x0517EC, i_0517ec),
    (0x0517F0, i_0517f0),
    (0x0517F2, i_0517f2),
    (0x0517F4, i_0517f4),
    (0x0517F8, i_0517f8),
    (0x0517FA, i_0517fa),
    (0x0517FC, i_0517fc),
    (0x0517FE, i_0517fe),
    (0x051804, i_051804),
    (0x05180A, i_05180a),
    (0x05180C, i_05180c),
    (0x05180E, i_05180e),
    (0x051812, i_051812),
    (0x051828, i_051828),
    (0x05182A, i_05182a),
    (0x05182C, i_05182c),
    (0x05182E, i_05182e),
    (0x051832, i_051832),
    (0x051834, i_051834),
    (0x051836, i_051836),
    (0x051838, i_051838),
    (0x05183A, i_05183a),
    (0x05183C, i_05183c),
    (0x051840, i_051840),
    (0x051842, i_051842),
    (0x051844, i_051844),
    (0x051846, i_051846),
    (0x051848, i_051848),
    (0x05184A, i_05184a),
    (0x05184E, i_05184e),
    (0x051850, i_051850),
    (0x051852, i_051852),
    (0x051854, i_051854),
    (0x051856, i_051856),
    (0x051858, i_051858),
    (0x05185C, i_05185c),
    (0x05185E, i_05185e),
    (0x051860, i_051860),
    (0x051862, i_051862),
    (0x051864, i_051864),
    (0x051866, i_051866),
    (0x051868, i_051868),
    (0x05186A, i_05186a),
    (0x05186C, i_05186c),
    (0x05186E, i_05186e),
    (0x051870, i_051870),
    (0x051872, i_051872),
    (0x051874, i_051874),
    (0x051876, i_051876),
    (0x051878, i_051878),
    (0x05187A, i_05187a),
    (0x05187C, i_05187c),
    (0x05187E, i_05187e),
    (0x051880, i_051880),
    (0x051882, i_051882),
    (0x051884, i_051884),
    (0x051886, i_051886),
    (0x051888, i_051888),
    (0x05188A, i_05188a),
    (0x051914, i_051914),
    (0x05191A, i_05191a),
    (0x05191E, i_05191e),
    (0x051924, i_051924),
    (0x051928, i_051928),
    (0x05192E, i_05192e),
    (0x051932, i_051932),
    (0x051936, i_051936),
    (0x05193A, i_05193a),
    (0x05193E, i_05193e),
    (0x051942, i_051942),
    (0x051946, i_051946),
    (0x05194C, i_05194c),
    (0x051950, i_051950),
    (0x051954, i_051954),
    (0x051956, i_051956),
    (0x05195A, i_05195a),
    (0x051960, i_051960),
    (0x051964, i_051964),
    (0x05196A, i_05196a),
    (0x051970, i_051970),
    (0x051972, i_051972),
    (0x051976, i_051976),
    (0x051978, i_051978),
    (0x05197C, i_05197c),
    (0x051980, i_051980),
    (0x051982, i_051982),
    (0x051984, i_051984),
    (0x051986, i_051986),
    (0x051988, i_051988),
    (0x05198C, i_05198c),
    (0x051990, i_051990),
    (0x051994, i_051994),
    (0x051998, i_051998),
    (0x051A10, i_051a10),
    (0x051A14, i_051a14),
    (0x051A16, i_051a16),
    (0x051A18, i_051a18),
    (0x051A1A, i_051a1a),
    (0x051A1C, i_051a1c),
    (0x051A20, i_051a20),
    (0x051A26, i_051a26),
    (0x051A44, i_051a44),
    (0x051A48, i_051a48),
    (0x051A4C, i_051a4c),
    (0x051A50, i_051a50),
    (0x051A54, i_051a54),
    (0x051A5A, i_051a5a),
    (0x051A5E, i_051a5e),
    (0x051A64, i_051a64),
    (0x051A6A, i_051a6a),
    (0x051A70, i_051a70),
    (0x051A72, i_051a72),
    (0x051A76, i_051a76),
    (0x051A78, i_051a78),
    (0x051A7A, i_051a7a),
    (0x051A7C, i_051a7c),
    (0x051A80, i_051a80),
    (0x051A84, i_051a84),
    (0x051A86, i_051a86),
    (0x051A8A, i_051a8a),
    (0x051A8C, i_051a8c),
    (0x051A90, i_051a90),
    (0x051A94, i_051a94),
    (0x051A98, i_051a98),
    (0x051A9C, i_051a9c),
    (0x051AA2, i_051aa2),
    (0x051AA4, i_051aa4),
    (0x051AA8, i_051aa8),
    (0x051AAE, i_051aae),
    (0x051AB2, i_051ab2),
    (0x051AB6, i_051ab6),
    (0x051AB8, i_051ab8),
    (0x051ABC, i_051abc),
    (0x051ABE, i_051abe),
    (0x051AC2, i_051ac2),
    (0x051AC6, i_051ac6),
    (0x051ACA, i_051aca),
    (0x051AD0, i_051ad0),
    (0x051AD6, i_051ad6),
    (0x051ADC, i_051adc),
    (0x051AE0, i_051ae0),
    (0x051AE4, i_051ae4),
    (0x051AE8, i_051ae8),
    (0x051AEC, i_051aec),
    (0x051AF0, i_051af0),
    (0x051AF4, i_051af4),
    (0x051AF8, i_051af8),
    (0x051AFA, i_051afa),
    (0x051AFE, i_051afe),
    (0x051B02, i_051b02),
    (0x051B06, i_051b06),
    (0x051B0A, i_051b0a),
    (0x051B0E, i_051b0e),
    (0x051B12, i_051b12),
    (0x051B14, i_051b14),
    (0x051B16, i_051b16),
    (0x051B1A, i_051b1a),
    (0x051B1C, i_051b1c),
    (0x051B20, i_051b20),
    (0x051B24, i_051b24),
    (0x051B28, i_051b28),
    (0x051B2A, i_051b2a),
    (0x051B2E, i_051b2e),
    (0x051B32, i_051b32),
    (0x051B36, i_051b36),
    (0x051B38, i_051b38),
    (0x051B3C, i_051b3c),
    (0x051B3E, i_051b3e),
    (0x051B42, i_051b42),
    (0x051B46, i_051b46),
    (0x051B48, i_051b48),
    (0x051B4A, i_051b4a),
    (0x051B4E, i_051b4e),
    (0x051B52, i_051b52),
    (0x051B56, i_051b56),
    (0x051B5A, i_051b5a),
    (0x051B5C, i_051b5c),
    (0x051B5E, i_051b5e),
    (0x051B62, i_051b62),
    (0x051B66, i_051b66),
    (0x051B6A, i_051b6a),
    (0x051B6E, i_051b6e),
    (0x051B72, i_051b72),
    (0x051B76, i_051b76),
    (0x051B7A, i_051b7a),
    (0x051B7C, i_051b7c),
    (0x051B7E, i_051b7e),
    (0x051B80, i_051b80),
    (0x051B84, i_051b84),
    (0x051B86, i_051b86),
    (0x051B8A, i_051b8a),
    (0x051B8C, i_051b8c),
    (0x051B8E, i_051b8e),
    (0x051B90, i_051b90),
    (0x051B92, i_051b92),
    (0x051B96, i_051b96),
    (0x051B98, i_051b98),
    (0x051B9C, i_051b9c),
    (0x051B9E, i_051b9e),
    (0x051BA0, i_051ba0),
    (0x051BA2, i_051ba2),
    (0x051BA6, i_051ba6),
    (0x051BA8, i_051ba8),
    (0x051BAC, i_051bac),
    (0x051BB0, i_051bb0),
    (0x051BB2, i_051bb2),
    (0x051BB6, i_051bb6),
    (0x051BB8, i_051bb8),
    (0x051BBC, i_051bbc),
    (0x051BBE, i_051bbe),
    (0x051BC2, i_051bc2),
    (0x051BC6, i_051bc6),
    (0x051BCA, i_051bca),
    (0x051BCE, i_051bce),
    (0x051BD0, i_051bd0),
    (0x051BD2, i_051bd2),
    (0x051BD6, i_051bd6),
    (0x051BDC, i_051bdc),
    (0x051BE0, i_051be0),
    (0x051BE4, i_051be4),
    (0x051BE6, i_051be6),
    (0x051BEC, i_051bec),
    (0x051BF0, i_051bf0),
    (0x051BF4, i_051bf4),
    (0x051BF6, i_051bf6),
    (0x051BFA, i_051bfa),
    (0x051BFE, i_051bfe),
    (0x051C02, i_051c02),
    (0x051C06, i_051c06),
    (0x051C08, i_051c08),
    (0x051C0C, i_051c0c),
    (0x051C10, i_051c10),
    (0x051C14, i_051c14),
    (0x051C18, i_051c18),
    (0x051C1A, i_051c1a),
    (0x051C1E, i_051c1e),
    (0x051C20, i_051c20),
    (0x051C24, i_051c24),
    (0x051C28, i_051c28),
    (0x051C2A, i_051c2a),
    (0x051C2C, i_051c2c),
    (0x051C30, i_051c30),
    (0x051C34, i_051c34),
    (0x051C38, i_051c38),
    (0x051C3C, i_051c3c),
    (0x051C40, i_051c40),
    (0x051C44, i_051c44),
    (0x051C46, i_051c46),
    (0x051C4A, i_051c4a),
    (0x051C4E, i_051c4e),
    (0x051C52, i_051c52),
    (0x051C54, i_051c54),
    (0x051C58, i_051c58),
    (0x051C5C, i_051c5c),
    (0x051C62, i_051c62),
    (0x051C68, i_051c68),
    (0x051C6A, i_051c6a),
    (0x051C6E, i_051c6e),
    (0x051C72, i_051c72),
    (0x051C76, i_051c76),
    (0x051C7A, i_051c7a),
    (0x051C7E, i_051c7e),
    (0x051C80, i_051c80),
    (0x051C82, i_051c82),
    (0x051C86, i_051c86),
    (0x051C8A, i_051c8a),
    (0x051C8E, i_051c8e),
    (0x051C92, i_051c92),
    (0x051C94, i_051c94),
    (0x051C98, i_051c98),
    (0x051C9A, i_051c9a),
    (0x051C9E, i_051c9e),
    (0x051CA2, i_051ca2),
    (0x051CA6, i_051ca6),
    (0x051CA8, i_051ca8),
    (0x051CAC, i_051cac),
    (0x051CB0, i_051cb0),
    (0x051CB2, i_051cb2),
    (0x051CB4, i_051cb4),
    (0x051CB6, i_051cb6),
    (0x051CB8, i_051cb8),
    (0x051CBC, i_051cbc),
    (0x051CC0, i_051cc0),
    (0x051CC4, i_051cc4),
    (0x051CC8, i_051cc8),
    (0x051CCC, i_051ccc),
    (0x051CCE, i_051cce),
    (0x051CD2, i_051cd2),
    (0x051CD6, i_051cd6),
    (0x051CDC, i_051cdc),
    (0x051CE2, i_051ce2),
    (0x051CE6, i_051ce6),
    (0x051CEA, i_051cea),
    (0x051CEE, i_051cee),
    (0x051CF0, i_051cf0),
    (0x051CF4, i_051cf4),
    (0x051CF6, i_051cf6),
    (0x051CFA, i_051cfa),
    (0x051CFE, i_051cfe),
    (0x051D02, i_051d02),
    (0x051D06, i_051d06),
    (0x051D08, i_051d08),
    (0x051D0C, i_051d0c),
    (0x051D0E, i_051d0e),
    (0x051D12, i_051d12),
    (0x051D16, i_051d16),
    (0x051D1A, i_051d1a),
    (0x051D1E, i_051d1e),
    (0x051D22, i_051d22),
    (0x051D26, i_051d26),
    (0x051D28, i_051d28),
    (0x051D2A, i_051d2a),
    (0x051D2C, i_051d2c),
    (0x051D30, i_051d30),
    (0x051D32, i_051d32),
    (0x051D36, i_051d36),
    (0x051D3A, i_051d3a),
    (0x051D3E, i_051d3e),
    (0x051D42, i_051d42),
    (0x051D44, i_051d44),
    (0x051D48, i_051d48),
    (0x051D4C, i_051d4c),
    (0x051D50, i_051d50),
    (0x051D52, i_051d52),
    (0x051D56, i_051d56),
    (0x051D5A, i_051d5a),
    (0x051D60, i_051d60),
    (0x051D66, i_051d66),
    (0x051D68, i_051d68),
    (0x051D6C, i_051d6c),
    (0x051D70, i_051d70),
    (0x051D74, i_051d74),
    (0x051D78, i_051d78),
    (0x051D7C, i_051d7c),
    (0x051D7E, i_051d7e),
    (0x051D82, i_051d82),
    (0x051D84, i_051d84),
    (0x051D86, i_051d86),
    (0x051D88, i_051d88),
    (0x051D8A, i_051d8a),
    (0x051D8E, i_051d8e),
    (0x051D90, i_051d90),
    (0x051D94, i_051d94),
    (0x051D96, i_051d96),
    (0x051D98, i_051d98),
    (0x051D9A, i_051d9a),
    (0x051D9E, i_051d9e),
    (0x051DA0, i_051da0),
    (0x051DA2, i_051da2),
    (0x051DA6, i_051da6),
    (0x051DA8, i_051da8),
    (0x051DAC, i_051dac),
    (0x051DAE, i_051dae),
    (0x051DB2, i_051db2),
    (0x051DB4, i_051db4),
    (0x051DB8, i_051db8),
    (0x051DBA, i_051dba),
    (0x051DBC, i_051dbc),
    (0x051DBE, i_051dbe),
    (0x051DC2, i_051dc2),
    (0x051DC4, i_051dc4),
    (0x051DC6, i_051dc6),
    (0x051DCA, i_051dca),
    (0x051DD0, i_051dd0),
    (0x051DD6, i_051dd6),
    (0x051DDA, i_051dda),
    (0x051DDC, i_051ddc),
    (0x051DE0, i_051de0),
    (0x051DE2, i_051de2),
    (0x051DE8, i_051de8),
    (0x051DEE, i_051dee),
    (0x051DF2, i_051df2),
    (0x051DF4, i_051df4),
    (0x051DF6, i_051df6),
    (0x051DF8, i_051df8),
    (0x051DFA, i_051dfa),
    (0x051DFE, i_051dfe),
    (0x051E02, i_051e02),
    (0x051E04, i_051e04),
    (0x051E06, i_051e06),
    (0x051E08, i_051e08),
    (0x051E0A, i_051e0a),
    (0x051E0C, i_051e0c),
    (0x051E10, i_051e10),
    (0x051E14, i_051e14),
    (0x051E18, i_051e18),
    (0x051E1C, i_051e1c),
    (0x051E20, i_051e20),
    (0x051E24, i_051e24),
    (0x051E28, i_051e28),
    (0x051E2A, i_051e2a),
    (0x051E2E, i_051e2e),
    (0x051E30, i_051e30),
    (0x051E32, i_051e32),
    (0x051E36, i_051e36),
    (0x051E3C, i_051e3c),
    (0x051E42, i_051e42),
    (0x051E48, i_051e48),
    (0x051E4A, i_051e4a),
    (0x051E50, i_051e50),
    (0x051E56, i_051e56),
    (0x051E5C, i_051e5c),
    (0x051E5E, i_051e5e),
    (0x051E62, i_051e62),
    (0x051E66, i_051e66),
    (0x051E6A, i_051e6a),
    (0x051E6C, i_051e6c),
    (0x051E6E, i_051e6e),
    (0x051E72, i_051e72),
    (0x051ECE, i_051ece),
    (0x051ED4, i_051ed4),
    (0x051ED6, i_051ed6),
    (0x051EDC, i_051edc),
    (0x051EDE, i_051ede),
    (0x051EE2, i_051ee2),
    (0x051EE6, i_051ee6),
    (0x051EEC, i_051eec),
    (0x051EF2, i_051ef2),
    (0x051EFA, i_051efa),
    (0x051EFC, i_051efc),
    (0x051F00, i_051f00),
    (0x051F02, i_051f02),
    (0x051F08, i_051f08),
    (0x051F0C, i_051f0c),
    (0x051F0E, i_051f0e),
    (0x051F12, i_051f12),
    (0x051F14, i_051f14),
    (0x051F16, i_051f16),
    (0x051F18, i_051f18),
    (0x051F1A, i_051f1a),
    (0x051F20, i_051f20),
    (0x051F22, i_051f22),
    (0x051F28, i_051f28),
    (0x051F2A, i_051f2a),
    (0x051F2E, i_051f2e),
    (0x051F30, i_051f30),
    (0x051F36, i_051f36),
    (0x051F3A, i_051f3a),
    (0x051F3C, i_051f3c),
    (0x051F40, i_051f40),
    (0x051F44, i_051f44),
    (0x051F46, i_051f46),
    (0x051F48, i_051f48),
    (0x051F4A, i_051f4a),
    (0x051F4C, i_051f4c),
    (0x051F4E, i_051f4e),
    (0x051F52, i_051f52),
    (0x051F56, i_051f56),
    (0x051F5A, i_051f5a),
    (0x051F5E, i_051f5e),
    (0x051F62, i_051f62),
    (0x051F64, i_051f64),
    (0x051F66, i_051f66),
    (0x051F6A, i_051f6a),
    (0x051F6E, i_051f6e),
    (0x051F72, i_051f72),
    (0x051F74, i_051f74),
    (0x051F76, i_051f76),
    (0x051F78, i_051f78),
    (0x051F7C, i_051f7c),
    (0x051F80, i_051f80),
    (0x051F82, i_051f82),
    (0x051F86, i_051f86),
    (0x051F8C, i_051f8c),
    (0x051F8E, i_051f8e),
    (0x051F92, i_051f92),
    (0x051F94, i_051f94),
    (0x051F98, i_051f98),
    (0x051F9C, i_051f9c),
    (0x051FA0, i_051fa0),
    (0x051FA4, i_051fa4),
    (0x051FA6, i_051fa6),
    (0x051FA8, i_051fa8),
    (0x051FAE, i_051fae),
    (0x051FB4, i_051fb4),
    (0x051FBC, i_051fbc),
    (0x051FBE, i_051fbe),
    (0x051FC2, i_051fc2),
    (0x051FC4, i_051fc4),
    (0x051FC6, i_051fc6),
    (0x051FC8, i_051fc8),
    (0x051FCC, i_051fcc),
    (0x051FD0, i_051fd0),
    (0x051FD4, i_051fd4),
    (0x051FD6, i_051fd6),
    (0x051FD8, i_051fd8),
    (0x051FDE, i_051fde),
    (0x051FE4, i_051fe4),
    (0x051FE8, i_051fe8),
    (0x051FEA, i_051fea),
    (0x051FEC, i_051fec),
    (0x051FF0, i_051ff0),
    (0x051FF6, i_051ff6),
    (0x051FFA, i_051ffa),
    (0x052000, i_052000),
    (0x052006, i_052006),
    (0x052008, i_052008),
    (0x05200C, i_05200c),
    (0x052012, i_052012),
    (0x052016, i_052016),
    (0x052018, i_052018),
    (0x05201A, i_05201a),
    (0x05201C, i_05201c),
    (0x052022, i_052022),
    (0x052028, i_052028),
    (0x052030, i_052030),
    (0x052050, i_052050),
    (0x052056, i_052056),
    (0x05205A, i_05205a),
    (0x052060, i_052060),
    (0x052064, i_052064),
    (0x052068, i_052068),
    (0x052070, i_052070),
    (0x052074, i_052074),
    (0x05207C, i_05207c),
    (0x052080, i_052080),
    (0x052084, i_052084),
    (0x05208A, i_05208a),
    (0x05208E, i_05208e),
    (0x052094, i_052094),
    (0x05209A, i_05209a),
    (0x05209C, i_05209c),
    (0x0520A2, i_0520a2),
    (0x0520A6, i_0520a6),
    (0x0520AC, i_0520ac),
    (0x0520B0, i_0520b0),
    (0x0520B8, i_0520b8),
    (0x0520BC, i_0520bc),
    (0x0520C0, i_0520c0),
    (0x0520C6, i_0520c6),
    (0x0520CA, i_0520ca),
    (0x0520D2, i_0520d2),
    (0x0520D4, i_0520d4),
    (0x0520D8, i_0520d8),
    (0x0520DC, i_0520dc),
    (0x0520DE, i_0520de),
    (0x0520E2, i_0520e2),
    (0x0520E6, i_0520e6),
    (0x0520E8, i_0520e8),
    (0x0520EA, i_0520ea),
    (0x0520F2, i_0520f2),
    (0x0520F6, i_0520f6),
    (0x0520FA, i_0520fa),
    (0x0520FC, i_0520fc),
    (0x0520FE, i_0520fe),
    (0x052102, i_052102),
    (0x052106, i_052106),
    (0x05210A, i_05210a),
    (0x052112, i_052112),
    (0x052116, i_052116),
    (0x052118, i_052118),
    (0x052120, i_052120),
    (0x052124, i_052124),
    (0x052126, i_052126),
    (0x052128, i_052128),
    (0x05212C, i_05212c),
    (0x052130, i_052130),
    (0x052132, i_052132),
    (0x052136, i_052136),
    (0x05213E, i_05213e),
    (0x052142, i_052142),
    (0x052144, i_052144),
    (0x052148, i_052148),
    (0x05214A, i_05214a),
    (0x05214E, i_05214e),
    (0x052152, i_052152),
    (0x05215A, i_05215a),
    (0x05215E, i_05215e),
    (0x052162, i_052162),
    (0x052166, i_052166),
    (0x05216E, i_05216e),
    (0x052172, i_052172),
    (0x052176, i_052176),
    (0x05217A, i_05217a),
    (0x05217C, i_05217c),
    (0x052180, i_052180),
    (0x052184, i_052184),
    (0x05218C, i_05218c),
    (0x052190, i_052190),
    (0x052194, i_052194),
    (0x052198, i_052198),
    (0x0521A0, i_0521a0),
    (0x0521A4, i_0521a4),
    (0x0521A8, i_0521a8),
    (0x0521B0, i_0521b0),
    (0x0521B2, i_0521b2),
    (0x0521B6, i_0521b6),
    (0x0521BA, i_0521ba),
    (0x0521BE, i_0521be),
    (0x0521C2, i_0521c2),
    (0x0521CA, i_0521ca),
    (0x0521CE, i_0521ce),
    (0x0521D2, i_0521d2),
    (0x0521D6, i_0521d6),
    (0x0521DE, i_0521de),
    (0x0521E2, i_0521e2),
    (0x0521E6, i_0521e6),
    (0x0521EA, i_0521ea),
    (0x0521EE, i_0521ee),
    (0x0521F6, i_0521f6),
    (0x0521FA, i_0521fa),
    (0x0521FE, i_0521fe),
    (0x052202, i_052202),
    (0x052206, i_052206),
    (0x05220E, i_05220e),
    (0x052212, i_052212),
    (0x052216, i_052216),
    (0x05221A, i_05221a),
    (0x05221E, i_05221e),
    (0x052226, i_052226),
    (0x05222A, i_05222a),
    (0x05222E, i_05222e),
    (0x05223A, i_05223a),
    (0x052240, i_052240),
    (0x052244, i_052244),
    (0x05224A, i_05224a),
    (0x05224E, i_05224e),
    (0x052252, i_052252),
    (0x05225A, i_05225a),
    (0x05225E, i_05225e),
    (0x052266, i_052266),
    (0x05226A, i_05226a),
    (0x05226E, i_05226e),
    (0x052272, i_052272),
    (0x052274, i_052274),
    (0x052278, i_052278),
    (0x052280, i_052280),
    (0x052284, i_052284),
    (0x052288, i_052288),
    (0x05228A, i_05228a),
    (0x052292, i_052292),
    (0x052296, i_052296),
    (0x05229A, i_05229a),
    (0x0522A0, i_0522a0),
    (0x0522A4, i_0522a4),
    (0x0522AC, i_0522ac),
    (0x0522AE, i_0522ae),
    (0x0522B2, i_0522b2),
    (0x0522B4, i_0522b4),
    (0x0522B8, i_0522b8),
    (0x0522BC, i_0522bc),
    (0x0522C4, i_0522c4),
    (0x0522C8, i_0522c8),
    (0x0522CC, i_0522cc),
    (0x0522D0, i_0522d0),
    (0x0522D8, i_0522d8),
    (0x0522DC, i_0522dc),
    (0x0522E0, i_0522e0),
    (0x0522E8, i_0522e8),
    (0x052328, i_052328),
    (0x05232A, i_05232a),
    (0x052330, i_052330),
    (0x052334, i_052334),
    (0x052338, i_052338),
    (0x05233A, i_05233a),
    (0x052342, i_052342),
    (0x05234A, i_05234a),
    (0x052352, i_052352),
    (0x05235A, i_05235a),
    (0x05235C, i_05235c),
    (0x052362, i_052362),
    (0x052364, i_052364),
    (0x05236A, i_05236a),
    (0x05236E, i_05236e),
    (0x052372, i_052372),
    (0x052378, i_052378),
    (0x05237C, i_05237c),
    (0x052380, i_052380),
    (0x05239E, i_05239e),
    (0x0523A0, i_0523a0),
    (0x0523A4, i_0523a4),
    (0x0523AA, i_0523aa),
    (0x0523AC, i_0523ac),
    (0x0523B0, i_0523b0),
    (0x0523B2, i_0523b2),
    (0x0523B4, i_0523b4),
    (0x0523B8, i_0523b8),
    (0x0523BE, i_0523be),
    (0x0523C0, i_0523c0),
    (0x0523C4, i_0523c4),
    (0x0523C6, i_0523c6),
    (0x0523C8, i_0523c8),
    (0x0523CC, i_0523cc),
    (0x0523D2, i_0523d2),
    (0x0523D4, i_0523d4),
    (0x0523D8, i_0523d8),
    (0x0523DA, i_0523da),
    (0x0523DC, i_0523dc),
    (0x0523E0, i_0523e0),
    (0x0523E6, i_0523e6),
    (0x0523E8, i_0523e8),
    (0x0523EC, i_0523ec),
    (0x0523EE, i_0523ee),
    (0x0523F6, i_0523f6),
    (0x0523FA, i_0523fa),
    (0x052402, i_052402),
    (0x052404, i_052404),
    (0x05240A, i_05240a),
    (0x052410, i_052410),
    (0x052416, i_052416),
    (0x05241C, i_05241c),
    (0x05241E, i_05241e),
    (0x052424, i_052424),
    (0x052428, i_052428),
    (0x05242C, i_05242c),
    (0x05242E, i_05242e),
    (0x052430, i_052430),
    (0x052434, i_052434),
    (0x052438, i_052438),
    (0x05243E, i_05243e),
    (0x052444, i_052444),
    (0x05244A, i_05244a),
    (0x05244E, i_05244e),
    (0x052452, i_052452),
    (0x052454, i_052454),
    (0x052458, i_052458),
    (0x05245C, i_05245c),
    (0x052462, i_052462),
    (0x052466, i_052466),
    (0x05246C, i_05246c),
    (0x052472, i_052472),
    (0x052478, i_052478),
    (0x05247C, i_05247c),
    (0x052482, i_052482),
    (0x052486, i_052486),
    (0x052488, i_052488),
    (0x05248E, i_05248e),
    (0x052494, i_052494),
    (0x05249A, i_05249a),
    (0x0524A2, i_0524a2),
    (0x0524A8, i_0524a8),
    (0x0524AA, i_0524aa),
    (0x0524B2, i_0524b2),
    (0x0524B6, i_0524b6),
    (0x0524BE, i_0524be),
    (0x0524C0, i_0524c0),
    (0x0524C6, i_0524c6),
    (0x0524CC, i_0524cc),
    (0x0524D2, i_0524d2),
    (0x0524DA, i_0524da),
    (0x0524DC, i_0524dc),
    (0x0524E2, i_0524e2),
    (0x0524E6, i_0524e6),
    (0x0524EA, i_0524ea),
    (0x0524EC, i_0524ec),
    (0x0524EE, i_0524ee),
    (0x0524F2, i_0524f2),
    (0x0524F6, i_0524f6),
    (0x0524FC, i_0524fc),
    (0x052502, i_052502),
    (0x052508, i_052508),
    (0x05250C, i_05250c),
    (0x052510, i_052510),
    (0x052512, i_052512),
    (0x052514, i_052514),
    (0x052518, i_052518),
    (0x05251C, i_05251c),
    (0x052522, i_052522),
    (0x052526, i_052526),
    (0x05252C, i_05252c),
    (0x052532, i_052532),
    (0x052538, i_052538),
    (0x05253C, i_05253c),
    (0x052542, i_052542),
    (0x052546, i_052546),
    (0x052548, i_052548),
    (0x05254E, i_05254e),
    (0x052554, i_052554),
    (0x05255A, i_05255a),
    (0x052560, i_052560),
    (0x052568, i_052568),
    (0x05256E, i_05256e),
    (0x052580, i_052580),
    (0x052584, i_052584),
    (0x052588, i_052588),
    (0x05258E, i_05258e),
    (0x052592, i_052592),
    (0x052596, i_052596),
    (0x05259A, i_05259a),
    (0x05259E, i_05259e),
    (0x0525A2, i_0525a2),
    (0x0525A4, i_0525a4),
    (0x0525AC, i_0525ac),
    (0x0525B4, i_0525b4),
    (0x0525BC, i_0525bc),
    (0x0525C4, i_0525c4),
    (0x0525C6, i_0525c6),
    (0x0525CA, i_0525ca),
    (0x0525CE, i_0525ce),
    (0x0525D2, i_0525d2),
    (0x0525D4, i_0525d4),
    (0x0525DA, i_0525da),
    (0x0525DC, i_0525dc),
    (0x0525E0, i_0525e0),
    (0x0525E2, i_0525e2),
    (0x0525E4, i_0525e4),
    (0x0525E6, i_0525e6),
    (0x0525EA, i_0525ea),
    (0x0525EE, i_0525ee),
    (0x0525F0, i_0525f0),
    (0x0525F6, i_0525f6),
    (0x0525F8, i_0525f8),
    (0x0525FC, i_0525fc),
    (0x0525FE, i_0525fe),
    (0x052600, i_052600),
    (0x052602, i_052602),
    (0x052606, i_052606),
    (0x05260A, i_05260a),
    (0x05260C, i_05260c),
    (0x052612, i_052612),
    (0x052614, i_052614),
    (0x052618, i_052618),
    (0x05261A, i_05261a),
    (0x05261C, i_05261c),
    (0x05261E, i_05261e),
    (0x052622, i_052622),
    (0x052626, i_052626),
    (0x05262A, i_05262a),
    (0x05262C, i_05262c),
    (0x052630, i_052630),
    (0x052634, i_052634),
    (0x052638, i_052638),
    (0x05263C, i_05263c),
    (0x052640, i_052640),
    (0x052644, i_052644),
    (0x052648, i_052648),
    (0x05264C, i_05264c),
    (0x052650, i_052650),
    (0x052652, i_052652),
    (0x052656, i_052656),
    (0x052658, i_052658),
    (0x05265E, i_05265e),
    (0x052662, i_052662),
    (0x052664, i_052664),
    (0x052668, i_052668),
    (0x05266A, i_05266a),
    (0x052670, i_052670),
    (0x052674, i_052674),
    (0x052676, i_052676),
    (0x05267A, i_05267a),
    (0x05267C, i_05267c),
    (0x052682, i_052682),
    (0x052684, i_052684),
    (0x052688, i_052688),
    (0x05268E, i_05268e),
    (0x052692, i_052692),
    (0x052698, i_052698),
    (0x05269C, i_05269c),
    (0x0526A2, i_0526a2),
    (0x0526A6, i_0526a6),
    (0x0526A8, i_0526a8),
    (0x0526AA, i_0526aa),
    (0x0526B0, i_0526b0),
    (0x0526B8, i_0526b8),
    (0x0526BE, i_0526be),
    (0x0526C2, i_0526c2),
    (0x0526C6, i_0526c6),
    (0x0526C8, i_0526c8),
    (0x0526CA, i_0526ca),
    (0x0526CC, i_0526cc),
    (0x0526CE, i_0526ce),
    (0x0526D0, i_0526d0),
    (0x0526D2, i_0526d2),
    (0x0526D4, i_0526d4),
    (0x0526D6, i_0526d6),
    (0x0526D8, i_0526d8),
    (0x0526DC, i_0526dc),
    (0x0526E2, i_0526e2),
    (0x0526E6, i_0526e6),
    (0x0526EC, i_0526ec),
    (0x0526EE, i_0526ee),
    (0x0526F0, i_0526f0),
    (0x0526F2, i_0526f2),
    (0x0526F4, i_0526f4),
    (0x0526F8, i_0526f8),
    (0x0526FA, i_0526fa),
    (0x052700, i_052700),
    (0x052706, i_052706),
    (0x05270C, i_05270c),
    (0x052712, i_052712),
    (0x052714, i_052714),
    (0x05271A, i_05271a),
    (0x052720, i_052720),
    (0x052726, i_052726),
    (0x05272C, i_05272c),
    (0x052732, i_052732),
    (0x052738, i_052738),
    (0x056ACC, i_056acc),
    (0x056AD2, i_056ad2),
    (0x056AD6, i_056ad6),
    (0x056ADA, i_056ada),
    (0x056ADE, i_056ade),
    (0x056AE2, i_056ae2),
    (0x056AE6, i_056ae6),
    (0x056AEA, i_056aea),
    (0x056AEE, i_056aee),
    (0x056AF4, i_056af4),
    (0x056AF8, i_056af8),
    (0x056AFC, i_056afc),
    (0x056B02, i_056b02),
    (0x056B06, i_056b06),
    (0x056B0C, i_056b0c),
    (0x056B10, i_056b10),
    (0x056B12, i_056b12),
    (0x056B14, i_056b14),
    (0x056B16, i_056b16),
    (0x056B18, i_056b18),
    (0x056B1A, i_056b1a),
    (0x056B1C, i_056b1c),
    (0x056B1E, i_056b1e),
    (0x056B20, i_056b20),
    (0x056B22, i_056b22),
    (0x056B24, i_056b24),
    (0x056B28, i_056b28),
    (0x056B2A, i_056b2a),
    (0x056B2C, i_056b2c),
    (0x056B32, i_056b32),
    (0x056B34, i_056b34),
    (0x056B36, i_056b36),
    (0x056B38, i_056b38),
    (0x056B3A, i_056b3a),
    (0x056B3E, i_056b3e),
    (0x056B40, i_056b40),
    (0x056B46, i_056b46),
    (0x056B4A, i_056b4a),
    (0x056B4E, i_056b4e),
    (0x056B52, i_056b52),
    (0x056B56, i_056b56),
    (0x056B5A, i_056b5a),
    (0x056B5C, i_056b5c),
    (0x056B60, i_056b60),
    (0x056B62, i_056b62),
    (0x056B64, i_056b64),
    (0x056B6A, i_056b6a),
    (0x056B6E, i_056b6e),
    (0x056B72, i_056b72),
    (0x056B76, i_056b76),
    (0x056B7A, i_056b7a),
    (0x056B7E, i_056b7e),
    (0x056B80, i_056b80),
    (0x056B82, i_056b82),
    (0x056B86, i_056b86),
    (0x056B88, i_056b88),
    (0x056B8C, i_056b8c),
    (0x056B8E, i_056b8e),
    (0x056B90, i_056b90),
    (0x056B92, i_056b92),
    (0x056B96, i_056b96),
    (0x056B9A, i_056b9a),
    (0x056B9C, i_056b9c),
    (0x056BA0, i_056ba0),
    (0x056BA4, i_056ba4),
    (0x056BA8, i_056ba8),
    (0x056BAA, i_056baa),
    (0x056BAE, i_056bae),
    (0x056BB4, i_056bb4),
    (0x056BBA, i_056bba),
    (0x056BC0, i_056bc0),
    (0x056BC4, i_056bc4),
    (0x056BC8, i_056bc8),
    (0x056BCC, i_056bcc),
    (0x056BD2, i_056bd2),
    (0x056BD6, i_056bd6),
    (0x056BD8, i_056bd8),
    (0x056BDC, i_056bdc),
    (0x056BDE, i_056bde),
    (0x056BE2, i_056be2),
    (0x056BE8, i_056be8),
    (0x056BEC, i_056bec),
    (0x056BF2, i_056bf2),
    (0x056BF6, i_056bf6),
    (0x056BFC, i_056bfc),
    (0x056C00, i_056c00),
    (0x056C04, i_056c04),
    (0x056C06, i_056c06),
    (0x056C0A, i_056c0a),
    (0x056C10, i_056c10),
    (0x056C14, i_056c14),
    (0x056C18, i_056c18),
    (0x056C1C, i_056c1c),
    (0x056C20, i_056c20),
    (0x056C26, i_056c26),
    (0x056C2C, i_056c2c),
    (0x056C30, i_056c30),
    (0x056C36, i_056c36),
    (0x056C3A, i_056c3a),
    (0x056C40, i_056c40),
    (0x056C44, i_056c44),
    (0x056C4A, i_056c4a),
    (0x056C50, i_056c50),
    (0x056C54, i_056c54),
    (0x056C5A, i_056c5a),
    (0x056C5E, i_056c5e),
    (0x056C64, i_056c64),
    (0x056C68, i_056c68),
    (0x056C6C, i_056c6c),
    (0x056C72, i_056c72),
    (0x056C76, i_056c76),
    (0x056C78, i_056c78),
    (0x056C7C, i_056c7c),
    (0x056C82, i_056c82),
    (0x056C88, i_056c88),
    (0x056C8E, i_056c8e),
    (0x056C92, i_056c92),
    (0x056C98, i_056c98),
    (0x056C9A, i_056c9a),
    (0x056C9E, i_056c9e),
    (0x056CA4, i_056ca4),
    (0x056CAA, i_056caa),
    (0x056CB0, i_056cb0),
    (0x056CB4, i_056cb4),
    (0x056CB8, i_056cb8),
    (0x056CBC, i_056cbc),
    (0x056CC2, i_056cc2),
    (0x056CC6, i_056cc6),
    (0x056CCA, i_056cca),
    (0x056CD0, i_056cd0),
    (0x056CD6, i_056cd6),
    (0x056CDC, i_056cdc),
    (0x056CE0, i_056ce0),
    (0x056CE6, i_056ce6),
    (0x056CEA, i_056cea),
    (0x056CEE, i_056cee),
    (0x056CF2, i_056cf2),
    (0x056CF6, i_056cf6),
    (0x056CFA, i_056cfa),
    (0x056CFE, i_056cfe),
    (0x056D04, i_056d04),
    (0x056D08, i_056d08),
    (0x056D0C, i_056d0c),
    (0x056D12, i_056d12),
    (0x056D16, i_056d16),
    (0x056D1C, i_056d1c),
    (0x056D20, i_056d20),
    (0x056D24, i_056d24),
    (0x056D28, i_056d28),
    (0x056D2E, i_056d2e),
    (0x056D34, i_056d34),
    (0x056D38, i_056d38),
    (0x056D3E, i_056d3e),
    (0x056D44, i_056d44),
    (0x056D48, i_056d48),
    (0x056D4A, i_056d4a),
    (0x056D4E, i_056d4e),
    (0x056D52, i_056d52),
    (0x056D56, i_056d56),
    (0x056D5A, i_056d5a),
    (0x056D5C, i_056d5c),
    (0x056D60, i_056d60),
    (0x056D62, i_056d62),
    (0x056D66, i_056d66),
    (0x056D68, i_056d68),
    (0x056D6A, i_056d6a),
    (0x056D6E, i_056d6e),
    (0x056D72, i_056d72),
    (0x056D76, i_056d76),
    (0x056D78, i_056d78),
    (0x056D7C, i_056d7c),
    (0x056D80, i_056d80),
    (0x056D82, i_056d82),
    (0x056D84, i_056d84),
    (0x056D88, i_056d88),
    (0x056D8C, i_056d8c),
    (0x056D90, i_056d90),
    (0x056D94, i_056d94),
    (0x056D96, i_056d96),
    (0x056D9A, i_056d9a),
    (0x056D9C, i_056d9c),
    (0x056DA0, i_056da0),
    (0x056DA2, i_056da2),
    (0x056DA6, i_056da6),
    (0x056DAC, i_056dac),
    (0x056DB0, i_056db0),
    (0x056DB2, i_056db2),
    (0x056DB4, i_056db4),
    (0x056DBA, i_056dba),
    (0x056DBE, i_056dbe),
    (0x056DC2, i_056dc2),
    (0x056DC6, i_056dc6),
    (0x056DCA, i_056dca),
    (0x056DCE, i_056dce),
    (0x056DD2, i_056dd2),
    (0x056DD6, i_056dd6),
    (0x056DDA, i_056dda),
    (0x056DDC, i_056ddc),
    (0x056DE2, i_056de2),
    (0x056DE6, i_056de6),
    (0x056DEA, i_056dea),
    (0x056DEE, i_056dee),
    (0x056DF0, i_056df0),
    (0x056DF6, i_056df6),
    (0x056DFA, i_056dfa),
    (0x056E00, i_056e00),
    (0x056E04, i_056e04),
    (0x056E08, i_056e08),
    (0x056E0C, i_056e0c),
    (0x056E10, i_056e10),
    (0x056E14, i_056e14),
    (0x056E18, i_056e18),
    (0x056E1C, i_056e1c),
    (0x056E1E, i_056e1e),
    (0x056E24, i_056e24),
    (0x056E2A, i_056e2a),
    (0x056E2E, i_056e2e),
    (0x056E34, i_056e34),
    (0x056E36, i_056e36),
    (0x056E3A, i_056e3a),
    (0x056E3E, i_056e3e),
    (0x056E42, i_056e42),
    (0x056E44, i_056e44),
    (0x056E48, i_056e48),
    (0x056E4A, i_056e4a),
    (0x056E4E, i_056e4e),
    (0x056E52, i_056e52),
    (0x056E56, i_056e56),
    (0x056E5C, i_056e5c),
    (0x056E60, i_056e60),
    (0x056E64, i_056e64),
    (0x056E68, i_056e68),
    (0x056E6A, i_056e6a),
    (0x056E70, i_056e70),
    (0x056E72, i_056e72),
    (0x056E76, i_056e76),
    (0x056E7A, i_056e7a),
    (0x056E7C, i_056e7c),
    (0x056E80, i_056e80),
    (0x056E82, i_056e82),
    (0x056E86, i_056e86),
    (0x056E8A, i_056e8a),
    (0x056E8E, i_056e8e),
    (0x056E90, i_056e90),
    (0x056E94, i_056e94),
    (0x056E98, i_056e98),
    (0x056E9C, i_056e9c),
    (0x056E9E, i_056e9e),
    (0x056EA2, i_056ea2),
    (0x056EA6, i_056ea6),
    (0x056EAA, i_056eaa),
    (0x056EAC, i_056eac),
    (0x056EAE, i_056eae),
    (0x056EB2, i_056eb2),
    (0x056EB8, i_056eb8),
    (0x056EBC, i_056ebc),
    (0x056EBE, i_056ebe),
    (0x056EC2, i_056ec2),
    (0x056EC4, i_056ec4),
    (0x056EC6, i_056ec6),
    (0x056ECA, i_056eca),
    (0x056ECC, i_056ecc),
    (0x056ED0, i_056ed0),
    (0x056EDA, i_056eda),
    (0x056EDE, i_056ede),
    (0x056EE2, i_056ee2),
    (0x056EE6, i_056ee6),
    (0x056EEC, i_056eec),
    (0x056EF0, i_056ef0),
    (0x056EF4, i_056ef4),
    (0x056EF8, i_056ef8),
    (0x056EFA, i_056efa),
    (0x056F00, i_056f00),
    (0x056F04, i_056f04),
    (0x056F08, i_056f08),
    (0x056F0A, i_056f0a),
    (0x056F0E, i_056f0e),
    (0x056F8A, i_056f8a),
    (0x056F8E, i_056f8e),
    (0x056F94, i_056f94),
    (0x056F98, i_056f98),
    (0x056F9A, i_056f9a),
    (0x056F9E, i_056f9e),
    (0x056FA0, i_056fa0),
    (0x056FA6, i_056fa6),
    (0x056FAA, i_056faa),
    (0x056FAE, i_056fae),
    (0x056FB2, i_056fb2),
    (0x056FB6, i_056fb6),
    (0x056FBA, i_056fba),
    (0x056FBE, i_056fbe),
    (0x056FC2, i_056fc2),
    (0x056FC6, i_056fc6),
    (0x056FCC, i_056fcc),
    (0x056FD0, i_056fd0),
    (0x056FD6, i_056fd6),
    (0x056FDA, i_056fda),
    (0x056FE0, i_056fe0),
    (0x056FE4, i_056fe4),
    (0x056FE6, i_056fe6),
    (0x056FEA, i_056fea),
    (0x056FEC, i_056fec),
    (0x056FF2, i_056ff2),
    (0x056FF4, i_056ff4),
    (0x056FF8, i_056ff8),
    (0x056FFA, i_056ffa),
    (0x056FFC, i_056ffc),
    (0x057000, i_057000),
    (0x057006, i_057006),
    (0x057008, i_057008),
    (0x05700C, i_05700c),
    (0x057010, i_057010),
    (0x057016, i_057016),
    (0x057018, i_057018),
    (0x05701A, i_05701a),
    (0x05701E, i_05701e),
    (0x057022, i_057022),
    (0x057026, i_057026),
    (0x057028, i_057028),
    (0x05702A, i_05702a),
    (0x057030, i_057030),
    (0x057032, i_057032),
    (0x057034, i_057034),
    (0x057038, i_057038),
    (0x05703C, i_05703c),
    (0x057040, i_057040),
    (0x057042, i_057042),
    (0x0570A8, i_0570a8),
    (0x0570AE, i_0570ae),
    (0x0570B2, i_0570b2),
    (0x0570B8, i_0570b8),
    (0x0570BE, i_0570be),
    (0x0570C4, i_0570c4),
    (0x0570C6, i_0570c6),
    (0x0570CA, i_0570ca),
    (0x0570CE, i_0570ce),
    (0x0570D0, i_0570d0),
    (0x0570D2, i_0570d2),
    (0x0570D4, i_0570d4),
    (0x0570D8, i_0570d8),
    (0x0570DE, i_0570de),
    (0x0570E4, i_0570e4),
    (0x0570E6, i_0570e6),
    (0x0570EA, i_0570ea),
    (0x0570EE, i_0570ee),
    (0x0570F2, i_0570f2),
    (0x0570F4, i_0570f4),
    (0x0570F6, i_0570f6),
    (0x0570FA, i_0570fa),
    (0x0570FE, i_0570fe),
    (0x057102, i_057102),
    (0x057106, i_057106),
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
    (0x05719C, i_05719c),
    (0x0571A2, i_0571a2),
    (0x0571A6, i_0571a6),
    (0x0571AC, i_0571ac),
    (0x0571B0, i_0571b0),
    (0x0571B6, i_0571b6),
    (0x0571BC, i_0571bc),
    (0x0571C2, i_0571c2),
    (0x05736A, i_05736a),
    (0x057370, i_057370),
    (0x057374, i_057374),
    (0x05737A, i_05737a),
    (0x05737E, i_05737e),
    (0x057382, i_057382),
    (0x0573A2, i_0573a2),
    (0x0573A6, i_0573a6),
    (0x0573AA, i_0573aa),
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
];
