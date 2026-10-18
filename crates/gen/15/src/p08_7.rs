// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $08D8CC: jsr $28d70.l
pub fn i_08d8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D8D2);
    cpu.pc = 0x28D70;
    16
}
/// $08D8D2: bcc.w $8d8f2
pub fn i_08d8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D8F2; return 10; }
    8
}
/// $08D8D6: move.w #$3c, $70(a6)
pub fn i_08d8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D8DC: move.l #$ffff8000, $88(a6)
pub fn i_08d8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8E4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFF8000;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D8E4: move.l #$0, $8c(a6)
pub fn i_08d8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8EC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== SetTaskHandler_08d8ec  ($08D8EC, 8 B, kernel/task_handlers.c)
/// $08D8EC: lea.l $8d8f4(pc), a1
pub fn i_08d8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D8F4;
    16
}
/// $08D8F0: move.l a1, (a6)
pub fn i_08d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D8F2: rts
pub fn i_08d8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_ScrollUp_08d8f4  ($08D8F4, 60 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D8F4: cmpi.w #$d8, $106f54.l
pub fn i_08d8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8FC;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0xD8, Size::Word, false);
    20
}
/// $08D8FC: bhi.w $8d926
pub fn i_08d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D900;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8D926; return 10; }
    8
}
/// $08D900: move.w #$b4, $70(a6)
pub fn i_08d900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D906;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D906: move.l #$0, $106f64.l
pub fn i_08d906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D910;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D910: move.l #$0, $88(a6)
pub fn i_08d910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D918;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D918: move.l #$0, $8c(a6)
pub fn i_08d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D920;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D920: lea.l $8d938(pc), a1
pub fn i_08d920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D924;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D938;
    16
}
/// $08D924: move.l a1, (a6)
pub fn i_08d924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D926;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D926: jsr $8ccd8(pc)
pub fn i_08d926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D92A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D92A);
    cpu.pc = 0x8CCD8;
    16
}
/// $08D92A: jsr $2783a.l
pub fn i_08d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D930);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d930  ($08D930, 8 B, thunks/jsr_abs_thunks.c)
/// $08D930: jsr $28d70.l
pub fn i_08d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D936);
    cpu.pc = 0x28D70;
    16
}
/// $08D936: rts
pub fn i_08d936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D938;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_Wait_08d938  ($08D938, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D938: subq.w #$1, $70(a6)
pub fn i_08d938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D93C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D93C: cmpi.w #$0, $70(a6)
pub fn i_08d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D942;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D942: bgt.w $8d978
pub fn i_08d942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D946;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D978; return 10; }
    8
}
/// $08D946: lea.l $8ccea(pc), a1
pub fn i_08d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D94A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CCEA;
    16
}
/// $08D94A: jsr $4ae.l
pub fn i_08d94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D950);
    cpu.pc = 0x4AE;
    16
}
/// $08D950: move.b #$a, $98(a0)
pub fn i_08d950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D956;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D956: move.b #$8, $99(a0)
pub fn i_08d956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D95C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D95C: move.w #$a0, $22(a0)
pub fn i_08d95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D962;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D962: move.w #$100, $24(a0)
pub fn i_08d962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D968;
    cpu.ir = 0x317C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D968: move.l #$0, $106f64.l
