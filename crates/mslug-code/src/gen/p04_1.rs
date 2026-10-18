// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0466A6: move.w #$78, $70(a6)
pub fn i_0466a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_0466ac  ($0466AC, 8 B, kernel/task_handlers.c)
/// $0466AC: lea.l $466b4(pc), a1
pub fn i_0466ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x466B4;
    16
}
/// $0466B0: move.l a1, (a6)
pub fn i_0466b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0466B2: rts
pub fn i_0466b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneC_Load_Spawn2_0466b4  ($0466B4, 30 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0466B4: subq.w #$1, $70(a6)
pub fn i_0466b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466B8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0466B8: cmpi.w #$0, $70(a6)
pub fn i_0466b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466BE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0466BE: bgt.w $466d8
pub fn i_0466be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466C2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x466D8; return 10; }
    8
}
/// $0466C2: move.w #$2, d0
pub fn i_0466c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0466C6: jsr $5239e.l
pub fn i_0466c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x466CC);
    cpu.pc = 0x5239E;
    16
}
/// $0466CC: move.w #$3c, $70(a6)
pub fn i_0466cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_0466d2  ($0466D2, 8 B, kernel/task_handlers.c)
/// $0466D2: lea.l $466da(pc), a1
pub fn i_0466d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x466DA;
    16
}
/// $0466D6: move.l a1, (a6)
pub fn i_0466d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0466D8: rts
pub fn i_0466d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneC_Load_Finish_0466da  ($0466DA, 28 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0466DA: subq.w #$1, $70(a6)
pub fn i_0466da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466DE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0466DE: cmpi.w #$0, $70(a6)
pub fn i_0466de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466E4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0466E4: bgt.w $466f4
pub fn i_0466e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466E8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x466F4; return 10; }
    8
}
/// $0466E8: clr.b $106ed2.l
pub fn i_0466e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466EE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $0466EE: jmp $518.l
pub fn i_0466ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466F4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0466F4: rts
pub fn i_0466f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ContinueDigits_P1P2_Task_0466f6  ($0466F6, 256 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0466F6: move.w #$0, $74(a6)
pub fn i_0466f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0466FC: move.w #$7183, d0
pub fn i_0466fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46700;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7183;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046700: move.w d0, $22(a6)
pub fn i_046700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46704;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046704: jsr $260c4.l
pub fn i_046704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4670A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4670A);
    cpu.pc = 0x260C4;
    16
}
/// $04670A: jsr $2604c.l
pub fn i_04670a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46710);
    cpu.pc = 0x2604C;
    16
}
/// $046710: bra.w $4673a
pub fn i_046710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46714;
    cpu.ir = 0x6000;
    cpu.pc = 0x4673A;
    10
}
/// $04673A: move.w #$9, $5c(a6)
pub fn i_04673a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46740;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046740: move.w #$3b, $70(a6)
pub fn i_046740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46746;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046746: move.w $22(a6), d0
pub fn i_046746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4674A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04674A: move.w $5c(a6), d1
pub fn i_04674a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4674E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04674E: addi.w #$3460, d1
pub fn i_04674e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46752;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x3460, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046752: movem.w d0-d1, $3c0000.l
pub fn i_046752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4675A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $04675A: addq.w #$1, d0
pub fn i_04675a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4675C;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04675C: addi.w #$10, d1
pub fn i_04675c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46760;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $046760: movem.w d0-d1, $3c0000.l
pub fn i_046760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46768;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $046768: lea.l $4676e(pc), a1
pub fn i_046768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4676C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4676E;
    16
}
/// $04676C: move.l a1, (a6)
pub fn i_04676c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4676E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04676E: movea.l $c(a6), a1
pub fn i_04676e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46772;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $046772: cmpi.b #$0, $20(a1)
pub fn i_046772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46778;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $046778: bne.w $46784
pub fn i_046778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4677C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46784; return 10; }
    8
}
/// $04677C: jmp $518.l
pub fn i_04677c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46782;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $046784: cmpi.w #$0, $74(a6)
pub fn i_046784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4678A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04678A: bne.w $46796
pub fn i_04678a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4678E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46796; return 10; }
    8
}
/// $04678E: jsr $4694a(pc)
pub fn i_04678e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46792;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x46792);
    cpu.pc = 0x4694A;
    16
}
/// $046792: bra.w $4679a
pub fn i_046792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46796;
    cpu.ir = 0x6000;
    cpu.pc = 0x4679A;
    10
}
/// $046796: jsr $4698c(pc)
pub fn i_046796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4679A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4679A);
    cpu.pc = 0x4698C;
    16
}
/// $04679A: bcc.w $467a2
pub fn i_04679a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4679E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x467A2; return 10; }
    8
}
/// $04679E: clr.w $70(a6)
pub fn i_04679e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467A2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0467A2: subq.w #$1, $70(a6)
pub fn i_0467a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467A6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0467A6: cmpi.w #$0, $70(a6)
pub fn i_0467a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467AC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0467AC: bgt.w $467f4
pub fn i_0467ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467B0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x467F4; return 10; }
    8
}
/// $0467B0: subq.w #$1, $5c(a6)
pub fn i_0467b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467B4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0467B4: cmpi.w #$0, $5c(a6)
pub fn i_0467b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467BA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0467BA: bge.w $467cc
pub fn i_0467ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467BE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x467CC; return 10; }
    8
}
/// $0467BE: movea.l $c(a6), a1
pub fn i_0467be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467C2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0467C2: clr.b $21(a1)
pub fn i_0467c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467C6;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0467C6: jmp $518.l
pub fn i_0467c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467CC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0467CC: move.w $22(a6), d0
pub fn i_0467cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0467D0: move.w $5c(a6), d1
pub fn i_0467d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0467D4: addi.w #$3460, d1
pub fn i_0467d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467D8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x3460, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0467D8: movem.w d0-d1, $3c0000.l
pub fn i_0467d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467E0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0467E0: addq.w #$1, d0
pub fn i_0467e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467E2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0467E2: addi.w #$10, d1
pub fn i_0467e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467E6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0467E6: movem.w d0-d1, $3c0000.l
pub fn i_0467e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467EE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0467EE: move.w #$3b, $70(a6)
pub fn i_0467ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0467F4: rts
pub fn i_0467f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ContinueBig_Init_0467f6  ($0467F6, 130 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0467F6: move.w #$0, $74(a6)
pub fn i_0467f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x467FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0467FC: bra.w $46806
pub fn i_0467fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46800;
    cpu.ir = 0x6000;
    cpu.pc = 0x46806;
    10
}
/// $046806: move.w #$ffff, $38(a6)
pub fn i_046806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4680C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04680C: bset.b #$6, $12(a6)
pub fn i_04680c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46812;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $046812: move.w #$5f, d1
pub fn i_046812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46816;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046816: jsr $236e.l
pub fn i_046816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4681C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4681C);
    cpu.pc = 0x236E;
    16
}
/// $04681C: move.w $14(a6), d1
pub fn i_04681c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46820;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $046820: jsr $2c66.l
pub fn i_046820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46826);
    cpu.pc = 0x2C66;
    16
}
/// $046826: lea.l $28de2a.l, a0
pub fn i_046826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4682C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28DE2A;
    20
}
/// $04682C: jsr $28cd4.l
pub fn i_04682c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46832);
    cpu.pc = 0x28CD4;
    16
}
/// $046832: move.w #$a0, $22(a6)
pub fn i_046832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46838;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046838: move.w #$178, $24(a6)
pub fn i_046838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4683E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04683E: clr.w $26(a6)
pub fn i_04683e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46842;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $046842: move.w #$9, $5c(a6)
pub fn i_046842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46848;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $046848: move.w #$3b, $70(a6)
pub fn i_046848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4684E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04684E: move.b #$ff, $72(a6)
pub fn i_04684e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46854;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046854: movea.w #$716a, a1
pub fn i_046854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46858;
    cpu.ir = 0x327C;
    let v1: u32 = 0x716A;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046858: lea.l $28de36.l, a2
