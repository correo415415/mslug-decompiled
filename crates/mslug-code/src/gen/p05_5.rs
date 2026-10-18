// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0591FE: bcc.w $5920e
pub fn i_0591fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59202;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5920E; return 10; }
    8
}
/// $059202: lea.l $59228(pc), a1
pub fn i_059202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59206;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59228;
    16
}
/// $059206: move.l a1, (a6)
pub fn i_059206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59208;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059208: jsr $8f308.l
pub fn i_059208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5920E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5920E);
    cpu.pc = 0x8F308;
    16
}
/// $05920E: tst.b $78(a6)
pub fn i_05920e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59212;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $059212: bne.w $5921c
pub fn i_059212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59216;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5921C; return 10; }
    8
}
/// $059216: lea.l $592b2(pc), a1
pub fn i_059216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5921A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x592B2;
    16
}
/// $05921A: move.l a1, (a6)
pub fn i_05921a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5921C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05921C: jsr $49fd0.l
pub fn i_05921c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59222);
    cpu.pc = 0x49FD0;
    16
}
/// $059222: bsr.w $56e1e
pub fn i_059222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59226;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x59226);
    cpu.pc = 0x56E1E;
    18
}
/// $059226: rts
pub fn i_059226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59228;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnBrake_059228  ($059228, 50 B, enemies/soldiers/states_057d04.s)
/// $059228: lea.l $2b5cfa.l, a0
pub fn i_059228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5922E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5CFA;
    20
}
/// $05922E: jsr $28cd4.l
pub fn i_05922e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59234;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59234);
    cpu.pc = 0x28CD4;
    16
}
/// $059234: lea.l $5923a(pc), a1
pub fn i_059234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59238;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5923A;
    16
}
/// $059238: move.l a1, (a6)
pub fn i_059238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5923A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05923A: jsr $56acc(pc)
pub fn i_05923a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5923E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5923E);
    cpu.pc = 0x56ACC;
    16
}
/// $05923E: jsr $28d70.l
pub fn i_05923e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59244;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59244);
    cpu.pc = 0x28D70;
    16
}
/// $059244: bcc.w $5924e
pub fn i_059244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59248;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5924E; return 10; }
    8
}
/// $059248: lea.l $59086(pc), a1
pub fn i_059248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5924C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59086;
    16
}
/// $05924C: move.l a1, (a6)
pub fn i_05924c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5924E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05924E: jsr $49fd0.l
pub fn i_05924e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59254;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59254);
    cpu.pc = 0x49FD0;
    16
}
/// $059254: bsr.w $56e1e
pub fn i_059254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59258;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x59258);
    cpu.pc = 0x56E1E;
    18
}
/// $059258: rts
pub fn i_059258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5925A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnLeap_05925a  ($05925A, 88 B, enemies/soldiers/states_057d04.s)
/// $05925A: lea.l $2b5d58.l, a0
pub fn i_05925a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59260;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5D58;
    20
}
/// $059260: jsr $28cd4.l
pub fn i_059260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59266);
    cpu.pc = 0x28CD4;
    16
}
/// $059266: move.w #$fe15, d0
pub fn i_059266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5926A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE15;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05926A: jsr $5dca4.l
pub fn i_05926a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59270);
    cpu.pc = 0x5DCA4;
    16
}
/// $059270: move.w d0, $28(a6)
pub fn i_059270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59274;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059274: move.w #$7aa, $2a(a6)
pub fn i_059274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5927A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05927A: move.w #$ff63, $2e(a6)
pub fn i_05927a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59280;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF63;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059280: move.w #$0, $2c(a6)
pub fn i_059280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59286;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059286: lea.l $5928c(pc), a1
pub fn i_059286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5928A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5928C;
    16
}
/// $05928A: move.l a1, (a6)
pub fn i_05928a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5928C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05928C: jsr $2831e.l
pub fn i_05928c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59292);
    cpu.pc = 0x2831E;
    16
}
/// $059292: scs.b $78(a6)
pub fn i_059292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59296;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $059296: bcc.w $592a0
pub fn i_059296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5929A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x592A0; return 10; }
    8
}
/// $05929A: lea.l $592f6(pc), a1
pub fn i_05929a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5929E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x592F6;
    16
}
/// $05929E: move.l a1, (a6)
pub fn i_05929e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0592A0: jsr $28d70.l
pub fn i_0592a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x592A6);
    cpu.pc = 0x28D70;
    16
}
/// $0592A6: jsr $49fd0.l
pub fn i_0592a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x592AC);
    cpu.pc = 0x49FD0;
    16
}
/// $0592AC: bsr.w $56e1e
pub fn i_0592ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592B0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x592B0);
    cpu.pc = 0x56E1E;
    18
}
/// $0592B0: rts
pub fn i_0592b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnHurt_0592b2  ($0592B2, 68 B, enemies/soldiers/states_057d04.s)
/// $0592B2: lea.l $2b5b92.l, a0
pub fn i_0592b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5B92;
    20
}
/// $0592B8: jsr $28cd4.l
pub fn i_0592b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x592BE);
    cpu.pc = 0x28CD4;
    16
}
/// $0592BE: clr.w $2a(a6)
pub fn i_0592be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592C2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0592C2: lea.l $592c8(pc), a1
pub fn i_0592c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x592C8;
    16
}
/// $0592C6: move.l a1, (a6)
pub fn i_0592c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0592C8: move.w $28(a6), d0
pub fn i_0592c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592CC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0592CC: asr.w #$6, d0
pub fn i_0592cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592CE;
    cpu.ir = 0xEC40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $0592CE: sub.w d0, $28(a6)
pub fn i_0592ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592D2;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0592D2: bsr.w $56acc
pub fn i_0592d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592D6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x592D6);
    cpu.pc = 0x56ACC;
    18
}
/// $0592D6: tst.b $78(a6)
pub fn i_0592d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592DA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0592DA: beq.w $592e4
pub fn i_0592da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x592E4; return 10; }
    8
}
/// $0592DE: lea.l $592f6(pc), a1
pub fn i_0592de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x592F6;
    16
}
/// $0592E2: move.l a1, (a6)
pub fn i_0592e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0592E4: jsr $28d70.l
pub fn i_0592e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x592EA);
    cpu.pc = 0x28D70;
    16
}
/// $0592EA: jsr $49fd0.l
pub fn i_0592ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x592F0);
    cpu.pc = 0x49FD0;
    16
}
/// $0592F0: bsr.w $56e1e
pub fn i_0592f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x592F4);
    cpu.pc = 0x56E1E;
    18
}
/// $0592F4: rts
pub fn i_0592f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SpawnRecover_0592f6  ($0592F6, 60 B, enemies/soldiers/states_057d04.s)
/// $0592F6: bsr.w $5740e
pub fn i_0592f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592FA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x592FA);
    cpu.pc = 0x5740E;
    18
}
/// $0592FA: clr.w $28(a6)
pub fn i_0592fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592FE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0592FE: lea.l $59304(pc), a1
pub fn i_0592fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59302;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59304;
    16
}
/// $059302: move.l a1, (a6)
pub fn i_059302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59304;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059304: jsr $56acc(pc)
pub fn i_059304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59308;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x59308);
    cpu.pc = 0x56ACC;
    16
}
/// $059308: jsr $28d70.l
pub fn i_059308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5930E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5930E);
    cpu.pc = 0x28D70;
    16
}
/// $05930E: bcc.w $59318
pub fn i_05930e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59312;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x59318; return 10; }
    8
}
/// $059312: lea.l $59086(pc), a1
pub fn i_059312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59316;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59086;
    16
}
/// $059316: move.l a1, (a6)
pub fn i_059316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59318;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059318: tst.b $78(a6)
pub fn i_059318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5931C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05931C: bne.w $59326
pub fn i_05931c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59320;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59326; return 10; }
    8
}
/// $059320: lea.l $592b2(pc), a1
pub fn i_059320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59324;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x592B2;
    16
}
/// $059324: move.l a1, (a6)
pub fn i_059324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59326;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059326: jsr $49fd0.l
pub fn i_059326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5932C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5932C);
    cpu.pc = 0x49FD0;
    16
}
/// $05932C: bsr.w $56e1e
pub fn i_05932c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59330;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x59330);
    cpu.pc = 0x56E1E;
    18
}
/// $059330: rts
pub fn i_059330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59332;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpField10WithLink8_059332  ($059332, 16 B, enemies/soldiers/states_057d04.s)
/// $059332: movea.l $8(a6), a1
pub fn i_059332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59336;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $059336: move.b $10(a6), d0
pub fn i_059336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5933A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05933A: cmp.b $10(a1), d0
pub fn i_05933a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5933E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05933E: bcs.w $59348
pub fn i_05933e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59342;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x59348; return 10; }
    8
}
// ===== ClearXN_059342  ($059342, 6 B, thunks/ccr_helpers.c)
/// $059342: andi.b #$ee, ccr
pub fn i_059342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59346;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $059346: rts
pub fn i_059346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59348;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_059348  ($059348, 6 B, thunks/ccr_helpers.c)
/// $059348: ori.b #$11, ccr
pub fn i_059348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5934C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05934C: rts
pub fn i_05934c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5934E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_ShowMission_0593fa  ($0593FA, 56 B, mission/result_ending_gunner_walker_05934e.s)
/// $0593FA: lea.l $4737e.l, a1
pub fn i_0593fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59400;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4737E;
    20
}
/// $059400: jsr $4ae.l
pub fn i_059400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59406;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59406);
    cpu.pc = 0x4AE;
    16
}
/// $059406: move.w #$708f, d0
pub fn i_059406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5940A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x708F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05940A: move.w d0, $22(a0)
pub fn i_05940a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5940E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05940E: move.l #$5936a, $3c(a0)
pub fn i_05940e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59416;
    cpu.ir = 0x217C;
    let v1: u32 = 0x5936A;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059416: move.w #$1, $30(a0)
pub fn i_059416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5941C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05941C: move.b #$4, $16(a0)
pub fn i_05941c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59422;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059422: lea.l $59428(pc), a1
pub fn i_059422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59426;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59428;
    16
}
/// $059426: move.l a1, (a6)
pub fn i_059426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59428;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059428: jsr $6f0.l
pub fn i_059428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5942E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5942E);
    cpu.pc = 0x6F0;
    16
}
/// $05942E: bcs.w $59438
pub fn i_05942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59432;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x59438; return 10; }
    8
}
// ===== SetTaskHandler_059432  ($059432, 8 B, kernel/task_handlers.c)
/// $059432: lea.l $5943a(pc), a1
pub fn i_059432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59436;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5943A;
    16
}
/// $059436: move.l a1, (a6)
pub fn i_059436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59438;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059438: rts
pub fn i_059438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5943A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_ShowAll_05943a  ($05943A, 56 B, mission/result_ending_gunner_walker_05934e.s)
/// $05943A: lea.l $4737e.l, a1
pub fn i_05943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59440;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4737E;
    20
}
/// $059440: jsr $4ae.l
pub fn i_059440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59446;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59446);
    cpu.pc = 0x4AE;
    16
}
/// $059446: move.w #$726f, d0
pub fn i_059446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5944A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x726F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05944A: move.w d0, $22(a0)
pub fn i_05944a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5944E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05944E: move.l #$59362, $3c(a0)
pub fn i_05944e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59456;
    cpu.ir = 0x217C;
    let v1: u32 = 0x59362;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059456: move.w #$1, $30(a0)
pub fn i_059456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5945C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05945C: move.b #$4, $16(a0)
pub fn i_05945c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59462;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059462: lea.l $59468(pc), a1
pub fn i_059462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59466;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59468;
    16
}
/// $059466: move.l a1, (a6)
pub fn i_059466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59468;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059468: jsr $6f0.l
pub fn i_059468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5946E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5946E);
    cpu.pc = 0x6F0;
    16
}
/// $05946E: bcs.w $59478
pub fn i_05946e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59472;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x59478; return 10; }
    8
}
// ===== SetTaskHandler_059472  ($059472, 8 B, kernel/task_handlers.c)
/// $059472: lea.l $5947a(pc), a1
pub fn i_059472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59476;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5947A;
    16
}
/// $059476: move.l a1, (a6)
pub fn i_059476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59478;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059478: rts
pub fn i_059478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5947A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_ShowOver_05947a  ($05947A, 56 B, mission/result_ending_gunner_walker_05934e.s)
/// $05947A: lea.l $4737e.l, a1
pub fn i_05947a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59480;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4737E;
    20
}
/// $059480: jsr $4ae.l
pub fn i_059480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59486);
    cpu.pc = 0x4AE;
    16
}
/// $059486: move.w #$734f, d0
pub fn i_059486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5948A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x734F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05948A: move.w d0, $22(a0)
pub fn i_05948a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5948E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05948E: move.l #$5937a, $3c(a0)
pub fn i_05948e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59496;
    cpu.ir = 0x217C;
    let v1: u32 = 0x5937A;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059496: move.w #$1, $30(a0)
pub fn i_059496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5949C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05949C: move.b #$4, $16(a0)
pub fn i_05949c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594A2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0594A2: lea.l $594a8(pc), a1
pub fn i_0594a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x594A8;
    16
}
/// $0594A6: move.l a1, (a6)
pub fn i_0594a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0594A8: jsr $6f0.l
pub fn i_0594a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x594AE);
    cpu.pc = 0x6F0;
    16
}
/// $0594AE: bcs.w $594b8
pub fn i_0594ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594B2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x594B8; return 10; }
    8
}
// ===== SetTaskHandler_0594b2  ($0594B2, 8 B, kernel/task_handlers.c)
/// $0594B2: lea.l $594ba(pc), a1
pub fn i_0594b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x594BA;
    16
}
/// $0594B6: move.l a1, (a6)
pub fn i_0594b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0594B8: rts
pub fn i_0594b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Exit_0594ba  ($0594BA, 12 B, mission/result_ending_gunner_walker_05934e.s)
/// $0594BA: jsr $5b6.l
pub fn i_0594ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x594C0);
    cpu.pc = 0x5B6;
    16
}
/// $0594C0: jmp $518.l
pub fn i_0594c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594C6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Ending_WipeAllOver_0594c6  ($0594C6, 218 B, mission/result_ending_gunner_walker_05934e.s)
/// $0594C6: move.w #$f, $30(a6)
pub fn i_0594c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0594CC: lea.l $594d2(pc), a1
pub fn i_0594cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x594D2;
    16
}
/// $0594D0: move.l a1, (a6)
pub fn i_0594d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0594D2: move.w #$4, d0
pub fn i_0594d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594D6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0594D6: lsl.w #$5, d0
pub fn i_0594d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594D8;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0594D8: add.w $30(a6), d0
pub fn i_0594d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594DC;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0594DC: addi.w #$7000, d0
pub fn i_0594dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594E0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7000, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0594E0: movea.w d0, a1
pub fn i_0594e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594E2;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $0594E2: movea.l #$59372, a2
pub fn i_0594e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594E8;
    cpu.ir = 0x247C;
    let v1: u32 = 0x59372;
    cpu.a[2] = v1;
    4
}
/// $0594E8: move.w #$4, d1
pub fn i_0594e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594EC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0594EC: jsr $4784c.l
pub fn i_0594ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x594F2);
    cpu.pc = 0x4784C;
    16
}
/// $0594F2: move.w #$13, d0
pub fn i_0594f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x13;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0594F6: lsl.w #$5, d0
pub fn i_0594f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594F8;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0594F8: add.w $30(a6), d0
pub fn i_0594f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x594FC;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0594FC: addi.w #$7000, d0
pub fn i_0594fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59500;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7000, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $059500: movea.w d0, a1
pub fn i_059500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59502;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $059502: movea.l #$59366, a2
pub fn i_059502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59508;
    cpu.ir = 0x247C;
    let v1: u32 = 0x59366;
    cpu.a[2] = v1;
    4
}
/// $059508: move.w #$4, d1
pub fn i_059508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5950C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05950C: jsr $4784c.l
pub fn i_05950c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59512);
    cpu.pc = 0x4784C;
    16
}
/// $059512: move.w #$1a, d0
pub fn i_059512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59516;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059516: lsl.w #$5, d0
pub fn i_059516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59518;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $059518: add.w $30(a6), d0
pub fn i_059518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5951C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05951C: addi.w #$7000, d0
pub fn i_05951c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59520;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7000, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $059520: movea.w d0, a1
pub fn i_059520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59522;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $059522: movea.l #$59380, a2
pub fn i_059522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59528;
    cpu.ir = 0x247C;
    let v1: u32 = 0x59380;
    cpu.a[2] = v1;
    4
}
/// $059528: move.w #$4, d1
pub fn i_059528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5952C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05952C: jsr $4784c.l
pub fn i_05952c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59532);
    cpu.pc = 0x4784C;
    16
}
/// $059532: subq.w #$1, $30(a6)
pub fn i_059532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59536;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $059536: move.w #$4, d0
pub fn i_059536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5953A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05953A: lsl.w #$5, d0
pub fn i_05953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5953C;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05953C: add.w $30(a6), d0
pub fn i_05953c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59540;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $059540: addi.w #$7000, d0
pub fn i_059540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59544;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7000, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $059544: movea.w d0, a1
pub fn i_059544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59546;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $059546: movea.l #$5936a, a2
pub fn i_059546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5954C;
    cpu.ir = 0x247C;
    let v1: u32 = 0x5936A;
    cpu.a[2] = v1;
    4
}
/// $05954C: move.w #$4, d1
pub fn i_05954c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59550;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059550: jsr $4784c.l
pub fn i_059550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59556;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59556);
    cpu.pc = 0x4784C;
    16
}
/// $059556: move.w #$13, d0
pub fn i_059556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5955A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x13;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05955A: lsl.w #$5, d0
pub fn i_05955a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5955C;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05955C: add.w $30(a6), d0
pub fn i_05955c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59560;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $059560: addi.w #$7000, d0
pub fn i_059560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59564;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7000, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $059564: movea.w d0, a1
pub fn i_059564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59566;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $059566: movea.l #$59362, a2
pub fn i_059566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5956C;
    cpu.ir = 0x247C;
    let v1: u32 = 0x59362;
    cpu.a[2] = v1;
    4
}
/// $05956C: move.w #$4, d1
pub fn i_05956c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59570;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059570: jsr $4784c.l
pub fn i_059570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59576;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59576);
    cpu.pc = 0x4784C;
    16
}
/// $059576: move.w #$1a, d0
pub fn i_059576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5957A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05957A: lsl.w #$5, d0
pub fn i_05957a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5957C;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05957C: add.w $30(a6), d0
pub fn i_05957c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59580;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $059580: addi.w #$7000, d0
pub fn i_059580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59584;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7000, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $059584: movea.w d0, a1
pub fn i_059584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59586;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $059586: movea.l #$5937a, a2
pub fn i_059586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5958C;
    cpu.ir = 0x247C;
    let v1: u32 = 0x5937A;
    cpu.a[2] = v1;
    4
}
/// $05958C: move.w #$4, d1
pub fn i_05958c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59590;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059590: jsr $4784c.l
pub fn i_059590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59596);
    cpu.pc = 0x4784C;
    16
}
/// $059596: cmpi.w #$5, $30(a6)
pub fn i_059596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5959C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Word, false);
    16
}
/// $05959C: bne.w $595ac
pub fn i_05959c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x595AC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0595a0  ($0595A0, 14 B, kernel/jsr5b6_jmp518.c)
/// $0595A0: jsr $5b6.l
pub fn i_0595a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x595A6);
    cpu.pc = 0x5B6;
    16
}
/// $0595A6: jmp $518.l
pub fn i_0595a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595AC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0595AC: rts
pub fn i_0595ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_DrawDigits_0595ae  ($0595AE, 160 B, mission/result_ending_gunner_walker_05934e.s)
/// $0595AE: clr.b d6
pub fn i_0595ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595B0;
    cpu.ir = 0x4206;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0595B0: bra.w $5960c
pub fn i_0595b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5960C;
    10
}
/// $0595B4: clr.w d1
pub fn i_0595b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595B6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0595B6: move.b (a0)+, d1
pub fn i_0595b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595B8;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0595B8: tst.l d7
pub fn i_0595b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595BA;
    cpu.ir = 0x4A87;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0595BA: bne.w $595c2
pub fn i_0595ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x595C2; return 10; }
    8
}
/// $0595BE: move.b #$ff, d6
pub fn i_0595be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595C2;
    cpu.ir = 0x1C3C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0595C2: or.b d1, d6
pub fn i_0595c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595C4;
    cpu.ir = 0x8C01;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0595C4: movem.l d0/d6-d7/a0, -(a7)
pub fn i_0595c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595C8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0595C8: beq.w $595ea
pub fn i_0595c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x595EA; return 10; }
    8
}
/// $0595CC: addi.w #$3460, d1
pub fn i_0595cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595D0;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x3460, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0595D0: movem.w d0-d1, $3c0000.l
pub fn i_0595d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595D8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0595D8: addq.w #$1, d0
pub fn i_0595d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595DA;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0595DA: addi.w #$10, d1
pub fn i_0595da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595DE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0595DE: movem.w d0-d1, $3c0000.l
pub fn i_0595de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595E6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0595E6: bra.w $59604
pub fn i_0595e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x59604;
    10
}
/// $0595EA: move.w #$b40, d1
pub fn i_0595ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595EE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0595EE: movem.w d0-d1, $3c0000.l
pub fn i_0595ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595F6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0595F6: addq.w #$1, d0
pub fn i_0595f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595F8;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0595F8: addi.w #$10, d1
pub fn i_0595f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x595FC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0595FC: movem.w d0-d1, $3c0000.l
pub fn i_0595fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59604;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $059604: movem.l (a7)+, d0/d6-d7/a0
pub fn i_059604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59608;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $059608: addi.w #$20, d0
pub fn i_059608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5960C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05960C: dbra d7, $595b4
pub fn i_05960c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59610;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x59610; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x59610; return 14; }
    cpu.pc = 0x595B4;
    10
}
/// $059610: rts
pub fn i_059610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59612;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $059612: andi.l #$ff, d1
pub fn i_059612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59618;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $059618: divu.w #$a, d1
pub fn i_059618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5961C;
    cpu.ir = 0x82FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[1] / dv; let rm = cpu.d[1] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[1] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05961C: swap d1
pub fn i_05961c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5961E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05961E: move.w d1, d2
pub fn i_05961e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59620;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059620: clr.w d1
pub fn i_059620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59622;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $059622: swap d1
pub fn i_059622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59624;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $059624: divu.w #$a, d1
pub fn i_059624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59628;
    cpu.ir = 0x82FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[1] / dv; let rm = cpu.d[1] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[1] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $059628: swap d1
pub fn i_059628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5962A;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05962A: lsl.w #$8, d1
pub fn i_05962a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5962C;
    cpu.ir = 0xE149;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05962C: or.w d1, d2
pub fn i_05962c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5962E;
    cpu.ir = 0x8441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[2]) & 0xFFFF;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05962E: clr.w d1
pub fn i_05962e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59630;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $059630: swap d1
pub fn i_059630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59632;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $059632: swap d2
pub fn i_059632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59634;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $059634: divu.w #$a, d1
pub fn i_059634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59638;
    cpu.ir = 0x82FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[1] / dv; let rm = cpu.d[1] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[1] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $059638: swap d1
pub fn i_059638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5963A;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05963A: move.w d1, d2
pub fn i_05963a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5963C;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05963C: swap d2
pub fn i_05963c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5963E;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05963E: move.l d2, -(a7)
pub fn i_05963e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59640;
    cpu.ir = 0x2F02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $059640: lea.l $1(a7), a0
pub fn i_059640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59644;
    cpu.ir = 0x41EF;
    let ad1 = cpu.a[7].wrapping_add(0x1);
    cpu.a[0] = ad1;
    16
}
/// $059644: moveq #$3, d7
pub fn i_059644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59646;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $059646: bsr.w $595ae
pub fn i_059646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5964A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5964A);
    cpu.pc = 0x595AE;
    18
}
/// $05964A: move.l (a7)+, d2
pub fn i_05964a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5964C;
    cpu.ir = 0x241F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05964C: rts