pub fn i_08d968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D972;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D972: lea.l $8d986(pc), a1
pub fn i_08d972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D976;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D986;
    16
}
/// $08D976: move.l a1, (a6)
pub fn i_08d976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D978;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D978: jsr $2783a.l
pub fn i_08d978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D97E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D97E);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d97e  ($08D97E, 8 B, thunks/jsr_abs_thunks.c)
/// $08D97E: jsr $28d70.l
pub fn i_08d97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D984);
    cpu.pc = 0x28D70;
    16
}
/// $08D984: rts
pub fn i_08d984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D986;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_Idle_08d986  ($08D986, 6 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D986: jsr $2783a.l
pub fn i_08d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D98C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D98C);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d98c  ($08D98C, 8 B, thunks/jsr_abs_thunks.c)
/// $08D98C: jsr $28d70.l
pub fn i_08d98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D992;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D992);
    cpu.pc = 0x28D70;
    16
}
/// $08D992: rts
pub fn i_08d992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D994;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl157_08d994  ($08D994, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D994: jsr $8f108(pc)
pub fn i_08d994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D998;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D998);
    cpu.pc = 0x8F108;
    16
}
/// $08D998: jsr $8f002(pc)
pub fn i_08d998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D99C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D99C);
    cpu.pc = 0x8F002;
    16
}
/// $08D99C: jsr $8f02c(pc)
pub fn i_08d99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9A0);
    cpu.pc = 0x8F02C;
    16
}
/// $08D9A0: jsr $8f084(pc)
pub fn i_08d9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9A4);
    cpu.pc = 0x8F084;
    16
}
/// $08D9A4: lea.l $2f3ab4.l, a0
pub fn i_08d9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3AB4;
    20
}
/// $08D9AA: jsr $28cd4.l
pub fn i_08d9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9B0);
    cpu.pc = 0x28CD4;
    16
}
/// $08D9B0: lea.l $8d9b6(pc), a1
pub fn i_08d9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D9B6;
    16
}
/// $08D9B4: move.l a1, (a6)
pub fn i_08d9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D9B6: jsr $8f040(pc)
pub fn i_08d9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9BA);
    cpu.pc = 0x8F040;
    16
}
/// $08D9BA: bcc.w $8d9d0
pub fn i_08d9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D9D0; return 10; }
    8
}
/// $08D9BE: lea.l $2f3ac0.l, a0
pub fn i_08d9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3AC0;
    20
}
/// $08D9C4: jsr $28cd4.l
pub fn i_08d9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9CA);
    cpu.pc = 0x28CD4;
    16
}
/// $08D9CA: lea.l $8d9d0(pc), a1
pub fn i_08d9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D9D0;
    16
}
/// $08D9CE: move.l a1, (a6)
pub fn i_08d9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D9D0: jsr $2783a.l
pub fn i_08d9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9D6);
    cpu.pc = 0x2783A;
    16
}
/// $08D9D6: jsr $8efb0(pc)
pub fn i_08d9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9DA);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08d9da  ($08D9DA, 8 B, thunks/jsr_abs_thunks.c)
/// $08D9DA: jsr $28d70.l
pub fn i_08d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9E0);
    cpu.pc = 0x28D70;
    16
}
/// $08D9E0: rts
pub fn i_08d9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl158_08d9e2  ($08D9E2, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D9E2: jsr $8f108(pc)
pub fn i_08d9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9E6);
    cpu.pc = 0x8F108;
    16
}
/// $08D9E6: jsr $8f002(pc)
pub fn i_08d9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9EA);
    cpu.pc = 0x8F002;
    16
}
/// $08D9EA: jsr $8f02c(pc)
pub fn i_08d9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9EE);
    cpu.pc = 0x8F02C;
    16
}
/// $08D9EE: jsr $8f084(pc)
pub fn i_08d9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9F2);
    cpu.pc = 0x8F084;
    16
}
/// $08D9F2: lea.l $2f3af0.l, a0
pub fn i_08d9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3AF0;
    20
}
/// $08D9F8: jsr $28cd4.l
pub fn i_08d9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9FE);
    cpu.pc = 0x28CD4;
    16
}
/// $08D9FE: lea.l $8da04(pc), a1
pub fn i_08d9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DA04;
    16
}
/// $08DA02: move.l a1, (a6)
pub fn i_08da02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DA04: jsr $8f040(pc)
pub fn i_08da04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA08);
    cpu.pc = 0x8F040;
    16
}
/// $08DA08: bcc.w $8da1e
pub fn i_08da08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DA1E; return 10; }
    8
}
/// $08DA0C: lea.l $2f3afc.l, a0
pub fn i_08da0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3AFC;
    20
}
/// $08DA12: jsr $28cd4.l
pub fn i_08da12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA18);
    cpu.pc = 0x28CD4;
    16
}
/// $08DA18: lea.l $8da1e(pc), a1
pub fn i_08da18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DA1E;
    16
}
/// $08DA1C: move.l a1, (a6)
pub fn i_08da1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DA1E: jsr $2783a.l
pub fn i_08da1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA24);
    cpu.pc = 0x2783A;
    16
}
/// $08DA24: jsr $8efb0(pc)
pub fn i_08da24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA28);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08da28  ($08DA28, 8 B, thunks/jsr_abs_thunks.c)
/// $08DA28: jsr $28d70.l
pub fn i_08da28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA2E);
    cpu.pc = 0x28D70;
    16
}
/// $08DA2E: rts
pub fn i_08da2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Carrier_08da30  ($08DA30, 102 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DA30: move.b #$0, $98(a6)
pub fn i_08da30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA36;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DA36: move.b #$0, $99(a6)
pub fn i_08da36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA3C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DA3C: move.w #$40, $22(a6)
pub fn i_08da3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DA42: move.w #$150, $24(a6)
pub fn i_08da42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA48;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DA48: clr.w $26(a6)
pub fn i_08da48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA4C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DA4C: jsr $8f108(pc)
pub fn i_08da4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA50);
    cpu.pc = 0x8F108;
    16
}
/// $08DA50: jsr $8f002(pc)
pub fn i_08da50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA54);
    cpu.pc = 0x8F002;
    16
}
/// $08DA54: jsr $8f02c(pc)
pub fn i_08da54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA58);
    cpu.pc = 0x8F02C;
    16
}
/// $08DA58: jsr $8f0d0(pc)
pub fn i_08da58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA5C);
    cpu.pc = 0x8F0D0;
    16
}
/// $08DA5C: clr.w $5c(a6)
pub fn i_08da5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA60;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DA60: lea.l $2f3b2c.l, a0
pub fn i_08da60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3B2C;
    20
}
/// $08DA66: jsr $28cd4.l
pub fn i_08da66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA6C);
    cpu.pc = 0x28CD4;
    16
}
/// $08DA6C: lea.l $8da72(pc), a1
pub fn i_08da6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DA72;
    16
}
/// $08DA70: move.l a1, (a6)
pub fn i_08da70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DA72: jsr $8f040(pc)
pub fn i_08da72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA76);
    cpu.pc = 0x8F040;
    16
}
/// $08DA76: bcc.w $8da8c
pub fn i_08da76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DA8C; return 10; }
    8
}
/// $08DA7A: lea.l $2f3b70.l, a0
pub fn i_08da7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3B70;
    20
}
/// $08DA80: jsr $28cd4.l
pub fn i_08da80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA86);
    cpu.pc = 0x28CD4;
    16
}
/// $08DA86: lea.l $8da9e(pc), a1
pub fn i_08da86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DA9E;
    16
}
/// $08DA8A: move.l a1, (a6)
pub fn i_08da8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DA8C: jsr $2783a.l
pub fn i_08da8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA92);
    cpu.pc = 0x2783A;
    16
}
/// $08DA92: jsr $8efb0(pc)
pub fn i_08da92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA96);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08da96  ($08DA96, 8 B, thunks/jsr_abs_thunks.c)
/// $08DA96: jsr $28d70.l
pub fn i_08da96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA9C);
    cpu.pc = 0x28D70;
    16
}
/// $08DA9C: rts
pub fn i_08da9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Carrier_Drop_08da9e  ($08DA9E, 60 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DA9E: cmpi.w #$0, $5c(a6)
pub fn i_08da9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAA4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08DAA4: beq.w $8dad0
pub fn i_08daa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8DAD0; return 10; }
    8
}
/// $08DAA8: lea.l $8dae2(pc), a1
pub fn i_08daa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DAE2;
    16
}
/// $08DAAC: jsr $4ae.l
pub fn i_08daac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DAB2);
    cpu.pc = 0x4AE;
    16
}
/// $08DAB2: jsr $5dd02.l
pub fn i_08dab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DAB8);
    cpu.pc = 0x5DD02;
    16
}
/// $08DAB8: move.b $3a(a6), $98(a0)
pub fn i_08dab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DABE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08DABE: move.b #$30, $99(a0)
pub fn i_08dabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAC4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DAC4: move.b $9a(a6), $9a(a0)
pub fn i_08dac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DACA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08DACA: lea.l $8dad0(pc), a1
pub fn i_08daca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DACE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DAD0;
    16
}
/// $08DACE: move.l a1, (a6)
pub fn i_08dace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DAD0: jsr $2783a.l
pub fn i_08dad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DAD6);
    cpu.pc = 0x2783A;
    16
}
/// $08DAD6: jsr $8efb0(pc)
pub fn i_08dad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DADA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DADA);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08dada  ($08DADA, 8 B, thunks/jsr_abs_thunks.c)
/// $08DADA: jsr $28d70.l
pub fn i_08dada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DAE0);
    cpu.pc = 0x28D70;
    16
}
/// $08DAE0: rts
pub fn i_08dae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Dropped_08dae2  ($08DAE2, 102 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DAE2: move.w #$c000, $38(a6)
pub fn i_08dae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DAE8: andi.w #$ffe3, $38(a6)
pub fn i_08dae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAEE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08DAEE: ori.w #$1c, $38(a6)
pub fn i_08daee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAF4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08DAF4: jsr $8f002(pc)
pub fn i_08daf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DAF8);
    cpu.pc = 0x8F002;
    16
}
/// $08DAF8: jsr $8f010(pc)
pub fn i_08daf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DAFC);
    cpu.pc = 0x8F010;
    16
}
/// $08DAFC: neg.w $28(a6)
pub fn i_08dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB00;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DB00: move.w #$10, $2e(a6)
pub fn i_08db00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DB06: addi.w #$20, $24(a6)
pub fn i_08db06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB0C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08DB0C: jsr $8f0d0(pc)
pub fn i_08db0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DB10);
    cpu.pc = 0x8F0D0;
    16
}
/// $08DB10: lea.l $2f3c04.l, a0
pub fn i_08db10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C04;
    20
}
/// $08DB16: jsr $28cd4.l
pub fn i_08db16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB1C);
    cpu.pc = 0x28CD4;
    16
}
/// $08DB1C: lea.l $8db22(pc), a1
pub fn i_08db1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB22;
    16
}
/// $08DB20: move.l a1, (a6)
pub fn i_08db20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DB22: jsr $2783a.l
pub fn i_08db22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB28);
    cpu.pc = 0x2783A;
    16
}
/// $08DB28: jsr $8d34e(pc)
pub fn i_08db28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DB2C);
    cpu.pc = 0x8D34E;
    16
}
/// $08DB2C: movea.l #$ffffffff, a0
pub fn i_08db2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB32;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08DB32: jsr $5dd56.l
pub fn i_08db32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB38);
    cpu.pc = 0x5DD56;
    16
}
/// $08DB38: bcc.w $8db48
pub fn i_08db38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DB48; return 10; }
    8
}
/// $08DB3C: jsr $5b6.l
pub fn i_08db3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB42);
    cpu.pc = 0x5B6;
    16
}
/// $08DB42: jmp $518.l
pub fn i_08db42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_08db48  ($08DB48, 8 B, thunks/jsr_abs_thunks.c)
/// $08DB48: jsr $28d70.l
pub fn i_08db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB4E);
    cpu.pc = 0x28D70;
    16
}
/// $08DB4E: rts
pub fn i_08db4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl162_08db50  ($08DB50, 42 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DB50: move.w #$d000, $38(a6)
pub fn i_08db50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB56;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DB56: jsr $8f002(pc)
pub fn i_08db56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DB5A);
    cpu.pc = 0x8F002;
    16
}
/// $08DB5A: jsr $8f010(pc)
pub fn i_08db5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DB5E);
    cpu.pc = 0x8F010;
    16
}
/// $08DB5E: move.w #$f8, d1
pub fn i_08db5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB62;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DB62: jsr $236e.l
pub fn i_08db62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB68);
    cpu.pc = 0x236E;
    16
}
/// $08DB68: lea.l $2f3c84.l, a0
pub fn i_08db68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C84;
    20
}
/// $08DB6E: jsr $28cd4.l
pub fn i_08db6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB74);
    cpu.pc = 0x28CD4;
    16
}
/// $08DB74: lea.l $8db7a(pc), a1
pub fn i_08db74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB7A;
    16
}
/// $08DB78: move.l a1, (a6)
pub fn i_08db78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Grunt_HopDown_08db7a  ($08DB7A, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DB7A: move.w #$e, $2e(a6)
pub fn i_08db7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DB80: move.w #$8, $70(a6)
pub fn i_08db80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB86;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DB86: move.w #$7, d0
pub fn i_08db86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB8A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DB8A: jsr $5ea1c.l
pub fn i_08db8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB90);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DB90: add.w d0, $70(a6)
pub fn i_08db90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB94;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DB94: lea.l $2f3c84.l, a0
pub fn i_08db94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C84;
    20
}
/// $08DB9A: jsr $28cd4.l
pub fn i_08db9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DBA0);
    cpu.pc = 0x28CD4;
    16
}
/// $08DBA0: lea.l $8dba6(pc), a1
pub fn i_08dba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DBA6;
    16
}
/// $08DBA4: move.l a1, (a6)
pub fn i_08dba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DBA6: jsr $2783a.l
pub fn i_08dba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DBAC);
    cpu.pc = 0x2783A;
    16
}
/// $08DBAC: jsr $8d34e(pc)
pub fn i_08dbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DBB0);
    cpu.pc = 0x8D34E;
    16
}
/// $08DBB0: jsr $28d70.l
pub fn i_08dbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DBB6);
    cpu.pc = 0x28D70;
    16
}
/// $08DBB6: cmpi.w #$1d0, $24(a6)
pub fn i_08dbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBBC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1D0, Size::Word, false);
    16
}
/// $08DBBC: blt.w $8dbc8
pub fn i_08dbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBC0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8DBC8; return 10; }
    8
}
// ===== SetTaskHandler_08dbc0  ($08DBC0, 8 B, kernel/task_handlers.c)
/// $08DBC0: lea.l $8dbe2(pc), a1
pub fn i_08dbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DBE2;
    16
}
/// $08DBC4: move.l a1, (a6)
pub fn i_08dbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DBC6: rts
pub fn i_08dbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_HopDown_Wait_08dbc8  ($08DBC8, 20 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DBC8: subq.w #$1, $70(a6)
pub fn i_08dbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBCC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DBCC: cmpi.w #$0, $70(a6)
pub fn i_08dbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBD2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08DBD2: bhi.w $8dbdc
pub fn i_08dbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBD6;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8DBDC; return 10; }
    8
}
/// $08DBD6: lea.l $8dbe2(pc), a1
pub fn i_08dbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DBE2;
    16
}
/// $08DBDA: move.l a1, (a6)
pub fn i_08dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08dbdc  ($08DBDC, 6 B, thunks/jsr_pc_thunks.c)
/// $08DBDC: jsr $8efb0(pc)
pub fn i_08dbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DBE0);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DBE0: rts
pub fn i_08dbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_HopUp_08dbe2  ($08DBE2, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DBE2: move.w #$fff2, $2e(a6)
pub fn i_08dbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DBE8: move.w #$8, $70(a6)
pub fn i_08dbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DBEE: move.w #$7, d0
pub fn i_08dbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DBF2: jsr $5ea1c.l
pub fn i_08dbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DBF8);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DBF8: add.w d0, $70(a6)
pub fn i_08dbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBFC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DBFC: lea.l $2f3cb4.l, a0
pub fn i_08dbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3CB4;
    20
}
/// $08DC02: jsr $28cd4.l
pub fn i_08dc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC08);
    cpu.pc = 0x28CD4;
    16
}
/// $08DC08: lea.l $8dc0e(pc), a1
pub fn i_08dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DC0E;
    16
}
/// $08DC0C: move.l a1, (a6)
pub fn i_08dc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DC0E: jsr $2783a.l
pub fn i_08dc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC14);
    cpu.pc = 0x2783A;
    16
}
/// $08DC14: jsr $8d34e(pc)
pub fn i_08dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC18);
    cpu.pc = 0x8D34E;
    16
}
/// $08DC18: jsr $28d70.l
pub fn i_08dc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC1E);
    cpu.pc = 0x28D70;
    16
}
/// $08DC1E: cmpi.w #$160, $24(a6)
pub fn i_08dc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $08DC24: bge.w $8dc30
pub fn i_08dc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC28;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8DC30; return 10; }
    8
}
// ===== SetTaskHandler_08dc28  ($08DC28, 8 B, kernel/task_handlers.c)
/// $08DC28: lea.l $8db7a(pc), a1
pub fn i_08dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB7A;
    16
}
/// $08DC2C: move.l a1, (a6)
pub fn i_08dc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DC2E: rts
pub fn i_08dc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_HopUp_Wait_08dc30  ($08DC30, 20 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DC30: subq.w #$1, $70(a6)
pub fn i_08dc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC34;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DC34: cmpi.w #$0, $70(a6)
pub fn i_08dc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC3A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08DC3A: bhi.w $8dc44
pub fn i_08dc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC3E;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8DC44; return 10; }
    8
}
/// $08DC3E: lea.l $8db7a(pc), a1
pub fn i_08dc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB7A;
    16
}
/// $08DC42: move.l a1, (a6)
pub fn i_08dc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08dc44  ($08DC44, 6 B, thunks/jsr_pc_thunks.c)
/// $08DC44: jsr $8efb0(pc)
pub fn i_08dc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC48);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DC48: rts
pub fn i_08dc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Rand_Idle_08dc4a  ($08DC4A, 68 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DC4A: move.w #$40, $22(a6)
pub fn i_08dc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DC50: move.w #$150, $24(a6)
pub fn i_08dc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC56;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DC56: clr.w $26(a6)
pub fn i_08dc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC5A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DC5A: jsr $8f108(pc)
pub fn i_08dc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC5E);
    cpu.pc = 0x8F108;
    16
}
/// $08DC5E: jsr $8f002(pc)
pub fn i_08dc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC62);
    cpu.pc = 0x8F002;
    16
}
/// $08DC62: jsr $8f02c(pc)
pub fn i_08dc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC66);
    cpu.pc = 0x8F02C;
    16
}
/// $08DC66: move.w #$f8, d1
pub fn i_08dc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC6A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DC6A: jsr $236e.l
pub fn i_08dc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC70);
    cpu.pc = 0x236E;
    16
}
/// $08DC70: lea.l $2f3c58.l, a0
pub fn i_08dc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C58;
    20
}
/// $08DC76: jsr $28cd4.l
pub fn i_08dc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC7C);
    cpu.pc = 0x28CD4;
    16
}
/// $08DC7C: jsr $27cee.l
pub fn i_08dc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC82);
    cpu.pc = 0x27CEE;
    16
}
/// $08DC82: jsr $28d70.l
pub fn i_08dc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC88);
    cpu.pc = 0x28D70;
    16
}
/// $08DC88: lea.l $8dc94(pc), a1
pub fn i_08dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DC94;
    16
}
/// $08DC8C: move.l a1, (a6)
pub fn i_08dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08dc8e  ($08DC8E, 6 B, thunks/jsr_pc_thunks.c)
/// $08DC8E: jsr $8efb0(pc)
pub fn i_08dc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC92);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DC92: rts
pub fn i_08dc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Rand_Stand_08dc94  ($08DC94, 96 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DC94: lea.l $2f3c58.l, a0
pub fn i_08dc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C58;
    20
}
/// $08DC9A: jsr $28cd4.l
pub fn i_08dc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DCA0);
    cpu.pc = 0x28CD4;
    16
}
/// $08DCA0: lea.l $8dca6(pc), a1
pub fn i_08dca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DCA6;
    16
}
/// $08DCA4: move.l a1, (a6)
pub fn i_08dca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DCA6: jsr $2783a.l
pub fn i_08dca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DCAC);
    cpu.pc = 0x2783A;
    16
}
/// $08DCAC: jsr $28d70.l
pub fn i_08dcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DCB2);
    cpu.pc = 0x28D70;
    16
}
/// $08DCB2: bcc.w $8dcf4
pub fn i_08dcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DCF4; return 10; }
    8
}
/// $08DCB6: cmpi.b #$0, $99(a6)
pub fn i_08dcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCBC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08DCBC: bne.w $8dcde
pub fn i_08dcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8DCDE; return 10; }
    8
}
/// $08DCC0: move.w #$60, d0
pub fn i_08dcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCC4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DCC4: jsr $8efce(pc)
pub fn i_08dcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCC8);
    cpu.pc = 0x8EFCE;
    16
}
/// $08DCC8: bcc.w $8dcd6
pub fn i_08dcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCCC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DCD6; return 10; }
    8
}
/// $08DCCC: lea.l $8dd7e(pc), a1
pub fn i_08dccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD7E;
    16
}
/// $08DCD0: move.l a1, (a6)
pub fn i_08dcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DCD2: bra.w $8dcda
pub fn i_08dcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x8DCDA;
    10
}
/// $08DCD6: jsr $8dec2(pc)
pub fn i_08dcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCDA);
    cpu.pc = 0x8DEC2;
    16
}
/// $08DCDA: bra.w $8dcf4
pub fn i_08dcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8DCF4;
    10
}
/// $08DCDE: jsr $8f040(pc)
pub fn i_08dcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCE2);
    cpu.pc = 0x8F040;
    16
}
/// $08DCE2: bcc.w $8dcf0
pub fn i_08dce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DCF0; return 10; }
    8
}
/// $08DCE6: lea.l $8dd7e(pc), a1
pub fn i_08dce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD7E;
    16
}
/// $08DCEA: move.l a1, (a6)
pub fn i_08dcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DCEC: bra.w $8dcf4
pub fn i_08dcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x8DCF4;
    10
}
/// $08DCF0: jsr $8dec2(pc)
pub fn i_08dcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCF4);
    cpu.pc = 0x8DEC2;
    16
}
// ===== JsrPcThunk_08dcf4  ($08DCF4, 6 B, thunks/jsr_pc_thunks.c)
/// $08DCF4: jsr $8efb0(pc)
pub fn i_08dcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCF8);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DCF8: rts
pub fn i_08dcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Rand_Stand2_08dcfa  ($08DCFA, 38 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DCFA: lea.l $2f3c64.l, a0
pub fn i_08dcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C64;
    20
}
/// $08DD00: jsr $28cd4.l
pub fn i_08dd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD06);
    cpu.pc = 0x28CD4;
    16
}
/// $08DD06: lea.l $8dd0c(pc), a1
pub fn i_08dd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD0C;
    16
}
/// $08DD0A: move.l a1, (a6)
pub fn i_08dd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DD0C: jsr $2783a.l
pub fn i_08dd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD12);
    cpu.pc = 0x2783A;
    16
}
/// $08DD12: jsr $28d70.l
pub fn i_08dd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD18);
    cpu.pc = 0x28D70;
    16
}
/// $08DD18: bcc.w $8dd20
pub fn i_08dd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DD20; return 10; }
    8
}
/// $08DD1C: jsr $8dec2(pc)
pub fn i_08dd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD20);
    cpu.pc = 0x8DEC2;
    16
}
// ===== JsrPcThunk_08dd20  ($08DD20, 6 B, thunks/jsr_pc_thunks.c)
/// $08DD20: jsr $8efb0(pc)
pub fn i_08dd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD24;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD24);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DD24: rts
pub fn i_08dd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Rand_Walk_08dd26  ($08DD26, 82 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DD26: move.w #$1, d0
pub fn i_08dd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD2A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DD2A: jsr $5ea1c.l
pub fn i_08dd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD30);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DD30: move.b d0, $98(a6)
pub fn i_08dd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD34;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DD34: bne.w $8dd42
pub fn i_08dd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD38;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8DD42; return 10; }
    8
}
/// $08DD38: move.w #$ff80, $28(a6)
pub fn i_08dd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DD3E: bra.w $8dd48
pub fn i_08dd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD42;
    cpu.ir = 0x6000;
    cpu.pc = 0x8DD48;
    10
}
/// $08DD42: move.w #$80, $28(a6)
pub fn i_08dd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD48;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DD48: jsr $8f002(pc)
pub fn i_08dd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD4C);
    cpu.pc = 0x8F002;
    16
}
/// $08DD4C: lea.l $2f3c10.l, a0
pub fn i_08dd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD52;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C10;
    20
}
/// $08DD52: jsr $28cd4.l
pub fn i_08dd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD58);
    cpu.pc = 0x28CD4;
    16
}
/// $08DD58: lea.l $8dd5e(pc), a1
pub fn i_08dd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD5E;
    16
}
/// $08DD5C: move.l a1, (a6)
pub fn i_08dd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DD5E: jsr $2783a.l
pub fn i_08dd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD64);
    cpu.pc = 0x2783A;
    16
}
/// $08DD64: jsr $8d2f8(pc)
pub fn i_08dd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD68);
    cpu.pc = 0x8D2F8;
    16
}
/// $08DD68: jsr $28d70.l
pub fn i_08dd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD6E);
    cpu.pc = 0x28D70;
    16
}
/// $08DD6E: bcc.w $8dd78
pub fn i_08dd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DD78; return 10; }
    8
}
/// $08DD72: lea.l $8dc94(pc), a1
pub fn i_08dd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DC94;
    16
}
/// $08DD76: move.l a1, (a6)
pub fn i_08dd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08dd78  ($08DD78, 6 B, thunks/jsr_pc_thunks.c)
/// $08DD78: jsr $8efb0(pc)
pub fn i_08dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD7C);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DD7C: rts
pub fn i_08dd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Hit_08dd7e  ($08DD7E, 44 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DD7E: addi.w #$4, $24(a6)
pub fn i_08dd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD84;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08DD84: move.w #$1e, $70(a6)
pub fn i_08dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD8A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DD8A: move.w #$f, d0
pub fn i_08dd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DD8E: jsr $5ea1c.l
pub fn i_08dd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD94);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DD94: add.w d0, $70(a6)
pub fn i_08dd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD98;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DD98: jsr $2783a.l
pub fn i_08dd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD9E);
    cpu.pc = 0x2783A;
    16
}
/// $08DD9E: jsr $28d70.l
pub fn i_08dd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDA4);
    cpu.pc = 0x28D70;
    16
}
/// $08DDA4: lea.l $8ddb0(pc), a1
pub fn i_08dda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DDB0;
    16
}
/// $08DDA8: move.l a1, (a6)
pub fn i_08dda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08ddaa  ($08DDAA, 6 B, thunks/jsr_pc_thunks.c)
/// $08DDAA: jsr $8efb0(pc)
pub fn i_08ddaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DDAE);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DDAE: rts
pub fn i_08ddae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Hit_Angle_08ddb0  ($08DDB0, 36 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DDB0: move.w #$1f, d0
pub fn i_08ddb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DDB4: jsr $5ea1c.l
pub fn i_08ddb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDBA);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DDBA: addi.w #$10, d0
pub fn i_08ddba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDBE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08DDBE: move.w d0, $34(a6)
pub fn i_08ddbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDC2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DDC2: jsr $2783a.l
pub fn i_08ddc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDC8);
    cpu.pc = 0x2783A;
    16
}
/// $08DDC8: jsr $28d70.l
pub fn i_08ddc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDCE);
    cpu.pc = 0x28D70;
    16
}
/// $08DDCE: lea.l $8ddda(pc), a1
pub fn i_08ddce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DDDA;
    16
}
/// $08DDD2: move.l a1, (a6)
pub fn i_08ddd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08ddd4  ($08DDD4, 6 B, thunks/jsr_pc_thunks.c)
/// $08DDD4: jsr $8efb0(pc)
pub fn i_08ddd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DDD8);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DDD8: rts
pub fn i_08ddd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Hit_Launch_08ddda  ($08DDDA, 74 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DDDA: move.w #$ff, d0
pub fn i_08ddda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DDDE: jsr $5ea1c.l
pub fn i_08ddde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDE4);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DDE4: move.w #$200, d1
pub fn i_08dde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDE8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DDE8: add.w d0, d1
pub fn i_08dde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDEA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08DDEA: move.w d1, $36(a6)
pub fn i_08ddea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDEE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DDEE: move.w $34(a6), d0
pub fn i_08ddee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDF2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08DDF2: jsr $13c0e.l
pub fn i_08ddf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDF8);
    cpu.pc = 0x13C0E;
    16
}
/// $08DDF8: move.w d1, $28(a6)
pub fn i_08ddf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDFC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DDFC: move.w d2, $2a(a6)
pub fn i_08ddfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE00;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DE00: move.b #$1, $3a(a6)
pub fn i_08de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE06;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DE06: lea.l $2f3c84.l, a0
pub fn i_08de06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C84;
    20
}
/// $08DE0C: jsr $28cd4.l
pub fn i_08de0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE12);
    cpu.pc = 0x28CD4;
    16
}
/// $08DE12: jsr $2783a.l
pub fn i_08de12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE18);
    cpu.pc = 0x2783A;
    16
}
/// $08DE18: jsr $28d70.l
pub fn i_08de18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE1E);
    cpu.pc = 0x28D70;
    16
}
/// $08DE1E: lea.l $8de2a(pc), a1
pub fn i_08de1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DE2A;
    16
}
/// $08DE22: move.l a1, (a6)
pub fn i_08de22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08de24  ($08DE24, 6 B, thunks/jsr_pc_thunks.c)
/// $08DE24: jsr $8efb0(pc)
pub fn i_08de24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE28);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DE28: rts
pub fn i_08de28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Hit_Fly_08de2a  ($08DE2A, 52 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DE2A: jsr $2783a.l
pub fn i_08de2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE30);
    cpu.pc = 0x2783A;
    16
}
/// $08DE30: jsr $8d2f8(pc)
pub fn i_08de30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE34);
    cpu.pc = 0x8D2F8;
    16
}
/// $08DE34: jsr $28d70.l
pub fn i_08de34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE3A);
    cpu.pc = 0x28D70;
    16
}
/// $08DE3A: subq.w #$1, $70(a6)
pub fn i_08de3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE3E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DE3E: cmpi.w #$0, $70(a6)
pub fn i_08de3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE44;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08DE44: bhi.w $8de5e
pub fn i_08de44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE48;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8DE5E; return 10; }
    8
}
/// $08DE48: move.w #$d000, $38(a6)
pub fn i_08de48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DE4E: move.w $36(a6), $28(a6)
pub fn i_08de4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE54;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08DE54: clr.w $2a(a6)
pub fn i_08de54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE58;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DE58: lea.l $8db7a(pc), a1
pub fn i_08de58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB7A;
    16
}
/// $08DE5C: move.l a1, (a6)
pub fn i_08de5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08de5e  ($08DE5E, 6 B, thunks/jsr_pc_thunks.c)
/// $08DE5E: jsr $8efb0(pc)
pub fn i_08de5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE62);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DE62: rts
pub fn i_08de62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl_Standing_08de64  ($08DE64, 88 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DE64: move.b #$8, $99(a6)
pub fn i_08de64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE6A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DE6A: move.b #$1, $98(a6)
pub fn i_08de6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE70;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DE70: jsr $8f108(pc)
pub fn i_08de70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE74);
    cpu.pc = 0x8F108;
    16
}
/// $08DE74: jsr $8f002(pc)
pub fn i_08de74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE78);
    cpu.pc = 0x8F002;
    16
}
/// $08DE78: jsr $8f010(pc)
pub fn i_08de78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE7C);
    cpu.pc = 0x8F010;
    16
}
/// $08DE7C: move.w #$f8, d1
pub fn i_08de7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE80;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DE80: jsr $236e.l
pub fn i_08de80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE86);
    cpu.pc = 0x236E;
    16
}
/// $08DE86: lea.l $8de8c(pc), a1
pub fn i_08de86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DE8C;
    16
}
/// $08DE8A: move.l a1, (a6)
pub fn i_08de8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DE8C: lea.l $2f3c58.l, a0
pub fn i_08de8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE92;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C58;
    20
}
/// $08DE92: jsr $28cd4.l
pub fn i_08de92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE98);
    cpu.pc = 0x28CD4;
    16
}
/// $08DE98: lea.l $8de9e(pc), a1
pub fn i_08de98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DE9E;
    16
}
/// $08DE9C: move.l a1, (a6)
pub fn i_08de9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DE9E: jsr $2783a.l
pub fn i_08de9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DEA4);
    cpu.pc = 0x2783A;
    16
}
/// $08DEA4: jsr $28d70.l
pub fn i_08dea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DEAA);
    cpu.pc = 0x28D70;
    16
}
/// $08DEAA: move.w #$60, d0
pub fn i_08deaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DEAE: jsr $8efce(pc)
pub fn i_08deae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DEB2);
    cpu.pc = 0x8EFCE;
    16
}
/// $08DEB2: bcc.w $8debc
pub fn i_08deb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DEBC; return 10; }
    8
}
/// $08DEB6: lea.l $8dd7e(pc), a1
pub fn i_08deb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD7E;
    16
}
/// $08DEBA: move.l a1, (a6)
pub fn i_08deba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08debc  ($08DEBC, 6 B, thunks/jsr_pc_thunks.c)
/// $08DEBC: jsr $8efb0(pc)
pub fn i_08debc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DEC0);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DEC0: rts
pub fn i_08dec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_PickRand_2f3712_08dec2  ($08DEC2, 22 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DEC2: move.w #$3, d0
pub fn i_08dec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DEC6: jsr $5ea1c.l
pub fn i_08dec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DECC);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DECC: lea.l $2f3712.l, a0
pub fn i_08decc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DED2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3712;
    20
}
/// $08DED2: movea.l #$ffffffff, a1
pub fn i_08ded2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DED8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08ded8  ($08DED8, 8 B, thunks/jsr_abs_thunks.c)
/// $08DED8: jsr $772.l
pub fn i_08ded8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DEDE);
    cpu.pc = 0x772;
    16
}
/// $08DEDE: rts
pub fn i_08dede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Runner_08dee0  ($08DEE0, 98 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DEE0: move.w #$60, $22(a6)
pub fn i_08dee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEE6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DEE6: move.w #$150, $24(a6)
pub fn i_08dee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEEC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DEEC: clr.w $26(a6)
pub fn i_08deec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEF0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DEF0: move.b #$10, $99(a6)
pub fn i_08def0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DEF6: move.b #$1, $98(a6)
pub fn i_08def6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEFC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DEFC: cmpi.b #$0, $99(a6)
pub fn i_08defc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF02;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08DF02: bne.w $8df0c
pub fn i_08df02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8DF0C; return 10; }
    8
}
/// $08DF06: move.b #$10, $99(a6)
pub fn i_08df06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DF0C: jsr $8f108(pc)
pub fn i_08df0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF10);
    cpu.pc = 0x8F108;
    16
}
/// $08DF10: jsr $8f002(pc)
pub fn i_08df10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF14);
    cpu.pc = 0x8F002;
    16
}
/// $08DF14: jsr $8f010(pc)
pub fn i_08df14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF18);
    cpu.pc = 0x8F010;
    16
}
/// $08DF18: move.w #$f9, d1
pub fn i_08df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF1C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DF1C: jsr $236e.l
pub fn i_08df1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF22);
    cpu.pc = 0x236E;
    16
}
/// $08DF22: lea.l $2f3cc0.l, a0
pub fn i_08df22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3CC0;
    20
}
/// $08DF28: jsr $28cd4.l
pub fn i_08df28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF2E);
    cpu.pc = 0x28CD4;
    16
}
/// $08DF2E: lea.l $8df34(pc), a1
pub fn i_08df2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DF34;
    16
}
/// $08DF32: move.l a1, (a6)
pub fn i_08df32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF34;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DF34: jsr $2783a.l
pub fn i_08df34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF3A);
    cpu.pc = 0x2783A;
    16
}
/// $08DF3A: jsr $8d2b0(pc)
pub fn i_08df3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF3E);
    cpu.pc = 0x8D2B0;
    16
}
/// $08DF3E: jsr $8efb0(pc)
pub fn i_08df3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF42);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08df42  ($08DF42, 8 B, thunks/jsr_abs_thunks.c)
/// $08DF42: jsr $28d70.l
pub fn i_08df42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF48);
    cpu.pc = 0x28D70;
    16
}
/// $08DF48: rts
pub fn i_08df48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl153_08df4a  ($08DF4A, 32 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DF4A: jsr $8f108(pc)
pub fn i_08df4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF4E);
    cpu.pc = 0x8F108;
    16
}
/// $08DF4E: jsr $8f002(pc)
pub fn i_08df4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF52);
    cpu.pc = 0x8F002;
    16
}
/// $08DF52: jsr $8f010(pc)
pub fn i_08df52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF56);
    cpu.pc = 0x8F010;
    16
}
/// $08DF56: jsr $8f070(pc)
pub fn i_08df56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF5A);
    cpu.pc = 0x8F070;
    16
}
/// $08DF5A: jsr $8e172(pc)
pub fn i_08df5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF5E);
    cpu.pc = 0x8E172;
    16
}
/// $08DF5E: jsr $27cee.l
pub fn i_08df5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF64);
    cpu.pc = 0x27CEE;
    16
}
/// $08DF64: jsr $28d70.l
pub fn i_08df64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF6A);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_08df6a  ($08DF6A, 8 B, kernel/task_handlers.c)
/// $08DF6A: lea.l $8df72(pc), a1
pub fn i_08df6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DF72;
    16
}
/// $08DF6E: move.l a1, (a6)
pub fn i_08df6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DF70: rts
pub fn i_08df70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Run_08df72  ($08DF72, 24 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DF72: jsr $8d2b0(pc)
pub fn i_08df72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF76);
    cpu.pc = 0x8D2B0;
    16
}
/// $08DF76: jsr $2783a.l
pub fn i_08df76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF7C);
    cpu.pc = 0x2783A;
    16
}
/// $08DF7C: jsr $8d184(pc)
pub fn i_08df7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF80);
    cpu.pc = 0x8D184;
    16
}
/// $08DF80: bcc.w $8df8c
pub fn i_08df80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DF8C; return 10; }
    8
}
/// $08DF84: jmp $518.l
pub fn i_08df84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF8A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08df8a  ($08DF8A, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DF8A: rts
pub fn i_08df8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08df8c  ($08DF8C, 8 B, thunks/jsr_abs_thunks.c)
/// $08DF8C: jsr $28d70.l
pub fn i_08df8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DF92);
    cpu.pc = 0x28D70;
    16
}
/// $08DF92: rts
pub fn i_08df92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl154_08df94  ($08DF94, 28 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DF94: jsr $8f108(pc)
pub fn i_08df94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF98);
    cpu.pc = 0x8F108;
    16
}
/// $08DF98: jsr $8f002(pc)
pub fn i_08df98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DF9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DF9C);
    cpu.pc = 0x8F002;
    16
}
/// $08DF9C: jsr $8f010(pc)
pub fn i_08df9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DFA0);
    cpu.pc = 0x8F010;
    16
}
/// $08DFA0: jsr $8f070(pc)
pub fn i_08dfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DFA4);
    cpu.pc = 0x8F070;
    16
}
/// $08DFA4: jsr $27cee.l
pub fn i_08dfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFAA);
    cpu.pc = 0x27CEE;
    16
}
/// $08DFAA: jsr $28d70.l
pub fn i_08dfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFB0);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_08dfb0  ($08DFB0, 8 B, kernel/task_handlers.c)
/// $08DFB0: lea.l $8dfb8(pc), a1
pub fn i_08dfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DFB8;
    16
}
/// $08DFB4: move.l a1, (a6)
pub fn i_08dfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DFB6: rts
pub fn i_08dfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Rand_Stand_08dfb8  ($08DFB8, 114 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DFB8: lea.l $2f3e4a.l, a0
pub fn i_08dfb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E4A;
    20
}
/// $08DFBE: jsr $28cd4.l
pub fn i_08dfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFC4);
    cpu.pc = 0x28CD4;
    16
}
/// $08DFC4: lea.l $8dfca(pc), a1
pub fn i_08dfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DFCA;
    16
}
/// $08DFC8: move.l a1, (a6)
pub fn i_08dfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DFCA: jsr $2783a.l
pub fn i_08dfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFD0);
    cpu.pc = 0x2783A;
    16
}
/// $08DFD0: jsr $28d70.l
pub fn i_08dfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFD6);
    cpu.pc = 0x28D70;
    16
}
/// $08DFD6: bcc.w $8e01a
pub fn i_08dfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E01A; return 10; }
    8
}
/// $08DFDA: move.w #$60, d0
pub fn i_08dfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DFDE: jsr $8efce(pc)
pub fn i_08dfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DFE2);
    cpu.pc = 0x8EFCE;
    16
}
/// $08DFE2: bcc.w $8e016
pub fn i_08dfe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E016; return 10; }
    8
}
/// $08DFE6: move.w #$f, d0
pub fn i_08dfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DFEA: jsr $5ea1c.l
pub fn i_08dfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DFF0);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DFF0: move.b #$20, $99(a6)
pub fn i_08dff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DFF6: add.b d0, $99(a6)
pub fn i_08dff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DFFA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08DFFA: move.b #$1, $98(a6)
pub fn i_08dffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E000;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E000: jsr $8f002(pc)
pub fn i_08e000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E004;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E004);
    cpu.pc = 0x8F002;
    16
}
/// $08E004: jsr $8f010(pc)
pub fn i_08e004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E008;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E008);
    cpu.pc = 0x8F010;
    16
}
/// $08E008: jsr $8e172(pc)
pub fn i_08e008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E00C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E00C);
    cpu.pc = 0x8E172;
    16
}
/// $08E00C: lea.l $8df72(pc), a1
pub fn i_08e00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E010;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DF72;
    16
}
/// $08E010: move.l a1, (a6)
pub fn i_08e010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E012;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E012: bra.w $8e01a
pub fn i_08e012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E016;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E01A;
    10
}
/// $08E016: jsr $8e09c(pc)
pub fn i_08e016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E01A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E01A);
    cpu.pc = 0x8E09C;
    16
}
/// $08E01A: jsr $8d184(pc)
pub fn i_08e01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E01E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E01E);
    cpu.pc = 0x8D184;
    16
}
/// $08E01E: bcc.w $8e028
pub fn i_08e01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E022;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E028; return 10; }
    8
}
/// $08E022: jmp $518.l
pub fn i_08e022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E028;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E028: rts
pub fn i_08e028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E02A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Rand_Stand2_08e02a  ($08E02A, 54 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E02A: lea.l $2f3e56.l, a0
pub fn i_08e02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E030;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E56;
    20
}
/// $08E030: jsr $28cd4.l
pub fn i_08e030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E036);
    cpu.pc = 0x28CD4;
    16
}
/// $08E036: lea.l $8e03c(pc), a1
pub fn i_08e036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E03A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E03C;
    16
}
/// $08E03A: move.l a1, (a6)
pub fn i_08e03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E03C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E03C: jsr $2783a.l
pub fn i_08e03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E042);
    cpu.pc = 0x2783A;
    16
}
/// $08E042: jsr $28d70.l
pub fn i_08e042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E048);
    cpu.pc = 0x28D70;
    16
}
/// $08E048: bcc.w $8e050
pub fn i_08e048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E04C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E050; return 10; }
    8
}
/// $08E04C: jsr $8e09c(pc)
pub fn i_08e04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E050;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E050);
    cpu.pc = 0x8E09C;
    16
}
/// $08E050: jsr $8d184(pc)
pub fn i_08e050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E054;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E054);
    cpu.pc = 0x8D184;
    16
}
/// $08E054: bcc.w $8e05e
pub fn i_08e054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E058;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E05E; return 10; }
    8
}
/// $08E058: jmp $518.l
pub fn i_08e058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E05E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E05E: rts
pub fn i_08e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E060;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Rand_Walk_08e060  ($08E060, 60 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E060: lea.l $2f3e0c.l, a0
pub fn i_08e060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E066;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E0C;
    20
}
/// $08E066: jsr $28cd4.l
pub fn i_08e066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E06C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E06C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E06C: lea.l $8e072(pc), a1
pub fn i_08e06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E070;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E072;
    16
}
/// $08E070: move.l a1, (a6)
pub fn i_08e070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E072;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E072: jsr $2783a.l
pub fn i_08e072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E078);
    cpu.pc = 0x2783A;
    16
}
/// $08E078: jsr $8d2b0(pc)
pub fn i_08e078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E07C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E07C);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E07C: jsr $28d70.l
pub fn i_08e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E082);
    cpu.pc = 0x28D70;
    16
}
/// $08E082: bcc.w $8e08c
pub fn i_08e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E086;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E08C; return 10; }
    8
}
/// $08E086: lea.l $8dfb8(pc), a1
pub fn i_08e086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E08A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DFB8;
    16
}
/// $08E08A: move.l a1, (a6)
pub fn i_08e08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E08C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E08C: jsr $8d184(pc)
pub fn i_08e08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E090;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E090);
    cpu.pc = 0x8D184;
    16
}
/// $08E090: bcc.w $8e09a
pub fn i_08e090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E094;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E09A; return 10; }
    8
}
/// $08E094: jmp $518.l
pub fn i_08e094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E09A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E09A: rts
pub fn i_08e09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E09C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_PickRand_2f3722_08e09c  ($08E09C, 22 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E09C: move.w #$3, d0
pub fn i_08e09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E0A0: jsr $5ea1c.l
pub fn i_08e0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E0A6);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E0A6: lea.l $2f3722.l, a0
pub fn i_08e0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3722;
    20
}
/// $08E0AC: movea.l #$ffffffff, a1
pub fn i_08e0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0B2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08e0b2  ($08E0B2, 8 B, thunks/jsr_abs_thunks.c)
/// $08E0B2: jsr $772.l
pub fn i_08e0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E0B8);
    cpu.pc = 0x772;
    16
}
/// $08E0B8: rts
pub fn i_08e0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Spawn_08e0ba  ($08E0BA, 74 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E0BA: move.w #$80, $22(a6)
pub fn i_08e0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E0C0: move.w #$150, $24(a6)
pub fn i_08e0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E0C6: clr.w $26(a6)
pub fn i_08e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0CA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E0CA: move.b #$1, $98(a6)
pub fn i_08e0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E0D0: move.b #$2, $9a(a6)
pub fn i_08e0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E0D6: move.b #$8, $99(a6)
pub fn i_08e0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0DC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E0DC: jsr $8f108(pc)
pub fn i_08e0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E0E0);
    cpu.pc = 0x8F108;
    16
}
/// $08E0E0: jsr $8f002(pc)
pub fn i_08e0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E0E4);
    cpu.pc = 0x8F002;
    16
}
/// $08E0E4: jsr $8f010(pc)
pub fn i_08e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E0E8);
    cpu.pc = 0x8F010;
    16
}
/// $08E0E8: jsr $8f070(pc)
pub fn i_08e0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E0EC);
    cpu.pc = 0x8F070;
    16
}
/// $08E0EC: jsr $27cee.l
pub fn i_08e0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E0F2);
    cpu.pc = 0x27CEE;
    16
}
/// $08E0F2: jsr $28d70.l
pub fn i_08e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E0F8);
    cpu.pc = 0x28D70;
    16
}
/// $08E0F8: lea.l $8e10c(pc), a1
pub fn i_08e0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E10C;
    16
}
/// $08E0FC: move.l a1, (a6)
pub fn i_08e0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E0FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E0FE: lea.l $2f3e0c.l, a0
pub fn i_08e0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E104;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E0C;
    20
}
// ===== JsrAbsThunk_08e104  ($08E104, 8 B, thunks/jsr_abs_thunks.c)
/// $08E104: jsr $28cd4.l
pub fn i_08e104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E10A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E10A);
    cpu.pc = 0x28CD4;
    16
}
/// $08E10A: rts
pub fn i_08e10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E10C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt2_Run_08e10c  ($08E10C, 102 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E10C: jsr $8d2b0(pc)
pub fn i_08e10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E110;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E110);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E110: jsr $28d70.l
pub fn i_08e110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E116);
    cpu.pc = 0x28D70;
    16
}
/// $08E116: bcc.w $8e162
pub fn i_08e116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E11A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E162; return 10; }
    8
}
/// $08E11A: move.w #$60, d0
pub fn i_08e11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E11E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E11E: jsr $8efce(pc)
pub fn i_08e11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E122;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E122);
    cpu.pc = 0x8EFCE;
    16
}
/// $08E122: bcc.w $8e156
pub fn i_08e122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E126;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E156; return 10; }
    8
}
/// $08E126: move.w #$f, d0
pub fn i_08e126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E12A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E12A: jsr $5ea1c.l
pub fn i_08e12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E130);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E130: move.b #$20, $99(a6)
pub fn i_08e130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E136;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E136: add.b d0, $99(a6)
pub fn i_08e136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E13A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08E13A: move.b #$1, $98(a6)
pub fn i_08e13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E140;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E140: jsr $8f002(pc)
pub fn i_08e140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E144;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E144);
    cpu.pc = 0x8F002;
    16
}
/// $08E144: jsr $8f010(pc)
pub fn i_08e144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E148;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E148);
    cpu.pc = 0x8F010;
    16
}
/// $08E148: jsr $8e172(pc)
pub fn i_08e148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E14C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E14C);
    cpu.pc = 0x8E172;
    16
}
/// $08E14C: lea.l $8df72(pc), a1
pub fn i_08e14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E150;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DF72;
    16
}
/// $08E150: move.l a1, (a6)
pub fn i_08e150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E152;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E152: bra.w $8e162
pub fn i_08e152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E156;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E162;
    10
}
/// $08E156: lea.l $2f3e0c.l, a0
pub fn i_08e156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E15C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E0C;
    20
}
/// $08E15C: jsr $28cd4.l
pub fn i_08e15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E162);
    cpu.pc = 0x28CD4;
    16
}
/// $08E162: jsr $8d184(pc)
pub fn i_08e162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E166;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E166);
    cpu.pc = 0x8D184;
    16
}
/// $08E166: bcc.w $8e170
pub fn i_08e166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E16A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E170; return 10; }
    8
}
/// $08E16A: jmp $518.l
pub fn i_08e16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E170;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E170: rts
pub fn i_08e170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E172;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_LoadMapByDir_08e172  ($08E172, 34 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E172: clr.w d0
pub fn i_08e172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E174;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08E174: move.b $99(a6), d0
pub fn i_08e174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E178;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08E178: lsr.b #$4, d0
pub fn i_08e178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E17A;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $08E17A: andi.b #$7, d0
pub fn i_08e17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E17E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08E17E: movea.l #$2f3cdc, a0
pub fn i_08e17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E184;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F3CDC;
    cpu.a[0] = v1;
    4
}
/// $08E184: lsl.w #$2, d0
pub fn i_08e184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E186;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08E186: movea.l (a0, d0.w), a0
pub fn i_08e186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E18A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08E18A: cmpa.l #$ffffffff, a0
pub fn i_08e18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E190;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08E190: beq.w $8e19a
pub fn i_08e190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E194;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8E19A; return 10; }
    8
}
// ===== JsrAbsThunk_08e194  ($08E194, 8 B, thunks/jsr_abs_thunks.c)
/// $08E194: jsr $28cd4.l
pub fn i_08e194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E19A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E19A);
    cpu.pc = 0x28CD4;
    16
}
/// $08E19A: rts
pub fn i_08e19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E19C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl160_08e19c  ($08E19C, 86 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E19C: jsr $8f108(pc)
pub fn i_08e19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1A0);
    cpu.pc = 0x8F108;
    16
}
/// $08E1A0: jsr $8f002(pc)
pub fn i_08e1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1A4);
    cpu.pc = 0x8F002;
    16
}
/// $08E1A4: jsr $8f02c(pc)
pub fn i_08e1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1A8);
    cpu.pc = 0x8F02C;
    16
}
/// $08E1A8: move.w #$fd, d1
pub fn i_08e1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1AC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E1AC: jsr $236e.l
pub fn i_08e1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1B2);
    cpu.pc = 0x236E;
    16
}
/// $08E1B2: lea.l $2f3e86.l, a0
pub fn i_08e1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E86;
    20
}
/// $08E1B8: jsr $28cd4.l
pub fn i_08e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1BE);
    cpu.pc = 0x28CD4;
    16
}
/// $08E1BE: lea.l $8e1c4(pc), a1
pub fn i_08e1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E1C4;
    16
}
/// $08E1C2: move.l a1, (a6)
pub fn i_08e1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E1C4: jsr $8f040(pc)
pub fn i_08e1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1C8);
    cpu.pc = 0x8F040;
    16
}
/// $08E1C8: bcc.w $8e1de
pub fn i_08e1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E1DE; return 10; }
    8
}
/// $08E1CC: lea.l $2f3e92.l, a0
pub fn i_08e1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E92;
    20
}
/// $08E1D2: jsr $28cd4.l
pub fn i_08e1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1D8);
    cpu.pc = 0x28CD4;
    16
}
/// $08E1D8: lea.l $8e1de(pc), a1
pub fn i_08e1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E1DE;
    16
}
/// $08E1DC: move.l a1, (a6)
pub fn i_08e1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E1DE: jsr $2783a.l
pub fn i_08e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1E4);
    cpu.pc = 0x2783A;
    16
}
/// $08E1E4: jsr $8d1e8(pc)
pub fn i_08e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E1E8);
    cpu.pc = 0x8D1E8;
    16
}
/// $08E1E8: bcc.w $8e1f2
pub fn i_08e1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E1F2; return 10; }
    8
}
/// $08E1EC: jmp $518.l
pub fn i_08e1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_08e1f2  ($08E1F2, 8 B, thunks/jsr_abs_thunks.c)
/// $08E1F2: jsr $28d70.l
pub fn i_08e1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E1F8);
    cpu.pc = 0x28D70;
    16
}
/// $08E1F8: rts
pub fn i_08e1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E1FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sentry_Init_08e1fa  ($08E1FA, 134 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E1FA: move.b #$1, $98(a6)
pub fn i_08e1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E200;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E200: move.b #$20, $99(a6)
pub fn i_08e200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E206;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E206: move.b #$14, $9a(a6)
pub fn i_08e206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E20C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E20C: move.w #$40, $22(a6)
pub fn i_08e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E212;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E212: move.w #$180, $24(a6)
pub fn i_08e212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E218;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E218: clr.w $26(a6)
pub fn i_08e218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E21C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E21C: jsr $8f002(pc)
pub fn i_08e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E220;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E220);
    cpu.pc = 0x8F002;
    16
}
/// $08E220: jsr $8f010(pc)
pub fn i_08e220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E224;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E224);
    cpu.pc = 0x8F010;
    16
}
/// $08E224: eori.b #$1, $3a(a6)
pub fn i_08e224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E22A;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E22A: move.w #$fe, d1
pub fn i_08e22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E22E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E22E: jsr $236e.l
pub fn i_08e22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E234;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E234);
    cpu.pc = 0x236E;
    16
}
/// $08E234: lea.l $2f3e9e.l, a0
pub fn i_08e234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E23A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E9E;
    20
}
/// $08E23A: jsr $28cd4.l
pub fn i_08e23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E240);
    cpu.pc = 0x28CD4;
    16
}
/// $08E240: move.w #$ffff, $38(a6)
pub fn i_08e240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E246;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E246: bset.b #$6, $12(a6)
pub fn i_08e246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E24C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E24C: lea.l $8e252(pc), a1
pub fn i_08e24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E250;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E252;
    16
}
/// $08E250: move.l a1, (a6)
pub fn i_08e250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E252;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E252: jsr $8efb0(pc)
pub fn i_08e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E256;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E256);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E256: jsr $28d70.l
pub fn i_08e256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E25C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E25C);
    cpu.pc = 0x28D70;
    16
}
/// $08E25C: bcc.w $8e286
pub fn i_08e25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E260;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E286; return 10; }
    8
}
/// $08E260: cmpi.b #$0, $98(a6)
pub fn i_08e260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E266;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E266: bne.w $8e274
pub fn i_08e266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E26A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E274; return 10; }
    8
}
/// $08E26A: subi.w #$48, $22(a6)
pub fn i_08e26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E270;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08E270: bra.w $8e27a
pub fn i_08e270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E274;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E27A;
    10
}
/// $08E274: addi.w #$48, $22(a6)
pub fn i_08e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E27A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08E27A: move.b $9a(a6), $70(a6)
pub fn i_08e27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E280;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetTaskHandler_08e280  ($08E280, 8 B, kernel/task_handlers.c)
/// $08E280: lea.l $8e288(pc), a1
pub fn i_08e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E284;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E288;
    16
}
/// $08E284: move.l a1, (a6)
pub fn i_08e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E286;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E286: rts
pub fn i_08e286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E288;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sentry_Wait_08e288  ($08E288, 30 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E288: subq.b #$1, $70(a6)
pub fn i_08e288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E28C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08E28C: bne.w $8e2a2
pub fn i_08e28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E290;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E2A2; return 10; }
    8
}
/// $08E290: lea.l $2f3e9e.l, a0
pub fn i_08e290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E296;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3E9E;
    20
}
/// $08E296: jsr $28cd4.l
pub fn i_08e296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E29C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E29C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E29C: lea.l $8e252(pc), a1
pub fn i_08e29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E252;
    16
}
/// $08E2A0: move.l a1, (a6)
pub fn i_08e2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E2A2: jsr $8d2f8(pc)
pub fn i_08e2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2A6);
    cpu.pc = 0x8D2F8;
    16
}
// ===== JsrPcThunk_08e2a6  ($08E2A6, 6 B, thunks/jsr_pc_thunks.c)
/// $08E2A6: jsr $8efb0(pc)
pub fn i_08e2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2AA);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E2AA: rts
pub fn i_08e2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl180_08e2ac  ($08E2AC, 60 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E2AC: jsr $8f108(pc)
pub fn i_08e2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2B0);
    cpu.pc = 0x8F108;
    16
}
/// $08E2B0: jsr $8f002(pc)
pub fn i_08e2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2B4);
    cpu.pc = 0x8F002;
    16
}
/// $08E2B4: jsr $8f02c(pc)
pub fn i_08e2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2B8);
    cpu.pc = 0x8F02C;
    16
}
/// $08E2B8: move.w #$124, d1
pub fn i_08e2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2BC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E2BC: jsr $236e.l
pub fn i_08e2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E2C2);
    cpu.pc = 0x236E;
    16
}
/// $08E2C2: lea.l $2f3f4c.l, a0
pub fn i_08e2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3F4C;
    20
}
/// $08E2C8: jsr $28cd4.l
pub fn i_08e2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E2CE);
    cpu.pc = 0x28CD4;
    16
}
/// $08E2CE: move.w #$ffff, $38(a6)
pub fn i_08e2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E2D4: bset.b #$6, $12(a6)
pub fn i_08e2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2DA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E2DA: lea.l $8e2f0(pc), a1
pub fn i_08e2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E2F0;
    16
}
/// $08E2DE: move.l a1, (a6)
pub fn i_08e2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E2E0: jsr $8f040(pc)
pub fn i_08e2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2E4);
    cpu.pc = 0x8F040;
    16
}
/// $08E2E4: bcc.w $8e2ee
pub fn i_08e2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E2EE; return 10; }
    8
}
// ===== SetTaskHandler_08e2e8  ($08E2E8, 8 B, kernel/task_handlers.c)
/// $08E2E8: lea.l $8e2f0(pc), a1
pub fn i_08e2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E2F0;
    16
}
/// $08E2EC: move.l a1, (a6)
pub fn i_08e2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E2EE: rts
pub fn i_08e2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl180_Run_08e2f0  ($08E2F0, 28 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E2F0: jsr $2783a.l
pub fn i_08e2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E2F6);
    cpu.pc = 0x2783A;
    16
}
/// $08E2F6: jsr $8efb0(pc)
pub fn i_08e2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E2FA);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E2FA: jsr $28d70.l
pub fn i_08e2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E300);
    cpu.pc = 0x28D70;
    16
}
/// $08E300: bcc.w $8e30a
pub fn i_08e300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E304;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E30A; return 10; }
    8
}
/// $08E304: jmp $518.l
pub fn i_08e304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E30A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08E30A: rts
pub fn i_08e30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E30C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swinger_Tmpl181_08e30c  ($08E30C, 174 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E30C: jsr $8f002(pc)
pub fn i_08e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E310;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E310);
    cpu.pc = 0x8F002;
    16
}
/// $08E310: jsr $8f010(pc)
pub fn i_08e310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E314;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E314);
    cpu.pc = 0x8F010;
    16
}
/// $08E314: asr.w $28(a6)
pub fn i_08e314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E318;
    cpu.ir = 0xE0EE;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = do_shift_rotate(cpu, v2, 1, Size::Word, 0, false);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08E318: move.w #$125, d1