pub fn i_046858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4685E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x28DE36;
    20
}
/// $04685E: move.w #$4, d1
pub fn i_04685e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46862;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046862: jsr $4784c.l
pub fn i_046862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46868);
    cpu.pc = 0x4784C;
    16
}
/// $046868: lea.l $46878(pc), a1
pub fn i_046868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4686C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46878;
    16
}
/// $04686C: move.l a1, (a6)
pub fn i_04686c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4686E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04686E: move.w #$10d4, d0
pub fn i_04686e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46872;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046872: jsr $2352.l
pub fn i_046872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46878);
    cpu.pc = 0x2352;
    16
}
// ===== ContinueBig_Countdown_046878  ($046878, 174 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046878: movea.l $c(a6), a1
pub fn i_046878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4687C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04687C: cmpi.b #$0, $20(a1)
pub fn i_04687c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46882;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $046882: bne.w $4688a
pub fn i_046882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46886;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4688A; return 10; }
    8
}
/// $046886: bra.w $4692e
pub fn i_046886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4688A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4692E;
    10
}
/// $04688A: jsr $469ce(pc)
pub fn i_04688a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4688E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4688E);
    cpu.pc = 0x469CE;
    16
}
/// $04688E: bcc.w $4689c
pub fn i_04688e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46892;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4689C; return 10; }
    8
}
/// $046892: move.b #$ff, $72(a6)
pub fn i_046892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46898;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046898: clr.w $70(a6)
pub fn i_046898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4689C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04689C: subq.w #$1, $70(a6)
pub fn i_04689c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468A0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0468A0: cmpi.w #$0, $70(a6)
pub fn i_0468a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468A6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0468A6: bgt.w $468ea
pub fn i_0468a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468AA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x468EA; return 10; }
    8
}
/// $0468AA: subq.w #$1, $5c(a6)
pub fn i_0468aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468AE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0468AE: cmpi.w #$0, $5c(a6)
pub fn i_0468ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0468B4: bge.w $468c4
pub fn i_0468b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468B8;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x468C4; return 10; }
    8
}
/// $0468B8: movea.l $c(a6), a1
pub fn i_0468b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468BC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0468BC: clr.b $21(a1)
pub fn i_0468bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468C0;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0468C0: bra.w $4692e
pub fn i_0468c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4692E;
    10
}
/// $0468C4: move.w #$10d4, d0
pub fn i_0468c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468C8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0468C8: jsr $2352.l
pub fn i_0468c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x468CE);
    cpu.pc = 0x2352;
    16
}
/// $0468CE: move.w #$3b, $70(a6)
pub fn i_0468ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0468D4: lea.l $28dd96.l, a1
pub fn i_0468d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DD96;
    20
}
/// $0468DA: move.w $5c(a6), d0
pub fn i_0468da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0468DE: lsl.w #$2, d0
pub fn i_0468de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468E0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0468E0: movea.l (a1, d0.w), a0
pub fn i_0468e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468E4;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0468E4: jsr $28cd4.l
pub fn i_0468e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x468EA);
    cpu.pc = 0x28CD4;
    16
}
/// $0468EA: lea.l $28de54.l, a1
pub fn i_0468ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DE54;
    20
}
/// $0468F0: move.w $70(a6), d1
pub fn i_0468f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468F4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0468F4: move.b (a1, d1.w), d0
pub fn i_0468f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468F8;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0468F8: cmpi.w #$37, $70(a6)
pub fn i_0468f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x468FE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x37, Size::Word, false);
    16
}
/// $0468FE: ble.w $4691e
pub fn i_0468fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46902;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4691E; return 10; }
    8
}
/// $046902: cmpi.b #$0, $72(a6)
pub fn i_046902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46908;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $046908: bne.w $46914
pub fn i_046908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4690C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x46914; return 10; }
    8
}
/// $04690C: move.b d0, $32(a6)
pub fn i_04690c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46910;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $046910: bra.w $4691a
pub fn i_046910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46914;
    cpu.ir = 0x6000;
    cpu.pc = 0x4691A;
    10
}
/// $046914: move.b #$ff, $32(a6)
pub fn i_046914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4691A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04691A: bra.w $46926
pub fn i_04691a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4691E;
    cpu.ir = 0x6000;
    cpu.pc = 0x46926;
    10
}
/// $04691E: clr.b $72(a6)
pub fn i_04691e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46922;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $046922: move.b d0, $32(a6)
pub fn i_046922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46926;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrAbsThunk_046926  ($046926, 8 B, thunks/jsr_abs_thunks.c)
/// $046926: jsr $28d70.l
pub fn i_046926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4692C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4692C);
    cpu.pc = 0x28D70;
    16
}
/// $04692C: rts
pub fn i_04692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4692E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ContinueBig_ClearAndExit_04692e  ($04692E, 26 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04692E: movea.w #$716a, a1
pub fn i_04692e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46932;
    cpu.ir = 0x327C;
    let v1: u32 = 0x716A;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046932: lea.l $28de40.l, a2
pub fn i_046932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46938;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x28DE40;
    20
}
/// $046938: move.w #$4, d1
pub fn i_046938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4693C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04693C: jsr $4784c.l
pub fn i_04693c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46942);
    cpu.pc = 0x4784C;
    16
}
/// $046942: jmp $518.l
pub fn i_046942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46948;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Continue_IsStartP1_04694a  ($04694A, 12 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04694A: cmpi.b #$2, $10fdb6.l
pub fn i_04694a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46952;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $046952: beq.w $4695c
pub fn i_046952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46956;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4695C; return 10; }
    8
}
// ===== ClearXN_046956  ($046956, 6 B, thunks/ccr_helpers.c)
/// $046956: andi.b #$ee, ccr
pub fn i_046956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4695A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04695A: rts
pub fn i_04695a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4695C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP1_CheckCount_04695c  ($04695C, 10 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04695C: cmpi.w #$8, $5c(a6)
pub fn i_04695c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46962;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $046962: bls.w $4696c
pub fn i_046962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46966;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x4696C; return 10; }
    8
}
// ===== ClearXN_046966  ($046966, 6 B, thunks/ccr_helpers.c)
/// $046966: andi.b #$ee, ccr
pub fn i_046966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4696A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04696A: rts
pub fn i_04696a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4696C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP1_CheckTask_04696c  ($04696C, 14 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04696C: move.b $10e203.l, d0
pub fn i_04696c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46972;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E203) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $046972: andi.b #$f0, d0
pub fn i_046972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46976;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $046976: beq.w $46984
pub fn i_046976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4697A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x46984; return 10; }
    8
}
// ===== SetXN_04697a  ($04697A, 6 B, thunks/ccr_helpers.c)
/// $04697A: ori.b #$11, ccr
pub fn i_04697a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4697E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04697E: rts
pub fn i_04697e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46980;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_046984  ($046984, 6 B, thunks/ccr_helpers.c)
/// $046984: andi.b #$ee, ccr
pub fn i_046984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46988;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $046988: rts
pub fn i_046988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4698A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP2_04698c  ($04698C, 12 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04698C: cmpi.b #$2, $10fdb7.l
pub fn i_04698c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46994;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $046994: beq.w $4699e
pub fn i_046994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46998;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4699E; return 10; }
    8
}
// ===== ClearXN_046998  ($046998, 6 B, thunks/ccr_helpers.c)
/// $046998: andi.b #$ee, ccr
pub fn i_046998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4699C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04699C: rts
pub fn i_04699c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4699E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP2_CheckCount_04699e  ($04699E, 10 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04699E: cmpi.w #$8, $5c(a6)
pub fn i_04699e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469A4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $0469A4: bls.w $469ae
pub fn i_0469a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469A8;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x469AE; return 10; }
    8
}
// ===== ClearXN_0469a8  ($0469A8, 6 B, thunks/ccr_helpers.c)
/// $0469A8: andi.b #$ee, ccr
pub fn i_0469a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469AC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0469AC: rts
pub fn i_0469ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartP2_CheckTask_0469ae  ($0469AE, 14 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469AE: move.b $10e209.l, d0
pub fn i_0469ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469B4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E209) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0469B4: andi.b #$f0, d0
pub fn i_0469b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469B8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0469B8: beq.w $469c6
pub fn i_0469b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x469C6; return 10; }
    8
}
// ===== SetXN_0469bc  ($0469BC, 6 B, thunks/ccr_helpers.c)
/// $0469BC: ori.b #$11, ccr
pub fn i_0469bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469C0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0469C0: rts
pub fn i_0469c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0469c6  ($0469C6, 6 B, thunks/ccr_helpers.c)
/// $0469C6: andi.b #$ee, ccr
pub fn i_0469c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0469CA: rts
pub fn i_0469ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_IsStartAny_0469ce  ($0469CE, 10 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469CE: jsr $4694a(pc)
pub fn i_0469ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x469D2);
    cpu.pc = 0x4694A;
    16
}
/// $0469D2: bcc.w $469dc
pub fn i_0469d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x469DC; return 10; }
    8
}
/// $0469D6: rts
pub fn i_0469d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_0469dc  ($0469DC, 6 B, thunks/jsr_pc_thunks.c)
/// $0469DC: jsr $4698c(pc)
pub fn i_0469dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x469E0);
    cpu.pc = 0x4698C;
    16
}
/// $0469E0: rts
pub fn i_0469e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Continue_StoreCount_Exit_0469e2  ($0469E2, 26 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0469E2: move.b $98(a6), d0
pub fn i_0469e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469E6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0469E6: cmpi.b #$ff, d0
pub fn i_0469e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469EA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0469EA: bne.w $469f0
pub fn i_0469ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x469F0; return 10; }
    8
}
/// $0469EE: clr.b d0
pub fn i_0469ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469F0;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0469F0: move.b d0, $1081b0.l
pub fn i_0469f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469F6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081B0, v1 as u8);
    16
}
/// $0469F6: jmp $518.l
pub fn i_0469f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x469FC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== FixLayer_QuadBatch_046AC6  ($046AC6, 90 B, entity/misc_helpers_024fec.s)
/// $046AC6: movea.w #$7000, a1
pub fn i_046ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ACA;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046ACA: move.w #$20, d0
pub fn i_046aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ACE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046ACE: move.w #$28, d1
pub fn i_046ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AD2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AD2: move.w #$2, d2
pub fn i_046ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AD6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AD6: jsr $5da9c.l
pub fn i_046ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46ADC);
    cpu.pc = 0x5DA9C;
    16
}
/// $046ADC: movea.w #$701e, a1
pub fn i_046adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AE0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x701E;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046AE0: move.w #$20, d0
pub fn i_046ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AE4: move.w #$28, d1
pub fn i_046ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AE8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AE8: move.w #$2, d2
pub fn i_046ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AEC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AEC: jsr $5da9c.l
pub fn i_046aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46AF2);
    cpu.pc = 0x5DA9C;
    16
}
/// $046AF2: movea.w #$7000, a1
pub fn i_046af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AF6;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046AF6: move.w #$20, d0
pub fn i_046af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AFA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AFA: move.w #$1, d1
pub fn i_046afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46AFE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046AFE: move.w #$20, d2
pub fn i_046afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B02;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B02: jsr $5da9c.l
pub fn i_046b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46B08);
    cpu.pc = 0x5DA9C;
    16
}
/// $046B08: movea.w #$74e0, a1
pub fn i_046b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B0C;
    cpu.ir = 0x327C;
    let v1: u32 = 0x74E0;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $046B0C: move.w #$20, d0
