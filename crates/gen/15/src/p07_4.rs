// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $078142: move.l a0, $70(a6)
pub fn i_078142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78146;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078146: lea.l $2df256.l, a0
pub fn i_078146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7814C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF256;
    20
}
/// $07814C: move.l a0, $4c(a6)
pub fn i_07814c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78150;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078150: jsr $283ca.l
pub fn i_078150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78156);
    cpu.pc = 0x283CA;
    16
}
/// $078156: bra.w $781aa
pub fn i_078156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7815A;
    cpu.ir = 0x6000;
    cpu.pc = 0x781AA;
    10
}
// ===== Explosion_FireAttack_07815a  ($07815A, 182 B, props/fx_explosions_platforms_crew_076000.s)
/// $07815A: lea.l $2dd594.l, a0
pub fn i_07815a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78160;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD594;
    20
}
/// $078160: jsr $28cd4.l
pub fn i_078160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78166;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78166);
    cpu.pc = 0x28CD4;
    16
}
/// $078166: lea.l $2df206.l, a0
pub fn i_078166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7816C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF206;
    20
}
/// $07816C: move.l a0, $70(a6)
pub fn i_07816c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78170;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078170: lea.l $2df2fa.l, a0
pub fn i_078170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78176;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF2FA;
    20
}
/// $078176: move.l a0, $4c(a6)
pub fn i_078176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7817A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07817A: jsr $283ca.l
pub fn i_07817a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78180;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78180);
    cpu.pc = 0x283CA;
    16
}
/// $078180: bra.w $781aa
pub fn i_078180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78184;
    cpu.ir = 0x6000;
    cpu.pc = 0x781AA;
    10
}
/// $078184: lea.l $2dd37e.l, a0
pub fn i_078184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7818A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD37E;
    20
}
/// $07818A: jsr $28cd4.l
pub fn i_07818a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78190;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78190);
    cpu.pc = 0x28CD4;
    16
}
/// $078190: lea.l $2df210.l, a0
pub fn i_078190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78196;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF210;
    20
}
/// $078196: move.l a0, $70(a6)
pub fn i_078196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7819A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07819A: lea.l $2df39e.l, a0
pub fn i_07819a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF39E;
    20
}
/// $0781A0: move.l a0, $4c(a6)
pub fn i_0781a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781A4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0781A4: jsr $283ca.l
pub fn i_0781a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x781AA);
    cpu.pc = 0x283CA;
    16
}
/// $0781AA: move.w #$4, d1
pub fn i_0781aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0781AE: jsr $236e.l
pub fn i_0781ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x781B4);
    cpu.pc = 0x236E;
    16
}
/// $0781B4: move.b #$ff, $32(a6)
pub fn i_0781b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781BA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0781BA: move.b #$ff, $33(a6)
pub fn i_0781ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781C0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0781C0: jsr $77e00(pc)
pub fn i_0781c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x781C4);
    cpu.pc = 0x77E00;
    16
}
/// $0781C4: movea.l $70(a6), a0
pub fn i_0781c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781C8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0781C8: jsr $5dd56.l
pub fn i_0781c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x781CE);
    cpu.pc = 0x5DD56;
    16
}
/// $0781CE: bcs.w $7820a
pub fn i_0781ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781D2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7820A; return 10; }
    8
}
/// $0781D2: jsr $283ca.l
pub fn i_0781d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x781D8);
    cpu.pc = 0x283CA;
    16
}
/// $0781D8: lea.l $ffff.w, a0
pub fn i_0781d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781DC;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0781DC: move.l a0, $48(a6)
pub fn i_0781dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781E0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0781E0: lea.l $781e6(pc), a1
pub fn i_0781e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x781E6;
    16
}
/// $0781E4: move.l a1, (a6)
pub fn i_0781e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0781E6: jsr $44048.l
pub fn i_0781e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x781EC);
    cpu.pc = 0x44048;
    16
}
/// $0781EC: jsr $283ca.l
pub fn i_0781ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x781F2);
    cpu.pc = 0x283CA;
    16
}
/// $0781F2: jsr $28d70.l
pub fn i_0781f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x781F8);
    cpu.pc = 0x28D70;
    16
}
/// $0781F8: bcs.w $7820a
pub fn i_0781f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x781FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7820A; return 10; }
    8
}
/// $0781FC: movea.l $70(a6), a0
pub fn i_0781fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78200;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078200: jsr $5dd56.l
pub fn i_078200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78206;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78206);
    cpu.pc = 0x5DD56;
    16
}
/// $078206: bcc.w $78210
pub fn i_078206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7820A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78210; return 10; }
    8
}
/// $07820A: jmp $518.l
pub fn i_07820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78210;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_078210  ($078210, 8 B, thunks/jsr_abs_thunks.c)
/// $078210: jsr $283d8.l
pub fn i_078210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78216;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78216);
    cpu.pc = 0x283D8;
    16
}
/// $078216: rts
pub fn i_078216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78218;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Explosion_VarTbl_078218  ($078218, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078218: lea.l $2ddad0.l, a0
pub fn i_078218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7821E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDAD0;
    20
}
/// $07821E: jsr $28cd4.l
pub fn i_07821e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78224);
    cpu.pc = 0x28CD4;
    16
}
/// $078224: lea.l $2df21a.l, a0
pub fn i_078224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7822A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $07822A: move.l a0, $70(a6)
pub fn i_07822a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7822E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07822E: move.w #$a, d1
pub fn i_07822e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78232;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078232: bra.w $78778
pub fn i_078232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78236;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var01_078236  ($078236, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078236: lea.l $2ddb62.l, a0
pub fn i_078236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7823C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDB62;
    20
}
/// $07823C: jsr $28cd4.l
pub fn i_07823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78242);
    cpu.pc = 0x28CD4;
    16
}
/// $078242: lea.l $2df21a.l, a0
pub fn i_078242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78248;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078248: move.l a0, $70(a6)
pub fn i_078248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7824C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07824C: move.w #$a, d1
pub fn i_07824c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78250;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078250: bra.w $78778
pub fn i_078250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78254;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var02_078254  ($078254, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078254: lea.l $2ddbf4.l, a0
pub fn i_078254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7825A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDBF4;
    20
}
/// $07825A: jsr $28cd4.l
pub fn i_07825a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78260;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78260);
    cpu.pc = 0x28CD4;
    16
}
/// $078260: lea.l $2df21a.l, a0
pub fn i_078260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078266: move.l a0, $70(a6)
pub fn i_078266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7826A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07826A: move.w #$a, d1
pub fn i_07826a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7826E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07826E: bra.w $78778
pub fn i_07826e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78272;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var03_078272  ($078272, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078272: lea.l $2ddc86.l, a0
pub fn i_078272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78278;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDC86;
    20
}
/// $078278: jsr $28cd4.l
pub fn i_078278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7827E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7827E);
    cpu.pc = 0x28CD4;
    16
}
/// $07827E: lea.l $2df21a.l, a0
pub fn i_07827e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78284;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078284: move.l a0, $70(a6)
pub fn i_078284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78288;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078288: move.w #$a, d1
pub fn i_078288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7828C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07828C: bra.w $78778
pub fn i_07828c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78290;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var04_078290  ($078290, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078290: lea.l $2ddd18.l, a0
pub fn i_078290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78296;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDD18;
    20
}
/// $078296: jsr $28cd4.l
pub fn i_078296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7829C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7829C);
    cpu.pc = 0x28CD4;
    16
}
/// $07829C: lea.l $2df224.l, a0
pub fn i_07829c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $0782A2: move.l a0, $70(a6)
pub fn i_0782a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782A6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0782A6: move.w #$a, d1
pub fn i_0782a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0782AA: bra.w $78778
pub fn i_0782aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var05_0782ae  ($0782AE, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0782AE: lea.l $2dddaa.l, a0
pub fn i_0782ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDDAA;
    20
}
/// $0782B4: jsr $28cd4.l
pub fn i_0782b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x782BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0782BA: lea.l $2df21a.l, a0
pub fn i_0782ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0782C0: move.l a0, $70(a6)
pub fn i_0782c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782C4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0782C4: move.w #$a, d1
pub fn i_0782c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0782C8: bra.w $78778
pub fn i_0782c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var06_0782cc  ($0782CC, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $0782CC: lea.l $2dde3c.l, a0
pub fn i_0782cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDE3C;
    20
}
/// $0782D2: jsr $28cd4.l
pub fn i_0782d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x782D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0782D8: lea.l $2df21a.l, a0
pub fn i_0782d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0782DE: move.l a0, $70(a6)
pub fn i_0782de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782E2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0782E2: move.w #$a, d1
pub fn i_0782e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782E6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0782E6: bra.w $78778
pub fn i_0782e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $0782EA: lea.l $2ddeb2.l, a0
pub fn i_0782ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDEB2;
    20
}
/// $0782F0: jsr $28cd4.l
pub fn i_0782f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x782F6);
    cpu.pc = 0x28CD4;
    16
}
/// $0782F6: lea.l $2df21a.l, a0
pub fn i_0782f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x782FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0782FC: move.l a0, $70(a6)
pub fn i_0782fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78300;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078300: move.w #$a, d1
pub fn i_078300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78304;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078304: bra.w $78778
pub fn i_078304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78308;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var07_078308  ($078308, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078308: lea.l $2ddf28.l, a0
pub fn i_078308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7830E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDF28;
    20
}
/// $07830E: jsr $28cd4.l
pub fn i_07830e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78314);
    cpu.pc = 0x28CD4;
    16
}
/// $078314: lea.l $2df224.l, a0
pub fn i_078314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7831A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $07831A: move.l a0, $70(a6)
pub fn i_07831a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7831E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07831E: move.w #$a, d1
pub fn i_07831e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78322;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078322: bra.w $78778
pub fn i_078322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78326;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var08_078326  ($078326, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078326: lea.l $2ddf9e.l, a0
pub fn i_078326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7832C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDF9E;
    20
}
/// $07832C: jsr $28cd4.l
pub fn i_07832c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78332);
    cpu.pc = 0x28CD4;
    16
}
/// $078332: lea.l $2df224.l, a0
pub fn i_078332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78338;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078338: move.l a0, $70(a6)
pub fn i_078338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7833C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07833C: move.w #$a, d1
pub fn i_07833c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78340;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078340: bra.w $78778
pub fn i_078340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78344;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var09_078344  ($078344, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $078344: lea.l $2de014.l, a0
pub fn i_078344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7834A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE014;
    20
}
/// $07834A: jsr $28cd4.l
pub fn i_07834a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78350);
    cpu.pc = 0x28CD4;
    16
}
/// $078350: lea.l $2df21a.l, a0
pub fn i_078350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78356;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078356: move.l a0, $70(a6)
pub fn i_078356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7835A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07835A: move.w #$a, d1
pub fn i_07835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7835E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07835E: bra.w $78778
pub fn i_07835e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78362;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $078362: lea.l $2de08a.l, a0
pub fn i_078362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78368;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE08A;
    20
}
/// $078368: jsr $28cd4.l
pub fn i_078368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7836E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7836E);
    cpu.pc = 0x28CD4;
    16
}
/// $07836E: lea.l $2df224.l, a0
pub fn i_07836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78374;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078374: move.l a0, $70(a6)
pub fn i_078374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78378;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078378: move.w #$a, d1
pub fn i_078378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7837C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07837C: bra.w $78778
pub fn i_07837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78380;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var10_078380  ($078380, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $078380: lea.l $2de100.l, a0
pub fn i_078380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78386;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE100;
    20
}
/// $078386: jsr $28cd4.l
pub fn i_078386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7838C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7838C);
    cpu.pc = 0x28CD4;
    16
}
/// $07838C: lea.l $2df224.l, a0
pub fn i_07838c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78392;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078392: move.l a0, $70(a6)
pub fn i_078392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78396;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078396: move.w #$a, d1
pub fn i_078396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7839A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07839A: bra.w $78778
pub fn i_07839a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7839E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $07839E: lea.l $2de176.l, a0
pub fn i_07839e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE176;
    20
}
/// $0783A4: jsr $28cd4.l
pub fn i_0783a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x783AA);
    cpu.pc = 0x28CD4;
    16
}
/// $0783AA: lea.l $2df21a.l, a0
pub fn i_0783aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0783B0: move.l a0, $70(a6)
pub fn i_0783b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783B4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0783B4: move.w #$a, d1
pub fn i_0783b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0783B8: bra.w $78778
pub fn i_0783b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var11_0783bc  ($0783BC, 120 B, props/fx_explosions_platforms_crew_076000.s)
/// $0783BC: lea.l $2de1ec.l, a0
pub fn i_0783bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE1EC;
    20
}
/// $0783C2: jsr $28cd4.l
pub fn i_0783c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x783C8);
    cpu.pc = 0x28CD4;
    16
}
/// $0783C8: lea.l $2df21a.l, a0
pub fn i_0783c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0783CE: move.l a0, $70(a6)
pub fn i_0783ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783D2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0783D2: move.w #$a, d1
pub fn i_0783d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783D6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0783D6: bra.w $78778
pub fn i_0783d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $0783DA: lea.l $2dde3c.l, a0
pub fn i_0783da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDE3C;
    20
}
/// $0783E0: jsr $28cd4.l
pub fn i_0783e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x783E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0783E6: lea.l $2df21a.l, a0
pub fn i_0783e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0783EC: move.l a0, $70(a6)
pub fn i_0783ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783F0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0783F0: move.w #$9e, d1
pub fn i_0783f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0783F4: bra.w $78778
pub fn i_0783f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $0783F8: lea.l $2ddeb2.l, a0
pub fn i_0783f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x783FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDEB2;
    20
}
/// $0783FE: jsr $28cd4.l
pub fn i_0783fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78404;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78404);
    cpu.pc = 0x28CD4;
    16
}
/// $078404: lea.l $2df21a.l, a0
pub fn i_078404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7840A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $07840A: move.l a0, $70(a6)
pub fn i_07840a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7840E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07840E: move.w #$9e, d1
pub fn i_07840e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78412;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078412: bra.w $78778
pub fn i_078412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78416;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $078416: lea.l $2ddf28.l, a0
pub fn i_078416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7841C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDF28;
    20
}
/// $07841C: jsr $28cd4.l
pub fn i_07841c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78422);
    cpu.pc = 0x28CD4;
    16
}
/// $078422: lea.l $2df224.l, a0
pub fn i_078422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78428;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078428: move.l a0, $70(a6)
pub fn i_078428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7842C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07842C: move.w #$9e, d1
pub fn i_07842c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78430;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078430: bra.w $78778
pub fn i_078430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78434;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var12_078434  ($078434, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078434: lea.l $2ddf9e.l, a0
pub fn i_078434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7843A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DDF9E;
    20
}
/// $07843A: jsr $28cd4.l
pub fn i_07843a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78440);
    cpu.pc = 0x28CD4;
    16
}
/// $078440: lea.l $2df224.l, a0
pub fn i_078440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78446;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078446: move.l a0, $70(a6)
pub fn i_078446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7844A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07844A: move.w #$9e, d1
pub fn i_07844a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7844E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07844E: bra.w $78778
pub fn i_07844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78452;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var13_078452  ($078452, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $078452: lea.l $2de014.l, a0
pub fn i_078452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78458;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE014;
    20
}
/// $078458: jsr $28cd4.l
pub fn i_078458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7845E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7845E);
    cpu.pc = 0x28CD4;
    16
}
/// $07845E: lea.l $2df21a.l, a0
pub fn i_07845e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78464;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078464: move.l a0, $70(a6)
pub fn i_078464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78468;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078468: move.w #$9e, d1
pub fn i_078468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7846C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07846C: bra.w $78778
pub fn i_07846c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78470;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $078470: lea.l $2de08a.l, a0
pub fn i_078470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78476;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE08A;
    20
}
/// $078476: jsr $28cd4.l
pub fn i_078476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7847C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7847C);
    cpu.pc = 0x28CD4;
    16
}
/// $07847C: lea.l $2df224.l, a0
pub fn i_07847c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78482;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078482: move.l a0, $70(a6)
pub fn i_078482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78486;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078486: move.w #$9e, d1
pub fn i_078486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7848A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07848A: bra.w $78778
pub fn i_07848a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7848E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var14_07848e  ($07848E, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $07848E: lea.l $2de100.l, a0
pub fn i_07848e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78494;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE100;
    20
}
/// $078494: jsr $28cd4.l
pub fn i_078494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7849A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7849A);
    cpu.pc = 0x28CD4;
    16
}
/// $07849A: lea.l $2df224.l, a0
pub fn i_07849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $0784A0: move.l a0, $70(a6)
pub fn i_0784a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784A4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0784A4: move.w #$9e, d1
pub fn i_0784a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784A8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0784A8: bra.w $78778
pub fn i_0784a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $0784AC: lea.l $2de176.l, a0
pub fn i_0784ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE176;
    20
}
/// $0784B2: jsr $28cd4.l
pub fn i_0784b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x784B8);
    cpu.pc = 0x28CD4;
    16
}
/// $0784B8: lea.l $2df21a.l, a0
pub fn i_0784b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0784BE: move.l a0, $70(a6)
pub fn i_0784be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784C2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0784C2: move.w #$9e, d1
pub fn i_0784c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0784C6: bra.w $78778
pub fn i_0784c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var15_0784ca  ($0784CA, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $0784CA: lea.l $2de1ec.l, a0
pub fn i_0784ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE1EC;
    20
}
/// $0784D0: jsr $28cd4.l
pub fn i_0784d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x784D6);
    cpu.pc = 0x28CD4;
    16
}
/// $0784D6: lea.l $2df21a.l, a0
pub fn i_0784d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0784DC: move.l a0, $70(a6)
pub fn i_0784dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784E0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0784E0: move.w #$9e, d1
pub fn i_0784e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784E4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0784E4: bra.w $78778
pub fn i_0784e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $0784E8: lea.l $2de08a.l, a0
pub fn i_0784e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE08A;
    20
}
/// $0784EE: jsr $28cd4.l
pub fn i_0784ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x784F4);
    cpu.pc = 0x28CD4;
    16
}
/// $0784F4: lea.l $2df224.l, a0
pub fn i_0784f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $0784FA: move.l a0, $70(a6)
pub fn i_0784fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784FE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0784FE: move.w #$ad, d1
pub fn i_0784fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78502;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078502: bra.w $78778
pub fn i_078502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78506;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var16_078506  ($078506, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $078506: lea.l $2de100.l, a0
pub fn i_078506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7850C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE100;
    20
}
/// $07850C: jsr $28cd4.l
pub fn i_07850c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78512);
    cpu.pc = 0x28CD4;
    16
}
/// $078512: lea.l $2df224.l, a0
pub fn i_078512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78518;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078518: move.l a0, $70(a6)
pub fn i_078518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7851C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07851C: move.w #$ad, d1
pub fn i_07851c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78520;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078520: bra.w $78778
pub fn i_078520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78524;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $078524: lea.l $2de176.l, a0
pub fn i_078524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7852A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE176;
    20
}
/// $07852A: jsr $28cd4.l
pub fn i_07852a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78530);
    cpu.pc = 0x28CD4;
    16
}
/// $078530: lea.l $2df21a.l, a0
pub fn i_078530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78536;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078536: move.l a0, $70(a6)
pub fn i_078536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7853A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07853A: move.w #$ad, d1
pub fn i_07853a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7853E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07853E: bra.w $78778
pub fn i_07853e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78542;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var17_078542  ($078542, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078542: lea.l $2de1ec.l, a0
pub fn i_078542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78548;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE1EC;
    20
}
/// $078548: jsr $28cd4.l
pub fn i_078548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7854E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7854E);
    cpu.pc = 0x28CD4;
    16
}
/// $07854E: lea.l $2df21a.l, a0
pub fn i_07854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78554;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078554: move.l a0, $70(a6)
pub fn i_078554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78558;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078558: move.w #$ad, d1
pub fn i_078558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7855C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07855C: bra.w $78778
pub fn i_07855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78560;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var18_078560  ($078560, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078560: lea.l $2de262.l, a0
pub fn i_078560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78566;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE262;
    20
}
/// $078566: jsr $28cd4.l
pub fn i_078566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7856C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7856C);
    cpu.pc = 0x28CD4;
    16
}
/// $07856C: lea.l $2df21a.l, a0
pub fn i_07856c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78572;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078572: move.l a0, $70(a6)
pub fn i_078572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78576;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078576: move.w #$a, d1
pub fn i_078576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7857A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07857A: bra.w $78778
pub fn i_07857a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7857E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var19_07857e  ($07857E, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $07857E: lea.l $2de262.l, a0
pub fn i_07857e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78584;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE262;
    20
}
/// $078584: jsr $28cd4.l
pub fn i_078584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7858A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7858A);
    cpu.pc = 0x28CD4;
    16
}
/// $07858A: lea.l $2df21a.l, a0
pub fn i_07858a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78590;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078590: move.l a0, $70(a6)
pub fn i_078590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78594;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078594: move.w #$72, d1
pub fn i_078594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78598;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078598: bra.w $78778
pub fn i_078598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7859C;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var20_07859c  ($07859C, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $07859C: lea.l $2de2d8.l, a0
pub fn i_07859c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE2D8;
    20
}
/// $0785A2: jsr $28cd4.l
pub fn i_0785a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x785A8);
    cpu.pc = 0x28CD4;
    16
}
/// $0785A8: lea.l $2df21a.l, a0
pub fn i_0785a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0785AE: move.l a0, $70(a6)
pub fn i_0785ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785B2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0785B2: move.w #$a, d1
pub fn i_0785b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0785B6: bra.w $78778
pub fn i_0785b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var21_0785ba  ($0785BA, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0785BA: lea.l $2de2d8.l, a0
pub fn i_0785ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE2D8;
    20
}
/// $0785C0: jsr $28cd4.l
pub fn i_0785c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x785C6);
    cpu.pc = 0x28CD4;
    16
}
/// $0785C6: lea.l $2df21a.l, a0
pub fn i_0785c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0785CC: move.l a0, $70(a6)
pub fn i_0785cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785D0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0785D0: move.w #$72, d1
pub fn i_0785d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0785D4: bra.w $78778
pub fn i_0785d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var22_0785d8  ($0785D8, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0785D8: lea.l $2de2d8.l, a0
pub fn i_0785d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE2D8;
    20
}
/// $0785DE: jsr $28cd4.l
pub fn i_0785de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x785E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0785E4: lea.l $2df21a.l, a0
pub fn i_0785e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0785EA: move.l a0, $70(a6)
pub fn i_0785ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785EE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0785EE: move.w #$88, d1
pub fn i_0785ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0785F2: bra.w $78778
pub fn i_0785f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var23_0785f6  ($0785F6, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0785F6: lea.l $2de34e.l, a0
pub fn i_0785f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE34E;
    20
}
/// $0785FC: jsr $28cd4.l
pub fn i_0785fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78602);
    cpu.pc = 0x28CD4;
    16
}
/// $078602: lea.l $2df224.l, a0
pub fn i_078602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78608;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078608: move.l a0, $70(a6)
pub fn i_078608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7860C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07860C: move.w #$a, d1
pub fn i_07860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78610;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078610: bra.w $78778
pub fn i_078610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78614;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var24_078614  ($078614, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078614: lea.l $2de34e.l, a0
pub fn i_078614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7861A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE34E;
    20
}
/// $07861A: jsr $28cd4.l
pub fn i_07861a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78620);
    cpu.pc = 0x28CD4;
    16
}
/// $078620: lea.l $2df224.l, a0
pub fn i_078620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78626;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078626: move.l a0, $70(a6)
pub fn i_078626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7862A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07862A: move.w #$72, d1
pub fn i_07862a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7862E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07862E: bra.w $78778
pub fn i_07862e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78632;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var25_078632  ($078632, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078632: lea.l $2de34e.l, a0
pub fn i_078632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78638;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE34E;
    20
}
/// $078638: jsr $28cd4.l
pub fn i_078638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7863E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7863E);
    cpu.pc = 0x28CD4;
    16
}
/// $07863E: lea.l $2df224.l, a0
pub fn i_07863e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78644;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078644: move.l a0, $70(a6)
pub fn i_078644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78648;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078648: move.w #$77, d1
pub fn i_078648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7864C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07864C: bra.w $78778
pub fn i_07864c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78650;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var26_078650  ($078650, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078650: lea.l $2de3c4.l, a0
pub fn i_078650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78656;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE3C4;
    20
}
/// $078656: jsr $28cd4.l
pub fn i_078656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7865C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7865C);
    cpu.pc = 0x28CD4;
    16
}
/// $07865C: lea.l $2df21a.l, a0
pub fn i_07865c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78662;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078662: move.l a0, $70(a6)
pub fn i_078662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78666;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078666: move.w #$a, d1
pub fn i_078666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7866A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07866A: bra.w $78778
pub fn i_07866a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7866E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var27_07866e  ($07866E, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $07866E: lea.l $2de3c4.l, a0
pub fn i_07866e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78674;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE3C4;
    20
}
/// $078674: jsr $28cd4.l
pub fn i_078674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7867A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7867A);
    cpu.pc = 0x28CD4;
    16
}
/// $07867A: lea.l $2df21a.l, a0
pub fn i_07867a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78680;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078680: move.l a0, $70(a6)
pub fn i_078680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78684;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078684: move.w #$72, d1
pub fn i_078684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78688;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078688: bra.w $78778
pub fn i_078688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7868C;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var28_07868c  ($07868C, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $07868C: lea.l $2de43a.l, a0
pub fn i_07868c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78692;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $078692: jsr $28cd4.l
pub fn i_078692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78698);
    cpu.pc = 0x28CD4;
    16
}
/// $078698: lea.l $2df21a.l, a0
pub fn i_078698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7869E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $07869E: move.l a0, $70(a6)
pub fn i_07869e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786A2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0786A2: move.w #$a, d1
pub fn i_0786a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0786A6: bra.w $78778
pub fn i_0786a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var29_0786aa  ($0786AA, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0786AA: lea.l $2de43a.l, a0
pub fn i_0786aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $0786B0: jsr $28cd4.l
pub fn i_0786b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x786B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0786B6: lea.l $2df21a.l, a0
pub fn i_0786b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0786BC: move.l a0, $70(a6)
pub fn i_0786bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786C0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0786C0: move.w #$72, d1
pub fn i_0786c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786C4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0786C4: bra.w $78778
pub fn i_0786c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var30_0786c8  ($0786C8, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0786C8: lea.l $2de43a.l, a0
pub fn i_0786c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $0786CE: jsr $28cd4.l
pub fn i_0786ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x786D4);
    cpu.pc = 0x28CD4;
    16
}
/// $0786D4: lea.l $2df21a.l, a0
pub fn i_0786d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0786DA: move.l a0, $70(a6)
pub fn i_0786da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786DE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0786DE: move.w #$77, d1
pub fn i_0786de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0786E2: bra.w $78778
pub fn i_0786e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var31_0786e6  ($0786E6, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0786E6: lea.l $2de43a.l, a0
pub fn i_0786e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $0786EC: jsr $28cd4.l
pub fn i_0786ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x786F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0786F2: lea.l $2df21a.l, a0
pub fn i_0786f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0786F8: move.l a0, $70(a6)
pub fn i_0786f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786FC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0786FC: move.w #$88, d1
pub fn i_0786fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78700;
    cpu.ir = 0x323C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078700: bra.w $78778
pub fn i_078700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78704;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var32_078704  ($078704, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $078704: lea.l $2de43a.l, a0
pub fn i_078704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7870A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $07870A: jsr $28cd4.l
pub fn i_07870a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78710);
    cpu.pc = 0x28CD4;
    16
}
/// $078710: lea.l $2df21a.l, a0
pub fn i_078710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78716;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078716: move.l a0, $70(a6)
pub fn i_078716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7871A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07871A: move.w #$28, d1
pub fn i_07871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7871E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07871E: bra.w $78778
pub fn i_07871e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78722;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
/// $078722: lea.l $2de4b0.l, a0
pub fn i_078722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78728;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE4B0;
    20
}
/// $078728: jsr $28cd4.l
pub fn i_078728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7872E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7872E);
    cpu.pc = 0x28CD4;
    16
}
/// $07872E: lea.l $2df21a.l, a0
pub fn i_07872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78734;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078734: move.l a0, $70(a6)
pub fn i_078734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78738;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078738: move.w #$b, d1
pub fn i_078738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7873C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07873C: bra.w $78778
pub fn i_07873c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78740;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Debris_ArcA_078740  ($078740, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078740: lea.l $2de5f4.l, a0
pub fn i_078740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78746;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE5F4;
    20
}
/// $078746: jsr $28cd4.l
pub fn i_078746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7874C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7874C);
    cpu.pc = 0x28CD4;
    16
}
/// $07874C: lea.l $2df21a.l, a0
pub fn i_07874c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78752;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078752: move.l a0, $70(a6)
pub fn i_078752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78756;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078756: move.w #$b, d1
pub fn i_078756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7875A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07875A: bra.w $78778
pub fn i_07875a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7875E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Debris_ArcB_07875e  ($07875E, 226 B, props/fx_explosions_platforms_crew_076000.s)
/// $07875E: lea.l $2de66a.l, a0
pub fn i_07875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78764;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE66A;
    20
}
/// $078764: jsr $28cd4.l
pub fn i_078764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7876A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7876A);
    cpu.pc = 0x28CD4;
    16
}
/// $07876A: lea.l $2df224.l, a0
pub fn i_07876a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78770;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078770: move.l a0, $70(a6)
pub fn i_078770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78774;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078774: move.w #$b, d1
pub fn i_078774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78778;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078778: jsr $236e.l
pub fn i_078778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7877E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7877E);
    cpu.pc = 0x236E;
    16
}
/// $07877E: move.b #$ff, $32(a6)
pub fn i_07877e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78784;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078784: move.b #$ff, $33(a6)
pub fn i_078784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7878A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07878A: jsr $77e00(pc)
pub fn i_07878a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7878E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7878E);
    cpu.pc = 0x77E00;
    16
}
/// $07878E: move.w #$0, d0
pub fn i_07878e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78792;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078792: jsr $5dca4.l
pub fn i_078792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78798);
    cpu.pc = 0x5DCA4;
    16
}
/// $078798: move.w d0, $28(a6)
pub fn i_078798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7879C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07879C: move.w #$7f8, $2a(a6)
pub fn i_07879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0787A2: move.w #$ffbc, $2e(a6)
pub fn i_0787a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0787A8: move.w #$0, $2c(a6)
pub fn i_0787a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0787AE: move.w #$3ff, d0
pub fn i_0787ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0787B2: jsr $5e9e4.l
pub fn i_0787b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x787B8);
    cpu.pc = 0x5E9E4;
    16
}
/// $0787B8: btst.b #$0, d0
pub fn i_0787b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787BC;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0787BC: beq.w $787c2
pub fn i_0787bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x787C2; return 10; }
    8
}
/// $0787C0: neg.w d0
pub fn i_0787c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787C2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0787C2: move.w d0, $28(a6)
pub fn i_0787c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0787C6: move.w #$7ff, d0
pub fn i_0787c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787CA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0787CA: jsr $5e9e4.l
pub fn i_0787ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x787D0);
    cpu.pc = 0x5E9E4;
    16
}
/// $0787D0: sub.w d0, $2a(a6)
pub fn i_0787d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787D4;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0787D4: andi.b #$1, d0
pub fn i_0787d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787D8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0787D8: or.b d0, $3a(a6)
pub fn i_0787d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787DC;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0787DC: movea.l $70(a6), a0
pub fn i_0787dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787E0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0787E0: jsr $5dd56.l
pub fn i_0787e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x787E6);
    cpu.pc = 0x5DD56;
    16
}
/// $0787E6: bcs.w $78838
pub fn i_0787e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787EA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78838; return 10; }
    8
}
/// $0787EA: lea.l $ffff.w, a0
pub fn i_0787ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787EE;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0787EE: move.l a0, $48(a6)
pub fn i_0787ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787F2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0787F2: lea.l $787f8(pc), a1
pub fn i_0787f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x787F8;
    16
}
/// $0787F6: move.l a1, (a6)
pub fn i_0787f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0787F8: cmpi.w #$fb00, $2a(a6)
pub fn i_0787f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787FE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFB00, Size::Word, false);
    16
}
/// $0787FE: bgt.w $78808
pub fn i_0787fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78802;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x78808; return 10; }
    8
}
/// $078802: move.w #$fb01, $2a(a6)
pub fn i_078802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78808;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFB01;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078808: jsr $27cee.l
pub fn i_078808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7880E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7880E);
    cpu.pc = 0x27CEE;
    16
}
/// $07880E: jsr $5e804.l
pub fn i_07880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78814);
    cpu.pc = 0x5E804;
    16
}
/// $078814: bcc.w $78820
pub fn i_078814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78818;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78820; return 10; }
    8
}
/// $078818: andi.b #$ee, ccr
pub fn i_078818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7881C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07881C: bra.w $78826
pub fn i_07881c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78820;
    cpu.ir = 0x6000;
    cpu.pc = 0x78826;
    10
}
/// $078820: jsr $28d70.l
pub fn i_078820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78826);
    cpu.pc = 0x28D70;
    16
}
/// $078826: bcs.w $7820a
pub fn i_078826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7882A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7820A; return 10; }
    8
}
/// $07882A: movea.l $70(a6), a0
pub fn i_07882a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7882E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07882E: jsr $5dd56.l
pub fn i_07882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78834;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78834);
    cpu.pc = 0x5DD56;
    16
}
/// $078834: bcc.w $7883e
pub fn i_078834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78838;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7883E; return 10; }
    8
}
/// $078838: jmp $518.l
pub fn i_078838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7883E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07883E: rts
pub fn i_07883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78840;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Smoke_Small_078840  ($078840, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $078840: move.w #$c, d1
pub fn i_078840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78844;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078844: jsr $236e.l
pub fn i_078844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7884A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7884A);
    cpu.pc = 0x236E;
    16
}
/// $07884A: lea.l $2ded26.l, a0
pub fn i_07884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78850;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DED26;
    20
}
/// $078850: jsr $28cd4.l
pub fn i_078850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78856);
    cpu.pc = 0x28CD4;
    16
}
/// $078856: lea.l $2df238.l, a0
pub fn i_078856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7885C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $07885C: move.l a0, $70(a6)
pub fn i_07885c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78860;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078860: bra.w $789ae
pub fn i_078860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78864;
    cpu.ir = 0x6000;
    cpu.pc = 0x789AE;
    10
}
// ===== Smoke_Pair40_078864  ($078864, 44 B, props/fx_explosions_platforms_crew_076000.s)
/// $078864: lea.l $78908(pc), a1
pub fn i_078864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78868;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78908;
    16
}
/// $078868: jsr $4ae.l
pub fn i_078868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7886E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7886E);
    cpu.pc = 0x4AE;
    16
}
/// $07886E: jsr $5dd02.l
pub fn i_07886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78874);
    cpu.pc = 0x5DD02;
    16
}
/// $078874: addi.w #$20, $22(a0)
pub fn i_078874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7887A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07887A: lea.l $78908(pc), a1
pub fn i_07887a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7887E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78908;
    16
}
/// $07887E: jsr $4ae.l
pub fn i_07887e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78884);
    cpu.pc = 0x4AE;
    16
}
/// $078884: jsr $5dd02.l
pub fn i_078884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7888A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7888A);
    cpu.pc = 0x5DD02;
    16
}
/// $07888A: addi.w #$ffe0, $22(a0)
pub fn i_07888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78890;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== Smoke_Pair20_078890  ($078890, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $078890: lea.l $78908(pc), a1
pub fn i_078890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78894;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78908;
    16
}
/// $078894: jsr $4ae.l
pub fn i_078894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7889A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7889A);
    cpu.pc = 0x4AE;
    16
}
/// $07889A: jsr $5dd02.l
pub fn i_07889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788A0);
    cpu.pc = 0x5DD02;
    16
}
/// $0788A0: addi.w #$10, $22(a0)
pub fn i_0788a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788A6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0788A6: lea.l $78908(pc), a1
pub fn i_0788a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78908;
    16
}
/// $0788AA: jsr $4ae.l
pub fn i_0788aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788B0);
    cpu.pc = 0x4AE;
    16
}
/// $0788B0: jsr $5dd02.l
pub fn i_0788b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788B6);
    cpu.pc = 0x5DD02;
    16
}
/// $0788B6: addi.w #$fff0, $22(a0)
pub fn i_0788b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788BC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0788BC: bra.w $78908
pub fn i_0788bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x78908;
    10
}
// ===== Smoke_Std_0788c0  ($0788C0, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $0788C0: move.w #$c, d1
pub fn i_0788c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788C4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0788C4: jsr $236e.l
pub fn i_0788c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788CA);
    cpu.pc = 0x236E;
    16
}
/// $0788CA: lea.l $2de7c2.l, a0
pub fn i_0788ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE7C2;
    20
}
/// $0788D0: jsr $28cd4.l
pub fn i_0788d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788D6);
    cpu.pc = 0x28CD4;
    16
}
/// $0788D6: lea.l $2df238.l, a0
pub fn i_0788d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $0788DC: move.l a0, $70(a6)
pub fn i_0788dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788E0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0788E0: bra.w $78998
pub fn i_0788e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x78998;
    10
}
// ===== Smoke_StdB_0788e4  ($0788E4, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $0788E4: move.w #$c, d1
pub fn i_0788e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788E8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0788E8: jsr $236e.l
pub fn i_0788e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788EE);
    cpu.pc = 0x236E;
    16
}
/// $0788EE: lea.l $2de7c2.l, a0
pub fn i_0788ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE7C2;
    20
}
/// $0788F4: jsr $28cd4.l
pub fn i_0788f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788FA);
    cpu.pc = 0x28CD4;
    16
}
/// $0788FA: lea.l $2df238.l, a0
pub fn i_0788fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78900;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $078900: move.l a0, $70(a6)
pub fn i_078900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78904;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078904: bra.w $789ae
pub fn i_078904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78908;
    cpu.ir = 0x6000;
    cpu.pc = 0x789AE;
    10
}
// ===== Smoke_Big_078908  ($078908, 232 B, props/fx_explosions_platforms_crew_076000.s)
/// $078908: move.w #$139, d1
pub fn i_078908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7890C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x139;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07890C: jsr $236e.l
pub fn i_07890c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78912);
    cpu.pc = 0x236E;
    16
}
/// $078912: lea.l $2de7c2.l, a0
pub fn i_078912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78918;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE7C2;
    20
}
/// $078918: jsr $28cd4.l
pub fn i_078918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7891E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7891E);
    cpu.pc = 0x28CD4;
    16
}
/// $07891E: lea.l $2df238.l, a0
pub fn i_07891e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78924;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $078924: move.l a0, $70(a6)
pub fn i_078924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78928;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078928: bra.w $78998
pub fn i_078928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7892C;
    cpu.ir = 0x6000;
    cpu.pc = 0x78998;
    10
}
/// $07892C: move.w #$c, d1
pub fn i_07892c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78930;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078930: jsr $236e.l
pub fn i_078930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78936);
    cpu.pc = 0x236E;
    16
}
/// $078936: lea.l $2de93e.l, a0
pub fn i_078936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7893C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE93E;
    20
}
/// $07893C: jsr $28cd4.l
pub fn i_07893c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78942);
    cpu.pc = 0x28CD4;
    16
}
/// $078942: lea.l $2df242.l, a0
pub fn i_078942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78948;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF242;
    20
}
/// $078948: move.l a0, $70(a6)
pub fn i_078948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7894C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07894C: bra.w $78998
pub fn i_07894c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78950;
    cpu.ir = 0x6000;
    cpu.pc = 0x78998;
    10
}
/// $078950: move.w #$c, d1
pub fn i_078950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78954;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078954: jsr $236e.l
pub fn i_078954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7895A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7895A);
    cpu.pc = 0x236E;
    16
}
/// $07895A: lea.l $2deaba.l, a0
pub fn i_07895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78960;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DEABA;
    20
}
/// $078960: jsr $28cd4.l
pub fn i_078960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78966;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78966);
    cpu.pc = 0x28CD4;
    16
}
/// $078966: lea.l $2df238.l, a0
pub fn i_078966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7896C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $07896C: move.l a0, $70(a6)
pub fn i_07896c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78970;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078970: bra.w $78998
pub fn i_078970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78974;
    cpu.ir = 0x6000;
    cpu.pc = 0x78998;
    10
}
/// $078974: move.w #$c, d1
pub fn i_078974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78978;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078978: jsr $236e.l
pub fn i_078978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7897E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7897E);
    cpu.pc = 0x236E;
    16
}
/// $07897E: lea.l $2dec0c.l, a0
pub fn i_07897e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78984;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DEC0C;
    20
}
/// $078984: jsr $28cd4.l
pub fn i_078984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7898A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7898A);
    cpu.pc = 0x28CD4;
    16
}
/// $07898A: lea.l $2df242.l, a0
pub fn i_07898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78990;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF242;
    20
}
/// $078990: move.l a0, $70(a6)
pub fn i_078990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78994;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078994: bra.w $78998
pub fn i_078994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78998;
    cpu.ir = 0x6000;
    cpu.pc = 0x78998;
    10
}
/// $078998: move.b #$ff, $32(a6)
pub fn i_078998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7899E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07899E: move.b #$ff, $33(a6)
pub fn i_07899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0789A4: jsr $77e00(pc)
pub fn i_0789a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x789A8);
    cpu.pc = 0x77E00;
    16
}
/// $0789A8: bset.b #$6, $12(a6)
pub fn i_0789a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789AE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0789AE: movea.l $70(a6), a0
pub fn i_0789ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789B2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0789B2: jsr $5dd56.l
pub fn i_0789b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789B8);
    cpu.pc = 0x5DD56;
    16
}
/// $0789B8: bcs.w $789e8
pub fn i_0789b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789BC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x789E8; return 10; }
    8
}
/// $0789BC: lea.l $ffff.w, a0
pub fn i_0789bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789C0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0789C0: move.l a0, $48(a6)
pub fn i_0789c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789C4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0789C4: lea.l $789ca(pc), a1
pub fn i_0789c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x789CA;
    16
}
/// $0789C8: move.l a1, (a6)
pub fn i_0789c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0789CA: jsr $44048.l
pub fn i_0789ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789D0);
    cpu.pc = 0x44048;
    16
}
/// $0789D0: jsr $28d70.l
pub fn i_0789d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789D6);
    cpu.pc = 0x28D70;
    16
}
/// $0789D6: bcs.w $789e8
pub fn i_0789d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x789E8; return 10; }
    8
}
/// $0789DA: movea.l $70(a6), a0
pub fn i_0789da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789DE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0789DE: jsr $5dd56.l
pub fn i_0789de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789E4);
    cpu.pc = 0x5DD56;
    16
}
/// $0789E4: bcc.w $789ee
pub fn i_0789e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x789EE; return 10; }
    8
}
/// $0789E8: jmp $518.l
pub fn i_0789e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789EE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0789EE: rts
pub fn i_0789ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Smoke_Column_0789f0  ($0789F0, 120 B, props/fx_explosions_platforms_crew_076000.s)
/// $0789F0: lea.l $2deea2.l, a0
pub fn i_0789f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DEEA2;
    20
}
/// $0789F6: jsr $28cd4.l
pub fn i_0789f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789FC);
    cpu.pc = 0x28CD4;
    16
}
/// $0789FC: lea.l $2df24c.l, a0
pub fn i_0789fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF24C;
    20
}
/// $078A02: move.l a0, $70(a6)
pub fn i_078a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A06;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078A06: bra.w $78a0a
pub fn i_078a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A0A;
    cpu.ir = 0x6000;
    cpu.pc = 0x78A0A;
    10
}
/// $078A0A: move.w #$10, d1
pub fn i_078a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A0E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078A0E: jsr $236e.l
pub fn i_078a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A14);
    cpu.pc = 0x236E;
    16
}
/// $078A14: bset.b #$6, $12(a6)
pub fn i_078a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A1A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078A1A: move.b #$ff, $32(a6)
pub fn i_078a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A20;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078A20: move.b #$ff, $33(a6)
pub fn i_078a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078A26: movea.l $70(a6), a0
pub fn i_078a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A2A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078A2A: jsr $5dd56.l
pub fn i_078a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A30);
    cpu.pc = 0x5DD56;
    16
}
/// $078A30: bcs.w $78a60
pub fn i_078a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A34;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78A60; return 10; }
    8
}
/// $078A34: lea.l $ffff.w, a0
pub fn i_078a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A38;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $078A38: move.l a0, $48(a6)
pub fn i_078a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A3C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078A3C: lea.l $78a42(pc), a1
pub fn i_078a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78A42;
    16
}
/// $078A40: move.l a1, (a6)
pub fn i_078a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078A42: jsr $44048.l
pub fn i_078a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A48);
    cpu.pc = 0x44048;
    16
}
/// $078A48: jsr $28d70.l
pub fn i_078a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A4E);
    cpu.pc = 0x28D70;
    16
}
/// $078A4E: bcs.w $78a60
pub fn i_078a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A52;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78A60; return 10; }
    8
}
/// $078A52: movea.l $70(a6), a0
pub fn i_078a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A56;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078A56: jsr $5dd56.l
pub fn i_078a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A5C);
    cpu.pc = 0x5DD56;
    16
}
/// $078A5C: bcc.w $78a66
pub fn i_078a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78A66; return 10; }
    8
}
/// $078A60: jmp $518.l
pub fn i_078a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A66;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078A66: rts
pub fn i_078a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_RetGate_078a68  ($078A68, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $078A68: movea.l $8(a6), a1
pub fn i_078a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A6C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $078A6C: move.b $10(a6), d0
pub fn i_078a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A70;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $078A70: cmp.b $10(a1), d0
pub fn i_078a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A74;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $078A74: bcs.w $78a7e
pub fn i_078a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A78;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78A7E; return 10; }
    8
}
// ===== ClearXN_078a78  ($078A78, 6 B, thunks/ccr_helpers.c)
/// $078A78: andi.b #$ee, ccr
pub fn i_078a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A7C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $078A7C: rts
pub fn i_078a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_078a7e  ($078A7E, 6 B, thunks/ccr_helpers.c)
/// $078A7E: ori.b #$11, ccr
pub fn i_078a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A82;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $078A82: rts
pub fn i_078a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $078C64: move.w $22(a6), d0
pub fn i_078c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C68;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078C68: addi.w #$40, d0
pub fn i_078c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C6C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $078C6C: bpl.w $78c76
pub fn i_078c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C70;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x78C76; return 10; }
    8
}
/// $078C70: jmp $518.l
pub fn i_078c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C76;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078C76: rts
pub fn i_078c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Tmpl24_078c78  ($078C78, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $078C78: move.w #$1, $86(a6)
pub fn i_078c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C7E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078C7E: bset.b #$0, $3a(a6)
pub fn i_078c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C84;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078C84: bra.w $78ca8
pub fn i_078c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C88;
    cpu.ir = 0x6000;
    cpu.pc = 0x78CA8;
    10
}
// ===== Breakable_Tmpl26_078c88  ($078C88, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $078C88: move.w #$0, $86(a6)
pub fn i_078c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078C8E: bset.b #$0, $3a(a6)
pub fn i_078c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C94;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078C94: bra.w $78ca8
pub fn i_078c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C98;
    cpu.ir = 0x6000;
    cpu.pc = 0x78CA8;
    10
}
// ===== Breakable_Tmpl23_078c98  ($078C98, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $078C98: move.w #$1, $86(a6)
pub fn i_078c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C9E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078C9E: bra.w $78ca8
pub fn i_078c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x78CA8;
    10
}
// ===== Breakable_Tmpl25_078ca2  ($078CA2, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $078CA2: move.w #$0, $86(a6)
pub fn i_078ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CA8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078CA8: bclr.b #$1, $12(a6)
pub fn i_078ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CAE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078CAE: move.w #$1e, d1
pub fn i_078cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078CB2: jsr $236e.l
pub fn i_078cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CB8);
    cpu.pc = 0x236E;
    16
}
/// $078CB8: lea.l $78b90(pc), a0
pub fn i_078cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CBC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78B90;
    16
}
/// $078CBC: jsr $28cd4.l
pub fn i_078cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CC2);
    cpu.pc = 0x28CD4;
    16
}
/// $078CC2: lea.l $78cc8(pc), a1
pub fn i_078cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78CC8;
    16
}
/// $078CC6: move.l a1, (a6)
pub fn i_078cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078CC8: jsr $2783a.l
pub fn i_078cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CCE);
    cpu.pc = 0x2783A;
    16
}
/// $078CCE: jsr $28d70.l
pub fn i_078cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CD4);
    cpu.pc = 0x28D70;
    16
}
/// $078CD4: moveq #$0, d0
pub fn i_078cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CD6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $078CD6: move.b $98(a6), d0
pub fn i_078cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CDA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $078CDA: lsl.w #$3, d0
pub fn i_078cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CDC;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078CDC: neg.w d0
pub fn i_078cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CDE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $078CDE: addi.w #$140, d0
pub fn i_078cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CE2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x140, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $078CE2: cmp.w $22(a6), d0
pub fn i_078ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CE6;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $078CE6: bcs.w $78cf0
pub fn i_078ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CEA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78CF0; return 10; }
    8
}
/// $078CEA: lea.l $78cf4(pc), a1
pub fn i_078cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78CF4;
    16
}
/// $078CEE: move.l a1, (a6)
pub fn i_078cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078CF0: bra.w $78c64
pub fn i_078cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x78C64;
    10
}
// ===== Breakable_Destroyed_078cf4  ($078CF4, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $078CF4: lea.l $78a84(pc), a0
pub fn i_078cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CF8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78A84;
    16
}
/// $078CF8: jsr $28cd4.l
pub fn i_078cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CFE);
    cpu.pc = 0x28CD4;
    16
}
/// $078CFE: lea.l $78d58(pc), a1
pub fn i_078cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78D58;
    16
}
/// $078D02: jsr $4ae.l
pub fn i_078d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D08);
    cpu.pc = 0x4AE;
    16
}
/// $078D08: move.w $86(a6), $86(a0)
pub fn i_078d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D0E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x86);
    bus.write16(ad4, v3 as u16);
    20
}
/// $078D0E: lea.l $78d14(pc), a1
pub fn i_078d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78D14;
    16
}
/// $078D12: move.l a1, (a6)
pub fn i_078d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078D14: jsr $2783a.l
pub fn i_078d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D1A);
    cpu.pc = 0x2783A;
    16
}
/// $078D1A: jsr $28d70.l
pub fn i_078d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D20);
    cpu.pc = 0x28D70;
    16
}
/// $078D20: bcc.w $78d2a
pub fn i_078d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78D2A; return 10; }
    8
}
/// $078D24: lea.l $78d2e(pc), a1
pub fn i_078d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78D2E;
    16
}
/// $078D28: move.l a1, (a6)
pub fn i_078d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078D2A: bra.w $78c64
pub fn i_078d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78C64;
    10
}
// ===== Breakable_FadeOut_078d2e  ($078D2E, 42 B, props/fx_explosions_platforms_crew_076000.s)
/// $078D2E: move.b #$1e, $59(a6)
pub fn i_078d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078D34: lea.l $78d3a(pc), a1
pub fn i_078d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78D3A;
    16
}
/// $078D38: move.l a1, (a6)
pub fn i_078d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078D3A: jsr $2783a.l
pub fn i_078d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D40);
    cpu.pc = 0x2783A;
    16
}
/// $078D40: jsr $28d70.l
pub fn i_078d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D46);
    cpu.pc = 0x28D70;
    16
}
/// $078D46: tst.b $59(a6)
pub fn i_078d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D4A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $078D4A: bne.w $78d54
pub fn i_078d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x78D54; return 10; }
    8
}
/// $078D4E: jmp $518.l
pub fn i_078d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D54;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078D54: bra.w $78c64
pub fn i_078d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D58;
    cpu.ir = 0x6000;
    cpu.pc = 0x78C64;
    10
}
// ===== Breakable_Wreck_Fall_078d58  ($078D58, 150 B, props/fx_explosions_platforms_crew_076000.s)
/// $078D58: move.w #$8000, d0
pub fn i_078d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D5C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078D5C: jsr $28134.l
pub fn i_078d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D62);
    cpu.pc = 0x28134;
    16
}
/// $078D62: andi.w #$ffe3, $38(a6)
pub fn i_078d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D68;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $078D68: ori.w #$18, $38(a6)
pub fn i_078d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D6E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $078D6E: bset.b #$1, $12(a6)
pub fn i_078d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D74;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078D74: move.w #$e, d1
pub fn i_078d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D78;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078D78: jsr $236e.l
pub fn i_078d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D7E);
    cpu.pc = 0x236E;
    16
}
/// $078D7E: lea.l $78b9c(pc), a0
pub fn i_078d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D82;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78B9C;
    16
}
/// $078D82: jsr $28cd4.l
pub fn i_078d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D88);
    cpu.pc = 0x28CD4;
    16
}
/// $078D88: movea.l $c(a6), a0
pub fn i_078d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D8C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078D8C: lea.l $98(a0), a1
pub fn i_078d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D90;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    cpu.a[1] = ad1;
    16
}
/// $078D90: lea.l $98(a6), a2
pub fn i_078d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D94;
    cpu.ir = 0x45EE;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    cpu.a[2] = ad1;
    16
}
/// $078D94: move.b (a1)+, (a2)+
pub fn i_078d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D96;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D96: move.b (a1)+, (a2)+
pub fn i_078d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D98;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D98: move.b (a1)+, (a2)+
pub fn i_078d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D9A;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D9A: move.b (a1)+, (a2)+
pub fn i_078d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D9C;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D9C: move.b (a1)+, (a2)+
pub fn i_078d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D9E;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D9E: move.b (a1)+, (a2)+
pub fn i_078d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DA0;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078DA0: move.b (a1)+, (a2)+
pub fn i_078da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DA2;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078DA2: move.b (a1)+, (a2)+
pub fn i_078da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DA4;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078DA4: move.w $3a(a0), $3a(a6)
pub fn i_078da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DAA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $078DAA: lea.l $78be0(pc), a0
pub fn i_078daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DAE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78BE0;
    16
}
/// $078DAE: move.l a0, $70(a6)
pub fn i_078dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DB2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078DB2: lea.l $78db8(pc), a1
pub fn i_078db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78DB8;
    16
}
/// $078DB6: move.l a1, (a6)
pub fn i_078db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078DB8: movea.l $70(a6), a0
pub fn i_078db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DBC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078DBC: move.w (a0)+, d0
pub fn i_078dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DBE;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $078DBE: move.w (a0)+, d1
pub fn i_078dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DC0;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $078DC0: btst.b #$0, $3a(a6)
pub fn i_078dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DC6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $078DC6: beq.w $78dcc
pub fn i_078dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x78DCC; return 10; }
    8
}
/// $078DCA: neg.w d0
pub fn i_078dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DCC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $078DCC: movea.l $c(a6), a1
pub fn i_078dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DD0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $078DD0: add.w $22(a1), d0
pub fn i_078dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DD4;
    cpu.ir = 0xD069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078DD4: add.w $24(a1), d1