pub fn i_05964c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5964E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_TickSound_05964e  ($05964E, 16 B, mission/result_ending_gunner_walker_05934e.s)
/// $05964E: move.b $30(a6), d0
pub fn i_05964e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59652;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $059652: andi.b #$3, d0
pub fn i_059652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59656;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $059656: bne.w $59664
pub fn i_059656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5965A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59664; return 10; }
    8
}
/// $05965A: move.w #$10d8, d0
pub fn i_05965a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5965E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_05965e  ($05965E, 8 B, thunks/jsr_abs_thunks.c)
/// $05965E: jsr $2352.l
pub fn i_05965e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59664;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59664);
    cpu.pc = 0x2352;
    16
}
/// $059664: rts
pub fn i_059664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59666;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_PlayerPanel_Init_059666  ($059666, 180 B, mission/result_ending_gunner_walker_05934e.s)
/// $059666: tst.b $98(a6)
pub fn i_059666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5966A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05966A: bne.w $596a6
pub fn i_05966a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5966E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x596A6; return 10; }
    8
}
/// $05966E: cmpi.b #$1, $10e3b8.l
pub fn i_05966e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59676;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E3B8) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $059676: beq.w $59686
pub fn i_059676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5967A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x59686; return 10; }
    8
}
/// $05967A: jsr $5b6.l
pub fn i_05967a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59680);
    cpu.pc = 0x5B6;
    16
}
/// $059680: jmp $518.l
pub fn i_059680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59686;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $059686: move.w #$7040, d0
pub fn i_059686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5968A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7040;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05968A: move.l #$106e94, $72(a6)
pub fn i_05968a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59692;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x106E94;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059692: move.b $10e3ba.l, $77(a6)
pub fn i_059692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5969A;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10E3BA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad3, v2 as u8);
    24
}
/// $05969A: move.b $10e3bc.l, $76(a6)
pub fn i_05969a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596A2;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10E3BC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0596A2: bra.w $596da
pub fn i_0596a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x596DA;
    10
}
/// $0596A6: cmpi.b #$1, $10e3b9.l
pub fn i_0596a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596AE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E3B9) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0596AE: beq.w $596be
pub fn i_0596ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x596BE; return 10; }
    8
}
/// $0596B2: jsr $5b6.l
pub fn i_0596b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x596B8);
    cpu.pc = 0x5B6;
    16
}
/// $0596B8: jmp $518.l
pub fn i_0596b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596BE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0596BE: move.w #$72a0, d0
pub fn i_0596be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596C2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0596C2: move.l #$106e9c, $72(a6)
pub fn i_0596c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596CA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x106E9C;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0596CA: move.b $10e3bb.l, $77(a6)
pub fn i_0596ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596D2;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10E3BB) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0596D2: move.b $10e3bd.l, $76(a6)
pub fn i_0596d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596DA;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10E3BD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0596DA: move.w d0, $70(a6)
pub fn i_0596da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596DE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0596DE: tst.b $98(a6)
pub fn i_0596de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596E2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0596E2: bne.w $596ee
pub fn i_0596e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x596EE; return 10; }
    8
}
/// $0596E6: lea.l $59386(pc), a2
pub fn i_0596e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596EA;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x59386;
    16
}
/// $0596EA: bra.w $596f2
pub fn i_0596ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x596F2;
    10
}
/// $0596EE: lea.l $59393(pc), a2
pub fn i_0596ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596F2;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x59393;
    16
}
/// $0596F2: movea.l #$a, a1
pub fn i_0596f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596F8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xA;
    cpu.a[1] = v1;
    4
}
/// $0596F8: adda.w $70(a6), a1
pub fn i_0596f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596FC;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $0596FC: move.b #$3, d1
pub fn i_0596fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59700;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $059700: jsr $477fc.l
pub fn i_059700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59706;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59706);
    cpu.pc = 0x477FC;
    16
}
/// $059706: move.w #$f, $30(a6)
pub fn i_059706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5970C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05970C: lea.l $59712(pc), a1
pub fn i_05970c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59710;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59712;
    16
}
/// $059710: move.l a1, (a6)
pub fn i_059710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59712;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059712: subq.w #$1, $30(a6)
pub fn i_059712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59716;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $059716: bne.w $59720
pub fn i_059716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5971A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59720; return 10; }
    8
}
// ===== SetTaskHandler_05971a  ($05971A, 8 B, kernel/task_handlers.c)
/// $05971A: lea.l $59722(pc), a1
pub fn i_05971a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5971E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59722;
    16
}
/// $05971E: move.l a1, (a6)
pub fn i_05971e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59720;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059720: rts
pub fn i_059720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59722;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_PrintContinueLabel_059722  ($059722, 44 B, mission/result_ending_gunner_walker_05934e.s)
/// $059722: movea.l #$d, a1
pub fn i_059722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59728;
    cpu.ir = 0x227C;
    let v1: u32 = 0xD;
    cpu.a[1] = v1;
    4
}
/// $059728: adda.w $70(a6), a1
pub fn i_059728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5972C;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $05972C: lea.l $593a6(pc), a2
pub fn i_05972c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59730;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x593A6;
    16
}
/// $059730: move.b #$3, d1
pub fn i_059730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59734;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $059734: jsr $477fc.l
pub fn i_059734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5973A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5973A);
    cpu.pc = 0x477FC;
    16
}
/// $05973A: move.w #$8, $30(a6)
pub fn i_05973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59740;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059740: lea.l $59746(pc), a1
pub fn i_059740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59744;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59746;
    16
}
/// $059744: move.l a1, (a6)
pub fn i_059744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59746;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059746: subq.w #$1, $30(a6)
pub fn i_059746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5974A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05974A: bne.w $59754
pub fn i_05974a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5974E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59754; return 10; }
    8
}
// ===== SetTaskHandler_05974e  ($05974E, 8 B, kernel/task_handlers.c)
/// $05974E: lea.l $59756(pc), a1
pub fn i_05974e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59752;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59756;
    16
}
/// $059752: move.l a1, (a6)
pub fn i_059752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59754;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059754: rts
pub fn i_059754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59756;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_RollContinues_059756  ($059756, 62 B, mission/result_ending_gunner_walker_05934e.s)
/// $059756: move.b #$1e, $30(a6)
pub fn i_059756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5975C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05975C: lea.l $59762(pc), a1
pub fn i_05975c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59760;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59762;
    16
}
/// $059760: move.l a1, (a6)
pub fn i_059760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59762;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059762: bsr.w $5964e
pub fn i_059762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59766;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x59766);
    cpu.pc = 0x5964E;
    18
}
/// $059766: subq.b #$1, $30(a6)
pub fn i_059766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5976A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05976A: beq.w $5977a
pub fn i_05976a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5976E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5977A; return 10; }
    8
}
/// $05976E: jsr $5e9b6.l
pub fn i_05976e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59774);
    cpu.pc = 0x5E9B6;
    16
}
/// $059774: move.b d0, d1
pub fn i_059774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59776;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $059776: bra.w $59784
pub fn i_059776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5977A;
    cpu.ir = 0x6000;
    cpu.pc = 0x59784;
    10
}
/// $05977A: move.b $77(a6), d1
pub fn i_05977a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5977E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05977E: lea.l $59794(pc), a1
pub fn i_05977e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59782;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59794;
    16
}
/// $059782: move.l a1, (a6)
pub fn i_059782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59784;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059784: movea.l #$1ad, a1
pub fn i_059784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5978A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x1AD;
    cpu.a[1] = v1;
    4
}
/// $05978A: move.l a1, d0
pub fn i_05978a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5978C;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05978C: add.w $70(a6), d0
pub fn i_05978c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59790;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $059790: bra.w $59612
pub fn i_059790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59794;
    cpu.ir = 0x6000;
    cpu.pc = 0x59612;
    10
}
// ===== Result_Wait15_059794  ($059794, 20 B, mission/result_ending_gunner_walker_05934e.s)
/// $059794: move.w #$f, $30(a6)
pub fn i_059794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5979A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05979A: lea.l $597a0(pc), a1
pub fn i_05979a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5979E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x597A0;
    16
}
/// $05979E: move.l a1, (a6)
pub fn i_05979e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0597A0: subq.w #$1, $30(a6)
pub fn i_0597a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597A4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0597A4: bne.w $597ae
pub fn i_0597a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x597AE; return 10; }
    8
}
// ===== SetTaskHandler_0597a8  ($0597A8, 8 B, kernel/task_handlers.c)
/// $0597A8: lea.l $597b0(pc), a1
pub fn i_0597a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x597B0;
    16
}
/// $0597AC: move.l a1, (a6)
pub fn i_0597ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0597AE: rts
pub fn i_0597ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_PrintPrisonerLabel_0597b0  ($0597B0, 82 B, mission/result_ending_gunner_walker_05934e.s)
/// $0597B0: tst.b $10fd83.l
pub fn i_0597b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597B6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD83) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0597B6: bne.w $597d6
pub fn i_0597b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x597D6; return 10; }
    8
}
/// $0597BA: movea.l #$10, a1
pub fn i_0597ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597C0;
    cpu.ir = 0x227C;
    let v1: u32 = 0x10;
    cpu.a[1] = v1;
    4
}
/// $0597C0: adda.w $70(a6), a1
pub fn i_0597c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597C4;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $0597C4: lea.l $593cc(pc), a2
pub fn i_0597c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597C8;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x593CC;
    16
}
/// $0597C8: move.b #$9, d1
pub fn i_0597c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597CC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0597CC: jsr $47888.l
pub fn i_0597cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x597D2);
    cpu.pc = 0x47888;
    16
}
/// $0597D2: bra.w $597ee
pub fn i_0597d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x597EE;
    10
}
/// $0597D6: movea.l #$10, a1
pub fn i_0597d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597DC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x10;
    cpu.a[1] = v1;
    4
}
/// $0597DC: adda.w $70(a6), a1
pub fn i_0597dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597E0;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $0597E0: lea.l $593b5(pc), a2
pub fn i_0597e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597E4;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x593B5;
    16
}
/// $0597E4: move.w #$2300, d0
pub fn i_0597e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597E8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0597E8: jsr $5dad8.l
pub fn i_0597e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x597EE);
    cpu.pc = 0x5DAD8;
    16
}
/// $0597EE: move.w #$8, $30(a6)
pub fn i_0597ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0597F4: lea.l $597fa(pc), a1
pub fn i_0597f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x597FA;
    16
}
/// $0597F8: move.l a1, (a6)
pub fn i_0597f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0597FA: subq.w #$1, $30(a6)
pub fn i_0597fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x597FE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0597FE: bne.w $59808
pub fn i_0597fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59802;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59808; return 10; }
    8
}
// ===== SetTaskHandler_059802  ($059802, 8 B, kernel/task_handlers.c)
/// $059802: lea.l $5980a(pc), a1
pub fn i_059802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59806;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5980A;
    16
}
/// $059806: move.l a1, (a6)
pub fn i_059806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59808;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059808: rts
pub fn i_059808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5980A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_RollPrisoners_05980a  ($05980A, 62 B, mission/result_ending_gunner_walker_05934e.s)
/// $05980A: move.b #$1e, $30(a6)
pub fn i_05980a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59810;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059810: lea.l $59816(pc), a1
pub fn i_059810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59814;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59816;
    16
}
/// $059814: move.l a1, (a6)
pub fn i_059814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59816;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059816: bsr.w $5964e
pub fn i_059816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5981A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5981A);
    cpu.pc = 0x5964E;
    18
}
/// $05981A: subq.b #$1, $30(a6)
pub fn i_05981a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5981E;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05981E: beq.w $5982e
pub fn i_05981e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59822;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5982E; return 10; }
    8
}
/// $059822: jsr $5e9b6.l
pub fn i_059822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59828;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59828);
    cpu.pc = 0x5E9B6;
    16
}
/// $059828: move.b d0, d1
pub fn i_059828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5982A;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05982A: bra.w $59838
pub fn i_05982a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5982E;
    cpu.ir = 0x6000;
    cpu.pc = 0x59838;
    10
}
/// $05982E: lea.l $59848(pc), a1
pub fn i_05982e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59832;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59848;
    16
}
/// $059832: move.l a1, (a6)
pub fn i_059832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59834;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059834: move.b $76(a6), d1
pub fn i_059834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59838;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $059838: movea.l #$1b0, a1
pub fn i_059838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5983E;
    cpu.ir = 0x227C;
    let v1: u32 = 0x1B0;
    cpu.a[1] = v1;
    4
}
/// $05983E: move.l a1, d0
pub fn i_05983e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59840;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $059840: add.w $70(a6), d0
pub fn i_059840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59844;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $059844: bra.w $59612
pub fn i_059844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59848;
    cpu.ir = 0x6000;
    cpu.pc = 0x59612;
    10
}
// ===== Result_Wait15_B_059848  ($059848, 20 B, mission/result_ending_gunner_walker_05934e.s)
/// $059848: move.w #$f, $30(a6)
pub fn i_059848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5984E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05984E: lea.l $59854(pc), a1
pub fn i_05984e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59852;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59854;
    16
}
/// $059852: move.l a1, (a6)
pub fn i_059852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59854;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059854: subq.w #$1, $30(a6)
pub fn i_059854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59858;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $059858: bne.w $59862
pub fn i_059858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5985C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59862; return 10; }
    8
}
// ===== SetTaskHandler_05985c  ($05985C, 8 B, kernel/task_handlers.c)
/// $05985C: lea.l $59864(pc), a1
pub fn i_05985c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59860;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59864;
    16
}
/// $059860: move.l a1, (a6)
pub fn i_059860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59862;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059862: rts
pub fn i_059862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59864;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_PrintScore_059864  ($059864, 66 B, mission/result_ending_gunner_walker_05934e.s)
/// $059864: movea.l #$14, a1
pub fn i_059864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5986A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x14;
    cpu.a[1] = v1;
    4
}
/// $05986A: adda.w $70(a6), a1
pub fn i_05986a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5986E;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $05986E: lea.l $593a0(pc), a2
pub fn i_05986e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59872;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x593A0;
    16
}
/// $059872: move.b #$3, d1
pub fn i_059872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59876;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $059876: jsr $477fc.l
pub fn i_059876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5987C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5987C);
    cpu.pc = 0x477FC;
    16
}
/// $05987C: movea.l #$114, a1
pub fn i_05987c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59882;
    cpu.ir = 0x227C;
    let v1: u32 = 0x114;
    cpu.a[1] = v1;
    4
}
/// $059882: move.l a1, d0
pub fn i_059882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59884;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $059884: add.w $70(a6), d0
pub fn i_059884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59888;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $059888: movea.l $72(a6), a0
pub fn i_059888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5988C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05988C: moveq #$8, d7
pub fn i_05988c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5988E;
    cpu.ir = 0x7E08;
    cpu.d[7] = 0x8;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05988E: bsr.w $595ae
pub fn i_05988e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59892;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x59892);
    cpu.pc = 0x595AE;
    18
}
/// $059892: move.w #$1e, $30(a6)
pub fn i_059892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59898;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059898: lea.l $5989e(pc), a1
pub fn i_059898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5989C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5989E;
    16
}
/// $05989C: move.l a1, (a6)
pub fn i_05989c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5989E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05989E: subq.w #$1, $30(a6)
pub fn i_05989e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598A2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0598A2: bne.w $598ac
pub fn i_0598a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x598AC; return 10; }
    8
}
// ===== SetTaskHandler_0598a6  ($0598A6, 8 B, kernel/task_handlers.c)
/// $0598A6: lea.l $598ae(pc), a1
pub fn i_0598a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x598AE;
    16
}
/// $0598AA: move.l a1, (a6)
pub fn i_0598aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0598AC: rts
pub fn i_0598ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Result_HiScoreEntry_0598ae  ($0598AE, 64 B, mission/result_ending_gunner_walker_05934e.s)
/// $0598AE: move.b #$ff, $21(a6)
pub fn i_0598ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0598B4: movea.l $72(a6), a0
pub fn i_0598b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598B8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0598B8: jsr $51aa4.l
pub fn i_0598b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x598BE);
    cpu.pc = 0x51AA4;
    16
}
/// $0598BE: movea.l #$78, a1
pub fn i_0598be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598C4;
    cpu.ir = 0x227C;
    let v1: u32 = 0x78;
    cpu.a[1] = v1;
    4
}
/// $0598C4: adda.w $70(a6), a1
pub fn i_0598c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598C8;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $0598C8: tst.b $98(a6)
pub fn i_0598c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598CC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0598CC: bne.w $598da
pub fn i_0598cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x598DA; return 10; }
    8
}
/// $0598D0: jsr $97a60.l
pub fn i_0598d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x598D6);
    cpu.pc = 0x97A60;
    16
}
/// $0598D6: bra.w $598e0
pub fn i_0598d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x598E0;
    10
}
/// $0598DA: jsr $97a72.l
pub fn i_0598da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x598E0);
    cpu.pc = 0x97A72;
    16
}
/// $0598E0: lea.l $598e6(pc), a1
pub fn i_0598e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x598E6;
    16
}
/// $0598E4: move.l a1, (a6)
pub fn i_0598e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0598E6: tst.b $21(a6)
pub fn i_0598e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598EA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0598EA: bne.w $598fa
pub fn i_0598ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x598FA; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0598ee  ($0598EE, 14 B, kernel/jsr5b6_jmp518.c)
/// $0598EE: jsr $5b6.l
pub fn i_0598ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x598F4);
    cpu.pc = 0x5B6;
    16
}
/// $0598F4: jmp $518.l
pub fn i_0598f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598FA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0598FA: rts
pub fn i_0598fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_SpawnOrbitFx_0598fc  ($0598FC, 70 B, mission/result_ending_gunner_walker_05934e.s)
/// $0598FC: move.l a6, -(a7)
pub fn i_0598fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x598FE;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0598FE: lea.l $100800.l, a6
pub fn i_0598fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59904;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $059904: lea.l $59a86(pc), a1
pub fn i_059904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59908;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59A86;
    16
}
/// $059908: jsr $4ae.l
pub fn i_059908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5990E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5990E);
    cpu.pc = 0x4AE;
    16
}
/// $05990E: movea.l (a7)+, a6
pub fn i_05990e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59910;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $059910: move.w #$1, d0
pub fn i_059910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59914;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059914: lea.l $593d8(pc), a0
pub fn i_059914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59918;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x593D8;
    16
}
/// $059918: jsr $2b58.l
pub fn i_059918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5991E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5991E);
    cpu.pc = 0x2B58;
    16
}
/// $05991E: jsr $523b2.l
pub fn i_05991e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59924);
    cpu.pc = 0x523B2;
    16
}
/// $059924: move.w #$2b, d0
pub fn i_059924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59928;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059928: jsr $2352.l
pub fn i_059928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5992E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5992E);
    cpu.pc = 0x2352;
    16
}
/// $05992E: move.w #$1e, $30(a6)
pub fn i_05992e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59934;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059934: lea.l $5993a(pc), a1
pub fn i_059934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59938;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5993A;
    16
}
/// $059938: move.l a1, (a6)
pub fn i_059938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5993A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05993A: subq.w #$1, $30(a6)
pub fn i_05993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5993E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05993E: bne.w $59948
pub fn i_05993e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59942;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59948; return 10; }
    8
}
// ===== SetTaskHandler_059942  ($059942, 8 B, kernel/task_handlers.c)
/// $059942: lea.l $5994a(pc), a1
pub fn i_059942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5994A;
    16
}
/// $059946: move.l a1, (a6)
pub fn i_059946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59948;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059948: rts
pub fn i_059948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5994A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_ShowMission_05994a  ($05994A, 26 B, mission/result_ending_gunner_walker_05934e.s)
/// $05994A: lea.l $593fa(pc), a1
pub fn i_05994a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5994E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x593FA;
    16
}
/// $05994E: jsr $4ae.l
pub fn i_05994e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59954);
    cpu.pc = 0x4AE;
    16
}
/// $059954: lea.l $5995a(pc), a1
pub fn i_059954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59958;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5995A;
    16
}
/// $059958: move.l a1, (a6)
pub fn i_059958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5995A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05995A: jsr $6f0.l
pub fn i_05995a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59960);
    cpu.pc = 0x6F0;
    16
}
/// $059960: bcs.w $5996a
pub fn i_059960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59964;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5996A; return 10; }
    8
}
// ===== SetTaskHandler_059964  ($059964, 8 B, kernel/task_handlers.c)
/// $059964: lea.l $5996c(pc), a1
pub fn i_059964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59968;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5996C;
    16
}
/// $059968: move.l a1, (a6)
pub fn i_059968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5996A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05996A: rts
pub fn i_05996a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5996C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_Wait30_05996c  ($05996C, 20 B, mission/result_ending_gunner_walker_05934e.s)
/// $05996C: move.w #$1e, $30(a6)
pub fn i_05996c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59972;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059972: lea.l $59978(pc), a1
pub fn i_059972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59976;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59978;
    16
}
/// $059976: move.l a1, (a6)
pub fn i_059976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59978;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059978: subq.w #$1, $30(a6)
pub fn i_059978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5997C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05997C: bne.w $59986
pub fn i_05997c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59980;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59986; return 10; }
    8
}
// ===== SetTaskHandler_059980  ($059980, 8 B, kernel/task_handlers.c)
/// $059980: lea.l $59988(pc), a1
pub fn i_059980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59984;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59988;
    16
}
/// $059984: move.l a1, (a6)
pub fn i_059984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59986;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059986: rts
pub fn i_059986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59988;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_Wipe_059988  ($059988, 26 B, mission/result_ending_gunner_walker_05934e.s)
/// $059988: lea.l $594c6(pc), a1
pub fn i_059988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5998C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x594C6;
    16
}
/// $05998C: jsr $4ae.l
pub fn i_05998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59992;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59992);
    cpu.pc = 0x4AE;
    16
}
/// $059992: lea.l $59998(pc), a1
pub fn i_059992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59996;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59998;
    16
}
/// $059996: move.l a1, (a6)
pub fn i_059996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59998;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059998: jsr $6f0.l
pub fn i_059998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5999E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5999E);
    cpu.pc = 0x6F0;
    16
}
/// $05999E: bcs.w $599a8
pub fn i_05999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599A2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x599A8; return 10; }
    8
}
// ===== SetTaskHandler_0599a2  ($0599A2, 8 B, kernel/task_handlers.c)
/// $0599A2: lea.l $599aa(pc), a1
pub fn i_0599a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x599AA;
    16
}
/// $0599A6: move.l a1, (a6)
pub fn i_0599a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0599A8: rts
pub fn i_0599a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_Wait15_0599aa  ($0599AA, 20 B, mission/result_ending_gunner_walker_05934e.s)
/// $0599AA: move.w #$f, $30(a6)
pub fn i_0599aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0599B0: lea.l $599b6(pc), a1
pub fn i_0599b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x599B6;
    16
}
/// $0599B4: move.l a1, (a6)
pub fn i_0599b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0599B6: subq.w #$1, $30(a6)
pub fn i_0599b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599BA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0599BA: bne.w $599c4
pub fn i_0599ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x599C4; return 10; }
    8
}
// ===== SetTaskHandler_0599be  ($0599BE, 8 B, kernel/task_handlers.c)
/// $0599BE: lea.l $599c6(pc), a1
pub fn i_0599be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x599C6;
    16
}
/// $0599C2: move.l a1, (a6)
pub fn i_0599c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0599C4: rts
pub fn i_0599c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_PanelP1_0599c6  ($0599C6, 36 B, mission/result_ending_gunner_walker_05934e.s)
/// $0599C6: lea.l $59666(pc), a1
pub fn i_0599c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59666;
    16
}
/// $0599CA: jsr $4ae.l
pub fn i_0599ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x599D0);
    cpu.pc = 0x4AE;
    16
}
/// $0599D0: move.b #$0, $98(a0)
pub fn i_0599d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599D6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0599D6: move.w #$4, $30(a6)
pub fn i_0599d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0599DC: lea.l $599e2(pc), a1
pub fn i_0599dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x599E2;
    16
}
/// $0599E0: move.l a1, (a6)
pub fn i_0599e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0599E2: subq.w #$1, $30(a6)
pub fn i_0599e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599E6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0599E6: bne.w $599f0
pub fn i_0599e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x599F0; return 10; }
    8
}
// ===== SetTaskHandler_0599ea  ($0599EA, 8 B, kernel/task_handlers.c)
/// $0599EA: lea.l $599f2(pc), a1
pub fn i_0599ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x599F2;
    16
}
/// $0599EE: move.l a1, (a6)
pub fn i_0599ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0599F0: rts
pub fn i_0599f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_PanelP2_0599f2  ($0599F2, 32 B, mission/result_ending_gunner_walker_05934e.s)
/// $0599F2: lea.l $59666(pc), a1
pub fn i_0599f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59666;
    16
}
/// $0599F6: jsr $4ae.l
pub fn i_0599f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x599FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x599FC);
    cpu.pc = 0x4AE;
    16
}
/// $0599FC: move.b #$1, $98(a0)
pub fn i_0599fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A02;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059A02: lea.l $59a08(pc), a1
pub fn i_059a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59A08;
    16
}
/// $059A06: move.l a1, (a6)
pub fn i_059a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059A08: jsr $6f0.l
pub fn i_059a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59A0E);
    cpu.pc = 0x6F0;
    16
}
/// $059A0E: bcs.w $59a18
pub fn i_059a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A12;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x59A18; return 10; }
    8
}
// ===== SetTaskHandler_059a12  ($059A12, 8 B, kernel/task_handlers.c)
/// $059A12: lea.l $59a1a(pc), a1
pub fn i_059a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59A1A;
    16
}
/// $059A16: move.l a1, (a6)
pub fn i_059a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059A18: rts
pub fn i_059a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_FadeA0_059a1a  ($059A1A, 30 B, mission/result_ending_gunner_walker_05934e.s)
/// $059A1A: move.b #$a0, d1
pub fn i_059a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A1E;
    cpu.ir = 0x123C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $059A1E: jsr $2308.l
pub fn i_059a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59A24);
    cpu.pc = 0x2308;
    16
}
/// $059A24: move.w #$c8, $30(a6)
pub fn i_059a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A2A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059A2A: lea.l $59a30(pc), a1
pub fn i_059a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59A30;
    16
}
/// $059A2E: move.l a1, (a6)
pub fn i_059a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059A30: subq.w #$1, $30(a6)
pub fn i_059a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A34;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $059A34: bne.w $59a3e
pub fn i_059a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A38;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59A3E; return 10; }
    8
}
// ===== SetTaskHandler_059a38  ($059A38, 8 B, kernel/task_handlers.c)
/// $059A38: lea.l $59a40(pc), a1
pub fn i_059a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59A40;
    16
}
/// $059A3C: move.l a1, (a6)
pub fn i_059a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059A3E: rts
pub fn i_059a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_Fade40_059a40  ($059A40, 40 B, mission/result_ending_gunner_walker_05934e.s)
/// $059A40: move.w #$1, d0
pub fn i_059a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A44;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059A44: jsr $5239e.l
pub fn i_059a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59A4A);
    cpu.pc = 0x5239E;
    16
}
/// $059A4A: move.b #$40, d0
pub fn i_059a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A4E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $059A4E: jsr $2308.l
pub fn i_059a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59A54);
    cpu.pc = 0x2308;
    16
}
/// $059A54: move.w #$3c, $30(a6)
pub fn i_059a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A5A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059A5A: lea.l $59a60(pc), a1
pub fn i_059a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59A60;
    16
}
/// $059A5E: move.l a1, (a6)
pub fn i_059a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059A60: subq.w #$1, $30(a6)
pub fn i_059a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A64;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $059A64: bne.w $59a6e
pub fn i_059a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A68;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59A6E; return 10; }
    8
}
// ===== SetTaskHandler_059a68  ($059A68, 8 B, kernel/task_handlers.c)
/// $059A68: lea.l $59a70(pc), a1
pub fn i_059a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59A70;
    16
}
/// $059A6C: move.l a1, (a6)
pub fn i_059a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059A6E: rts
pub fn i_059a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_Seq_Done_059a70  ($059A70, 22 B, mission/result_ending_gunner_walker_05934e.s)
/// $059A70: movea.l $c(a6), a0
pub fn i_059a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A74;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $059A74: move.b #$ff, $20(a0)
pub fn i_059a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A7A;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059A7A: jsr $5b6.l
pub fn i_059a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59A80);
    cpu.pc = 0x5B6;
    16
}
/// $059A80: jmp $518.l
pub fn i_059a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A86;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== EndingOrbit_Parent_059a86  ($059A86, 128 B, mission/result_ending_gunner_walker_05934e.s)
/// $059A86: lea.l $59b06(pc), a1
pub fn i_059a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59B06;
    16
}
/// $059A8A: jsr $4ae.l
pub fn i_059a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59A90);
    cpu.pc = 0x4AE;
    16
}
/// $059A90: move.l #$24e208, $3c(a0)
pub fn i_059a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A98;
    cpu.ir = 0x217C;
    let v1: u32 = 0x24E208;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059A98: move.w #$0, $34(a0)