pub fn i_08e318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E31C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E31C: jsr $236e.l
pub fn i_08e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E322);
    cpu.pc = 0x236E;
    16
}
/// $08E322: move.w #$ffff, $38(a6)
pub fn i_08e322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E328;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E328: bset.b #$6, $12(a6)
pub fn i_08e328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E32E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E32E: move.w $28(a6), $74(a6)
pub fn i_08e32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E334;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08E334: lea.l $2f3fc6.l, a0
pub fn i_08e334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E33A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3FC6;
    20
}
/// $08E33A: jsr $28cd4.l
pub fn i_08e33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E340);
    cpu.pc = 0x28CD4;
    16
}
/// $08E340: clr.w $28(a6)
pub fn i_08e340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E344;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E344: cmpi.b #$0, $98(a6)
pub fn i_08e344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E34A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E34A: bne.w $8e358
pub fn i_08e34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E34E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E358; return 10; }
    8
}
/// $08E34E: move.w #$fff8, $2c(a6)
pub fn i_08e34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E354;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E354: bra.w $8e35e
pub fn i_08e354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E358;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E35E;
    10
}
/// $08E358: move.w #$8, $2c(a6)
pub fn i_08e358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E35E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E35E: jsr $8f002(pc)
pub fn i_08e35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E362;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E362);
    cpu.pc = 0x8F002;
    16
}
/// $08E362: lea.l $8e368(pc), a1
pub fn i_08e362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E366;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E368;
    16
}
/// $08E366: move.l a1, (a6)
pub fn i_08e366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E368;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E368: jsr $8d2b0(pc)
pub fn i_08e368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E36C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E36C);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E36C: jsr $2783a.l
pub fn i_08e36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E372;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E372);
    cpu.pc = 0x2783A;
    16
}
/// $08E372: move.w $2c(a6), d0
pub fn i_08e372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E376;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08E376: add.w d0, $28(a6)
pub fn i_08e376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E37A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E37A: cmpi.b #$0, $98(a6)
pub fn i_08e37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E380;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E380: bne.w $8e39a
pub fn i_08e380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E384;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E39A; return 10; }
    8
}
/// $08E384: move.w $28(a6), d0
pub fn i_08e384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E388;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08E388: cmp.w $74(a6), d0
pub fn i_08e388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E38C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08E38C: bgt.w $8e396
pub fn i_08e38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E390;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E396; return 10; }
    8
}
/// $08E390: lea.l $8e3c4(pc), a1
pub fn i_08e390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E394;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E3C4;
    16
}
/// $08E394: move.l a1, (a6)
pub fn i_08e394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E396;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E396: bra.w $8e3ac
pub fn i_08e396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E39A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E3AC;
    10
}
/// $08E39A: move.w $28(a6), d0
pub fn i_08e39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E39E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08E39E: cmp.w $74(a6), d0
pub fn i_08e39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3A2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08E3A2: blt.w $8e3ac
pub fn i_08e3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3A6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E3AC; return 10; }
    8
}
/// $08E3A6: lea.l $8e3c4(pc), a1
pub fn i_08e3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E3C4;
    16
}
/// $08E3AA: move.l a1, (a6)
pub fn i_08e3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E3AC: jsr $8d184(pc)
pub fn i_08e3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E3B0);
    cpu.pc = 0x8D184;
    16
}
/// $08E3B0: bcc.w $8e3bc
pub fn i_08e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E3BC; return 10; }
    8
}
/// $08E3B4: jmp $518.l
pub fn i_08e3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3BA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e3ba  ($08E3BA, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E3BA: rts
pub fn i_08e3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08e3bc  ($08E3BC, 8 B, thunks/jsr_abs_thunks.c)
/// $08E3BC: jsr $28d70.l
pub fn i_08e3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E3C2);
    cpu.pc = 0x28D70;
    16
}
/// $08E3C2: rts
pub fn i_08e3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swinger_Pause_08e3c4  ($08E3C4, 64 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E3C4: move.w #$7, d0
pub fn i_08e3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3C8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E3C8: jsr $5ea1c.l
pub fn i_08e3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E3CE);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E3CE: add.w d0, $70(a6)
pub fn i_08e3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3D2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E3D2: lea.l $8e3d8(pc), a1
pub fn i_08e3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E3D8;
    16
}
/// $08E3D6: move.l a1, (a6)
pub fn i_08e3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E3D8: subq.w #$1, $70(a6)
pub fn i_08e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3DC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E3DC: cmpi.w #$0, $70(a6)
pub fn i_08e3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3E2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E3E2: bgt.w $8e3ec
pub fn i_08e3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3E6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E3EC; return 10; }
    8
}
/// $08E3E6: lea.l $8e40e(pc), a1
pub fn i_08e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E40E;
    16
}
/// $08E3EA: move.l a1, (a6)
pub fn i_08e3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E3EC: jsr $8d2b0(pc)
pub fn i_08e3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E3F0);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E3F0: jsr $2783a.l
pub fn i_08e3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E3F6);
    cpu.pc = 0x2783A;
    16
}
/// $08E3F6: jsr $8d184(pc)
pub fn i_08e3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E3FA);
    cpu.pc = 0x8D184;
    16
}
/// $08E3FA: bcc.w $8e406
pub fn i_08e3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E3FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E406; return 10; }
    8
}
/// $08E3FE: jmp $518.l
pub fn i_08e3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E404;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e404  ($08E404, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E404: rts
pub fn i_08e404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E406;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08e406  ($08E406, 8 B, thunks/jsr_abs_thunks.c)
/// $08E406: jsr $28d70.l
pub fn i_08e406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E40C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E40C);
    cpu.pc = 0x28D70;
    16
}
/// $08E40C: rts
pub fn i_08e40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E40E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swinger_Back_08e40e  ($08E40E, 138 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E40E: move.w #$7, d0
pub fn i_08e40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E412;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E412: jsr $5ea1c.l
pub fn i_08e412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E418);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E418: addi.w #$c, d0
pub fn i_08e418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E41C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08E41C: move.w d0, $70(a6)
pub fn i_08e41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E420;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E420: neg.w $2c(a6)
pub fn i_08e420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E424;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E424: lea.l $2f3fe2.l, a0
pub fn i_08e424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E42A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3FE2;
    20
}
/// $08E42A: jsr $28cd4.l
pub fn i_08e42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E430);
    cpu.pc = 0x28CD4;
    16
}
/// $08E430: lea.l $8e436(pc), a1
pub fn i_08e430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E434;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E436;
    16
}
/// $08E434: move.l a1, (a6)
pub fn i_08e434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E436;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E436: jsr $8d2b0(pc)
pub fn i_08e436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E43A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E43A);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E43A: jsr $2783a.l
pub fn i_08e43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E440);
    cpu.pc = 0x2783A;
    16
}
/// $08E440: move.w $2c(a6), d0
pub fn i_08e440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E444;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08E444: add.w d0, $28(a6)
pub fn i_08e444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E448;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E448: subq.w #$1, $70(a6)
pub fn i_08e448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E44C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E44C: cmpi.w #$0, $70(a6)
pub fn i_08e44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E452;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E452: bgt.w $8e45c
pub fn i_08e452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E456;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E45C; return 10; }
    8
}
/// $08E456: lea.l $8e4a2(pc), a1
pub fn i_08e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E45A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4A2;
    16
}
/// $08E45A: move.l a1, (a6)
pub fn i_08e45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E45C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E45C: cmpi.b #$0, $98(a6)
pub fn i_08e45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E462;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E462: bne.w $8e47a
pub fn i_08e462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E466;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8E47A; return 10; }
    8
}
/// $08E466: cmpi.w #$0, $28(a6)
pub fn i_08e466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E46C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E46C: blt.w $8e476
pub fn i_08e46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E470;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E476; return 10; }
    8
}
/// $08E470: lea.l $8e4a2(pc), a1
pub fn i_08e470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E474;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4A2;
    16
}
/// $08E474: move.l a1, (a6)
pub fn i_08e474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E476;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E476: bra.w $8e48a
pub fn i_08e476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E47A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E48A;
    10
}
/// $08E47A: cmpi.w #$0, $28(a6)
pub fn i_08e47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E480;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E480: bgt.w $8e48a
pub fn i_08e480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E484;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E48A; return 10; }
    8
}
/// $08E484: lea.l $8e4a2(pc), a1
pub fn i_08e484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E488;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4A2;
    16
}
/// $08E488: move.l a1, (a6)
pub fn i_08e488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E48A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E48A: jsr $8d184(pc)
pub fn i_08e48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E48E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E48E);
    cpu.pc = 0x8D184;
    16
}
/// $08E48E: bcc.w $8e49a
pub fn i_08e48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E492;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E49A; return 10; }
    8
}
/// $08E492: jmp $518.l
pub fn i_08e492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E498;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e498  ($08E498, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E498: rts
pub fn i_08e498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E49A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08e49a  ($08E49A, 8 B, thunks/jsr_abs_thunks.c)
/// $08E49A: jsr $28d70.l
pub fn i_08e49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4A0);
    cpu.pc = 0x28D70;
    16
}
/// $08E4A0: rts
pub fn i_08e4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swinger_Turn_08e4a2  ($08E4A2, 64 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E4A2: lea.l $2f3fee.l, a0
pub fn i_08e4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3FEE;
    20
}
/// $08E4A8: jsr $28cd4.l
pub fn i_08e4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4AE);
    cpu.pc = 0x28CD4;
    16
}
/// $08E4AE: lea.l $8e4b4(pc), a1
pub fn i_08e4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4B4;
    16
}
/// $08E4B2: move.l a1, (a6)
pub fn i_08e4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E4B4: jsr $2783a.l
pub fn i_08e4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4BA);
    cpu.pc = 0x2783A;
    16
}
/// $08E4BA: jsr $28d70.l
pub fn i_08e4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4C0);
    cpu.pc = 0x28D70;
    16
}
/// $08E4C0: bcc.w $8e4d4
pub fn i_08e4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E4D4; return 10; }
    8
}
/// $08E4C4: neg.w $74(a6)
pub fn i_08e4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4C8;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E4C8: eori.b #$1, $98(a6)
pub fn i_08e4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4CE;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E4CE: lea.l $8e334(pc), a1
pub fn i_08e4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E334;
    16
}
/// $08E4D2: move.l a1, (a6)
pub fn i_08e4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E4D4: jsr $8d184(pc)
pub fn i_08e4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E4D8);
    cpu.pc = 0x8D184;
    16
}
/// $08E4D8: bcc.w $8e4e4
pub fn i_08e4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E4E4; return 10; }
    8
}
/// $08E4DC: jmp $518.l
pub fn i_08e4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4E2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e4e2  ($08E4E2, 2 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08E4E2: rts
pub fn i_08e4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0008E4E4  ($08E4E4, 2 B, thunks/trivial_rts.c)
/// $08E4E4: rts
pub fn i_08e4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_Tmpl182_08e4e6  ($08E4E6, 24 B, enemies/critters_rings_08e4e6.s)
/// $08E4E6: move.w #$c000, $38(a6)
pub fn i_08e4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E4EC: jsr $8f002(pc)
pub fn i_08e4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E4F0);
    cpu.pc = 0x8F002;
    16
}
/// $08E4F0: jsr $8f010(pc)
pub fn i_08e4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E4F4);
    cpu.pc = 0x8F010;
    16
}
/// $08E4F4: move.w #$12a, d1
pub fn i_08e4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E4F8: jsr $236e.l
pub fn i_08e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4FE);
    cpu.pc = 0x236E;
    16
}
// ===== Bobber_Descend_08e4fe  ($08E4FE, 70 B, enemies/critters_rings_08e4e6.s)
/// $08E4FE: move.w #$4, $2e(a6)
pub fn i_08e4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E504;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E504: move.w #$6, $70(a6)
pub fn i_08e504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E50A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E50A: move.w #$7, d0
pub fn i_08e50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E50E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E50E: jsr $5ea1c.l
pub fn i_08e50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E514);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E514: add.w d0, $70(a6)
pub fn i_08e514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E518;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E518: lea.l $2f4036.l, a0
pub fn i_08e518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E51E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4036;
    20
}
/// $08E51E: jsr $28cd4.l
pub fn i_08e51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E524);
    cpu.pc = 0x28CD4;
    16
}
/// $08E524: lea.l $8e52a(pc), a1
pub fn i_08e524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E528;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E52A;
    16
}
/// $08E528: move.l a1, (a6)
pub fn i_08e528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E52A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E52A: jsr $2783a.l
pub fn i_08e52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E530);
    cpu.pc = 0x2783A;
    16
}
/// $08E530: jsr $8d34e(pc)
pub fn i_08e530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E534;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E534);
    cpu.pc = 0x8D34E;
    16
}
/// $08E534: jsr $28d70.l
pub fn i_08e534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E53A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E53A);
    cpu.pc = 0x28D70;
    16
}
/// $08E53A: cmpi.w #$1d0, $24(a6)
pub fn i_08e53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E540;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1D0, Size::Word, false);
    16
}
/// $08E540: blt.w $8e54c
pub fn i_08e540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E544;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E54C; return 10; }
    8
}
// ===== SetTaskHandler_08e544  ($08E544, 8 B, kernel/task_handlers.c)
/// $08E544: lea.l $8e566(pc), a1
pub fn i_08e544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E548;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E566;
    16
}
/// $08E548: move.l a1, (a6)
pub fn i_08e548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E54A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E54A: rts
pub fn i_08e54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E54C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_DescendEnd_08e54c  ($08E54C, 20 B, enemies/critters_rings_08e4e6.s)
/// $08E54C: subq.w #$1, $70(a6)
pub fn i_08e54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E550;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E550: cmpi.w #$0, $70(a6)
pub fn i_08e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E556;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E556: bhi.w $8e560
pub fn i_08e556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E55A;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8E560; return 10; }
    8
}
/// $08E55A: lea.l $8e566(pc), a1
pub fn i_08e55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E55E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E566;
    16
}
/// $08E55E: move.l a1, (a6)
pub fn i_08e55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E560;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08e560  ($08E560, 6 B, thunks/jsr_pc_thunks.c)
/// $08E560: jsr $8efb0(pc)
pub fn i_08e560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E564;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E564);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E564: rts
pub fn i_08e564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E566;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_Ascend_08e566  ($08E566, 70 B, enemies/critters_rings_08e4e6.s)
/// $08E566: move.w #$fffc, $2e(a6)
pub fn i_08e566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E56C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E56C: move.w #$6, $70(a6)
pub fn i_08e56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E572;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E572: move.w #$7, d0
pub fn i_08e572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E576;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E576: jsr $5ea1c.l
pub fn i_08e576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E57C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E57C);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E57C: add.w d0, $70(a6)
pub fn i_08e57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E580;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E580: lea.l $2f4036.l, a0
pub fn i_08e580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E586;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4036;
    20
}
/// $08E586: jsr $28cd4.l
pub fn i_08e586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E58C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E58C: lea.l $8e592(pc), a1
pub fn i_08e58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E590;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E592;
    16
}
/// $08E590: move.l a1, (a6)
pub fn i_08e590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E592;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E592: jsr $2783a.l
pub fn i_08e592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E598;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E598);
    cpu.pc = 0x2783A;
    16
}
/// $08E598: jsr $8d34e(pc)
pub fn i_08e598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E59C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E59C);
    cpu.pc = 0x8D34E;
    16
}
/// $08E59C: jsr $28d70.l
pub fn i_08e59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E5A2);
    cpu.pc = 0x28D70;
    16
}
/// $08E5A2: cmpi.w #$160, $24(a6)
pub fn i_08e5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5A8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $08E5A8: bge.w $8e5b4
pub fn i_08e5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5AC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8E5B4; return 10; }
    8
}
// ===== SetTaskHandler_08e5ac  ($08E5AC, 8 B, kernel/task_handlers.c)
/// $08E5AC: lea.l $8e4fe(pc), a1
pub fn i_08e5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4FE;
    16
}
/// $08E5B0: move.l a1, (a6)
pub fn i_08e5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E5B2: rts
pub fn i_08e5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_AscendEnd_08e5b4  ($08E5B4, 20 B, enemies/critters_rings_08e4e6.s)
/// $08E5B4: subq.w #$1, $70(a6)
pub fn i_08e5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E5B8: cmpi.w #$0, $70(a6)
pub fn i_08e5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5BE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E5BE: bhi.w $8e5c8
pub fn i_08e5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C2;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8E5C8; return 10; }
    8
}
/// $08E5C2: lea.l $8e4fe(pc), a1
pub fn i_08e5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4FE;
    16
}
/// $08E5C6: move.l a1, (a6)
pub fn i_08e5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08e5c8  ($08E5C8, 6 B, thunks/jsr_pc_thunks.c)
/// $08E5C8: jsr $8efb0(pc)
pub fn i_08e5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5CC);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E5CC: rts
pub fn i_08e5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Leaper_Tmpl183_08e5ce  ($08E5CE, 76 B, enemies/critters_rings_08e4e6.s)
/// $08E5CE: jsr $8f108(pc)
pub fn i_08e5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5D2);
    cpu.pc = 0x8F108;
    16
}
/// $08E5D2: jsr $8f002(pc)
pub fn i_08e5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5D6);
    cpu.pc = 0x8F002;
    16
}
/// $08E5D6: jsr $8f010(pc)
pub fn i_08e5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5DA);
    cpu.pc = 0x8F010;
    16
}
/// $08E5DA: move.w #$fd00, $2a(a6)
pub fn i_08e5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E5E0: move.w #$10, $2e(a6)
pub fn i_08e5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E5E6: move.w #$124, d1
pub fn i_08e5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E5EA: jsr $236e.l
pub fn i_08e5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E5F0);
    cpu.pc = 0x236E;
    16
}
/// $08E5F0: lea.l $2f4072.l, a0
pub fn i_08e5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4072;
    20
}
/// $08E5F6: jsr $28cd4.l
pub fn i_08e5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E5FC);
    cpu.pc = 0x28CD4;
    16
}
/// $08E5FC: jsr $27cee.l
pub fn i_08e5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E602);
    cpu.pc = 0x27CEE;
    16
}
/// $08E602: jsr $28d70.l
pub fn i_08e602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E608);
    cpu.pc = 0x28D70;
    16
}
/// $08E608: move.w #$f, d0
pub fn i_08e608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E60C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E60C: jsr $5ea1c.l
pub fn i_08e60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E612);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E612: addi.w #$c, d0
pub fn i_08e612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E616;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08E616: move.w d0, $70(a6)
pub fn i_08e616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E61A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08e61a  ($08E61A, 8 B, kernel/task_handlers.c)
/// $08E61A: lea.l $8e622(pc), a1
pub fn i_08e61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E61E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E622;
    16
}
/// $08E61E: move.l a1, (a6)
pub fn i_08e61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E620;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E620: rts
pub fn i_08e620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E622;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Leaper_Jump_08e622  ($08E622, 50 B, enemies/critters_rings_08e4e6.s)
/// $08E622: subq.w #$1, $70(a6)
pub fn i_08e622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E626;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E626: cmpi.w #$0, $70(a6)
pub fn i_08e626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E62C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E62C: bgt.w $8e636
pub fn i_08e62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E630;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E636; return 10; }
    8
}
/// $08E630: lea.l $8e65c(pc), a1
pub fn i_08e630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E634;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E65C;
    16
}
/// $08E634: move.l a1, (a6)
pub fn i_08e634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E636;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E636: cmpi.w #$40, $2a(a6)
pub fn i_08e636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E63C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Word, false);
    16
}
/// $08E63C: blt.w $8e646
pub fn i_08e63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E640;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E646; return 10; }
    8
}
/// $08E640: lea.l $8e65c(pc), a1
pub fn i_08e640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E644;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E65C;
    16
}
/// $08E644: move.l a1, (a6)
pub fn i_08e644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E646;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E646: jsr $8d34e(pc)
pub fn i_08e646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E64A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E64A);
    cpu.pc = 0x8D34E;
    16
}
/// $08E64A: jsr $2783a.l
pub fn i_08e64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E650);
    cpu.pc = 0x2783A;
    16
}
/// $08E650: jsr $8efb0(pc)
pub fn i_08e650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E654;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E654);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08e654  ($08E654, 8 B, thunks/jsr_abs_thunks.c)
/// $08E654: jsr $28d70.l
pub fn i_08e654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E65A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E65A);
    cpu.pc = 0x28D70;
    16
}
/// $08E65A: rts
pub fn i_08e65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E65C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Leaper_Hop_08e65c  ($08E65C, 84 B, enemies/critters_rings_08e4e6.s)
/// $08E65C: move.w #$3f, d0
pub fn i_08e65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E660;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E660: jsr $5ea1c.l
pub fn i_08e660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E666);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E666: addi.w #$40, d0
pub fn i_08e666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E66A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08E66A: neg.w d0
pub fn i_08e66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E66C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08E66C: move.w d0, $2a(a6)
pub fn i_08e66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E670;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E670: move.w #$4, $2e(a6)
pub fn i_08e670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E676;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E676: move.w #$b, $70(a6)
pub fn i_08e676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E67C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E67C: lea.l $2f407e.l, a0
pub fn i_08e67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E682;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F407E;
    20
}
/// $08E682: jsr $28cd4.l
pub fn i_08e682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E688);
    cpu.pc = 0x28CD4;
    16
}
/// $08E688: lea.l $8e68e(pc), a1
pub fn i_08e688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E68C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E68E;
    16
}
/// $08E68C: move.l a1, (a6)
pub fn i_08e68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E68E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E68E: subq.w #$1, $70(a6)
pub fn i_08e68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E692;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E692: cmpi.w #$0, $70(a6)
pub fn i_08e692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E698;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E698: bgt.w $8e6a2
pub fn i_08e698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E69C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8E6A2; return 10; }
    8
}
/// $08E69C: lea.l $8e6b8(pc), a1
pub fn i_08e69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E6B8;
    16
}
/// $08E6A0: move.l a1, (a6)
pub fn i_08e6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E6A2: jsr $8d34e(pc)
pub fn i_08e6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6A6);
    cpu.pc = 0x8D34E;
    16
}
/// $08E6A6: jsr $2783a.l
pub fn i_08e6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6AC);
    cpu.pc = 0x2783A;
    16
}
/// $08E6AC: jsr $8efb0(pc)
pub fn i_08e6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6B0);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08e6b0  ($08E6B0, 8 B, thunks/jsr_abs_thunks.c)
/// $08E6B0: jsr $28d70.l
pub fn i_08e6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6B6);
    cpu.pc = 0x28D70;
    16
}
/// $08E6B6: rts
pub fn i_08e6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Leaper_Fall_08e6b8  ($08E6B8, 32 B, enemies/critters_rings_08e4e6.s)
/// $08E6B8: lea.l $2f408a.l, a0
pub fn i_08e6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F408A;
    20
}
/// $08E6BE: jsr $28cd4.l
pub fn i_08e6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6C4);
    cpu.pc = 0x28CD4;
    16
}
/// $08E6C4: lea.l $8e6ca(pc), a1
pub fn i_08e6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E6CA;
    16
}
/// $08E6C8: move.l a1, (a6)
pub fn i_08e6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E6CA: jsr $8d2f8(pc)
pub fn i_08e6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6CE);
    cpu.pc = 0x8D2F8;
    16
}
/// $08E6CE: jsr $2783a.l
pub fn i_08e6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6D4);
    cpu.pc = 0x2783A;
    16
}
/// $08E6D4: jsr $8efb0(pc)
pub fn i_08e6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6D8);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08e6d8  ($08E6D8, 8 B, thunks/jsr_abs_thunks.c)
/// $08E6D8: jsr $28d70.l
pub fn i_08e6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6DE);
    cpu.pc = 0x28D70;
    16
}
/// $08E6DE: rts
pub fn i_08e6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Runner_Tmpl184_08e6e0  ($08E6E0, 46 B, enemies/critters_rings_08e4e6.s)
/// $08E6E0: jsr $8f108(pc)
pub fn i_08e6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6E4);
    cpu.pc = 0x8F108;
    16
}
/// $08E6E4: jsr $8f002(pc)
pub fn i_08e6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6E8);
    cpu.pc = 0x8F002;
    16
}
/// $08E6E8: jsr $8f010(pc)
pub fn i_08e6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E6EC);
    cpu.pc = 0x8F010;
    16
}
/// $08E6EC: move.w #$128, d1
pub fn i_08e6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E6F0: jsr $236e.l
pub fn i_08e6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E6F6);
    cpu.pc = 0x236E;
    16
}
/// $08E6F6: lea.l $2f4096.l, a0
pub fn i_08e6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E6FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4096;
    20
}
/// $08E6FC: jsr $28cd4.l
pub fn i_08e6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E702);
    cpu.pc = 0x28CD4;
    16
}
/// $08E702: jsr $27cee.l
pub fn i_08e702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E708);
    cpu.pc = 0x27CEE;
    16
}
/// $08E708: jsr $28d70.l
pub fn i_08e708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E70E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E70E);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_08e70e  ($08E70E, 8 B, kernel/task_handlers.c)
/// $08E70E: lea.l $8e716(pc), a1
pub fn i_08e70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E712;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E716;
    16
}
/// $08E712: move.l a1, (a6)
pub fn i_08e712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E714;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E714: rts
pub fn i_08e714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E716;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Runner_Run_08e716  ($08E716, 24 B, enemies/critters_rings_08e4e6.s)
/// $08E716: jsr $8d2b0(pc)
pub fn i_08e716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E71A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E71A);
    cpu.pc = 0x8D2B0;
    16
}
/// $08E71A: jsr $2783a.l
pub fn i_08e71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E720;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E720);
    cpu.pc = 0x2783A;
    16
}
/// $08E720: jsr $8d184(pc)
pub fn i_08e720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E724;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E724);
    cpu.pc = 0x8D184;
    16
}
/// $08E724: bcc.w $8e730
pub fn i_08e724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E728;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E730; return 10; }
    8
}
/// $08E728: jmp $518.l
pub fn i_08e728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E72E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08e72e  ($08E72E, 2 B, enemies/critters_rings_08e4e6.s)
/// $08E72E: rts
pub fn i_08e72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E730;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08e730  ($08E730, 8 B, thunks/jsr_abs_thunks.c)
/// $08E730: jsr $28d70.l
pub fn i_08e730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E736);
    cpu.pc = 0x28D70;
    16
}
/// $08E736: rts
pub fn i_08e736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E738;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Spawn3_08e738  ($08E738, 14 B, enemies/critters_rings_08e4e6.s)
/// $08E738: move.b #$3, $9a(a6)
pub fn i_08e738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E73E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08E73E: clr.w $5c(a6)
pub fn i_08e73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E742;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E742: bra.w $8e750
pub fn i_08e742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E746;
    cpu.ir = 0x6000;
    cpu.pc = 0x8E750;
    10
}
// ===== Nest_Tmpl185_08e746  ($08E746, 132 B, enemies/critters_rings_08e4e6.s)
/// $08E746: jsr $8f108(pc)
pub fn i_08e746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E74A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E74A);
    cpu.pc = 0x8F108;
    16
}
/// $08E74A: move.w #$1, $5c(a6)
pub fn i_08e74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E750;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E750: jsr $8f002(pc)
pub fn i_08e750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E754;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E754);
    cpu.pc = 0x8F002;
    16
}
/// $08E754: move.w #$125, d1
pub fn i_08e754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E758;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E758: jsr $236e.l
pub fn i_08e758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E75E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E75E);
    cpu.pc = 0x236E;
    16
}
/// $08E75E: lea.l $2f41f2.l, a0
pub fn i_08e75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E764;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F41F2;
    20
}
/// $08E764: jsr $28cd4.l
pub fn i_08e764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E76A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E76A);
    cpu.pc = 0x28CD4;
    16
}
/// $08E76A: lea.l $2f414a.l, a0
pub fn i_08e76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E770;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F414A;
    20
}
/// $08E770: move.l a0, $48(a6)
pub fn i_08e770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E774;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08E774: bclr.b #$3, $13(a6)
pub fn i_08e774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E77A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E77A: move.w #$64, $66(a6)
pub fn i_08e77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E780;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E780: subq.b #$1, $9a(a6)
pub fn i_08e780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E784;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x9A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08E784: cmpi.b #$0, $9a(a6)
pub fn i_08e784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E78A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08E78A: blt.w $8e7a0
pub fn i_08e78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E78E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E7A0; return 10; }
    8
}
/// $08E78E: lea.l $8eb56(pc), a1
pub fn i_08e78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E792;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB56;
    16
}
/// $08E792: jsr $4ae.l
pub fn i_08e792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E798);
    cpu.pc = 0x4AE;
    16
}
/// $08E798: jsr $5dd02.l
pub fn i_08e798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E79E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E79E);
    cpu.pc = 0x5DD02;
    16
}
/// $08E79E: bra.b $8e780
pub fn i_08e79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7A0;
    cpu.ir = 0x60E0;
    cpu.pc = 0x8E780;
    10
}
/// $08E7A0: lea.l $8e7a6(pc), a1
pub fn i_08e7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E7A6;
    16
}
/// $08E7A4: move.l a1, (a6)
pub fn i_08e7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E7A6: jsr $2783a.l
pub fn i_08e7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7AC);
    cpu.pc = 0x2783A;
    16
}
/// $08E7AC: jsr $28d70.l
pub fn i_08e7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7B2);
    cpu.pc = 0x28D70;
    16
}
/// $08E7B2: jsr $2870a.l
pub fn i_08e7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7B8);
    cpu.pc = 0x2870A;
    16
}
/// $08E7B8: bcc.w $8e7c2
pub fn i_08e7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E7C2; return 10; }
    8
}
/// $08E7BC: lea.l $8e7de(pc), a1
pub fn i_08e7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E7DE;
    16
}
/// $08E7C0: move.l a1, (a6)
pub fn i_08e7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E7C2: jsr $8d184(pc)
pub fn i_08e7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E7C6);
    cpu.pc = 0x8D184;
    16
}
/// $08E7C6: bcc.w $8e7d8
pub fn i_08e7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E7D8; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e7ca  ($08E7CA, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E7CA: jsr $5b6.l
pub fn i_08e7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7D0);
    cpu.pc = 0x5B6;
    16
}
/// $08E7D0: jmp $518.l
pub fn i_08e7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7D6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e7d8  ($08E7D8, 6 B, thunks/jsr_pc_thunks.c)
/// $08E7D8: jsr $8ea50(pc)
pub fn i_08e7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E7DC);
    cpu.pc = 0x8EA50;
    16
}
/// $08E7DC: rts
pub fn i_08e7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage1_Hit_08e7de  ($08E7DE, 48 B, enemies/critters_rings_08e4e6.s)
/// $08E7DE: lea.l $2f4202.l, a0
pub fn i_08e7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4202;
    20
}
/// $08E7E4: jsr $28cd4.l
pub fn i_08e7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7EA);
    cpu.pc = 0x28CD4;
    16
}
/// $08E7EA: lea.l $8e7f0(pc), a1
pub fn i_08e7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E7F0;
    16
}
/// $08E7EE: move.l a1, (a6)
pub fn i_08e7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E7F0: jsr $2783a.l
pub fn i_08e7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7F6);
    cpu.pc = 0x2783A;
    16
}
/// $08E7F6: jsr $28d70.l
pub fn i_08e7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E7FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E7FC);
    cpu.pc = 0x28D70;
    16
}
/// $08E7FC: bcc.w $8e806
pub fn i_08e7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E800;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E806; return 10; }
    8
}
/// $08E800: lea.l $8e822(pc), a1
pub fn i_08e800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E804;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E822;
    16
}
/// $08E804: move.l a1, (a6)
pub fn i_08e804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E806;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E806: jsr $8d184(pc)
pub fn i_08e806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E80A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E80A);
    cpu.pc = 0x8D184;
    16
}
/// $08E80A: bcc.w $8e81c
pub fn i_08e80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E80E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E81C; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e80e  ($08E80E, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E80E: jsr $5b6.l
pub fn i_08e80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E814);
    cpu.pc = 0x5B6;
    16
}
/// $08E814: jmp $518.l
pub fn i_08e814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E81A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e81c  ($08E81C, 6 B, thunks/jsr_pc_thunks.c)
/// $08E81C: jsr $8ea50(pc)
pub fn i_08e81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E820;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E820);
    cpu.pc = 0x8EA50;
    16
}
/// $08E820: rts
pub fn i_08e820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E822;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage2_08e822  ($08E822, 54 B, enemies/critters_rings_08e4e6.s)
/// $08E822: bclr.b #$3, $13(a6)
pub fn i_08e822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E828;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E828: move.w #$64, $66(a6)
pub fn i_08e828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E82E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E82E: lea.l $8e834(pc), a1
pub fn i_08e82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E832;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E834;
    16
}
/// $08E832: move.l a1, (a6)
pub fn i_08e832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E834;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E834: jsr $2783a.l
pub fn i_08e834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E83A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E83A);
    cpu.pc = 0x2783A;
    16
}
/// $08E83A: jsr $28d70.l
pub fn i_08e83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E840);
    cpu.pc = 0x28D70;
    16
}
/// $08E840: jsr $2870a.l
pub fn i_08e840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E846);
    cpu.pc = 0x2870A;
    16
}
/// $08E846: bcc.w $8e850
pub fn i_08e846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E84A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E850; return 10; }
    8
}
/// $08E84A: lea.l $8e86c(pc), a1
pub fn i_08e84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E84E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E86C;
    16
}
/// $08E84E: move.l a1, (a6)
pub fn i_08e84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E850;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E850: jsr $8d184(pc)
pub fn i_08e850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E854;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E854);
    cpu.pc = 0x8D184;
    16
}
/// $08E854: bcc.w $8e866
pub fn i_08e854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E858;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E866; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e858  ($08E858, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E858: jsr $5b6.l
pub fn i_08e858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E85E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E85E);
    cpu.pc = 0x5B6;
    16
}
/// $08E85E: jmp $518.l
pub fn i_08e85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E864;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e866  ($08E866, 6 B, thunks/jsr_pc_thunks.c)
/// $08E866: jsr $8ea50(pc)
pub fn i_08e866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E86A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E86A);
    cpu.pc = 0x8EA50;
    16
}
/// $08E86A: rts
pub fn i_08e86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E86C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage2_Hit_08e86c  ($08E86C, 48 B, enemies/critters_rings_08e4e6.s)
/// $08E86C: lea.l $2f4284.l, a0
pub fn i_08e86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E872;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4284;
    20
}
/// $08E872: jsr $28cd4.l
pub fn i_08e872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E878);
    cpu.pc = 0x28CD4;
    16
}
/// $08E878: lea.l $8e87e(pc), a1
pub fn i_08e878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E87C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E87E;
    16
}
/// $08E87C: move.l a1, (a6)
pub fn i_08e87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E87E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E87E: jsr $2783a.l
pub fn i_08e87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E884);
    cpu.pc = 0x2783A;
    16
}
/// $08E884: jsr $28d70.l
pub fn i_08e884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E88A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E88A);
    cpu.pc = 0x28D70;
    16
}
/// $08E88A: bcc.w $8e894
pub fn i_08e88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E88E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E894; return 10; }
    8
}
/// $08E88E: lea.l $8e8b0(pc), a1
pub fn i_08e88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E892;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E8B0;
    16
}
/// $08E892: move.l a1, (a6)
pub fn i_08e892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E894;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E894: jsr $8d184(pc)
pub fn i_08e894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E898;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E898);
    cpu.pc = 0x8D184;
    16
}
/// $08E898: bcc.w $8e8aa
pub fn i_08e898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E89C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E8AA; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e89c  ($08E89C, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E89C: jsr $5b6.l
pub fn i_08e89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8A2);
    cpu.pc = 0x5B6;
    16
}
/// $08E8A2: jmp $518.l
pub fn i_08e8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e8aa  ($08E8AA, 6 B, thunks/jsr_pc_thunks.c)
/// $08E8AA: jsr $8ea50(pc)
pub fn i_08e8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E8AE);
    cpu.pc = 0x8EA50;
    16
}
/// $08E8AE: rts
pub fn i_08e8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage3_08e8b0  ($08E8B0, 54 B, enemies/critters_rings_08e4e6.s)
/// $08E8B0: bclr.b #$3, $13(a6)
pub fn i_08e8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8B6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E8B6: move.w #$64, $66(a6)
pub fn i_08e8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E8BC: lea.l $8e8c2(pc), a1
pub fn i_08e8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E8C2;
    16
}
/// $08E8C0: move.l a1, (a6)
pub fn i_08e8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E8C2: jsr $2783a.l
pub fn i_08e8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8C8);
    cpu.pc = 0x2783A;
    16
}
/// $08E8C8: jsr $28d70.l
pub fn i_08e8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8CE);
    cpu.pc = 0x28D70;
    16
}
/// $08E8CE: jsr $2870a.l
pub fn i_08e8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8D4);
    cpu.pc = 0x2870A;
    16
}
/// $08E8D4: bcc.w $8e8de
pub fn i_08e8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E8DE; return 10; }
    8
}
/// $08E8D8: lea.l $8e8fa(pc), a1
pub fn i_08e8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E8FA;
    16
}
/// $08E8DC: move.l a1, (a6)
pub fn i_08e8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E8DE: jsr $8d184(pc)
pub fn i_08e8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E8E2);
    cpu.pc = 0x8D184;
    16
}
/// $08E8E2: bcc.w $8e8f4
pub fn i_08e8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E8F4; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e8e6  ($08E8E6, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E8E6: jsr $5b6.l
pub fn i_08e8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E8EC);
    cpu.pc = 0x5B6;
    16
}
/// $08E8EC: jmp $518.l
pub fn i_08e8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e8f4  ($08E8F4, 6 B, thunks/jsr_pc_thunks.c)
/// $08E8F4: jsr $8ea50(pc)
pub fn i_08e8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E8F8);
    cpu.pc = 0x8EA50;
    16
}
/// $08E8F8: rts
pub fn i_08e8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage3_Hit_08e8fa  ($08E8FA, 48 B, enemies/critters_rings_08e4e6.s)
/// $08E8FA: lea.l $2f4306.l, a0
pub fn i_08e8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E900;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4306;
    20
}
/// $08E900: jsr $28cd4.l
pub fn i_08e900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E906);
    cpu.pc = 0x28CD4;
    16
}
/// $08E906: lea.l $8e90c(pc), a1
pub fn i_08e906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E90A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E90C;
    16
}
/// $08E90A: move.l a1, (a6)
pub fn i_08e90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E90C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E90C: jsr $2783a.l
pub fn i_08e90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E912);
    cpu.pc = 0x2783A;
    16
}
/// $08E912: jsr $28d70.l
pub fn i_08e912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E918);
    cpu.pc = 0x28D70;
    16
}
/// $08E918: bcc.w $8e922
pub fn i_08e918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E91C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E922; return 10; }
    8
}
/// $08E91C: lea.l $8e93e(pc), a1
pub fn i_08e91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E920;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E93E;
    16
}
/// $08E920: move.l a1, (a6)
pub fn i_08e920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E922;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E922: jsr $8d184(pc)
pub fn i_08e922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E926;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E926);
    cpu.pc = 0x8D184;
    16
}
/// $08E926: bcc.w $8e938
pub fn i_08e926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E92A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E938; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e92a  ($08E92A, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E92A: jsr $5b6.l
pub fn i_08e92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E930);
    cpu.pc = 0x5B6;
    16
}
/// $08E930: jmp $518.l
pub fn i_08e930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E936;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e938  ($08E938, 6 B, thunks/jsr_pc_thunks.c)
/// $08E938: jsr $8ea50(pc)
pub fn i_08e938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E93C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E93C);
    cpu.pc = 0x8EA50;
    16
}
/// $08E93C: rts
pub fn i_08e93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E93E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Stage4_08e93e  ($08E93E, 54 B, enemies/critters_rings_08e4e6.s)
/// $08E93E: bclr.b #$3, $13(a6)
pub fn i_08e93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E944;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08E944: move.w #$64, $66(a6)
pub fn i_08e944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E94A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E94A: lea.l $8e950(pc), a1
pub fn i_08e94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E94E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E950;
    16
}
/// $08E94E: move.l a1, (a6)
pub fn i_08e94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E950: jsr $2783a.l
pub fn i_08e950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E956);
    cpu.pc = 0x2783A;
    16
}
/// $08E956: jsr $28d70.l
pub fn i_08e956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E95C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E95C);
    cpu.pc = 0x28D70;
    16
}
/// $08E95C: jsr $2870a.l
pub fn i_08e95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E962);
    cpu.pc = 0x2870A;
    16
}
/// $08E962: bcc.w $8e96c
pub fn i_08e962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E966;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E96C; return 10; }
    8
}
/// $08E966: lea.l $8e988(pc), a1
pub fn i_08e966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E96A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E988;
    16
}
/// $08E96A: move.l a1, (a6)
pub fn i_08e96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E96C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E96C: jsr $8d184(pc)
pub fn i_08e96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E970;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E970);
    cpu.pc = 0x8D184;
    16
}
/// $08E970: bcc.w $8e982
pub fn i_08e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E974;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E982; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e974  ($08E974, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E974: jsr $5b6.l
pub fn i_08e974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E97A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E97A);
    cpu.pc = 0x5B6;
    16
}
/// $08E97A: jmp $518.l
pub fn i_08e97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E980;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e982  ($08E982, 6 B, thunks/jsr_pc_thunks.c)
/// $08E982: jsr $8ea50(pc)
pub fn i_08e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E986;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E986);
    cpu.pc = 0x8EA50;
    16
}
/// $08E986: rts
pub fn i_08e986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E988;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_Destroyed_08e988  ($08E988, 56 B, enemies/critters_rings_08e4e6.s)
/// $08E988: lea.l $ffff.w, a0
pub fn i_08e988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E98C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08E98C: move.l a0, $48(a6)
pub fn i_08e98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E990;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08E990: lea.l $2f4388.l, a0
pub fn i_08e990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E996;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4388;
    20
}
/// $08E996: jsr $28cd4.l
pub fn i_08e996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E99C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E99C: lea.l $8e9a2(pc), a1
pub fn i_08e99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E9A2;
    16
}
/// $08E9A0: move.l a1, (a6)
pub fn i_08e9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E9A2: jsr $2783a.l
pub fn i_08e9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9A8);
    cpu.pc = 0x2783A;
    16
}
/// $08E9A8: jsr $28d70.l
pub fn i_08e9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9AE);
    cpu.pc = 0x28D70;
    16
}
/// $08E9AE: bcc.w $8e9b8
pub fn i_08e9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E9B8; return 10; }
    8
}
/// $08E9B2: lea.l $8e9d4(pc), a1
pub fn i_08e9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E9D4;
    16
}
/// $08E9B6: move.l a1, (a6)
pub fn i_08e9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E9B8: jsr $8d184(pc)
pub fn i_08e9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E9BC);
    cpu.pc = 0x8D184;
    16
}
/// $08E9BC: bcc.w $8e9ce
pub fn i_08e9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E9CE; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e9c0  ($08E9C0, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E9C0: jsr $5b6.l
pub fn i_08e9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9C6);
    cpu.pc = 0x5B6;
    16
}
/// $08E9C6: jmp $518.l
pub fn i_08e9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9CC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e9ce  ($08E9CE, 6 B, thunks/jsr_pc_thunks.c)
/// $08E9CE: jsr $8ea50(pc)
pub fn i_08e9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E9D2);
    cpu.pc = 0x8EA50;
    16
}
/// $08E9D2: rts
pub fn i_08e9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_DestroyedWait_08e9d4  ($08E9D4, 20 B, enemies/critters_rings_08e4e6.s)
/// $08E9D4: jsr $2783a.l
pub fn i_08e9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9DA);
    cpu.pc = 0x2783A;
    16
}
/// $08E9DA: jsr $28d70.l
pub fn i_08e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9E0);
    cpu.pc = 0x28D70;
    16
}
/// $08E9E0: jsr $8d184(pc)
pub fn i_08e9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E9E4);
    cpu.pc = 0x8D184;
    16
}
/// $08E9E4: bcc.w $8e9f6
pub fn i_08e9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8E9F6; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08e9e8  ($08E9E8, 14 B, kernel/jsr5b6_jmp518.c)
/// $08E9E8: jsr $5b6.l
pub fn i_08e9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E9EE);
    cpu.pc = 0x5B6;
    16
}
/// $08E9EE: jmp $518.l
pub fn i_08e9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9F4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_08e9f6  ($08E9F6, 6 B, thunks/jsr_pc_thunks.c)
/// $08E9F6: jsr $8ea50(pc)
pub fn i_08e9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E9FA);
    cpu.pc = 0x8EA50;
    16
}
/// $08E9FA: rts
pub fn i_08e9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E9FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_FlyOff_08e9fc  ($08E9FC, 26 B, enemies/critters_rings_08e4e6.s)
/// $08E9FC: lea.l $ffff.w, a0
pub fn i_08e9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA00;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08EA00: move.l a0, $48(a6)
pub fn i_08ea00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA04;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08EA04: move.w #$ff80, $2a(a6)
pub fn i_08ea04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EA0A: move.w #$ffc0, $2e(a6)
pub fn i_08ea0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EA10: lea.l $8ea16(pc), a1
pub fn i_08ea10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EA16;
    16
}
/// $08EA14: move.l a1, (a6)
pub fn i_08ea14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Nest_FlyOff_Run_08ea16  ($08EA16, 42 B, enemies/critters_rings_08e4e6.s)
/// $08EA16: jsr $27c8c.l
pub fn i_08ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA1C);
    cpu.pc = 0x27C8C;
    16
}
/// $08EA1C: bcc.w $8ea2c
pub fn i_08ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EA2C; return 10; }
    8
}
/// $08EA20: jsr $5b6.l
pub fn i_08ea20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA26);
    cpu.pc = 0x5B6;
    16
}
/// $08EA26: jmp $518.l
pub fn i_08ea26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA2C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EA2C: jsr $2783a.l
pub fn i_08ea2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA32);
    cpu.pc = 0x2783A;
    16
}
/// $08EA32: jsr $28d70.l
pub fn i_08ea32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA38);
    cpu.pc = 0x28D70;
    16
}
/// $08EA38: jsr $8d184(pc)
pub fn i_08ea38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EA3C);
    cpu.pc = 0x8D184;
    16
}
/// $08EA3C: bcc.w $8ea4e
pub fn i_08ea3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EA4E; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08ea40  ($08EA40, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EA40: jsr $5b6.l
pub fn i_08ea40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EA46);
    cpu.pc = 0x5B6;
    16
}
/// $08EA46: jmp $518.l
pub fn i_08ea46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA4C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Stub_0008EA4E  ($08EA4E, 2 B, thunks/trivial_rts.c)
/// $08EA4E: rts
pub fn i_08ea4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest_DieIfParentGone_08ea50  ($08EA50, 28 B, enemies/critters_rings_08e4e6.s)
/// $08EA50: cmpi.w #$0, $5c(a6)
pub fn i_08ea50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA56;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EA56: beq.w $8ea5c
pub fn i_08ea56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8EA5C; return 10; }
    8
}
/// $08EA5A: rts
pub fn i_08ea5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08EA5C: movea.l $c(a6), a1
pub fn i_08ea5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA60;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EA60: cmpi.l #$ffffffff, $48(a1)
pub fn i_08ea60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA68;
    cpu.ir = 0xCA9;
    let ad2 = cpu.a[1].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $08EA68: bne.w $8ea72