pub fn i_078dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DD8;
    cpu.ir = 0xD269;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078DD8: move.w d0, $22(a6)
pub fn i_078dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DDC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078DDC: move.w d1, $24(a6)
pub fn i_078ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DE0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078DE0: move.l a0, $70(a6)
pub fn i_078de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DE4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078DE4: cmpa.l #$78c64, a0
pub fn i_078de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DEA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x78C64;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $078DEA: bcc.w $78df6
pub fn i_078dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78DF6; return 10; }
    8
}
// ===== JsrAbsThunk_078dee  ($078DEE, 8 B, thunks/jsr_abs_thunks.c)
/// $078DEE: jsr $28d70.l
pub fn i_078dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78DF4);
    cpu.pc = 0x28D70;
    16
}
/// $078DF4: rts
pub fn i_078df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Wreck_SpawnSoldier_078df6  ($078DF6, 86 B, props/fx_explosions_platforms_crew_076000.s)
/// $078DF6: jsr $28d70.l
pub fn i_078df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78DFC);
    cpu.pc = 0x28D70;
    16
}
/// $078DFC: move.w $86(a6), d0
pub fn i_078dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E00;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078E00: cmpi.w #$1, d0
pub fn i_078e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E04;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $078E04: bne.w $78e1e
pub fn i_078e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x78E1E; return 10; }
    8
}
/// $078E08: lea.l $57880.l, a1
pub fn i_078e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E0E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x57880;
    20
}
/// $078E0E: jsr $4498e.l
pub fn i_078e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78E14);
    cpu.pc = 0x4498E;
    16
}
/// $078E14: jsr $5dd02.l
pub fn i_078e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78E1A);
    cpu.pc = 0x5DD02;
    16
}
/// $078E1A: bra.w $78e30
pub fn i_078e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78E30;
    10
}
/// $078E1E: lea.l $5783e.l, a1
pub fn i_078e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E24;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5783E;
    20
}
/// $078E24: jsr $4498e.l
pub fn i_078e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78E2A);
    cpu.pc = 0x4498E;
    16
}
/// $078E2A: jsr $5dd02.l
pub fn i_078e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78E30);
    cpu.pc = 0x5DD02;
    16
}
/// $078E30: lea.l $99(a6), a1
pub fn i_078e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E34;
    cpu.ir = 0x43EE;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    cpu.a[1] = ad1;
    16
}
/// $078E34: lea.l $99(a0), a2
pub fn i_078e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E38;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x99);
    cpu.a[2] = ad1;
    16
}
/// $078E38: move.b (a1)+, (a2)+
pub fn i_078e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E3A;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E3A: move.b (a1)+, (a2)+
pub fn i_078e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E3C;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E3C: move.b (a1)+, (a2)+
pub fn i_078e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E3E;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E3E: move.b (a1)+, (a2)+
pub fn i_078e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E40;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E40: move.b (a1)+, (a2)+
pub fn i_078e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E42;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E42: move.b (a1)+, (a2)+
pub fn i_078e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E44;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E44: move.b (a1)+, (a2)+
pub fn i_078e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E46;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E46: jmp $518.l
pub fn i_078e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E4C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Breakable_Cb_SpawnChip_078ede  ($078EDE, 54 B, props/fx_explosions_platforms_crew_076000.s)
/// $078EDE: lea.l $78f2c(pc), a1
pub fn i_078ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78F2C;
    16
}
/// $078EE2: jsr $4ae.l
pub fn i_078ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78EE8);
    cpu.pc = 0x4AE;
    16
}
/// $078EE8: move.b $3a(a6), $3a(a0)
pub fn i_078ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EEE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $078EEE: move.w $74(a6), d0
pub fn i_078eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EF2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078EF2: move.w $78(a6), d1
pub fn i_078ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EF6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078EF6: btst.b #$0, $3a(a6)
pub fn i_078ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EFC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $078EFC: beq.w $78f02
pub fn i_078efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x78F02; return 10; }
    8
}
/// $078F00: neg.w d0
pub fn i_078f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F02;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $078F02: add.w $22(a6), d0
pub fn i_078f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F06;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078F06: add.w $24(a6), d1
pub fn i_078f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F0A;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078F0A: move.w d0, $22(a0)
pub fn i_078f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F0E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078F0E: move.w d1, $24(a0)
pub fn i_078f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F12;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078F12: rts
pub fn i_078f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Cb_SpawnChipFlip_078f14  ($078F14, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F14: lea.l $78f2c(pc), a1
pub fn i_078f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78F2C;
    16
}
/// $078F18: jsr $4ae.l
pub fn i_078f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F1E);
    cpu.pc = 0x4AE;
    16
}
/// $078F1E: move.b $3a(a6), d0
pub fn i_078f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F22;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $078F22: bchg.b #$0, d0
pub fn i_078f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F26;
    cpu.ir = 0x840;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 ^ mk;
    cpu.d[0] = r2;
    8
}
/// $078F26: move.b d0, $3a(a0)
pub fn i_078f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F2A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078F2A: bra.b $78eee
pub fn i_078f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F2C;
    cpu.ir = 0x60C2;
    cpu.pc = 0x78EEE;
    10
}
// ===== Breakable_Chip_Run_078f2c  ($078F2C, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F2C: move.w #$1e, d1
pub fn i_078f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078F30: jsr $236e.l
pub fn i_078f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F36);
    cpu.pc = 0x236E;
    16
}
/// $078F36: move.w #$fffc, $2e(a6)
pub fn i_078f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F3C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078F3C: lea.l $78e4c(pc), a0
pub fn i_078f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F40;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78E4C;
    16
}
/// $078F40: jsr $28cd4.l
pub fn i_078f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F46);
    cpu.pc = 0x28CD4;
    16
}
/// $078F46: lea.l $78f4c(pc), a1
pub fn i_078f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78F4C;
    16
}
/// $078F4A: move.l a1, (a6)
pub fn i_078f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078F4C: jsr $27d50.l
pub fn i_078f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F52);
    cpu.pc = 0x27D50;
    16
}
/// $078F52: bcc.w $78f5c
pub fn i_078f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78F5C; return 10; }
    8
}
/// $078F56: jmp $518.l
pub fn i_078f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F5C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078F5C: jsr $28d70.l
pub fn i_078f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F62);
    cpu.pc = 0x28D70;
    16
}
/// $078F62: bcc.w $78f6c
pub fn i_078f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78F6C; return 10; }
    8
}
/// $078F66: jmp $518.l
pub fn i_078f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F6C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078F6C: rts
pub fn i_078f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PathScript_RetGate_078f6e  ($078F6E, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F6E: movea.l $8(a6), a1
pub fn i_078f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F72;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $078F72: move.b $10(a6), d0
pub fn i_078f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F76;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $078F76: cmp.b $10(a1), d0
pub fn i_078f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F7A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $078F7A: bcs.w $78f84
pub fn i_078f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F7E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78F84; return 10; }
    8
}
// ===== ClearXN_078f7e  ($078F7E, 6 B, thunks/ccr_helpers.c)
/// $078F7E: andi.b #$ee, ccr
pub fn i_078f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F82;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $078F82: rts
pub fn i_078f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_078f84  ($078F84, 6 B, thunks/ccr_helpers.c)
/// $078F84: ori.b #$11, ccr
pub fn i_078f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F88;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $078F88: rts
pub fn i_078f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PathScript_StepCtx_078f8a  ($078F8A, 6 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F8A: jsr $2783a.l
pub fn i_078f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F90);
    cpu.pc = 0x2783A;
    16
}
// ===== PathScript_Step_078f90  ($078F90, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F90: move.w $36(a6), d5
pub fn i_078f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F94;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078F94: movea.l $90(a6), a0
pub fn i_078f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F98;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078F98: lea.l $78fc0(pc), a1
pub fn i_078f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78FC0;
    16
}
/// $078F9C: move.b (a0), d0
pub fn i_078f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F9E;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $078F9E: cmpi.b #$b, d0
pub fn i_078f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FA2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $078FA2: bcs.w $78fb2
pub fn i_078fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FA6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78FB2; return 10; }
    8
}
/// $078FA6: nop
pub fn i_078fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FA8;
    cpu.ir = 0x4E71;
    4
}
/// $078FA8: nop
pub fn i_078fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FAA;
    cpu.ir = 0x4E71;
    4
}
/// $078FAA: cmpi.b #$b, d0
pub fn i_078faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FAE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $078FAE: nop
pub fn i_078fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FB0;
    cpu.ir = 0x4E71;
    4
}
/// $078FB0: trap #$f
pub fn i_078fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FB2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $078FB2: andi.l #$f, d0
pub fn i_078fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FB8;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $078FB8: lsl.l #$2, d0
pub fn i_078fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FBA;
    cpu.ir = 0xE588;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 1, true);
    cpu.d[0] = r;
    10
}
/// $078FBA: movea.l (a1, d0.w), a2
pub fn i_078fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FBE;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $078FBE: jmp (a2)
pub fn i_078fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FC0;
    cpu.ir = 0x4ED2;
    cpu.jump_to(cpu.a[2]);
    8
}
/// $079000: trap #$f
pub fn i_079000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79002;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $079002: rts
pub fn i_079002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79004;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PathScript_OpMoveDist_079004  ($079004, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $079004: move.w d5, d0
pub fn i_079004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79006;
    cpu.ir = 0x3005;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079006: move.b $34(a6), d1
pub fn i_079006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7900A;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07900A: jsr $791e0(pc)
pub fn i_07900a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7900E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7900E);
    cpu.pc = 0x791E0;
    16
}
// ===== SetXN_07900e  ($07900E, 6 B, thunks/ccr_helpers.c)
/// $07900E: ori.b #$11, ccr
pub fn i_07900e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79012;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079012: rts
pub fn i_079012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79014;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PathScript_OpJump_079014  ($079014, 12 B, props/fx_explosions_platforms_crew_076000.s)
/// $079014: movea.l $2(a0), a0
pub fn i_079014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79018;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079018: move.l a0, $90(a6)
pub fn i_079018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7901C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07901C: bra.w $78f98
pub fn i_07901c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79020;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
// ===== PathScript_OpCall_079020  ($079020, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $079020: movem.l d5/a0, -(a7)
pub fn i_079020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79024;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $079024: movea.l $2(a0), a1
pub fn i_079024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79028;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079028: jsr (a1)
pub fn i_079028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7902A;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x7902A);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $07902A: movem.l (a7)+, d5/a0
pub fn i_07902a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7902E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07902E: addq.l #$6, a0
pub fn i_07902e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79030;
    cpu.ir = 0x5C88;
    cpu.a[0] = cpu.a[0].wrapping_add(6);
    8
}
/// $079030: move.l a0, $90(a6)
pub fn i_079030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79034;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $079034: bra.w $78f98
pub fn i_079034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79038;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
// ===== PathScript_OpMusic_079038  ($079038, 28 B, props/fx_explosions_platforms_crew_076000.s)
/// $079038: movem.l d5/a0, -(a7)
pub fn i_079038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7903C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $07903C: move.w $2(a0), d0
pub fn i_07903c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79040;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079040: jsr $2352.l
pub fn i_079040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79046;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79046);
    cpu.pc = 0x2352;
    16
}
/// $079046: movem.l (a7)+, d5/a0
pub fn i_079046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7904A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07904A: addq.l #$4, a0
pub fn i_07904a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7904C;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $07904C: move.l a0, $90(a6)
pub fn i_07904c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79050;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $079050: bra.w $78f98
pub fn i_079050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79054;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
// ===== PathScript_OpSetByte_079054  ($079054, 42 B, props/fx_explosions_platforms_crew_076000.s)
/// $079054: move.b $1(a0), d0
pub fn i_079054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79058;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079058: move.w $2(a0), d1
pub fn i_079058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7905C;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07905C: cmpi.w #$a0, d1
pub fn i_07905c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79060;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $079060: bcs.w $79070
pub fn i_079060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79064;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79070; return 10; }
    8
}
/// $079064: nop
pub fn i_079064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79066;
    cpu.ir = 0x4E71;
    4
}
/// $079066: nop
pub fn i_079066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79068;
    cpu.ir = 0x4E71;
    4
}
/// $079068: cmpi.w #$a0, d1
pub fn i_079068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7906C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $07906C: nop
pub fn i_07906c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7906E;
    cpu.ir = 0x4E71;
    4
}
/// $07906E: trap #$f
pub fn i_07906e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79070;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $079070: move.b d0, (a6, d1.w)
pub fn i_079070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79074;
    cpu.ir = 0x1D80;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $079074: addq.l #$4, a0