pub fn i_059a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A9E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059A9E: lea.l $59b06(pc), a1
pub fn i_059a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59B06;
    16
}
/// $059AA2: jsr $4ae.l
pub fn i_059aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59AA8);
    cpu.pc = 0x4AE;
    16
}
/// $059AA8: bset.b #$0, $3a(a0)
pub fn i_059aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AAE;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $059AAE: move.l #$24e030, $3c(a0)
pub fn i_059aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AB6;
    cpu.ir = 0x217C;
    let v1: u32 = 0x24E030;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059AB6: move.w #$4000, $34(a0)
pub fn i_059ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59ABC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059ABC: lea.l $59b06(pc), a1
pub fn i_059abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59B06;
    16
}
/// $059AC0: jsr $4ae.l
pub fn i_059ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59AC6);
    cpu.pc = 0x4AE;
    16
}
/// $059AC6: bset.b #$0, $3a(a0)
pub fn i_059ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59ACC;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $059ACC: move.l #$24e208, $3c(a0)
pub fn i_059acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AD4;
    cpu.ir = 0x217C;
    let v1: u32 = 0x24E208;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059AD4: move.w #$8000, $34(a0)
pub fn i_059ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59ADA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059ADA: lea.l $59b06(pc), a1
pub fn i_059ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59ADE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59B06;
    16
}
/// $059ADE: jsr $4ae.l
pub fn i_059ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59AE4);
    cpu.pc = 0x4AE;
    16
}
/// $059AE4: move.l #$24e030, $3c(a0)
pub fn i_059ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AEC;
    cpu.ir = 0x217C;
    let v1: u32 = 0x24E030;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059AEC: move.w #$c000, $34(a0)
pub fn i_059aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AF2;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059AF2: move.w #$4000, $34(a6)
pub fn i_059af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059AF8: lea.l $59afe(pc), a1
pub fn i_059af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59AFE;
    16
}
/// $059AFC: move.l a1, (a6)
pub fn i_059afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59AFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059AFE: subi.w #$100, $34(a6)
pub fn i_059afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B04;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $059B04: rts
pub fn i_059b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EndingOrbit_Child_059b06  ($059B06, 76 B, mission/result_ending_gunner_walker_05934e.s)
/// $059B06: move.w #$15f, d1
pub fn i_059b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B0A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059B0A: jsr $236e.l
pub fn i_059b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59B10);
    cpu.pc = 0x236E;
    16
}
/// $059B10: move.w #$180, $24(a6)
pub fn i_059b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B16;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059B16: lea.l $59b1c(pc), a1
pub fn i_059b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59B1C;
    16
}
/// $059B1A: move.l a1, (a6)
pub fn i_059b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059B1C: movea.l $c(a6), a0
pub fn i_059b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B20;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $059B20: move.w $34(a0), d0
pub fn i_059b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B24;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $059B24: add.w $34(a6), d0
pub fn i_059b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B28;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $059B28: bmi.w $59b58
pub fn i_059b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B2C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x59B58; return 10; }
    8
}
/// $059B2C: lsr.w #$8, d0
pub fn i_059b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B2E;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $059B2E: move.w d0, -(a7)
pub fn i_059b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B30;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $059B30: move.w #$ff, d1
pub fn i_059b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B34;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059B34: jsr $13c0e.l
pub fn i_059b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59B3A);
    cpu.pc = 0x13C0E;
    16
}
/// $059B3A: move.b d2, $32(a6)
pub fn i_059b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B3E;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059B3E: move.w (a7)+, d0
pub fn i_059b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B40;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $059B40: move.w #$88, d1
pub fn i_059b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059B44: jsr $13c0e.l
pub fn i_059b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59B4A);
    cpu.pc = 0x13C0E;
    16
}
/// $059B4A: addi.w #$a0, d1
pub fn i_059b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B4E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA0, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $059B4E: move.w d1, $22(a6)
pub fn i_059b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B52;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_059b52  ($059B52, 8 B, thunks/jsr_abs_thunks.c)
/// $059B52: jsr $5ca2a.l
pub fn i_059b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59B58);
    cpu.pc = 0x5CA2A;
    16
}
/// $059B58: rts
pub fn i_059b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_PeaceWait_059b6a  ($059B6A, 20 B, mission/result_ending_gunner_walker_05934e.s)
/// $059B6A: move.w #$2d, $30(a6)
pub fn i_059b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059B70: lea.l $59b76(pc), a1
pub fn i_059b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59B76;
    16
}
/// $059B74: move.l a1, (a6)
pub fn i_059b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059B76: subq.w #$1, $30(a6)
pub fn i_059b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B7A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $059B7A: bne.w $59b84
pub fn i_059b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59B84; return 10; }
    8
}
// ===== SetTaskHandler_059b7e  ($059B7E, 8 B, kernel/task_handlers.c)
/// $059B7E: lea.l $59b86(pc), a1
pub fn i_059b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59B86;
    16
}
/// $059B82: move.l a1, (a6)
pub fn i_059b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059B84: rts
pub fn i_059b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_ShowPeaceForever_059b86  ($059B86, 56 B, mission/result_ending_gunner_walker_05934e.s)
/// $059B86: lea.l $4737e.l, a1
pub fn i_059b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4737E;
    20
}
/// $059B8C: jsr $4ae.l
pub fn i_059b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59B92);
    cpu.pc = 0x4AE;
    16
}
/// $059B92: move.w #$711a, d0
pub fn i_059b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B96;
    cpu.ir = 0x303C;
    let v1: u32 = 0x711A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059B96: move.w d0, $22(a0)
pub fn i_059b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59B9A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059B9A: move.l #$59b5a, $3c(a0)
pub fn i_059b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BA2;
    cpu.ir = 0x217C;
    let v1: u32 = 0x59B5A;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059BA2: move.w #$1, $30(a0)
pub fn i_059ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BA8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059BA8: move.b #$4, $16(a0)
pub fn i_059ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BAE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059BAE: lea.l $59bb4(pc), a1
pub fn i_059bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59BB4;
    16
}
/// $059BB2: move.l a1, (a6)
pub fn i_059bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059BB4: jsr $6f0.l
pub fn i_059bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59BBA);
    cpu.pc = 0x6F0;
    16
}
/// $059BBA: bcs.w $59bc4
pub fn i_059bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BBE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x59BC4; return 10; }
    8
}
// ===== SetTaskHandler_059bbe  ($059BBE, 8 B, kernel/task_handlers.c)
/// $059BBE: lea.l $59bc6(pc), a1
pub fn i_059bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59BC6;
    16
}
/// $059BC2: move.l a1, (a6)
pub fn i_059bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059BC4: rts
pub fn i_059bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ending_PeaceExit_059bc6  ($059BC6, 12 B, mission/result_ending_gunner_walker_05934e.s)
/// $059BC6: jsr $5b6.l
pub fn i_059bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59BCC);
    cpu.pc = 0x5B6;
    16
}
/// $059BCC: jmp $518.l
pub fn i_059bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BD2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Entity_CmpDepthToParent_059bd2  ($059BD2, 16 B, mission/result_ending_gunner_walker_05934e.s)
/// $059BD2: movea.l $8(a6), a1
pub fn i_059bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BD6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $059BD6: move.b $10(a6), d0
pub fn i_059bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BDA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $059BDA: cmp.b $10(a1), d0
pub fn i_059bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BDE;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $059BDE: bcs.w $59be8
pub fn i_059bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BE2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x59BE8; return 10; }
    8
}
// ===== ClearXN_059be2  ($059BE2, 6 B, thunks/ccr_helpers.c)
/// $059BE2: andi.b #$ee, ccr
pub fn i_059be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BE6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $059BE6: rts
pub fn i_059be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_059be8  ($059BE8, 6 B, thunks/ccr_helpers.c)
/// $059BE8: ori.b #$11, ccr
pub fn i_059be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BEC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $059BEC: rts
pub fn i_059bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner_InitShadeRingId_059bee  ($059BEE, 28 B, mission/result_ending_gunner_walker_05934e.s)
/// $059BEE: move.w #$8000, d0
pub fn i_059bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059BF2: jsr $28134.l
pub fn i_059bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59BF8);
    cpu.pc = 0x28134;
    16
}
/// $059BF8: andi.w #$ffe3, $38(a6)
pub fn i_059bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59BFE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $059BFE: ori.w #$4, $38(a6)
pub fn i_059bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C04;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $059C04: jsr $8f3a6.l
pub fn i_059c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C0A);
    cpu.pc = 0x8F3A6;
    16
}
// ===== SetTaskB_059c0a  ($059C0A, 6 B, kernel/task_setters_b.c)
/// $059C0A: move.b d0, $70(a6)
pub fn i_059c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C0E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059C0E: rts
pub fn i_059c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner_ProbeOrDie_059c10  ($059C10, 16 B, mission/result_ending_gunner_walker_05934e.s)
/// $059C10: lea.l $2b73b4.l, a0
pub fn i_059c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B73B4;
    20
}
/// $059C16: jsr $5dd5c.l
pub fn i_059c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C1C);
    cpu.pc = 0x5DD5C;
    16
}
/// $059C1C: bcc.w $59c2c
pub fn i_059c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x59C2C; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_059c20  ($059C20, 14 B, kernel/jsr5b6_jmp518.c)
/// $059C20: jsr $5b6.l
pub fn i_059c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C26);
    cpu.pc = 0x5B6;
    16
}
/// $059C26: jmp $518.l
pub fn i_059c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C2C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $059C2C: rts
pub fn i_059c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner_Boot_059c2e  ($059C2E, 12 B, mission/result_ending_gunner_walker_05934e.s)
/// $059C2E: bsr.b $59bee
pub fn i_059c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C30;
    cpu.ir = 0x61BE;
    cpu.push32(bus, 0x59C30);
    cpu.pc = 0x59BEE;
    18
}
/// $059C30: lea.l $59d50(pc), a1
pub fn i_059c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59D50;
    16
}
/// $059C34: jsr $4ae.l
pub fn i_059c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C3A);
    cpu.pc = 0x4AE;
    16
}
// ===== SetTaskHandler_059c3a  ($059C3A, 8 B, kernel/task_handlers.c)
/// $059C3A: lea.l $59c42(pc), a1
pub fn i_059c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59C42;
    16
}
/// $059C3E: move.l a1, (a6)
pub fn i_059c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059C40: rts
pub fn i_059c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner_Search_059c42  ($059C42, 94 B, mission/result_ending_gunner_walker_05934e.s)
/// $059C42: lea.l $2b732c.l, a0
pub fn i_059c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B732C;
    20
}
/// $059C48: jsr $28cd4.l
pub fn i_059c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C4E);
    cpu.pc = 0x28CD4;
    16
}
/// $059C4E: lea.l $59c54(pc), a1
pub fn i_059c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59C54;
    16
}
/// $059C52: move.l a1, (a6)
pub fn i_059c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059C54: jsr $2783a.l
pub fn i_059c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C5A);
    cpu.pc = 0x2783A;
    16
}
/// $059C5A: jsr $28d70.l
pub fn i_059c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C60);
    cpu.pc = 0x28D70;
    16
}
/// $059C60: move.w $22(a6), d0
pub fn i_059c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $059C64: cmpi.w #$20, d0
pub fn i_059c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C68;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $059C68: ble.w $59c9c
pub fn i_059c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C6C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x59C9C; return 10; }
    8
}
/// $059C6C: move.w $24(a6), d1
pub fn i_059c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C70;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $059C70: move.b $70(a6), d2
pub fn i_059c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C74;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $059C74: jsr $8f3be.l
pub fn i_059c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C7A);
    cpu.pc = 0x8F3BE;
    16
}
/// $059C7A: tst.b d4
pub fn i_059c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C7C;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $059C7C: bmi.w $59c9c
pub fn i_059c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C80;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x59C9C; return 10; }
    8
}
/// $059C80: move.w d4, -(a7)
pub fn i_059c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C82;
    cpu.ir = 0x3F04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $059C82: jsr $13600.l
pub fn i_059c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C88);
    cpu.pc = 0x13600;
    16
}
/// $059C88: move.w (a7)+, d4
pub fn i_059c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C8A;
    cpu.ir = 0x381F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $059C8A: jsr $8f69c.l
pub fn i_059c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C90);
    cpu.pc = 0x8F69C;
    16
}
/// $059C90: jsr $236e.l
pub fn i_059c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59C96);
    cpu.pc = 0x236E;
    16
}
/// $059C96: lea.l $59ca0(pc), a1
pub fn i_059c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59CA0;
    16
}
/// $059C9A: move.l a1, (a6)
pub fn i_059c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59C9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059C9C: bra.w $59c10
pub fn i_059c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CA0;
    cpu.ir = 0x6000;
    cpu.pc = 0x59C10;
    10
}
// ===== Gunner_Fire_059ca0  ($059CA0, 74 B, mission/result_ending_gunner_walker_05934e.s)
/// $059CA0: lea.l $2b7340.l, a0
pub fn i_059ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7340;
    20
}
/// $059CA6: jsr $28cd4.l
pub fn i_059ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59CAC);
    cpu.pc = 0x28CD4;
    16
}
/// $059CAC: move.b #$3, $76(a6)
pub fn i_059cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CB2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059CB2: lea.l $59cb8(pc), a1
pub fn i_059cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59CB8;
    16
}
/// $059CB6: move.l a1, (a6)
pub fn i_059cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059CB8: jsr $2783a.l
pub fn i_059cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59CBE);
    cpu.pc = 0x2783A;
    16
}
/// $059CBE: jsr $28d70.l
pub fn i_059cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59CC4);
    cpu.pc = 0x28D70;
    16
}
/// $059CC4: bcc.w $59ce6
pub fn i_059cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x59CE6; return 10; }
    8
}
/// $059CC8: subq.b #$1, $76(a6)
pub fn i_059cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CCC;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $059CCC: beq.w $59ce0
pub fn i_059ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x59CE0; return 10; }
    8
}
/// $059CD0: lea.l $2b7340.l, a0
pub fn i_059cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7340;
    20
}
/// $059CD6: jsr $28cd4.l
pub fn i_059cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59CDC);
    cpu.pc = 0x28CD4;
    16
}
/// $059CDC: bra.w $59ce6
pub fn i_059cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x59CE6;
    10
}
/// $059CE0: lea.l $59cea(pc), a1
pub fn i_059ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59CEA;
    16
}
/// $059CE4: move.l a1, (a6)
pub fn i_059ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059CE6: bra.w $59d1c
pub fn i_059ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x59D1C;
    10
}
// ===== Gunner_Reload_059cea  ($059CEA, 102 B, mission/result_ending_gunner_walker_05934e.s)
/// $059CEA: move.b #$78, $76(a6)
pub fn i_059cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CF0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $059CF0: lea.l $2b737c.l, a0
pub fn i_059cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CF6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B737C;
    20
}
/// $059CF6: jsr $28cd4.l
pub fn i_059cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59CFC);
    cpu.pc = 0x28CD4;
    16
}
/// $059CFC: lea.l $59d02(pc), a1
pub fn i_059cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59D02;
    16
}
/// $059D00: move.l a1, (a6)
pub fn i_059d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059D02: jsr $2783a.l
pub fn i_059d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59D08);
    cpu.pc = 0x2783A;
    16
}
/// $059D08: jsr $28d70.l
pub fn i_059d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59D0E);
    cpu.pc = 0x28D70;
    16
}
/// $059D0E: subq.b #$1, $76(a6)
pub fn i_059d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D12;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $059D12: bne.w $59d1c
pub fn i_059d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x59D1C; return 10; }
    8
}
/// $059D16: lea.l $59ca0(pc), a1
pub fn i_059d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59CA0;
    16
}
/// $059D1A: move.l a1, (a6)
pub fn i_059d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059D1C: move.w $22(a6), d0
pub fn i_059d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D20;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $059D20: cmpi.w #$20, d0
pub fn i_059d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D24;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $059D24: ble.w $59d46
pub fn i_059d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D28;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x59D46; return 10; }
    8
}
/// $059D28: move.w $24(a6), d1
pub fn i_059d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D2C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $059D2C: move.b $70(a6), d2
pub fn i_059d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D30;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $059D30: jsr $8f3be.l
pub fn i_059d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59D36);
    cpu.pc = 0x8F3BE;
    16
}
/// $059D36: tst.b d4
pub fn i_059d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D38;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $059D38: bpl.w $59d42
pub fn i_059d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D3C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x59D42; return 10; }
    8
}
/// $059D3C: lea.l $59c42(pc), a1
pub fn i_059d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59C42;
    16
}
/// $059D40: move.l a1, (a6)
pub fn i_059d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059D42: bra.w $59d4c
pub fn i_059d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D46;
    cpu.ir = 0x6000;
    cpu.pc = 0x59D4C;
    10
}
/// $059D46: lea.l $59c42(pc), a1
pub fn i_059d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59C42;
    16
}
/// $059D4A: move.l a1, (a6)
pub fn i_059d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059D4C: bra.w $59c10
pub fn i_059d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D50;
    cpu.ir = 0x6000;
    cpu.pc = 0x59C10;
    10
}
// ===== Gunner_Child_Init_059d50  ($059D50, 10 B, mission/result_ending_gunner_walker_05934e.s)
/// $059D50: move.w #$56, d1
pub fn i_059d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D54;
    cpu.ir = 0x323C;
    let v1: u32 = 0x56;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059D54: jsr $236e.l
pub fn i_059d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59D5A);
    cpu.pc = 0x236E;
    16
}
// ===== SetTaskHandler_059d5a  ($059D5A, 8 B, kernel/task_handlers.c)
/// $059D5A: lea.l $59d62(pc), a1
pub fn i_059d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59D62;
    16
}
/// $059D5E: move.l a1, (a6)
pub fn i_059d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059D60: rts
pub fn i_059d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner_Child_Sync_059d62  ($059D62, 34 B, mission/result_ending_gunner_walker_05934e.s)
/// $059D62: movea.l $c(a6), a0
pub fn i_059d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D66;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $059D66: move.w $22(a0), $22(a6)
pub fn i_059d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D6C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $059D6C: move.w $24(a0), $24(a6)
pub fn i_059d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D72;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $059D72: move.w $38(a0), $38(a6)
pub fn i_059d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D78;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $059D78: move.l $72(a0), $3c(a6)
pub fn i_059d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D7E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $059D7E: jmp $5ca2a.l
pub fn i_059d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D84;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CA2A;
    8
}
// ===== Gunner_SpawnShell_059d84  ($059D84, 34 B, mission/result_ending_gunner_walker_05934e.s)
/// $059D84: move.l a6, -(a7)
pub fn i_059d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D86;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $059D86: lea.l $100800.l, a6
pub fn i_059d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D8C;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $059D8C: lea.l $59da6(pc), a1
pub fn i_059d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59DA6;
    16
}
/// $059D90: jsr $4ae.l
pub fn i_059d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59D96);
    cpu.pc = 0x4AE;
    16
}
/// $059D96: movea.l (a7)+, a6
pub fn i_059d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D98;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $059D98: move.w $22(a6), $22(a0)
pub fn i_059d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59D9E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $059D9E: move.w $24(a6), $24(a0)
pub fn i_059d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DA4;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $059DA4: rts
pub fn i_059da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner_Shell_059da6  ($059DA6, 142 B, mission/result_ending_gunner_walker_05934e.s)
/// $059DA6: move.w #$108c, d0
pub fn i_059da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DAA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059DAA: jsr $2352.l
pub fn i_059daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59DB0);
    cpu.pc = 0x2352;
    16
}
/// $059DB0: move.w #$57, d1
pub fn i_059db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DB4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x57;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $059DB4: jsr $236e.l
pub fn i_059db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59DBA);
    cpu.pc = 0x236E;
    16
}
/// $059DBA: move.w #$d000, $38(a6)
pub fn i_059dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059DC0: addi.w #$20, $22(a6)
pub fn i_059dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DC6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $059DC6: addi.w #$14, $24(a6)
pub fn i_059dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DCC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $059DCC: move.w #$400, $28(a6)
pub fn i_059dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DD2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $059DD2: lea.l $2b7390.l, a0
pub fn i_059dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DD8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7390;
    20
}
/// $059DD8: jsr $28cd4.l
pub fn i_059dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59DDE);
    cpu.pc = 0x28CD4;
    16
}
/// $059DDE: lea.l $2b73bc.l, a0
pub fn i_059dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B73BC;
    20
}
/// $059DE4: move.l a0, $4c(a6)
pub fn i_059de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DE8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $059DE8: jsr $283ca.l
pub fn i_059de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59DEE);
    cpu.pc = 0x283CA;
    16
}
/// $059DEE: jsr $283ca.l
pub fn i_059dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59DF4);
    cpu.pc = 0x283CA;
    16
}
/// $059DF4: lea.l $59dfa(pc), a1
pub fn i_059df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x59DFA;
    16
}
/// $059DF8: move.l a1, (a6)
pub fn i_059df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59DFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $059DFA: jsr $27cee.l
pub fn i_059dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59E00);
    cpu.pc = 0x27CEE;
    16
}
/// $059E00: move.w $22(a6), d0
pub fn i_059e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E04;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $059E04: addi.w #$10, d0
pub fn i_059e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E08;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $059E08: subi.w #$160, d0
pub fn i_059e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E0C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x160, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $059E0C: bcs.w $59e16
pub fn i_059e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E10;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x59E16; return 10; }
    8
}
/// $059E10: jmp $518.l
pub fn i_059e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E16;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $059E16: jsr $28d70.l
pub fn i_059e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59E1C);
    cpu.pc = 0x28D70;
    16
}
/// $059E1C: jsr $283d8.l
pub fn i_059e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x59E22);
    cpu.pc = 0x283D8;
    16
}
/// $059E22: btst.b #$1, $13(a6)
pub fn i_059e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E28;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $059E28: beq.w $59e32
pub fn i_059e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E2C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x59E32; return 10; }
    8
}
/// $059E2C: jmp $518.l
pub fn i_059e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E32;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $059E32: rts
pub fn i_059e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_059e34  ($059E34, 16 B, mission/result_ending_gunner_walker_05934e.s)
/// $059E34: movea.l $8(a6), a1
pub fn i_059e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E38;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $059E38: move.b $10(a6), d0
pub fn i_059e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E3C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $059E3C: cmp.b $10(a1), d0
pub fn i_059e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E40;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $059E40: bcs.w $59e4a
pub fn i_059e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E44;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x59E4A; return 10; }
    8
}
// ===== ClearXN_059e44  ($059E44, 6 B, thunks/ccr_helpers.c)
/// $059E44: andi.b #$ee, ccr
pub fn i_059e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E48;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $059E48: rts
pub fn i_059e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_059e4a  ($059E4A, 6 B, thunks/ccr_helpers.c)
/// $059E4A: ori.b #$11, ccr
pub fn i_059e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E4E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $059E4E: rts
pub fn i_059e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner2_AimFromAngle_05a1b6  ($05A1B6, 128 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A1B6: moveq #$0, d0
pub fn i_05a1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1B8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A1B8: move.b $70(a6), d0
pub fn i_05a1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1BC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05A1BC: lsr.b #$4, d0
pub fn i_05a1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1BE;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $05A1BE: move.w d0, d1
pub fn i_05a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1C0;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A1C0: andi.b #$7, d0
pub fn i_05a1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1C4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A1C4: cmpi.b #$5, d0
pub fn i_05a1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1C8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $05A1C8: bcs.w $5a1d0
pub fn i_05a1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5A1D0; return 10; }
    8
}
/// $05A1CC: neg.b d0
pub fn i_05a1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1CE;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05A1CE: addq.b #$8, d0
pub fn i_05a1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1D0;
    cpu.ir = 0x5000;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05A1D0: addq.w #$3, d1
pub fn i_05a1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1D2;
    cpu.ir = 0x5641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05A1D2: andi.b #$f, d1
pub fn i_05a1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1D6;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A1D6: cmpi.w #$7, d1
pub fn i_05a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1DA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7, Size::Word, false);
    8
}
/// $05A1DA: bcc.w $5a1e8
pub fn i_05a1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5A1E8; return 10; }
    8
}
/// $05A1DE: bset.b #$0, $3a(a6)
pub fn i_05a1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1E4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05A1E4: bra.w $5a1ee
pub fn i_05a1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A1EE;
    10
}
/// $05A1E8: bclr.b #$0, $3a(a6)
pub fn i_05a1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1EE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05A1EE: add.w d0, d0
pub fn i_05a1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1F0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05A1F0: add.w d0, d0
pub fn i_05a1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1F2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05A1F2: move.w d0, $7c(a6)
pub fn i_05a1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1F6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A1F6: movea.l (a0, d0.w), a0
pub fn i_05a1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A1FA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $05A1FA: jsr $28cd4.l
pub fn i_05a1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A200);
    cpu.pc = 0x28CD4;
    16
}
/// $05A200: lea.l $5a18e(pc), a1
pub fn i_05a200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A204;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A18E;
    16
}
/// $05A204: lea.l $5a1a2(pc), a2
pub fn i_05a204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A208;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5A1A2;
    16
}
/// $05A208: move.l (a1, d0.w), d1
pub fn i_05a208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A20C;
    cpu.ir = 0x2231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05A20C: move.l (a2, d0.w), d2