pub fn i_046b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B10;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B10: move.w #$1, d1
pub fn i_046b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B14: move.w #$20, d2
pub fn i_046b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B18;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046B18: jsr $5da9c.l
pub fn i_046b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46B1E);
    cpu.pc = 0x5DA9C;
    16
}
/// $046B1E: rts
pub fn i_046b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46B20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionNum_TileByMission_04707e  ($04707E, 24 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04707E: clr.l d1
pub fn i_04707e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47080;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $047080: move.b $106ecf.l, d1
pub fn i_047080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47086;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106ECF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $047086: andi.w #$7, d1
pub fn i_047086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4708A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04708A: lsl.w #$1, d1
pub fn i_04708a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4708C;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04708C: lea.l $28df00.l, a1
pub fn i_04708c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47092;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DF00;
    20
}
/// $047092: move.w (a1, d1.w), d0
pub fn i_047092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47096;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
// ===== JsrAbsThunk_047096  ($047096, 8 B, thunks/jsr_abs_thunks.c)
/// $047096: jsr $2352.l
pub fn i_047096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4709C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4709C);
    cpu.pc = 0x2352;
    16
}
/// $04709C: rts
pub fn i_04709c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4709E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionNum_PalByMission_04709e  ($04709E, 24 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04709E: clr.l d1
pub fn i_04709e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470A0;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $0470A0: move.b $106ecf.l, d1
pub fn i_0470a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470A6;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106ECF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0470A6: andi.w #$7, d1
pub fn i_0470a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470AA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0470AA: lsl.w #$1, d1
pub fn i_0470aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470AC;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0470AC: lea.l $28df10.l, a1
pub fn i_0470ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470B2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DF10;
    20
}
/// $0470B2: move.w (a1, d1.w), d0
pub fn i_0470b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470B6;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
// ===== JsrAbsThunk_0470b6  ($0470B6, 8 B, thunks/jsr_abs_thunks.c)
/// $0470B6: jsr $2352.l
pub fn i_0470b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x470BC);
    cpu.pc = 0x2352;
    16
}
/// $0470BC: rts
pub fn i_0470bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debug_DrawHexCounter_047482  ($047482, 156 B, video/debug_hex_counter_047482.s)
/// $047482: cmpi.b #$1, $10fdaf.l
pub fn i_047482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4748A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $04748A: bne.w $47490
pub fn i_04748a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4748E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x47490; return 10; }
    8
}
/// $04748E: rts
pub fn i_04748e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47490;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $047490: cmpi.w #$63, $106e92.l
pub fn i_047490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47498;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106E92) as u32;
    sub_with_flags(cpu, v2, 0x63, Size::Word, false);
    20
}
/// $047498: bls.w $4751e
pub fn i_047498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4749C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x4751E; return 10; }
    8
}
/// $04749C: move.w #$7243, d0
pub fn i_04749c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7243;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0474A0: move.w #$63b6, d1
pub fn i_0474a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474A4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x63B6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0474A4: movem.w d0-d1, $3c0000.l
pub fn i_0474a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474AC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0474AC: addi.w #$20, d0
pub fn i_0474ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474B0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0474B0: addq.w #$1, d1
pub fn i_0474b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474B2;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0474B2: movem.w d0-d1, $3c0000.l
pub fn i_0474b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474BA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0474BA: subi.w #$1f, d0
pub fn i_0474ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474BE;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0474BE: addi.w #$f, d1
pub fn i_0474be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474C2;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0474C2: movem.w d0-d1, $3c0000.l
pub fn i_0474c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474CA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0474CA: addi.w #$20, d0
pub fn i_0474ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474CE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0474CE: addq.w #$1, d1
pub fn i_0474ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474D0;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0474D0: movem.w d0-d1, $3c0000.l
pub fn i_0474d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474D8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0474D8: subi.w #$21, d0
pub fn i_0474d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474DC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0474DC: move.w #$7283, d0
pub fn i_0474dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7283;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0474E0: move.w #$63b8, d1
pub fn i_0474e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474E4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x63B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0474E4: movem.w d0-d1, $3c0000.l
pub fn i_0474e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474EC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0474EC: addi.w #$20, d0
pub fn i_0474ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474F0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0474F0: addq.w #$1, d1
pub fn i_0474f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474F2;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0474F2: movem.w d0-d1, $3c0000.l
pub fn i_0474f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474FA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0474FA: subi.w #$1f, d0
pub fn i_0474fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x474FE;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0474FE: addi.w #$f, d1
pub fn i_0474fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47502;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047502: movem.w d0-d1, $3c0000.l
pub fn i_047502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4750A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $04750A: addi.w #$20, d0
pub fn i_04750a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4750E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04750E: addq.w #$1, d1
pub fn i_04750e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47510;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047510: movem.w d0-d1, $3c0000.l
pub fn i_047510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47518;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047518: subi.w #$21, d0
pub fn i_047518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4751C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04751C: rts
pub fn i_04751c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4751E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debug_DrawHexCounter_ClampBranch_04751E  ($04751E, 138 B, video/debug_hex_counter_047482.s)
/// $04751E: cmpi.b #$0, d1
pub fn i_04751e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47522;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $047522: bne.w $475a8
pub fn i_047522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47526;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x475A8; return 10; }
    8
}
/// $047526: move.w #$7243, d0
pub fn i_047526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4752A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7243;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04752A: move.w #$4b40, d1
pub fn i_04752a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4752E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04752E: movem.w d0-d1, $3c0000.l
pub fn i_04752e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47536;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047536: addi.w #$20, d0
pub fn i_047536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4753A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04753A: addq.w #$1, d1
pub fn i_04753a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4753C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04753C: movem.w d0-d1, $3c0000.l
pub fn i_04753c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47544;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047544: subi.w #$1f, d0
pub fn i_047544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47548;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047548: addi.w #$f, d1
pub fn i_047548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4754C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04754C: movem.w d0-d1, $3c0000.l
pub fn i_04754c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47554;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047554: addi.w #$20, d0
pub fn i_047554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47558;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047558: addq.w #$1, d1
pub fn i_047558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4755A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04755A: movem.w d0-d1, $3c0000.l
pub fn i_04755a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47562;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047562: subi.w #$21, d0
pub fn i_047562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47566;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047566: move.w #$7283, d0
pub fn i_047566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4756A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7283;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04756A: move.w #$4b40, d1
pub fn i_04756a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4756E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04756E: movem.w d0-d1, $3c0000.l
pub fn i_04756e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47576;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047576: addi.w #$20, d0
pub fn i_047576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4757A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04757A: addq.w #$1, d1
pub fn i_04757a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4757C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04757C: movem.w d0-d1, $3c0000.l
pub fn i_04757c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47584;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047584: subi.w #$1f, d0
pub fn i_047584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47588;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047588: addi.w #$f, d1
pub fn i_047588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4758C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04758C: movem.w d0-d1, $3c0000.l
pub fn i_04758c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47594;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047594: addi.w #$20, d0
pub fn i_047594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47598;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047598: addq.w #$1, d1
pub fn i_047598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4759A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04759A: movem.w d0-d1, $3c0000.l
pub fn i_04759a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475A2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0475A2: subi.w #$21, d0
pub fn i_0475a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475A6;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0475A6: rts
pub fn i_0475a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debug_DrawHexCounter_Fallback_0475A8  ($0475A8, 166 B, video/debug_hex_counter_047482.s)
/// $0475A8: move.w $106e92.l, d0
pub fn i_0475a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475AE;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106E92) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0475AE: cmpi.w #$63, d0
pub fn i_0475ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475B2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x63, Size::Word, false);
    8
}
/// $0475B2: bls.w $475ba
pub fn i_0475b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475B6;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x475BA; return 10; }
    8
}
/// $0475B6: move.w #$63, d0
pub fn i_0475b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0475BA: jsr $47656(pc)
pub fn i_0475ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x475BE);
    cpu.pc = 0x47656;
    16
}
/// $0475BE: lea.l $28decc.l, a1
pub fn i_0475be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475C4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DECC;
    20
}
/// $0475C4: lsl.w #$1, d0
pub fn i_0475c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475C6;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0475C6: move.w (a1, d0.w), d2
pub fn i_0475c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475CA;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0475CA: lsl.w #$1, d1
pub fn i_0475ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475CC;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0475CC: move.w (a1, d1.w), d3
pub fn i_0475cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475D0;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0475D0: move.w #$7243, d0
pub fn i_0475d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7243;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0475D4: move.w d3, d1
pub fn i_0475d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475D6;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0475D6: movem.w d0-d1, $3c0000.l
pub fn i_0475d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475DE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0475DE: addi.w #$20, d0
pub fn i_0475de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475E2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0475E2: addq.w #$1, d1
pub fn i_0475e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475E4;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0475E4: movem.w d0-d1, $3c0000.l
pub fn i_0475e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475EC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0475EC: subi.w #$1f, d0
pub fn i_0475ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475F0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0475F0: addi.w #$f, d1
pub fn i_0475f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475F4;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0475F4: movem.w d0-d1, $3c0000.l
pub fn i_0475f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x475FC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0475FC: addi.w #$20, d0
pub fn i_0475fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47600;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047600: addq.w #$1, d1
pub fn i_047600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47602;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047602: movem.w d0-d1, $3c0000.l
pub fn i_047602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4760A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $04760A: subi.w #$21, d0
pub fn i_04760a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4760E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04760E: move.w #$7283, d0
pub fn i_04760e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47612;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7283;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047612: move.w d2, d1
pub fn i_047612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47614;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047614: movem.w d0-d1, $3c0000.l
pub fn i_047614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4761C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $04761C: addi.w #$20, d0
pub fn i_04761c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47620;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047620: addq.w #$1, d1
pub fn i_047620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47622;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047622: movem.w d0-d1, $3c0000.l
pub fn i_047622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4762A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $04762A: subi.w #$1f, d0
pub fn i_04762a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4762E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04762E: addi.w #$f, d1
pub fn i_04762e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47632;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047632: movem.w d0-d1, $3c0000.l
pub fn i_047632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4763A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $04763A: addi.w #$20, d0
pub fn i_04763a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4763E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04763E: addq.w #$1, d1
pub fn i_04763e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47640;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047640: movem.w d0-d1, $3c0000.l
pub fn i_047640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47648;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047648: subi.w #$21, d0
pub fn i_047648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4764C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04764C: rts
pub fn i_04764c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4764E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_Divide10_047656  ($047656, 32 B, video/debug_hex_counter_047482.s)
/// $047656: cmpi.w #$63, d0
pub fn i_047656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4765A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x63, Size::Word, false);
    8
}
/// $04765A: bls.w $47662
pub fn i_04765a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4765E;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x47662; return 10; }
    8
}
/// $04765E: move.w #$63, d0
pub fn i_04765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47662;
    cpu.ir = 0x303C;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047662: move.w #$a, d2
