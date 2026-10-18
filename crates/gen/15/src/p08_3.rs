// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0862F8: move.w #$c000, $38(a0)
pub fn i_0862f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862FE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0862FE: rts
pub fn i_0862fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86300;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_Spawn4Finale_086300  ($086300, 40 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086300: clr.b d0
pub fn i_086300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86302;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086302: movem.w d0, -(a7)
pub fn i_086302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86306;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $086306: lea.l $846e0(pc), a1
pub fn i_086306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8630A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x846E0;
    16
}
/// $08630A: jsr $6fe.l
pub fn i_08630a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86310);
    cpu.pc = 0x6FE;
    16
}
/// $086310: jsr $5dd02.l
pub fn i_086310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86316);
    cpu.pc = 0x5DD02;
    16
}
/// $086316: movem.w (a7)+, d0
pub fn i_086316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8631A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08631A: move.b d0, $21(a0)
pub fn i_08631a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8631E;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08631E: addq.b #$1, d0
pub fn i_08631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86320;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086320: cmpi.b #$4, d0
pub fn i_086320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86324;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $086324: blt.b $86302
pub fn i_086324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86326;
    cpu.ir = 0x6DDC;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x86302; return 10; }
    8
}
/// $086326: rts
pub fn i_086326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86328;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnRow8_086328  ($086328, 60 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086328: clr.b d0
pub fn i_086328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8632A;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08632A: move.w $22(a6), d1
pub fn i_08632a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8632E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08632E: movem.l d0, -(a7)
pub fn i_08632e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86332;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086332: movem.l d1, -(a7)
pub fn i_086332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86336;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086336: lea.l $83fd0(pc), a1
pub fn i_086336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8633A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83FD0;
    16
}
/// $08633A: jsr $4ae.l
pub fn i_08633a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86340);
    cpu.pc = 0x4AE;
    16
}
/// $086340: jsr $5dd22.l
pub fn i_086340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86346);
    cpu.pc = 0x5DD22;
    16
}
/// $086346: movem.l (a7)+, d1
pub fn i_086346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8634A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08634A: movem.l (a7)+, d0
pub fn i_08634a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8634E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08634E: move.b d0, $21(a0)
pub fn i_08634e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86352;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086352: move.w d1, $22(a0)
pub fn i_086352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86356;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086356: addq.b #$1, d0
pub fn i_086356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86358;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086358: addi.w #$20, d1
pub fn i_086358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8635C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08635C: cmpi.b #$8, d0
pub fn i_08635c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86360;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $086360: blt.b $8632e
pub fn i_086360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86362;
    cpu.ir = 0x6DCC;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8632E; return 10; }
    8
}
/// $086362: rts
pub fn i_086362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86364;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnRow9_086364  ($086364, 60 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086364: clr.b d0
pub fn i_086364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86366;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086366: move.w $22(a6), d1
pub fn i_086366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8636A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08636A: movem.l d0, -(a7)
pub fn i_08636a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8636E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $08636E: movem.l d1, -(a7)
pub fn i_08636e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86372;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086372: lea.l $84282(pc), a1
pub fn i_086372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86376;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84282;
    16
}
/// $086376: jsr $4ae.l
pub fn i_086376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8637C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8637C);
    cpu.pc = 0x4AE;
    16
}
/// $08637C: jsr $5dd22.l
pub fn i_08637c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86382);
    cpu.pc = 0x5DD22;
    16
}
/// $086382: movem.l (a7)+, d1
pub fn i_086382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86386;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $086386: movem.l (a7)+, d0
pub fn i_086386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8638A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08638A: move.b d0, $21(a0)
pub fn i_08638a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8638E;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08638E: move.w d1, $22(a0)
pub fn i_08638e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86392;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086392: addq.b #$1, d0
pub fn i_086392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86394;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086394: addi.w #$10, d1
pub fn i_086394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86398;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $086398: cmpi.b #$9, d0
pub fn i_086398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8639C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $08639C: blt.b $8636a
pub fn i_08639c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8639E;
    cpu.ir = 0x6DCC;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8636A; return 10; }
    8
}
/// $08639E: rts
pub fn i_08639e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_BobY1_0863a0  ($0863A0, 30 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0863A0: addq.b #$1, $72(a6)
pub fn i_0863a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863A4;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0863A4: btst.b #$0, $72(a6)
pub fn i_0863a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863AA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0863AA: bne.w $863b6
pub fn i_0863aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x863B6; return 10; }
    8
}
/// $0863AE: subi.w #$1, $24(a6)
pub fn i_0863ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863B4;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0863B4: rts
pub fn i_0863b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0863B6: addi.w #$1, $24(a6)
pub fn i_0863b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863BC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0863BC: rts
pub fn i_0863bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TickIfState2_0863be  ($0863BE, 16 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0863BE: cmpi.b #$2, $21(a6)
pub fn i_0863be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0863C4: bne.w $863d4
pub fn i_0863c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x863D4; return 10; }
    8
}
/// $0863C8: jsr $283ca.l
pub fn i_0863c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863CE);
    cpu.pc = 0x283CA;
    16
}
// ===== JsrAbsThunk_0863ce  ($0863CE, 8 B, thunks/jsr_abs_thunks.c)
/// $0863CE: jsr $283d8.l
pub fn i_0863ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863D4);
    cpu.pc = 0x283D8;
    16
}
/// $0863D4: rts
pub fn i_0863d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0863d6  ($0863D6, 14 B, kernel/state_dispatch_stubs.c)
/// $0863D6: lea.l $2e9c8c.l, a2
pub fn i_0863d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863DC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9C8C;
    20
}
/// $0863DC: jsr $5022a.l
pub fn i_0863dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863E2);
    cpu.pc = 0x5022A;
    16
}
/// $0863E2: rts
pub fn i_0863e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0863e4  ($0863E4, 14 B, kernel/state_dispatch_stubs.c)
/// $0863E4: lea.l $2e9c50.l, a2
pub fn i_0863e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863EA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9C50;
    20
}
/// $0863EA: jsr $5022a.l
pub fn i_0863ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863F0);
    cpu.pc = 0x5022A;
    16
}
/// $0863F0: rts
pub fn i_0863f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0863f2  ($0863F2, 14 B, kernel/state_dispatch_stubs.c)
/// $0863F2: lea.l $2e9c64.l, a2
pub fn i_0863f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863F8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9C64;
    20
}
/// $0863F8: jsr $5022a.l
pub fn i_0863f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x863FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x863FE);
    cpu.pc = 0x5022A;
    16
}
/// $0863FE: rts
pub fn i_0863fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86400;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086400  ($086400, 14 B, kernel/state_dispatch_stubs.c)
/// $086400: lea.l $2e9c78.l, a2
pub fn i_086400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86406;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9C78;
    20
}
/// $086406: jsr $5022a.l
pub fn i_086406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8640C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8640C);
    cpu.pc = 0x5022A;
    16
}
/// $08640C: rts
pub fn i_08640c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8640E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08640e  ($08640E, 14 B, kernel/state_dispatch_stubs.c)
/// $08640E: lea.l $2e9ca0.l, a2
pub fn i_08640e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86414;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9CA0;
    20
}
/// $086414: jsr $5022a.l
pub fn i_086414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8641A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8641A);
    cpu.pc = 0x5022A;
    16
}
/// $08641A: rts
pub fn i_08641a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8641C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08641c  ($08641C, 14 B, kernel/state_dispatch_stubs.c)
/// $08641C: lea.l $2e9cb4.l, a2
pub fn i_08641c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86422;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9CB4;
    20
}
/// $086422: jsr $5022a.l
pub fn i_086422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86428);
    cpu.pc = 0x5022A;
    16
}
/// $086428: rts
pub fn i_086428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8642A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08642a  ($08642A, 14 B, kernel/state_dispatch_stubs.c)
/// $08642A: lea.l $2e9f20.l, a2
pub fn i_08642a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86430;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E9F20;
    20
}
/// $086430: jsr $5022a.l
pub fn i_086430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86436);
    cpu.pc = 0x5022A;
    16
}
/// $086436: rts
pub fn i_086436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86438;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086438  ($086438, 14 B, kernel/state_dispatch_stubs.c)
/// $086438: lea.l $2ea7a4.l, a2
pub fn i_086438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8643E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7A4;
    20
}
/// $08643E: jsr $5022a.l
pub fn i_08643e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86444;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86444);
    cpu.pc = 0x5022A;
    16
}
/// $086444: rts
pub fn i_086444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86446;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086446  ($086446, 14 B, kernel/state_dispatch_stubs.c)
/// $086446: lea.l $2ea7b8.l, a2
pub fn i_086446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8644C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7B8;
    20
}
/// $08644C: jsr $5022a.l
pub fn i_08644c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86452);
    cpu.pc = 0x5022A;
    16
}
/// $086452: rts
pub fn i_086452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086454  ($086454, 14 B, kernel/state_dispatch_stubs.c)
/// $086454: lea.l $2ea7cc.l, a2
pub fn i_086454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8645A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7CC;
    20
}
/// $08645A: jsr $5022a.l
pub fn i_08645a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86460);
    cpu.pc = 0x5022A;
    16
}
/// $086460: rts
pub fn i_086460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86462;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086462  ($086462, 14 B, kernel/state_dispatch_stubs.c)
/// $086462: lea.l $2ea7e0.l, a2
pub fn i_086462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86468;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7E0;
    20
}
/// $086468: jsr $5022a.l
pub fn i_086468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8646E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8646E);
    cpu.pc = 0x5022A;
    16
}
/// $08646E: rts
pub fn i_08646e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86470;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_086470  ($086470, 14 B, kernel/state_dispatch_stubs.c)
/// $086470: lea.l $2ea7f4.l, a2
pub fn i_086470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86476;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA7F4;
    20
}
/// $086476: jsr $5022a.l
pub fn i_086476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8647C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8647C);
    cpu.pc = 0x5022A;
    16
}
/// $08647C: rts
pub fn i_08647c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8647E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08647e  ($08647E, 14 B, kernel/state_dispatch_stubs.c)
/// $08647E: lea.l $2ea808.l, a2
pub fn i_08647e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86484;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA808;
    20
}
/// $086484: jsr $5022a.l
pub fn i_086484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8648A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8648A);
    cpu.pc = 0x5022A;
    16
}
/// $08648A: rts
pub fn i_08648a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8648C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08648c  ($08648C, 14 B, kernel/state_dispatch_stubs.c)
/// $08648C: lea.l $2ea81c.l, a2
pub fn i_08648c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86492;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA81C;
    20
}
/// $086492: jsr $5022a.l
pub fn i_086492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86498);
    cpu.pc = 0x5022A;
    16
}
/// $086498: rts
pub fn i_086498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8649A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_08649a  ($08649A, 14 B, kernel/state_dispatch_stubs.c)
/// $08649A: lea.l $2ea830.l, a2
pub fn i_08649a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864A0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA830;
    20
}
/// $0864A0: jsr $5022a.l
pub fn i_0864a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x864A6);
    cpu.pc = 0x5022A;
    16
}
/// $0864A6: rts
pub fn i_0864a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0864a8  ($0864A8, 14 B, kernel/state_dispatch_stubs.c)
/// $0864A8: lea.l $2ea844.l, a2
pub fn i_0864a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864AE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA844;
    20
}
/// $0864AE: jsr $5022a.l
pub fn i_0864ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x864B4);
    cpu.pc = 0x5022A;
    16
}
/// $0864B4: rts
pub fn i_0864b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_A_0864b6  ($0864B6, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0864B6: lea.l $2eaa60.l, a0
pub fn i_0864b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAA60;
    20
}
/// $0864BC: clr.l d0
pub fn i_0864bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864BE;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0864BE: move.b $21(a6), d0
pub fn i_0864be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864C2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0864C2: asl.l #$4, d0
pub fn i_0864c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864C4;
    cpu.ir = 0xE980;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, true);
    cpu.d[0] = r;
    14
}
/// $0864C4: movea.l $4(a0, d0.w), a2
pub fn i_0864c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864C8;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_0864c8  ($0864C8, 8 B, thunks/jsr_abs_thunks.c)
/// $0864C8: jsr $5022a.l
pub fn i_0864c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x864CE);
    cpu.pc = 0x5022A;
    16
}
/// $0864CE: rts
pub fn i_0864ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_B_0864d0  ($0864D0, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0864D0: lea.l $2eaa60.l, a0
pub fn i_0864d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAA60;
    20
}
/// $0864D6: clr.l d0
pub fn i_0864d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864D8;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0864D8: move.b $21(a6), d0
pub fn i_0864d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864DC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0864DC: asl.l #$4, d0
pub fn i_0864dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864DE;
    cpu.ir = 0xE980;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, true);
    cpu.d[0] = r;
    14
}
/// $0864DE: movea.l $c(a0, d0.w), a2
pub fn i_0864de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864E2;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0xC).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_0864e2  ($0864E2, 8 B, thunks/jsr_abs_thunks.c)
/// $0864E2: jsr $5022a.l
pub fn i_0864e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x864E8);
    cpu.pc = 0x5022A;
    16
}
/// $0864E8: rts
pub fn i_0864e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_C_0864ea  ($0864EA, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0864EA: lea.l $2eaae0.l, a0
pub fn i_0864ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAAE0;
    20
}
/// $0864F0: clr.l d0
pub fn i_0864f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864F2;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0864F2: move.b $21(a6), d0
pub fn i_0864f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864F6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0864F6: asl.l #$2, d0
pub fn i_0864f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864F8;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $0864F8: movea.l (a0, d0.w), a2
pub fn i_0864f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x864FC;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_0864fc  ($0864FC, 8 B, thunks/jsr_abs_thunks.c)
/// $0864FC: jsr $5022a.l
pub fn i_0864fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86502);
    cpu.pc = 0x5022A;
    16
}
/// $086502: rts
pub fn i_086502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86504;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_D_086504  ($086504, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086504: lea.l $2eab04.l, a0
pub fn i_086504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8650A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAB04;
    20
}
/// $08650A: clr.l d0
pub fn i_08650a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8650C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08650C: move.b $21(a6), d0
pub fn i_08650c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86510;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086510: asl.l #$2, d0
pub fn i_086510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86512;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $086512: movea.l (a0, d0.w), a2
pub fn i_086512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86516;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_086516  ($086516, 8 B, thunks/jsr_abs_thunks.c)
/// $086516: jsr $5022a.l
pub fn i_086516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8651C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8651C);
    cpu.pc = 0x5022A;
    16
}
/// $08651C: rts
pub fn i_08651c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8651E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_E_08651e  ($08651E, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08651E: lea.l $2eab24.l, a0
pub fn i_08651e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86524;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAB24;
    20
}
/// $086524: clr.l d0
pub fn i_086524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86526;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086526: move.b $21(a6), d0
pub fn i_086526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8652A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08652A: asl.l #$3, d0
pub fn i_08652a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8652C;
    cpu.ir = 0xE780;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, true);
    cpu.d[0] = r;
    12
}
/// $08652C: movea.l (a0, d0.w), a2
pub fn i_08652c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86530;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_086530  ($086530, 8 B, thunks/jsr_abs_thunks.c)
/// $086530: jsr $5022a.l
pub fn i_086530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86536);
    cpu.pc = 0x5022A;
    16
}
/// $086536: rts
pub fn i_086536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86538;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_BlitTable_F_086538  ($086538, 18 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086538: lea.l $2eab24.l, a0
pub fn i_086538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8653E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAB24;
    20
}
/// $08653E: clr.l d0
pub fn i_08653e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86540;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086540: move.b $21(a6), d0
pub fn i_086540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86544;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086544: asl.l #$3, d0
pub fn i_086544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86546;
    cpu.ir = 0xE780;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, true);
    cpu.d[0] = r;
    12
}
/// $086546: movea.l $4(a0, d0.w), a2
pub fn i_086546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8654A;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_08654a  ($08654A, 8 B, thunks/jsr_abs_thunks.c)
/// $08654A: jsr $5022a.l
pub fn i_08654a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86550);
    cpu.pc = 0x5022A;
    16
}
/// $086550: rts
pub fn i_086550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86552;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_086552  ($086552, 16 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086552: movea.l $8(a6), a1
pub fn i_086552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86556;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086556: move.b $10(a6), d0
pub fn i_086556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8655A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08655A: cmp.b $10(a1), d0
pub fn i_08655a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8655E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08655E: bcs.w $86568
pub fn i_08655e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86562;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x86568; return 10; }
    8
}
// ===== ClearXN_086562  ($086562, 6 B, thunks/ccr_helpers.c)
/// $086562: andi.b #$ee, ccr
pub fn i_086562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86566;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $086566: rts
pub fn i_086566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86568;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_086568  ($086568, 6 B, thunks/ccr_helpers.c)
/// $086568: ori.b #$11, ccr
pub fn i_086568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8656C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08656C: rts
pub fn i_08656c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8656E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScreenNuke_Init_08656e  ($08656E, 72 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08656E: lea.l $2ecec4.l, a0
pub fn i_08656e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86574;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2ECEC4;
    20
}
/// $086574: move.l a0, $4c(a6)
pub fn i_086574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86578;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086578: jsr $283ca.l
pub fn i_086578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8657E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8657E);
    cpu.pc = 0x283CA;
    16
}
/// $08657E: bra.w $86596
pub fn i_08657e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86582;
    cpu.ir = 0x6000;
    cpu.pc = 0x86596;
    10
}
/// $086586: lea.l $2ecd80.l, a0
pub fn i_086586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8658C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2ECD80;
    20
}
/// $08658C: move.l a0, $4c(a6)
pub fn i_08658c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86590;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086590: jsr $283ca.l
pub fn i_086590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86596);
    cpu.pc = 0x283CA;
    16
}
/// $086596: move.w #$0, $22(a6)
pub fn i_086596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8659C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08659C: move.w #$100, $24(a6)
pub fn i_08659c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0865A2: jsr $283ca.l
pub fn i_0865a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x865A8);
    cpu.pc = 0x283CA;
    16
}
/// $0865A8: jsr $283d8.l
pub fn i_0865a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x865AE);
    cpu.pc = 0x283D8;
    16
}
/// $0865AE: move.b #$1, $10e39e.l
pub fn i_0865ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865B6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
// ===== SetTaskHandler_0865b6  ($0865B6, 8 B, kernel/task_handlers.c)
/// $0865B6: lea.l $865be(pc), a1
pub fn i_0865b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x865BE;
    16
}
/// $0865BA: move.l a1, (a6)
pub fn i_0865ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0865BC: rts
pub fn i_0865bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScreenNuke_Clear_0865be  ($0865BE, 28 B, enemies/bosses/fort_scene4_0865be.s)
/// $0865BE: move.b #$0, $10e39e.l
pub fn i_0865be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865C6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $0865C6: lea.l $ffff.w, a0
pub fn i_0865c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865CA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0865CA: move.l a0, $4c(a6)
pub fn i_0865ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865CE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0865CE: jsr $283ca.l
pub fn i_0865ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x865D4);
    cpu.pc = 0x283CA;
    16
}
/// $0865D4: jmp $518.l
pub fn i_0865d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0865da  ($0865DA, 2 B, enemies/bosses/fort_scene4_0865be.s)
/// $0865DA: rts
pub fn i_0865da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_Init_V0_0865dc  ($0865DC, 72 B, enemies/bosses/fort_scene4_0865be.s)
/// $0865DC: movea.l $3c(a6), a1
pub fn i_0865dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865E0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0865E0: jsr $2942a.l
pub fn i_0865e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x865E6);
    cpu.pc = 0x2942A;
    16
}
/// $0865E6: move.b #$0, $20(a6)
pub fn i_0865e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0865EC: jsr $884a6(pc)
pub fn i_0865ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x865F0);
    cpu.pc = 0x884A6;
    16
}
/// $0865F0: lea.l $2edcb0.l, a1
pub fn i_0865f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDCB0;
    20
}
/// $0865F6: move.l a1, $78(a6)
pub fn i_0865f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x865FA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0865FA: lea.l $edc8c.l, a1
pub fn i_0865fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86600;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDC8C;
    20
}
/// $086600: move.l a1, $7c(a6)
pub fn i_086600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86604;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086604: lea.l $86e4a(pc), a1
pub fn i_086604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86608;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86E4A;
    16
}
/// $086608: jsr $4ae.l
pub fn i_086608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8660E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8660E);
    cpu.pc = 0x4AE;
    16
}
/// $08660E: jsr $5dd22.l
pub fn i_08660e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86614;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86614);
    cpu.pc = 0x5DD22;
    16
}
/// $086614: addi.w #$50, $22(a0)
pub fn i_086614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8661A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08661A: addi.w #$10, $24(a0)
pub fn i_08661a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86620;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086620: bra.w $86738
pub fn i_086620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86624;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
// ===== Fort_Init_V1_086624  ($086624, 66 B, enemies/bosses/fort_scene4_0865be.s)
/// $086624: movea.l $3c(a6), a1
pub fn i_086624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86628;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086628: jsr $2942a.l
pub fn i_086628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8662E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8662E);
    cpu.pc = 0x2942A;
    16
}
/// $08662E: move.b #$1, $20(a6)
pub fn i_08662e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86634;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086634: jsr $885b4(pc)
pub fn i_086634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86638;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86638);
    cpu.pc = 0x885B4;
    16
}
/// $086638: lea.l $2edcba.l, a1
pub fn i_086638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8663E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDCBA;
    20
}
/// $08663E: move.l a1, $78(a6)
pub fn i_08663e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86642;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086642: lea.l $edcd2.l, a1
pub fn i_086642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86648;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDCD2;
    20
}
/// $086648: move.l a1, $7c(a6)
pub fn i_086648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8664C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08664C: lea.l $86e74(pc), a1
pub fn i_08664c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86650;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86E74;
    16
}
/// $086650: jsr $4ae.l
pub fn i_086650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86656);
    cpu.pc = 0x4AE;
    16
}
/// $086656: jsr $5dd22.l
pub fn i_086656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8665C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8665C);
    cpu.pc = 0x5DD22;
    16
}
/// $08665C: addi.w #$50, $22(a0)
pub fn i_08665c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86662;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086662: bra.w $86738
pub fn i_086662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86666;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
// ===== Fort_Init_V2_086666  ($086666, 72 B, enemies/bosses/fort_scene4_0865be.s)
/// $086666: movea.l $3c(a6), a1
pub fn i_086666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8666A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08666A: jsr $2942a.l
pub fn i_08666a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86670;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86670);
    cpu.pc = 0x2942A;
    16
}
/// $086670: move.b #$2, $20(a6)
pub fn i_086670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86676;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086676: jsr $886c2(pc)
pub fn i_086676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8667A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8667A);
    cpu.pc = 0x886C2;
    16
}
/// $08667A: lea.l $2edcfe.l, a1
pub fn i_08667a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86680;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDCFE;
    20
}
/// $086680: move.l a1, $78(a6)
pub fn i_086680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86684;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086684: lea.l $edd2a.l, a1
pub fn i_086684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8668A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDD2A;
    20
}
/// $08668A: move.l a1, $7c(a6)
pub fn i_08668a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8668E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08668E: lea.l $86e9e(pc), a1
pub fn i_08668e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86692;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86E9E;
    16
}
/// $086692: jsr $4ae.l
pub fn i_086692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86698);
    cpu.pc = 0x4AE;
    16
}
/// $086698: jsr $5dd22.l
pub fn i_086698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8669E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8669E);
    cpu.pc = 0x5DD22;
    16
}
/// $08669E: addi.w #$50, $22(a0)
pub fn i_08669e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866A4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0866A4: addi.w #$10, $24(a0)
pub fn i_0866a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866AA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0866AA: bra.w $86738
pub fn i_0866aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
// ===== Fort_Init_V3_0866ae  ($0866AE, 66 B, enemies/bosses/fort_scene4_0865be.s)
/// $0866AE: movea.l $3c(a6), a1
pub fn i_0866ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866B2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0866B2: jsr $2942a.l
pub fn i_0866b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x866B8);
    cpu.pc = 0x2942A;
    16
}
/// $0866B8: move.b #$3, $20(a6)
pub fn i_0866b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0866BE: jsr $887d0(pc)
pub fn i_0866be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x866C2);
    cpu.pc = 0x887D0;
    16
}
/// $0866C2: lea.l $2edd0c.l, a1
pub fn i_0866c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866C8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD0C;
    20
}
/// $0866C8: move.l a1, $78(a6)
pub fn i_0866c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866CC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0866CC: lea.l $edda6.l, a1
pub fn i_0866cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDDA6;
    20
}
/// $0866D2: move.l a1, $7c(a6)
pub fn i_0866d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866D6;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0866D6: lea.l $86ec8(pc), a1
pub fn i_0866d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86EC8;
    16
}
/// $0866DA: jsr $4ae.l
pub fn i_0866da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x866E0);
    cpu.pc = 0x4AE;
    16
}
/// $0866E0: jsr $5dd22.l
pub fn i_0866e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x866E6);
    cpu.pc = 0x5DD22;
    16
}
/// $0866E6: addi.w #$50, $22(a0)
pub fn i_0866e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866EC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0866EC: bra.w $86738
pub fn i_0866ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
// ===== Fort_Init_V4_0866f0  ($0866F0, 228 B, enemies/bosses/fort_scene4_0865be.s)
/// $0866F0: movea.l $3c(a6), a1
pub fn i_0866f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866F4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0866F4: jsr $2942a.l
pub fn i_0866f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x866FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x866FA);
    cpu.pc = 0x2942A;
    16
}
/// $0866FA: move.b #$4, $20(a6)
pub fn i_0866fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86700;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086700: jsr $8891a(pc)
pub fn i_086700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86704;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86704);
    cpu.pc = 0x8891A;
    16
}
/// $086704: lea.l $2edd20.l, a1
pub fn i_086704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8670A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD20;
    20
}
/// $08670A: move.l a1, $78(a6)
pub fn i_08670a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8670E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08670E: lea.l $ede22.l, a1
pub fn i_08670e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86714;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDE22;
    20
}
/// $086714: move.l a1, $7c(a6)
pub fn i_086714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86718;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086718: lea.l $86ef2(pc), a1
pub fn i_086718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8671C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86EF2;
    16
}
/// $08671C: jsr $4ae.l
pub fn i_08671c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86722;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86722);
    cpu.pc = 0x4AE;
    16
}
/// $086722: jsr $5dd22.l
pub fn i_086722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86728);
    cpu.pc = 0x5DD22;
    16
}
/// $086728: addi.w #$50, $22(a0)
pub fn i_086728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8672E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08672E: addi.w #$10, $24(a0)
pub fn i_08672e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86734;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086734: bra.w $86738
pub fn i_086734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86738;
    cpu.ir = 0x6000;
    cpu.pc = 0x86738;
    10
}
/// $086738: move.w #$c0, $70(a6)
pub fn i_086738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8673E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08673E: move.w #$140, $66(a6)
pub fn i_08673e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86744;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086744: move.b #$0, $21(a6)
pub fn i_086744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8674A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08674A: lea.l $2ec41e.l, a0
pub fn i_08674a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86750;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC41E;
    20
}
/// $086750: move.l a0, $48(a6)
pub fn i_086750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86754;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086754: lea.l $8675a(pc), a1
pub fn i_086754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86758;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8675A;
    16
}
/// $086758: move.l a1, (a6)
pub fn i_086758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8675A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08675A: cmpi.w #$ffc0, $22(a6)
pub fn i_08675a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86760;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFC0, Size::Word, false);
    16
}
/// $086760: bgt.w $8676c
pub fn i_086760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86764;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8676C; return 10; }
    8
}
/// $086764: lea.l $ffff.w, a0
pub fn i_086764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86768;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $086768: move.l a0, $48(a6)
pub fn i_086768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8676C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08676C: jsr $2783a.l
pub fn i_08676c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86772;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86772);
    cpu.pc = 0x2783A;
    16
}
/// $086772: jsr $2870a.l
pub fn i_086772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86778);
    cpu.pc = 0x2870A;
    16
}
/// $086778: bcc.w $8678e
pub fn i_086778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8677C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8678E; return 10; }
    8
}
/// $08677C: lea.l $5e766.l, a0
pub fn i_08677c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86782;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $086782: jsr $5e770.l
pub fn i_086782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86788);
    cpu.pc = 0x5E770;
    16
}
/// $086788: bclr.b #$3, $13(a6)
pub fn i_086788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8678E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08678E: cmpi.w #$c0, $22(a6)
pub fn i_08678e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86794;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Word, false);
    16
}
/// $086794: blt.w $8679e
pub fn i_086794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86798;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8679E; return 10; }
    8
}
/// $086798: move.w #$140, $66(a6)
pub fn i_086798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8679E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08679E: cmpi.w #$0, $66(a6)
pub fn i_08679e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867A4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0867A4: bgt.w $867bc
pub fn i_0867a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867A8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x867BC; return 10; }
    8
}
/// $0867A8: lea.l $ffff.w, a0
pub fn i_0867a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867AC;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0867AC: move.l a0, $48(a6)
pub fn i_0867ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867B0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0867B0: move.b #$ff, $21(a6)
pub fn i_0867b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0867B6: lea.l $867d4(pc), a1
pub fn i_0867b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x867D4;
    16
}
/// $0867BA: move.l a1, (a6)
pub fn i_0867ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0867BC: jsr $4fa70.l
pub fn i_0867bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867C2);
    cpu.pc = 0x4FA70;
    16
}
/// $0867C2: bcc.w $867d2
pub fn i_0867c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x867D2; return 10; }
    8
}
/// $0867C6: move.b #$ff, $21(a6)
pub fn i_0867c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0867CC: jmp $518.l
pub fn i_0867cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867D2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0867D2: rts
pub fn i_0867d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_Destroyed_0867d4  ($0867D4, 120 B, enemies/bosses/fort_scene4_0865be.s)
/// $0867D4: jsr $2783a.l
pub fn i_0867d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867DA);
    cpu.pc = 0x2783A;
    16
}
/// $0867DA: move.w #$1028, d0
pub fn i_0867da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0867DE: jsr $2352.l
pub fn i_0867de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867E4);
    cpu.pc = 0x2352;
    16
}
/// $0867E4: lea.l $2ecf68.l, a1
pub fn i_0867e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECF68;
    20
}
/// $0867EA: jsr $77c7e.l
pub fn i_0867ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867F0);
    cpu.pc = 0x77C7E;
    16
}
/// $0867F0: lea.l $2ecf7a.l, a1
pub fn i_0867f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECF7A;
    20
}
/// $0867F6: jsr $77c7e.l
pub fn i_0867f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x867FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x867FC);
    cpu.pc = 0x77C7E;
    16
}
/// $0867FC: lea.l $2ed04a.l, a1
pub fn i_0867fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86802;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED04A;
    20
}
/// $086802: jsr $77c7e.l
pub fn i_086802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86808);
    cpu.pc = 0x77C7E;
    16
}
/// $086808: jsr $88a28(pc)
pub fn i_086808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8680C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8680C);
    cpu.pc = 0x88A28;
    16
}
/// $08680C: movea.l $78(a6), a1
pub fn i_08680c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86810;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086810: jsr $43fac.l
pub fn i_086810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86816);
    cpu.pc = 0x43FAC;
    16
}
/// $086816: jsr $434dc.l
pub fn i_086816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8681C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8681C);
    cpu.pc = 0x434DC;
    16
}
/// $08681C: lea.l $2ecc88.l, a0
pub fn i_08681c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86822;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2ECC88;
    20
}
/// $086822: move.l a0, $4c(a6)
pub fn i_086822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86826;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086826: jsr $283ca.l
pub fn i_086826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8682C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8682C);
    cpu.pc = 0x283CA;
    16
}
/// $08682C: jsr $283ca.l
pub fn i_08682c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86832);
    cpu.pc = 0x283CA;
    16
}
/// $086832: jsr $283d8.l
pub fn i_086832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86838);
    cpu.pc = 0x283D8;
    16
}
/// $086838: movea.l $7c(a6), a1
pub fn i_086838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8683C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08683C: move.w #$82, d0
pub fn i_08683c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86840;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086840: move.b #$0, $82(a6)
pub fn i_086840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86846;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086846: jsr $4429e.l
pub fn i_086846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8684C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8684C);
    cpu.pc = 0x4429E;
    16
}
// ===== SetTaskHandler_08684c  ($08684C, 8 B, kernel/task_handlers.c)
/// $08684C: lea.l $86854(pc), a1
pub fn i_08684c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86850;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86854;
    16
}
/// $086850: move.l a1, (a6)
pub fn i_086850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86852;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086852: rts
pub fn i_086852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86854;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_Idle_086854  ($086854, 20 B, enemies/bosses/fort_scene4_0865be.s)
/// $086854: lea.l $ffff.w, a0
pub fn i_086854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86858;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $086858: move.l a0, $4c(a6)
pub fn i_086858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8685C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08685C: jsr $283ca.l
pub fn i_08685c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86862;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86862);
    cpu.pc = 0x283CA;
    16
}
/// $086862: jmp $518.l
pub fn i_086862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86868;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_086868  ($086868, 2 B, enemies/bosses/fort_scene4_0865be.s)
/// $086868: rts
pub fn i_086868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8686A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Init_08686a  ($08686A, 314 B, enemies/bosses/fort_scene4_0865be.s)
/// $08686A: move.w #$e3, d1
pub fn i_08686a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8686E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08686E: jsr $236e.l
pub fn i_08686e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86874);
    cpu.pc = 0x236E;
    16
}
/// $086874: move.b #$0, $21(a6)
pub fn i_086874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8687A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08687A: bra.w $868e2
pub fn i_08687a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8687E;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $086892: move.w #$e3, d1
pub fn i_086892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86896;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086896: jsr $236e.l
pub fn i_086896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8689C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8689C);
    cpu.pc = 0x236E;
    16
}
/// $08689C: move.b #$60, $21(a6)
pub fn i_08689c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868A2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868A2: bra.w $868e2
pub fn i_0868a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $0868A6: move.w #$e3, d1
pub fn i_0868a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0868AA: jsr $236e.l
pub fn i_0868aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x868B0);
    cpu.pc = 0x236E;
    16
}
/// $0868B0: move.b #$1, $21(a6)
pub fn i_0868b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868B6: bra.w $868e2
pub fn i_0868b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $0868BA: move.w #$e4, d1
pub fn i_0868ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868BE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0868BE: jsr $236e.l
pub fn i_0868be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x868C4);
    cpu.pc = 0x236E;
    16
}
/// $0868C4: move.b #$31, $21(a6)
pub fn i_0868c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x31;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868CA: bra.w $868e2
pub fn i_0868ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x868E2;
    10
}
/// $0868E2: move.b #$ff, $32(a6)
pub fn i_0868e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868E8: move.b #$ff, $33(a6)
pub fn i_0868e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868EE: move.b #$0, $3a(a6)
pub fn i_0868ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0868F4: move.w #$28, d0
pub fn i_0868f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0868F8: move.w d0, $66(a6)
pub fn i_0868f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0868FC: move.w d0, $80(a6)
pub fn i_0868fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86900;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086900: move.w #$8000, $38(a6)
pub fn i_086900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86906;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086906: jsr $267e2.l
pub fn i_086906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8690C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8690C);
    cpu.pc = 0x267E2;
    16
}
/// $08690C: jsr $27cee.l
pub fn i_08690c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86912);
    cpu.pc = 0x27CEE;
    16
}
/// $086912: clr.l d0
pub fn i_086912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86914;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086914: move.b $21(a6), d0
pub fn i_086914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86918;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086918: asr.l #$4, d0
pub fn i_086918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8691A;
    cpu.ir = 0xE880;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[0] = r;
    14
}
/// $08691A: movea.l #$2eb2c0, a0
pub fn i_08691a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86920;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EB2C0;
    cpu.a[0] = v1;
    4
}
/// $086920: lsl.w #$2, d0
pub fn i_086920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86922;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $086922: movea.l (a0, d0.w), a0
pub fn i_086922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86926;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $086926: cmpa.l #$ffffffff, a0
pub fn i_086926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8692C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08692C: beq.w $86936
pub fn i_08692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86930;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86936; return 10; }
    8
}
/// $086930: jsr $28cd4.l
pub fn i_086930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86936);
    cpu.pc = 0x28CD4;
    16
}
/// $086936: move.w $72(a6), d0
pub fn i_086936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8693A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08693A: move.b d0, $44(a6)
pub fn i_08693a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8693E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08693E: lea.l $86944(pc), a1
pub fn i_08693e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86944;
    16
}
/// $086942: move.l a1, (a6)
pub fn i_086942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086944: movea.l $c(a6), a0
pub fn i_086944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86948;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086948: cmpi.b #$ff, $21(a0)
pub fn i_086948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8694E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08694E: beq.w $86990
pub fn i_08694e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86952;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86990; return 10; }
    8
}
/// $086952: jsr $2783a.l
pub fn i_086952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86958;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86958);
    cpu.pc = 0x2783A;
    16
}
/// $086958: jsr $28d70.l
pub fn i_086958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8695E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8695E);
    cpu.pc = 0x28D70;
    16
}
/// $08695E: jsr $2870a.l
pub fn i_08695e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86964);
    cpu.pc = 0x2870A;
    16
}
/// $086964: bcc.w $8697c
pub fn i_086964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86968;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8697C; return 10; }
    8
}
/// $086968: move.w #$108d, d0
pub fn i_086968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8696C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08696C: jsr $2352.l
pub fn i_08696c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86972);
    cpu.pc = 0x2352;
    16
}
/// $086972: bclr.b #$3, $13(a6)
pub fn i_086972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86978;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086978: jsr $8848c(pc)
pub fn i_086978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8697C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8697C);
    cpu.pc = 0x8848C;
    16
}
/// $08697C: cmpi.w #$1e, $66(a6)
pub fn i_08697c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86982;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $086982: bgt.w $869a2
pub fn i_086982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86986;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x869A2; return 10; }
    8
}
/// $086986: lea.l $869a4(pc), a1
pub fn i_086986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8698A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x869A4;
    16
}
/// $08698A: move.l a1, (a6)
pub fn i_08698a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8698C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08698C: bra.w $869a2
pub fn i_08698c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86990;
    cpu.ir = 0x6000;
    cpu.pc = 0x869A2;
    10
}
/// $086990: lea.l $2edd48.l, a1
pub fn i_086990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86996;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086996: jsr $43fac.l
pub fn i_086996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8699C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8699C);
    cpu.pc = 0x43FAC;
    16
}
/// $08699C: jmp $518.l
pub fn i_08699c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0869A2: rts
pub fn i_0869a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Damaged1_0869a4  ($0869A4, 140 B, enemies/bosses/fort_scene4_0865be.s)
/// $0869A4: clr.l d0
pub fn i_0869a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869A6;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0869A6: move.b $21(a6), d0
pub fn i_0869a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869AA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0869AA: asr.l #$4, d0
pub fn i_0869aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869AC;
    cpu.ir = 0xE880;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[0] = r;
    14
}
/// $0869AC: addq.l #$1, d0
pub fn i_0869ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869AE;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $0869AE: movea.l #$2eb2c0, a0
pub fn i_0869ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EB2C0;
    cpu.a[0] = v1;
    4
}
/// $0869B4: lsl.w #$2, d0
pub fn i_0869b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869B6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0869B6: movea.l (a0, d0.w), a0
pub fn i_0869b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869BA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0869BA: cmpa.l #$ffffffff, a0
pub fn i_0869ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869C0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0869C0: beq.w $869ca
pub fn i_0869c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x869CA; return 10; }
    8
}
/// $0869C4: jsr $28cd4.l
pub fn i_0869c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0869CA: lea.l $869d0(pc), a1
pub fn i_0869ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x869D0;
    16
}
/// $0869CE: move.l a1, (a6)
pub fn i_0869ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0869D0: movea.l $c(a6), a0
pub fn i_0869d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869D4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0869D4: cmpi.b #$ff, $21(a0)
pub fn i_0869d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869DA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0869DA: beq.w $86a1c
pub fn i_0869da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86A1C; return 10; }
    8
}
/// $0869DE: jsr $2783a.l
pub fn i_0869de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869E4);
    cpu.pc = 0x2783A;
    16
}
/// $0869E4: jsr $28d70.l
pub fn i_0869e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869EA);
    cpu.pc = 0x28D70;
    16
}
/// $0869EA: jsr $2870a.l
pub fn i_0869ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869F0);
    cpu.pc = 0x2870A;
    16
}
/// $0869F0: bcc.w $86a08
pub fn i_0869f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86A08; return 10; }
    8
}
/// $0869F4: move.w #$108d, d0
pub fn i_0869f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0869F8: jsr $2352.l
pub fn i_0869f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x869FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x869FE);
    cpu.pc = 0x2352;
    16
}
/// $0869FE: bclr.b #$3, $13(a6)
pub fn i_0869fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A04;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086A04: jsr $8848c(pc)
pub fn i_086a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86A08);
    cpu.pc = 0x8848C;
    16
}
/// $086A08: cmpi.w #$1a, $66(a6)
pub fn i_086a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A0E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1A, Size::Word, false);
    16
}
/// $086A0E: bgt.w $86a2e
pub fn i_086a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A12;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x86A2E; return 10; }
    8
}
/// $086A12: lea.l $86a30(pc), a1
pub fn i_086a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86A30;
    16
}
/// $086A16: move.l a1, (a6)
pub fn i_086a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086A18: bra.w $86a2e
pub fn i_086a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x86A2E;
    10
}
/// $086A1C: lea.l $2edd48.l, a1
pub fn i_086a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A22;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086A22: jsr $43fac.l
pub fn i_086a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A28);
    cpu.pc = 0x43FAC;
    16
}
/// $086A28: jmp $518.l
pub fn i_086a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A2E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086A2E: rts
pub fn i_086a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Damaged2_086a30  ($086A30, 140 B, enemies/bosses/fort_scene4_0865be.s)
/// $086A30: clr.l d0
pub fn i_086a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A32;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086A32: move.b $21(a6), d0
pub fn i_086a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A36;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086A36: asr.l #$4, d0
pub fn i_086a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A38;
    cpu.ir = 0xE880;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[0] = r;
    14
}
/// $086A38: addq.l #$2, d0
pub fn i_086a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A3A;
    cpu.ir = 0x5480;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $086A3A: movea.l #$2eb2c0, a0