pub fn i_05a20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A210;
    cpu.ir = 0x2432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05A210: btst.b #$0, $3a(a6)
pub fn i_05a210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A216;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05A216: beq.w $5a21e
pub fn i_05a216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A21A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5A21E; return 10; }
    8
}
/// $05A21A: neg.w d1
pub fn i_05a21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A21C;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05A21C: neg.w d2
pub fn i_05a21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A21E;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05A21E: add.w $22(a6), d1
pub fn i_05a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A222;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05A222: move.w d1, $76(a6)
pub fn i_05a222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A226;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A226: add.w $22(a6), d2
pub fn i_05a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A22A;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05A22A: swap d2
pub fn i_05a22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A22C;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A22C: add.w $24(a6), d2
pub fn i_05a22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A230;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05A230: move.l d2, $78(a6)
pub fn i_05a230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A234;
    cpu.ir = 0x2D42;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05A234: rts
pub fn i_05a234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A236;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner2_ProbeBox_05a236  ($05A236, 8 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A236: dc.w $ffd0
pub fn i_05a236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A238;
    cpu.ir = 0xFFD0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Gunner2_ProbeOrDie_05a23e  ($05A23E, 14 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A23E: lea.l $5a236(pc), a0
pub fn i_05a23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A242;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A236;
    16
}
/// $05A242: jsr $5dd5c.l
pub fn i_05a242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A248;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A248);
    cpu.pc = 0x5DD5C;
    16
}
/// $05A248: bcc.w $5a258
pub fn i_05a248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A24C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5A258; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_05a24c  ($05A24C, 14 B, kernel/jsr5b6_jmp518.c)
/// $05A24C: jsr $5b6.l
pub fn i_05a24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A252);
    cpu.pc = 0x5B6;
    16
}
/// $05A252: jmp $518.l
pub fn i_05a252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A258;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05A258: rts
pub fn i_05a258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A25A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner2_Init_05a25a  ($05A25A, 40 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A25A: move.w $4000.w, d0
pub fn i_05a25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A25E;
    cpu.ir = 0x3038;
    let v2 = bus.read16(0x4000) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $05A25E: ori.w #$4, d0
pub fn i_05a25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A262;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05A262: move.w d0, $38(a6)
pub fn i_05a262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A266;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A266: jsr $8f3a6.l
pub fn i_05a266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A26C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A26C);
    cpu.pc = 0x8F3A6;
    16
}
/// $05A26C: not.b d0
pub fn i_05a26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A26E;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05A26E: move.b d0, $72(a6)
pub fn i_05a26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A272;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05A272: lea.l $5a65c(pc), a1
pub fn i_05a272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A276;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A65C;
    16
}
/// $05A276: jsr $4ae.l
pub fn i_05a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A27C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A27C);
    cpu.pc = 0x4AE;
    16
}
/// $05A27C: move.w #$8000, $70(a6)
pub fn i_05a27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A282;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_05a282  ($05A282, 8 B, kernel/task_handlers.c)
/// $05A282: lea.l $5a28a(pc), a1
pub fn i_05a282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A286;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A28A;
    16
}
/// $05A286: move.l a1, (a6)
pub fn i_05a286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A288;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A288: rts
pub fn i_05a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A28A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner2_Search_05a28a  ($05A28A, 60 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A28A: lea.l $5a13e(pc), a0
pub fn i_05a28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A28E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A13E;
    16
}
/// $05A28E: bsr.w $5a1b6
pub fn i_05a28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A292;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5A292);
    cpu.pc = 0x5A1B6;
    18
}
/// $05A292: jsr $2783a.l
pub fn i_05a292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A298);
    cpu.pc = 0x2783A;
    16
}
/// $05A298: jsr $28d70.l
pub fn i_05a298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A29E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A29E);
    cpu.pc = 0x28D70;
    16
}
/// $05A29E: move.w $76(a6), d0
pub fn i_05a29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2A2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A2A2: move.w $24(a6), d1
pub fn i_05a2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2A6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A2A6: move.b $72(a6), d2
pub fn i_05a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2AA;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05A2AA: jsr $8f3be.l
pub fn i_05a2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A2B0);
    cpu.pc = 0x8F3BE;
    16
}
/// $05A2B0: tst.b d4
pub fn i_05a2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2B2;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05A2B2: bmi.w $5a2c2
pub fn i_05a2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2B6;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5A2C2; return 10; }
    8
}
/// $05A2B6: move.b #$0, $73(a6)
pub fn i_05a2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05A2BC: lea.l $5a2c6(pc), a1
pub fn i_05a2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A2C6;
    16
}
/// $05A2C0: move.l a1, (a6)
pub fn i_05a2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A2C2: bra.w $5a23e
pub fn i_05a2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A23E;
    10
}
// ===== Gunner2_Acquire_05a2c6  ($05A2C6, 22 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A2C6: jsr $13600.l
pub fn i_05a2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A2CC);
    cpu.pc = 0x13600;
    16
}
/// $05A2CC: move.b $73(a6), d4
pub fn i_05a2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2D0;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05A2D0: jsr $8f69c.l
pub fn i_05a2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A2D6);
    cpu.pc = 0x8F69C;
    16
}
/// $05A2D6: jsr $236e.l
pub fn i_05a2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A2DC);
    cpu.pc = 0x236E;
    16
}
// ===== Gunner2_Track_05a2dc  ($05A2DC, 172 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A2DC: move.w #$78, $74(a6)
pub fn i_05a2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2E2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A2E2: lea.l $5a2e8(pc), a1
pub fn i_05a2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A2E8;
    16
}
/// $05A2E6: move.l a1, (a6)
pub fn i_05a2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A2E8: cmpi.b #$2, $73(a6)
pub fn i_05a2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2EE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05A2EE: bne.w $5a320
pub fn i_05a2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5A320; return 10; }
    8
}
/// $05A2F2: jsr $5e136.l
pub fn i_05a2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A2F8);
    cpu.pc = 0x5E136;
    16
}
/// $05A2F8: lsl.w #$8, d0
pub fn i_05a2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2FA;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05A2FA: sub.w $70(a6), d0
pub fn i_05a2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A2FE;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05A2FE: asr.w #$4, d0
pub fn i_05a2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A300;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05A300: add.w d0, $70(a6)
pub fn i_05a300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A304;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A304: jsr $5e1aa.l
pub fn i_05a304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A30A);
    cpu.pc = 0x5E1AA;
    16
}
/// $05A30A: bcs.w $5a31c
pub fn i_05a30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A30E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5A31C; return 10; }
    8
}
/// $05A30E: subq.w #$1, $74(a6)
pub fn i_05a30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A312;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A312: bne.w $5a31c
pub fn i_05a312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A316;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5A31C; return 10; }
    8
}
/// $05A316: lea.l $5a388(pc), a1
pub fn i_05a316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A31A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A388;
    16
}
/// $05A31A: move.l a1, (a6)
pub fn i_05a31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A31C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A31C: bra.w $5a370
pub fn i_05a31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A320;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A370;
    10
}
/// $05A320: cmpi.b #$0, $73(a6)
pub fn i_05a320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A326;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $05A326: bne.w $5a334
pub fn i_05a326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A32A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5A334; return 10; }
    8
}
/// $05A32A: lea.l $10e200.l, a2
pub fn i_05a32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A330;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05A330: bra.w $5a33a
pub fn i_05a330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A334;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A33A;
    10
}
/// $05A334: lea.l $10e206.l, a2
pub fn i_05a334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A33A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05A33A: move.b $2(a2), d0
pub fn i_05a33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A33E;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05A33E: andi.w #$f, d0
pub fn i_05a33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A342;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05A342: add.w d0, d0
pub fn i_05a342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A344;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05A344: lea.l $5d326.l, a0
pub fn i_05a344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A34A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D326;
    20
}
/// $05A34A: move.w (a0, d0.w), d0
pub fn i_05a34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A34E;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05A34E: cmpi.w #$ffff, d0
pub fn i_05a34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A352;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $05A352: beq.w $5a360
pub fn i_05a352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A356;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5A360; return 10; }
    8
}
/// $05A356: sub.w $70(a6), d0
pub fn i_05a356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A35A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05A35A: asr.w #$4, d0
pub fn i_05a35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A35C;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05A35C: add.w d0, $70(a6)
pub fn i_05a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A360;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A360: btst.b #$4, $3(a2)
pub fn i_05a360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A366;
    cpu.ir = 0x82A;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[2].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05A366: beq.w $5a370
pub fn i_05a366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A36A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5A370; return 10; }
    8
}
/// $05A36A: lea.l $5a388(pc), a1
pub fn i_05a36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A36E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A388;
    16
}
/// $05A36E: move.l a1, (a6)
pub fn i_05a36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A370;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A370: jsr $2783a.l
pub fn i_05a370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A376);
    cpu.pc = 0x2783A;
    16
}
/// $05A376: lea.l $5a152(pc), a0
pub fn i_05a376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A37A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A152;
    16
}
/// $05A37A: bsr.w $5a1b6
pub fn i_05a37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A37E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5A37E);
    cpu.pc = 0x5A1B6;
    18
}
/// $05A37E: jsr $28d70.l
pub fn i_05a37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A384);
    cpu.pc = 0x28D70;
    16
}
/// $05A384: bra.w $5a3ec
pub fn i_05a384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A388;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A3EC;
    10
}
// ===== Gunner2_Fire_05a388  ($05A388, 134 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A388: lea.l $5a166(pc), a0
pub fn i_05a388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A38C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A166;
    16
}
/// $05A38C: bsr.w $5a1b6
pub fn i_05a38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A390;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5A390);
    cpu.pc = 0x5A1B6;
    18
}
/// $05A390: lea.l $5a396(pc), a1
pub fn i_05a390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A394;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A396;
    16
}
/// $05A394: move.l a1, (a6)
pub fn i_05a394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A396;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A396: jsr $2783a.l
pub fn i_05a396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A39C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A39C);
    cpu.pc = 0x2783A;
    16
}
/// $05A39C: jsr $28d70.l
pub fn i_05a39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A3A2);
    cpu.pc = 0x28D70;
    16
}
/// $05A3A2: bcc.w $5a3ec
pub fn i_05a3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5A3EC; return 10; }
    8
}
/// $05A3A6: move.l a6, -(a7)
pub fn i_05a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3A8;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05A3A8: lea.l $100800.l, a6
pub fn i_05a3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3AE;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $05A3AE: lea.l $5a55a(pc), a1
pub fn i_05a3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A55A;
    16
}
/// $05A3B2: jsr $4ae.l
pub fn i_05a3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A3B8);
    cpu.pc = 0x4AE;
    16
}
/// $05A3B8: movea.l (a7)+, a6
pub fn i_05a3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3BA;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05A3BA: move.b $73(a6), $98(a0)
pub fn i_05a3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3C0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05A3C0: move.w $78(a6), $22(a0)
pub fn i_05a3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3C6;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05A3C6: move.w $7a(a6), $24(a0)
pub fn i_05a3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3CC;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05A3CC: move.b $3a(a6), $3a(a0)
pub fn i_05a3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3D2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05A3D2: move.w $7c(a6), $5c(a0)
pub fn i_05a3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3D8;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05A3D8: move.w $70(a6), d0
pub fn i_05a3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3DC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A3DC: ble.w $5a3e2
pub fn i_05a3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3E0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5A3E2; return 10; }
    8
}
/// $05A3E0: neg.w d0
pub fn i_05a3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3E2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05A3E2: move.w d0, $34(a0)
pub fn i_05a3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3E6;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A3E6: lea.l $5a2dc(pc), a1
pub fn i_05a3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A2DC;
    16
}
/// $05A3EA: move.l a1, (a6)
pub fn i_05a3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A3EC: move.w $76(a6), d0
pub fn i_05a3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3F0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A3F0: move.w $24(a6), d1
pub fn i_05a3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3F4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A3F4: move.b $72(a6), d2
pub fn i_05a3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3F8;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05A3F8: jsr $8f3be.l
pub fn i_05a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A3FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A3FE);
    cpu.pc = 0x8F3BE;
    16
}
/// $05A3FE: tst.b d4
pub fn i_05a3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A400;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05A400: bpl.w $5a40a
pub fn i_05a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A404;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5A40A; return 10; }
    8
}
/// $05A404: lea.l $5a28a(pc), a1
pub fn i_05a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A408;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A28A;
    16
}
/// $05A408: move.l a1, (a6)
pub fn i_05a408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A40A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A40A: bra.w $5a23e
pub fn i_05a40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A40E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A23E;
    10
}
// ===== Shell_HitboxList_A_05a40e  ($05A40E, 84 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A40E: ?
pub fn i_05a40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A412;
    cpu.ir = 0xE;
    let v1 = cpu.a[6] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[6] = r2;
    8
}
/// $05A412: andi.b #$0, d4
pub fn i_05a412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A416;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A416: ori.b #$f8, d0
pub fn i_05a416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A41A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xF8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A41A: ?
pub fn i_05a41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A41E;
    cpu.ir = 0x8;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = (v1 | 0xF8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[0] = r2;
    8
}
/// $05A41E: ?
pub fn i_05a41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A422;
    cpu.ir = 0x8;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[0] = r2;
    8
}
/// $05A422: ori.b #$3a, -(a4)
pub fn i_05a422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A426;
    cpu.ir = 0x24;
    let ad1 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x3A) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $05A426: dc.w $ffff
pub fn i_05a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A428;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Shell_HitboxList_B_05a462  ($05A462, 164 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A462: btst.l d1, d6
pub fn i_05a462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A464;
    cpu.ir = 0x306;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A464: ori.b #$4, (a2, a7.l * 8)
pub fn i_05a464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A46A;
    cpu.ir = 0x32;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add(cpu.a[7]);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $05A46A: ori.b #$f0, d0
pub fn i_05a46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A46E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A46E: ori.b #$f0, (a0)
pub fn i_05a46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A472;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A472: ori.b #$0, (a0)
pub fn i_05a472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A476;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A476: ori.b #$3a, -(a4)
pub fn i_05a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A47A;
    cpu.ir = 0x24;
    let ad1 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x3A) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $05A47A: dc.w $ffff
pub fn i_05a47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A47C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Shell_HitboxList_C_05a506  ($05A506, 84 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A506: ori.b #$ff, d1
pub fn i_05a506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A50A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A50A: dc.w $ffff
pub fn i_05a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A50C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Gunner2_Shell_05a55a  ($05A55A, 226 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A55A: cmpi.b #$2, $98(a6)
pub fn i_05a55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A560;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05A560: bne.w $5a584
pub fn i_05a560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A564;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5A584; return 10; }
    8
}
/// $05A564: lea.l $5a506(pc), a0
pub fn i_05a564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A568;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A506;
    16
}
/// $05A568: move.l a0, $48(a6)
pub fn i_05a568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A56C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05A56C: lea.l $5a40e(pc), a0
pub fn i_05a56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A570;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A40E;
    16
}
/// $05A570: move.l a0, $4c(a6)
pub fn i_05a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A574;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05A574: jsr $283ca.l
pub fn i_05a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A57A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A57A);
    cpu.pc = 0x283CA;
    16
}
/// $05A57A: move.w #$200, $36(a6)
pub fn i_05a57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A580;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A580: bra.w $5a5a0
pub fn i_05a580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A584;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A5A0;
    10
}
/// $05A584: lea.l $ffff.w, a0
pub fn i_05a584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A588;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $05A588: move.l a0, $48(a6)
pub fn i_05a588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A58C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05A58C: lea.l $5a462(pc), a0
pub fn i_05a58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A590;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A462;
    16
}
/// $05A590: move.l a0, $4c(a6)
pub fn i_05a590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A594;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05A594: jsr $283ca.l
pub fn i_05a594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A59A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A59A);
    cpu.pc = 0x283CA;
    16
}
/// $05A59A: move.w #$400, $36(a6)
pub fn i_05a59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A5A0: move.w #$1064, d0
pub fn i_05a5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A5A4: jsr $2352.l
pub fn i_05a5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A5AA);
    cpu.pc = 0x2352;
    16
}
/// $05A5AA: move.w #$57, d1
pub fn i_05a5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x57;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A5AE: jsr $236e.l
pub fn i_05a5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A5B4);
    cpu.pc = 0x236E;
    16
}
/// $05A5B4: bset.b #$4, $6b(a6)
pub fn i_05a5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5BA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05A5BA: move.w #$d000, $38(a6)
pub fn i_05a5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A5C0: move.w $36(a6), d1
pub fn i_05a5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5C4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A5C4: moveq #$0, d0
pub fn i_05a5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5C6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A5C6: move.b $34(a6), d0
pub fn i_05a5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5CA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05A5CA: andi.b #$f0, d0
pub fn i_05a5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5CE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A5CE: jsr $13c0e.l
pub fn i_05a5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A5D4);
    cpu.pc = 0x13C0E;
    16
}
/// $05A5D4: move.w d1, $28(a6)
pub fn i_05a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5D8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A5D8: move.w d2, $2a(a6)
pub fn i_05a5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5DC;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A5DC: lea.l $5a17a(pc), a0
pub fn i_05a5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A17A;
    16
}
/// $05A5E0: move.w $5c(a6), d0
pub fn i_05a5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5E4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A5E4: movea.l (a0, d0.w), a0
pub fn i_05a5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5E8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $05A5E8: jsr $28cd4.l
pub fn i_05a5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A5EE);
    cpu.pc = 0x28CD4;
    16
}
/// $05A5EE: lea.l $5a5f4(pc), a1
pub fn i_05a5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A5F4;
    16
}
/// $05A5F2: move.l a1, (a6)
pub fn i_05a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A5F4: jsr $27cee.l
pub fn i_05a5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A5FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A5FA);
    cpu.pc = 0x27CEE;
    16
}
/// $05A5FA: jsr $28d70.l
pub fn i_05a5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A600);
    cpu.pc = 0x28D70;
    16
}
/// $05A600: jsr $283d8.l
pub fn i_05a600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A606;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A606);
    cpu.pc = 0x283D8;
    16
}
/// $05A606: btst.b #$1, $13(a6)
pub fn i_05a606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A60C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05A60C: bne.w $5a642
pub fn i_05a60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A610;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5A642; return 10; }
    8
}
/// $05A610: jsr $2870a.l
pub fn i_05a610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A616;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A616);
    cpu.pc = 0x2870A;
    16
}
/// $05A616: bcs.w $5a642
pub fn i_05a616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A61A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5A642; return 10; }
    8
}
/// $05A61A: move.w $22(a6), d0
pub fn i_05a61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A61E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A61E: addi.w #$10, d0
pub fn i_05a61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A622;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05A622: cmpi.w #$160, d0
pub fn i_05a622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A626;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x160, Size::Word, false);
    8
}
/// $05A626: bcc.w $5a63c
pub fn i_05a626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A62A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5A63C; return 10; }
    8
}
/// $05A62A: move.w $24(a6), d0
pub fn i_05a62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A62E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A62E: subi.w #$f0, d0
pub fn i_05a62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A632;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xF0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05A632: cmpi.w #$120, d0
pub fn i_05a632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A636;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $05A636: bcc.w $5a63c
pub fn i_05a636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A63A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5A63C; return 10; }
    8
}
/// $05A63A: rts
pub fn i_05a63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A63C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_05a63c  ($05A63C, 6 B, thunks/jmp_abs_thunks.c)
/// $05A63C: jmp $518.l
pub fn i_05a63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A642;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Gunner2_Shell_Explode_05a642  ($05A642, 26 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A642: move.w #$1027, d0
pub fn i_05a642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A646;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A646: jsr $2352.l
pub fn i_05a646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A64C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A64C);
    cpu.pc = 0x2352;
    16
}
/// $05A64C: jsr $13600.l
pub fn i_05a64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A652);
    cpu.pc = 0x13600;
    16
}
/// $05A652: lea.l $77f6a.l, a1
pub fn i_05a652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A658;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $05A658: move.l a1, (a6)
pub fn i_05a658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A65A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A65A: rts
pub fn i_05a65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A65C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner2_Child_Init_05a65c  ($05A65C, 10 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A65C: move.w #$56, d1
pub fn i_05a65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A660;
    cpu.ir = 0x323C;
    let v1: u32 = 0x56;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A660: jsr $236e.l
pub fn i_05a660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A666);
    cpu.pc = 0x236E;
    16
}
// ===== SetTaskHandler_05a666  ($05A666, 8 B, kernel/task_handlers.c)
/// $05A666: lea.l $5a66e(pc), a1
pub fn i_05a666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A66A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A66E;
    16
}
/// $05A66A: move.l a1, (a6)
pub fn i_05a66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A66C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A66C: rts
pub fn i_05a66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A66E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunner2_Child_Sync_05a66e  ($05A66E, 40 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A66E: movea.l $c(a6), a0
pub fn i_05a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A672;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05A672: move.b $3a(a0), $3a(a6)
pub fn i_05a672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A678;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05A678: move.w $22(a0), $22(a6)
pub fn i_05a678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A67E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05A67E: move.w $24(a0), $24(a6)
pub fn i_05a67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A684;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05A684: move.w $38(a0), $38(a6)
pub fn i_05a684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A68A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05A68A: move.l $5c(a0), $3c(a6)
pub fn i_05a68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A690;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $05A690: jmp $5ca2a.l
pub fn i_05a690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A696;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CA2A;
    8
}
// ===== Walker_Init_05a72e  ($05A72E, 46 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A72E: move.w #$e, d1
pub fn i_05a72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A732;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A732: jsr $236e.l
pub fn i_05a732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A738;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A738);
    cpu.pc = 0x236E;
    16
}
/// $05A738: move.w #$8000, d0
pub fn i_05a738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A73C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A73C: jsr $28134.l
pub fn i_05a73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A742);
    cpu.pc = 0x28134;
    16
}
/// $05A742: andi.w #$ffe3, $38(a6)
pub fn i_05a742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A748;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $05A748: ori.w #$14, $38(a6)
pub fn i_05a748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A74E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $05A74E: lea.l $5a696(pc), a0
pub fn i_05a74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A752;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5A696;
    16
}
/// $05A752: jsr $28cd4.l
pub fn i_05a752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A758);
    cpu.pc = 0x28CD4;
    16
}
/// $05A758: clr.w $28(a6)
pub fn i_05a758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A75C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetTaskHandler_05a75c  ($05A75C, 8 B, kernel/task_handlers.c)
/// $05A75C: lea.l $5a764(pc), a1
pub fn i_05a75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A760;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A764;
    16
}
/// $05A760: move.l a1, (a6)
pub fn i_05a760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A762;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A762: rts
pub fn i_05a762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A764;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_Patrol_05a764  ($05A764, 102 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A764: jsr $2783a.l
pub fn i_05a764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A76A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A76A);
    cpu.pc = 0x2783A;
    16
}
/// $05A76A: jsr $27eba.l
pub fn i_05a76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A770);
    cpu.pc = 0x27EBA;
    16
}
/// $05A770: bcc.w $5a77e
pub fn i_05a770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A774;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5A77E; return 10; }
    8
}
/// $05A774: jsr $27c8c.l
pub fn i_05a774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A77A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A77A);
    cpu.pc = 0x27C8C;
    16
}
/// $05A77A: bra.w $5a784
pub fn i_05a77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A77E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A784;
    10
}
/// $05A77E: jsr $27a92.l
pub fn i_05a77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A784;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A784);
    cpu.pc = 0x27A92;
    16
}
/// $05A784: jsr $8f470.l
pub fn i_05a784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A78A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A78A);
    cpu.pc = 0x8F470;
    16
}
/// $05A78A: bcc.w $5a7a4
pub fn i_05a78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A78E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5A7A4; return 10; }
    8
}
/// $05A78E: tst.w d2
pub fn i_05a78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A790;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05A790: bmi.w $5a7a4
pub fn i_05a790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A794;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5A7A4; return 10; }
    8
}
/// $05A794: move.b d2, $72(a6)
pub fn i_05a794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A798;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05A798: lea.l $5a7ca(pc), a1
pub fn i_05a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A79C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A7CA;
    16
}
/// $05A79C: move.l a1, (a6)
pub fn i_05a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A79E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A79E: jsr $8f520.l
pub fn i_05a79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A7A4);
    cpu.pc = 0x8F520;
    16
}
/// $05A7A4: btst.b #$5, $5a(a6)
pub fn i_05a7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7AA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05A7AA: beq.w $5a7b8
pub fn i_05a7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5A7B8; return 10; }
    8
}
/// $05A7AE: bchg.b #$0, $3a(a6)
pub fn i_05a7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7B4;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05A7B4: neg.w $28(a6)
pub fn i_05a7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7B8;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A7B8: jsr $28d70.l
pub fn i_05a7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A7BE);
    cpu.pc = 0x28D70;
    16
}
/// $05A7BE: jsr $49fd0.l
pub fn i_05a7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A7C4);
    cpu.pc = 0x49FD0;
    16
}
/// $05A7C4: jmp $56e1e.l
pub fn i_05a7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x56E1E;
    8
}
// ===== Walker_Claimed_05a7ca  ($05A7CA, 30 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A7CA: lea.l $29b4a4.l, a0
pub fn i_05a7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B4A4;
    20
}
/// $05A7D0: move.l a0, $48(a6)
pub fn i_05a7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7D4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05A7D4: lea.l $5a7da(pc), a1
pub fn i_05a7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A7DA;
    16
}
/// $05A7D8: move.l a1, (a6)
pub fn i_05a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A7DA: move.b $72(a6), d0
pub fn i_05a7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7DE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05A7DE: jsr $8f5dc.l
pub fn i_05a7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A7E4);
    cpu.pc = 0x8F5DC;
    16
}
/// $05A7E4: bcs.w $5a7f4
pub fn i_05a7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5A7F4; return 10; }
    8
}
// ===== SetTaskHandler_05a7e8  ($05A7E8, 8 B, kernel/task_handlers.c)
/// $05A7E8: lea.l $5a764(pc), a1
pub fn i_05a7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A764;
    16
}
/// $05A7EC: move.l a1, (a6)
pub fn i_05a7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05A7EE: rts
pub fn i_05a7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_Tail_05a7f0  ($05A7F0, 24 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A7F0: bra.w $5a7fc
pub fn i_05a7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A7FC;
    10
}
/// $05A7F4: move.w d0, $22(a6)
pub fn i_05a7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7F8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A7F8: move.w d1, $24(a6)
pub fn i_05a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A7FC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A7FC: jsr $49fd0.l
pub fn i_05a7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5A802);
    cpu.pc = 0x49FD0;
    16
}
/// $05A802: jmp $56e1e.l
pub fn i_05a802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A808;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x56E1E;
    8
}
// ===== Entity_CmpDepthToParent_05a808  ($05A808, 16 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A808: movea.l $8(a6), a1
pub fn i_05a808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A80C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05A80C: move.b $10(a6), d0
pub fn i_05a80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A810;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05A810: cmp.b $10(a1), d0
pub fn i_05a810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A814;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05A814: bcs.w $5a81e
pub fn i_05a814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A818;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5A81E; return 10; }
    8
}
// ===== ClearXN_05a818  ($05A818, 6 B, thunks/ccr_helpers.c)
/// $05A818: andi.b #$ee, ccr
pub fn i_05a818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A81C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05A81C: rts
pub fn i_05a81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A81E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05a81e  ($05A81E, 6 B, thunks/ccr_helpers.c)
/// $05A81E: ori.b #$11, ccr
pub fn i_05a81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A822;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05A822: rts
pub fn i_05a822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A824;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VRAM_FixLayerAutoclear_05A824  ($05A824, 150 B, video/fix_layer_autoclear_05a824.s)
/// $05A824: moveq #$0, d2
pub fn i_05a824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A826;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A826: move.w #$1, $3c0004.l
pub fn i_05a826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A82E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0004, v1 as u16);
    16
}
/// $05A82E: move.w #$8200, $3c0000.l
pub fn i_05a82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A836;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x8200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $05A836: movea.l #$3c0002, a0
pub fn i_05a836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A83C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x3C0002;
    cpu.a[0] = v1;
    4
}
/// $05A83C: move.w #$17c, d0
pub fn i_05a83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A840;
    cpu.ir = 0x303C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A840: move.w d2, (a0)