pub fn i_047662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47666;
    cpu.ir = 0x343C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047666: clr.w d1
pub fn i_047666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47668;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047668: sub.w d2, d0
pub fn i_047668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4766A;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04766A: bmi.w $47672
pub fn i_04766a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4766E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x47672; return 10; }
    8
}
/// $04766E: addq.w #$1, d1
pub fn i_04766e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47670;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047670: bra.b $47668
pub fn i_047670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47672;
    cpu.ir = 0x60F6;
    cpu.pc = 0x47668;
    10
}
/// $047672: add.w d2, d0
pub fn i_047672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47674;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $047674: rts
pub fn i_047674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47676;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Font_SmallGlyphToTile_0477d4  ($0477D4, 32 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0477D4: move.w d0, d2
pub fn i_0477d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477D6;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0477D6: andi.w #$f0, d2
pub fn i_0477d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477DA;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0477DA: asl.w #$1, d2
pub fn i_0477da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477DC;
    cpu.ir = 0xE342;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0477DC: andi.w #$f, d0
pub fn i_0477dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0477E0: addi.w #$400, d0
pub fn i_0477e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477E4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x400, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0477E4: add.w d2, d0
pub fn i_0477e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477E6;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0477E6: asl.w #$8, d1
pub fn i_0477e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477E8;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0477E8: asl.w #$4, d1
pub fn i_0477e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477EA;
    cpu.ir = 0xE941;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0477EA: or.w d1, d0
pub fn i_0477ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477EC;
    cpu.ir = 0x8041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0477EC: move.w #$1, d1
pub fn i_0477ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0477F0: move.w #$2, d2
pub fn i_0477f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477F4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_0477f4  ($0477F4, 8 B, thunks/jsr_abs_thunks.c)
/// $0477F4: jsr $5da56.l
pub fn i_0477f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x477FA);
    cpu.pc = 0x5DA56;
    16
}
/// $0477FA: rts
pub fn i_0477fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== List_ApplyWithSentinelFF_0477FC  ($0477FC, 38 B, kernel/list_apply_sentinel_0477fc.s)
/// $0477FC: move.w d1, d4
pub fn i_0477fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477FE;
    cpu.ir = 0x3801;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0477FE: movea.l a1, a4
pub fn i_0477fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47800;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $047800: movea.l a2, a5
pub fn i_047800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47802;
    cpu.ir = 0x2A4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[5] = v1;
    4
}
/// $047802: move.b (a5)+, d0
pub fn i_047802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47804;
    cpu.ir = 0x101D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $047804: cmpi.b #$ff, d0
pub fn i_047804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47808;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $047808: beq.w $47820
pub fn i_047808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4780C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x47820; return 10; }
    8
}
/// $04780C: move.w d4, d1
pub fn i_04780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4780E;
    cpu.ir = 0x3204;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04780E: movea.l a4, a1
pub fn i_04780e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47810;
    cpu.ir = 0x224C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $047810: jsr $477d4(pc)
pub fn i_047810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47814;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47814);
    cpu.pc = 0x477D4;
    16
}
/// $047814: move.l a4, d0
pub fn i_047814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47816;
    cpu.ir = 0x200C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $047816: addi.l #$20, d0
pub fn i_047816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4781C;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $04781C: movea.l d0, a4
pub fn i_04781c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4781E;
    cpu.ir = 0x2840;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $04781E: bra.b $47802
pub fn i_04781e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47820;
    cpu.ir = 0x60E2;
    cpu.pc = 0x47802;
    10
}
/// $047820: rts
pub fn i_047820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47822;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Font_BigGlyphToTile_047822  ($047822, 34 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047822: move.w d0, d2
pub fn i_047822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47824;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047824: andi.w #$8, d0
pub fn i_047824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47828;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047828: asl.w #$5, d0
pub fn i_047828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4782A;
    cpu.ir = 0xEB40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04782A: addi.w #$b00, d0
pub fn i_04782a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4782E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xB00, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04782E: andi.w #$77, d2
pub fn i_04782e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47832;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x77) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047832: asl.w #$1, d2
pub fn i_047832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47834;
    cpu.ir = 0xE342;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $047834: add.w d2, d0
pub fn i_047834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47836;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $047836: asl.w #$8, d1
pub fn i_047836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47838;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $047838: asl.w #$4, d1
pub fn i_047838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4783A;
    cpu.ir = 0xE941;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $04783A: or.w d1, d0
pub fn i_04783a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4783C;
    cpu.ir = 0x8041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04783C: move.w #$2, d1
pub fn i_04783c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47840;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047840: move.w #$2, d2
pub fn i_047840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47844;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_047844  ($047844, 8 B, thunks/jsr_abs_thunks.c)
/// $047844: jsr $5da56.l
pub fn i_047844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4784A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4784A);
    cpu.pc = 0x5DA56;
    16
}
/// $04784A: rts
pub fn i_04784a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4784C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== List_ApplyWithSentinelFF_04784C  ($04784C, 38 B, kernel/list_apply_sentinel_04784c.s)
/// $04784C: move.w d1, d4
pub fn i_04784c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4784E;
    cpu.ir = 0x3801;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04784E: movea.l a1, a4
pub fn i_04784e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47850;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $047850: movea.l a2, a5
pub fn i_047850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47852;
    cpu.ir = 0x2A4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[5] = v1;
    4
}
/// $047852: move.b (a5)+, d0
pub fn i_047852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47854;
    cpu.ir = 0x101D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $047854: cmpi.b #$ff, d0
pub fn i_047854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47858;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $047858: beq.w $47870
pub fn i_047858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4785C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x47870; return 10; }
    8
}
/// $04785C: move.w d4, d1
pub fn i_04785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4785E;
    cpu.ir = 0x3204;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04785E: movea.l a4, a1
pub fn i_04785e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47860;
    cpu.ir = 0x224C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $047860: jsr $47822(pc)
pub fn i_047860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47864;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47864);
    cpu.pc = 0x47822;
    16
}
/// $047864: move.l a4, d0
pub fn i_047864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47866;
    cpu.ir = 0x200C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $047866: addi.l #$40, d0
pub fn i_047866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4786C;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $04786C: movea.l d0, a4
pub fn i_04786c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4786E;
    cpu.ir = 0x2840;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $04786E: bra.b $47852