pub fn i_08ea68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EA72; return 10; }
    8
}
// ===== SetTaskHandler_08ea6c  ($08EA6C, 8 B, kernel/task_handlers.c)
/// $08EA6C: lea.l $8ea16(pc), a1
pub fn i_08ea6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EA16;
    16
}
/// $08EA70: move.l a1, (a6)
pub fn i_08ea70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EA72: rts
pub fn i_08ea72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest2_Tmpl187_08ea74  ($08EA74, 106 B, enemies/critters_rings_08e4e6.s)
/// $08EA74: jsr $8f108(pc)
pub fn i_08ea74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EA78);
    cpu.pc = 0x8F108;
    16
}
/// $08EA78: lea.l $2f419e.l, a0
pub fn i_08ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F419E;
    20
}
/// $08EA7E: move.l a0, $48(a6)
pub fn i_08ea7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA82;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08EA82: bclr.b #$3, $13(a6)
pub fn i_08ea82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08EA88: move.w #$64, $66(a6)
pub fn i_08ea88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EA8E: clr.b $20(a6)
pub fn i_08ea8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA92;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EA92: subq.b #$1, $98(a6)
pub fn i_08ea92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA96;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EA96: cmpi.b #$0, $98(a6)
pub fn i_08ea96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA9C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EA9C: blt.w $8eab2
pub fn i_08ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAA0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8EAB2; return 10; }
    8
}
/// $08EAA0: lea.l $8eb56(pc), a1
pub fn i_08eaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB56;
    16
}
/// $08EAA4: jsr $4ae.l
pub fn i_08eaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EAAA);
    cpu.pc = 0x4AE;
    16
}
/// $08EAAA: jsr $5dd02.l
pub fn i_08eaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EAB0);
    cpu.pc = 0x5DD02;
    16
}
/// $08EAB0: bra.b $8ea92
pub fn i_08eab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAB2;
    cpu.ir = 0x60E0;
    cpu.pc = 0x8EA92;
    10
}
/// $08EAB2: lea.l $8eab8(pc), a1
pub fn i_08eab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EAB8;
    16
}
/// $08EAB6: move.l a1, (a6)
pub fn i_08eab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EAB8: jsr $2783a.l
pub fn i_08eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EABE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EABE);
    cpu.pc = 0x2783A;
    16
}
/// $08EABE: jsr $2870a.l
pub fn i_08eabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EAC4);
    cpu.pc = 0x2870A;
    16
}
/// $08EAC4: bcc.w $8eace
pub fn i_08eac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EACE; return 10; }
    8
}
/// $08EAC8: lea.l $8eaee(pc), a1
pub fn i_08eac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EACC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EAEE;
    16
}
/// $08EACC: move.l a1, (a6)
pub fn i_08eacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EACE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EACE: jsr $8d184(pc)
pub fn i_08eace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EAD2);
    cpu.pc = 0x8D184;
    16
}
/// $08EAD2: bcc.w $8eaec
pub fn i_08ead2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EAEC; return 10; }
    8
}
/// $08EAD6: lea.l $ffff.w, a0
pub fn i_08ead6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EADA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08EADA: move.l a0, $48(a6)
pub fn i_08eada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EADE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Jsr5B6ThenJmpScheduler_08eade  ($08EADE, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EADE: jsr $5b6.l
pub fn i_08eade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EAE4);
    cpu.pc = 0x5B6;
    16
}
/// $08EAE4: jmp $518.l
pub fn i_08eae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAEA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Stub_0008EAEC  ($08EAEC, 2 B, thunks/trivial_rts.c)
/// $08EAEC: rts
pub fn i_08eaec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest2_Hit_08eaee  ($08EAEE, 12 B, enemies/critters_rings_08e4e6.s)
/// $08EAEE: move.b #$1, $20(a6)
pub fn i_08eaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EAF4: move.w #$14, $70(a6)
pub fn i_08eaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08eafa  ($08EAFA, 8 B, kernel/task_handlers.c)
/// $08EAFA: lea.l $8eb02(pc), a1
pub fn i_08eafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EAFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB02;
    16
}
/// $08EAFE: move.l a1, (a6)
pub fn i_08eafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EB00: rts
pub fn i_08eb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nest2_HitWait_08eb02  ($08EB02, 68 B, enemies/critters_rings_08e4e6.s)
/// $08EB02: clr.b $20(a6)
pub fn i_08eb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB06;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EB06: lea.l $8eb0c(pc), a1
pub fn i_08eb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB0C;
    16
}
/// $08EB0A: move.l a1, (a6)
pub fn i_08eb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EB0C: jsr $2783a.l
pub fn i_08eb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB12);
    cpu.pc = 0x2783A;
    16
}
/// $08EB12: subq.w #$1, $70(a6)
pub fn i_08eb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB16;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08EB16: cmpi.w #$0, $70(a6)
pub fn i_08eb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB1C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EB1C: bgt.w $8eb36
pub fn i_08eb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB20;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8EB36; return 10; }
    8
}
/// $08EB20: bclr.b #$3, $13(a6)
pub fn i_08eb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB26;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08EB26: move.w #$64, $66(a6)
pub fn i_08eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EB2C: clr.b $20(a6)
pub fn i_08eb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB30;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EB30: lea.l $8eab8(pc), a1
pub fn i_08eb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EAB8;
    16
}
/// $08EB34: move.l a1, (a6)
pub fn i_08eb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EB36: jsr $8d184(pc)
pub fn i_08eb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EB3A);
    cpu.pc = 0x8D184;
    16
}
/// $08EB3A: bcc.w $8eb54
pub fn i_08eb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EB54; return 10; }
    8
}
/// $08EB3E: lea.l $ffff.w, a0
pub fn i_08eb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB42;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08EB42: move.l a0, $48(a6)
pub fn i_08eb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB46;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Jsr5B6ThenJmpScheduler_08eb46  ($08EB46, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EB46: jsr $5b6.l
pub fn i_08eb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB4C);
    cpu.pc = 0x5B6;
    16
}
/// $08EB4C: jmp $518.l
pub fn i_08eb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB52;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Stub_0008EB54  ($08EB54, 2 B, thunks/trivial_rts.c)
/// $08EB54: rts
pub fn i_08eb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swarmer_Init_08eb56  ($08EB56, 22 B, enemies/critters_rings_08e4e6.s)
/// $08EB56: move.w #$125, d1
pub fn i_08eb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB5A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EB5A: jsr $236e.l
pub fn i_08eb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB60);
    cpu.pc = 0x236E;
    16
}
/// $08EB60: lea.l $2f440a.l, a0
pub fn i_08eb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F440A;
    20
}
/// $08EB66: jsr $28cd4.l
pub fn i_08eb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB6C);
    cpu.pc = 0x28CD4;
    16
}
// ===== Swarmer_AimParent_08eb6c  ($08EB6C, 42 B, enemies/critters_rings_08e4e6.s)
/// $08EB6C: movea.l $c(a6), a1
pub fn i_08eb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB70;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EB70: move.w $22(a1), d0
pub fn i_08eb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB74;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EB74: subi.w #$c, d0
pub fn i_08eb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB78;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EB78: move.w $24(a1), d1
pub fn i_08eb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB7C;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EB7C: subi.w #$18, d1
pub fn i_08eb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB80;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EB80: sub.w $22(a6), d0
pub fn i_08eb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB84;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EB84: sub.w $24(a6), d1
pub fn i_08eb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB88;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EB88: jsr $5e018.l
pub fn i_08eb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EB8E);
    cpu.pc = 0x5E018;
    16
}
/// $08EB8E: move.w d0, $34(a6)
pub fn i_08eb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB92;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EB92: bra.w $8eba4
pub fn i_08eb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB96;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EBA4;
    10
}
// ===== Swarmer_AimRandom_08eb96  ($08EB96, 180 B, enemies/critters_rings_08e4e6.s)
/// $08EB96: move.w #$ff, d0
pub fn i_08eb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EB9A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EB9A: jsr $5ea1c.l
pub fn i_08eb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EBA0);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EBA0: move.w d0, $34(a6)
pub fn i_08eba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBA4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EBA4: move.w #$ff, d0
pub fn i_08eba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBA8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EBA8: jsr $5ea1c.l
pub fn i_08eba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EBAE);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EBAE: move.w d0, d1
pub fn i_08ebae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBB0;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EBB0: addi.w #$180, d1
pub fn i_08ebb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBB4;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x180, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EBB4: move.w $34(a6), d0
pub fn i_08ebb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EBB8: jsr $13c0e.l
pub fn i_08ebb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EBBE);
    cpu.pc = 0x13C0E;
    16
}
/// $08EBBE: move.w d1, $28(a6)
pub fn i_08ebbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBC2;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EBC2: move.w d2, $2a(a6)
pub fn i_08ebc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBC6;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EBC6: cmpi.w #$0, $28(a6)
pub fn i_08ebc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBCC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EBCC: bge.w $8ebd8
pub fn i_08ebcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBD0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8EBD8; return 10; }
    8
}
/// $08EBD0: clr.b $3a(a6)
pub fn i_08ebd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBD4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EBD4: bra.w $8ebde
pub fn i_08ebd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EBDE;
    10
}
/// $08EBD8: move.b #$1, $3a(a6)
pub fn i_08ebd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBDE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EBDE: move.w #$3, d0
pub fn i_08ebde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EBE2: jsr $5ea1c.l
pub fn i_08ebe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EBE8);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EBE8: addi.w #$1, d0
pub fn i_08ebe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBEC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EBEC: move.w d0, $70(a6)
pub fn i_08ebec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBF0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EBF0: lea.l $8ebf6(pc), a1
pub fn i_08ebf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EBF6;
    16
}
/// $08EBF4: move.l a1, (a6)
pub fn i_08ebf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EBF6: jsr $8d2f8(pc)
pub fn i_08ebf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EBFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EBFA);
    cpu.pc = 0x8D2F8;
    16
}
/// $08EBFA: jsr $2783a.l
pub fn i_08ebfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC00);
    cpu.pc = 0x2783A;
    16
}
/// $08EC00: subq.w #$1, $70(a6)
pub fn i_08ec00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC04;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08EC04: cmpi.w #$0, $70(a6)
pub fn i_08ec04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC0A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EC0A: bgt.w $8ec30
pub fn i_08ec0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC0E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8EC30; return 10; }
    8
}
/// $08EC0E: move.w #$3, d0
pub fn i_08ec0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC12;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EC12: jsr $5ea1c.l
pub fn i_08ec12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC18);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EC18: cmpi.w #$0, d0
pub fn i_08ec18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC1C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $08EC1C: bne.w $8ec2a
pub fn i_08ec1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EC2A; return 10; }
    8
}
/// $08EC20: lea.l $8eb6c(pc), a1
pub fn i_08ec20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB6C;
    16
}
/// $08EC24: move.l a1, (a6)
pub fn i_08ec24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EC26: bra.w $8ec30
pub fn i_08ec26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EC30;
    10
}
/// $08EC2A: lea.l $8eb96(pc), a1
pub fn i_08ec2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB96;
    16
}
/// $08EC2E: move.l a1, (a6)
pub fn i_08ec2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EC30: movea.l $c(a6), a1
pub fn i_08ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC34;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EC34: cmpi.b #$0, $20(a1)
pub fn i_08ec34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC3A;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EC3A: beq.w $8ec44
pub fn i_08ec3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8EC44; return 10; }
    8
}
/// $08EC3E: lea.l $8ec50(pc), a1
pub fn i_08ec3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EC50;
    16
}
/// $08EC42: move.l a1, (a6)
pub fn i_08ec42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EC44: jsr $28d70.l
pub fn i_08ec44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC4A);
    cpu.pc = 0x28D70;
    16
}
// ===== JsrPcThunk_08ec4a  ($08EC4A, 6 B, thunks/jsr_pc_thunks.c)
/// $08EC4A: jsr $8efb0(pc)
pub fn i_08ec4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EC4E);
    cpu.pc = 0x8EFB0;
    16
}
/// $08EC4E: rts
pub fn i_08ec4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swarmer_Flee_08ec50  ($08EC50, 132 B, enemies/critters_rings_08e4e6.s)
/// $08EC50: movea.l $c(a6), a1
pub fn i_08ec50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC54;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EC54: move.w $22(a6), d0
pub fn i_08ec54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC58;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EC58: addi.w #$c, d0
pub fn i_08ec58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC5C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EC5C: move.w $24(a6), d1
pub fn i_08ec5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC60;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EC60: addi.w #$18, d1
pub fn i_08ec60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC64;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EC64: sub.w $22(a1), d0
pub fn i_08ec64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC68;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EC68: sub.w $24(a1), d1
pub fn i_08ec68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC6C;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EC6C: jsr $5e018.l
pub fn i_08ec6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC72);
    cpu.pc = 0x5E018;
    16
}
/// $08EC72: move.w d0, $34(a6)
pub fn i_08ec72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC76;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EC76: move.w #$300, d1
pub fn i_08ec76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC7A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EC7A: move.w $34(a6), d0
pub fn i_08ec7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC7E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EC7E: jsr $13c0e.l
pub fn i_08ec7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EC84);
    cpu.pc = 0x13C0E;
    16
}
/// $08EC84: move.w d1, $28(a6)
pub fn i_08ec84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC88;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EC88: move.w d2, $2a(a6)
pub fn i_08ec88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC8C;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EC8C: cmpi.w #$0, $28(a6)
pub fn i_08ec8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC92;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08EC92: bge.w $8ec9e
pub fn i_08ec92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC96;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8EC9E; return 10; }
    8
}
/// $08EC96: clr.b $3a(a6)
pub fn i_08ec96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC9A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08EC9A: bra.w $8eca4
pub fn i_08ec9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EC9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8ECA4;
    10
}
/// $08EC9E: move.b #$1, $3a(a6)
pub fn i_08ec9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECA4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ECA4: move.w #$4, $70(a6)
pub fn i_08eca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ECAA: lea.l $8ecb0(pc), a1
pub fn i_08ecaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8ECB0;
    16
}
/// $08ECAE: move.l a1, (a6)
pub fn i_08ecae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ECB0: jsr $8d2f8(pc)
pub fn i_08ecb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ECB4);
    cpu.pc = 0x8D2F8;
    16
}
/// $08ECB4: jsr $2783a.l
pub fn i_08ecb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ECBA);
    cpu.pc = 0x2783A;
    16
}
/// $08ECBA: subq.w #$1, $70(a6)
pub fn i_08ecba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECBE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08ECBE: cmpi.w #$0, $70(a6)
pub fn i_08ecbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08ECC4: bgt.w $8ecce
pub fn i_08ecc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECC8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8ECCE; return 10; }
    8
}
/// $08ECC8: lea.l $8eb6c(pc), a1
pub fn i_08ecc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EB6C;
    16
}
/// $08ECCC: move.l a1, (a6)
pub fn i_08eccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ECCE: jsr $28d70.l
pub fn i_08ecce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ECD4);
    cpu.pc = 0x28D70;
    16
}
// ===== JsrPcThunk_08ecd4  ($08ECD4, 6 B, thunks/jsr_pc_thunks.c)
/// $08ECD4: jsr $8efb0(pc)
pub fn i_08ecd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ECD8);
    cpu.pc = 0x8EFB0;
    16
}
/// $08ECD8: rts
pub fn i_08ecd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Swarmer_Tmpl186_08ecda  ($08ECDA, 30 B, enemies/critters_rings_08e4e6.s)
/// $08ECDA: jsr $8f108(pc)
pub fn i_08ecda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ECDE);
    cpu.pc = 0x8F108;
    16
}
/// $08ECDE: jsr $8f002(pc)
pub fn i_08ecde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ECE2);
    cpu.pc = 0x8F002;
    16
}
/// $08ECE2: move.w #$125, d1
pub fn i_08ece2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ECE6: jsr $236e.l
pub fn i_08ece6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ECEC);
    cpu.pc = 0x236E;
    16
}
/// $08ECEC: lea.l $2f440a.l, a0
pub fn i_08ecec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F440A;
    20
}
/// $08ECF2: jsr $28cd4.l
pub fn i_08ecf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ECF8);
    cpu.pc = 0x28CD4;
    16
}
// ===== Swarmer_Wander_08ecf8  ($08ECF8, 132 B, enemies/critters_rings_08e4e6.s)
/// $08ECF8: move.w #$ff, d0
pub fn i_08ecf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ECFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ECFC: jsr $5ea1c.l
pub fn i_08ecfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED02);
    cpu.pc = 0x5EA1C;
    16
}
/// $08ED02: move.w d0, $34(a6)
pub fn i_08ed02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED06;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED06: move.w #$ff, d0
pub fn i_08ed06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED0A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ED0A: jsr $5ea1c.l
pub fn i_08ed0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED10);
    cpu.pc = 0x5EA1C;
    16
}
/// $08ED10: move.w d0, d1
pub fn i_08ed10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED12;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ED12: addi.w #$100, d1
pub fn i_08ed12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED16;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08ED16: move.w $34(a6), d0
pub fn i_08ed16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED1A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08ED1A: jsr $13c0e.l
pub fn i_08ed1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED20);
    cpu.pc = 0x13C0E;
    16
}
/// $08ED20: move.w d1, $28(a6)
pub fn i_08ed20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED24;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED24: move.w d2, $2a(a6)
pub fn i_08ed24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED28;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED28: move.w #$3, d0
pub fn i_08ed28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ED2C: jsr $5ea1c.l
pub fn i_08ed2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED32);
    cpu.pc = 0x5EA1C;
    16
}
/// $08ED32: addi.w #$2, d0
pub fn i_08ed32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED36;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08ED36: move.w d0, $70(a6)
pub fn i_08ed36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED3A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED3A: cmpi.w #$0, $28(a6)
pub fn i_08ed3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED40;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08ED40: bge.w $8ed4c
pub fn i_08ed40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED44;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8ED4C; return 10; }
    8
}
/// $08ED44: clr.b $3a(a6)
pub fn i_08ed44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED48;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08ED48: bra.w $8ed52
pub fn i_08ed48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8ED52;
    10
}
/// $08ED4C: move.b #$1, $3a(a6)
pub fn i_08ed4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ED52: lea.l $8ed58(pc), a1
pub fn i_08ed52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8ED58;
    16
}
/// $08ED56: move.l a1, (a6)
pub fn i_08ed56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ED58: jsr $8d2f8(pc)
pub fn i_08ed58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ED5C);
    cpu.pc = 0x8D2F8;
    16
}
/// $08ED5C: jsr $2783a.l
pub fn i_08ed5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED62);
    cpu.pc = 0x2783A;
    16
}
/// $08ED62: subq.w #$1, $70(a6)
pub fn i_08ed62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED66;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08ED66: cmpi.w #$0, $70(a6)
pub fn i_08ed66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED6C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08ED6C: bgt.w $8ed76
pub fn i_08ed6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED70;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8ED76; return 10; }
    8
}
/// $08ED70: lea.l $8ecf8(pc), a1
pub fn i_08ed70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8ECF8;
    16
}
/// $08ED74: move.l a1, (a6)
pub fn i_08ed74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ED76: jsr $28d70.l
pub fn i_08ed76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED7C);
    cpu.pc = 0x28D70;
    16
}
// ===== JsrPcThunk_08ed7c  ($08ED7C, 6 B, thunks/jsr_pc_thunks.c)
/// $08ED7C: jsr $8efb0(pc)
pub fn i_08ed7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ED80);
    cpu.pc = 0x8EFB0;
    16
}
/// $08ED80: rts
pub fn i_08ed80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Static_Tmpl188_08ed82  ($08ED82, 58 B, enemies/critters_rings_08e4e6.s)
/// $08ED82: move.w #$0, $38(a6)
pub fn i_08ed82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED88;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08ED88: jsr $8f002(pc)
pub fn i_08ed88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8ED8C);
    cpu.pc = 0x8F002;
    16
}
/// $08ED8C: move.w #$12a, d1
pub fn i_08ed8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED90;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ED90: jsr $236e.l
pub fn i_08ed90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ED96);
    cpu.pc = 0x236E;
    16
}
/// $08ED96: lea.l $2f4426.l, a0
pub fn i_08ed96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ED9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4426;
    20
}
/// $08ED9C: jsr $28cd4.l
pub fn i_08ed9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDA2);
    cpu.pc = 0x28CD4;
    16
}
/// $08EDA2: lea.l $8eda8(pc), a1
pub fn i_08eda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EDA8;
    16
}
/// $08EDA6: move.l a1, (a6)
pub fn i_08eda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EDA8: jsr $2783a.l
pub fn i_08eda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDAE);
    cpu.pc = 0x2783A;
    16
}
/// $08EDAE: jsr $8d184(pc)
pub fn i_08edae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EDB2);
    cpu.pc = 0x8D184;
    16
}
/// $08EDB2: bcc.w $8edbe
pub fn i_08edb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EDBE; return 10; }
    8
}
/// $08EDB6: jmp $518.l
pub fn i_08edb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDBC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08edbc  ($08EDBC, 2 B, enemies/critters_rings_08e4e6.s)
/// $08EDBC: rts
pub fn i_08edbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08edbe  ($08EDBE, 8 B, thunks/jsr_abs_thunks.c)
/// $08EDBE: jsr $28d70.l
pub fn i_08edbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDC4);
    cpu.pc = 0x28D70;
    16
}
/// $08EDC4: rts
pub fn i_08edc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CamProp_Tmpl189_08edc6  ($08EDC6, 16 B, enemies/critters_rings_08e4e6.s)
/// $08EDC6: lea.l $2f4442.l, a0
pub fn i_08edc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4442;
    20
}
/// $08EDCC: jsr $28cd4.l
pub fn i_08edcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDD2);
    cpu.pc = 0x28CD4;
    16
}
/// $08EDD2: bra.w $8ede2
pub fn i_08edd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EDE2;
    10
}
// ===== CamProp_Tmpl190_08edd6  ($08EDD6, 54 B, enemies/critters_rings_08e4e6.s)
/// $08EDD6: lea.l $2f44ae.l, a0
pub fn i_08edd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F44AE;
    20
}
/// $08EDDC: jsr $28cd4.l
pub fn i_08eddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDE2);
    cpu.pc = 0x28CD4;
    16
}
/// $08EDE2: move.w #$0, $38(a6)
pub fn i_08ede2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EDE8: move.w #$f4, d1
pub fn i_08ede8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDEC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EDEC: jsr $236e.l
pub fn i_08edec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDF2);
    cpu.pc = 0x236E;
    16
}
/// $08EDF2: lea.l $8edf8(pc), a1
pub fn i_08edf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EDF8;
    16
}
/// $08EDF6: move.l a1, (a6)
pub fn i_08edf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EDF8: jsr $4407a.l
pub fn i_08edf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EDFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EDFE);
    cpu.pc = 0x4407A;
    16
}
/// $08EDFE: jsr $8d184(pc)
pub fn i_08edfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE02);
    cpu.pc = 0x8D184;
    16
}
/// $08EE02: bcc.w $8ee0e
pub fn i_08ee02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EE0E; return 10; }
    8
}
/// $08EE06: jmp $518.l
pub fn i_08ee06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE0C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08ee0c  ($08EE0C, 2 B, enemies/critters_rings_08e4e6.s)
/// $08EE0C: rts
pub fn i_08ee0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08ee0e  ($08EE0E, 8 B, thunks/jsr_abs_thunks.c)
/// $08EE0E: jsr $28d70.l
pub fn i_08ee0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE14);
    cpu.pc = 0x28D70;
    16
}
/// $08EE14: rts
pub fn i_08ee14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Lob_Tmpl191_08ee16  ($08EE16, 92 B, enemies/critters_rings_08e4e6.s)
/// $08EE16: jsr $8f002(pc)
pub fn i_08ee16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE1A);
    cpu.pc = 0x8F002;
    16
}
/// $08EE1A: jsr $8f010(pc)
pub fn i_08ee1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE1E);
    cpu.pc = 0x8F010;
    16
}
/// $08EE1E: clr.w d0
pub fn i_08ee1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE20;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08EE20: move.b $99(a6), d0
pub fn i_08ee20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE24;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08EE24: lsl.w #$5, d0
pub fn i_08ee24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE26;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08EE26: neg.w d0
pub fn i_08ee26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE28;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08EE28: move.w d0, $2a(a6)
pub fn i_08ee28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE2C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EE2C: move.w #$0, $38(a6)
pub fn i_08ee2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE32;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EE32: move.w #$f4, d1
pub fn i_08ee32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE36;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EE36: jsr $236e.l
pub fn i_08ee36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE3C);
    cpu.pc = 0x236E;
    16
}
/// $08EE3C: lea.l $2f44ca.l, a0
pub fn i_08ee3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F44CA;
    20
}
/// $08EE42: jsr $28cd4.l
pub fn i_08ee42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE48);
    cpu.pc = 0x28CD4;
    16
}
/// $08EE48: lea.l $8ee4e(pc), a1
pub fn i_08ee48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EE4E;
    16
}
/// $08EE4C: move.l a1, (a6)
pub fn i_08ee4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EE4E: jsr $8d2f8(pc)
pub fn i_08ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE52);
    cpu.pc = 0x8D2F8;
    16
}
/// $08EE52: cmpi.w #$190, $24(a6)
pub fn i_08ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE58;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x190, Size::Word, false);
    16
}
/// $08EE58: bgt.w $8ee6e
pub fn i_08ee58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE5C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8EE6E; return 10; }
    8
}
/// $08EE5C: lea.l $2f44d6.l, a0
pub fn i_08ee5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F44D6;
    20
}
/// $08EE62: jsr $28cd4.l
pub fn i_08ee62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE68);
    cpu.pc = 0x28CD4;
    16
}
/// $08EE68: lea.l $8ee7a(pc), a1
pub fn i_08ee68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EE7A;
    16
}
/// $08EE6C: move.l a1, (a6)
pub fn i_08ee6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EE6E: jsr $8efb0(pc)
pub fn i_08ee6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE72);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08ee72  ($08EE72, 8 B, thunks/jsr_abs_thunks.c)
/// $08EE72: jsr $28d70.l
pub fn i_08ee72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE78);
    cpu.pc = 0x28D70;
    16
}
/// $08EE78: rts
pub fn i_08ee78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Lob_Landed_08ee7a  ($08EE7A, 28 B, enemies/critters_rings_08e4e6.s)
/// $08EE7A: jsr $4407a.l
pub fn i_08ee7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE80);
    cpu.pc = 0x4407A;
    16
}
/// $08EE80: jsr $8efb0(pc)
pub fn i_08ee80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EE84);
    cpu.pc = 0x8EFB0;
    16
}
/// $08EE84: jsr $28d70.l
pub fn i_08ee84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EE8A);
    cpu.pc = 0x28D70;
    16
}
/// $08EE8A: bcc.w $8ee94
pub fn i_08ee8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EE94; return 10; }
    8
}
/// $08EE8E: jmp $518.l
pub fn i_08ee8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE94;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EE94: rts
pub fn i_08ee94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shard_V0_08ee96  ($08EE96, 22 B, enemies/critters_rings_08e4e6.s)
/// $08EE96: lea.l $2f40c6.l, a0
pub fn i_08ee96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F40C6;
    20
}
/// $08EE9C: jsr $28cd4.l
pub fn i_08ee9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EEA2);
    cpu.pc = 0x28CD4;
    16
}
/// $08EEA2: move.b #$0, $74(a6)
pub fn i_08eea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEA8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EEA8: bra.w $8eed4
pub fn i_08eea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EED4;
    10
}
// ===== Shard_V1_08eeac  ($08EEAC, 22 B, enemies/critters_rings_08e4e6.s)
/// $08EEAC: lea.l $2f40f2.l, a0
pub fn i_08eeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F40F2;
    20
}
/// $08EEB2: jsr $28cd4.l
pub fn i_08eeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EEB8);
    cpu.pc = 0x28CD4;
    16
}
/// $08EEB8: move.b #$1, $74(a6)
pub fn i_08eeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EEBE: bra.w $8eed4
pub fn i_08eebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EED4;
    10
}
// ===== Shard_V2_08eec2  ($08EEC2, 118 B, enemies/critters_rings_08e4e6.s)
/// $08EEC2: lea.l $2f411e.l, a0
pub fn i_08eec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F411E;
    20
}
/// $08EEC8: jsr $28cd4.l
pub fn i_08eec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EECE);
    cpu.pc = 0x28CD4;
    16
}
/// $08EECE: move.b #$2, $74(a6)
pub fn i_08eece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EED4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08EED4: move.w #$1f, d0
pub fn i_08eed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EED8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EED8: jsr $5ea1c.l
pub fn i_08eed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EEDE);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EEDE: neg.w d0
pub fn i_08eede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEE0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08EEE0: move.w d0, $2a(a6)
pub fn i_08eee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEE4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EEE4: move.w #$1f, d0
pub fn i_08eee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEE8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EEE8: jsr $5ea1c.l
pub fn i_08eee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EEEE);
    cpu.pc = 0x5EA1C;
    16
}
/// $08EEEE: subi.w #$30, d0
pub fn i_08eeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEF2;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08EEF2: move.w d0, $2e(a6)
pub fn i_08eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEF6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08EEF6: jsr $8f002(pc)
pub fn i_08eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EEFA);
    cpu.pc = 0x8F002;
    16
}
/// $08EEFA: move.w #$ad, d1
pub fn i_08eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EEFE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08EEFE: jsr $236e.l
pub fn i_08eefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF04);
    cpu.pc = 0x236E;
    16
}
/// $08EF04: movea.l $c(a6), a1
pub fn i_08ef04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF08;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08EF08: movea.l $c(a1), a2
pub fn i_08ef08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF0C;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $08EF0C: move.w $24(a2), $5c(a6)
pub fn i_08ef0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF12;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08EF12: lea.l $8ef18(pc), a1
pub fn i_08ef12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EF18;
    16
}
/// $08EF16: move.l a1, (a6)
pub fn i_08ef16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EF18: jsr $8d34e(pc)
pub fn i_08ef18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EF1C);
    cpu.pc = 0x8D34E;
    16
}
/// $08EF1C: move.w $5c(a6), d0
pub fn i_08ef1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF20;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EF20: cmp.w $24(a6), d0
pub fn i_08ef20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF24;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08EF24: blt.w $8ef2e
pub fn i_08ef24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF28;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8EF2E; return 10; }
    8
}
/// $08EF28: lea.l $8ef40(pc), a1
pub fn i_08ef28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EF40;
    16
}
/// $08EF2C: move.l a1, (a6)
pub fn i_08ef2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EF2E: jsr $2783a.l
pub fn i_08ef2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF34);
    cpu.pc = 0x2783A;
    16
}
/// $08EF34: jsr $8efb0(pc)
pub fn i_08ef34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EF38);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08ef38  ($08EF38, 8 B, thunks/jsr_abs_thunks.c)
/// $08EF38: jsr $28d70.l
pub fn i_08ef38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF3E);
    cpu.pc = 0x28D70;
    16
}
/// $08EF3E: rts
pub fn i_08ef3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shard_Land_08ef40  ($08EF40, 112 B, enemies/critters_rings_08e4e6.s)
/// $08EF40: cmpi.b #$0, $74(a6)
pub fn i_08ef40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF46;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EF46: bne.w $8ef5a
pub fn i_08ef46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF4A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EF5A; return 10; }
    8
}
/// $08EF4A: lea.l $2f40d2.l, a0
pub fn i_08ef4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF50;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F40D2;
    20
}
/// $08EF50: jsr $28cd4.l
pub fn i_08ef50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF56);
    cpu.pc = 0x28CD4;
    16
}
/// $08EF56: bra.w $8ef8e
pub fn i_08ef56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EF8E;
    10
}
/// $08EF5A: cmpi.b #$0, $74(a6)
pub fn i_08ef5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF60;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EF60: bne.w $8ef74
pub fn i_08ef60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EF74; return 10; }
    8
}
/// $08EF64: lea.l $2f40fe.l, a0
pub fn i_08ef64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F40FE;
    20
}
/// $08EF6A: jsr $28cd4.l
pub fn i_08ef6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF70);
    cpu.pc = 0x28CD4;
    16
}
/// $08EF70: bra.w $8ef8e
pub fn i_08ef70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF74;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EF8E;
    10
}
/// $08EF74: cmpi.b #$0, $74(a6)
pub fn i_08ef74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF7A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08EF7A: bne.w $8ef8e
pub fn i_08ef7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8EF8E; return 10; }
    8
}
/// $08EF7E: lea.l $2f412a.l, a0
pub fn i_08ef7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F412A;
    20
}
/// $08EF84: jsr $28cd4.l
pub fn i_08ef84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF8A);
    cpu.pc = 0x28CD4;
    16
}
/// $08EF8A: bra.w $8ef8e
pub fn i_08ef8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8EF8E;
    10
}
/// $08EF8E: lea.l $8ef94(pc), a1
pub fn i_08ef8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EF94;
    16
}
/// $08EF92: move.l a1, (a6)
pub fn i_08ef92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08EF94: jsr $2783a.l
pub fn i_08ef94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EF9A);
    cpu.pc = 0x2783A;
    16
}
/// $08EF9A: jsr $8efb0(pc)
pub fn i_08ef9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EF9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8EF9E);
    cpu.pc = 0x8EFB0;
    16
}
/// $08EF9E: jsr $28d70.l
pub fn i_08ef9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EFA4);
    cpu.pc = 0x28D70;
    16
}
/// $08EFA4: bcc.w $8efae
pub fn i_08efa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EFAE; return 10; }
    8
}
/// $08EFA8: jmp $518.l
pub fn i_08efa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EFAE: rts
pub fn i_08efae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_GroundKill_08efb0  ($08EFB0, 16 B, enemies/critters_rings_08e4e6.s)
/// $08EFB0: movea.l #$ffffffff, a0
pub fn i_08efb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFB6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08EFB6: jsr $5dd5c.l
pub fn i_08efb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EFBC);
    cpu.pc = 0x5DD5C;
    16
}
/// $08EFBC: bcc.w $8efcc
pub fn i_08efbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EFCC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08efc0  ($08EFC0, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EFC0: jsr $5b6.l
pub fn i_08efc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EFC6);
    cpu.pc = 0x5B6;
    16
}
/// $08EFC6: jmp $518.l
pub fn i_08efc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFCC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EFCC: rts
pub fn i_08efcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_PlayerNearX_08efce  ($08EFCE, 40 B, enemies/critters_rings_08e4e6.s)
/// $08EFCE: lea.l $100440.l, a1
pub fn i_08efce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFD4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $08EFD4: move.w $22(a6), d1
pub fn i_08efd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFD8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EFD8: sub.w $22(a1), d1
pub fn i_08efd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFDC;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EFDC: cmp.w d0, d1
pub fn i_08efdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFDE;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $08EFDE: blt.w $8effc
pub fn i_08efde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFE2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8EFFC; return 10; }
    8
}
/// $08EFE2: lea.l $1004e0.l, a1
pub fn i_08efe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $08EFE8: move.w $22(a6), d1
pub fn i_08efe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFEC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08EFEC: sub.w $22(a1), d1
pub fn i_08efec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFF0;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08EFF0: cmp.w d0, d1
pub fn i_08eff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFF2;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $08EFF2: blt.w $8effc
pub fn i_08eff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFF6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8EFFC; return 10; }
    8
}
// ===== ClearXN_08eff6  ($08EFF6, 6 B, thunks/ccr_helpers.c)
/// $08EFF6: andi.b #$ee, ccr
pub fn i_08eff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFFA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08EFFA: rts
pub fn i_08effa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08effc  ($08EFFC, 6 B, thunks/ccr_helpers.c)
/// $08EFFC: ori.b #$11, ccr
pub fn i_08effc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F000;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F000: rts
pub fn i_08f000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F002;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_FacingFromParam_08f002  ($08F002, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F002: andi.b #$1, $98(a6)
pub fn i_08f002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F008;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08F008: move.b $98(a6), $3a(a6)
pub fn i_08f008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F00E;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08F00E: rts
pub fn i_08f00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F010;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_VelXFromParam_08f010  ($08F010, 28 B, enemies/critters_rings_08e4e6.s)
/// $08F010: clr.w d0
pub fn i_08f010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F012;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F012: move.b $99(a6), d0
pub fn i_08f012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F016;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F016: lsl.w #$5, d0
pub fn i_08f016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F018;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F018: move.w d0, $28(a6)
pub fn i_08f018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F01C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F01C: cmpi.b #$0, $3a(a6)
pub fn i_08f01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F022;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F022: bne.w $8f02a
pub fn i_08f022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F026;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F02A; return 10; }
    8
}
/// $08F026: neg.w $28(a6)
pub fn i_08f026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F02A;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F02A: rts
pub fn i_08f02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F02C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_ScrollTarget_08f02c  ($08F02C, 20 B, enemies/critters_rings_08e4e6.s)
/// $08F02C: clr.w d0
pub fn i_08f02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F02E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F02E: move.b $99(a6), d0
pub fn i_08f02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F032;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F032: move.w $106f50.l, $72(a6)
pub fn i_08f032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F03A;
    cpu.ir = 0x3D79;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad3, v2 as u16);
    24
}
/// $08F03A: add.w d0, $72(a6)
pub fn i_08f03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F03E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F03E: rts
pub fn i_08f03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F040;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_ScrollReached_08f040  ($08F040, 10 B, enemies/critters_rings_08e4e6.s)
/// $08F040: cmpi.b #$0, $99(a6)
pub fn i_08f040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F046;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F046: bne.w $8f050
pub fn i_08f046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F04A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F050; return 10; }
    8
}
// ===== ClearXN_08f04a  ($08F04A, 6 B, thunks/ccr_helpers.c)
/// $08F04A: andi.b #$ee, ccr
pub fn i_08f04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F04E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F04E: rts
pub fn i_08f04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F050;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_ScrollReached_Cmp_08f050  ($08F050, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F050: move.w $106f50.l, d0
pub fn i_08f050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F056;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $08F056: cmp.w $72(a6), d0
pub fn i_08f056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F05A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F05A: bcs.w $8f068
pub fn i_08f05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F05E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F068; return 10; }
    8
}
// ===== SetXN_08f05e  ($08F05E, 6 B, thunks/ccr_helpers.c)
/// $08F05E: ori.b #$11, ccr
pub fn i_08f05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F062;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F062: rts
pub fn i_08f062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F064;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_ScrollReached_Yes_08f064  ($08F064, 4 B, enemies/critters_rings_08e4e6.s)
/// $08F064: bra.w $8f06e
pub fn i_08f064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F068;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F06E;
    10
}
// ===== ClearXN_08f068  ($08F068, 6 B, thunks/ccr_helpers.c)
/// $08F068: andi.b #$ee, ccr
pub fn i_08f068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F06C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F06C: rts
pub fn i_08f06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F06E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rts_08f06e  ($08F06E, 2 B, enemies/critters_rings_08e4e6.s)
/// $08F06E: rts
pub fn i_08f06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F070;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Snd_ByParam9A_Base_08f070  ($08F070, 12 B, enemies/critters_rings_08e4e6.s)
/// $08F070: clr.w d0
pub fn i_08f070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F072;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F072: move.b $9a(a6), d0
pub fn i_08f072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F076;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F076: addi.w #$fa, d0
pub fn i_08f076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F07A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFA, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F07A: move.w d0, d1
pub fn i_08f07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F07C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_08f07c  ($08F07C, 8 B, thunks/jsr_abs_thunks.c)
/// $08F07C: jsr $236e.l
pub fn i_08f07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F082);
    cpu.pc = 0x236E;
    16
}
/// $08F082: rts
pub fn i_08f082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F084;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Snd_ByParam9A_A_08f084  ($08F084, 68 B, enemies/critters_rings_08e4e6.s)
/// $08F084: cmpi.b #$0, $9a(a6)
pub fn i_08f084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F08A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F08A: bne.w $8f098
pub fn i_08f08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F08E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F098; return 10; }
    8
}
/// $08F08E: move.w #$14a, d1
pub fn i_08f08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F092;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F092: jsr $236e.l
pub fn i_08f092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F098);
    cpu.pc = 0x236E;
    16
}
/// $08F098: cmpi.b #$1, $9a(a6)
pub fn i_08f098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F09E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08F09E: bne.w $8f0ac
pub fn i_08f09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F0AC; return 10; }
    8
}
/// $08F0A2: move.w #$f6, d1
pub fn i_08f0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F0A6: jsr $236e.l
pub fn i_08f0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0AC);
    cpu.pc = 0x236E;
    16
}
/// $08F0AC: cmpi.b #$1, $9a(a6)
pub fn i_08f0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0B2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08F0B2: bne.w $8f0c4
pub fn i_08f0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F0C4; return 10; }
    8
}
/// $08F0B6: move.w #$13d, d1
pub fn i_08f0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x13D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F0BA: jsr $236e.l
pub fn i_08f0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0C0);
    cpu.pc = 0x236E;
    16
}
/// $08F0C0: bra.w $8f0ce
pub fn i_08f0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F0CE;
    10
}
/// $08F0C4: move.w #$14a, d1
pub fn i_08f0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_08f0c8  ($08F0C8, 8 B, thunks/jsr_abs_thunks.c)
/// $08F0C8: jsr $236e.l
pub fn i_08f0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0CE);
    cpu.pc = 0x236E;
    16
}
/// $08F0CE: rts
pub fn i_08f0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Snd_ByParam9A_B_08f0d0  ($08F0D0, 48 B, enemies/critters_rings_08e4e6.s)
/// $08F0D0: cmpi.b #$0, $9a(a6)
pub fn i_08f0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0D6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F0D6: bne.w $8f0e4
pub fn i_08f0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F0E4; return 10; }
    8
}
/// $08F0DA: move.w #$149, d1
pub fn i_08f0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x149;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F0DE: jsr $236e.l
pub fn i_08f0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0E4);
    cpu.pc = 0x236E;
    16
}
/// $08F0E4: cmpi.b #$1, $9a(a6)
pub fn i_08f0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0EA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08F0EA: bne.w $8f0fc
pub fn i_08f0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F0FC; return 10; }
    8
}
/// $08F0EE: move.w #$f7, d1
pub fn i_08f0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F0F2: jsr $236e.l
pub fn i_08f0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F0F8);
    cpu.pc = 0x236E;
    16
}
/// $08F0F8: bra.w $8f106
pub fn i_08f0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F106;
    10
}
/// $08F0FC: move.w #$149, d1
pub fn i_08f0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F100;
    cpu.ir = 0x323C;
    let v1: u32 = 0x149;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_08f100  ($08F100, 8 B, thunks/jsr_abs_thunks.c)
