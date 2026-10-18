// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $047112: move.w #$1e, $70(a6)
pub fn i_047112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47118;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047118: lea.l $47124(pc), a1
pub fn i_047118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4711C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47124;
    16
}
/// $04711C: move.l a1, (a6)
pub fn i_04711c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4711E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_04711e  ($04711E, 6 B, thunks/jsr_pc_thunks.c)
/// $04711E: jsr $4707e(pc)
pub fn i_04711e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47122;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47122);
    cpu.pc = 0x4707E;
    16
}
/// $047122: rts
pub fn i_047122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47124;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionStart_Pause_047124  ($047124, 26 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047124: subq.w #$1, $70(a6)
pub fn i_047124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47128;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047128: cmpi.w #$0, $70(a6)
pub fn i_047128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4712E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04712E: bgt.w $47144
pub fn i_04712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47132;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x47144; return 10; }
    8
}
/// $047132: move.w #$a, $70(a6)
pub fn i_047132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47138;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047138: move.w #$2, $5c(a6)
pub fn i_047138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4713E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_04713e  ($04713E, 8 B, kernel/task_handlers.c)
/// $04713E: lea.l $47146(pc), a1
pub fn i_04713e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47142;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47146;
    16
}
/// $047142: move.l a1, (a6)
pub fn i_047142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47144;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047144: rts
pub fn i_047144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47146;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionStart_Clear_047146  ($047146, 60 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047146: move.w $22(a6), d2
pub fn i_047146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4714A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04714A: movea.w #$7000, a1
pub fn i_04714a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4714E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $04714E: asl.w #$5, d2
pub fn i_04714e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47150;
    cpu.ir = 0xEB42;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $047150: adda.w d2, a1
pub fn i_047150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47152;
    cpu.ir = 0xD2C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $047152: adda.w $24(a6), a1
pub fn i_047152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47156;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $047156: move.w #$2320, d0
pub fn i_047156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4715A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04715A: move.w #$12, d1
pub fn i_04715a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4715E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04715E: move.w #$4, d2
pub fn i_04715e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47162;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047162: jsr $5da9c.l
pub fn i_047162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47168;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47168);
    cpu.pc = 0x5DA9C;
    16
}
/// $047168: move.w #$a, $70(a6)
pub fn i_047168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4716E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04716E: lea.l $47174(pc), a1
pub fn i_04716e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47172;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47174;
    16
}
/// $047172: move.l a1, (a6)
pub fn i_047172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47174;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047174: subq.w #$1, $70(a6)
pub fn i_047174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47178;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047178: cmpi.w #$0, $70(a6)
pub fn i_047178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4717E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04717E: bgt.w $47188
pub fn i_04717e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47182;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x47188; return 10; }
    8
}
// ===== SetTaskHandler_047182  ($047182, 8 B, kernel/task_handlers.c)
/// $047182: lea.l $4718a(pc), a1
pub fn i_047182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47186;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4718A;
    16
}
/// $047186: move.l a1, (a6)
pub fn i_047186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47188;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047188: rts
pub fn i_047188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4718A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionStart_Redraw_04718a  ($04718A, 72 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04718A: move.w $22(a6), d2
pub fn i_04718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4718E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04718E: move.w $24(a6), d3
pub fn i_04718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47192;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047192: move.w $14(a6), d4
pub fn i_047192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47196;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047196: jsr $46d0e(pc)
pub fn i_047196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4719A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4719A);
    cpu.pc = 0x46D0E;
    16
}
/// $04719A: move.w #$a, $70(a6)
pub fn i_04719a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0471A0: lea.l $471a6(pc), a1
pub fn i_0471a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x471A6;
    16
}
/// $0471A4: move.l a1, (a6)
pub fn i_0471a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0471A6: subq.w #$1, $70(a6)
pub fn i_0471a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471AA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0471AA: cmpi.w #$0, $70(a6)
pub fn i_0471aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0471B0: bgt.w $471d8
pub fn i_0471b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x471D8; return 10; }
    8
}
/// $0471B4: subq.w #$1, $5c(a6)
pub fn i_0471b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471B8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0471B8: cmpi.w #$0, $5c(a6)
pub fn i_0471b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471BE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0471BE: bgt.w $471d2
pub fn i_0471be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471C2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x471D2; return 10; }
    8
}
/// $0471C2: move.b #$1, $20(a6)
pub fn i_0471c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471C8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0471C8: lea.l $471da(pc), a1
pub fn i_0471c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x471DA;
    16
}
/// $0471CC: move.l a1, (a6)
pub fn i_0471cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0471CE: bra.w $471d8
pub fn i_0471ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x471D8;
    10
}
// ===== SetTaskHandler_0471d2  ($0471D2, 8 B, kernel/task_handlers.c)
/// $0471D2: lea.l $47146(pc), a1
pub fn i_0471d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47146;
    16
}
/// $0471D6: move.l a1, (a6)
pub fn i_0471d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0471D8: rts
pub fn i_0471d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionStart_ScrollOut_0471da  ($0471DA, 50 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0471DA: move.w $28(a6), d0
pub fn i_0471da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0471DE: add.w d0, $22(a6)
pub fn i_0471de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471E2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0471E2: move.w $22(a6), d2
pub fn i_0471e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471E6;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0471E6: move.w $24(a6), d3
pub fn i_0471e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471EA;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0471EA: move.w $14(a6), d4
pub fn i_0471ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471EE;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0471EE: jsr $46d0e(pc)
pub fn i_0471ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x471F2);
    cpu.pc = 0x46D0E;
    16
}
/// $0471F2: cmpi.w #$ffee, $22(a6)
pub fn i_0471f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFEE, Size::Word, false);
    16
}
/// $0471F8: bgt.w $4720a
pub fn i_0471f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x471FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4720A; return 10; }
    8
}
/// $0471FC: movea.l $c(a6), a1
pub fn i_0471fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47200;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $047200: clr.b $21(a1)
pub fn i_047200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47204;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $047204: jmp $518.l
pub fn i_047204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4720A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04720A: rts
pub fn i_04720a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4720C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionComplete_Task_04720c  ($04720C, 100 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04720C: move.w #$20, d0
pub fn i_04720c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47210;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047210: jsr $2352.l
pub fn i_047210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47216;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47216);
    cpu.pc = 0x2352;
    16
}
/// $047216: clr.w d0
pub fn i_047216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47218;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047218: move.w d0, $14(a6)
pub fn i_047218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4721C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04721C: lea.l $46fd6(pc), a1
pub fn i_04721c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47220;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46FD6;
    16
}
/// $047220: jsr $4ae.l
pub fn i_047220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47226);
    cpu.pc = 0x4AE;
    16
}
/// $047226: move.w $14(a6), $14(a0)
pub fn i_047226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4722C;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04722C: clr.b $20(a6)
pub fn i_04722c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47230;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $047230: move.w #$26, $22(a6)
pub fn i_047230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47236;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047236: move.w #$f, $24(a6)
pub fn i_047236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4723C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04723C: move.w #$fffe, $28(a6)
pub fn i_04723c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47242;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047242: lea.l $47248(pc), a1
pub fn i_047242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47246;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47248;
    16
}
/// $047246: move.l a1, (a6)
pub fn i_047246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47248;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047248: move.w $28(a6), d0
pub fn i_047248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4724C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04724C: add.w d0, $22(a6)
pub fn i_04724c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47250;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047250: move.w $22(a6), d2
pub fn i_047250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47254;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047254: move.w $24(a6), d3
pub fn i_047254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47258;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047258: move.w $14(a6), d4
pub fn i_047258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4725C;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04725C: jsr $46e72(pc)
pub fn i_04725c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47260;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47260);
    cpu.pc = 0x46E72;
    16
}
/// $047260: cmpi.w #$6, $22(a6)
pub fn i_047260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47266;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Word, false);
    16
}
/// $047266: bgt.w $47276
pub fn i_047266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4726A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x47276; return 10; }
    8
}
/// $04726A: move.w #$5a, $70(a6)
pub fn i_04726a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47270;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_047270  ($047270, 8 B, kernel/task_handlers.c)
/// $047270: lea.l $47278(pc), a1
pub fn i_047270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47278;
    16
}
/// $047274: move.l a1, (a6)
pub fn i_047274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47276;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047276: rts
pub fn i_047276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47278;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionComplete_Pause_047278  ($047278, 14 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047278: subq.w #$1, $70(a6)
pub fn i_047278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4727C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04727C: cmpi.w #$0, $70(a6)
pub fn i_04727c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47282;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $047282: bgt.w $4728c
pub fn i_047282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47286;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4728C; return 10; }
    8
}
// ===== SetTaskHandler_047286  ($047286, 8 B, kernel/task_handlers.c)
/// $047286: lea.l $4731c(pc), a1
pub fn i_047286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4728A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4731C;
    16
}
/// $04728A: move.l a1, (a6)
pub fn i_04728a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4728C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04728C: rts
pub fn i_04728c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4728E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionComplete_Clear_04728e  ($04728E, 60 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04728E: move.w $22(a6), d2
pub fn i_04728e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47292;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047292: movea.w #$7000, a1
pub fn i_047292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47296;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $047296: asl.w #$5, d2
pub fn i_047296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47298;
    cpu.ir = 0xEB42;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $047298: adda.w d2, a1
pub fn i_047298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4729A;
    cpu.ir = 0xD2C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $04729A: adda.w $24(a6), a1
pub fn i_04729a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4729E;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $04729E: move.w #$2320, d0
pub fn i_04729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472A2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0472A2: move.w #$1c, d1
pub fn i_0472a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0472A6: move.w #$4, d2
pub fn i_0472a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472AA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0472AA: jsr $5da9c.l
pub fn i_0472aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x472B0);
    cpu.pc = 0x5DA9C;
    16
}
/// $0472B0: move.w #$a, $70(a6)
pub fn i_0472b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0472B6: lea.l $472bc(pc), a1
pub fn i_0472b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x472BC;
    16
}
/// $0472BA: move.l a1, (a6)
pub fn i_0472ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0472BC: subq.w #$1, $70(a6)
pub fn i_0472bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472C0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0472C0: cmpi.w #$0, $70(a6)
pub fn i_0472c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472C6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0472C6: bgt.w $472d0
pub fn i_0472c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472CA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x472D0; return 10; }
    8
}
// ===== SetTaskHandler_0472ca  ($0472CA, 8 B, kernel/task_handlers.c)
/// $0472CA: lea.l $472d2(pc), a1
pub fn i_0472ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x472D2;
    16
}
/// $0472CE: move.l a1, (a6)
pub fn i_0472ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0472D0: rts
pub fn i_0472d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionComplete_Redraw_0472d2  ($0472D2, 66 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0472D2: move.w $22(a6), d2
pub fn i_0472d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472D6;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0472D6: move.w $24(a6), d3
pub fn i_0472d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472DA;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0472DA: move.w $14(a6), d4
pub fn i_0472da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472DE;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0472DE: jsr $46e72(pc)
pub fn i_0472de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x472E2);
    cpu.pc = 0x46E72;
    16
}
/// $0472E2: move.w #$a, $70(a6)
pub fn i_0472e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0472E8: lea.l $472ee(pc), a1
pub fn i_0472e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x472EE;
    16
}
/// $0472EC: move.l a1, (a6)
pub fn i_0472ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0472EE: subq.w #$1, $70(a6)
pub fn i_0472ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472F2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0472F2: cmpi.w #$0, $70(a6)
pub fn i_0472f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0472F8: bgt.w $4731a
pub fn i_0472f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x472FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4731A; return 10; }
    8
}
/// $0472FC: subq.w #$1, $5c(a6)
pub fn i_0472fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47300;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047300: cmpi.w #$0, $5c(a6)
pub fn i_047300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47306;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $047306: bgt.w $47314
pub fn i_047306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4730A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x47314; return 10; }
    8
}
/// $04730A: lea.l $4731c(pc), a1
pub fn i_04730a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4730E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4731C;
    16
}
/// $04730E: move.l a1, (a6)
pub fn i_04730e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47310;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047310: bra.w $4731a
pub fn i_047310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47314;
    cpu.ir = 0x6000;
    cpu.pc = 0x4731A;
    10
}
// ===== SetTaskHandler_047314  ($047314, 8 B, kernel/task_handlers.c)
/// $047314: lea.l $4728e(pc), a1
pub fn i_047314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47318;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4728E;
    16
}
/// $047318: move.l a1, (a6)
pub fn i_047318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4731A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04731A: rts
pub fn i_04731a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4731C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionComplete_ScrollOut_04731c  ($04731C, 62 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04731C: move.b #$1, $20(a6)
pub fn i_04731c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47322;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047322: lea.l $47328(pc), a1
pub fn i_047322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47326;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47328;
    16
}
/// $047326: move.l a1, (a6)
pub fn i_047326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47328;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047328: move.w $28(a6), d0
pub fn i_047328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4732C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04732C: add.w d0, $22(a6)
pub fn i_04732c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47330;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047330: move.w $22(a6), d2
pub fn i_047330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47334;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047334: move.w $24(a6), d3
pub fn i_047334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47338;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047338: move.w $14(a6), d4
pub fn i_047338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4733C;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04733C: jsr $46e72(pc)
pub fn i_04733c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47340;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47340);
    cpu.pc = 0x46E72;
    16
}
/// $047340: cmpi.w #$ffe4, $22(a6)
pub fn i_047340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47346;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE4, Size::Word, false);
    16
}
/// $047346: bgt.w $47360
pub fn i_047346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4734A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x47360; return 10; }
    8
}
/// $04734A: move.w #$3, d0
pub fn i_04734a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4734E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04734E: jsr $5239e.l
pub fn i_04734e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47354;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47354);
    cpu.pc = 0x5239E;
    16
}
/// $047354: move.w #$32, $70(a6)
pub fn i_047354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4735A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_04735a  ($04735A, 8 B, kernel/task_handlers.c)
/// $04735A: lea.l $47362(pc), a1
pub fn i_04735a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4735E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47362;
    16
}
/// $04735E: move.l a1, (a6)
pub fn i_04735e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47360;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047360: rts
pub fn i_047360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47362;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionComplete_Finish_047362  ($047362, 28 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047362: subq.w #$1, $70(a6)
pub fn i_047362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47366;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047366: cmpi.w #$0, $70(a6)
pub fn i_047366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4736C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04736C: bgt.w $4737c
pub fn i_04736c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47370;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4737C; return 10; }
    8
}
/// $047370: clr.b $106ed2.l
pub fn i_047370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47376;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $047376: jmp $518.l
pub fn i_047376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4737C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04737C: rts
pub fn i_04737c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4737E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BigText_Typewriter_04737e  ($04737E, 48 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04737E: move.w $30(a6), $5c(a6)
pub fn i_04737e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47384;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $047384: lea.l $4738a(pc), a1
pub fn i_047384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47388;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4738A;
    16
}
/// $047388: move.l a1, (a6)
pub fn i_047388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4738A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04738A: cmpi.w #$0, $30(a6)
pub fn i_04738a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47390;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $047390: ble.w $4739a
pub fn i_047390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47394;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4739A; return 10; }
    8
}
/// $047394: subq.w #$1, $30(a6)
pub fn i_047394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47398;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047398: rts
pub fn i_047398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4739A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04739A: move.w $5c(a6), $30(a6)
pub fn i_04739a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473A0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0473A0: movea.l $3c(a6), a1
pub fn i_0473a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473A4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0473A4: move.b (a1), d0
pub fn i_0473a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473A6;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0473A6: cmpi.b #$ff, d0
pub fn i_0473a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473AA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0473AA: bne.w $473bc
pub fn i_0473aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x473BC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0473ae  ($0473AE, 14 B, kernel/jsr5b6_jmp518.c)
/// $0473AE: jsr $5b6.l
pub fn i_0473ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x473B4);
    cpu.pc = 0x5B6;
    16
}
/// $0473B4: jmp $518.l
pub fn i_0473b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473BA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== BigText_Typewriter_CheckVramLo_0473bc  ($0473BC, 12 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0473BC: movea.w $22(a6), a1
pub fn i_0473bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473C0;
    cpu.ir = 0x326E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = v3 as i16 as i32 as u32;
    12
}
/// $0473C0: cmpa.w #$7000, a1
pub fn i_0473c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473C4;
    cpu.ir = 0xB2FC;
    let v1: u32 = 0x7000;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, (v1 as u16 as i16 as i32 as u32), Size::Long, false);
    6
}
/// $0473C4: bcc.w $473d6
pub fn i_0473c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x473D6; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0473c8  ($0473C8, 14 B, kernel/jsr5b6_jmp518.c)
/// $0473C8: jsr $5b6.l
pub fn i_0473c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x473CE);
    cpu.pc = 0x5B6;
    16
}
/// $0473CE: jmp $518.l
pub fn i_0473ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473D4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== BigText_Typewriter_CheckVramHi_0473d6  ($0473D6, 8 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0473D6: cmpa.w #$74ff, a1
pub fn i_0473d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473DA;
    cpu.ir = 0xB2FC;
    let v1: u32 = 0x74FF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, (v1 as u16 as i16 as i32 as u32), Size::Long, false);
    6
}
/// $0473DA: bls.w $473ec
pub fn i_0473da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473DE;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x473EC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0473de  ($0473DE, 14 B, kernel/jsr5b6_jmp518.c)
/// $0473DE: jsr $5b6.l
pub fn i_0473de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x473E4);
    cpu.pc = 0x5B6;
    16
}
/// $0473E4: jmp $518.l
pub fn i_0473e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473EA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== BigText_Typewriter_PutGlyph_0473ec  ($0473EC, 20 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0473EC: move.b $16(a6), d1
pub fn i_0473ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473F0;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0473F0: jsr $47822(pc)
pub fn i_0473f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x473F4);
    cpu.pc = 0x47822;
    16
}
/// $0473F4: addq.l #$1, $3c(a6)
pub fn i_0473f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473F8;
    cpu.ir = 0x52AE;
    let ad1 = cpu.a[6].wrapping_add(0x3C);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $0473F8: addi.w #$40, $22(a6)
pub fn i_0473f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x473FE;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0473FE: rts
pub fn i_0473fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47400;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SmallText_Typewriter_047400  ($047400, 48 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047400: move.w $30(a6), $5c(a6)
pub fn i_047400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47406;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $047406: lea.l $4740c(pc), a1
pub fn i_047406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4740A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4740C;
    16
}
/// $04740A: move.l a1, (a6)
pub fn i_04740a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4740C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04740C: cmpi.w #$0, $30(a6)
pub fn i_04740c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47412;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $047412: ble.w $4741c
pub fn i_047412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47416;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4741C; return 10; }
    8
}
/// $047416: subq.w #$1, $30(a6)
pub fn i_047416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4741A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04741A: rts
pub fn i_04741a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4741C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04741C: move.w $5c(a6), $30(a6)
pub fn i_04741c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47422;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad4, v3 as u16);
    20
}
/// $047422: movea.l $3c(a6), a1
pub fn i_047422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47426;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $047426: move.b (a1), d0
pub fn i_047426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47428;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $047428: cmpi.b #$ff, d0
pub fn i_047428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4742C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $04742C: bne.w $4743e
pub fn i_04742c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47430;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4743E; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_047430  ($047430, 14 B, kernel/jsr5b6_jmp518.c)
/// $047430: jsr $5b6.l
pub fn i_047430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47436);
    cpu.pc = 0x5B6;
    16
}
/// $047436: jmp $518.l
pub fn i_047436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4743C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== SmallText_Typewriter_CheckVramLo_04743e  ($04743E, 12 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04743E: movea.w $22(a6), a1
pub fn i_04743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47442;
    cpu.ir = 0x326E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = v3 as i16 as i32 as u32;
    12
}
/// $047442: cmpa.w #$7000, a1
pub fn i_047442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47446;
    cpu.ir = 0xB2FC;
    let v1: u32 = 0x7000;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, (v1 as u16 as i16 as i32 as u32), Size::Long, false);
    6
}
/// $047446: bcc.w $47458
pub fn i_047446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4744A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47458; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_04744a  ($04744A, 14 B, kernel/jsr5b6_jmp518.c)
/// $04744A: jsr $5b6.l
pub fn i_04744a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47450);
    cpu.pc = 0x5B6;
    16
}
/// $047450: jmp $518.l
pub fn i_047450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47456;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== SmallText_Typewriter_CheckVramHi_047458  ($047458, 8 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047458: cmpa.w #$74ff, a1
pub fn i_047458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4745C;
    cpu.ir = 0xB2FC;
    let v1: u32 = 0x74FF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, (v1 as u16 as i16 as i32 as u32), Size::Long, false);
    6
}
/// $04745C: bls.w $4746e
pub fn i_04745c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47460;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x4746E; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_047460  ($047460, 14 B, kernel/jsr5b6_jmp518.c)
/// $047460: jsr $5b6.l
pub fn i_047460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47466;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47466);
    cpu.pc = 0x5B6;
    16
}
/// $047466: jmp $518.l
pub fn i_047466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4746C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== SmallText_Typewriter_PutGlyph_04746e  ($04746E, 20 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04746E: move.b $16(a6), d1
pub fn i_04746e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47472;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $047472: jsr $477d4(pc)
pub fn i_047472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47476;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47476);
    cpu.pc = 0x477D4;
    16
}
/// $047476: addq.l #$1, $3c(a6)
pub fn i_047476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4747A;
    cpu.ir = 0x52AE;
    let ad1 = cpu.a[6].wrapping_add(0x3C);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $04747A: addi.w #$20, $22(a6)
pub fn i_04747a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47480;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $047480: rts
pub fn i_047480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47482;
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
// ===== Debug_SetCounter_04764E  ($04764E, 8 B, video/debug_hex_counter_047482.s)
/// $04764E: move.w d0, $106e92.l
pub fn i_04764e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47654;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106E92, v1 as u16);
    16
}
/// $047654: rts
pub fn i_047654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47656;
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
// ===== Fix_Clear5Tiles_723C_047676  ($047676, 12 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $047676: movea.w #$723c, a1
pub fn i_047676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4767A;
    cpu.ir = 0x327C;
    let v1: u32 = 0x723C;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $04767A: move.w #$238b, d0
pub fn i_04767a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4767E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x238B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04767E: moveq #$5, d1
pub fn i_04767e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47680;
    cpu.ir = 0x7205;
    cpu.d[1] = 0x5;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $047680: moveq #$1, d2
pub fn i_047680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47682;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrAbsThunk_047682  ($047682, 8 B, thunks/jsr_abs_thunks.c)
/// $047682: jsr $5da56.l
pub fn i_047682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47688);
    cpu.pc = 0x5DA56;
    16
}
/// $047688: rts
pub fn i_047688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4768A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_DrawBigNumber2Digit_04768a  ($04768A, 160 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04768A: cmpi.w #$63, d0
pub fn i_04768a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4768E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x63, Size::Word, false);
    8
}
/// $04768E: bls.w $47696
pub fn i_04768e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47692;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x47696; return 10; }
    8
}
/// $047692: move.w #$63, d0
pub fn i_047692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47696;
    cpu.ir = 0x303C;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047696: jsr $47656(pc)