pub fn i_04786e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47870;
    cpu.ir = 0x60E2;
    cpu.pc = 0x47852;
    10
}
/// $047870: rts
pub fn i_047870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47872;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Font_TileWithPal_047872  ($047872, 14 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047872: asl.w #$8, d1
pub fn i_047872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47874;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $047874: asl.w #$4, d1
pub fn i_047874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47876;
    cpu.ir = 0xE941;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $047876: or.w d1, d0
pub fn i_047876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47878;
    cpu.ir = 0x8041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $047878: move.w #$2, d1
pub fn i_047878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4787C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04787C: move.w #$2, d2
pub fn i_04787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47880;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_047880  ($047880, 8 B, thunks/jsr_abs_thunks.c)
/// $047880: jsr $5da56.l
pub fn i_047880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47886;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47886);
    cpu.pc = 0x5DA56;
    16
}
/// $047886: rts
pub fn i_047886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47888;
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
// ===== SpawnStream_ReadNext_04cac4  ($04CAC4, 128 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CAC4: movea.l $1081b2.l, a0
pub fn i_04cac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CACA;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x1081B2);
    cpu.a[0] = v2;
    20
}
/// $04CACA: cmpa.w #$0, a0
pub fn i_04caca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CACE;
    cpu.ir = 0xB0FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, (v1 as u16 as i16 as i32 as u32), Size::Long, false);
    6
}
/// $04CACE: bpl.w $4caec
pub fn i_04cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAD2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4CAEC; return 10; }
    8
}
/// $04CAD2: move.w #$50, d0
pub fn i_04cad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAD6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CAD6: move.w #$1f8, d1
pub fn i_04cad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CADA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CADA: move.b #$0, d2
pub fn i_04cada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CADE;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04CADE: move.w #$10, d3
pub fn i_04cade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE2;
    cpu.ir = 0x363C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CAE2: clr.b d4
pub fn i_04cae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE4;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE4: clr.b d5
pub fn i_04cae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE6;
    cpu.ir = 0x4205;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE6: clr.b d6
pub fn i_04cae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE8;
    cpu.ir = 0x4206;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE8: bra.w $4cb42
pub fn i_04cae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x4CB42;
    10
}
/// $04CAEC: move.w $106f5c.l, d0
pub fn i_04caec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAF2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04CAF2: cmp.w $10(a0), d0
pub fn i_04caf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAF6;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04CAF6: bcs.w $4cb04
pub fn i_04caf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAFA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CB04; return 10; }
    8
}
/// $04CAFA: adda.w #$10, a0
pub fn i_04cafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAFE;
    cpu.ir = 0xD0FC;
    let v1: u32 = 0x10;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $04CAFE: move.l a0, $1081b2.l
pub fn i_04cafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB04;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x1081B2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04CB04: move.w $4(a0), d0
pub fn i_04cb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB08;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB08: tst.b $2(a0)
pub fn i_04cb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB0C;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04CB0C: beq.w $4cb16
pub fn i_04cb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB10;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CB16; return 10; }
    8
}
/// $04CB10: sub.w $106f50.l, d0
pub fn i_04cb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB16;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04CB16: move.w $6(a0), d1
pub fn i_04cb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB1A;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB1A: tst.b $3(a0)
pub fn i_04cb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB1E;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04CB1E: beq.w $4cb28
pub fn i_04cb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CB28; return 10; }
    8
}
/// $04CB22: sub.w $106f54.l, d1
pub fn i_04cb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB28;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04CB28: neg.w d1
pub fn i_04cb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB2A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04CB2A: addi.w #$200, d1
pub fn i_04cb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB2E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04CB2E: move.w $8(a0), d2
pub fn i_04cb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB32;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB32: move.w $a(a0), d3
pub fn i_04cb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB36;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB36: move.b $c(a0), d4
pub fn i_04cb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB3A;
    cpu.ir = 0x1828;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB3A: move.b $d(a0), d5
pub fn i_04cb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB3E;
    cpu.ir = 0x1A28;
    let ad2 = cpu.a[0].wrapping_add(0xD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB3E: move.b $e(a0), d6
pub fn i_04cb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB42;
    cpu.ir = 0x1C28;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB42: rts
pub fn i_04cb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Table_LoadPtrByIdxClamp6_04CB5C  ($04CB5C, 44 B, kernel/table_load_ptr_by_idx_04cb5c.s)
/// $04CB5C: move.l #$ffffffff, $1081b2.l
pub fn i_04cb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB66;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xFFFFFFFF;
    bus.write32(0x1081B2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04CB66: moveq #$0, d0
pub fn i_04cb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB68;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CB68: move.b $106ece.l, d0
pub fn i_04cb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB6E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $04CB6E: cmpi.b #$6, d0
pub fn i_04cb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB72;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $04CB72: bcc.w $4cb86
pub fn i_04cb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CB86; return 10; }
    8
}
/// $04CB76: lea.l $4cb44(pc), a0
pub fn i_04cb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4CB44;
    16
}
/// $04CB7A: add.w d0, d0
pub fn i_04cb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04CB7C: add.w d0, d0
pub fn i_04cb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04CB7E: move.l (a0, d0.w), $1081b2.l
pub fn i_04cb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB86;
    cpu.ir = 0x23F0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(0x1081B2, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    34
}
/// $04CB86: rts
pub fn i_04cb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpawnStream_Dispatch_04cb88  ($04CB88, 40 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CB88: jsr $4cac4(pc)
pub fn i_04cb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CB8C);
    cpu.pc = 0x4CAC4;
    16
}
/// $04CB8C: btst.b #$0, $100001.l
pub fn i_04cb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB94;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $04CB94: beq.w $4cbb6
pub fn i_04cb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CBB6; return 10; }
    8
}
/// $04CB98: swap d0
pub fn i_04cb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB9A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CB9A: move.w d1, d0
pub fn i_04cb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB9C;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CB9C: move.w #$ffff, d1
pub fn i_04cb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CBA0: move.w #$0, d2
pub fn i_04cba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CBA4: moveq #$ff, d3
pub fn i_04cba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA6;
    cpu.ir = 0x76FF;
    cpu.d[3] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBA6: moveq #$ff, d4
pub fn i_04cba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA8;
    cpu.ir = 0x78FF;
    cpu.d[4] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBA8: moveq #$0, d5
pub fn i_04cba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBAA;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBAA: lea.l $243608.l, a0
pub fn i_04cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x243608;
    20
}
// ===== JsrAbsThunk_04cbb0  ($04CBB0, 8 B, thunks/jsr_abs_thunks.c)
/// $04CBB0: jsr $5a9d6.l
pub fn i_04cbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CBB6);
    cpu.pc = 0x5A9D6;
    16
}
/// $04CBB6: rts
pub fn i_04cbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04D70C: movea.l $3c(a6), a1
pub fn i_04d70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D710;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04D710: jsr $2942a.l
pub fn i_04d710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D716);
    cpu.pc = 0x2942A;
    16
}
/// $04D716: move.b #$ff, $32(a6)
pub fn i_04d716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D71C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D71C: move.b #$ff, $33(a6)
pub fn i_04d71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D722;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D722: move.b #$0, $3a(a6)
pub fn i_04d722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D728;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D728: lea.l $4d72e(pc), a1
pub fn i_04d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D72C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D72E;
    16
}
/// $04D72C: move.l a1, (a6)
pub fn i_04d72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D72E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D72E: jsr $2783a.l
pub fn i_04d72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D734);
    cpu.pc = 0x2783A;
    16
}
/// $04D734: jsr $28d70.l
pub fn i_04d734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D73A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D73A);
    cpu.pc = 0x28D70;
    16
}
/// $04D73A: jsr $4fa70(pc)
pub fn i_04d73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D73E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D73E);
    cpu.pc = 0x4FA70;
    16
}
/// $04D73E: bcc.w $4d748
pub fn i_04d73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D742;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D748; return 10; }
    8
}
/// $04D742: jmp $518.l
pub fn i_04d742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D748;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D748: rts
pub fn i_04d748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D74A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Sensor_04f46a  ($04F46A, 16 B, props/fortress_04e580.s)
/// $04F46A: movea.l $3c(a6), a1
pub fn i_04f46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F46E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04F46E: jsr $2942a.l
pub fn i_04f46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F474;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F474);
    cpu.pc = 0x2942A;
    16
}
/// $04F474: move.w #$90, $70(a6)
pub fn i_04f474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F47A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Prop_SensorWait_04f47a  ($04F47A, 60 B, props/fortress_04e580.s)
/// $04F47A: lea.l $4f480(pc), a1
pub fn i_04f47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F47E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F480;
    16
}
/// $04F47E: move.l a1, (a6)
pub fn i_04f47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F480;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F480: jsr $2783a.l
pub fn i_04f480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F486);
    cpu.pc = 0x2783A;
    16
}
/// $04F486: lea.l $295fb0.l, a0
pub fn i_04f486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F48C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x295FB0;
    20
}
/// $04F48C: jsr $5e086.l
pub fn i_04f48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F492;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F492);
    cpu.pc = 0x5E086;
    16
}
/// $04F492: bcs.w $4f4a6
pub fn i_04f492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F496;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4F4A6; return 10; }
    8
}
/// $04F496: jsr $31fc2.l
pub fn i_04f496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F49C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F49C);
    cpu.pc = 0x31FC2;
    16
}
/// $04F49C: beq.w $4f4a6
pub fn i_04f49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4F4A6; return 10; }
    8
}
/// $04F4A0: lea.l $4f4b6(pc), a1
pub fn i_04f4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F4B6;
    16
}
/// $04F4A4: move.l a1, (a6)
pub fn i_04f4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F4A6: jsr $4fa70(pc)
pub fn i_04f4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F4AA);
    cpu.pc = 0x4FA70;
    16
}
/// $04F4AA: bcc.w $4f4b4
pub fn i_04f4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F4B4; return 10; }
    8
}
/// $04F4AE: jmp $518.l
pub fn i_04f4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4B4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F4B4: rts
pub fn i_04f4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SensorTriggered_04f4b6  ($04F4B6, 56 B, props/fortress_04e580.s)
/// $04F4B6: lea.l $2948a6.l, a0
pub fn i_04f4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2948A6;
    20
}
/// $04F4BC: jsr $28cd4.l
pub fn i_04f4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F4C2);
    cpu.pc = 0x28CD4;
    16
}
/// $04F4C2: lea.l $4f4c8(pc), a1
pub fn i_04f4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F4C8;
    16
}
/// $04F4C6: move.l a1, (a6)
pub fn i_04f4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F4C8: jsr $2783a.l
pub fn i_04f4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F4CE);
    cpu.pc = 0x2783A;
    16
}
/// $04F4CE: jsr $28d70.l
pub fn i_04f4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F4D4);
    cpu.pc = 0x28D70;
    16
}
/// $04F4D4: bcc.w $4f4de
pub fn i_04f4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F4DE; return 10; }
    8
}
/// $04F4D8: lea.l $4f47a(pc), a1
pub fn i_04f4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F47A;
    16
}
/// $04F4DC: move.l a1, (a6)
pub fn i_04f4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F4DE: jsr $4fa70(pc)
pub fn i_04f4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4F4E2);
    cpu.pc = 0x4FA70;
    16
}
/// $04F4E2: bcc.w $4f4ec
pub fn i_04f4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F4EC; return 10; }
    8
}
/// $04F4E6: jmp $518.l
pub fn i_04f4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4EC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F4EC: rts
pub fn i_04f4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F4EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SignA_04f70e  ($04F70E, 94 B, props/fortress_04e580.s)
/// $04F70E: move.w #$186, d1
pub fn i_04f70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F712;
    cpu.ir = 0x323C;
    let v1: u32 = 0x186;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04F712: jsr $236e.l
