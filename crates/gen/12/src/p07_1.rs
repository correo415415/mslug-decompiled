// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $071B0A: lea.l $71b10(pc), a1
pub fn i_071b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71B10;
    16
}
/// $071B0E: move.l a1, (a6)
pub fn i_071b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071B10: jsr $72bce(pc)
pub fn i_071b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71B14);
    cpu.pc = 0x72BCE;
    16
}
/// $071B14: jsr $72bde(pc)
pub fn i_071b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71B18);
    cpu.pc = 0x72BDE;
    16
}
/// $071B18: subq.b #$1, $70(a6)
pub fn i_071b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B1C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $071B1C: bne.w $71b26
pub fn i_071b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71B26; return 10; }
    8
}
/// $071B20: lea.l $71b56(pc), a1
pub fn i_071b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71B56;
    16
}
/// $071B24: move.l a1, (a6)
pub fn i_071b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071B26: btst.b #$0, $70(a6)
pub fn i_071b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B2C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $071B2C: bne.w $71b36
pub fn i_071b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71B36; return 10; }
    8
}
/// $071B30: jsr $28d70.l
pub fn i_071b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71B36);
    cpu.pc = 0x28D70;
    16
}
/// $071B36: movea.l $c(a6), a1
pub fn i_071b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B3A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $071B3A: move.w $86(a6), d1
pub fn i_071b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B3E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071B3E: move.b (a1, d1.w), d0
pub fn i_071b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B42;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $071B42: clr.b (a1, d1.w)
pub fn i_071b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B46;
    cpu.ir = 0x4231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $071B46: cmpi.b #$5, d0
pub fn i_071b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B4A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $071B4A: bne.w $71b54
pub fn i_071b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71B54; return 10; }
    8
}
// ===== SetTaskHandler_071b4e  ($071B4E, 8 B, kernel/task_handlers.c)
/// $071B4E: lea.l $71b9a(pc), a1
pub fn i_071b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71B9A;
    16
}
/// $071B52: move.l a1, (a6)
pub fn i_071b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071B54: rts
pub fn i_071b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_HeadRespawn_071b56  ($071B56, 60 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071B56: lea.l $2badb6.l, a0
pub fn i_071b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B5C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BADB6;
    20
}
/// $071B5C: jsr $799de.l
pub fn i_071b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71B62);
    cpu.pc = 0x799DE;
    16
}
/// $071B62: move.w d0, $70(a6)
pub fn i_071b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B66;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071B66: lea.l $71b6c(pc), a1
pub fn i_071b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71B6C;
    16
}
/// $071B6A: move.l a1, (a6)
pub fn i_071b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071B6C: subq.w #$1, $70(a6)
pub fn i_071b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B70;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071B70: bgt.w $71b7a
pub fn i_071b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B74;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x71B7A; return 10; }
    8
}
/// $071B74: lea.l $71976(pc), a1
pub fn i_071b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71976;
    16
}
/// $071B78: move.l a1, (a6)
pub fn i_071b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071B7A: movea.l $c(a6), a1
pub fn i_071b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B7E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $071B7E: move.w $86(a6), d1
pub fn i_071b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B82;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071B82: move.b (a1, d1.w), d0
pub fn i_071b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B86;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $071B86: clr.b (a1, d1.w)
pub fn i_071b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B8A;
    cpu.ir = 0x4231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $071B8A: cmpi.b #$5, d0