pub fn i_047696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4769A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4769A);
    cpu.pc = 0x47656;
    16
}
/// $04769A: lea.l $28dee0.l, a1
pub fn i_04769a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DEE0;
    20
}
/// $0476A0: lsl.w #$1, d0
pub fn i_0476a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476A2;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0476A2: move.w (a1, d0.w), d2
pub fn i_0476a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476A6;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0476A6: lsl.w #$1, d1
pub fn i_0476a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476A8;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0476A8: move.w (a1, d1.w), d3
pub fn i_0476a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476AC;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0476AC: move.w d4, d0
pub fn i_0476ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476AE;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0476AE: move.w d3, d1
pub fn i_0476ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476B0;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0476B0: movem.w d0-d1, $3c0000.l
pub fn i_0476b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476B8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0476B8: addi.w #$20, d0
pub fn i_0476b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476BC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0476BC: addq.w #$1, d1
pub fn i_0476bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476BE;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0476BE: movem.w d0-d1, $3c0000.l
pub fn i_0476be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476C6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0476C6: subi.w #$1f, d0
pub fn i_0476c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476CA;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0476CA: addi.w #$f, d1
pub fn i_0476ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476CE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0476CE: movem.w d0-d1, $3c0000.l
pub fn i_0476ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476D6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0476D6: addi.w #$20, d0
pub fn i_0476d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476DA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0476DA: addq.w #$1, d1
pub fn i_0476da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476DC;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0476DC: movem.w d0-d1, $3c0000.l
pub fn i_0476dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476E4;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0476E4: subi.w #$21, d0
pub fn i_0476e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476E8;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0476E8: move.w d4, d0
pub fn i_0476e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476EA;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0476EA: addi.w #$40, d0
pub fn i_0476ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476EE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0476EE: move.w d2, d1
pub fn i_0476ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476F0;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0476F0: movem.w d0-d1, $3c0000.l
pub fn i_0476f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476F8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0476F8: addi.w #$20, d0
pub fn i_0476f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476FC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0476FC: addq.w #$1, d1
pub fn i_0476fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476FE;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0476FE: movem.w d0-d1, $3c0000.l
pub fn i_0476fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47706;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047706: subi.w #$1f, d0
pub fn i_047706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4770A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04770A: addi.w #$f, d1
pub fn i_04770a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4770E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04770E: movem.w d0-d1, $3c0000.l
pub fn i_04770e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47716;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047716: addi.w #$20, d0
pub fn i_047716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4771A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04771A: addq.w #$1, d1
pub fn i_04771a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4771C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04771C: movem.w d0-d1, $3c0000.l
pub fn i_04771c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47724;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047724: subi.w #$21, d0
pub fn i_047724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47728;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047728: rts
pub fn i_047728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4772A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_DrawBigNumber2Digit_NoLeadZero_04772a  ($04772A, 170 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $04772A: cmpi.w #$63, d0
pub fn i_04772a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4772E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x63, Size::Word, false);
    8
}
/// $04772E: bls.w $47736
pub fn i_04772e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47732;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x47736; return 10; }
    8
}
/// $047732: move.w #$63, d0
pub fn i_047732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47736;
    cpu.ir = 0x303C;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047736: jsr $47656(pc)
pub fn i_047736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4773A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4773A);
    cpu.pc = 0x47656;
    16
}
/// $04773A: lea.l $28dee0.l, a1
pub fn i_04773a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47740;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DEE0;
    20
}
/// $047740: lsl.w #$1, d0
pub fn i_047740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47742;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $047742: move.w (a1, d0.w), d2
pub fn i_047742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47746;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $047746: lsl.w #$1, d1
pub fn i_047746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47748;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $047748: move.w (a1, d1.w), d3
pub fn i_047748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4774C;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $04774C: cmp.w $28dee0.l, d3
pub fn i_04774c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47752;
    cpu.ir = 0xB679;
    let v2 = bus.read16(0x28DEE0) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $047752: beq.w $47792
pub fn i_047752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47756;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x47792; return 10; }
    8
}
/// $047756: move.w d4, d0
pub fn i_047756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47758;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047758: move.w d3, d1
pub fn i_047758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4775A;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04775A: movem.w d0-d1, $3c0000.l
pub fn i_04775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47762;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047762: addi.w #$20, d0
pub fn i_047762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47766;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047766: addq.w #$1, d1
pub fn i_047766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47768;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047768: movem.w d0-d1, $3c0000.l
pub fn i_047768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47770;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047770: subi.w #$1f, d0
pub fn i_047770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47774;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047774: addi.w #$f, d1
pub fn i_047774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47778;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047778: movem.w d0-d1, $3c0000.l
pub fn i_047778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47780;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $047780: addi.w #$20, d0
pub fn i_047780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47784;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047784: addq.w #$1, d1
pub fn i_047784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47786;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $047786: movem.w d0-d1, $3c0000.l
pub fn i_047786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4778E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $04778E: subi.w #$21, d0
pub fn i_04778e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47792;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047792: move.w d4, d0
pub fn i_047792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47794;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $047794: addi.w #$40, d0
pub fn i_047794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47798;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $047798: move.w d2, d1
pub fn i_047798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4779A;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04779A: movem.w d0-d1, $3c0000.l
pub fn i_04779a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477A2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0477A2: addi.w #$20, d0
pub fn i_0477a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477A6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0477A6: addq.w #$1, d1
pub fn i_0477a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477A8;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0477A8: movem.w d0-d1, $3c0000.l
pub fn i_0477a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477B0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0477B0: subi.w #$1f, d0
pub fn i_0477b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477B4;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0477B4: addi.w #$f, d1
pub fn i_0477b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477B8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0477B8: movem.w d0-d1, $3c0000.l
pub fn i_0477b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477C0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0477C0: addi.w #$20, d0
pub fn i_0477c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477C4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0477C4: addq.w #$1, d1
pub fn i_0477c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477C6;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0477C6: movem.w d0-d1, $3c0000.l
pub fn i_0477c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477CE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0477CE: subi.w #$21, d0
pub fn i_0477ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477D2;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0477D2: rts
pub fn i_0477d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x477D4;
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
// ===== List_ApplyWithSentinelFF_047888  ($047888, 38 B, kernel/list_apply_sentinel_047888.s)
/// $047888: move.w d1, d4
pub fn i_047888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4788A;
    cpu.ir = 0x3801;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04788A: movea.l a1, a4
pub fn i_04788a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4788C;
    cpu.ir = 0x2849;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $04788C: movea.l a2, a5
pub fn i_04788c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4788E;
    cpu.ir = 0x2A4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[5] = v1;
    4
}
/// $04788E: move.w (a5)+, d0
pub fn i_04788e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47890;
    cpu.ir = 0x301D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $047890: cmpi.w #$ff, d0
pub fn i_047890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47894;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Word, false);
    8
}
/// $047894: beq.w $478ac
pub fn i_047894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47898;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x478AC; return 10; }
    8
}
/// $047898: move.w d4, d1
pub fn i_047898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4789A;
    cpu.ir = 0x3204;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04789A: movea.l a4, a1
pub fn i_04789a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4789C;
    cpu.ir = 0x224C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $04789C: jsr $47872(pc)
pub fn i_04789c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x478A0);
    cpu.pc = 0x47872;
    16
}
/// $0478A0: move.l a4, d0
pub fn i_0478a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478A2;
    cpu.ir = 0x200C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0478A2: addi.l #$40, d0
pub fn i_0478a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478A8;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $0478A8: movea.l d0, a4
pub fn i_0478a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478AA;
    cpu.ir = 0x2840;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $0478AA: bra.b $4788e