/// $08F100: jsr $236e.l
pub fn i_08f100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F106);
    cpu.pc = 0x236E;
    16
}
/// $08F106: rts
pub fn i_08f106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F108;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prio_Set8018_08f108  ($08F108, 20 B, enemies/critters_rings_08e4e6.s)
/// $08F108: move.w #$8000, $38(a6)
pub fn i_08f108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F10E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F10E: andi.w #$ffe3, $38(a6)
pub fn i_08f10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F114;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08F114: ori.w #$18, $38(a6)
pub fn i_08f114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F11A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08F11A: rts
pub fn i_08f11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F11C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08f11c  ($08F11C, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F11C: movea.l $8(a6), a1
pub fn i_08f11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F120;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08F120: move.b $10(a6), d0
pub fn i_08f120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F124;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F124: cmp.b $10(a1), d0
pub fn i_08f124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F128;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08F128: bcs.w $8f132
pub fn i_08f128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F12C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F132; return 10; }
    8
}
// ===== ClearXN_08f12c  ($08F12C, 6 B, thunks/ccr_helpers.c)
/// $08F12C: andi.b #$ee, ccr
pub fn i_08f12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F130;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F130: rts
pub fn i_08f130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F132;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08f132  ($08F132, 6 B, thunks/ccr_helpers.c)
/// $08F132: ori.b #$11, ccr
pub fn i_08f132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F136;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F136: rts
pub fn i_08f136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F138;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ring_Reset_08f138  ($08F138, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F138: clr.w (a0)
pub fn i_08f138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F13A;
    cpu.ir = 0x4250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $08F13A: clr.w $2(a0)
pub fn i_08f13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F13E;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F13E: clr.w $4(a0)
pub fn i_08f13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F142;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F142: clr.w $6(a0)
pub fn i_08f142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F146;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F146: rts
pub fn i_08f146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ring_Compact_08f148  ($08F148, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F148: move.w $4(a0), (a0)
pub fn i_08f148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F14C;
    cpu.ir = 0x30A8;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v3 as u16);
    16
}
/// $08F14C: move.w $2(a0), $4(a0)
pub fn i_08f14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F152;
    cpu.ir = 0x3168;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F152: clr.w $6(a0)