pub fn i_071b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B8E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $071B8E: bne.w $71b98
pub fn i_071b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71B98; return 10; }
    8
}
// ===== SetTaskHandler_071b92  ($071B92, 8 B, kernel/task_handlers.c)
/// $071B92: lea.l $716f2(pc), a1
pub fn i_071b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071B96: move.l a1, (a6)
pub fn i_071b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071B98: rts
pub fn i_071b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71B9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_HeadDieB_071b9a  ($071B9A, 68 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071B9A: cmpi.b #$4, $8a(a6)
pub fn i_071b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BA0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $071BA0: bne.w $71bb4
pub fn i_071ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71BB4; return 10; }
    8
}
/// $071BA4: lea.l $7229c(pc), a1
pub fn i_071ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7229C;
    16
}
/// $071BA8: jsr $4ae.l
pub fn i_071ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71BAE);
    cpu.pc = 0x4AE;
    16
}
/// $071BAE: jsr $5dd02.l
pub fn i_071bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71BB4);
    cpu.pc = 0x5DD02;
    16
}
/// $071BB4: jsr $72bce(pc)
pub fn i_071bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71BB8);
    cpu.pc = 0x72BCE;
    16
}
/// $071BB8: jsr $72bde(pc)
pub fn i_071bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71BBC);
    cpu.pc = 0x72BDE;
    16
}
/// $071BBC: jsr $28d70.l
pub fn i_071bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71BC2);
    cpu.pc = 0x28D70;
    16
}
/// $071BC2: move.w #$d000, d0
pub fn i_071bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071BC6: jsr $28134.l
pub fn i_071bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71BCC);
    cpu.pc = 0x28134;
    16
}
/// $071BCC: andi.w #$ffe3, $38(a6)
pub fn i_071bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BD2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071BD2: ori.w #$14, $38(a6)
pub fn i_071bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BD8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071BD8: jsr $4a0d4.l
pub fn i_071bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71BDE);
    cpu.pc = 0x4A0D4;
    16
}
// ===== JsrAbsThunk_071bde  ($071BDE, 8 B, thunks/jsr_abs_thunks.c)
/// $071BDE: jsr $4a166.l
pub fn i_071bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71BE4);
    cpu.pc = 0x4A166;
    16
}
/// $071BE4: rts
pub fn i_071be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Cannon_071be6  ($071BE6, 150 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071BE6: lea.l $2d9cbe.l, a0
pub fn i_071be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9CBE;
    20
}
/// $071BEC: jsr $28cd4.l
pub fn i_071bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71BF2);
    cpu.pc = 0x28CD4;
    16
}
/// $071BF2: bra.w $71c02
pub fn i_071bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71BF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x71C02;
    10
}
/// $071C02: movea.l $c(a6), a0
pub fn i_071c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C06;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $071C06: tst.b $77(a0)
pub fn i_071c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C0A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $071C0A: beq.w $71c16
pub fn i_071c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x71C16; return 10; }
    8
}
/// $071C0E: move.w #$5d, d1
pub fn i_071c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C12;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071C12: bra.w $71c1a
pub fn i_071c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C16;
    cpu.ir = 0x6000;
    cpu.pc = 0x71C1A;
    10
}
/// $071C16: move.w #$1e6, d1
pub fn i_071c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C1A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071C1A: jsr $236e.l
pub fn i_071c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71C20);
    cpu.pc = 0x236E;
    16
}
/// $071C20: bset.b #$4, $6b(a6)
pub fn i_071c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C26;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071C26: move.w #$0, $84(a6)
pub fn i_071c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071C2C: lea.l $71c32(pc), a1
pub fn i_071c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71C32;
    16
}
/// $071C30: move.l a1, (a6)
pub fn i_071c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071C32: jsr $72bce(pc)
pub fn i_071c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71C36);
    cpu.pc = 0x72BCE;
    16
}
/// $071C36: move.w $9a(a6), d0
pub fn i_071c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C3A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071C3A: add.w d0, $22(a6)
pub fn i_071c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C3E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071C3E: move.w $9c(a6), d0
pub fn i_071c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C42;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071C42: add.w d0, $24(a6)
pub fn i_071c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C46;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071C46: jsr $28d70.l
pub fn i_071c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71C4C);
    cpu.pc = 0x28D70;
    16
}
/// $071C4C: bcc.w $71c56
pub fn i_071c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71C56; return 10; }
    8
}
/// $071C50: lea.l $71c84(pc), a1
pub fn i_071c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71C84;
    16
}
/// $071C54: move.l a1, (a6)
pub fn i_071c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071C56: jsr $5e45a.l
pub fn i_071c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71C5C);
    cpu.pc = 0x5E45A;
    16
}
/// $071C5C: bcc.w $71c66
pub fn i_071c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71C66; return 10; }
    8
}
/// $071C60: lea.l $71dba(pc), a1
pub fn i_071c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71DBA;
    16
}
/// $071C64: move.l a1, (a6)
pub fn i_071c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071C66: movea.l #$ffffffff, a0
pub fn i_071c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C6C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $071C6C: lea.l $2d90c4.l, a0
pub fn i_071c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90C4;
    20
}
/// $071C72: jsr $5dd56.l
pub fn i_071c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71C78);
    cpu.pc = 0x5DD56;
    16
}
/// $071C78: bcc.w $71c82
pub fn i_071c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C7C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71C82; return 10; }
    8
}
// ===== SetTaskHandler_071c7c  ($071C7C, 8 B, kernel/task_handlers.c)
/// $071C7C: lea.l $716f2(pc), a1
pub fn i_071c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071C80: move.l a1, (a6)
pub fn i_071c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071C82: rts
pub fn i_071c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_CannonShell_071c84  ($071C84, 302 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071C84: lea.l $2bae38.l, a0
pub fn i_071c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAE38;
    20
}
/// $071C8A: cmpi.b #$1, $20(a6)
pub fn i_071c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C90;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $071C90: bne.w $71c9a
pub fn i_071c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x71C9A; return 10; }
    8
}
/// $071C94: lea.l $2bae98.l, a0
pub fn i_071c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71C9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAE98;
    20
}
/// $071C9A: move.b #$0, $3a(a6)
pub fn i_071c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CA0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $071CA0: move.b $98(a6), d0
pub fn i_071ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CA4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $071CA4: andi.w #$f, d0
pub fn i_071ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CA8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $071CA8: move.w d0, d1
pub fn i_071ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CAA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071CAA: add.w d0, d0
pub fn i_071caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CAC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $071CAC: add.w d1, d0
pub fn i_071cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CAE;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $071CAE: add.w d0, d0
pub fn i_071cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CB0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $071CB0: move.w $2(a0, d0.w), $2e(a6)
pub fn i_071cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CB6;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $071CB6: move.w $4(a0, d0.w), $2a(a6)
pub fn i_071cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CBC;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $071CBC: move.w (a0, d0.w), d0
pub fn i_071cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CC0;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $071CC0: jsr $72782(pc)
pub fn i_071cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71CC4);
    cpu.pc = 0x72782;
    16
}
/// $071CC4: move.w d0, $28(a6)
pub fn i_071cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CC8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071CC8: jsr $5e5e0.l
pub fn i_071cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71CCE);
    cpu.pc = 0x5E5E0;
    16
}
/// $071CCE: bcc.w $71cd6
pub fn i_071cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71CD6; return 10; }
    8
}
/// $071CD2: neg.w $28(a6)
pub fn i_071cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CD6;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071CD6: move.w #$32, $66(a6)
pub fn i_071cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CDC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071CDC: move.l #$2d8a84, $48(a6)
pub fn i_071cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CE4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D8A84;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $071CE4: lea.l $2d9e36.l, a0
pub fn i_071ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9E36;
    20
}
/// $071CEA: jsr $28cd4.l
pub fn i_071cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71CF0);
    cpu.pc = 0x28CD4;
    16
}
/// $071CF0: lea.l $71cf6(pc), a1
pub fn i_071cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71CF6;
    16
}
/// $071CF4: move.l a1, (a6)
pub fn i_071cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071CF6: cmpi.w #$0, $2a(a6)
pub fn i_071cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71CFC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $071CFC: bgt.w $71d0c
pub fn i_071cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D00;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x71D0C; return 10; }
    8
}
/// $071D00: move.w #$d000, $38(a6)
pub fn i_071d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071D06: lea.l $71d0c(pc), a1
pub fn i_071d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71D0C;
    16
}
/// $071D0A: move.l a1, (a6)
pub fn i_071d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071D0C: jsr $27d50.l
pub fn i_071d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71D12);
    cpu.pc = 0x27D50;
    16
}
/// $071D12: bcc.w $71d1c
pub fn i_071d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71D1C; return 10; }
    8
}
/// $071D16: lea.l $71dba(pc), a1
pub fn i_071d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71DBA;
    16
}
/// $071D1A: move.l a1, (a6)
pub fn i_071d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071D1C: cmpi.w #$0, $2a(a6)
pub fn i_071d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D22;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $071D22: blt.w $71d30
pub fn i_071d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D26;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x71D30; return 10; }
    8
}
/// $071D26: movea.l $c(a6), a0
pub fn i_071d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D2A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $071D2A: move.w $38(a0), $38(a6)
pub fn i_071d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D30;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $071D30: move.w $28(a6), d0
pub fn i_071d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D34;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071D34: move.w $2a(a6), d1
pub fn i_071d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D38;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071D38: asr.w #$4, d0
pub fn i_071d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D3A;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $071D3A: asr.w #$4, d1
pub fn i_071d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D3C;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $071D3C: jsr $5e018.l
pub fn i_071d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71D42);
    cpu.pc = 0x5E018;
    16
}
/// $071D42: lsr.w #$3, d0
pub fn i_071d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D44;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $071D44: move.w d0, $34(a6)
pub fn i_071d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D48;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071D48: jsr $28d70.l
pub fn i_071d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71D4E);
    cpu.pc = 0x28D70;
    16
}
/// $071D4E: jsr $283d8.l
pub fn i_071d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71D54);
    cpu.pc = 0x283D8;
    16
}
/// $071D54: btst.b #$1, $13(a6)
pub fn i_071d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D5A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $071D5A: beq.w $71d64
pub fn i_071d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x71D64; return 10; }
    8
}
/// $071D5E: lea.l $72364(pc), a1
pub fn i_071d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72364;
    16
}
/// $071D62: move.l a1, (a6)
pub fn i_071d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071D64: jsr $2870a.l
pub fn i_071d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71D6A);
    cpu.pc = 0x2870A;
    16
}
/// $071D6A: lea.l $5e766.l, a0
pub fn i_071d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $071D70: jsr $5e770.l
pub fn i_071d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71D76);
    cpu.pc = 0x5E770;
    16
}
/// $071D76: bclr.b #$3, $13(a6)
pub fn i_071d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071D7C: jsr $28758.l
pub fn i_071d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71D82);
    cpu.pc = 0x28758;
    16
}
/// $071D82: bcc.w $71d8c
pub fn i_071d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71D8C; return 10; }
    8
}
/// $071D86: lea.l $71dba(pc), a1
pub fn i_071d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71DBA;
    16
}
/// $071D8A: move.l a1, (a6)
pub fn i_071d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071D8C: jsr $5e45a.l
pub fn i_071d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71D92);
    cpu.pc = 0x5E45A;
    16
}
/// $071D92: bcc.w $71d9c
pub fn i_071d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71D9C; return 10; }
    8
}
/// $071D96: lea.l $71dba(pc), a1
pub fn i_071d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71DBA;
    16
}
/// $071D9A: move.l a1, (a6)
pub fn i_071d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71D9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071D9C: movea.l #$ffffffff, a0
pub fn i_071d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DA2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $071DA2: lea.l $2d90c4.l, a0
pub fn i_071da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90C4;
    20
}
/// $071DA8: jsr $5dd56.l
pub fn i_071da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71DAE);
    cpu.pc = 0x5DD56;
    16
}
/// $071DAE: bcc.w $71db8
pub fn i_071dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71DB8; return 10; }
    8
}
// ===== SetTaskHandler_071db2  ($071DB2, 8 B, kernel/task_handlers.c)
/// $071DB2: lea.l $716f2(pc), a1
pub fn i_071db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071DB6: move.l a1, (a6)
pub fn i_071db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071DB8: rts
pub fn i_071db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_CannonHit_071dba  ($071DBA, 14 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071DBA: move.w #$1022, d0
pub fn i_071dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DBE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071DBE: jsr $2352.l
pub fn i_071dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71DC4);
    cpu.pc = 0x2352;
    16
}
/// $071DC4: bra.w $72364
pub fn i_071dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x72364;
    10
}
// ===== FinalBoss_CannonB_071dc8  ($071DC8, 128 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071DC8: movea.l $c(a6), a0
pub fn i_071dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DCC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $071DCC: tst.b $77(a0)
pub fn i_071dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DD0;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $071DD0: beq.w $71ddc
pub fn i_071dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x71DDC; return 10; }
    8
}
/// $071DD4: move.w #$5d, d1
pub fn i_071dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071DD8: bra.w $71de0
pub fn i_071dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x71DE0;
    10
}
/// $071DDC: move.w #$1e6, d1
pub fn i_071ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DE0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071DE0: jsr $236e.l
pub fn i_071de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71DE6);
    cpu.pc = 0x236E;
    16
}
/// $071DE6: move.w #$0, $84(a6)
pub fn i_071de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DEC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071DEC: lea.l $2d9bb8.l, a0
pub fn i_071dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9BB8;
    20
}
/// $071DF2: jsr $28cd4.l
pub fn i_071df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71DF8);
    cpu.pc = 0x28CD4;
    16
}
/// $071DF8: lea.l $71dfe(pc), a1
pub fn i_071df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71DFE;
    16
}
/// $071DFC: move.l a1, (a6)
pub fn i_071dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71DFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071DFE: jsr $72bce(pc)
pub fn i_071dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71E02);
    cpu.pc = 0x72BCE;
    16
}
/// $071E02: move.w $9a(a6), d0
pub fn i_071e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E06;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071E06: add.w d0, $22(a6)
pub fn i_071e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E0A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071E0A: move.w $9c(a6), d0
pub fn i_071e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E0E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $071E0E: add.w d0, $24(a6)
pub fn i_071e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E12;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071E12: jsr $28d70.l
pub fn i_071e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71E18);
    cpu.pc = 0x28D70;
    16
}
/// $071E18: bcc.w $71e22
pub fn i_071e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71E22; return 10; }
    8
}
/// $071E1C: lea.l $716f2(pc), a1
pub fn i_071e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071E20: move.l a1, (a6)
pub fn i_071e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071E22: jsr $5e45a.l
pub fn i_071e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71E28);
    cpu.pc = 0x5E45A;
    16
}
/// $071E28: bcc.w $71e32
pub fn i_071e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71E32; return 10; }
    8
}
/// $071E2C: lea.l $716f2(pc), a1
pub fn i_071e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071E30: move.l a1, (a6)
pub fn i_071e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071E32: movea.l #$ffffffff, a0
pub fn i_071e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E38;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $071E38: lea.l $2d90c4.l, a0
pub fn i_071e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E3E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90C4;
    20
}
/// $071E3E: jsr $5dd56.l
pub fn i_071e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71E44);
    cpu.pc = 0x5DD56;
    16
}
/// $071E44: bcc.w $71e4e
pub fn i_071e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71E4E; return 10; }
    8
}
// ===== SetTaskHandler_071e48  ($071E48, 8 B, kernel/task_handlers.c)
/// $071E48: lea.l $716f2(pc), a1
pub fn i_071e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071E4C: move.l a1, (a6)
pub fn i_071e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071E4E: rts
pub fn i_071e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Casing_071e50  ($071E50, 94 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071E50: move.w #$8, d1
pub fn i_071e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E54;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071E54: jsr $236e.l
pub fn i_071e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71E5A);
    cpu.pc = 0x236E;
    16
}
/// $071E5A: lea.l $2c4022.l, a0
pub fn i_071e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C4022;
    20
}
/// $071E60: jsr $28cd4.l
pub fn i_071e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71E66);
    cpu.pc = 0x28CD4;
    16
}
/// $071E66: eori.b #$1, $3a(a6)
pub fn i_071e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E6C;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071E6C: move.w #$4000, d0
pub fn i_071e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071E70: jsr $28134.l
pub fn i_071e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71E76);
    cpu.pc = 0x28134;
    16
}
/// $071E76: andi.w #$ffe3, $38(a6)
pub fn i_071e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E7C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071E7C: ori.w #$1c, $38(a6)
pub fn i_071e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E82;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071E82: lea.l $71e88(pc), a1
pub fn i_071e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71E88;
    16
}
/// $071E86: move.l a1, (a6)
pub fn i_071e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071E88: jsr $2783a.l
pub fn i_071e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71E8E);
    cpu.pc = 0x2783A;
    16
}
/// $071E8E: jsr $28d70.l
pub fn i_071e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71E94);
    cpu.pc = 0x28D70;
    16
}
/// $071E94: bcc.w $71e9e
pub fn i_071e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71E9E; return 10; }
    8
}
/// $071E98: lea.l $716f2(pc), a1
pub fn i_071e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071E9C: move.l a1, (a6)
pub fn i_071e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71E9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071E9E: movea.l #$ffffffff, a0
pub fn i_071e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EA4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $071EA4: jsr $5dd56.l
pub fn i_071ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71EAA);
    cpu.pc = 0x5DD56;
    16
}
/// $071EAA: bcc.w $71eb4
pub fn i_071eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71EB4; return 10; }
    8
}
// ===== SetTaskHandler_071eae  ($071EAE, 8 B, kernel/task_handlers.c)
/// $071EAE: lea.l $716f2(pc), a1
pub fn i_071eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071EB2: move.l a1, (a6)
pub fn i_071eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071EB4: rts
pub fn i_071eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Flame_071eb6  ($071EB6, 48 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071EB6: move.w #$5e, d1
pub fn i_071eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EBA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071EBA: jsr $236e.l
pub fn i_071eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71EC0);
    cpu.pc = 0x236E;
    16
}
/// $071EC0: lea.l $2d997a.l, a0
pub fn i_071ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D997A;
    20
}
/// $071EC6: jsr $28cd4.l
pub fn i_071ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71ECC);
    cpu.pc = 0x28CD4;
    16
}
/// $071ECC: move.w #$ffea, d0
pub fn i_071ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ED0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFEA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071ED0: jsr $72782(pc)
pub fn i_071ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ED4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71ED4);
    cpu.pc = 0x72782;
    16
}
/// $071ED4: move.w d0, $9a(a6)
pub fn i_071ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71ED8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071ED8: move.w #$28, $9c(a6)
pub fn i_071ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071EDE: clr.w $84(a6)
pub fn i_071ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EE2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $071EE2: bra.w $71df8
pub fn i_071ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x71DF8;
    10
}
// ===== FinalBoss_Beam_071ee6  ($071EE6, 132 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071EE6: move.w #$1, $84(a6)
pub fn i_071ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EEC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071EEC: move.w #$1cf, d1
pub fn i_071eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071EF0: jsr $236e.l
pub fn i_071ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71EF6);
    cpu.pc = 0x236E;
    16
}
/// $071EF6: lea.l $2d9a1c.l, a0
pub fn i_071ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71EFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9A1C;
    20
}
/// $071EFC: jsr $28cd4.l
pub fn i_071efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71F02);
    cpu.pc = 0x28CD4;
    16
}
/// $071F02: lea.l $71f08(pc), a1
pub fn i_071f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71F08;
    16
}
/// $071F06: move.l a1, (a6)
pub fn i_071f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071F08: jsr $72bce(pc)
pub fn i_071f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71F0C);
    cpu.pc = 0x72BCE;
    16
}
/// $071F0C: move.w #$ffb0, d0
pub fn i_071f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F10;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071F10: jsr $72782(pc)
pub fn i_071f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71F14);
    cpu.pc = 0x72782;
    16
}
/// $071F14: add.w d0, $22(a6)
pub fn i_071f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F18;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $071F18: addi.w #$38, $24(a6)
pub fn i_071f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F1E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $071F1E: jsr $28d70.l
pub fn i_071f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71F24);
    cpu.pc = 0x28D70;
    16
}
/// $071F24: bcc.w $71f2e
pub fn i_071f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F28;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71F2E; return 10; }
    8
}
/// $071F28: lea.l $71f72(pc), a1
pub fn i_071f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71F72;
    16
}
/// $071F2C: move.l a1, (a6)
pub fn i_071f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071F2E: jsr $283d8.l
pub fn i_071f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71F34);
    cpu.pc = 0x283D8;
    16
}
/// $071F34: btst.b #$1, $13(a6)
pub fn i_071f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F3A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $071F3A: beq.w $71f44
pub fn i_071f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x71F44; return 10; }
    8
}
/// $071F3E: lea.l $7233a(pc), a1
pub fn i_071f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $071F42: move.l a1, (a6)
pub fn i_071f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071F44: jsr $5e45a.l
pub fn i_071f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71F4A);
    cpu.pc = 0x5E45A;
    16
}
/// $071F4A: bcc.w $71f54
pub fn i_071f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71F54; return 10; }
    8
}
/// $071F4E: lea.l $7233a(pc), a1
pub fn i_071f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $071F52: move.l a1, (a6)
pub fn i_071f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071F54: movea.l #$ffffffff, a0
pub fn i_071f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F5A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $071F5A: lea.l $2d90bc.l, a0
pub fn i_071f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90BC;
    20
}
/// $071F60: jsr $5dd56.l
pub fn i_071f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71F66);
    cpu.pc = 0x5DD56;
    16
}
/// $071F66: bcc.w $71f70
pub fn i_071f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71F70; return 10; }
    8
}
// ===== SetTaskHandler_071f6a  ($071F6A, 8 B, kernel/task_handlers.c)
/// $071F6A: lea.l $716f2(pc), a1
pub fn i_071f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $071F6E: move.l a1, (a6)
pub fn i_071f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071F70: rts
pub fn i_071f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Debris_071f72  ($071F72, 138 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $071F72: bset.b #$4, $6b(a6)
pub fn i_071f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F78;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $071F78: lea.l $2baa28.l, a0
pub fn i_071f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAA28;
    20
}
/// $071F7E: jsr $799de.l
pub fn i_071f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71F84);
    cpu.pc = 0x799DE;
    16
}
/// $071F84: neg.w d0
pub fn i_071f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F86;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $071F86: jsr $72782(pc)
pub fn i_071f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x71F8A);
    cpu.pc = 0x72782;
    16
}
/// $071F8A: move.w d0, $28(a6)
pub fn i_071f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F8E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $071F8E: move.w #$d000, d0
pub fn i_071f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F92;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071F92: jsr $28134.l
pub fn i_071f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71F98);
    cpu.pc = 0x28134;
    16
}
/// $071F98: andi.w #$ffe3, $38(a6)
pub fn i_071f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71F9E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071F9E: ori.w #$1c, $38(a6)
pub fn i_071f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FA4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $071FA4: move.w #$1cf, d1
pub fn i_071fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FA8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $071FA8: jsr $236e.l
pub fn i_071fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71FAE);
    cpu.pc = 0x236E;
    16
}
/// $071FAE: lea.l $2d9a9c.l, a0
pub fn i_071fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FB4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9A9C;
    20
}
/// $071FB4: jsr $28cd4.l
pub fn i_071fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71FBA);
    cpu.pc = 0x28CD4;
    16
}
/// $071FBA: lea.l $71fc0(pc), a1
pub fn i_071fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71FC0;
    16
}
/// $071FBE: move.l a1, (a6)
pub fn i_071fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071FC0: jsr $27cee.l
pub fn i_071fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71FC6);
    cpu.pc = 0x27CEE;
    16
}
/// $071FC6: bcc.w $71fd0
pub fn i_071fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71FD0; return 10; }
    8
}
/// $071FCA: lea.l $7231e(pc), a1
pub fn i_071fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7231E;
    16
}
/// $071FCE: move.l a1, (a6)
pub fn i_071fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071FD0: jsr $28d70.l
pub fn i_071fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71FD6);
    cpu.pc = 0x28D70;
    16
}
/// $071FD6: jsr $283d8.l
pub fn i_071fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71FDC);
    cpu.pc = 0x283D8;
    16
}
/// $071FDC: btst.b #$1, $13(a6)
pub fn i_071fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FE2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $071FE2: beq.w $71fec
pub fn i_071fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FE6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x71FEC; return 10; }
    8
}
/// $071FE6: lea.l $7231e(pc), a1
pub fn i_071fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7231E;
    16
}
/// $071FEA: move.l a1, (a6)
pub fn i_071fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $071FEC: jsr $5e45a.l
pub fn i_071fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x71FF2);
    cpu.pc = 0x5E45A;
    16
}
/// $071FF2: bcc.w $71ffc
pub fn i_071ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x71FFC; return 10; }
    8
}
/// $071FF6: lea.l $7231e(pc), a1
pub fn i_071ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7231E;
    16
}
/// $071FFA: move.l a1, (a6)
pub fn i_071ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71FFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== FinalBoss_FlameTail_FreeIfOffWorld_071ffc  ($071FFC, 22 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $071FFC: movea.l #$ffffffff, a0
pub fn i_071ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72002;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $072002: lea.l $2d90bc.l, a0
pub fn i_072002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72008;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90BC;
    20
}
/// $072008: jsr $5dd56.l
pub fn i_072008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7200E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7200E);
    cpu.pc = 0x5DD56;
    16
}
/// $07200E: bcc.w $72018
pub fn i_07200e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72012;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72018; return 10; }
    8
}
// ===== SetTaskHandler_072012  ($072012, 8 B, kernel/task_handlers.c)
/// $072012: lea.l $716f2(pc), a1
pub fn i_072012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72016;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $072016: move.l a1, (a6)
pub fn i_072016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72018;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072018: rts
pub fn i_072018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7201A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_WreckA_07201a  ($07201A, 132 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07201A: move.w #$1, $84(a6)
pub fn i_07201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72020;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072020: move.w #$1cf, d1
pub fn i_072020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72024;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072024: jsr $236e.l
pub fn i_072024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7202A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7202A);
    cpu.pc = 0x236E;
    16
}
/// $07202A: lea.l $2d9aea.l, a0
pub fn i_07202a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72030;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9AEA;
    20
}
/// $072030: jsr $28cd4.l
pub fn i_072030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72036);
    cpu.pc = 0x28CD4;
    16
}
/// $072036: lea.l $7203c(pc), a1
pub fn i_072036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7203A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7203C;
    16
}
/// $07203A: move.l a1, (a6)
pub fn i_07203a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7203C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07203C: jsr $72bce(pc)
pub fn i_07203c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72040;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72040);
    cpu.pc = 0x72BCE;
    16
}
/// $072040: move.w #$ffc4, d0
pub fn i_072040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72044;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFC4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072044: jsr $72782(pc)
pub fn i_072044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72048;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72048);
    cpu.pc = 0x72782;
    16
}
/// $072048: add.w d0, $22(a6)
pub fn i_072048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7204C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07204C: addi.w #$28, $24(a6)
pub fn i_07204c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72052;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072052: jsr $28d70.l
pub fn i_072052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72058);
    cpu.pc = 0x28D70;
    16
}
/// $072058: bcc.w $72062
pub fn i_072058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7205C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72062; return 10; }
    8
}
/// $07205C: lea.l $720a6(pc), a1
pub fn i_07205c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72060;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x720A6;
    16
}
/// $072060: move.l a1, (a6)
pub fn i_072060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72062;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072062: jsr $283d8.l
pub fn i_072062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72068;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72068);
    cpu.pc = 0x283D8;
    16
}
/// $072068: btst.b #$1, $13(a6)
pub fn i_072068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7206E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07206E: beq.w $72078
pub fn i_07206e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72072;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72078; return 10; }
    8
}
/// $072072: lea.l $7233a(pc), a1
pub fn i_072072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72076;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $072076: move.l a1, (a6)
pub fn i_072076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72078;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072078: jsr $5e45a.l
pub fn i_072078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7207E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7207E);
    cpu.pc = 0x5E45A;
    16
}
/// $07207E: bcc.w $72088
pub fn i_07207e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72082;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72088; return 10; }
    8
}
/// $072082: lea.l $7233a(pc), a1
pub fn i_072082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72086;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $072086: move.l a1, (a6)
pub fn i_072086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72088;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072088: movea.l #$ffffffff, a0
pub fn i_072088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7208E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07208E: lea.l $2d90bc.l, a0
pub fn i_07208e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72094;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90BC;
    20
}
/// $072094: jsr $5dd56.l
pub fn i_072094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7209A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7209A);
    cpu.pc = 0x5DD56;
    16
}
/// $07209A: bcc.w $720a4
pub fn i_07209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7209E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x720A4; return 10; }
    8
}
// ===== SetTaskHandler_07209e  ($07209E, 8 B, kernel/task_handlers.c)
/// $07209E: lea.l $716f2(pc), a1
pub fn i_07209e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $0720A2: move.l a1, (a6)
pub fn i_0720a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0720A4: rts
pub fn i_0720a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_WreckA_Fall_0720a6  ($0720A6, 150 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0720A6: bset.b #$4, $6b(a6)
pub fn i_0720a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720AC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0720AC: lea.l $2baaaa.l, a0
pub fn i_0720ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAAAA;
    20
}
/// $0720B2: jsr $799de.l
pub fn i_0720b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x720B8);
    cpu.pc = 0x799DE;
    16
}
/// $0720B8: neg.w d0
pub fn i_0720b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720BA;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0720BA: jsr $72782(pc)
pub fn i_0720ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x720BE);
    cpu.pc = 0x72782;
    16
}
/// $0720BE: move.w d0, $28(a6)
pub fn i_0720be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720C2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0720C2: move.w #$d000, d0
pub fn i_0720c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0720C6: jsr $28134.l
pub fn i_0720c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x720CC);
    cpu.pc = 0x28134;
    16
}
/// $0720CC: andi.w #$ffe3, $38(a6)
pub fn i_0720cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720D2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0720D2: ori.w #$1c, $38(a6)
pub fn i_0720d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720D8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0720D8: lea.l $2d9b6a.l, a0
pub fn i_0720d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9B6A;
    20
}
/// $0720DE: jsr $28cd4.l
pub fn i_0720de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x720E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0720E4: lea.l $720ea(pc), a1
pub fn i_0720e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x720EA;
    16
}
/// $0720E8: move.l a1, (a6)
pub fn i_0720e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0720EA: jsr $27cee.l
pub fn i_0720ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x720F0);
    cpu.pc = 0x27CEE;
    16
}
/// $0720F0: bcc.w $720fa
pub fn i_0720f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x720FA; return 10; }
    8
}
/// $0720F4: lea.l $72326(pc), a1
pub fn i_0720f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72326;
    16
}
/// $0720F8: move.l a1, (a6)
pub fn i_0720f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0720FA: jsr $28d70.l
pub fn i_0720fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72100);
    cpu.pc = 0x28D70;
    16
}
/// $072100: jsr $283d8.l
pub fn i_072100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72106);
    cpu.pc = 0x283D8;
    16
}
/// $072106: btst.b #$1, $13(a6)
pub fn i_072106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7210C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07210C: beq.w $72116
pub fn i_07210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72110;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72116; return 10; }
    8
}
/// $072110: lea.l $72326(pc), a1
pub fn i_072110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72114;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72326;
    16
}
/// $072114: move.l a1, (a6)
pub fn i_072114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72116;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072116: jsr $5e45a.l
pub fn i_072116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7211C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7211C);
    cpu.pc = 0x5E45A;
    16
}
/// $07211C: bcc.w $72126
pub fn i_07211c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72120;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72126; return 10; }
    8
}
/// $072120: lea.l $72326(pc), a1
pub fn i_072120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72124;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72326;
    16
}
/// $072124: move.l a1, (a6)
pub fn i_072124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72126;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072126: movea.l #$ffffffff, a0
pub fn i_072126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7212C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07212C: lea.l $2d90bc.l, a0
pub fn i_07212c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72132;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90BC;
    20
}
/// $072132: jsr $5dd56.l
pub fn i_072132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72138;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72138);
    cpu.pc = 0x5DD56;
    16
}
/// $072138: bcc.w $72142
pub fn i_072138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7213C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72142; return 10; }
    8
}
// ===== SetTaskHandler_07213c  ($07213C, 8 B, kernel/task_handlers.c)
/// $07213C: lea.l $716f2(pc), a1
pub fn i_07213c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72140;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $072140: move.l a1, (a6)
pub fn i_072140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72142;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072142: rts
pub fn i_072142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72144;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_WreckB_072144  ($072144, 158 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072144: move.w #$39, d1
pub fn i_072144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72148;
    cpu.ir = 0x323C;
    let v1: u32 = 0x39;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072148: jsr $236e.l
pub fn i_072148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7214E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7214E);
    cpu.pc = 0x236E;
    16
}
/// $07214E: move.w #$d000, d0
pub fn i_07214e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72152;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072152: jsr $28134.l
pub fn i_072152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72158;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72158);
    cpu.pc = 0x28134;
    16
}
/// $072158: andi.w #$ffe3, $38(a6)
pub fn i_072158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7215E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07215E: ori.w #$14, $38(a6)
pub fn i_07215e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72164;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $072164: lea.l $2da526.l, a0
pub fn i_072164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7216A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DA526;
    20
}
/// $07216A: jsr $28cd4.l
pub fn i_07216a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72170;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72170);
    cpu.pc = 0x28CD4;
    16
}
/// $072170: lea.l $72176(pc), a1
pub fn i_072170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72174;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72176;
    16
}
/// $072174: move.l a1, (a6)
pub fn i_072174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72176;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072176: movea.l $c(a6), a0
pub fn i_072176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7217A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07217A: cmpi.b #$5, $8a(a0)
pub fn i_07217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72180;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $072180: beq.w $7233a
pub fn i_072180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72184;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7233A; return 10; }
    8
}
/// $072184: move.w $22(a0), d0
pub fn i_072184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72188;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072188: add.w $90(a6), d0
pub fn i_072188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7218C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07218C: move.w d0, $22(a6)
pub fn i_07218c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72190;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072190: move.w $24(a0), d0
pub fn i_072190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72194;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072194: add.w $92(a6), d0
pub fn i_072194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72198;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $072198: move.w d0, $24(a6)
pub fn i_072198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7219C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07219C: jsr $28d70.l
pub fn i_07219c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x721A2);
    cpu.pc = 0x28D70;
    16
}
/// $0721A2: bcc.w $721ac
pub fn i_0721a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x721AC; return 10; }
    8
}
/// $0721A6: lea.l $721ea(pc), a1
pub fn i_0721a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x721EA;
    16
}
/// $0721AA: move.l a1, (a6)
pub fn i_0721aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0721AC: jsr $283d8.l
pub fn i_0721ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x721B2);
    cpu.pc = 0x283D8;
    16
}
/// $0721B2: btst.b #$1, $13(a6)
pub fn i_0721b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721B8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0721B8: beq.w $721c2
pub fn i_0721b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x721C2; return 10; }
    8
}
/// $0721BC: lea.l $7233a(pc), a1
pub fn i_0721bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $0721C0: move.l a1, (a6)
pub fn i_0721c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0721C2: jsr $5e45a.l
pub fn i_0721c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x721C8);
    cpu.pc = 0x5E45A;
    16
}
/// $0721C8: bcc.w $721d2
pub fn i_0721c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x721D2; return 10; }
    8
}
/// $0721CC: lea.l $7233a(pc), a1
pub fn i_0721cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $0721D0: move.l a1, (a6)
pub fn i_0721d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0721D2: movea.l #$ffffffff, a0
pub fn i_0721d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721D8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0721D8: jsr $5dd56.l
pub fn i_0721d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x721DE);
    cpu.pc = 0x5DD56;
    16
}
/// $0721DE: bcc.w $721e8
pub fn i_0721de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x721E8; return 10; }
    8
}
// ===== SetTaskHandler_0721e2  ($0721E2, 8 B, kernel/task_handlers.c)
/// $0721E2: lea.l $716f2(pc), a1
pub fn i_0721e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $0721E6: move.l a1, (a6)
pub fn i_0721e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0721E8: rts
pub fn i_0721e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_WreckB_Fall_0721ea  ($0721EA, 134 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0721EA: lea.l $2bacb2.l, a0
pub fn i_0721ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BACB2;
    20
}
/// $0721F0: jsr $799de.l
pub fn i_0721f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x721F6);
    cpu.pc = 0x799DE;
    16
}
/// $0721F6: jsr $72782(pc)
pub fn i_0721f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x721FA);
    cpu.pc = 0x72782;
    16
}
/// $0721FA: move.w d0, $28(a6)
pub fn i_0721fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x721FE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0721FE: lea.l $2bad34.l, a0
pub fn i_0721fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72204;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAD34;
    20
}
/// $072204: jsr $799de.l
pub fn i_072204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7220A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7220A);
    cpu.pc = 0x799DE;
    16
}
/// $07220A: jsr $72782(pc)
pub fn i_07220a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7220E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7220E);
    cpu.pc = 0x72782;
    16
}
/// $07220E: move.w d0, $2c(a6)
pub fn i_07220e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72212;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072212: lea.l $2da58a.l, a0
pub fn i_072212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72218;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DA58A;
    20
}
/// $072218: jsr $28cd4.l
pub fn i_072218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7221E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7221E);
    cpu.pc = 0x28CD4;
    16
}
/// $07221E: lea.l $72224(pc), a1
pub fn i_07221e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72222;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72224;
    16
}
/// $072222: move.l a1, (a6)
pub fn i_072222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72224;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072224: jsr $27cee.l
pub fn i_072224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7222A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7222A);
    cpu.pc = 0x27CEE;
    16
}
/// $07222A: bcc.w $72234
pub fn i_07222a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7222E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72234; return 10; }
    8
}
/// $07222E: lea.l $7233a(pc), a1
pub fn i_07222e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72232;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $072232: move.l a1, (a6)
pub fn i_072232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72234;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072234: jsr $28d70.l
pub fn i_072234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7223A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7223A);
    cpu.pc = 0x28D70;
    16
}
/// $07223A: jsr $283d8.l
pub fn i_07223a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72240);
    cpu.pc = 0x283D8;
    16
}
/// $072240: btst.b #$1, $13(a6)
pub fn i_072240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72246;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072246: beq.w $72250
pub fn i_072246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7224A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72250; return 10; }
    8
}
/// $07224A: lea.l $7233a(pc), a1
pub fn i_07224a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7224E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $07224E: move.l a1, (a6)
pub fn i_07224e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72250;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072250: jsr $5e45a.l
pub fn i_072250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72256);
    cpu.pc = 0x5E45A;
    16
}
/// $072256: bcc.w $72260
pub fn i_072256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7225A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72260; return 10; }
    8
}
/// $07225A: lea.l $7233a(pc), a1
pub fn i_07225a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7225E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7233A;
    16
}
/// $07225E: move.l a1, (a6)
pub fn i_07225e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72260;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072260: movea.l #$ffffffff, a0
pub fn i_072260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72266;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $072266: jsr $5dd56.l
pub fn i_072266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7226C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7226C);
    cpu.pc = 0x5DD56;
    16
}
/// $07226C: bcc.w $72276
pub fn i_07226c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72270;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72276; return 10; }
    8
}
// ===== SetTaskHandler_072270  ($072270, 8 B, kernel/task_handlers.c)
/// $072270: lea.l $716f2(pc), a1
pub fn i_072270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $072274: move.l a1, (a6)
pub fn i_072274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72276;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072276: rts
pub fn i_072276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72278;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Spark_Left_072278  ($072278, 36 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072278: move.w #$ffbc, d0
pub fn i_072278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7227C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07227C: jsr $5dca4.l
pub fn i_07227c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72282;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72282);
    cpu.pc = 0x5DCA4;
    16
}
/// $072282: move.w d0, $28(a6)
pub fn i_072282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72286;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072286: move.w #$32a, $2a(a6)
pub fn i_072286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7228C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07228C: move.w #$ffca, $2e(a6)
pub fn i_07228c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72292;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFCA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072292: move.w #$0, $2c(a6)
pub fn i_072292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72298;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072298: bra.w $722bc
pub fn i_072298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7229C;
    cpu.ir = 0x6000;
    cpu.pc = 0x722BC;
    10
}
// ===== FinalBoss_Spark_07229c  ($07229C, 98 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07229C: move.w #$111, d0
pub fn i_07229c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x111;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0722A0: jsr $5dca4.l
pub fn i_0722a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x722A6);
    cpu.pc = 0x5DCA4;
    16
}
/// $0722A6: move.w d0, $28(a6)
pub fn i_0722a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0722AA: move.w #$87f, $2a(a6)
pub fn i_0722aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x87F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0722B0: move.w #$ff6f, $2e(a6)
pub fn i_0722b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF6F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0722B6: move.w #$0, $2c(a6)
pub fn i_0722b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0722BC: move.w #$38, d1
pub fn i_0722bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0722C0: jsr $236e.l
pub fn i_0722c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x722C6);
    cpu.pc = 0x236E;
    16
}
/// $0722C6: lea.l $29ca36.l, a0
pub fn i_0722c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CA36;
    20
}
/// $0722CC: jsr $28cd4.l
pub fn i_0722cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x722D2);
    cpu.pc = 0x28CD4;
    16
}
/// $0722D2: lea.l $722d8(pc), a1
pub fn i_0722d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x722D8;
    16
}
/// $0722D6: move.l a1, (a6)
pub fn i_0722d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0722D8: jsr $27bc8.l
pub fn i_0722d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x722DE);
    cpu.pc = 0x27BC8;
    16
}
/// $0722DE: bcc.w $722e8
pub fn i_0722de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x722E8; return 10; }
    8
}
/// $0722E2: lea.l $716f2(pc), a1
pub fn i_0722e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $0722E6: move.l a1, (a6)
pub fn i_0722e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0722E8: jsr $28d70.l
pub fn i_0722e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x722EE);
    cpu.pc = 0x28D70;
    16
}
/// $0722EE: movea.l #$ffffffff, a0
pub fn i_0722ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722F4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0722F4: jsr $5dd56.l
pub fn i_0722f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x722FA);
    cpu.pc = 0x5DD56;
    16
}
/// $0722FA: bcc.w $72304
pub fn i_0722fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x722FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72304; return 10; }
    8
}
// ===== SetTaskHandler_0722fe  ($0722FE, 8 B, kernel/task_handlers.c)
/// $0722FE: lea.l $716f2(pc), a1
pub fn i_0722fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72302;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $072302: move.l a1, (a6)
pub fn i_072302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72304;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072304: rts
pub fn i_072304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72306;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_RandDispatch4_072306  ($072306, 24 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072306: jsr $5e9b6.l
pub fn i_072306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7230C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7230C);
    cpu.pc = 0x5E9B6;
    16
}
/// $07230C: andi.w #$3, d0
pub fn i_07230c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72310;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072310: lsl.w #$2, d0
pub fn i_072310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72312;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $072312: lea.l $2d9322.l, a0
pub fn i_072312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72318;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D9322;
    20
}
/// $072318: movea.l (a0, d0.w), a1
pub fn i_072318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7231C;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $07231C: jmp (a1)
pub fn i_07231c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7231E;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
// ===== FinalBoss_ShiftX_Explode_07231e  ($07231E, 28 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07231E: move.w #$ffd8, d0
pub fn i_07231e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72322;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072322: bra.w $7232a
pub fn i_072322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72326;
    cpu.ir = 0x6000;
    cpu.pc = 0x7232A;
    10
}
/// $07232A: btst.b #$0, $3a(a6)
pub fn i_07232a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72330;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072330: beq.w $72336
pub fn i_072330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72334;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72336; return 10; }
    8
}
/// $072334: neg.w d0
pub fn i_072334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72336;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072336: add.w d0, $22(a6)
pub fn i_072336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7233A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== FinalBoss_FlushAndExplodeB_07233a  ($07233A, 42 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07233A: jsr $13600.l
pub fn i_07233a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72340);
    cpu.pc = 0x13600;
    16
}
/// $072340: move.w #$4000, d0
pub fn i_072340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72344;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072344: jsr $28134.l
pub fn i_072344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7234A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7234A);
    cpu.pc = 0x28134;
    16
}
/// $07234A: andi.w #$ffe3, $38(a6)
pub fn i_07234a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72350;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $072350: ori.w #$0, $38(a6)
pub fn i_072350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72356;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $072356: move.l #$ffffffff, $48(a6)
pub fn i_072356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7235E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07235E: jmp $77f6a.l
pub fn i_07235e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72364;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== FinalBoss_ExplodeC_072364  ($072364, 36 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072364: move.w #$4000, d0
pub fn i_072364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72368;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072368: jsr $28134.l
pub fn i_072368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7236E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7236E);
    cpu.pc = 0x28134;
    16
}
/// $07236E: andi.w #$ffe3, $38(a6)
pub fn i_07236e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72374;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $072374: ori.w #$0, $38(a6)
pub fn i_072374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7237A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07237A: move.l #$ffffffff, $48(a6)
pub fn i_07237a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72382;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $072382: jmp $77fd6.l
pub fn i_072382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72388;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
// ===== FinalBoss_SmokePuff_072388  ($072388, 66 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072388: move.w #$c000, d0
pub fn i_072388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7238C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07238C: jsr $28134.l
pub fn i_07238c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72392;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72392);
    cpu.pc = 0x28134;
    16
}
/// $072392: andi.w #$ffe3, $38(a6)
pub fn i_072392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72398;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $072398: ori.w #$14, $38(a6)
pub fn i_072398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7239E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07239E: move.w #$148, d1
pub fn i_07239e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723A2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x148;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0723A2: jsr $236e.l
pub fn i_0723a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x723A8);
    cpu.pc = 0x236E;
    16
}
/// $0723A8: lea.l $2c3f0a.l, a0
pub fn i_0723a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C3F0A;
    20
}
/// $0723AE: jsr $28cd4.l
pub fn i_0723ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x723B4);
    cpu.pc = 0x28CD4;
    16
}
/// $0723B4: lea.l $723ba(pc), a1
pub fn i_0723b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x723BA;
    16
}
/// $0723B8: move.l a1, (a6)
pub fn i_0723b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0723BA: jsr $2783a.l
pub fn i_0723ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x723C0);
    cpu.pc = 0x2783A;
    16
}
/// $0723C0: jsr $28d70.l
pub fn i_0723c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x723C6);
    cpu.pc = 0x28D70;
    16
}
/// $0723C6: bcc.w $723d0
pub fn i_0723c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x723D0; return 10; }
    8
}
// ===== SetTaskHandler_0723ca  ($0723CA, 8 B, kernel/task_handlers.c)
/// $0723CA: lea.l $716f2(pc), a1
pub fn i_0723ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $0723CE: move.l a1, (a6)
pub fn i_0723ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0723D0: rts
pub fn i_0723d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_LimbPart_Init_0723d2  ($0723D2, 92 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0723D2: clr.b $9a(a6)
pub fn i_0723d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723D6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0723D6: bra.w $723e0
pub fn i_0723d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x723E0;
    10
}
/// $0723E0: movea.l $c(a6), a0
pub fn i_0723e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723E4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0723E4: move.w $66(a0), d0
pub fn i_0723e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723E8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0723E8: lsr.w #$1, d0
pub fn i_0723e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723EA;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0723EA: move.w d0, $66(a6)
pub fn i_0723ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723EE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0723EE: lea.l $2da750.l, a0
pub fn i_0723ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DA750;
    20
}
/// $0723F4: jsr $28cd4.l
pub fn i_0723f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x723FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x723FA);
    cpu.pc = 0x28CD4;
    16
}
/// $0723FA: jsr $28d70.l
pub fn i_0723fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72400);
    cpu.pc = 0x28D70;
    16
}
/// $072400: lea.l $72406(pc), a1
pub fn i_072400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72404;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72406;
    16
}
/// $072404: move.l a1, (a6)
pub fn i_072404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72406;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072406: jsr $724dc(pc)
pub fn i_072406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7240A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7240A);
    cpu.pc = 0x724DC;
    16
}
/// $07240A: bgt.w $72414
pub fn i_07240a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7240E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x72414; return 10; }
    8
}
/// $07240E: lea.l $72436(pc), a1
pub fn i_07240e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72412;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72436;
    16
}
/// $072412: move.l a1, (a6)
pub fn i_072412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72414;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072414: btst.b #$0, $13(a6)
pub fn i_072414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7241A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07241A: beq.w $72424
pub fn i_07241a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7241E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72424; return 10; }
    8
}
/// $07241E: lea.l $724d4(pc), a1
pub fn i_07241e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72422;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x724D4;
    16
}
/// $072422: move.l a1, (a6)
pub fn i_072422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72424;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072424: jsr $5e45a.l
pub fn i_072424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7242A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7242A);
    cpu.pc = 0x5E45A;
    16
}
/// $07242A: bcc.w $72434
pub fn i_07242a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7242E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72434; return 10; }
    8
}
// ===== SetTaskHandler_07242e  ($07242E, 8 B, kernel/task_handlers.c)
/// $07242E: lea.l $724d4(pc), a1
pub fn i_07242e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72432;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x724D4;
    16
}
/// $072432: move.l a1, (a6)
pub fn i_072432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72434;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072434: rts
pub fn i_072434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72436;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_LimbPart_Damaged_072436  ($072436, 76 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072436: move.w $66(a6), d0
pub fn i_072436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7243A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07243A: lsr.w #$1, d0
pub fn i_07243a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7243C;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07243C: move.w d0, $66(a6)
pub fn i_07243c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72440;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072440: lea.l $72446(pc), a1
pub fn i_072440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72444;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72446;
    16
}
/// $072444: move.l a1, (a6)
pub fn i_072444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72446;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072446: jsr $724dc(pc)
pub fn i_072446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7244A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7244A);
    cpu.pc = 0x724DC;
    16
}
/// $07244A: bgt.w $72454
pub fn i_07244a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7244E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x72454; return 10; }
    8
}
/// $07244E: lea.l $7248a(pc), a1
pub fn i_07244e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72452;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7248A;
    16
}
/// $072452: move.l a1, (a6)
pub fn i_072452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72454;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072454: addq.b #$1, $70(a6)
pub fn i_072454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72458;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $072458: move.b $70(a6), d0
pub fn i_072458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7245C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07245C: andi.b #$7, d0
pub fn i_07245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72460;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $072460: bne.w $72468
pub fn i_072460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72464;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72468; return 10; }
    8
}
/// $072464: jsr $72554(pc)
pub fn i_072464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72468;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72468);
    cpu.pc = 0x72554;
    16
}
/// $072468: btst.b #$0, $13(a6)
pub fn i_072468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7246E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07246E: beq.w $72478
pub fn i_07246e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72472;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72478; return 10; }
    8
}
/// $072472: lea.l $724d4(pc), a1
pub fn i_072472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72476;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x724D4;
    16
}
/// $072476: move.l a1, (a6)
pub fn i_072476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72478;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072478: jsr $5e45a.l
pub fn i_072478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7247E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7247E);
    cpu.pc = 0x5E45A;
    16
}
/// $07247E: bcc.w $72488
pub fn i_07247e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72482;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72488; return 10; }
    8
}
// ===== SetTaskHandler_072482  ($072482, 8 B, kernel/task_handlers.c)
/// $072482: lea.l $724d4(pc), a1
pub fn i_072482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72486;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x724D4;
    16
}
/// $072486: move.l a1, (a6)
pub fn i_072486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72488;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072488: rts
pub fn i_072488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7248A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_LimbPart_Critical_07248a  ($07248A, 66 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07248A: jsr $724dc(pc)
pub fn i_07248a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7248E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7248E);
    cpu.pc = 0x724DC;
    16
}
/// $07248E: addq.b #$1, $70(a6)
pub fn i_07248e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72492;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $072492: move.b $70(a6), d0
pub fn i_072492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72496;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $072496: andi.b #$f, d0
pub fn i_072496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7249A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07249A: bne.w $724a6
pub fn i_07249a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7249E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x724A6; return 10; }
    8
}
/// $07249E: jsr $72540(pc)
pub fn i_07249e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x724A2);
    cpu.pc = 0x72540;
    16
}
/// $0724A2: bra.w $724b2
pub fn i_0724a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x724B2;
    10
}
/// $0724A6: andi.b #$3, d0
pub fn i_0724a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724AA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0724AA: bne.w $724b2
pub fn i_0724aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x724B2; return 10; }
    8
}
/// $0724AE: jsr $72554(pc)
pub fn i_0724ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x724B2);
    cpu.pc = 0x72554;
    16
}
/// $0724B2: btst.b #$0, $13(a6)
pub fn i_0724b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724B8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0724B8: beq.w $724c2
pub fn i_0724b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x724C2; return 10; }
    8
}
/// $0724BC: lea.l $724d4(pc), a1
pub fn i_0724bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x724D4;
    16
}
/// $0724C0: move.l a1, (a6)
pub fn i_0724c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0724C2: jsr $5e45a.l
pub fn i_0724c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x724C8);
    cpu.pc = 0x5E45A;
    16
}
/// $0724C8: bcc.w $724d2
pub fn i_0724c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x724D2; return 10; }
    8
}
// ===== SetTaskHandler_0724cc  ($0724CC, 8 B, kernel/task_handlers.c)
/// $0724CC: lea.l $724d4(pc), a1
pub fn i_0724cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x724D4;
    16
}
/// $0724D0: move.l a1, (a6)
pub fn i_0724d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0724D2: rts
pub fn i_0724d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0724d4  ($0724D4, 8 B, kernel/jmp_to_scheduler.c)
/// $0724D4: jmp $518.l
pub fn i_0724d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== FinalBoss_LimbPart_SyncWithParent_0724dc  ($0724DC, 100 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0724DC: movea.l $c(a6), a0
pub fn i_0724dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724E0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0724E0: move.b $32(a0), $32(a6)
pub fn i_0724e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724E6;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0724E6: move.b $33(a0), $33(a6)
pub fn i_0724e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724EC;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0724EC: move.w $22(a0), $22(a6)
pub fn i_0724ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724F2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0724F2: move.w $24(a0), d0
pub fn i_0724f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724F6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0724F6: add.w $98(a6), d0
pub fn i_0724f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724FA;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0724FA: move.w d0, $24(a6)
pub fn i_0724fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x724FE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0724FE: move.w $38(a0), d0
pub fn i_0724fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72502;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072502: andi.w #$ffe3, d0
pub fn i_072502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72506;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072506: ori.w #$0, d0
pub fn i_072506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7250A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07250A: move.w d0, $38(a6)
pub fn i_07250a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7250E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07250E: move.b $13(a0), $13(a6)
pub fn i_07250e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72514;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x13);
    bus.write8(ad4, v3 as u8);
    20
}
/// $072514: cmpi.b #$0, $9a(a6)
pub fn i_072514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7251A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07251A: beq.w $72536
pub fn i_07251a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7251E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72536; return 10; }
    8
}
/// $07251E: bclr.b #$0, $13(a6)
pub fn i_07251e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72524;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $072524: jsr $7b2.l
pub fn i_072524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7252A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7252A);
    cpu.pc = 0x7B2;
    16
}
/// $07252A: bcc.w $72536
pub fn i_07252a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7252E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72536; return 10; }
    8
}
/// $07252E: bset.b #$0, $13(a6)
pub fn i_07252e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72534;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $072534: rts
pub fn i_072534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72536;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $072536: move.w $66(a0), d0
pub fn i_072536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7253A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07253A: cmp.w $66(a6), d0
pub fn i_07253a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7253E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07253E: rts
pub fn i_07253e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72540;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnSmokeA_072540  ($072540, 138 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072540: lea.l $72606(pc), a1
pub fn i_072540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72544;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72606;
    16
}
/// $072544: jsr $4ae.l
pub fn i_072544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7254A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7254A);
    cpu.pc = 0x4AE;
    16
}
/// $07254A: jsr $5dd02.l
pub fn i_07254a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72550);
    cpu.pc = 0x5DD02;
    16
}
/// $072550: bra.w $72564
pub fn i_072550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72554;
    cpu.ir = 0x6000;
    cpu.pc = 0x72564;
    10
}
/// $072554: lea.l $725f6(pc), a1
pub fn i_072554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72558;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x725F6;
    16
}
/// $072558: jsr $4ae.l
pub fn i_072558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7255E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7255E);
    cpu.pc = 0x4AE;
    16
}
/// $07255E: jsr $5dd02.l
pub fn i_07255e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72564);
    cpu.pc = 0x5DD02;
    16
}
/// $072564: move.w #$1e, $70(a0)
pub fn i_072564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7256A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07256A: jsr $5e9b6.l
pub fn i_07256a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72570;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72570);
    cpu.pc = 0x5E9B6;
    16
}
/// $072570: andi.w #$3f, d0
pub fn i_072570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72574;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072574: subi.w #$20, d0
pub fn i_072574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72578;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072578: cmpi.b #$ff, $32(a6)
pub fn i_072578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7257E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07257E: beq.w $72590
pub fn i_07257e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72582;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72590; return 10; }
    8
}
/// $072582: move.b $32(a6), d1
pub fn i_072582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72586;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $072586: andi.w #$ff, d1
pub fn i_072586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7258A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07258A: addq.w #$1, d1
pub fn i_07258a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7258C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07258C: muls.w d1, d0
pub fn i_07258c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7258E;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $07258E: asr.w #$8, d0
pub fn i_07258e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72590;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $072590: add.w d0, $22(a0)
pub fn i_072590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72594;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072594: jsr $5e9b6.l
pub fn i_072594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7259A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7259A);
    cpu.pc = 0x5E9B6;
    16
}
/// $07259A: andi.w #$1f, d0
pub fn i_07259a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7259E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07259E: addq.w #$8, d0
pub fn i_07259e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725A0;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0725A0: cmpi.b #$ff, $33(a6)
pub fn i_0725a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725A6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0725A6: beq.w $725b8
pub fn i_0725a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x725B8; return 10; }
    8
}
/// $0725AA: move.b $33(a6), d1
pub fn i_0725aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725AE;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0725AE: andi.w #$ff, d1
pub fn i_0725ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725B2;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0725B2: addq.w #$1, d1
pub fn i_0725b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725B4;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0725B4: muls.w d1, d0
pub fn i_0725b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725B6;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0725B6: asr.w #$8, d0
pub fn i_0725b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725B8;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0725B8: add.w d0, $24(a0)
pub fn i_0725b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725BC;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0725BC: move.b $32(a6), $32(a0)
pub fn i_0725bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725C2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0725C2: move.b $33(a6), $33(a0)
pub fn i_0725c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725C8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0725C8: rts
pub fn i_0725c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_Smoke_Reposition_0725ca  ($0725CA, 44 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0725CA: jsr $5e9b6.l
pub fn i_0725ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x725D0);
    cpu.pc = 0x5E9B6;
    16
}
/// $0725D0: andi.w #$1f, d0
pub fn i_0725d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725D4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0725D4: addi.w #$14, d0
pub fn i_0725d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725D8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x14, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0725D8: move.w d0, $70(a6)
pub fn i_0725d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0725DC: jsr $5e9b6.l
pub fn i_0725dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x725E2);
    cpu.pc = 0x5E9B6;
    16
}
/// $0725E2: andi.w #$3f, d0
pub fn i_0725e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725E6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0725E6: subi.w #$20, d0
pub fn i_0725e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725EA;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0725EA: add.w d0, $22(a6)
pub fn i_0725ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725EE;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0725EE: subq.w #$1, $38(a6)
pub fn i_0725ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725F2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0725F2: bra.w $7262a
pub fn i_0725f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x7262A;
    10
}
// ===== FinalBoss_SmokeB_0725f6  ($0725F6, 16 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0725F6: lea.l $2da5c4.l, a0
pub fn i_0725f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x725FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DA5C4;
    20
}
/// $0725FC: jsr $28cd4.l
pub fn i_0725fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72602);
    cpu.pc = 0x28CD4;
    16
}
/// $072602: bra.w $72612
pub fn i_072602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72606;
    cpu.ir = 0x6000;
    cpu.pc = 0x72612;
    10
}
// ===== FinalBoss_SmokeA_072606  ($072606, 198 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072606: lea.l $2da680.l, a0
pub fn i_072606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7260C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DA680;
    20
}
/// $07260C: jsr $28cd4.l
pub fn i_07260c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72612);
    cpu.pc = 0x28CD4;
    16
}
/// $072612: move.w #$d, d1
pub fn i_072612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72616;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072616: jsr $236e.l
pub fn i_072616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7261C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7261C);
    cpu.pc = 0x236E;
    16
}
/// $07261C: addq.w #$1, $38(a6)
pub fn i_07261c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72620;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072620: cmpi.b #$ff, $33(a6)
pub fn i_072620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72626;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $072626: bne.w $7262a
pub fn i_072626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7262A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7262A; return 10; }
    8
}
/// $07262A: jsr $267e2.l
pub fn i_07262a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72630);
    cpu.pc = 0x267E2;
    16
}
/// $072630: jsr $5e9b6.l
pub fn i_072630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72636);
    cpu.pc = 0x5E9B6;
    16
}
/// $072636: andi.w #$ff, d0
pub fn i_072636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7263A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07263A: addi.w #$40, d0
pub fn i_07263a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7263E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07263E: move.w d0, $2a(a6)
pub fn i_07263e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72642;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072642: lsr.w #$6, d0
pub fn i_072642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72644;
    cpu.ir = 0xEC48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $072644: neg.w d0