pub fn i_0478aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478AC;
    cpu.ir = 0x60E2;
    cpu.pc = 0x4788E;
    10
}
/// $0478AC: rts
pub fn i_0478ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_0478e0  ($0478E0, 16 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $0478E0: movea.l $8(a6), a1
pub fn i_0478e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478E4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0478E4: move.b $10(a6), d0
pub fn i_0478e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478E8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0478E8: cmp.b $10(a1), d0
pub fn i_0478e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478EC;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0478EC: bcs.w $478f6
pub fn i_0478ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478F0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x478F6; return 10; }
    8
}
// ===== ClearXN_0478f0  ($0478F0, 6 B, thunks/ccr_helpers.c)
/// $0478F0: andi.b #$ee, ccr
pub fn i_0478f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478F4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0478F4: rts
pub fn i_0478f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0478f6  ($0478F6, 6 B, thunks/ccr_helpers.c)
/// $0478F6: ori.b #$11, ccr
pub fn i_0478f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478FA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0478FA: rts
pub fn i_0478fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x478FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04791C: move.b #$f0, $7d(a6)
pub fn i_04791c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47922;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047922: move.b #$8, $7e(a6)
pub fn i_047922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47928;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047928: bra.w $47992
pub fn i_047928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4792C;
    cpu.ir = 0x6000;
    cpu.pc = 0x47992;
    10
}
// ===== Pow_SpawnInit_04797c  ($04797C, 198 B, pow/prisoner_0478fc.s)
/// $04797C: jsr $13600.l
pub fn i_04797c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47982;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47982);
    cpu.pc = 0x13600;
    16
}
/// $047982: move.b #$f0, $7d(a6)
pub fn i_047982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47988;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047988: move.b #$8, $7e(a6)
pub fn i_047988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4798E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04798E: bra.w $47998
pub fn i_04798e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47992;
    cpu.ir = 0x6000;
    cpu.pc = 0x47998;
    10
}
/// $047992: jsr $5e7c0.l
pub fn i_047992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47998;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47998);
    cpu.pc = 0x5E7C0;
    16
}
/// $047998: clr.b $86(a6)
pub fn i_047998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4799C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04799C: clr.b $85(a6)
pub fn i_04799c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x479A0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x85);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0479A0: jsr $48ea6(pc)
pub fn i_0479a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x479A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x479A4);
    cpu.pc = 0x48EA6;
    16
}
/// $0479A4: jsr $5e0d4.l
pub fn i_0479a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x479AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x479AA);
    cpu.pc = 0x5E0D4;
    16
}
/// $0479AA: move.l a0, $94(a6)
pub fn i_0479aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x479AE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0479AE: tst.b $98(a6)
pub fn i_0479ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x479B2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0479B2: beq.w $47a4c
pub fn i_0479b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x479B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x47A4C; return 10; }
    8
}
/// $0479B6: bra.w $47b1e
pub fn i_0479b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x479BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x47B1E;
    10
}
// ===== Pow_RetargetPlayer_047a42  ($047A42, 10 B, pow/prisoner_0478fc.s)
/// $047A42: jsr $5e1ea.l
pub fn i_047a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47A48);
    cpu.pc = 0x5E1EA;
    16
}
/// $047A48: move.l a0, $94(a6)
pub fn i_047a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A4C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Pow_Idle_047a4c  ($047A4C, 210 B, pow/prisoner_0478fc.s)
/// $047A4C: clr.w $28(a6)
pub fn i_047a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A50;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $047A50: move.b $3a(a6), d0
pub fn i_047a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A54;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $047A54: eori.b #$1, d0
pub fn i_047a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A58;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $047A58: move.b d0, $7c(a6)
pub fn i_047a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A5C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047A5C: lea.l $28e312.l, a0
pub fn i_047a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E312;
    20
}
/// $047A62: jsr $28cd4.l
pub fn i_047a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47A68);
    cpu.pc = 0x28CD4;
    16
}
/// $047A68: lea.l $47a6e(pc), a1
pub fn i_047a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A6E;
    16
}
/// $047A6C: move.l a1, (a6)
pub fn i_047a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047A6E: jsr $48fb0(pc)
pub fn i_047a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47A72);
    cpu.pc = 0x48FB0;
    16
}
/// $047A72: jsr $28d70.l
pub fn i_047a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47A78);
    cpu.pc = 0x28D70;
    16
}
/// $047A78: tst.b $98(a6)
pub fn i_047a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A7C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $047A7C: beq.w $47aea
pub fn i_047a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x47AEA; return 10; }
    8
}
/// $047A80: movea.l $94(a6), a0
pub fn i_047a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A84;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $047A84: jsr $5e338.l
pub fn i_047a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47A8A);
    cpu.pc = 0x5E338;
    16
}
/// $047A8A: bcs.w $47ae0
pub fn i_047a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A8E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47AE0; return 10; }
    8
}
/// $047A8E: jsr $48fcc(pc)
pub fn i_047a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47A92);
    cpu.pc = 0x48FCC;
    16
}
/// $047A92: lea.l $28e1be.l, a0
pub fn i_047a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E1BE;
    20
}
/// $047A98: movea.l #$ffffffff, a1
pub fn i_047a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47A9E;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $047A9E: jsr $772.l
pub fn i_047a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47AA4);
    cpu.pc = 0x772;
    16
}
/// $047AA4: jsr $49172(pc)
pub fn i_047aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47AA8);
    cpu.pc = 0x49172;
    16
}
/// $047AA8: bcc.w $47ab2
pub fn i_047aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47AB2; return 10; }
    8
}
/// $047AAC: lea.l $47b1e(pc), a1
pub fn i_047aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47B1E;
    16
}
/// $047AB0: move.l a1, (a6)
pub fn i_047ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047AB2: jsr $49256(pc)
pub fn i_047ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47AB6);
    cpu.pc = 0x49256;
    16
}
/// $047AB6: bcc.w $47ac0
pub fn i_047ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ABA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47AC0; return 10; }
    8
}
/// $047ABA: lea.l $47ed2(pc), a1
pub fn i_047aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ABE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47ED2;
    16
}
/// $047ABE: move.l a1, (a6)
pub fn i_047abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047AC0: jsr $49196(pc)
pub fn i_047ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47AC4);
    cpu.pc = 0x49196;
    16
}
/// $047AC4: bcc.w $47ace
pub fn i_047ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47ACE; return 10; }
    8
}
/// $047AC8: lea.l $47cfe(pc), a1
pub fn i_047ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ACC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47CFE;
    16
}
/// $047ACC: move.l a1, (a6)
pub fn i_047acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ACE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047ACE: jsr $491de(pc)
pub fn i_047ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47AD2);
    cpu.pc = 0x491DE;
    16
}
/// $047AD2: bcc.w $47adc
pub fn i_047ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47ADC; return 10; }
    8
}
/// $047AD6: lea.l $47dbc(pc), a1
pub fn i_047ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ADA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47DBC;
    16
}
/// $047ADA: move.l a1, (a6)
pub fn i_047ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ADC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047ADC: bra.w $47aea
pub fn i_047adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x47AEA;
    10
}
/// $047AE0: jsr $5e1ea.l
pub fn i_047ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47AE6);
    cpu.pc = 0x5E1EA;
    16
}
/// $047AE6: move.l a0, $94(a6)
pub fn i_047ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AEA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $047AEA: cmpi.w #$18, $22(a6)
pub fn i_047aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AF0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Word, false);
    16
}
/// $047AF0: blt.w $47b0c
pub fn i_047af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AF4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x47B0C; return 10; }
    8
}
/// $047AF4: cmpi.w #$128, $22(a6)
pub fn i_047af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AFA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x128, Size::Word, false);
    16
}
/// $047AFA: bgt.w $47b0c
pub fn i_047afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47AFE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x47B0C; return 10; }
    8
}
/// $047AFE: jsr $49010(pc)
pub fn i_047afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B02);
    cpu.pc = 0x49010;
    16
}
/// $047B02: bcc.w $47b0c
pub fn i_047b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47B0C; return 10; }
    8
}
/// $047B06: lea.l $47f84(pc), a1
pub fn i_047b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F84;
    16
}
/// $047B0A: move.l a1, (a6)
pub fn i_047b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047B0C: jsr $4932c(pc)
pub fn i_047b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B10);
    cpu.pc = 0x4932C;
    16
}
/// $047B10: bcc.w $47b1a
pub fn i_047b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47B1A; return 10; }
    8
}
/// $047B14: lea.l $47f12(pc), a1
pub fn i_047b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047B18: move.l a1, (a6)
pub fn i_047b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047B1A: bra.w $489f8
pub fn i_047b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_WalkToward_047b1e  ($047B1E, 142 B, pow/prisoner_0478fc.s)
/// $047B1E: move.b $3a(a6), $7c(a6)
pub fn i_047b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B24;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $047B24: lea.l $2bfb2e.l, a0
pub fn i_047b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFB2E;
    20
}
/// $047B2A: jsr $799de.l
pub fn i_047b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47B30);
    cpu.pc = 0x799DE;
    16
}
/// $047B30: move.w d0, $36(a6)
pub fn i_047b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B34;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047B34: jsr $48f04(pc)
pub fn i_047b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B38);
    cpu.pc = 0x48F04;
    16
}
/// $047B38: lea.l $28e394.l, a0
pub fn i_047b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B3E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E394;
    20
}
/// $047B3E: jsr $28cd4.l
pub fn i_047b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47B44);
    cpu.pc = 0x28CD4;
    16
}
/// $047B44: lea.l $47b4a(pc), a1
pub fn i_047b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47B4A;
    16
}
/// $047B48: move.l a1, (a6)
pub fn i_047b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047B4A: jsr $48f2e(pc)
pub fn i_047b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B4E);
    cpu.pc = 0x48F2E;
    16
}
/// $047B4E: jsr $28d70.l
pub fn i_047b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47B54);
    cpu.pc = 0x28D70;
    16
}
/// $047B54: jsr $49346(pc)
pub fn i_047b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B58);
    cpu.pc = 0x49346;
    16
}
/// $047B58: bcc.w $47b62
pub fn i_047b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47B62; return 10; }
    8
}
/// $047B5C: lea.l $47e88(pc), a1
pub fn i_047b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E88;
    16
}
/// $047B60: move.l a1, (a6)
pub fn i_047b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047B62: jsr $490fa(pc)
pub fn i_047b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B66);
    cpu.pc = 0x490FA;
    16
}
/// $047B66: bcc.w $47b70
pub fn i_047b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47B70; return 10; }
    8
}
/// $047B6A: lea.l $47e88(pc), a1
pub fn i_047b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E88;
    16
}
/// $047B6E: move.l a1, (a6)
pub fn i_047b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047B70: jsr $49256(pc)
pub fn i_047b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B74);
    cpu.pc = 0x49256;
    16
}
/// $047B74: bcc.w $47b7e
pub fn i_047b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47B7E; return 10; }
    8
}
/// $047B78: lea.l $47e88(pc), a1
pub fn i_047b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E88;
    16
}
/// $047B7C: move.l a1, (a6)
pub fn i_047b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047B7E: jsr $49196(pc)
pub fn i_047b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B82);
    cpu.pc = 0x49196;
    16
}
/// $047B82: bcc.w $47b8c
pub fn i_047b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47B8C; return 10; }
    8
}
/// $047B86: lea.l $47e88(pc), a1
pub fn i_047b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E88;
    16
}
/// $047B8A: move.l a1, (a6)
pub fn i_047b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047B8C: jsr $491de(pc)
pub fn i_047b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B90);
    cpu.pc = 0x491DE;
    16
}
/// $047B90: bcc.w $47b9a
pub fn i_047b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B94;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47B9A; return 10; }
    8
}
/// $047B94: lea.l $47e88(pc), a1
pub fn i_047b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E88;
    16
}
/// $047B98: move.l a1, (a6)
pub fn i_047b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047B9A: jsr $4932c(pc)
pub fn i_047b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47B9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47B9E);
    cpu.pc = 0x4932C;
    16
}
/// $047B9E: bcc.w $47ba8
pub fn i_047b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47BA8; return 10; }
    8
}
/// $047BA2: lea.l $47f12(pc), a1
pub fn i_047ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047BA6: move.l a1, (a6)
pub fn i_047ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047BA8: bra.w $489f8
pub fn i_047ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RunRight_047bac  ($047BAC, 164 B, pow/prisoner_0478fc.s)
/// $047BAC: move.b $3a(a6), $7c(a6)
pub fn i_047bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BB2;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $047BB2: jsr $492f8(pc)
pub fn i_047bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47BB6);
    cpu.pc = 0x492F8;
    16
}
/// $047BB6: bcs.w $47c50
pub fn i_047bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BBA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47C50; return 10; }
    8
}
/// $047BBA: lea.l $2bfbb0.l, a0
pub fn i_047bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFBB0;
    20
}
/// $047BC0: jsr $799de.l
pub fn i_047bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47BC6);
    cpu.pc = 0x799DE;
    16
}
/// $047BC6: move.w d0, $36(a6)
pub fn i_047bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BCA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047BCA: jsr $48f04(pc)
pub fn i_047bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BCE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47BCE);
    cpu.pc = 0x48F04;
    16
}
/// $047BCE: lea.l $28e44e.l, a0
pub fn i_047bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E44E;
    20
}
/// $047BD4: jsr $28cd4.l
pub fn i_047bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47BDA);
    cpu.pc = 0x28CD4;
    16
}
/// $047BDA: lea.l $47be0(pc), a1
pub fn i_047bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47BE0;
    16
}
/// $047BDE: move.l a1, (a6)
pub fn i_047bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047BE0: jsr $48f2e(pc)
pub fn i_047be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47BE4);
    cpu.pc = 0x48F2E;
    16
}
/// $047BE4: jsr $28d70.l
pub fn i_047be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47BEA);
    cpu.pc = 0x28D70;
    16
}
/// $047BEA: jsr $49346(pc)
pub fn i_047bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47BEE);
    cpu.pc = 0x49346;
    16
}
/// $047BEE: bcc.w $47bf8
pub fn i_047bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47BF8; return 10; }
    8
}
/// $047BF2: lea.l $47c50(pc), a1
pub fn i_047bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47C50;
    16
}
/// $047BF6: move.l a1, (a6)
pub fn i_047bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047BF8: jsr $490fa(pc)
pub fn i_047bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47BFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47BFC);
    cpu.pc = 0x490FA;
    16
}
/// $047BFC: bcc.w $47c06
pub fn i_047bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47C06; return 10; }
    8
}
/// $047C00: lea.l $47a4c(pc), a1
pub fn i_047c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A4C;
    16
}
/// $047C04: move.l a1, (a6)
pub fn i_047c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047C06: jsr $49256(pc)
pub fn i_047c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C0A);
    cpu.pc = 0x49256;
    16
}
/// $047C0A: bcc.w $47c14
pub fn i_047c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47C14; return 10; }
    8
}
/// $047C0E: lea.l $47ed2(pc), a1
pub fn i_047c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47ED2;
    16
}
/// $047C12: move.l a1, (a6)
pub fn i_047c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047C14: jsr $49196(pc)
pub fn i_047c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C18);
    cpu.pc = 0x49196;
    16
}
/// $047C18: bcc.w $47c22
pub fn i_047c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47C22; return 10; }
    8
}
/// $047C1C: lea.l $47cfe(pc), a1
pub fn i_047c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47CFE;
    16
}
/// $047C20: move.l a1, (a6)
pub fn i_047c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047C22: jsr $492f8(pc)
pub fn i_047c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C26);
    cpu.pc = 0x492F8;
    16
}
/// $047C26: bcc.w $47c30
pub fn i_047c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47C30; return 10; }
    8
}
/// $047C2A: lea.l $47c50(pc), a1
pub fn i_047c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47C50;
    16
}
/// $047C2E: move.l a1, (a6)
pub fn i_047c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047C30: jsr $491de(pc)
pub fn i_047c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C34);
    cpu.pc = 0x491DE;
    16
}
/// $047C34: bcc.w $47c3e
pub fn i_047c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47C3E; return 10; }
    8
}
/// $047C38: lea.l $47dbc(pc), a1
pub fn i_047c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47DBC;
    16
}
/// $047C3C: move.l a1, (a6)
pub fn i_047c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047C3E: jsr $4932c(pc)
pub fn i_047c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C42);
    cpu.pc = 0x4932C;
    16
}
/// $047C42: bcc.w $47c4c
pub fn i_047c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C46;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47C4C; return 10; }
    8
}
/// $047C46: lea.l $47f12(pc), a1
pub fn i_047c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047C4A: move.l a1, (a6)
pub fn i_047c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047C4C: bra.w $489f8
pub fn i_047c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C50;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RunLeft_047c50  ($047C50, 174 B, pow/prisoner_0478fc.s)
/// $047C50: move.b $3a(a6), d0
pub fn i_047c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C54;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $047C54: eori.b #$1, d0
pub fn i_047c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C58;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $047C58: move.b d0, $7c(a6)
pub fn i_047c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C5C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047C5C: jsr $492f8(pc)
pub fn i_047c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C60);
    cpu.pc = 0x492F8;
    16
}
/// $047C60: bcs.w $47bac
pub fn i_047c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C64;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47BAC; return 10; }
    8
}
/// $047C64: lea.l $2bfbb0.l, a0
pub fn i_047c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFBB0;
    20
}
/// $047C6A: jsr $799de.l
pub fn i_047c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47C70);
    cpu.pc = 0x799DE;
    16
}
/// $047C70: move.w d0, $36(a6)
pub fn i_047c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C74;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047C74: jsr $48f04(pc)
pub fn i_047c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C78);
    cpu.pc = 0x48F04;
    16
}
/// $047C78: neg.w $28(a6)
pub fn i_047c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C7C;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047C7C: lea.l $28e49e.l, a0
pub fn i_047c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E49E;
    20
}
/// $047C82: jsr $28cd4.l
pub fn i_047c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47C88);
    cpu.pc = 0x28CD4;
    16
}
/// $047C88: lea.l $47c8e(pc), a1
pub fn i_047c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47C8E;
    16
}
/// $047C8C: move.l a1, (a6)
pub fn i_047c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047C8E: jsr $48f2e(pc)
pub fn i_047c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C92);
    cpu.pc = 0x48F2E;
    16
}
/// $047C92: jsr $28d70.l
pub fn i_047c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47C98);
    cpu.pc = 0x28D70;
    16
}
/// $047C98: jsr $49346(pc)
pub fn i_047c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47C9C);
    cpu.pc = 0x49346;
    16
}
/// $047C9C: bcc.w $47ca6
pub fn i_047c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47CA6; return 10; }
    8
}
/// $047CA0: lea.l $47bac(pc), a1
pub fn i_047ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47BAC;
    16
}
/// $047CA4: move.l a1, (a6)
pub fn i_047ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047CA6: jsr $490fa(pc)
pub fn i_047ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47CAA);
    cpu.pc = 0x490FA;
    16
}
/// $047CAA: bcc.w $47cb4
pub fn i_047caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47CB4; return 10; }
    8
}
/// $047CAE: lea.l $47a4c(pc), a1
pub fn i_047cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A4C;
    16
}
/// $047CB2: move.l a1, (a6)
pub fn i_047cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047CB4: jsr $49256(pc)
pub fn i_047cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47CB8);
    cpu.pc = 0x49256;
    16
}
/// $047CB8: bcc.w $47cc2
pub fn i_047cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47CC2; return 10; }
    8
}
/// $047CBC: lea.l $47ed2(pc), a1
pub fn i_047cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47ED2;
    16
}
/// $047CC0: move.l a1, (a6)
pub fn i_047cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047CC2: jsr $49196(pc)
pub fn i_047cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47CC6);
    cpu.pc = 0x49196;
    16
}
/// $047CC6: bcc.w $47cd0
pub fn i_047cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47CD0; return 10; }
    8
}
/// $047CCA: lea.l $47cfe(pc), a1
pub fn i_047cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47CFE;
    16
}
/// $047CCE: move.l a1, (a6)
pub fn i_047cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047CD0: jsr $492f8(pc)
pub fn i_047cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47CD4);
    cpu.pc = 0x492F8;
    16
}
/// $047CD4: bcc.w $47cde
pub fn i_047cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CD8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47CDE; return 10; }
    8
}
/// $047CD8: lea.l $47bac(pc), a1
pub fn i_047cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47BAC;
    16
}
/// $047CDC: move.l a1, (a6)
pub fn i_047cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047CDE: jsr $491de(pc)
pub fn i_047cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47CE2);
    cpu.pc = 0x491DE;
    16
}
/// $047CE2: bcc.w $47cec
pub fn i_047ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47CEC; return 10; }
    8
}
/// $047CE6: lea.l $47dbc(pc), a1
pub fn i_047ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47DBC;
    16
}
/// $047CEA: move.l a1, (a6)
pub fn i_047cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047CEC: jsr $4932c(pc)
pub fn i_047cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CF0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47CF0);
    cpu.pc = 0x4932C;
    16
}
/// $047CF0: bcc.w $47cfa
pub fn i_047cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47CFA; return 10; }
    8
}
/// $047CF4: lea.l $47f12(pc), a1
pub fn i_047cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047CF8: move.l a1, (a6)
pub fn i_047cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047CFA: bra.w $489f8
pub fn i_047cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47CFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RunAway_047cfe  ($047CFE, 190 B, pow/prisoner_0478fc.s)
/// $047CFE: move.b $3a(a6), d0
pub fn i_047cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D02;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $047D02: eori.b #$1, d0
pub fn i_047d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D06;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $047D06: move.b d0, $7c(a6)
pub fn i_047d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D0A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047D0A: jsr $492f8(pc)
pub fn i_047d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47D0E);
    cpu.pc = 0x492F8;
    16
}
/// $047D0E: bcs.w $47a4c
pub fn i_047d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D12;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47A4C; return 10; }
    8
}
/// $047D12: lea.l $28e1b6.l, a0
pub fn i_047d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E1B6;
    20
}
/// $047D18: jsr $5e086.l
pub fn i_047d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47D1E);
    cpu.pc = 0x5E086;
    16
}
/// $047D1E: move.l a0, $94(a6)
pub fn i_047d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D22;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $047D22: lea.l $2bfbb0.l, a0
pub fn i_047d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFBB0;
    20
}
/// $047D28: jsr $799de.l
pub fn i_047d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47D2E);
    cpu.pc = 0x799DE;
    16
}
/// $047D2E: move.w d0, $36(a6)
pub fn i_047d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D32;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047D32: jsr $48f04(pc)
pub fn i_047d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47D36);
    cpu.pc = 0x48F04;
    16
}
/// $047D36: neg.w $28(a6)
pub fn i_047d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D3A;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $047D3A: lea.l $28e49e.l, a0
pub fn i_047d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E49E;
    20
}
/// $047D40: jsr $28cd4.l
pub fn i_047d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47D46);
    cpu.pc = 0x28CD4;
    16
}
/// $047D46: lea.l $47d4c(pc), a1
pub fn i_047d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47D4C;
    16
}
/// $047D4A: move.l a1, (a6)
pub fn i_047d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047D4C: jsr $48f2e(pc)
pub fn i_047d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47D50);
    cpu.pc = 0x48F2E;
    16
}
/// $047D50: jsr $28d70.l
pub fn i_047d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47D56);
    cpu.pc = 0x28D70;
    16
}
/// $047D56: jsr $49346(pc)
pub fn i_047d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47D5A);
    cpu.pc = 0x49346;
    16
}
/// $047D5A: bcc.w $47d64
pub fn i_047d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47D64; return 10; }
    8
}
/// $047D5E: lea.l $47f84(pc), a1
pub fn i_047d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F84;
    16
}
/// $047D62: move.l a1, (a6)
pub fn i_047d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047D64: jsr $49010(pc)
pub fn i_047d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47D68);
    cpu.pc = 0x49010;
    16
}
/// $047D68: bcc.w $47d72
pub fn i_047d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D6C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47D72; return 10; }
    8
}
/// $047D6C: lea.l $47a4c(pc), a1
pub fn i_047d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A4C;
    16
}
/// $047D70: move.l a1, (a6)
pub fn i_047d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047D72: jsr $49256(pc)
pub fn i_047d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47D76);
    cpu.pc = 0x49256;
    16
}
/// $047D76: bcc.w $47d80
pub fn i_047d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47D80; return 10; }
    8
}
/// $047D7A: lea.l $47ed2(pc), a1
pub fn i_047d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47ED2;
    16
}
/// $047D7E: move.l a1, (a6)
pub fn i_047d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047D80: jsr $49196(pc)
pub fn i_047d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47D84);
    cpu.pc = 0x49196;
    16
}
/// $047D84: bcs.w $47d8e
pub fn i_047d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D88;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47D8E; return 10; }
    8
}
/// $047D88: lea.l $47a4c(pc), a1
pub fn i_047d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A4C;
    16
}
/// $047D8C: move.l a1, (a6)
pub fn i_047d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047D8E: jsr $492f8(pc)
pub fn i_047d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47D92);
    cpu.pc = 0x492F8;
    16
}
/// $047D92: bcc.w $47d9c
pub fn i_047d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47D9C; return 10; }
    8
}
/// $047D96: lea.l $47a4c(pc), a1
pub fn i_047d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A4C;
    16
}
/// $047D9A: move.l a1, (a6)
pub fn i_047d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47D9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047D9C: jsr $491de(pc)
pub fn i_047d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47DA0);
    cpu.pc = 0x491DE;
    16
}
/// $047DA0: bcc.w $47daa
pub fn i_047da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DA4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47DAA; return 10; }
    8
}
/// $047DA4: lea.l $47dbc(pc), a1
pub fn i_047da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47DBC;
    16
}
/// $047DA8: move.l a1, (a6)
pub fn i_047da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047DAA: jsr $4932c(pc)
pub fn i_047daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47DAE);
    cpu.pc = 0x4932C;
    16
}
/// $047DAE: bcc.w $47db8
pub fn i_047dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47DB8; return 10; }
    8
}
/// $047DB2: lea.l $47f12(pc), a1
pub fn i_047db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047DB6: move.l a1, (a6)
pub fn i_047db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047DB8: bra.w $489f8
pub fn i_047db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_Wait_047dbc  ($047DBC, 104 B, pow/prisoner_0478fc.s)
/// $047DBC: clr.w $28(a6)
pub fn i_047dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DC0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $047DC0: move.b $3a(a6), d0
pub fn i_047dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DC4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $047DC4: andi.b #$1, d0
pub fn i_047dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DC8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $047DC8: move.b d0, $7c(a6)
pub fn i_047dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DCC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047DCC: jsr $492f8(pc)
pub fn i_047dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47DD0);
    cpu.pc = 0x492F8;
    16
}
/// $047DD0: bcs.w $47ed2
pub fn i_047dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DD4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47ED2; return 10; }
    8
}
/// $047DD4: jsr $4926a(pc)
pub fn i_047dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47DD8);
    cpu.pc = 0x4926A;
    16
}
/// $047DD8: bcs.w $47e24
pub fn i_047dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DDC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47E24; return 10; }
    8
}
/// $047DDC: lea.l $28e1b6.l, a0
pub fn i_047ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E1B6;
    20
}
/// $047DE2: jsr $5e086.l
pub fn i_047de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47DE8);
    cpu.pc = 0x5E086;
    16
}
/// $047DE8: move.l a0, $94(a6)
pub fn i_047de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DEC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $047DEC: lea.l $28e7bc.l, a0
pub fn i_047dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E7BC;
    20
}
/// $047DF2: jsr $28cd4.l
pub fn i_047df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47DF8);
    cpu.pc = 0x28CD4;
    16
}
/// $047DF8: lea.l $47dfe(pc), a1
pub fn i_047df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47DFE;
    16
}
/// $047DFC: move.l a1, (a6)
pub fn i_047dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47DFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047DFE: jsr $48fb0(pc)
pub fn i_047dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47E02);
    cpu.pc = 0x48FB0;
    16
}
/// $047E02: jsr $28d70.l
pub fn i_047e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47E08);
    cpu.pc = 0x28D70;
    16
}
/// $047E08: bcc.w $47e12
pub fn i_047e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47E12; return 10; }
    8
}
/// $047E0C: lea.l $47e24(pc), a1
pub fn i_047e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E24;
    16
}
/// $047E10: move.l a1, (a6)
pub fn i_047e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047E12: jsr $4932c(pc)
pub fn i_047e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47E16);
    cpu.pc = 0x4932C;
    16
}
/// $047E16: bcc.w $47e20
pub fn i_047e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E1A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47E20; return 10; }
    8
}
/// $047E1A: lea.l $47f12(pc), a1
pub fn i_047e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047E1E: move.l a1, (a6)
pub fn i_047e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047E20: bra.w $489f8
pub fn i_047e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E24;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_WalkFree_047e24  ($047E24, 100 B, pow/prisoner_0478fc.s)
/// $047E24: move.b $3a(a6), $7c(a6)
pub fn i_047e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E2A;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $047E2A: lea.l $2bfb2e.l, a0
pub fn i_047e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFB2E;
    20
}
/// $047E30: jsr $799de.l
pub fn i_047e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47E36);
    cpu.pc = 0x799DE;
    16
}
/// $047E36: move.w d0, $36(a6)
pub fn i_047e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E3A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047E3A: jsr $48f04(pc)
pub fn i_047e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47E3E);
    cpu.pc = 0x48F04;
    16
}
/// $047E3E: lea.l $28e394.l, a0
pub fn i_047e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E394;
    20
}
/// $047E44: jsr $28cd4.l
pub fn i_047e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47E4A);
    cpu.pc = 0x28CD4;
    16
}
/// $047E4A: lea.l $47e50(pc), a1
pub fn i_047e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E50;
    16
}
/// $047E4E: move.l a1, (a6)
pub fn i_047e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047E50: jsr $48f2e(pc)
pub fn i_047e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47E54);
    cpu.pc = 0x48F2E;
    16
}
/// $047E54: jsr $28d70.l
pub fn i_047e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47E5A);
    cpu.pc = 0x28D70;
    16
}
/// $047E5A: jsr $4921e(pc)
pub fn i_047e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47E5E);
    cpu.pc = 0x4921E;
    16
}
/// $047E5E: bcs.w $47e68
pub fn i_047e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E62;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47E68; return 10; }
    8
}
/// $047E62: lea.l $47e88(pc), a1
pub fn i_047e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E88;
    16
}
/// $047E66: move.l a1, (a6)
pub fn i_047e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047E68: jsr $492f8(pc)
pub fn i_047e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47E6C);
    cpu.pc = 0x492F8;
    16
}
/// $047E6C: bcc.w $47e76
pub fn i_047e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47E76; return 10; }
    8
}
/// $047E70: lea.l $47e88(pc), a1
pub fn i_047e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47E88;
    16
}
/// $047E74: move.l a1, (a6)
pub fn i_047e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047E76: jsr $4932c(pc)
pub fn i_047e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47E7A);
    cpu.pc = 0x4932C;
    16
}
/// $047E7A: bcc.w $47e84
pub fn i_047e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47E84; return 10; }
    8
}
/// $047E7E: lea.l $47f12(pc), a1
pub fn i_047e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047E82: move.l a1, (a6)
pub fn i_047e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047E84: bra.w $489f8
pub fn i_047e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E88;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_Stop_047e88  ($047E88, 74 B, pow/prisoner_0478fc.s)
/// $047E88: movea.l $94(a6), a0
pub fn i_047e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E8C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $047E8C: jsr $5e338.l
pub fn i_047e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47E92);
    cpu.pc = 0x5E338;
    16
}
/// $047E92: bcs.w $47a42
pub fn i_047e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E96;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47A42; return 10; }
    8
}
/// $047E96: clr.w $28(a6)
pub fn i_047e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47E9A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $047E9A: lea.l $28e416.l, a0
pub fn i_047e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E416;
    20
}
/// $047EA0: jsr $28cd4.l
pub fn i_047ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47EA6);
    cpu.pc = 0x28CD4;
    16
}
/// $047EA6: lea.l $47eac(pc), a1
pub fn i_047ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47EAC;
    16
}
/// $047EAA: move.l a1, (a6)
pub fn i_047eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047EAC: jsr $48fb0(pc)
pub fn i_047eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47EB0);
    cpu.pc = 0x48FB0;
    16
}
/// $047EB0: jsr $28d70.l
pub fn i_047eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47EB6);
    cpu.pc = 0x28D70;
    16
}
/// $047EB6: bcc.w $47ec0
pub fn i_047eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47EC0; return 10; }
    8
}
/// $047EBA: lea.l $47a4c(pc), a1
pub fn i_047eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A4C;
    16
}
/// $047EBE: move.l a1, (a6)
pub fn i_047ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047EC0: jsr $4932c(pc)
pub fn i_047ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47EC4);
    cpu.pc = 0x4932C;
    16
}
/// $047EC4: bcc.w $47ece
pub fn i_047ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47ECE; return 10; }
    8
}
/// $047EC8: lea.l $47f12(pc), a1
pub fn i_047ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ECC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047ECC: move.l a1, (a6)
pub fn i_047ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ECE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047ECE: bra.w $489f8
pub fn i_047ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ED2;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_Turn_047ed2  ($047ED2, 64 B, pow/prisoner_0478fc.s)
/// $047ED2: addq.b #$1, $77(a6)
pub fn i_047ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47ED6;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $047ED6: clr.w $28(a6)
pub fn i_047ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EDA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $047EDA: lea.l $28e858.l, a0
pub fn i_047eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E858;
    20
}
/// $047EE0: jsr $28cd4.l
pub fn i_047ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47EE6);
    cpu.pc = 0x28CD4;
    16
}
/// $047EE6: lea.l $47eec(pc), a1
pub fn i_047ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47EEC;
    16
}
/// $047EEA: move.l a1, (a6)
pub fn i_047eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047EEC: jsr $48fb0(pc)
pub fn i_047eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EF0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47EF0);
    cpu.pc = 0x48FB0;
    16
}
/// $047EF0: jsr $28d70.l
pub fn i_047ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47EF6);
    cpu.pc = 0x28D70;
    16
}
/// $047EF6: bcc.w $47f00
pub fn i_047ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47F00; return 10; }
    8
}
/// $047EFA: lea.l $47a4c(pc), a1
pub fn i_047efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47EFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A4C;
    16
}
/// $047EFE: move.l a1, (a6)
pub fn i_047efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047F00: jsr $4932c(pc)
pub fn i_047f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47F04);
    cpu.pc = 0x4932C;
    16
}
/// $047F04: bcc.w $47f0e
pub fn i_047f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47F0E; return 10; }
    8
}
/// $047F08: lea.l $47f12(pc), a1
pub fn i_047f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047F0C: move.l a1, (a6)
pub fn i_047f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047F0E: bra.w $489f8
pub fn i_047f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F12;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_Hurt_047f12  ($047F12, 54 B, pow/prisoner_0478fc.s)
/// $047F12: move.w $28(a6), d0
pub fn i_047f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F16;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047F16: asr.w #$1, d0
pub fn i_047f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F18;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $047F18: move.w d0, $28(a6)
pub fn i_047f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F1C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $047F1C: lea.l $28e75e.l, a0
pub fn i_047f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E75E;
    20
}
/// $047F22: jsr $28cd4.l
pub fn i_047f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47F28);
    cpu.pc = 0x28CD4;
    16
}
/// $047F28: lea.l $47f2e(pc), a1
pub fn i_047f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F2E;
    16
}
/// $047F2C: move.l a1, (a6)
pub fn i_047f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047F2E: jsr $27c8c.l
pub fn i_047f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47F34);
    cpu.pc = 0x27C8C;
    16
}
/// $047F34: bcc.w $47f3e
pub fn i_047f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47F3E; return 10; }
    8
}
/// $047F38: lea.l $47f48(pc), a1
pub fn i_047f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F48;
    16
}
/// $047F3C: move.l a1, (a6)
pub fn i_047f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047F3E: jsr $28d70.l
pub fn i_047f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47F44);
    cpu.pc = 0x28D70;
    16
}
/// $047F44: bra.w $489f8
pub fn i_047f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F48;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_GetUp_047f48  ($047F48, 60 B, pow/prisoner_0478fc.s)
/// $047F48: clr.w $28(a6)
pub fn i_047f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F4C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $047F4C: lea.l $28e79c.l, a0
pub fn i_047f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F52;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E79C;
    20
}
/// $047F52: jsr $28cd4.l
pub fn i_047f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47F58);
    cpu.pc = 0x28CD4;
    16
}
/// $047F58: lea.l $47f5e(pc), a1
pub fn i_047f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F5E;
    16
}
/// $047F5C: move.l a1, (a6)
pub fn i_047f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047F5E: jsr $48fb0(pc)
pub fn i_047f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47F62);
    cpu.pc = 0x48FB0;
    16
}
/// $047F62: jsr $28d70.l
pub fn i_047f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47F68);
    cpu.pc = 0x28D70;
    16
}
/// $047F68: bcc.w $47f72
pub fn i_047f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F6C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47F72; return 10; }
    8
}
/// $047F6C: lea.l $47a4c(pc), a1
pub fn i_047f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A4C;
    16
}
/// $047F70: move.l a1, (a6)
pub fn i_047f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047F72: jsr $4932c(pc)
pub fn i_047f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47F76);
    cpu.pc = 0x4932C;
    16
}
/// $047F76: bcc.w $47f80
pub fn i_047f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47F80; return 10; }
    8
}
/// $047F7A: lea.l $47f12(pc), a1
pub fn i_047f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $047F7E: move.l a1, (a6)
pub fn i_047f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $047F80: bra.w $489f8
pub fn i_047f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F84;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RescueStart_047f84  ($047F84, 192 B, pow/prisoner_0478fc.s)
/// $047F84: tst.b $85(a6)
pub fn i_047f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F88;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $047F88: beq.w $47faa
pub fn i_047f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x47FAA; return 10; }
    8
}
/// $047F8C: movea.l $94(a6), a0
pub fn i_047f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F90;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $047F90: jsr $32de0.l
pub fn i_047f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47F96);
    cpu.pc = 0x32DE0;
    16
}
/// $047F96: bcc.w $47faa
pub fn i_047f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x47FAA; return 10; }
    8
}
/// $047F9A: movea.l $94(a6), a0
pub fn i_047f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47F9E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $047F9E: move.w $24(a0), d0
pub fn i_047f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FA2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $047FA2: cmp.w $24(a6), d0
pub fn i_047fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FA6;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $047FA6: beq.w $482c0
pub fn i_047fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x482C0; return 10; }
    8
}
/// $047FAA: clr.b $77(a6)
pub fn i_047faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FAE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $047FAE: clr.w $28(a6)
pub fn i_047fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FB2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $047FB2: movea.l $94(a6), a0
pub fn i_047fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FB6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $047FB6: jsr $5e070.l
pub fn i_047fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47FBC);
    cpu.pc = 0x5E070;
    16
}
/// $047FBC: add.b $7e(a6), d0
pub fn i_047fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FC0;
    cpu.ir = 0xD02E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $047FC0: move.b d0, $7a(a6)