pub fn i_079074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79076;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $079076: move.l a0, $90(a6)
pub fn i_079076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7907A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07907A: bra.w $78f98
pub fn i_07907a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7907E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
// ===== PathScript_OpSetWord_07907e  ($07907E, 42 B, props/fx_explosions_platforms_crew_076000.s)
/// $07907E: move.w $2(a0), d0
pub fn i_07907e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79082;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079082: move.w $4(a0), d1
pub fn i_079082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79086;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079086: cmpi.w #$a0, d1
pub fn i_079086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7908A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $07908A: bcs.w $7909a
pub fn i_07908a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7908E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7909A; return 10; }
    8
}
/// $07908E: nop
pub fn i_07908e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79090;
    cpu.ir = 0x4E71;
    4
}
/// $079090: nop
pub fn i_079090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79092;
    cpu.ir = 0x4E71;
    4
}
/// $079092: cmpi.w #$a0, d1
pub fn i_079092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79096;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $079096: nop
pub fn i_079096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79098;
    cpu.ir = 0x4E71;
    4
}
/// $079098: trap #$f
pub fn i_079098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7909A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $07909A: move.w d0, (a6, d1.w)
pub fn i_07909a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7909E;
    cpu.ir = 0x3D80;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07909E: addq.l #$6, a0
pub fn i_07909e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790A0;
    cpu.ir = 0x5C88;
    cpu.a[0] = cpu.a[0].wrapping_add(6);
    8
}
/// $0790A0: move.l a0, $90(a6)
pub fn i_0790a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790A4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0790A4: bra.w $78f98
pub fn i_0790a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
// ===== PathScript_OpSetLong_0790a8  ($0790A8, 42 B, props/fx_explosions_platforms_crew_076000.s)
/// $0790A8: move.l $4(a0), d0
pub fn i_0790a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790AC;
    cpu.ir = 0x2028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0790AC: move.w $2(a0), d1
pub fn i_0790ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790B0;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0790B0: cmpi.w #$a0, d1
pub fn i_0790b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790B4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0790B4: bcs.w $790c4
pub fn i_0790b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790B8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x790C4; return 10; }
    8
}
/// $0790B8: nop
pub fn i_0790b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790BA;
    cpu.ir = 0x4E71;
    4
}
/// $0790BA: nop
pub fn i_0790ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790BC;
    cpu.ir = 0x4E71;
    4
}
/// $0790BC: cmpi.w #$a0, d1
pub fn i_0790bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790C0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0790C0: nop
pub fn i_0790c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790C2;
    cpu.ir = 0x4E71;
    4
}
/// $0790C2: trap #$f
pub fn i_0790c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790C4;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0790C4: move.l d0, (a6, d1.w)
pub fn i_0790c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790C8;
    cpu.ir = 0x2D80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $0790C8: addq.l #$8, a0