pub fn i_08f152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F156;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F156: rts
pub fn i_08f156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F158;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rings_InitAll_08f158  ($08F158, 26 B, enemies/critters_rings_08e4e6.s)
/// $08F158: lea.l $10e2f2.l, a0
pub fn i_08f158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F15E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F15E: bsr.b $8f138
pub fn i_08f15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F160;
    cpu.ir = 0x61D8;
    cpu.push32(bus, 0x8F160);
    cpu.pc = 0x8F138;
    18
}
/// $08F160: lea.l $10e33a.l, a0
pub fn i_08f160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F166;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F166: bsr.b $8f138
pub fn i_08f166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F168;
    cpu.ir = 0x61D0;
    cpu.push32(bus, 0x8F168);
    cpu.pc = 0x8F138;
    18
}
/// $08F168: lea.l $10e362.l, a0
pub fn i_08f168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F16E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F16E: bsr.b $8f138
pub fn i_08f16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F170;
    cpu.ir = 0x61C8;
    cpu.push32(bus, 0x8F170);
    cpu.pc = 0x8F138;
    18
}
/// $08F170: rts
pub fn i_08f170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F172;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rings_CompactAll_08f172  ($08F172, 26 B, enemies/critters_rings_08e4e6.s)
/// $08F172: lea.l $10e2f2.l, a0
pub fn i_08f172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F178;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F178: bsr.b $8f148
pub fn i_08f178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F17A;
    cpu.ir = 0x61CE;
    cpu.push32(bus, 0x8F17A);
    cpu.pc = 0x8F148;
    18
}
/// $08F17A: lea.l $10e33a.l, a0
pub fn i_08f17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F180;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F180: bsr.b $8f148
pub fn i_08f180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F182;
    cpu.ir = 0x61C6;
    cpu.push32(bus, 0x8F182);
    cpu.pc = 0x8F148;
    18
}
/// $08F182: lea.l $10e362.l, a0
pub fn i_08f182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F188;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F188: bsr.b $8f148
pub fn i_08f188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F18A;
    cpu.ir = 0x61BE;
    cpu.push32(bus, 0x8F18A);
    cpu.pc = 0x8F148;
    18
}
/// $08F18A: rts
pub fn i_08f18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F18C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Zone_Tmpl152_08f18c  ($08F18C, 82 B, enemies/critters_rings_08e4e6.s)
/// $08F18C: move.w $22(a6), d0
pub fn i_08f18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F190;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F190: add.w $106f50.l, d0
pub fn i_08f190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F196;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F196: move.w d0, $22(a6)
pub fn i_08f196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F19A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F19A: clr.w d1
pub fn i_08f19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F19C;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F19C: move.b $98(a6), d1
pub fn i_08f19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1A0;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F1A0: lsl.w #$4, d1
pub fn i_08f1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1A2;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $08F1A2: add.w d1, d0
pub fn i_08f1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1A4;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F1A4: move.w d0, $70(a6)
pub fn i_08f1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1A8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F1A8: move.w $24(a6), d0
pub fn i_08f1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1AC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F1AC: subi.w #$200, d0
pub fn i_08f1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1B0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F1B0: neg.w d0
pub fn i_08f1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1B2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F1B2: add.w $106f54.l, d0
pub fn i_08f1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1B8;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F1B8: move.w d0, $24(a6)
pub fn i_08f1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1BC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F1BC: clr.w d1
pub fn i_08f1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1BE;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F1BE: move.b $99(a6), d1
pub fn i_08f1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1C2;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F1C2: lsl.w #$4, d1
pub fn i_08f1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1C4;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $08F1C4: add.w d1, d0
pub fn i_08f1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1C6;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F1C6: move.w d0, $72(a6)
pub fn i_08f1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1CA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F1CA: lea.l $8f1d0(pc), a1
pub fn i_08f1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8F1D0;
    16
}
/// $08F1CE: move.l a1, (a6)
pub fn i_08f1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08F1D0: move.w $70(a6), d0
pub fn i_08f1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1D4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F1D4: cmp.w $106f50.l, d0
pub fn i_08f1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1DA;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $08F1DA: bcc.w $8f1ec
pub fn i_08f1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F1EC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08f1de  ($08F1DE, 14 B, kernel/jsr5b6_jmp518.c)
/// $08F1DE: jsr $5b6.l
pub fn i_08f1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F1E4);
    cpu.pc = 0x5B6;
    16
}
/// $08F1E4: jmp $518.l
pub fn i_08f1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1EA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== ZoneRing_Push_08f1ec  ($08F1EC, 72 B, enemies/critters_rings_08e4e6.s)
/// $08F1EC: lea.l $10e2f2.l, a0
pub fn i_08f1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F1F2: lea.l $8(a0), a1
pub fn i_08f1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1F6;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F1F6: adda.w $2(a0), a1
pub fn i_08f1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1FA;
    cpu.ir = 0xD2E8;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $08F1FA: move.w $22(a6), (a1)
pub fn i_08f1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F1FE;
    cpu.ir = 0x32AE;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v3 as u16);
    16
}
/// $08F1FE: move.w $24(a6), $2(a1)
pub fn i_08f1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F204;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F204: move.w $70(a6), $4(a1)
pub fn i_08f204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F20A;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x4);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F20A: move.w $72(a6), $6(a1)
pub fn i_08f20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F210;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x6);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F210: cmpi.w #$4, $6(a0)
pub fn i_08f210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F216;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F216: bcc.w $8f232
pub fn i_08f216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F21A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F232; return 10; }
    8
}
/// $08F21A: move.w $2(a0), d7
pub fn i_08f21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F21E;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F21E: addq.w #$8, d7
pub fn i_08f21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F220;
    cpu.ir = 0x5047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F220: cmpi.w #$40, d7
pub fn i_08f220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F224;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F224: bcs.w $8f22a
pub fn i_08f224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F228;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F22A; return 10; }
    8
}
/// $08F228: clr.w d7
pub fn i_08f228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F22A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F22A: move.w d7, $2(a0)
pub fn i_08f22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F22E;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F22E: addq.w #$1, $6(a0)
pub fn i_08f22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F232;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F232: rts
pub fn i_08f232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F234;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_PushOffset_08f234  ($08F234, 108 B, enemies/critters_rings_08e4e6.s)
/// $08F234: move.w $22(a6), d0
pub fn i_08f234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F238;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F238: move.w $24(a6), d1
pub fn i_08f238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F23C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F23C: add.w $106f50.l, d0
pub fn i_08f23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F242;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F242: subi.w #$200, d1
pub fn i_08f242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F246;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F246: neg.w d1
pub fn i_08f246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F248;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F248: add.w $106f54.l, d1
pub fn i_08f248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F24E;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F24E: move.w d0, d2
pub fn i_08f24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F250;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F250: move.w d1, d3
pub fn i_08f250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F252;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F252: add.w (a0), d0
pub fn i_08f252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F254;
    cpu.ir = 0xD050;
    let v2 = bus.read16(cpu.a[0]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08F254: add.w $2(a0), d2
pub fn i_08f254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F258;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F258: add.w $4(a0), d1
pub fn i_08f258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F25C;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F25C: add.w $6(a0), d3
pub fn i_08f25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F260;
    cpu.ir = 0xD668;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F260: lea.l $10e2f2.l, a0
pub fn i_08f260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F266: lea.l $8(a0), a1
pub fn i_08f266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F26A;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F26A: adda.w $2(a0), a1
pub fn i_08f26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F26E;
    cpu.ir = 0xD2E8;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $08F26E: move.w d0, (a1)
pub fn i_08f26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F270;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $08F270: move.w d1, $2(a1)
pub fn i_08f270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F274;
    cpu.ir = 0x3341;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F274: move.w d2, $4(a1)
pub fn i_08f274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F278;
    cpu.ir = 0x3342;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F278: move.w d3, $6(a1)
pub fn i_08f278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F27C;
    cpu.ir = 0x3343;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F27C: cmpi.w #$4, $6(a0)
pub fn i_08f27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F282;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F282: bcc.w $8f29e
pub fn i_08f282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F286;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F29E; return 10; }
    8
}
/// $08F286: move.w $2(a0), d7
pub fn i_08f286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F28A;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F28A: addq.w #$8, d7
pub fn i_08f28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F28C;
    cpu.ir = 0x5047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F28C: cmpi.w #$40, d7