pub fn i_047fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FC4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047FC4: move.b $7d(a6), d1
pub fn i_047fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FC8;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $047FC8: jsr $4939c(pc)
pub fn i_047fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x47FCC);
    cpu.pc = 0x4939C;
    16
}
/// $047FCC: bcs.w $47fd4
pub fn i_047fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FD0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x47FD4; return 10; }
    8
}
/// $047FD0: move.b #$80, d1
pub fn i_047fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FD4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $047FD4: move.b $7a(a6), d0
pub fn i_047fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FD8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $047FD8: and.b d1, d0
pub fn i_047fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FDA;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $047FDA: lsr.b #$4, d0
pub fn i_047fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FDC;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $047FDC: move.b d0, $7a(a6)
pub fn i_047fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FE0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $047FE0: clr.b $81(a6)
pub fn i_047fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FE4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $047FE4: lea.l $28e8f4.l, a0
pub fn i_047fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E8F4;
    20
}
/// $047FEA: jsr $28cd4.l
pub fn i_047fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x47FF0);
    cpu.pc = 0x28CD4;
    16
}
/// $047FF0: andi.b #$f, $7a(a6)
pub fn i_047ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FF6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $047FF6: move.b $7a(a6), $7b(a6)
pub fn i_047ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47FFC;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $047FFC: cmpi.b #$8, $7a(a6)
pub fn i_047ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48002;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Byte, false);
    16
}
/// $048002: ble.w $48018
pub fn i_048002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48006;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x48018; return 10; }
    8
}
/// $048006: move.b #$1, $81(a6)
pub fn i_048006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4800C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04800C: lea.l $28e914.l, a0
pub fn i_04800c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48012;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E914;
    20
}
/// $048012: jsr $28cd4.l
pub fn i_048012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48018);
    cpu.pc = 0x28CD4;
    16
}
/// $048018: lea.l $4801e(pc), a1
pub fn i_048018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4801C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4801E;
    16
}
/// $04801C: move.l a1, (a6)
pub fn i_04801c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4801E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04801E: jsr $48fb0(pc)
pub fn i_04801e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48022;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48022);
    cpu.pc = 0x48FB0;
    16
}
/// $048022: jsr $28d70.l
pub fn i_048022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48028;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48028);
    cpu.pc = 0x28D70;
    16
}
/// $048028: bcc.w $48032
pub fn i_048028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4802C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48032; return 10; }
    8
}
/// $04802C: lea.l $48044(pc), a1
pub fn i_04802c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48030;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48044;
    16
}
/// $048030: move.l a1, (a6)
pub fn i_048030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48032;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048032: jsr $4932c(pc)
pub fn i_048032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48036;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48036);
    cpu.pc = 0x4932C;
    16
}
/// $048036: bcc.w $48040
pub fn i_048036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4803A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48040; return 10; }
    8
}
/// $04803A: lea.l $47f12(pc), a1
pub fn i_04803a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4803E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $04803E: move.l a1, (a6)
pub fn i_04803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48040;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048040: bra.w $489f8
pub fn i_048040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48044;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RescueFaceCount_048044  ($048044, 172 B, pow/prisoner_0478fc.s)
/// $048044: clr.w $28(a6)
pub fn i_048044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48048;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048048: andi.b #$f, $7a(a6)
pub fn i_048048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4804E;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04804E: tst.b $81(a6)
pub fn i_04804e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48052;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048052: bne.w $48080
pub fn i_048052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48056;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48080; return 10; }
    8
}
/// $048056: lea.l $28e974.l, a0
pub fn i_048056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4805C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E974;
    20
}
/// $04805C: jsr $28cd4.l
pub fn i_04805c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48062;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48062);
    cpu.pc = 0x28CD4;
    16
}
/// $048062: clr.w $78(a6)
pub fn i_048062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48066;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048066: btst.b #$0, $3a(a6)
pub fn i_048066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4806C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04806C: bne.w $480ac
pub fn i_04806c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48070;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x480AC; return 10; }
    8
}
/// $048070: move.b #$8, d0
pub fn i_048070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48074;
    cpu.ir = 0x103C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $048074: sub.b $7a(a6), d0
pub fn i_048074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48078;
    cpu.ir = 0x902E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $048078: move.b d0, $7a(a6)
pub fn i_048078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4807C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04807C: bra.w $480ac
pub fn i_04807c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48080;
    cpu.ir = 0x6000;
    cpu.pc = 0x480AC;
    10
}
/// $048080: lea.l $28eab0.l, a0
pub fn i_048080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48086;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28EAB0;
    20
}
/// $048086: jsr $28cd4.l
pub fn i_048086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4808C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4808C);
    cpu.pc = 0x28CD4;
    16
}
/// $04808C: clr.w $78(a6)
pub fn i_04808c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48090;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048090: subi.b #$9, $7a(a6)
pub fn i_048090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48096;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x9, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $048096: btst.b #$0, $3a(a6)
pub fn i_048096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4809C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04809C: beq.w $480ac
pub fn i_04809c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x480AC; return 10; }
    8
}
/// $0480A0: move.b #$6, d0
pub fn i_0480a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480A4;
    cpu.ir = 0x103C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0480A4: sub.b $7a(a6), d0
pub fn i_0480a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480A8;
    cpu.ir = 0x902E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $0480A8: move.b d0, $7a(a6)