pub fn i_0790c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790CA;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $0790CA: move.l a0, $90(a6)
pub fn i_0790ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790CE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0790CE: bra.w $78f98
pub fn i_0790ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
// ===== PathScript_OpSetAngle_0790d2  ($0790D2, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $0790D2: move.b $1(a0), d0
pub fn i_0790d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790D6;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0790D6: lsl.w #$8, d0
pub fn i_0790d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790D8;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0790D8: move.w d0, $34(a6)
pub fn i_0790d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0790DC: addq.l #$2, a0
pub fn i_0790dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790DE;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $0790DE: move.l a0, $90(a6)
pub fn i_0790de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790E2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0790E2: bra.w $78f98
pub fn i_0790e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
// ===== PathScript_OpRepeat_0790e6  ($0790E6, 26 B, props/fx_explosions_platforms_crew_076000.s)
/// $0790E6: addq.w #$1, $94(a6)
pub fn i_0790e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790EA;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0790EA: move.w $2(a0), d0
pub fn i_0790ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790EE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0790EE: cmp.w $94(a6), d0
pub fn i_0790ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790F2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0790F2: bhi.w $79100
pub fn i_0790f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790F6;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x79100; return 10; }
    8
}
/// $0790F6: clr.w $94(a6)
pub fn i_0790f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790FA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0790FA: addq.l #$4, a0
pub fn i_0790fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790FC;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $0790FC: move.l a0, $90(a6)
pub fn i_0790fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79100;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== ClearXN_079100  ($079100, 6 B, thunks/ccr_helpers.c)
/// $079100: andi.b #$ee, ccr
pub fn i_079100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79104;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079104: rts
pub fn i_079104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79106;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PathScript_OpMoveLen_079106  ($079106, 110 B, props/fx_explosions_platforms_crew_076000.s)
/// $079106: cmpi.w #$0, d5
pub fn i_079106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7910A;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $07910A: beq.w $79174
pub fn i_07910a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7910E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79174; return 10; }
    8
}
/// $07910E: move.w $2(a0), d0
pub fn i_07910e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79112;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079112: swap d0
pub fn i_079112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79114;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079114: andi.l #$ffff0000, d0
pub fn i_079114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7911A;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF0000) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $07911A: move.l $94(a6), d1
pub fn i_07911a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7911E;
    cpu.ir = 0x222E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07911E: andi.l #$ffffff00, d1
pub fn i_07911e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79124;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFFFF00) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $079124: sub.l d1, d0
pub fn i_079124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79126;
    cpu.ir = 0x9081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $079126: cmpi.l #$1000000, d0
pub fn i_079126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7912C;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x1000000, Size::Long, false);
    8
}
/// $07912C: bcc.w $7915c
pub fn i_07912c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79130;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7915C; return 10; }
    8
}
/// $079130: lsr.l #$8, d0
pub fn i_079130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79132;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $079132: cmp.w d0, d5
pub fn i_079132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79134;
    cpu.ir = 0xBA40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $079134: bcs.w $7915c
pub fn i_079134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79138;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7915C; return 10; }
    8
}
/// $079138: sub.w d0, d5
pub fn i_079138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7913A;
    cpu.ir = 0x9A40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07913A: clr.b $96(a6)
pub fn i_07913a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7913E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07913E: clr.w $94(a6)
pub fn i_07913e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79142;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $079142: move.b $34(a6), d1
pub fn i_079142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79146;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079146: movem.l d5/a0, -(a7)
pub fn i_079146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7914A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $07914A: jsr $791e0(pc)
pub fn i_07914a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7914E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7914E);
    cpu.pc = 0x791E0;
    16
}
/// $07914E: movem.l (a7)+, d5/a0
pub fn i_07914e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79152;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $079152: addq.l #$4, a0
pub fn i_079152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79154;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $079154: move.l a0, $90(a6)
pub fn i_079154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79158;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $079158: bra.w $78f98
pub fn i_079158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7915C;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
/// $07915C: move.w d5, d0
pub fn i_07915c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7915E;
    cpu.ir = 0x3005;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07915E: andi.l #$ffff, d0
pub fn i_07915e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79164;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $079164: lsl.l #$8, d0
pub fn i_079164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79166;
    cpu.ir = 0xE188;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[0] = r;
    22
}
/// $079166: add.l d0, $94(a6)
pub fn i_079166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7916A;
    cpu.ir = 0xD1AE;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $07916A: move.w d5, d0
pub fn i_07916a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7916C;
    cpu.ir = 0x3005;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07916C: move.b $34(a6), d1
pub fn i_07916c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79170;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079170: jsr $791e0(pc)
pub fn i_079170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79174;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79174);
    cpu.pc = 0x791E0;
    16
}
// ===== ClearXN_079174  ($079174, 6 B, thunks/ccr_helpers.c)
/// $079174: andi.b #$ee, ccr
pub fn i_079174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79178;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079178: rts
pub fn i_079178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7917A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PathScript_OpArc_07917a  ($07917A, 102 B, props/fx_explosions_platforms_crew_076000.s)
/// $07917A: cmpi.w #$0, d5
pub fn i_07917a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7917E;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $07917E: beq.b $79174
pub fn i_07917e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79180;
    cpu.ir = 0x67F4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79174; return 10; }
    8
}
/// $079180: move.w $4(a0), d0
pub fn i_079180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79184;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079184: swap d0
pub fn i_079184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79186;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079186: andi.l #$ffff0000, d0
pub fn i_079186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7918C;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF0000) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $07918C: lsr.l #$8, d0
pub fn i_07918c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7918E;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $07918E: move.l $94(a6), d1
pub fn i_07918e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79192;
    cpu.ir = 0x222E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $079192: andi.l #$ffffff00, d1
pub fn i_079192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79198;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFFFF00) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $079198: sub.l d1, d0
pub fn i_079198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7919A;
    cpu.ir = 0x9081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $07919A: cmpi.l #$1000000, d0
pub fn i_07919a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791A0;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x1000000, Size::Long, false);
    8
}
/// $0791A0: bcc.b $7915c
pub fn i_0791a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791A2;
    cpu.ir = 0x64BA;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7915C; return 10; }
    8
}
/// $0791A2: lsr.l #$8, d0
pub fn i_0791a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791A4;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $0791A4: cmp.w d0, d5
pub fn i_0791a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791A6;
    cpu.ir = 0xBA40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0791A6: bcs.b $7915c
pub fn i_0791a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791A8;
    cpu.ir = 0x65B4;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7915C; return 10; }
    8
}
/// $0791A8: sub.w d0, d5
pub fn i_0791a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791AA;
    cpu.ir = 0x9A40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0791AA: clr.b $96(a6)
pub fn i_0791aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791AE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0791AE: clr.w $94(a6)
pub fn i_0791ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791B2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0791B2: move.b $34(a6), d1
pub fn i_0791b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791B6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0791B6: movem.l d5/a0, -(a7)
pub fn i_0791b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791BA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0791BA: jsr $791e0(pc)
pub fn i_0791ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x791BE);
    cpu.pc = 0x791E0;
    16
}
/// $0791BE: movem.l (a7)+, d5/a0
pub fn i_0791be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791C2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0791C2: move.b $1(a0), d0
pub fn i_0791c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791C6;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0791C6: add.b d0, $34(a6)
pub fn i_0791c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791CA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0791CA: move.b $34(a6), d1
pub fn i_0791ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791CE;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0791CE: cmp.b $2(a0), d1
pub fn i_0791ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791D2;
    cpu.ir = 0xB228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0791D2: bne.w $791dc
pub fn i_0791d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x791DC; return 10; }
    8
}
/// $0791D6: addq.l #$6, a0
pub fn i_0791d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791D8;
    cpu.ir = 0x5C88;
    cpu.a[0] = cpu.a[0].wrapping_add(6);
    8
}
/// $0791D8: move.l a0, $90(a6)
pub fn i_0791d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791DC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0791DC: bra.w $78f98
pub fn i_0791dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x78F98;
    10
}
// ===== PathScript_IntegratePolar_0791e0  ($0791E0, 106 B, props/fx_explosions_platforms_crew_076000.s)
/// $0791E0: lea.l $2c072c.l, a2
pub fn i_0791e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791E6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $0791E6: lea.l $2c07ac.l, a3
pub fn i_0791e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791EC;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x2C07AC;
    20
}
/// $0791EC: andi.w #$ff, d1
pub fn i_0791ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791F0;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0791F0: add.w d1, d1
pub fn i_0791f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791F2;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0791F2: move.w (a2, d1.w), d2
pub fn i_0791f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791F6;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0791F6: move.w (a3, d1.w), d3
pub fn i_0791f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791FA;
    cpu.ir = 0x3633;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0791FA: muls.w d0, d2
pub fn i_0791fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791FC;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0791FC: muls.w d0, d3
pub fn i_0791fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x791FE;
    cpu.ir = 0xC7C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[3] as u16 as i16 as i32)) as u32;
    cpu.d[3] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0791FE: moveq #$7f, d1
pub fn i_0791fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79200;
    cpu.ir = 0x727F;
    cpu.d[1] = 0x7F;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079200: cmpi.l #$0, d2
pub fn i_079200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79206;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Long, false);
    8
}
/// $079206: bge.w $79210
pub fn i_079206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7920A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x79210; return 10; }
    8
}
/// $07920A: move.l #$80, d1
pub fn i_07920a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79210;
    cpu.ir = 0x223C;
    let v1: u32 = 0x80;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079210: add.l d1, d2
pub fn i_079210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79212;
    cpu.ir = 0xD481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $079212: moveq #$7f, d1
pub fn i_079212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79214;
    cpu.ir = 0x727F;
    cpu.d[1] = 0x7F;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079214: cmpi.l #$0, d3
pub fn i_079214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7921A;
    cpu.ir = 0xC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Long, false);
    8
}
/// $07921A: bge.w $79224
pub fn i_07921a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7921E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x79224; return 10; }
    8
}
/// $07921E: move.l #$80, d1
pub fn i_07921e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79224;
    cpu.ir = 0x223C;
    let v1: u32 = 0x80;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079224: add.l d1, d3
pub fn i_079224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79226;
    cpu.ir = 0xD681;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[3] = r;
    4
}
/// $079226: move.l d2, d1
pub fn i_079226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79228;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079228: swap d2
pub fn i_079228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7922A;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07922A: asr.w #$8, d1
pub fn i_07922a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7922C;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07922C: move.w $24(a6), d0
pub fn i_07922c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79230;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079230: add.b d1, $27(a6)
pub fn i_079230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79234;
    cpu.ir = 0xD32E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $079234: addx.w d2, d0
pub fn i_079234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79236;
    cpu.ir = 0xD142;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[2] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079236: move.w d0, $24(a6)
pub fn i_079236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7923A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07923A: move.l d3, d1
pub fn i_07923a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7923C;
    cpu.ir = 0x2203;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07923C: swap d3
pub fn i_07923c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7923E;
    cpu.ir = 0x4843;
    cpu.d[3] = cpu.d[3].rotate_left(16);
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07923E: asr.w #$8, d1
pub fn i_07923e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79240;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $079240: move.w $22(a6), d0
pub fn i_079240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79244;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079244: add.b d1, $26(a6)
pub fn i_079244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79248;
    cpu.ir = 0xD32E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $079248: addx.w d3, d0
pub fn i_079248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7924A;
    cpu.ir = 0xD143;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[3] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
// ===== SetTaskW_07924a  ($07924A, 6 B, kernel/task_setters_w.c)
/// $07924A: move.w d0, $22(a6)
pub fn i_07924a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7924E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07924E: rts
pub fn i_07924e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79250;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_RetGate_079250  ($079250, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079250: movea.l $8(a6), a1
pub fn i_079250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79254;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079254: move.b $10(a6), d0
pub fn i_079254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79258;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079258: cmp.b $10(a1), d0
pub fn i_079258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7925C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07925C: bcs.w $79266
pub fn i_07925c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79260;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79266; return 10; }
    8
}
// ===== ClearXN_079260  ($079260, 6 B, thunks/ccr_helpers.c)
/// $079260: andi.b #$ee, ccr
pub fn i_079260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79264;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079264: rts
pub fn i_079264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79266;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_079266  ($079266, 6 B, thunks/ccr_helpers.c)
/// $079266: ori.b #$11, ccr
pub fn i_079266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7926A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07926A: rts
pub fn i_07926a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7926C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_RecordBest_07926c  ($07926C, 42 B, props/fx_explosions_platforms_crew_076000.s)
/// $07926C: move.b $98(a6), d0
pub fn i_07926c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79270;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079270: andi.w #$ff, d0
pub fn i_079270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79274;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079274: cmpi.w #$63, d0
pub fn i_079274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79278;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x63, Size::Word, false);
    8
}
/// $079278: ble.w $79280
pub fn i_079278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7927C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x79280; return 10; }
    8
}
/// $07927C: move.w #$63, d0
pub fn i_07927c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79280;
    cpu.ir = 0x303C;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079280: cmp.w $106e92.l, d0
pub fn i_079280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79286;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106E92) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $079286: ble.w $79290
pub fn i_079286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7928A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x79290; return 10; }
    8
}
/// $07928A: move.w d0, $106e92.l
pub fn i_07928a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79290;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106E92, v1 as u16);
    16
}
/// $079290: jmp $518.l
pub fn i_079290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79296;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== AutoDemo_Rts_079296  ($079296, 2 B, props/fx_explosions_platforms_crew_076000.s)
/// $079296: rts
pub fn i_079296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79298;
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
// ===== AutoDemo_Stop_079326  ($079326, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $079326: clr.b $10e39d.l
pub fn i_079326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7932C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39D) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39D, r2 as u8);
    16
}
/// $07932C: clr.b $10e39c.l
pub fn i_07932c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79332;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $079332: jmp $518.l
pub fn i_079332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79338;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079338: rts
pub fn i_079338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7933A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_Level4Script_07933a  ($07933A, 258 B, props/fx_explosions_platforms_crew_076000.s)
/// $07933A: move.w #$14, $70(a6)
pub fn i_07933a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79340;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079340: clr.b d4
pub fn i_079340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79342;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $079342: move.b $10e2e4.l, $10e2e3.l
pub fn i_079342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7934C;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $07934C: move.b $10e2ea.l, $10e2e9.l
pub fn i_07934c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79356;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $079356: move.b d4, $10e2e3.l
pub fn i_079356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7935C;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $07935C: move.b d4, $10e2e9.l
pub fn i_07935c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79362;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $079362: move.b d4, $10e2e4.l
pub fn i_079362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79368;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $079368: move.b d4, $10e2ea.l
pub fn i_079368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7936E;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $07936E: move.b d4, $10e2e5.l
pub fn i_07936e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79374;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $079374: move.b d4, $10e2eb.l
pub fn i_079374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7937A;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $07937A: move.b d4, $10e2e6.l
pub fn i_07937a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79380;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $079380: move.b d4, $10e2ec.l
pub fn i_079380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79386;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $079386: lea.l $7938c(pc), a1
pub fn i_079386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7938A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7938C;
    16
}
/// $07938A: move.l a1, (a6)
pub fn i_07938a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7938C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07938C: cmpi.b #$4, $10e39d.l
pub fn i_07938c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79394;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39D) as u32;
    sub_with_flags(cpu, v2, 0x4, Size::Byte, false);
    20
}
/// $079394: bne.b $79326
pub fn i_079394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79396;
    cpu.ir = 0x6690;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79326; return 10; }
    8
}
/// $079396: cmpi.b #$0, $10e39c.l
pub fn i_079396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7939E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39C) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $07939E: beq.b $79326
pub fn i_07939e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793A0;
    cpu.ir = 0x6786;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79326; return 10; }
    8
}
/// $0793A0: clr.b d4
pub fn i_0793a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793A2;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0793A2: move.b $10e2e4.l, $10e2e3.l
pub fn i_0793a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793AC;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $0793AC: move.b $10e2ea.l, $10e2e9.l
pub fn i_0793ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793B6;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $0793B6: move.b d4, $10e2e3.l
pub fn i_0793b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793BC;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $0793BC: move.b d4, $10e2e9.l
pub fn i_0793bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793C2;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $0793C2: move.b d4, $10e2e4.l
pub fn i_0793c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793C8;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $0793C8: move.b d4, $10e2ea.l
pub fn i_0793c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793CE;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $0793CE: move.b d4, $10e2e5.l
pub fn i_0793ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793D4;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $0793D4: move.b d4, $10e2eb.l
pub fn i_0793d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793DA;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $0793DA: move.b d4, $10e2e6.l
pub fn i_0793da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793E0;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $0793E0: move.b d4, $10e2ec.l
pub fn i_0793e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793E6;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $0793E6: clr.w d0
pub fn i_0793e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793E8;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0793E8: jsr $5e3a2.l
pub fn i_0793e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x793EE);
    cpu.pc = 0x5E3A2;
    16
}
/// $0793EE: bcc.w $79404
pub fn i_0793ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79404; return 10; }
    8
}
/// $0793F2: cmpi.w #$c0, $22(a0)
pub fn i_0793f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793F8;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Word, false);
    16
}
/// $0793F8: bgt.w $79404
pub fn i_0793f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x793FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x79404; return 10; }
    8
}
/// $0793FC: ori.b #$8, $10e2e4.l
pub fn i_0793fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79404;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E4) as u32;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, r2 as u8);
    20
}
/// $079404: move.w #$1, d0
pub fn i_079404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79408;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079408: jsr $5e3a2.l
pub fn i_079408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7940E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7940E);
    cpu.pc = 0x5E3A2;
    16
}
/// $07940E: bcc.w $79424
pub fn i_07940e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79412;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79424; return 10; }
    8
}
/// $079412: cmpi.w #$c0, $22(a0)
pub fn i_079412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79418;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Word, false);
    16
}
/// $079418: bgt.w $79424
pub fn i_079418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7941C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x79424; return 10; }
    8
}
/// $07941C: ori.b #$8, $10e2ea.l
pub fn i_07941c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79424;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EA) as u32;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, r2 as u8);
    20
}
/// $079424: tst.w $106f5e.l
pub fn i_079424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7942A;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $07942A: beq.w $79442
pub fn i_07942a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7942E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79442; return 10; }
    8
}
/// $07942E: subq.w #$1, $70(a6)
pub fn i_07942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79432;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $079432: cmpi.w #$0, $70(a6)
pub fn i_079432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79438;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $079438: bgt.w $79442
pub fn i_079438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7943C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x79442; return 10; }
    8
}
// ===== SetTaskHandler_07943c  ($07943C, 8 B, kernel/task_handlers.c)
/// $07943C: lea.l $79326(pc), a1
pub fn i_07943c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79440;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79326;
    16
}
/// $079440: move.l a1, (a6)
pub fn i_079440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79442;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079442: rts
pub fn i_079442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79444;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_Level3Script_079444  ($079444, 102 B, props/fx_explosions_platforms_crew_076000.s)
/// $079444: clr.b d4
pub fn i_079444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79446;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $079446: move.b $10e2e4.l, $10e2e3.l
pub fn i_079446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79450;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $079450: move.b $10e2ea.l, $10e2e9.l
pub fn i_079450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7945A;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $07945A: move.b d4, $10e2e3.l
pub fn i_07945a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79460;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $079460: move.b d4, $10e2e9.l
pub fn i_079460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79466;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $079466: move.b d4, $10e2e4.l
pub fn i_079466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7946C;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $07946C: move.b d4, $10e2ea.l
pub fn i_07946c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79472;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $079472: move.b d4, $10e2e5.l
pub fn i_079472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79478;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $079478: move.b d4, $10e2eb.l
pub fn i_079478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7947E;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $07947E: move.b d4, $10e2e6.l
pub fn i_07947e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79484;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $079484: move.b d4, $10e2ec.l
pub fn i_079484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7948A;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $07948A: lea.l $79490(pc), a1
pub fn i_07948a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7948E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79490;
    16
}
/// $07948E: move.l a1, (a6)
pub fn i_07948e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79490;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079490: cmpi.b #$3, $10e39d.l
pub fn i_079490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79498;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39D) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $079498: bne.w $79326
pub fn i_079498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7949C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79326; return 10; }
    8
}
/// $07949C: cmpi.b #$0, $10e39c.l
pub fn i_07949c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794A4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39C) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $0794A4: beq.w $79326
pub fn i_0794a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79326; return 10; }
    8
}
/// $0794A8: rts
pub fn i_0794a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_Level1Script_0794aa  ($0794AA, 208 B, props/fx_explosions_platforms_crew_076000.s)
/// $0794AA: lea.l $794b0(pc), a1
pub fn i_0794aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x794B0;
    16
}
/// $0794AE: move.l a1, (a6)
pub fn i_0794ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0794B0: clr.b d4
pub fn i_0794b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794B2;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0794B2: move.b $10e2e4.l, $10e2e3.l
pub fn i_0794b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794BC;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $0794BC: move.b $10e2ea.l, $10e2e9.l
pub fn i_0794bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794C6;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $0794C6: move.b d4, $10e2e3.l
pub fn i_0794c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794CC;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $0794CC: move.b d4, $10e2e9.l
pub fn i_0794cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794D2;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $0794D2: move.b d4, $10e2e4.l
pub fn i_0794d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794D8;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $0794D8: move.b d4, $10e2ea.l
pub fn i_0794d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794DE;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $0794DE: move.b d4, $10e2e5.l
pub fn i_0794de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794E4;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $0794E4: move.b d4, $10e2eb.l
pub fn i_0794e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794EA;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $0794EA: move.b d4, $10e2e6.l
pub fn i_0794ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794F0;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $0794F0: move.b d4, $10e2ec.l
pub fn i_0794f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794F6;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $0794F6: cmpi.b #$1, $10e39d.l
pub fn i_0794f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794FE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39D) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0794FE: bne.w $79326
pub fn i_0794fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79502;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79326; return 10; }
    8
}
/// $079502: cmpi.b #$0, $10e39c.l
pub fn i_079502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7950A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39C) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $07950A: beq.w $79326
pub fn i_07950a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7950E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79326; return 10; }
    8
}
/// $07950E: clr.w d0
pub fn i_07950e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79510;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079510: jsr $5e3a2.l
pub fn i_079510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79516);
    cpu.pc = 0x5E3A2;
    16
}
/// $079516: bcc.w $79542
pub fn i_079516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7951A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79542; return 10; }
    8
}
/// $07951A: cmpi.w #$78, $22(a0)
pub fn i_07951a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79520;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x78, Size::Word, false);
    16
}
/// $079520: blt.w $7953a
pub fn i_079520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79524;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7953A; return 10; }
    8
}
/// $079524: cmpi.w #$88, $22(a0)
pub fn i_079524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7952A;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Word, false);
    16
}
/// $07952A: ble.w $79542
pub fn i_07952a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7952E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x79542; return 10; }
    8
}
/// $07952E: ori.b #$4, $10e2e4.l
pub fn i_07952e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79536;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E4) as u32;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, r2 as u8);
    20
}
/// $079536: bra.w $79542
pub fn i_079536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7953A;
    cpu.ir = 0x6000;
    cpu.pc = 0x79542;
    10
}
/// $07953A: ori.b #$8, $10e2e4.l
pub fn i_07953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79542;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E4) as u32;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, r2 as u8);
    20
}
/// $079542: move.w #$1, d0
pub fn i_079542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79546;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079546: jsr $5e3a2.l
pub fn i_079546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7954C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7954C);
    cpu.pc = 0x5E3A2;
    16
}
/// $07954C: bcc.w $79578
pub fn i_07954c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79550;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79578; return 10; }
    8
}
/// $079550: cmpi.w #$40, $22(a0)
pub fn i_079550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79556;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Word, false);
    16
}
/// $079556: blt.w $79570
pub fn i_079556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7955A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x79570; return 10; }
    8
}
/// $07955A: cmpi.w #$50, $22(a0)
pub fn i_07955a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79560;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x50, Size::Word, false);
    16
}
/// $079560: ble.w $79578
pub fn i_079560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79564;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x79578; return 10; }
    8
}
/// $079564: ori.b #$4, $10e2ea.l
pub fn i_079564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7956C;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EA) as u32;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, r2 as u8);
    20
}
/// $07956C: bra.w $79578
pub fn i_07956c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79570;
    cpu.ir = 0x6000;
    cpu.pc = 0x79578;
    10
}
/// $079570: ori.b #$8, $10e2ea.l
pub fn i_079570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79578;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EA) as u32;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, r2 as u8);
    20
}
/// $079578: rts
pub fn i_079578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7957A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_Level2Script_07957a  ($07957A, 176 B, props/fx_explosions_platforms_crew_076000.s)
/// $07957A: clr.b $80(a6)
pub fn i_07957a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7957E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07957E: clr.b $81(a6)
pub fn i_07957e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79582;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $079582: move.w #$12c, $70(a6)
pub fn i_079582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79588;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079588: move.w #$14, $72(a6)
pub fn i_079588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7958E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07958E: lea.l $79594(pc), a1
pub fn i_07958e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79592;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79594;
    16
}
/// $079592: move.l a1, (a6)
pub fn i_079592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79594;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079594: cmpi.b #$2, $10e39d.l
pub fn i_079594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7959C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39D) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $07959C: bne.w $79326
pub fn i_07959c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79326; return 10; }
    8
}
/// $0795A0: cmpi.b #$0, $10e39c.l
pub fn i_0795a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795A8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39C) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $0795A8: beq.w $79326
pub fn i_0795a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79326; return 10; }
    8
}
/// $0795AC: clr.b d4
pub fn i_0795ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795AE;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0795AE: move.b $10e2e4.l, $10e2e3.l
pub fn i_0795ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795B8;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $0795B8: move.b $10e2ea.l, $10e2e9.l
pub fn i_0795b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795C2;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $0795C2: move.b d4, $10e2e3.l
pub fn i_0795c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795C8;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $0795C8: move.b d4, $10e2e9.l
pub fn i_0795c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795CE;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $0795CE: move.b d4, $10e2e4.l
pub fn i_0795ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795D4;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $0795D4: move.b d4, $10e2ea.l
pub fn i_0795d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795DA;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $0795DA: move.b d4, $10e2e5.l
pub fn i_0795da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795E0;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $0795E0: move.b d4, $10e2eb.l
pub fn i_0795e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795E6;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $0795E6: move.b d4, $10e2e6.l
pub fn i_0795e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795EC;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $0795EC: move.b d4, $10e2ec.l
pub fn i_0795ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795F2;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $0795F2: cmpi.w #$0, $72(a6)
pub fn i_0795f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0795F8: bgt.w $79614
pub fn i_0795f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x795FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x79614; return 10; }
    8
}
/// $0795FC: ori.b #$8, $10e2e4.l
pub fn i_0795fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79604;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E4) as u32;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, r2 as u8);
    20
}
/// $079604: ori.b #$8, $10e2ea.l
pub fn i_079604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7960C;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EA) as u32;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, r2 as u8);
    20
}
/// $07960C: jsr $796f6(pc)
pub fn i_07960c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79610;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79610);
    cpu.pc = 0x796F6;
    16
}
/// $079610: bra.w $79628
pub fn i_079610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79614;
    cpu.ir = 0x6000;
    cpu.pc = 0x79628;
    10
}
/// $079614: subq.w #$1, $72(a6)
pub fn i_079614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79618;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $079618: ori.b #$4, $10e2e4.l
pub fn i_079618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79620;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E4) as u32;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, r2 as u8);
    20
}
/// $079620: ori.b #$4, $10e2ea.l
pub fn i_079620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79628;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EA) as u32;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, r2 as u8);
    20
}
/// $079628: rts
pub fn i_079628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7962A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0007962A  ($07962A, 2 B, thunks/trivial_rts.c)
/// $07962A: rts
pub fn i_07962a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7962C;
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
// ===== AutoDemo_JumpIfBlocked_0796f6  ($0796F6, 216 B, props/fx_explosions_platforms_crew_076000.s)
/// $0796F6: tst.b $80(a6)
pub fn i_0796f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796FA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0796FA: bne.w $79760
pub fn i_0796fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79760; return 10; }
    8
}
/// $0796FE: clr.w d0
pub fn i_0796fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79700;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079700: jsr $5e3a2.l
pub fn i_079700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79706;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79706);
    cpu.pc = 0x5E3A2;
    16
}
/// $079706: bcc.w $79760
pub fn i_079706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7970A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79760; return 10; }
    8
}
/// $07970A: movem.l a6, -(a7)
pub fn i_07970a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7970E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $07970E: move.l a0, $5c(a6)
pub fn i_07970e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79712;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $079712: movea.l a0, a6
pub fn i_079712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79714;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $079714: jsr $2a8c0.l
pub fn i_079714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7971A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7971A);
    cpu.pc = 0x2A8C0;
    16
}
/// $07971A: movem.l (a7)+, a6
pub fn i_07971a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7971E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07971E: bcs.w $79760
pub fn i_07971e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79722;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79760; return 10; }
    8
}
/// $079722: movea.l $5c(a6), a0
pub fn i_079722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79726;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079726: move.w $22(a0), d1
pub fn i_079726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7972A;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07972A: move.w $24(a0), d2
pub fn i_07972a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7972E;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07972E: addi.w #$18, d1
pub fn i_07972e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79732;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079732: subq.w #$8, d2
pub fn i_079732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79734;
    cpu.ir = 0x5142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079734: jsr $27db2.l