pub fn i_05a840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A842;
    cpu.ir = 0x3082;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $05A842: move.b d0, d0
pub fn i_05a842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A844;
    cpu.ir = 0x1000;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05A844: dbra d0, $5a840
pub fn i_05a844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A848;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5A848; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5A848; return 14; }
    cpu.pc = 0x5A840;
    10
}
/// $05A848: move.w #$0, d1
pub fn i_05a848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A84C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A84C: moveq #$0, d2
pub fn i_05a84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A84E;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A84E: move.w #$1, $3c0004.l
pub fn i_05a84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A856;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0004, v1 as u16);
    16
}
/// $05A856: move.w #$0, $3c0000.l
pub fn i_05a856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A85E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $05A85E: movea.l #$3c0002, a0
pub fn i_05a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A864;
    cpu.ir = 0x207C;
    let v1: u32 = 0x3C0002;
    cpu.a[0] = v1;
    4
}
/// $05A864: move.w #$2f9f, d0
pub fn i_05a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A868;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2F9F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05A868: nop
pub fn i_05a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A86A;
    cpu.ir = 0x4E71;
    4
}
/// $05A86A: nop
pub fn i_05a86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A86C;
    cpu.ir = 0x4E71;
    4
}
/// $05A86C: move.w d1, (a0)
pub fn i_05a86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A86E;
    cpu.ir = 0x3081;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $05A86E: nop
pub fn i_05a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A870;
    cpu.ir = 0x4E71;
    4
}
/// $05A870: nop
pub fn i_05a870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A872;
    cpu.ir = 0x4E71;
    4
}
/// $05A872: move.w d2, (a0)
pub fn i_05a872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A874;
    cpu.ir = 0x3082;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $05A874: dbra d0, $5a868
pub fn i_05a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A878;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5A878; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5A878; return 14; }
    cpu.pc = 0x5A868;
    10
}
/// $05A878: clr.w $10e1fa.l
pub fn i_05a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A87E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1FA) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1FA, r2 as u16);
    16
}
/// $05A87E: clr.w $10e1f6.l
pub fn i_05a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A884;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1F6) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1F6, r2 as u16);
    16
}
/// $05A884: clr.w $10e1f8.l
pub fn i_05a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A88A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1F8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1F8, r2 as u16);
    16
}
/// $05A88A: lea.l $108080.l, a5
pub fn i_05a88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A890;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A890: clr.b $616c(a5)
pub fn i_05a890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A894;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0x616C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A894: clr.w $6168(a5)
pub fn i_05a894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A898;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x6168);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A898: clr.w $616a(a5)
pub fn i_05a898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A89C;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x616A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A89C: lea.l $614c(a5), a0
pub fn i_05a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8A0;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x614C);
    cpu.a[0] = ad1;
    16
}
/// $05A8A0: clr.b $8(a0)
pub fn i_05a8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8A4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A8A4: move.w #$ffff, $4(a0)
pub fn i_05a8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8AA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A8AA: lea.l $6158(a5), a0
pub fn i_05a8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8AE;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x6158);
    cpu.a[0] = ad1;
    16
}
/// $05A8AE: clr.b $8(a0)
pub fn i_05a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8B2;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A8B2: move.w #$ffff, $4(a0)
pub fn i_05a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8B8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A8B8: rts
pub fn i_05a8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FadeLut_16x16_05a8ba  ($05A8BA, 256 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A8BA: movep.w $f0f(a7), d7
pub fn i_05a8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8BE;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A8BE: movep.w $f0f(a7), d7
pub fn i_05a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8C2;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A8C2: movep.w $f0f(a7), d7
pub fn i_05a8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8C6;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A8C6: movep.w $f0f(a7), d7
pub fn i_05a8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8CA;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A8CA: ?
pub fn i_05a8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8CE;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A8CE: ?
pub fn i_05a8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8D2;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A8D2: movep.w $f0f(a7), d7
pub fn i_05a8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8D6;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A8D6: movep.w $f0f(a7), d7
pub fn i_05a8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8DA;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A8DA: subi.b #$4, d4
pub fn i_05a8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8DE;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A8DE: ?
pub fn i_05a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8E2;
    cpu.ir = 0x408;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x8, Size::Byte, true);
    cpu.a[0] = r2;
    8
}
/// $05A8E2: ?
pub fn i_05a8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8E6;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A8E6: movep.w $f0f(a7), d7
pub fn i_05a8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8EA;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A8EA: subi.b #$4, d4
pub fn i_05a8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8EE;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A8EE: ?
pub fn i_05a8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8F2;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A8F2: ?
pub fn i_05a8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8F6;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $05A8F6: movep.w $f0f(a7), d7
pub fn i_05a8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8FA;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A8FA: andi.b #$4, d2
pub fn i_05a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8FE;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A8FE: subi.b #$8, d4
pub fn i_05a8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A902;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x8, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A902: ?
pub fn i_05a902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A906;
    cpu.ir = 0x80C;
    let mk: u32 = 1u32 << ((12) % 32);
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A906: movep.w $f0f(a7), d7
pub fn i_05a906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A90A;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $05A90A: andi.b #$4, d2
pub fn i_05a90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A90E;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A90E: ?
pub fn i_05a90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A912;
    cpu.ir = 0x408;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x8, Size::Byte, true);
    cpu.a[0] = r2;
    8
}
/// $05A912: ?
pub fn i_05a912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A916;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $05A916: ?
pub fn i_05a916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A91A;
    cpu.ir = 0xC0F;
    let v1 = cpu.a[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $05A91A: andi.b #$4, d2
pub fn i_05a91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A91E;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A91E: addi.b #$8, d6
pub fn i_05a91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A922;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A922: ?
pub fn i_05a922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A926;
    cpu.ir = 0x80A;
    let mk: u32 = 1u32 << ((12) % 32);
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A926: ?
pub fn i_05a926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A92A;
    cpu.ir = 0xC0F;
    let v1 = cpu.a[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $05A92A: andi.b #$4, d2
pub fn i_05a92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A92E;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A92E: addi.b #$8, d6
pub fn i_05a92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A932;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A932: ?
pub fn i_05a932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A936;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $05A936: ?
pub fn i_05a936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A93A;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $05A93A: ori.b #$4, d2
pub fn i_05a93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A93E;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A93E: subi.b #$8, d6
pub fn i_05a93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A942;
    cpu.ir = 0x406;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x8, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A942: ?
pub fn i_05a942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A946;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $05A946: ?
pub fn i_05a946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A94A;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $05A94A: ori.b #$4, d2
pub fn i_05a94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A94E;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A94E: addi.b #$7, d6
pub fn i_05a94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A952;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A952: ?
pub fn i_05a952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A956;
    cpu.ir = 0x80A;
    let mk: u32 = 1u32 << ((12) % 32);
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A956: ?
pub fn i_05a956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A95A;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $05A95A: ori.b #$3, d2
pub fn i_05a95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A95E;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A95E: subi.b #$7, d6
pub fn i_05a95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A962;
    cpu.ir = 0x406;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A962: ?
pub fn i_05a962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A966;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((12) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A966: ?
pub fn i_05a966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A96A;
    cpu.ir = 0xC0E;
    let v1 = cpu.a[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $05A96A: ori.b #$3, d2
pub fn i_05a96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A96E;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A96E: subi.b #$7, d6
pub fn i_05a96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A972;
    cpu.ir = 0x406;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A972: ?
pub fn i_05a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A976;
    cpu.ir = 0x80A;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A976: ?
pub fn i_05a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A97A;
    cpu.ir = 0xC0E;
    let v1 = cpu.a[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $05A97A: ori.b #$3, d1
pub fn i_05a97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A97E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A97E: subi.b #$7, d4
pub fn i_05a97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A982;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A982: ?
pub fn i_05a982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A986;
    cpu.ir = 0x80A;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A986: ?
pub fn i_05a986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A98A;
    cpu.ir = 0xC0E;
    let v1 = cpu.a[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $05A98A: ori.b #$3, d1
pub fn i_05a98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A98E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A98E: subi.b #$7, d6
pub fn i_05a98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A992;
    cpu.ir = 0x406;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A992: ?
pub fn i_05a992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A996;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A996: ?
pub fn i_05a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A99A;
    cpu.ir = 0xC0E;
    let v1 = cpu.a[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $05A99A: ori.b #$3, d1
pub fn i_05a99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A99E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A99E: subi.b #$7, d5
pub fn i_05a99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9A2;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9A2: ?
pub fn i_05a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9A6;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A9A6: ?
pub fn i_05a9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9AA;
    cpu.ir = 0xC0E;
    let v1 = cpu.a[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $05A9AA: ori.b #$3, d1
pub fn i_05a9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9AE;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9AE: subi.b #$7, d5
pub fn i_05a9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9B2;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9B2: ?
pub fn i_05a9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9B6;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05A9B6: ?
pub fn i_05a9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9BA;
    cpu.ir = 0xC0D;
    let v1 = cpu.a[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
// ===== VRAM_FixAutoclear_Reset_05a9ba  ($05A9BA, 28 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A9BA: lea.l $108080.l, a5
pub fn i_05a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9C0;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A9C0: clr.b $10e1ec.l
pub fn i_05a9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9C6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E1EC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E1EC, r2 as u8);
    16
}
/// $05A9C6: clr.w $4254(a5)
pub fn i_05a9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9CA;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x4254);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A9CA: clr.w $6148(a5)
pub fn i_05a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9CE;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x6148);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A9CE: move.w #$348, $614a(a5)
pub fn i_05a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9D4;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x348;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A9D4: rts
pub fn i_05a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_Dispatch_05A9D6  ($05A9D6, 192 B, video/sprite_dispatch_dual_05a9d6.s)
/// $05A9D6: andi.b #$3, d5
pub fn i_05a9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9DA;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9DA: ori.b #$4, d5
pub fn i_05a9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9DE;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9DE: bra.w $5a9e6
pub fn i_05a9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A9E6;
    10
}
/// $05A9E2: andi.b #$3, d5
pub fn i_05a9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9E6;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9E6: jsr $5b1b2(pc)
pub fn i_05a9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5A9EA);
    cpu.pc = 0x5B1B2;
    16
}
/// $05A9EA: move a1, usp
pub fn i_05a9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9EC;
    cpu.ir = 0x4E61;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.usp = cpu.a[1];
    4
}
/// $05A9EC: lea.l $108080.l, a5
pub fn i_05a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A9F2: move.l a0, d7
pub fn i_05a9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F4;
    cpu.ir = 0x2E08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A9F4: bmi.w $5aa92
pub fn i_05a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F8;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05A9F8: swap d1
pub fn i_05a9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9FA;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A9FA: move.w $4254(a5), d1
pub fn i_05a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9FE;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A9FE: cmpi.w #$11d0, d1
pub fn i_05a9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA02;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x11D0, Size::Word, false);
    8
}
/// $05AA02: bcc.w $5aa92
pub fn i_05aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05AA06: move.w $6148(a5), d6
pub fn i_05aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA0A;
    cpu.ir = 0x3C2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA0A: move.w $614a(a5), d7
pub fn i_05aa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA0E;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA0E: cmp.w d7, d6
pub fn i_05aa0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA10;
    cpu.ir = 0xBC47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05AA10: bcc.w $5aa92
pub fn i_05aa10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05AA14: btst.b #$2, d5
pub fn i_05aa14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA18;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05AA18: bne.w $5aa2e
pub fn i_05aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA1C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AA2E; return 10; }
    8
}
/// $05AA1C: lea.l $5428(a5), a4
pub fn i_05aa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA20;
    cpu.ir = 0x49ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[4] = ad1;
    16
}
/// $05AA20: move.l d1, (a4, d6.w)
pub fn i_05aa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA24;
    cpu.ir = 0x2981;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05AA24: addq.w #$4, d6
pub fn i_05aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA26;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AA26: move.w d6, $6148(a5)
pub fn i_05aa26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA2A;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA2A: bra.w $5aa3c
pub fn i_05aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA3C;
    10
}
/// $05AA2E: subq.w #$4, d7
pub fn i_05aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA30;
    cpu.ir = 0x5947;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AA30: lea.l $5428(a5), a4
pub fn i_05aa30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA34;
    cpu.ir = 0x49ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[4] = ad1;
    16
}
/// $05AA34: move.l d1, (a4, d7.w)
pub fn i_05aa34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA38;
    cpu.ir = 0x2981;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05AA38: move.w d7, $614a(a5)
pub fn i_05aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA3C;
    cpu.ir = 0x3B47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA3C: move.w d5, -(a7)
pub fn i_05aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA3E;
    cpu.ir = 0x3F05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05AA3E: lsl.w #$8, d2
pub fn i_05aa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA40;
    cpu.ir = 0xE14A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AA40: lea.l $4258(a5), a3
pub fn i_05aa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA44;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05AA44: lea.l $11d0(a3), a2
pub fn i_05aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA48;
    cpu.ir = 0x45EB;
    let ad1 = cpu.a[3].wrapping_add(0x11D0);
    cpu.a[2] = ad1;
    16
}
/// $05AA48: adda.w d1, a3
pub fn i_05aa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4A;
    cpu.ir = 0xD6C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AA4A: move.b d3, d7
pub fn i_05aa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4C;
    cpu.ir = 0x1E03;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AA4C: and.b d4, d7
pub fn i_05aa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4E;
    cpu.ir = 0xCE04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 & cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AA4E: addq.b #$1, d7
pub fn i_05aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA50;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AA50: sne.b d7
pub fn i_05aa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA52;
    cpu.ir = 0x56C7;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[7] & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AA52: andi.w #$4, d7
pub fn i_05aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA56;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x4) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AA56: andi.b #$3, d5
pub fn i_05aa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AA5A: or.b d5, d7
pub fn i_05aa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5C;
    cpu.ir = 0x8E05;
    let v1 = cpu.d[5] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AA5C: add.w d7, d7
pub fn i_05aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5E;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AA5E: add.w d7, d7
pub fn i_05aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA60;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AA60: jsr $5aa96(pc, d7.w)
pub fn i_05aa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA64;
    cpu.ir = 0x4EBB;
    let ad1 = 0x5AA96_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.push32(bus, 0x5AA64);
    cpu.jump_to(ad1);
    16
}
/// $05AA64: move.w (a7)+, d5
pub fn i_05aa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA66;
    cpu.ir = 0x3A1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05AA66: lea.l $4258(a5), a2
pub fn i_05aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA6A;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AA6A: suba.l a2, a3
pub fn i_05aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA6C;
    cpu.ir = 0x97CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_sub(v1);
    8
}
/// $05AA6C: move.w $4254(a5), d0
pub fn i_05aa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA70;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA70: cmp.w a3, d0
pub fn i_05aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA72;
    cpu.ir = 0xB04B;
    let v1 = cpu.a[3] as i16 as i32 as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05AA72: beq.w $5aa7e
pub fn i_05aa72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AA7E; return 10; }
    8
}
/// $05AA76: move.w a3, $4254(a5)
pub fn i_05aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA7A;
    cpu.ir = 0x3B4B;
    let v1 = cpu.a[3] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA7A: bra.w $5aa92
pub fn i_05aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA92;
    10
}
/// $05AA7E: btst.b #$2, d5
pub fn i_05aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA82;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05AA82: bne.w $5aa8e
pub fn i_05aa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AA8E; return 10; }
    8
}
/// $05AA86: subq.w #$4, $6148(a5)
pub fn i_05aa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA8A;
    cpu.ir = 0x596D;
    let ad1 = cpu.a[5].wrapping_add(0x6148);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05AA8A: bra.w $5aa92