pub fn i_0480a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480AC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0480AC: move.b $7a(a6), d0
pub fn i_0480ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480B0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0480B0: add.b d0, $7a(a6)
pub fn i_0480b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480B4;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0480B4: lea.l $480ba(pc), a1
pub fn i_0480b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x480BA;
    16
}
/// $0480B8: move.l a1, (a6)
pub fn i_0480b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0480BA: jsr $48fb0(pc)
pub fn i_0480ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x480BE);
    cpu.pc = 0x48FB0;
    16
}
/// $0480BE: jsr $28d70.l
pub fn i_0480be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x480C4);
    cpu.pc = 0x28D70;
    16
}
/// $0480C4: move.b $7a(a6), d0
pub fn i_0480c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480C8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0480C8: andi.w #$1f, d0
pub fn i_0480c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480CC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0480CC: cmp.w $78(a6), d0
pub fn i_0480cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480D0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0480D0: bne.w $480da
pub fn i_0480d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x480DA; return 10; }
    8
}
/// $0480D4: lea.l $480f0(pc), a1
pub fn i_0480d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x480F0;
    16
}
/// $0480D8: move.l a1, (a6)
pub fn i_0480d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0480DA: addq.w #$1, $78(a6)
pub fn i_0480da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480DE;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0480DE: jsr $4932c(pc)
pub fn i_0480de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x480E2);
    cpu.pc = 0x4932C;
    16
}
/// $0480E2: bcc.w $480ec
pub fn i_0480e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x480EC; return 10; }
    8
}
/// $0480E6: lea.l $47f12(pc), a1
pub fn i_0480e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $0480EA: move.l a1, (a6)
pub fn i_0480ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0480EC: bra.w $489f8
pub fn i_0480ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RescueStartSalute_0480f0  ($0480F0, 24 B, pow/prisoner_0478fc.s)
/// $0480F0: clr.w $28(a6)
pub fn i_0480f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480F4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0480F4: subq.w #$1, $78(a6)
pub fn i_0480f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480F8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0480F8: lea.l $2bfd36.l, a0
pub fn i_0480f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x480FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFD36;
    20
}
/// $0480FE: jsr $799de.l
pub fn i_0480fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48104;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48104);
    cpu.pc = 0x799DE;
    16
}
/// $048104: move.b d0, $82(a6)
pub fn i_048104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48108;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Pow_RescueSalute_048108  ($048108, 182 B, pow/prisoner_0478fc.s)
/// $048108: move.b $7b(a6), d0
pub fn i_048108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4810C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04810C: andi.w #$f, d0
pub fn i_04810c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48110;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048110: btst.b #$0, $3a(a6)
pub fn i_048110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48116;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $048116: bne.w $4813a
pub fn i_048116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4811A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4813A; return 10; }
    8
}
/// $04811A: movea.l #$28e1ca, a0
pub fn i_04811a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48120;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28E1CA;
    cpu.a[0] = v1;
    4
}
/// $048120: lsl.w #$2, d0
pub fn i_048120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48122;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $048122: movea.l (a0, d0.w), a0
pub fn i_048122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48126;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $048126: cmpa.l #$ffffffff, a0
pub fn i_048126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4812C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04812C: beq.w $48136
pub fn i_04812c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48130;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48136; return 10; }
    8
}
/// $048130: jsr $28cd4.l
pub fn i_048130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48136);
    cpu.pc = 0x28CD4;
    16
}
/// $048136: bra.w $48156
pub fn i_048136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4813A;
    cpu.ir = 0x6000;
    cpu.pc = 0x48156;
    10
}
/// $04813A: movea.l #$28e20a, a0
pub fn i_04813a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48140;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28E20A;
    cpu.a[0] = v1;
    4
}
/// $048140: lsl.w #$2, d0
pub fn i_048140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48142;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $048142: movea.l (a0, d0.w), a0
pub fn i_048142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48146;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $048146: cmpa.l #$ffffffff, a0
pub fn i_048146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4814C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04814C: beq.w $48156
pub fn i_04814c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48150;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48156; return 10; }
    8
}
/// $048150: jsr $28cd4.l
pub fn i_048150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48156);
    cpu.pc = 0x28CD4;
    16
}
/// $048156: jsr $4940e(pc)
pub fn i_048156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4815A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4815A);
    cpu.pc = 0x4940E;
    16
}
/// $04815A: lea.l $2bfcb4.l, a0
pub fn i_04815a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48160;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFCB4;
    20
}
/// $048160: jsr $799de.l
pub fn i_048160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48166;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48166);
    cpu.pc = 0x799DE;
    16
}
/// $048166: move.w d0, $72(a6)
pub fn i_048166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4816A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04816A: lea.l $48170(pc), a1
pub fn i_04816a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4816E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48170;
    16
}
/// $04816E: move.l a1, (a6)
pub fn i_04816e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48170;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048170: jsr $48fb0(pc)
pub fn i_048170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48174;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48174);
    cpu.pc = 0x48FB0;
    16
}
/// $048174: jsr $28d70.l
pub fn i_048174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4817A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4817A);
    cpu.pc = 0x28D70;
    16
}
/// $04817A: bcs.w $48188
pub fn i_04817a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4817E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x48188; return 10; }
    8
}
/// $04817E: cmpi.b #$1, $82(a6)
pub fn i_04817e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48184;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $048184: ble.w $481ac
pub fn i_048184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48188;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x481AC; return 10; }
    8
}
/// $048188: cmpi.w #$0, $72(a6)
pub fn i_048188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4818E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04818E: bgt.w $481ac
pub fn i_04818e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48192;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x481AC; return 10; }
    8
}
/// $048192: lea.l $48108(pc), a1
pub fn i_048192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48196;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48108;
    16
}
/// $048196: move.l a1, (a6)
pub fn i_048196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48198;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048198: subq.b #$1, $82(a6)
pub fn i_048198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4819C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04819C: cmpi.b #$0, $82(a6)
pub fn i_04819c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481A2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0481A2: bgt.w $481ac
pub fn i_0481a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481A6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x481AC; return 10; }
    8
}
/// $0481A6: lea.l $481be(pc), a1
pub fn i_0481a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x481BE;
    16
}
/// $0481AA: move.l a1, (a6)
pub fn i_0481aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0481AC: jsr $4932c(pc)
pub fn i_0481ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x481B0);
    cpu.pc = 0x4932C;
    16
}
/// $0481B0: bcc.w $481ba
pub fn i_0481b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x481BA; return 10; }
    8
}
/// $0481B4: lea.l $47f12(pc), a1
pub fn i_0481b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $0481B8: move.l a1, (a6)
pub fn i_0481b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0481BA: bra.w $489f8
pub fn i_0481ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RescueTurnBack_0481be  ($0481BE, 162 B, pow/prisoner_0478fc.s)
/// $0481BE: clr.w $28(a6)
pub fn i_0481be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481C2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0481C2: tst.b $81(a6)
pub fn i_0481c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481C6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0481C6: bne.w $481f6
pub fn i_0481c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x481F6; return 10; }
    8
}
/// $0481CA: lea.l $28e974.l, a0
pub fn i_0481ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E974;
    20
}
/// $0481D0: jsr $28cd4.l
pub fn i_0481d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x481D6);
    cpu.pc = 0x28CD4;
    16
}
/// $0481D6: cmpi.w #$8, $78(a6)
pub fn i_0481d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481DC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $0481DC: blt.w $4821e
pub fn i_0481dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481E0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4821E; return 10; }
    8
}
/// $0481E0: move.w #$10, d0
pub fn i_0481e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0481E4: sub.w $78(a6), d0
pub fn i_0481e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481E8;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0481E8: move.w d0, $78(a6)
pub fn i_0481e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481EC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0481EC: bchg.b #$0, $3a(a6)
pub fn i_0481ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481F2;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0481F2: bra.w $4821e
pub fn i_0481f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4821E;
    10
}
/// $0481F6: lea.l $28eab0.l, a0
pub fn i_0481f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x481FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28EAB0;
    20
}
/// $0481FC: jsr $28cd4.l
pub fn i_0481fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48202;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48202);
    cpu.pc = 0x28CD4;
    16
}
/// $048202: cmpi.w #$6, $78(a6)
pub fn i_048202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48208;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Word, false);
    16
}
/// $048208: blt.w $4821e
pub fn i_048208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4820C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4821E; return 10; }
    8
}
/// $04820C: move.w #$c, d0
pub fn i_04820c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48210;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048210: sub.w $78(a6), d0
pub fn i_048210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48214;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $048214: move.w d0, $78(a6)
pub fn i_048214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48218;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048218: bchg.b #$0, $3a(a6)
pub fn i_048218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4821E;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04821E: lea.l $48224(pc), a1
pub fn i_04821e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48222;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48224;
    16
}
/// $048222: move.l a1, (a6)
pub fn i_048222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48224;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048224: jsr $48fb0(pc)
pub fn i_048224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48228;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48228);
    cpu.pc = 0x48FB0;
    16
}
/// $048228: jsr $28d70.l
pub fn i_048228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4822E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4822E);
    cpu.pc = 0x28D70;
    16
}
/// $04822E: tst.w $78(a6)
pub fn i_04822e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48232;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $048232: bne.w $4823c
pub fn i_048232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48236;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4823C; return 10; }
    8
}
/// $048236: lea.l $48260(pc), a1
pub fn i_048236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4823A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48260;
    16
}
/// $04823A: move.l a1, (a6)
pub fn i_04823a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4823C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04823C: move.b $106f28.l, d0
pub fn i_04823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48242;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $048242: andi.b #$1, d0
pub fn i_048242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48246;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $048246: bne.w $4824e
pub fn i_048246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4824A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4824E; return 10; }
    8
}
/// $04824A: subq.w #$1, $78(a6)
pub fn i_04824a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4824E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04824E: jsr $4932c(pc)
pub fn i_04824e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48252;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48252);
    cpu.pc = 0x4932C;
    16
}
/// $048252: bcc.w $4825c
pub fn i_048252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48256;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4825C; return 10; }
    8
}
/// $048256: lea.l $47f12(pc), a1
pub fn i_048256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4825A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $04825A: move.l a1, (a6)
pub fn i_04825a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4825C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04825C: bra.w $489f8
pub fn i_04825c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48260;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RescueGiveItem_048260  ($048260, 160 B, pow/prisoner_0478fc.s)
/// $048260: clr.w $28(a6)
pub fn i_048260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48264;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048264: lea.l $2bfc32.l, a0
pub fn i_048264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4826A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFC32;
    20
}
/// $04826A: jsr $799de.l
pub fn i_04826a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48270);
    cpu.pc = 0x799DE;
    16
}
/// $048270: move.w d0, $72(a6)
pub fn i_048270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48274;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048274: lea.l $28e934.l, a0
pub fn i_048274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4827A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E934;
    20
}
/// $04827A: jsr $28cd4.l
pub fn i_04827a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48280);
    cpu.pc = 0x28CD4;
    16
}
/// $048280: tst.b $81(a6)
pub fn i_048280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48284;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048284: beq.w $48294
pub fn i_048284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48288;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48294; return 10; }
    8
}
/// $048288: lea.l $28e954.l, a0
pub fn i_048288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4828E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E954;
    20
}
/// $04828E: jsr $28cd4.l
pub fn i_04828e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48294;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48294);
    cpu.pc = 0x28CD4;
    16
}
/// $048294: lea.l $4829a(pc), a1
pub fn i_048294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48298;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4829A;
    16
}
/// $048298: move.l a1, (a6)
pub fn i_048298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4829A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04829A: jsr $48fb0(pc)
pub fn i_04829a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4829E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4829E);
    cpu.pc = 0x48FB0;
    16
}
/// $04829E: jsr $28d70.l
pub fn i_04829e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x482A4);
    cpu.pc = 0x28D70;
    16
}
/// $0482A4: bcc.w $482ae
pub fn i_0482a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x482AE; return 10; }
    8
}
/// $0482A8: lea.l $47a42(pc), a1
pub fn i_0482a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A42;
    16
}
/// $0482AC: move.l a1, (a6)
pub fn i_0482ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0482AE: jsr $4932c(pc)
pub fn i_0482ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x482B2);
    cpu.pc = 0x4932C;
    16
}
/// $0482B2: bcc.w $482bc
pub fn i_0482b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x482BC; return 10; }
    8
}
/// $0482B6: lea.l $47f12(pc), a1
pub fn i_0482b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $0482BA: move.l a1, (a6)
pub fn i_0482ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0482BC: bra.w $489f8
pub fn i_0482bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
/// $0482C0: move.l #$28e03e, $48(a6)
pub fn i_0482c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482C8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28E03E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0482C8: lea.l $28e656.l, a0
pub fn i_0482c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E656;
    20
}
/// $0482CE: jsr $28cd4.l
pub fn i_0482ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x482D4);
    cpu.pc = 0x28CD4;
    16
}
/// $0482D4: lea.l $482da(pc), a1
pub fn i_0482d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x482DA;
    16
}
/// $0482D8: move.l a1, (a6)
pub fn i_0482d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0482DA: jsr $48fb0(pc)
pub fn i_0482da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x482DE);
    cpu.pc = 0x48FB0;
    16
}
/// $0482DE: jsr $28d70.l
pub fn i_0482de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x482E4);
    cpu.pc = 0x28D70;
    16
}
/// $0482E4: bcc.w $482ee
pub fn i_0482e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x482EE; return 10; }
    8
}
/// $0482E8: lea.l $48300(pc), a1
pub fn i_0482e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48300;
    16
}
/// $0482EC: move.l a1, (a6)
pub fn i_0482ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0482EE: jsr $4932c(pc)
pub fn i_0482ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x482F2);
    cpu.pc = 0x4932C;
    16
}
/// $0482F2: bcc.w $482fc
pub fn i_0482f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x482FC; return 10; }
    8
}
/// $0482F6: lea.l $47f12(pc), a1
pub fn i_0482f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $0482FA: move.l a1, (a6)
pub fn i_0482fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0482FC: bra.w $489f8
pub fn i_0482fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48300;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_RescueThanksInit_048300  ($048300, 20 B, pow/prisoner_0478fc.s)
/// $048300: clr.w $28(a6)
pub fn i_048300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48304;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048304: lea.l $2bfd36.l, a0
pub fn i_048304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4830A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFD36;
    20
}
/// $04830A: jsr $799de.l
pub fn i_04830a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48310);
    cpu.pc = 0x799DE;
    16
}
/// $048310: move.b d0, $82(a6)
pub fn i_048310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48314;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Pow_RescueThanks_048314  ($048314, 126 B, pow/prisoner_0478fc.s)
/// $048314: lea.l $2bfcb4.l, a0
pub fn i_048314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4831A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFCB4;
    20
}
/// $04831A: jsr $799de.l
pub fn i_04831a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48320;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48320);
    cpu.pc = 0x799DE;
    16
}
/// $048320: move.w d0, $72(a6)
pub fn i_048320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48324;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048324: move.l #$28e092, $48(a6)
pub fn i_048324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4832C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28E092;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04832C: lea.l $28e6c8.l, a0
pub fn i_04832c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48332;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6C8;
    20
}
/// $048332: jsr $28cd4.l
pub fn i_048332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48338);
    cpu.pc = 0x28CD4;
    16
}
/// $048338: lea.l $4833e(pc), a1
pub fn i_048338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4833C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4833E;
    16
}
/// $04833C: move.l a1, (a6)
pub fn i_04833c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4833E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04833E: jsr $48fb0(pc)
pub fn i_04833e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48342;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48342);
    cpu.pc = 0x48FB0;
    16
}
/// $048342: jsr $28d70.l
pub fn i_048342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48348);
    cpu.pc = 0x28D70;
    16
}
/// $048348: bcc.w $48380
pub fn i_048348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4834C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48380; return 10; }
    8
}
/// $04834C: cmpi.w #$0, $72(a6)
pub fn i_04834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48352;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $048352: bgt.w $48380
pub fn i_048352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48356;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48380; return 10; }
    8
}
/// $048356: lea.l $48314(pc), a1
pub fn i_048356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4835A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48314;
    16
}
/// $04835A: move.l a1, (a6)
pub fn i_04835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4835C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04835C: subq.b #$1, $82(a6)
pub fn i_04835c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48360;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $048360: cmpi.b #$0, $82(a6)
pub fn i_048360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48366;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $048366: bgt.w $48380
pub fn i_048366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4836A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48380; return 10; }
    8
}
/// $04836A: lea.l $48392(pc), a1
pub fn i_04836a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4836E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48392;
    16
}
/// $04836E: move.l a1, (a6)
pub fn i_04836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48370;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048370: lea.l $2bfc32.l, a0
pub fn i_048370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48376;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFC32;
    20
}
/// $048376: jsr $799de.l
pub fn i_048376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4837C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4837C);
    cpu.pc = 0x799DE;
    16
}
/// $04837C: move.w d0, $72(a6)
pub fn i_04837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48380;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048380: jsr $4932c(pc)
pub fn i_048380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48384;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48384);
    cpu.pc = 0x4932C;
    16
}
/// $048384: bcc.w $4838e
pub fn i_048384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48388;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4838E; return 10; }
    8
}
/// $048388: lea.l $47f12(pc), a1
pub fn i_048388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4838C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $04838C: move.l a1, (a6)
pub fn i_04838c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4838E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04838E: bra.w $48a44
pub fn i_04838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48392;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A44;
    10
}
// ===== Pow_RescueLeave_048392  ($048392, 64 B, pow/prisoner_0478fc.s)
/// $048392: lea.l $28e6ee.l, a0
pub fn i_048392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48398;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6EE;
    20
}
/// $048398: jsr $28cd4.l
pub fn i_048398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4839E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4839E);
    cpu.pc = 0x28CD4;
    16
}
/// $04839E: lea.l $483a4(pc), a1
pub fn i_04839e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x483A4;
    16
}
/// $0483A2: move.l a1, (a6)
pub fn i_0483a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0483A4: jsr $48fb0(pc)
pub fn i_0483a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x483A8);
    cpu.pc = 0x48FB0;
    16
}
/// $0483A8: jsr $28d70.l
pub fn i_0483a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x483AE);
    cpu.pc = 0x28D70;
    16
}
/// $0483AE: bcc.w $483c0
pub fn i_0483ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x483C0; return 10; }
    8
}
/// $0483B2: move.l #$28df42, $48(a6)
pub fn i_0483b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483BA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF42;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0483BA: lea.l $47a42(pc), a1
pub fn i_0483ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47A42;
    16
}
/// $0483BE: move.l a1, (a6)
pub fn i_0483be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0483C0: jsr $4932c(pc)
pub fn i_0483c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x483C4);
    cpu.pc = 0x4932C;
    16
}
/// $0483C4: bcc.w $483ce
pub fn i_0483c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x483CE; return 10; }
    8
}
/// $0483C8: lea.l $47f12(pc), a1
pub fn i_0483c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x47F12;
    16
}
/// $0483CC: move.l a1, (a6)
pub fn i_0483cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0483CE: bra.w $48a44
pub fn i_0483ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A44;
    10
}
// ===== Pow_SpawnFreeVariantA_0483d2  ($0483D2, 16 B, pow/prisoner_0478fc.s)
/// $0483D2: move.b #$1, $9b(a6)
pub fn i_0483d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0483D8: move.b #$1, $86(a6)
pub fn i_0483d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0483DE: bra.w $483e6
pub fn i_0483de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x483E6;
    10
}
// ===== Pow_SpawnFreeVariantB_0483e2  ($0483E2, 62 B, pow/prisoner_0478fc.s)
/// $0483E2: clr.b $86(a6)
pub fn i_0483e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483E6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0483E6: jsr $5e7c0.l
pub fn i_0483e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x483EC);
    cpu.pc = 0x5E7C0;
    16
}
/// $0483EC: jsr $48ea6(pc)
pub fn i_0483ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x483F0);
    cpu.pc = 0x48EA6;
    16
}
/// $0483F0: tst.b $98(a6)
pub fn i_0483f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483F4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0483F4: bne.w $48404
pub fn i_0483f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48404; return 10; }
    8
}
/// $0483F8: tst.b $9b(a6)
pub fn i_0483f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483FC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0483FC: bne.w $4857c
pub fn i_0483fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48400;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4857C; return 10; }
    8
}
/// $048400: bra.w $48710
pub fn i_048400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48404;
    cpu.ir = 0x6000;
    cpu.pc = 0x48710;
    10
}
/// $048404: tst.b $9c(a6)
pub fn i_048404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48408;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048408: beq.w $48418
pub fn i_048408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4840C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48418; return 10; }
    8
}
/// $04840C: move.b $9c(a6), d0
pub fn i_04840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48410;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $048410: andi.w #$ff, d0
pub fn i_048410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48414;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048414: move.w d0, $72(a6)
pub fn i_048414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48418;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048418: jsr $4932c(pc)
pub fn i_048418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4841C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4841C);
    cpu.pc = 0x4932C;
    16
}
/// $04841C: bcc.w $48492
pub fn i_04841c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48420;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48492; return 10; }
    8
}
// ===== Pow_FreeHurt_048420  ($048420, 54 B, pow/prisoner_0478fc.s)
/// $048420: move.w $28(a6), d0
pub fn i_048420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48424;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $048424: asr.w #$1, d0
pub fn i_048424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48426;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $048426: move.w d0, $28(a6)
pub fn i_048426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4842A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04842A: lea.l $28e75e.l, a0
pub fn i_04842a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48430;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E75E;
    20
}
/// $048430: jsr $28cd4.l
pub fn i_048430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48436);
    cpu.pc = 0x28CD4;
    16
}
/// $048436: lea.l $4843c(pc), a1
pub fn i_048436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4843A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4843C;
    16
}
/// $04843A: move.l a1, (a6)
pub fn i_04843a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4843C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04843C: jsr $27c8c.l
pub fn i_04843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48442);
    cpu.pc = 0x27C8C;
    16
}
/// $048442: bcc.w $4844c
pub fn i_048442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48446;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4844C; return 10; }
    8
}
/// $048446: lea.l $48456(pc), a1
pub fn i_048446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4844A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48456;
    16
}
/// $04844A: move.l a1, (a6)
pub fn i_04844a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4844C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04844C: jsr $28d70.l
pub fn i_04844c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48452);
    cpu.pc = 0x28D70;
    16
}
/// $048452: bra.w $489f8
pub fn i_048452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48456;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeGetUp_048456  ($048456, 60 B, pow/prisoner_0478fc.s)
/// $048456: clr.w $28(a6)
pub fn i_048456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4845A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04845A: lea.l $28e79c.l, a0
pub fn i_04845a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48460;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E79C;
    20
}
/// $048460: jsr $28cd4.l
pub fn i_048460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48466;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48466);
    cpu.pc = 0x28CD4;
    16
}
/// $048466: lea.l $4846c(pc), a1
pub fn i_048466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4846A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4846C;
    16
}
/// $04846A: move.l a1, (a6)
pub fn i_04846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4846C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04846C: jsr $48fb0(pc)
pub fn i_04846c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48470;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48470);
    cpu.pc = 0x48FB0;
    16
}
/// $048470: jsr $28d70.l
pub fn i_048470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48476);
    cpu.pc = 0x28D70;
    16
}
/// $048476: bcc.w $48480
pub fn i_048476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4847A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48480; return 10; }
    8
}
/// $04847A: lea.l $48492(pc), a1
pub fn i_04847a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4847E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48492;
    16
}
/// $04847E: move.l a1, (a6)
pub fn i_04847e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48480;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048480: jsr $4932c(pc)
pub fn i_048480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48484;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48484);
    cpu.pc = 0x4932C;
    16
}
/// $048484: bcc.w $4848e
pub fn i_048484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48488;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4848E; return 10; }
    8
}
/// $048488: lea.l $48420(pc), a1
pub fn i_048488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4848C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $04848C: move.l a1, (a6)
pub fn i_04848c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4848E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04848E: bra.w $489f8
pub fn i_04848e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48492;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeWalkOut_048492  ($048492, 112 B, pow/prisoner_0478fc.s)
/// $048492: lea.l $2bfb2e.l, a0
pub fn i_048492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48498;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFB2E;
    20
}
/// $048498: jsr $799de.l
pub fn i_048498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4849E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4849E);
    cpu.pc = 0x799DE;
    16
}
/// $04849E: move.w d0, $36(a6)
pub fn i_04849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0484A2: jsr $48f04(pc)
pub fn i_0484a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x484A6);
    cpu.pc = 0x48F04;
    16
}
/// $0484A6: lea.l $28e394.l, a0
pub fn i_0484a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E394;
    20
}
/// $0484AC: jsr $28cd4.l
pub fn i_0484ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x484B2);
    cpu.pc = 0x28CD4;
    16
}
/// $0484B2: lea.l $484b8(pc), a1
pub fn i_0484b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x484B8;
    16
}
/// $0484B6: move.l a1, (a6)
pub fn i_0484b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0484B8: jsr $48f2e(pc)
pub fn i_0484b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x484BC);
    cpu.pc = 0x48F2E;
    16
}
/// $0484BC: jsr $28d70.l
pub fn i_0484bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x484C2);
    cpu.pc = 0x28D70;
    16
}
/// $0484C2: move.b $98(a6), d0
pub fn i_0484c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484C6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0484C6: andi.w #$ff, d0
pub fn i_0484c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484CA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0484CA: lsl.w #$4, d0
pub fn i_0484ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484CC;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0484CC: btst.b #$0, $3a(a6)
pub fn i_0484cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484D2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0484D2: bne.w $484e2
pub fn i_0484d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x484E2; return 10; }
    8
}
/// $0484D6: cmp.w $22(a6), d0
pub fn i_0484d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484DA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0484DA: bgt.w $484ea
pub fn i_0484da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x484EA; return 10; }
    8
}
/// $0484DE: bra.w $484f0
pub fn i_0484de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x484F0;
    10
}
/// $0484E2: cmp.w $22(a6), d0
pub fn i_0484e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484E6;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0484E6: bgt.w $484f0
pub fn i_0484e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x484F0; return 10; }
    8
}
/// $0484EA: lea.l $48502(pc), a1
pub fn i_0484ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48502;
    16
}
/// $0484EE: move.l a1, (a6)
pub fn i_0484ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0484F0: jsr $4932c(pc)
pub fn i_0484f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x484F4);
    cpu.pc = 0x4932C;
    16
}
/// $0484F4: bcc.w $484fe
pub fn i_0484f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x484FE; return 10; }
    8
}
/// $0484F8: lea.l $48420(pc), a1
pub fn i_0484f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $0484FC: move.l a1, (a6)
pub fn i_0484fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0484FE: bra.w $489f8
pub fn i_0484fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48502;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeJumpOut_048502  ($048502, 122 B, pow/prisoner_0478fc.s)
/// $048502: tst.b $9b(a6)
pub fn i_048502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48506;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048506: beq.w $486d4
pub fn i_048506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4850A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x486D4; return 10; }
    8
}
/// $04850A: move.w #$fccd, d0
pub fn i_04850a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4850E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFCCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04850E: jsr $5dca4.l
pub fn i_04850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48514);
    cpu.pc = 0x5DCA4;
    16
}
/// $048514: move.w d0, $28(a6)
pub fn i_048514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48518;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048518: move.w #$ccb, $2a(a6)
pub fn i_048518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4851E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xCCB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04851E: move.w #$fae2, $2e(a6)
pub fn i_04851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48524;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFAE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048524: move.w #$0, $2c(a6)
pub fn i_048524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4852A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04852A: lea.l $28e656.l, a0
pub fn i_04852a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48530;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E656;
    20
}
/// $048530: jsr $28cd4.l
pub fn i_048530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48536);
    cpu.pc = 0x28CD4;
    16
}
/// $048536: lea.l $4853c(pc), a1
pub fn i_048536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4853A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4853C;
    16
}
/// $04853A: move.l a1, (a6)
pub fn i_04853a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4853C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04853C: jsr $27bc8.l
pub fn i_04853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48542);
    cpu.pc = 0x27BC8;
    16
}
/// $048542: bcc.w $4854a
pub fn i_048542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48546;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4854A; return 10; }
    8
}
/// $048546: clr.w $28(a6)
pub fn i_048546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4854A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04854A: jsr $28d70.l
pub fn i_04854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48550);
    cpu.pc = 0x28D70;
    16
}
/// $048550: bcc.w $48562
pub fn i_048550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48554;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48562; return 10; }
    8
}
/// $048554: tst.w $28(a6)
pub fn i_048554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48558;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $048558: bne.w $48562
pub fn i_048558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4855C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48562; return 10; }
    8
}
/// $04855C: lea.l $4857c(pc), a1
pub fn i_04855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48560;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4857C;
    16
}
/// $048560: move.l a1, (a6)
pub fn i_048560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48562;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048562: tst.b $86(a6)
pub fn i_048562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48566;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048566: beq.w $48578
pub fn i_048566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4856A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48578; return 10; }
    8
}
/// $04856A: jsr $4932c(pc)
pub fn i_04856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4856E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4856E);
    cpu.pc = 0x4932C;
    16
}
/// $04856E: bcc.w $48578
pub fn i_04856e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48572;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48578; return 10; }
    8
}
/// $048572: lea.l $48420(pc), a1
pub fn i_048572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48576;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048576: move.l a1, (a6)
pub fn i_048576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48578;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048578: bra.w $489f8
pub fn i_048578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4857C;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeThanksLoop_04857c  ($04857C, 108 B, pow/prisoner_0478fc.s)
/// $04857C: clr.w $28(a6)
pub fn i_04857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48580;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048580: move.l #$28e03e, $48(a6)
pub fn i_048580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48588;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28E03E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048588: lea.l $28e6b0.l, a0
pub fn i_048588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4858E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6B0;
    20
}
/// $04858E: jsr $28cd4.l
pub fn i_04858e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48594;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48594);
    cpu.pc = 0x28CD4;
    16
}
/// $048594: lea.l $4859a(pc), a1
pub fn i_048594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48598;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4859A;
    16
}
/// $048598: move.l a1, (a6)
pub fn i_048598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4859A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04859A: jsr $48fb0(pc)
pub fn i_04859a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4859E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4859E);
    cpu.pc = 0x48FB0;
    16
}
/// $04859E: jsr $28d70.l
pub fn i_04859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x485A4);
    cpu.pc = 0x28D70;
    16
}
/// $0485A4: jsr $492a4(pc)
pub fn i_0485a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x485A8);
    cpu.pc = 0x492A4;
    16
}
/// $0485A8: bcc.w $485b2
pub fn i_0485a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x485B2; return 10; }
    8
}
/// $0485AC: lea.l $4867a(pc), a1
pub fn i_0485ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4867A;
    16
}
/// $0485B0: move.l a1, (a6)
pub fn i_0485b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0485B2: cmpi.w #$18, $22(a6)
pub fn i_0485b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485B8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Word, false);
    16
}
/// $0485B8: blt.w $485d6
pub fn i_0485b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485BC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x485D6; return 10; }
    8
}
/// $0485BC: cmpi.w #$128, $22(a6)
pub fn i_0485bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485C2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x128, Size::Word, false);
    16
}
/// $0485C2: bgt.w $485d6
pub fn i_0485c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485C6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x485D6; return 10; }
    8
}
/// $0485C6: cmpi.w #$0, $72(a6)
pub fn i_0485c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485CC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0485CC: bgt.w $485d6
pub fn i_0485cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485D0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x485D6; return 10; }
    8
}
/// $0485D0: lea.l $485e8(pc), a1
pub fn i_0485d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x485E8;
    16
}
/// $0485D4: move.l a1, (a6)
pub fn i_0485d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0485D6: jsr $4932c(pc)
pub fn i_0485d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x485DA);
    cpu.pc = 0x4932C;
    16
}
/// $0485DA: bcc.w $485e4
pub fn i_0485da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x485E4; return 10; }
    8
}
/// $0485DE: lea.l $48420(pc), a1
pub fn i_0485de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $0485E2: move.l a1, (a6)
pub fn i_0485e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0485E4: bra.w $48a44
pub fn i_0485e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A44;
    10
}
// ===== Pow_FreeThanksInit_0485e8  ($0485E8, 20 B, pow/prisoner_0478fc.s)
/// $0485E8: clr.w $28(a6)
pub fn i_0485e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485EC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0485EC: lea.l $2bfd36.l, a0
pub fn i_0485ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFD36;
    20
}
/// $0485F2: jsr $799de.l
pub fn i_0485f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x485F8);
    cpu.pc = 0x799DE;
    16
}
/// $0485F8: move.b d0, $82(a6)
pub fn i_0485f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485FC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Pow_FreeThanks_0485fc  ($0485FC, 126 B, pow/prisoner_0478fc.s)
/// $0485FC: lea.l $2bfcb4.l, a0
pub fn i_0485fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48602;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFCB4;
    20
}
/// $048602: jsr $799de.l
pub fn i_048602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48608);
    cpu.pc = 0x799DE;
    16
}
/// $048608: move.w d0, $72(a6)
pub fn i_048608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4860C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04860C: move.l #$28e092, $48(a6)
pub fn i_04860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48614;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28E092;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048614: lea.l $28e6c8.l, a0
pub fn i_048614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4861A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6C8;
    20
}
/// $04861A: jsr $28cd4.l
pub fn i_04861a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48620);
    cpu.pc = 0x28CD4;
    16
}
/// $048620: lea.l $48626(pc), a1
pub fn i_048620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48624;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48626;
    16
}
/// $048624: move.l a1, (a6)
pub fn i_048624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48626;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048626: jsr $48fb0(pc)
pub fn i_048626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4862A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4862A);
    cpu.pc = 0x48FB0;
    16
}
/// $04862A: jsr $28d70.l
pub fn i_04862a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48630);
    cpu.pc = 0x28D70;
    16
}
/// $048630: bcc.w $48668
pub fn i_048630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48634;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48668; return 10; }
    8
}
/// $048634: cmpi.w #$0, $72(a6)
pub fn i_048634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4863A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04863A: bgt.w $48668
pub fn i_04863a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4863E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48668; return 10; }
    8
}
/// $04863E: lea.l $485fc(pc), a1
pub fn i_04863e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48642;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x485FC;
    16
}
/// $048642: move.l a1, (a6)
pub fn i_048642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48644;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048644: subq.b #$1, $82(a6)
pub fn i_048644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48648;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $048648: cmpi.b #$0, $82(a6)
pub fn i_048648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4864E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04864E: bgt.w $48668
pub fn i_04864e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48652;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48668; return 10; }
    8
}
/// $048652: lea.l $4857c(pc), a1
pub fn i_048652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48656;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4857C;
    16
}
/// $048656: move.l a1, (a6)
pub fn i_048656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48658;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048658: lea.l $2bfc32.l, a0
pub fn i_048658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4865E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFC32;
    20
}
/// $04865E: jsr $799de.l
pub fn i_04865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48664;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48664);
    cpu.pc = 0x799DE;
    16
}
/// $048664: move.w d0, $72(a6)
pub fn i_048664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48668;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048668: jsr $4932c(pc)
pub fn i_048668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4866C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4866C);
    cpu.pc = 0x4932C;
    16
}
/// $04866C: bcc.w $48676
pub fn i_04866c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48670;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48676; return 10; }
    8
}
/// $048670: lea.l $48420(pc), a1
pub fn i_048670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48674;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048674: move.l a1, (a6)
pub fn i_048674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48676;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048676: bra.w $48a44
pub fn i_048676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4867A;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A44;
    10
}
// ===== Pow_FreeLeave_04867a  ($04867A, 150 B, pow/prisoner_0478fc.s)
/// $04867A: clr.w $28(a6)
pub fn i_04867a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4867E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04867E: move.l #$28df42, $48(a6)
pub fn i_04867e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48686;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF42;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048686: lea.l $28e6ee.l, a0
pub fn i_048686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4868C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6EE;
    20
}
/// $04868C: jsr $28cd4.l
pub fn i_04868c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48692;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48692);
    cpu.pc = 0x28CD4;
    16
}
/// $048692: lea.l $48698(pc), a1
pub fn i_048692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48696;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48698;
    16
}
/// $048696: move.l a1, (a6)
pub fn i_048696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48698;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048698: jsr $48fb0(pc)
pub fn i_048698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4869C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4869C);
    cpu.pc = 0x48FB0;
    16
}
/// $04869C: jsr $28d70.l
pub fn i_04869c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486A2);
    cpu.pc = 0x28D70;
    16
}
/// $0486A2: bcc.w $486c2
pub fn i_0486a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x486C2; return 10; }
    8
}
/// $0486A6: jsr $58fe2.l
pub fn i_0486a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486AC);
    cpu.pc = 0x58FE2;
    16
}
/// $0486AC: lea.l $48ca4(pc), a1
pub fn i_0486ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $0486B0: jsr $4ae.l
pub fn i_0486b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486B6);
    cpu.pc = 0x4AE;
    16
}
/// $0486B6: jsr $5dd02.l
pub fn i_0486b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486BC);
    cpu.pc = 0x5DD02;
    16
}
/// $0486BC: addi.w #$18, $24(a0)
pub fn i_0486bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486C2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0486C2: jsr $4932c(pc)
pub fn i_0486c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x486C6);
    cpu.pc = 0x4932C;
    16
}
/// $0486C6: bcc.w $486d0
pub fn i_0486c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x486D0; return 10; }
    8
}
/// $0486CA: lea.l $48420(pc), a1
pub fn i_0486ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $0486CE: move.l a1, (a6)
pub fn i_0486ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0486D0: bra.w $48a44
pub fn i_0486d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A44;
    10
}
/// $0486D4: clr.w $28(a6)
pub fn i_0486d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486D8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0486D8: lea.l $28e4ee.l, a0
pub fn i_0486d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E4EE;
    20
}
/// $0486DE: jsr $28cd4.l
pub fn i_0486de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0486E4: lea.l $486ea(pc), a1
pub fn i_0486e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x486EA;
    16
}
/// $0486E8: move.l a1, (a6)
pub fn i_0486e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0486EA: jsr $48fb0(pc)
pub fn i_0486ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x486EE);
    cpu.pc = 0x48FB0;
    16
}
/// $0486EE: jsr $28d70.l
pub fn i_0486ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486F4);
    cpu.pc = 0x28D70;
    16
}
/// $0486F4: bcc.w $486fe
pub fn i_0486f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x486FE; return 10; }
    8
}
/// $0486F8: lea.l $48710(pc), a1
pub fn i_0486f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48710;
    16
}
/// $0486FC: move.l a1, (a6)
pub fn i_0486fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0486FE: jsr $4932c(pc)
pub fn i_0486fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48702;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48702);
    cpu.pc = 0x4932C;
    16
}
/// $048702: bcc.w $4870c
pub fn i_048702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48706;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4870C; return 10; }
    8
}
/// $048706: lea.l $48420(pc), a1
pub fn i_048706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4870A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $04870A: move.l a1, (a6)
pub fn i_04870a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4870C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04870C: bra.w $489f8
pub fn i_04870c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48710;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeStand_048710  ($048710, 88 B, pow/prisoner_0478fc.s)
/// $048710: clr.w $28(a6)
pub fn i_048710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48714;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048714: move.l #$28dfea, $48(a6)
pub fn i_048714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4871C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DFEA;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04871C: lea.l $28e536.l, a0
pub fn i_04871c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48722;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E536;
    20
}
/// $048722: jsr $28cd4.l
pub fn i_048722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48728);
    cpu.pc = 0x28CD4;
    16
}
/// $048728: lea.l $4872e(pc), a1
pub fn i_048728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4872C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4872E;
    16
}
/// $04872C: move.l a1, (a6)
pub fn i_04872c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4872E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04872E: jsr $48fb0(pc)
pub fn i_04872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48732;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48732);
    cpu.pc = 0x48FB0;
    16
}
/// $048732: jsr $28d70.l
pub fn i_048732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48738;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48738);
    cpu.pc = 0x28D70;
    16
}
/// $048738: jsr $492a4(pc)
pub fn i_048738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4873C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4873C);
    cpu.pc = 0x492A4;
    16
}
/// $04873C: bcc.w $48746
pub fn i_04873c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48740;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48746; return 10; }
    8
}
/// $048740: lea.l $4883e(pc), a1
pub fn i_048740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48744;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4883E;
    16
}
/// $048744: move.l a1, (a6)
pub fn i_048744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48746;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048746: cmpi.w #$0, $72(a6)
pub fn i_048746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4874C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04874C: bgt.w $48756
pub fn i_04874c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48750;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48756; return 10; }
    8
}
/// $048750: lea.l $48768(pc), a1
pub fn i_048750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48754;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48768;
    16
}
/// $048754: move.l a1, (a6)
pub fn i_048754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48756;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048756: jsr $4932c(pc)
pub fn i_048756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4875A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4875A);
    cpu.pc = 0x4932C;
    16
}
/// $04875A: bcc.w $48764
pub fn i_04875a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4875E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48764; return 10; }
    8
}
/// $04875E: lea.l $48420(pc), a1
pub fn i_04875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48762;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048762: move.l a1, (a6)
pub fn i_048762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48764;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048764: bra.w $489f8
pub fn i_048764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48768;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeSaluteInit_048768  ($048768, 44 B, pow/prisoner_0478fc.s)
/// $048768: clr.w $28(a6)
pub fn i_048768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4876C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04876C: move.l #$28df42, $48(a6)
pub fn i_04876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48774;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF42;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048774: lea.l $2bfd36.l, a0
pub fn i_048774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4877A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFD36;
    20
}
/// $04877A: jsr $799de.l
pub fn i_04877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48780);
    cpu.pc = 0x799DE;
    16
}
/// $048780: move.b d0, $82(a6)
pub fn i_048780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48784;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048784: lea.l $28e5b6.l, a0
pub fn i_048784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4878A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E5B6;
    20
}
/// $04878A: jsr $28cd4.l
pub fn i_04878a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48790);
    cpu.pc = 0x28CD4;
    16
}
/// $048790: bra.w $487a0
pub fn i_048790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48794;
    cpu.ir = 0x6000;
    cpu.pc = 0x487A0;
    10
}
// ===== Pow_FreeSalute_048794  ($048794, 102 B, pow/prisoner_0478fc.s)
/// $048794: lea.l $28e610.l, a0
pub fn i_048794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4879A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E610;
    20
}
/// $04879A: jsr $28cd4.l
pub fn i_04879a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x487A0);
    cpu.pc = 0x28CD4;
    16
}
/// $0487A0: lea.l $487a6(pc), a1
pub fn i_0487a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x487A6;
    16
}
/// $0487A4: move.l a1, (a6)
pub fn i_0487a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0487A6: jsr $48fb0(pc)
pub fn i_0487a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x487AA);
    cpu.pc = 0x48FB0;
    16
}
/// $0487AA: jsr $28d70.l
pub fn i_0487aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x487B0);
    cpu.pc = 0x28D70;
    16
}
/// $0487B0: bcc.w $487e8
pub fn i_0487b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x487E8; return 10; }
    8
}
/// $0487B4: cmpi.w #$0, $72(a6)
pub fn i_0487b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487BA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0487BA: bgt.w $487e8
pub fn i_0487ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487BE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x487E8; return 10; }
    8
}
/// $0487BE: lea.l $48794(pc), a1
pub fn i_0487be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48794;
    16
}
/// $0487C2: move.l a1, (a6)
pub fn i_0487c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0487C4: subq.b #$1, $82(a6)
pub fn i_0487c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487C8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0487C8: cmpi.b #$0, $82(a6)
pub fn i_0487c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487CE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0487CE: bgt.w $487e8
pub fn i_0487ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x487E8; return 10; }
    8
}
/// $0487D2: lea.l $487fa(pc), a1
pub fn i_0487d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x487FA;
    16
}
/// $0487D6: move.l a1, (a6)
pub fn i_0487d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0487D8: lea.l $2bfc32.l, a0
pub fn i_0487d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFC32;
    20
}
/// $0487DE: jsr $799de.l
pub fn i_0487de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x487E4);
    cpu.pc = 0x799DE;
    16
}
/// $0487E4: move.w d0, $72(a6)
pub fn i_0487e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487E8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0487E8: jsr $4932c(pc)
pub fn i_0487e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x487EC);
    cpu.pc = 0x4932C;
    16
}
/// $0487EC: bcc.w $487f6
pub fn i_0487ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x487F6; return 10; }
    8
}
/// $0487F0: lea.l $48420(pc), a1
pub fn i_0487f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $0487F4: move.l a1, (a6)
pub fn i_0487f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0487F6: bra.w $489f8
pub fn i_0487f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeIdle_0487fa  ($0487FA, 68 B, pow/prisoner_0478fc.s)
/// $0487FA: clr.w $28(a6)
pub fn i_0487fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487FE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0487FE: move.l #$28dfea, $48(a6)
pub fn i_0487fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48806;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DFEA;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048806: lea.l $28e632.l, a0
pub fn i_048806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4880C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E632;
    20
}
/// $04880C: jsr $28cd4.l
pub fn i_04880c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48812);
    cpu.pc = 0x28CD4;
    16
}
/// $048812: lea.l $48818(pc), a1
pub fn i_048812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48818;
    16
}
/// $048816: move.l a1, (a6)
pub fn i_048816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48818;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048818: jsr $48fb0(pc)
pub fn i_048818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4881C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4881C);
    cpu.pc = 0x48FB0;
    16
}
/// $04881C: jsr $28d70.l
pub fn i_04881c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48822);
    cpu.pc = 0x28D70;
    16
}
/// $048822: bcc.w $4882c
pub fn i_048822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48826;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4882C; return 10; }
    8
}
/// $048826: lea.l $48710(pc), a1
pub fn i_048826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4882A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48710;
    16
}
/// $04882A: move.l a1, (a6)
pub fn i_04882a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4882C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04882C: jsr $4932c(pc)
pub fn i_04882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48830;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48830);
    cpu.pc = 0x4932C;
    16
}
/// $048830: bcc.w $4883a
pub fn i_048830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48834;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4883A; return 10; }
    8
}
/// $048834: lea.l $48420(pc), a1
pub fn i_048834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48838;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048838: move.l a1, (a6)
pub fn i_048838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4883A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04883A: bra.w $489f8
pub fn i_04883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4883E;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeExit_04883e  ($04883E, 90 B, pow/prisoner_0478fc.s)
/// $04883E: clr.w $28(a6)
pub fn i_04883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48842;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048842: move.l #$28df42, $48(a6)
pub fn i_048842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4884A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF42;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04884A: lea.l $28e578.l, a0
pub fn i_04884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48850;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E578;
    20
}
/// $048850: jsr $28cd4.l
pub fn i_048850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48856);
    cpu.pc = 0x28CD4;
    16
}
/// $048856: lea.l $4885c(pc), a1
pub fn i_048856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4885A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4885C;
    16
}
/// $04885A: move.l a1, (a6)
pub fn i_04885a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4885C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04885C: jsr $48fb0(pc)
pub fn i_04885c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48860;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48860);
    cpu.pc = 0x48FB0;
    16
}
/// $048860: jsr $28d70.l
pub fn i_048860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48866);
    cpu.pc = 0x28D70;
    16
}
/// $048866: bcc.w $48886
pub fn i_048866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4886A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48886; return 10; }
    8
}
/// $04886A: jsr $58fe2.l
pub fn i_04886a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48870);
    cpu.pc = 0x58FE2;
    16
}
/// $048870: lea.l $48ca4(pc), a1
pub fn i_048870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $048874: jsr $4ae.l
pub fn i_048874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4887A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4887A);
    cpu.pc = 0x4AE;
    16
}
/// $04887A: jsr $5dd02.l
pub fn i_04887a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48880;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48880);
    cpu.pc = 0x5DD02;
    16
}
/// $048880: addi.w #$18, $24(a0)
pub fn i_048880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48886;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $048886: jsr $4932c(pc)
pub fn i_048886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4888A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4888A);
    cpu.pc = 0x4932C;
    16
}
/// $04888A: bcc.w $48894
pub fn i_04888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4888E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48894; return 10; }
    8
}
/// $04888E: lea.l $48420(pc), a1
pub fn i_04888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48892;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048892: move.l a1, (a6)
pub fn i_048892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48894;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048894: bra.w $489f8
pub fn i_048894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48898;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_SpawnTiedVariant_048898  ($048898, 130 B, pow/prisoner_0478fc.s)
/// $048898: clr.b $80(a6)
pub fn i_048898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4889C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04889C: bra.w $488a6
pub fn i_04889c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x488A6;
    10
}
/// $0488A0: move.b #$1, $80(a6)
pub fn i_0488a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488A6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0488A6: move.w #$38, d1
pub fn i_0488a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0488AA: jsr $236e.l
pub fn i_0488aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x488B0);
    cpu.pc = 0x236E;
    16
}
/// $0488B0: move.w #$1, $66(a6)
pub fn i_0488b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0488B6: move.w #$8000, d0
pub fn i_0488b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0488BA: jsr $28134.l
pub fn i_0488ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x488C0);
    cpu.pc = 0x28134;
    16
}
/// $0488C0: andi.w #$ffe3, $38(a6)
pub fn i_0488c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488C6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0488C6: ori.w #$18, $38(a6)
pub fn i_0488c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488CC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0488CC: move.l #$28df96, $48(a6)
pub fn i_0488cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488D4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF96;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0488D4: move.b $9e(a6), d0
pub fn i_0488d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0488D8: ext.w d0
pub fn i_0488d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488DA;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0488DA: lsl.w #$4, d0
pub fn i_0488da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488DC;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0488DC: move.w d0, $8c(a6)
pub fn i_0488dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488E0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0488E0: lea.l $2bff8e.l, a0
pub fn i_0488e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFF8E;
    20
}
/// $0488E6: jsr $799de.l
pub fn i_0488e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x488EC);
    cpu.pc = 0x799DE;
    16
}
/// $0488EC: move.w d0, $8e(a6)
pub fn i_0488ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488F0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0488F0: jsr $5e0d4.l
pub fn i_0488f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x488F6);
    cpu.pc = 0x5E0D4;
    16
}
/// $0488F6: move.l a0, $94(a6)
pub fn i_0488f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488FA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0488FA: clr.w $78(a6)
pub fn i_0488fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488FE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0488FE: move.b #$8, $7e(a6)
pub fn i_0488fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48904;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048904: move.b #$f0, $7d(a6)
pub fn i_048904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4890A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04890A: lea.l $48d0e(pc), a1
pub fn i_04890a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4890E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48D0E;
    16
}
/// $04890E: jsr $4ae.l
pub fn i_04890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48914;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48914);
    cpu.pc = 0x4AE;
    16
}
/// $048914: jsr $5dd02.l
pub fn i_048914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4891A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4891A);
    cpu.pc = 0x5DD02;
    16
}
// ===== Pow_TiedIdle_04891a  ($04891A, 84 B, pow/prisoner_0478fc.s)
/// $04891A: lea.l $2bff0c.l, a0
pub fn i_04891a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48920;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFF0C;
    20
}
/// $048920: jsr $799de.l
pub fn i_048920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48926;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48926);
    cpu.pc = 0x799DE;
    16
}
/// $048926: move.w d0, $72(a6)
pub fn i_048926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4892A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04892A: move.b #$0, $83(a6)
pub fn i_04892a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48930;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048930: lea.l $28ed0a.l, a0
pub fn i_048930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48936;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28ED0A;
    20
}
/// $048936: jsr $28cd4.l
pub fn i_048936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4893C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4893C);
    cpu.pc = 0x28CD4;
    16
}
/// $04893C: lea.l $48942(pc), a1
pub fn i_04893c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48940;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48942;
    16
}
/// $048940: move.l a1, (a6)
pub fn i_048940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48942;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048942: jsr $48f54(pc)
pub fn i_048942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48946;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48946);
    cpu.pc = 0x48F54;
    16
}
/// $048946: bcc.w $48950
pub fn i_048946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4894A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48950; return 10; }
    8
}
/// $04894A: lea.l $489c6(pc), a1
pub fn i_04894a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4894E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x489C6;
    16
}
/// $04894E: move.l a1, (a6)
pub fn i_04894e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048950: jsr $28d70.l
pub fn i_048950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48956);
    cpu.pc = 0x28D70;
    16
}
/// $048956: jsr $49054(pc)
pub fn i_048956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4895A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4895A);
    cpu.pc = 0x49054;
    16
}
/// $04895A: bcc.w $4896a
pub fn i_04895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4895E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4896A; return 10; }
    8
}
/// $04895E: move.b #$1, $83(a6)
pub fn i_04895e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48964;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048964: lea.l $4896e(pc), a1
pub fn i_048964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48968;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4896E;
    16
}
/// $048968: move.l a1, (a6)
pub fn i_048968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4896A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04896A: bra.w $48a90
pub fn i_04896a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4896E;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A90;
    10
}
// ===== Pow_TiedStruggle_04896e  ($04896E, 88 B, pow/prisoner_0478fc.s)
/// $04896E: move.b #$1, $83(a6)
pub fn i_04896e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48974;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048974: jsr $493e4(pc)
pub fn i_048974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48978;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48978);
    cpu.pc = 0x493E4;
    16
}
/// $048978: move.w $78(a6), d0
pub fn i_048978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4897C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04897C: lsr.w #$1, d0
pub fn i_04897c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4897E;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04897E: andi.w #$f, d0
pub fn i_04897e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48982;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048982: movea.l #$28e24a, a0
pub fn i_048982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48988;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28E24A;
    cpu.a[0] = v1;
    4
}
/// $048988: lsl.w #$2, d0
pub fn i_048988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4898A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04898A: movea.l (a0, d0.w), a0
pub fn i_04898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4898E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04898E: cmpa.l #$ffffffff, a0
pub fn i_04898e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48994;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $048994: beq.w $4899e
pub fn i_048994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48998;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4899E; return 10; }
    8
}
/// $048998: jsr $28cd4.l
pub fn i_048998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4899E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4899E);
    cpu.pc = 0x28CD4;
    16
}
/// $04899E: lea.l $489a4(pc), a1
pub fn i_04899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x489A4;
    16
}
/// $0489A2: move.l a1, (a6)
pub fn i_0489a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489A4: jsr $48f54(pc)
pub fn i_0489a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x489A8);
    cpu.pc = 0x48F54;
    16
}
/// $0489A8: bcc.w $489b2
pub fn i_0489a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x489B2; return 10; }
    8
}
/// $0489AC: lea.l $489c6(pc), a1
pub fn i_0489ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x489C6;
    16
}
/// $0489B0: move.l a1, (a6)
pub fn i_0489b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489B2: jsr $28d70.l
pub fn i_0489b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x489B8);
    cpu.pc = 0x28D70;
    16
}
/// $0489B8: bcc.w $489c2
pub fn i_0489b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x489C2; return 10; }
    8
}
/// $0489BC: lea.l $4891a(pc), a1
pub fn i_0489bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4891A;
    16
}
/// $0489C0: move.l a1, (a6)
pub fn i_0489c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489C2: bra.w $48a90
pub fn i_0489c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A90;
    10
}
// ===== Pow_TiedFreed_0489c6  ($0489C6, 118 B, pow/prisoner_0478fc.s)
/// $0489C6: move.b #$3, $83(a6)
pub fn i_0489c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0489CC: lea.l $28ef0c.l, a0
pub fn i_0489cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28EF0C;
    20
}
/// $0489D2: jsr $28cd4.l
pub fn i_0489d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x489D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0489D8: lea.l $489de(pc), a1
pub fn i_0489d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x489DE;
    16
}
/// $0489DC: move.l a1, (a6)
pub fn i_0489dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489DE: jsr $2783a.l
pub fn i_0489de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x489E4);
    cpu.pc = 0x2783A;
    16
}
/// $0489E4: jsr $28d70.l
pub fn i_0489e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x489EA);
    cpu.pc = 0x28D70;
    16
}
/// $0489EA: bcc.w $489f4
pub fn i_0489ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x489F4; return 10; }
    8
}
/// $0489EE: lea.l $4797c(pc), a1
pub fn i_0489ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4797C;
    16
}
/// $0489F2: move.l a1, (a6)
pub fn i_0489f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489F4: bra.w $48a90
pub fn i_0489f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A90;
    10
}
/// $0489F8: jsr $4936e(pc)
pub fn i_0489f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x489FC);
    cpu.pc = 0x4936E;
    16
}
/// $0489FC: subq.w #$1, $72(a6)
pub fn i_0489fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A00;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048A00: jsr $2870a.l
pub fn i_048a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A06);
    cpu.pc = 0x2870A;
    16
}
/// $048A06: bcc.w $48a26
pub fn i_048a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48A26; return 10; }
    8
}
/// $048A0A: lea.l $48ca4(pc), a1
pub fn i_048a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $048A0E: jsr $4ae.l
pub fn i_048a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A14);
    cpu.pc = 0x4AE;
    16
}
/// $048A14: jsr $5dd02.l
pub fn i_048a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A1A);
    cpu.pc = 0x5DD02;
    16
}
/// $048A1A: addi.w #$18, $24(a0)
pub fn i_048a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A20;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $048A20: jsr $49fd0.l
pub fn i_048a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A26);
    cpu.pc = 0x49FD0;
    16
}
/// $048A26: movea.l #$ffffffff, a0
pub fn i_048a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A2C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $048A2C: lea.l $28e196.l, a0
pub fn i_048a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E196;
    20
}
/// $048A32: jsr $5dd5c.l
pub fn i_048a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A38);
    cpu.pc = 0x5DD5C;
    16
}
/// $048A38: bcc.w $48a42
pub fn i_048a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48A42; return 10; }
    8
}
// ===== SetTaskHandler_048a3c  ($048A3C, 8 B, kernel/task_handlers.c)
/// $048A3C: lea.l $48b1e(pc), a1
pub fn i_048a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B1E;
    16
}
/// $048A40: move.l a1, (a6)
pub fn i_048a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048A42: rts
pub fn i_048a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_FreeStateTail_048a44  ($048A44, 68 B, pow/helpers_048a44.s)
/// $048A44: jsr $4936e(pc)
pub fn i_048a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48A48);
    cpu.pc = 0x4936E;
    16
}
/// $048A48: subq.w #$1, $72(a6)
pub fn i_048a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A4C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048A4C: jsr $2870a.l
pub fn i_048a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A52);
    cpu.pc = 0x2870A;
    16
}
/// $048A52: bcc.w $48a72
pub fn i_048a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48A72; return 10; }
    8
}
/// $048A56: lea.l $48ca4(pc), a1
pub fn i_048a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $048A5A: jsr $4ae.l
pub fn i_048a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A60);
    cpu.pc = 0x4AE;
    16
}
/// $048A60: jsr $5dd02.l
pub fn i_048a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A66);
    cpu.pc = 0x5DD02;
    16
}
/// $048A66: addi.w #$10, $24(a0)
pub fn i_048a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A6C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $048A6C: jsr $49ff2.l
pub fn i_048a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A72);
    cpu.pc = 0x49FF2;
    16
}
/// $048A72: movea.l #$ffffffff, a0
pub fn i_048a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A78;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $048A78: lea.l $28e196.l, a0
pub fn i_048a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E196;
    20
}
/// $048A7E: jsr $5dd5c.l
pub fn i_048a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A84);
    cpu.pc = 0x5DD5C;
    16
}
/// $048A84: bcc.w $48a8e
pub fn i_048a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48A8E; return 10; }
    8
}
// ===== SetTaskHandler_048a88  ($048A88, 8 B, kernel/task_handlers.c)
/// $048A88: lea.l $48b1e(pc), a1
pub fn i_048a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B1E;
    16
}
/// $048A8C: move.l a1, (a6)
pub fn i_048a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048A8E: rts
pub fn i_048a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TiedStateTail_048a90  ($048A90, 134 B, pow/helpers_048a44.s)
/// $048A90: jsr $4936e(pc)
pub fn i_048a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48A94);
    cpu.pc = 0x4936E;
    16
}
/// $048A94: subq.w #$1, $72(a6)
pub fn i_048a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A98;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048A98: jsr $2870a.l
pub fn i_048a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A9E);
    cpu.pc = 0x2870A;
    16
}
/// $048A9E: bcc.w $48ad2
pub fn i_048a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48AD2; return 10; }
    8
}
/// $048AA2: lea.l $48ca4(pc), a1
pub fn i_048aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $048AA6: jsr $4ae.l
pub fn i_048aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48AAC);
    cpu.pc = 0x4AE;
    16
}
/// $048AAC: jsr $5dd02.l
pub fn i_048aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48AB2);
    cpu.pc = 0x5DD02;
    16
}
/// $048AB2: addi.w #$18, $24(a0)
pub fn i_048ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AB8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $048AB8: movea.l $50(a6), a1
pub fn i_048ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48ABC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $048ABC: move.w $28(a1), d0
pub fn i_048abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AC0;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $048AC0: asr.w #$4, d0
pub fn i_048ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AC2;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $048AC2: move.w d0, $28(a0)
pub fn i_048ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AC6;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048AC6: jsr $49fd0.l
pub fn i_048ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48ACC);
    cpu.pc = 0x49FD0;
    16
}
/// $048ACC: move.b #$2, $83(a6)
pub fn i_048acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AD2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048AD2: cmpi.b #$0, $9e(a6)
pub fn i_048ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AD8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $048AD8: bge.w $48af0
pub fn i_048ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48ADC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x48AF0; return 10; }
    8
}
/// $048ADC: cmpi.w #$ffe0, $22(a6)
pub fn i_048adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AE2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $048AE2: bgt.w $48aec
pub fn i_048ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AE6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48AEC; return 10; }
    8
}
/// $048AE6: lea.l $48b1e(pc), a1
pub fn i_048ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B1E;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x047112, i_047112),
    (0x047118, i_047118),
    (0x04711C, i_04711c),
    (0x04711E, i_04711e),
    (0x047122, i_047122),
    (0x047124, i_047124),
    (0x047128, i_047128),
    (0x04712E, i_04712e),
    (0x047132, i_047132),
    (0x047138, i_047138),
    (0x04713E, i_04713e),
    (0x047142, i_047142),
    (0x047144, i_047144),
    (0x047146, i_047146),
    (0x04714A, i_04714a),
    (0x04714E, i_04714e),
    (0x047150, i_047150),
    (0x047152, i_047152),
    (0x047156, i_047156),
    (0x04715A, i_04715a),
    (0x04715E, i_04715e),
    (0x047162, i_047162),
    (0x047168, i_047168),
    (0x04716E, i_04716e),
    (0x047172, i_047172),
    (0x047174, i_047174),
    (0x047178, i_047178),
    (0x04717E, i_04717e),
    (0x047182, i_047182),
    (0x047186, i_047186),
    (0x047188, i_047188),
    (0x04718A, i_04718a),
    (0x04718E, i_04718e),
    (0x047192, i_047192),
    (0x047196, i_047196),
    (0x04719A, i_04719a),
    (0x0471A0, i_0471a0),
    (0x0471A4, i_0471a4),
    (0x0471A6, i_0471a6),
    (0x0471AA, i_0471aa),
    (0x0471B0, i_0471b0),
    (0x0471B4, i_0471b4),
    (0x0471B8, i_0471b8),
    (0x0471BE, i_0471be),
    (0x0471C2, i_0471c2),
    (0x0471C8, i_0471c8),
    (0x0471CC, i_0471cc),
    (0x0471CE, i_0471ce),
    (0x0471D2, i_0471d2),
    (0x0471D6, i_0471d6),
    (0x0471D8, i_0471d8),
    (0x0471DA, i_0471da),
    (0x0471DE, i_0471de),
    (0x0471E2, i_0471e2),
    (0x0471E6, i_0471e6),
    (0x0471EA, i_0471ea),
    (0x0471EE, i_0471ee),
    (0x0471F2, i_0471f2),
    (0x0471F8, i_0471f8),
    (0x0471FC, i_0471fc),
    (0x047200, i_047200),
    (0x047204, i_047204),
    (0x04720A, i_04720a),
    (0x04720C, i_04720c),
    (0x047210, i_047210),
    (0x047216, i_047216),
    (0x047218, i_047218),
    (0x04721C, i_04721c),
    (0x047220, i_047220),
    (0x047226, i_047226),
    (0x04722C, i_04722c),
    (0x047230, i_047230),
    (0x047236, i_047236),
    (0x04723C, i_04723c),
    (0x047242, i_047242),
    (0x047246, i_047246),
    (0x047248, i_047248),
    (0x04724C, i_04724c),
    (0x047250, i_047250),
    (0x047254, i_047254),
    (0x047258, i_047258),
    (0x04725C, i_04725c),
    (0x047260, i_047260),
    (0x047266, i_047266),
    (0x04726A, i_04726a),
    (0x047270, i_047270),
    (0x047274, i_047274),
    (0x047276, i_047276),
    (0x047278, i_047278),
    (0x04727C, i_04727c),
    (0x047282, i_047282),
    (0x047286, i_047286),
    (0x04728A, i_04728a),
    (0x04728C, i_04728c),
    (0x04728E, i_04728e),
    (0x047292, i_047292),
    (0x047296, i_047296),
    (0x047298, i_047298),
    (0x04729A, i_04729a),
    (0x04729E, i_04729e),
    (0x0472A2, i_0472a2),
    (0x0472A6, i_0472a6),
    (0x0472AA, i_0472aa),
    (0x0472B0, i_0472b0),
    (0x0472B6, i_0472b6),
    (0x0472BA, i_0472ba),
    (0x0472BC, i_0472bc),
    (0x0472C0, i_0472c0),
    (0x0472C6, i_0472c6),
    (0x0472CA, i_0472ca),
    (0x0472CE, i_0472ce),
    (0x0472D0, i_0472d0),
    (0x0472D2, i_0472d2),
    (0x0472D6, i_0472d6),
    (0x0472DA, i_0472da),
    (0x0472DE, i_0472de),
    (0x0472E2, i_0472e2),
    (0x0472E8, i_0472e8),
    (0x0472EC, i_0472ec),
    (0x0472EE, i_0472ee),
    (0x0472F2, i_0472f2),
    (0x0472F8, i_0472f8),
    (0x0472FC, i_0472fc),
    (0x047300, i_047300),
    (0x047306, i_047306),
    (0x04730A, i_04730a),
    (0x04730E, i_04730e),
    (0x047310, i_047310),
    (0x047314, i_047314),
    (0x047318, i_047318),
    (0x04731A, i_04731a),
    (0x04731C, i_04731c),
    (0x047322, i_047322),
    (0x047326, i_047326),
    (0x047328, i_047328),
    (0x04732C, i_04732c),
    (0x047330, i_047330),
    (0x047334, i_047334),
    (0x047338, i_047338),
    (0x04733C, i_04733c),
    (0x047340, i_047340),
    (0x047346, i_047346),
    (0x04734A, i_04734a),
    (0x04734E, i_04734e),
    (0x047354, i_047354),
    (0x04735A, i_04735a),
    (0x04735E, i_04735e),
    (0x047360, i_047360),
    (0x047362, i_047362),
    (0x047366, i_047366),
    (0x04736C, i_04736c),
    (0x047370, i_047370),
    (0x047376, i_047376),
    (0x04737C, i_04737c),
    (0x04737E, i_04737e),
    (0x047384, i_047384),
    (0x047388, i_047388),
    (0x04738A, i_04738a),
    (0x047390, i_047390),
    (0x047394, i_047394),
    (0x047398, i_047398),
    (0x04739A, i_04739a),
    (0x0473A0, i_0473a0),
    (0x0473A4, i_0473a4),
    (0x0473A6, i_0473a6),
    (0x0473AA, i_0473aa),
    (0x0473AE, i_0473ae),
    (0x0473B4, i_0473b4),
    (0x0473BC, i_0473bc),
    (0x0473C0, i_0473c0),
    (0x0473C4, i_0473c4),
    (0x0473C8, i_0473c8),
    (0x0473CE, i_0473ce),
    (0x0473D6, i_0473d6),
    (0x0473DA, i_0473da),
    (0x0473DE, i_0473de),
    (0x0473E4, i_0473e4),
    (0x0473EC, i_0473ec),
    (0x0473F0, i_0473f0),
    (0x0473F4, i_0473f4),
    (0x0473F8, i_0473f8),
    (0x0473FE, i_0473fe),
    (0x047400, i_047400),
    (0x047406, i_047406),
    (0x04740A, i_04740a),
    (0x04740C, i_04740c),
    (0x047412, i_047412),
    (0x047416, i_047416),
    (0x04741A, i_04741a),
    (0x04741C, i_04741c),
    (0x047422, i_047422),
    (0x047426, i_047426),
    (0x047428, i_047428),
    (0x04742C, i_04742c),
    (0x047430, i_047430),
    (0x047436, i_047436),
    (0x04743E, i_04743e),
    (0x047442, i_047442),
    (0x047446, i_047446),
    (0x04744A, i_04744a),
    (0x047450, i_047450),
    (0x047458, i_047458),
    (0x04745C, i_04745c),
    (0x047460, i_047460),
    (0x047466, i_047466),
    (0x04746E, i_04746e),
    (0x047472, i_047472),
    (0x047476, i_047476),
    (0x04747A, i_04747a),
    (0x047480, i_047480),
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
    (0x04764E, i_04764e),
    (0x047654, i_047654),
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
    (0x047676, i_047676),
    (0x04767A, i_04767a),
    (0x04767E, i_04767e),
    (0x047680, i_047680),
    (0x047682, i_047682),
    (0x047688, i_047688),
    (0x04768A, i_04768a),
    (0x04768E, i_04768e),
    (0x047692, i_047692),
    (0x047696, i_047696),
    (0x04769A, i_04769a),
    (0x0476A0, i_0476a0),
    (0x0476A2, i_0476a2),
    (0x0476A6, i_0476a6),
    (0x0476A8, i_0476a8),
    (0x0476AC, i_0476ac),
    (0x0476AE, i_0476ae),
    (0x0476B0, i_0476b0),
    (0x0476B8, i_0476b8),
    (0x0476BC, i_0476bc),
    (0x0476BE, i_0476be),
    (0x0476C6, i_0476c6),
    (0x0476CA, i_0476ca),
    (0x0476CE, i_0476ce),
    (0x0476D6, i_0476d6),
    (0x0476DA, i_0476da),
    (0x0476DC, i_0476dc),
    (0x0476E4, i_0476e4),
    (0x0476E8, i_0476e8),
    (0x0476EA, i_0476ea),
    (0x0476EE, i_0476ee),
    (0x0476F0, i_0476f0),
    (0x0476F8, i_0476f8),
    (0x0476FC, i_0476fc),
    (0x0476FE, i_0476fe),
    (0x047706, i_047706),
    (0x04770A, i_04770a),
    (0x04770E, i_04770e),
    (0x047716, i_047716),
    (0x04771A, i_04771a),
    (0x04771C, i_04771c),
    (0x047724, i_047724),
    (0x047728, i_047728),
    (0x04772A, i_04772a),
    (0x04772E, i_04772e),
    (0x047732, i_047732),
    (0x047736, i_047736),
    (0x04773A, i_04773a),
    (0x047740, i_047740),
    (0x047742, i_047742),
    (0x047746, i_047746),
    (0x047748, i_047748),
    (0x04774C, i_04774c),
    (0x047752, i_047752),
    (0x047756, i_047756),
    (0x047758, i_047758),
    (0x04775A, i_04775a),
    (0x047762, i_047762),
    (0x047766, i_047766),
    (0x047768, i_047768),
    (0x047770, i_047770),
    (0x047774, i_047774),
    (0x047778, i_047778),
    (0x047780, i_047780),
    (0x047784, i_047784),
    (0x047786, i_047786),
    (0x04778E, i_04778e),
    (0x047792, i_047792),
    (0x047794, i_047794),
    (0x047798, i_047798),
    (0x04779A, i_04779a),
    (0x0477A2, i_0477a2),
    (0x0477A6, i_0477a6),
    (0x0477A8, i_0477a8),
    (0x0477B0, i_0477b0),
    (0x0477B4, i_0477b4),
    (0x0477B8, i_0477b8),
    (0x0477C0, i_0477c0),
    (0x0477C4, i_0477c4),
    (0x0477C6, i_0477c6),
    (0x0477CE, i_0477ce),
    (0x0477D2, i_0477d2),
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
    (0x047888, i_047888),
    (0x04788A, i_04788a),
    (0x04788C, i_04788c),
    (0x04788E, i_04788e),
    (0x047890, i_047890),
    (0x047894, i_047894),
    (0x047898, i_047898),
    (0x04789A, i_04789a),
    (0x04789C, i_04789c),
    (0x0478A0, i_0478a0),
    (0x0478A2, i_0478a2),
    (0x0478A8, i_0478a8),
    (0x0478AA, i_0478aa),
    (0x0478AC, i_0478ac),
    (0x0478E0, i_0478e0),
    (0x0478E4, i_0478e4),
    (0x0478E8, i_0478e8),
    (0x0478EC, i_0478ec),
    (0x0478F0, i_0478f0),
    (0x0478F4, i_0478f4),
    (0x0478F6, i_0478f6),
    (0x0478FA, i_0478fa),
    (0x04791C, i_04791c),
    (0x047922, i_047922),
    (0x047928, i_047928),
    (0x04797C, i_04797c),
    (0x047982, i_047982),
    (0x047988, i_047988),
    (0x04798E, i_04798e),
    (0x047992, i_047992),
    (0x047998, i_047998),
    (0x04799C, i_04799c),
    (0x0479A0, i_0479a0),
    (0x0479A4, i_0479a4),
    (0x0479AA, i_0479aa),
    (0x0479AE, i_0479ae),
    (0x0479B2, i_0479b2),
    (0x0479B6, i_0479b6),
    (0x047A42, i_047a42),
    (0x047A48, i_047a48),
    (0x047A4C, i_047a4c),
    (0x047A50, i_047a50),
    (0x047A54, i_047a54),
    (0x047A58, i_047a58),
    (0x047A5C, i_047a5c),
    (0x047A62, i_047a62),
    (0x047A68, i_047a68),
    (0x047A6C, i_047a6c),
    (0x047A6E, i_047a6e),
    (0x047A72, i_047a72),
    (0x047A78, i_047a78),
    (0x047A7C, i_047a7c),
    (0x047A80, i_047a80),
    (0x047A84, i_047a84),
    (0x047A8A, i_047a8a),
    (0x047A8E, i_047a8e),
    (0x047A92, i_047a92),
    (0x047A98, i_047a98),
    (0x047A9E, i_047a9e),
    (0x047AA4, i_047aa4),
    (0x047AA8, i_047aa8),
    (0x047AAC, i_047aac),
    (0x047AB0, i_047ab0),
    (0x047AB2, i_047ab2),
    (0x047AB6, i_047ab6),
    (0x047ABA, i_047aba),
    (0x047ABE, i_047abe),
    (0x047AC0, i_047ac0),
    (0x047AC4, i_047ac4),
    (0x047AC8, i_047ac8),
    (0x047ACC, i_047acc),
    (0x047ACE, i_047ace),
    (0x047AD2, i_047ad2),
    (0x047AD6, i_047ad6),
    (0x047ADA, i_047ada),
    (0x047ADC, i_047adc),
    (0x047AE0, i_047ae0),
    (0x047AE6, i_047ae6),
    (0x047AEA, i_047aea),
    (0x047AF0, i_047af0),
    (0x047AF4, i_047af4),
    (0x047AFA, i_047afa),
    (0x047AFE, i_047afe),
    (0x047B02, i_047b02),
    (0x047B06, i_047b06),
    (0x047B0A, i_047b0a),
    (0x047B0C, i_047b0c),
    (0x047B10, i_047b10),
    (0x047B14, i_047b14),
    (0x047B18, i_047b18),
    (0x047B1A, i_047b1a),
    (0x047B1E, i_047b1e),
    (0x047B24, i_047b24),
    (0x047B2A, i_047b2a),
    (0x047B30, i_047b30),
    (0x047B34, i_047b34),
    (0x047B38, i_047b38),
    (0x047B3E, i_047b3e),
    (0x047B44, i_047b44),
    (0x047B48, i_047b48),
    (0x047B4A, i_047b4a),
    (0x047B4E, i_047b4e),
    (0x047B54, i_047b54),
    (0x047B58, i_047b58),
    (0x047B5C, i_047b5c),
    (0x047B60, i_047b60),
    (0x047B62, i_047b62),
    (0x047B66, i_047b66),
    (0x047B6A, i_047b6a),
    (0x047B6E, i_047b6e),
    (0x047B70, i_047b70),
    (0x047B74, i_047b74),
    (0x047B78, i_047b78),
    (0x047B7C, i_047b7c),
    (0x047B7E, i_047b7e),
    (0x047B82, i_047b82),
    (0x047B86, i_047b86),
    (0x047B8A, i_047b8a),
    (0x047B8C, i_047b8c),
    (0x047B90, i_047b90),
    (0x047B94, i_047b94),
    (0x047B98, i_047b98),
    (0x047B9A, i_047b9a),
    (0x047B9E, i_047b9e),
    (0x047BA2, i_047ba2),
    (0x047BA6, i_047ba6),
    (0x047BA8, i_047ba8),
    (0x047BAC, i_047bac),
    (0x047BB2, i_047bb2),
    (0x047BB6, i_047bb6),
    (0x047BBA, i_047bba),
    (0x047BC0, i_047bc0),
    (0x047BC6, i_047bc6),
    (0x047BCA, i_047bca),
    (0x047BCE, i_047bce),
    (0x047BD4, i_047bd4),
    (0x047BDA, i_047bda),
    (0x047BDE, i_047bde),
    (0x047BE0, i_047be0),
    (0x047BE4, i_047be4),
    (0x047BEA, i_047bea),
    (0x047BEE, i_047bee),
    (0x047BF2, i_047bf2),
    (0x047BF6, i_047bf6),
    (0x047BF8, i_047bf8),
    (0x047BFC, i_047bfc),
    (0x047C00, i_047c00),
    (0x047C04, i_047c04),
    (0x047C06, i_047c06),
    (0x047C0A, i_047c0a),
    (0x047C0E, i_047c0e),
    (0x047C12, i_047c12),
    (0x047C14, i_047c14),
    (0x047C18, i_047c18),
    (0x047C1C, i_047c1c),
    (0x047C20, i_047c20),
    (0x047C22, i_047c22),
    (0x047C26, i_047c26),
    (0x047C2A, i_047c2a),
    (0x047C2E, i_047c2e),
    (0x047C30, i_047c30),
    (0x047C34, i_047c34),
    (0x047C38, i_047c38),
    (0x047C3C, i_047c3c),
    (0x047C3E, i_047c3e),
    (0x047C42, i_047c42),
    (0x047C46, i_047c46),
    (0x047C4A, i_047c4a),
    (0x047C4C, i_047c4c),
    (0x047C50, i_047c50),
    (0x047C54, i_047c54),
    (0x047C58, i_047c58),
    (0x047C5C, i_047c5c),
    (0x047C60, i_047c60),
    (0x047C64, i_047c64),
    (0x047C6A, i_047c6a),
    (0x047C70, i_047c70),
    (0x047C74, i_047c74),
    (0x047C78, i_047c78),
    (0x047C7C, i_047c7c),
    (0x047C82, i_047c82),
    (0x047C88, i_047c88),
    (0x047C8C, i_047c8c),
    (0x047C8E, i_047c8e),
    (0x047C92, i_047c92),
    (0x047C98, i_047c98),
    (0x047C9C, i_047c9c),
    (0x047CA0, i_047ca0),
    (0x047CA4, i_047ca4),
    (0x047CA6, i_047ca6),
    (0x047CAA, i_047caa),
    (0x047CAE, i_047cae),
    (0x047CB2, i_047cb2),
    (0x047CB4, i_047cb4),
    (0x047CB8, i_047cb8),
    (0x047CBC, i_047cbc),
    (0x047CC0, i_047cc0),
    (0x047CC2, i_047cc2),
    (0x047CC6, i_047cc6),
    (0x047CCA, i_047cca),
    (0x047CCE, i_047cce),
    (0x047CD0, i_047cd0),
    (0x047CD4, i_047cd4),
    (0x047CD8, i_047cd8),
    (0x047CDC, i_047cdc),
    (0x047CDE, i_047cde),
    (0x047CE2, i_047ce2),
    (0x047CE6, i_047ce6),
    (0x047CEA, i_047cea),
    (0x047CEC, i_047cec),
    (0x047CF0, i_047cf0),
    (0x047CF4, i_047cf4),
    (0x047CF8, i_047cf8),
    (0x047CFA, i_047cfa),
    (0x047CFE, i_047cfe),
    (0x047D02, i_047d02),
    (0x047D06, i_047d06),
    (0x047D0A, i_047d0a),
    (0x047D0E, i_047d0e),
    (0x047D12, i_047d12),
    (0x047D18, i_047d18),
    (0x047D1E, i_047d1e),
    (0x047D22, i_047d22),
    (0x047D28, i_047d28),
    (0x047D2E, i_047d2e),
    (0x047D32, i_047d32),
    (0x047D36, i_047d36),
    (0x047D3A, i_047d3a),
    (0x047D40, i_047d40),
    (0x047D46, i_047d46),
    (0x047D4A, i_047d4a),
    (0x047D4C, i_047d4c),
    (0x047D50, i_047d50),
    (0x047D56, i_047d56),
    (0x047D5A, i_047d5a),
    (0x047D5E, i_047d5e),
    (0x047D62, i_047d62),
    (0x047D64, i_047d64),
    (0x047D68, i_047d68),
    (0x047D6C, i_047d6c),
    (0x047D70, i_047d70),
    (0x047D72, i_047d72),
    (0x047D76, i_047d76),
    (0x047D7A, i_047d7a),
    (0x047D7E, i_047d7e),
    (0x047D80, i_047d80),
    (0x047D84, i_047d84),
    (0x047D88, i_047d88),
    (0x047D8C, i_047d8c),
    (0x047D8E, i_047d8e),
    (0x047D92, i_047d92),
    (0x047D96, i_047d96),
    (0x047D9A, i_047d9a),
    (0x047D9C, i_047d9c),
    (0x047DA0, i_047da0),
    (0x047DA4, i_047da4),
    (0x047DA8, i_047da8),
    (0x047DAA, i_047daa),
    (0x047DAE, i_047dae),
    (0x047DB2, i_047db2),
    (0x047DB6, i_047db6),
    (0x047DB8, i_047db8),
    (0x047DBC, i_047dbc),
    (0x047DC0, i_047dc0),
    (0x047DC4, i_047dc4),
    (0x047DC8, i_047dc8),
    (0x047DCC, i_047dcc),
    (0x047DD0, i_047dd0),
    (0x047DD4, i_047dd4),
    (0x047DD8, i_047dd8),
    (0x047DDC, i_047ddc),
    (0x047DE2, i_047de2),
    (0x047DE8, i_047de8),
    (0x047DEC, i_047dec),
    (0x047DF2, i_047df2),
    (0x047DF8, i_047df8),
    (0x047DFC, i_047dfc),
    (0x047DFE, i_047dfe),
    (0x047E02, i_047e02),
    (0x047E08, i_047e08),
    (0x047E0C, i_047e0c),
    (0x047E10, i_047e10),
    (0x047E12, i_047e12),
    (0x047E16, i_047e16),
    (0x047E1A, i_047e1a),
    (0x047E1E, i_047e1e),
    (0x047E20, i_047e20),
    (0x047E24, i_047e24),
    (0x047E2A, i_047e2a),
    (0x047E30, i_047e30),
    (0x047E36, i_047e36),
    (0x047E3A, i_047e3a),
    (0x047E3E, i_047e3e),
    (0x047E44, i_047e44),
    (0x047E4A, i_047e4a),
    (0x047E4E, i_047e4e),
    (0x047E50, i_047e50),
    (0x047E54, i_047e54),
    (0x047E5A, i_047e5a),
    (0x047E5E, i_047e5e),
    (0x047E62, i_047e62),
    (0x047E66, i_047e66),
    (0x047E68, i_047e68),
    (0x047E6C, i_047e6c),
    (0x047E70, i_047e70),
    (0x047E74, i_047e74),
    (0x047E76, i_047e76),
    (0x047E7A, i_047e7a),
    (0x047E7E, i_047e7e),
    (0x047E82, i_047e82),
    (0x047E84, i_047e84),
    (0x047E88, i_047e88),
    (0x047E8C, i_047e8c),
    (0x047E92, i_047e92),
    (0x047E96, i_047e96),
    (0x047E9A, i_047e9a),
    (0x047EA0, i_047ea0),
    (0x047EA6, i_047ea6),
    (0x047EAA, i_047eaa),
    (0x047EAC, i_047eac),
    (0x047EB0, i_047eb0),
    (0x047EB6, i_047eb6),
    (0x047EBA, i_047eba),
    (0x047EBE, i_047ebe),
    (0x047EC0, i_047ec0),
    (0x047EC4, i_047ec4),
    (0x047EC8, i_047ec8),
    (0x047ECC, i_047ecc),
    (0x047ECE, i_047ece),
    (0x047ED2, i_047ed2),
    (0x047ED6, i_047ed6),
    (0x047EDA, i_047eda),
    (0x047EE0, i_047ee0),
    (0x047EE6, i_047ee6),
    (0x047EEA, i_047eea),
    (0x047EEC, i_047eec),
    (0x047EF0, i_047ef0),
    (0x047EF6, i_047ef6),
    (0x047EFA, i_047efa),
    (0x047EFE, i_047efe),
    (0x047F00, i_047f00),
    (0x047F04, i_047f04),
    (0x047F08, i_047f08),
    (0x047F0C, i_047f0c),
    (0x047F0E, i_047f0e),
    (0x047F12, i_047f12),
    (0x047F16, i_047f16),
    (0x047F18, i_047f18),
    (0x047F1C, i_047f1c),
    (0x047F22, i_047f22),
    (0x047F28, i_047f28),
    (0x047F2C, i_047f2c),
    (0x047F2E, i_047f2e),
    (0x047F34, i_047f34),
    (0x047F38, i_047f38),
    (0x047F3C, i_047f3c),
    (0x047F3E, i_047f3e),
    (0x047F44, i_047f44),
    (0x047F48, i_047f48),
    (0x047F4C, i_047f4c),
    (0x047F52, i_047f52),
    (0x047F58, i_047f58),
    (0x047F5C, i_047f5c),
    (0x047F5E, i_047f5e),
    (0x047F62, i_047f62),
    (0x047F68, i_047f68),
    (0x047F6C, i_047f6c),
    (0x047F70, i_047f70),
    (0x047F72, i_047f72),
    (0x047F76, i_047f76),
    (0x047F7A, i_047f7a),
    (0x047F7E, i_047f7e),
    (0x047F80, i_047f80),
    (0x047F84, i_047f84),
    (0x047F88, i_047f88),
    (0x047F8C, i_047f8c),
    (0x047F90, i_047f90),
    (0x047F96, i_047f96),
    (0x047F9A, i_047f9a),
    (0x047F9E, i_047f9e),
    (0x047FA2, i_047fa2),
    (0x047FA6, i_047fa6),
    (0x047FAA, i_047faa),
    (0x047FAE, i_047fae),
    (0x047FB2, i_047fb2),
    (0x047FB6, i_047fb6),
    (0x047FBC, i_047fbc),
    (0x047FC0, i_047fc0),
    (0x047FC4, i_047fc4),
    (0x047FC8, i_047fc8),
    (0x047FCC, i_047fcc),
    (0x047FD0, i_047fd0),
    (0x047FD4, i_047fd4),
    (0x047FD8, i_047fd8),
    (0x047FDA, i_047fda),
    (0x047FDC, i_047fdc),
    (0x047FE0, i_047fe0),
    (0x047FE4, i_047fe4),
    (0x047FEA, i_047fea),
    (0x047FF0, i_047ff0),
    (0x047FF6, i_047ff6),
    (0x047FFC, i_047ffc),
    (0x048002, i_048002),
    (0x048006, i_048006),
    (0x04800C, i_04800c),
    (0x048012, i_048012),
    (0x048018, i_048018),
    (0x04801C, i_04801c),
    (0x04801E, i_04801e),
    (0x048022, i_048022),
    (0x048028, i_048028),
    (0x04802C, i_04802c),
    (0x048030, i_048030),
    (0x048032, i_048032),
    (0x048036, i_048036),
    (0x04803A, i_04803a),
    (0x04803E, i_04803e),
    (0x048040, i_048040),
    (0x048044, i_048044),
    (0x048048, i_048048),
    (0x04804E, i_04804e),
    (0x048052, i_048052),
    (0x048056, i_048056),
    (0x04805C, i_04805c),
    (0x048062, i_048062),
    (0x048066, i_048066),
    (0x04806C, i_04806c),
    (0x048070, i_048070),
    (0x048074, i_048074),
    (0x048078, i_048078),
    (0x04807C, i_04807c),
    (0x048080, i_048080),
    (0x048086, i_048086),
    (0x04808C, i_04808c),
    (0x048090, i_048090),
    (0x048096, i_048096),
    (0x04809C, i_04809c),
    (0x0480A0, i_0480a0),
    (0x0480A4, i_0480a4),
    (0x0480A8, i_0480a8),
    (0x0480AC, i_0480ac),
    (0x0480B0, i_0480b0),
    (0x0480B4, i_0480b4),
    (0x0480B8, i_0480b8),
    (0x0480BA, i_0480ba),
    (0x0480BE, i_0480be),
    (0x0480C4, i_0480c4),
    (0x0480C8, i_0480c8),
    (0x0480CC, i_0480cc),
    (0x0480D0, i_0480d0),
    (0x0480D4, i_0480d4),
    (0x0480D8, i_0480d8),
    (0x0480DA, i_0480da),
    (0x0480DE, i_0480de),
    (0x0480E2, i_0480e2),
    (0x0480E6, i_0480e6),
    (0x0480EA, i_0480ea),
    (0x0480EC, i_0480ec),
    (0x0480F0, i_0480f0),
    (0x0480F4, i_0480f4),
    (0x0480F8, i_0480f8),
    (0x0480FE, i_0480fe),
    (0x048104, i_048104),
    (0x048108, i_048108),
    (0x04810C, i_04810c),
    (0x048110, i_048110),
    (0x048116, i_048116),
    (0x04811A, i_04811a),
    (0x048120, i_048120),
    (0x048122, i_048122),
    (0x048126, i_048126),
    (0x04812C, i_04812c),
    (0x048130, i_048130),
    (0x048136, i_048136),
    (0x04813A, i_04813a),
    (0x048140, i_048140),
    (0x048142, i_048142),
    (0x048146, i_048146),
    (0x04814C, i_04814c),
    (0x048150, i_048150),
    (0x048156, i_048156),
    (0x04815A, i_04815a),
    (0x048160, i_048160),
    (0x048166, i_048166),
    (0x04816A, i_04816a),
    (0x04816E, i_04816e),
    (0x048170, i_048170),
    (0x048174, i_048174),
    (0x04817A, i_04817a),
    (0x04817E, i_04817e),
    (0x048184, i_048184),
    (0x048188, i_048188),
    (0x04818E, i_04818e),
    (0x048192, i_048192),
    (0x048196, i_048196),
    (0x048198, i_048198),
    (0x04819C, i_04819c),
    (0x0481A2, i_0481a2),
    (0x0481A6, i_0481a6),
    (0x0481AA, i_0481aa),
    (0x0481AC, i_0481ac),
    (0x0481B0, i_0481b0),
    (0x0481B4, i_0481b4),
    (0x0481B8, i_0481b8),
    (0x0481BA, i_0481ba),
    (0x0481BE, i_0481be),
    (0x0481C2, i_0481c2),
    (0x0481C6, i_0481c6),
    (0x0481CA, i_0481ca),
    (0x0481D0, i_0481d0),
    (0x0481D6, i_0481d6),
    (0x0481DC, i_0481dc),
    (0x0481E0, i_0481e0),
    (0x0481E4, i_0481e4),
    (0x0481E8, i_0481e8),
    (0x0481EC, i_0481ec),
    (0x0481F2, i_0481f2),
    (0x0481F6, i_0481f6),
    (0x0481FC, i_0481fc),
    (0x048202, i_048202),
    (0x048208, i_048208),
    (0x04820C, i_04820c),
    (0x048210, i_048210),
    (0x048214, i_048214),
    (0x048218, i_048218),
    (0x04821E, i_04821e),
    (0x048222, i_048222),
    (0x048224, i_048224),
    (0x048228, i_048228),
    (0x04822E, i_04822e),
    (0x048232, i_048232),
    (0x048236, i_048236),
    (0x04823A, i_04823a),
    (0x04823C, i_04823c),
    (0x048242, i_048242),
    (0x048246, i_048246),
    (0x04824A, i_04824a),
    (0x04824E, i_04824e),
    (0x048252, i_048252),
    (0x048256, i_048256),
    (0x04825A, i_04825a),
    (0x04825C, i_04825c),
    (0x048260, i_048260),
    (0x048264, i_048264),
    (0x04826A, i_04826a),
    (0x048270, i_048270),
    (0x048274, i_048274),
    (0x04827A, i_04827a),
    (0x048280, i_048280),
    (0x048284, i_048284),
    (0x048288, i_048288),
    (0x04828E, i_04828e),
    (0x048294, i_048294),
    (0x048298, i_048298),
    (0x04829A, i_04829a),
    (0x04829E, i_04829e),
    (0x0482A4, i_0482a4),
    (0x0482A8, i_0482a8),
    (0x0482AC, i_0482ac),
    (0x0482AE, i_0482ae),
    (0x0482B2, i_0482b2),
    (0x0482B6, i_0482b6),
    (0x0482BA, i_0482ba),
    (0x0482BC, i_0482bc),
    (0x0482C0, i_0482c0),
    (0x0482C8, i_0482c8),
    (0x0482CE, i_0482ce),
    (0x0482D4, i_0482d4),
    (0x0482D8, i_0482d8),
    (0x0482DA, i_0482da),
    (0x0482DE, i_0482de),
    (0x0482E4, i_0482e4),
    (0x0482E8, i_0482e8),
    (0x0482EC, i_0482ec),
    (0x0482EE, i_0482ee),
    (0x0482F2, i_0482f2),
    (0x0482F6, i_0482f6),
    (0x0482FA, i_0482fa),
    (0x0482FC, i_0482fc),
    (0x048300, i_048300),
    (0x048304, i_048304),
    (0x04830A, i_04830a),
    (0x048310, i_048310),
    (0x048314, i_048314),
    (0x04831A, i_04831a),
    (0x048320, i_048320),
    (0x048324, i_048324),
    (0x04832C, i_04832c),
    (0x048332, i_048332),
    (0x048338, i_048338),
    (0x04833C, i_04833c),
    (0x04833E, i_04833e),
    (0x048342, i_048342),
    (0x048348, i_048348),
    (0x04834C, i_04834c),
    (0x048352, i_048352),
    (0x048356, i_048356),
    (0x04835A, i_04835a),
    (0x04835C, i_04835c),
    (0x048360, i_048360),
    (0x048366, i_048366),
    (0x04836A, i_04836a),
    (0x04836E, i_04836e),
    (0x048370, i_048370),
    (0x048376, i_048376),
    (0x04837C, i_04837c),
    (0x048380, i_048380),
    (0x048384, i_048384),
    (0x048388, i_048388),
    (0x04838C, i_04838c),
    (0x04838E, i_04838e),
    (0x048392, i_048392),
    (0x048398, i_048398),
    (0x04839E, i_04839e),
    (0x0483A2, i_0483a2),
    (0x0483A4, i_0483a4),
    (0x0483A8, i_0483a8),
    (0x0483AE, i_0483ae),
    (0x0483B2, i_0483b2),
    (0x0483BA, i_0483ba),
    (0x0483BE, i_0483be),
    (0x0483C0, i_0483c0),
    (0x0483C4, i_0483c4),
    (0x0483C8, i_0483c8),
    (0x0483CC, i_0483cc),
    (0x0483CE, i_0483ce),
    (0x0483D2, i_0483d2),
    (0x0483D8, i_0483d8),
    (0x0483DE, i_0483de),
    (0x0483E2, i_0483e2),
    (0x0483E6, i_0483e6),
    (0x0483EC, i_0483ec),
    (0x0483F0, i_0483f0),
    (0x0483F4, i_0483f4),
    (0x0483F8, i_0483f8),
    (0x0483FC, i_0483fc),
    (0x048400, i_048400),
    (0x048404, i_048404),
    (0x048408, i_048408),
    (0x04840C, i_04840c),
    (0x048410, i_048410),
    (0x048414, i_048414),
    (0x048418, i_048418),
    (0x04841C, i_04841c),
    (0x048420, i_048420),
    (0x048424, i_048424),
    (0x048426, i_048426),
    (0x04842A, i_04842a),
    (0x048430, i_048430),
    (0x048436, i_048436),
    (0x04843A, i_04843a),
    (0x04843C, i_04843c),
    (0x048442, i_048442),
    (0x048446, i_048446),
    (0x04844A, i_04844a),
    (0x04844C, i_04844c),
    (0x048452, i_048452),
    (0x048456, i_048456),
    (0x04845A, i_04845a),
    (0x048460, i_048460),
    (0x048466, i_048466),
    (0x04846A, i_04846a),
    (0x04846C, i_04846c),
    (0x048470, i_048470),
    (0x048476, i_048476),
    (0x04847A, i_04847a),
    (0x04847E, i_04847e),
    (0x048480, i_048480),
    (0x048484, i_048484),
    (0x048488, i_048488),
    (0x04848C, i_04848c),
    (0x04848E, i_04848e),
    (0x048492, i_048492),
    (0x048498, i_048498),
    (0x04849E, i_04849e),
    (0x0484A2, i_0484a2),
    (0x0484A6, i_0484a6),
    (0x0484AC, i_0484ac),
    (0x0484B2, i_0484b2),
    (0x0484B6, i_0484b6),
    (0x0484B8, i_0484b8),
    (0x0484BC, i_0484bc),
    (0x0484C2, i_0484c2),
    (0x0484C6, i_0484c6),
    (0x0484CA, i_0484ca),
    (0x0484CC, i_0484cc),
    (0x0484D2, i_0484d2),
    (0x0484D6, i_0484d6),
    (0x0484DA, i_0484da),
    (0x0484DE, i_0484de),
    (0x0484E2, i_0484e2),
    (0x0484E6, i_0484e6),
    (0x0484EA, i_0484ea),
    (0x0484EE, i_0484ee),
    (0x0484F0, i_0484f0),
    (0x0484F4, i_0484f4),
    (0x0484F8, i_0484f8),
    (0x0484FC, i_0484fc),
    (0x0484FE, i_0484fe),
    (0x048502, i_048502),
    (0x048506, i_048506),
    (0x04850A, i_04850a),
    (0x04850E, i_04850e),
    (0x048514, i_048514),
    (0x048518, i_048518),
    (0x04851E, i_04851e),
    (0x048524, i_048524),
    (0x04852A, i_04852a),
    (0x048530, i_048530),
    (0x048536, i_048536),
    (0x04853A, i_04853a),
    (0x04853C, i_04853c),
    (0x048542, i_048542),
    (0x048546, i_048546),
    (0x04854A, i_04854a),
    (0x048550, i_048550),
    (0x048554, i_048554),
    (0x048558, i_048558),
    (0x04855C, i_04855c),
    (0x048560, i_048560),
    (0x048562, i_048562),
    (0x048566, i_048566),
    (0x04856A, i_04856a),
    (0x04856E, i_04856e),
    (0x048572, i_048572),
    (0x048576, i_048576),
    (0x048578, i_048578),
    (0x04857C, i_04857c),
    (0x048580, i_048580),
    (0x048588, i_048588),
    (0x04858E, i_04858e),
    (0x048594, i_048594),
    (0x048598, i_048598),
    (0x04859A, i_04859a),
    (0x04859E, i_04859e),
    (0x0485A4, i_0485a4),
    (0x0485A8, i_0485a8),
    (0x0485AC, i_0485ac),
    (0x0485B0, i_0485b0),
    (0x0485B2, i_0485b2),
    (0x0485B8, i_0485b8),
    (0x0485BC, i_0485bc),
    (0x0485C2, i_0485c2),
    (0x0485C6, i_0485c6),
    (0x0485CC, i_0485cc),
    (0x0485D0, i_0485d0),
    (0x0485D4, i_0485d4),
    (0x0485D6, i_0485d6),
    (0x0485DA, i_0485da),
    (0x0485DE, i_0485de),
    (0x0485E2, i_0485e2),
    (0x0485E4, i_0485e4),
    (0x0485E8, i_0485e8),
    (0x0485EC, i_0485ec),
    (0x0485F2, i_0485f2),
    (0x0485F8, i_0485f8),
    (0x0485FC, i_0485fc),
    (0x048602, i_048602),
    (0x048608, i_048608),
    (0x04860C, i_04860c),
    (0x048614, i_048614),
    (0x04861A, i_04861a),
    (0x048620, i_048620),
    (0x048624, i_048624),
    (0x048626, i_048626),
    (0x04862A, i_04862a),
    (0x048630, i_048630),
    (0x048634, i_048634),
    (0x04863A, i_04863a),
    (0x04863E, i_04863e),
    (0x048642, i_048642),
    (0x048644, i_048644),
    (0x048648, i_048648),
    (0x04864E, i_04864e),
    (0x048652, i_048652),
    (0x048656, i_048656),
    (0x048658, i_048658),
    (0x04865E, i_04865e),
    (0x048664, i_048664),
    (0x048668, i_048668),
    (0x04866C, i_04866c),
    (0x048670, i_048670),
    (0x048674, i_048674),
    (0x048676, i_048676),
    (0x04867A, i_04867a),
    (0x04867E, i_04867e),
    (0x048686, i_048686),
    (0x04868C, i_04868c),
    (0x048692, i_048692),
    (0x048696, i_048696),
    (0x048698, i_048698),
    (0x04869C, i_04869c),
    (0x0486A2, i_0486a2),
    (0x0486A6, i_0486a6),
    (0x0486AC, i_0486ac),
    (0x0486B0, i_0486b0),
    (0x0486B6, i_0486b6),
    (0x0486BC, i_0486bc),
    (0x0486C2, i_0486c2),
    (0x0486C6, i_0486c6),
    (0x0486CA, i_0486ca),
    (0x0486CE, i_0486ce),
    (0x0486D0, i_0486d0),
    (0x0486D4, i_0486d4),
    (0x0486D8, i_0486d8),
    (0x0486DE, i_0486de),
    (0x0486E4, i_0486e4),
    (0x0486E8, i_0486e8),
    (0x0486EA, i_0486ea),
    (0x0486EE, i_0486ee),
    (0x0486F4, i_0486f4),
    (0x0486F8, i_0486f8),
    (0x0486FC, i_0486fc),
    (0x0486FE, i_0486fe),
    (0x048702, i_048702),
    (0x048706, i_048706),
    (0x04870A, i_04870a),
    (0x04870C, i_04870c),
    (0x048710, i_048710),
    (0x048714, i_048714),
    (0x04871C, i_04871c),
    (0x048722, i_048722),
    (0x048728, i_048728),
    (0x04872C, i_04872c),
    (0x04872E, i_04872e),
    (0x048732, i_048732),
    (0x048738, i_048738),
    (0x04873C, i_04873c),
    (0x048740, i_048740),
    (0x048744, i_048744),
    (0x048746, i_048746),
    (0x04874C, i_04874c),
    (0x048750, i_048750),
    (0x048754, i_048754),
    (0x048756, i_048756),
    (0x04875A, i_04875a),
    (0x04875E, i_04875e),
    (0x048762, i_048762),
    (0x048764, i_048764),
    (0x048768, i_048768),
    (0x04876C, i_04876c),
    (0x048774, i_048774),
    (0x04877A, i_04877a),
    (0x048780, i_048780),
    (0x048784, i_048784),
    (0x04878A, i_04878a),
    (0x048790, i_048790),
    (0x048794, i_048794),
    (0x04879A, i_04879a),
    (0x0487A0, i_0487a0),
    (0x0487A4, i_0487a4),
    (0x0487A6, i_0487a6),
    (0x0487AA, i_0487aa),
    (0x0487B0, i_0487b0),
    (0x0487B4, i_0487b4),
    (0x0487BA, i_0487ba),
    (0x0487BE, i_0487be),
    (0x0487C2, i_0487c2),
    (0x0487C4, i_0487c4),
    (0x0487C8, i_0487c8),
    (0x0487CE, i_0487ce),
    (0x0487D2, i_0487d2),
    (0x0487D6, i_0487d6),
    (0x0487D8, i_0487d8),
    (0x0487DE, i_0487de),
    (0x0487E4, i_0487e4),
    (0x0487E8, i_0487e8),
    (0x0487EC, i_0487ec),
    (0x0487F0, i_0487f0),
    (0x0487F4, i_0487f4),
    (0x0487F6, i_0487f6),
    (0x0487FA, i_0487fa),
    (0x0487FE, i_0487fe),
    (0x048806, i_048806),
    (0x04880C, i_04880c),
    (0x048812, i_048812),
    (0x048816, i_048816),
    (0x048818, i_048818),
    (0x04881C, i_04881c),
    (0x048822, i_048822),
    (0x048826, i_048826),
    (0x04882A, i_04882a),
    (0x04882C, i_04882c),
    (0x048830, i_048830),
    (0x048834, i_048834),
    (0x048838, i_048838),
    (0x04883A, i_04883a),
    (0x04883E, i_04883e),
    (0x048842, i_048842),
    (0x04884A, i_04884a),
    (0x048850, i_048850),
    (0x048856, i_048856),
    (0x04885A, i_04885a),
    (0x04885C, i_04885c),
    (0x048860, i_048860),
    (0x048866, i_048866),
    (0x04886A, i_04886a),
    (0x048870, i_048870),
    (0x048874, i_048874),
    (0x04887A, i_04887a),
    (0x048880, i_048880),
    (0x048886, i_048886),
    (0x04888A, i_04888a),
    (0x04888E, i_04888e),
    (0x048892, i_048892),
    (0x048894, i_048894),
    (0x048898, i_048898),
    (0x04889C, i_04889c),
    (0x0488A0, i_0488a0),
    (0x0488A6, i_0488a6),
    (0x0488AA, i_0488aa),
    (0x0488B0, i_0488b0),
    (0x0488B6, i_0488b6),
    (0x0488BA, i_0488ba),
    (0x0488C0, i_0488c0),
    (0x0488C6, i_0488c6),
    (0x0488CC, i_0488cc),
    (0x0488D4, i_0488d4),
    (0x0488D8, i_0488d8),
    (0x0488DA, i_0488da),
    (0x0488DC, i_0488dc),
    (0x0488E0, i_0488e0),
    (0x0488E6, i_0488e6),
    (0x0488EC, i_0488ec),
    (0x0488F0, i_0488f0),
    (0x0488F6, i_0488f6),
    (0x0488FA, i_0488fa),
    (0x0488FE, i_0488fe),
    (0x048904, i_048904),
    (0x04890A, i_04890a),
    (0x04890E, i_04890e),
    (0x048914, i_048914),
    (0x04891A, i_04891a),
    (0x048920, i_048920),
    (0x048926, i_048926),
    (0x04892A, i_04892a),
    (0x048930, i_048930),
    (0x048936, i_048936),
    (0x04893C, i_04893c),
    (0x048940, i_048940),
    (0x048942, i_048942),
    (0x048946, i_048946),
    (0x04894A, i_04894a),
    (0x04894E, i_04894e),
    (0x048950, i_048950),
    (0x048956, i_048956),
    (0x04895A, i_04895a),
    (0x04895E, i_04895e),
    (0x048964, i_048964),
    (0x048968, i_048968),
    (0x04896A, i_04896a),
    (0x04896E, i_04896e),
    (0x048974, i_048974),
    (0x048978, i_048978),
    (0x04897C, i_04897c),
    (0x04897E, i_04897e),
    (0x048982, i_048982),
    (0x048988, i_048988),
    (0x04898A, i_04898a),
    (0x04898E, i_04898e),
    (0x048994, i_048994),
    (0x048998, i_048998),
    (0x04899E, i_04899e),
    (0x0489A2, i_0489a2),
    (0x0489A4, i_0489a4),
    (0x0489A8, i_0489a8),
    (0x0489AC, i_0489ac),
    (0x0489B0, i_0489b0),
    (0x0489B2, i_0489b2),
    (0x0489B8, i_0489b8),
    (0x0489BC, i_0489bc),
    (0x0489C0, i_0489c0),
    (0x0489C2, i_0489c2),
    (0x0489C6, i_0489c6),
    (0x0489CC, i_0489cc),
    (0x0489D2, i_0489d2),
    (0x0489D8, i_0489d8),
    (0x0489DC, i_0489dc),
    (0x0489DE, i_0489de),
    (0x0489E4, i_0489e4),
    (0x0489EA, i_0489ea),
    (0x0489EE, i_0489ee),
    (0x0489F2, i_0489f2),
    (0x0489F4, i_0489f4),
    (0x0489F8, i_0489f8),
    (0x0489FC, i_0489fc),
    (0x048A00, i_048a00),
    (0x048A06, i_048a06),
    (0x048A0A, i_048a0a),
    (0x048A0E, i_048a0e),
    (0x048A14, i_048a14),
    (0x048A1A, i_048a1a),
    (0x048A20, i_048a20),
    (0x048A26, i_048a26),
    (0x048A2C, i_048a2c),
    (0x048A32, i_048a32),
    (0x048A38, i_048a38),
    (0x048A3C, i_048a3c),
    (0x048A40, i_048a40),
    (0x048A42, i_048a42),
    (0x048A44, i_048a44),
    (0x048A48, i_048a48),
    (0x048A4C, i_048a4c),
    (0x048A52, i_048a52),
    (0x048A56, i_048a56),
    (0x048A5A, i_048a5a),
    (0x048A60, i_048a60),
    (0x048A66, i_048a66),
    (0x048A6C, i_048a6c),
    (0x048A72, i_048a72),
    (0x048A78, i_048a78),
    (0x048A7E, i_048a7e),
    (0x048A84, i_048a84),
    (0x048A88, i_048a88),
    (0x048A8C, i_048a8c),
    (0x048A8E, i_048a8e),
    (0x048A90, i_048a90),
    (0x048A94, i_048a94),
    (0x048A98, i_048a98),
    (0x048A9E, i_048a9e),
    (0x048AA2, i_048aa2),
    (0x048AA6, i_048aa6),
    (0x048AAC, i_048aac),
    (0x048AB2, i_048ab2),
    (0x048AB8, i_048ab8),
    (0x048ABC, i_048abc),
    (0x048AC0, i_048ac0),
    (0x048AC2, i_048ac2),
    (0x048AC6, i_048ac6),
    (0x048ACC, i_048acc),
    (0x048AD2, i_048ad2),
    (0x048AD8, i_048ad8),
    (0x048ADC, i_048adc),
    (0x048AE2, i_048ae2),
    (0x048AE6, i_048ae6),
];