pub fn i_04f712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F718);
    cpu.pc = 0x236E;
    16
}
/// $04F718: lea.l $2949ae.l, a0
pub fn i_04f718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F71E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2949AE;
    20
}
/// $04F71E: jsr $28cd4.l
pub fn i_04f71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F724);
    cpu.pc = 0x28CD4;
    16
}
/// $04F724: move.w #$0, $38(a6)
pub fn i_04f724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F72A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04F72A: jsr $2783a.l
pub fn i_04f72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F730;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F730);
    cpu.pc = 0x2783A;
    16
}
/// $04F730: lea.l $295f6e.l, a1
pub fn i_04f730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F736;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x295F6E;
    20
}
/// $04F736: jsr $43fac.l
pub fn i_04f736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F73C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F73C);
    cpu.pc = 0x43FAC;
    16
}
/// $04F73C: lea.l $4f742(pc), a1
pub fn i_04f73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F740;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F742;
    16
}
/// $04F740: move.l a1, (a6)
pub fn i_04f740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F742;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04F742: jsr $2783a.l
pub fn i_04f742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F748);
    cpu.pc = 0x2783A;
    16
}
/// $04F748: jsr $28d70.l
pub fn i_04f748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F74E);
    cpu.pc = 0x28D70;
    16
}
/// $04F74E: movea.l #$ffffffff, a0
pub fn i_04f74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F754;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04F754: lea.l $29625e.l, a0
pub fn i_04f754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F75A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29625E;
    20
}
/// $04F75A: jsr $5dd5c.l
pub fn i_04f75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4F760);
    cpu.pc = 0x5DD5C;
    16
}
/// $04F760: bcc.w $4f76a
pub fn i_04f760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F764;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4F76A; return 10; }
    8
}
/// $04F764: jmp $518.l
pub fn i_04f764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F76A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04F76A: rts
pub fn i_04f76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F76C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixTile_Set11C2_04fa40  ($04FA40, 16 B, props/fortress_04e580.s)
/// $04FA40: move.w #$11, d1
pub fn i_04fa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA44: move.w #$c2, d2
pub fn i_04fa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA48;
    cpu.ir = 0x343C;
    let v1: u32 = 0xC2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA48: move.w #$ffff, d3