pub fn i_072644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72646;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072646: move.w d0, $2e(a6)
pub fn i_072646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7264A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07264A: clr.w $8c(a6)
pub fn i_07264a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7264E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07264E: cmpi.w #$a, $70(a6)
pub fn i_07264e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72654;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Word, false);
    16
}
/// $072654: bgt.w $7265e
pub fn i_072654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72658;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7265E; return 10; }
    8
}
/// $072658: move.w #$a, $70(a6)
pub fn i_072658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7265E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07265E: lea.l $72664(pc), a1
pub fn i_07265e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72662;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72664;
    16
}
/// $072662: move.l a1, (a6)
pub fn i_072662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72664;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $072664: jsr $5e9b6.l
pub fn i_072664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7266A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7266A);
    cpu.pc = 0x5E9B6;
    16
}
/// $07266A: move.w d0, d1
pub fn i_07266a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7266C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07266C: andi.w #$3ff, d0
pub fn i_07266c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72670;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072670: cmpi.w #$0, d1
pub fn i_072670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72674;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $072674: bgt.w $7267a
pub fn i_072674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72678;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7267A; return 10; }
    8
}
/// $072678: neg.w d0
pub fn i_072678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7267A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07267A: add.w d0, $8c(a6)
pub fn i_07267a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7267E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07267E: move.b $8c(a6), d0
pub fn i_07267e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72682;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $072682: andi.w #$ff, d0
pub fn i_072682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72686;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072686: add.w d0, d0
pub fn i_072686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72688;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $072688: lea.l $2c072c.l, a0
pub fn i_072688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7268E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C072C;
    20
}
/// $07268E: move.w (a0, d0.w), d0
pub fn i_07268e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72692;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $072692: lsl.w #$1, d0
pub fn i_072692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72694;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $072694: move.w d0, $28(a6)
pub fn i_072694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72698;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072698: jsr $27cee.l
pub fn i_072698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7269E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7269E);
    cpu.pc = 0x27CEE;
    16
}
/// $07269E: jsr $28d70.l
pub fn i_07269e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x726A4);
    cpu.pc = 0x28D70;
    16
}
/// $0726A4: bcc.w $726ae
pub fn i_0726a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x726AE; return 10; }
    8
}
/// $0726A8: lea.l $716f2(pc), a1
pub fn i_0726a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $0726AC: move.l a1, (a6)
pub fn i_0726ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0726AE: subq.w #$1, $70(a6)
pub fn i_0726ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726B2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0726B2: bgt.w $726bc
pub fn i_0726b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726B6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x726BC; return 10; }
    8
}
/// $0726B6: lea.l $716f2(pc), a1
pub fn i_0726b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $0726BA: move.l a1, (a6)
pub fn i_0726ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0726BC: movea.l #$ffffffff, a0
pub fn i_0726bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726C2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0726C2: jsr $5dd56.l
pub fn i_0726c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x726C8);
    cpu.pc = 0x5DD56;
    16
}
/// $0726C8: bcc.w $726d2
pub fn i_0726c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x726D2; return 10; }
    8
}
// ===== SetTaskHandler_0726cc  ($0726CC, 8 B, kernel/task_handlers.c)
/// $0726CC: lea.l $716f2(pc), a1
pub fn i_0726cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x716F2;
    16
}
/// $0726D0: move.l a1, (a6)
pub fn i_0726d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0726D2: rts
pub fn i_0726d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_InScreenByVel_0726d4  ($0726D4, 34 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0726D4: cmpi.w #$0, $28(a6)
pub fn i_0726d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726DA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0726DA: bgt.w $726ec
pub fn i_0726da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x726EC; return 10; }
    8
}
/// $0726DE: cmpi.w #$50, $22(a6)
pub fn i_0726de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726E4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x50, Size::Word, false);
    16
}
/// $0726E4: blt.w $726fc
pub fn i_0726e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726E8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x726FC; return 10; }
    8
}
/// $0726E8: bra.w $726f6
pub fn i_0726e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x726F6;
    10
}
/// $0726EC: cmpi.w #$f0, $22(a6)
pub fn i_0726ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726F2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF0, Size::Word, false);
    16
}
/// $0726F2: bgt.w $726fc
pub fn i_0726f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726F6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x726FC; return 10; }
    8
}
// ===== ClearXN_0726f6  ($0726F6, 6 B, thunks/ccr_helpers.c)
/// $0726F6: andi.b #$ee, ccr
pub fn i_0726f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726FA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0726FA: rts
pub fn i_0726fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0726fc  ($0726FC, 6 B, thunks/ccr_helpers.c)
/// $0726FC: ori.b #$11, ccr
pub fn i_0726fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72700;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $072700: rts
pub fn i_072700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72702;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_TargetAheadByVel_072702  ($072702, 66 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072702: move.b $106f28.l, d0
pub fn i_072702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72708;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $072708: andi.b #$3, d0
pub fn i_072708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7270C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07270C: bne.w $7274a
pub fn i_07270c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72710;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7274A; return 10; }
    8
}
/// $072710: move.b $3a(a6), d0
pub fn i_072710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72714;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $072714: cmpi.w #$0, $28(a6)
pub fn i_072714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7271A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $07271A: beq.w $7274a
pub fn i_07271a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7271E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7274A; return 10; }
    8
}
/// $07271E: blt.w $72726
pub fn i_07271e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72722;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x72726; return 10; }
    8
}
/// $072722: eori.b #$1, d0
pub fn i_072722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72726;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $072726: lea.l $2d90d4.l, a0
pub fn i_072726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7272C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90D4;
    20
}
/// $07272C: btst.b #$0, d0
pub fn i_07272c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72730;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $072730: beq.w $7273a
pub fn i_072730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72734;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7273A; return 10; }
    8
}
/// $072734: lea.l $2d90dc.l, a0
pub fn i_072734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7273A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90DC;
    20
}
/// $07273A: jsr $5e086.l
pub fn i_07273a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72740);
    cpu.pc = 0x5E086;
    16
}
/// $072740: bcs.w $7274a
pub fn i_072740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72744;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7274A; return 10; }
    8
}
// ===== SetXN_072744  ($072744, 6 B, thunks/ccr_helpers.c)
/// $072744: ori.b #$11, ccr
pub fn i_072744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72748;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $072748: rts
pub fn i_072748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7274A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_07274a  ($07274A, 6 B, thunks/ccr_helpers.c)
/// $07274A: andi.b #$ee, ccr
pub fn i_07274a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7274E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07274E: rts
pub fn i_07274e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72750;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_EdgeFlagByVel_072750  ($072750, 38 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072750: cmpi.w #$0, $28(a6)
pub fn i_072750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72756;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $072756: beq.w $7277c
pub fn i_072756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7275A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7277C; return 10; }
    8
}
/// $07275A: bgt.w $7276c
pub fn i_07275a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7275E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7276C; return 10; }
    8
}
/// $07275E: btst.b #$1, $69(a6)
pub fn i_07275e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72764;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072764: bne.w $72776
pub fn i_072764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72768;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72776; return 10; }
    8
}
/// $072768: bra.w $7277c
pub fn i_072768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7276C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7277C;
    10
}
/// $07276C: btst.b #$0, $69(a6)
pub fn i_07276c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72772;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072772: beq.w $7277c
pub fn i_072772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72776;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7277C; return 10; }
    8
}
// ===== SetXN_072776  ($072776, 6 B, thunks/ccr_helpers.c)
/// $072776: ori.b #$11, ccr
pub fn i_072776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7277A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07277A: rts
pub fn i_07277a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7277C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_07277c  ($07277C, 6 B, thunks/ccr_helpers.c)
/// $07277C: andi.b #$ee, ccr
pub fn i_07277c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72780;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $072780: rts
pub fn i_072780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72782;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Facing_NegIfLeft_072782  ($072782, 12 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072782: btst.b #$0, $3a(a6)
pub fn i_072782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72788;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072788: beq.w $72794
pub fn i_072788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7278C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72794; return 10; }
    8
}
/// $07278C: neg.w d0
pub fn i_07278c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7278E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_07278e  ($07278E, 6 B, thunks/ccr_helpers.c)
/// $07278E: ori.b #$11, ccr
pub fn i_07278e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72792;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $072792: rts
pub fn i_072792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72794;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_072794  ($072794, 6 B, thunks/ccr_helpers.c)
/// $072794: andi.b #$ee, ccr
pub fn i_072794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72798;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $072798: rts
pub fn i_072798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7279A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SyncStateToParentAndTick_07279a  ($07279A, 42 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07279A: btst.b #$0, $77(a6)
pub fn i_07279a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727A0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0727A0: beq.w $727c2
pub fn i_0727a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x727C2; return 10; }
    8
}
/// $0727A4: move.l $c(a6), d0
pub fn i_0727a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727A8;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0727A8: cmpi.l #$ffffffff, d0
pub fn i_0727a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727AE;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $0727AE: beq.w $727c2
pub fn i_0727ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x727C2; return 10; }
    8
}
/// $0727B2: movea.l d0, a0
pub fn i_0727b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727B4;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0727B4: addq.b #$1, $21(a0)
pub fn i_0727b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727B8;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0727B8: addq.b #$1, $76(a0)
pub fn i_0727b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727BC;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0727BC: move.b $20(a0), $20(a6)
pub fn i_0727bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727C2;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0727C2: rts
pub fn i_0727c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SyncStateToParent_0727c4  ($0727C4, 38 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0727C4: btst.b #$0, $77(a6)
pub fn i_0727c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0727CA: beq.w $727e8
pub fn i_0727ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x727E8; return 10; }
    8
}
/// $0727CE: move.l $c(a6), d0
pub fn i_0727ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727D2;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0727D2: cmpi.l #$ffffffff, d0
pub fn i_0727d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727D8;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $0727D8: beq.w $727e8
pub fn i_0727d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x727E8; return 10; }
    8
}
/// $0727DC: movea.l d0, a0
pub fn i_0727dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727DE;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0727DE: addq.b #$1, $21(a0)
pub fn i_0727de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727E2;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0727E2: move.b $20(a0), $20(a6)
pub fn i_0727e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727E8;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0727E8: rts
pub fn i_0727e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_TickAttackTimers_0727ea  ($0727EA, 60 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0727EA: subq.w #$1, $72(a6)
pub fn i_0727ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727EE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0727EE: cmpi.w #$0, $72(a6)
pub fn i_0727ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727F4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0727F4: bgt.w $72806
pub fn i_0727f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727F8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x72806; return 10; }
    8
}
/// $0727F8: cmpi.b #$0, $88(a6)
pub fn i_0727f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x727FE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0727FE: bne.w $72802
pub fn i_0727fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72802;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72802; return 10; }
    8
}
/// $072802: jsr $7282c(pc)
pub fn i_072802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72806;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72806);
    cpu.pc = 0x7282C;
    16
}
/// $072806: subq.w #$1, $74(a6)
pub fn i_072806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7280A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07280A: cmpi.w #$0, $74(a6)
pub fn i_07280a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72810;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $072810: bgt.w $7282a
pub fn i_072810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72814;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7282A; return 10; }
    8
}
/// $072814: cmpi.b #$1, $20(a6)
pub fn i_072814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7281A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07281A: beq.w $72826
pub fn i_07281a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7281E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72826; return 10; }
    8
}
/// $07281E: jsr $72996(pc)
pub fn i_07281e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72822;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72822);
    cpu.pc = 0x72996;
    16
}
/// $072822: bra.w $7282a
pub fn i_072822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72826;
    cpu.ir = 0x6000;
    cpu.pc = 0x7282A;
    10
}
// ===== JsrPcThunk_072826  ($072826, 6 B, thunks/jsr_pc_thunks.c)
/// $072826: jsr $72a94(pc)
pub fn i_072826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7282A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7282A);
    cpu.pc = 0x72A94;
    16
}
/// $07282A: rts
pub fn i_07282a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7282C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_PickCannonPattern_07282c  ($07282C, 362 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07282C: cmpi.b #$0, $88(a6)
pub fn i_07282c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72832;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $072832: bne.w $728ca
pub fn i_072832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72836;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x728CA; return 10; }
    8
}
/// $072836: cmpi.b #$1, $20(a6)
pub fn i_072836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7283C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07283C: beq.w $7288a
pub fn i_07283c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72840;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7288A; return 10; }
    8
}
/// $072840: lea.l $2ba9a6.l, a0
pub fn i_072840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72846;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA9A6;
    20
}
/// $072846: jsr $799de.l
pub fn i_072846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7284C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7284C);
    cpu.pc = 0x799DE;
    16
}
/// $07284C: move.b d0, $89(a6)
pub fn i_07284c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72850;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072850: lea.l $2ba924.l, a0
pub fn i_072850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72856;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA924;
    20
}
/// $072856: jsr $799de.l
pub fn i_072856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7285C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7285C);
    cpu.pc = 0x799DE;
    16
}
/// $07285C: addi.w #$1e, d0
pub fn i_07285c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72860;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072860: move.w d0, $72(a6)
pub fn i_072860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72864;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072864: move.b #$1, $88(a6)
pub fn i_072864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7286A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07286A: move.b #$2, $84(a6)
pub fn i_07286a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72870;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072870: move.b #$2, $85(a6)
pub fn i_072870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72876;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072876: move.b #$2, $86(a6)
pub fn i_072876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7287C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07287C: move.b #$0, $87(a6)
pub fn i_07287c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72882;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072882: move.b #$2, $83(a6)
pub fn i_072882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72888;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072888: rts
pub fn i_072888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7288A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07288A: move.b #$7f, $89(a6)
pub fn i_07288a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72890;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072890: lea.l $2ba924.l, a0
pub fn i_072890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72896;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA924;
    20
}
/// $072896: jsr $799de.l
pub fn i_072896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7289C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7289C);
    cpu.pc = 0x799DE;
    16
}
/// $07289C: addi.w #$10, d0
pub fn i_07289c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728A0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0728A0: move.w d0, $72(a6)
pub fn i_0728a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728A4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0728A4: move.b #$1, $88(a6)
pub fn i_0728a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728AA: move.b #$2, $84(a6)
pub fn i_0728aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728B0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728B0: move.b #$2, $85(a6)
pub fn i_0728b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728B6: move.b #$0, $86(a6)
pub fn i_0728b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728BC: move.b #$0, $87(a6)
pub fn i_0728bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728C2: move.b #$2, $83(a6)
pub fn i_0728c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728C8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728C8: rts
pub fn i_0728c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0728CA: cmpi.b #$1, $88(a6)
pub fn i_0728ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728D0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0728D0: bne.w $72952
pub fn i_0728d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72952; return 10; }
    8
}
/// $0728D4: move.b #$3, $84(a6)
pub fn i_0728d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728DA: move.b #$3, $85(a6)
pub fn i_0728da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728E0: move.b #$0, $86(a6)
pub fn i_0728e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728E6: move.b #$0, $87(a6)
pub fn i_0728e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728EC: move.b #$0, $83(a6)
pub fn i_0728ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0728F2: btst.b #$0, $89(a6)
pub fn i_0728f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728F8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0728F8: beq.w $7291a
pub fn i_0728f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x728FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7291A; return 10; }
    8
}
/// $0728FC: move.b #$0, $84(a6)
pub fn i_0728fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72902;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072902: move.b #$0, $85(a6)
pub fn i_072902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72908;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072908: move.b #$3, $86(a6)
pub fn i_072908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7290E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07290E: move.b #$0, $87(a6)
pub fn i_07290e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72914;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072914: move.b #$0, $83(a6)
pub fn i_072914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7291A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07291A: lea.l $2ba924.l, a0
pub fn i_07291a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72920;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA924;
    20
}
/// $072920: jsr $799de.l
pub fn i_072920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72926;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72926);
    cpu.pc = 0x799DE;
    16
}
/// $072926: addi.w #$1e, d0
pub fn i_072926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7292A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07292A: cmpi.b #$1, $20(a6)
pub fn i_07292a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72930;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $072930: beq.w $72938
pub fn i_072930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72934;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72938; return 10; }
    8
}
/// $072934: addi.w #$fff2, d0
pub fn i_072934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72938;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFF2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072938: move.w d0, $72(a6)
pub fn i_072938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7293C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07293C: subq.b #$1, $89(a6)
pub fn i_07293c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72940;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x89);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $072940: cmpi.b #$0, $89(a6)
pub fn i_072940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72946;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $072946: bge.w $72950
pub fn i_072946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7294A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x72950; return 10; }
    8
}
/// $07294A: move.b #$2, $88(a6)
pub fn i_07294a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72950;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072950: rts
pub fn i_072950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72952;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $072952: move.b #$0, $88(a6)
pub fn i_072952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72958;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072958: lea.l $2ba8a2.l, a0
pub fn i_072958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7295E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA8A2;
    20
}
/// $07295E: jsr $799de.l
pub fn i_07295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72964);
    cpu.pc = 0x799DE;
    16
}
/// $072964: move.w d0, $72(a6)
pub fn i_072964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72968;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072968: jsr $5e9b6.l
pub fn i_072968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7296E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7296E);
    cpu.pc = 0x5E9B6;
    16
}
/// $07296E: andi.w #$3f, d0
pub fn i_07296e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72972;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072972: add.w d0, $72(a6)
pub fn i_072972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72976;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072976: move.b #$4, $84(a6)
pub fn i_072976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7297C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07297C: move.b #$4, $85(a6)
pub fn i_07297c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72982;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072982: move.b #$4, $86(a6)
pub fn i_072982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72988;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072988: move.b #$0, $87(a6)
pub fn i_072988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7298E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07298E: move.b #$0, $83(a6)
pub fn i_07298e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72994;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072994: rts
pub fn i_072994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72996;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_PickBeamPattern_072996  ($072996, 254 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072996: cmpi.b #$0, $8a(a6)
pub fn i_072996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7299C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07299C: bne.w $729ea
pub fn i_07299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x729EA; return 10; }
    8
}
/// $0729A0: lea.l $2bac30.l, a0
pub fn i_0729a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAC30;
    20
}
/// $0729A6: jsr $799de.l
pub fn i_0729a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x729AC);
    cpu.pc = 0x799DE;
    16
}
/// $0729AC: move.b d0, $8b(a6)
pub fn i_0729ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729B0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0729B0: lea.l $2babae.l, a0
pub fn i_0729b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BABAE;
    20
}
/// $0729B6: jsr $799de.l
pub fn i_0729b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x729BC);
    cpu.pc = 0x799DE;
    16
}
/// $0729BC: addi.w #$1e, d0
pub fn i_0729bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729C0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0729C0: move.w d0, $74(a6)
pub fn i_0729c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729C4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0729C4: move.b #$1, $8a(a6)
pub fn i_0729c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0729CA: move.b #$0, $84(a6)
pub fn i_0729ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0729D0: move.b #$0, $85(a6)
pub fn i_0729d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0729D6: move.b #$0, $86(a6)
pub fn i_0729d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729DC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0729DC: move.b #$2, $87(a6)
pub fn i_0729dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729E2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0729E2: move.b #$3, $83(a6)
pub fn i_0729e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0729E8: rts
pub fn i_0729e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0729EA: cmpi.b #$1, $8a(a6)
pub fn i_0729ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729F0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0729F0: bne.w $72a2e
pub fn i_0729f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72A2E; return 10; }
    8
}
/// $0729F4: move.b #$0, $84(a6)
pub fn i_0729f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x729FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0729FA: move.b #$0, $85(a6)
pub fn i_0729fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A00: move.b #$0, $86(a6)
pub fn i_072a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A06;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A06: move.b #$3, $87(a6)
pub fn i_072a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A0C: move.b #$0, $83(a6)
pub fn i_072a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A12: lea.l $2babae.l, a0
pub fn i_072a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BABAE;
    20
}
/// $072A18: jsr $799de.l
pub fn i_072a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72A1E);
    cpu.pc = 0x799DE;
    16
}
/// $072A1E: addi.w #$1e, d0
pub fn i_072a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A22;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072A22: move.w d0, $74(a6)
pub fn i_072a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A26;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072A26: move.b #$2, $8a(a6)
pub fn i_072a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A2C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A2C: rts
pub fn i_072a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $072A2E: lea.l $2babae.l, a0
pub fn i_072a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BABAE;
    20
}
/// $072A34: jsr $799de.l
pub fn i_072a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72A3A);
    cpu.pc = 0x799DE;
    16
}
/// $072A3A: addi.w #$1e, d0
pub fn i_072a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A3E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072A3E: move.w d0, $74(a6)
pub fn i_072a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072A42: subq.b #$1, $8b(a6)
pub fn i_072a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A46;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x8B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $072A46: cmpi.b #$0, $8b(a6)
pub fn i_072a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A4C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $072A4C: bge.w $72a6e
pub fn i_072a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A50;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x72A6E; return 10; }
    8
}
/// $072A50: lea.l $2bab2c.l, a0
pub fn i_072a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A56;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAB2C;
    20
}
/// $072A56: jsr $799de.l
pub fn i_072a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72A5C);
    cpu.pc = 0x799DE;
    16
}
/// $072A5C: move.w d0, $74(a6)
pub fn i_072a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A60;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072A60: jsr $5e9b6.l
pub fn i_072a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72A66);
    cpu.pc = 0x5E9B6;
    16
}
/// $072A66: andi.w #$3f, d0
pub fn i_072a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A6A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072A6A: add.w d0, $74(a6)
pub fn i_072a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A6E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072A6E: move.b #$0, $8a(a6)
pub fn i_072a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A74: move.b #$0, $84(a6)
pub fn i_072a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A7A: move.b #$0, $85(a6)
pub fn i_072a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A80;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A80: move.b #$0, $86(a6)
pub fn i_072a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A86;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A86: move.b #$4, $87(a6)
pub fn i_072a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A8C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A8C: move.b #$0, $83(a6)
pub fn i_072a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A92;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072A92: rts
pub fn i_072a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_PickBeamPatternB_072a94  ($072A94, 198 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072A94: cmpi.b #$0, $8a(a6)
pub fn i_072a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A9A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $072A9A: bne.w $72ada
pub fn i_072a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72ADA; return 10; }
    8
}
/// $072A9E: lea.l $2babae.l, a0
pub fn i_072a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BABAE;
    20
}
/// $072AA4: jsr $799de.l
pub fn i_072aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72AAA);
    cpu.pc = 0x799DE;
    16
}
/// $072AAA: asr.w #$1, d0
pub fn i_072aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AAC;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $072AAC: addi.w #$14, d0
pub fn i_072aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AB0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x14, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072AB0: move.w d0, $74(a6)
pub fn i_072ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AB4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072AB4: move.b #$1, $8a(a6)
pub fn i_072ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72ABA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072ABA: move.b #$0, $84(a6)
pub fn i_072aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AC0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072AC0: move.b #$0, $85(a6)
pub fn i_072ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072AC6: move.b #$0, $86(a6)
pub fn i_072ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72ACC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072ACC: move.b #$2, $87(a6)
pub fn i_072acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AD2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072AD2: move.b #$0, $83(a6)
pub fn i_072ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AD8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072AD8: rts
pub fn i_072ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72ADA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $072ADA: cmpi.b #$1, $8a(a6)
pub fn i_072ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AE0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $072AE0: bne.w $72b1e
pub fn i_072ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72B1E; return 10; }
    8
}
/// $072AE4: move.b #$0, $84(a6)
pub fn i_072ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072AEA: move.b #$0, $85(a6)
pub fn i_072aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AF0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072AF0: move.b #$0, $86(a6)
pub fn i_072af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072AF6: move.b #$3, $87(a6)
pub fn i_072af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72AFC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072AFC: move.b #$0, $83(a6)
pub fn i_072afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B02;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072B02: lea.l $2babae.l, a0
pub fn i_072b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BABAE;
    20
}
/// $072B08: jsr $799de.l
pub fn i_072b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72B0E);
    cpu.pc = 0x799DE;
    16
}
/// $072B0E: asr.w #$1, d0
pub fn i_072b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B10;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $072B10: addq.w #$4, d0
pub fn i_072b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B12;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072B12: move.w d0, $74(a6)
pub fn i_072b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B16;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072B16: move.b #$2, $8a(a6)
pub fn i_072b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B1C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072B1C: rts
pub fn i_072b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $072B1E: lea.l $2babae.l, a0
pub fn i_072b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BABAE;
    20
}
/// $072B24: jsr $799de.l
pub fn i_072b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72B2A);
    cpu.pc = 0x799DE;
    16
}
/// $072B2A: asr.w #$1, d0
pub fn i_072b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B2C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $072B2C: addi.w #$a, d0
pub fn i_072b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B30;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072B30: move.w d0, $74(a6)
pub fn i_072b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B34;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072B34: move.b #$1, $8a(a6)
pub fn i_072b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072B3A: move.b #$0, $84(a6)
pub fn i_072b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072B40: move.b #$0, $85(a6)
pub fn i_072b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072B46: move.b #$0, $86(a6)
pub fn i_072b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072B4C: move.b #$6, $87(a6)
pub fn i_072b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072B52: move.b #$0, $83(a6)
pub fn i_072b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B58;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072B58: rts
pub fn i_072b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_HeadTargetTimer_072b5a  ($072B5A, 48 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072B5A: subq.w #$1, $72(a6)
pub fn i_072b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B5E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072B5E: cmpi.w #$0, $72(a6)
pub fn i_072b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B64;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $072B64: bgt.w $72b90
pub fn i_072b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B68;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x72B90; return 10; }
    8
}
/// $072B68: lea.l $2d90cc.l, a0
pub fn i_072b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D90CC;
    20
}
/// $072B6E: jsr $5e086.l
pub fn i_072b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72B74);
    cpu.pc = 0x5E086;
    16
}
/// $072B74: bcs.w $72b90
pub fn i_072b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B78;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x72B90; return 10; }
    8
}
/// $072B78: jsr $5e9b6.l
pub fn i_072b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72B7E);
    cpu.pc = 0x5E9B6;
    16
}
/// $072B7E: andi.w #$3f, d0
pub fn i_072b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B82;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072B82: addi.w #$1e, d0
pub fn i_072b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B86;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072B86: move.w d0, $72(a6)
pub fn i_072b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B8A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_072b8a  ($072B8A, 6 B, thunks/ccr_helpers.c)
/// $072B8A: ori.b #$11, ccr
pub fn i_072b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B8E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $072B8E: rts
pub fn i_072b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_072b90  ($072B90, 6 B, thunks/ccr_helpers.c)
/// $072B90: andi.b #$ee, ccr
pub fn i_072b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B94;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $072B94: rts
pub fn i_072b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_PlayPartCry_072b96  ($072B96, 48 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072B96: move.w #$5b, d1
pub fn i_072b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B9A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072B9A: movea.l $c(a6), a0
pub fn i_072b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72B9E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $072B9E: cmpi.b #$0, $80(a0)
pub fn i_072b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BA4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $072BA4: beq.w $72bac
pub fn i_072ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72BAC; return 10; }
    8
}
/// $072BA8: move.w #$5c, d1
pub fn i_072ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BAC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072BAC: btst.b #$0, $77(a6)
pub fn i_072bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BB2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072BB2: bne.w $72bba
pub fn i_072bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72BBA; return 10; }
    8
}
/// $072BB6: move.w #$1e5, d1
pub fn i_072bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BBA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072BBA: jsr $236e.l
pub fn i_072bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72BC0);
    cpu.pc = 0x236E;
    16
}
/// $072BC0: move.w #$c, $1c(a6)
pub fn i_072bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_072bc6  ($072BC6, 8 B, thunks/jsr_abs_thunks.c)
/// $072BC6: jsr $138fe.l
pub fn i_072bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72BCC);
    cpu.pc = 0x138FE;
    16
}
/// $072BCC: rts
pub fn i_072bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_CopyParentPosPrio_072bce  ($072BCE, 16 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072BCE: jsr $5e506.l
pub fn i_072bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72BD4);
    cpu.pc = 0x5E506;
    16
}
/// $072BD4: move.w $84(a6), d0
pub fn i_072bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BD8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072BD8: add.w d0, $38(a6)
pub fn i_072bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BDC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072BDC: rts
pub fn i_072bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BDE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_OffsetByLimbTable_072bde  ($072BDE, 42 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072BDE: movea.l $80(a6), a1
pub fn i_072bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BE2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $072BE2: movea.l $c(a6), a0
pub fn i_072be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BE6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $072BE6: move.b $82(a0), d1
pub fn i_072be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BEA;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $072BEA: subq.b #$1, d1
pub fn i_072bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BEC;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $072BEC: andi.w #$f, d1
pub fn i_072bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BF0;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072BF0: lsl.w #$2, d1
pub fn i_072bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BF2;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $072BF2: move.w $2(a1, d1.w), d0
pub fn i_072bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BF6;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $072BF6: add.w d0, $24(a6)
pub fn i_072bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BFA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072BFA: move.w (a1, d1.w), d0
pub fn i_072bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72BFE;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $072BFE: jsr $72782(pc)
pub fn i_072bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72C02);
    cpu.pc = 0x72782;
    16
}
/// $072C02: add.w d0, $22(a6)
pub fn i_072c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C06;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072C06: rts
pub fn i_072c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_FetchPartSprite_072c08  ($072C08, 42 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072C08: movea.l $c(a6), a1
pub fn i_072c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C0C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $072C0C: move.w $86(a6), d1
pub fn i_072c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C10;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072C10: move.b (a1, d1.w), d0
pub fn i_072c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C14;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $072C14: clr.b (a1, d1.w)
pub fn i_072c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C18;
    cpu.ir = 0x4231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $072C18: andi.w #$f, d0
pub fn i_072c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C1C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072C1C: beq.w $72c38
pub fn i_072c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C20;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72C38; return 10; }
    8
}
/// $072C20: lsl.w #$2, d0
pub fn i_072c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C22;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $072C22: move.l (a0, d0.w), d0
pub fn i_072c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C26;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $072C26: cmpi.l #$ffffffff, d0
pub fn i_072c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C2C;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $072C2C: beq.w $72c38
pub fn i_072c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72C38; return 10; }
    8
}
/// $072C30: movea.l d0, a0
pub fn i_072c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C32;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
// ===== SetXN_072c32  ($072C32, 6 B, thunks/ccr_helpers.c)
/// $072C32: ori.b #$11, ccr
pub fn i_072c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C36;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $072C36: rts
pub fn i_072c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_NullSprite_072c38  ($072C38, 6 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072C38: movea.l #$ffffffff, a0
pub fn i_072c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C3E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
// ===== ClearXN_072c3e  ($072C3E, 6 B, thunks/ccr_helpers.c)
/// $072C3E: andi.b #$ee, ccr
pub fn i_072c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C42;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $072C42: rts
pub fn i_072c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_HitTestPlayers_072c44  ($072C44, 78 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072C44: clr.w d0
pub fn i_072c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C46;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072C46: jsr $5e42a.l
pub fn i_072c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72C4C);
    cpu.pc = 0x5E42A;
    16
}
/// $072C4C: bcc.w $72c6c
pub fn i_072c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72C6C; return 10; }
    8
}
/// $072C50: tst.b d0
pub fn i_072c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C52;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $072C52: bne.w $72c6c
pub fn i_072c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72C6C; return 10; }
    8
}
/// $072C56: movea.l $74(a0), a1
pub fn i_072c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C5A;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $072C5A: btst.b #$0, $3a(a6)
pub fn i_072c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C60;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072C60: beq.w $72c68
pub fn i_072c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C64;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72C68; return 10; }
    8
}
/// $072C64: movea.l $78(a0), a1
pub fn i_072c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C68;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $072C68: jsr $72c98(pc)
pub fn i_072c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72C6C);
    cpu.pc = 0x72C98;
    16
}
/// $072C6C: move.w #$1, d0
pub fn i_072c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072C70: jsr $5e42a.l
pub fn i_072c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72C76);
    cpu.pc = 0x5E42A;
    16
}
/// $072C76: bcc.w $72c96
pub fn i_072c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72C96; return 10; }
    8
}
/// $072C7A: tst.b d0
pub fn i_072c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C7C;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $072C7C: bne.w $72c96
pub fn i_072c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C80;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x72C96; return 10; }
    8
}
/// $072C80: movea.l $74(a0), a1
pub fn i_072c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C84;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $072C84: btst.b #$0, $3a(a6)
pub fn i_072c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C8A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072C8A: beq.w $72c92
pub fn i_072c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72C92; return 10; }
    8
}
/// $072C8E: movea.l $78(a0), a1
pub fn i_072c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C92;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
// ===== JsrPcThunk_072c92  ($072C92, 6 B, thunks/jsr_pc_thunks.c)
/// $072C92: jsr $72c98(pc)
pub fn i_072c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72C96);
    cpu.pc = 0x72C98;
    16
}
/// $072C96: rts
pub fn i_072c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CheckActiveBoxOverlap_072C98  ($072C98, 144 B, entity/check_active_box_overlap_072c98.s)
/// $072C98: move.l a1, d0
pub fn i_072c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C9A;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $072C9A: cmpi.l #$ffffffff, d0
pub fn i_072c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CA0;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $072CA0: beq.w $72d26
pub fn i_072ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72D26; return 10; }
    8
}
/// $072CA4: movea.l $94(a6), a2
pub fn i_072ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CA8;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $072CA8: move.w (a2), d0
pub fn i_072ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CAA;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $072CAA: move.w $2(a2), d1
pub fn i_072caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CAE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072CAE: btst.b #$0, $3a(a6)
pub fn i_072cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CB4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072CB4: beq.w $72cbe
pub fn i_072cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CB8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72CBE; return 10; }
    8
}
/// $072CB8: exg.l d0, d1
pub fn i_072cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CBA;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $072CBA: neg.w d0
pub fn i_072cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CBC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072CBC: neg.w d1
pub fn i_072cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CBE;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072CBE: move.w $22(a6), d2
pub fn i_072cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CC2;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072CC2: add.w d2, d0
pub fn i_072cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CC4;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $072CC4: add.w d2, d1
pub fn i_072cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CC6;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $072CC6: move.w $22(a1), d2
pub fn i_072cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CCA;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072CCA: cmp.w d0, d2
pub fn i_072cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CCC;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $072CCC: blt.w $72d26
pub fn i_072ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CD0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x72D26; return 10; }
    8
}
/// $072CD0: cmp.w d1, d2
pub fn i_072cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CD2;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $072CD2: bgt.w $72d26
pub fn i_072cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CD6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x72D26; return 10; }
    8
}
/// $072CD6: move.w $4(a2), d0
pub fn i_072cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CDA;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072CDA: move.w $6(a2), d1
pub fn i_072cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CDE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072CDE: move.w $24(a6), d2
pub fn i_072cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CE2;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072CE2: add.w d2, d0
pub fn i_072ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CE4;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $072CE4: add.w d2, d1
pub fn i_072ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CE6;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $072CE6: move.w $24(a1), d2
pub fn i_072ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CEA;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072CEA: cmp.w d0, d2
pub fn i_072cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CEC;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $072CEC: blt.w $72d26
pub fn i_072cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CF0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x72D26; return 10; }
    8
}
/// $072CF0: cmp.w d1, d2
pub fn i_072cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CF2;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $072CF2: bgt.w $72d26
pub fn i_072cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CF6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x72D26; return 10; }
    8
}
/// $072CF6: move.w (a2), d0
pub fn i_072cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CF8;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $072CF8: add.w $22(a6), d0
pub fn i_072cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72CFC;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $072CFC: sub.w $22(a1), d0
pub fn i_072cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D00;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $072D00: btst.b #$0, $3a(a6)
pub fn i_072d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D06;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $072D06: beq.w $72d18
pub fn i_072d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D0A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x72D18; return 10; }
    8
}
/// $072D0A: move.w (a2), d1
pub fn i_072d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D0C;
    cpu.ir = 0x3212;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $072D0C: neg.w d1