pub fn i_05aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA92;
    10
}
/// $05AA8E: addq.w #$4, $614a(a5)
pub fn i_05aa8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA92;
    cpu.ir = 0x586D;
    let ad1 = cpu.a[5].wrapping_add(0x614A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05AA92: move usp, a1
pub fn i_05aa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA94;
    cpu.ir = 0x4E69;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[1] = cpu.usp;
    4
}
/// $05AA94: rts
pub fn i_05aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteDispatchJT_05AA96  ($05AA96, 1820 B, video/sprite_queue_render_05aa96.s)
/// $05AA96: bra.w $5aab6
pub fn i_05aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AAB6;
    10
}
/// $05AA9A: bra.w $5ab34
pub fn i_05aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AB34;
    10
}
/// $05AA9E: bra.w $5abb8
pub fn i_05aa9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5ABB8;
    10
}
/// $05AAA2: bra.w $5ac44
pub fn i_05aaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AC44;
    10
}
/// $05AAA6: bra.w $5acd6
pub fn i_05aaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5ACD6;
    10
}
/// $05AAAA: bra.w $5ae00
pub fn i_05aaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AE00;
    10
}
/// $05AAAE: bra.w $5af30
pub fn i_05aaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AF30;
    10
}
/// $05AAB2: bra.w $5b06e
pub fn i_05aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B06E;
    10
}
/// $05AAB6: cmpa.l a2, a3
pub fn i_05aab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AAB8: bcc.w $5ab32
pub fn i_05aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AABC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB32; return 10; }
    8
}
/// $05AABC: move.b (a0)+, -(a7)
pub fn i_05aabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AABE;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AABE: move.b (a0)+, d2
pub fn i_05aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC0;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AAC0: eor.b d5, d2
pub fn i_05aac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC2;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AAC2: move.w #$fff, d4
pub fn i_05aac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC6;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAC6: moveq #$0, d5
pub fn i_05aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC8;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAC8: move.l (a0)+, d1
pub fn i_05aac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACA;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AACA: moveq #$0, d5
pub fn i_05aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACC;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AACC: move.b (a0)+, d5
pub fn i_05aacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACE;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AACE: swap d5
pub fn i_05aace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD0;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAD0: move.b (a0)+, d5
pub fn i_05aad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD2;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AAD2: move.w d5, d6
pub fn i_05aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD4;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAD4: swap d6
pub fn i_05aad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD6;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAD6: move.w d5, d6
pub fn i_05aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD8;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAD8: add.w d0, d1
pub fn i_05aad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AADA: swap d1
pub fn i_05aada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADC;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AADC: swap d0
pub fn i_05aadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AADE: add.w d0, d1
pub fn i_05aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE0;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AAE0: swap d0
pub fn i_05aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE2;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAE2: andi.w #$1ff, d1
pub fn i_05aae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE6;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AAE6: lsl.l #$7, d1
pub fn i_05aae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE8;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AAE8: or.l d6, d1
pub fn i_05aae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEA;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAEA: clr.w d5
pub fn i_05aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEC;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AAEC: swap d5
pub fn i_05aaec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEE;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAEE: subq.b #$1, d5
pub fn i_05aaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF0;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AAF0: add.w d6, d6
pub fn i_05aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF2;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AAF2: move.w #$800, d3
pub fn i_05aaf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF6;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAF6: cmpi.w #$f800, d1
pub fn i_05aaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAFA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AAFA: bcc.w $5ab06
pub fn i_05aafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB06; return 10; }
    8
}
/// $05AAFE: cmpi.w #$a000, d1
pub fn i_05aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB02;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AB02: bcc.w $5ab14
pub fn i_05ab02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB14; return 10; }
    8
}
/// $05AB06: move.l a0, (a3)+
pub fn i_05ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB08;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB08: move.w d2, (a3)+
pub fn i_05ab08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0A;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB0A: move.w d4, (a3)+
pub fn i_05ab0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0C;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB0C: move.l d1, (a3)+
pub fn i_05ab0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0E;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB0E: cmpa.l a2, a3
pub fn i_05ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB10;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB10: bcc.w $5ab20
pub fn i_05ab10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB20; return 10; }
    8
}
/// $05AB14: add.w d3, d1
pub fn i_05ab14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB16;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB16: adda.w d6, a0
pub fn i_05ab16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB18;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AB18: dbra d5, $5aaf6
pub fn i_05ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB1C;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5AB1C; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5AB1C; return 14; }
    cpu.pc = 0x5AAF6;
    10
}
/// $05AB1C: subq.b #$1, (a7)
pub fn i_05ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB1E;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05AB1E: bne.b $5aac6
pub fn i_05ab1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB20;
    cpu.ir = 0x66A6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AAC6; return 10; }
    8
}
/// $05AB20: addq.w #$2, a7
pub fn i_05ab20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB22;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05AB22: lea.l $4258(a5), a2
pub fn i_05ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB26;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AB26: cmpa.l a2, a3
pub fn i_05ab26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB28;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB28: beq.w $5ab32
pub fn i_05ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB2C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AB32; return 10; }
    8
}
/// $05AB2C: ori.b #$80, -$6(a3)
pub fn i_05ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB32;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05AB32: rts
pub fn i_05ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AB34: cmpa.l a2, a3
pub fn i_05ab34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB36;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB36: bcc.w $5abb6
pub fn i_05ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ABB6; return 10; }
    8
}
/// $05AB3A: move.b (a0)+, -(a7)
pub fn i_05ab3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3C;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AB3C: move.b (a0)+, d2
pub fn i_05ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3E;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB3E: eor.b d5, d2
pub fn i_05ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB40;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AB40: move.w #$fff, d4
pub fn i_05ab40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB44;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB44: moveq #$0, d5
pub fn i_05ab44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB46;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB46: move.l (a0)+, d1
pub fn i_05ab46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB48;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB48: moveq #$0, d5
pub fn i_05ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB4A: move.b (a0)+, d5
pub fn i_05ab4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4C;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB4C: swap d5
pub fn i_05ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB4E: move.b (a0)+, d5
pub fn i_05ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB50;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB50: swap d1
pub fn i_05ab50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB52;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB52: not.w d1
pub fn i_05ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB54;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AB54: swap d1
pub fn i_05ab54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB56;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB56: move.w d5, d6
pub fn i_05ab56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB58;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB58: swap d6
pub fn i_05ab58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5A;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB5A: move.w d5, d6
pub fn i_05ab5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5C;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB5C: add.w d0, d1
pub fn i_05ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5E;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB5E: swap d1
pub fn i_05ab5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB60;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB60: swap d0
pub fn i_05ab60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB62;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB62: add.w d0, d1
pub fn i_05ab62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB64;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB64: swap d0
pub fn i_05ab64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB66;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB66: andi.w #$1ff, d1
pub fn i_05ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AB6A: lsl.l #$7, d1
pub fn i_05ab6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6C;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AB6C: or.l d6, d1
pub fn i_05ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6E;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB6E: clr.w d5
pub fn i_05ab6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB70;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AB70: swap d5
pub fn i_05ab70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB72;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB72: subq.b #$1, d5
pub fn i_05ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB74;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AB74: add.w d6, d6
pub fn i_05ab74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB76;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB76: move.w #$800, d3
pub fn i_05ab76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB7A;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB7A: sub.w d3, d1
pub fn i_05ab7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB7C;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB7C: cmpi.w #$f800, d1
pub fn i_05ab7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB80;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AB80: bcc.w $5ab8c
pub fn i_05ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB8C; return 10; }
    8
}
/// $05AB84: cmpi.w #$a000, d1
pub fn i_05ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB88;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AB88: bcc.w $5ab9a
pub fn i_05ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB9A; return 10; }
    8
}
/// $05AB8C: move.l a0, (a3)+
pub fn i_05ab8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB8E;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB8E: move.w d2, (a3)+
pub fn i_05ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB90;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB90: move.w d4, (a3)+
pub fn i_05ab90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB92;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB92: move.l d1, (a3)+
pub fn i_05ab92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB94;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB94: cmpa.l a2, a3
pub fn i_05ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB96;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB96: bcc.w $5aba4
pub fn i_05ab96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ABA4; return 10; }
    8
}
/// $05AB9A: adda.w d6, a0
pub fn i_05ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB9C;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AB9C: dbra d5, $5ab7a
pub fn i_05ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA0;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ABA0; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ABA0; return 14; }
    cpu.pc = 0x5AB7A;
    10
}
/// $05ABA0: subq.b #$1, (a7)
pub fn i_05aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA2;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ABA2: bne.b $5ab44
pub fn i_05aba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA4;
    cpu.ir = 0x66A0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AB44; return 10; }
    8
}
/// $05ABA4: addq.w #$2, a7
pub fn i_05aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA6;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ABA6: lea.l $4258(a5), a2
pub fn i_05aba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABAA;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05ABAA: cmpa.l a2, a3
pub fn i_05abaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABAC;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ABAC: beq.w $5abb6
pub fn i_05abac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ABB6; return 10; }
    8
}
/// $05ABB0: ori.b #$80, -$6(a3)
pub fn i_05abb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB6;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ABB6: rts
pub fn i_05abb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05ABB8: cmpa.l a2, a3
pub fn i_05abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABBA;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ABBA: bcc.w $5ac42
pub fn i_05abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC42; return 10; }
    8
}
/// $05ABBE: move.b (a0)+, -(a7)
pub fn i_05abbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC0;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05ABC0: move.b (a0)+, d2
pub fn i_05abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC2;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABC2: eor.b d5, d2
pub fn i_05abc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC4;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ABC4: move.w #$1fff, d4
pub fn i_05abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABC8: moveq #$0, d5
pub fn i_05abc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCA;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABCA: move.l (a0)+, d1
pub fn i_05abca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCC;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ABCC: moveq #$0, d5
pub fn i_05abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCE;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABCE: move.b (a0)+, d5
pub fn i_05abce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD0;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABD0: swap d5
pub fn i_05abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD2;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABD2: move.b (a0)+, d5
pub fn i_05abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD4;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABD4: neg.w d1
pub fn i_05abd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD6;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABD6: move.w d5, d6
pub fn i_05abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD8;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABD8: lsl.w #$4, d6
pub fn i_05abd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDA;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05ABDA: subq.w #$1, d6
pub fn i_05abda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDC;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABDC: add.w d6, d1
pub fn i_05abdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDE;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABDE: move.w d5, d6
pub fn i_05abde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE0;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABE0: swap d6
pub fn i_05abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE2;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABE2: move.w d5, d6
pub fn i_05abe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE4;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABE4: add.w d0, d1
pub fn i_05abe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE6;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABE6: swap d1
pub fn i_05abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE8;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABE8: swap d0
pub fn i_05abe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABEA: add.w d0, d1
pub fn i_05abea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEC;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABEC: swap d0
pub fn i_05abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABEE: andi.w #$1ff, d1
pub fn i_05abee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF2;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ABF2: lsl.l #$7, d1
pub fn i_05abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF4;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05ABF4: or.l d6, d1
pub fn i_05abf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF6;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABF6: clr.w d5
pub fn i_05abf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF8;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABF8: swap d5
pub fn i_05abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFA;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABFA: subq.b #$1, d5
pub fn i_05abfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFC;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ABFC: add.w d6, d6
pub fn i_05abfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFE;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABFE: adda.w d6, a0
pub fn i_05abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC00;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC00: move.w #$800, d3
pub fn i_05ac00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC04;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC04: cmpi.w #$f800, d1
pub fn i_05ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC08;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AC08: bcc.w $5ac14
pub fn i_05ac08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC14; return 10; }
    8
}
/// $05AC0C: cmpi.w #$a000, d1
pub fn i_05ac0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC10;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AC10: bcc.w $5ac22
pub fn i_05ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC22; return 10; }
    8
}
/// $05AC14: move.l a0, (a3)+
pub fn i_05ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC16;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC16: move.w d2, (a3)+
pub fn i_05ac16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC18;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AC18: move.w d4, (a3)+
pub fn i_05ac18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1A;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AC1A: move.l d1, (a3)+
pub fn i_05ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1C;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC1C: cmpa.l a2, a3
pub fn i_05ac1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1E;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC1E: bcc.w $5ac30
pub fn i_05ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC30; return 10; }
    8
}
/// $05AC22: add.w d3, d1
pub fn i_05ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC24;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC24: adda.w d6, a0
pub fn i_05ac24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC26;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC26: dbra d5, $5ac04
pub fn i_05ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2A;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5AC2A; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5AC2A; return 14; }
    cpu.pc = 0x5AC04;
    10
}
/// $05AC2A: suba.w d6, a0
pub fn i_05ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2C;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC2C: subq.b #$1, (a7)
pub fn i_05ac2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2E;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05AC2E: bne.b $5abc8
pub fn i_05ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC30;
    cpu.ir = 0x6698;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5ABC8; return 10; }
    8
}
/// $05AC30: addq.w #$2, a7
pub fn i_05ac30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC32;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05AC32: lea.l $4258(a5), a2
pub fn i_05ac32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC36;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AC36: cmpa.l a2, a3
pub fn i_05ac36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC38;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC38: beq.w $5ac42
pub fn i_05ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AC42; return 10; }
    8
}
/// $05AC3C: ori.b #$80, -$6(a3)
pub fn i_05ac3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC42;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05AC42: rts
pub fn i_05ac42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AC44: cmpa.l a2, a3
pub fn i_05ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC46;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC46: bcc.w $5acd4
pub fn i_05ac46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACD4; return 10; }
    8
}
/// $05AC4A: move.b (a0)+, -(a7)
pub fn i_05ac4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4C;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AC4C: move.b (a0)+, d2
pub fn i_05ac4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4E;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC4E: eor.b d5, d2
pub fn i_05ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC50;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AC50: move.w #$1fff, d4
pub fn i_05ac50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC54;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC54: moveq #$0, d5
pub fn i_05ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC56;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC56: move.l (a0)+, d1
pub fn i_05ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC58;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC58: moveq #$0, d5
pub fn i_05ac58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC5A: move.b (a0)+, d5
pub fn i_05ac5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5C;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC5C: swap d5
pub fn i_05ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC5E: move.b (a0)+, d5
pub fn i_05ac5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC60;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC60: swap d1
pub fn i_05ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC62;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC62: not.w d1
pub fn i_05ac62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC64;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC64: swap d1
pub fn i_05ac64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC66;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC66: neg.w d1
pub fn i_05ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC68;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC68: move.w d5, d6
pub fn i_05ac68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6A;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC6A: lsl.w #$4, d6
pub fn i_05ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6C;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AC6C: subq.w #$1, d6
pub fn i_05ac6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6E;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC6E: add.w d6, d1
pub fn i_05ac6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC70;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC70: move.w d5, d6
pub fn i_05ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC72;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC72: swap d6
pub fn i_05ac72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC74;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC74: move.w d5, d6
pub fn i_05ac74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC76;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC76: add.w d0, d1
pub fn i_05ac76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC78;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC78: swap d1
pub fn i_05ac78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7A;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC7A: swap d0
pub fn i_05ac7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC7C: add.w d0, d1
pub fn i_05ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7E;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC7E: swap d0
pub fn i_05ac7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC80;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC80: andi.w #$1ff, d1
pub fn i_05ac80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC84;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AC84: lsl.l #$7, d1
pub fn i_05ac84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC86;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AC86: or.l d6, d1
pub fn i_05ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC88;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC88: clr.w d5
pub fn i_05ac88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8A;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC8A: swap d5
pub fn i_05ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8C;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC8C: subq.b #$1, d5
pub fn i_05ac8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8E;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AC8E: add.w d6, d6
pub fn i_05ac8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC90;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC90: adda.w d6, a0
pub fn i_05ac90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC92;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC92: move.w #$800, d3
pub fn i_05ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC96;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC96: sub.w d3, d1
pub fn i_05ac96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC98;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC98: cmpi.w #$f800, d1
pub fn i_05ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC9C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AC9C: bcc.w $5aca8
pub fn i_05ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACA8; return 10; }
    8
}
/// $05ACA0: cmpi.w #$a000, d1
pub fn i_05aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05ACA4: bcc.w $5acb6
pub fn i_05aca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACB6; return 10; }
    8
}
/// $05ACA8: move.l a0, (a3)+
pub fn i_05aca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAA;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ACAA: move.w d2, (a3)+
pub fn i_05acaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAC;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ACAC: move.w d4, (a3)+
pub fn i_05acac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAE;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ACAE: move.l d1, (a3)+
pub fn i_05acae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB0;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ACB0: cmpa.l a2, a3
pub fn i_05acb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB2;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACB2: bcc.w $5acc2
pub fn i_05acb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACC2; return 10; }
    8
}
/// $05ACB6: adda.w d6, a0
pub fn i_05acb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB8;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ACB8: dbra d5, $5ac96
pub fn i_05acb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACBC;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ACBC; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ACBC; return 14; }
    cpu.pc = 0x5AC96;
    10
}
/// $05ACBC: suba.w d6, a0
pub fn i_05acbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACBE;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ACBE: subq.b #$1, (a7)
pub fn i_05acbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC0;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ACC0: bne.b $5ac54
pub fn i_05acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC2;
    cpu.ir = 0x6692;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AC54; return 10; }
    8
}
/// $05ACC2: addq.w #$2, a7
pub fn i_05acc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC4;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ACC4: lea.l $4258(a5), a2
pub fn i_05acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC8;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05ACC8: cmpa.l a2, a3
pub fn i_05acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACCA;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACCA: beq.w $5acd4
pub fn i_05acca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACCE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ACD4; return 10; }
    8
}
/// $05ACCE: ori.b #$80, -$6(a3)
pub fn i_05acce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD4;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ACD4: rts
pub fn i_05acd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05ACD6: cmpa.l a2, a3
pub fn i_05acd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACD8: bcc.w $5adfe
pub fn i_05acd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADFE; return 10; }
    8
}
/// $05ACDC: move.b (a0)+, -(a7)
pub fn i_05acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACDE;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05ACDE: move.b (a0)+, d2
pub fn i_05acde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE0;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ACE0: eor.b d5, d2
pub fn i_05ace0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE2;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ACE2: andi.w #$ff, d4
pub fn i_05ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE6;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ACE6: move.w d4, d7
pub fn i_05ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE8;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ACE8: addq.w #$1, d7
pub fn i_05ace8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEA;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ACEA: swap d7
pub fn i_05acea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEC;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ACEC: move.w d3, d7
pub fn i_05acec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEE;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ACEE: andi.w #$ff, d7
pub fn i_05acee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF2;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ACF2: addq.w #$1, d7
pub fn i_05acf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF4;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ACF4: lsl.w #$4, d4
pub fn i_05acf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF6;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05ACF6: eori.b #$f0, d4
pub fn i_05acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACFA;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05ACFA: move.b d4, $616e(a5)
pub fn i_05acfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACFE;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05ACFE: move.b d3, d4
pub fn i_05acfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD00;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD00: andi.b #$f0, d4
pub fn i_05ad00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD04;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AD04: move.w d4, d6
pub fn i_05ad04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD06;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD06: subi.w #$100, d4
pub fn i_05ad06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0A;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD0A: swap d4
pub fn i_05ad0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD0C: move.w d6, d4
pub fn i_05ad0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0E;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD0E: andi.w #$f, d3
pub fn i_05ad0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD12;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD12: swap d2
pub fn i_05ad12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD14;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD14: move.w d3, d2
pub fn i_05ad14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD16;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD16: swap d2
pub fn i_05ad16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD18;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD18: moveq #$0, d5
pub fn i_05ad18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD1A: move.l (a0)+, d1
pub fn i_05ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1C;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AD1C: moveq #$0, d5
pub fn i_05ad1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1E;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD1E: move.b (a0)+, d5
pub fn i_05ad1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD20;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AD20: swap d5
pub fn i_05ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD22;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD22: move.b (a0)+, d5
pub fn i_05ad22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD24;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AD24: moveq #$0, d3
pub fn i_05ad24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD26;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD26: move.l d1, d6
pub fn i_05ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD28;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD28: muls.w d7, d6
pub fn i_05ad28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2A;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD2A: spl.b d3
pub fn i_05ad2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2C;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AD2C: add.l d3, d6
pub fn i_05ad2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2E;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05AD2E: asr.l #$8, d6
pub fn i_05ad2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD30;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05AD30: swap d7
pub fn i_05ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD32;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD32: swap d1
pub fn i_05ad32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD34;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD34: muls.w d7, d1
pub fn i_05ad34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD36;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD36: move.b d1, $616d(a5)
pub fn i_05ad36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3A;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AD3A: lsl.l #$8, d1
pub fn i_05ad3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3C;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05AD3C: move.w d6, d1
pub fn i_05ad3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3E;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD3E: swap d7
pub fn i_05ad3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD40;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD40: move.w d5, d6
pub fn i_05ad40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD42;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD42: cmpi.b #$11, d6
pub fn i_05ad42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD46;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05AD46: bcc.w $5ad60
pub fn i_05ad46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AD60; return 10; }
    8
}
/// $05AD4A: subq.b #$1, d6
pub fn i_05ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4C;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AD4C: lsl.b #$4, d6
pub fn i_05ad4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4E;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05AD4E: swap d2
pub fn i_05ad4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD50;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD50: or.b d2, d6
pub fn i_05ad50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD52;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AD52: lea.l $5a8ba(pc), a4
pub fn i_05ad52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD56;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05AD56: move.b (a4, d6.w), d6
pub fn i_05ad56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD5A;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05AD5A: swap d2
pub fn i_05ad5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD5C;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD5C: bra.w $5ad66
pub fn i_05ad5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD60;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AD66;
    10
}
/// $05AD60: swap d2
pub fn i_05ad60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD62;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD62: move.w d2, d6
pub fn i_05ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD64;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD64: swap d2
pub fn i_05ad64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD66;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD66: andi.b #$f0, d4
pub fn i_05ad66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6A;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AD6A: or.b d4, d6
pub fn i_05ad6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6C;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AD6C: move.b d6, d4
pub fn i_05ad6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6E;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD6E: swap d4
pub fn i_05ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD70;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD70: move.b d6, d4
pub fn i_05ad70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD72;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD72: swap d4
pub fn i_05ad72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD74;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD74: move.w d7, d6
pub fn i_05ad74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD76;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD76: mulu.w d5, d6
pub fn i_05ad76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD78;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD78: addi.w #$ff, d6
pub fn i_05ad78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD7C;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD7C: lsr.w #$8, d6
pub fn i_05ad7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD7E;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AD7E: swap d6
pub fn i_05ad7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD80;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD80: move.w d5, d6
pub fn i_05ad80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD82;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD82: add.w d0, d1
pub fn i_05ad82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD84;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD84: swap d1
pub fn i_05ad84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD86;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD86: swap d0
pub fn i_05ad86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD88;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD88: add.w d0, d1
pub fn i_05ad88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD8A;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD8A: swap d0
pub fn i_05ad8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD8C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD8C: andi.w #$1ff, d1
pub fn i_05ad8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD90;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD90: lsl.l #$7, d1
pub fn i_05ad90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD92;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AD92: or.l d6, d1
pub fn i_05ad92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD94;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD94: clr.w d5
pub fn i_05ad94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD96;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AD96: swap d5
pub fn i_05ad96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD98;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD98: subq.b #$1, d5
pub fn i_05ad98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9A;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AD9A: add.w d6, d6
pub fn i_05ad9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9C;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD9C: movea.l d4, a4
pub fn i_05ad9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9E;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05AD9E: move.b $616e(a5), d3
pub fn i_05ad9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADA2;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05ADA2: sub.b d3, $616d(a5)
pub fn i_05ada2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADA6;
    cpu.ir = 0x972D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05ADA6: bcc.w $5adb2