pub fn i_086a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A40;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EB2C0;
    cpu.a[0] = v1;
    4
}
/// $086A40: lsl.w #$2, d0
pub fn i_086a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A42;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $086A42: movea.l (a0, d0.w), a0
pub fn i_086a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A46;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $086A46: cmpa.l #$ffffffff, a0
pub fn i_086a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A4C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $086A4C: beq.w $86a56
pub fn i_086a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86A56; return 10; }
    8
}
/// $086A50: jsr $28cd4.l
pub fn i_086a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A56);
    cpu.pc = 0x28CD4;
    16
}
/// $086A56: lea.l $86a5c(pc), a1
pub fn i_086a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86A5C;
    16
}
/// $086A5A: move.l a1, (a6)
pub fn i_086a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086A5C: movea.l $c(a6), a0
pub fn i_086a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086A60: cmpi.b #$ff, $21(a0)
pub fn i_086a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A66;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086A66: beq.w $86aa8
pub fn i_086a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A6A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86AA8; return 10; }
    8
}
/// $086A6A: jsr $2783a.l
pub fn i_086a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A70);
    cpu.pc = 0x2783A;
    16
}
/// $086A70: jsr $28d70.l
pub fn i_086a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A76);
    cpu.pc = 0x28D70;
    16
}
/// $086A76: jsr $2870a.l
pub fn i_086a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A7C);
    cpu.pc = 0x2870A;
    16
}
/// $086A7C: bcc.w $86a94
pub fn i_086a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86A94; return 10; }
    8
}
/// $086A80: move.w #$108d, d0
pub fn i_086a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086A84: jsr $2352.l
pub fn i_086a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86A8A);
    cpu.pc = 0x2352;
    16
}
/// $086A8A: bclr.b #$3, $13(a6)
pub fn i_086a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A90;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086A90: jsr $8848c(pc)
pub fn i_086a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86A94);
    cpu.pc = 0x8848C;
    16
}
/// $086A94: cmpi.w #$d, $66(a6)
pub fn i_086a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A9A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD, Size::Word, false);
    16
}
/// $086A9A: bgt.w $86aba
pub fn i_086a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86A9E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x86ABA; return 10; }
    8
}
/// $086A9E: lea.l $86abc(pc), a1
pub fn i_086a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86ABC;
    16
}
/// $086AA2: move.l a1, (a6)
pub fn i_086aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086AA4: bra.w $86aba
pub fn i_086aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x86ABA;
    10
}
/// $086AA8: lea.l $2edd48.l, a1
pub fn i_086aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AAE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086AAE: jsr $43fac.l
pub fn i_086aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86AB4);
    cpu.pc = 0x43FAC;
    16
}
/// $086AB4: jmp $518.l
pub fn i_086ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ABA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086ABA: rts
pub fn i_086aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ABC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Critical_086abc  ($086ABC, 232 B, enemies/bosses/fort_scene4_0865be.s)
/// $086ABC: lea.l $86ac2(pc), a1
pub fn i_086abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86AC2;
    16
}
/// $086AC0: move.l a1, (a6)
pub fn i_086ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086AC2: movea.l $c(a6), a0
pub fn i_086ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AC6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086AC6: cmpi.b #$ff, $21(a0)
pub fn i_086ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ACC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086ACC: beq.w $86b90
pub fn i_086acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86B90; return 10; }
    8
}
/// $086AD0: jsr $2783a.l
pub fn i_086ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86AD6);
    cpu.pc = 0x2783A;
    16
}
/// $086AD6: jsr $28d70.l
pub fn i_086ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86ADC);
    cpu.pc = 0x28D70;
    16
}
/// $086ADC: jsr $2870a.l
pub fn i_086adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86AE2);
    cpu.pc = 0x2870A;
    16
}
/// $086AE2: bcc.w $86afa
pub fn i_086ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86AFA; return 10; }
    8
}
/// $086AE6: move.w #$108d, d0
pub fn i_086ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086AEA: jsr $2352.l
pub fn i_086aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86AF0);
    cpu.pc = 0x2352;
    16
}
/// $086AF0: bclr.b #$3, $13(a6)
pub fn i_086af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AF6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086AF6: jsr $8848c(pc)
pub fn i_086af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86AFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86AFA);
    cpu.pc = 0x8848C;
    16
}
/// $086AFA: jsr $28758.l
pub fn i_086afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B00);
    cpu.pc = 0x28758;
    16
}
/// $086B00: bcc.w $86ba2
pub fn i_086b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B04;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86BA2; return 10; }
    8
}
/// $086B04: bclr.b #$0, $13(a6)
pub fn i_086b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B0A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086B0A: lea.l $ffff.w, a0
pub fn i_086b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B0E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $086B0E: move.l a0, $48(a6)
pub fn i_086b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B12;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086B12: btst.b #$0, $21(a6)
pub fn i_086b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B18;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $086B18: beq.w $86b26
pub fn i_086b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B1C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86B26; return 10; }
    8
}
/// $086B1C: lea.l $86ba4(pc), a1
pub fn i_086b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86BA4;
    16
}
/// $086B20: move.l a1, (a6)
pub fn i_086b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086B22: bra.w $86ba2
pub fn i_086b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B26;
    cpu.ir = 0x6000;
    cpu.pc = 0x86BA2;
    10
}
/// $086B26: move.l #$100, d0
pub fn i_086b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B2C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086B2C: jsr $51a28.l
pub fn i_086b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B32);
    cpu.pc = 0x51A28;
    16
}
/// $086B32: move.w #$102f, d0
pub fn i_086b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B36;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086B36: jsr $2352.l
pub fn i_086b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B3C);
    cpu.pc = 0x2352;
    16
}
/// $086B3C: lea.l $78066.l, a1
pub fn i_086b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B42;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78066;
    20
}
/// $086B42: jsr $4ae.l
pub fn i_086b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B48);
    cpu.pc = 0x4AE;
    16
}
/// $086B48: jsr $5dd22.l
pub fn i_086b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B4E);
    cpu.pc = 0x5DD22;
    16
}
/// $086B4E: addi.w #$10, $24(a0)
pub fn i_086b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B54;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086B54: movea.l $74(a6), a2
pub fn i_086b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B58;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $086B58: jsr $5022a.l
pub fn i_086b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B5E);
    cpu.pc = 0x5022A;
    16
}
/// $086B5E: lea.l $2ed05c.l, a1
pub fn i_086b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B64;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED05C;
    20
}
/// $086B64: jsr $77c7e.l
pub fn i_086b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B6A);
    cpu.pc = 0x77C7E;
    16
}
/// $086B6A: addi.w #$10, $24(a0)
pub fn i_086b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B70;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086B70: movea.l $7c(a6), a1
pub fn i_086b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B74;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086B74: move.l a1, d0
pub fn i_086b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B76;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086B76: cmpi.l #$ffffffff, d0
pub fn i_086b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B7C;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $086B7C: beq.w $86b90
pub fn i_086b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86B90; return 10; }
    8
}
/// $086B80: move.w #$82, d0
pub fn i_086b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086B84: move.b #$0, $82(a6)
pub fn i_086b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B8A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086B8A: jsr $4429e.l
pub fn i_086b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B90);
    cpu.pc = 0x4429E;
    16
}
/// $086B90: lea.l $2edd48.l, a1
pub fn i_086b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086B96: jsr $43fac.l
pub fn i_086b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B9C);
    cpu.pc = 0x43FAC;
    16
}
/// $086B9C: jmp $518.l
pub fn i_086b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BA2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086BA2: rts
pub fn i_086ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Collapse_086ba4  ($086BA4, 190 B, enemies/bosses/fort_scene4_0865be.s)
/// $086BA4: lea.l $86c62(pc), a1
pub fn i_086ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86C62;
    16
}
/// $086BA8: jsr $4ae.l
pub fn i_086ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BAE);
    cpu.pc = 0x4AE;
    16
}
/// $086BAE: jsr $5dd22.l
pub fn i_086bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BB4);
    cpu.pc = 0x5DD22;
    16
}
/// $086BB4: lea.l $86bba(pc), a1
pub fn i_086bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86BBA;
    16
}
/// $086BB8: move.l a1, (a6)
pub fn i_086bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086BBA: movea.l $c(a6), a0
pub fn i_086bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BBE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086BBE: cmpi.b #$ff, $21(a0)
pub fn i_086bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BC4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086BC4: beq.w $86c4e
pub fn i_086bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BC8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86C4E; return 10; }
    8
}
/// $086BC8: jsr $2783a.l
pub fn i_086bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BCE);
    cpu.pc = 0x2783A;
    16
}
/// $086BCE: jsr $28d70.l
pub fn i_086bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BD4);
    cpu.pc = 0x28D70;
    16
}
/// $086BD4: move.b $21(a6), d0
pub fn i_086bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BD8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086BD8: andi.b #$f, d0
pub fn i_086bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BDC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $086BDC: cmpi.b #$f, d0
pub fn i_086bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BE0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $086BE0: bne.w $86c60
pub fn i_086be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86C60; return 10; }
    8
}
/// $086BE4: move.l #$100, d0
pub fn i_086be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BEA;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086BEA: jsr $51a28.l
pub fn i_086bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BF0);
    cpu.pc = 0x51A28;
    16
}
/// $086BF0: move.w #$102c, d0
pub fn i_086bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BF4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086BF4: jsr $2352.l
pub fn i_086bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BFA);
    cpu.pc = 0x2352;
    16
}
/// $086BFA: lea.l $78066.l, a1
pub fn i_086bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78066;
    20
}
/// $086C00: jsr $4ae.l
pub fn i_086c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C06);
    cpu.pc = 0x4AE;
    16
}
/// $086C06: jsr $5dd22.l
pub fn i_086c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C0C);
    cpu.pc = 0x5DD22;
    16
}
/// $086C0C: addi.w #$10, $24(a0)
pub fn i_086c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C12;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086C12: movea.l $74(a6), a2
pub fn i_086c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C16;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $086C16: jsr $5022a.l
pub fn i_086c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C1C);
    cpu.pc = 0x5022A;
    16
}
/// $086C1C: lea.l $2ed05c.l, a1
pub fn i_086c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C22;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED05C;
    20
}
/// $086C22: jsr $77c7e.l
pub fn i_086c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C28);
    cpu.pc = 0x77C7E;
    16
}
/// $086C28: addi.w #$10, $24(a0)
pub fn i_086c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C2E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086C2E: movea.l $7c(a6), a1
pub fn i_086c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C32;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086C32: move.l a1, d0
pub fn i_086c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C34;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086C34: cmpi.l #$ffffffff, d0
pub fn i_086c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C3A;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $086C3A: beq.w $86c4e
pub fn i_086c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86C4E; return 10; }
    8
}
/// $086C3E: move.w #$82, d0
pub fn i_086c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086C42: move.b #$0, $82(a6)
pub fn i_086c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C48;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086C48: jsr $4429e.l
pub fn i_086c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C4E);
    cpu.pc = 0x4429E;
    16
}
/// $086C4E: lea.l $2edd48.l, a1
pub fn i_086c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C54;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086C54: jsr $43fac.l
pub fn i_086c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C5A);
    cpu.pc = 0x43FAC;
    16
}
/// $086C5A: jmp $518.l
pub fn i_086c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C60;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086C60: rts
pub fn i_086c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Fragment_086c62  ($086C62, 110 B, enemies/bosses/fort_scene4_0865be.s)
/// $086C62: move.w #$6c, d1
pub fn i_086c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C66;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086C66: jsr $236e.l
pub fn i_086c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C6C);
    cpu.pc = 0x236E;
    16
}
/// $086C6C: move.b #$ff, $32(a6)
pub fn i_086c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C72;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086C72: move.b #$ff, $33(a6)
pub fn i_086c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086C78: move.b #$0, $3a(a6)
pub fn i_086c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C7E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086C7E: move.w #$0, $38(a6)
pub fn i_086c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C84;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086C84: addi.w #$8, $24(a6)
pub fn i_086c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C8A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086C8A: lea.l $2eb2e4.l, a0
pub fn i_086c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C90;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB2E4;
    20
}
/// $086C90: jsr $28cd4.l
pub fn i_086c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C96);
    cpu.pc = 0x28CD4;
    16
}
/// $086C96: lea.l $86c9c(pc), a1
pub fn i_086c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86C9C;
    16
}
/// $086C9A: move.l a1, (a6)
pub fn i_086c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086C9C: movea.l $c(a6), a0
pub fn i_086c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CA0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086CA0: movea.l $c(a0), a0
pub fn i_086ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CA4;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086CA4: cmpi.b #$ff, $21(a0)
pub fn i_086ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CAA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086CAA: beq.w $86cc8
pub fn i_086caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86CC8; return 10; }
    8
}
/// $086CAE: jsr $2783a.l
pub fn i_086cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86CB4);
    cpu.pc = 0x2783A;
    16
}
/// $086CB4: jsr $28d70.l
pub fn i_086cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86CBA);
    cpu.pc = 0x28D70;
    16
}
/// $086CBA: bcc.w $86cce
pub fn i_086cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86CCE; return 10; }
    8
}
/// $086CBE: movea.l $c(a6), a0
pub fn i_086cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CC2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086CC2: ori.b #$f, $21(a0)
pub fn i_086cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CC8;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086CC8: jmp $518.l
pub fn i_086cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CCE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086CCE: rts
pub fn i_086cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Init_086cd0  ($086CD0, 222 B, enemies/bosses/fort_scene4_0865be.s)
/// $086CD0: move.w #$e7, d1
pub fn i_086cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CD4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086CD4: jsr $236e.l
pub fn i_086cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86CDA);
    cpu.pc = 0x236E;
    16
}
/// $086CDA: move.b #$0, $21(a6)
pub fn i_086cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CE0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086CE0: bra.w $86d34
pub fn i_086ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086CE4: move.w #$e8, d1
pub fn i_086ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CE8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086CE8: jsr $236e.l
pub fn i_086ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86CEE);
    cpu.pc = 0x236E;
    16
}
/// $086CEE: move.b #$1, $21(a6)
pub fn i_086cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086CF4: bra.w $86d34
pub fn i_086cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086CF8: move.w #$e9, d1
pub fn i_086cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CFC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086CFC: jsr $236e.l
pub fn i_086cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D02);
    cpu.pc = 0x236E;
    16
}
/// $086D02: move.b #$2, $21(a6)
pub fn i_086d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D08;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D08: bra.w $86d34
pub fn i_086d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086D0C: move.w #$ea, d1
pub fn i_086d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D10;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086D10: jsr $236e.l
pub fn i_086d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D16);
    cpu.pc = 0x236E;
    16
}
/// $086D16: move.b #$3, $21(a6)
pub fn i_086d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D1C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D1C: bra.w $86d34
pub fn i_086d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D20;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086D20: move.w #$eb, d1
pub fn i_086d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D24;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086D24: jsr $236e.l
pub fn i_086d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D2A);
    cpu.pc = 0x236E;
    16
}
/// $086D2A: move.b #$4, $21(a6)
pub fn i_086d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D30: bra.w $86d34
pub fn i_086d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D34;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086D34: move.b #$ff, $32(a6)
pub fn i_086d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D3A: move.b #$ff, $33(a6)
pub fn i_086d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D40: move.b #$0, $3a(a6)
pub fn i_086d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D46: move.w #$14, $66(a6)
pub fn i_086d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D4C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086D4C: move.w #$0, $38(a6)
pub fn i_086d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D52;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086D52: lea.l $86d58(pc), a1
pub fn i_086d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86D58;
    16
}
/// $086D56: move.l a1, (a6)
pub fn i_086d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086D58: movea.l $c(a6), a0
pub fn i_086d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D5C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086D5C: cmpi.b #$ff, $21(a0)
pub fn i_086d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D62;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086D62: beq.w $86da8
pub fn i_086d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86DA8; return 10; }
    8
}
/// $086D66: jsr $2783a.l
pub fn i_086d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D6C);
    cpu.pc = 0x2783A;
    16
}
/// $086D6C: jsr $2870a.l
pub fn i_086d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D72);
    cpu.pc = 0x2870A;
    16
}
/// $086D72: bcc.w $86d8c
pub fn i_086d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86D8C; return 10; }
    8
}
/// $086D76: lea.l $5e766.l, a0
pub fn i_086d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $086D7C: jsr $5e770.l
pub fn i_086d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D82);
    cpu.pc = 0x5E770;
    16
}
/// $086D82: bclr.b #$3, $13(a6)
pub fn i_086d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086D88: jsr $8848c(pc)
pub fn i_086d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86D8C);
    cpu.pc = 0x8848C;
    16
}
/// $086D8C: jsr $28758.l
pub fn i_086d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D92);
    cpu.pc = 0x28758;
    16
}
/// $086D92: bcc.w $86dae
pub fn i_086d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86DAE; return 10; }
    8
}
/// $086D96: lea.l $ffff.w, a0
pub fn i_086d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D9A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $086D9A: move.l a0, $48(a6)
pub fn i_086d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D9E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086D9E: lea.l $86db4(pc), a1
pub fn i_086d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86DB4;
    16
}
/// $086DA2: move.l a1, (a6)
pub fn i_086da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086DA4: bra.w $86dae
pub fn i_086da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x86DAE;
    10
}
/// $086DA8: jmp $518.l
pub fn i_086da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_086dae  ($086DAE, 6 B, thunks/jsr_pc_thunks.c)
/// $086DAE: jsr $88438(pc)
pub fn i_086dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86DB2);
    cpu.pc = 0x88438;
    16
}
/// $086DB2: rts
pub fn i_086db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Burst_086db4  ($086DB4, 150 B, enemies/bosses/fort_scene4_0865be.s)
/// $086DB4: move.l #$100, d0
pub fn i_086db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DBA;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086DBA: jsr $51a28.l
pub fn i_086dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86DC0);
    cpu.pc = 0x51A28;
    16
}
/// $086DC0: move.w #$1039, d0
pub fn i_086dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DC4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1039;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086DC4: jsr $2352.l
pub fn i_086dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86DCA);
    cpu.pc = 0x2352;
    16
}
/// $086DCA: clr.l d0
pub fn i_086dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DCC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086DCC: move.b $21(a6), d0
pub fn i_086dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DD0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086DD0: movea.l #$2eb6ba, a0
pub fn i_086dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DD6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EB6BA;
    cpu.a[0] = v1;
    4
}
/// $086DD6: lsl.w #$2, d0
pub fn i_086dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DD8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $086DD8: movea.l (a0, d0.w), a0
pub fn i_086dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DDC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $086DDC: cmpa.l #$ffffffff, a0
pub fn i_086ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DE2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $086DE2: beq.w $86dec
pub fn i_086de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DE6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86DEC; return 10; }
    8
}
/// $086DE6: jsr $28cd4.l
pub fn i_086de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86DEC);
    cpu.pc = 0x28CD4;
    16
}
/// $086DEC: lea.l $86df2(pc), a1
pub fn i_086dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86DF2;
    16
}
/// $086DF0: move.l a1, (a6)
pub fn i_086df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086DF2: movea.l $c(a6), a0
pub fn i_086df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DF6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086DF6: cmpi.b #$ff, $21(a0)
pub fn i_086df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DFC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086DFC: beq.w $86e42
pub fn i_086dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86E42; return 10; }
    8
}
/// $086E00: jsr $2783a.l
pub fn i_086e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E06);
    cpu.pc = 0x2783A;
    16
}
/// $086E06: jsr $28d70.l
pub fn i_086e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E0C);
    cpu.pc = 0x28D70;
    16
}
/// $086E0C: bcc.w $86e48
pub fn i_086e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86E48; return 10; }
    8
}
/// $086E10: move.w #$102f, d0
pub fn i_086e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E14;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086E14: jsr $2352.l
pub fn i_086e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E1A);
    cpu.pc = 0x2352;
    16
}
/// $086E1A: lea.l $7808a.l, a1
pub fn i_086e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $086E20: jsr $4ae.l
pub fn i_086e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E26);
    cpu.pc = 0x4AE;
    16
}
/// $086E26: jsr $5dd22.l
pub fn i_086e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E2C);
    cpu.pc = 0x5DD22;
    16
}
/// $086E2C: movea.l $74(a6), a2
pub fn i_086e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E30;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $086E30: jsr $5022a.l
pub fn i_086e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E36);
    cpu.pc = 0x5022A;
    16
}
/// $086E36: lea.l $2ed05c.l, a1
pub fn i_086e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E3C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED05C;
    20
}
/// $086E3C: jsr $77c7e.l
pub fn i_086e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E42);
    cpu.pc = 0x77C7E;
    16
}
/// $086E42: jmp $518.l
pub fn i_086e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086E48: rts
pub fn i_086e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_Init_086e4a  ($086E4A, 282 B, enemies/bosses/fort_scene4_0865be.s)
/// $086E4A: move.w #$30, $70(a6)
pub fn i_086e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086E50: move.b #$0, $20(a6)
pub fn i_086e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E56;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086E56: move.b #$1, $21(a6)
pub fn i_086e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086E5C: lea.l $ede9e.l, a1
pub fn i_086e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E62;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDE9E;
    20
}
/// $086E62: move.l a1, $7c(a6)
pub fn i_086e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E66;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086E66: lea.l $edcb2.l, a1
pub fn i_086e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDCB2;
    20
}
/// $086E6C: move.l a1, $8c(a6)
pub fn i_086e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E70;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086E70: bra.w $86f64
pub fn i_086e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E74;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086E74: move.w #$30, $70(a6)
pub fn i_086e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E7A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086E7A: move.b #$1, $20(a6)
pub fn i_086e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E80;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086E80: move.b #$1, $21(a6)
pub fn i_086e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E86;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086E86: lea.l $ededa.l, a1
pub fn i_086e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDEDA;
    20
}
/// $086E8C: move.l a1, $7c(a6)
pub fn i_086e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E90;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086E90: lea.l $edd0a.l, a1
pub fn i_086e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDD0A;
    20
}
/// $086E96: move.l a1, $8c(a6)
pub fn i_086e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E9A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086E9A: bra.w $86f64
pub fn i_086e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086E9E: move.w #$30, $70(a6)
pub fn i_086e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086EA4: move.b #$2, $20(a6)
pub fn i_086ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086EAA: move.b #$1, $21(a6)
pub fn i_086eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086EB0: lea.l $edefa.l, a1
pub fn i_086eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EB6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDEFA;
    20
}
/// $086EB6: move.l a1, $7c(a6)
pub fn i_086eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EBA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086EBA: lea.l $edd86.l, a1
pub fn i_086eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EC0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDD86;
    20
}
/// $086EC0: move.l a1, $8c(a6)
pub fn i_086ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EC4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086EC4: bra.w $86f64
pub fn i_086ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086EC8: move.w #$20, $70(a6)
pub fn i_086ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ECE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086ECE: move.b #$3, $20(a6)
pub fn i_086ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ED4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086ED4: move.b #$1, $21(a6)
pub fn i_086ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086EDA: lea.l $edf1a.l, a1
pub fn i_086eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EE0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDF1A;
    20
}
/// $086EE0: move.l a1, $7c(a6)
pub fn i_086ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EE4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086EE4: lea.l $ede02.l, a1
pub fn i_086ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EEA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDE02;
    20
}
/// $086EEA: move.l a1, $8c(a6)
pub fn i_086eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EEE;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086EEE: bra.w $86f64
pub fn i_086eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086EF2: move.w #$20, $70(a6)
pub fn i_086ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086EF8: move.b #$4, $20(a6)
pub fn i_086ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EFE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086EFE: move.b #$1, $21(a6)
pub fn i_086efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F04;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086F04: lea.l $edf52.l, a1
pub fn i_086f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F0A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDF52;
    20
}
/// $086F0A: move.l a1, $7c(a6)
pub fn i_086f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F0E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086F0E: lea.l $ede7e.l, a1
pub fn i_086f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F14;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDE7E;
    20
}
/// $086F14: move.l a1, $8c(a6)
pub fn i_086f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F18;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086F18: bra.w $86f64
pub fn i_086f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086F1C: movea.l $3c(a6), a1
pub fn i_086f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F20;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086F20: jsr $2942a.l
pub fn i_086f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86F26);
    cpu.pc = 0x2942A;
    16
}
/// $086F26: move.w #$30, $70(a6)
pub fn i_086f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086F2C: move.b #$5, $20(a6)
pub fn i_086f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086F32: lea.l $edf72.l, a1
pub fn i_086f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F38;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDF72;
    20
}
/// $086F38: move.l a1, $7c(a6)
pub fn i_086f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F3C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086F3C: bra.w $86f64
pub fn i_086f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F40;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086F40: movea.l $3c(a6), a1
pub fn i_086f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F44;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086F44: jsr $2942a.l
pub fn i_086f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86F4A);
    cpu.pc = 0x2942A;
    16
}
/// $086F4A: move.w #$30, $70(a6)
pub fn i_086f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086F50: move.b #$6, $20(a6)
pub fn i_086f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F56;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086F56: lea.l $edfaa.l, a1
pub fn i_086f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F5C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDFAA;
    20
}
/// $086F5C: move.l a1, $7c(a6)
pub fn i_086f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F60;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086F60: bra.w $86f64
pub fn i_086f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F64;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
// ===== Hatch_WaitClosed_086f64  ($086F64, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $086F64: move.w #$5a, $72(a6)
pub fn i_086f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086F6A: lea.l $86f70(pc), a1
pub fn i_086f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86F70;
    16
}
/// $086F6E: move.l a1, (a6)
pub fn i_086f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086F70: jsr $2783a.l
pub fn i_086f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86F76);
    cpu.pc = 0x2783A;
    16
}
/// $086F76: subq.w #$1, $72(a6)
pub fn i_086f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F7A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $086F7A: bne.w $86f84
pub fn i_086f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86F84; return 10; }
    8
}
/// $086F7E: lea.l $86fb4(pc), a1
pub fn i_086f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86FB4;
    16
}
/// $086F82: move.l a1, (a6)
pub fn i_086f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086F84: cmpi.b #$1, $21(a6)
pub fn i_086f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F8A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $086F8A: bne.w $86fa2
pub fn i_086f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86FA2; return 10; }
    8
}
/// $086F8E: movea.l $c(a6), a0
pub fn i_086f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F92;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086F92: cmpi.b #$ff, $21(a0)
pub fn i_086f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F98;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086F98: bne.w $86fa2
pub fn i_086f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86FA2; return 10; }
    8
}
/// $086F9C: lea.l $87108(pc), a1
pub fn i_086f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87108;
    16
}
/// $086FA0: move.l a1, (a6)
pub fn i_086fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086FA2: jsr $4fa70.l
pub fn i_086fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FA8);
    cpu.pc = 0x4FA70;
    16
}
/// $086FA8: bcc.w $86fb2
pub fn i_086fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86FB2; return 10; }
    8
}
/// $086FAC: jmp $518.l
pub fn i_086fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FB2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086FB2: rts
pub fn i_086fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_Opening_086fb4  ($086FB4, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $086FB4: clr.l d0
pub fn i_086fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FB6;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086FB6: move.b $20(a6), d0
pub fn i_086fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FBA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086FBA: movea.l #$2ebfae, a0
pub fn i_086fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FC0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EBFAE;
    cpu.a[0] = v1;
    4
}
/// $086FC0: lsl.w #$2, d0
pub fn i_086fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FC2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $086FC2: movea.l (a0, d0.w), a0
pub fn i_086fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FC6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $086FC6: cmpa.l #$ffffffff, a0
pub fn i_086fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FCC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $086FCC: beq.w $86fd6
pub fn i_086fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86FD6; return 10; }
    8
}
/// $086FD0: jsr $28cd4.l
pub fn i_086fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FD6);
    cpu.pc = 0x28CD4;
    16
}
/// $086FD6: lea.l $86fdc(pc), a1
pub fn i_086fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86FDC;
    16
}
/// $086FDA: move.l a1, (a6)
pub fn i_086fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086FDC: jsr $2783a.l
pub fn i_086fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FE2);
    cpu.pc = 0x2783A;
    16
}
/// $086FE2: jsr $28d70.l
pub fn i_086fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FE8);
    cpu.pc = 0x28D70;
    16
}
/// $086FE8: bcc.w $86ff2
pub fn i_086fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86FF2; return 10; }
    8
}
/// $086FEC: lea.l $87004(pc), a1
pub fn i_086fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87004;
    16
}
/// $086FF0: move.l a1, (a6)
pub fn i_086ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086FF2: jsr $4fa70.l
pub fn i_086ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FF8);
    cpu.pc = 0x4FA70;
    16
}
/// $086FF8: bcc.w $87002
pub fn i_086ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87002; return 10; }
    8
}
/// $086FFC: jmp $518.l
pub fn i_086ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87002;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087002: rts
pub fn i_087002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87004;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_SpawnTroops_087004  ($087004, 100 B, enemies/bosses/fort_scene4_0865be.s)
/// $087004: move.w #$28, $72(a6)
pub fn i_087004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8700A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08700A: movea.l $7c(a6), a1
pub fn i_08700a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8700E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08700E: move.w #$82, d0
pub fn i_08700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87012;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087012: move.b #$0, $82(a6)
pub fn i_087012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87018;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087018: jsr $4429e.l
pub fn i_087018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8701E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8701E);
    cpu.pc = 0x4429E;
    16
}
/// $08701E: lea.l $87024(pc), a1
pub fn i_08701e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87022;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87024;
    16
}
/// $087022: move.l a1, (a6)
pub fn i_087022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87024;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087024: jsr $2783a.l
pub fn i_087024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8702A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8702A);
    cpu.pc = 0x2783A;
    16
}
/// $08702A: jsr $86552(pc)
pub fn i_08702a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8702E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8702E);
    cpu.pc = 0x86552;
    16
}
/// $08702E: bcs.w $87038
pub fn i_08702e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87032;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x87038; return 10; }
    8
}
/// $087032: lea.l $87068(pc), a1
pub fn i_087032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87036;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87068;
    16
}
/// $087036: move.l a1, (a6)
pub fn i_087036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87038;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087038: cmpi.b #$1, $21(a6)
pub fn i_087038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8703E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08703E: bne.w $87056
pub fn i_08703e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87042;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x87056; return 10; }
    8
}
/// $087042: movea.l $c(a6), a0
pub fn i_087042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87046;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $087046: cmpi.b #$ff, $21(a0)
pub fn i_087046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8704C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08704C: bne.w $87056
pub fn i_08704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87050;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x87056; return 10; }
    8
}
/// $087050: lea.l $87130(pc), a1
pub fn i_087050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87054;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87130;
    16
}
/// $087054: move.l a1, (a6)
pub fn i_087054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87056;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087056: jsr $4fa70.l
pub fn i_087056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8705C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8705C);
    cpu.pc = 0x4FA70;
    16
}
/// $08705C: bcc.w $87066
pub fn i_08705c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87060;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87066; return 10; }
    8
}
/// $087060: jmp $518.l
pub fn i_087060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87066;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087066: rts
pub fn i_087066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87068;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_WaitOpen_087068  ($087068, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $087068: move.w #$50, $72(a6)
pub fn i_087068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8706E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08706E: lea.l $87074(pc), a1
pub fn i_08706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87072;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87074;
    16
}
/// $087072: move.l a1, (a6)
pub fn i_087072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87074;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087074: jsr $2783a.l
pub fn i_087074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8707A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8707A);
    cpu.pc = 0x2783A;
    16
}
/// $08707A: subq.w #$1, $72(a6)
pub fn i_08707a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8707E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08707E: bne.w $87088
pub fn i_08707e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87082;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x87088; return 10; }
    8
}
/// $087082: lea.l $870b8(pc), a1
pub fn i_087082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87086;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x870B8;
    16
}
/// $087086: move.l a1, (a6)
pub fn i_087086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87088;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087088: cmpi.b #$1, $21(a6)
pub fn i_087088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8708E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08708E: bne.w $870a6
pub fn i_08708e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x870A6; return 10; }
    8
}
/// $087092: movea.l $c(a6), a0
pub fn i_087092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87096;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $087096: cmpi.b #$ff, $21(a0)
pub fn i_087096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8709C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08709C: bne.w $870a6
pub fn i_08709c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x870A6; return 10; }
    8
}
/// $0870A0: lea.l $87130(pc), a1
pub fn i_0870a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87130;
    16
}
/// $0870A4: move.l a1, (a6)
pub fn i_0870a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0870A6: jsr $4fa70.l
pub fn i_0870a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870AC);
    cpu.pc = 0x4FA70;
    16
}
/// $0870AC: bcc.w $870b6
pub fn i_0870ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x870B6; return 10; }
    8
}
/// $0870B0: jmp $518.l
pub fn i_0870b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0870B6: rts
pub fn i_0870b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_Closing_0870b8  ($0870B8, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $0870B8: clr.l d0
pub fn i_0870b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870BA;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0870BA: move.b $20(a6), d0
pub fn i_0870ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870BE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0870BE: movea.l #$2ebfca, a0
pub fn i_0870be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870C4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EBFCA;
    cpu.a[0] = v1;
    4
}
/// $0870C4: lsl.w #$2, d0
pub fn i_0870c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870C6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0870C6: movea.l (a0, d0.w), a0
pub fn i_0870c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870CA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0870CA: cmpa.l #$ffffffff, a0
pub fn i_0870ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870D0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0870D0: beq.w $870da
pub fn i_0870d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x870DA; return 10; }
    8
}
/// $0870D4: jsr $28cd4.l
pub fn i_0870d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0870DA: lea.l $870e0(pc), a1
pub fn i_0870da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x870E0;
    16
}
/// $0870DE: move.l a1, (a6)
pub fn i_0870de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0870E0: jsr $2783a.l
pub fn i_0870e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870E6);
    cpu.pc = 0x2783A;
    16
}
/// $0870E6: jsr $28d70.l
pub fn i_0870e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870EC);
    cpu.pc = 0x28D70;
    16
}
/// $0870EC: bcc.w $870f6
pub fn i_0870ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x870F6; return 10; }
    8
}
/// $0870F0: lea.l $86f64(pc), a1
pub fn i_0870f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86F64;
    16
}
/// $0870F4: move.l a1, (a6)
pub fn i_0870f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0870F6: jsr $4fa70.l
pub fn i_0870f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870FC);
    cpu.pc = 0x4FA70;
    16
}
/// $0870FC: bcc.w $87106
pub fn i_0870fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87100;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87106; return 10; }
    8
}
/// $087100: jmp $518.l
pub fn i_087100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87106;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087106: rts
pub fn i_087106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87108;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_FinalOpen_087108  ($087108, 40 B, enemies/bosses/fort_scene4_0865be.s)
/// $087108: clr.l d0
pub fn i_087108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8710A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08710A: move.b $20(a6), d0
pub fn i_08710a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8710E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08710E: movea.l #$2ebfae, a0
pub fn i_08710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87114;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EBFAE;
    cpu.a[0] = v1;
    4
}
/// $087114: lsl.w #$2, d0
pub fn i_087114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87116;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $087116: movea.l (a0, d0.w), a0
pub fn i_087116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8711A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08711A: cmpa.l #$ffffffff, a0
pub fn i_08711a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87120;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $087120: beq.w $8712a
pub fn i_087120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87124;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8712A; return 10; }
    8
}
/// $087124: jsr $28cd4.l
pub fn i_087124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8712A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8712A);
    cpu.pc = 0x28CD4;
    16
}
/// $08712A: lea.l $87130(pc), a1
pub fn i_08712a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8712E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87130;
    16
}
/// $08712E: move.l a1, (a6)
pub fn i_08712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87130;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Hatch_FinalSpawn_087130  ($087130, 58 B, enemies/bosses/fort_scene4_0865be.s)
/// $087130: jsr $2783a.l
pub fn i_087130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87136);
    cpu.pc = 0x2783A;
    16
}
/// $087136: jsr $28d70.l
pub fn i_087136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8713C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8713C);
    cpu.pc = 0x28D70;
    16
}
/// $08713C: bcc.w $87158
pub fn i_08713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87140;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87158; return 10; }
    8
}
/// $087140: movea.l $8c(a6), a1
pub fn i_087140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87144;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087144: move.w #$82, d0
pub fn i_087144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87148;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087148: move.b #$0, $82(a6)
pub fn i_087148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8714E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08714E: jsr $4429e.l
pub fn i_08714e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87154);
    cpu.pc = 0x4429E;
    16
}
/// $087154: bra.w $87162
pub fn i_087154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87158;
    cpu.ir = 0x6000;
    cpu.pc = 0x87162;
    10
}
/// $087158: jsr $4fa70.l
pub fn i_087158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8715E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8715E);
    cpu.pc = 0x4FA70;
    16
}
/// $08715E: bcc.w $87168
pub fn i_08715e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87162;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87168; return 10; }
    8
}
/// $087162: jmp $518.l
pub fn i_087162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87168;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087168: rts
pub fn i_087168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8716A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable_A_08716a  ($08716A, 144 B, enemies/bosses/fort_scene4_0865be.s)
/// $08716A: movea.l $3c(a6), a1
pub fn i_08716a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8716E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08716E: jsr $2942a.l
pub fn i_08716e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87174);
    cpu.pc = 0x2942A;
    16
}
/// $087174: move.w #$20, $70(a6)
pub fn i_087174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8717A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08717A: lea.l $2ec726.l, a0
pub fn i_08717a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87180;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC726;
    20
}
/// $087180: move.l a0, $48(a6)
pub fn i_087180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87184;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087184: lea.l $8718a(pc), a1
pub fn i_087184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87188;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8718A;
    16
}
/// $087188: move.l a1, (a6)
pub fn i_087188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8718A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08718A: jsr $2783a.l
pub fn i_08718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87190;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87190);
    cpu.pc = 0x2783A;
    16
}
/// $087190: jsr $2870a.l
pub fn i_087190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87196;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87196);
    cpu.pc = 0x2870A;
    16
}
/// $087196: bcc.w $871ac
pub fn i_087196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8719A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x871AC; return 10; }
    8
}
/// $08719A: lea.l $5e766.l, a0
pub fn i_08719a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0871A0: jsr $5e770.l
pub fn i_0871a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871A6);
    cpu.pc = 0x5E770;
    16
}
/// $0871A6: bclr.b #$3, $13(a6)
pub fn i_0871a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871AC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0871AC: jsr $28758.l
pub fn i_0871ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871B2);
    cpu.pc = 0x28758;
    16
}
/// $0871B2: bcc.w $871e8
pub fn i_0871b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x871E8; return 10; }
    8
}
/// $0871B6: move.w #$10a6, d0
pub fn i_0871b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0871BA: jsr $2352.l
pub fn i_0871ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871C0);
    cpu.pc = 0x2352;
    16
}
/// $0871C0: lea.l $2ed0ae.l, a1
pub fn i_0871c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0AE;
    20
}
/// $0871C6: jsr $77c7e.l
pub fn i_0871c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871CC);
    cpu.pc = 0x77C7E;
    16
}
/// $0871CC: lea.l $2ed878.l, a2
pub fn i_0871cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871D2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED878;
    20
}
/// $0871D2: jsr $5022a.l
pub fn i_0871d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871D8);
    cpu.pc = 0x5022A;
    16
}
/// $0871D8: lea.l $2ed864.l, a2
pub fn i_0871d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED864;
    20
}
/// $0871DE: jsr $5022a.l
pub fn i_0871de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871E4);
    cpu.pc = 0x5022A;
    16
}
/// $0871E4: bra.w $871f2
pub fn i_0871e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x871F2;
    10
}
/// $0871E8: jsr $4fa70.l
pub fn i_0871e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871EE);
    cpu.pc = 0x4FA70;
    16
}
/// $0871EE: bcc.w $871f8
pub fn i_0871ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x871F8; return 10; }
    8
}
/// $0871F2: jmp $518.l
pub fn i_0871f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871F8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0871F8: rts
pub fn i_0871f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable_B_0871fa  ($0871FA, 134 B, enemies/bosses/fort_scene4_0865be.s)
/// $0871FA: movea.l $3c(a6), a1
pub fn i_0871fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871FE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0871FE: jsr $2942a.l
pub fn i_0871fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87204);
    cpu.pc = 0x2942A;
    16
}
/// $087204: move.w #$20, $70(a6)
pub fn i_087204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8720A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08720A: lea.l $2ec726.l, a0
pub fn i_08720a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87210;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC726;
    20
}
/// $087210: move.l a0, $48(a6)
pub fn i_087210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87214;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087214: lea.l $8721a(pc), a1
pub fn i_087214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87218;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8721A;
    16
}
/// $087218: move.l a1, (a6)
pub fn i_087218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8721A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08721A: jsr $2783a.l
pub fn i_08721a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87220);
    cpu.pc = 0x2783A;
    16
}
/// $087220: jsr $2870a.l
pub fn i_087220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87226);
    cpu.pc = 0x2870A;
    16
}
/// $087226: bcc.w $8723c
pub fn i_087226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8722A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8723C; return 10; }
    8
}
/// $08722A: lea.l $5e766.l, a0
pub fn i_08722a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87230;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087230: jsr $5e770.l
pub fn i_087230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87236);
    cpu.pc = 0x5E770;
    16
}
/// $087236: bclr.b #$3, $13(a6)
pub fn i_087236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8723C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08723C: jsr $28758.l
pub fn i_08723c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87242);
    cpu.pc = 0x28758;
    16
}
/// $087242: bcc.w $8726e
pub fn i_087242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87246;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8726E; return 10; }
    8
}
/// $087246: lea.l $2ed0ae.l, a1
pub fn i_087246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8724C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0AE;
    20
}
/// $08724C: jsr $77c7e.l
pub fn i_08724c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87252);
    cpu.pc = 0x77C7E;
    16
}
/// $087252: lea.l $2ed968.l, a2
pub fn i_087252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87258;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED968;
    20
}
/// $087258: jsr $5022a.l
pub fn i_087258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8725E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8725E);
    cpu.pc = 0x5022A;
    16
}
/// $08725E: lea.l $2ed97c.l, a2
pub fn i_08725e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87264;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED97C;
    20
}
/// $087264: jsr $5022a.l
pub fn i_087264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8726A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8726A);
    cpu.pc = 0x5022A;
    16
}
/// $08726A: bra.w $87278
pub fn i_08726a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8726E;
    cpu.ir = 0x6000;
    cpu.pc = 0x87278;
    10
}
/// $08726E: jsr $4fa70.l
pub fn i_08726e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87274);
    cpu.pc = 0x4FA70;
    16
}
/// $087274: bcc.w $8727e
pub fn i_087274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87278;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8727E; return 10; }
    8
}
/// $087278: jmp $518.l
pub fn i_087278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8727E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08727E: rts
pub fn i_08727e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87280;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable_C_087280  ($087280, 122 B, enemies/bosses/fort_scene4_0865be.s)
/// $087280: movea.l $3c(a6), a1
pub fn i_087280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87284;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087284: jsr $2942a.l
pub fn i_087284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8728A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8728A);
    cpu.pc = 0x2942A;
    16
}
/// $08728A: move.w #$20, $70(a6)
pub fn i_08728a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87290;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087290: lea.l $2ec726.l, a0
pub fn i_087290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87296;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC726;
    20
}
/// $087296: move.l a0, $48(a6)
pub fn i_087296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8729A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08729A: lea.l $872a0(pc), a1
pub fn i_08729a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8729E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x872A0;
    16
}
/// $08729E: move.l a1, (a6)
pub fn i_08729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0872A0: jsr $2783a.l
pub fn i_0872a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872A6);
    cpu.pc = 0x2783A;
    16
}
/// $0872A6: jsr $2870a.l
pub fn i_0872a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872AC);
    cpu.pc = 0x2870A;
    16
}
/// $0872AC: bcc.w $872c2
pub fn i_0872ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x872C2; return 10; }
    8
}
/// $0872B0: lea.l $5e766.l, a0
pub fn i_0872b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0872B6: jsr $5e770.l
pub fn i_0872b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872BC);
    cpu.pc = 0x5E770;
    16
}
/// $0872BC: bclr.b #$3, $13(a6)
pub fn i_0872bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872C2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0872C2: jsr $28758.l
pub fn i_0872c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872C8);
    cpu.pc = 0x28758;
    16
}
/// $0872C8: bcc.w $872e8
pub fn i_0872c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x872E8; return 10; }
    8
}
/// $0872CC: lea.l $2ed0ae.l, a1
pub fn i_0872cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0AE;
    20
}
/// $0872D2: jsr $77c7e.l
pub fn i_0872d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872D8);
    cpu.pc = 0x77C7E;
    16
}
/// $0872D8: lea.l $2eda6c.l, a2
pub fn i_0872d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDA6C;
    20
}
/// $0872DE: jsr $5022a.l
pub fn i_0872de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872E4);
    cpu.pc = 0x5022A;
    16
}
/// $0872E4: bra.w $872f2
pub fn i_0872e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x872F2;
    10
}
/// $0872E8: jsr $4fa70.l
pub fn i_0872e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872EE);
    cpu.pc = 0x4FA70;
    16
}
/// $0872EE: bcc.w $872f8
pub fn i_0872ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x872F8; return 10; }
    8
}
/// $0872F2: jmp $518.l
pub fn i_0872f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872F8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0872F8: rts
pub fn i_0872f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Signboard_0872fa  ($0872FA, 108 B, enemies/bosses/fort_scene4_0865be.s)
/// $0872FA: movea.l $3c(a6), a1
pub fn i_0872fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872FE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0872FE: jsr $2942a.l
pub fn i_0872fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87304);
    cpu.pc = 0x2942A;
    16
}
/// $087304: move.w #$20, $70(a6)
pub fn i_087304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8730A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08730A: lea.l $2ebfe6.l, a0
pub fn i_08730a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87310;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EBFE6;
    20
}
/// $087310: jsr $28cd4.l
pub fn i_087310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87316);
    cpu.pc = 0x28CD4;
    16
}
/// $087316: lea.l $8731c(pc), a1
pub fn i_087316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8731A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8731C;
    16
}
/// $08731A: move.l a1, (a6)
pub fn i_08731a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8731C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08731C: jsr $2783a.l
pub fn i_08731c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87322);
    cpu.pc = 0x2783A;
    16
}
/// $087322: jsr $28d70.l
pub fn i_087322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87328);
    cpu.pc = 0x28D70;
    16
}
/// $087328: jsr $2870a.l
pub fn i_087328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8732E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8732E);
    cpu.pc = 0x2870A;
    16
}
/// $08732E: bcc.w $8734e
pub fn i_08732e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87332;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8734E; return 10; }
    8
}
/// $087332: move.w #$10a9, d0
pub fn i_087332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87336;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087336: jsr $2352.l
pub fn i_087336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8733C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8733C);
    cpu.pc = 0x2352;
    16
}
/// $08733C: lea.l $2ebffc.l, a0
pub fn i_08733c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87342;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EBFFC;
    20
}
/// $087342: jsr $28cd4.l
pub fn i_087342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87348);
    cpu.pc = 0x28CD4;
    16
}
/// $087348: bclr.b #$3, $13(a6)
pub fn i_087348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8734E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08734E: move.w #$7fff, $66(a6)
pub fn i_08734e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87354;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087354: jsr $4fa70.l
pub fn i_087354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8735A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8735A);
    cpu.pc = 0x4FA70;
    16
}
/// $08735A: bcc.w $87364
pub fn i_08735a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8735E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87364; return 10; }
    8
}
/// $08735E: jmp $518.l
pub fn i_08735e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87364;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087364: rts
pub fn i_087364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87366;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CrateWall_Spawn13_087366  ($087366, 338 B, enemies/bosses/fort_scene4_0865be.s)
/// $087366: movea.l $3c(a6), a1
pub fn i_087366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8736A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08736A: jsr $2942a.l
pub fn i_08736a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87370);
    cpu.pc = 0x2942A;
    16
}
/// $087370: lea.l $874ba(pc), a1
pub fn i_087370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87374;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x874BA;
    16
}
/// $087374: jsr $4ae.l
pub fn i_087374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8737A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8737A);
    cpu.pc = 0x4AE;
    16
}
/// $08737A: jsr $5dd22.l
pub fn i_08737a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87380;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87380);
    cpu.pc = 0x5DD22;
    16
}
/// $087380: subi.w #$10, $22(a0)
pub fn i_087380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87386;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087386: addi.w #$40, $24(a0)
pub fn i_087386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8738C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08738C: lea.l $874ea(pc), a1
pub fn i_08738c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87390;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x874EA;
    16
}
/// $087390: jsr $4ae.l
pub fn i_087390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87396;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87396);
    cpu.pc = 0x4AE;
    16
}
/// $087396: jsr $5dd22.l
pub fn i_087396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8739C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8739C);
    cpu.pc = 0x5DD22;
    16
}
/// $08739C: addi.w #$10, $22(a0)
pub fn i_08739c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873A2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873A2: addi.w #$a0, $24(a0)
pub fn i_0873a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873A8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873A8: lea.l $8751c(pc), a1
pub fn i_0873a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8751C;
    16
}
/// $0873AC: jsr $4ae.l
pub fn i_0873ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873B2);
    cpu.pc = 0x4AE;
    16
}
/// $0873B2: jsr $5dd22.l
pub fn i_0873b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873B8);
    cpu.pc = 0x5DD22;
    16
}
/// $0873B8: addi.w #$50, $22(a0)
pub fn i_0873b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873BE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873BE: addi.w #$a0, $24(a0)
pub fn i_0873be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873C4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873C4: lea.l $8754e(pc), a1
pub fn i_0873c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8754E;
    16
}
/// $0873C8: jsr $4ae.l
pub fn i_0873c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873CE);
    cpu.pc = 0x4AE;
    16
}
/// $0873CE: jsr $5dd22.l
pub fn i_0873ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873D4);
    cpu.pc = 0x5DD22;
    16
}
/// $0873D4: addi.w #$80, $22(a0)
pub fn i_0873d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873DA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873DA: addi.w #$a0, $24(a0)
pub fn i_0873da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873E0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873E0: lea.l $87580(pc), a1
pub fn i_0873e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87580;
    16
}
/// $0873E4: jsr $4ae.l
pub fn i_0873e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873EA);
    cpu.pc = 0x4AE;
    16
}
/// $0873EA: jsr $5dd22.l
pub fn i_0873ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873F0);
    cpu.pc = 0x5DD22;
    16
}
/// $0873F0: addi.w #$10, $22(a0)
pub fn i_0873f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873F6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873F6: addi.w #$80, $24(a0)
pub fn i_0873f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873FC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873FC: lea.l $875b0(pc), a1
pub fn i_0873fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87400;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x875B0;
    16
}
/// $087400: jsr $4ae.l
pub fn i_087400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87406;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87406);
    cpu.pc = 0x4AE;
    16
}
/// $087406: jsr $5dd22.l
pub fn i_087406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8740C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8740C);
    cpu.pc = 0x5DD22;
    16
}
/// $08740C: addi.w #$60, $22(a0)
pub fn i_08740c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87412;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087412: addi.w #$80, $24(a0)
pub fn i_087412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87418;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087418: lea.l $875e0(pc), a1
pub fn i_087418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8741C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x875E0;
    16
}
/// $08741C: jsr $4ae.l
pub fn i_08741c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87422);
    cpu.pc = 0x4AE;
    16
}
/// $087422: jsr $5dd22.l
pub fn i_087422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87428);
    cpu.pc = 0x5DD22;
    16
}
/// $087428: addi.w #$50, $22(a0)
pub fn i_087428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8742E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08742E: addi.w #$50, $24(a0)
pub fn i_08742e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87434;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087434: lea.l $87610(pc), a1
pub fn i_087434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87438;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87610;
    16
}
/// $087438: jsr $4ae.l
pub fn i_087438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8743E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8743E);
    cpu.pc = 0x4AE;
    16
}
/// $08743E: jsr $5dd22.l
pub fn i_08743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87444;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87444);
    cpu.pc = 0x5DD22;
    16
}
/// $087444: lea.l $87640(pc), a1
pub fn i_087444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87448;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87640;
    16
}
/// $087448: jsr $4ae.l
pub fn i_087448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8744E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8744E);
    cpu.pc = 0x4AE;
    16
}
/// $08744E: jsr $5dd22.l
pub fn i_08744e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87454);
    cpu.pc = 0x5DD22;
    16
}
/// $087454: addi.w #$20, $22(a0)
pub fn i_087454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8745A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08745A: lea.l $87670(pc), a1
pub fn i_08745a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8745E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87670;
    16
}
/// $08745E: jsr $4ae.l
pub fn i_08745e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87464);
    cpu.pc = 0x4AE;
    16
}
/// $087464: jsr $5dd22.l
pub fn i_087464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8746A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8746A);
    cpu.pc = 0x5DD22;
    16
}
/// $08746A: addi.w #$40, $22(a0)
pub fn i_08746a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87470;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087470: lea.l $876a0(pc), a1
pub fn i_087470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87474;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x876A0;
    16
}
/// $087474: jsr $4ae.l
pub fn i_087474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8747A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8747A);
    cpu.pc = 0x4AE;
    16
}
/// $08747A: jsr $5dd22.l
pub fn i_08747a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87480;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87480);
    cpu.pc = 0x5DD22;
    16
}
/// $087480: addi.w #$60, $22(a0)
pub fn i_087480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87486;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087486: lea.l $876d0(pc), a1
pub fn i_087486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8748A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x876D0;
    16
}
/// $08748A: jsr $4ae.l
pub fn i_08748a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87490;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87490);
    cpu.pc = 0x4AE;
    16
}
/// $087490: jsr $5dd22.l
pub fn i_087490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87496;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87496);
    cpu.pc = 0x5DD22;
    16
}
/// $087496: addi.w #$80, $22(a0)
pub fn i_087496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8749C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08749C: lea.l $87700(pc), a1
pub fn i_08749c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87700;
    16
}
/// $0874A0: jsr $4ae.l
pub fn i_0874a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x874A6);
    cpu.pc = 0x4AE;
    16
}
/// $0874A6: jsr $5dd22.l
pub fn i_0874a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x874AC);
    cpu.pc = 0x5DD22;
    16
}
/// $0874AC: addi.w #$a0, $22(a0)
pub fn i_0874ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874B2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0874B2: jmp $518.l
pub fn i_0874b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874B8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0874b8  ($0874B8, 2 B, enemies/bosses/fort_scene4_0865be.s)
/// $0874B8: rts
pub fn i_0874b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CrateWall_Piece00_0874ba  ($0874BA, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0874BA: lea.l $2ed990.l, a1
pub fn i_0874ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874C0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED990;
    20
}
/// $0874C0: move.l a1, $74(a6)
pub fn i_0874c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874C4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874C4: move.l #$ffffffff, $7c(a6)
pub fn i_0874c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874CC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874CC: move.w #$0, $72(a6)
pub fn i_0874cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0874D2: lea.l $2ec822.l, a0
pub fn i_0874d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC822;
    20
}
/// $0874D8: move.l a0, $48(a6)
pub fn i_0874d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874DC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874DC: lea.l $2ecfde.l, a1
pub fn i_0874dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFDE;
    20
}
/// $0874E2: move.l a1, $88(a6)
pub fn i_0874e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874E6;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874E6: bra.w $87730
pub fn i_0874e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece01_0874ea  ($0874EA, 50 B, enemies/bosses/fort_scene4_0865be.s)
/// $0874EA: lea.l $2ed9a4.l, a1
pub fn i_0874ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9A4;
    20
}
/// $0874F0: move.l a1, $74(a6)
pub fn i_0874f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874F4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874F4: lea.l $edfe2.l, a1
pub fn i_0874f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDFE2;
    20
}
/// $0874FA: move.l a1, $7c(a6)
pub fn i_0874fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874FE;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874FE: move.w #$1, $72(a6)
pub fn i_0874fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87504;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087504: lea.l $2ec876.l, a0
pub fn i_087504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8750A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC876;
    20
}
/// $08750A: move.l a0, $48(a6)
pub fn i_08750a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8750E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08750E: lea.l $2ecff0.l, a1
pub fn i_08750e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87514;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFF0;
    20
}
/// $087514: move.l a1, $88(a6)
pub fn i_087514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87518;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087518: bra.w $87730
pub fn i_087518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8751C;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece02_08751c  ($08751C, 50 B, enemies/bosses/fort_scene4_0865be.s)
/// $08751C: lea.l $2ed9b8.l, a1
pub fn i_08751c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87522;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9B8;
    20
}
/// $087522: move.l a1, $74(a6)
pub fn i_087522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87526;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087526: lea.l $ee002.l, a1
pub fn i_087526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8752C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE002;
    20
}
/// $08752C: move.l a1, $7c(a6)
pub fn i_08752c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87530;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087530: move.w #$2, $72(a6)
pub fn i_087530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87536;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087536: lea.l $2ec822.l, a0
pub fn i_087536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8753C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC822;
    20
}
/// $08753C: move.l a0, $48(a6)
pub fn i_08753c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87540;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087540: lea.l $2ecfde.l, a1
pub fn i_087540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87546;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFDE;
    20
}
/// $087546: move.l a1, $88(a6)
pub fn i_087546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8754A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08754A: bra.w $87730
pub fn i_08754a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8754E;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece03_08754e  ($08754E, 50 B, enemies/bosses/fort_scene4_0865be.s)
/// $08754E: lea.l $2ed9cc.l, a1
pub fn i_08754e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87554;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9CC;
    20
}
/// $087554: move.l a1, $74(a6)
pub fn i_087554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87558;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087558: lea.l $ee022.l, a1
pub fn i_087558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8755E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE022;
    20
}
/// $08755E: move.l a1, $7c(a6)
pub fn i_08755e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87562;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087562: move.w #$3, $72(a6)
pub fn i_087562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87568;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087568: lea.l $2ec822.l, a0
pub fn i_087568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8756E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC822;
    20
}
/// $08756E: move.l a0, $48(a6)
pub fn i_08756e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87572;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087572: lea.l $2ecfde.l, a1
pub fn i_087572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87578;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFDE;
    20
}
/// $087578: move.l a1, $88(a6)
pub fn i_087578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8757C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08757C: bra.w $87730
pub fn i_08757c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87580;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece04_087580  ($087580, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $087580: lea.l $2eda80.l, a1
pub fn i_087580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87586;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA80;
    20
}
/// $087586: move.l a1, $74(a6)
pub fn i_087586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8758A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08758A: move.l #$ffffffff, $7c(a6)
pub fn i_08758a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87592;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087592: move.w #$4, $72(a6)
pub fn i_087592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87598;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087598: lea.l $2ec8ca.l, a0
pub fn i_087598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8759E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC8CA;
    20
}
/// $08759E: move.l a0, $48(a6)
pub fn i_08759e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875A2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875A2: lea.l $2ed002.l, a1
pub fn i_0875a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875A8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED002;
    20
}
/// $0875A8: move.l a1, $88(a6)
pub fn i_0875a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875AC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875AC: bra.w $87730
pub fn i_0875ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece05_0875b0  ($0875B0, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0875B0: lea.l $2eda94.l, a1
pub fn i_0875b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875B6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA94;
    20
}
/// $0875B6: move.l a1, $74(a6)
pub fn i_0875b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875BA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875BA: move.l #$ffffffff, $7c(a6)
pub fn i_0875ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875C2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875C2: move.w #$5, $72(a6)
pub fn i_0875c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0875C8: lea.l $2ec8ca.l, a0
pub fn i_0875c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC8CA;
    20
}
/// $0875CE: move.l a0, $48(a6)
pub fn i_0875ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875D2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875D2: lea.l $2ed002.l, a1
pub fn i_0875d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED002;
    20
}
/// $0875D8: move.l a1, $88(a6)
pub fn i_0875d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875DC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875DC: bra.w $87730
pub fn i_0875dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece06_0875e0  ($0875E0, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0875E0: lea.l $2ed9e0.l, a1
pub fn i_0875e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9E0;
    20
}
/// $0875E6: move.l a1, $74(a6)
pub fn i_0875e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875EA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875EA: move.l #$ffffffff, $7c(a6)
pub fn i_0875ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875F2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875F2: move.w #$6, $72(a6)
pub fn i_0875f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0875F8: lea.l $2ec91e.l, a0
pub fn i_0875f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC91E;
    20
}
/// $0875FE: move.l a0, $48(a6)
pub fn i_0875fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87602;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087602: lea.l $2ed014.l, a1
pub fn i_087602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87608;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED014;
    20
}
/// $087608: move.l a1, $88(a6)
pub fn i_087608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8760C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08760C: bra.w $87730
pub fn i_08760c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87610;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece07_087610  ($087610, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $087610: lea.l $2ed9f4.l, a1
pub fn i_087610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87616;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9F4;
    20
}
/// $087616: move.l a1, $74(a6)
pub fn i_087616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8761A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08761A: move.l #$ffffffff, $7c(a6)
pub fn i_08761a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87622;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087622: move.w #$7, $72(a6)
pub fn i_087622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87628;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087628: lea.l $2ec972.l, a0
pub fn i_087628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8762E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC972;
    20
}
/// $08762E: move.l a0, $48(a6)
pub fn i_08762e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87632;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087632: lea.l $2ed026.l, a1
pub fn i_087632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87638;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED026;
    20
}
/// $087638: move.l a1, $88(a6)
pub fn i_087638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8763C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08763C: bra.w $87730
pub fn i_08763c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87640;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece08_087640  ($087640, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $087640: lea.l $2eda08.l, a1
pub fn i_087640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87646;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA08;
    20
}
/// $087646: move.l a1, $74(a6)
pub fn i_087646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8764A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08764A: move.l #$ffffffff, $7c(a6)
pub fn i_08764a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87652;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087652: move.w #$8, $72(a6)
pub fn i_087652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87658;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087658: lea.l $2ec9c6.l, a0
pub fn i_087658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8765E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $08765E: move.l a0, $48(a6)
pub fn i_08765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87662;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087662: lea.l $2ed038.l, a1
pub fn i_087662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87668;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $087668: move.l a1, $88(a6)
pub fn i_087668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8766C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08766C: bra.w $87730
pub fn i_08766c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87670;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece09_087670  ($087670, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $087670: lea.l $2eda1c.l, a1
pub fn i_087670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87676;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA1C;
    20
}
/// $087676: move.l a1, $74(a6)
pub fn i_087676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8767A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08767A: move.l #$ffffffff, $7c(a6)
pub fn i_08767a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87682;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087682: move.w #$9, $72(a6)
pub fn i_087682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87688;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087688: lea.l $2ec9c6.l, a0
pub fn i_087688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8768E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $08768E: move.l a0, $48(a6)
pub fn i_08768e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87692;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087692: lea.l $2ed038.l, a1
pub fn i_087692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87698;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $087698: move.l a1, $88(a6)
pub fn i_087698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8769C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08769C: bra.w $87730
pub fn i_08769c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece10_0876a0  ($0876A0, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0876A0: lea.l $2eda30.l, a1
pub fn i_0876a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876A6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA30;
    20
}
/// $0876A6: move.l a1, $74(a6)
pub fn i_0876a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876AA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876AA: move.l #$ffffffff, $7c(a6)
pub fn i_0876aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876B2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876B2: move.w #$a, $72(a6)
pub fn i_0876b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0876B8: lea.l $2ec9c6.l, a0
pub fn i_0876b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $0876BE: move.l a0, $48(a6)
pub fn i_0876be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876C2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876C2: lea.l $2ed038.l, a1
pub fn i_0876c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876C8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $0876C8: move.l a1, $88(a6)
pub fn i_0876c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876CC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876CC: bra.w $87730
pub fn i_0876cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece11_0876d0  ($0876D0, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0876D0: lea.l $2eda44.l, a1
pub fn i_0876d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876D6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA44;
    20
}
/// $0876D6: move.l a1, $74(a6)
pub fn i_0876d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876DA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876DA: move.l #$ffffffff, $7c(a6)
pub fn i_0876da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876E2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876E2: move.w #$b, $72(a6)
pub fn i_0876e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0876E8: lea.l $2ec9c6.l, a0
pub fn i_0876e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $0876EE: move.l a0, $48(a6)
pub fn i_0876ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876F2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876F2: lea.l $2ed038.l, a1
pub fn i_0876f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876F8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $0876F8: move.l a1, $88(a6)
pub fn i_0876f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876FC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876FC: bra.w $87730
pub fn i_0876fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87700;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece12_087700  ($087700, 206 B, enemies/bosses/fort_scene4_0865be.s)
/// $087700: lea.l $2eda58.l, a1
pub fn i_087700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87706;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA58;
    20
}
/// $087706: move.l a1, $74(a6)
pub fn i_087706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8770A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08770A: move.l #$ffffffff, $7c(a6)
pub fn i_08770a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87712;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087712: move.w #$c, $72(a6)
pub fn i_087712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87718;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087718: lea.l $2ec9c6.l, a0
pub fn i_087718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8771E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $08771E: move.l a0, $48(a6)
pub fn i_08771e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87722;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087722: lea.l $2ed038.l, a1
pub fn i_087722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87728;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $087728: move.l a1, $88(a6)
pub fn i_087728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8772C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08772C: bra.w $87730
pub fn i_08772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87730;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
/// $087730: move.w #$30, $70(a6)
pub fn i_087730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87736;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087736: move.w #$14, $66(a6)
pub fn i_087736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8773C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08773C: move.l $48(a6), $84(a6)
pub fn i_08773c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87742;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $087742: lea.l $87748(pc), a1
pub fn i_087742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87746;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87748;
    16
}
/// $087746: move.l a1, (a6)
pub fn i_087746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87748;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087748: jsr $2783a.l
pub fn i_087748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8774E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8774E);
    cpu.pc = 0x2783A;
    16
}
/// $08774E: jsr $2870a.l
pub fn i_08774e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87754);
    cpu.pc = 0x2870A;
    16
}
/// $087754: bcc.w $8776a
pub fn i_087754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87758;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8776A; return 10; }
    8
}
/// $087758: lea.l $5e766.l, a0
pub fn i_087758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8775E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08775E: jsr $5e770.l
pub fn i_08775e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87764);
    cpu.pc = 0x5E770;
    16
}
/// $087764: bclr.b #$3, $13(a6)
pub fn i_087764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8776A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08776A: jsr $28758.l
pub fn i_08776a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87770);
    cpu.pc = 0x28758;
    16
}
/// $087770: bcc.w $877be
pub fn i_087770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87774;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x877BE; return 10; }
    8
}
/// $087774: lea.l $ffff.w, a0
pub fn i_087774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87778;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $087778: move.l a0, $48(a6)
pub fn i_087778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8777C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08777C: move.w #$1027, d0
pub fn i_08777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87780;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087780: jsr $2352.l
pub fn i_087780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87786);
    cpu.pc = 0x2352;
    16
}
/// $087786: movea.l $74(a6), a2
pub fn i_087786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8778A;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $08778A: jsr $5022a.l
pub fn i_08778a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87790);
    cpu.pc = 0x5022A;
    16
}
/// $087790: movea.l $88(a6), a1
pub fn i_087790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87794;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087794: jsr $77c7e.l
pub fn i_087794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8779A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8779A);
    cpu.pc = 0x77C7E;
    16
}
/// $08779A: cmpi.l #$ffffffff, $7c(a6)
pub fn i_08779a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877A2;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0877A2: beq.w $877c8
pub fn i_0877a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x877C8; return 10; }
    8
}
/// $0877A6: movea.l $7c(a6), a1
pub fn i_0877a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877AA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0877AA: move.w #$82, d0
pub fn i_0877aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877AE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0877AE: move.b #$0, $82(a6)
pub fn i_0877ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0877B4: jsr $4429e.l
pub fn i_0877b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x877BA);
    cpu.pc = 0x4429E;
    16
}
/// $0877BA: bra.w $877c8
pub fn i_0877ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x877C8;
    10
}
/// $0877BE: jsr $4fa70.l
pub fn i_0877be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x877C4);
    cpu.pc = 0x4FA70;
    16
}
/// $0877C4: bcc.w $877ce
pub fn i_0877c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x877CE; return 10; }
    8
}
/// $0877C8: jmp $518.l
pub fn i_0877c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877CE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_0877ce  ($0877CE, 6 B, thunks/jsr_pc_thunks.c)
/// $0877CE: jsr $8846a(pc)
pub fn i_0877ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x877D2);
    cpu.pc = 0x8846A;
    16
}
/// $0877D2: rts
pub fn i_0877d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Init_0877d4  ($0877D4, 106 B, enemies/bosses/fort_scene4_0865be.s)
/// $0877D4: movea.l $3c(a6), a1
pub fn i_0877d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877D8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0877D8: jsr $2942a.l
pub fn i_0877d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x877DE);
    cpu.pc = 0x2942A;
    16
}
/// $0877DE: move.w #$e5, d1
pub fn i_0877de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0877E2: jsr $236e.l
pub fn i_0877e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x877E8);
    cpu.pc = 0x236E;
    16
}
/// $0877E8: move.b #$ff, $32(a6)
pub fn i_0877e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0877EE: move.b #$ff, $33(a6)
pub fn i_0877ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0877F4: move.b #$0, $3a(a6)
pub fn i_0877f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0877FA: move.w #$f0, $70(a6)
pub fn i_0877fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87800;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087800: move.w #$0, $38(a6)
pub fn i_087800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87806;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087806: lea.l $2ec180.l, a0
pub fn i_087806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8780C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC180;
    20
}
/// $08780C: jsr $28cd4.l
pub fn i_08780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87812);
    cpu.pc = 0x28CD4;
    16
}
/// $087812: lea.l $87aa0(pc), a1
pub fn i_087812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87AA0;
    16
}
/// $087816: jsr $4ae.l
pub fn i_087816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8781C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8781C);
    cpu.pc = 0x4AE;
    16
}
/// $08781C: jsr $5dd22.l
pub fn i_08781c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87822);
    cpu.pc = 0x5DD22;
    16
}
/// $087822: lea.l $87828(pc), a1
pub fn i_087822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87826;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87828;
    16
}
/// $087826: move.l a1, (a6)
pub fn i_087826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87828;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087828: cmpi.w #$100, $22(a6)
pub fn i_087828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8782E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $08782E: blt.w $87846
pub fn i_08782e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87832;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x87846; return 10; }
    8
}
/// $087832: clr.b $10e39a.l
pub fn i_087832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87838;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $087838: jsr $2783a.l
pub fn i_087838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8783E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8783E);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08783e  ($08783E, 8 B, thunks/jsr_abs_thunks.c)
/// $08783E: jsr $28d70.l
pub fn i_08783e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87844;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87844);
    cpu.pc = 0x28D70;
    16
}
/// $087844: rts
pub fn i_087844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87846;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Engage_087846  ($087846, 146 B, enemies/bosses/fort_scene4_0865be.s)
/// $087846: bclr.b #$3, $13(a6)
pub fn i_087846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8784C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08784C: bclr.b #$0, $13(a6)
pub fn i_08784c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87852;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087852: lea.l $2c0628.l, a0
pub fn i_087852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87858;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0628;
    20
}
/// $087858: jsr $799de.l
pub fn i_087858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8785E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8785E);
    cpu.pc = 0x799DE;
    16
}
/// $08785E: move.w d0, $66(a6)
pub fn i_08785e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87862;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087862: addi.w #$2e4, $66(a6)
pub fn i_087862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87868;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2E4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087868: move.l #$2ede46, $60(a6)
pub fn i_087868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87870;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2EDE46;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087870: lea.l $87876(pc), a1
pub fn i_087870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87876;
    16
}
/// $087874: move.l a1, (a6)
pub fn i_087874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87876;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087876: clr.b $10e39a.l
pub fn i_087876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8787C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08787C: jsr $2783a.l
pub fn i_08787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87882);
    cpu.pc = 0x2783A;
    16
}
/// $087882: jsr $28998.l
pub fn i_087882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87888);
    cpu.pc = 0x28998;
    16
}
/// $087888: jsr $28d70.l
pub fn i_087888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8788E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8788E);
    cpu.pc = 0x28D70;
    16
}
/// $08788E: jsr $2870a.l
pub fn i_08788e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87894);
    cpu.pc = 0x2870A;
    16
}
/// $087894: bcc.w $878aa
pub fn i_087894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87898;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x878AA; return 10; }
    8
}
/// $087898: lea.l $5e766.l, a0
pub fn i_087898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8789E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08789E: jsr $5e770.l
pub fn i_08789e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878A4);
    cpu.pc = 0x5E770;
    16
}
/// $0878A4: bclr.b #$3, $13(a6)
pub fn i_0878a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878AA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0878AA: cmpi.w #$29a, $66(a6)
pub fn i_0878aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x29A, Size::Word, false);
    16
}
/// $0878B0: bgt.w $878c6
pub fn i_0878b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x878C6; return 10; }
    8
}
/// $0878B4: lea.l $2ed0c0.l, a1
pub fn i_0878b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0C0;
    20
}
/// $0878BA: jsr $77c7e.l
pub fn i_0878ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878C0);
    cpu.pc = 0x77C7E;
    16
}
/// $0878C0: lea.l $878d8(pc), a1
pub fn i_0878c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x878D8;
    16
}
/// $0878C4: move.l a1, (a6)
pub fn i_0878c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0878C6: jsr $4fa70.l
pub fn i_0878c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878CC);
    cpu.pc = 0x4FA70;
    16
}
/// $0878CC: bcc.w $878d6
pub fn i_0878cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x878D6; return 10; }
    8
}
/// $0878D0: jmp $518.l
pub fn i_0878d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878D6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0878D6: rts
pub fn i_0878d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Damaged1_0878d8  ($0878D8, 126 B, enemies/bosses/fort_scene4_0865be.s)
/// $0878D8: move.w #$102b, d0
pub fn i_0878d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878DC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0878DC: jsr $2352.l
pub fn i_0878dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878E2);
    cpu.pc = 0x2352;
    16
}
/// $0878E2: lea.l $2ec19a.l, a0
pub fn i_0878e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC19A;
    20
}
/// $0878E8: jsr $28cd4.l
pub fn i_0878e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878EE);
    cpu.pc = 0x28CD4;
    16
}
/// $0878EE: lea.l $878f4(pc), a1
pub fn i_0878ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x878F4;
    16
}
/// $0878F2: move.l a1, (a6)
pub fn i_0878f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0878F4: clr.b $10e39a.l
pub fn i_0878f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878FA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0878FA: jsr $2783a.l
pub fn i_0878fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87900);
    cpu.pc = 0x2783A;
    16
}
/// $087900: jsr $28998.l
pub fn i_087900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87906);
    cpu.pc = 0x28998;
    16
}
/// $087906: jsr $28d70.l
pub fn i_087906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8790C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8790C);
    cpu.pc = 0x28D70;
    16
}
/// $08790C: jsr $2870a.l
pub fn i_08790c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87912);
    cpu.pc = 0x2870A;
    16
}
/// $087912: bcc.w $87928
pub fn i_087912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87916;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87928; return 10; }
    8
}
/// $087916: lea.l $5e766.l, a0
pub fn i_087916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8791C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08791C: jsr $5e770.l
pub fn i_08791c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87922);
    cpu.pc = 0x5E770;
    16
}
/// $087922: bclr.b #$3, $13(a6)
pub fn i_087922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87928;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087928: cmpi.w #$f6, $66(a6)
pub fn i_087928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8792E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF6, Size::Word, false);
    16
}
/// $08792E: bgt.w $87944
pub fn i_08792e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87932;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x87944; return 10; }
    8
}
/// $087932: lea.l $2ed0d2.l, a1
pub fn i_087932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87938;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0D2;
    20
}
/// $087938: jsr $77c7e.l
pub fn i_087938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8793E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8793E);
    cpu.pc = 0x77C7E;
    16
}
/// $08793E: lea.l $87956(pc), a1
pub fn i_08793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87956;
    16
}
/// $087942: move.l a1, (a6)
pub fn i_087942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087944: jsr $4fa70.l
pub fn i_087944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8794A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8794A);
    cpu.pc = 0x4FA70;
    16
}
/// $08794A: bcc.w $87954
pub fn i_08794a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8794E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87954; return 10; }
    8
}
/// $08794E: jmp $518.l
pub fn i_08794e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87954;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087954: rts
pub fn i_087954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87956;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Damaged2_087956  ($087956, 194 B, enemies/bosses/fort_scene4_0865be.s)
/// $087956: move.w #$102b, d0
pub fn i_087956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8795A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08795A: jsr $2352.l
pub fn i_08795a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87960);
    cpu.pc = 0x2352;
    16
}
/// $087960: lea.l $2ec1ae.l, a0
pub fn i_087960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87966;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC1AE;
    20
}
/// $087966: jsr $28cd4.l
pub fn i_087966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8796C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8796C);
    cpu.pc = 0x28CD4;
    16
}
/// $08796C: lea.l $87972(pc), a1
pub fn i_08796c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87972;
    16
}
/// $087970: move.l a1, (a6)
pub fn i_087970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087972: clr.b $10e39a.l
pub fn i_087972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87978;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $087978: jsr $2783a.l
pub fn i_087978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8797E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8797E);
    cpu.pc = 0x2783A;
    16
}
/// $08797E: jsr $28998.l
pub fn i_08797e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87984);
    cpu.pc = 0x28998;
    16
}
/// $087984: jsr $28d70.l
pub fn i_087984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8798A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8798A);
    cpu.pc = 0x28D70;
    16
}
/// $08798A: jsr $2870a.l
pub fn i_08798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87990);
    cpu.pc = 0x2870A;
    16
}
/// $087990: bcc.w $879a6
pub fn i_087990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87994;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x879A6; return 10; }
    8
}
/// $087994: lea.l $5e766.l, a0
pub fn i_087994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8799A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08799A: jsr $5e770.l
pub fn i_08799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879A0);
    cpu.pc = 0x5E770;
    16
}
/// $0879A0: bclr.b #$3, $13(a6)
pub fn i_0879a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879A6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0879A6: jsr $28758.l
pub fn i_0879a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879AC);
    cpu.pc = 0x28758;
    16
}
/// $0879AC: bcc.w $87a06
pub fn i_0879ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87A06; return 10; }
    8
}
/// $0879B0: move.w #$1038, d0
pub fn i_0879b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879B4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1038;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0879B4: jsr $2352.l
pub fn i_0879b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879BA);
    cpu.pc = 0x2352;
    16
}
/// $0879BA: lea.l $2ed0e4.l, a1
pub fn i_0879ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879C0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0E4;
    20
}
/// $0879C0: jsr $77c7e.l
pub fn i_0879c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879C6);
    cpu.pc = 0x77C7E;
    16
}
/// $0879C6: lea.l $2ecfcc.l, a1
pub fn i_0879c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFCC;
    20
}
/// $0879CC: jsr $77c7e.l
pub fn i_0879cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879D2);
    cpu.pc = 0x77C7E;
    16
}
/// $0879D2: lea.l $2edd56.l, a1
pub fn i_0879d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD56;
    20
}
/// $0879D8: jsr $43fac.l
pub fn i_0879d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879DE);
    cpu.pc = 0x43FAC;
    16
}
/// $0879DE: lea.l $2edd68.l, a1
pub fn i_0879de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD68;
    20
}
/// $0879E4: jsr $43fac.l
pub fn i_0879e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879EA);
    cpu.pc = 0x43FAC;
    16
}
/// $0879EA: jsr $434dc.l
pub fn i_0879ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879F0);
    cpu.pc = 0x434DC;
    16
}
/// $0879F0: lea.l $87a62(pc), a1
pub fn i_0879f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87A62;
    16
}
/// $0879F4: jsr $4ae.l
pub fn i_0879f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879FA);
    cpu.pc = 0x4AE;
    16
}
/// $0879FA: jsr $5dd02.l
pub fn i_0879fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A00);
    cpu.pc = 0x5DD02;
    16
}
/// $087A00: lea.l $87a18(pc), a1
pub fn i_087a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87A18;
    16
}
/// $087A04: move.l a1, (a6)
pub fn i_087a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087A06: jsr $4fa70.l
pub fn i_087a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A0C);
    cpu.pc = 0x4FA70;
    16
}
/// $087A0C: bcc.w $87a16
pub fn i_087a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87A16; return 10; }
    8
}
/// $087A10: jmp $518.l
pub fn i_087a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A16;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087A16: rts
pub fn i_087a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Wreck_087a18  ($087A18, 74 B, enemies/bosses/fort_scene4_0865be.s)
/// $087A18: move.l #$5000, d0
pub fn i_087a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A1E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $087A1E: jsr $51a28.l
pub fn i_087a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A24);
    cpu.pc = 0x51A28;
    16
}
/// $087A24: lea.l $ffff.w, a0
pub fn i_087a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A28;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $087A28: move.l a0, $48(a6)
pub fn i_087a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A2C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087A2C: lea.l $2ec1c2.l, a0
pub fn i_087a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC1C2;
    20
}
/// $087A32: jsr $28cd4.l
pub fn i_087a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A38);
    cpu.pc = 0x28CD4;
    16
}
/// $087A38: lea.l $87a3e(pc), a1
pub fn i_087a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87A3E;
    16
}
/// $087A3C: move.l a1, (a6)
pub fn i_087a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087A3E: jsr $2783a.l
pub fn i_087a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A44);
    cpu.pc = 0x2783A;
    16
}
/// $087A44: jsr $28d70.l
pub fn i_087a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A4A);
    cpu.pc = 0x28D70;
    16
}
/// $087A4A: jsr $283d8.l
pub fn i_087a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A50);
    cpu.pc = 0x283D8;
    16
}
/// $087A50: jsr $4fa70.l
pub fn i_087a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A56);
    cpu.pc = 0x4FA70;
    16
}
/// $087A56: bcc.w $87a60
pub fn i_087a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A5A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87A60; return 10; }
    8
}
/// $087A5A: jmp $518.l
pub fn i_087a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A60;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087A60: rts
pub fn i_087a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_BlastBox_087a62  ($087A62, 62 B, enemies/bosses/fort_scene4_0865be.s)
/// $087A62: move.w #$10, $72(a6)
pub fn i_087a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087A68: lea.l $87a6e(pc), a1
pub fn i_087a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87A6E;
    16
}
/// $087A6C: move.l a1, (a6)
pub fn i_087a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087A6E: jsr $2783a.l
pub fn i_087a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A74);
    cpu.pc = 0x2783A;
    16
}
/// $087A74: lea.l $2ecc34.l, a0
pub fn i_087a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2ECC34;
    20
}
/// $087A7A: move.l a0, $4c(a6)
pub fn i_087a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A7E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087A7E: jsr $283ca.l
pub fn i_087a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A84);
    cpu.pc = 0x283CA;
    16
}
/// $087A84: jsr $283ca.l
pub fn i_087a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A8A);
    cpu.pc = 0x283CA;
    16
}
/// $087A8A: jsr $283d8.l
pub fn i_087a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A90);
    cpu.pc = 0x283D8;
    16
}
/// $087A90: subq.w #$1, $72(a6)
pub fn i_087a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A94;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $087A94: bpl.w $87a9e
pub fn i_087a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A98;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x87A9E; return 10; }
    8
}
/// $087A98: jmp $518.l
pub fn i_087a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A9E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087A9E: rts
pub fn i_087a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Turret_087aa0  ($087AA0, 124 B, enemies/bosses/fort_scene4_0865be.s)
/// $087AA0: move.w #$e5, d1
pub fn i_087aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AA4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087AA4: jsr $236e.l
pub fn i_087aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87AAA);
    cpu.pc = 0x236E;
    16
}
/// $087AAA: move.b #$ff, $32(a6)
pub fn i_087aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087AB0: move.b #$ff, $33(a6)
pub fn i_087ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087AB6: move.b #$0, $3a(a6)
pub fn i_087ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ABC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087ABC: move.w #$8000, d0
pub fn i_087abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AC0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087AC0: move.w #$80, d1
pub fn i_087ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AC4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087AC4: jsr $2813c.l
pub fn i_087ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ACA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87ACA);
    cpu.pc = 0x2813C;
    16
}
/// $087ACA: lea.l $87ad0(pc), a1
pub fn i_087aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ACE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87AD0;
    16
}
/// $087ACE: move.l a1, (a6)
pub fn i_087ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087AD0: movea.l $c(a6), a0
pub fn i_087ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $087AD4: move.w $22(a0), $22(a6)
pub fn i_087ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ADA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $087ADA: move.w $24(a0), $24(a6)
pub fn i_087ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AE0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $087AE0: clr.l d0
pub fn i_087ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AE2;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $087AE2: move.b $20(a0), d0
pub fn i_087ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AE6;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $087AE6: cmpi.b #$3, d0
pub fn i_087ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AEA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $087AEA: beq.w $87b14
pub fn i_087aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x87B14; return 10; }
    8
}
/// $087AEE: movea.l #$2ec21a, a0
pub fn i_087aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AF4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EC21A;
    cpu.a[0] = v1;
    4
}
/// $087AF4: lsl.w #$2, d0
pub fn i_087af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AF6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $087AF6: movea.l (a0, d0.w), a0
pub fn i_087af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AFA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $087AFA: cmpa.l #$ffffffff, a0
pub fn i_087afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B00;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $087B00: beq.w $87b0a
pub fn i_087b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x87B0A; return 10; }
    8
}
/// $087B04: jsr $28cd4.l
pub fn i_087b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B0A);
    cpu.pc = 0x28CD4;
    16
}
/// $087B0A: jsr $28d70.l
pub fn i_087b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B10);
    cpu.pc = 0x28D70;
    16
}
/// $087B10: bra.w $87b1a
pub fn i_087b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B14;
    cpu.ir = 0x6000;
    cpu.pc = 0x87B1A;
    10
}
/// $087B14: jmp $518.l
pub fn i_087b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087B1A: rts
pub fn i_087b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Init_087b1c  ($087B1C, 10 B, enemies/bosses/fort_scene4_0865be.s)
/// $087B1C: movea.l $3c(a6), a1
pub fn i_087b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B20;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087B20: jsr $2942a.l
pub fn i_087b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B26);
    cpu.pc = 0x2942A;
    16
}
// ===== Heli_InitTmpl_087b26  ($087B26, 252 B, enemies/bosses/fort_scene4_0865be.s)
/// $087B26: jsr $2783a.l
pub fn i_087b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B2C);
    cpu.pc = 0x2783A;
    16
}
/// $087B2C: move.w #$86, d1
pub fn i_087b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x86;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087B30: jsr $236e.l
pub fn i_087b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B36);
    cpu.pc = 0x236E;
    16
}
/// $087B36: move.w #$87, d1
pub fn i_087b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B3A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087B3A: jsr $236e.l
pub fn i_087b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B40);
    cpu.pc = 0x236E;
    16
}
/// $087B40: move.b #$ff, $32(a6)
pub fn i_087b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087B46: move.b #$ff, $33(a6)
pub fn i_087b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087B4C: move.b #$0, $3a(a6)
pub fn i_087b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087B52: move.w #$50, $70(a6)
pub fn i_087b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B58;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087B58: lea.l $2c05a6.l, a0
pub fn i_087b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C05A6;
    20
}
/// $087B5E: jsr $799de.l
pub fn i_087b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B64);
    cpu.pc = 0x799DE;
    16
}
/// $087B64: move.w d0, $66(a6)
pub fn i_087b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B68;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087B68: addi.w #$17c, $66(a6)
pub fn i_087b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B6E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x17C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087B6E: move.w #$0, $72(a6)
pub fn i_087b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B74;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087B74: move.b #$0, $20(a6)
pub fn i_087b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087B7A: move.w #$2000, $38(a6)
pub fn i_087b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087B80: lea.l $2ec226.l, a0
pub fn i_087b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC226;
    20
}
/// $087B86: jsr $28cd4.l
pub fn i_087b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B8C);
    cpu.pc = 0x28CD4;
    16
}
/// $087B8C: lea.l $87eae(pc), a1
pub fn i_087b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87EAE;
    16
}
/// $087B90: jsr $4ae.l
pub fn i_087b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B96);
    cpu.pc = 0x4AE;
    16
}
/// $087B96: jsr $5dd22.l
pub fn i_087b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B9C);
    cpu.pc = 0x5DD22;
    16
}
/// $087B9C: lea.l $2edd7e.l, a1
pub fn i_087b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BA2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD7E;
    20
}
/// $087BA2: jsr $43fac.l
pub fn i_087ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BA8);
    cpu.pc = 0x43FAC;
    16
}
/// $087BA8: move.l #$2eddfa, $60(a6)
pub fn i_087ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BB0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2EDDFA;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087BB0: lea.l $87bb6(pc), a1
pub fn i_087bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87BB6;
    16
}
/// $087BB4: move.l a1, (a6)
pub fn i_087bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087BB6: jsr $28998.l
pub fn i_087bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BBC);
    cpu.pc = 0x28998;
    16
}
/// $087BBC: jsr $2783a.l
pub fn i_087bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BC2);
    cpu.pc = 0x2783A;
    16
}
/// $087BC2: jsr $883ec(pc)
pub fn i_087bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87BC6);
    cpu.pc = 0x883EC;
    16
}
/// $087BC6: jsr $28d70.l
pub fn i_087bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BCC);
    cpu.pc = 0x28D70;
    16
}
/// $087BCC: jsr $2870a.l
pub fn i_087bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BD2);
    cpu.pc = 0x2870A;
    16
}
/// $087BD2: bcc.w $87bee
pub fn i_087bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87BEE; return 10; }
    8
}
/// $087BD6: lea.l $5e766.l, a0
pub fn i_087bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087BDC: jsr $5e770.l
pub fn i_087bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BE2);
    cpu.pc = 0x5E770;
    16
}
/// $087BE2: bclr.b #$3, $13(a6)
pub fn i_087be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BE8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087BE8: move.w #$f, $72(a6)
pub fn i_087be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087BEE: bclr.b #$0, $13(a6)
pub fn i_087bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BF4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087BF4: move.w #$17c, $66(a6)
pub fn i_087bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087BFA: cmpi.w #$100, $22(a6)
pub fn i_087bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C00;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $087C00: bgt.w $87c0a
pub fn i_087c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C04;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x87C0A; return 10; }
    8
}
/// $087C04: lea.l $87c22(pc), a1
pub fn i_087c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87C22;
    16
}
/// $087C08: move.l a1, (a6)
pub fn i_087c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087C0A: jsr $4fa70.l
pub fn i_087c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C10);
    cpu.pc = 0x4FA70;
    16
}
/// $087C10: bcc.w $87c20
pub fn i_087c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87C20; return 10; }
    8
}
/// $087C14: move.b #$ff, $20(a6)
pub fn i_087c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C1A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087C1A: jmp $518.l
pub fn i_087c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C20;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087C20: rts
pub fn i_087c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Approach_087c22  ($087C22, 120 B, enemies/bosses/fort_scene4_0865be.s)
/// $087C22: lea.l $2ec23c.l, a0
pub fn i_087c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC23C;
    20
}
/// $087C28: jsr $28cd4.l
pub fn i_087c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C2E);
    cpu.pc = 0x28CD4;
    16
}
/// $087C2E: lea.l $87c34(pc), a1
pub fn i_087c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87C34;
    16
}
/// $087C32: move.l a1, (a6)
pub fn i_087c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C34;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087C34: jsr $28998.l
pub fn i_087c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C3A);
    cpu.pc = 0x28998;
    16
}
/// $087C3A: jsr $2783a.l
pub fn i_087c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C40);
    cpu.pc = 0x2783A;
    16
}
/// $087C40: jsr $883ec(pc)
pub fn i_087c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87C44);
    cpu.pc = 0x883EC;
    16
}
/// $087C44: jsr $28d70.l
pub fn i_087c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C4A);
    cpu.pc = 0x28D70;
    16
}
/// $087C4A: bcc.w $87c54
pub fn i_087c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87C54; return 10; }
    8
}
/// $087C4E: lea.l $87c9a(pc), a1
pub fn i_087c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87C9A;
    16
}
/// $087C52: move.l a1, (a6)
pub fn i_087c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087C54: jsr $2870a.l
pub fn i_087c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C5A);
    cpu.pc = 0x2870A;
    16
}
/// $087C5A: bcc.w $87c76
pub fn i_087c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87C76; return 10; }
    8
}
/// $087C5E: lea.l $5e766.l, a0
pub fn i_087c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087C64: jsr $5e770.l
pub fn i_087c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C6A);
    cpu.pc = 0x5E770;
    16
}
/// $087C6A: bclr.b #$3, $13(a6)
pub fn i_087c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087C70: move.w #$f, $72(a6)
pub fn i_087c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C76;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087C76: bclr.b #$0, $13(a6)
pub fn i_087c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087C7C: move.w #$17c, $66(a6)
pub fn i_087c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087C82: jsr $4fa70.l
pub fn i_087c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C88);
    cpu.pc = 0x4FA70;
    16
}
/// $087C88: bcc.w $87c98
pub fn i_087c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87C98; return 10; }
    8
}
/// $087C8C: move.b #$ff, $20(a6)
pub fn i_087c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C92;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087C92: jmp $518.l
pub fn i_087c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C98;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087C98: rts
pub fn i_087c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Hover_087c9a  ($087C9A, 130 B, enemies/bosses/fort_scene4_0865be.s)
/// $087C9A: lea.l $ee042.l, a1
pub fn i_087c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CA0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE042;
    20
}
/// $087CA0: move.w #$82, d0
pub fn i_087ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087CA4: move.b #$0, $82(a6)
pub fn i_087ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087CAA: jsr $4429e.l
pub fn i_087caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CB0);
    cpu.pc = 0x4429E;
    16
}
/// $087CB0: lea.l $87cb6(pc), a1
pub fn i_087cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87CB6;
    16
}
/// $087CB4: move.l a1, (a6)
pub fn i_087cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087CB6: jsr $28998.l
pub fn i_087cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CBC);
    cpu.pc = 0x28998;
    16
}
/// $087CBC: jsr $2783a.l
pub fn i_087cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CC2);
    cpu.pc = 0x2783A;
    16
}
/// $087CC2: jsr $883ec(pc)
pub fn i_087cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87CC6);
    cpu.pc = 0x883EC;
    16
}
/// $087CC6: jsr $28d70.l
pub fn i_087cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CCC);
    cpu.pc = 0x28D70;
    16
}
/// $087CCC: jsr $2870a.l
pub fn i_087ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CD2);
    cpu.pc = 0x2870A;
    16
}
/// $087CD2: bcc.w $87cee
pub fn i_087cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87CEE; return 10; }
    8
}
/// $087CD6: lea.l $5e766.l, a0
pub fn i_087cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087CDC: jsr $5e770.l
pub fn i_087cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CE2);
    cpu.pc = 0x5E770;
    16
}
/// $087CE2: bclr.b #$3, $13(a6)
pub fn i_087ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CE8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087CE8: move.w #$f, $72(a6)
pub fn i_087ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087CEE: cmpi.w #$fc, $66(a6)
pub fn i_087cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CF4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFC, Size::Word, false);
    16
}
/// $087CF4: bgt.w $87cfe
pub fn i_087cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CF8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x87CFE; return 10; }
    8
}
/// $087CF8: lea.l $87d1c(pc), a1
pub fn i_087cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87D1C;
    16
}
/// $087CFC: move.l a1, (a6)
pub fn i_087cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087CFE: jsr $4fa70.l
pub fn i_087cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D04);
    cpu.pc = 0x4FA70;
    16
}
/// $087D04: bcc.w $87d1a
pub fn i_087d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87D1A; return 10; }
    8
}
/// $087D08: move.b #$ff, $82(a6)
pub fn i_087d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D0E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087D0E: move.b #$ff, $20(a6)
pub fn i_087d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D14;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087D14: jmp $518.l
pub fn i_087d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087D1A: rts
pub fn i_087d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Damaged1_087d1c  ($087D1C, 130 B, enemies/bosses/fort_scene4_0865be.s)
/// $087D1C: move.w #$1027, d0
pub fn i_087d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087D20: jsr $2352.l
pub fn i_087d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D26);
    cpu.pc = 0x2352;
    16
}
/// $087D26: lea.l $2ec2ce.l, a0
pub fn i_087d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC2CE;
    20
}
/// $087D2C: jsr $28cd4.l
pub fn i_087d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D32);
    cpu.pc = 0x28CD4;
    16
}
/// $087D32: lea.l $87d38(pc), a1
pub fn i_087d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87D38;
    16
}
/// $087D36: move.l a1, (a6)
pub fn i_087d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087D38: jsr $28998.l
pub fn i_087d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D3E);
    cpu.pc = 0x28998;
    16
}
/// $087D3E: jsr $2783a.l
pub fn i_087d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D44);
    cpu.pc = 0x2783A;
    16
}
/// $087D44: jsr $883ec(pc)
pub fn i_087d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87D48);
    cpu.pc = 0x883EC;
    16
}
/// $087D48: jsr $28d70.l
pub fn i_087d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D4E);
    cpu.pc = 0x28D70;
    16
}
/// $087D4E: jsr $2870a.l
pub fn i_087d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D54);
    cpu.pc = 0x2870A;
    16
}
/// $087D54: bcc.w $87d70
pub fn i_087d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87D70; return 10; }
    8
}
/// $087D58: lea.l $5e766.l, a0
pub fn i_087d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087D5E: jsr $5e770.l
pub fn i_087d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D64);
    cpu.pc = 0x5E770;
    16
}
/// $087D64: bclr.b #$3, $13(a6)
pub fn i_087d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D6A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087D6A: move.w #$f, $72(a6)
pub fn i_087d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087D70: cmpi.w #$7e, $66(a6)
pub fn i_087d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D76;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7E, Size::Word, false);
    16
}
/// $087D76: bgt.w $87d80
pub fn i_087d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D7A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x87D80; return 10; }
    8
}
/// $087D7A: lea.l $87d9e(pc), a1
pub fn i_087d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87D9E;
    16
}
/// $087D7E: move.l a1, (a6)
pub fn i_087d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087D80: jsr $4fa70.l
pub fn i_087d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D86);
    cpu.pc = 0x4FA70;
    16
}
/// $087D86: bcc.w $87d9c
pub fn i_087d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87D9C; return 10; }
    8
}
/// $087D8A: move.b #$ff, $82(a6)
pub fn i_087d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087D90: move.b #$ff, $20(a6)
pub fn i_087d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087D96: jmp $518.l
pub fn i_087d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D9C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087D9C: rts
pub fn i_087d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Damaged2_087d9e  ($087D9E, 130 B, enemies/bosses/fort_scene4_0865be.s)
/// $087D9E: move.w #$1027, d0
pub fn i_087d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DA2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087DA2: jsr $2352.l
pub fn i_087da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DA8);
    cpu.pc = 0x2352;
    16
}
/// $087DA8: lea.l $2ec2de.l, a0
pub fn i_087da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC2DE;
    20
}
/// $087DAE: jsr $28cd4.l
pub fn i_087dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DB4);
    cpu.pc = 0x28CD4;
    16
}
/// $087DB4: lea.l $87dba(pc), a1
pub fn i_087db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87DBA;
    16
}
/// $087DB8: move.l a1, (a6)
pub fn i_087db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087DBA: jsr $28998.l
pub fn i_087dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DC0);
    cpu.pc = 0x28998;
    16
}
/// $087DC0: jsr $2783a.l
pub fn i_087dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DC6);
    cpu.pc = 0x2783A;
    16
}
/// $087DC6: jsr $883ec(pc)
pub fn i_087dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87DCA);
    cpu.pc = 0x883EC;
    16
}
/// $087DCA: jsr $28d70.l
pub fn i_087dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DD0);
    cpu.pc = 0x28D70;
    16
}
/// $087DD0: jsr $2870a.l
pub fn i_087dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DD6);
    cpu.pc = 0x2870A;
    16
}
/// $087DD6: bcc.w $87df2
pub fn i_087dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87DF2; return 10; }
    8
}
/// $087DDA: lea.l $5e766.l, a0
pub fn i_087dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087DE0: jsr $5e770.l
pub fn i_087de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DE6);
    cpu.pc = 0x5E770;
    16
}
/// $087DE6: bclr.b #$3, $13(a6)
pub fn i_087de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DEC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087DEC: move.w #$f, $72(a6)
pub fn i_087dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DF2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087DF2: jsr $28758.l
pub fn i_087df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DF8);
    cpu.pc = 0x28758;
    16
}
/// $087DF8: bcc.w $87e02
pub fn i_087df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87E02; return 10; }
    8
}
/// $087DFC: lea.l $87e20(pc), a1
pub fn i_087dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87E20;
    16
}
/// $087E00: move.l a1, (a6)
pub fn i_087e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087E02: jsr $4fa70.l
pub fn i_087e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E08);
    cpu.pc = 0x4FA70;
    16
}
/// $087E08: bcc.w $87e1e
pub fn i_087e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87E1E; return 10; }
    8
}
/// $087E0C: move.b #$ff, $82(a6)
pub fn i_087e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087E12: move.b #$ff, $20(a6)
pub fn i_087e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087E18: jmp $518.l
pub fn i_087e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E1E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087E1E: rts
pub fn i_087e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Crash_087e20  ($087E20, 142 B, enemies/bosses/fort_scene4_0865be.s)
/// $087E20: move.l #$5000, d0
pub fn i_087e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E26;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $087E26: jsr $51a28.l
pub fn i_087e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E2C);
    cpu.pc = 0x51A28;
    16
}
/// $087E2C: move.w #$1023, d0
pub fn i_087e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E30;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087E30: jsr $2352.l
pub fn i_087e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E36);
    cpu.pc = 0x2352;
    16
}
/// $087E36: jsr $2783a.l
pub fn i_087e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E3C);
    cpu.pc = 0x2783A;
    16
}
/// $087E3C: move.b #$ff, $82(a6)
pub fn i_087e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087E42: move.b #$ff, $20(a6)
pub fn i_087e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E48;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087E48: lea.l $77fd6.l, a1
pub fn i_087e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E4E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $087E4E: jsr $4ae.l
pub fn i_087e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E54);
    cpu.pc = 0x4AE;
    16
}
/// $087E54: jsr $5dd22.l
pub fn i_087e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E5A);
    cpu.pc = 0x5DD22;
    16
}
/// $087E5A: lea.l $2edd92.l, a1
pub fn i_087e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E60;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD92;
    20
}
/// $087E60: jsr $43fac.l
pub fn i_087e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E66);
    cpu.pc = 0x43FAC;
    16
}
/// $087E66: lea.l $2ed176.l, a1
pub fn i_087e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED176;
    20
}
/// $087E6C: jsr $77c7e.l
pub fn i_087e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E72);
    cpu.pc = 0x77C7E;
    16
}
/// $087E72: lea.l $2ed188.l, a1
pub fn i_087e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED188;
    20
}
/// $087E78: jsr $77c7e.l
pub fn i_087e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E7E);
    cpu.pc = 0x77C7E;
    16
}
/// $087E7E: lea.l $2ec2ee.l, a0
pub fn i_087e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC2EE;
    20
}
/// $087E84: jsr $28cd4.l
pub fn i_087e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E8A);
    cpu.pc = 0x28CD4;
    16
}
/// $087E8A: lea.l $87e90(pc), a1
pub fn i_087e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87E90;
    16
}
/// $087E8E: move.l a1, (a6)
pub fn i_087e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087E90: jsr $2783a.l
pub fn i_087e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E96);
    cpu.pc = 0x2783A;
    16
}
/// $087E96: jsr $28d70.l
pub fn i_087e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E9C);
    cpu.pc = 0x28D70;
    16
}
/// $087E9C: jsr $4fa70.l
pub fn i_087e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EA2);
    cpu.pc = 0x4FA70;
    16
}
/// $087EA2: bcc.w $87eac
pub fn i_087ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87EAC; return 10; }
    8
}
/// $087EA6: jmp $518.l
pub fn i_087ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EAC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087EAC: rts
pub fn i_087eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Dropper_087eae  ($087EAE, 158 B, enemies/bosses/fort_scene4_0865be.s)
/// $087EAE: move.w #$88, d1
pub fn i_087eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087EB2: jsr $236e.l
pub fn i_087eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EB8);
    cpu.pc = 0x236E;
    16
}
/// $087EB8: move.b #$ff, $32(a6)
pub fn i_087eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087EBE: move.b #$ff, $33(a6)
pub fn i_087ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087EC4: move.b #$0, $3a(a6)
pub fn i_087ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ECA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087ECA: move.w #$0, $38(a6)
pub fn i_087eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ED0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087ED0: lea.l $2ec2fe.l, a0
pub fn i_087ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ED6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC2FE;
    20
}
/// $087ED6: jsr $28cd4.l
pub fn i_087ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EDC);
    cpu.pc = 0x28CD4;
    16
}
/// $087EDC: move.b #$0, $83(a6)
pub fn i_087edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EE2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087EE2: lea.l $77228.l, a1
pub fn i_087ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77228;
    20
}
/// $087EE8: jsr $4ae.l
pub fn i_087ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EEE);
    cpu.pc = 0x4AE;
    16
}
/// $087EEE: jsr $5dd22.l
pub fn i_087eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EF4);
    cpu.pc = 0x5DD22;
    16
}
/// $087EF4: addi.w #$58, $22(a0)
pub fn i_087ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EFA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x58, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087EFA: move.b #$30, $98(a0)
pub fn i_087efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F00;
    cpu.ir = 0x117C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087F00: move.b #$83, $99(a0)