pub fn i_072d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D0E;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072D0E: add.w $22(a6), d1
pub fn i_072d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D12;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $072D12: move.w $22(a1), d0
pub fn i_072d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D16;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $072D16: sub.w d1, d0
pub fn i_072d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D18;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $072D18: neg.w d0
pub fn i_072d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D1A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072D1A: add.w $24(a6), d0
pub fn i_072d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D1E;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $072D1E: sub.w $24(a1), d0
pub fn i_072d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D22;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $072D22: move.w d0, $8e(a1)
pub fn i_072d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D26;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072D26: rts
pub fn i_072d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnBeamAndCasing_072d28  ($072D28, 72 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072D28: lea.l $71ee6(pc), a1
pub fn i_072d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71EE6;
    16
}
/// $072D2C: jsr $4ae.l
pub fn i_072d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D32);
    cpu.pc = 0x4AE;
    16
}
/// $072D32: jsr $5dd02.l
pub fn i_072d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D38);
    cpu.pc = 0x5DD02;
    16
}
/// $072D38: move.w #$ffd6, d0
pub fn i_072d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D3C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFD6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072D3C: jsr $72782(pc)
pub fn i_072d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72D40);
    cpu.pc = 0x72782;
    16
}
/// $072D40: add.w d0, $22(a0)
pub fn i_072d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D44;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072D44: addi.w #$40, $24(a0)
pub fn i_072d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D4A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072D4A: lea.l $71e50(pc), a1
pub fn i_072d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71E50;
    16
}
/// $072D4E: jsr $4ae.l
pub fn i_072d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D54);
    cpu.pc = 0x4AE;
    16
}
/// $072D54: jsr $5dd02.l
pub fn i_072d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D5A);
    cpu.pc = 0x5DD02;
    16
}
/// $072D5A: move.w #$fff0, d0
pub fn i_072d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D5E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072D5E: jsr $72782(pc)
pub fn i_072d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72D62);
    cpu.pc = 0x72782;
    16
}
/// $072D62: add.w d0, $22(a0)
pub fn i_072d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D66;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072D66: addi.w #$16, $24(a0)
pub fn i_072d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D6C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x16, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072D6C: move.w #$1064, d0
pub fn i_072d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_072d70  ($072D70, 8 B, thunks/jsr_abs_thunks.c)
/// $072D70: jsr $2352.l
pub fn i_072d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D76);
    cpu.pc = 0x2352;
    16
}
/// $072D76: rts
pub fn i_072d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnWreckAndFlame_072d78  ($072D78, 36 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072D78: lea.l $7201a(pc), a1
pub fn i_072d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7201A;
    16
}
/// $072D7C: jsr $4ae.l
pub fn i_072d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D82);
    cpu.pc = 0x4AE;
    16
}
/// $072D82: jsr $5dd02.l
pub fn i_072d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D88);
    cpu.pc = 0x5DD02;
    16
}
/// $072D88: lea.l $71eb6(pc), a1
pub fn i_072d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71EB6;
    16
}
/// $072D8C: jsr $4ae.l
pub fn i_072d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D92);
    cpu.pc = 0x4AE;
    16
}
/// $072D92: jsr $5dd02.l
pub fn i_072d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72D98);
    cpu.pc = 0x5DD02;
    16
}
/// $072D98: move.w #$1066, d0
pub fn i_072d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72D9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1066;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_072d9c  ($072D9C, 8 B, thunks/jsr_abs_thunks.c)
/// $072D9C: jsr $2352.l
pub fn i_072d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72DA2);
    cpu.pc = 0x2352;
    16
}
/// $072DA2: rts
pub fn i_072da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnDebris2D91F8_072da4  ($072DA4, 12 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072DA4: lea.l $2d91f8.l, a1
pub fn i_072da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DAA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D91F8;
    20
}
/// $072DAA: jsr $77c7e.l
pub fn i_072daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72DB0);
    cpu.pc = 0x77C7E;
    16
}
// ===== JsrAbsThunk_072db0  ($072DB0, 8 B, thunks/jsr_abs_thunks.c)
/// $072DB0: jsr $6e412.l
pub fn i_072db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72DB6);
    cpu.pc = 0x6E412;
    16
}
/// $072DB6: rts
pub fn i_072db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnSmokeColumn_072db8  ($072DB8, 48 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072DB8: jsr $6e412.l
pub fn i_072db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72DBE);
    cpu.pc = 0x6E412;
    16
}
/// $072DBE: move.w #$10, d0
pub fn i_072dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DC2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072DC2: movem.w d0, -(a7)
pub fn i_072dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DC6;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $072DC6: lea.l $62536.l, a1
pub fn i_072dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DCC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x62536;
    20
}
/// $072DCC: jsr $4ae.l
pub fn i_072dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72DD2);
    cpu.pc = 0x4AE;
    16
}
/// $072DD2: jsr $5dd02.l
pub fn i_072dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72DD8);
    cpu.pc = 0x5DD02;
    16
}
/// $072DD8: addi.w #$18, $24(a0)
pub fn i_072dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DDE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072DDE: movem.w (a7)+, d0
pub fn i_072dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DE2;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $072DE2: subq.w #$1, d0
pub fn i_072de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DE4;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072DE4: bcc.b $72dc2
pub fn i_072de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DE6;
    cpu.ir = 0x64DC;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x72DC2; return 10; }
    8
}
/// $072DE6: rts
pub fn i_072de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00072DE8  ($072DE8, 2 B, thunks/trivial_rts.c)
/// $072DE8: rts
pub fn i_072de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_AttackThenDebris_072dea  ($072DEA, 34 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072DEA: move.l #$2d8f1c, $4c(a6)
pub fn i_072dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DF2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D8F1C;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $072DF2: jsr $283ca.l
pub fn i_072df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72DF8);
    cpu.pc = 0x283CA;
    16
}
/// $072DF8: jsr $283d8.l
pub fn i_072df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72DFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72DFE);
    cpu.pc = 0x283D8;
    16
}
/// $072DFE: move.l #$ffffffff, $4c(a6)
pub fn i_072dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E06;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $072E06: lea.l $2d920a.l, a1
pub fn i_072e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E0C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D920A;
    20
}
// ===== JsrAbsThunk_072e0c  ($072E0C, 8 B, thunks/jsr_abs_thunks.c)
/// $072E0C: jsr $43fac.l
pub fn i_072e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E12);
    cpu.pc = 0x43FAC;
    16
}
/// $072E12: rts
pub fn i_072e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnCannonPair_072e14  ($072E14, 100 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072E14: lea.l $71dc8(pc), a1
pub fn i_072e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71DC8;
    16
}
/// $072E18: jsr $4ae.l
pub fn i_072e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E1E);
    cpu.pc = 0x4AE;
    16
}
/// $072E1E: jsr $5dd02.l
pub fn i_072e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E24);
    cpu.pc = 0x5DD02;
    16
}
/// $072E24: move.w #$29, d0
pub fn i_072e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E28;
    cpu.ir = 0x303C;
    let v1: u32 = 0x29;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072E28: jsr $72782(pc)