pub fn i_079734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7973A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7973A);
    cpu.pc = 0x27DB2;
    16
}
/// $07973A: cmpi.b #$40, d7
pub fn i_07973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7973E;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $07973E: beq.w $79760
pub fn i_07973e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79742;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79760; return 10; }
    8
}
/// $079742: move.b #$1, $80(a6)
pub fn i_079742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79748;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079748: ori.b #$1, $10e2e4.l
pub fn i_079748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79750;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E4) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, r2 as u8);
    20
}
/// $079750: ori.b #$1, $10e2e5.l
pub fn i_079750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79758;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E5) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, r2 as u8);
    20
}
/// $079758: ori.b #$1, $10e2e6.l
pub fn i_079758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79760;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E6) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, r2 as u8);
    20
}
/// $079760: tst.b $81(a6)
pub fn i_079760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79764;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079764: bne.w $797cc
pub fn i_079764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79768;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x797CC; return 10; }
    8
}
/// $079768: move.w #$1, d0
pub fn i_079768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7976C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07976C: jsr $5e3a2.l
pub fn i_07976c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79772;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79772);
    cpu.pc = 0x5E3A2;
    16
}
/// $079772: bcc.w $797cc
pub fn i_079772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79776;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x797CC; return 10; }
    8
}
/// $079776: movem.l a6, -(a7)
pub fn i_079776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7977A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $07977A: move.l a0, $5c(a6)
pub fn i_07977a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7977E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07977E: movea.l a0, a6
pub fn i_07977e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79780;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $079780: jsr $2a8c0.l
pub fn i_079780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79786);
    cpu.pc = 0x2A8C0;
    16
}
/// $079786: movem.l (a7)+, a6
pub fn i_079786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7978A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07978A: bcs.w $797cc
pub fn i_07978a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7978E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x797CC; return 10; }
    8
}
/// $07978E: movea.l $5c(a6), a0
pub fn i_07978e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79792;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079792: move.w $22(a0), d1
pub fn i_079792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79796;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079796: move.w $24(a0), d2
pub fn i_079796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7979A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07979A: addi.w #$18, d1
pub fn i_07979a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7979E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07979E: subq.w #$8, d2
pub fn i_07979e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797A0;
    cpu.ir = 0x5142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0797A0: jsr $27db2.l
pub fn i_0797a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x797A6);
    cpu.pc = 0x27DB2;
    16
}
/// $0797A6: cmpi.b #$40, d7
pub fn i_0797a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797AA;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $0797AA: beq.w $797cc
pub fn i_0797aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x797CC; return 10; }
    8
}
/// $0797AE: move.b #$1, $81(a6)
pub fn i_0797ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0797B4: ori.b #$1, $10e2ea.l
pub fn i_0797b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797BC;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EA) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, r2 as u8);
    20
}
/// $0797BC: ori.b #$1, $10e2eb.l
pub fn i_0797bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797C4;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EB) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, r2 as u8);
    20
}
/// $0797C4: ori.b #$1, $10e2ec.l
pub fn i_0797c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797CC;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EC) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, r2 as u8);
    20
}
/// $0797CC: rts
pub fn i_0797cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_DebugWarpKeys_0797ce  ($0797CE, 90 B, props/fx_explosions_platforms_crew_076000.s)
/// $0797CE: lea.l $10e200.l, a0
pub fn i_0797ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E200;
    20
}
/// $0797D4: move.b $2(a0), d0
pub fn i_0797d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797D8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0797D8: move.b $3(a0), d1
pub fn i_0797d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797DC;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0797DC: move.b d1, d2
pub fn i_0797dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797DE;
    cpu.ir = 0x1401;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0797DE: andi.b #$80, d0
pub fn i_0797de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797E2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0797E2: andi.b #$40, d1
pub fn i_0797e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797E6;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0797E6: andi.b #$20, d2
pub fn i_0797e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797EA;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0797EA: tst.b d0
pub fn i_0797ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797EC;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0797EC: beq.w $79826
pub fn i_0797ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79826; return 10; }
    8
}
/// $0797F0: tst.b d1
pub fn i_0797f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797F2;
    cpu.ir = 0x4A01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0797F2: beq.w $7981a
pub fn i_0797f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7981A; return 10; }
    8
}
/// $0797F6: move.w #$a0, $22(a6)
pub fn i_0797f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0797FC: move.w #$180, $24(a6)
pub fn i_0797fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79802;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079802: move.l #$2df456, $4c(a6)
pub fn i_079802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7980A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DF456;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07980A: jsr $283ca.l
pub fn i_07980a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79810;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79810);
    cpu.pc = 0x283CA;
    16
}
/// $079810: jsr $283d8.l
pub fn i_079810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79816);
    cpu.pc = 0x283D8;
    16
}
/// $079816: bra.w $79826
pub fn i_079816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7981A;
    cpu.ir = 0x6000;
    cpu.pc = 0x79826;
    10
}
/// $07981A: tst.b d2
pub fn i_07981a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7981C;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $07981C: beq.w $79826
pub fn i_07981c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79820;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79826; return 10; }
    8
}
/// $079820: clr.w $106e92.l
pub fn i_079820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79826;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106E92) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106E92, r2 as u16);
    16
}
/// $079826: rts
pub fn i_079826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79828;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_RetGateB_079828  ($079828, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079828: movea.l $8(a6), a1
pub fn i_079828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7982C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07982C: move.b $10(a6), d0
pub fn i_07982c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79830;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079830: cmp.b $10(a1), d0
pub fn i_079830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79834;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079834: bcs.w $7983e
pub fn i_079834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79838;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7983E; return 10; }
    8
}
// ===== ClearXN_079838  ($079838, 6 B, thunks/ccr_helpers.c)
/// $079838: andi.b #$ee, ccr
pub fn i_079838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7983C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07983C: rts
pub fn i_07983c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7983E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07983e  ($07983E, 6 B, thunks/ccr_helpers.c)
/// $07983E: ori.b #$11, ccr
pub fn i_07983e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79842;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079842: rts
pub fn i_079842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79844;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_TestPlayersAndWeapons_079844  ($079844, 98 B, props/fx_explosions_platforms_crew_076000.s)
/// $079844: move.l a2, d0
pub fn i_079844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79846;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079846: cmpi.l #$ffffffff, d0
pub fn i_079846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7984C;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $07984C: beq.w $798aa
pub fn i_07984c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79850;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x798AA; return 10; }
    8
}
/// $079850: movem.l a2, -(a7)
pub fn i_079850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79854;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $079854: clr.w d0
pub fn i_079854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79856;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079856: jsr $5e3a2.l
pub fn i_079856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7985C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7985C);
    cpu.pc = 0x5E3A2;
    16
}
/// $07985C: movem.l (a7)+, a2
pub fn i_07985c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79860;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $079860: bcc.w $7987c
pub fn i_079860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79864;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7987C; return 10; }
    8
}
/// $079864: movea.l $74(a0), a1
pub fn i_079864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79868;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079868: jsr $798ac(pc)
pub fn i_079868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7986C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7986C);
    cpu.pc = 0x798AC;
    16
}
/// $07986C: movea.l $7c(a0), a1
pub fn i_07986c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79870;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079870: jsr $798ac(pc)
pub fn i_079870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79874;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79874);
    cpu.pc = 0x798AC;
    16
}
/// $079874: movea.l $78(a0), a1
pub fn i_079874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79878;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079878: jsr $798ac(pc)
pub fn i_079878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7987C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7987C);
    cpu.pc = 0x798AC;
    16
}
/// $07987C: movem.l a2, -(a7)
pub fn i_07987c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79880;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $079880: move.w #$1, d0
pub fn i_079880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79884;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079884: jsr $5e3a2.l
pub fn i_079884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7988A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7988A);
    cpu.pc = 0x5E3A2;
    16
}
/// $07988A: movem.l (a7)+, a2
pub fn i_07988a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7988E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07988E: bcc.w $798aa
pub fn i_07988e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79892;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x798AA; return 10; }
    8
}
/// $079892: movea.l $74(a0), a1
pub fn i_079892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79896;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079896: jsr $798ac(pc)
pub fn i_079896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7989A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7989A);
    cpu.pc = 0x798AC;
    16
}
/// $07989A: movea.l $7c(a0), a1
pub fn i_07989a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7989E;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07989E: jsr $798ac(pc)
pub fn i_07989e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x798A2);
    cpu.pc = 0x798AC;
    16
}
/// $0798A2: movea.l $78(a0), a1
pub fn i_0798a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798A6;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
// ===== JsrPcThunk_0798a6  ($0798A6, 6 B, thunks/jsr_pc_thunks.c)
/// $0798A6: jsr $798ac(pc)
pub fn i_0798a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x798AA);
    cpu.pc = 0x798AC;
    16
}
/// $0798AA: rts
pub fn i_0798aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CheckBoxOverlapWithSelector_0798AC  ($0798AC, 164 B, entity/check_box_overlap_selector_0798ac.s)
/// $0798AC: move.l a1, d0
pub fn i_0798ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798AE;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0798AE: cmpi.l #$ffffffff, d0
pub fn i_0798ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798B4;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $0798B4: beq.w $7994e
pub fn i_0798b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798B8: move.w (a2), d0
pub fn i_0798b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798BA;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0798BA: move.w $2(a2), d1
pub fn i_0798ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798BE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798BE: move.w $22(a6), d2
pub fn i_0798be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798C2;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798C2: add.w d2, d0
pub fn i_0798c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798C4;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0798C4: add.w d2, d1
pub fn i_0798c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798C6;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0798C6: move.w $22(a1), d2
pub fn i_0798c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798CA;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798CA: cmp.w d0, d2
pub fn i_0798ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798CC;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0798CC: blt.w $7994e
pub fn i_0798cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798D0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798D0: cmp.w d1, d2
pub fn i_0798d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798D2;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0798D2: bgt.w $7994e
pub fn i_0798d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798D6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798D6: move.w $4(a2), d0
pub fn i_0798d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798DA;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798DA: move.w $6(a2), d1
pub fn i_0798da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798DE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798DE: move.w $24(a6), d2
pub fn i_0798de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798E2;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798E2: add.w d2, d0
pub fn i_0798e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798E4;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0798E4: add.w d2, d1
pub fn i_0798e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798E6;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0798E6: move.w $24(a1), d2
pub fn i_0798e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798EA;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798EA: cmp.w d0, d2
pub fn i_0798ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798EC;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0798EC: blt.w $7994e
pub fn i_0798ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798F0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798F0: cmp.w d1, d2
pub fn i_0798f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798F2;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0798F2: bgt.w $7994e
pub fn i_0798f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798F6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798F6: move.w (a2), d0
pub fn i_0798f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798F8;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0798F8: move.w d0, d1
pub fn i_0798f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798FA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0798FA: add.w $22(a6), d0
pub fn i_0798fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798FE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0798FE: sub.w $22(a1), d0
pub fn i_0798fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79902;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $079902: move.w $2(a2), d2
pub fn i_079902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79906;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079906: sub.w d1, d2
pub fn i_079906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79908;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079908: asr.w #$1, d2
pub fn i_079908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7990A;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07990A: cmp.w d2, d0
pub fn i_07990a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7990C;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $07990C: ble.w $7991e
pub fn i_07990c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79910;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7991E; return 10; }
    8
}
/// $079910: move.w (a2), d1
pub fn i_079910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79912;
    cpu.ir = 0x3212;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $079912: neg.w d1
pub fn i_079912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79914;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079914: add.w $22(a6), d1
pub fn i_079914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79918;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $079918: move.w $22(a1), d0
pub fn i_079918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7991C;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07991C: sub.w d1, d0
pub fn i_07991c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7991E;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07991E: neg.w d0
pub fn i_07991e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79920;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079920: move.b $8(a2), d3
pub fn i_079920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79924;
    cpu.ir = 0x162A;
    let ad2 = cpu.a[2].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079924: andi.w #$3, d3
pub fn i_079924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79928;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079928: lsl.w #$2, d3
pub fn i_079928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7992A;
    cpu.ir = 0xE54B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07992A: lea.l $2df4aa.l, a3
pub fn i_07992a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79930;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x2DF4AA;
    20
}
/// $079930: movea.l (a3, d3.w), a3
pub fn i_079930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79934;
    cpu.ir = 0x2673;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $079934: jsr (a3)
pub fn i_079934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79936;
    cpu.ir = 0x4E93;
    cpu.push32(bus, 0x79936);
    cpu.jump_to(cpu.a[3]);
    16
}
/// $079936: add.w $24(a6), d0
pub fn i_079936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7993A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07993A: sub.w $24(a1), d0
pub fn i_07993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7993E;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07993E: cmp.w $6(a2), d0
pub fn i_07993e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79942;
    cpu.ir = 0xB06A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $079942: ble.w $7994a
pub fn i_079942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79946;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7994A; return 10; }
    8
}
/// $079946: move.w $6(a2), d0
pub fn i_079946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7994A;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07994A: move.w d0, $8e(a1)
pub fn i_07994a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7994E;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07994E: rts
pub fn i_07994e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79950;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00079950  ($079950, 2 B, thunks/trivial_rts.c)
/// $079950: rts
pub fn i_079950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79952;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scale_X2_079952  ($079952, 4 B, props/fx_explosions_platforms_crew_076000.s)
/// $079952: add.w d0, d0
pub fn i_079952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79954;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079954: rts
pub fn i_079954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79956;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scale_X1_5_079956  ($079956, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079956: move.w d0, d1
pub fn i_079956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79958;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079958: asr.w #$1, d1
pub fn i_079958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7995A;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07995A: add.w d1, d0
pub fn i_07995a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7995C;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07995C: rts
pub fn i_07995c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7995E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_TestPlayersDefault_07995e  ($07995E, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $07995E: lea.l $2df4ba.l, a2
pub fn i_07995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79964;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2DF4BA;
    20
}
/// $079964: jmp $79844(pc)
pub fn i_079964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79968;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x79844;
    8
}
// ===== Rank_DelayTbl_079968  ($079968, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079968: ori.b #$46, ccr
pub fn i_079968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7996C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x6);
    20
}
/// $07996C: ori.w #$5a, (a0)
pub fn i_07996c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79970;
    cpu.ir = 0x50;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x5A) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
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
// ===== Rank_RetGate_079988  ($079988, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079988: movea.l $8(a6), a1
pub fn i_079988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7998C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07998C: move.b $10(a6), d0
pub fn i_07998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79990;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079990: cmp.b $10(a1), d0
pub fn i_079990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79994;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079994: bcs.w $7999e
pub fn i_079994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79998;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7999E; return 10; }
    8
}
// ===== ClearXN_079998  ($079998, 6 B, thunks/ccr_helpers.c)
/// $079998: andi.b #$ee, ccr
pub fn i_079998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7999C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07999C: rts
pub fn i_07999c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7999E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07999e  ($07999E, 6 B, thunks/ccr_helpers.c)
/// $07999E: ori.b #$11, ccr
pub fn i_07999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799A2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0799A2: rts
pub fn i_0799a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799A4;
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
// ===== Crew_RetGate_079a50  ($079A50, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079A50: movea.l $8(a6), a1
pub fn i_079a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A54;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079A54: move.b $10(a6), d0
pub fn i_079a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A58;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079A58: cmp.b $10(a1), d0
pub fn i_079a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A5C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079A5C: bcs.w $79a66
pub fn i_079a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A60;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79A66; return 10; }
    8
}
// ===== ClearXN_079a60  ($079A60, 6 B, thunks/ccr_helpers.c)
/// $079A60: andi.b #$ee, ccr
pub fn i_079a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A64;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079A64: rts
pub fn i_079a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_079a66  ($079A66, 6 B, thunks/ccr_helpers.c)
/// $079A66: ori.b #$11, ccr
pub fn i_079a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A6A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079A6A: rts
pub fn i_079a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Tmpl125_079a6c  ($079A6C, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $079A6C: move.w #$19, d1
pub fn i_079a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A70;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079A70: jsr $236e.l
pub fn i_079a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A76);
    cpu.pc = 0x236E;
    16
}
/// $079A76: move.w #$1, $66(a6)
pub fn i_079a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079A7C: jsr $267e2.l
pub fn i_079a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A82);
    cpu.pc = 0x267E2;
    16
}
// ===== SetTaskHandler_079a82  ($079A82, 8 B, kernel/task_handlers.c)
/// $079A82: lea.l $79a8a(pc), a1
pub fn i_079a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79A8A;
    16
}
/// $079A86: move.l a1, (a6)
pub fn i_079a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079A88: rts
pub fn i_079a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Jump_Run_079a8a  ($079A8A, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $079A8A: jsr $5e7c0.l
pub fn i_079a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A90);
    cpu.pc = 0x5E7C0;
    16
}
/// $079A90: move.w #$8000, d0
pub fn i_079a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A94;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079A94: jsr $28134.l
pub fn i_079a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A9A);
    cpu.pc = 0x28134;
    16
}
/// $079A9A: andi.w #$ffe3, $38(a6)
pub fn i_079a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AA0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079AA0: ori.w #$c, $38(a6)
pub fn i_079aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AA6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079AA6: lea.l $29c0ae.l, a0
pub fn i_079aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C0AE;
    20
}
/// $079AAC: jsr $28cd4.l
pub fn i_079aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79AB2);
    cpu.pc = 0x28CD4;
    16
}
/// $079AB2: lea.l $79ab8(pc), a1
pub fn i_079ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79AB8;
    16
}
/// $079AB6: move.l a1, (a6)
pub fn i_079ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079AB8: jsr $2783a.l
pub fn i_079ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ABE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79ABE);
    cpu.pc = 0x2783A;
    16
}
/// $079ABE: jsr $28d70.l
pub fn i_079abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79AC4);
    cpu.pc = 0x28D70;
    16
}
/// $079AC4: jsr $2870a.l
pub fn i_079ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ACA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79ACA);
    cpu.pc = 0x2870A;
    16
}
/// $079ACA: bcc.w $79ad4
pub fn i_079aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ACE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79AD4; return 10; }
    8
}
/// $079ACE: lea.l $79ae4(pc), a1
pub fn i_079ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79AE4;
    16
}
/// $079AD2: move.l a1, (a6)
pub fn i_079ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079AD4: jsr $79b4a(pc)
pub fn i_079ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79AD8);
    cpu.pc = 0x79B4A;
    16
}
/// $079AD8: bcc.w $79ae2
pub fn i_079ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ADC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79AE2; return 10; }
    8
}
// ===== SetTaskHandler_079adc  ($079ADC, 8 B, kernel/task_handlers.c)
/// $079ADC: lea.l $79b42(pc), a1
pub fn i_079adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79B42;
    16
}
/// $079AE0: move.l a1, (a6)
pub fn i_079ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079AE2: rts
pub fn i_079ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Land_079ae4  ($079AE4, 54 B, props/fx_explosions_platforms_crew_076000.s)
/// $079AE4: bclr.b #$1, $12(a6)
pub fn i_079ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AEA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079AEA: lea.l $29c1d4.l, a0
pub fn i_079aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C1D4;
    20
}
/// $079AF0: jsr $28cd4.l
pub fn i_079af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79AF6);
    cpu.pc = 0x28CD4;
    16
}
/// $079AF6: lea.l $79afc(pc), a1
pub fn i_079af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79AFC;
    16
}
/// $079AFA: move.l a1, (a6)
pub fn i_079afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079AFC: jsr $2783a.l
pub fn i_079afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B02);
    cpu.pc = 0x2783A;
    16
}
/// $079B02: jsr $28d70.l
pub fn i_079b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B08);
    cpu.pc = 0x28D70;
    16
}
/// $079B08: bcc.w $79b12
pub fn i_079b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79B12; return 10; }
    8
}
/// $079B0C: lea.l $79b22(pc), a1
pub fn i_079b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79B22;
    16
}
/// $079B10: move.l a1, (a6)
pub fn i_079b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079B12: jsr $79b4a(pc)
pub fn i_079b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79B16);
    cpu.pc = 0x79B4A;
    16
}
/// $079B16: bcc.w $79b20
pub fn i_079b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B1A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79B20; return 10; }
    8
}
// ===== SetTaskHandler_079b1a  ($079B1A, 8 B, kernel/task_handlers.c)
/// $079B1A: lea.l $79b42(pc), a1
pub fn i_079b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79B42;
    16
}
/// $079B1E: move.l a1, (a6)
pub fn i_079b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079B20: rts
pub fn i_079b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Flee_079b22  ($079B22, 32 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B22: move.b #$28, $59(a6)
pub fn i_079b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B28;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079B28: lea.l $79b2e(pc), a1
pub fn i_079b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79B2E;
    16
}
/// $079B2C: move.l a1, (a6)
pub fn i_079b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079B2E: jsr $2783a.l
pub fn i_079b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B34);
    cpu.pc = 0x2783A;
    16
}
/// $079B34: jsr $28d70.l
pub fn i_079b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B3A);
    cpu.pc = 0x28D70;
    16
}
/// $079B3A: tst.b $59(a6)
pub fn i_079b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B3E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079B3E: bne.w $79b48
pub fn i_079b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B42;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79B48; return 10; }
    8
}
// ===== Crew_Free_079b42  ($079B42, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B42: jmp $518.l
pub fn i_079b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079B48: rts
pub fn i_079b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_FreeIfOffWorld_079b4a  ($079B4A, 14 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B4A: lea.l $79b64(pc), a0
pub fn i_079b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B4E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x79B64;
    16
}
/// $079B4E: jsr $5dd5c.l
pub fn i_079b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B54);
    cpu.pc = 0x5DD5C;
    16
}
/// $079B54: bcc.w $79b5e
pub fn i_079b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79B5E; return 10; }
    8
}
// ===== SetC_079b58  ($079B58, 6 B, thunks/ccr_helpers.c)
/// $079B58: ori.b #$1, ccr
pub fn i_079b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B5C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $079B5C: rts
pub fn i_079b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_079b5e  ($079B5E, 6 B, thunks/ccr_helpers.c)
/// $079B5E: andi.b #$fe, ccr
pub fn i_079b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B62;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $079B62: rts
pub fn i_079b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hatch_079b6e  ($079B6E, 156 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B6E: jsr $5e9b6.l
pub fn i_079b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B74);
    cpu.pc = 0x5E9B6;
    16
}
/// $079B74: btst.b #$0, d0
pub fn i_079b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B78;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $079B78: beq.w $79b82
pub fn i_079b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B7C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79B82; return 10; }
    8
}
/// $079B7C: jmp $79d9a.l
pub fn i_079b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B82;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x79D9A;
    8
}
/// $079B82: movea.l $c(a6), a0
pub fn i_079b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B86;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079B86: move.w $72(a6), d0
pub fn i_079b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B8A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079B8A: bset.b d0, $70(a0)
pub fn i_079b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B8E;
    cpu.ir = 0x1E8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079B8E: move.w #$19, d1