pub fn i_087f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F06;
    cpu.ir = 0x117C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087F06: lea.l $87f0c(pc), a1
pub fn i_087f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87F0C;
    16
}
/// $087F0A: move.l a1, (a6)
pub fn i_087f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087F0C: jsr $2783a.l
pub fn i_087f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F12);
    cpu.pc = 0x2783A;
    16
}
/// $087F12: jsr $28d70.l
pub fn i_087f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F18);
    cpu.pc = 0x28D70;
    16
}
/// $087F18: movea.l $c(a6), a0
pub fn i_087f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F1C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $087F1C: cmpi.b #$ff, $20(a0)
pub fn i_087f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F22;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $087F22: bne.w $87f4a
pub fn i_087f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x87F4A; return 10; }
    8
}
/// $087F26: move.b #$ff, $83(a6)
pub fn i_087f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F2C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087F2C: lea.l $77f6a.l, a1
pub fn i_087f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F32;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $087F32: jsr $4ae.l
pub fn i_087f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F38);
    cpu.pc = 0x4AE;
    16
}
/// $087F38: jsr $5dd22.l
pub fn i_087f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F3E);
    cpu.pc = 0x5DD22;
    16
}
/// $087F3E: addi.w #$58, $22(a0)
pub fn i_087f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F44;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x58, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087F44: jmp $518.l
pub fn i_087f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F4A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087F4A: rts
pub fn i_087f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debris_Scatter_087f4c  ($087F4C, 112 B, enemies/bosses/fort_scene4_0865be.s)
/// $087F4C: move.w #$f3, d1
pub fn i_087f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F50;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087F50: jsr $236e.l
pub fn i_087f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F56);
    cpu.pc = 0x236E;
    16
}
/// $087F56: jsr $267e2.l
pub fn i_087f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F5C);
    cpu.pc = 0x267E2;
    16
}
/// $087F5C: move.w #$7f, d0
pub fn i_087f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F60;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087F60: jsr $5ea1c.l
pub fn i_087f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F66);
    cpu.pc = 0x5EA1C;
    16
}
/// $087F66: btst.b #$0, d0
pub fn i_087f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F6A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $087F6A: beq.w $87f70
pub fn i_087f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F6E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x87F70; return 10; }
    8
}
/// $087F6E: neg.w d0
pub fn i_087f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F70;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $087F70: move.w d0, $28(a6)
pub fn i_087f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F74;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087F74: andi.w #$7f, d0
pub fn i_087f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F78;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $087F78: sub.w d0, $2a(a6)
pub fn i_087f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F7C;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $087F7C: lea.l $2de4b0.l, a0
pub fn i_087f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE4B0;
    20
}
/// $087F82: jsr $28cd4.l
pub fn i_087f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F88);
    cpu.pc = 0x28CD4;
    16
}
/// $087F88: lea.l $87f8e(pc), a1
pub fn i_087f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87F8E;
    16
}
/// $087F8C: move.l a1, (a6)
pub fn i_087f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087F8E: jsr $27cee.l
pub fn i_087f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F94);
    cpu.pc = 0x27CEE;
    16
}
/// $087F94: jsr $28d70.l
pub fn i_087f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F9A);
    cpu.pc = 0x28D70;
    16
}
/// $087F9A: bcs.w $87fb4
pub fn i_087f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F9E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x87FB4; return 10; }
    8
}
/// $087F9E: movea.l #$ffffffff, a0
pub fn i_087f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FA4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $087FA4: lea.l $298736.l, a0
pub fn i_087fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298736;
    20
}
/// $087FAA: jsr $5dd56.l
pub fn i_087faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FB0);
    cpu.pc = 0x5DD56;
    16
}
/// $087FB0: bcc.w $87fba
pub fn i_087fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87FBA; return 10; }
    8
}
/// $087FB4: jmp $518.l
pub fn i_087fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FBA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087FBA: rts
pub fn i_087fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Quad_Spawn_087fbc  ($087FBC, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $087FBC: movea.l $3c(a6), a1
pub fn i_087fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FC0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0862F8, i_0862f8),
    (0x0862FE, i_0862fe),
    (0x086300, i_086300),
    (0x086302, i_086302),
    (0x086306, i_086306),
    (0x08630A, i_08630a),
    (0x086310, i_086310),
    (0x086316, i_086316),
    (0x08631A, i_08631a),
    (0x08631E, i_08631e),
    (0x086320, i_086320),
    (0x086324, i_086324),
    (0x086326, i_086326),
    (0x086328, i_086328),
    (0x08632A, i_08632a),
    (0x08632E, i_08632e),
    (0x086332, i_086332),
    (0x086336, i_086336),
    (0x08633A, i_08633a),
    (0x086340, i_086340),
    (0x086346, i_086346),
    (0x08634A, i_08634a),
    (0x08634E, i_08634e),
    (0x086352, i_086352),
    (0x086356, i_086356),
    (0x086358, i_086358),
    (0x08635C, i_08635c),
    (0x086360, i_086360),
    (0x086362, i_086362),
    (0x086364, i_086364),
    (0x086366, i_086366),
    (0x08636A, i_08636a),
    (0x08636E, i_08636e),
    (0x086372, i_086372),
    (0x086376, i_086376),
    (0x08637C, i_08637c),
    (0x086382, i_086382),
    (0x086386, i_086386),
    (0x08638A, i_08638a),
    (0x08638E, i_08638e),
    (0x086392, i_086392),
    (0x086394, i_086394),
    (0x086398, i_086398),
    (0x08639C, i_08639c),
    (0x08639E, i_08639e),
    (0x0863A0, i_0863a0),
    (0x0863A4, i_0863a4),
    (0x0863AA, i_0863aa),
    (0x0863AE, i_0863ae),
    (0x0863B4, i_0863b4),
    (0x0863B6, i_0863b6),
    (0x0863BC, i_0863bc),
    (0x0863BE, i_0863be),
    (0x0863C4, i_0863c4),
    (0x0863C8, i_0863c8),
    (0x0863CE, i_0863ce),
    (0x0863D4, i_0863d4),
    (0x0863D6, i_0863d6),
    (0x0863DC, i_0863dc),
    (0x0863E2, i_0863e2),
    (0x0863E4, i_0863e4),
    (0x0863EA, i_0863ea),
    (0x0863F0, i_0863f0),
    (0x0863F2, i_0863f2),
    (0x0863F8, i_0863f8),
    (0x0863FE, i_0863fe),
    (0x086400, i_086400),
    (0x086406, i_086406),
    (0x08640C, i_08640c),
    (0x08640E, i_08640e),
    (0x086414, i_086414),
    (0x08641A, i_08641a),
    (0x08641C, i_08641c),
    (0x086422, i_086422),
    (0x086428, i_086428),
    (0x08642A, i_08642a),
    (0x086430, i_086430),
    (0x086436, i_086436),
    (0x086438, i_086438),
    (0x08643E, i_08643e),
    (0x086444, i_086444),
    (0x086446, i_086446),
    (0x08644C, i_08644c),
    (0x086452, i_086452),
    (0x086454, i_086454),
    (0x08645A, i_08645a),
    (0x086460, i_086460),
    (0x086462, i_086462),
    (0x086468, i_086468),
    (0x08646E, i_08646e),
    (0x086470, i_086470),
    (0x086476, i_086476),
    (0x08647C, i_08647c),
    (0x08647E, i_08647e),
    (0x086484, i_086484),
    (0x08648A, i_08648a),
    (0x08648C, i_08648c),
    (0x086492, i_086492),
    (0x086498, i_086498),
    (0x08649A, i_08649a),
    (0x0864A0, i_0864a0),
    (0x0864A6, i_0864a6),
    (0x0864A8, i_0864a8),
    (0x0864AE, i_0864ae),
    (0x0864B4, i_0864b4),
    (0x0864B6, i_0864b6),
    (0x0864BC, i_0864bc),
    (0x0864BE, i_0864be),
    (0x0864C2, i_0864c2),
    (0x0864C4, i_0864c4),
    (0x0864C8, i_0864c8),
    (0x0864CE, i_0864ce),
    (0x0864D0, i_0864d0),
    (0x0864D6, i_0864d6),
    (0x0864D8, i_0864d8),
    (0x0864DC, i_0864dc),
    (0x0864DE, i_0864de),
    (0x0864E2, i_0864e2),
    (0x0864E8, i_0864e8),
    (0x0864EA, i_0864ea),
    (0x0864F0, i_0864f0),
    (0x0864F2, i_0864f2),
    (0x0864F6, i_0864f6),
    (0x0864F8, i_0864f8),
    (0x0864FC, i_0864fc),
    (0x086502, i_086502),
    (0x086504, i_086504),
    (0x08650A, i_08650a),
    (0x08650C, i_08650c),
    (0x086510, i_086510),
    (0x086512, i_086512),
    (0x086516, i_086516),
    (0x08651C, i_08651c),
    (0x08651E, i_08651e),
    (0x086524, i_086524),
    (0x086526, i_086526),
    (0x08652A, i_08652a),
    (0x08652C, i_08652c),
    (0x086530, i_086530),
    (0x086536, i_086536),
    (0x086538, i_086538),
    (0x08653E, i_08653e),
    (0x086540, i_086540),
    (0x086544, i_086544),
    (0x086546, i_086546),
    (0x08654A, i_08654a),
    (0x086550, i_086550),
    (0x086552, i_086552),
    (0x086556, i_086556),
    (0x08655A, i_08655a),
    (0x08655E, i_08655e),
    (0x086562, i_086562),
    (0x086566, i_086566),
    (0x086568, i_086568),
    (0x08656C, i_08656c),
    (0x08656E, i_08656e),
    (0x086574, i_086574),
    (0x086578, i_086578),
    (0x08657E, i_08657e),
    (0x086586, i_086586),
    (0x08658C, i_08658c),
    (0x086590, i_086590),
    (0x086596, i_086596),
    (0x08659C, i_08659c),
    (0x0865A2, i_0865a2),
    (0x0865A8, i_0865a8),
    (0x0865AE, i_0865ae),
    (0x0865B6, i_0865b6),
    (0x0865BA, i_0865ba),
    (0x0865BC, i_0865bc),
    (0x0865BE, i_0865be),
    (0x0865C6, i_0865c6),
    (0x0865CA, i_0865ca),
    (0x0865CE, i_0865ce),
    (0x0865D4, i_0865d4),
    (0x0865DA, i_0865da),
    (0x0865DC, i_0865dc),
    (0x0865E0, i_0865e0),
    (0x0865E6, i_0865e6),
    (0x0865EC, i_0865ec),
    (0x0865F0, i_0865f0),
    (0x0865F6, i_0865f6),
    (0x0865FA, i_0865fa),
    (0x086600, i_086600),
    (0x086604, i_086604),
    (0x086608, i_086608),
    (0x08660E, i_08660e),
    (0x086614, i_086614),
    (0x08661A, i_08661a),
    (0x086620, i_086620),
    (0x086624, i_086624),
    (0x086628, i_086628),
    (0x08662E, i_08662e),
    (0x086634, i_086634),
    (0x086638, i_086638),
    (0x08663E, i_08663e),
    (0x086642, i_086642),
    (0x086648, i_086648),
    (0x08664C, i_08664c),
    (0x086650, i_086650),
    (0x086656, i_086656),
    (0x08665C, i_08665c),
    (0x086662, i_086662),
    (0x086666, i_086666),
    (0x08666A, i_08666a),
    (0x086670, i_086670),
    (0x086676, i_086676),
    (0x08667A, i_08667a),
    (0x086680, i_086680),
    (0x086684, i_086684),
    (0x08668A, i_08668a),
    (0x08668E, i_08668e),
    (0x086692, i_086692),
    (0x086698, i_086698),
    (0x08669E, i_08669e),
    (0x0866A4, i_0866a4),
    (0x0866AA, i_0866aa),
    (0x0866AE, i_0866ae),
    (0x0866B2, i_0866b2),
    (0x0866B8, i_0866b8),
    (0x0866BE, i_0866be),
    (0x0866C2, i_0866c2),
    (0x0866C8, i_0866c8),
    (0x0866CC, i_0866cc),
    (0x0866D2, i_0866d2),
    (0x0866D6, i_0866d6),
    (0x0866DA, i_0866da),
    (0x0866E0, i_0866e0),
    (0x0866E6, i_0866e6),
    (0x0866EC, i_0866ec),
    (0x0866F0, i_0866f0),
    (0x0866F4, i_0866f4),
    (0x0866FA, i_0866fa),
    (0x086700, i_086700),
    (0x086704, i_086704),
    (0x08670A, i_08670a),
    (0x08670E, i_08670e),
    (0x086714, i_086714),
    (0x086718, i_086718),
    (0x08671C, i_08671c),
    (0x086722, i_086722),
    (0x086728, i_086728),
    (0x08672E, i_08672e),
    (0x086734, i_086734),
    (0x086738, i_086738),
    (0x08673E, i_08673e),
    (0x086744, i_086744),
    (0x08674A, i_08674a),
    (0x086750, i_086750),
    (0x086754, i_086754),
    (0x086758, i_086758),
    (0x08675A, i_08675a),
    (0x086760, i_086760),
    (0x086764, i_086764),
    (0x086768, i_086768),
    (0x08676C, i_08676c),
    (0x086772, i_086772),
    (0x086778, i_086778),
    (0x08677C, i_08677c),
    (0x086782, i_086782),
    (0x086788, i_086788),
    (0x08678E, i_08678e),
    (0x086794, i_086794),
    (0x086798, i_086798),
    (0x08679E, i_08679e),
    (0x0867A4, i_0867a4),
    (0x0867A8, i_0867a8),
    (0x0867AC, i_0867ac),
    (0x0867B0, i_0867b0),
    (0x0867B6, i_0867b6),
    (0x0867BA, i_0867ba),
    (0x0867BC, i_0867bc),
    (0x0867C2, i_0867c2),
    (0x0867C6, i_0867c6),
    (0x0867CC, i_0867cc),
    (0x0867D2, i_0867d2),
    (0x0867D4, i_0867d4),
    (0x0867DA, i_0867da),
    (0x0867DE, i_0867de),
    (0x0867E4, i_0867e4),
    (0x0867EA, i_0867ea),
    (0x0867F0, i_0867f0),
    (0x0867F6, i_0867f6),
    (0x0867FC, i_0867fc),
    (0x086802, i_086802),
    (0x086808, i_086808),
    (0x08680C, i_08680c),
    (0x086810, i_086810),
    (0x086816, i_086816),
    (0x08681C, i_08681c),
    (0x086822, i_086822),
    (0x086826, i_086826),
    (0x08682C, i_08682c),
    (0x086832, i_086832),
    (0x086838, i_086838),
    (0x08683C, i_08683c),
    (0x086840, i_086840),
    (0x086846, i_086846),
    (0x08684C, i_08684c),
    (0x086850, i_086850),
    (0x086852, i_086852),
    (0x086854, i_086854),
    (0x086858, i_086858),
    (0x08685C, i_08685c),
    (0x086862, i_086862),
    (0x086868, i_086868),
    (0x08686A, i_08686a),
    (0x08686E, i_08686e),
    (0x086874, i_086874),
    (0x08687A, i_08687a),
    (0x086892, i_086892),
    (0x086896, i_086896),
    (0x08689C, i_08689c),
    (0x0868A2, i_0868a2),
    (0x0868A6, i_0868a6),
    (0x0868AA, i_0868aa),
    (0x0868B0, i_0868b0),
    (0x0868B6, i_0868b6),
    (0x0868BA, i_0868ba),
    (0x0868BE, i_0868be),
    (0x0868C4, i_0868c4),
    (0x0868CA, i_0868ca),
    (0x0868E2, i_0868e2),
    (0x0868E8, i_0868e8),
    (0x0868EE, i_0868ee),
    (0x0868F4, i_0868f4),
    (0x0868F8, i_0868f8),
    (0x0868FC, i_0868fc),
    (0x086900, i_086900),
    (0x086906, i_086906),
    (0x08690C, i_08690c),
    (0x086912, i_086912),
    (0x086914, i_086914),
    (0x086918, i_086918),
    (0x08691A, i_08691a),
    (0x086920, i_086920),
    (0x086922, i_086922),
    (0x086926, i_086926),
    (0x08692C, i_08692c),
    (0x086930, i_086930),
    (0x086936, i_086936),
    (0x08693A, i_08693a),
    (0x08693E, i_08693e),
    (0x086942, i_086942),
    (0x086944, i_086944),
    (0x086948, i_086948),
    (0x08694E, i_08694e),
    (0x086952, i_086952),
    (0x086958, i_086958),
    (0x08695E, i_08695e),
    (0x086964, i_086964),
    (0x086968, i_086968),
    (0x08696C, i_08696c),
    (0x086972, i_086972),
    (0x086978, i_086978),
    (0x08697C, i_08697c),
    (0x086982, i_086982),
    (0x086986, i_086986),
    (0x08698A, i_08698a),
    (0x08698C, i_08698c),
    (0x086990, i_086990),
    (0x086996, i_086996),
    (0x08699C, i_08699c),
    (0x0869A2, i_0869a2),
    (0x0869A4, i_0869a4),
    (0x0869A6, i_0869a6),
    (0x0869AA, i_0869aa),
    (0x0869AC, i_0869ac),
    (0x0869AE, i_0869ae),
    (0x0869B4, i_0869b4),
    (0x0869B6, i_0869b6),
    (0x0869BA, i_0869ba),
    (0x0869C0, i_0869c0),
    (0x0869C4, i_0869c4),
    (0x0869CA, i_0869ca),
    (0x0869CE, i_0869ce),
    (0x0869D0, i_0869d0),
    (0x0869D4, i_0869d4),
    (0x0869DA, i_0869da),
    (0x0869DE, i_0869de),
    (0x0869E4, i_0869e4),
    (0x0869EA, i_0869ea),
    (0x0869F0, i_0869f0),
    (0x0869F4, i_0869f4),
    (0x0869F8, i_0869f8),
    (0x0869FE, i_0869fe),
    (0x086A04, i_086a04),
    (0x086A08, i_086a08),
    (0x086A0E, i_086a0e),
    (0x086A12, i_086a12),
    (0x086A16, i_086a16),
    (0x086A18, i_086a18),
    (0x086A1C, i_086a1c),
    (0x086A22, i_086a22),
    (0x086A28, i_086a28),
    (0x086A2E, i_086a2e),
    (0x086A30, i_086a30),
    (0x086A32, i_086a32),
    (0x086A36, i_086a36),
    (0x086A38, i_086a38),
    (0x086A3A, i_086a3a),
    (0x086A40, i_086a40),
    (0x086A42, i_086a42),
    (0x086A46, i_086a46),
    (0x086A4C, i_086a4c),
    (0x086A50, i_086a50),
    (0x086A56, i_086a56),
    (0x086A5A, i_086a5a),
    (0x086A5C, i_086a5c),
    (0x086A60, i_086a60),
    (0x086A66, i_086a66),
    (0x086A6A, i_086a6a),
    (0x086A70, i_086a70),
    (0x086A76, i_086a76),
    (0x086A7C, i_086a7c),
    (0x086A80, i_086a80),
    (0x086A84, i_086a84),
    (0x086A8A, i_086a8a),
    (0x086A90, i_086a90),
    (0x086A94, i_086a94),
    (0x086A9A, i_086a9a),
    (0x086A9E, i_086a9e),
    (0x086AA2, i_086aa2),
    (0x086AA4, i_086aa4),
    (0x086AA8, i_086aa8),
    (0x086AAE, i_086aae),
    (0x086AB4, i_086ab4),
    (0x086ABA, i_086aba),
    (0x086ABC, i_086abc),
    (0x086AC0, i_086ac0),
    (0x086AC2, i_086ac2),
    (0x086AC6, i_086ac6),
    (0x086ACC, i_086acc),
    (0x086AD0, i_086ad0),
    (0x086AD6, i_086ad6),
    (0x086ADC, i_086adc),
    (0x086AE2, i_086ae2),
    (0x086AE6, i_086ae6),
    (0x086AEA, i_086aea),
    (0x086AF0, i_086af0),
    (0x086AF6, i_086af6),
    (0x086AFA, i_086afa),
    (0x086B00, i_086b00),
    (0x086B04, i_086b04),
    (0x086B0A, i_086b0a),
    (0x086B0E, i_086b0e),
    (0x086B12, i_086b12),
    (0x086B18, i_086b18),
    (0x086B1C, i_086b1c),
    (0x086B20, i_086b20),
    (0x086B22, i_086b22),
    (0x086B26, i_086b26),
    (0x086B2C, i_086b2c),
    (0x086B32, i_086b32),
    (0x086B36, i_086b36),
    (0x086B3C, i_086b3c),
    (0x086B42, i_086b42),
    (0x086B48, i_086b48),
    (0x086B4E, i_086b4e),
    (0x086B54, i_086b54),
    (0x086B58, i_086b58),
    (0x086B5E, i_086b5e),
    (0x086B64, i_086b64),
    (0x086B6A, i_086b6a),
    (0x086B70, i_086b70),
    (0x086B74, i_086b74),
    (0x086B76, i_086b76),
    (0x086B7C, i_086b7c),
    (0x086B80, i_086b80),
    (0x086B84, i_086b84),
    (0x086B8A, i_086b8a),
    (0x086B90, i_086b90),
    (0x086B96, i_086b96),
    (0x086B9C, i_086b9c),
    (0x086BA2, i_086ba2),
    (0x086BA4, i_086ba4),
    (0x086BA8, i_086ba8),
    (0x086BAE, i_086bae),
    (0x086BB4, i_086bb4),
    (0x086BB8, i_086bb8),
    (0x086BBA, i_086bba),
    (0x086BBE, i_086bbe),
    (0x086BC4, i_086bc4),
    (0x086BC8, i_086bc8),
    (0x086BCE, i_086bce),
    (0x086BD4, i_086bd4),
    (0x086BD8, i_086bd8),
    (0x086BDC, i_086bdc),
    (0x086BE0, i_086be0),
    (0x086BE4, i_086be4),
    (0x086BEA, i_086bea),
    (0x086BF0, i_086bf0),
    (0x086BF4, i_086bf4),
    (0x086BFA, i_086bfa),
    (0x086C00, i_086c00),
    (0x086C06, i_086c06),
    (0x086C0C, i_086c0c),
    (0x086C12, i_086c12),
    (0x086C16, i_086c16),
    (0x086C1C, i_086c1c),
    (0x086C22, i_086c22),
    (0x086C28, i_086c28),
    (0x086C2E, i_086c2e),
    (0x086C32, i_086c32),
    (0x086C34, i_086c34),
    (0x086C3A, i_086c3a),
    (0x086C3E, i_086c3e),
    (0x086C42, i_086c42),
    (0x086C48, i_086c48),
    (0x086C4E, i_086c4e),
    (0x086C54, i_086c54),
    (0x086C5A, i_086c5a),
    (0x086C60, i_086c60),
    (0x086C62, i_086c62),
    (0x086C66, i_086c66),
    (0x086C6C, i_086c6c),
    (0x086C72, i_086c72),
    (0x086C78, i_086c78),
    (0x086C7E, i_086c7e),
    (0x086C84, i_086c84),
    (0x086C8A, i_086c8a),
    (0x086C90, i_086c90),
    (0x086C96, i_086c96),
    (0x086C9A, i_086c9a),
    (0x086C9C, i_086c9c),
    (0x086CA0, i_086ca0),
    (0x086CA4, i_086ca4),
    (0x086CAA, i_086caa),
    (0x086CAE, i_086cae),
    (0x086CB4, i_086cb4),
    (0x086CBA, i_086cba),
    (0x086CBE, i_086cbe),
    (0x086CC2, i_086cc2),
    (0x086CC8, i_086cc8),
    (0x086CCE, i_086cce),
    (0x086CD0, i_086cd0),
    (0x086CD4, i_086cd4),
    (0x086CDA, i_086cda),
    (0x086CE0, i_086ce0),
    (0x086CE4, i_086ce4),
    (0x086CE8, i_086ce8),
    (0x086CEE, i_086cee),
    (0x086CF4, i_086cf4),
    (0x086CF8, i_086cf8),
    (0x086CFC, i_086cfc),
    (0x086D02, i_086d02),
    (0x086D08, i_086d08),
    (0x086D0C, i_086d0c),
    (0x086D10, i_086d10),
    (0x086D16, i_086d16),
    (0x086D1C, i_086d1c),
    (0x086D20, i_086d20),
    (0x086D24, i_086d24),
    (0x086D2A, i_086d2a),
    (0x086D30, i_086d30),
    (0x086D34, i_086d34),
    (0x086D3A, i_086d3a),
    (0x086D40, i_086d40),
    (0x086D46, i_086d46),
    (0x086D4C, i_086d4c),
    (0x086D52, i_086d52),
    (0x086D56, i_086d56),
    (0x086D58, i_086d58),
    (0x086D5C, i_086d5c),
    (0x086D62, i_086d62),
    (0x086D66, i_086d66),
    (0x086D6C, i_086d6c),
    (0x086D72, i_086d72),
    (0x086D76, i_086d76),
    (0x086D7C, i_086d7c),
    (0x086D82, i_086d82),
    (0x086D88, i_086d88),
    (0x086D8C, i_086d8c),
    (0x086D92, i_086d92),
    (0x086D96, i_086d96),
    (0x086D9A, i_086d9a),
    (0x086D9E, i_086d9e),
    (0x086DA2, i_086da2),
    (0x086DA4, i_086da4),
    (0x086DA8, i_086da8),
    (0x086DAE, i_086dae),
    (0x086DB2, i_086db2),
    (0x086DB4, i_086db4),
    (0x086DBA, i_086dba),
    (0x086DC0, i_086dc0),
    (0x086DC4, i_086dc4),
    (0x086DCA, i_086dca),
    (0x086DCC, i_086dcc),
    (0x086DD0, i_086dd0),
    (0x086DD6, i_086dd6),
    (0x086DD8, i_086dd8),
    (0x086DDC, i_086ddc),
    (0x086DE2, i_086de2),
    (0x086DE6, i_086de6),
    (0x086DEC, i_086dec),
    (0x086DF0, i_086df0),
    (0x086DF2, i_086df2),
    (0x086DF6, i_086df6),
    (0x086DFC, i_086dfc),
    (0x086E00, i_086e00),
    (0x086E06, i_086e06),
    (0x086E0C, i_086e0c),
    (0x086E10, i_086e10),
    (0x086E14, i_086e14),
    (0x086E1A, i_086e1a),
    (0x086E20, i_086e20),
    (0x086E26, i_086e26),
    (0x086E2C, i_086e2c),
    (0x086E30, i_086e30),
    (0x086E36, i_086e36),
    (0x086E3C, i_086e3c),
    (0x086E42, i_086e42),
    (0x086E48, i_086e48),
    (0x086E4A, i_086e4a),
    (0x086E50, i_086e50),
    (0x086E56, i_086e56),
    (0x086E5C, i_086e5c),
    (0x086E62, i_086e62),
    (0x086E66, i_086e66),
    (0x086E6C, i_086e6c),
    (0x086E70, i_086e70),
    (0x086E74, i_086e74),
    (0x086E7A, i_086e7a),
    (0x086E80, i_086e80),
    (0x086E86, i_086e86),
    (0x086E8C, i_086e8c),
    (0x086E90, i_086e90),
    (0x086E96, i_086e96),
    (0x086E9A, i_086e9a),
    (0x086E9E, i_086e9e),
    (0x086EA4, i_086ea4),
    (0x086EAA, i_086eaa),
    (0x086EB0, i_086eb0),
    (0x086EB6, i_086eb6),
    (0x086EBA, i_086eba),
    (0x086EC0, i_086ec0),
    (0x086EC4, i_086ec4),
    (0x086EC8, i_086ec8),
    (0x086ECE, i_086ece),
    (0x086ED4, i_086ed4),
    (0x086EDA, i_086eda),
    (0x086EE0, i_086ee0),
    (0x086EE4, i_086ee4),
    (0x086EEA, i_086eea),
    (0x086EEE, i_086eee),
    (0x086EF2, i_086ef2),
    (0x086EF8, i_086ef8),
    (0x086EFE, i_086efe),
    (0x086F04, i_086f04),
    (0x086F0A, i_086f0a),
    (0x086F0E, i_086f0e),
    (0x086F14, i_086f14),
    (0x086F18, i_086f18),
    (0x086F1C, i_086f1c),
    (0x086F20, i_086f20),
    (0x086F26, i_086f26),
    (0x086F2C, i_086f2c),
    (0x086F32, i_086f32),
    (0x086F38, i_086f38),
    (0x086F3C, i_086f3c),
    (0x086F40, i_086f40),
    (0x086F44, i_086f44),
    (0x086F4A, i_086f4a),
    (0x086F50, i_086f50),
    (0x086F56, i_086f56),
    (0x086F5C, i_086f5c),
    (0x086F60, i_086f60),
    (0x086F64, i_086f64),
    (0x086F6A, i_086f6a),
    (0x086F6E, i_086f6e),
    (0x086F70, i_086f70),
    (0x086F76, i_086f76),
    (0x086F7A, i_086f7a),
    (0x086F7E, i_086f7e),
    (0x086F82, i_086f82),
    (0x086F84, i_086f84),
    (0x086F8A, i_086f8a),
    (0x086F8E, i_086f8e),
    (0x086F92, i_086f92),
    (0x086F98, i_086f98),
    (0x086F9C, i_086f9c),
    (0x086FA0, i_086fa0),
    (0x086FA2, i_086fa2),
    (0x086FA8, i_086fa8),
    (0x086FAC, i_086fac),
    (0x086FB2, i_086fb2),
    (0x086FB4, i_086fb4),
    (0x086FB6, i_086fb6),
    (0x086FBA, i_086fba),
    (0x086FC0, i_086fc0),
    (0x086FC2, i_086fc2),
    (0x086FC6, i_086fc6),
    (0x086FCC, i_086fcc),
    (0x086FD0, i_086fd0),
    (0x086FD6, i_086fd6),
    (0x086FDA, i_086fda),
    (0x086FDC, i_086fdc),
    (0x086FE2, i_086fe2),
    (0x086FE8, i_086fe8),
    (0x086FEC, i_086fec),
    (0x086FF0, i_086ff0),
    (0x086FF2, i_086ff2),
    (0x086FF8, i_086ff8),
    (0x086FFC, i_086ffc),
    (0x087002, i_087002),
    (0x087004, i_087004),
    (0x08700A, i_08700a),
    (0x08700E, i_08700e),
    (0x087012, i_087012),
    (0x087018, i_087018),
    (0x08701E, i_08701e),
    (0x087022, i_087022),
    (0x087024, i_087024),
    (0x08702A, i_08702a),
    (0x08702E, i_08702e),
    (0x087032, i_087032),
    (0x087036, i_087036),
    (0x087038, i_087038),
    (0x08703E, i_08703e),
    (0x087042, i_087042),
    (0x087046, i_087046),
    (0x08704C, i_08704c),
    (0x087050, i_087050),
    (0x087054, i_087054),
    (0x087056, i_087056),
    (0x08705C, i_08705c),
    (0x087060, i_087060),
    (0x087066, i_087066),
    (0x087068, i_087068),
    (0x08706E, i_08706e),
    (0x087072, i_087072),
    (0x087074, i_087074),
    (0x08707A, i_08707a),
    (0x08707E, i_08707e),
    (0x087082, i_087082),
    (0x087086, i_087086),
    (0x087088, i_087088),
    (0x08708E, i_08708e),
    (0x087092, i_087092),
    (0x087096, i_087096),
    (0x08709C, i_08709c),
    (0x0870A0, i_0870a0),
    (0x0870A4, i_0870a4),
    (0x0870A6, i_0870a6),
    (0x0870AC, i_0870ac),
    (0x0870B0, i_0870b0),
    (0x0870B6, i_0870b6),
    (0x0870B8, i_0870b8),
    (0x0870BA, i_0870ba),
    (0x0870BE, i_0870be),
    (0x0870C4, i_0870c4),
    (0x0870C6, i_0870c6),
    (0x0870CA, i_0870ca),
    (0x0870D0, i_0870d0),
    (0x0870D4, i_0870d4),
    (0x0870DA, i_0870da),
    (0x0870DE, i_0870de),
    (0x0870E0, i_0870e0),
    (0x0870E6, i_0870e6),
    (0x0870EC, i_0870ec),
    (0x0870F0, i_0870f0),
    (0x0870F4, i_0870f4),
    (0x0870F6, i_0870f6),
    (0x0870FC, i_0870fc),
    (0x087100, i_087100),
    (0x087106, i_087106),
    (0x087108, i_087108),
    (0x08710A, i_08710a),
    (0x08710E, i_08710e),
    (0x087114, i_087114),
    (0x087116, i_087116),
    (0x08711A, i_08711a),
    (0x087120, i_087120),
    (0x087124, i_087124),
    (0x08712A, i_08712a),
    (0x08712E, i_08712e),
    (0x087130, i_087130),
    (0x087136, i_087136),
    (0x08713C, i_08713c),
    (0x087140, i_087140),
    (0x087144, i_087144),
    (0x087148, i_087148),
    (0x08714E, i_08714e),
    (0x087154, i_087154),
    (0x087158, i_087158),
    (0x08715E, i_08715e),
    (0x087162, i_087162),
    (0x087168, i_087168),
    (0x08716A, i_08716a),
    (0x08716E, i_08716e),
    (0x087174, i_087174),
    (0x08717A, i_08717a),
    (0x087180, i_087180),
    (0x087184, i_087184),
    (0x087188, i_087188),
    (0x08718A, i_08718a),
    (0x087190, i_087190),
    (0x087196, i_087196),
    (0x08719A, i_08719a),
    (0x0871A0, i_0871a0),
    (0x0871A6, i_0871a6),
    (0x0871AC, i_0871ac),
    (0x0871B2, i_0871b2),
    (0x0871B6, i_0871b6),
    (0x0871BA, i_0871ba),
    (0x0871C0, i_0871c0),
    (0x0871C6, i_0871c6),
    (0x0871CC, i_0871cc),
    (0x0871D2, i_0871d2),
    (0x0871D8, i_0871d8),
    (0x0871DE, i_0871de),
    (0x0871E4, i_0871e4),
    (0x0871E8, i_0871e8),
    (0x0871EE, i_0871ee),
    (0x0871F2, i_0871f2),
    (0x0871F8, i_0871f8),
    (0x0871FA, i_0871fa),
    (0x0871FE, i_0871fe),
    (0x087204, i_087204),
    (0x08720A, i_08720a),
    (0x087210, i_087210),
    (0x087214, i_087214),
    (0x087218, i_087218),
    (0x08721A, i_08721a),
    (0x087220, i_087220),
    (0x087226, i_087226),
    (0x08722A, i_08722a),
    (0x087230, i_087230),
    (0x087236, i_087236),
    (0x08723C, i_08723c),
    (0x087242, i_087242),
    (0x087246, i_087246),
    (0x08724C, i_08724c),
    (0x087252, i_087252),
    (0x087258, i_087258),
    (0x08725E, i_08725e),
    (0x087264, i_087264),
    (0x08726A, i_08726a),
    (0x08726E, i_08726e),
    (0x087274, i_087274),
    (0x087278, i_087278),
    (0x08727E, i_08727e),
    (0x087280, i_087280),
    (0x087284, i_087284),
    (0x08728A, i_08728a),
    (0x087290, i_087290),
    (0x087296, i_087296),
    (0x08729A, i_08729a),
    (0x08729E, i_08729e),
    (0x0872A0, i_0872a0),
    (0x0872A6, i_0872a6),
    (0x0872AC, i_0872ac),
    (0x0872B0, i_0872b0),
    (0x0872B6, i_0872b6),
    (0x0872BC, i_0872bc),
    (0x0872C2, i_0872c2),
    (0x0872C8, i_0872c8),
    (0x0872CC, i_0872cc),
    (0x0872D2, i_0872d2),
    (0x0872D8, i_0872d8),
    (0x0872DE, i_0872de),
    (0x0872E4, i_0872e4),
    (0x0872E8, i_0872e8),
    (0x0872EE, i_0872ee),
    (0x0872F2, i_0872f2),
    (0x0872F8, i_0872f8),
    (0x0872FA, i_0872fa),
    (0x0872FE, i_0872fe),
    (0x087304, i_087304),
    (0x08730A, i_08730a),
    (0x087310, i_087310),
    (0x087316, i_087316),
    (0x08731A, i_08731a),
    (0x08731C, i_08731c),
    (0x087322, i_087322),
    (0x087328, i_087328),
    (0x08732E, i_08732e),
    (0x087332, i_087332),
    (0x087336, i_087336),
    (0x08733C, i_08733c),
    (0x087342, i_087342),
    (0x087348, i_087348),
    (0x08734E, i_08734e),
    (0x087354, i_087354),
    (0x08735A, i_08735a),
    (0x08735E, i_08735e),
    (0x087364, i_087364),
    (0x087366, i_087366),
    (0x08736A, i_08736a),
    (0x087370, i_087370),
    (0x087374, i_087374),
    (0x08737A, i_08737a),
    (0x087380, i_087380),
    (0x087386, i_087386),
    (0x08738C, i_08738c),
    (0x087390, i_087390),
    (0x087396, i_087396),
    (0x08739C, i_08739c),
    (0x0873A2, i_0873a2),
    (0x0873A8, i_0873a8),
    (0x0873AC, i_0873ac),
    (0x0873B2, i_0873b2),
    (0x0873B8, i_0873b8),
    (0x0873BE, i_0873be),
    (0x0873C4, i_0873c4),
    (0x0873C8, i_0873c8),
    (0x0873CE, i_0873ce),
    (0x0873D4, i_0873d4),
    (0x0873DA, i_0873da),
    (0x0873E0, i_0873e0),
    (0x0873E4, i_0873e4),
    (0x0873EA, i_0873ea),
    (0x0873F0, i_0873f0),
    (0x0873F6, i_0873f6),
    (0x0873FC, i_0873fc),
    (0x087400, i_087400),
    (0x087406, i_087406),
    (0x08740C, i_08740c),
    (0x087412, i_087412),
    (0x087418, i_087418),
    (0x08741C, i_08741c),
    (0x087422, i_087422),
    (0x087428, i_087428),
    (0x08742E, i_08742e),
    (0x087434, i_087434),
    (0x087438, i_087438),
    (0x08743E, i_08743e),
    (0x087444, i_087444),
    (0x087448, i_087448),
    (0x08744E, i_08744e),
    (0x087454, i_087454),
    (0x08745A, i_08745a),
    (0x08745E, i_08745e),
    (0x087464, i_087464),
    (0x08746A, i_08746a),
    (0x087470, i_087470),
    (0x087474, i_087474),
    (0x08747A, i_08747a),
    (0x087480, i_087480),
    (0x087486, i_087486),
    (0x08748A, i_08748a),
    (0x087490, i_087490),
    (0x087496, i_087496),
    (0x08749C, i_08749c),
    (0x0874A0, i_0874a0),
    (0x0874A6, i_0874a6),
    (0x0874AC, i_0874ac),
    (0x0874B2, i_0874b2),
    (0x0874B8, i_0874b8),
    (0x0874BA, i_0874ba),
    (0x0874C0, i_0874c0),
    (0x0874C4, i_0874c4),
    (0x0874CC, i_0874cc),
    (0x0874D2, i_0874d2),
    (0x0874D8, i_0874d8),
    (0x0874DC, i_0874dc),
    (0x0874E2, i_0874e2),
    (0x0874E6, i_0874e6),
    (0x0874EA, i_0874ea),
    (0x0874F0, i_0874f0),
    (0x0874F4, i_0874f4),
    (0x0874FA, i_0874fa),
    (0x0874FE, i_0874fe),
    (0x087504, i_087504),
    (0x08750A, i_08750a),
    (0x08750E, i_08750e),
    (0x087514, i_087514),
    (0x087518, i_087518),
    (0x08751C, i_08751c),
    (0x087522, i_087522),
    (0x087526, i_087526),
    (0x08752C, i_08752c),
    (0x087530, i_087530),
    (0x087536, i_087536),
    (0x08753C, i_08753c),
    (0x087540, i_087540),
    (0x087546, i_087546),
    (0x08754A, i_08754a),
    (0x08754E, i_08754e),
    (0x087554, i_087554),
    (0x087558, i_087558),
    (0x08755E, i_08755e),
    (0x087562, i_087562),
    (0x087568, i_087568),
    (0x08756E, i_08756e),
    (0x087572, i_087572),
    (0x087578, i_087578),
    (0x08757C, i_08757c),
    (0x087580, i_087580),
    (0x087586, i_087586),
    (0x08758A, i_08758a),
    (0x087592, i_087592),
    (0x087598, i_087598),
    (0x08759E, i_08759e),
    (0x0875A2, i_0875a2),
    (0x0875A8, i_0875a8),
    (0x0875AC, i_0875ac),
    (0x0875B0, i_0875b0),
    (0x0875B6, i_0875b6),
    (0x0875BA, i_0875ba),
    (0x0875C2, i_0875c2),
    (0x0875C8, i_0875c8),
    (0x0875CE, i_0875ce),
    (0x0875D2, i_0875d2),
    (0x0875D8, i_0875d8),
    (0x0875DC, i_0875dc),
    (0x0875E0, i_0875e0),
    (0x0875E6, i_0875e6),
    (0x0875EA, i_0875ea),
    (0x0875F2, i_0875f2),
    (0x0875F8, i_0875f8),
    (0x0875FE, i_0875fe),
    (0x087602, i_087602),
    (0x087608, i_087608),
    (0x08760C, i_08760c),
    (0x087610, i_087610),
    (0x087616, i_087616),
    (0x08761A, i_08761a),
    (0x087622, i_087622),
    (0x087628, i_087628),
    (0x08762E, i_08762e),
    (0x087632, i_087632),
    (0x087638, i_087638),
    (0x08763C, i_08763c),
    (0x087640, i_087640),
    (0x087646, i_087646),
    (0x08764A, i_08764a),
    (0x087652, i_087652),
    (0x087658, i_087658),
    (0x08765E, i_08765e),
    (0x087662, i_087662),
    (0x087668, i_087668),
    (0x08766C, i_08766c),
    (0x087670, i_087670),
    (0x087676, i_087676),
    (0x08767A, i_08767a),
    (0x087682, i_087682),
    (0x087688, i_087688),
    (0x08768E, i_08768e),
    (0x087692, i_087692),
    (0x087698, i_087698),
    (0x08769C, i_08769c),
    (0x0876A0, i_0876a0),
    (0x0876A6, i_0876a6),
    (0x0876AA, i_0876aa),
    (0x0876B2, i_0876b2),
    (0x0876B8, i_0876b8),
    (0x0876BE, i_0876be),
    (0x0876C2, i_0876c2),
    (0x0876C8, i_0876c8),
    (0x0876CC, i_0876cc),
    (0x0876D0, i_0876d0),
    (0x0876D6, i_0876d6),
    (0x0876DA, i_0876da),
    (0x0876E2, i_0876e2),
    (0x0876E8, i_0876e8),
    (0x0876EE, i_0876ee),
    (0x0876F2, i_0876f2),
    (0x0876F8, i_0876f8),
    (0x0876FC, i_0876fc),
    (0x087700, i_087700),
    (0x087706, i_087706),
    (0x08770A, i_08770a),
    (0x087712, i_087712),
    (0x087718, i_087718),
    (0x08771E, i_08771e),
    (0x087722, i_087722),
    (0x087728, i_087728),
    (0x08772C, i_08772c),
    (0x087730, i_087730),
    (0x087736, i_087736),
    (0x08773C, i_08773c),
    (0x087742, i_087742),
    (0x087746, i_087746),
    (0x087748, i_087748),
    (0x08774E, i_08774e),
    (0x087754, i_087754),
    (0x087758, i_087758),
    (0x08775E, i_08775e),
    (0x087764, i_087764),
    (0x08776A, i_08776a),
    (0x087770, i_087770),
    (0x087774, i_087774),
    (0x087778, i_087778),
    (0x08777C, i_08777c),
    (0x087780, i_087780),
    (0x087786, i_087786),
    (0x08778A, i_08778a),
    (0x087790, i_087790),
    (0x087794, i_087794),
    (0x08779A, i_08779a),
    (0x0877A2, i_0877a2),
    (0x0877A6, i_0877a6),
    (0x0877AA, i_0877aa),
    (0x0877AE, i_0877ae),
    (0x0877B4, i_0877b4),
    (0x0877BA, i_0877ba),
    (0x0877BE, i_0877be),
    (0x0877C4, i_0877c4),
    (0x0877C8, i_0877c8),
    (0x0877CE, i_0877ce),
    (0x0877D2, i_0877d2),
    (0x0877D4, i_0877d4),
    (0x0877D8, i_0877d8),
    (0x0877DE, i_0877de),
    (0x0877E2, i_0877e2),
    (0x0877E8, i_0877e8),
    (0x0877EE, i_0877ee),
    (0x0877F4, i_0877f4),
    (0x0877FA, i_0877fa),
    (0x087800, i_087800),
    (0x087806, i_087806),
    (0x08780C, i_08780c),
    (0x087812, i_087812),
    (0x087816, i_087816),
    (0x08781C, i_08781c),
    (0x087822, i_087822),
    (0x087826, i_087826),
    (0x087828, i_087828),
    (0x08782E, i_08782e),
    (0x087832, i_087832),
    (0x087838, i_087838),
    (0x08783E, i_08783e),
    (0x087844, i_087844),
    (0x087846, i_087846),
    (0x08784C, i_08784c),
    (0x087852, i_087852),
    (0x087858, i_087858),
    (0x08785E, i_08785e),
    (0x087862, i_087862),
    (0x087868, i_087868),
    (0x087870, i_087870),
    (0x087874, i_087874),
    (0x087876, i_087876),
    (0x08787C, i_08787c),
    (0x087882, i_087882),
    (0x087888, i_087888),
    (0x08788E, i_08788e),
    (0x087894, i_087894),
    (0x087898, i_087898),
    (0x08789E, i_08789e),
    (0x0878A4, i_0878a4),
    (0x0878AA, i_0878aa),
    (0x0878B0, i_0878b0),
    (0x0878B4, i_0878b4),
    (0x0878BA, i_0878ba),
    (0x0878C0, i_0878c0),
    (0x0878C4, i_0878c4),
    (0x0878C6, i_0878c6),
    (0x0878CC, i_0878cc),
    (0x0878D0, i_0878d0),
    (0x0878D6, i_0878d6),
    (0x0878D8, i_0878d8),
    (0x0878DC, i_0878dc),
    (0x0878E2, i_0878e2),
    (0x0878E8, i_0878e8),
    (0x0878EE, i_0878ee),
    (0x0878F2, i_0878f2),
    (0x0878F4, i_0878f4),
    (0x0878FA, i_0878fa),
    (0x087900, i_087900),
    (0x087906, i_087906),
    (0x08790C, i_08790c),
    (0x087912, i_087912),
    (0x087916, i_087916),
    (0x08791C, i_08791c),
    (0x087922, i_087922),
    (0x087928, i_087928),
    (0x08792E, i_08792e),
    (0x087932, i_087932),
    (0x087938, i_087938),
    (0x08793E, i_08793e),
    (0x087942, i_087942),
    (0x087944, i_087944),
    (0x08794A, i_08794a),
    (0x08794E, i_08794e),
    (0x087954, i_087954),
    (0x087956, i_087956),
    (0x08795A, i_08795a),
    (0x087960, i_087960),
    (0x087966, i_087966),
    (0x08796C, i_08796c),
    (0x087970, i_087970),
    (0x087972, i_087972),
    (0x087978, i_087978),
    (0x08797E, i_08797e),
    (0x087984, i_087984),
    (0x08798A, i_08798a),
    (0x087990, i_087990),
    (0x087994, i_087994),
    (0x08799A, i_08799a),
    (0x0879A0, i_0879a0),
    (0x0879A6, i_0879a6),
    (0x0879AC, i_0879ac),
    (0x0879B0, i_0879b0),
    (0x0879B4, i_0879b4),
    (0x0879BA, i_0879ba),
    (0x0879C0, i_0879c0),
    (0x0879C6, i_0879c6),
    (0x0879CC, i_0879cc),
    (0x0879D2, i_0879d2),
    (0x0879D8, i_0879d8),
    (0x0879DE, i_0879de),
    (0x0879E4, i_0879e4),
    (0x0879EA, i_0879ea),
    (0x0879F0, i_0879f0),
    (0x0879F4, i_0879f4),
    (0x0879FA, i_0879fa),
    (0x087A00, i_087a00),
    (0x087A04, i_087a04),
    (0x087A06, i_087a06),
    (0x087A0C, i_087a0c),
    (0x087A10, i_087a10),
    (0x087A16, i_087a16),
    (0x087A18, i_087a18),
    (0x087A1E, i_087a1e),
    (0x087A24, i_087a24),
    (0x087A28, i_087a28),
    (0x087A2C, i_087a2c),
    (0x087A32, i_087a32),
    (0x087A38, i_087a38),
    (0x087A3C, i_087a3c),
    (0x087A3E, i_087a3e),
    (0x087A44, i_087a44),
    (0x087A4A, i_087a4a),
    (0x087A50, i_087a50),
    (0x087A56, i_087a56),
    (0x087A5A, i_087a5a),
    (0x087A60, i_087a60),
    (0x087A62, i_087a62),
    (0x087A68, i_087a68),
    (0x087A6C, i_087a6c),
    (0x087A6E, i_087a6e),
    (0x087A74, i_087a74),
    (0x087A7A, i_087a7a),
    (0x087A7E, i_087a7e),
    (0x087A84, i_087a84),
    (0x087A8A, i_087a8a),
    (0x087A90, i_087a90),
    (0x087A94, i_087a94),
    (0x087A98, i_087a98),
    (0x087A9E, i_087a9e),
    (0x087AA0, i_087aa0),
    (0x087AA4, i_087aa4),
    (0x087AAA, i_087aaa),
    (0x087AB0, i_087ab0),
    (0x087AB6, i_087ab6),
    (0x087ABC, i_087abc),
    (0x087AC0, i_087ac0),
    (0x087AC4, i_087ac4),
    (0x087ACA, i_087aca),
    (0x087ACE, i_087ace),
    (0x087AD0, i_087ad0),
    (0x087AD4, i_087ad4),
    (0x087ADA, i_087ada),
    (0x087AE0, i_087ae0),
    (0x087AE2, i_087ae2),
    (0x087AE6, i_087ae6),
    (0x087AEA, i_087aea),
    (0x087AEE, i_087aee),
    (0x087AF4, i_087af4),
    (0x087AF6, i_087af6),
    (0x087AFA, i_087afa),
    (0x087B00, i_087b00),
    (0x087B04, i_087b04),
    (0x087B0A, i_087b0a),
    (0x087B10, i_087b10),
    (0x087B14, i_087b14),
    (0x087B1A, i_087b1a),
    (0x087B1C, i_087b1c),
    (0x087B20, i_087b20),
    (0x087B26, i_087b26),
    (0x087B2C, i_087b2c),
    (0x087B30, i_087b30),
    (0x087B36, i_087b36),
    (0x087B3A, i_087b3a),
    (0x087B40, i_087b40),
    (0x087B46, i_087b46),
    (0x087B4C, i_087b4c),
    (0x087B52, i_087b52),
    (0x087B58, i_087b58),
    (0x087B5E, i_087b5e),
    (0x087B64, i_087b64),
    (0x087B68, i_087b68),
    (0x087B6E, i_087b6e),
    (0x087B74, i_087b74),
    (0x087B7A, i_087b7a),
    (0x087B80, i_087b80),
    (0x087B86, i_087b86),
    (0x087B8C, i_087b8c),
    (0x087B90, i_087b90),
    (0x087B96, i_087b96),
    (0x087B9C, i_087b9c),
    (0x087BA2, i_087ba2),
    (0x087BA8, i_087ba8),
    (0x087BB0, i_087bb0),
    (0x087BB4, i_087bb4),
    (0x087BB6, i_087bb6),
    (0x087BBC, i_087bbc),
    (0x087BC2, i_087bc2),
    (0x087BC6, i_087bc6),
    (0x087BCC, i_087bcc),
    (0x087BD2, i_087bd2),
    (0x087BD6, i_087bd6),
    (0x087BDC, i_087bdc),
    (0x087BE2, i_087be2),
    (0x087BE8, i_087be8),
    (0x087BEE, i_087bee),
    (0x087BF4, i_087bf4),
    (0x087BFA, i_087bfa),
    (0x087C00, i_087c00),
    (0x087C04, i_087c04),
    (0x087C08, i_087c08),
    (0x087C0A, i_087c0a),
    (0x087C10, i_087c10),
    (0x087C14, i_087c14),
    (0x087C1A, i_087c1a),
    (0x087C20, i_087c20),
    (0x087C22, i_087c22),
    (0x087C28, i_087c28),
    (0x087C2E, i_087c2e),
    (0x087C32, i_087c32),
    (0x087C34, i_087c34),
    (0x087C3A, i_087c3a),
    (0x087C40, i_087c40),
    (0x087C44, i_087c44),
    (0x087C4A, i_087c4a),
    (0x087C4E, i_087c4e),
    (0x087C52, i_087c52),
    (0x087C54, i_087c54),
    (0x087C5A, i_087c5a),
    (0x087C5E, i_087c5e),
    (0x087C64, i_087c64),
    (0x087C6A, i_087c6a),
    (0x087C70, i_087c70),
    (0x087C76, i_087c76),
    (0x087C7C, i_087c7c),
    (0x087C82, i_087c82),
    (0x087C88, i_087c88),
    (0x087C8C, i_087c8c),
    (0x087C92, i_087c92),
    (0x087C98, i_087c98),
    (0x087C9A, i_087c9a),
    (0x087CA0, i_087ca0),
    (0x087CA4, i_087ca4),
    (0x087CAA, i_087caa),
    (0x087CB0, i_087cb0),
    (0x087CB4, i_087cb4),
    (0x087CB6, i_087cb6),
    (0x087CBC, i_087cbc),
    (0x087CC2, i_087cc2),
    (0x087CC6, i_087cc6),
    (0x087CCC, i_087ccc),
    (0x087CD2, i_087cd2),
    (0x087CD6, i_087cd6),
    (0x087CDC, i_087cdc),
    (0x087CE2, i_087ce2),
    (0x087CE8, i_087ce8),
    (0x087CEE, i_087cee),
    (0x087CF4, i_087cf4),
    (0x087CF8, i_087cf8),
    (0x087CFC, i_087cfc),
    (0x087CFE, i_087cfe),
    (0x087D04, i_087d04),
    (0x087D08, i_087d08),
    (0x087D0E, i_087d0e),
    (0x087D14, i_087d14),
    (0x087D1A, i_087d1a),
    (0x087D1C, i_087d1c),
    (0x087D20, i_087d20),
    (0x087D26, i_087d26),
    (0x087D2C, i_087d2c),
    (0x087D32, i_087d32),
    (0x087D36, i_087d36),
    (0x087D38, i_087d38),
    (0x087D3E, i_087d3e),
    (0x087D44, i_087d44),
    (0x087D48, i_087d48),
    (0x087D4E, i_087d4e),
    (0x087D54, i_087d54),
    (0x087D58, i_087d58),
    (0x087D5E, i_087d5e),
    (0x087D64, i_087d64),
    (0x087D6A, i_087d6a),
    (0x087D70, i_087d70),
    (0x087D76, i_087d76),
    (0x087D7A, i_087d7a),
    (0x087D7E, i_087d7e),
    (0x087D80, i_087d80),
    (0x087D86, i_087d86),
    (0x087D8A, i_087d8a),
    (0x087D90, i_087d90),
    (0x087D96, i_087d96),
    (0x087D9C, i_087d9c),
    (0x087D9E, i_087d9e),
    (0x087DA2, i_087da2),
    (0x087DA8, i_087da8),
    (0x087DAE, i_087dae),
    (0x087DB4, i_087db4),
    (0x087DB8, i_087db8),
    (0x087DBA, i_087dba),
    (0x087DC0, i_087dc0),
    (0x087DC6, i_087dc6),
    (0x087DCA, i_087dca),
    (0x087DD0, i_087dd0),
    (0x087DD6, i_087dd6),
    (0x087DDA, i_087dda),
    (0x087DE0, i_087de0),
    (0x087DE6, i_087de6),
    (0x087DEC, i_087dec),
    (0x087DF2, i_087df2),
    (0x087DF8, i_087df8),
    (0x087DFC, i_087dfc),
    (0x087E00, i_087e00),
    (0x087E02, i_087e02),
    (0x087E08, i_087e08),
    (0x087E0C, i_087e0c),
    (0x087E12, i_087e12),
    (0x087E18, i_087e18),
    (0x087E1E, i_087e1e),
    (0x087E20, i_087e20),
    (0x087E26, i_087e26),
    (0x087E2C, i_087e2c),
    (0x087E30, i_087e30),
    (0x087E36, i_087e36),
    (0x087E3C, i_087e3c),
    (0x087E42, i_087e42),
    (0x087E48, i_087e48),
    (0x087E4E, i_087e4e),
    (0x087E54, i_087e54),
    (0x087E5A, i_087e5a),
    (0x087E60, i_087e60),
    (0x087E66, i_087e66),
    (0x087E6C, i_087e6c),
    (0x087E72, i_087e72),
    (0x087E78, i_087e78),
    (0x087E7E, i_087e7e),
    (0x087E84, i_087e84),
    (0x087E8A, i_087e8a),
    (0x087E8E, i_087e8e),
    (0x087E90, i_087e90),
    (0x087E96, i_087e96),
    (0x087E9C, i_087e9c),
    (0x087EA2, i_087ea2),
    (0x087EA6, i_087ea6),
    (0x087EAC, i_087eac),
    (0x087EAE, i_087eae),
    (0x087EB2, i_087eb2),
    (0x087EB8, i_087eb8),
    (0x087EBE, i_087ebe),
    (0x087EC4, i_087ec4),
    (0x087ECA, i_087eca),
    (0x087ED0, i_087ed0),
    (0x087ED6, i_087ed6),
    (0x087EDC, i_087edc),
    (0x087EE2, i_087ee2),
    (0x087EE8, i_087ee8),
    (0x087EEE, i_087eee),
    (0x087EF4, i_087ef4),
    (0x087EFA, i_087efa),
    (0x087F00, i_087f00),
    (0x087F06, i_087f06),
    (0x087F0A, i_087f0a),
    (0x087F0C, i_087f0c),
    (0x087F12, i_087f12),
    (0x087F18, i_087f18),
    (0x087F1C, i_087f1c),
    (0x087F22, i_087f22),
    (0x087F26, i_087f26),
    (0x087F2C, i_087f2c),
    (0x087F32, i_087f32),
    (0x087F38, i_087f38),
    (0x087F3E, i_087f3e),
    (0x087F44, i_087f44),
    (0x087F4A, i_087f4a),
    (0x087F4C, i_087f4c),
    (0x087F50, i_087f50),
    (0x087F56, i_087f56),
    (0x087F5C, i_087f5c),
    (0x087F60, i_087f60),
    (0x087F66, i_087f66),
    (0x087F6A, i_087f6a),
    (0x087F6E, i_087f6e),
    (0x087F70, i_087f70),
    (0x087F74, i_087f74),
    (0x087F78, i_087f78),
    (0x087F7C, i_087f7c),
    (0x087F82, i_087f82),
    (0x087F88, i_087f88),
    (0x087F8C, i_087f8c),
    (0x087F8E, i_087f8e),
    (0x087F94, i_087f94),
    (0x087F9A, i_087f9a),
    (0x087F9E, i_087f9e),
    (0x087FA4, i_087fa4),
    (0x087FAA, i_087faa),
    (0x087FB0, i_087fb0),
    (0x087FB4, i_087fb4),
    (0x087FBA, i_087fba),
    (0x087FBC, i_087fbc),
];