pub fn i_072e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72E2C);
    cpu.pc = 0x72782;
    16
}
/// $072E2C: move.w d0, $9a(a0)
pub fn i_072e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E30;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072E30: addi.w #$1e, $9c(a0)
pub fn i_072e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E36;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x9C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072E36: jsr $5e9b6.l
pub fn i_072e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E3C);
    cpu.pc = 0x5E9B6;
    16
}
/// $072E3C: movem.w d0, -(a7)
pub fn i_072e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E40;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $072E40: lea.l $71be6(pc), a1
pub fn i_072e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71BE6;
    16
}
/// $072E44: jsr $4ae.l
pub fn i_072e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E4A);
    cpu.pc = 0x4AE;
    16
}
/// $072E4A: jsr $5dd02.l
pub fn i_072e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E50);
    cpu.pc = 0x5DD02;
    16
}
/// $072E50: movem.w (a7)+, d0
pub fn i_072e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E54;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $072E54: andi.w #$7, d0
pub fn i_072e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E58;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072E58: move.b d0, $98(a0)
pub fn i_072e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E5C;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072E5C: move.b $20(a6), $20(a0)
pub fn i_072e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E62;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $072E62: move.w #$25, d0
pub fn i_072e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E66;
    cpu.ir = 0x303C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072E66: jsr $72782(pc)
pub fn i_072e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72E6A);
    cpu.pc = 0x72782;
    16
}
/// $072E6A: move.w d0, $9a(a0)
pub fn i_072e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E6E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072E6E: move.w #$44, $9c(a0)
pub fn i_072e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E74;
    cpu.ir = 0x317C;
    let v1: u32 = 0x44;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072E74: move.w #$10c6, d0
pub fn i_072e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E78;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_072e78  ($072E78, 8 B, thunks/jsr_abs_thunks.c)
/// $072E78: jsr $2352.l
pub fn i_072e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E7E);
    cpu.pc = 0x2352;
    16
}
/// $072E7E: rts
pub fn i_072e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnCannonPairB_072e80  ($072E80, 96 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072E80: lea.l $71dc8(pc), a1
pub fn i_072e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71DC8;
    16
}
/// $072E84: jsr $4ae.l
pub fn i_072e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E8A);
    cpu.pc = 0x4AE;
    16
}
/// $072E8A: jsr $5dd02.l
pub fn i_072e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72E90);
    cpu.pc = 0x5DD02;
    16
}
/// $072E90: move.w #$2c, d0
pub fn i_072e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E94;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072E94: jsr $72782(pc)
pub fn i_072e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72E98);
    cpu.pc = 0x72782;
    16
}
/// $072E98: move.w d0, $9a(a0)
pub fn i_072e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72E9C;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072E9C: addi.w #$1e, $9c(a0)
pub fn i_072e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EA2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x9C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072EA2: jsr $5e9b6.l
pub fn i_072ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72EA8);
    cpu.pc = 0x5E9B6;
    16
}
/// $072EA8: movem.w d0, -(a7)
pub fn i_072ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EAC;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $072EAC: lea.l $71bf6(pc), a1
pub fn i_072eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x71BF6;
    16
}
/// $072EB0: jsr $4ae.l
pub fn i_072eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72EB6);
    cpu.pc = 0x4AE;
    16
}
/// $072EB6: jsr $5dd02.l
pub fn i_072eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72EBC);
    cpu.pc = 0x5DD02;
    16
}
/// $072EBC: movem.w (a7)+, d0
pub fn i_072ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EC0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $072EC0: andi.w #$7, d0
pub fn i_072ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EC4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $072EC4: addq.w #$8, d0
pub fn i_072ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EC6;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $072EC6: move.b d0, $98(a6)
pub fn i_072ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72ECA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $072ECA: move.w #$2c, d0
pub fn i_072eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72ECE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072ECE: jsr $72782(pc)
pub fn i_072ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72ED2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72ED2);
    cpu.pc = 0x72782;
    16
}
/// $072ED2: move.w d0, $9a(a0)
pub fn i_072ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72ED6;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072ED6: move.w #$44, $9c(a0)
pub fn i_072ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EDC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x44;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072EDC: move.w #$10c6, d0
pub fn i_072edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_072ee0  ($072EE0, 8 B, thunks/jsr_abs_thunks.c)
/// $072EE0: jsr $2352.l
pub fn i_072ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72EE6);
    cpu.pc = 0x2352;
    16
}
/// $072EE6: rts
pub fn i_072ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnSmokeAndWreckB_072ee8  ($072EE8, 80 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072EE8: lea.l $66b6c.l, a1
pub fn i_072ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EEE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x66B6C;
    20
}
/// $072EEE: jsr $4ae.l
pub fn i_072eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72EF4);
    cpu.pc = 0x4AE;
    16
}
/// $072EF4: jsr $5dd02.l
pub fn i_072ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72EFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72EFA);
    cpu.pc = 0x5DD02;
    16
}
/// $072EFA: eori.b #$1, $3a(a0)
pub fn i_072efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F00;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $072F00: move.w #$18, d0
pub fn i_072f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072F04: jsr $72782(pc)
pub fn i_072f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72F08);
    cpu.pc = 0x72782;
    16
}
/// $072F08: add.w d0, $22(a0)
pub fn i_072f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F0C;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072F0C: addi.w #$10, $24(a0)
pub fn i_072f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F12;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072F12: lea.l $72144(pc), a1
pub fn i_072f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72144;
    16
}
/// $072F16: jsr $4ae.l
pub fn i_072f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F1C);
    cpu.pc = 0x4AE;
    16
}
/// $072F1C: jsr $5dd02.l
pub fn i_072f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F22);
    cpu.pc = 0x5DD02;
    16
}
/// $072F22: move.w #$20, d0
pub fn i_072f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F26;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072F26: jsr $72782(pc)
pub fn i_072f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72F2A);
    cpu.pc = 0x72782;
    16
}
/// $072F2A: move.w d0, $90(a0)
pub fn i_072f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F2E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072F2E: addi.w #$10, $92(a0)
pub fn i_072f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F34;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x92);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072F34: move.w #$10a4, d0
pub fn i_072f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F38;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_072f38  ($072F38, 8 B, thunks/jsr_abs_thunks.c)
/// $072F38: jsr $2352.l
pub fn i_072f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F3E);
    cpu.pc = 0x2352;
    16
}
/// $072F3E: rts
pub fn i_072f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnPuffA_072f40  ($072F40, 36 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072F40: lea.l $72388(pc), a1
pub fn i_072f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72388;
    16
}
/// $072F44: jsr $4ae.l
pub fn i_072f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F4A);
    cpu.pc = 0x4AE;
    16
}
/// $072F4A: jsr $5dd02.l
pub fn i_072f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F50);
    cpu.pc = 0x5DD02;
    16
}
/// $072F50: move.w #$fff0, d0
pub fn i_072f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F54;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072F54: jsr $72782(pc)
pub fn i_072f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72F58);
    cpu.pc = 0x72782;
    16
}
/// $072F58: add.w d0, $22(a0)
pub fn i_072f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F5C;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072F5C: addi.w #$e, $24(a0)
pub fn i_072f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F62;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xE, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072F62: rts
pub fn i_072f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnPuffB_072f64  ($072F64, 36 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072F64: lea.l $72388(pc), a1
pub fn i_072f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72388;
    16
}
/// $072F68: jsr $4ae.l
pub fn i_072f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F6E);
    cpu.pc = 0x4AE;
    16
}
/// $072F6E: jsr $5dd02.l
pub fn i_072f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F74);
    cpu.pc = 0x5DD02;
    16
}
/// $072F74: move.w #$fff4, d0
pub fn i_072f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F78;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072F78: jsr $72782(pc)
pub fn i_072f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72F7C);
    cpu.pc = 0x72782;
    16
}
/// $072F7C: add.w d0, $22(a0)
pub fn i_072f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F80;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072F80: addi.w #$fffe, $24(a0)
pub fn i_072f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F86;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFFE, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072F86: rts
pub fn i_072f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_SpawnPuffC_072f88  ($072F88, 36 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072F88: lea.l $72388(pc), a1
pub fn i_072f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x72388;
    16
}
/// $072F8C: jsr $4ae.l
pub fn i_072f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F92);
    cpu.pc = 0x4AE;
    16
}
/// $072F92: jsr $5dd02.l
pub fn i_072f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72F98);
    cpu.pc = 0x5DD02;
    16
}
/// $072F98: move.w #$4, d0
pub fn i_072f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72F9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072F9C: jsr $72782(pc)
pub fn i_072f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x72FA0);
    cpu.pc = 0x72782;
    16
}
/// $072FA0: add.w d0, $22(a0)
pub fn i_072fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FA4;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $072FA4: addi.w #$fffe, $24(a0)
pub fn i_072fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FAA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFFE, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $072FAA: rts
pub fn i_072faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FinalBoss_CmpDepthToParent_072fac  ($072FAC, 16 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072FAC: movea.l $8(a6), a1
pub fn i_072fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FB0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $072FB0: move.b $10(a6), d0
pub fn i_072fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FB4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $072FB4: cmp.b $10(a1), d0
pub fn i_072fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FB8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $072FB8: bcs.w $72fc2
pub fn i_072fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FBC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x72FC2; return 10; }
    8
}
// ===== ClearXN_072fbc  ($072FBC, 6 B, thunks/ccr_helpers.c)
/// $072FBC: andi.b #$ee, ccr
pub fn i_072fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FC0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $072FC0: rts
pub fn i_072fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_072fc2  ($072FC2, 6 B, thunks/ccr_helpers.c)
/// $072FC2: ori.b #$11, ccr
pub fn i_072fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FC6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $072FC6: rts
pub fn i_072fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Tmpl76_072fc8  ($072FC8, 244 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $072FC8: move.w #$28, d0
pub fn i_072fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FCC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072FCC: jsr $2352.l
pub fn i_072fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72FD2);
    cpu.pc = 0x2352;
    16
}
/// $072FD2: move.w #$ec, d1
pub fn i_072fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FD6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $072FD6: jsr $236e.l
pub fn i_072fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72FDC);
    cpu.pc = 0x236E;
    16
}
/// $072FDC: move.w #$16, $1c(a6)
pub fn i_072fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $072FE2: jsr $138fe.l
pub fn i_072fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x72FE8);
    cpu.pc = 0x138FE;
    16
}
/// $072FE8: move.l #$2dab18, $4c(a6)
pub fn i_072fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FF0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DAB18;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $072FF0: move.l #$2da784, $48(a6)
pub fn i_072ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72FF8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DA784;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $072FF8: move.l #$2db018, $60(a6)
pub fn i_072ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73000;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DB018;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073000: move.w #$4000, d0
pub fn i_073000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73004;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073004: jsr $28134.l
pub fn i_073004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7300A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7300A);
    cpu.pc = 0x28134;
    16
}
/// $07300A: andi.w #$ffe3, $38(a6)
pub fn i_07300a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73010;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $073010: ori.w #$14, $38(a6)
pub fn i_073010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73016;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $073016: lea.l $2baef8.l, a0
pub fn i_073016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7301C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAEF8;
    20
}
/// $07301C: jsr $799de.l
pub fn i_07301c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73022);
    cpu.pc = 0x799DE;
    16
}
/// $073022: move.w d0, $66(a6)
pub fn i_073022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73026;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073026: lea.l $2baf7a.l, a0
pub fn i_073026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7302C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAF7A;
    20
}
/// $07302C: jsr $799de.l
pub fn i_07302c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73032;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73032);
    cpu.pc = 0x799DE;
    16
}
/// $073032: move.w d0, $82(a6)
pub fn i_073032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73036;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073036: jsr $5e7c0.l
pub fn i_073036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7303C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7303C);
    cpu.pc = 0x5E7C0;
    16
}
/// $07303C: lea.l $723d2.l, a1
pub fn i_07303c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73042;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723D2;
    20
}
/// $073042: jsr $4ae.l
pub fn i_073042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73048);
    cpu.pc = 0x4AE;
    16
}
/// $073048: jsr $5dd02.l
pub fn i_073048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7304E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7304E);
    cpu.pc = 0x5DD02;
    16
}
/// $07304E: move.w #$3c, $98(a0)
pub fn i_07304e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73054;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073054: lea.l $7350a(pc), a1
pub fn i_073054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73058;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7350A;
    16
}
/// $073058: jsr $4ae.l
pub fn i_073058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7305E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7305E);
    cpu.pc = 0x4AE;
    16
}
/// $07305E: jsr $5dd02.l
pub fn i_07305e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73064;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73064);
    cpu.pc = 0x5DD02;
    16
}
/// $073064: clr.w $7c(a0)
pub fn i_073064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73068;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $073068: lea.l $735b8(pc), a1
pub fn i_073068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7306C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x735B8;
    16
}
/// $07306C: jsr $4ae.l
pub fn i_07306c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73072);
    cpu.pc = 0x4AE;
    16
}
/// $073072: jsr $5dd02.l
pub fn i_073072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73078);
    cpu.pc = 0x5DD02;
    16
}
/// $073078: move.w #$ffff, $7c(a0)
pub fn i_073078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7307E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07307E: lea.l $73688(pc), a1
pub fn i_07307e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73082;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73688;
    16
}
/// $073082: jsr $4ae.l
pub fn i_073082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73088);
    cpu.pc = 0x4AE;
    16
}
/// $073088: jsr $5dd02.l
pub fn i_073088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7308E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7308E);
    cpu.pc = 0x5DD02;
    16
}
/// $07308E: move.w #$ffff, $7c(a0)
pub fn i_07308e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73094;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073094: lea.l $73952(pc), a1
pub fn i_073094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73098;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73952;
    16
}
/// $073098: jsr $4ae.l
pub fn i_073098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7309E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7309E);
    cpu.pc = 0x4AE;
    16
}
/// $07309E: jsr $5dd02.l
pub fn i_07309e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x730A4);
    cpu.pc = 0x5DD02;
    16
}
/// $0730A4: clr.b $73(a6)
pub fn i_0730a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730A8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0730A8: clr.b $75(a6)
pub fn i_0730a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730AC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0730AC: move.w #$140, $36(a6)
pub fn i_0730ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0730B2: move.w #$1073, d0
pub fn i_0730b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1073;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0730B6: jsr $2352.l
pub fn i_0730b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x730BC);
    cpu.pc = 0x2352;
    16
}
// ===== M5Tank_Drive_0730bc  ($0730BC, 140 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0730BC: tst.b $75(a6)
pub fn i_0730bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730C0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0730C0: bne.w $731b4
pub fn i_0730c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x731B4; return 10; }
    8
}
/// $0730C4: jsr $74122(pc)
pub fn i_0730c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x730C8);
    cpu.pc = 0x74122;
    16
}
/// $0730C8: bcs.w $73148
pub fn i_0730c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x73148; return 10; }
    8
}
/// $0730CC: move.w $36(a6), d0
pub fn i_0730cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0730D0: move.w #$4, d1
pub fn i_0730d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0730D4: btst.b #$0, $73(a6)
pub fn i_0730d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730DA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0730DA: bne.w $730e2
pub fn i_0730da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x730E2; return 10; }
    8
}
/// $0730DE: clr.w d1
pub fn i_0730de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730E0;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0730E0: neg.w d0
pub fn i_0730e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730E2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0730E2: move.w d0, $28(a6)
pub fn i_0730e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0730E6: move.b $74(a6), d0
pub fn i_0730e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0730EA: andi.w #$3, d0
pub fn i_0730ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730EE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0730EE: add.w d1, d0
pub fn i_0730ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730F0;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0730F0: movea.l #$2db11c, a0
pub fn i_0730f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730F6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DB11C;
    cpu.a[0] = v1;
    4
}
/// $0730F6: lsl.w #$2, d0
pub fn i_0730f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730F8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0730F8: movea.l (a0, d0.w), a0
pub fn i_0730f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730FC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0730FC: cmpa.l #$ffffffff, a0
pub fn i_0730fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73102;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $073102: beq.w $7310c
pub fn i_073102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73106;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7310C; return 10; }
    8
}
/// $073106: jsr $28cd4.l
pub fn i_073106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7310C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7310C);
    cpu.pc = 0x28CD4;
    16
}
/// $07310C: move.l #$2daa24, $4c(a6)
pub fn i_07310c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73114;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DAA24;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073114: lea.l $7311a(pc), a1
pub fn i_073114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73118;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7311A;
    16
}
/// $073118: move.l a1, (a6)
pub fn i_073118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7311A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07311A: jsr $28998.l
pub fn i_07311a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73120);
    cpu.pc = 0x28998;
    16
}
/// $073120: jsr $27afc.l
pub fn i_073120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73126;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73126);
    cpu.pc = 0x27AFC;
    16
}
/// $073126: bcc.w $73130
pub fn i_073126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7312A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73130; return 10; }
    8
}
/// $07312A: lea.l $73148(pc), a1
pub fn i_07312a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7312E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73148;
    16
}
/// $07312E: move.l a1, (a6)
pub fn i_07312e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73130;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073130: jsr $7410e(pc)
pub fn i_073130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73134;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73134);
    cpu.pc = 0x7410E;
    16
}
/// $073134: bcc.w $7313e
pub fn i_073134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73138;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7313E; return 10; }
    8
}
/// $073138: lea.l $73148(pc), a1
pub fn i_073138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7313C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73148;
    16
}
/// $07313C: move.l a1, (a6)
pub fn i_07313c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7313E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07313E: jsr $28d70.l
pub fn i_07313e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73144;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73144);
    cpu.pc = 0x28D70;
    16
}
/// $073144: bra.w $73382
pub fn i_073144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73148;
    cpu.ir = 0x6000;
    cpu.pc = 0x73382;
    10
}
// ===== M5Tank_Turn_073148  ($073148, 108 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073148: eori.b #$1, $73(a6)
pub fn i_073148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7314E;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07314E: clr.w $28(a6)
pub fn i_07314e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73152;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $073152: move.w #$3c, $70(a6)
pub fn i_073152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73158;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073158: move.b $74(a6), d0
pub fn i_073158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7315C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07315C: andi.w #$7, d0
pub fn i_07315c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73160;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $073160: movea.l #$2db0fc, a0
pub fn i_073160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73166;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DB0FC;
    cpu.a[0] = v1;
    4
}
/// $073166: lsl.w #$2, d0
pub fn i_073166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73168;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $073168: movea.l (a0, d0.w), a0
pub fn i_073168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7316C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07316C: cmpa.l #$ffffffff, a0
pub fn i_07316c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73172;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $073172: beq.w $7317c
pub fn i_073172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73176;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7317C; return 10; }
    8
}
/// $073176: jsr $28cd4.l
pub fn i_073176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7317C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7317C);
    cpu.pc = 0x28CD4;
    16
}
/// $07317C: move.l #$2dab18, $4c(a6)
pub fn i_07317c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73184;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DAB18;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073184: lea.l $7318a(pc), a1
pub fn i_073184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73188;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7318A;
    16
}
/// $073188: move.l a1, (a6)
pub fn i_073188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7318A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07318A: jsr $28998.l
pub fn i_07318a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73190;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73190);
    cpu.pc = 0x28998;
    16
}
/// $073190: jsr $2783a.l
pub fn i_073190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73196;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73196);
    cpu.pc = 0x2783A;
    16
}
/// $073196: jsr $28d70.l
pub fn i_073196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7319C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7319C);
    cpu.pc = 0x28D70;
    16
}
/// $07319C: subq.w #$1, $70(a6)
pub fn i_07319c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731A0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0731A0: cmpi.w #$0, $70(a6)
pub fn i_0731a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731A6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0731A6: bgt.w $731b0
pub fn i_0731a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731AA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x731B0; return 10; }
    8
}
/// $0731AA: lea.l $730bc(pc), a1
pub fn i_0731aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x730BC;
    16
}
/// $0731AE: move.l a1, (a6)
pub fn i_0731ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0731B0: bra.w $73382
pub fn i_0731b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x73382;
    10
}
// ===== M5Tank_Retreat_0731b4  ($0731B4, 170 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0731B4: move.w #$1050, d0
pub fn i_0731b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731B8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1050;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0731B8: move.w #$cf, d1
pub fn i_0731b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731BC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0731BC: jsr $440d0.l
pub fn i_0731bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x731C2);
    cpu.pc = 0x440D0;
    16
}
/// $0731C2: move.w d0, $5c(a6)
pub fn i_0731c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0731C6: move.w d0, d2
pub fn i_0731c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731C8;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0731C8: move.w $36(a6), d0
pub fn i_0731c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731CC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0731CC: asr.w #$1, d0
pub fn i_0731cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731CE;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0731CE: move.w #$4, d1
pub fn i_0731ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0731D2: move.b #$1, $73(a6)
pub fn i_0731d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0731D8: cmp.w $22(a6), d2
pub fn i_0731d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731DC;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0731DC: bge.w $731e8
pub fn i_0731dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731E0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x731E8; return 10; }
    8
}
/// $0731E0: clr.w d1
pub fn i_0731e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731E2;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0731E2: neg.w d0
pub fn i_0731e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731E4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0731E4: clr.b $73(a6)
pub fn i_0731e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731E8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0731E8: move.w d0, $28(a6)
pub fn i_0731e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731EC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0731EC: move.b $74(a6), d0
pub fn i_0731ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731F0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0731F0: andi.w #$3, d0
pub fn i_0731f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731F4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0731F4: add.w d1, d0
pub fn i_0731f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731F6;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0731F6: movea.l #$2db13c, a0
pub fn i_0731f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731FC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DB13C;
    cpu.a[0] = v1;
    4
}
/// $0731FC: lsl.w #$2, d0
pub fn i_0731fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x731FE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0731FE: movea.l (a0, d0.w), a0
pub fn i_0731fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73202;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $073202: cmpa.l #$ffffffff, a0
pub fn i_073202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73208;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $073208: beq.w $73212
pub fn i_073208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7320C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73212; return 10; }
    8
}
/// $07320C: jsr $28cd4.l
pub fn i_07320c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73212);
    cpu.pc = 0x28CD4;
    16
}
/// $073212: move.l #$2daa24, $4c(a6)
pub fn i_073212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7321A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DAA24;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07321A: move.w #$b4, $70(a6)
pub fn i_07321a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73220;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073220: lea.l $73226(pc), a1
pub fn i_073220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73224;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73226;
    16
}
/// $073224: move.l a1, (a6)
pub fn i_073224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73226;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073226: jsr $28998.l
pub fn i_073226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7322C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7322C);
    cpu.pc = 0x28998;
    16
}
/// $07322C: jsr $27afc.l
pub fn i_07322c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73232);
    cpu.pc = 0x27AFC;
    16
}
/// $073232: jsr $74196(pc)
pub fn i_073232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73236;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73236);
    cpu.pc = 0x74196;
    16
}
/// $073236: bcc.w $73240
pub fn i_073236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7323A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73240; return 10; }
    8
}
/// $07323A: lea.l $7325e(pc), a1
pub fn i_07323a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7323E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7325E;
    16
}
/// $07323E: move.l a1, (a6)
pub fn i_07323e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73240;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073240: jsr $28d70.l
pub fn i_073240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73246);
    cpu.pc = 0x28D70;
    16
}
/// $073246: subq.w #$1, $70(a6)
pub fn i_073246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7324A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07324A: cmpi.w #$0, $70(a6)
pub fn i_07324a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73250;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $073250: bgt.w $7325a
pub fn i_073250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73254;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7325A; return 10; }
    8
}
/// $073254: lea.l $7325e(pc), a1
pub fn i_073254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73258;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7325E;
    16
}
/// $073258: move.l a1, (a6)
pub fn i_073258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7325A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07325A: bra.w $73382
pub fn i_07325a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7325E;
    cpu.ir = 0x6000;
    cpu.pc = 0x73382;
    10
}
// ===== M5Tank_Pause_07325e  ($07325E, 50 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07325E: move.w #$14, $70(a6)
pub fn i_07325e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73264;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073264: move.b #$2, $75(a6)
pub fn i_073264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7326A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07326A: move.l #$2dab18, $4c(a6)
pub fn i_07326a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73272;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DAB18;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073272: lea.l $73278(pc), a1
pub fn i_073272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73276;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73278;
    16
}
/// $073276: move.l a1, (a6)
pub fn i_073276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73278;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073278: subq.w #$1, $70(a6)
pub fn i_073278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7327C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07327C: cmpi.w #$0, $70(a6)
pub fn i_07327c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73282;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $073282: bgt.w $7328c
pub fn i_073282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73286;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7328C; return 10; }
    8
}
/// $073286: lea.l $73290(pc), a1
pub fn i_073286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7328A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73290;
    16
}
/// $07328A: move.l a1, (a6)
pub fn i_07328a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7328C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07328C: bra.w $732ac
pub fn i_07328c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73290;
    cpu.ir = 0x6000;
    cpu.pc = 0x732AC;
    10
}
// ===== M5Tank_Phase2_073290  ($073290, 88 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073290: move.w #$10a7, d0
pub fn i_073290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73294;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073294: jsr $2352.l
pub fn i_073294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7329A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7329A);
    cpu.pc = 0x2352;
    16
}
/// $07329A: lea.l $2db5a4.l, a0
pub fn i_07329a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB5A4;
    20
}
/// $0732A0: jsr $28cd4.l
pub fn i_0732a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x732A6);
    cpu.pc = 0x28CD4;
    16
}
/// $0732A6: lea.l $732ac(pc), a1
pub fn i_0732a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x732AC;
    16
}
/// $0732AA: move.l a1, (a6)
pub fn i_0732aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0732AC: jsr $28998.l
pub fn i_0732ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x732B2);
    cpu.pc = 0x28998;
    16
}
/// $0732B2: jsr $2783a.l
pub fn i_0732b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x732B8);
    cpu.pc = 0x2783A;
    16
}
/// $0732B8: jsr $28d70.l
pub fn i_0732b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x732BE);
    cpu.pc = 0x28D70;
    16
}
/// $0732BE: bcc.w $732c8
pub fn i_0732be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x732C8; return 10; }
    8
}
/// $0732C2: lea.l $732e8(pc), a1
pub fn i_0732c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x732E8;
    16
}
/// $0732C6: move.l a1, (a6)
pub fn i_0732c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0732C8: jsr $2870a.l
pub fn i_0732c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x732CE);
    cpu.pc = 0x2870A;
    16
}
/// $0732CE: bcc.w $732e4
pub fn i_0732ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x732E4; return 10; }
    8
}
/// $0732D2: bclr.b #$3, $13(a6)
pub fn i_0732d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732D8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0732D8: lea.l $5e766.l, a0
pub fn i_0732d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0732DE: jsr $5e770.l
pub fn i_0732de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x732E4);
    cpu.pc = 0x5E770;
    16
}
/// $0732E4: bra.w $733d4
pub fn i_0732e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x733D4;
    10
}
// ===== M5Tank_Phase3_0732e8  ($0732E8, 282 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0732E8: lea.l $2bb07e.l, a0
pub fn i_0732e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB07E;
    20
}
/// $0732EE: jsr $799de.l
pub fn i_0732ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x732F4);
    cpu.pc = 0x799DE;
    16
}
/// $0732F4: move.w d0, $90(a6)
pub fn i_0732f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732F8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0732F8: clr.b $92(a6)
pub fn i_0732f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x732FC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0732FC: move.w #$92, d0
pub fn i_0732fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73300;
    cpu.ir = 0x303C;
    let v1: u32 = 0x92;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073300: lea.l $ee1b4.l, a1