pub fn i_05ada6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADB2; return 10; }
    8
}
/// $05ADAA: swap d4
pub fn i_05adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAC;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ADAC: tst.w d4
pub fn i_05adac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAE;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05ADAE: bmi.w $5adde
pub fn i_05adae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5ADDE; return 10; }
    8
}
/// $05ADB2: move.w d4, d3
pub fn i_05adb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB4;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ADB4: andi.w #$f00, d3
pub fn i_05adb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB8;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ADB8: addi.w #$100, d3
pub fn i_05adb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADBC;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ADBC: lsr.w #$1, d3
pub fn i_05adbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADBE;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05ADBE: cmpi.w #$f800, d1
pub fn i_05adbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADC2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05ADC2: bcc.w $5adce
pub fn i_05adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADCE; return 10; }
    8
}
/// $05ADC6: cmpi.w #$a000, d1
pub fn i_05adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADCA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05ADCA: bcc.w $5addc
pub fn i_05adca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADDC; return 10; }
    8
}
/// $05ADCE: move.l a0, (a3)+
pub fn i_05adce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD0;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ADD0: move.w d2, (a3)+
pub fn i_05add0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD2;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ADD2: move.w d4, (a3)+
pub fn i_05add2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD4;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ADD4: move.l d1, (a3)+
pub fn i_05add4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD6;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ADD6: cmpa.l a2, a3
pub fn i_05add6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ADD8: bcc.w $5adec
pub fn i_05add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADEC; return 10; }
    8
}
/// $05ADDC: add.w d3, d1
pub fn i_05addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADDE;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ADDE: move.l a4, d4
pub fn i_05adde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE0;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ADE0: adda.w d6, a0
pub fn i_05ade0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE2;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ADE2: dbra d5, $5ad9e
pub fn i_05ade2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE6;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ADE6; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ADE6; return 14; }
    cpu.pc = 0x5AD9E;
    10
}
/// $05ADE6: subq.b #$1, (a7)
pub fn i_05ade6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE8;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ADE8: bne.w $5ad18
pub fn i_05ade8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AD18; return 10; }
    8
}
/// $05ADEC: addq.w #$2, a7
pub fn i_05adec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADEE;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ADEE: lea.l $4258(a5), a2
pub fn i_05adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF2;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05ADF2: cmpa.l a2, a3
pub fn i_05adf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF4;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ADF4: beq.w $5adfe
pub fn i_05adf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ADFE; return 10; }
    8
}
/// $05ADF8: ori.b #$80, -$6(a3)
pub fn i_05adf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADFE;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ADFE: rts
pub fn i_05adfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AE00: cmpa.l a2, a3
pub fn i_05ae00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE02;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AE02: bcc.w $5af2e
pub fn i_05ae02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF2E; return 10; }
    8
}
/// $05AE06: move.b (a0)+, -(a7)
pub fn i_05ae06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE08;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AE08: move.b (a0)+, d2
pub fn i_05ae08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE0A;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AE0A: eor.b d5, d2
pub fn i_05ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE0C;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0591FE, i_0591fe),
    (0x059202, i_059202),
    (0x059206, i_059206),
    (0x059208, i_059208),
    (0x05920E, i_05920e),
    (0x059212, i_059212),
    (0x059216, i_059216),
    (0x05921A, i_05921a),
    (0x05921C, i_05921c),
    (0x059222, i_059222),
    (0x059226, i_059226),
    (0x059228, i_059228),
    (0x05922E, i_05922e),
    (0x059234, i_059234),
    (0x059238, i_059238),
    (0x05923A, i_05923a),
    (0x05923E, i_05923e),
    (0x059244, i_059244),
    (0x059248, i_059248),
    (0x05924C, i_05924c),
    (0x05924E, i_05924e),
    (0x059254, i_059254),
    (0x059258, i_059258),
    (0x05925A, i_05925a),
    (0x059260, i_059260),
    (0x059266, i_059266),
    (0x05926A, i_05926a),
    (0x059270, i_059270),
    (0x059274, i_059274),
    (0x05927A, i_05927a),
    (0x059280, i_059280),
    (0x059286, i_059286),
    (0x05928A, i_05928a),
    (0x05928C, i_05928c),
    (0x059292, i_059292),
    (0x059296, i_059296),
    (0x05929A, i_05929a),
    (0x05929E, i_05929e),
    (0x0592A0, i_0592a0),
    (0x0592A6, i_0592a6),
    (0x0592AC, i_0592ac),
    (0x0592B0, i_0592b0),
    (0x0592B2, i_0592b2),
    (0x0592B8, i_0592b8),
    (0x0592BE, i_0592be),
    (0x0592C2, i_0592c2),
    (0x0592C6, i_0592c6),
    (0x0592C8, i_0592c8),
    (0x0592CC, i_0592cc),
    (0x0592CE, i_0592ce),
    (0x0592D2, i_0592d2),
    (0x0592D6, i_0592d6),
    (0x0592DA, i_0592da),
    (0x0592DE, i_0592de),
    (0x0592E2, i_0592e2),
    (0x0592E4, i_0592e4),
    (0x0592EA, i_0592ea),
    (0x0592F0, i_0592f0),
    (0x0592F4, i_0592f4),
    (0x0592F6, i_0592f6),
    (0x0592FA, i_0592fa),
    (0x0592FE, i_0592fe),
    (0x059302, i_059302),
    (0x059304, i_059304),
    (0x059308, i_059308),
    (0x05930E, i_05930e),
    (0x059312, i_059312),
    (0x059316, i_059316),
    (0x059318, i_059318),
    (0x05931C, i_05931c),
    (0x059320, i_059320),
    (0x059324, i_059324),
    (0x059326, i_059326),
    (0x05932C, i_05932c),
    (0x059330, i_059330),
    (0x059332, i_059332),
    (0x059336, i_059336),
    (0x05933A, i_05933a),
    (0x05933E, i_05933e),
    (0x059342, i_059342),
    (0x059346, i_059346),
    (0x059348, i_059348),
    (0x05934C, i_05934c),
    (0x0593FA, i_0593fa),
    (0x059400, i_059400),
    (0x059406, i_059406),
    (0x05940A, i_05940a),
    (0x05940E, i_05940e),
    (0x059416, i_059416),
    (0x05941C, i_05941c),
    (0x059422, i_059422),
    (0x059426, i_059426),
    (0x059428, i_059428),
    (0x05942E, i_05942e),
    (0x059432, i_059432),
    (0x059436, i_059436),
    (0x059438, i_059438),
    (0x05943A, i_05943a),
    (0x059440, i_059440),
    (0x059446, i_059446),
    (0x05944A, i_05944a),
    (0x05944E, i_05944e),
    (0x059456, i_059456),
    (0x05945C, i_05945c),
    (0x059462, i_059462),
    (0x059466, i_059466),
    (0x059468, i_059468),
    (0x05946E, i_05946e),
    (0x059472, i_059472),
    (0x059476, i_059476),
    (0x059478, i_059478),
    (0x05947A, i_05947a),
    (0x059480, i_059480),
    (0x059486, i_059486),
    (0x05948A, i_05948a),
    (0x05948E, i_05948e),
    (0x059496, i_059496),
    (0x05949C, i_05949c),
    (0x0594A2, i_0594a2),
    (0x0594A6, i_0594a6),
    (0x0594A8, i_0594a8),
    (0x0594AE, i_0594ae),
    (0x0594B2, i_0594b2),
    (0x0594B6, i_0594b6),
    (0x0594B8, i_0594b8),
    (0x0594BA, i_0594ba),
    (0x0594C0, i_0594c0),
    (0x0594C6, i_0594c6),
    (0x0594CC, i_0594cc),
    (0x0594D0, i_0594d0),
    (0x0594D2, i_0594d2),
    (0x0594D6, i_0594d6),
    (0x0594D8, i_0594d8),
    (0x0594DC, i_0594dc),
    (0x0594E0, i_0594e0),
    (0x0594E2, i_0594e2),
    (0x0594E8, i_0594e8),
    (0x0594EC, i_0594ec),
    (0x0594F2, i_0594f2),
    (0x0594F6, i_0594f6),
    (0x0594F8, i_0594f8),
    (0x0594FC, i_0594fc),
    (0x059500, i_059500),
    (0x059502, i_059502),
    (0x059508, i_059508),
    (0x05950C, i_05950c),
    (0x059512, i_059512),
    (0x059516, i_059516),
    (0x059518, i_059518),
    (0x05951C, i_05951c),
    (0x059520, i_059520),
    (0x059522, i_059522),
    (0x059528, i_059528),
    (0x05952C, i_05952c),
    (0x059532, i_059532),
    (0x059536, i_059536),
    (0x05953A, i_05953a),
    (0x05953C, i_05953c),
    (0x059540, i_059540),
    (0x059544, i_059544),
    (0x059546, i_059546),
    (0x05954C, i_05954c),
    (0x059550, i_059550),
    (0x059556, i_059556),
    (0x05955A, i_05955a),
    (0x05955C, i_05955c),
    (0x059560, i_059560),
    (0x059564, i_059564),
    (0x059566, i_059566),
    (0x05956C, i_05956c),
    (0x059570, i_059570),
    (0x059576, i_059576),
    (0x05957A, i_05957a),
    (0x05957C, i_05957c),
    (0x059580, i_059580),
    (0x059584, i_059584),
    (0x059586, i_059586),
    (0x05958C, i_05958c),
    (0x059590, i_059590),
    (0x059596, i_059596),
    (0x05959C, i_05959c),
    (0x0595A0, i_0595a0),
    (0x0595A6, i_0595a6),
    (0x0595AC, i_0595ac),
    (0x0595AE, i_0595ae),
    (0x0595B0, i_0595b0),
    (0x0595B4, i_0595b4),
    (0x0595B6, i_0595b6),
    (0x0595B8, i_0595b8),
    (0x0595BA, i_0595ba),
    (0x0595BE, i_0595be),
    (0x0595C2, i_0595c2),
    (0x0595C4, i_0595c4),
    (0x0595C8, i_0595c8),
    (0x0595CC, i_0595cc),
    (0x0595D0, i_0595d0),
    (0x0595D8, i_0595d8),
    (0x0595DA, i_0595da),
    (0x0595DE, i_0595de),
    (0x0595E6, i_0595e6),
    (0x0595EA, i_0595ea),
    (0x0595EE, i_0595ee),
    (0x0595F6, i_0595f6),
    (0x0595F8, i_0595f8),
    (0x0595FC, i_0595fc),
    (0x059604, i_059604),
    (0x059608, i_059608),
    (0x05960C, i_05960c),
    (0x059610, i_059610),
    (0x059612, i_059612),
    (0x059618, i_059618),
    (0x05961C, i_05961c),
    (0x05961E, i_05961e),
    (0x059620, i_059620),
    (0x059622, i_059622),
    (0x059624, i_059624),
    (0x059628, i_059628),
    (0x05962A, i_05962a),
    (0x05962C, i_05962c),
    (0x05962E, i_05962e),
    (0x059630, i_059630),
    (0x059632, i_059632),
    (0x059634, i_059634),
    (0x059638, i_059638),
    (0x05963A, i_05963a),
    (0x05963C, i_05963c),
    (0x05963E, i_05963e),
    (0x059640, i_059640),
    (0x059644, i_059644),
    (0x059646, i_059646),
    (0x05964A, i_05964a),
    (0x05964C, i_05964c),
    (0x05964E, i_05964e),
    (0x059652, i_059652),
    (0x059656, i_059656),
    (0x05965A, i_05965a),
    (0x05965E, i_05965e),
    (0x059664, i_059664),
    (0x059666, i_059666),
    (0x05966A, i_05966a),
    (0x05966E, i_05966e),
    (0x059676, i_059676),
    (0x05967A, i_05967a),
    (0x059680, i_059680),
    (0x059686, i_059686),
    (0x05968A, i_05968a),
    (0x059692, i_059692),
    (0x05969A, i_05969a),
    (0x0596A2, i_0596a2),
    (0x0596A6, i_0596a6),
    (0x0596AE, i_0596ae),
    (0x0596B2, i_0596b2),
    (0x0596B8, i_0596b8),
    (0x0596BE, i_0596be),
    (0x0596C2, i_0596c2),
    (0x0596CA, i_0596ca),
    (0x0596D2, i_0596d2),
    (0x0596DA, i_0596da),
    (0x0596DE, i_0596de),
    (0x0596E2, i_0596e2),
    (0x0596E6, i_0596e6),
    (0x0596EA, i_0596ea),
    (0x0596EE, i_0596ee),
    (0x0596F2, i_0596f2),
    (0x0596F8, i_0596f8),
    (0x0596FC, i_0596fc),
    (0x059700, i_059700),
    (0x059706, i_059706),
    (0x05970C, i_05970c),
    (0x059710, i_059710),
    (0x059712, i_059712),
    (0x059716, i_059716),
    (0x05971A, i_05971a),
    (0x05971E, i_05971e),
    (0x059720, i_059720),
    (0x059722, i_059722),
    (0x059728, i_059728),
    (0x05972C, i_05972c),
    (0x059730, i_059730),
    (0x059734, i_059734),
    (0x05973A, i_05973a),
    (0x059740, i_059740),
    (0x059744, i_059744),
    (0x059746, i_059746),
    (0x05974A, i_05974a),
    (0x05974E, i_05974e),
    (0x059752, i_059752),
    (0x059754, i_059754),
    (0x059756, i_059756),
    (0x05975C, i_05975c),
    (0x059760, i_059760),
    (0x059762, i_059762),
    (0x059766, i_059766),
    (0x05976A, i_05976a),
    (0x05976E, i_05976e),
    (0x059774, i_059774),
    (0x059776, i_059776),
    (0x05977A, i_05977a),
    (0x05977E, i_05977e),
    (0x059782, i_059782),
    (0x059784, i_059784),
    (0x05978A, i_05978a),
    (0x05978C, i_05978c),
    (0x059790, i_059790),
    (0x059794, i_059794),
    (0x05979A, i_05979a),
    (0x05979E, i_05979e),
    (0x0597A0, i_0597a0),
    (0x0597A4, i_0597a4),
    (0x0597A8, i_0597a8),
    (0x0597AC, i_0597ac),
    (0x0597AE, i_0597ae),
    (0x0597B0, i_0597b0),
    (0x0597B6, i_0597b6),
    (0x0597BA, i_0597ba),
    (0x0597C0, i_0597c0),
    (0x0597C4, i_0597c4),
    (0x0597C8, i_0597c8),
    (0x0597CC, i_0597cc),
    (0x0597D2, i_0597d2),
    (0x0597D6, i_0597d6),
    (0x0597DC, i_0597dc),
    (0x0597E0, i_0597e0),
    (0x0597E4, i_0597e4),
    (0x0597E8, i_0597e8),
    (0x0597EE, i_0597ee),
    (0x0597F4, i_0597f4),
    (0x0597F8, i_0597f8),
    (0x0597FA, i_0597fa),
    (0x0597FE, i_0597fe),
    (0x059802, i_059802),
    (0x059806, i_059806),
    (0x059808, i_059808),
    (0x05980A, i_05980a),
    (0x059810, i_059810),
    (0x059814, i_059814),
    (0x059816, i_059816),
    (0x05981A, i_05981a),
    (0x05981E, i_05981e),
    (0x059822, i_059822),
    (0x059828, i_059828),
    (0x05982A, i_05982a),
    (0x05982E, i_05982e),
    (0x059832, i_059832),
    (0x059834, i_059834),
    (0x059838, i_059838),
    (0x05983E, i_05983e),
    (0x059840, i_059840),
    (0x059844, i_059844),
    (0x059848, i_059848),
    (0x05984E, i_05984e),
    (0x059852, i_059852),
    (0x059854, i_059854),
    (0x059858, i_059858),
    (0x05985C, i_05985c),
    (0x059860, i_059860),
    (0x059862, i_059862),
    (0x059864, i_059864),
    (0x05986A, i_05986a),
    (0x05986E, i_05986e),
    (0x059872, i_059872),
    (0x059876, i_059876),
    (0x05987C, i_05987c),
    (0x059882, i_059882),
    (0x059884, i_059884),
    (0x059888, i_059888),
    (0x05988C, i_05988c),
    (0x05988E, i_05988e),
    (0x059892, i_059892),
    (0x059898, i_059898),
    (0x05989C, i_05989c),
    (0x05989E, i_05989e),
    (0x0598A2, i_0598a2),
    (0x0598A6, i_0598a6),
    (0x0598AA, i_0598aa),
    (0x0598AC, i_0598ac),
    (0x0598AE, i_0598ae),
    (0x0598B4, i_0598b4),
    (0x0598B8, i_0598b8),
    (0x0598BE, i_0598be),
    (0x0598C4, i_0598c4),
    (0x0598C8, i_0598c8),
    (0x0598CC, i_0598cc),
    (0x0598D0, i_0598d0),
    (0x0598D6, i_0598d6),
    (0x0598DA, i_0598da),
    (0x0598E0, i_0598e0),
    (0x0598E4, i_0598e4),
    (0x0598E6, i_0598e6),
    (0x0598EA, i_0598ea),
    (0x0598EE, i_0598ee),
    (0x0598F4, i_0598f4),
    (0x0598FA, i_0598fa),
    (0x0598FC, i_0598fc),
    (0x0598FE, i_0598fe),
    (0x059904, i_059904),
    (0x059908, i_059908),
    (0x05990E, i_05990e),
    (0x059910, i_059910),
    (0x059914, i_059914),
    (0x059918, i_059918),
    (0x05991E, i_05991e),
    (0x059924, i_059924),
    (0x059928, i_059928),
    (0x05992E, i_05992e),
    (0x059934, i_059934),
    (0x059938, i_059938),
    (0x05993A, i_05993a),
    (0x05993E, i_05993e),
    (0x059942, i_059942),
    (0x059946, i_059946),
    (0x059948, i_059948),
    (0x05994A, i_05994a),
    (0x05994E, i_05994e),
    (0x059954, i_059954),
    (0x059958, i_059958),
    (0x05995A, i_05995a),
    (0x059960, i_059960),
    (0x059964, i_059964),
    (0x059968, i_059968),
    (0x05996A, i_05996a),
    (0x05996C, i_05996c),
    (0x059972, i_059972),
    (0x059976, i_059976),
    (0x059978, i_059978),
    (0x05997C, i_05997c),
    (0x059980, i_059980),
    (0x059984, i_059984),
    (0x059986, i_059986),
    (0x059988, i_059988),
    (0x05998C, i_05998c),
    (0x059992, i_059992),
    (0x059996, i_059996),
    (0x059998, i_059998),
    (0x05999E, i_05999e),
    (0x0599A2, i_0599a2),
    (0x0599A6, i_0599a6),
    (0x0599A8, i_0599a8),
    (0x0599AA, i_0599aa),
    (0x0599B0, i_0599b0),
    (0x0599B4, i_0599b4),
    (0x0599B6, i_0599b6),
    (0x0599BA, i_0599ba),
    (0x0599BE, i_0599be),
    (0x0599C2, i_0599c2),
    (0x0599C4, i_0599c4),
    (0x0599C6, i_0599c6),
    (0x0599CA, i_0599ca),
    (0x0599D0, i_0599d0),
    (0x0599D6, i_0599d6),
    (0x0599DC, i_0599dc),
    (0x0599E0, i_0599e0),
    (0x0599E2, i_0599e2),
    (0x0599E6, i_0599e6),
    (0x0599EA, i_0599ea),
    (0x0599EE, i_0599ee),
    (0x0599F0, i_0599f0),
    (0x0599F2, i_0599f2),
    (0x0599F6, i_0599f6),
    (0x0599FC, i_0599fc),
    (0x059A02, i_059a02),
    (0x059A06, i_059a06),
    (0x059A08, i_059a08),
    (0x059A0E, i_059a0e),
    (0x059A12, i_059a12),
    (0x059A16, i_059a16),
    (0x059A18, i_059a18),
    (0x059A1A, i_059a1a),
    (0x059A1E, i_059a1e),
    (0x059A24, i_059a24),
    (0x059A2A, i_059a2a),
    (0x059A2E, i_059a2e),
    (0x059A30, i_059a30),
    (0x059A34, i_059a34),
    (0x059A38, i_059a38),
    (0x059A3C, i_059a3c),
    (0x059A3E, i_059a3e),
    (0x059A40, i_059a40),
    (0x059A44, i_059a44),
    (0x059A4A, i_059a4a),
    (0x059A4E, i_059a4e),
    (0x059A54, i_059a54),
    (0x059A5A, i_059a5a),
    (0x059A5E, i_059a5e),
    (0x059A60, i_059a60),
    (0x059A64, i_059a64),
    (0x059A68, i_059a68),
    (0x059A6C, i_059a6c),
    (0x059A6E, i_059a6e),
    (0x059A70, i_059a70),
    (0x059A74, i_059a74),
    (0x059A7A, i_059a7a),
    (0x059A80, i_059a80),
    (0x059A86, i_059a86),
    (0x059A8A, i_059a8a),
    (0x059A90, i_059a90),
    (0x059A98, i_059a98),
    (0x059A9E, i_059a9e),
    (0x059AA2, i_059aa2),
    (0x059AA8, i_059aa8),
    (0x059AAE, i_059aae),
    (0x059AB6, i_059ab6),
    (0x059ABC, i_059abc),
    (0x059AC0, i_059ac0),
    (0x059AC6, i_059ac6),
    (0x059ACC, i_059acc),
    (0x059AD4, i_059ad4),
    (0x059ADA, i_059ada),
    (0x059ADE, i_059ade),
    (0x059AE4, i_059ae4),
    (0x059AEC, i_059aec),
    (0x059AF2, i_059af2),
    (0x059AF8, i_059af8),
    (0x059AFC, i_059afc),
    (0x059AFE, i_059afe),
    (0x059B04, i_059b04),
    (0x059B06, i_059b06),
    (0x059B0A, i_059b0a),
    (0x059B10, i_059b10),
    (0x059B16, i_059b16),
    (0x059B1A, i_059b1a),
    (0x059B1C, i_059b1c),
    (0x059B20, i_059b20),
    (0x059B24, i_059b24),
    (0x059B28, i_059b28),
    (0x059B2C, i_059b2c),
    (0x059B2E, i_059b2e),
    (0x059B30, i_059b30),
    (0x059B34, i_059b34),
    (0x059B3A, i_059b3a),
    (0x059B3E, i_059b3e),
    (0x059B40, i_059b40),
    (0x059B44, i_059b44),
    (0x059B4A, i_059b4a),
    (0x059B4E, i_059b4e),
    (0x059B52, i_059b52),
    (0x059B58, i_059b58),
    (0x059B6A, i_059b6a),
    (0x059B70, i_059b70),
    (0x059B74, i_059b74),
    (0x059B76, i_059b76),
    (0x059B7A, i_059b7a),
    (0x059B7E, i_059b7e),
    (0x059B82, i_059b82),
    (0x059B84, i_059b84),
    (0x059B86, i_059b86),
    (0x059B8C, i_059b8c),
    (0x059B92, i_059b92),
    (0x059B96, i_059b96),
    (0x059B9A, i_059b9a),
    (0x059BA2, i_059ba2),
    (0x059BA8, i_059ba8),
    (0x059BAE, i_059bae),
    (0x059BB2, i_059bb2),
    (0x059BB4, i_059bb4),
    (0x059BBA, i_059bba),
    (0x059BBE, i_059bbe),
    (0x059BC2, i_059bc2),
    (0x059BC4, i_059bc4),
    (0x059BC6, i_059bc6),
    (0x059BCC, i_059bcc),
    (0x059BD2, i_059bd2),
    (0x059BD6, i_059bd6),
    (0x059BDA, i_059bda),
    (0x059BDE, i_059bde),
    (0x059BE2, i_059be2),
    (0x059BE6, i_059be6),
    (0x059BE8, i_059be8),
    (0x059BEC, i_059bec),
    (0x059BEE, i_059bee),
    (0x059BF2, i_059bf2),
    (0x059BF8, i_059bf8),
    (0x059BFE, i_059bfe),
    (0x059C04, i_059c04),
    (0x059C0A, i_059c0a),
    (0x059C0E, i_059c0e),
    (0x059C10, i_059c10),
    (0x059C16, i_059c16),
    (0x059C1C, i_059c1c),
    (0x059C20, i_059c20),
    (0x059C26, i_059c26),
    (0x059C2C, i_059c2c),
    (0x059C2E, i_059c2e),
    (0x059C30, i_059c30),
    (0x059C34, i_059c34),
    (0x059C3A, i_059c3a),
    (0x059C3E, i_059c3e),
    (0x059C40, i_059c40),
    (0x059C42, i_059c42),
    (0x059C48, i_059c48),
    (0x059C4E, i_059c4e),
    (0x059C52, i_059c52),
    (0x059C54, i_059c54),
    (0x059C5A, i_059c5a),
    (0x059C60, i_059c60),
    (0x059C64, i_059c64),
    (0x059C68, i_059c68),
    (0x059C6C, i_059c6c),
    (0x059C70, i_059c70),
    (0x059C74, i_059c74),
    (0x059C7A, i_059c7a),
    (0x059C7C, i_059c7c),
    (0x059C80, i_059c80),
    (0x059C82, i_059c82),
    (0x059C88, i_059c88),
    (0x059C8A, i_059c8a),
    (0x059C90, i_059c90),
    (0x059C96, i_059c96),
    (0x059C9A, i_059c9a),
    (0x059C9C, i_059c9c),
    (0x059CA0, i_059ca0),
    (0x059CA6, i_059ca6),
    (0x059CAC, i_059cac),
    (0x059CB2, i_059cb2),
    (0x059CB6, i_059cb6),
    (0x059CB8, i_059cb8),
    (0x059CBE, i_059cbe),
    (0x059CC4, i_059cc4),
    (0x059CC8, i_059cc8),
    (0x059CCC, i_059ccc),
    (0x059CD0, i_059cd0),
    (0x059CD6, i_059cd6),
    (0x059CDC, i_059cdc),
    (0x059CE0, i_059ce0),
    (0x059CE4, i_059ce4),
    (0x059CE6, i_059ce6),
    (0x059CEA, i_059cea),
    (0x059CF0, i_059cf0),
    (0x059CF6, i_059cf6),
    (0x059CFC, i_059cfc),
    (0x059D00, i_059d00),
    (0x059D02, i_059d02),
    (0x059D08, i_059d08),
    (0x059D0E, i_059d0e),
    (0x059D12, i_059d12),
    (0x059D16, i_059d16),
    (0x059D1A, i_059d1a),
    (0x059D1C, i_059d1c),
    (0x059D20, i_059d20),
    (0x059D24, i_059d24),
    (0x059D28, i_059d28),
    (0x059D2C, i_059d2c),
    (0x059D30, i_059d30),
    (0x059D36, i_059d36),
    (0x059D38, i_059d38),
    (0x059D3C, i_059d3c),
    (0x059D40, i_059d40),
    (0x059D42, i_059d42),
    (0x059D46, i_059d46),
    (0x059D4A, i_059d4a),
    (0x059D4C, i_059d4c),
    (0x059D50, i_059d50),
    (0x059D54, i_059d54),
    (0x059D5A, i_059d5a),
    (0x059D5E, i_059d5e),
    (0x059D60, i_059d60),
    (0x059D62, i_059d62),
    (0x059D66, i_059d66),
    (0x059D6C, i_059d6c),
    (0x059D72, i_059d72),
    (0x059D78, i_059d78),
    (0x059D7E, i_059d7e),
    (0x059D84, i_059d84),
    (0x059D86, i_059d86),
    (0x059D8C, i_059d8c),
    (0x059D90, i_059d90),
    (0x059D96, i_059d96),
    (0x059D98, i_059d98),
    (0x059D9E, i_059d9e),
    (0x059DA4, i_059da4),
    (0x059DA6, i_059da6),
    (0x059DAA, i_059daa),
    (0x059DB0, i_059db0),
    (0x059DB4, i_059db4),
    (0x059DBA, i_059dba),
    (0x059DC0, i_059dc0),
    (0x059DC6, i_059dc6),
    (0x059DCC, i_059dcc),
    (0x059DD2, i_059dd2),
    (0x059DD8, i_059dd8),
    (0x059DDE, i_059dde),
    (0x059DE4, i_059de4),
    (0x059DE8, i_059de8),
    (0x059DEE, i_059dee),
    (0x059DF4, i_059df4),
    (0x059DF8, i_059df8),
    (0x059DFA, i_059dfa),
    (0x059E00, i_059e00),
    (0x059E04, i_059e04),
    (0x059E08, i_059e08),
    (0x059E0C, i_059e0c),
    (0x059E10, i_059e10),
    (0x059E16, i_059e16),
    (0x059E1C, i_059e1c),
    (0x059E22, i_059e22),
    (0x059E28, i_059e28),
    (0x059E2C, i_059e2c),
    (0x059E32, i_059e32),
    (0x059E34, i_059e34),
    (0x059E38, i_059e38),
    (0x059E3C, i_059e3c),
    (0x059E40, i_059e40),
    (0x059E44, i_059e44),
    (0x059E48, i_059e48),
    (0x059E4A, i_059e4a),
    (0x059E4E, i_059e4e),
    (0x05A1B6, i_05a1b6),
    (0x05A1B8, i_05a1b8),
    (0x05A1BC, i_05a1bc),
    (0x05A1BE, i_05a1be),
    (0x05A1C0, i_05a1c0),
    (0x05A1C4, i_05a1c4),
    (0x05A1C8, i_05a1c8),
    (0x05A1CC, i_05a1cc),
    (0x05A1CE, i_05a1ce),
    (0x05A1D0, i_05a1d0),
    (0x05A1D2, i_05a1d2),
    (0x05A1D6, i_05a1d6),
    (0x05A1DA, i_05a1da),
    (0x05A1DE, i_05a1de),
    (0x05A1E4, i_05a1e4),
    (0x05A1E8, i_05a1e8),
    (0x05A1EE, i_05a1ee),
    (0x05A1F0, i_05a1f0),
    (0x05A1F2, i_05a1f2),
    (0x05A1F6, i_05a1f6),
    (0x05A1FA, i_05a1fa),
    (0x05A200, i_05a200),
    (0x05A204, i_05a204),
    (0x05A208, i_05a208),
    (0x05A20C, i_05a20c),
    (0x05A210, i_05a210),
    (0x05A216, i_05a216),
    (0x05A21A, i_05a21a),
    (0x05A21C, i_05a21c),
    (0x05A21E, i_05a21e),
    (0x05A222, i_05a222),
    (0x05A226, i_05a226),
    (0x05A22A, i_05a22a),
    (0x05A22C, i_05a22c),
    (0x05A230, i_05a230),
    (0x05A234, i_05a234),
    (0x05A236, i_05a236),
    (0x05A23E, i_05a23e),
    (0x05A242, i_05a242),
    (0x05A248, i_05a248),
    (0x05A24C, i_05a24c),
    (0x05A252, i_05a252),
    (0x05A258, i_05a258),
    (0x05A25A, i_05a25a),
    (0x05A25E, i_05a25e),
    (0x05A262, i_05a262),
    (0x05A266, i_05a266),
    (0x05A26C, i_05a26c),
    (0x05A26E, i_05a26e),
    (0x05A272, i_05a272),
    (0x05A276, i_05a276),
    (0x05A27C, i_05a27c),
    (0x05A282, i_05a282),
    (0x05A286, i_05a286),
    (0x05A288, i_05a288),
    (0x05A28A, i_05a28a),
    (0x05A28E, i_05a28e),
    (0x05A292, i_05a292),
    (0x05A298, i_05a298),
    (0x05A29E, i_05a29e),
    (0x05A2A2, i_05a2a2),
    (0x05A2A6, i_05a2a6),
    (0x05A2AA, i_05a2aa),
    (0x05A2B0, i_05a2b0),
    (0x05A2B2, i_05a2b2),
    (0x05A2B6, i_05a2b6),
    (0x05A2BC, i_05a2bc),
    (0x05A2C0, i_05a2c0),
    (0x05A2C2, i_05a2c2),
    (0x05A2C6, i_05a2c6),
    (0x05A2CC, i_05a2cc),
    (0x05A2D0, i_05a2d0),
    (0x05A2D6, i_05a2d6),
    (0x05A2DC, i_05a2dc),
    (0x05A2E2, i_05a2e2),
    (0x05A2E6, i_05a2e6),
    (0x05A2E8, i_05a2e8),
    (0x05A2EE, i_05a2ee),
    (0x05A2F2, i_05a2f2),
    (0x05A2F8, i_05a2f8),
    (0x05A2FA, i_05a2fa),
    (0x05A2FE, i_05a2fe),
    (0x05A300, i_05a300),
    (0x05A304, i_05a304),
    (0x05A30A, i_05a30a),
    (0x05A30E, i_05a30e),
    (0x05A312, i_05a312),
    (0x05A316, i_05a316),
    (0x05A31A, i_05a31a),
    (0x05A31C, i_05a31c),
    (0x05A320, i_05a320),
    (0x05A326, i_05a326),
    (0x05A32A, i_05a32a),
    (0x05A330, i_05a330),
    (0x05A334, i_05a334),
    (0x05A33A, i_05a33a),
    (0x05A33E, i_05a33e),
    (0x05A342, i_05a342),
    (0x05A344, i_05a344),
    (0x05A34A, i_05a34a),
    (0x05A34E, i_05a34e),
    (0x05A352, i_05a352),
    (0x05A356, i_05a356),
    (0x05A35A, i_05a35a),
    (0x05A35C, i_05a35c),
    (0x05A360, i_05a360),
    (0x05A366, i_05a366),
    (0x05A36A, i_05a36a),
    (0x05A36E, i_05a36e),
    (0x05A370, i_05a370),
    (0x05A376, i_05a376),
    (0x05A37A, i_05a37a),
    (0x05A37E, i_05a37e),
    (0x05A384, i_05a384),
    (0x05A388, i_05a388),
    (0x05A38C, i_05a38c),
    (0x05A390, i_05a390),
    (0x05A394, i_05a394),
    (0x05A396, i_05a396),
    (0x05A39C, i_05a39c),
    (0x05A3A2, i_05a3a2),
    (0x05A3A6, i_05a3a6),
    (0x05A3A8, i_05a3a8),
    (0x05A3AE, i_05a3ae),
    (0x05A3B2, i_05a3b2),
    (0x05A3B8, i_05a3b8),
    (0x05A3BA, i_05a3ba),
    (0x05A3C0, i_05a3c0),
    (0x05A3C6, i_05a3c6),
    (0x05A3CC, i_05a3cc),
    (0x05A3D2, i_05a3d2),
    (0x05A3D8, i_05a3d8),
    (0x05A3DC, i_05a3dc),
    (0x05A3E0, i_05a3e0),
    (0x05A3E2, i_05a3e2),
    (0x05A3E6, i_05a3e6),
    (0x05A3EA, i_05a3ea),
    (0x05A3EC, i_05a3ec),
    (0x05A3F0, i_05a3f0),
    (0x05A3F4, i_05a3f4),
    (0x05A3F8, i_05a3f8),
    (0x05A3FE, i_05a3fe),
    (0x05A400, i_05a400),
    (0x05A404, i_05a404),
    (0x05A408, i_05a408),
    (0x05A40A, i_05a40a),
    (0x05A40E, i_05a40e),
    (0x05A412, i_05a412),
    (0x05A416, i_05a416),
    (0x05A41A, i_05a41a),
    (0x05A41E, i_05a41e),
    (0x05A422, i_05a422),
    (0x05A426, i_05a426),
    (0x05A462, i_05a462),
    (0x05A464, i_05a464),
    (0x05A46A, i_05a46a),
    (0x05A46E, i_05a46e),
    (0x05A472, i_05a472),
    (0x05A476, i_05a476),
    (0x05A47A, i_05a47a),
    (0x05A506, i_05a506),
    (0x05A50A, i_05a50a),
    (0x05A55A, i_05a55a),
    (0x05A560, i_05a560),
    (0x05A564, i_05a564),
    (0x05A568, i_05a568),
    (0x05A56C, i_05a56c),
    (0x05A570, i_05a570),
    (0x05A574, i_05a574),
    (0x05A57A, i_05a57a),
    (0x05A580, i_05a580),
    (0x05A584, i_05a584),
    (0x05A588, i_05a588),
    (0x05A58C, i_05a58c),
    (0x05A590, i_05a590),
    (0x05A594, i_05a594),
    (0x05A59A, i_05a59a),
    (0x05A5A0, i_05a5a0),
    (0x05A5A4, i_05a5a4),
    (0x05A5AA, i_05a5aa),
    (0x05A5AE, i_05a5ae),
    (0x05A5B4, i_05a5b4),
    (0x05A5BA, i_05a5ba),
    (0x05A5C0, i_05a5c0),
    (0x05A5C4, i_05a5c4),
    (0x05A5C6, i_05a5c6),
    (0x05A5CA, i_05a5ca),
    (0x05A5CE, i_05a5ce),
    (0x05A5D4, i_05a5d4),
    (0x05A5D8, i_05a5d8),
    (0x05A5DC, i_05a5dc),
    (0x05A5E0, i_05a5e0),
    (0x05A5E4, i_05a5e4),
    (0x05A5E8, i_05a5e8),
    (0x05A5EE, i_05a5ee),
    (0x05A5F2, i_05a5f2),
    (0x05A5F4, i_05a5f4),
    (0x05A5FA, i_05a5fa),
    (0x05A600, i_05a600),
    (0x05A606, i_05a606),
    (0x05A60C, i_05a60c),
    (0x05A610, i_05a610),
    (0x05A616, i_05a616),
    (0x05A61A, i_05a61a),
    (0x05A61E, i_05a61e),
    (0x05A622, i_05a622),
    (0x05A626, i_05a626),
    (0x05A62A, i_05a62a),
    (0x05A62E, i_05a62e),
    (0x05A632, i_05a632),
    (0x05A636, i_05a636),
    (0x05A63A, i_05a63a),
    (0x05A63C, i_05a63c),
    (0x05A642, i_05a642),
    (0x05A646, i_05a646),
    (0x05A64C, i_05a64c),
    (0x05A652, i_05a652),
    (0x05A658, i_05a658),
    (0x05A65A, i_05a65a),
    (0x05A65C, i_05a65c),
    (0x05A660, i_05a660),
    (0x05A666, i_05a666),
    (0x05A66A, i_05a66a),
    (0x05A66C, i_05a66c),
    (0x05A66E, i_05a66e),
    (0x05A672, i_05a672),
    (0x05A678, i_05a678),
    (0x05A67E, i_05a67e),
    (0x05A684, i_05a684),
    (0x05A68A, i_05a68a),
    (0x05A690, i_05a690),
    (0x05A72E, i_05a72e),
    (0x05A732, i_05a732),
    (0x05A738, i_05a738),
    (0x05A73C, i_05a73c),
    (0x05A742, i_05a742),
    (0x05A748, i_05a748),
    (0x05A74E, i_05a74e),
    (0x05A752, i_05a752),
    (0x05A758, i_05a758),
    (0x05A75C, i_05a75c),
    (0x05A760, i_05a760),
    (0x05A762, i_05a762),
    (0x05A764, i_05a764),
    (0x05A76A, i_05a76a),
    (0x05A770, i_05a770),
    (0x05A774, i_05a774),
    (0x05A77A, i_05a77a),
    (0x05A77E, i_05a77e),
    (0x05A784, i_05a784),
    (0x05A78A, i_05a78a),
    (0x05A78E, i_05a78e),
    (0x05A790, i_05a790),
    (0x05A794, i_05a794),
    (0x05A798, i_05a798),
    (0x05A79C, i_05a79c),
    (0x05A79E, i_05a79e),
    (0x05A7A4, i_05a7a4),
    (0x05A7AA, i_05a7aa),
    (0x05A7AE, i_05a7ae),
    (0x05A7B4, i_05a7b4),
    (0x05A7B8, i_05a7b8),
    (0x05A7BE, i_05a7be),
    (0x05A7C4, i_05a7c4),
    (0x05A7CA, i_05a7ca),
    (0x05A7D0, i_05a7d0),
    (0x05A7D4, i_05a7d4),
    (0x05A7D8, i_05a7d8),
    (0x05A7DA, i_05a7da),
    (0x05A7DE, i_05a7de),
    (0x05A7E4, i_05a7e4),
    (0x05A7E8, i_05a7e8),
    (0x05A7EC, i_05a7ec),
    (0x05A7EE, i_05a7ee),
    (0x05A7F0, i_05a7f0),
    (0x05A7F4, i_05a7f4),
    (0x05A7F8, i_05a7f8),
    (0x05A7FC, i_05a7fc),
    (0x05A802, i_05a802),
    (0x05A808, i_05a808),
    (0x05A80C, i_05a80c),
    (0x05A810, i_05a810),
    (0x05A814, i_05a814),
    (0x05A818, i_05a818),
    (0x05A81C, i_05a81c),
    (0x05A81E, i_05a81e),
    (0x05A822, i_05a822),
    (0x05A824, i_05a824),
    (0x05A826, i_05a826),
    (0x05A82E, i_05a82e),
    (0x05A836, i_05a836),
    (0x05A83C, i_05a83c),
    (0x05A840, i_05a840),
    (0x05A842, i_05a842),
    (0x05A844, i_05a844),
    (0x05A848, i_05a848),
    (0x05A84C, i_05a84c),
    (0x05A84E, i_05a84e),
    (0x05A856, i_05a856),
    (0x05A85E, i_05a85e),
    (0x05A864, i_05a864),
    (0x05A868, i_05a868),
    (0x05A86A, i_05a86a),
    (0x05A86C, i_05a86c),
    (0x05A86E, i_05a86e),
    (0x05A870, i_05a870),
    (0x05A872, i_05a872),
    (0x05A874, i_05a874),
    (0x05A878, i_05a878),
    (0x05A87E, i_05a87e),
    (0x05A884, i_05a884),
    (0x05A88A, i_05a88a),
    (0x05A890, i_05a890),
    (0x05A894, i_05a894),
    (0x05A898, i_05a898),
    (0x05A89C, i_05a89c),
    (0x05A8A0, i_05a8a0),
    (0x05A8A4, i_05a8a4),
    (0x05A8AA, i_05a8aa),
    (0x05A8AE, i_05a8ae),
    (0x05A8B2, i_05a8b2),
    (0x05A8B8, i_05a8b8),
    (0x05A8BA, i_05a8ba),
    (0x05A8BE, i_05a8be),
    (0x05A8C2, i_05a8c2),
    (0x05A8C6, i_05a8c6),
    (0x05A8CA, i_05a8ca),
    (0x05A8CE, i_05a8ce),
    (0x05A8D2, i_05a8d2),
    (0x05A8D6, i_05a8d6),
    (0x05A8DA, i_05a8da),
    (0x05A8DE, i_05a8de),
    (0x05A8E2, i_05a8e2),
    (0x05A8E6, i_05a8e6),
    (0x05A8EA, i_05a8ea),
    (0x05A8EE, i_05a8ee),
    (0x05A8F2, i_05a8f2),
    (0x05A8F6, i_05a8f6),
    (0x05A8FA, i_05a8fa),
    (0x05A8FE, i_05a8fe),
    (0x05A902, i_05a902),
    (0x05A906, i_05a906),
    (0x05A90A, i_05a90a),
    (0x05A90E, i_05a90e),
    (0x05A912, i_05a912),
    (0x05A916, i_05a916),
    (0x05A91A, i_05a91a),
    (0x05A91E, i_05a91e),
    (0x05A922, i_05a922),
    (0x05A926, i_05a926),
    (0x05A92A, i_05a92a),
    (0x05A92E, i_05a92e),
    (0x05A932, i_05a932),
    (0x05A936, i_05a936),
    (0x05A93A, i_05a93a),
    (0x05A93E, i_05a93e),
    (0x05A942, i_05a942),
    (0x05A946, i_05a946),
    (0x05A94A, i_05a94a),
    (0x05A94E, i_05a94e),
    (0x05A952, i_05a952),
    (0x05A956, i_05a956),
    (0x05A95A, i_05a95a),
    (0x05A95E, i_05a95e),
    (0x05A962, i_05a962),
    (0x05A966, i_05a966),
    (0x05A96A, i_05a96a),
    (0x05A96E, i_05a96e),
    (0x05A972, i_05a972),
    (0x05A976, i_05a976),
    (0x05A97A, i_05a97a),
    (0x05A97E, i_05a97e),
    (0x05A982, i_05a982),
    (0x05A986, i_05a986),
    (0x05A98A, i_05a98a),
    (0x05A98E, i_05a98e),
    (0x05A992, i_05a992),
    (0x05A996, i_05a996),
    (0x05A99A, i_05a99a),
    (0x05A99E, i_05a99e),
    (0x05A9A2, i_05a9a2),
    (0x05A9A6, i_05a9a6),
    (0x05A9AA, i_05a9aa),
    (0x05A9AE, i_05a9ae),
    (0x05A9B2, i_05a9b2),
    (0x05A9B6, i_05a9b6),
    (0x05A9BA, i_05a9ba),
    (0x05A9C0, i_05a9c0),
    (0x05A9C6, i_05a9c6),
    (0x05A9CA, i_05a9ca),
    (0x05A9CE, i_05a9ce),
    (0x05A9D4, i_05a9d4),
    (0x05A9D6, i_05a9d6),
    (0x05A9DA, i_05a9da),
    (0x05A9DE, i_05a9de),
    (0x05A9E2, i_05a9e2),
    (0x05A9E6, i_05a9e6),
    (0x05A9EA, i_05a9ea),
    (0x05A9EC, i_05a9ec),
    (0x05A9F2, i_05a9f2),
    (0x05A9F4, i_05a9f4),
    (0x05A9F8, i_05a9f8),
    (0x05A9FA, i_05a9fa),
    (0x05A9FE, i_05a9fe),
    (0x05AA02, i_05aa02),
    (0x05AA06, i_05aa06),
    (0x05AA0A, i_05aa0a),
    (0x05AA0E, i_05aa0e),
    (0x05AA10, i_05aa10),
    (0x05AA14, i_05aa14),
    (0x05AA18, i_05aa18),
    (0x05AA1C, i_05aa1c),
    (0x05AA20, i_05aa20),
    (0x05AA24, i_05aa24),
    (0x05AA26, i_05aa26),
    (0x05AA2A, i_05aa2a),
    (0x05AA2E, i_05aa2e),
    (0x05AA30, i_05aa30),
    (0x05AA34, i_05aa34),
    (0x05AA38, i_05aa38),
    (0x05AA3C, i_05aa3c),
    (0x05AA3E, i_05aa3e),
    (0x05AA40, i_05aa40),
    (0x05AA44, i_05aa44),
    (0x05AA48, i_05aa48),
    (0x05AA4A, i_05aa4a),
    (0x05AA4C, i_05aa4c),
    (0x05AA4E, i_05aa4e),
    (0x05AA50, i_05aa50),
    (0x05AA52, i_05aa52),
    (0x05AA56, i_05aa56),
    (0x05AA5A, i_05aa5a),
    (0x05AA5C, i_05aa5c),
    (0x05AA5E, i_05aa5e),
    (0x05AA60, i_05aa60),
    (0x05AA64, i_05aa64),
    (0x05AA66, i_05aa66),
    (0x05AA6A, i_05aa6a),
    (0x05AA6C, i_05aa6c),
    (0x05AA70, i_05aa70),
    (0x05AA72, i_05aa72),
    (0x05AA76, i_05aa76),
    (0x05AA7A, i_05aa7a),
    (0x05AA7E, i_05aa7e),
    (0x05AA82, i_05aa82),
    (0x05AA86, i_05aa86),
    (0x05AA8A, i_05aa8a),
    (0x05AA8E, i_05aa8e),
    (0x05AA92, i_05aa92),
    (0x05AA94, i_05aa94),
    (0x05AA96, i_05aa96),
    (0x05AA9A, i_05aa9a),
    (0x05AA9E, i_05aa9e),
    (0x05AAA2, i_05aaa2),
    (0x05AAA6, i_05aaa6),
    (0x05AAAA, i_05aaaa),
    (0x05AAAE, i_05aaae),
    (0x05AAB2, i_05aab2),
    (0x05AAB6, i_05aab6),
    (0x05AAB8, i_05aab8),
    (0x05AABC, i_05aabc),
    (0x05AABE, i_05aabe),
    (0x05AAC0, i_05aac0),
    (0x05AAC2, i_05aac2),
    (0x05AAC6, i_05aac6),
    (0x05AAC8, i_05aac8),
    (0x05AACA, i_05aaca),
    (0x05AACC, i_05aacc),
    (0x05AACE, i_05aace),
    (0x05AAD0, i_05aad0),
    (0x05AAD2, i_05aad2),
    (0x05AAD4, i_05aad4),
    (0x05AAD6, i_05aad6),
    (0x05AAD8, i_05aad8),
    (0x05AADA, i_05aada),
    (0x05AADC, i_05aadc),
    (0x05AADE, i_05aade),
    (0x05AAE0, i_05aae0),
    (0x05AAE2, i_05aae2),
    (0x05AAE6, i_05aae6),
    (0x05AAE8, i_05aae8),
    (0x05AAEA, i_05aaea),
    (0x05AAEC, i_05aaec),
    (0x05AAEE, i_05aaee),
    (0x05AAF0, i_05aaf0),
    (0x05AAF2, i_05aaf2),
    (0x05AAF6, i_05aaf6),
    (0x05AAFA, i_05aafa),
    (0x05AAFE, i_05aafe),
    (0x05AB02, i_05ab02),
    (0x05AB06, i_05ab06),
    (0x05AB08, i_05ab08),
    (0x05AB0A, i_05ab0a),
    (0x05AB0C, i_05ab0c),
    (0x05AB0E, i_05ab0e),
    (0x05AB10, i_05ab10),
    (0x05AB14, i_05ab14),
    (0x05AB16, i_05ab16),
    (0x05AB18, i_05ab18),
    (0x05AB1C, i_05ab1c),
    (0x05AB1E, i_05ab1e),
    (0x05AB20, i_05ab20),
    (0x05AB22, i_05ab22),
    (0x05AB26, i_05ab26),
    (0x05AB28, i_05ab28),
    (0x05AB2C, i_05ab2c),
    (0x05AB32, i_05ab32),
    (0x05AB34, i_05ab34),
    (0x05AB36, i_05ab36),
    (0x05AB3A, i_05ab3a),
    (0x05AB3C, i_05ab3c),
    (0x05AB3E, i_05ab3e),
    (0x05AB40, i_05ab40),
    (0x05AB44, i_05ab44),
    (0x05AB46, i_05ab46),
    (0x05AB48, i_05ab48),
    (0x05AB4A, i_05ab4a),
    (0x05AB4C, i_05ab4c),
    (0x05AB4E, i_05ab4e),
    (0x05AB50, i_05ab50),
    (0x05AB52, i_05ab52),
    (0x05AB54, i_05ab54),
    (0x05AB56, i_05ab56),
    (0x05AB58, i_05ab58),
    (0x05AB5A, i_05ab5a),
    (0x05AB5C, i_05ab5c),
    (0x05AB5E, i_05ab5e),
    (0x05AB60, i_05ab60),
    (0x05AB62, i_05ab62),
    (0x05AB64, i_05ab64),
    (0x05AB66, i_05ab66),
    (0x05AB6A, i_05ab6a),
    (0x05AB6C, i_05ab6c),
    (0x05AB6E, i_05ab6e),
    (0x05AB70, i_05ab70),
    (0x05AB72, i_05ab72),
    (0x05AB74, i_05ab74),
    (0x05AB76, i_05ab76),
    (0x05AB7A, i_05ab7a),
    (0x05AB7C, i_05ab7c),
    (0x05AB80, i_05ab80),
    (0x05AB84, i_05ab84),
    (0x05AB88, i_05ab88),
    (0x05AB8C, i_05ab8c),
    (0x05AB8E, i_05ab8e),
    (0x05AB90, i_05ab90),
    (0x05AB92, i_05ab92),
    (0x05AB94, i_05ab94),
    (0x05AB96, i_05ab96),
    (0x05AB9A, i_05ab9a),
    (0x05AB9C, i_05ab9c),
    (0x05ABA0, i_05aba0),
    (0x05ABA2, i_05aba2),
    (0x05ABA4, i_05aba4),
    (0x05ABA6, i_05aba6),
    (0x05ABAA, i_05abaa),
    (0x05ABAC, i_05abac),
    (0x05ABB0, i_05abb0),
    (0x05ABB6, i_05abb6),
    (0x05ABB8, i_05abb8),
    (0x05ABBA, i_05abba),
    (0x05ABBE, i_05abbe),
    (0x05ABC0, i_05abc0),
    (0x05ABC2, i_05abc2),
    (0x05ABC4, i_05abc4),
    (0x05ABC8, i_05abc8),
    (0x05ABCA, i_05abca),
    (0x05ABCC, i_05abcc),
    (0x05ABCE, i_05abce),
    (0x05ABD0, i_05abd0),
    (0x05ABD2, i_05abd2),
    (0x05ABD4, i_05abd4),
    (0x05ABD6, i_05abd6),
    (0x05ABD8, i_05abd8),
    (0x05ABDA, i_05abda),
    (0x05ABDC, i_05abdc),
    (0x05ABDE, i_05abde),
    (0x05ABE0, i_05abe0),
    (0x05ABE2, i_05abe2),
    (0x05ABE4, i_05abe4),
    (0x05ABE6, i_05abe6),
    (0x05ABE8, i_05abe8),
    (0x05ABEA, i_05abea),
    (0x05ABEC, i_05abec),
    (0x05ABEE, i_05abee),
    (0x05ABF2, i_05abf2),
    (0x05ABF4, i_05abf4),
    (0x05ABF6, i_05abf6),
    (0x05ABF8, i_05abf8),
    (0x05ABFA, i_05abfa),
    (0x05ABFC, i_05abfc),
    (0x05ABFE, i_05abfe),
    (0x05AC00, i_05ac00),
    (0x05AC04, i_05ac04),
    (0x05AC08, i_05ac08),
    (0x05AC0C, i_05ac0c),
    (0x05AC10, i_05ac10),
    (0x05AC14, i_05ac14),
    (0x05AC16, i_05ac16),
    (0x05AC18, i_05ac18),
    (0x05AC1A, i_05ac1a),
    (0x05AC1C, i_05ac1c),
    (0x05AC1E, i_05ac1e),
    (0x05AC22, i_05ac22),
    (0x05AC24, i_05ac24),
    (0x05AC26, i_05ac26),
    (0x05AC2A, i_05ac2a),
    (0x05AC2C, i_05ac2c),
    (0x05AC2E, i_05ac2e),
    (0x05AC30, i_05ac30),
    (0x05AC32, i_05ac32),
    (0x05AC36, i_05ac36),
    (0x05AC38, i_05ac38),
    (0x05AC3C, i_05ac3c),
    (0x05AC42, i_05ac42),
    (0x05AC44, i_05ac44),
    (0x05AC46, i_05ac46),
    (0x05AC4A, i_05ac4a),
    (0x05AC4C, i_05ac4c),
    (0x05AC4E, i_05ac4e),
    (0x05AC50, i_05ac50),
    (0x05AC54, i_05ac54),
    (0x05AC56, i_05ac56),
    (0x05AC58, i_05ac58),
    (0x05AC5A, i_05ac5a),
    (0x05AC5C, i_05ac5c),
    (0x05AC5E, i_05ac5e),
    (0x05AC60, i_05ac60),
    (0x05AC62, i_05ac62),
    (0x05AC64, i_05ac64),
    (0x05AC66, i_05ac66),
    (0x05AC68, i_05ac68),
    (0x05AC6A, i_05ac6a),
    (0x05AC6C, i_05ac6c),
    (0x05AC6E, i_05ac6e),
    (0x05AC70, i_05ac70),
    (0x05AC72, i_05ac72),
    (0x05AC74, i_05ac74),
    (0x05AC76, i_05ac76),
    (0x05AC78, i_05ac78),
    (0x05AC7A, i_05ac7a),
    (0x05AC7C, i_05ac7c),
    (0x05AC7E, i_05ac7e),
    (0x05AC80, i_05ac80),
    (0x05AC84, i_05ac84),
    (0x05AC86, i_05ac86),
    (0x05AC88, i_05ac88),
    (0x05AC8A, i_05ac8a),
    (0x05AC8C, i_05ac8c),
    (0x05AC8E, i_05ac8e),
    (0x05AC90, i_05ac90),
    (0x05AC92, i_05ac92),
    (0x05AC96, i_05ac96),
    (0x05AC98, i_05ac98),
    (0x05AC9C, i_05ac9c),
    (0x05ACA0, i_05aca0),
    (0x05ACA4, i_05aca4),
    (0x05ACA8, i_05aca8),
    (0x05ACAA, i_05acaa),
    (0x05ACAC, i_05acac),
    (0x05ACAE, i_05acae),
    (0x05ACB0, i_05acb0),
    (0x05ACB2, i_05acb2),
    (0x05ACB6, i_05acb6),
    (0x05ACB8, i_05acb8),
    (0x05ACBC, i_05acbc),
    (0x05ACBE, i_05acbe),
    (0x05ACC0, i_05acc0),
    (0x05ACC2, i_05acc2),
    (0x05ACC4, i_05acc4),
    (0x05ACC8, i_05acc8),
    (0x05ACCA, i_05acca),
    (0x05ACCE, i_05acce),
    (0x05ACD4, i_05acd4),
    (0x05ACD6, i_05acd6),
    (0x05ACD8, i_05acd8),
    (0x05ACDC, i_05acdc),
    (0x05ACDE, i_05acde),
    (0x05ACE0, i_05ace0),
    (0x05ACE2, i_05ace2),
    (0x05ACE6, i_05ace6),
    (0x05ACE8, i_05ace8),
    (0x05ACEA, i_05acea),
    (0x05ACEC, i_05acec),
    (0x05ACEE, i_05acee),
    (0x05ACF2, i_05acf2),
    (0x05ACF4, i_05acf4),
    (0x05ACF6, i_05acf6),
    (0x05ACFA, i_05acfa),
    (0x05ACFE, i_05acfe),
    (0x05AD00, i_05ad00),
    (0x05AD04, i_05ad04),
    (0x05AD06, i_05ad06),
    (0x05AD0A, i_05ad0a),
    (0x05AD0C, i_05ad0c),
    (0x05AD0E, i_05ad0e),
    (0x05AD12, i_05ad12),
    (0x05AD14, i_05ad14),
    (0x05AD16, i_05ad16),
    (0x05AD18, i_05ad18),
    (0x05AD1A, i_05ad1a),
    (0x05AD1C, i_05ad1c),
    (0x05AD1E, i_05ad1e),
    (0x05AD20, i_05ad20),
    (0x05AD22, i_05ad22),
    (0x05AD24, i_05ad24),
    (0x05AD26, i_05ad26),
    (0x05AD28, i_05ad28),
    (0x05AD2A, i_05ad2a),
    (0x05AD2C, i_05ad2c),
    (0x05AD2E, i_05ad2e),
    (0x05AD30, i_05ad30),
    (0x05AD32, i_05ad32),
    (0x05AD34, i_05ad34),
    (0x05AD36, i_05ad36),
    (0x05AD3A, i_05ad3a),
    (0x05AD3C, i_05ad3c),
    (0x05AD3E, i_05ad3e),
    (0x05AD40, i_05ad40),
    (0x05AD42, i_05ad42),
    (0x05AD46, i_05ad46),
    (0x05AD4A, i_05ad4a),
    (0x05AD4C, i_05ad4c),
    (0x05AD4E, i_05ad4e),
    (0x05AD50, i_05ad50),
    (0x05AD52, i_05ad52),
    (0x05AD56, i_05ad56),
    (0x05AD5A, i_05ad5a),
    (0x05AD5C, i_05ad5c),
    (0x05AD60, i_05ad60),
    (0x05AD62, i_05ad62),
    (0x05AD64, i_05ad64),
    (0x05AD66, i_05ad66),
    (0x05AD6A, i_05ad6a),
    (0x05AD6C, i_05ad6c),
    (0x05AD6E, i_05ad6e),
    (0x05AD70, i_05ad70),
    (0x05AD72, i_05ad72),
    (0x05AD74, i_05ad74),
    (0x05AD76, i_05ad76),
    (0x05AD78, i_05ad78),
    (0x05AD7C, i_05ad7c),
    (0x05AD7E, i_05ad7e),
    (0x05AD80, i_05ad80),
    (0x05AD82, i_05ad82),
    (0x05AD84, i_05ad84),
    (0x05AD86, i_05ad86),
    (0x05AD88, i_05ad88),
    (0x05AD8A, i_05ad8a),
    (0x05AD8C, i_05ad8c),
    (0x05AD90, i_05ad90),
    (0x05AD92, i_05ad92),
    (0x05AD94, i_05ad94),
    (0x05AD96, i_05ad96),
    (0x05AD98, i_05ad98),
    (0x05AD9A, i_05ad9a),
    (0x05AD9C, i_05ad9c),
    (0x05AD9E, i_05ad9e),
    (0x05ADA2, i_05ada2),
    (0x05ADA6, i_05ada6),
    (0x05ADAA, i_05adaa),
    (0x05ADAC, i_05adac),
    (0x05ADAE, i_05adae),
    (0x05ADB2, i_05adb2),
    (0x05ADB4, i_05adb4),
    (0x05ADB8, i_05adb8),
    (0x05ADBC, i_05adbc),
    (0x05ADBE, i_05adbe),
    (0x05ADC2, i_05adc2),
    (0x05ADC6, i_05adc6),
    (0x05ADCA, i_05adca),
    (0x05ADCE, i_05adce),
    (0x05ADD0, i_05add0),
    (0x05ADD2, i_05add2),
    (0x05ADD4, i_05add4),
    (0x05ADD6, i_05add6),
    (0x05ADD8, i_05add8),
    (0x05ADDC, i_05addc),
    (0x05ADDE, i_05adde),
    (0x05ADE0, i_05ade0),
    (0x05ADE2, i_05ade2),
    (0x05ADE6, i_05ade6),
    (0x05ADE8, i_05ade8),
    (0x05ADEC, i_05adec),
    (0x05ADEE, i_05adee),
    (0x05ADF2, i_05adf2),
    (0x05ADF4, i_05adf4),
    (0x05ADF8, i_05adf8),
    (0x05ADFE, i_05adfe),
    (0x05AE00, i_05ae00),
    (0x05AE02, i_05ae02),
    (0x05AE06, i_05ae06),
    (0x05AE08, i_05ae08),
    (0x05AE0A, i_05ae0a),
];