pub fn i_079b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079B92: jsr $236e.l
pub fn i_079b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B98);
    cpu.pc = 0x236E;
    16
}
/// $079B98: addq.w #$3, $38(a6)
pub fn i_079b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B9C;
    cpu.ir = 0x566E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $079B9C: move.w $38(a6), d0
pub fn i_079b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BA0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079BA0: jsr $28134.l
pub fn i_079ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BA6);
    cpu.pc = 0x28134;
    16
}
/// $079BA6: move.w #$1, $66(a6)
pub fn i_079ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BAC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079BAC: jsr $267e2.l
pub fn i_079bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BB2);
    cpu.pc = 0x267E2;
    16
}
/// $079BB2: lea.l $29c0ae.l, a0
pub fn i_079bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C0AE;
    20
}
/// $079BB8: jsr $28cd4.l
pub fn i_079bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BBE);
    cpu.pc = 0x28CD4;
    16
}
/// $079BBE: lea.l $79bc4(pc), a1
pub fn i_079bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79BC4;
    16
}
/// $079BC2: move.l a1, (a6)
pub fn i_079bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079BC4: jsr $7fdfa.l
pub fn i_079bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BCA);
    cpu.pc = 0x7FDFA;
    16
}
/// $079BCA: jsr $28d70.l
pub fn i_079bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BD0);
    cpu.pc = 0x28D70;
    16
}
/// $079BD0: movea.l $c(a6), a0
pub fn i_079bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079BD4: cmpi.b #$3, $21(a0)
pub fn i_079bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BDA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $079BDA: beq.w $79be8
pub fn i_079bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BDE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79BE8; return 10; }
    8
}
/// $079BDE: cmpi.b #$ff, $20(a0)
pub fn i_079bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BE4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079BE4: bne.w $79bf0
pub fn i_079be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79BF0; return 10; }
    8
}
/// $079BE8: lea.l $77efe.l, a1
pub fn i_079be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BEE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EFE;
    20
}
/// $079BEE: move.l a1, (a6)
pub fn i_079bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079BF0: jsr $2870a.l
pub fn i_079bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BF6);
    cpu.pc = 0x2870A;
    16
}
/// $079BF6: bcc.w $79c00
pub fn i_079bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79C00; return 10; }
    8
}
/// $079BFA: lea.l $79c12(pc), a1
pub fn i_079bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C12;
    16
}
/// $079BFE: move.l a1, (a6)
pub fn i_079bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C00: jsr $7fde0.l
pub fn i_079c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C06);
    cpu.pc = 0x7FDE0;
    16
}
/// $079C06: bcc.w $79c10
pub fn i_079c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79C10; return 10; }
    8
}
// ===== SetTaskHandler_079c0a  ($079C0A, 8 B, kernel/task_handlers.c)
/// $079C0A: lea.l $79c68(pc), a1
pub fn i_079c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C68;
    16
}
/// $079C0E: move.l a1, (a6)
pub fn i_079c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C10: rts
pub fn i_079c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hatch_Land_079c12  ($079C12, 34 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C12: lea.l $29c1d4.l, a0
pub fn i_079c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C1D4;
    20
}
/// $079C18: jsr $28cd4.l
pub fn i_079c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C1E);
    cpu.pc = 0x28CD4;
    16
}
/// $079C1E: lea.l $79c24(pc), a1
pub fn i_079c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C24;
    16
}
/// $079C22: move.l a1, (a6)
pub fn i_079c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C24: jsr $7fdfa.l
pub fn i_079c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C2A);
    cpu.pc = 0x7FDFA;
    16
}
/// $079C2A: jsr $28d70.l
pub fn i_079c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C30);
    cpu.pc = 0x28D70;
    16
}
/// $079C30: bcc.w $79c3a
pub fn i_079c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79C3A; return 10; }
    8
}
// ===== SetTaskHandler_079c34  ($079C34, 8 B, kernel/task_handlers.c)
/// $079C34: lea.l $79c3c(pc), a1
pub fn i_079c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C3C;
    16
}
/// $079C38: move.l a1, (a6)
pub fn i_079c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C3A: rts
pub fn i_079c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hatch_Flee_079c3c  ($079C3C, 44 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C3C: move.b #$28, $59(a6)
pub fn i_079c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079C42: lea.l $79c48(pc), a1
pub fn i_079c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C48;
    16
}
/// $079C46: move.l a1, (a6)
pub fn i_079c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C48: jsr $7fdfa.l
pub fn i_079c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C4E);
    cpu.pc = 0x7FDFA;
    16
}
/// $079C4E: jsr $28d70.l
pub fn i_079c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C54);
    cpu.pc = 0x28D70;
    16
}
/// $079C54: tst.b $59(a6)
pub fn i_079c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C58;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079C58: bne.w $79c6e
pub fn i_079c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C5C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79C6E; return 10; }
    8
}
/// $079C5C: movea.l $c(a6), a0
pub fn i_079c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079C60: move.w $72(a6), d0
pub fn i_079c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079C64: bclr.b d0, $70(a0)
pub fn i_079c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C68;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Crew_Hatch_Free_079c68  ($079C68, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C68: jmp $518.l
pub fn i_079c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C6E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079C6E: rts
pub fn i_079c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_RetGateB_079c70  ($079C70, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C70: movea.l $8(a6), a1
pub fn i_079c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C74;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079C74: move.b $10(a6), d0
pub fn i_079c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C78;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079C78: cmp.b $10(a1), d0
pub fn i_079c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C7C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079C7C: bcs.w $79c86
pub fn i_079c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C80;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79C86; return 10; }
    8
}
// ===== ClearXN_079c80  ($079C80, 6 B, thunks/ccr_helpers.c)
/// $079C80: andi.b #$ee, ccr
pub fn i_079c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C84;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079C84: rts
pub fn i_079c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_079c86  ($079C86, 6 B, thunks/ccr_helpers.c)
/// $079C86: ori.b #$11, ccr
pub fn i_079c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C8A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079C8A: rts
pub fn i_079c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Tmpl126_079c8c  ($079C8C, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C8C: move.w #$19, d1
pub fn i_079c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C90;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079C90: jsr $236e.l
pub fn i_079c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C96);
    cpu.pc = 0x236E;
    16
}
/// $079C96: move.w #$1, $66(a6)
pub fn i_079c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C9C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079C9C: jsr $267e2.l
pub fn i_079c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CA2);
    cpu.pc = 0x267E2;
    16
}
// ===== SetTaskHandler_079ca2  ($079CA2, 8 B, kernel/task_handlers.c)
/// $079CA2: lea.l $79caa(pc), a1
pub fn i_079ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79CAA;
    16
}
/// $079CA6: move.l a1, (a6)
pub fn i_079ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079CA8: rts
pub fn i_079ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_JumpB_Run_079caa  ($079CAA, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $079CAA: jsr $5e7c0.l
pub fn i_079caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CB0);
    cpu.pc = 0x5E7C0;
    16
}
/// $079CB0: move.w #$8000, d0
pub fn i_079cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079CB4: jsr $28134.l
pub fn i_079cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CBA);
    cpu.pc = 0x28134;
    16
}
/// $079CBA: andi.w #$ffe3, $38(a6)
pub fn i_079cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CC0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079CC0: ori.w #$c, $38(a6)
pub fn i_079cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CC6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079CC6: lea.l $29c244.l, a0
pub fn i_079cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C244;
    20
}
/// $079CCC: jsr $28cd4.l
pub fn i_079ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CD2);
    cpu.pc = 0x28CD4;
    16
}
/// $079CD2: lea.l $79cd8(pc), a1
pub fn i_079cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79CD8;
    16
}
/// $079CD6: move.l a1, (a6)
pub fn i_079cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079CD8: jsr $2783a.l
pub fn i_079cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CDE);
    cpu.pc = 0x2783A;
    16
}
/// $079CDE: jsr $28d70.l
pub fn i_079cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CE4);
    cpu.pc = 0x28D70;
    16
}
/// $079CE4: jsr $2870a.l
pub fn i_079ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CEA);
    cpu.pc = 0x2870A;
    16
}
/// $079CEA: bcc.w $79cf4
pub fn i_079cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79CF4; return 10; }
    8
}
/// $079CEE: lea.l $79d04(pc), a1
pub fn i_079cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D04;
    16
}
/// $079CF2: move.l a1, (a6)
pub fn i_079cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079CF4: jsr $79d7e(pc)
pub fn i_079cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79CF8);
    cpu.pc = 0x79D7E;
    16
}
/// $079CF8: bcc.w $79d02
pub fn i_079cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79D02; return 10; }
    8
}
// ===== SetTaskHandler_079cfc  ($079CFC, 8 B, kernel/task_handlers.c)
/// $079CFC: lea.l $79d76(pc), a1
pub fn i_079cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D76;
    16
}
/// $079D00: move.l a1, (a6)
pub fn i_079d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D02: rts
pub fn i_079d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_LandB_079d04  ($079D04, 74 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D04: tst.b $10fd8f.l
pub fn i_079d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D0A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $079D0A: bne.w $79d18
pub fn i_079d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D0E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79D18; return 10; }
    8
}
/// $079D0E: move.w #$1d5, d1
pub fn i_079d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D12;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079D12: jsr $236e.l
pub fn i_079d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D18);
    cpu.pc = 0x236E;
    16
}
/// $079D18: bclr.b #$1, $12(a6)
pub fn i_079d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D1E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079D1E: lea.l $29c28c.l, a0
pub fn i_079d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C28C;
    20
}
/// $079D24: jsr $28cd4.l
pub fn i_079d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D2A);
    cpu.pc = 0x28CD4;
    16
}
/// $079D2A: lea.l $79d30(pc), a1
pub fn i_079d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D30;
    16
}
/// $079D2E: move.l a1, (a6)
pub fn i_079d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D30: jsr $2783a.l
pub fn i_079d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D36);
    cpu.pc = 0x2783A;
    16
}
/// $079D36: jsr $28d70.l
pub fn i_079d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D3C);
    cpu.pc = 0x28D70;
    16
}
/// $079D3C: bcc.w $79d46
pub fn i_079d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79D46; return 10; }
    8
}
/// $079D40: lea.l $79d56(pc), a1
pub fn i_079d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D56;
    16
}
/// $079D44: move.l a1, (a6)
pub fn i_079d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D46: jsr $79d7e(pc)
pub fn i_079d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79D4A);
    cpu.pc = 0x79D7E;
    16
}
/// $079D4A: bcc.w $79d54
pub fn i_079d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79D54; return 10; }
    8
}
// ===== SetTaskHandler_079d4e  ($079D4E, 8 B, kernel/task_handlers.c)
/// $079D4E: lea.l $79d76(pc), a1
pub fn i_079d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D76;
    16
}
/// $079D52: move.l a1, (a6)
pub fn i_079d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D54: rts
pub fn i_079d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_FleeB_079d56  ($079D56, 32 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D56: move.b #$28, $59(a6)
pub fn i_079d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079D5C: lea.l $79d62(pc), a1
pub fn i_079d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D62;
    16
}
/// $079D60: move.l a1, (a6)
pub fn i_079d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D62: jsr $2783a.l
pub fn i_079d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D68);
    cpu.pc = 0x2783A;
    16
}
/// $079D68: jsr $28d70.l
pub fn i_079d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D6E);
    cpu.pc = 0x28D70;
    16
}
/// $079D6E: tst.b $59(a6)
pub fn i_079d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D72;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079D72: bne.w $79d7c
pub fn i_079d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79D7C; return 10; }
    8
}
// ===== Crew_FreeB_079d76  ($079D76, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D76: jmp $518.l
pub fn i_079d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D7C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079D7C: rts
pub fn i_079d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_FreeIfOffWorldB_079d7e  ($079D7E, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D7E: lea.l $2df4e2.l, a0
pub fn i_079d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF4E2;
    20
}
/// $079D84: jsr $5dd5c.l
pub fn i_079d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D8A);
    cpu.pc = 0x5DD5C;
    16
}
/// $079D8A: bcc.w $79d94
pub fn i_079d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79D94; return 10; }
    8
}
// ===== SetC_079d8e  ($079D8E, 6 B, thunks/ccr_helpers.c)
/// $079D8E: ori.b #$1, ccr
pub fn i_079d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D92;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $079D92: rts
pub fn i_079d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_079d94  ($079D94, 6 B, thunks/ccr_helpers.c)
/// $079D94: andi.b #$fe, ccr
pub fn i_079d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D98;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $079D98: rts
pub fn i_079d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_HatchB_079d9a  ($079D9A, 136 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D9A: movea.l $c(a6), a0
pub fn i_079d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D9E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079D9E: move.w $72(a6), d0
pub fn i_079d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DA2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079DA2: bset.b d0, $70(a0)
pub fn i_079da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DA6;
    cpu.ir = 0x1E8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079DA6: move.w #$19, d1
pub fn i_079da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DAA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079DAA: jsr $236e.l
pub fn i_079daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DB0);
    cpu.pc = 0x236E;
    16
}
/// $079DB0: addq.w #$3, $38(a6)
pub fn i_079db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DB4;
    cpu.ir = 0x566E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $079DB4: move.w $38(a6), d0