pub fn i_073300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73306;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE1B4;
    20
}
/// $073306: jsr $4429e.l
pub fn i_073306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7330C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7330C);
    cpu.pc = 0x4429E;
    16
}
/// $07330C: lea.l $73312(pc), a1
pub fn i_07330c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73310;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73312;
    16
}
/// $073310: move.l a1, (a6)
pub fn i_073310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73312;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073312: jsr $28998.l
pub fn i_073312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73318);
    cpu.pc = 0x28998;
    16
}
/// $073318: jsr $2783a.l
pub fn i_073318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7331E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7331E);
    cpu.pc = 0x2783A;
    16
}
/// $07331E: jsr $28d70.l
pub fn i_07331e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73324);
    cpu.pc = 0x28D70;
    16
}
/// $073324: jsr $2870a.l
pub fn i_073324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7332A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7332A);
    cpu.pc = 0x2870A;
    16
}
/// $07332A: bcc.w $73340
pub fn i_07332a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7332E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73340; return 10; }
    8
}
/// $07332E: bclr.b #$3, $13(a6)
pub fn i_07332e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73334;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073334: lea.l $5e766.l, a0
pub fn i_073334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7333A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07333A: jsr $5e770.l
pub fn i_07333a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73340);
    cpu.pc = 0x5E770;
    16
}
/// $073340: subq.w #$1, $90(a6)
pub fn i_073340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73344;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $073344: cmpi.w #$ffff, $90(a6)
pub fn i_073344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7334A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $07334A: bgt.w $73354
pub fn i_07334a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7334E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x73354; return 10; }
    8
}
/// $07334E: move.w #$ffff, $90(a6)
pub fn i_07334e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73354;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073354: jsr $28758.l
pub fn i_073354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7335A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7335A);
    cpu.pc = 0x28758;
    16
}
/// $07335A: bcc.w $7337e
pub fn i_07335a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7335E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7337E; return 10; }
    8
}
/// $07335E: cmpi.w #$0, $90(a6)
pub fn i_07335e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73364;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $073364: bgt.w $73372
pub fn i_073364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73368;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x73372; return 10; }
    8
}
/// $073368: lea.l $7340a(pc), a1
pub fn i_073368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7336C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7340A;
    16
}
/// $07336C: move.l a1, (a6)
pub fn i_07336c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7336E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07336E: bra.w $7337e
pub fn i_07336e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73372;
    cpu.ir = 0x6000;
    cpu.pc = 0x7337E;
    10
}
/// $073372: bclr.b #$0, $13(a6)
pub fn i_073372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73378;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073378: move.w #$14, $66(a6)
pub fn i_073378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7337E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07337E: bra.w $733fc
pub fn i_07337e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73382;
    cpu.ir = 0x6000;
    cpu.pc = 0x733FC;
    10
}
/// $073382: jsr $2870a.l
pub fn i_073382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73388;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73388);
    cpu.pc = 0x2870A;
    16
}
/// $073388: bcc.w $733d4
pub fn i_073388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7338C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x733D4; return 10; }
    8
}
/// $07338C: bclr.b #$3, $13(a6)
pub fn i_07338c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73392;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073392: lea.l $5e766.l, a0
pub fn i_073392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73398;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $073398: jsr $5e770.l
pub fn i_073398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7339E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7339E);
    cpu.pc = 0x5E770;
    16
}
/// $07339E: tst.b $75(a6)
pub fn i_07339e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733A2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0733A2: bne.w $733d4
pub fn i_0733a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x733D4; return 10; }
    8
}
/// $0733A6: move.w $66(a6), d0
pub fn i_0733a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733AA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0733AA: cmp.w $82(a6), d0
pub fn i_0733aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733AE;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0733AE: bgt.w $733d4
pub fn i_0733ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733B2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x733D4; return 10; }
    8
}
/// $0733B2: move.b #$1, $75(a6)
pub fn i_0733b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733B8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0733B8: lea.l $2db268.l, a1
pub fn i_0733b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DB268;
    20
}
/// $0733BE: jsr $77c7e.l
pub fn i_0733be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x733C4);
    cpu.pc = 0x77C7E;
    16
}
/// $0733C4: move.w #$1032, d0
pub fn i_0733c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733C8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1032;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0733C8: jsr $2352.l
pub fn i_0733c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x733CE);
    cpu.pc = 0x2352;
    16
}
/// $0733CE: lea.l $731b4(pc), a1
pub fn i_0733ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x731B4;
    16
}
/// $0733D2: move.l a1, (a6)
pub fn i_0733d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0733D4: jsr $28758.l
pub fn i_0733d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x733DA);
    cpu.pc = 0x28758;
    16
}
/// $0733DA: bcc.w $733fc
pub fn i_0733da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x733FC; return 10; }
    8
}
/// $0733DE: move.w #$14, $66(a6)
pub fn i_0733de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0733E4: bclr.b #$0, $13(a6)
pub fn i_0733e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733EA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0733EA: bset.b #$0, $5a(a6)
pub fn i_0733ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733F0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0733F0: lea.l $5e766.l, a0
pub fn i_0733f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0733F6: jsr $5e770.l
pub fn i_0733f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x733FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x733FC);
    cpu.pc = 0x5E770;
    16
}
/// $0733FC: jsr $283ca.l
pub fn i_0733fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73402;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73402);
    cpu.pc = 0x283CA;
    16
}
// ===== JsrAbsThunk_073402  ($073402, 8 B, thunks/jsr_abs_thunks.c)
/// $073402: jsr $283d8.l
pub fn i_073402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73408);
    cpu.pc = 0x283D8;
    16
}
/// $073408: rts
pub fn i_073408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7340A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Die_07340a  ($07340A, 66 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07340A: jsr $745b0(pc)
pub fn i_07340a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7340E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7340E);
    cpu.pc = 0x745B0;
    16
}
/// $07340E: clr.b $106ed3.l
pub fn i_07340e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73414;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED3) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED3, r2 as u8);
    16
}
/// $073414: move.w #$1033, d0
pub fn i_073414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73418;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073418: jsr $2352.l
pub fn i_073418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7341E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7341E);
    cpu.pc = 0x2352;
    16
}
/// $07341E: move.b #$6, $75(a6)
pub fn i_07341e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73424;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073424: lea.l $2db6ce.l, a0
pub fn i_073424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7342A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB6CE;
    20
}
/// $07342A: jsr $28cd4.l
pub fn i_07342a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73430);
    cpu.pc = 0x28CD4;
    16
}
/// $073430: lea.l $73436(pc), a1
pub fn i_073430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73434;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73436;
    16
}
/// $073434: move.l a1, (a6)
pub fn i_073434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73436;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073436: jsr $2783a.l
pub fn i_073436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7343C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7343C);
    cpu.pc = 0x2783A;
    16
}
/// $07343C: jsr $28d70.l
pub fn i_07343c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73442);
    cpu.pc = 0x28D70;
    16
}
/// $073442: bcc.w $73452
pub fn i_073442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73446;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73452; return 10; }
    8
}
/// $073446: clr.b $106ed2.l
pub fn i_073446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7344C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
// ===== SetTaskHandler_07344c  ($07344C, 8 B, kernel/task_handlers.c)
/// $07344C: lea.l $73454(pc), a1
pub fn i_07344c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73450;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73454;
    16
}
/// $073450: move.l a1, (a6)
pub fn i_073450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73452;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073452: rts
pub fn i_073452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Sink_073454  ($073454, 48 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073454: lea.l $2db6de.l, a0
pub fn i_073454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7345A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB6DE;
    20
}
/// $07345A: jsr $28cd4.l
pub fn i_07345a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73460);
    cpu.pc = 0x28CD4;
    16
}
/// $073460: lea.l $73466(pc), a1
pub fn i_073460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73464;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73466;
    16
}
/// $073464: move.l a1, (a6)
pub fn i_073464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73466;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073466: jsr $2783a.l
pub fn i_073466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7346C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7346C);
    cpu.pc = 0x2783A;
    16
}
/// $07346C: jsr $28d70.l
pub fn i_07346c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73472);
    cpu.pc = 0x28D70;
    16
}
/// $073472: bcc.w $7347c
pub fn i_073472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73476;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7347C; return 10; }
    8
}
/// $073476: lea.l $73484(pc), a1
pub fn i_073476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7347A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73484;
    16
}
/// $07347A: move.l a1, (a6)
pub fn i_07347a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7347C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07347C: move.b #$ff, $92(a6)
pub fn i_07347c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73482;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073482: rts
pub fn i_073482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73484;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Explode_073484  ($073484, 110 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073484: addi.w #$20, $24(a6)
pub fn i_073484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7348A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07348A: move.w #$102f, d0
pub fn i_07348a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7348E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07348E: jsr $2352.l
pub fn i_07348e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73494);
    cpu.pc = 0x2352;
    16
}
/// $073494: lea.l $77fd6.l, a1
pub fn i_073494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7349A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07349A: jsr $4ae.l
pub fn i_07349a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x734A0);
    cpu.pc = 0x4AE;
    16
}
/// $0734A0: jsr $5dd02.l
pub fn i_0734a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x734A6);
    cpu.pc = 0x5DD02;
    16
}
/// $0734A6: addi.w #$ffc8, $22(a0)
pub fn i_0734a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734AC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFC8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0734AC: lea.l $77fd6.l, a1
pub fn i_0734ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734B2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0734B2: jsr $4ae.l
pub fn i_0734b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x734B8);
    cpu.pc = 0x4AE;
    16
}
/// $0734B8: jsr $5dd02.l
pub fn i_0734b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x734BE);
    cpu.pc = 0x5DD02;
    16
}
/// $0734BE: addi.w #$0, $22(a0)
pub fn i_0734be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734C4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0734C4: lea.l $77fd6.l, a1
pub fn i_0734c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734CA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0734CA: jsr $4ae.l
pub fn i_0734ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x734D0);
    cpu.pc = 0x4AE;
    16
}
/// $0734D0: jsr $5dd02.l
pub fn i_0734d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x734D6);
    cpu.pc = 0x5DD02;
    16
}
/// $0734D6: addi.w #$38, $22(a0)
pub fn i_0734d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734DC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0734DC: jsr $7455a(pc)
pub fn i_0734dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x734E0);
    cpu.pc = 0x7455A;
    16
}
/// $0734E0: subi.w #$20, $24(a6)
pub fn i_0734e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734E6;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0734E6: jsr $28d70.l
pub fn i_0734e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x734EC);
    cpu.pc = 0x28D70;
    16
}
/// $0734EC: jmp $518.l
pub fn i_0734ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M5Tank_Rts_0734f2  ($0734F2, 2 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0734F2: rts
pub fn i_0734f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0734f4  ($0734F4, 8 B, kernel/jmp_to_scheduler.c)
/// $0734F4: jmp $518.l
pub fn i_0734f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734FA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Jsr5B6ThenJmpScheduler_0734fc  ($0734FC, 14 B, kernel/jsr5b6_jmp518.c)
/// $0734FC: jsr $5b6.l
pub fn i_0734fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73502);
    cpu.pc = 0x5B6;
    16
}
/// $073502: jmp $518.l
pub fn i_073502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73508;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M5Tank_Hull_07350a  ($07350A, 148 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07350A: move.l #$2db15c, $8c(a6)
pub fn i_07350a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73512;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DB15C;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073512: move.l #$2dadb4, $48(a6)
pub fn i_073512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7351A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DADB4;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07351A: move.w #$64, $66(a6)
pub fn i_07351a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73520;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073520: move.w #$ed, d1
pub fn i_073520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73524;
    cpu.ir = 0x323C;
    let v1: u32 = 0xED;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073524: jsr $236e.l
pub fn i_073524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7352A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7352A);
    cpu.pc = 0x236E;
    16
}
/// $07352A: move.w #$17, $1c(a6)
pub fn i_07352a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73530;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x17;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073530: jsr $138fe.l
pub fn i_073530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73536);
    cpu.pc = 0x138FE;
    16
}
/// $073536: lea.l $2db6ee.l, a0
pub fn i_073536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7353C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB6EE;
    20
}
/// $07353C: jsr $28cd4.l
pub fn i_07353c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73542);
    cpu.pc = 0x28CD4;
    16
}
/// $073542: lea.l $73548(pc), a1
pub fn i_073542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73546;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73548;
    16
}
/// $073546: move.l a1, (a6)
pub fn i_073546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73548;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073548: jsr $741f8(pc)
pub fn i_073548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7354C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7354C);
    cpu.pc = 0x741F8;
    16
}
/// $07354C: jsr $28d70.l
pub fn i_07354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73552);
    cpu.pc = 0x28D70;
    16
}
/// $073552: jsr $2870a.l
pub fn i_073552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73558);
    cpu.pc = 0x2870A;
    16
}
/// $073558: bcc.w $7356e
pub fn i_073558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7355C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7356E; return 10; }
    8
}
/// $07355C: bclr.b #$3, $13(a6)
pub fn i_07355c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73562;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073562: lea.l $5e766.l, a0
pub fn i_073562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73568;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $073568: jsr $5e770.l
pub fn i_073568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7356E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7356E);
    cpu.pc = 0x5E770;
    16
}
/// $07356E: jsr $28758.l
pub fn i_07356e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73574);
    cpu.pc = 0x28758;
    16
}
/// $073574: bcc.w $7357e
pub fn i_073574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73578;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7357E; return 10; }
    8
}
/// $073578: lea.l $735a6(pc), a1
pub fn i_073578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7357C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x735A6;
    16
}
/// $07357C: move.l a1, (a6)
pub fn i_07357c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7357E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07357E: movea.l $c(a6), a0
pub fn i_07357e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73582;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073582: tst.b $75(a0)
pub fn i_073582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73586;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $073586: beq.w $73590
pub fn i_073586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7358A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73590; return 10; }
    8
}
/// $07358A: lea.l $735a6(pc), a1
pub fn i_07358a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7358E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x735A6;
    16
}
/// $07358E: move.l a1, (a6)
pub fn i_07358e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73590;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x071B0A, i_071b0a),
    (0x071B0E, i_071b0e),
    (0x071B10, i_071b10),
    (0x071B14, i_071b14),
    (0x071B18, i_071b18),
    (0x071B1C, i_071b1c),
    (0x071B20, i_071b20),
    (0x071B24, i_071b24),
    (0x071B26, i_071b26),
    (0x071B2C, i_071b2c),
    (0x071B30, i_071b30),
    (0x071B36, i_071b36),
    (0x071B3A, i_071b3a),
    (0x071B3E, i_071b3e),
    (0x071B42, i_071b42),
    (0x071B46, i_071b46),
    (0x071B4A, i_071b4a),
    (0x071B4E, i_071b4e),
    (0x071B52, i_071b52),
    (0x071B54, i_071b54),
    (0x071B56, i_071b56),
    (0x071B5C, i_071b5c),
    (0x071B62, i_071b62),
    (0x071B66, i_071b66),
    (0x071B6A, i_071b6a),
    (0x071B6C, i_071b6c),
    (0x071B70, i_071b70),
    (0x071B74, i_071b74),
    (0x071B78, i_071b78),
    (0x071B7A, i_071b7a),
    (0x071B7E, i_071b7e),
    (0x071B82, i_071b82),
    (0x071B86, i_071b86),
    (0x071B8A, i_071b8a),
    (0x071B8E, i_071b8e),
    (0x071B92, i_071b92),
    (0x071B96, i_071b96),
    (0x071B98, i_071b98),
    (0x071B9A, i_071b9a),
    (0x071BA0, i_071ba0),
    (0x071BA4, i_071ba4),
    (0x071BA8, i_071ba8),
    (0x071BAE, i_071bae),
    (0x071BB4, i_071bb4),
    (0x071BB8, i_071bb8),
    (0x071BBC, i_071bbc),
    (0x071BC2, i_071bc2),
    (0x071BC6, i_071bc6),
    (0x071BCC, i_071bcc),
    (0x071BD2, i_071bd2),
    (0x071BD8, i_071bd8),
    (0x071BDE, i_071bde),
    (0x071BE4, i_071be4),
    (0x071BE6, i_071be6),
    (0x071BEC, i_071bec),
    (0x071BF2, i_071bf2),
    (0x071C02, i_071c02),
    (0x071C06, i_071c06),
    (0x071C0A, i_071c0a),
    (0x071C0E, i_071c0e),
    (0x071C12, i_071c12),
    (0x071C16, i_071c16),
    (0x071C1A, i_071c1a),
    (0x071C20, i_071c20),
    (0x071C26, i_071c26),
    (0x071C2C, i_071c2c),
    (0x071C30, i_071c30),
    (0x071C32, i_071c32),
    (0x071C36, i_071c36),
    (0x071C3A, i_071c3a),
    (0x071C3E, i_071c3e),
    (0x071C42, i_071c42),
    (0x071C46, i_071c46),
    (0x071C4C, i_071c4c),
    (0x071C50, i_071c50),
    (0x071C54, i_071c54),
    (0x071C56, i_071c56),
    (0x071C5C, i_071c5c),
    (0x071C60, i_071c60),
    (0x071C64, i_071c64),
    (0x071C66, i_071c66),
    (0x071C6C, i_071c6c),
    (0x071C72, i_071c72),
    (0x071C78, i_071c78),
    (0x071C7C, i_071c7c),
    (0x071C80, i_071c80),
    (0x071C82, i_071c82),
    (0x071C84, i_071c84),
    (0x071C8A, i_071c8a),
    (0x071C90, i_071c90),
    (0x071C94, i_071c94),
    (0x071C9A, i_071c9a),
    (0x071CA0, i_071ca0),
    (0x071CA4, i_071ca4),
    (0x071CA8, i_071ca8),
    (0x071CAA, i_071caa),
    (0x071CAC, i_071cac),
    (0x071CAE, i_071cae),
    (0x071CB0, i_071cb0),
    (0x071CB6, i_071cb6),
    (0x071CBC, i_071cbc),
    (0x071CC0, i_071cc0),
    (0x071CC4, i_071cc4),
    (0x071CC8, i_071cc8),
    (0x071CCE, i_071cce),
    (0x071CD2, i_071cd2),
    (0x071CD6, i_071cd6),
    (0x071CDC, i_071cdc),
    (0x071CE4, i_071ce4),
    (0x071CEA, i_071cea),
    (0x071CF0, i_071cf0),
    (0x071CF4, i_071cf4),
    (0x071CF6, i_071cf6),
    (0x071CFC, i_071cfc),
    (0x071D00, i_071d00),
    (0x071D06, i_071d06),
    (0x071D0A, i_071d0a),
    (0x071D0C, i_071d0c),
    (0x071D12, i_071d12),
    (0x071D16, i_071d16),
    (0x071D1A, i_071d1a),
    (0x071D1C, i_071d1c),
    (0x071D22, i_071d22),
    (0x071D26, i_071d26),
    (0x071D2A, i_071d2a),
    (0x071D30, i_071d30),
    (0x071D34, i_071d34),
    (0x071D38, i_071d38),
    (0x071D3A, i_071d3a),
    (0x071D3C, i_071d3c),
    (0x071D42, i_071d42),
    (0x071D44, i_071d44),
    (0x071D48, i_071d48),
    (0x071D4E, i_071d4e),
    (0x071D54, i_071d54),
    (0x071D5A, i_071d5a),
    (0x071D5E, i_071d5e),
    (0x071D62, i_071d62),
    (0x071D64, i_071d64),
    (0x071D6A, i_071d6a),
    (0x071D70, i_071d70),
    (0x071D76, i_071d76),
    (0x071D7C, i_071d7c),
    (0x071D82, i_071d82),
    (0x071D86, i_071d86),
    (0x071D8A, i_071d8a),
    (0x071D8C, i_071d8c),
    (0x071D92, i_071d92),
    (0x071D96, i_071d96),
    (0x071D9A, i_071d9a),
    (0x071D9C, i_071d9c),
    (0x071DA2, i_071da2),
    (0x071DA8, i_071da8),
    (0x071DAE, i_071dae),
    (0x071DB2, i_071db2),
    (0x071DB6, i_071db6),
    (0x071DB8, i_071db8),
    (0x071DBA, i_071dba),
    (0x071DBE, i_071dbe),
    (0x071DC4, i_071dc4),
    (0x071DC8, i_071dc8),
    (0x071DCC, i_071dcc),
    (0x071DD0, i_071dd0),
    (0x071DD4, i_071dd4),
    (0x071DD8, i_071dd8),
    (0x071DDC, i_071ddc),
    (0x071DE0, i_071de0),
    (0x071DE6, i_071de6),
    (0x071DEC, i_071dec),
    (0x071DF2, i_071df2),
    (0x071DF8, i_071df8),
    (0x071DFC, i_071dfc),
    (0x071DFE, i_071dfe),
    (0x071E02, i_071e02),
    (0x071E06, i_071e06),
    (0x071E0A, i_071e0a),
    (0x071E0E, i_071e0e),
    (0x071E12, i_071e12),
    (0x071E18, i_071e18),
    (0x071E1C, i_071e1c),
    (0x071E20, i_071e20),
    (0x071E22, i_071e22),
    (0x071E28, i_071e28),
    (0x071E2C, i_071e2c),
    (0x071E30, i_071e30),
    (0x071E32, i_071e32),
    (0x071E38, i_071e38),
    (0x071E3E, i_071e3e),
    (0x071E44, i_071e44),
    (0x071E48, i_071e48),
    (0x071E4C, i_071e4c),
    (0x071E4E, i_071e4e),
    (0x071E50, i_071e50),
    (0x071E54, i_071e54),
    (0x071E5A, i_071e5a),
    (0x071E60, i_071e60),
    (0x071E66, i_071e66),
    (0x071E6C, i_071e6c),
    (0x071E70, i_071e70),
    (0x071E76, i_071e76),
    (0x071E7C, i_071e7c),
    (0x071E82, i_071e82),
    (0x071E86, i_071e86),
    (0x071E88, i_071e88),
    (0x071E8E, i_071e8e),
    (0x071E94, i_071e94),
    (0x071E98, i_071e98),
    (0x071E9C, i_071e9c),
    (0x071E9E, i_071e9e),
    (0x071EA4, i_071ea4),
    (0x071EAA, i_071eaa),
    (0x071EAE, i_071eae),
    (0x071EB2, i_071eb2),
    (0x071EB4, i_071eb4),
    (0x071EB6, i_071eb6),
    (0x071EBA, i_071eba),
    (0x071EC0, i_071ec0),
    (0x071EC6, i_071ec6),
    (0x071ECC, i_071ecc),
    (0x071ED0, i_071ed0),
    (0x071ED4, i_071ed4),
    (0x071ED8, i_071ed8),
    (0x071EDE, i_071ede),
    (0x071EE2, i_071ee2),
    (0x071EE6, i_071ee6),
    (0x071EEC, i_071eec),
    (0x071EF0, i_071ef0),
    (0x071EF6, i_071ef6),
    (0x071EFC, i_071efc),
    (0x071F02, i_071f02),
    (0x071F06, i_071f06),
    (0x071F08, i_071f08),
    (0x071F0C, i_071f0c),
    (0x071F10, i_071f10),
    (0x071F14, i_071f14),
    (0x071F18, i_071f18),
    (0x071F1E, i_071f1e),
    (0x071F24, i_071f24),
    (0x071F28, i_071f28),
    (0x071F2C, i_071f2c),
    (0x071F2E, i_071f2e),
    (0x071F34, i_071f34),
    (0x071F3A, i_071f3a),
    (0x071F3E, i_071f3e),
    (0x071F42, i_071f42),
    (0x071F44, i_071f44),
    (0x071F4A, i_071f4a),
    (0x071F4E, i_071f4e),
    (0x071F52, i_071f52),
    (0x071F54, i_071f54),
    (0x071F5A, i_071f5a),
    (0x071F60, i_071f60),
    (0x071F66, i_071f66),
    (0x071F6A, i_071f6a),
    (0x071F6E, i_071f6e),
    (0x071F70, i_071f70),
    (0x071F72, i_071f72),
    (0x071F78, i_071f78),
    (0x071F7E, i_071f7e),
    (0x071F84, i_071f84),
    (0x071F86, i_071f86),
    (0x071F8A, i_071f8a),
    (0x071F8E, i_071f8e),
    (0x071F92, i_071f92),
    (0x071F98, i_071f98),
    (0x071F9E, i_071f9e),
    (0x071FA4, i_071fa4),
    (0x071FA8, i_071fa8),
    (0x071FAE, i_071fae),
    (0x071FB4, i_071fb4),
    (0x071FBA, i_071fba),
    (0x071FBE, i_071fbe),
    (0x071FC0, i_071fc0),
    (0x071FC6, i_071fc6),
    (0x071FCA, i_071fca),
    (0x071FCE, i_071fce),
    (0x071FD0, i_071fd0),
    (0x071FD6, i_071fd6),
    (0x071FDC, i_071fdc),
    (0x071FE2, i_071fe2),
    (0x071FE6, i_071fe6),
    (0x071FEA, i_071fea),
    (0x071FEC, i_071fec),
    (0x071FF2, i_071ff2),
    (0x071FF6, i_071ff6),
    (0x071FFA, i_071ffa),
    (0x071FFC, i_071ffc),
    (0x072002, i_072002),
    (0x072008, i_072008),
    (0x07200E, i_07200e),
    (0x072012, i_072012),
    (0x072016, i_072016),
    (0x072018, i_072018),
    (0x07201A, i_07201a),
    (0x072020, i_072020),
    (0x072024, i_072024),
    (0x07202A, i_07202a),
    (0x072030, i_072030),
    (0x072036, i_072036),
    (0x07203A, i_07203a),
    (0x07203C, i_07203c),
    (0x072040, i_072040),
    (0x072044, i_072044),
    (0x072048, i_072048),
    (0x07204C, i_07204c),
    (0x072052, i_072052),
    (0x072058, i_072058),
    (0x07205C, i_07205c),
    (0x072060, i_072060),
    (0x072062, i_072062),
    (0x072068, i_072068),
    (0x07206E, i_07206e),
    (0x072072, i_072072),
    (0x072076, i_072076),
    (0x072078, i_072078),
    (0x07207E, i_07207e),
    (0x072082, i_072082),
    (0x072086, i_072086),
    (0x072088, i_072088),
    (0x07208E, i_07208e),
    (0x072094, i_072094),
    (0x07209A, i_07209a),
    (0x07209E, i_07209e),
    (0x0720A2, i_0720a2),
    (0x0720A4, i_0720a4),
    (0x0720A6, i_0720a6),
    (0x0720AC, i_0720ac),
    (0x0720B2, i_0720b2),
    (0x0720B8, i_0720b8),
    (0x0720BA, i_0720ba),
    (0x0720BE, i_0720be),
    (0x0720C2, i_0720c2),
    (0x0720C6, i_0720c6),
    (0x0720CC, i_0720cc),
    (0x0720D2, i_0720d2),
    (0x0720D8, i_0720d8),
    (0x0720DE, i_0720de),
    (0x0720E4, i_0720e4),
    (0x0720E8, i_0720e8),
    (0x0720EA, i_0720ea),
    (0x0720F0, i_0720f0),
    (0x0720F4, i_0720f4),
    (0x0720F8, i_0720f8),
    (0x0720FA, i_0720fa),
    (0x072100, i_072100),
    (0x072106, i_072106),
    (0x07210C, i_07210c),
    (0x072110, i_072110),
    (0x072114, i_072114),
    (0x072116, i_072116),
    (0x07211C, i_07211c),
    (0x072120, i_072120),
    (0x072124, i_072124),
    (0x072126, i_072126),
    (0x07212C, i_07212c),
    (0x072132, i_072132),
    (0x072138, i_072138),
    (0x07213C, i_07213c),
    (0x072140, i_072140),
    (0x072142, i_072142),
    (0x072144, i_072144),
    (0x072148, i_072148),
    (0x07214E, i_07214e),
    (0x072152, i_072152),
    (0x072158, i_072158),
    (0x07215E, i_07215e),
    (0x072164, i_072164),
    (0x07216A, i_07216a),
    (0x072170, i_072170),
    (0x072174, i_072174),
    (0x072176, i_072176),
    (0x07217A, i_07217a),
    (0x072180, i_072180),
    (0x072184, i_072184),
    (0x072188, i_072188),
    (0x07218C, i_07218c),
    (0x072190, i_072190),
    (0x072194, i_072194),
    (0x072198, i_072198),
    (0x07219C, i_07219c),
    (0x0721A2, i_0721a2),
    (0x0721A6, i_0721a6),
    (0x0721AA, i_0721aa),
    (0x0721AC, i_0721ac),
    (0x0721B2, i_0721b2),
    (0x0721B8, i_0721b8),
    (0x0721BC, i_0721bc),
    (0x0721C0, i_0721c0),
    (0x0721C2, i_0721c2),
    (0x0721C8, i_0721c8),
    (0x0721CC, i_0721cc),
    (0x0721D0, i_0721d0),
    (0x0721D2, i_0721d2),
    (0x0721D8, i_0721d8),
    (0x0721DE, i_0721de),
    (0x0721E2, i_0721e2),
    (0x0721E6, i_0721e6),
    (0x0721E8, i_0721e8),
    (0x0721EA, i_0721ea),
    (0x0721F0, i_0721f0),
    (0x0721F6, i_0721f6),
    (0x0721FA, i_0721fa),
    (0x0721FE, i_0721fe),
    (0x072204, i_072204),
    (0x07220A, i_07220a),
    (0x07220E, i_07220e),
    (0x072212, i_072212),
    (0x072218, i_072218),
    (0x07221E, i_07221e),
    (0x072222, i_072222),
    (0x072224, i_072224),
    (0x07222A, i_07222a),
    (0x07222E, i_07222e),
    (0x072232, i_072232),
    (0x072234, i_072234),
    (0x07223A, i_07223a),
    (0x072240, i_072240),
    (0x072246, i_072246),
    (0x07224A, i_07224a),
    (0x07224E, i_07224e),
    (0x072250, i_072250),
    (0x072256, i_072256),
    (0x07225A, i_07225a),
    (0x07225E, i_07225e),
    (0x072260, i_072260),
    (0x072266, i_072266),
    (0x07226C, i_07226c),
    (0x072270, i_072270),
    (0x072274, i_072274),
    (0x072276, i_072276),
    (0x072278, i_072278),
    (0x07227C, i_07227c),
    (0x072282, i_072282),
    (0x072286, i_072286),
    (0x07228C, i_07228c),
    (0x072292, i_072292),
    (0x072298, i_072298),
    (0x07229C, i_07229c),
    (0x0722A0, i_0722a0),
    (0x0722A6, i_0722a6),
    (0x0722AA, i_0722aa),
    (0x0722B0, i_0722b0),
    (0x0722B6, i_0722b6),
    (0x0722BC, i_0722bc),
    (0x0722C0, i_0722c0),
    (0x0722C6, i_0722c6),
    (0x0722CC, i_0722cc),
    (0x0722D2, i_0722d2),
    (0x0722D6, i_0722d6),
    (0x0722D8, i_0722d8),
    (0x0722DE, i_0722de),
    (0x0722E2, i_0722e2),
    (0x0722E6, i_0722e6),
    (0x0722E8, i_0722e8),
    (0x0722EE, i_0722ee),
    (0x0722F4, i_0722f4),
    (0x0722FA, i_0722fa),
    (0x0722FE, i_0722fe),
    (0x072302, i_072302),
    (0x072304, i_072304),
    (0x072306, i_072306),
    (0x07230C, i_07230c),
    (0x072310, i_072310),
    (0x072312, i_072312),
    (0x072318, i_072318),
    (0x07231C, i_07231c),
    (0x07231E, i_07231e),
    (0x072322, i_072322),
    (0x07232A, i_07232a),
    (0x072330, i_072330),
    (0x072334, i_072334),
    (0x072336, i_072336),
    (0x07233A, i_07233a),
    (0x072340, i_072340),
    (0x072344, i_072344),
    (0x07234A, i_07234a),
    (0x072350, i_072350),
    (0x072356, i_072356),
    (0x07235E, i_07235e),
    (0x072364, i_072364),
    (0x072368, i_072368),
    (0x07236E, i_07236e),
    (0x072374, i_072374),
    (0x07237A, i_07237a),
    (0x072382, i_072382),
    (0x072388, i_072388),
    (0x07238C, i_07238c),
    (0x072392, i_072392),
    (0x072398, i_072398),
    (0x07239E, i_07239e),
    (0x0723A2, i_0723a2),
    (0x0723A8, i_0723a8),
    (0x0723AE, i_0723ae),
    (0x0723B4, i_0723b4),
    (0x0723B8, i_0723b8),
    (0x0723BA, i_0723ba),
    (0x0723C0, i_0723c0),
    (0x0723C6, i_0723c6),
    (0x0723CA, i_0723ca),
    (0x0723CE, i_0723ce),
    (0x0723D0, i_0723d0),
    (0x0723D2, i_0723d2),
    (0x0723D6, i_0723d6),
    (0x0723E0, i_0723e0),
    (0x0723E4, i_0723e4),
    (0x0723E8, i_0723e8),
    (0x0723EA, i_0723ea),
    (0x0723EE, i_0723ee),
    (0x0723F4, i_0723f4),
    (0x0723FA, i_0723fa),
    (0x072400, i_072400),
    (0x072404, i_072404),
    (0x072406, i_072406),
    (0x07240A, i_07240a),
    (0x07240E, i_07240e),
    (0x072412, i_072412),
    (0x072414, i_072414),
    (0x07241A, i_07241a),
    (0x07241E, i_07241e),
    (0x072422, i_072422),
    (0x072424, i_072424),
    (0x07242A, i_07242a),
    (0x07242E, i_07242e),
    (0x072432, i_072432),
    (0x072434, i_072434),
    (0x072436, i_072436),
    (0x07243A, i_07243a),
    (0x07243C, i_07243c),
    (0x072440, i_072440),
    (0x072444, i_072444),
    (0x072446, i_072446),
    (0x07244A, i_07244a),
    (0x07244E, i_07244e),
    (0x072452, i_072452),
    (0x072454, i_072454),
    (0x072458, i_072458),
    (0x07245C, i_07245c),
    (0x072460, i_072460),
    (0x072464, i_072464),
    (0x072468, i_072468),
    (0x07246E, i_07246e),
    (0x072472, i_072472),
    (0x072476, i_072476),
    (0x072478, i_072478),
    (0x07247E, i_07247e),
    (0x072482, i_072482),
    (0x072486, i_072486),
    (0x072488, i_072488),
    (0x07248A, i_07248a),
    (0x07248E, i_07248e),
    (0x072492, i_072492),
    (0x072496, i_072496),
    (0x07249A, i_07249a),
    (0x07249E, i_07249e),
    (0x0724A2, i_0724a2),
    (0x0724A6, i_0724a6),
    (0x0724AA, i_0724aa),
    (0x0724AE, i_0724ae),
    (0x0724B2, i_0724b2),
    (0x0724B8, i_0724b8),
    (0x0724BC, i_0724bc),
    (0x0724C0, i_0724c0),
    (0x0724C2, i_0724c2),
    (0x0724C8, i_0724c8),
    (0x0724CC, i_0724cc),
    (0x0724D0, i_0724d0),
    (0x0724D2, i_0724d2),
    (0x0724D4, i_0724d4),
    (0x0724DC, i_0724dc),
    (0x0724E0, i_0724e0),
    (0x0724E6, i_0724e6),
    (0x0724EC, i_0724ec),
    (0x0724F2, i_0724f2),
    (0x0724F6, i_0724f6),
    (0x0724FA, i_0724fa),
    (0x0724FE, i_0724fe),
    (0x072502, i_072502),
    (0x072506, i_072506),
    (0x07250A, i_07250a),
    (0x07250E, i_07250e),
    (0x072514, i_072514),
    (0x07251A, i_07251a),
    (0x07251E, i_07251e),
    (0x072524, i_072524),
    (0x07252A, i_07252a),
    (0x07252E, i_07252e),
    (0x072534, i_072534),
    (0x072536, i_072536),
    (0x07253A, i_07253a),
    (0x07253E, i_07253e),
    (0x072540, i_072540),
    (0x072544, i_072544),
    (0x07254A, i_07254a),
    (0x072550, i_072550),
    (0x072554, i_072554),
    (0x072558, i_072558),
    (0x07255E, i_07255e),
    (0x072564, i_072564),
    (0x07256A, i_07256a),
    (0x072570, i_072570),
    (0x072574, i_072574),
    (0x072578, i_072578),
    (0x07257E, i_07257e),
    (0x072582, i_072582),
    (0x072586, i_072586),
    (0x07258A, i_07258a),
    (0x07258C, i_07258c),
    (0x07258E, i_07258e),
    (0x072590, i_072590),
    (0x072594, i_072594),
    (0x07259A, i_07259a),
    (0x07259E, i_07259e),
    (0x0725A0, i_0725a0),
    (0x0725A6, i_0725a6),
    (0x0725AA, i_0725aa),
    (0x0725AE, i_0725ae),
    (0x0725B2, i_0725b2),
    (0x0725B4, i_0725b4),
    (0x0725B6, i_0725b6),
    (0x0725B8, i_0725b8),
    (0x0725BC, i_0725bc),
    (0x0725C2, i_0725c2),
    (0x0725C8, i_0725c8),
    (0x0725CA, i_0725ca),
    (0x0725D0, i_0725d0),
    (0x0725D4, i_0725d4),
    (0x0725D8, i_0725d8),
    (0x0725DC, i_0725dc),
    (0x0725E2, i_0725e2),
    (0x0725E6, i_0725e6),
    (0x0725EA, i_0725ea),
    (0x0725EE, i_0725ee),
    (0x0725F2, i_0725f2),
    (0x0725F6, i_0725f6),
    (0x0725FC, i_0725fc),
    (0x072602, i_072602),
    (0x072606, i_072606),
    (0x07260C, i_07260c),
    (0x072612, i_072612),
    (0x072616, i_072616),
    (0x07261C, i_07261c),
    (0x072620, i_072620),
    (0x072626, i_072626),
    (0x07262A, i_07262a),
    (0x072630, i_072630),
    (0x072636, i_072636),
    (0x07263A, i_07263a),
    (0x07263E, i_07263e),
    (0x072642, i_072642),
    (0x072644, i_072644),
    (0x072646, i_072646),
    (0x07264A, i_07264a),
    (0x07264E, i_07264e),
    (0x072654, i_072654),
    (0x072658, i_072658),
    (0x07265E, i_07265e),
    (0x072662, i_072662),
    (0x072664, i_072664),
    (0x07266A, i_07266a),
    (0x07266C, i_07266c),
    (0x072670, i_072670),
    (0x072674, i_072674),
    (0x072678, i_072678),
    (0x07267A, i_07267a),
    (0x07267E, i_07267e),
    (0x072682, i_072682),
    (0x072686, i_072686),
    (0x072688, i_072688),
    (0x07268E, i_07268e),
    (0x072692, i_072692),
    (0x072694, i_072694),
    (0x072698, i_072698),
    (0x07269E, i_07269e),
    (0x0726A4, i_0726a4),
    (0x0726A8, i_0726a8),
    (0x0726AC, i_0726ac),
    (0x0726AE, i_0726ae),
    (0x0726B2, i_0726b2),
    (0x0726B6, i_0726b6),
    (0x0726BA, i_0726ba),
    (0x0726BC, i_0726bc),
    (0x0726C2, i_0726c2),
    (0x0726C8, i_0726c8),
    (0x0726CC, i_0726cc),
    (0x0726D0, i_0726d0),
    (0x0726D2, i_0726d2),
    (0x0726D4, i_0726d4),
    (0x0726DA, i_0726da),
    (0x0726DE, i_0726de),
    (0x0726E4, i_0726e4),
    (0x0726E8, i_0726e8),
    (0x0726EC, i_0726ec),
    (0x0726F2, i_0726f2),
    (0x0726F6, i_0726f6),
    (0x0726FA, i_0726fa),
    (0x0726FC, i_0726fc),
    (0x072700, i_072700),
    (0x072702, i_072702),
    (0x072708, i_072708),
    (0x07270C, i_07270c),
    (0x072710, i_072710),
    (0x072714, i_072714),
    (0x07271A, i_07271a),
    (0x07271E, i_07271e),
    (0x072722, i_072722),
    (0x072726, i_072726),
    (0x07272C, i_07272c),
    (0x072730, i_072730),
    (0x072734, i_072734),
    (0x07273A, i_07273a),
    (0x072740, i_072740),
    (0x072744, i_072744),
    (0x072748, i_072748),
    (0x07274A, i_07274a),
    (0x07274E, i_07274e),
    (0x072750, i_072750),
    (0x072756, i_072756),
    (0x07275A, i_07275a),
    (0x07275E, i_07275e),
    (0x072764, i_072764),
    (0x072768, i_072768),
    (0x07276C, i_07276c),
    (0x072772, i_072772),
    (0x072776, i_072776),
    (0x07277A, i_07277a),
    (0x07277C, i_07277c),
    (0x072780, i_072780),
    (0x072782, i_072782),
    (0x072788, i_072788),
    (0x07278C, i_07278c),
    (0x07278E, i_07278e),
    (0x072792, i_072792),
    (0x072794, i_072794),
    (0x072798, i_072798),
    (0x07279A, i_07279a),
    (0x0727A0, i_0727a0),
    (0x0727A4, i_0727a4),
    (0x0727A8, i_0727a8),
    (0x0727AE, i_0727ae),
    (0x0727B2, i_0727b2),
    (0x0727B4, i_0727b4),
    (0x0727B8, i_0727b8),
    (0x0727BC, i_0727bc),
    (0x0727C2, i_0727c2),
    (0x0727C4, i_0727c4),
    (0x0727CA, i_0727ca),
    (0x0727CE, i_0727ce),
    (0x0727D2, i_0727d2),
    (0x0727D8, i_0727d8),
    (0x0727DC, i_0727dc),
    (0x0727DE, i_0727de),
    (0x0727E2, i_0727e2),
    (0x0727E8, i_0727e8),
    (0x0727EA, i_0727ea),
    (0x0727EE, i_0727ee),
    (0x0727F4, i_0727f4),
    (0x0727F8, i_0727f8),
    (0x0727FE, i_0727fe),
    (0x072802, i_072802),
    (0x072806, i_072806),
    (0x07280A, i_07280a),
    (0x072810, i_072810),
    (0x072814, i_072814),
    (0x07281A, i_07281a),
    (0x07281E, i_07281e),
    (0x072822, i_072822),
    (0x072826, i_072826),
    (0x07282A, i_07282a),
    (0x07282C, i_07282c),
    (0x072832, i_072832),
    (0x072836, i_072836),
    (0x07283C, i_07283c),
    (0x072840, i_072840),
    (0x072846, i_072846),
    (0x07284C, i_07284c),
    (0x072850, i_072850),
    (0x072856, i_072856),
    (0x07285C, i_07285c),
    (0x072860, i_072860),
    (0x072864, i_072864),
    (0x07286A, i_07286a),
    (0x072870, i_072870),
    (0x072876, i_072876),
    (0x07287C, i_07287c),
    (0x072882, i_072882),
    (0x072888, i_072888),
    (0x07288A, i_07288a),
    (0x072890, i_072890),
    (0x072896, i_072896),
    (0x07289C, i_07289c),
    (0x0728A0, i_0728a0),
    (0x0728A4, i_0728a4),
    (0x0728AA, i_0728aa),
    (0x0728B0, i_0728b0),
    (0x0728B6, i_0728b6),
    (0x0728BC, i_0728bc),
    (0x0728C2, i_0728c2),
    (0x0728C8, i_0728c8),
    (0x0728CA, i_0728ca),
    (0x0728D0, i_0728d0),
    (0x0728D4, i_0728d4),
    (0x0728DA, i_0728da),
    (0x0728E0, i_0728e0),
    (0x0728E6, i_0728e6),
    (0x0728EC, i_0728ec),
    (0x0728F2, i_0728f2),
    (0x0728F8, i_0728f8),
    (0x0728FC, i_0728fc),
    (0x072902, i_072902),
    (0x072908, i_072908),
    (0x07290E, i_07290e),
    (0x072914, i_072914),
    (0x07291A, i_07291a),
    (0x072920, i_072920),
    (0x072926, i_072926),
    (0x07292A, i_07292a),
    (0x072930, i_072930),
    (0x072934, i_072934),
    (0x072938, i_072938),
    (0x07293C, i_07293c),
    (0x072940, i_072940),
    (0x072946, i_072946),
    (0x07294A, i_07294a),
    (0x072950, i_072950),
    (0x072952, i_072952),
    (0x072958, i_072958),
    (0x07295E, i_07295e),
    (0x072964, i_072964),
    (0x072968, i_072968),
    (0x07296E, i_07296e),
    (0x072972, i_072972),
    (0x072976, i_072976),
    (0x07297C, i_07297c),
    (0x072982, i_072982),
    (0x072988, i_072988),
    (0x07298E, i_07298e),
    (0x072994, i_072994),
    (0x072996, i_072996),
    (0x07299C, i_07299c),
    (0x0729A0, i_0729a0),
    (0x0729A6, i_0729a6),
    (0x0729AC, i_0729ac),
    (0x0729B0, i_0729b0),
    (0x0729B6, i_0729b6),
    (0x0729BC, i_0729bc),
    (0x0729C0, i_0729c0),
    (0x0729C4, i_0729c4),
    (0x0729CA, i_0729ca),
    (0x0729D0, i_0729d0),
    (0x0729D6, i_0729d6),
    (0x0729DC, i_0729dc),
    (0x0729E2, i_0729e2),
    (0x0729E8, i_0729e8),
    (0x0729EA, i_0729ea),
    (0x0729F0, i_0729f0),
    (0x0729F4, i_0729f4),
    (0x0729FA, i_0729fa),
    (0x072A00, i_072a00),
    (0x072A06, i_072a06),
    (0x072A0C, i_072a0c),
    (0x072A12, i_072a12),
    (0x072A18, i_072a18),
    (0x072A1E, i_072a1e),
    (0x072A22, i_072a22),
    (0x072A26, i_072a26),
    (0x072A2C, i_072a2c),
    (0x072A2E, i_072a2e),
    (0x072A34, i_072a34),
    (0x072A3A, i_072a3a),
    (0x072A3E, i_072a3e),
    (0x072A42, i_072a42),
    (0x072A46, i_072a46),
    (0x072A4C, i_072a4c),
    (0x072A50, i_072a50),
    (0x072A56, i_072a56),
    (0x072A5C, i_072a5c),
    (0x072A60, i_072a60),
    (0x072A66, i_072a66),
    (0x072A6A, i_072a6a),
    (0x072A6E, i_072a6e),
    (0x072A74, i_072a74),
    (0x072A7A, i_072a7a),
    (0x072A80, i_072a80),
    (0x072A86, i_072a86),
    (0x072A8C, i_072a8c),
    (0x072A92, i_072a92),
    (0x072A94, i_072a94),
    (0x072A9A, i_072a9a),
    (0x072A9E, i_072a9e),
    (0x072AA4, i_072aa4),
    (0x072AAA, i_072aaa),
    (0x072AAC, i_072aac),
    (0x072AB0, i_072ab0),
    (0x072AB4, i_072ab4),
    (0x072ABA, i_072aba),
    (0x072AC0, i_072ac0),
    (0x072AC6, i_072ac6),
    (0x072ACC, i_072acc),
    (0x072AD2, i_072ad2),
    (0x072AD8, i_072ad8),
    (0x072ADA, i_072ada),
    (0x072AE0, i_072ae0),
    (0x072AE4, i_072ae4),
    (0x072AEA, i_072aea),
    (0x072AF0, i_072af0),
    (0x072AF6, i_072af6),
    (0x072AFC, i_072afc),
    (0x072B02, i_072b02),
    (0x072B08, i_072b08),
    (0x072B0E, i_072b0e),
    (0x072B10, i_072b10),
    (0x072B12, i_072b12),
    (0x072B16, i_072b16),
    (0x072B1C, i_072b1c),
    (0x072B1E, i_072b1e),
    (0x072B24, i_072b24),
    (0x072B2A, i_072b2a),
    (0x072B2C, i_072b2c),
    (0x072B30, i_072b30),
    (0x072B34, i_072b34),
    (0x072B3A, i_072b3a),
    (0x072B40, i_072b40),
    (0x072B46, i_072b46),
    (0x072B4C, i_072b4c),
    (0x072B52, i_072b52),
    (0x072B58, i_072b58),
    (0x072B5A, i_072b5a),
    (0x072B5E, i_072b5e),
    (0x072B64, i_072b64),
    (0x072B68, i_072b68),
    (0x072B6E, i_072b6e),
    (0x072B74, i_072b74),
    (0x072B78, i_072b78),
    (0x072B7E, i_072b7e),
    (0x072B82, i_072b82),
    (0x072B86, i_072b86),
    (0x072B8A, i_072b8a),
    (0x072B8E, i_072b8e),
    (0x072B90, i_072b90),
    (0x072B94, i_072b94),
    (0x072B96, i_072b96),
    (0x072B9A, i_072b9a),
    (0x072B9E, i_072b9e),
    (0x072BA4, i_072ba4),
    (0x072BA8, i_072ba8),
    (0x072BAC, i_072bac),
    (0x072BB2, i_072bb2),
    (0x072BB6, i_072bb6),
    (0x072BBA, i_072bba),
    (0x072BC0, i_072bc0),
    (0x072BC6, i_072bc6),
    (0x072BCC, i_072bcc),
    (0x072BCE, i_072bce),
    (0x072BD4, i_072bd4),
    (0x072BD8, i_072bd8),
    (0x072BDC, i_072bdc),
    (0x072BDE, i_072bde),
    (0x072BE2, i_072be2),
    (0x072BE6, i_072be6),
    (0x072BEA, i_072bea),
    (0x072BEC, i_072bec),
    (0x072BF0, i_072bf0),
    (0x072BF2, i_072bf2),
    (0x072BF6, i_072bf6),
    (0x072BFA, i_072bfa),
    (0x072BFE, i_072bfe),
    (0x072C02, i_072c02),
    (0x072C06, i_072c06),
    (0x072C08, i_072c08),
    (0x072C0C, i_072c0c),
    (0x072C10, i_072c10),
    (0x072C14, i_072c14),
    (0x072C18, i_072c18),
    (0x072C1C, i_072c1c),
    (0x072C20, i_072c20),
    (0x072C22, i_072c22),
    (0x072C26, i_072c26),
    (0x072C2C, i_072c2c),
    (0x072C30, i_072c30),
    (0x072C32, i_072c32),
    (0x072C36, i_072c36),
    (0x072C38, i_072c38),
    (0x072C3E, i_072c3e),
    (0x072C42, i_072c42),
    (0x072C44, i_072c44),
    (0x072C46, i_072c46),
    (0x072C4C, i_072c4c),
    (0x072C50, i_072c50),
    (0x072C52, i_072c52),
    (0x072C56, i_072c56),
    (0x072C5A, i_072c5a),
    (0x072C60, i_072c60),
    (0x072C64, i_072c64),
    (0x072C68, i_072c68),
    (0x072C6C, i_072c6c),
    (0x072C70, i_072c70),
    (0x072C76, i_072c76),
    (0x072C7A, i_072c7a),
    (0x072C7C, i_072c7c),
    (0x072C80, i_072c80),
    (0x072C84, i_072c84),
    (0x072C8A, i_072c8a),
    (0x072C8E, i_072c8e),
    (0x072C92, i_072c92),
    (0x072C96, i_072c96),
    (0x072C98, i_072c98),
    (0x072C9A, i_072c9a),
    (0x072CA0, i_072ca0),
    (0x072CA4, i_072ca4),
    (0x072CA8, i_072ca8),
    (0x072CAA, i_072caa),
    (0x072CAE, i_072cae),
    (0x072CB4, i_072cb4),
    (0x072CB8, i_072cb8),
    (0x072CBA, i_072cba),
    (0x072CBC, i_072cbc),
    (0x072CBE, i_072cbe),
    (0x072CC2, i_072cc2),
    (0x072CC4, i_072cc4),
    (0x072CC6, i_072cc6),
    (0x072CCA, i_072cca),
    (0x072CCC, i_072ccc),
    (0x072CD0, i_072cd0),
    (0x072CD2, i_072cd2),
    (0x072CD6, i_072cd6),
    (0x072CDA, i_072cda),
    (0x072CDE, i_072cde),
    (0x072CE2, i_072ce2),
    (0x072CE4, i_072ce4),
    (0x072CE6, i_072ce6),
    (0x072CEA, i_072cea),
    (0x072CEC, i_072cec),
    (0x072CF0, i_072cf0),
    (0x072CF2, i_072cf2),
    (0x072CF6, i_072cf6),
    (0x072CF8, i_072cf8),
    (0x072CFC, i_072cfc),
    (0x072D00, i_072d00),
    (0x072D06, i_072d06),
    (0x072D0A, i_072d0a),
    (0x072D0C, i_072d0c),
    (0x072D0E, i_072d0e),
    (0x072D12, i_072d12),
    (0x072D16, i_072d16),
    (0x072D18, i_072d18),
    (0x072D1A, i_072d1a),
    (0x072D1E, i_072d1e),
    (0x072D22, i_072d22),
    (0x072D26, i_072d26),
    (0x072D28, i_072d28),
    (0x072D2C, i_072d2c),
    (0x072D32, i_072d32),
    (0x072D38, i_072d38),
    (0x072D3C, i_072d3c),
    (0x072D40, i_072d40),
    (0x072D44, i_072d44),
    (0x072D4A, i_072d4a),
    (0x072D4E, i_072d4e),
    (0x072D54, i_072d54),
    (0x072D5A, i_072d5a),
    (0x072D5E, i_072d5e),
    (0x072D62, i_072d62),
    (0x072D66, i_072d66),
    (0x072D6C, i_072d6c),
    (0x072D70, i_072d70),
    (0x072D76, i_072d76),
    (0x072D78, i_072d78),
    (0x072D7C, i_072d7c),
    (0x072D82, i_072d82),
    (0x072D88, i_072d88),
    (0x072D8C, i_072d8c),
    (0x072D92, i_072d92),
    (0x072D98, i_072d98),
    (0x072D9C, i_072d9c),
    (0x072DA2, i_072da2),
    (0x072DA4, i_072da4),
    (0x072DAA, i_072daa),
    (0x072DB0, i_072db0),
    (0x072DB6, i_072db6),
    (0x072DB8, i_072db8),
    (0x072DBE, i_072dbe),
    (0x072DC2, i_072dc2),
    (0x072DC6, i_072dc6),
    (0x072DCC, i_072dcc),
    (0x072DD2, i_072dd2),
    (0x072DD8, i_072dd8),
    (0x072DDE, i_072dde),
    (0x072DE2, i_072de2),
    (0x072DE4, i_072de4),
    (0x072DE6, i_072de6),
    (0x072DE8, i_072de8),
    (0x072DEA, i_072dea),
    (0x072DF2, i_072df2),
    (0x072DF8, i_072df8),
    (0x072DFE, i_072dfe),
    (0x072E06, i_072e06),
    (0x072E0C, i_072e0c),
    (0x072E12, i_072e12),
    (0x072E14, i_072e14),
    (0x072E18, i_072e18),
    (0x072E1E, i_072e1e),
    (0x072E24, i_072e24),
    (0x072E28, i_072e28),
    (0x072E2C, i_072e2c),
    (0x072E30, i_072e30),
    (0x072E36, i_072e36),
    (0x072E3C, i_072e3c),
    (0x072E40, i_072e40),
    (0x072E44, i_072e44),
    (0x072E4A, i_072e4a),
    (0x072E50, i_072e50),
    (0x072E54, i_072e54),
    (0x072E58, i_072e58),
    (0x072E5C, i_072e5c),
    (0x072E62, i_072e62),
    (0x072E66, i_072e66),
    (0x072E6A, i_072e6a),
    (0x072E6E, i_072e6e),
    (0x072E74, i_072e74),
    (0x072E78, i_072e78),
    (0x072E7E, i_072e7e),
    (0x072E80, i_072e80),
    (0x072E84, i_072e84),
    (0x072E8A, i_072e8a),
    (0x072E90, i_072e90),
    (0x072E94, i_072e94),
    (0x072E98, i_072e98),
    (0x072E9C, i_072e9c),
    (0x072EA2, i_072ea2),
    (0x072EA8, i_072ea8),
    (0x072EAC, i_072eac),
    (0x072EB0, i_072eb0),
    (0x072EB6, i_072eb6),
    (0x072EBC, i_072ebc),
    (0x072EC0, i_072ec0),
    (0x072EC4, i_072ec4),
    (0x072EC6, i_072ec6),
    (0x072ECA, i_072eca),
    (0x072ECE, i_072ece),
    (0x072ED2, i_072ed2),
    (0x072ED6, i_072ed6),
    (0x072EDC, i_072edc),
    (0x072EE0, i_072ee0),
    (0x072EE6, i_072ee6),
    (0x072EE8, i_072ee8),
    (0x072EEE, i_072eee),
    (0x072EF4, i_072ef4),
    (0x072EFA, i_072efa),
    (0x072F00, i_072f00),
    (0x072F04, i_072f04),
    (0x072F08, i_072f08),
    (0x072F0C, i_072f0c),
    (0x072F12, i_072f12),
    (0x072F16, i_072f16),
    (0x072F1C, i_072f1c),
    (0x072F22, i_072f22),
    (0x072F26, i_072f26),
    (0x072F2A, i_072f2a),
    (0x072F2E, i_072f2e),
    (0x072F34, i_072f34),
    (0x072F38, i_072f38),
    (0x072F3E, i_072f3e),
    (0x072F40, i_072f40),
    (0x072F44, i_072f44),
    (0x072F4A, i_072f4a),
    (0x072F50, i_072f50),
    (0x072F54, i_072f54),
    (0x072F58, i_072f58),
    (0x072F5C, i_072f5c),
    (0x072F62, i_072f62),
    (0x072F64, i_072f64),
    (0x072F68, i_072f68),
    (0x072F6E, i_072f6e),
    (0x072F74, i_072f74),
    (0x072F78, i_072f78),
    (0x072F7C, i_072f7c),
    (0x072F80, i_072f80),
    (0x072F86, i_072f86),
    (0x072F88, i_072f88),
    (0x072F8C, i_072f8c),
    (0x072F92, i_072f92),
    (0x072F98, i_072f98),
    (0x072F9C, i_072f9c),
    (0x072FA0, i_072fa0),
    (0x072FA4, i_072fa4),
    (0x072FAA, i_072faa),
    (0x072FAC, i_072fac),
    (0x072FB0, i_072fb0),
    (0x072FB4, i_072fb4),
    (0x072FB8, i_072fb8),
    (0x072FBC, i_072fbc),
    (0x072FC0, i_072fc0),
    (0x072FC2, i_072fc2),
    (0x072FC6, i_072fc6),
    (0x072FC8, i_072fc8),
    (0x072FCC, i_072fcc),
    (0x072FD2, i_072fd2),
    (0x072FD6, i_072fd6),
    (0x072FDC, i_072fdc),
    (0x072FE2, i_072fe2),
    (0x072FE8, i_072fe8),
    (0x072FF0, i_072ff0),
    (0x072FF8, i_072ff8),
    (0x073000, i_073000),
    (0x073004, i_073004),
    (0x07300A, i_07300a),
    (0x073010, i_073010),
    (0x073016, i_073016),
    (0x07301C, i_07301c),
    (0x073022, i_073022),
    (0x073026, i_073026),
    (0x07302C, i_07302c),
    (0x073032, i_073032),
    (0x073036, i_073036),
    (0x07303C, i_07303c),
    (0x073042, i_073042),
    (0x073048, i_073048),
    (0x07304E, i_07304e),
    (0x073054, i_073054),
    (0x073058, i_073058),
    (0x07305E, i_07305e),
    (0x073064, i_073064),
    (0x073068, i_073068),
    (0x07306C, i_07306c),
    (0x073072, i_073072),
    (0x073078, i_073078),
    (0x07307E, i_07307e),
    (0x073082, i_073082),
    (0x073088, i_073088),
    (0x07308E, i_07308e),
    (0x073094, i_073094),
    (0x073098, i_073098),
    (0x07309E, i_07309e),
    (0x0730A4, i_0730a4),
    (0x0730A8, i_0730a8),
    (0x0730AC, i_0730ac),
    (0x0730B2, i_0730b2),
    (0x0730B6, i_0730b6),
    (0x0730BC, i_0730bc),
    (0x0730C0, i_0730c0),
    (0x0730C4, i_0730c4),
    (0x0730C8, i_0730c8),
    (0x0730CC, i_0730cc),
    (0x0730D0, i_0730d0),
    (0x0730D4, i_0730d4),
    (0x0730DA, i_0730da),
    (0x0730DE, i_0730de),
    (0x0730E0, i_0730e0),
    (0x0730E2, i_0730e2),
    (0x0730E6, i_0730e6),
    (0x0730EA, i_0730ea),
    (0x0730EE, i_0730ee),
    (0x0730F0, i_0730f0),
    (0x0730F6, i_0730f6),
    (0x0730F8, i_0730f8),
    (0x0730FC, i_0730fc),
    (0x073102, i_073102),
    (0x073106, i_073106),
    (0x07310C, i_07310c),
    (0x073114, i_073114),
    (0x073118, i_073118),
    (0x07311A, i_07311a),
    (0x073120, i_073120),
    (0x073126, i_073126),
    (0x07312A, i_07312a),
    (0x07312E, i_07312e),
    (0x073130, i_073130),
    (0x073134, i_073134),
    (0x073138, i_073138),
    (0x07313C, i_07313c),
    (0x07313E, i_07313e),
    (0x073144, i_073144),
    (0x073148, i_073148),
    (0x07314E, i_07314e),
    (0x073152, i_073152),
    (0x073158, i_073158),
    (0x07315C, i_07315c),
    (0x073160, i_073160),
    (0x073166, i_073166),
    (0x073168, i_073168),
    (0x07316C, i_07316c),
    (0x073172, i_073172),
    (0x073176, i_073176),
    (0x07317C, i_07317c),
    (0x073184, i_073184),
    (0x073188, i_073188),
    (0x07318A, i_07318a),
    (0x073190, i_073190),
    (0x073196, i_073196),
    (0x07319C, i_07319c),
    (0x0731A0, i_0731a0),
    (0x0731A6, i_0731a6),
    (0x0731AA, i_0731aa),
    (0x0731AE, i_0731ae),
    (0x0731B0, i_0731b0),
    (0x0731B4, i_0731b4),
    (0x0731B8, i_0731b8),
    (0x0731BC, i_0731bc),
    (0x0731C2, i_0731c2),
    (0x0731C6, i_0731c6),
    (0x0731C8, i_0731c8),
    (0x0731CC, i_0731cc),
    (0x0731CE, i_0731ce),
    (0x0731D2, i_0731d2),
    (0x0731D8, i_0731d8),
    (0x0731DC, i_0731dc),
    (0x0731E0, i_0731e0),
    (0x0731E2, i_0731e2),
    (0x0731E4, i_0731e4),
    (0x0731E8, i_0731e8),
    (0x0731EC, i_0731ec),
    (0x0731F0, i_0731f0),
    (0x0731F4, i_0731f4),
    (0x0731F6, i_0731f6),
    (0x0731FC, i_0731fc),
    (0x0731FE, i_0731fe),
    (0x073202, i_073202),
    (0x073208, i_073208),
    (0x07320C, i_07320c),
    (0x073212, i_073212),
    (0x07321A, i_07321a),
    (0x073220, i_073220),
    (0x073224, i_073224),
    (0x073226, i_073226),
    (0x07322C, i_07322c),
    (0x073232, i_073232),
    (0x073236, i_073236),
    (0x07323A, i_07323a),
    (0x07323E, i_07323e),
    (0x073240, i_073240),
    (0x073246, i_073246),
    (0x07324A, i_07324a),
    (0x073250, i_073250),
    (0x073254, i_073254),
    (0x073258, i_073258),
    (0x07325A, i_07325a),
    (0x07325E, i_07325e),
    (0x073264, i_073264),
    (0x07326A, i_07326a),
    (0x073272, i_073272),
    (0x073276, i_073276),
    (0x073278, i_073278),
    (0x07327C, i_07327c),
    (0x073282, i_073282),
    (0x073286, i_073286),
    (0x07328A, i_07328a),
    (0x07328C, i_07328c),
    (0x073290, i_073290),
    (0x073294, i_073294),
    (0x07329A, i_07329a),
    (0x0732A0, i_0732a0),
    (0x0732A6, i_0732a6),
    (0x0732AA, i_0732aa),
    (0x0732AC, i_0732ac),
    (0x0732B2, i_0732b2),
    (0x0732B8, i_0732b8),
    (0x0732BE, i_0732be),
    (0x0732C2, i_0732c2),
    (0x0732C6, i_0732c6),
    (0x0732C8, i_0732c8),
    (0x0732CE, i_0732ce),
    (0x0732D2, i_0732d2),
    (0x0732D8, i_0732d8),
    (0x0732DE, i_0732de),
    (0x0732E4, i_0732e4),
    (0x0732E8, i_0732e8),
    (0x0732EE, i_0732ee),
    (0x0732F4, i_0732f4),
    (0x0732F8, i_0732f8),
    (0x0732FC, i_0732fc),
    (0x073300, i_073300),
    (0x073306, i_073306),
    (0x07330C, i_07330c),
    (0x073310, i_073310),
    (0x073312, i_073312),
    (0x073318, i_073318),
    (0x07331E, i_07331e),
    (0x073324, i_073324),
    (0x07332A, i_07332a),
    (0x07332E, i_07332e),
    (0x073334, i_073334),
    (0x07333A, i_07333a),
    (0x073340, i_073340),
    (0x073344, i_073344),
    (0x07334A, i_07334a),
    (0x07334E, i_07334e),
    (0x073354, i_073354),
    (0x07335A, i_07335a),
    (0x07335E, i_07335e),
    (0x073364, i_073364),
    (0x073368, i_073368),
    (0x07336C, i_07336c),
    (0x07336E, i_07336e),
    (0x073372, i_073372),
    (0x073378, i_073378),
    (0x07337E, i_07337e),
    (0x073382, i_073382),
    (0x073388, i_073388),
    (0x07338C, i_07338c),
    (0x073392, i_073392),
    (0x073398, i_073398),
    (0x07339E, i_07339e),
    (0x0733A2, i_0733a2),
    (0x0733A6, i_0733a6),
    (0x0733AA, i_0733aa),
    (0x0733AE, i_0733ae),
    (0x0733B2, i_0733b2),
    (0x0733B8, i_0733b8),
    (0x0733BE, i_0733be),
    (0x0733C4, i_0733c4),
    (0x0733C8, i_0733c8),
    (0x0733CE, i_0733ce),
    (0x0733D2, i_0733d2),
    (0x0733D4, i_0733d4),
    (0x0733DA, i_0733da),
    (0x0733DE, i_0733de),
    (0x0733E4, i_0733e4),
    (0x0733EA, i_0733ea),
    (0x0733F0, i_0733f0),
    (0x0733F6, i_0733f6),
    (0x0733FC, i_0733fc),
    (0x073402, i_073402),
    (0x073408, i_073408),
    (0x07340A, i_07340a),
    (0x07340E, i_07340e),
    (0x073414, i_073414),
    (0x073418, i_073418),
    (0x07341E, i_07341e),
    (0x073424, i_073424),
    (0x07342A, i_07342a),
    (0x073430, i_073430),
    (0x073434, i_073434),
    (0x073436, i_073436),
    (0x07343C, i_07343c),
    (0x073442, i_073442),
    (0x073446, i_073446),
    (0x07344C, i_07344c),
    (0x073450, i_073450),
    (0x073452, i_073452),
    (0x073454, i_073454),
    (0x07345A, i_07345a),
    (0x073460, i_073460),
    (0x073464, i_073464),
    (0x073466, i_073466),
    (0x07346C, i_07346c),
    (0x073472, i_073472),
    (0x073476, i_073476),
    (0x07347A, i_07347a),
    (0x07347C, i_07347c),
    (0x073482, i_073482),
    (0x073484, i_073484),
    (0x07348A, i_07348a),
    (0x07348E, i_07348e),
    (0x073494, i_073494),
    (0x07349A, i_07349a),
    (0x0734A0, i_0734a0),
    (0x0734A6, i_0734a6),
    (0x0734AC, i_0734ac),
    (0x0734B2, i_0734b2),
    (0x0734B8, i_0734b8),
    (0x0734BE, i_0734be),
    (0x0734C4, i_0734c4),
    (0x0734CA, i_0734ca),
    (0x0734D0, i_0734d0),
    (0x0734D6, i_0734d6),
    (0x0734DC, i_0734dc),
    (0x0734E0, i_0734e0),
    (0x0734E6, i_0734e6),
    (0x0734EC, i_0734ec),
    (0x0734F2, i_0734f2),
    (0x0734F4, i_0734f4),
    (0x0734FC, i_0734fc),
    (0x073502, i_073502),
    (0x07350A, i_07350a),
    (0x073512, i_073512),
    (0x07351A, i_07351a),
    (0x073520, i_073520),
    (0x073524, i_073524),
    (0x07352A, i_07352a),
    (0x073530, i_073530),
    (0x073536, i_073536),
    (0x07353C, i_07353c),
    (0x073542, i_073542),
    (0x073546, i_073546),
    (0x073548, i_073548),
    (0x07354C, i_07354c),
    (0x073552, i_073552),
    (0x073558, i_073558),
    (0x07355C, i_07355c),
    (0x073562, i_073562),
    (0x073568, i_073568),
    (0x07356E, i_07356e),
    (0x073574, i_073574),
    (0x073578, i_073578),
    (0x07357C, i_07357c),
    (0x07357E, i_07357e),
    (0x073582, i_073582),
    (0x073586, i_073586),
    (0x07358A, i_07358a),
    (0x07358E, i_07358e),
];