pub fn i_08f28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F290;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F290: bcs.w $8f296
pub fn i_08f290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F294;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F296; return 10; }
    8
}
/// $08F294: clr.w d7
pub fn i_08f294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F296;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F296: move.w d7, $2(a0)
pub fn i_08f296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F29A;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F29A: addq.w #$1, $6(a0)
pub fn i_08f29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F29E;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F29E: rts
pub fn i_08f29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_HitTest_08f2a0  ($08F2A0, 78 B, enemies/critters_rings_08e4e6.s)
/// $08F2A0: move.w $22(a6), d0
pub fn i_08f2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2A4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F2A4: add.w $106f50.l, d0
pub fn i_08f2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2AA;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F2AA: move.w $24(a6), d1
pub fn i_08f2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2AE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F2AE: subi.w #$200, d1
pub fn i_08f2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2B2;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x08D8CC, i_08d8cc),
    (0x08D8D2, i_08d8d2),
    (0x08D8D6, i_08d8d6),
    (0x08D8DC, i_08d8dc),
    (0x08D8E4, i_08d8e4),
    (0x08D8EC, i_08d8ec),
    (0x08D8F0, i_08d8f0),
    (0x08D8F2, i_08d8f2),
    (0x08D8F4, i_08d8f4),
    (0x08D8FC, i_08d8fc),
    (0x08D900, i_08d900),
    (0x08D906, i_08d906),
    (0x08D910, i_08d910),
    (0x08D918, i_08d918),
    (0x08D920, i_08d920),
    (0x08D924, i_08d924),
    (0x08D926, i_08d926),
    (0x08D92A, i_08d92a),
    (0x08D930, i_08d930),
    (0x08D936, i_08d936),
    (0x08D938, i_08d938),
    (0x08D93C, i_08d93c),
    (0x08D942, i_08d942),
    (0x08D946, i_08d946),
    (0x08D94A, i_08d94a),
    (0x08D950, i_08d950),
    (0x08D956, i_08d956),
    (0x08D95C, i_08d95c),
    (0x08D962, i_08d962),
    (0x08D968, i_08d968),
    (0x08D972, i_08d972),
    (0x08D976, i_08d976),
    (0x08D978, i_08d978),
    (0x08D97E, i_08d97e),
    (0x08D984, i_08d984),
    (0x08D986, i_08d986),
    (0x08D98C, i_08d98c),
    (0x08D992, i_08d992),
    (0x08D994, i_08d994),
    (0x08D998, i_08d998),
    (0x08D99C, i_08d99c),
    (0x08D9A0, i_08d9a0),
    (0x08D9A4, i_08d9a4),
    (0x08D9AA, i_08d9aa),
    (0x08D9B0, i_08d9b0),
    (0x08D9B4, i_08d9b4),
    (0x08D9B6, i_08d9b6),
    (0x08D9BA, i_08d9ba),
    (0x08D9BE, i_08d9be),
    (0x08D9C4, i_08d9c4),
    (0x08D9CA, i_08d9ca),
    (0x08D9CE, i_08d9ce),
    (0x08D9D0, i_08d9d0),
    (0x08D9D6, i_08d9d6),
    (0x08D9DA, i_08d9da),
    (0x08D9E0, i_08d9e0),
    (0x08D9E2, i_08d9e2),
    (0x08D9E6, i_08d9e6),
    (0x08D9EA, i_08d9ea),
    (0x08D9EE, i_08d9ee),
    (0x08D9F2, i_08d9f2),
    (0x08D9F8, i_08d9f8),
    (0x08D9FE, i_08d9fe),
    (0x08DA02, i_08da02),
    (0x08DA04, i_08da04),
    (0x08DA08, i_08da08),
    (0x08DA0C, i_08da0c),
    (0x08DA12, i_08da12),
    (0x08DA18, i_08da18),
    (0x08DA1C, i_08da1c),
    (0x08DA1E, i_08da1e),
    (0x08DA24, i_08da24),
    (0x08DA28, i_08da28),
    (0x08DA2E, i_08da2e),
    (0x08DA30, i_08da30),
    (0x08DA36, i_08da36),
    (0x08DA3C, i_08da3c),
    (0x08DA42, i_08da42),
    (0x08DA48, i_08da48),
    (0x08DA4C, i_08da4c),
    (0x08DA50, i_08da50),
    (0x08DA54, i_08da54),
    (0x08DA58, i_08da58),
    (0x08DA5C, i_08da5c),
    (0x08DA60, i_08da60),
    (0x08DA66, i_08da66),
    (0x08DA6C, i_08da6c),
    (0x08DA70, i_08da70),
    (0x08DA72, i_08da72),
    (0x08DA76, i_08da76),
    (0x08DA7A, i_08da7a),
    (0x08DA80, i_08da80),
    (0x08DA86, i_08da86),
    (0x08DA8A, i_08da8a),
    (0x08DA8C, i_08da8c),
    (0x08DA92, i_08da92),
    (0x08DA96, i_08da96),
    (0x08DA9C, i_08da9c),
    (0x08DA9E, i_08da9e),
    (0x08DAA4, i_08daa4),
    (0x08DAA8, i_08daa8),
    (0x08DAAC, i_08daac),
    (0x08DAB2, i_08dab2),
    (0x08DAB8, i_08dab8),
    (0x08DABE, i_08dabe),
    (0x08DAC4, i_08dac4),
    (0x08DACA, i_08daca),
    (0x08DACE, i_08dace),
    (0x08DAD0, i_08dad0),
    (0x08DAD6, i_08dad6),
    (0x08DADA, i_08dada),
    (0x08DAE0, i_08dae0),
    (0x08DAE2, i_08dae2),
    (0x08DAE8, i_08dae8),
    (0x08DAEE, i_08daee),
    (0x08DAF4, i_08daf4),
    (0x08DAF8, i_08daf8),
    (0x08DAFC, i_08dafc),
    (0x08DB00, i_08db00),
    (0x08DB06, i_08db06),
    (0x08DB0C, i_08db0c),
    (0x08DB10, i_08db10),
    (0x08DB16, i_08db16),
    (0x08DB1C, i_08db1c),
    (0x08DB20, i_08db20),
    (0x08DB22, i_08db22),
    (0x08DB28, i_08db28),
    (0x08DB2C, i_08db2c),
    (0x08DB32, i_08db32),
    (0x08DB38, i_08db38),
    (0x08DB3C, i_08db3c),
    (0x08DB42, i_08db42),
    (0x08DB48, i_08db48),
    (0x08DB4E, i_08db4e),
    (0x08DB50, i_08db50),
    (0x08DB56, i_08db56),
    (0x08DB5A, i_08db5a),
    (0x08DB5E, i_08db5e),
    (0x08DB62, i_08db62),
    (0x08DB68, i_08db68),
    (0x08DB6E, i_08db6e),
    (0x08DB74, i_08db74),
    (0x08DB78, i_08db78),
    (0x08DB7A, i_08db7a),
    (0x08DB80, i_08db80),
    (0x08DB86, i_08db86),
    (0x08DB8A, i_08db8a),
    (0x08DB90, i_08db90),
    (0x08DB94, i_08db94),
    (0x08DB9A, i_08db9a),
    (0x08DBA0, i_08dba0),
    (0x08DBA4, i_08dba4),
    (0x08DBA6, i_08dba6),
    (0x08DBAC, i_08dbac),
    (0x08DBB0, i_08dbb0),
    (0x08DBB6, i_08dbb6),
    (0x08DBBC, i_08dbbc),
    (0x08DBC0, i_08dbc0),
    (0x08DBC4, i_08dbc4),
    (0x08DBC6, i_08dbc6),
    (0x08DBC8, i_08dbc8),
    (0x08DBCC, i_08dbcc),
    (0x08DBD2, i_08dbd2),
    (0x08DBD6, i_08dbd6),
    (0x08DBDA, i_08dbda),
    (0x08DBDC, i_08dbdc),
    (0x08DBE0, i_08dbe0),
    (0x08DBE2, i_08dbe2),
    (0x08DBE8, i_08dbe8),
    (0x08DBEE, i_08dbee),
    (0x08DBF2, i_08dbf2),
    (0x08DBF8, i_08dbf8),
    (0x08DBFC, i_08dbfc),
    (0x08DC02, i_08dc02),
    (0x08DC08, i_08dc08),
    (0x08DC0C, i_08dc0c),
    (0x08DC0E, i_08dc0e),
    (0x08DC14, i_08dc14),
    (0x08DC18, i_08dc18),
    (0x08DC1E, i_08dc1e),
    (0x08DC24, i_08dc24),
    (0x08DC28, i_08dc28),
    (0x08DC2C, i_08dc2c),
    (0x08DC2E, i_08dc2e),
    (0x08DC30, i_08dc30),
    (0x08DC34, i_08dc34),
    (0x08DC3A, i_08dc3a),
    (0x08DC3E, i_08dc3e),
    (0x08DC42, i_08dc42),
    (0x08DC44, i_08dc44),
    (0x08DC48, i_08dc48),
    (0x08DC4A, i_08dc4a),
    (0x08DC50, i_08dc50),
    (0x08DC56, i_08dc56),
    (0x08DC5A, i_08dc5a),
    (0x08DC5E, i_08dc5e),
    (0x08DC62, i_08dc62),
    (0x08DC66, i_08dc66),
    (0x08DC6A, i_08dc6a),
    (0x08DC70, i_08dc70),
    (0x08DC76, i_08dc76),
    (0x08DC7C, i_08dc7c),
    (0x08DC82, i_08dc82),
    (0x08DC88, i_08dc88),
    (0x08DC8C, i_08dc8c),
    (0x08DC8E, i_08dc8e),
    (0x08DC92, i_08dc92),
    (0x08DC94, i_08dc94),
    (0x08DC9A, i_08dc9a),
    (0x08DCA0, i_08dca0),
    (0x08DCA4, i_08dca4),
    (0x08DCA6, i_08dca6),
    (0x08DCAC, i_08dcac),
    (0x08DCB2, i_08dcb2),
    (0x08DCB6, i_08dcb6),
    (0x08DCBC, i_08dcbc),
    (0x08DCC0, i_08dcc0),
    (0x08DCC4, i_08dcc4),
    (0x08DCC8, i_08dcc8),
    (0x08DCCC, i_08dccc),
    (0x08DCD0, i_08dcd0),
    (0x08DCD2, i_08dcd2),
    (0x08DCD6, i_08dcd6),
    (0x08DCDA, i_08dcda),
    (0x08DCDE, i_08dcde),
    (0x08DCE2, i_08dce2),
    (0x08DCE6, i_08dce6),
    (0x08DCEA, i_08dcea),
    (0x08DCEC, i_08dcec),
    (0x08DCF0, i_08dcf0),
    (0x08DCF4, i_08dcf4),
    (0x08DCF8, i_08dcf8),
    (0x08DCFA, i_08dcfa),
    (0x08DD00, i_08dd00),
    (0x08DD06, i_08dd06),
    (0x08DD0A, i_08dd0a),
    (0x08DD0C, i_08dd0c),
    (0x08DD12, i_08dd12),
    (0x08DD18, i_08dd18),
    (0x08DD1C, i_08dd1c),
    (0x08DD20, i_08dd20),
    (0x08DD24, i_08dd24),
    (0x08DD26, i_08dd26),
    (0x08DD2A, i_08dd2a),
    (0x08DD30, i_08dd30),
    (0x08DD34, i_08dd34),
    (0x08DD38, i_08dd38),
    (0x08DD3E, i_08dd3e),
    (0x08DD42, i_08dd42),
    (0x08DD48, i_08dd48),
    (0x08DD4C, i_08dd4c),
    (0x08DD52, i_08dd52),
    (0x08DD58, i_08dd58),
    (0x08DD5C, i_08dd5c),
    (0x08DD5E, i_08dd5e),
    (0x08DD64, i_08dd64),
    (0x08DD68, i_08dd68),
    (0x08DD6E, i_08dd6e),
    (0x08DD72, i_08dd72),
    (0x08DD76, i_08dd76),
    (0x08DD78, i_08dd78),
    (0x08DD7C, i_08dd7c),
    (0x08DD7E, i_08dd7e),
    (0x08DD84, i_08dd84),
    (0x08DD8A, i_08dd8a),
    (0x08DD8E, i_08dd8e),
    (0x08DD94, i_08dd94),
    (0x08DD98, i_08dd98),
    (0x08DD9E, i_08dd9e),
    (0x08DDA4, i_08dda4),
    (0x08DDA8, i_08dda8),
    (0x08DDAA, i_08ddaa),
    (0x08DDAE, i_08ddae),
    (0x08DDB0, i_08ddb0),
    (0x08DDB4, i_08ddb4),
    (0x08DDBA, i_08ddba),
    (0x08DDBE, i_08ddbe),
    (0x08DDC2, i_08ddc2),
    (0x08DDC8, i_08ddc8),
    (0x08DDCE, i_08ddce),
    (0x08DDD2, i_08ddd2),
    (0x08DDD4, i_08ddd4),
    (0x08DDD8, i_08ddd8),
    (0x08DDDA, i_08ddda),
    (0x08DDDE, i_08ddde),
    (0x08DDE4, i_08dde4),
    (0x08DDE8, i_08dde8),
    (0x08DDEA, i_08ddea),
    (0x08DDEE, i_08ddee),
    (0x08DDF2, i_08ddf2),
    (0x08DDF8, i_08ddf8),
    (0x08DDFC, i_08ddfc),
    (0x08DE00, i_08de00),
    (0x08DE06, i_08de06),
    (0x08DE0C, i_08de0c),
    (0x08DE12, i_08de12),
    (0x08DE18, i_08de18),
    (0x08DE1E, i_08de1e),
    (0x08DE22, i_08de22),
    (0x08DE24, i_08de24),
    (0x08DE28, i_08de28),
    (0x08DE2A, i_08de2a),
    (0x08DE30, i_08de30),
    (0x08DE34, i_08de34),
    (0x08DE3A, i_08de3a),
    (0x08DE3E, i_08de3e),
    (0x08DE44, i_08de44),
    (0x08DE48, i_08de48),
    (0x08DE4E, i_08de4e),
    (0x08DE54, i_08de54),
    (0x08DE58, i_08de58),
    (0x08DE5C, i_08de5c),
    (0x08DE5E, i_08de5e),
    (0x08DE62, i_08de62),
    (0x08DE64, i_08de64),
    (0x08DE6A, i_08de6a),
    (0x08DE70, i_08de70),
    (0x08DE74, i_08de74),
    (0x08DE78, i_08de78),
    (0x08DE7C, i_08de7c),
    (0x08DE80, i_08de80),
    (0x08DE86, i_08de86),
    (0x08DE8A, i_08de8a),
    (0x08DE8C, i_08de8c),
    (0x08DE92, i_08de92),
    (0x08DE98, i_08de98),
    (0x08DE9C, i_08de9c),
    (0x08DE9E, i_08de9e),
    (0x08DEA4, i_08dea4),
    (0x08DEAA, i_08deaa),
    (0x08DEAE, i_08deae),
    (0x08DEB2, i_08deb2),
    (0x08DEB6, i_08deb6),
    (0x08DEBA, i_08deba),
    (0x08DEBC, i_08debc),
    (0x08DEC0, i_08dec0),
    (0x08DEC2, i_08dec2),
    (0x08DEC6, i_08dec6),
    (0x08DECC, i_08decc),
    (0x08DED2, i_08ded2),
    (0x08DED8, i_08ded8),
    (0x08DEDE, i_08dede),
    (0x08DEE0, i_08dee0),
    (0x08DEE6, i_08dee6),
    (0x08DEEC, i_08deec),
    (0x08DEF0, i_08def0),
    (0x08DEF6, i_08def6),
    (0x08DEFC, i_08defc),
    (0x08DF02, i_08df02),
    (0x08DF06, i_08df06),
    (0x08DF0C, i_08df0c),
    (0x08DF10, i_08df10),
    (0x08DF14, i_08df14),
    (0x08DF18, i_08df18),
    (0x08DF1C, i_08df1c),
    (0x08DF22, i_08df22),
    (0x08DF28, i_08df28),
    (0x08DF2E, i_08df2e),
    (0x08DF32, i_08df32),
    (0x08DF34, i_08df34),
    (0x08DF3A, i_08df3a),
    (0x08DF3E, i_08df3e),
    (0x08DF42, i_08df42),
    (0x08DF48, i_08df48),
    (0x08DF4A, i_08df4a),
    (0x08DF4E, i_08df4e),
    (0x08DF52, i_08df52),
    (0x08DF56, i_08df56),
    (0x08DF5A, i_08df5a),
    (0x08DF5E, i_08df5e),
    (0x08DF64, i_08df64),
    (0x08DF6A, i_08df6a),
    (0x08DF6E, i_08df6e),
    (0x08DF70, i_08df70),
    (0x08DF72, i_08df72),
    (0x08DF76, i_08df76),
    (0x08DF7C, i_08df7c),
    (0x08DF80, i_08df80),
    (0x08DF84, i_08df84),
    (0x08DF8A, i_08df8a),
    (0x08DF8C, i_08df8c),
    (0x08DF92, i_08df92),
    (0x08DF94, i_08df94),
    (0x08DF98, i_08df98),
    (0x08DF9C, i_08df9c),
    (0x08DFA0, i_08dfa0),
    (0x08DFA4, i_08dfa4),
    (0x08DFAA, i_08dfaa),
    (0x08DFB0, i_08dfb0),
    (0x08DFB4, i_08dfb4),
    (0x08DFB6, i_08dfb6),
    (0x08DFB8, i_08dfb8),
    (0x08DFBE, i_08dfbe),
    (0x08DFC4, i_08dfc4),
    (0x08DFC8, i_08dfc8),
    (0x08DFCA, i_08dfca),
    (0x08DFD0, i_08dfd0),
    (0x08DFD6, i_08dfd6),
    (0x08DFDA, i_08dfda),
    (0x08DFDE, i_08dfde),
    (0x08DFE2, i_08dfe2),
    (0x08DFE6, i_08dfe6),
    (0x08DFEA, i_08dfea),
    (0x08DFF0, i_08dff0),
    (0x08DFF6, i_08dff6),
    (0x08DFFA, i_08dffa),
    (0x08E000, i_08e000),
    (0x08E004, i_08e004),
    (0x08E008, i_08e008),
    (0x08E00C, i_08e00c),
    (0x08E010, i_08e010),
    (0x08E012, i_08e012),
    (0x08E016, i_08e016),
    (0x08E01A, i_08e01a),
    (0x08E01E, i_08e01e),
    (0x08E022, i_08e022),
    (0x08E028, i_08e028),
    (0x08E02A, i_08e02a),
    (0x08E030, i_08e030),
    (0x08E036, i_08e036),
    (0x08E03A, i_08e03a),
    (0x08E03C, i_08e03c),
    (0x08E042, i_08e042),
    (0x08E048, i_08e048),
    (0x08E04C, i_08e04c),
    (0x08E050, i_08e050),
    (0x08E054, i_08e054),
    (0x08E058, i_08e058),
    (0x08E05E, i_08e05e),
    (0x08E060, i_08e060),
    (0x08E066, i_08e066),
    (0x08E06C, i_08e06c),
    (0x08E070, i_08e070),
    (0x08E072, i_08e072),
    (0x08E078, i_08e078),
    (0x08E07C, i_08e07c),
    (0x08E082, i_08e082),
    (0x08E086, i_08e086),
    (0x08E08A, i_08e08a),
    (0x08E08C, i_08e08c),
    (0x08E090, i_08e090),
    (0x08E094, i_08e094),
    (0x08E09A, i_08e09a),
    (0x08E09C, i_08e09c),
    (0x08E0A0, i_08e0a0),
    (0x08E0A6, i_08e0a6),
    (0x08E0AC, i_08e0ac),
    (0x08E0B2, i_08e0b2),
    (0x08E0B8, i_08e0b8),
    (0x08E0BA, i_08e0ba),
    (0x08E0C0, i_08e0c0),
    (0x08E0C6, i_08e0c6),
    (0x08E0CA, i_08e0ca),
    (0x08E0D0, i_08e0d0),
    (0x08E0D6, i_08e0d6),
    (0x08E0DC, i_08e0dc),
    (0x08E0E0, i_08e0e0),
    (0x08E0E4, i_08e0e4),
    (0x08E0E8, i_08e0e8),
    (0x08E0EC, i_08e0ec),
    (0x08E0F2, i_08e0f2),
    (0x08E0F8, i_08e0f8),
    (0x08E0FC, i_08e0fc),
    (0x08E0FE, i_08e0fe),
    (0x08E104, i_08e104),
    (0x08E10A, i_08e10a),
    (0x08E10C, i_08e10c),
    (0x08E110, i_08e110),
    (0x08E116, i_08e116),
    (0x08E11A, i_08e11a),
    (0x08E11E, i_08e11e),
    (0x08E122, i_08e122),
    (0x08E126, i_08e126),
    (0x08E12A, i_08e12a),
    (0x08E130, i_08e130),
    (0x08E136, i_08e136),
    (0x08E13A, i_08e13a),
    (0x08E140, i_08e140),
    (0x08E144, i_08e144),
    (0x08E148, i_08e148),
    (0x08E14C, i_08e14c),
    (0x08E150, i_08e150),
    (0x08E152, i_08e152),
    (0x08E156, i_08e156),
    (0x08E15C, i_08e15c),
    (0x08E162, i_08e162),
    (0x08E166, i_08e166),
    (0x08E16A, i_08e16a),
    (0x08E170, i_08e170),
    (0x08E172, i_08e172),
    (0x08E174, i_08e174),
    (0x08E178, i_08e178),
    (0x08E17A, i_08e17a),
    (0x08E17E, i_08e17e),
    (0x08E184, i_08e184),
    (0x08E186, i_08e186),
    (0x08E18A, i_08e18a),
    (0x08E190, i_08e190),
    (0x08E194, i_08e194),
    (0x08E19A, i_08e19a),
    (0x08E19C, i_08e19c),
    (0x08E1A0, i_08e1a0),
    (0x08E1A4, i_08e1a4),
    (0x08E1A8, i_08e1a8),
    (0x08E1AC, i_08e1ac),
    (0x08E1B2, i_08e1b2),
    (0x08E1B8, i_08e1b8),
    (0x08E1BE, i_08e1be),
    (0x08E1C2, i_08e1c2),
    (0x08E1C4, i_08e1c4),
    (0x08E1C8, i_08e1c8),
    (0x08E1CC, i_08e1cc),
    (0x08E1D2, i_08e1d2),
    (0x08E1D8, i_08e1d8),
    (0x08E1DC, i_08e1dc),
    (0x08E1DE, i_08e1de),
    (0x08E1E4, i_08e1e4),
    (0x08E1E8, i_08e1e8),
    (0x08E1EC, i_08e1ec),
    (0x08E1F2, i_08e1f2),
    (0x08E1F8, i_08e1f8),
    (0x08E1FA, i_08e1fa),
    (0x08E200, i_08e200),
    (0x08E206, i_08e206),
    (0x08E20C, i_08e20c),
    (0x08E212, i_08e212),
    (0x08E218, i_08e218),
    (0x08E21C, i_08e21c),
    (0x08E220, i_08e220),
    (0x08E224, i_08e224),
    (0x08E22A, i_08e22a),
    (0x08E22E, i_08e22e),
    (0x08E234, i_08e234),
    (0x08E23A, i_08e23a),
    (0x08E240, i_08e240),
    (0x08E246, i_08e246),
    (0x08E24C, i_08e24c),
    (0x08E250, i_08e250),
    (0x08E252, i_08e252),
    (0x08E256, i_08e256),
    (0x08E25C, i_08e25c),
    (0x08E260, i_08e260),
    (0x08E266, i_08e266),
    (0x08E26A, i_08e26a),
    (0x08E270, i_08e270),
    (0x08E274, i_08e274),
    (0x08E27A, i_08e27a),
    (0x08E280, i_08e280),
    (0x08E284, i_08e284),
    (0x08E286, i_08e286),
    (0x08E288, i_08e288),
    (0x08E28C, i_08e28c),
    (0x08E290, i_08e290),
    (0x08E296, i_08e296),
    (0x08E29C, i_08e29c),
    (0x08E2A0, i_08e2a0),
    (0x08E2A2, i_08e2a2),
    (0x08E2A6, i_08e2a6),
    (0x08E2AA, i_08e2aa),
    (0x08E2AC, i_08e2ac),
    (0x08E2B0, i_08e2b0),
    (0x08E2B4, i_08e2b4),
    (0x08E2B8, i_08e2b8),
    (0x08E2BC, i_08e2bc),
    (0x08E2C2, i_08e2c2),
    (0x08E2C8, i_08e2c8),
    (0x08E2CE, i_08e2ce),
    (0x08E2D4, i_08e2d4),
    (0x08E2DA, i_08e2da),
    (0x08E2DE, i_08e2de),
    (0x08E2E0, i_08e2e0),
    (0x08E2E4, i_08e2e4),
    (0x08E2E8, i_08e2e8),
    (0x08E2EC, i_08e2ec),
    (0x08E2EE, i_08e2ee),
    (0x08E2F0, i_08e2f0),
    (0x08E2F6, i_08e2f6),
    (0x08E2FA, i_08e2fa),
    (0x08E300, i_08e300),
    (0x08E304, i_08e304),
    (0x08E30A, i_08e30a),
    (0x08E30C, i_08e30c),
    (0x08E310, i_08e310),
    (0x08E314, i_08e314),
    (0x08E318, i_08e318),
    (0x08E31C, i_08e31c),
    (0x08E322, i_08e322),
    (0x08E328, i_08e328),
    (0x08E32E, i_08e32e),
    (0x08E334, i_08e334),
    (0x08E33A, i_08e33a),
    (0x08E340, i_08e340),
    (0x08E344, i_08e344),
    (0x08E34A, i_08e34a),
    (0x08E34E, i_08e34e),
    (0x08E354, i_08e354),
    (0x08E358, i_08e358),
    (0x08E35E, i_08e35e),
    (0x08E362, i_08e362),
    (0x08E366, i_08e366),
    (0x08E368, i_08e368),
    (0x08E36C, i_08e36c),
    (0x08E372, i_08e372),
    (0x08E376, i_08e376),
    (0x08E37A, i_08e37a),
    (0x08E380, i_08e380),
    (0x08E384, i_08e384),
    (0x08E388, i_08e388),
    (0x08E38C, i_08e38c),
    (0x08E390, i_08e390),
    (0x08E394, i_08e394),
    (0x08E396, i_08e396),
    (0x08E39A, i_08e39a),
    (0x08E39E, i_08e39e),
    (0x08E3A2, i_08e3a2),
    (0x08E3A6, i_08e3a6),
    (0x08E3AA, i_08e3aa),
    (0x08E3AC, i_08e3ac),
    (0x08E3B0, i_08e3b0),
    (0x08E3B4, i_08e3b4),
    (0x08E3BA, i_08e3ba),
    (0x08E3BC, i_08e3bc),
    (0x08E3C2, i_08e3c2),
    (0x08E3C4, i_08e3c4),
    (0x08E3C8, i_08e3c8),
    (0x08E3CE, i_08e3ce),
    (0x08E3D2, i_08e3d2),
    (0x08E3D6, i_08e3d6),
    (0x08E3D8, i_08e3d8),
    (0x08E3DC, i_08e3dc),
    (0x08E3E2, i_08e3e2),
    (0x08E3E6, i_08e3e6),
    (0x08E3EA, i_08e3ea),
    (0x08E3EC, i_08e3ec),
    (0x08E3F0, i_08e3f0),
    (0x08E3F6, i_08e3f6),
    (0x08E3FA, i_08e3fa),
    (0x08E3FE, i_08e3fe),
    (0x08E404, i_08e404),
    (0x08E406, i_08e406),
    (0x08E40C, i_08e40c),
    (0x08E40E, i_08e40e),
    (0x08E412, i_08e412),
    (0x08E418, i_08e418),
    (0x08E41C, i_08e41c),
    (0x08E420, i_08e420),
    (0x08E424, i_08e424),
    (0x08E42A, i_08e42a),
    (0x08E430, i_08e430),
    (0x08E434, i_08e434),
    (0x08E436, i_08e436),
    (0x08E43A, i_08e43a),
    (0x08E440, i_08e440),
    (0x08E444, i_08e444),
    (0x08E448, i_08e448),
    (0x08E44C, i_08e44c),
    (0x08E452, i_08e452),
    (0x08E456, i_08e456),
    (0x08E45A, i_08e45a),
    (0x08E45C, i_08e45c),
    (0x08E462, i_08e462),
    (0x08E466, i_08e466),
    (0x08E46C, i_08e46c),
    (0x08E470, i_08e470),
    (0x08E474, i_08e474),
    (0x08E476, i_08e476),
    (0x08E47A, i_08e47a),
    (0x08E480, i_08e480),
    (0x08E484, i_08e484),
    (0x08E488, i_08e488),
    (0x08E48A, i_08e48a),
    (0x08E48E, i_08e48e),
    (0x08E492, i_08e492),
    (0x08E498, i_08e498),
    (0x08E49A, i_08e49a),
    (0x08E4A0, i_08e4a0),
    (0x08E4A2, i_08e4a2),
    (0x08E4A8, i_08e4a8),
    (0x08E4AE, i_08e4ae),
    (0x08E4B2, i_08e4b2),
    (0x08E4B4, i_08e4b4),
    (0x08E4BA, i_08e4ba),
    (0x08E4C0, i_08e4c0),
    (0x08E4C4, i_08e4c4),
    (0x08E4C8, i_08e4c8),
    (0x08E4CE, i_08e4ce),
    (0x08E4D2, i_08e4d2),
    (0x08E4D4, i_08e4d4),
    (0x08E4D8, i_08e4d8),
    (0x08E4DC, i_08e4dc),
    (0x08E4E2, i_08e4e2),
    (0x08E4E4, i_08e4e4),
    (0x08E4E6, i_08e4e6),
    (0x08E4EC, i_08e4ec),
    (0x08E4F0, i_08e4f0),
    (0x08E4F4, i_08e4f4),
    (0x08E4F8, i_08e4f8),
    (0x08E4FE, i_08e4fe),
    (0x08E504, i_08e504),
    (0x08E50A, i_08e50a),
    (0x08E50E, i_08e50e),
    (0x08E514, i_08e514),
    (0x08E518, i_08e518),
    (0x08E51E, i_08e51e),
    (0x08E524, i_08e524),
    (0x08E528, i_08e528),
    (0x08E52A, i_08e52a),
    (0x08E530, i_08e530),
    (0x08E534, i_08e534),
    (0x08E53A, i_08e53a),
    (0x08E540, i_08e540),
    (0x08E544, i_08e544),
    (0x08E548, i_08e548),
    (0x08E54A, i_08e54a),
    (0x08E54C, i_08e54c),
    (0x08E550, i_08e550),
    (0x08E556, i_08e556),
    (0x08E55A, i_08e55a),
    (0x08E55E, i_08e55e),
    (0x08E560, i_08e560),
    (0x08E564, i_08e564),
    (0x08E566, i_08e566),
    (0x08E56C, i_08e56c),
    (0x08E572, i_08e572),
    (0x08E576, i_08e576),
    (0x08E57C, i_08e57c),
    (0x08E580, i_08e580),
    (0x08E586, i_08e586),
    (0x08E58C, i_08e58c),
    (0x08E590, i_08e590),
    (0x08E592, i_08e592),
    (0x08E598, i_08e598),
    (0x08E59C, i_08e59c),
    (0x08E5A2, i_08e5a2),
    (0x08E5A8, i_08e5a8),
    (0x08E5AC, i_08e5ac),
    (0x08E5B0, i_08e5b0),
    (0x08E5B2, i_08e5b2),
    (0x08E5B4, i_08e5b4),
    (0x08E5B8, i_08e5b8),
    (0x08E5BE, i_08e5be),
    (0x08E5C2, i_08e5c2),
    (0x08E5C6, i_08e5c6),
    (0x08E5C8, i_08e5c8),
    (0x08E5CC, i_08e5cc),
    (0x08E5CE, i_08e5ce),
    (0x08E5D2, i_08e5d2),
    (0x08E5D6, i_08e5d6),
    (0x08E5DA, i_08e5da),
    (0x08E5E0, i_08e5e0),
    (0x08E5E6, i_08e5e6),
    (0x08E5EA, i_08e5ea),
    (0x08E5F0, i_08e5f0),
    (0x08E5F6, i_08e5f6),
    (0x08E5FC, i_08e5fc),
    (0x08E602, i_08e602),
    (0x08E608, i_08e608),
    (0x08E60C, i_08e60c),
    (0x08E612, i_08e612),
    (0x08E616, i_08e616),
    (0x08E61A, i_08e61a),
    (0x08E61E, i_08e61e),
    (0x08E620, i_08e620),
    (0x08E622, i_08e622),
    (0x08E626, i_08e626),
    (0x08E62C, i_08e62c),
    (0x08E630, i_08e630),
    (0x08E634, i_08e634),
    (0x08E636, i_08e636),
    (0x08E63C, i_08e63c),
    (0x08E640, i_08e640),
    (0x08E644, i_08e644),
    (0x08E646, i_08e646),
    (0x08E64A, i_08e64a),
    (0x08E650, i_08e650),
    (0x08E654, i_08e654),
    (0x08E65A, i_08e65a),
    (0x08E65C, i_08e65c),
    (0x08E660, i_08e660),
    (0x08E666, i_08e666),
    (0x08E66A, i_08e66a),
    (0x08E66C, i_08e66c),
    (0x08E670, i_08e670),
    (0x08E676, i_08e676),
    (0x08E67C, i_08e67c),
    (0x08E682, i_08e682),
    (0x08E688, i_08e688),
    (0x08E68C, i_08e68c),
    (0x08E68E, i_08e68e),
    (0x08E692, i_08e692),
    (0x08E698, i_08e698),
    (0x08E69C, i_08e69c),
    (0x08E6A0, i_08e6a0),
    (0x08E6A2, i_08e6a2),
    (0x08E6A6, i_08e6a6),
    (0x08E6AC, i_08e6ac),
    (0x08E6B0, i_08e6b0),
    (0x08E6B6, i_08e6b6),
    (0x08E6B8, i_08e6b8),
    (0x08E6BE, i_08e6be),
    (0x08E6C4, i_08e6c4),
    (0x08E6C8, i_08e6c8),
    (0x08E6CA, i_08e6ca),
    (0x08E6CE, i_08e6ce),
    (0x08E6D4, i_08e6d4),
    (0x08E6D8, i_08e6d8),
    (0x08E6DE, i_08e6de),
    (0x08E6E0, i_08e6e0),
    (0x08E6E4, i_08e6e4),
    (0x08E6E8, i_08e6e8),
    (0x08E6EC, i_08e6ec),
    (0x08E6F0, i_08e6f0),
    (0x08E6F6, i_08e6f6),
    (0x08E6FC, i_08e6fc),
    (0x08E702, i_08e702),
    (0x08E708, i_08e708),
    (0x08E70E, i_08e70e),
    (0x08E712, i_08e712),
    (0x08E714, i_08e714),
    (0x08E716, i_08e716),
    (0x08E71A, i_08e71a),
    (0x08E720, i_08e720),
    (0x08E724, i_08e724),
    (0x08E728, i_08e728),
    (0x08E72E, i_08e72e),
    (0x08E730, i_08e730),
    (0x08E736, i_08e736),
    (0x08E738, i_08e738),
    (0x08E73E, i_08e73e),
    (0x08E742, i_08e742),
    (0x08E746, i_08e746),
    (0x08E74A, i_08e74a),
    (0x08E750, i_08e750),
    (0x08E754, i_08e754),
    (0x08E758, i_08e758),
    (0x08E75E, i_08e75e),
    (0x08E764, i_08e764),
    (0x08E76A, i_08e76a),
    (0x08E770, i_08e770),
    (0x08E774, i_08e774),
    (0x08E77A, i_08e77a),
    (0x08E780, i_08e780),
    (0x08E784, i_08e784),
    (0x08E78A, i_08e78a),
    (0x08E78E, i_08e78e),
    (0x08E792, i_08e792),
    (0x08E798, i_08e798),
    (0x08E79E, i_08e79e),
    (0x08E7A0, i_08e7a0),
    (0x08E7A4, i_08e7a4),
    (0x08E7A6, i_08e7a6),
    (0x08E7AC, i_08e7ac),
    (0x08E7B2, i_08e7b2),
    (0x08E7B8, i_08e7b8),
    (0x08E7BC, i_08e7bc),
    (0x08E7C0, i_08e7c0),
    (0x08E7C2, i_08e7c2),
    (0x08E7C6, i_08e7c6),
    (0x08E7CA, i_08e7ca),
    (0x08E7D0, i_08e7d0),
    (0x08E7D8, i_08e7d8),
    (0x08E7DC, i_08e7dc),
    (0x08E7DE, i_08e7de),
    (0x08E7E4, i_08e7e4),
    (0x08E7EA, i_08e7ea),
    (0x08E7EE, i_08e7ee),
    (0x08E7F0, i_08e7f0),
    (0x08E7F6, i_08e7f6),
    (0x08E7FC, i_08e7fc),
    (0x08E800, i_08e800),
    (0x08E804, i_08e804),
    (0x08E806, i_08e806),
    (0x08E80A, i_08e80a),
    (0x08E80E, i_08e80e),
    (0x08E814, i_08e814),
    (0x08E81C, i_08e81c),
    (0x08E820, i_08e820),
    (0x08E822, i_08e822),
    (0x08E828, i_08e828),
    (0x08E82E, i_08e82e),
    (0x08E832, i_08e832),
    (0x08E834, i_08e834),
    (0x08E83A, i_08e83a),
    (0x08E840, i_08e840),
    (0x08E846, i_08e846),
    (0x08E84A, i_08e84a),
    (0x08E84E, i_08e84e),
    (0x08E850, i_08e850),
    (0x08E854, i_08e854),
    (0x08E858, i_08e858),
    (0x08E85E, i_08e85e),
    (0x08E866, i_08e866),
    (0x08E86A, i_08e86a),
    (0x08E86C, i_08e86c),
    (0x08E872, i_08e872),
    (0x08E878, i_08e878),
    (0x08E87C, i_08e87c),
    (0x08E87E, i_08e87e),
    (0x08E884, i_08e884),
    (0x08E88A, i_08e88a),
    (0x08E88E, i_08e88e),
    (0x08E892, i_08e892),
    (0x08E894, i_08e894),
    (0x08E898, i_08e898),
    (0x08E89C, i_08e89c),
    (0x08E8A2, i_08e8a2),
    (0x08E8AA, i_08e8aa),
    (0x08E8AE, i_08e8ae),
    (0x08E8B0, i_08e8b0),
    (0x08E8B6, i_08e8b6),
    (0x08E8BC, i_08e8bc),
    (0x08E8C0, i_08e8c0),
    (0x08E8C2, i_08e8c2),
    (0x08E8C8, i_08e8c8),
    (0x08E8CE, i_08e8ce),
    (0x08E8D4, i_08e8d4),
    (0x08E8D8, i_08e8d8),
    (0x08E8DC, i_08e8dc),
    (0x08E8DE, i_08e8de),
    (0x08E8E2, i_08e8e2),
    (0x08E8E6, i_08e8e6),
    (0x08E8EC, i_08e8ec),
    (0x08E8F4, i_08e8f4),
    (0x08E8F8, i_08e8f8),
    (0x08E8FA, i_08e8fa),
    (0x08E900, i_08e900),
    (0x08E906, i_08e906),
    (0x08E90A, i_08e90a),
    (0x08E90C, i_08e90c),
    (0x08E912, i_08e912),
    (0x08E918, i_08e918),
    (0x08E91C, i_08e91c),
    (0x08E920, i_08e920),
    (0x08E922, i_08e922),
    (0x08E926, i_08e926),
    (0x08E92A, i_08e92a),
    (0x08E930, i_08e930),
    (0x08E938, i_08e938),
    (0x08E93C, i_08e93c),
    (0x08E93E, i_08e93e),
    (0x08E944, i_08e944),
    (0x08E94A, i_08e94a),
    (0x08E94E, i_08e94e),
    (0x08E950, i_08e950),
    (0x08E956, i_08e956),
    (0x08E95C, i_08e95c),
    (0x08E962, i_08e962),
    (0x08E966, i_08e966),
    (0x08E96A, i_08e96a),
    (0x08E96C, i_08e96c),
    (0x08E970, i_08e970),
    (0x08E974, i_08e974),
    (0x08E97A, i_08e97a),
    (0x08E982, i_08e982),
    (0x08E986, i_08e986),
    (0x08E988, i_08e988),
    (0x08E98C, i_08e98c),
    (0x08E990, i_08e990),
    (0x08E996, i_08e996),
    (0x08E99C, i_08e99c),
    (0x08E9A0, i_08e9a0),
    (0x08E9A2, i_08e9a2),
    (0x08E9A8, i_08e9a8),
    (0x08E9AE, i_08e9ae),
    (0x08E9B2, i_08e9b2),
    (0x08E9B6, i_08e9b6),
    (0x08E9B8, i_08e9b8),
    (0x08E9BC, i_08e9bc),
    (0x08E9C0, i_08e9c0),
    (0x08E9C6, i_08e9c6),
    (0x08E9CE, i_08e9ce),
    (0x08E9D2, i_08e9d2),
    (0x08E9D4, i_08e9d4),
    (0x08E9DA, i_08e9da),
    (0x08E9E0, i_08e9e0),
    (0x08E9E4, i_08e9e4),
    (0x08E9E8, i_08e9e8),
    (0x08E9EE, i_08e9ee),
    (0x08E9F6, i_08e9f6),
    (0x08E9FA, i_08e9fa),
    (0x08E9FC, i_08e9fc),
    (0x08EA00, i_08ea00),
    (0x08EA04, i_08ea04),
    (0x08EA0A, i_08ea0a),
    (0x08EA10, i_08ea10),
    (0x08EA14, i_08ea14),
    (0x08EA16, i_08ea16),
    (0x08EA1C, i_08ea1c),
    (0x08EA20, i_08ea20),
    (0x08EA26, i_08ea26),
    (0x08EA2C, i_08ea2c),
    (0x08EA32, i_08ea32),
    (0x08EA38, i_08ea38),
    (0x08EA3C, i_08ea3c),
    (0x08EA40, i_08ea40),
    (0x08EA46, i_08ea46),
    (0x08EA4E, i_08ea4e),
    (0x08EA50, i_08ea50),
    (0x08EA56, i_08ea56),
    (0x08EA5A, i_08ea5a),
    (0x08EA5C, i_08ea5c),
    (0x08EA60, i_08ea60),
    (0x08EA68, i_08ea68),
    (0x08EA6C, i_08ea6c),
    (0x08EA70, i_08ea70),
    (0x08EA72, i_08ea72),
    (0x08EA74, i_08ea74),
    (0x08EA78, i_08ea78),
    (0x08EA7E, i_08ea7e),
    (0x08EA82, i_08ea82),
    (0x08EA88, i_08ea88),
    (0x08EA8E, i_08ea8e),
    (0x08EA92, i_08ea92),
    (0x08EA96, i_08ea96),
    (0x08EA9C, i_08ea9c),
    (0x08EAA0, i_08eaa0),
    (0x08EAA4, i_08eaa4),
    (0x08EAAA, i_08eaaa),
    (0x08EAB0, i_08eab0),
    (0x08EAB2, i_08eab2),
    (0x08EAB6, i_08eab6),
    (0x08EAB8, i_08eab8),
    (0x08EABE, i_08eabe),
    (0x08EAC4, i_08eac4),
    (0x08EAC8, i_08eac8),
    (0x08EACC, i_08eacc),
    (0x08EACE, i_08eace),
    (0x08EAD2, i_08ead2),
    (0x08EAD6, i_08ead6),
    (0x08EADA, i_08eada),
    (0x08EADE, i_08eade),
    (0x08EAE4, i_08eae4),
    (0x08EAEC, i_08eaec),
    (0x08EAEE, i_08eaee),
    (0x08EAF4, i_08eaf4),
    (0x08EAFA, i_08eafa),
    (0x08EAFE, i_08eafe),
    (0x08EB00, i_08eb00),
    (0x08EB02, i_08eb02),
    (0x08EB06, i_08eb06),
    (0x08EB0A, i_08eb0a),
    (0x08EB0C, i_08eb0c),
    (0x08EB12, i_08eb12),
    (0x08EB16, i_08eb16),
    (0x08EB1C, i_08eb1c),
    (0x08EB20, i_08eb20),
    (0x08EB26, i_08eb26),
    (0x08EB2C, i_08eb2c),
    (0x08EB30, i_08eb30),
    (0x08EB34, i_08eb34),
    (0x08EB36, i_08eb36),
    (0x08EB3A, i_08eb3a),
    (0x08EB3E, i_08eb3e),
    (0x08EB42, i_08eb42),
    (0x08EB46, i_08eb46),
    (0x08EB4C, i_08eb4c),
    (0x08EB54, i_08eb54),
    (0x08EB56, i_08eb56),
    (0x08EB5A, i_08eb5a),
    (0x08EB60, i_08eb60),
    (0x08EB66, i_08eb66),
    (0x08EB6C, i_08eb6c),
    (0x08EB70, i_08eb70),
    (0x08EB74, i_08eb74),
    (0x08EB78, i_08eb78),
    (0x08EB7C, i_08eb7c),
    (0x08EB80, i_08eb80),
    (0x08EB84, i_08eb84),
    (0x08EB88, i_08eb88),
    (0x08EB8E, i_08eb8e),
    (0x08EB92, i_08eb92),
    (0x08EB96, i_08eb96),
    (0x08EB9A, i_08eb9a),
    (0x08EBA0, i_08eba0),
    (0x08EBA4, i_08eba4),
    (0x08EBA8, i_08eba8),
    (0x08EBAE, i_08ebae),
    (0x08EBB0, i_08ebb0),
    (0x08EBB4, i_08ebb4),
    (0x08EBB8, i_08ebb8),
    (0x08EBBE, i_08ebbe),
    (0x08EBC2, i_08ebc2),
    (0x08EBC6, i_08ebc6),
    (0x08EBCC, i_08ebcc),
    (0x08EBD0, i_08ebd0),
    (0x08EBD4, i_08ebd4),
    (0x08EBD8, i_08ebd8),
    (0x08EBDE, i_08ebde),
    (0x08EBE2, i_08ebe2),
    (0x08EBE8, i_08ebe8),
    (0x08EBEC, i_08ebec),
    (0x08EBF0, i_08ebf0),
    (0x08EBF4, i_08ebf4),
    (0x08EBF6, i_08ebf6),
    (0x08EBFA, i_08ebfa),
    (0x08EC00, i_08ec00),
    (0x08EC04, i_08ec04),
    (0x08EC0A, i_08ec0a),
    (0x08EC0E, i_08ec0e),
    (0x08EC12, i_08ec12),
    (0x08EC18, i_08ec18),
    (0x08EC1C, i_08ec1c),
    (0x08EC20, i_08ec20),
    (0x08EC24, i_08ec24),
    (0x08EC26, i_08ec26),
    (0x08EC2A, i_08ec2a),
    (0x08EC2E, i_08ec2e),
    (0x08EC30, i_08ec30),
    (0x08EC34, i_08ec34),
    (0x08EC3A, i_08ec3a),
    (0x08EC3E, i_08ec3e),
    (0x08EC42, i_08ec42),
    (0x08EC44, i_08ec44),
    (0x08EC4A, i_08ec4a),
    (0x08EC4E, i_08ec4e),
    (0x08EC50, i_08ec50),
    (0x08EC54, i_08ec54),
    (0x08EC58, i_08ec58),
    (0x08EC5C, i_08ec5c),
    (0x08EC60, i_08ec60),
    (0x08EC64, i_08ec64),
    (0x08EC68, i_08ec68),
    (0x08EC6C, i_08ec6c),
    (0x08EC72, i_08ec72),
    (0x08EC76, i_08ec76),
    (0x08EC7A, i_08ec7a),
    (0x08EC7E, i_08ec7e),
    (0x08EC84, i_08ec84),
    (0x08EC88, i_08ec88),
    (0x08EC8C, i_08ec8c),
    (0x08EC92, i_08ec92),
    (0x08EC96, i_08ec96),
    (0x08EC9A, i_08ec9a),
    (0x08EC9E, i_08ec9e),
    (0x08ECA4, i_08eca4),
    (0x08ECAA, i_08ecaa),
    (0x08ECAE, i_08ecae),
    (0x08ECB0, i_08ecb0),
    (0x08ECB4, i_08ecb4),
    (0x08ECBA, i_08ecba),
    (0x08ECBE, i_08ecbe),
    (0x08ECC4, i_08ecc4),
    (0x08ECC8, i_08ecc8),
    (0x08ECCC, i_08eccc),
    (0x08ECCE, i_08ecce),
    (0x08ECD4, i_08ecd4),
    (0x08ECD8, i_08ecd8),
    (0x08ECDA, i_08ecda),
    (0x08ECDE, i_08ecde),
    (0x08ECE2, i_08ece2),
    (0x08ECE6, i_08ece6),
    (0x08ECEC, i_08ecec),
    (0x08ECF2, i_08ecf2),
    (0x08ECF8, i_08ecf8),
    (0x08ECFC, i_08ecfc),
    (0x08ED02, i_08ed02),
    (0x08ED06, i_08ed06),
    (0x08ED0A, i_08ed0a),
    (0x08ED10, i_08ed10),
    (0x08ED12, i_08ed12),
    (0x08ED16, i_08ed16),
    (0x08ED1A, i_08ed1a),
    (0x08ED20, i_08ed20),
    (0x08ED24, i_08ed24),
    (0x08ED28, i_08ed28),
    (0x08ED2C, i_08ed2c),
    (0x08ED32, i_08ed32),
    (0x08ED36, i_08ed36),
    (0x08ED3A, i_08ed3a),
    (0x08ED40, i_08ed40),
    (0x08ED44, i_08ed44),
    (0x08ED48, i_08ed48),
    (0x08ED4C, i_08ed4c),
    (0x08ED52, i_08ed52),
    (0x08ED56, i_08ed56),
    (0x08ED58, i_08ed58),
    (0x08ED5C, i_08ed5c),
    (0x08ED62, i_08ed62),
    (0x08ED66, i_08ed66),
    (0x08ED6C, i_08ed6c),
    (0x08ED70, i_08ed70),
    (0x08ED74, i_08ed74),
    (0x08ED76, i_08ed76),
    (0x08ED7C, i_08ed7c),
    (0x08ED80, i_08ed80),
    (0x08ED82, i_08ed82),
    (0x08ED88, i_08ed88),
    (0x08ED8C, i_08ed8c),
    (0x08ED90, i_08ed90),
    (0x08ED96, i_08ed96),
    (0x08ED9C, i_08ed9c),
    (0x08EDA2, i_08eda2),
    (0x08EDA6, i_08eda6),
    (0x08EDA8, i_08eda8),
    (0x08EDAE, i_08edae),
    (0x08EDB2, i_08edb2),
    (0x08EDB6, i_08edb6),
    (0x08EDBC, i_08edbc),
    (0x08EDBE, i_08edbe),
    (0x08EDC4, i_08edc4),
    (0x08EDC6, i_08edc6),
    (0x08EDCC, i_08edcc),
    (0x08EDD2, i_08edd2),
    (0x08EDD6, i_08edd6),
    (0x08EDDC, i_08eddc),
    (0x08EDE2, i_08ede2),
    (0x08EDE8, i_08ede8),
    (0x08EDEC, i_08edec),
    (0x08EDF2, i_08edf2),
    (0x08EDF6, i_08edf6),
    (0x08EDF8, i_08edf8),
    (0x08EDFE, i_08edfe),
    (0x08EE02, i_08ee02),
    (0x08EE06, i_08ee06),
    (0x08EE0C, i_08ee0c),
    (0x08EE0E, i_08ee0e),
    (0x08EE14, i_08ee14),
    (0x08EE16, i_08ee16),
    (0x08EE1A, i_08ee1a),
    (0x08EE1E, i_08ee1e),
    (0x08EE20, i_08ee20),
    (0x08EE24, i_08ee24),
    (0x08EE26, i_08ee26),
    (0x08EE28, i_08ee28),
    (0x08EE2C, i_08ee2c),
    (0x08EE32, i_08ee32),
    (0x08EE36, i_08ee36),
    (0x08EE3C, i_08ee3c),
    (0x08EE42, i_08ee42),
    (0x08EE48, i_08ee48),
    (0x08EE4C, i_08ee4c),
    (0x08EE4E, i_08ee4e),
    (0x08EE52, i_08ee52),
    (0x08EE58, i_08ee58),
    (0x08EE5C, i_08ee5c),
    (0x08EE62, i_08ee62),
    (0x08EE68, i_08ee68),
    (0x08EE6C, i_08ee6c),
    (0x08EE6E, i_08ee6e),
    (0x08EE72, i_08ee72),
    (0x08EE78, i_08ee78),
    (0x08EE7A, i_08ee7a),
    (0x08EE80, i_08ee80),
    (0x08EE84, i_08ee84),
    (0x08EE8A, i_08ee8a),
    (0x08EE8E, i_08ee8e),
    (0x08EE94, i_08ee94),
    (0x08EE96, i_08ee96),
    (0x08EE9C, i_08ee9c),
    (0x08EEA2, i_08eea2),
    (0x08EEA8, i_08eea8),
    (0x08EEAC, i_08eeac),
    (0x08EEB2, i_08eeb2),
    (0x08EEB8, i_08eeb8),
    (0x08EEBE, i_08eebe),
    (0x08EEC2, i_08eec2),
    (0x08EEC8, i_08eec8),
    (0x08EECE, i_08eece),
    (0x08EED4, i_08eed4),
    (0x08EED8, i_08eed8),
    (0x08EEDE, i_08eede),
    (0x08EEE0, i_08eee0),
    (0x08EEE4, i_08eee4),
    (0x08EEE8, i_08eee8),
    (0x08EEEE, i_08eeee),
    (0x08EEF2, i_08eef2),
    (0x08EEF6, i_08eef6),
    (0x08EEFA, i_08eefa),
    (0x08EEFE, i_08eefe),
    (0x08EF04, i_08ef04),
    (0x08EF08, i_08ef08),
    (0x08EF0C, i_08ef0c),
    (0x08EF12, i_08ef12),
    (0x08EF16, i_08ef16),
    (0x08EF18, i_08ef18),
    (0x08EF1C, i_08ef1c),
    (0x08EF20, i_08ef20),
    (0x08EF24, i_08ef24),
    (0x08EF28, i_08ef28),
    (0x08EF2C, i_08ef2c),
    (0x08EF2E, i_08ef2e),
    (0x08EF34, i_08ef34),
    (0x08EF38, i_08ef38),
    (0x08EF3E, i_08ef3e),
    (0x08EF40, i_08ef40),
    (0x08EF46, i_08ef46),
    (0x08EF4A, i_08ef4a),
    (0x08EF50, i_08ef50),
    (0x08EF56, i_08ef56),
    (0x08EF5A, i_08ef5a),
    (0x08EF60, i_08ef60),
    (0x08EF64, i_08ef64),
    (0x08EF6A, i_08ef6a),
    (0x08EF70, i_08ef70),
    (0x08EF74, i_08ef74),
    (0x08EF7A, i_08ef7a),
    (0x08EF7E, i_08ef7e),
    (0x08EF84, i_08ef84),
    (0x08EF8A, i_08ef8a),
    (0x08EF8E, i_08ef8e),
    (0x08EF92, i_08ef92),
    (0x08EF94, i_08ef94),
    (0x08EF9A, i_08ef9a),
    (0x08EF9E, i_08ef9e),
    (0x08EFA4, i_08efa4),
    (0x08EFA8, i_08efa8),
    (0x08EFAE, i_08efae),
    (0x08EFB0, i_08efb0),
    (0x08EFB6, i_08efb6),
    (0x08EFBC, i_08efbc),
    (0x08EFC0, i_08efc0),
    (0x08EFC6, i_08efc6),
    (0x08EFCC, i_08efcc),
    (0x08EFCE, i_08efce),
    (0x08EFD4, i_08efd4),
    (0x08EFD8, i_08efd8),
    (0x08EFDC, i_08efdc),
    (0x08EFDE, i_08efde),
    (0x08EFE2, i_08efe2),
    (0x08EFE8, i_08efe8),
    (0x08EFEC, i_08efec),
    (0x08EFF0, i_08eff0),
    (0x08EFF2, i_08eff2),
    (0x08EFF6, i_08eff6),
    (0x08EFFA, i_08effa),
    (0x08EFFC, i_08effc),
    (0x08F000, i_08f000),
    (0x08F002, i_08f002),
    (0x08F008, i_08f008),
    (0x08F00E, i_08f00e),
    (0x08F010, i_08f010),
    (0x08F012, i_08f012),
    (0x08F016, i_08f016),
    (0x08F018, i_08f018),
    (0x08F01C, i_08f01c),
    (0x08F022, i_08f022),
    (0x08F026, i_08f026),
    (0x08F02A, i_08f02a),
    (0x08F02C, i_08f02c),
    (0x08F02E, i_08f02e),
    (0x08F032, i_08f032),
    (0x08F03A, i_08f03a),
    (0x08F03E, i_08f03e),
    (0x08F040, i_08f040),
    (0x08F046, i_08f046),
    (0x08F04A, i_08f04a),
    (0x08F04E, i_08f04e),
    (0x08F050, i_08f050),
    (0x08F056, i_08f056),
    (0x08F05A, i_08f05a),
    (0x08F05E, i_08f05e),
    (0x08F062, i_08f062),
    (0x08F064, i_08f064),
    (0x08F068, i_08f068),
    (0x08F06C, i_08f06c),
    (0x08F06E, i_08f06e),
    (0x08F070, i_08f070),
    (0x08F072, i_08f072),
    (0x08F076, i_08f076),
    (0x08F07A, i_08f07a),
    (0x08F07C, i_08f07c),
    (0x08F082, i_08f082),
    (0x08F084, i_08f084),
    (0x08F08A, i_08f08a),
    (0x08F08E, i_08f08e),
    (0x08F092, i_08f092),
    (0x08F098, i_08f098),
    (0x08F09E, i_08f09e),
    (0x08F0A2, i_08f0a2),
    (0x08F0A6, i_08f0a6),
    (0x08F0AC, i_08f0ac),
    (0x08F0B2, i_08f0b2),
    (0x08F0B6, i_08f0b6),
    (0x08F0BA, i_08f0ba),
    (0x08F0C0, i_08f0c0),
    (0x08F0C4, i_08f0c4),
    (0x08F0C8, i_08f0c8),
    (0x08F0CE, i_08f0ce),
    (0x08F0D0, i_08f0d0),
    (0x08F0D6, i_08f0d6),
    (0x08F0DA, i_08f0da),
    (0x08F0DE, i_08f0de),
    (0x08F0E4, i_08f0e4),
    (0x08F0EA, i_08f0ea),
    (0x08F0EE, i_08f0ee),
    (0x08F0F2, i_08f0f2),
    (0x08F0F8, i_08f0f8),
    (0x08F0FC, i_08f0fc),
    (0x08F100, i_08f100),
    (0x08F106, i_08f106),
    (0x08F108, i_08f108),
    (0x08F10E, i_08f10e),
    (0x08F114, i_08f114),
    (0x08F11A, i_08f11a),
    (0x08F11C, i_08f11c),
    (0x08F120, i_08f120),
    (0x08F124, i_08f124),
    (0x08F128, i_08f128),
    (0x08F12C, i_08f12c),
    (0x08F130, i_08f130),
    (0x08F132, i_08f132),
    (0x08F136, i_08f136),
    (0x08F138, i_08f138),
    (0x08F13A, i_08f13a),
    (0x08F13E, i_08f13e),
    (0x08F142, i_08f142),
    (0x08F146, i_08f146),
    (0x08F148, i_08f148),
    (0x08F14C, i_08f14c),
    (0x08F152, i_08f152),
    (0x08F156, i_08f156),
    (0x08F158, i_08f158),
    (0x08F15E, i_08f15e),
    (0x08F160, i_08f160),
    (0x08F166, i_08f166),
    (0x08F168, i_08f168),
    (0x08F16E, i_08f16e),
    (0x08F170, i_08f170),
    (0x08F172, i_08f172),
    (0x08F178, i_08f178),
    (0x08F17A, i_08f17a),
    (0x08F180, i_08f180),
    (0x08F182, i_08f182),
    (0x08F188, i_08f188),
    (0x08F18A, i_08f18a),
    (0x08F18C, i_08f18c),
    (0x08F190, i_08f190),
    (0x08F196, i_08f196),
    (0x08F19A, i_08f19a),
    (0x08F19C, i_08f19c),
    (0x08F1A0, i_08f1a0),
    (0x08F1A2, i_08f1a2),
    (0x08F1A4, i_08f1a4),
    (0x08F1A8, i_08f1a8),
    (0x08F1AC, i_08f1ac),
    (0x08F1B0, i_08f1b0),
    (0x08F1B2, i_08f1b2),
    (0x08F1B8, i_08f1b8),
    (0x08F1BC, i_08f1bc),
    (0x08F1BE, i_08f1be),
    (0x08F1C2, i_08f1c2),
    (0x08F1C4, i_08f1c4),
    (0x08F1C6, i_08f1c6),
    (0x08F1CA, i_08f1ca),
    (0x08F1CE, i_08f1ce),
    (0x08F1D0, i_08f1d0),
    (0x08F1D4, i_08f1d4),
    (0x08F1DA, i_08f1da),
    (0x08F1DE, i_08f1de),
    (0x08F1E4, i_08f1e4),
    (0x08F1EC, i_08f1ec),
    (0x08F1F2, i_08f1f2),
    (0x08F1F6, i_08f1f6),
    (0x08F1FA, i_08f1fa),
    (0x08F1FE, i_08f1fe),
    (0x08F204, i_08f204),
    (0x08F20A, i_08f20a),
    (0x08F210, i_08f210),
    (0x08F216, i_08f216),
    (0x08F21A, i_08f21a),
    (0x08F21E, i_08f21e),
    (0x08F220, i_08f220),
    (0x08F224, i_08f224),
    (0x08F228, i_08f228),
    (0x08F22A, i_08f22a),
    (0x08F22E, i_08f22e),
    (0x08F232, i_08f232),
    (0x08F234, i_08f234),
    (0x08F238, i_08f238),
    (0x08F23C, i_08f23c),
    (0x08F242, i_08f242),
    (0x08F246, i_08f246),
    (0x08F248, i_08f248),
    (0x08F24E, i_08f24e),
    (0x08F250, i_08f250),
    (0x08F252, i_08f252),
    (0x08F254, i_08f254),
    (0x08F258, i_08f258),
    (0x08F25C, i_08f25c),
    (0x08F260, i_08f260),
    (0x08F266, i_08f266),
    (0x08F26A, i_08f26a),
    (0x08F26E, i_08f26e),
    (0x08F270, i_08f270),
    (0x08F274, i_08f274),
    (0x08F278, i_08f278),
    (0x08F27C, i_08f27c),
    (0x08F282, i_08f282),
    (0x08F286, i_08f286),
    (0x08F28A, i_08f28a),
    (0x08F28C, i_08f28c),
    (0x08F290, i_08f290),
    (0x08F294, i_08f294),
    (0x08F296, i_08f296),
    (0x08F29A, i_08f29a),
    (0x08F29E, i_08f29e),
    (0x08F2A0, i_08f2a0),
    (0x08F2A4, i_08f2a4),
    (0x08F2AA, i_08f2aa),
    (0x08F2AE, i_08f2ae),
];