pub fn i_079db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079DB8: jsr $28134.l
pub fn i_079db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DBE);
    cpu.pc = 0x28134;
    16
}
/// $079DBE: move.w #$1, $66(a6)
pub fn i_079dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079DC4: jsr $267e2.l
pub fn i_079dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DCA);
    cpu.pc = 0x267E2;
    16
}
/// $079DCA: lea.l $29c244.l, a0
pub fn i_079dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C244;
    20
}
/// $079DD0: jsr $28cd4.l
pub fn i_079dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DD6);
    cpu.pc = 0x28CD4;
    16
}
/// $079DD6: lea.l $79ddc(pc), a1
pub fn i_079dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79DDC;
    16
}
/// $079DDA: move.l a1, (a6)
pub fn i_079dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079DDC: jsr $7fdfa.l
pub fn i_079ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DE2);
    cpu.pc = 0x7FDFA;
    16
}
/// $079DE2: jsr $28d70.l
pub fn i_079de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DE8);
    cpu.pc = 0x28D70;
    16
}
/// $079DE8: movea.l $c(a6), a0
pub fn i_079de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079DEC: cmpi.b #$3, $21(a0)
pub fn i_079dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DF2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $079DF2: beq.w $79e00
pub fn i_079df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79E00; return 10; }
    8
}
/// $079DF6: cmpi.b #$ff, $20(a0)
pub fn i_079df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DFC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079DFC: bne.w $79e08
pub fn i_079dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79E08; return 10; }
    8
}
/// $079E00: lea.l $77efe.l, a1
pub fn i_079e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E06;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EFE;
    20
}
/// $079E06: move.l a1, (a6)
pub fn i_079e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x078142, i_078142),
    (0x078146, i_078146),
    (0x07814C, i_07814c),
    (0x078150, i_078150),
    (0x078156, i_078156),
    (0x07815A, i_07815a),
    (0x078160, i_078160),
    (0x078166, i_078166),
    (0x07816C, i_07816c),
    (0x078170, i_078170),
    (0x078176, i_078176),
    (0x07817A, i_07817a),
    (0x078180, i_078180),
    (0x078184, i_078184),
    (0x07818A, i_07818a),
    (0x078190, i_078190),
    (0x078196, i_078196),
    (0x07819A, i_07819a),
    (0x0781A0, i_0781a0),
    (0x0781A4, i_0781a4),
    (0x0781AA, i_0781aa),
    (0x0781AE, i_0781ae),
    (0x0781B4, i_0781b4),
    (0x0781BA, i_0781ba),
    (0x0781C0, i_0781c0),
    (0x0781C4, i_0781c4),
    (0x0781C8, i_0781c8),
    (0x0781CE, i_0781ce),
    (0x0781D2, i_0781d2),
    (0x0781D8, i_0781d8),
    (0x0781DC, i_0781dc),
    (0x0781E0, i_0781e0),
    (0x0781E4, i_0781e4),
    (0x0781E6, i_0781e6),
    (0x0781EC, i_0781ec),
    (0x0781F2, i_0781f2),
    (0x0781F8, i_0781f8),
    (0x0781FC, i_0781fc),
    (0x078200, i_078200),
    (0x078206, i_078206),
    (0x07820A, i_07820a),
    (0x078210, i_078210),
    (0x078216, i_078216),
    (0x078218, i_078218),
    (0x07821E, i_07821e),
    (0x078224, i_078224),
    (0x07822A, i_07822a),
    (0x07822E, i_07822e),
    (0x078232, i_078232),
    (0x078236, i_078236),
    (0x07823C, i_07823c),
    (0x078242, i_078242),
    (0x078248, i_078248),
    (0x07824C, i_07824c),
    (0x078250, i_078250),
    (0x078254, i_078254),
    (0x07825A, i_07825a),
    (0x078260, i_078260),
    (0x078266, i_078266),
    (0x07826A, i_07826a),
    (0x07826E, i_07826e),
    (0x078272, i_078272),
    (0x078278, i_078278),
    (0x07827E, i_07827e),
    (0x078284, i_078284),
    (0x078288, i_078288),
    (0x07828C, i_07828c),
    (0x078290, i_078290),
    (0x078296, i_078296),
    (0x07829C, i_07829c),
    (0x0782A2, i_0782a2),
    (0x0782A6, i_0782a6),
    (0x0782AA, i_0782aa),
    (0x0782AE, i_0782ae),
    (0x0782B4, i_0782b4),
    (0x0782BA, i_0782ba),
    (0x0782C0, i_0782c0),
    (0x0782C4, i_0782c4),
    (0x0782C8, i_0782c8),
    (0x0782CC, i_0782cc),
    (0x0782D2, i_0782d2),
    (0x0782D8, i_0782d8),
    (0x0782DE, i_0782de),
    (0x0782E2, i_0782e2),
    (0x0782E6, i_0782e6),
    (0x0782EA, i_0782ea),
    (0x0782F0, i_0782f0),
    (0x0782F6, i_0782f6),
    (0x0782FC, i_0782fc),
    (0x078300, i_078300),
    (0x078304, i_078304),
    (0x078308, i_078308),
    (0x07830E, i_07830e),
    (0x078314, i_078314),
    (0x07831A, i_07831a),
    (0x07831E, i_07831e),
    (0x078322, i_078322),
    (0x078326, i_078326),
    (0x07832C, i_07832c),
    (0x078332, i_078332),
    (0x078338, i_078338),
    (0x07833C, i_07833c),
    (0x078340, i_078340),
    (0x078344, i_078344),
    (0x07834A, i_07834a),
    (0x078350, i_078350),
    (0x078356, i_078356),
    (0x07835A, i_07835a),
    (0x07835E, i_07835e),
    (0x078362, i_078362),
    (0x078368, i_078368),
    (0x07836E, i_07836e),
    (0x078374, i_078374),
    (0x078378, i_078378),
    (0x07837C, i_07837c),
    (0x078380, i_078380),
    (0x078386, i_078386),
    (0x07838C, i_07838c),
    (0x078392, i_078392),
    (0x078396, i_078396),
    (0x07839A, i_07839a),
    (0x07839E, i_07839e),
    (0x0783A4, i_0783a4),
    (0x0783AA, i_0783aa),
    (0x0783B0, i_0783b0),
    (0x0783B4, i_0783b4),
    (0x0783B8, i_0783b8),
    (0x0783BC, i_0783bc),
    (0x0783C2, i_0783c2),
    (0x0783C8, i_0783c8),
    (0x0783CE, i_0783ce),
    (0x0783D2, i_0783d2),
    (0x0783D6, i_0783d6),
    (0x0783DA, i_0783da),
    (0x0783E0, i_0783e0),
    (0x0783E6, i_0783e6),
    (0x0783EC, i_0783ec),
    (0x0783F0, i_0783f0),
    (0x0783F4, i_0783f4),
    (0x0783F8, i_0783f8),
    (0x0783FE, i_0783fe),
    (0x078404, i_078404),
    (0x07840A, i_07840a),
    (0x07840E, i_07840e),
    (0x078412, i_078412),
    (0x078416, i_078416),
    (0x07841C, i_07841c),
    (0x078422, i_078422),
    (0x078428, i_078428),
    (0x07842C, i_07842c),
    (0x078430, i_078430),
    (0x078434, i_078434),
    (0x07843A, i_07843a),
    (0x078440, i_078440),
    (0x078446, i_078446),
    (0x07844A, i_07844a),
    (0x07844E, i_07844e),
    (0x078452, i_078452),
    (0x078458, i_078458),
    (0x07845E, i_07845e),
    (0x078464, i_078464),
    (0x078468, i_078468),
    (0x07846C, i_07846c),
    (0x078470, i_078470),
    (0x078476, i_078476),
    (0x07847C, i_07847c),
    (0x078482, i_078482),
    (0x078486, i_078486),
    (0x07848A, i_07848a),
    (0x07848E, i_07848e),
    (0x078494, i_078494),
    (0x07849A, i_07849a),
    (0x0784A0, i_0784a0),
    (0x0784A4, i_0784a4),
    (0x0784A8, i_0784a8),
    (0x0784AC, i_0784ac),
    (0x0784B2, i_0784b2),
    (0x0784B8, i_0784b8),
    (0x0784BE, i_0784be),
    (0x0784C2, i_0784c2),
    (0x0784C6, i_0784c6),
    (0x0784CA, i_0784ca),
    (0x0784D0, i_0784d0),
    (0x0784D6, i_0784d6),
    (0x0784DC, i_0784dc),
    (0x0784E0, i_0784e0),
    (0x0784E4, i_0784e4),
    (0x0784E8, i_0784e8),
    (0x0784EE, i_0784ee),
    (0x0784F4, i_0784f4),
    (0x0784FA, i_0784fa),
    (0x0784FE, i_0784fe),
    (0x078502, i_078502),
    (0x078506, i_078506),
    (0x07850C, i_07850c),
    (0x078512, i_078512),
    (0x078518, i_078518),
    (0x07851C, i_07851c),
    (0x078520, i_078520),
    (0x078524, i_078524),
    (0x07852A, i_07852a),
    (0x078530, i_078530),
    (0x078536, i_078536),
    (0x07853A, i_07853a),
    (0x07853E, i_07853e),
    (0x078542, i_078542),
    (0x078548, i_078548),
    (0x07854E, i_07854e),
    (0x078554, i_078554),
    (0x078558, i_078558),
    (0x07855C, i_07855c),
    (0x078560, i_078560),
    (0x078566, i_078566),
    (0x07856C, i_07856c),
    (0x078572, i_078572),
    (0x078576, i_078576),
    (0x07857A, i_07857a),
    (0x07857E, i_07857e),
    (0x078584, i_078584),
    (0x07858A, i_07858a),
    (0x078590, i_078590),
    (0x078594, i_078594),
    (0x078598, i_078598),
    (0x07859C, i_07859c),
    (0x0785A2, i_0785a2),
    (0x0785A8, i_0785a8),
    (0x0785AE, i_0785ae),
    (0x0785B2, i_0785b2),
    (0x0785B6, i_0785b6),
    (0x0785BA, i_0785ba),
    (0x0785C0, i_0785c0),
    (0x0785C6, i_0785c6),
    (0x0785CC, i_0785cc),
    (0x0785D0, i_0785d0),
    (0x0785D4, i_0785d4),
    (0x0785D8, i_0785d8),
    (0x0785DE, i_0785de),
    (0x0785E4, i_0785e4),
    (0x0785EA, i_0785ea),
    (0x0785EE, i_0785ee),
    (0x0785F2, i_0785f2),
    (0x0785F6, i_0785f6),
    (0x0785FC, i_0785fc),
    (0x078602, i_078602),
    (0x078608, i_078608),
    (0x07860C, i_07860c),
    (0x078610, i_078610),
    (0x078614, i_078614),
    (0x07861A, i_07861a),
    (0x078620, i_078620),
    (0x078626, i_078626),
    (0x07862A, i_07862a),
    (0x07862E, i_07862e),
    (0x078632, i_078632),
    (0x078638, i_078638),
    (0x07863E, i_07863e),
    (0x078644, i_078644),
    (0x078648, i_078648),
    (0x07864C, i_07864c),
    (0x078650, i_078650),
    (0x078656, i_078656),
    (0x07865C, i_07865c),
    (0x078662, i_078662),
    (0x078666, i_078666),
    (0x07866A, i_07866a),
    (0x07866E, i_07866e),
    (0x078674, i_078674),
    (0x07867A, i_07867a),
    (0x078680, i_078680),
    (0x078684, i_078684),
    (0x078688, i_078688),
    (0x07868C, i_07868c),
    (0x078692, i_078692),
    (0x078698, i_078698),
    (0x07869E, i_07869e),
    (0x0786A2, i_0786a2),
    (0x0786A6, i_0786a6),
    (0x0786AA, i_0786aa),
    (0x0786B0, i_0786b0),
    (0x0786B6, i_0786b6),
    (0x0786BC, i_0786bc),
    (0x0786C0, i_0786c0),
    (0x0786C4, i_0786c4),
    (0x0786C8, i_0786c8),
    (0x0786CE, i_0786ce),
    (0x0786D4, i_0786d4),
    (0x0786DA, i_0786da),
    (0x0786DE, i_0786de),
    (0x0786E2, i_0786e2),
    (0x0786E6, i_0786e6),
    (0x0786EC, i_0786ec),
    (0x0786F2, i_0786f2),
    (0x0786F8, i_0786f8),
    (0x0786FC, i_0786fc),
    (0x078700, i_078700),
    (0x078704, i_078704),
    (0x07870A, i_07870a),
    (0x078710, i_078710),
    (0x078716, i_078716),
    (0x07871A, i_07871a),
    (0x07871E, i_07871e),
    (0x078722, i_078722),
    (0x078728, i_078728),
    (0x07872E, i_07872e),
    (0x078734, i_078734),
    (0x078738, i_078738),
    (0x07873C, i_07873c),
    (0x078740, i_078740),
    (0x078746, i_078746),
    (0x07874C, i_07874c),
    (0x078752, i_078752),
    (0x078756, i_078756),
    (0x07875A, i_07875a),
    (0x07875E, i_07875e),
    (0x078764, i_078764),
    (0x07876A, i_07876a),
    (0x078770, i_078770),
    (0x078774, i_078774),
    (0x078778, i_078778),
    (0x07877E, i_07877e),
    (0x078784, i_078784),
    (0x07878A, i_07878a),
    (0x07878E, i_07878e),
    (0x078792, i_078792),
    (0x078798, i_078798),
    (0x07879C, i_07879c),
    (0x0787A2, i_0787a2),
    (0x0787A8, i_0787a8),
    (0x0787AE, i_0787ae),
    (0x0787B2, i_0787b2),
    (0x0787B8, i_0787b8),
    (0x0787BC, i_0787bc),
    (0x0787C0, i_0787c0),
    (0x0787C2, i_0787c2),
    (0x0787C6, i_0787c6),
    (0x0787CA, i_0787ca),
    (0x0787D0, i_0787d0),
    (0x0787D4, i_0787d4),
    (0x0787D8, i_0787d8),
    (0x0787DC, i_0787dc),
    (0x0787E0, i_0787e0),
    (0x0787E6, i_0787e6),
    (0x0787EA, i_0787ea),
    (0x0787EE, i_0787ee),
    (0x0787F2, i_0787f2),
    (0x0787F6, i_0787f6),
    (0x0787F8, i_0787f8),
    (0x0787FE, i_0787fe),
    (0x078802, i_078802),
    (0x078808, i_078808),
    (0x07880E, i_07880e),
    (0x078814, i_078814),
    (0x078818, i_078818),
    (0x07881C, i_07881c),
    (0x078820, i_078820),
    (0x078826, i_078826),
    (0x07882A, i_07882a),
    (0x07882E, i_07882e),
    (0x078834, i_078834),
    (0x078838, i_078838),
    (0x07883E, i_07883e),
    (0x078840, i_078840),
    (0x078844, i_078844),
    (0x07884A, i_07884a),
    (0x078850, i_078850),
    (0x078856, i_078856),
    (0x07885C, i_07885c),
    (0x078860, i_078860),
    (0x078864, i_078864),
    (0x078868, i_078868),
    (0x07886E, i_07886e),
    (0x078874, i_078874),
    (0x07887A, i_07887a),
    (0x07887E, i_07887e),
    (0x078884, i_078884),
    (0x07888A, i_07888a),
    (0x078890, i_078890),
    (0x078894, i_078894),
    (0x07889A, i_07889a),
    (0x0788A0, i_0788a0),
    (0x0788A6, i_0788a6),
    (0x0788AA, i_0788aa),
    (0x0788B0, i_0788b0),
    (0x0788B6, i_0788b6),
    (0x0788BC, i_0788bc),
    (0x0788C0, i_0788c0),
    (0x0788C4, i_0788c4),
    (0x0788CA, i_0788ca),
    (0x0788D0, i_0788d0),
    (0x0788D6, i_0788d6),
    (0x0788DC, i_0788dc),
    (0x0788E0, i_0788e0),
    (0x0788E4, i_0788e4),
    (0x0788E8, i_0788e8),
    (0x0788EE, i_0788ee),
    (0x0788F4, i_0788f4),
    (0x0788FA, i_0788fa),
    (0x078900, i_078900),
    (0x078904, i_078904),
    (0x078908, i_078908),
    (0x07890C, i_07890c),
    (0x078912, i_078912),
    (0x078918, i_078918),
    (0x07891E, i_07891e),
    (0x078924, i_078924),
    (0x078928, i_078928),
    (0x07892C, i_07892c),
    (0x078930, i_078930),
    (0x078936, i_078936),
    (0x07893C, i_07893c),
    (0x078942, i_078942),
    (0x078948, i_078948),
    (0x07894C, i_07894c),
    (0x078950, i_078950),
    (0x078954, i_078954),
    (0x07895A, i_07895a),
    (0x078960, i_078960),
    (0x078966, i_078966),
    (0x07896C, i_07896c),
    (0x078970, i_078970),
    (0x078974, i_078974),
    (0x078978, i_078978),
    (0x07897E, i_07897e),
    (0x078984, i_078984),
    (0x07898A, i_07898a),
    (0x078990, i_078990),
    (0x078994, i_078994),
    (0x078998, i_078998),
    (0x07899E, i_07899e),
    (0x0789A4, i_0789a4),
    (0x0789A8, i_0789a8),
    (0x0789AE, i_0789ae),
    (0x0789B2, i_0789b2),
    (0x0789B8, i_0789b8),
    (0x0789BC, i_0789bc),
    (0x0789C0, i_0789c0),
    (0x0789C4, i_0789c4),
    (0x0789C8, i_0789c8),
    (0x0789CA, i_0789ca),
    (0x0789D0, i_0789d0),
    (0x0789D6, i_0789d6),
    (0x0789DA, i_0789da),
    (0x0789DE, i_0789de),
    (0x0789E4, i_0789e4),
    (0x0789E8, i_0789e8),
    (0x0789EE, i_0789ee),
    (0x0789F0, i_0789f0),
    (0x0789F6, i_0789f6),
    (0x0789FC, i_0789fc),
    (0x078A02, i_078a02),
    (0x078A06, i_078a06),
    (0x078A0A, i_078a0a),
    (0x078A0E, i_078a0e),
    (0x078A14, i_078a14),
    (0x078A1A, i_078a1a),
    (0x078A20, i_078a20),
    (0x078A26, i_078a26),
    (0x078A2A, i_078a2a),
    (0x078A30, i_078a30),
    (0x078A34, i_078a34),
    (0x078A38, i_078a38),
    (0x078A3C, i_078a3c),
    (0x078A40, i_078a40),
    (0x078A42, i_078a42),
    (0x078A48, i_078a48),
    (0x078A4E, i_078a4e),
    (0x078A52, i_078a52),
    (0x078A56, i_078a56),
    (0x078A5C, i_078a5c),
    (0x078A60, i_078a60),
    (0x078A66, i_078a66),
    (0x078A68, i_078a68),
    (0x078A6C, i_078a6c),
    (0x078A70, i_078a70),
    (0x078A74, i_078a74),
    (0x078A78, i_078a78),
    (0x078A7C, i_078a7c),
    (0x078A7E, i_078a7e),
    (0x078A82, i_078a82),
    (0x078C64, i_078c64),
    (0x078C68, i_078c68),
    (0x078C6C, i_078c6c),
    (0x078C70, i_078c70),
    (0x078C76, i_078c76),
    (0x078C78, i_078c78),
    (0x078C7E, i_078c7e),
    (0x078C84, i_078c84),
    (0x078C88, i_078c88),
    (0x078C8E, i_078c8e),
    (0x078C94, i_078c94),
    (0x078C98, i_078c98),
    (0x078C9E, i_078c9e),
    (0x078CA2, i_078ca2),
    (0x078CA8, i_078ca8),
    (0x078CAE, i_078cae),
    (0x078CB2, i_078cb2),
    (0x078CB8, i_078cb8),
    (0x078CBC, i_078cbc),
    (0x078CC2, i_078cc2),
    (0x078CC6, i_078cc6),
    (0x078CC8, i_078cc8),
    (0x078CCE, i_078cce),
    (0x078CD4, i_078cd4),
    (0x078CD6, i_078cd6),
    (0x078CDA, i_078cda),
    (0x078CDC, i_078cdc),
    (0x078CDE, i_078cde),
    (0x078CE2, i_078ce2),
    (0x078CE6, i_078ce6),
    (0x078CEA, i_078cea),
    (0x078CEE, i_078cee),
    (0x078CF0, i_078cf0),
    (0x078CF4, i_078cf4),
    (0x078CF8, i_078cf8),
    (0x078CFE, i_078cfe),
    (0x078D02, i_078d02),
    (0x078D08, i_078d08),
    (0x078D0E, i_078d0e),
    (0x078D12, i_078d12),
    (0x078D14, i_078d14),
    (0x078D1A, i_078d1a),
    (0x078D20, i_078d20),
    (0x078D24, i_078d24),
    (0x078D28, i_078d28),
    (0x078D2A, i_078d2a),
    (0x078D2E, i_078d2e),
    (0x078D34, i_078d34),
    (0x078D38, i_078d38),
    (0x078D3A, i_078d3a),
    (0x078D40, i_078d40),
    (0x078D46, i_078d46),
    (0x078D4A, i_078d4a),
    (0x078D4E, i_078d4e),
    (0x078D54, i_078d54),
    (0x078D58, i_078d58),
    (0x078D5C, i_078d5c),
    (0x078D62, i_078d62),
    (0x078D68, i_078d68),
    (0x078D6E, i_078d6e),
    (0x078D74, i_078d74),
    (0x078D78, i_078d78),
    (0x078D7E, i_078d7e),
    (0x078D82, i_078d82),
    (0x078D88, i_078d88),
    (0x078D8C, i_078d8c),
    (0x078D90, i_078d90),
    (0x078D94, i_078d94),
    (0x078D96, i_078d96),
    (0x078D98, i_078d98),
    (0x078D9A, i_078d9a),
    (0x078D9C, i_078d9c),
    (0x078D9E, i_078d9e),
    (0x078DA0, i_078da0),
    (0x078DA2, i_078da2),
    (0x078DA4, i_078da4),
    (0x078DAA, i_078daa),
    (0x078DAE, i_078dae),
    (0x078DB2, i_078db2),
    (0x078DB6, i_078db6),
    (0x078DB8, i_078db8),
    (0x078DBC, i_078dbc),
    (0x078DBE, i_078dbe),
    (0x078DC0, i_078dc0),
    (0x078DC6, i_078dc6),
    (0x078DCA, i_078dca),
    (0x078DCC, i_078dcc),
    (0x078DD0, i_078dd0),
    (0x078DD4, i_078dd4),
    (0x078DD8, i_078dd8),
    (0x078DDC, i_078ddc),
    (0x078DE0, i_078de0),
    (0x078DE4, i_078de4),
    (0x078DEA, i_078dea),
    (0x078DEE, i_078dee),
    (0x078DF4, i_078df4),
    (0x078DF6, i_078df6),
    (0x078DFC, i_078dfc),
    (0x078E00, i_078e00),
    (0x078E04, i_078e04),
    (0x078E08, i_078e08),
    (0x078E0E, i_078e0e),
    (0x078E14, i_078e14),
    (0x078E1A, i_078e1a),
    (0x078E1E, i_078e1e),
    (0x078E24, i_078e24),
    (0x078E2A, i_078e2a),
    (0x078E30, i_078e30),
    (0x078E34, i_078e34),
    (0x078E38, i_078e38),
    (0x078E3A, i_078e3a),
    (0x078E3C, i_078e3c),
    (0x078E3E, i_078e3e),
    (0x078E40, i_078e40),
    (0x078E42, i_078e42),
    (0x078E44, i_078e44),
    (0x078E46, i_078e46),
    (0x078EDE, i_078ede),
    (0x078EE2, i_078ee2),
    (0x078EE8, i_078ee8),
    (0x078EEE, i_078eee),
    (0x078EF2, i_078ef2),
    (0x078EF6, i_078ef6),
    (0x078EFC, i_078efc),
    (0x078F00, i_078f00),
    (0x078F02, i_078f02),
    (0x078F06, i_078f06),
    (0x078F0A, i_078f0a),
    (0x078F0E, i_078f0e),
    (0x078F12, i_078f12),
    (0x078F14, i_078f14),
    (0x078F18, i_078f18),
    (0x078F1E, i_078f1e),
    (0x078F22, i_078f22),
    (0x078F26, i_078f26),
    (0x078F2A, i_078f2a),
    (0x078F2C, i_078f2c),
    (0x078F30, i_078f30),
    (0x078F36, i_078f36),
    (0x078F3C, i_078f3c),
    (0x078F40, i_078f40),
    (0x078F46, i_078f46),
    (0x078F4A, i_078f4a),
    (0x078F4C, i_078f4c),
    (0x078F52, i_078f52),
    (0x078F56, i_078f56),
    (0x078F5C, i_078f5c),
    (0x078F62, i_078f62),
    (0x078F66, i_078f66),
    (0x078F6C, i_078f6c),
    (0x078F6E, i_078f6e),
    (0x078F72, i_078f72),
    (0x078F76, i_078f76),
    (0x078F7A, i_078f7a),
    (0x078F7E, i_078f7e),
    (0x078F82, i_078f82),
    (0x078F84, i_078f84),
    (0x078F88, i_078f88),
    (0x078F8A, i_078f8a),
    (0x078F90, i_078f90),
    (0x078F94, i_078f94),
    (0x078F98, i_078f98),
    (0x078F9C, i_078f9c),
    (0x078F9E, i_078f9e),
    (0x078FA2, i_078fa2),
    (0x078FA6, i_078fa6),
    (0x078FA8, i_078fa8),
    (0x078FAA, i_078faa),
    (0x078FAE, i_078fae),
    (0x078FB0, i_078fb0),
    (0x078FB2, i_078fb2),
    (0x078FB8, i_078fb8),
    (0x078FBA, i_078fba),
    (0x078FBE, i_078fbe),
    (0x079000, i_079000),
    (0x079002, i_079002),
    (0x079004, i_079004),
    (0x079006, i_079006),
    (0x07900A, i_07900a),
    (0x07900E, i_07900e),
    (0x079012, i_079012),
    (0x079014, i_079014),
    (0x079018, i_079018),
    (0x07901C, i_07901c),
    (0x079020, i_079020),
    (0x079024, i_079024),
    (0x079028, i_079028),
    (0x07902A, i_07902a),
    (0x07902E, i_07902e),
    (0x079030, i_079030),
    (0x079034, i_079034),
    (0x079038, i_079038),
    (0x07903C, i_07903c),
    (0x079040, i_079040),
    (0x079046, i_079046),
    (0x07904A, i_07904a),
    (0x07904C, i_07904c),
    (0x079050, i_079050),
    (0x079054, i_079054),
    (0x079058, i_079058),
    (0x07905C, i_07905c),
    (0x079060, i_079060),
    (0x079064, i_079064),
    (0x079066, i_079066),
    (0x079068, i_079068),
    (0x07906C, i_07906c),
    (0x07906E, i_07906e),
    (0x079070, i_079070),
    (0x079074, i_079074),
    (0x079076, i_079076),
    (0x07907A, i_07907a),
    (0x07907E, i_07907e),
    (0x079082, i_079082),
    (0x079086, i_079086),
    (0x07908A, i_07908a),
    (0x07908E, i_07908e),
    (0x079090, i_079090),
    (0x079092, i_079092),
    (0x079096, i_079096),
    (0x079098, i_079098),
    (0x07909A, i_07909a),
    (0x07909E, i_07909e),
    (0x0790A0, i_0790a0),
    (0x0790A4, i_0790a4),
    (0x0790A8, i_0790a8),
    (0x0790AC, i_0790ac),
    (0x0790B0, i_0790b0),
    (0x0790B4, i_0790b4),
    (0x0790B8, i_0790b8),
    (0x0790BA, i_0790ba),
    (0x0790BC, i_0790bc),
    (0x0790C0, i_0790c0),
    (0x0790C2, i_0790c2),
    (0x0790C4, i_0790c4),
    (0x0790C8, i_0790c8),
    (0x0790CA, i_0790ca),
    (0x0790CE, i_0790ce),
    (0x0790D2, i_0790d2),
    (0x0790D6, i_0790d6),
    (0x0790D8, i_0790d8),
    (0x0790DC, i_0790dc),
    (0x0790DE, i_0790de),
    (0x0790E2, i_0790e2),
    (0x0790E6, i_0790e6),
    (0x0790EA, i_0790ea),
    (0x0790EE, i_0790ee),
    (0x0790F2, i_0790f2),
    (0x0790F6, i_0790f6),
    (0x0790FA, i_0790fa),
    (0x0790FC, i_0790fc),
    (0x079100, i_079100),
    (0x079104, i_079104),
    (0x079106, i_079106),
    (0x07910A, i_07910a),
    (0x07910E, i_07910e),
    (0x079112, i_079112),
    (0x079114, i_079114),
    (0x07911A, i_07911a),
    (0x07911E, i_07911e),
    (0x079124, i_079124),
    (0x079126, i_079126),
    (0x07912C, i_07912c),
    (0x079130, i_079130),
    (0x079132, i_079132),
    (0x079134, i_079134),
    (0x079138, i_079138),
    (0x07913A, i_07913a),
    (0x07913E, i_07913e),
    (0x079142, i_079142),
    (0x079146, i_079146),
    (0x07914A, i_07914a),
    (0x07914E, i_07914e),
    (0x079152, i_079152),
    (0x079154, i_079154),
    (0x079158, i_079158),
    (0x07915C, i_07915c),
    (0x07915E, i_07915e),
    (0x079164, i_079164),
    (0x079166, i_079166),
    (0x07916A, i_07916a),
    (0x07916C, i_07916c),
    (0x079170, i_079170),
    (0x079174, i_079174),
    (0x079178, i_079178),
    (0x07917A, i_07917a),
    (0x07917E, i_07917e),
    (0x079180, i_079180),
    (0x079184, i_079184),
    (0x079186, i_079186),
    (0x07918C, i_07918c),
    (0x07918E, i_07918e),
    (0x079192, i_079192),
    (0x079198, i_079198),
    (0x07919A, i_07919a),
    (0x0791A0, i_0791a0),
    (0x0791A2, i_0791a2),
    (0x0791A4, i_0791a4),
    (0x0791A6, i_0791a6),
    (0x0791A8, i_0791a8),
    (0x0791AA, i_0791aa),
    (0x0791AE, i_0791ae),
    (0x0791B2, i_0791b2),
    (0x0791B6, i_0791b6),
    (0x0791BA, i_0791ba),
    (0x0791BE, i_0791be),
    (0x0791C2, i_0791c2),
    (0x0791C6, i_0791c6),
    (0x0791CA, i_0791ca),
    (0x0791CE, i_0791ce),
    (0x0791D2, i_0791d2),
    (0x0791D6, i_0791d6),
    (0x0791D8, i_0791d8),
    (0x0791DC, i_0791dc),
    (0x0791E0, i_0791e0),
    (0x0791E6, i_0791e6),
    (0x0791EC, i_0791ec),
    (0x0791F0, i_0791f0),
    (0x0791F2, i_0791f2),
    (0x0791F6, i_0791f6),
    (0x0791FA, i_0791fa),
    (0x0791FC, i_0791fc),
    (0x0791FE, i_0791fe),
    (0x079200, i_079200),
    (0x079206, i_079206),
    (0x07920A, i_07920a),
    (0x079210, i_079210),
    (0x079212, i_079212),
    (0x079214, i_079214),
    (0x07921A, i_07921a),
    (0x07921E, i_07921e),
    (0x079224, i_079224),
    (0x079226, i_079226),
    (0x079228, i_079228),
    (0x07922A, i_07922a),
    (0x07922C, i_07922c),
    (0x079230, i_079230),
    (0x079234, i_079234),
    (0x079236, i_079236),
    (0x07923A, i_07923a),
    (0x07923C, i_07923c),
    (0x07923E, i_07923e),
    (0x079240, i_079240),
    (0x079244, i_079244),
    (0x079248, i_079248),
    (0x07924A, i_07924a),
    (0x07924E, i_07924e),
    (0x079250, i_079250),
    (0x079254, i_079254),
    (0x079258, i_079258),
    (0x07925C, i_07925c),
    (0x079260, i_079260),
    (0x079264, i_079264),
    (0x079266, i_079266),
    (0x07926A, i_07926a),
    (0x07926C, i_07926c),
    (0x079270, i_079270),
    (0x079274, i_079274),
    (0x079278, i_079278),
    (0x07927C, i_07927c),
    (0x079280, i_079280),
    (0x079286, i_079286),
    (0x07928A, i_07928a),
    (0x079290, i_079290),
    (0x079296, i_079296),
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
    (0x079326, i_079326),
    (0x07932C, i_07932c),
    (0x079332, i_079332),
    (0x079338, i_079338),
    (0x07933A, i_07933a),
    (0x079340, i_079340),
    (0x079342, i_079342),
    (0x07934C, i_07934c),
    (0x079356, i_079356),
    (0x07935C, i_07935c),
    (0x079362, i_079362),
    (0x079368, i_079368),
    (0x07936E, i_07936e),
    (0x079374, i_079374),
    (0x07937A, i_07937a),
    (0x079380, i_079380),
    (0x079386, i_079386),
    (0x07938A, i_07938a),
    (0x07938C, i_07938c),
    (0x079394, i_079394),
    (0x079396, i_079396),
    (0x07939E, i_07939e),
    (0x0793A0, i_0793a0),
    (0x0793A2, i_0793a2),
    (0x0793AC, i_0793ac),
    (0x0793B6, i_0793b6),
    (0x0793BC, i_0793bc),
    (0x0793C2, i_0793c2),
    (0x0793C8, i_0793c8),
    (0x0793CE, i_0793ce),
    (0x0793D4, i_0793d4),
    (0x0793DA, i_0793da),
    (0x0793E0, i_0793e0),
    (0x0793E6, i_0793e6),
    (0x0793E8, i_0793e8),
    (0x0793EE, i_0793ee),
    (0x0793F2, i_0793f2),
    (0x0793F8, i_0793f8),
    (0x0793FC, i_0793fc),
    (0x079404, i_079404),
    (0x079408, i_079408),
    (0x07940E, i_07940e),
    (0x079412, i_079412),
    (0x079418, i_079418),
    (0x07941C, i_07941c),
    (0x079424, i_079424),
    (0x07942A, i_07942a),
    (0x07942E, i_07942e),
    (0x079432, i_079432),
    (0x079438, i_079438),
    (0x07943C, i_07943c),
    (0x079440, i_079440),
    (0x079442, i_079442),
    (0x079444, i_079444),
    (0x079446, i_079446),
    (0x079450, i_079450),
    (0x07945A, i_07945a),
    (0x079460, i_079460),
    (0x079466, i_079466),
    (0x07946C, i_07946c),
    (0x079472, i_079472),
    (0x079478, i_079478),
    (0x07947E, i_07947e),
    (0x079484, i_079484),
    (0x07948A, i_07948a),
    (0x07948E, i_07948e),
    (0x079490, i_079490),
    (0x079498, i_079498),
    (0x07949C, i_07949c),
    (0x0794A4, i_0794a4),
    (0x0794A8, i_0794a8),
    (0x0794AA, i_0794aa),
    (0x0794AE, i_0794ae),
    (0x0794B0, i_0794b0),
    (0x0794B2, i_0794b2),
    (0x0794BC, i_0794bc),
    (0x0794C6, i_0794c6),
    (0x0794CC, i_0794cc),
    (0x0794D2, i_0794d2),
    (0x0794D8, i_0794d8),
    (0x0794DE, i_0794de),
    (0x0794E4, i_0794e4),
    (0x0794EA, i_0794ea),
    (0x0794F0, i_0794f0),
    (0x0794F6, i_0794f6),
    (0x0794FE, i_0794fe),
    (0x079502, i_079502),
    (0x07950A, i_07950a),
    (0x07950E, i_07950e),
    (0x079510, i_079510),
    (0x079516, i_079516),
    (0x07951A, i_07951a),
    (0x079520, i_079520),
    (0x079524, i_079524),
    (0x07952A, i_07952a),
    (0x07952E, i_07952e),
    (0x079536, i_079536),
    (0x07953A, i_07953a),
    (0x079542, i_079542),
    (0x079546, i_079546),
    (0x07954C, i_07954c),
    (0x079550, i_079550),
    (0x079556, i_079556),
    (0x07955A, i_07955a),
    (0x079560, i_079560),
    (0x079564, i_079564),
    (0x07956C, i_07956c),
    (0x079570, i_079570),
    (0x079578, i_079578),
    (0x07957A, i_07957a),
    (0x07957E, i_07957e),
    (0x079582, i_079582),
    (0x079588, i_079588),
    (0x07958E, i_07958e),
    (0x079592, i_079592),
    (0x079594, i_079594),
    (0x07959C, i_07959c),
    (0x0795A0, i_0795a0),
    (0x0795A8, i_0795a8),
    (0x0795AC, i_0795ac),
    (0x0795AE, i_0795ae),
    (0x0795B8, i_0795b8),
    (0x0795C2, i_0795c2),
    (0x0795C8, i_0795c8),
    (0x0795CE, i_0795ce),
    (0x0795D4, i_0795d4),
    (0x0795DA, i_0795da),
    (0x0795E0, i_0795e0),
    (0x0795E6, i_0795e6),
    (0x0795EC, i_0795ec),
    (0x0795F2, i_0795f2),
    (0x0795F8, i_0795f8),
    (0x0795FC, i_0795fc),
    (0x079604, i_079604),
    (0x07960C, i_07960c),
    (0x079610, i_079610),
    (0x079614, i_079614),
    (0x079618, i_079618),
    (0x079620, i_079620),
    (0x079628, i_079628),
    (0x07962A, i_07962a),
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
    (0x0796F6, i_0796f6),
    (0x0796FA, i_0796fa),
    (0x0796FE, i_0796fe),
    (0x079700, i_079700),
    (0x079706, i_079706),
    (0x07970A, i_07970a),
    (0x07970E, i_07970e),
    (0x079712, i_079712),
    (0x079714, i_079714),
    (0x07971A, i_07971a),
    (0x07971E, i_07971e),
    (0x079722, i_079722),
    (0x079726, i_079726),
    (0x07972A, i_07972a),
    (0x07972E, i_07972e),
    (0x079732, i_079732),
    (0x079734, i_079734),
    (0x07973A, i_07973a),
    (0x07973E, i_07973e),
    (0x079742, i_079742),
    (0x079748, i_079748),
    (0x079750, i_079750),
    (0x079758, i_079758),
    (0x079760, i_079760),
    (0x079764, i_079764),
    (0x079768, i_079768),
    (0x07976C, i_07976c),
    (0x079772, i_079772),
    (0x079776, i_079776),
    (0x07977A, i_07977a),
    (0x07977E, i_07977e),
    (0x079780, i_079780),
    (0x079786, i_079786),
    (0x07978A, i_07978a),
    (0x07978E, i_07978e),
    (0x079792, i_079792),
    (0x079796, i_079796),
    (0x07979A, i_07979a),
    (0x07979E, i_07979e),
    (0x0797A0, i_0797a0),
    (0x0797A6, i_0797a6),
    (0x0797AA, i_0797aa),
    (0x0797AE, i_0797ae),
    (0x0797B4, i_0797b4),
    (0x0797BC, i_0797bc),
    (0x0797C4, i_0797c4),
    (0x0797CC, i_0797cc),
    (0x0797CE, i_0797ce),
    (0x0797D4, i_0797d4),
    (0x0797D8, i_0797d8),
    (0x0797DC, i_0797dc),
    (0x0797DE, i_0797de),
    (0x0797E2, i_0797e2),
    (0x0797E6, i_0797e6),
    (0x0797EA, i_0797ea),
    (0x0797EC, i_0797ec),
    (0x0797F0, i_0797f0),
    (0x0797F2, i_0797f2),
    (0x0797F6, i_0797f6),
    (0x0797FC, i_0797fc),
    (0x079802, i_079802),
    (0x07980A, i_07980a),
    (0x079810, i_079810),
    (0x079816, i_079816),
    (0x07981A, i_07981a),
    (0x07981C, i_07981c),
    (0x079820, i_079820),
    (0x079826, i_079826),
    (0x079828, i_079828),
    (0x07982C, i_07982c),
    (0x079830, i_079830),
    (0x079834, i_079834),
    (0x079838, i_079838),
    (0x07983C, i_07983c),
    (0x07983E, i_07983e),
    (0x079842, i_079842),
    (0x079844, i_079844),
    (0x079846, i_079846),
    (0x07984C, i_07984c),
    (0x079850, i_079850),
    (0x079854, i_079854),
    (0x079856, i_079856),
    (0x07985C, i_07985c),
    (0x079860, i_079860),
    (0x079864, i_079864),
    (0x079868, i_079868),
    (0x07986C, i_07986c),
    (0x079870, i_079870),
    (0x079874, i_079874),
    (0x079878, i_079878),
    (0x07987C, i_07987c),
    (0x079880, i_079880),
    (0x079884, i_079884),
    (0x07988A, i_07988a),
    (0x07988E, i_07988e),
    (0x079892, i_079892),
    (0x079896, i_079896),
    (0x07989A, i_07989a),
    (0x07989E, i_07989e),
    (0x0798A2, i_0798a2),
    (0x0798A6, i_0798a6),
    (0x0798AA, i_0798aa),
    (0x0798AC, i_0798ac),
    (0x0798AE, i_0798ae),
    (0x0798B4, i_0798b4),
    (0x0798B8, i_0798b8),
    (0x0798BA, i_0798ba),
    (0x0798BE, i_0798be),
    (0x0798C2, i_0798c2),
    (0x0798C4, i_0798c4),
    (0x0798C6, i_0798c6),
    (0x0798CA, i_0798ca),
    (0x0798CC, i_0798cc),
    (0x0798D0, i_0798d0),
    (0x0798D2, i_0798d2),
    (0x0798D6, i_0798d6),
    (0x0798DA, i_0798da),
    (0x0798DE, i_0798de),
    (0x0798E2, i_0798e2),
    (0x0798E4, i_0798e4),
    (0x0798E6, i_0798e6),
    (0x0798EA, i_0798ea),
    (0x0798EC, i_0798ec),
    (0x0798F0, i_0798f0),
    (0x0798F2, i_0798f2),
    (0x0798F6, i_0798f6),
    (0x0798F8, i_0798f8),
    (0x0798FA, i_0798fa),
    (0x0798FE, i_0798fe),
    (0x079902, i_079902),
    (0x079906, i_079906),
    (0x079908, i_079908),
    (0x07990A, i_07990a),
    (0x07990C, i_07990c),
    (0x079910, i_079910),
    (0x079912, i_079912),
    (0x079914, i_079914),
    (0x079918, i_079918),
    (0x07991C, i_07991c),
    (0x07991E, i_07991e),
    (0x079920, i_079920),
    (0x079924, i_079924),
    (0x079928, i_079928),
    (0x07992A, i_07992a),
    (0x079930, i_079930),
    (0x079934, i_079934),
    (0x079936, i_079936),
    (0x07993A, i_07993a),
    (0x07993E, i_07993e),
    (0x079942, i_079942),
    (0x079946, i_079946),
    (0x07994A, i_07994a),
    (0x07994E, i_07994e),
    (0x079950, i_079950),
    (0x079952, i_079952),
    (0x079954, i_079954),
    (0x079956, i_079956),
    (0x079958, i_079958),
    (0x07995A, i_07995a),
    (0x07995C, i_07995c),
    (0x07995E, i_07995e),
    (0x079964, i_079964),
    (0x079968, i_079968),
    (0x07996C, i_07996c),
    (0x079970, i_079970),
    (0x079976, i_079976),
    (0x079978, i_079978),
    (0x07997C, i_07997c),
    (0x079980, i_079980),
    (0x079982, i_079982),
    (0x079986, i_079986),
    (0x079988, i_079988),
    (0x07998C, i_07998c),
    (0x079990, i_079990),
    (0x079994, i_079994),
    (0x079998, i_079998),
    (0x07999C, i_07999c),
    (0x07999E, i_07999e),
    (0x0799A2, i_0799a2),
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
    (0x079A50, i_079a50),
    (0x079A54, i_079a54),
    (0x079A58, i_079a58),
    (0x079A5C, i_079a5c),
    (0x079A60, i_079a60),
    (0x079A64, i_079a64),
    (0x079A66, i_079a66),
    (0x079A6A, i_079a6a),
    (0x079A6C, i_079a6c),
    (0x079A70, i_079a70),
    (0x079A76, i_079a76),
    (0x079A7C, i_079a7c),
    (0x079A82, i_079a82),
    (0x079A86, i_079a86),
    (0x079A88, i_079a88),
    (0x079A8A, i_079a8a),
    (0x079A90, i_079a90),
    (0x079A94, i_079a94),
    (0x079A9A, i_079a9a),
    (0x079AA0, i_079aa0),
    (0x079AA6, i_079aa6),
    (0x079AAC, i_079aac),
    (0x079AB2, i_079ab2),
    (0x079AB6, i_079ab6),
    (0x079AB8, i_079ab8),
    (0x079ABE, i_079abe),
    (0x079AC4, i_079ac4),
    (0x079ACA, i_079aca),
    (0x079ACE, i_079ace),
    (0x079AD2, i_079ad2),
    (0x079AD4, i_079ad4),
    (0x079AD8, i_079ad8),
    (0x079ADC, i_079adc),
    (0x079AE0, i_079ae0),
    (0x079AE2, i_079ae2),
    (0x079AE4, i_079ae4),
    (0x079AEA, i_079aea),
    (0x079AF0, i_079af0),
    (0x079AF6, i_079af6),
    (0x079AFA, i_079afa),
    (0x079AFC, i_079afc),
    (0x079B02, i_079b02),
    (0x079B08, i_079b08),
    (0x079B0C, i_079b0c),
    (0x079B10, i_079b10),
    (0x079B12, i_079b12),
    (0x079B16, i_079b16),
    (0x079B1A, i_079b1a),
    (0x079B1E, i_079b1e),
    (0x079B20, i_079b20),
    (0x079B22, i_079b22),
    (0x079B28, i_079b28),
    (0x079B2C, i_079b2c),
    (0x079B2E, i_079b2e),
    (0x079B34, i_079b34),
    (0x079B3A, i_079b3a),
    (0x079B3E, i_079b3e),
    (0x079B42, i_079b42),
    (0x079B48, i_079b48),
    (0x079B4A, i_079b4a),
    (0x079B4E, i_079b4e),
    (0x079B54, i_079b54),
    (0x079B58, i_079b58),
    (0x079B5C, i_079b5c),
    (0x079B5E, i_079b5e),
    (0x079B62, i_079b62),
    (0x079B6E, i_079b6e),
    (0x079B74, i_079b74),
    (0x079B78, i_079b78),
    (0x079B7C, i_079b7c),
    (0x079B82, i_079b82),
    (0x079B86, i_079b86),
    (0x079B8A, i_079b8a),
    (0x079B8E, i_079b8e),
    (0x079B92, i_079b92),
    (0x079B98, i_079b98),
    (0x079B9C, i_079b9c),
    (0x079BA0, i_079ba0),
    (0x079BA6, i_079ba6),
    (0x079BAC, i_079bac),
    (0x079BB2, i_079bb2),
    (0x079BB8, i_079bb8),
    (0x079BBE, i_079bbe),
    (0x079BC2, i_079bc2),
    (0x079BC4, i_079bc4),
    (0x079BCA, i_079bca),
    (0x079BD0, i_079bd0),
    (0x079BD4, i_079bd4),
    (0x079BDA, i_079bda),
    (0x079BDE, i_079bde),
    (0x079BE4, i_079be4),
    (0x079BE8, i_079be8),
    (0x079BEE, i_079bee),
    (0x079BF0, i_079bf0),
    (0x079BF6, i_079bf6),
    (0x079BFA, i_079bfa),
    (0x079BFE, i_079bfe),
    (0x079C00, i_079c00),
    (0x079C06, i_079c06),
    (0x079C0A, i_079c0a),
    (0x079C0E, i_079c0e),
    (0x079C10, i_079c10),
    (0x079C12, i_079c12),
    (0x079C18, i_079c18),
    (0x079C1E, i_079c1e),
    (0x079C22, i_079c22),
    (0x079C24, i_079c24),
    (0x079C2A, i_079c2a),
    (0x079C30, i_079c30),
    (0x079C34, i_079c34),
    (0x079C38, i_079c38),
    (0x079C3A, i_079c3a),
    (0x079C3C, i_079c3c),
    (0x079C42, i_079c42),
    (0x079C46, i_079c46),
    (0x079C48, i_079c48),
    (0x079C4E, i_079c4e),
    (0x079C54, i_079c54),
    (0x079C58, i_079c58),
    (0x079C5C, i_079c5c),
    (0x079C60, i_079c60),
    (0x079C64, i_079c64),
    (0x079C68, i_079c68),
    (0x079C6E, i_079c6e),
    (0x079C70, i_079c70),
    (0x079C74, i_079c74),
    (0x079C78, i_079c78),
    (0x079C7C, i_079c7c),
    (0x079C80, i_079c80),
    (0x079C84, i_079c84),
    (0x079C86, i_079c86),
    (0x079C8A, i_079c8a),
    (0x079C8C, i_079c8c),
    (0x079C90, i_079c90),
    (0x079C96, i_079c96),
    (0x079C9C, i_079c9c),
    (0x079CA2, i_079ca2),
    (0x079CA6, i_079ca6),
    (0x079CA8, i_079ca8),
    (0x079CAA, i_079caa),
    (0x079CB0, i_079cb0),
    (0x079CB4, i_079cb4),
    (0x079CBA, i_079cba),
    (0x079CC0, i_079cc0),
    (0x079CC6, i_079cc6),
    (0x079CCC, i_079ccc),
    (0x079CD2, i_079cd2),
    (0x079CD6, i_079cd6),
    (0x079CD8, i_079cd8),
    (0x079CDE, i_079cde),
    (0x079CE4, i_079ce4),
    (0x079CEA, i_079cea),
    (0x079CEE, i_079cee),
    (0x079CF2, i_079cf2),
    (0x079CF4, i_079cf4),
    (0x079CF8, i_079cf8),
    (0x079CFC, i_079cfc),
    (0x079D00, i_079d00),
    (0x079D02, i_079d02),
    (0x079D04, i_079d04),
    (0x079D0A, i_079d0a),
    (0x079D0E, i_079d0e),
    (0x079D12, i_079d12),
    (0x079D18, i_079d18),
    (0x079D1E, i_079d1e),
    (0x079D24, i_079d24),
    (0x079D2A, i_079d2a),
    (0x079D2E, i_079d2e),
    (0x079D30, i_079d30),
    (0x079D36, i_079d36),
    (0x079D3C, i_079d3c),
    (0x079D40, i_079d40),
    (0x079D44, i_079d44),
    (0x079D46, i_079d46),
    (0x079D4A, i_079d4a),
    (0x079D4E, i_079d4e),
    (0x079D52, i_079d52),
    (0x079D54, i_079d54),
    (0x079D56, i_079d56),
    (0x079D5C, i_079d5c),
    (0x079D60, i_079d60),
    (0x079D62, i_079d62),
    (0x079D68, i_079d68),
    (0x079D6E, i_079d6e),
    (0x079D72, i_079d72),
    (0x079D76, i_079d76),
    (0x079D7C, i_079d7c),
    (0x079D7E, i_079d7e),
    (0x079D84, i_079d84),
    (0x079D8A, i_079d8a),
    (0x079D8E, i_079d8e),
    (0x079D92, i_079d92),
    (0x079D94, i_079d94),
    (0x079D98, i_079d98),
    (0x079D9A, i_079d9a),
    (0x079D9E, i_079d9e),
    (0x079DA2, i_079da2),
    (0x079DA6, i_079da6),
    (0x079DAA, i_079daa),
    (0x079DB0, i_079db0),
    (0x079DB4, i_079db4),
    (0x079DB8, i_079db8),
    (0x079DBE, i_079dbe),
    (0x079DC4, i_079dc4),
    (0x079DCA, i_079dca),
    (0x079DD0, i_079dd0),
    (0x079DD6, i_079dd6),
    (0x079DDA, i_079dda),
    (0x079DDC, i_079ddc),
    (0x079DE2, i_079de2),
    (0x079DE8, i_079de8),
    (0x079DEC, i_079dec),
    (0x079DF2, i_079df2),
    (0x079DF6, i_079df6),
    (0x079DFC, i_079dfc),
    (0x079E00, i_079e00),
    (0x079E06, i_079e06),
];