pub fn i_04fa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA4C;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA4C: move.w #$1, d4
pub fn i_04fa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA50;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_04fa50  ($04FA50, 8 B, thunks/jsr_abs_thunks.c)
/// $04FA50: jsr $2c26.l
pub fn i_04fa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FA56);
    cpu.pc = 0x2C26;
    16
}
/// $04FA56: rts
pub fn i_04fa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixTile_Set11C1_04fa58  ($04FA58, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FA58: move.w #$11, d1
pub fn i_04fa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA5C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA5C: move.w #$c1, d2
pub fn i_04fa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA60;
    cpu.ir = 0x343C;
    let v1: u32 = 0xC1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA60: move.w #$ffff, d3
pub fn i_04fa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA64;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04FA64: move.w #$1, d4
pub fn i_04fa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA68;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_04fa68  ($04FA68, 8 B, thunks/jsr_abs_thunks.c)
/// $04FA68: jsr $2c26.l
pub fn i_04fa68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4FA6E);
    cpu.pc = 0x2C26;
    16
}
/// $04FA6E: rts
pub fn i_04fa6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_OffscreenLeftCheck_04fa70  ($04FA70, 14 B, enemies/bosses/allen_oneil_04fa58.s)
/// $04FA70: move.w $70(a6), d0
pub fn i_04fa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA74;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04FA74: neg.w d0
pub fn i_04fa74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA76;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04FA76: cmp.w $22(a6), d0
pub fn i_04fa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA7A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04FA7A: ble.w $4fa84
pub fn i_04fa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA7E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4FA84; return 10; }
    8
}
// ===== SetC_04fa7e  ($04FA7E, 6 B, thunks/ccr_helpers.c)
/// $04FA7E: ori.b #$1, ccr
pub fn i_04fa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA82;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $04FA82: rts
pub fn i_04fa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_04fa84  ($04FA84, 6 B, thunks/ccr_helpers.c)
/// $04FA84: andi.b #$fe, ccr
pub fn i_04fa84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA88;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $04FA88: rts
pub fn i_04fa88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0466A6, i_0466a6),
    (0x0466AC, i_0466ac),
    (0x0466B0, i_0466b0),
    (0x0466B2, i_0466b2),
    (0x0466B4, i_0466b4),
    (0x0466B8, i_0466b8),
    (0x0466BE, i_0466be),
    (0x0466C2, i_0466c2),
    (0x0466C6, i_0466c6),
    (0x0466CC, i_0466cc),
    (0x0466D2, i_0466d2),
    (0x0466D6, i_0466d6),
    (0x0466D8, i_0466d8),
    (0x0466DA, i_0466da),
    (0x0466DE, i_0466de),
    (0x0466E4, i_0466e4),
    (0x0466E8, i_0466e8),
    (0x0466EE, i_0466ee),
    (0x0466F4, i_0466f4),
    (0x0466F6, i_0466f6),
    (0x0466FC, i_0466fc),
    (0x046700, i_046700),
    (0x046704, i_046704),
    (0x04670A, i_04670a),
    (0x046710, i_046710),
    (0x04673A, i_04673a),
    (0x046740, i_046740),
    (0x046746, i_046746),
    (0x04674A, i_04674a),
    (0x04674E, i_04674e),
    (0x046752, i_046752),
    (0x04675A, i_04675a),
    (0x04675C, i_04675c),
    (0x046760, i_046760),
    (0x046768, i_046768),
    (0x04676C, i_04676c),
    (0x04676E, i_04676e),
    (0x046772, i_046772),
    (0x046778, i_046778),
    (0x04677C, i_04677c),
    (0x046784, i_046784),
    (0x04678A, i_04678a),
    (0x04678E, i_04678e),
    (0x046792, i_046792),
    (0x046796, i_046796),
    (0x04679A, i_04679a),
    (0x04679E, i_04679e),
    (0x0467A2, i_0467a2),
    (0x0467A6, i_0467a6),
    (0x0467AC, i_0467ac),
    (0x0467B0, i_0467b0),
    (0x0467B4, i_0467b4),
    (0x0467BA, i_0467ba),
    (0x0467BE, i_0467be),
    (0x0467C2, i_0467c2),
    (0x0467C6, i_0467c6),
    (0x0467CC, i_0467cc),
    (0x0467D0, i_0467d0),
    (0x0467D4, i_0467d4),
    (0x0467D8, i_0467d8),
    (0x0467E0, i_0467e0),
    (0x0467E2, i_0467e2),
    (0x0467E6, i_0467e6),
    (0x0467EE, i_0467ee),
    (0x0467F4, i_0467f4),
    (0x0467F6, i_0467f6),
    (0x0467FC, i_0467fc),
    (0x046806, i_046806),
    (0x04680C, i_04680c),
    (0x046812, i_046812),
    (0x046816, i_046816),
    (0x04681C, i_04681c),
    (0x046820, i_046820),
    (0x046826, i_046826),
    (0x04682C, i_04682c),
    (0x046832, i_046832),
    (0x046838, i_046838),
    (0x04683E, i_04683e),
    (0x046842, i_046842),
    (0x046848, i_046848),
    (0x04684E, i_04684e),
    (0x046854, i_046854),
    (0x046858, i_046858),
    (0x04685E, i_04685e),
    (0x046862, i_046862),
    (0x046868, i_046868),
    (0x04686C, i_04686c),
    (0x04686E, i_04686e),
    (0x046872, i_046872),
    (0x046878, i_046878),
    (0x04687C, i_04687c),
    (0x046882, i_046882),
    (0x046886, i_046886),
    (0x04688A, i_04688a),
    (0x04688E, i_04688e),
    (0x046892, i_046892),
    (0x046898, i_046898),
    (0x04689C, i_04689c),
    (0x0468A0, i_0468a0),
    (0x0468A6, i_0468a6),
    (0x0468AA, i_0468aa),
    (0x0468AE, i_0468ae),
    (0x0468B4, i_0468b4),
    (0x0468B8, i_0468b8),
    (0x0468BC, i_0468bc),
    (0x0468C0, i_0468c0),
    (0x0468C4, i_0468c4),
    (0x0468C8, i_0468c8),
    (0x0468CE, i_0468ce),
    (0x0468D4, i_0468d4),
    (0x0468DA, i_0468da),
    (0x0468DE, i_0468de),
    (0x0468E0, i_0468e0),
    (0x0468E4, i_0468e4),
    (0x0468EA, i_0468ea),
    (0x0468F0, i_0468f0),
    (0x0468F4, i_0468f4),
    (0x0468F8, i_0468f8),
    (0x0468FE, i_0468fe),
    (0x046902, i_046902),
    (0x046908, i_046908),
    (0x04690C, i_04690c),
    (0x046910, i_046910),
    (0x046914, i_046914),
    (0x04691A, i_04691a),
    (0x04691E, i_04691e),
    (0x046922, i_046922),
    (0x046926, i_046926),
    (0x04692C, i_04692c),
    (0x04692E, i_04692e),
    (0x046932, i_046932),
    (0x046938, i_046938),
    (0x04693C, i_04693c),
    (0x046942, i_046942),
    (0x04694A, i_04694a),
    (0x046952, i_046952),
    (0x046956, i_046956),
    (0x04695A, i_04695a),
    (0x04695C, i_04695c),
    (0x046962, i_046962),
    (0x046966, i_046966),
    (0x04696A, i_04696a),
    (0x04696C, i_04696c),
    (0x046972, i_046972),
    (0x046976, i_046976),
    (0x04697A, i_04697a),
    (0x04697E, i_04697e),
    (0x046984, i_046984),
    (0x046988, i_046988),
    (0x04698C, i_04698c),
    (0x046994, i_046994),
    (0x046998, i_046998),
    (0x04699C, i_04699c),
    (0x04699E, i_04699e),
    (0x0469A4, i_0469a4),
    (0x0469A8, i_0469a8),
    (0x0469AC, i_0469ac),
    (0x0469AE, i_0469ae),
    (0x0469B4, i_0469b4),
    (0x0469B8, i_0469b8),
    (0x0469BC, i_0469bc),
    (0x0469C0, i_0469c0),
    (0x0469C6, i_0469c6),
    (0x0469CA, i_0469ca),
    (0x0469CE, i_0469ce),
    (0x0469D2, i_0469d2),
    (0x0469D6, i_0469d6),
    (0x0469DC, i_0469dc),
    (0x0469E0, i_0469e0),
    (0x0469E2, i_0469e2),
    (0x0469E6, i_0469e6),
    (0x0469EA, i_0469ea),
    (0x0469EE, i_0469ee),
    (0x0469F0, i_0469f0),
    (0x0469F6, i_0469f6),
    (0x046AC6, i_046ac6),
    (0x046ACA, i_046aca),
    (0x046ACE, i_046ace),
    (0x046AD2, i_046ad2),
    (0x046AD6, i_046ad6),
    (0x046ADC, i_046adc),
    (0x046AE0, i_046ae0),
    (0x046AE4, i_046ae4),
    (0x046AE8, i_046ae8),
    (0x046AEC, i_046aec),
    (0x046AF2, i_046af2),
    (0x046AF6, i_046af6),
    (0x046AFA, i_046afa),
    (0x046AFE, i_046afe),
    (0x046B02, i_046b02),
    (0x046B08, i_046b08),
    (0x046B0C, i_046b0c),
    (0x046B10, i_046b10),
    (0x046B14, i_046b14),
    (0x046B18, i_046b18),
    (0x046B1E, i_046b1e),
    (0x04707E, i_04707e),
    (0x047080, i_047080),
    (0x047086, i_047086),
    (0x04708A, i_04708a),
    (0x04708C, i_04708c),
    (0x047092, i_047092),
    (0x047096, i_047096),
    (0x04709C, i_04709c),
    (0x04709E, i_04709e),
    (0x0470A0, i_0470a0),
    (0x0470A6, i_0470a6),
    (0x0470AA, i_0470aa),
    (0x0470AC, i_0470ac),
    (0x0470B2, i_0470b2),
    (0x0470B6, i_0470b6),
    (0x0470BC, i_0470bc),
    (0x047482, i_047482),
    (0x04748A, i_04748a),
    (0x04748E, i_04748e),
    (0x047490, i_047490),
    (0x047498, i_047498),
    (0x04749C, i_04749c),
    (0x0474A0, i_0474a0),
    (0x0474A4, i_0474a4),
    (0x0474AC, i_0474ac),
    (0x0474B0, i_0474b0),
    (0x0474B2, i_0474b2),
    (0x0474BA, i_0474ba),
    (0x0474BE, i_0474be),
    (0x0474C2, i_0474c2),
    (0x0474CA, i_0474ca),
    (0x0474CE, i_0474ce),
    (0x0474D0, i_0474d0),
    (0x0474D8, i_0474d8),
    (0x0474DC, i_0474dc),
    (0x0474E0, i_0474e0),
    (0x0474E4, i_0474e4),
    (0x0474EC, i_0474ec),
    (0x0474F0, i_0474f0),
    (0x0474F2, i_0474f2),
    (0x0474FA, i_0474fa),
    (0x0474FE, i_0474fe),
    (0x047502, i_047502),
    (0x04750A, i_04750a),
    (0x04750E, i_04750e),
    (0x047510, i_047510),
    (0x047518, i_047518),
    (0x04751C, i_04751c),
    (0x04751E, i_04751e),
    (0x047522, i_047522),
    (0x047526, i_047526),
    (0x04752A, i_04752a),
    (0x04752E, i_04752e),
    (0x047536, i_047536),
    (0x04753A, i_04753a),
    (0x04753C, i_04753c),
    (0x047544, i_047544),
    (0x047548, i_047548),
    (0x04754C, i_04754c),
    (0x047554, i_047554),
    (0x047558, i_047558),
    (0x04755A, i_04755a),
    (0x047562, i_047562),
    (0x047566, i_047566),
    (0x04756A, i_04756a),
    (0x04756E, i_04756e),
    (0x047576, i_047576),
    (0x04757A, i_04757a),
    (0x04757C, i_04757c),
    (0x047584, i_047584),
    (0x047588, i_047588),
    (0x04758C, i_04758c),
    (0x047594, i_047594),
    (0x047598, i_047598),
    (0x04759A, i_04759a),
    (0x0475A2, i_0475a2),
    (0x0475A6, i_0475a6),
    (0x0475A8, i_0475a8),
    (0x0475AE, i_0475ae),
    (0x0475B2, i_0475b2),
    (0x0475B6, i_0475b6),
    (0x0475BA, i_0475ba),
    (0x0475BE, i_0475be),
    (0x0475C4, i_0475c4),
    (0x0475C6, i_0475c6),
    (0x0475CA, i_0475ca),
    (0x0475CC, i_0475cc),
    (0x0475D0, i_0475d0),
    (0x0475D4, i_0475d4),
    (0x0475D6, i_0475d6),
    (0x0475DE, i_0475de),
    (0x0475E2, i_0475e2),
    (0x0475E4, i_0475e4),
    (0x0475EC, i_0475ec),
    (0x0475F0, i_0475f0),
    (0x0475F4, i_0475f4),
    (0x0475FC, i_0475fc),
    (0x047600, i_047600),
    (0x047602, i_047602),
    (0x04760A, i_04760a),
    (0x04760E, i_04760e),
    (0x047612, i_047612),
    (0x047614, i_047614),
    (0x04761C, i_04761c),
    (0x047620, i_047620),
    (0x047622, i_047622),
    (0x04762A, i_04762a),
    (0x04762E, i_04762e),
    (0x047632, i_047632),
    (0x04763A, i_04763a),
    (0x04763E, i_04763e),
    (0x047640, i_047640),
    (0x047648, i_047648),
    (0x04764C, i_04764c),
    (0x047656, i_047656),
    (0x04765A, i_04765a),
    (0x04765E, i_04765e),
    (0x047662, i_047662),
    (0x047666, i_047666),
    (0x047668, i_047668),
    (0x04766A, i_04766a),
    (0x04766E, i_04766e),
    (0x047670, i_047670),
    (0x047672, i_047672),
    (0x047674, i_047674),
    (0x0477D4, i_0477d4),
    (0x0477D6, i_0477d6),
    (0x0477DA, i_0477da),
    (0x0477DC, i_0477dc),
    (0x0477E0, i_0477e0),
    (0x0477E4, i_0477e4),
    (0x0477E6, i_0477e6),
    (0x0477E8, i_0477e8),
    (0x0477EA, i_0477ea),
    (0x0477EC, i_0477ec),
    (0x0477F0, i_0477f0),
    (0x0477F4, i_0477f4),
    (0x0477FA, i_0477fa),
    (0x0477FC, i_0477fc),
    (0x0477FE, i_0477fe),
    (0x047800, i_047800),
    (0x047802, i_047802),
    (0x047804, i_047804),
    (0x047808, i_047808),
    (0x04780C, i_04780c),
    (0x04780E, i_04780e),
    (0x047810, i_047810),
    (0x047814, i_047814),
    (0x047816, i_047816),
    (0x04781C, i_04781c),
    (0x04781E, i_04781e),
    (0x047820, i_047820),
    (0x047822, i_047822),
    (0x047824, i_047824),
    (0x047828, i_047828),
    (0x04782A, i_04782a),
    (0x04782E, i_04782e),
    (0x047832, i_047832),
    (0x047834, i_047834),
    (0x047836, i_047836),
    (0x047838, i_047838),
    (0x04783A, i_04783a),
    (0x04783C, i_04783c),
    (0x047840, i_047840),
    (0x047844, i_047844),
    (0x04784A, i_04784a),
    (0x04784C, i_04784c),
    (0x04784E, i_04784e),
    (0x047850, i_047850),
    (0x047852, i_047852),
    (0x047854, i_047854),
    (0x047858, i_047858),
    (0x04785C, i_04785c),
    (0x04785E, i_04785e),
    (0x047860, i_047860),
    (0x047864, i_047864),
    (0x047866, i_047866),
    (0x04786C, i_04786c),
    (0x04786E, i_04786e),
    (0x047870, i_047870),
    (0x047872, i_047872),
    (0x047874, i_047874),
    (0x047876, i_047876),
    (0x047878, i_047878),
    (0x04787C, i_04787c),
    (0x047880, i_047880),
    (0x047886, i_047886),
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
    (0x04A034, i_04a034),
    (0x04A03A, i_04a03a),
    (0x04A03E, i_04a03e),
    (0x04A040, i_04a040),
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
    (0x04A0F4, i_04a0f4),
    (0x04A0F8, i_04a0f8),
    (0x04A0FE, i_04a0fe),
    (0x04A102, i_04a102),
    (0x04A120, i_04a120),
    (0x04A124, i_04a124),
    (0x04A132, i_04a132),
    (0x04A136, i_04a136),
    (0x04A13C, i_04a13c),
    (0x04A140, i_04a140),
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
    (0x04CAC4, i_04cac4),
    (0x04CACA, i_04caca),
    (0x04CACE, i_04cace),
    (0x04CAD2, i_04cad2),
    (0x04CAD6, i_04cad6),
    (0x04CADA, i_04cada),
    (0x04CADE, i_04cade),
    (0x04CAE2, i_04cae2),
    (0x04CAE4, i_04cae4),
    (0x04CAE6, i_04cae6),
    (0x04CAE8, i_04cae8),
    (0x04CAEC, i_04caec),
    (0x04CAF2, i_04caf2),
    (0x04CAF6, i_04caf6),
    (0x04CAFA, i_04cafa),
    (0x04CAFE, i_04cafe),
    (0x04CB04, i_04cb04),
    (0x04CB08, i_04cb08),
    (0x04CB0C, i_04cb0c),
    (0x04CB10, i_04cb10),
    (0x04CB16, i_04cb16),
    (0x04CB1A, i_04cb1a),
    (0x04CB1E, i_04cb1e),
    (0x04CB22, i_04cb22),
    (0x04CB28, i_04cb28),
    (0x04CB2A, i_04cb2a),
    (0x04CB2E, i_04cb2e),
    (0x04CB32, i_04cb32),
    (0x04CB36, i_04cb36),
    (0x04CB3A, i_04cb3a),
    (0x04CB3E, i_04cb3e),
    (0x04CB42, i_04cb42),
    (0x04CB5C, i_04cb5c),
    (0x04CB66, i_04cb66),
    (0x04CB68, i_04cb68),
    (0x04CB6E, i_04cb6e),
    (0x04CB72, i_04cb72),
    (0x04CB76, i_04cb76),
    (0x04CB7A, i_04cb7a),
    (0x04CB7C, i_04cb7c),
    (0x04CB7E, i_04cb7e),
    (0x04CB86, i_04cb86),
    (0x04CB88, i_04cb88),
    (0x04CB8C, i_04cb8c),
    (0x04CB94, i_04cb94),
    (0x04CB98, i_04cb98),
    (0x04CB9A, i_04cb9a),
    (0x04CB9C, i_04cb9c),
    (0x04CBA0, i_04cba0),
    (0x04CBA4, i_04cba4),
    (0x04CBA6, i_04cba6),
    (0x04CBA8, i_04cba8),
    (0x04CBAA, i_04cbaa),
    (0x04CBB0, i_04cbb0),
    (0x04CBB6, i_04cbb6),
    (0x04D70C, i_04d70c),
    (0x04D710, i_04d710),
    (0x04D716, i_04d716),
    (0x04D71C, i_04d71c),
    (0x04D722, i_04d722),
    (0x04D728, i_04d728),
    (0x04D72C, i_04d72c),
    (0x04D72E, i_04d72e),
    (0x04D734, i_04d734),
    (0x04D73A, i_04d73a),
    (0x04D73E, i_04d73e),
    (0x04D742, i_04d742),
    (0x04D748, i_04d748),
    (0x04F46A, i_04f46a),
    (0x04F46E, i_04f46e),
    (0x04F474, i_04f474),
    (0x04F47A, i_04f47a),
    (0x04F47E, i_04f47e),
    (0x04F480, i_04f480),
    (0x04F486, i_04f486),
    (0x04F48C, i_04f48c),
    (0x04F492, i_04f492),
    (0x04F496, i_04f496),
    (0x04F49C, i_04f49c),
    (0x04F4A0, i_04f4a0),
    (0x04F4A4, i_04f4a4),
    (0x04F4A6, i_04f4a6),
    (0x04F4AA, i_04f4aa),
    (0x04F4AE, i_04f4ae),
    (0x04F4B4, i_04f4b4),
    (0x04F4B6, i_04f4b6),
    (0x04F4BC, i_04f4bc),
    (0x04F4C2, i_04f4c2),
    (0x04F4C6, i_04f4c6),
    (0x04F4C8, i_04f4c8),
    (0x04F4CE, i_04f4ce),
    (0x04F4D4, i_04f4d4),
    (0x04F4D8, i_04f4d8),
    (0x04F4DC, i_04f4dc),
    (0x04F4DE, i_04f4de),
    (0x04F4E2, i_04f4e2),
    (0x04F4E6, i_04f4e6),
    (0x04F4EC, i_04f4ec),
    (0x04F70E, i_04f70e),
    (0x04F712, i_04f712),
    (0x04F718, i_04f718),
    (0x04F71E, i_04f71e),
    (0x04F724, i_04f724),
    (0x04F72A, i_04f72a),
    (0x04F730, i_04f730),
    (0x04F736, i_04f736),
    (0x04F73C, i_04f73c),
    (0x04F740, i_04f740),
    (0x04F742, i_04f742),
    (0x04F748, i_04f748),
    (0x04F74E, i_04f74e),
    (0x04F754, i_04f754),
    (0x04F75A, i_04f75a),
    (0x04F760, i_04f760),
    (0x04F764, i_04f764),
    (0x04F76A, i_04f76a),
    (0x04FA40, i_04fa40),
    (0x04FA44, i_04fa44),
    (0x04FA48, i_04fa48),
    (0x04FA4C, i_04fa4c),
    (0x04FA50, i_04fa50),
    (0x04FA56, i_04fa56),
    (0x04FA58, i_04fa58),
    (0x04FA5C, i_04fa5c),
    (0x04FA60, i_04fa60),
    (0x04FA64, i_04fa64),
    (0x04FA68, i_04fa68),
    (0x04FA6E, i_04fa6e),
    (0x04FA70, i_04fa70),
    (0x04FA74, i_04fa74),
    (0x04FA76, i_04fa76),
    (0x04FA7A, i_04fa7a),
    (0x04FA7E, i_04fa7e),
    (0x04FA82, i_04fa82),
    (0x04FA84, i_04fa84),
    (0x04FA88, i_04fa88),
];
