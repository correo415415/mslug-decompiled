// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C1177C: addq.l #$2, a3
pub fn i_c1177c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1177E;
    cpu.ir = 0x548B;
    cpu.a[3] = cpu.a[3].wrapping_add(2);
    8
}
/// $C1177E: jsr $c11870(pc)
pub fn i_c1177e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11782;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11782);
    cpu.pc = 0xC11870;
    16
}
/// $C11782: jsr $c118b0(pc)
pub fn i_c11782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11786;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11786);
    cpu.pc = 0xC118B0;
    16
}
/// $C11786: tst.b d7
pub fn i_c11786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11788;
    cpu.ir = 0x4A07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11788: beq.b $c11794
pub fn i_c11788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1178A;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11794; return 10; }
    8
}
/// $C1178A: move.w -$7fbc(a4), $ada(a5)
pub fn i_c1178a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11790;
    cpu.ir = 0x3B6C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8044);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xADA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C11790: addq.b #$1, $be2(a5)
pub fn i_c11790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11794;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xBE2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11794: jsr $c1199e(pc)
pub fn i_c11794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11798;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11798);
    cpu.pc = 0xC1199E;
    16
}
/// $C11798: tst.b $bbc(a5)
pub fn i_c11798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1179C;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1179C: bne.w $c117f4
pub fn i_c1179c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC117F4; return 10; }
    8
}
/// $C117A0: move.b -$7fc8(a4), d0
pub fn i_c117a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117A4;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8038);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C117A4: beq.w $c117d0
pub fn i_c117a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC117D0; return 10; }
    8
}
/// $C117A8: lea.l $380061.l, a0
pub fn i_c117a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x380061;
    20
}
/// $C117AE: moveq #$0, d1
pub fn i_c117ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117B0;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C117B0: move.b -$7fc7(a4), d1
pub fn i_c117b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117B4;
    cpu.ir = 0x122C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8039);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C117B4: add.w d1, d1
pub fn i_c117b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117B6;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C117B6: subq.b #$1, d0
pub fn i_c117b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117B8;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C117B8: move.b d0, -$7fc8(a4)
pub fn i_c117b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117BC;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8038);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C117BC: andi.b #$10, d0
pub fn i_c117bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117C0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C117C0: beq.b $c117c8
pub fn i_c117c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117C2;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC117C8; return 10; }
    8
}
/// $C117C2: lea.l $3800e1.l, a0
pub fn i_c117c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x3800E1;
    20
}
/// $C117C8: move.b d0, (a0, d1.w)
pub fn i_c117c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117CC;
    cpu.ir = 0x1180;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C117CC: bra.w $c117f4
pub fn i_c117cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117D0;
    cpu.ir = 0x6000;
    cpu.pc = 0xC117F4;
    10
}
/// $C117D0: moveq #$0, d7
pub fn i_c117d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117D2;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C117D2: lea.l -$7fd4(a4), a6
pub fn i_c117d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117D6;
    cpu.ir = 0x4DEC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF802C);
    cpu.a[6] = ad1;
    16
}
/// $C117D6: tst.b (a6, d7.w)
pub fn i_c117d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117DA;
    cpu.ir = 0x4A36;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $C117DA: beq.b $c117ec
pub fn i_c117da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117DC;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC117EC; return 10; }
    8
}
/// $C117DC: subq.b #$1, (a6, d7.w)
pub fn i_c117dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117E0;
    cpu.ir = 0x5336;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C117E0: move.b d7, -$7fc7(a4)
pub fn i_c117e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117E4;
    cpu.ir = 0x1947;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8039);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C117E4: move.b #$1f, -$7fc8(a4)
pub fn i_c117e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117EA;
    cpu.ir = 0x197C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8038);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C117EA: bra.b $c117f4
pub fn i_c117ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117EC;
    cpu.ir = 0x6008;
    cpu.pc = 0xC117F4;
    10
}
/// $C117EC: addq.b #$1, d7
pub fn i_c117ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117EE;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C117EE: cmpi.b #$2, d7
pub fn i_c117ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117F2;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C117F2: bne.b $c117d6
pub fn i_c117f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117F4;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC117D6; return 10; }
    8
}
/// $C117F4: jsr $c1182a(pc)
pub fn i_c117f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC117F8);
    cpu.pc = 0xC1182A;
    16
}
/// $C117F8: tst.b $bbb(a5)
pub fn i_c117f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117FC;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C117FC: bne.b $c11828
pub fn i_c117fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117FE;
    cpu.ir = 0x662A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11828; return 10; }
    8
}
/// $C117FE: move.b -$7fcc(a4), d0
pub fn i_c117fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11802;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11802: move.b #$ef, d1
pub fn i_c11802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11806;
    cpu.ir = 0x123C;
    let v1: u32 = 0xEF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11806: btst.b #$0, $b8b(a5)
pub fn i_c11806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1180C;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xB8B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C1180C: beq.b $c11816
pub fn i_c1180c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1180E;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11816; return 10; }
    8
}
/// $C1180E: move.b -$7fcb(a4), d0
pub fn i_c1180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11812;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8035);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11812: move.b #$df, d1
pub fn i_c11812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11816;
    cpu.ir = 0x123C;
    let v1: u32 = 0xDF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11816: not.b d0
pub fn i_c11816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11818;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11818: move.b d0, $380041.l
pub fn i_c11818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1181E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380041, v1 as u8);
    16
}
/// $C1181E: jsr $c11190(pc)
pub fn i_c1181e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11822;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11822);
    cpu.pc = 0xC11190;
    16
}
/// $C11822: move.b d1, $380031.l
pub fn i_c11822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11828;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C11828: rts
pub fn i_c11828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1182A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1182A: subq.b #$1, -$7fa3(a4)
pub fn i_c1182a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1182E;
    cpu.ir = 0x532C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1182E: bne.w $c1186e
pub fn i_c1182e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11832;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1186E; return 10; }
    8
}
/// $C11832: move.b #$3c, -$7fa3(a4)
pub fn i_c11832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11838;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11838: addq.b #$1, -$7fa4(a4)
pub fn i_c11838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1183C;
    cpu.ir = 0x522C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1183C: cmpi.b #$3c, -$7fa4(a4)
pub fn i_c1183c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11842;
    cpu.ir = 0xC2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Byte, false);
    16
}
/// $C11842: bcs.w $c1186e
pub fn i_c11842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11846;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1186E; return 10; }
    8
}
/// $C11846: clr.b -$7fa4(a4)
pub fn i_c11846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1184A;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1184A: addq.b #$1, -$7fa5(a4)
pub fn i_c1184a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1184E;
    cpu.ir = 0x522C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1184E: cmpi.b #$3c, -$7fa5(a4)
pub fn i_c1184e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11854;
    cpu.ir = 0xC2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Byte, false);
    16
}
/// $C11854: bcs.w $c1186e
pub fn i_c11854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11858;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1186E; return 10; }
    8
}
/// $C11858: clr.b -$7fa5(a4)
pub fn i_c11858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1185C;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1185C: addq.b #$4, -$7fa6(a4)
pub fn i_c1185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11860;
    cpu.ir = 0x582C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11860: cmpi.b #$60, -$7fa6(a4)
pub fn i_c11860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11866;
    cpu.ir = 0xC2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x60, Size::Byte, false);
    16
}
/// $C11866: bcs.w $c1186e
pub fn i_c11866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1186A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1186E; return 10; }
    8
}
/// $C1186A: clr.b -$7fa6(a4)
pub fn i_c1186a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1186E;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1186E: rts
pub fn i_c1186e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11870;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11870: and.b d2, d3
pub fn i_c11870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11872;
    cpu.ir = 0xC602;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11872: beq.b $c118ae
pub fn i_c11872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11874;
    cpu.ir = 0x673A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC118AE; return 10; }
    8
}
/// $C11874: moveq #$1, d7
pub fn i_c11874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11876;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11876: add.b d7, (a6, d4.w)
pub fn i_c11876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1187A;
    cpu.ir = 0xDF36;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[7] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C1187A: add.b d7, $8(a6, d4.w)
pub fn i_c1187a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1187E;
    cpu.ir = 0xDF36;
    let ad1 = cpu.a[6].wrapping_add(0x8).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[7] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C1187E: tst.b $bbc(a5)
pub fn i_c1187e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11882;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C11882: bne.b $c1188c
pub fn i_c11882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11884;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1188C; return 10; }
    8
}
/// $C11884: add.l d7, (a2, d5.w)
pub fn i_c11884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11888;
    cpu.ir = 0xDFB2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C11888: add.l d7, (a1, d5.w)
pub fn i_c11888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1188C;
    cpu.ir = 0xDFB1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C1188C: move.b (a6, d4.w), d0
pub fn i_c1188c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11890;
    cpu.ir = 0x1036;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C11890: sub.b (a3), d0
pub fn i_c11890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11892;
    cpu.ir = 0x9013;
    let v2 = bus.read8(cpu.a[3]) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11892: bcs.b $c118ae
pub fn i_c11892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11894;
    cpu.ir = 0x651A;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC118AE; return 10; }
    8
}
/// $C11894: move.b d0, (a6, d4.w)
pub fn i_c11894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11898;
    cpu.ir = 0x1D80;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C11898: sub.b d0, d0
pub fn i_c11898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1189A;
    cpu.ir = 0x9000;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C1189A: move.b $1(a3), d0
pub fn i_c1189a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1189E;
    cpu.ir = 0x102B;
    let ad2 = cpu.a[3].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1189E: move.b -$7fcc(a4), d1
pub fn i_c1189e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118A2;
    cpu.ir = 0x122C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C118A2: abcd.b d1, d0
pub fn i_c118a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118A4;
    cpu.ir = 0xC101;
    let s = cpu.d[1] as u8; let d = cpu.d[0] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[0] = (cpu.d[0] & !0xFF) | r as u32;
    6
}
/// $C118A4: bcc.b $c118aa
pub fn i_c118a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118A6;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC118AA; return 10; }
    8
}
/// $C118A6: move.b #$99, d0
pub fn i_c118a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118AA;
    cpu.ir = 0x103C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C118AA: move.b d0, -$7fcc(a4)
pub fn i_c118aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118AE;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C118AE: rts
pub fn i_c118ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C118B0: btst.b #$2, -$7fde(a4)
pub fn i_c118b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118B6;
    cpu.ir = 0x82C;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8022);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C118B6: beq.b $c118da
pub fn i_c118b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118B8;
    cpu.ir = 0x6722;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC118DA; return 10; }
    8
}
/// $C118B8: moveq #$1, d7
pub fn i_c118b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118BA;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C118BA: tst.b $bbc(a5)
pub fn i_c118ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118BE;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C118BE: bne.b $c118c8
pub fn i_c118be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118C0;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC118C8; return 10; }
    8
}
/// $C118C0: add.l d7, $c(a2)
pub fn i_c118c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118C4;
    cpu.ir = 0xDFAA;
    let ad1 = cpu.a[2].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C118C4: add.l d7, $c(a1)
pub fn i_c118c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118C8;
    cpu.ir = 0xDFA9;
    let ad1 = cpu.a[1].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C118C8: sub.b d0, d0
pub fn i_c118c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118CA;
    cpu.ir = 0x9000;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C118CA: move.b -$7fcc(a4), d0
pub fn i_c118ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118CE;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C118CE: abcd.b d7, d0
pub fn i_c118ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118D0;
    cpu.ir = 0xC107;
    let s = cpu.d[7] as u8; let d = cpu.d[0] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[0] = (cpu.d[0] & !0xFF) | r as u32;
    6
}
/// $C118D0: bcc.b $c118d6
pub fn i_c118d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118D2;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC118D6; return 10; }
    8
}
/// $C118D2: move.b #$99, d0
pub fn i_c118d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118D6;
    cpu.ir = 0x103C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C118D6: move.b d0, -$7fcc(a4)
pub fn i_c118d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118DA;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C118DA: rts
pub fn i_c118da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C118DC: move.b d0, $3a001d.l
pub fn i_c118dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118E2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C118E2: lea.l $d08000.l, a4
pub fn i_c118e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118E8;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C118E8: lea.l $aba(a5), a1
pub fn i_c118e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118EC;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xABA);
    cpu.a[1] = ad1;
    16
}
/// $C118EC: move.b $ab4(a5), d0
pub fn i_c118ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118F0;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAB4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C118F0: jsr $c118fe(pc)
pub fn i_c118f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC118F4);
    cpu.pc = 0xC118FE;
    16
}
/// $C118F4: jsr $c118fe(pc)
pub fn i_c118f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC118F8);
    cpu.pc = 0xC118FE;
    16
}
/// $C118F8: jsr $c11190(pc)
pub fn i_c118f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC118FC);
    cpu.pc = 0xC11190;
    16
}
/// $C118FC: rts
pub fn i_c118fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C118FE: move.b (a1)+, d2
pub fn i_c118fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11900;
    cpu.ir = 0x1419;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C11900: ror.b #$1, d0
pub fn i_c11900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11902;
    cpu.ir = 0xE218;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 3, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11902: bcc.b $c11960
pub fn i_c11902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11904;
    cpu.ir = 0x645C;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC11960; return 10; }
    8
}
/// $C11904: moveq #$1, d4
pub fn i_c11904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11906;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11906: move.b $300001.l, d5
pub fn i_c11906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1190C;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1190C: andi.b #$40, d5
pub fn i_c1190c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11910;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11910: beq.b $c11922
pub fn i_c11910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11912;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11922; return 10; }
    8
}
/// $C11912: move.b -$7fcc(a4), d3
pub fn i_c11912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11916;
    cpu.ir = 0x162C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11916: sub.b d5, d5
pub fn i_c11916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11918;
    cpu.ir = 0x9A05;
    let v1 = cpu.d[5] & 0xFF;
    let h1 = cpu.d[5] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    4
}
/// $C11918: sbcd.b d4, d3
pub fn i_c11918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1191A;
    cpu.ir = 0x8704;
    let s = cpu.d[4] as u8; let d = cpu.d[3] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = sbcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[3] = (cpu.d[3] & !0xFF) | r as u32;
    6
}
/// $C1191A: bcc.b $c1191e
pub fn i_c1191a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1191C;
    cpu.ir = 0x6402;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1191E; return 10; }
    8
}
/// $C1191C: moveq #$0, d3
pub fn i_c1191c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1191E;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1191E: move.b d3, -$7fcc(a4)
pub fn i_c1191e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11922;
    cpu.ir = 0x1943;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11922: movea.l -$7ef6(a4), a2
pub fn i_c11922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11926;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810A);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11926: jsr $c11990(pc)
pub fn i_c11926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1192A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1192A);
    cpu.pc = 0xC11990;
    16
}
/// $C1192A: movea.l -$7ef2(a4), a2
pub fn i_c1192a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1192E;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810E);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C1192E: jsr $c11990(pc)
pub fn i_c1192e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11932;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11932);
    cpu.pc = 0xC11990;
    16
}
/// $C11932: movea.l -$7e5c(a4), a2
pub fn i_c11932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11936;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81A4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11936: jsr $c11990(pc)
pub fn i_c11936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1193A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1193A);
    cpu.pc = 0xC11990;
    16
}
/// $C1193A: movea.l -$7eee(a4), a2
pub fn i_c1193a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1193E;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8112);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C1193E: lea.l $240(a2), a2
pub fn i_c1193e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11942;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x240);
    cpu.a[2] = ad1;
    16
}
/// $C11942: jsr $c11990(pc)
pub fn i_c11942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11946;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11946);
    cpu.pc = 0xC11990;
    16
}
/// $C11946: movea.l -$7eea(a4), a2
pub fn i_c11946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1194A;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8116);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C1194A: lea.l $c0(a2), a2
pub fn i_c1194a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1194E;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0xC0);
    cpu.a[2] = ad1;
    16
}
/// $C1194E: jsr $c11990(pc)
pub fn i_c1194e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11952;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11952);
    cpu.pc = 0xC11990;
    16
}
/// $C11952: movea.l -$7e58(a4), a2
pub fn i_c11952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11956;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81A8);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11956: moveq #$0, d4
pub fn i_c11956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11958;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11958: move.b -$7fa6(a4), d4
pub fn i_c11958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1195C;
    cpu.ir = 0x182C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1195C: addq.l #$1, (a2, d4.w)
pub fn i_c1195c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11960;
    cpu.ir = 0x52B2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C11960: addq.l #$4, a0
pub fn i_c11960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11962;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C11962: rts
pub fn i_c11962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11964;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11964: move.b d0, $3a001d.l
pub fn i_c11964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1196A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C1196A: lea.l $d08000.l, a4
pub fn i_c1196a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11970;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11970: lea.l $aba(a5), a1
pub fn i_c11970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11974;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xABA);
    cpu.a[1] = ad1;
    16
}
/// $C11974: lea.l $ab0(a5), a3
pub fn i_c11974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11978;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0xAB0);
    cpu.a[3] = ad1;
    16
}
/// $C11978: jsr $c11986(pc)
pub fn i_c11978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1197C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1197C);
    cpu.pc = 0xC11986;
    16
}
/// $C1197C: jsr $c11986(pc)
pub fn i_c1197c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11980;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11980);
    cpu.pc = 0xC11986;
    16
}
/// $C11980: jsr $c11190(pc)
pub fn i_c11980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11984;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11984);
    cpu.pc = 0xC11190;
    16
}
/// $C11984: rts
pub fn i_c11984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11986;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11986: move.b (a1)+, d2
pub fn i_c11986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11988;
    cpu.ir = 0x1419;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C11988: move.b (a3)+, d4
pub fn i_c11988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1198A;
    cpu.ir = 0x181B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1198A: beq.b $c11960
pub fn i_c1198a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1198C;
    cpu.ir = 0x67D4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11960; return 10; }
    8
}
/// $C1198C: bra.w $c11906
pub fn i_c1198c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11990;
    cpu.ir = 0x6000;
    cpu.pc = 0xC11906;
    10
}
/// $C11990: addq.l #$1, $4(a2)
pub fn i_c11990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11994;
    cpu.ir = 0x52AA;
    let ad1 = cpu.a[2].wrapping_add(0x4);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11994: tst.b d2
pub fn i_c11994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11996;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11996: beq.b $c1199c
pub fn i_c11996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11998;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1199C; return 10; }
    8
}
/// $C11998: addq.l #$1, $8(a2)
pub fn i_c11998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1199C;
    cpu.ir = 0x52AA;
    let ad1 = cpu.a[2].wrapping_add(0x8);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C1199C: rts
pub fn i_c1199c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1199E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1199E: tst.b $be1(a5)
pub fn i_c1199e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119A2;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBE1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C119A2: bne.w $c119e0
pub fn i_c119a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119E0; return 10; }
    8
}
/// $C119A6: cmpi.b #$2, $a81(a5)
pub fn i_c119a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119AC;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $C119AC: bcs.w $c119e0
pub fn i_c119ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119B0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC119E0; return 10; }
    8
}
/// $C119B0: tst.b $bdb(a5)
pub fn i_c119b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119B4;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBDB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C119B4: bne.b $c119e0
pub fn i_c119b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119B6;
    cpu.ir = 0x662A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119E0; return 10; }
    8
}
/// $C119B6: tst.b $be2(a5)
pub fn i_c119b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119BA;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBE2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C119BA: beq.b $c119e0
pub fn i_c119ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119BC;
    cpu.ir = 0x6724;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC119E0; return 10; }
    8
}
/// $C119BC: subq.b #$1, $be2(a5)
pub fn i_c119bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119C0;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xBE2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C119C0: tst.b $b8d(a5)
pub fn i_c119c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119C4;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C119C4: beq.b $c119d0
pub fn i_c119c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119C6;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC119D0; return 10; }
    8
}
/// $C119C6: move.b #$7f, $320000.l
pub fn i_c119c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119CE;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C119CE: bra.b $c119e0
pub fn i_c119ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119D0;
    cpu.ir = 0x6010;
    cpu.pc = 0xC119E0;
    10
}
/// $C119D0: jsr $134.w
pub fn i_c119d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119D4;
    cpu.ir = 0x4EB8;
    cpu.push32(bus, 0xC119D4);
    cpu.pc = 0x134;
    16
}
/// $C119D4: lea.l $d08000.l, a4
pub fn i_c119d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119DA;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C119DA: lea.l $10f300.l, a5
pub fn i_c119da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119E0;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C119E0: rts
pub fn i_c119e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C119E2: cmpi.b #$1, $a81(a5)
pub fn i_c119e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119E8;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C119E8: beq.b $c119f2
pub fn i_c119e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119EA;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC119F2; return 10; }
    8
}
/// $C119EA: cmpi.b #$1, $aaf(a5)
pub fn i_c119ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119F0;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C119F0: bne.b $c119fe
pub fn i_c119f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119F2;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C119F2: move.b $380000.l, d0
pub fn i_c119f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119F8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C119F8: andi.b #$30, d0
pub fn i_c119f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119FC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C119FC: beq.b $c11a00
pub fn i_c119fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC119FE;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11A00; return 10; }
    8
}
/// $C119FE: rts
pub fn i_c119fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11A00: btst.b #$1, $bdd(a5)
pub fn i_c11a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A06;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C11A06: beq.b $c119fe
pub fn i_c11a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A08;
    cpu.ir = 0x67F6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A08: lea.l $d08000.l, a4
pub fn i_c11a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A0E;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11A0E: lea.l $800000.l, a6
pub fn i_c11a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A14;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C11A14: lea.l $40(a6), a0
pub fn i_c11a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A18;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x40);
    cpu.a[0] = ad1;
    16
}
/// $C11A18: lea.l $c11b62(pc), a1
pub fn i_c11a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11B62;
    16
}
/// $C11A1C: moveq #$7, d7
pub fn i_c11a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A1E;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11A1E: move.w (a0), d0
pub fn i_c11a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A20;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C11A20: cmp.b (a1)+, d0
pub fn i_c11a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A22;
    cpu.ir = 0xB019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    8
}
/// $C11A22: bne.b $c119fe
pub fn i_c11a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A24;
    cpu.ir = 0x66DA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A24: addq.l #$4, a0
pub fn i_c11a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A26;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C11A26: dbra d7, $c11a1e
pub fn i_c11a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A2A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11A2A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11A2A; return 14; }
    cpu.pc = 0xC11A1E;
    10
}
/// $C11A2A: move.w $10(a6), d0
pub fn i_c11a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A2E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C11A2E: andi.b #$f0, d0
pub fn i_c11a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A32;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11A32: cmpi.b #$20, d0
pub fn i_c11a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A36;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $C11A36: bne.b $c119fe
pub fn i_c11a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A38;
    cpu.ir = 0x66C6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A38: move.l -$7e54(a4), d0
pub fn i_c11a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A3C;
    cpu.ir = 0x202C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81AC);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C11A3C: cmp.l $70(a6), d0
pub fn i_c11a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A40;
    cpu.ir = 0xB0AE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $C11A40: bne.b $c119fe
pub fn i_c11a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A42;
    cpu.ir = 0x66BC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A42: move.w -$7e50(a4), d0
pub fn i_c11a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A46;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81B0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C11A46: bmi.w $c11a54
pub fn i_c11a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A4A;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC11A54; return 10; }
    8
}
/// $C11A4A: cmp.w $74(a6), d0
pub fn i_c11a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A4E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C11A4E: bhi.b $c119fe
pub fn i_c11a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A50;
    cpu.ir = 0x62AE;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A50: bra.w $c11a94
pub fn i_c11a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A54;
    cpu.ir = 0x6000;
    cpu.pc = 0xC11A94;
    10
}
/// $C11A54: move.w $74(a6), d0
pub fn i_c11a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A58;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C11A58: cmpi.w #$a, d0
pub fn i_c11a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A5C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $C11A5C: bcc.b $c119fe
pub fn i_c11a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A5E;
    cpu.ir = 0x64A0;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC119FE; return 10; }
    8
}
/// $C11A5E: move.w #$2700, sr
pub fn i_c11a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A62;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C11A62: moveq #$0, d7
pub fn i_c11a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A64;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11A64: move.b d7, $3a0019.l
pub fn i_c11a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A6A;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C11A6A: move.b d7, $380011.l
pub fn i_c11a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A70;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $C11A70: move.b d7, $3a0005.l
pub fn i_c11a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A76;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $C11A76: move.b d7, $3a0017.l
pub fn i_c11a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A7C;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $C11A7C: move.b d7, $3a001d.l
pub fn i_c11a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A82;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11A82: move.w d0, -$7e50(a4)
pub fn i_c11a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A86;
    cpu.ir = 0x3940;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81B0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C11A86: move.b d7, $3a000d.l
pub fn i_c11a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A8C;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C11A8C: move.w d0, d1
pub fn i_c11a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A8E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11A8E: addq.w #$1, d1
pub fn i_c11a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A90;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11A90: move.w d1, $74(a6)
pub fn i_c11a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A94;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C11A94: move.w #$2700, sr
pub fn i_c11a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A98;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C11A98: moveq #$0, d7
pub fn i_c11a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A9A;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11A9A: move.b d7, $bdd(a5)
pub fn i_c11a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11A9E;
    cpu.ir = 0x1B47;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11A9E: move.b d7, $3a0019.l
pub fn i_c11a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AA4;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C11AA4: move.b d7, $380011.l
pub fn i_c11aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AAA;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $C11AAA: move.b d7, $3a0005.l
pub fn i_c11aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AB0;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $C11AB0: move.b d7, $3a0017.l
pub fn i_c11ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AB6;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $C11AB6: move.w #$2800, d1
pub fn i_c11ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ABA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11ABA: mulu.w d0, d1
pub fn i_c11aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ABC;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C11ABC: lea.l $100(a6), a0
pub fn i_c11abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AC0;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x100);
    cpu.a[0] = ad1;
    16
}
/// $C11AC0: adda.l d1, a0
pub fn i_c11ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AC2;
    cpu.ir = 0xD1C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C11AC2: lea.l $300001.l, a3
pub fn i_c11ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AC8;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x300001;
    20
}
/// $C11AC8: movem.l d0/a0-a6, -(a7)
pub fn i_c11ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ACC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C11ACC: move.b d0, (a3)
pub fn i_c11acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ACE;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C11ACE: jsr $c11c42(pc)
pub fn i_c11ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11AD2);
    cpu.pc = 0xC11C42;
    16
}
/// $C11AD2: movem.l (a7)+, d0/a0-a6
pub fn i_c11ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AD6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C11AD6: move.l $ad2(a5), (a0)+
pub fn i_c11ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ADA;
    cpu.ir = 0x20ED;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C11ADA: move.l $ad6(a5), (a0)+
pub fn i_c11ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ADE;
    cpu.ir = 0x20ED;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C11ADE: move.l #$25, d7
pub fn i_c11ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AE4;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x25;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11AE4: lea.l -$7ee6(a4), a1
pub fn i_c11ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AE8;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF811A);
    cpu.a[1] = ad1;
    16
}
/// $C11AE8: jsr $c11b58(pc)
pub fn i_c11ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11AEC);
    cpu.pc = 0xC11B58;
    16
}
/// $C11AEC: move.l #$90, d7
pub fn i_c11aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AF2;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x90;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11AF2: lea.l -$7e00(a4), a1
pub fn i_c11af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AF6;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8200);
    cpu.a[1] = ad1;
    16
}
/// $C11AF6: jsr $c11b58(pc)
pub fn i_c11af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11AFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11AFA);
    cpu.pc = 0xC11B58;
    16
}
/// $C11AFA: move.l #$10f0, d7
pub fn i_c11afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B00;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x10F0;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11B00: lea.l $320(a4), a1
pub fn i_c11b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B04;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0x320);
    cpu.a[1] = ad1;
    16
}
/// $C11B04: jsr $c11b58(pc)
pub fn i_c11b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11B08);
    cpu.pc = 0xC11B58;
    16
}
/// $C11B08: move.b d7, $3a0007.l
pub fn i_c11b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B0E;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0007, v1 as u8);
    16
}
/// $C11B0E: move.b d7, $3a0015.l
pub fn i_c11b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B14;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0015, v1 as u8);
    16
}
/// $C11B14: move.l #$eee0000, $400002.l
pub fn i_c11b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B1E;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C11B1E: jsr $c12a0e(pc)
pub fn i_c11b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11B22);
    cpu.pc = 0xC12A0E;
    16
}
/// $C11B22: lea.l $c00(a5), a0
pub fn i_c11b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B26;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C11B26: lea.l $c11b6a(pc), a1
pub fn i_c11b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11B6A;
    16
}
/// $C11B2A: tst.b $a83(a5)
pub fn i_c11b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B2E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C11B2E: beq.b $c11b34
pub fn i_c11b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B30;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11B34; return 10; }
    8
}
/// $C11B30: lea.l $c11b7a(pc), a1
pub fn i_c11b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11B7A;
    16
}
/// $C11B34: move.l a1, (a0)+
pub fn i_c11b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B36;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C11B36: move.l a0, $abe(a5)
pub fn i_c11b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B3A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C11B3A: clr.b $abe(a5)
pub fn i_c11b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B3E;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xABE);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11B3E: jsr $c17290(pc)
pub fn i_c11b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11B42);
    cpu.pc = 0xC17290;
    16
}
/// $C11B42: move.l #$f4240, d7
pub fn i_c11b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B48;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0xF4240;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11B48: move.b d0, $300001.l
pub fn i_c11b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B4E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C11B4E: subq.l #$1, d7
pub fn i_c11b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B50;
    cpu.ir = 0x5387;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[7] = r2;
    4
}
/// $C11B50: bne.b $c11b48
pub fn i_c11b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B52;
    cpu.ir = 0x66F6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11B48; return 10; }
    8
}
/// $C11B52: jmp $c102d2.l
pub fn i_c11b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B58;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C11B58: move.b d0, (a3)
pub fn i_c11b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B5A;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C11B5A: move.w (a1)+, (a0)+
pub fn i_c11b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B5C;
    cpu.ir = 0x30D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C11B5C: subq.l #$1, d7
pub fn i_c11b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B5E;
    cpu.ir = 0x5387;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[7] = r2;
    4
}
/// $C11B5E: bne.b $c11b58
pub fn i_c11b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B60;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11B58; return 10; }
    8
}
/// $C11B60: rts
pub fn i_c11b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11B62: trap #$5
pub fn i_c11b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B64;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C11B64: dc.w $4f2d
pub fn i_c11b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B66;
    cpu.ir = 0x4F2D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C11B66: ?
pub fn i_c11b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B68;
    cpu.ir = 0x4745;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C11B68: chk.w d0, d7
pub fn i_c11b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B6A;
    cpu.ir = 0x4F80;
    let v1 = cpu.d[0] & 0xFFFF;
    let bd = v1 as u16 as i16 as i32; let vv = cpu.d[7] as u16 as i16 as i32;
    cpu.sr.set(StatusRegister::Z, false); cpu.sr.set(StatusRegister::V, false); cpu.sr.set(StatusRegister::C, false);
    if vv < 0 { cpu.sr.set(StatusRegister::N, true); cpu.enter_exception(bus, Exception::Chk); return 40; }
    if vv > bd { cpu.sr.set(StatusRegister::N, false); cpu.enter_exception(bus, Exception::Chk); return 40; }
    cpu.sr.set(StatusRegister::N, false);
    10
}
/// $C11B6A: ori.b #$cc, d3
pub fn i_c11b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B6E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xCC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11B6E: movep.w $5341(a0), d0
pub fn i_c11b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B72;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5341);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C11B72: addq.w #$3, d5
pub fn i_c11b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B74;
    cpu.ir = 0x5645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11B74: movea.l a7, a0
pub fn i_c11b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11B76;
    cpu.ir = 0x204F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C11BB0: move.b d0, $3a001d.l
pub fn i_c11bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BB6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11BB6: lea.l $d08000.l, a4
pub fn i_c11bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BBC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11BBC: lea.l $10f300.l, a5
pub fn i_c11bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BC2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C11BC2: cmpi.b #$1, $aba(a5)
pub fn i_c11bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BC8;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xABA);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C11BC8: beq.b $c11bd2
pub fn i_c11bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BCA;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11BD2; return 10; }
    8
}
/// $C11BCA: cmpi.b #$1, $abb(a5)
pub fn i_c11bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BD0;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xABB);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C11BD0: bne.b $c11be2
pub fn i_c11bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BD2;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11BE2; return 10; }
    8
}
/// $C11BD2: subq.b #$1, -$7fb4(a4)
pub fn i_c11bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BD6;
    cpu.ir = 0x532C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF804C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11BD6: bpl.b $c11be2
pub fn i_c11bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BD8;
    cpu.ir = 0x6A0A;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC11BE2; return 10; }
    8
}
/// $C11BD8: move.b #$3b, -$7fb4(a4)
pub fn i_c11bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BDE;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF804C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11BDE: addq.l #$1, -$7fb8(a4)
pub fn i_c11bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BE2;
    cpu.ir = 0x52AC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8048);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11BE2: move.l $ab6(a5), $aba(a5)
pub fn i_c11be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BE8;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAB6);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xABA);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C11BE8: jsr $c11190(pc)
pub fn i_c11be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11BEC);
    cpu.pc = 0xC11190;
    16
}
/// $C11BEC: rts
pub fn i_c11bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11BEE: lea.l $d08000.l, a4
pub fn i_c11bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BF4;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11BF4: move.b #$1, $bbf(a5)
pub fn i_c11bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11BFA;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11BFA: move.b d0, $3a001d.l
pub fn i_c11bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C00;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11C00: move.l -$7fb8(a4), d0
pub fn i_c11c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C04;
    cpu.ir = 0x202C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8048);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C11C04: movea.l -$7ef6(a4), a2
pub fn i_c11c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C08;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810A);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C08: add.l d0, $c(a2)
pub fn i_c11c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C0C;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C0C: movea.l -$7ef2(a4), a2
pub fn i_c11c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C10;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810E);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C10: add.l d0, $c(a2)
pub fn i_c11c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C14;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C14: movea.l -$7e5c(a4), a2
pub fn i_c11c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C18;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81A4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C18: add.l d0, $c(a2)
pub fn i_c11c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C1C;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C1C: movea.l -$7eee(a4), a2
pub fn i_c11c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C20;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8112);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C20: add.l d0, $24c(a2)
pub fn i_c11c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C24;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0x24C);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C24: movea.l -$7eea(a4), a2
pub fn i_c11c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C28;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8116);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11C28: add.l d0, $cc(a2)
pub fn i_c11c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C2C;
    cpu.ir = 0xD1AA;
    let ad1 = cpu.a[2].wrapping_add(0xCC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C11C2C: clr.l -$7fb8(a4)
pub fn i_c11c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C30;
    cpu.ir = 0x42AC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8048);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C11C30: move.b #$3b, -$7fb4(a4)
pub fn i_c11c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C36;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF804C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C36: clr.b $bbf(a5)
pub fn i_c11c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C3A;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11C3A: move.b d0, $3a000d.l
pub fn i_c11c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C40;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C11C40: rts
pub fn i_c11c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11C42: moveq #$3, d0
pub fn i_c11c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C44;
    cpu.ir = 0x7003;
    cpu.d[0] = 0x3;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11C44: jsr $c11d3a(pc)
pub fn i_c11c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C48);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11C48: bsr.w $c17066
pub fn i_c11c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C4C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11C4C);
    cpu.pc = 0xC17066;
    18
}
/// $C11C4C: moveq #$1, d0
pub fn i_c11c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C4E;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11C4E: jsr $c11d3a(pc)
pub fn i_c11c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C52);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11C52: bsr.w $c17066
pub fn i_c11c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C56;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11C56);
    cpu.pc = 0xC17066;
    18
}
/// $C11C56: lea.l $10fdd2.l, a0
pub fn i_c11c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C5C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDD2;
    20
}
/// $C11C5C: jsr $c11ca6(pc)
pub fn i_c11c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C60);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C60: move.b d0, $6(a0)
pub fn i_c11c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C64;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C64: jsr $c11ca6(pc)
pub fn i_c11c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C68);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C68: move.b d0, $5(a0)
pub fn i_c11c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C6C;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C6C: jsr $c11ca6(pc)
pub fn i_c11c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C70);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C70: move.b d0, $4(a0)
pub fn i_c11c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C74;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C74: jsr $c11ca6(pc)
pub fn i_c11c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C78);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C78: move.b d0, $2(a0)
pub fn i_c11c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C7C;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C7C: jsr $c11ca6(pc)
pub fn i_c11c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C80);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C80: move.b d0, d1
pub fn i_c11c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C82;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11C82: andi.b #$f, d1
pub fn i_c11c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C86;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11C86: move.b d1, $3(a0)
pub fn i_c11c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C8A;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C8A: lsr.b #$4, d0
pub fn i_c11c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C8C;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C11C8C: cmpi.b #$a, d0
pub fn i_c11c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C90;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C11C90: bcs.b $c11c94
pub fn i_c11c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C92;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC11C94; return 10; }
    8
}
/// $C11C92: addq.b #$6, d0
pub fn i_c11c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C94;
    cpu.ir = 0x5C00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11C94: move.b d0, $1(a0)
pub fn i_c11c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C98;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11C98: jsr $c11ca6(pc)
pub fn i_c11c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11C9C);
    cpu.pc = 0xC11CA6;
    16
}
/// $C11C9C: move.b d0, (a0)
pub fn i_c11c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11C9E;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $C11C9E: moveq #$3, d7
pub fn i_c11c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CA0;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11CA0: jsr $c11ca8(pc)
pub fn i_c11ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CA4);
    cpu.pc = 0xC11CA8;
    16
}
/// $C11CA4: rts
pub fn i_c11ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11CA6: moveq #$7, d7
pub fn i_c11ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CA8;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11CA8: move.b $320001.l, d2
pub fn i_c11ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CAE;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x320001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C11CAE: jsr $c11d6c(pc)
pub fn i_c11cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CB2);
    cpu.pc = 0xC11D6C;
    16
}
/// $C11CB2: andi.b #$80, d2
pub fn i_c11cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CB6;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11CB6: lsr.b #$1, d0
pub fn i_c11cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CB8;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11CB8: or.b d2, d0
pub fn i_c11cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CBA;
    cpu.ir = 0x8002;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11CBA: dbra d7, $c11ca8
pub fn i_c11cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CBE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11CBE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11CBE; return 14; }
    cpu.pc = 0xC11CA8;
    10
}
/// $C11CBE: rts
pub fn i_c11cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11CC0: lea.l $10f300.l, a5
pub fn i_c11cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CC6;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C11CC6: jsr $c11eca(pc)
pub fn i_c11cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CCA);
    cpu.pc = 0xC11ECA;
    16
}
/// $C11CCA: tst.b d7
pub fn i_c11cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CCC;
    cpu.ir = 0x4A07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11CCC: bne.w $c11d14
pub fn i_c11ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11D14; return 10; }
    8
}
/// $C11CD0: moveq #$1, d0
pub fn i_c11cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CD2;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11CD2: jsr $c11d3a(pc)
pub fn i_c11cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CD6);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11CD6: move.b $ad8(a5), d0
pub fn i_c11cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CDA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CDA: jsr $c11d16(pc)
pub fn i_c11cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CDE);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CDE: move.b $ad7(a5), d0
pub fn i_c11cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CE2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CE2: jsr $c11d16(pc)
pub fn i_c11ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CE6);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CE6: move.b $ad6(a5), d0
pub fn i_c11ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CEA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CEA: jsr $c11d16(pc)
pub fn i_c11cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CEE);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CEE: move.b $ad4(a5), d0
pub fn i_c11cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CF2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CF2: jsr $c11d16(pc)
pub fn i_c11cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CF6);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CF6: move.b $a01(a5), d0
pub fn i_c11cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CFA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA01);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11CFA: jsr $c11d16(pc)
pub fn i_c11cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11CFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11CFE);
    cpu.pc = 0xC11D16;
    16
}
/// $C11CFE: move.b $ad2(a5), d0
pub fn i_c11cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D02;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11D02: jsr $c11d16(pc)
pub fn i_c11d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D06);
    cpu.pc = 0xC11D16;
    16
}
/// $C11D06: moveq #$2, d0
pub fn i_c11d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D08;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D08: jsr $c11d3a(pc)
pub fn i_c11d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D0C);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11D0C: moveq #$0, d0
pub fn i_c11d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D0E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D0E: jsr $c11d3a(pc)
pub fn i_c11d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D12);
    cpu.pc = 0xC11D3A;
    16
}
/// $C11D12: moveq #$0, d7
pub fn i_c11d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D14;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D14: rts
pub fn i_c11d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D16: moveq #$7, d7
pub fn i_c11d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D18;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D18: move.b d0, d1
pub fn i_c11d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D1A;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D1A: andi.b #$1, d1
pub fn i_c11d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D1E;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D1E: jsr $c11d7e(pc)
pub fn i_c11d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D22);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D22: ori.b #$2, d1
pub fn i_c11d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D26;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D26: jsr $c11d7e(pc)
pub fn i_c11d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D2A);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D2A: eori.b #$2, d1
pub fn i_c11d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D2E;
    cpu.ir = 0xA01;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D2E: jsr $c11d7e(pc)
pub fn i_c11d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D32);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D32: lsr.b #$1, d0
pub fn i_c11d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D34;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11D34: dbra d7, $c11d18
pub fn i_c11d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D38;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11D38; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11D38; return 14; }
    cpu.pc = 0xC11D18;
    10
}
/// $C11D38: rts
pub fn i_c11d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D3A: moveq #$3, d7
pub fn i_c11d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D3C;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D3C: move.b d0, d1
pub fn i_c11d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D3E;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D3E: andi.b #$1, d1
pub fn i_c11d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D42;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D42: jsr $c11d7e(pc)
pub fn i_c11d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D46);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D46: ori.b #$2, d1
pub fn i_c11d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D4A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D4A: jsr $c11d7e(pc)
pub fn i_c11d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D4E);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D4E: eori.b #$2, d1
pub fn i_c11d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D52;
    cpu.ir = 0xA01;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11D52: jsr $c11d7e(pc)
pub fn i_c11d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D56);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D56: lsr.b #$1, d0
pub fn i_c11d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D58;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11D58: dbra d7, $c11d3c
pub fn i_c11d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D5C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11D5C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11D5C; return 14; }
    cpu.pc = 0xC11D3C;
    10
}
/// $C11D5C: move.b #$4, d1
pub fn i_c11d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D60;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D60: jsr $c11d7e(pc)
pub fn i_c11d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D64);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D64: moveq #$0, d1
pub fn i_c11d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D66;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D66: jsr $c11d7e(pc)
pub fn i_c11d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D6A);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D6A: rts
pub fn i_c11d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D6C: move.b #$2, d1
pub fn i_c11d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D70;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D70: jsr $c11d7e(pc)
pub fn i_c11d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D74);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D74: move.b #$0, d1
pub fn i_c11d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D78;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11D78: jsr $c11d7e(pc)
pub fn i_c11d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D7C);
    cpu.pc = 0xC11D7E;
    16
}
/// $C11D7C: rts
pub fn i_c11d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D7E: move.b d1, $380051.l
pub fn i_c11d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D84;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380051, v1 as u8);
    16
}
/// $C11D84: rts
pub fn i_c11d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11D86: moveq #$3, d0
pub fn i_c11d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D88;
    cpu.ir = 0x7003;
    cpu.d[0] = 0x3;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D88: jsr $c11e7e(pc)
pub fn i_c11d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D8C);
    cpu.pc = 0xC11E7E;
    16
}
/// $C11D8C: bsr.w $c17066
pub fn i_c11d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D90;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11D90);
    cpu.pc = 0xC17066;
    18
}
/// $C11D90: moveq #$1, d0
pub fn i_c11d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D92;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11D92: jsr $c11e7e(pc)
pub fn i_c11d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11D96);
    cpu.pc = 0xC11E7E;
    16
}
/// $C11D96: bsr.w $c17066
pub fn i_c11d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11D9A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11D9A);
    cpu.pc = 0xC17066;
    18
}
/// $C11D9A: lea.l $10fdd2.l, a0
pub fn i_c11d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDD2;
    20
}
/// $C11DA0: jsr $c11dea(pc)
pub fn i_c11da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DA4);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DA4: move.b d0, $6(a0)
pub fn i_c11da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DA8;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DA8: jsr $c11dea(pc)
pub fn i_c11da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DAC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DAC);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DAC: move.b d0, $5(a0)
pub fn i_c11dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DB0;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DB0: jsr $c11dea(pc)
pub fn i_c11db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DB4);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DB4: move.b d0, $4(a0)
pub fn i_c11db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DB8;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DB8: jsr $c11dea(pc)
pub fn i_c11db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DBC);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DBC: move.b d0, $2(a0)
pub fn i_c11dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DC0;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DC0: jsr $c11dea(pc)
pub fn i_c11dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DC4);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DC4: move.b d0, d1
pub fn i_c11dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DC6;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11DC6: andi.b #$f, d1
pub fn i_c11dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DCA;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11DCA: move.b d1, $3(a0)
pub fn i_c11dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DCE;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DCE: lsr.b #$4, d0
pub fn i_c11dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DD0;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C11DD0: cmpi.b #$a, d0
pub fn i_c11dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DD4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C11DD4: bcs.b $c11dd8
pub fn i_c11dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DD6;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC11DD8; return 10; }
    8
}
/// $C11DD6: addq.b #$6, d0
pub fn i_c11dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DD8;
    cpu.ir = 0x5C00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11DD8: move.b d0, $1(a0)
pub fn i_c11dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DDC;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11DDC: jsr $c11dea(pc)
pub fn i_c11ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DE0);
    cpu.pc = 0xC11DEA;
    16
}
/// $C11DE0: move.b d0, (a0)
pub fn i_c11de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DE2;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $C11DE2: moveq #$3, d7
pub fn i_c11de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DE4;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11DE4: jsr $c11dec(pc)
pub fn i_c11de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DE8);
    cpu.pc = 0xC11DEC;
    16
}
/// $C11DE8: rts
pub fn i_c11de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11DEA: moveq #$7, d7
pub fn i_c11dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DEC;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11DEC: move.b $220001.l, d2
pub fn i_c11dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DF2;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x220001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C11DF2: jsr $c11eb0(pc)
pub fn i_c11df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11DF6);
    cpu.pc = 0xC11EB0;
    16
}
/// $C11DF6: andi.b #$80, d2
pub fn i_c11df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DFA;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11DFA: lsr.b #$1, d0
pub fn i_c11dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DFC;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11DFC: or.b d2, d0
pub fn i_c11dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11DFE;
    cpu.ir = 0x8002;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11DFE: dbra d7, $c11dec
pub fn i_c11dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E02;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11E02; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11E02; return 14; }
    cpu.pc = 0xC11DEC;
    10
}
/// $C11E02: rts
pub fn i_c11e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11E04: lea.l $10f300.l, a5
pub fn i_c11e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E0A;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C11E0A: jsr $c11eca(pc)
pub fn i_c11e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E0E);
    cpu.pc = 0xC11ECA;
    16
}
/// $C11E0E: tst.b d7
pub fn i_c11e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E10;
    cpu.ir = 0x4A07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11E10: bne.w $c11e58
pub fn i_c11e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11E58; return 10; }
    8
}
/// $C11E14: moveq #$1, d0
pub fn i_c11e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E16;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11E16: jsr $c11e7e(pc)
pub fn i_c11e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E1A);
    cpu.pc = 0xC11E7E;
    16
}
/// $C11E1A: move.b $ad8(a5), d0
pub fn i_c11e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E1E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11E1E: jsr $c11e5a(pc)
pub fn i_c11e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E22);
    cpu.pc = 0xC11E5A;
    16
}
/// $C11E22: move.b $ad7(a5), d0
pub fn i_c11e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E26;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11E26: jsr $c11e5a(pc)
pub fn i_c11e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E2A);
    cpu.pc = 0xC11E5A;
    16
}
/// $C11E2A: move.b $ad6(a5), d0
pub fn i_c11e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E2E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11E2E: jsr $c11e5a(pc)
pub fn i_c11e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E32);
    cpu.pc = 0xC11E5A;
    16
}
/// $C11E32: move.b $ad4(a5), d0
pub fn i_c11e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E36;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11E36: jsr $c11e5a(pc)
pub fn i_c11e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E3A);
    cpu.pc = 0xC11E5A;
    16
}
/// $C11E3A: move.b $a01(a5), d0
pub fn i_c11e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E3E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA01);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11E3E: jsr $c11e5a(pc)
pub fn i_c11e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E42);
    cpu.pc = 0xC11E5A;
    16
}
/// $C11E42: move.b $ad2(a5), d0
pub fn i_c11e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E46;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11E46: jsr $c11e5a(pc)
pub fn i_c11e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E4A);
    cpu.pc = 0xC11E5A;
    16
}
/// $C11E4A: moveq #$2, d0
pub fn i_c11e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E4C;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11E4C: jsr $c11e7e(pc)
pub fn i_c11e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E50);
    cpu.pc = 0xC11E7E;
    16
}
/// $C11E50: moveq #$0, d0
pub fn i_c11e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E52;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11E52: jsr $c11e7e(pc)
pub fn i_c11e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E56);
    cpu.pc = 0xC11E7E;
    16
}
/// $C11E56: moveq #$0, d7
pub fn i_c11e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E58;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11E58: rts
pub fn i_c11e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11E5A: moveq #$7, d7
pub fn i_c11e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E5C;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11E5C: move.b d0, d1
pub fn i_c11e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E5E;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11E5E: andi.b #$1, d1
pub fn i_c11e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E62;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E62: jsr $c11ec2(pc)
pub fn i_c11e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E66);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E66: ori.b #$2, d1
pub fn i_c11e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E6A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E6A: jsr $c11ec2(pc)
pub fn i_c11e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E6E);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E6E: eori.b #$2, d1
pub fn i_c11e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E72;
    cpu.ir = 0xA01;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E72: jsr $c11ec2(pc)
pub fn i_c11e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E76);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E76: lsr.b #$1, d0
pub fn i_c11e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E78;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11E78: dbra d7, $c11e5c
pub fn i_c11e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E7C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11E7C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11E7C; return 14; }
    cpu.pc = 0xC11E5C;
    10
}
/// $C11E7C: rts
pub fn i_c11e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11E7E: moveq #$3, d7
pub fn i_c11e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E80;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11E80: move.b d0, d1
pub fn i_c11e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E82;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11E82: andi.b #$1, d1
pub fn i_c11e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E86;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E86: jsr $c11ec2(pc)
pub fn i_c11e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E8A);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E8A: ori.b #$2, d1
pub fn i_c11e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E8E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E8E: jsr $c11ec2(pc)
pub fn i_c11e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E92);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E92: eori.b #$2, d1
pub fn i_c11e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E96;
    cpu.ir = 0xA01;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11E96: jsr $c11ec2(pc)
pub fn i_c11e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11E9A);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11E9A: lsr.b #$1, d0
pub fn i_c11e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11E9C;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11E9C: dbra d7, $c11e80
pub fn i_c11e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EA0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11EA0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11EA0; return 14; }
    cpu.pc = 0xC11E80;
    10
}
/// $C11EA0: move.b #$4, d1
pub fn i_c11ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EA4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11EA4: jsr $c11ec2(pc)
pub fn i_c11ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EA8);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11EA8: moveq #$0, d1
pub fn i_c11ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EAA;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11EAA: jsr $c11ec2(pc)
pub fn i_c11eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EAE);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11EAE: rts
pub fn i_c11eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11EB0: move.b #$2, d1
pub fn i_c11eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EB4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11EB4: jsr $c11ec2(pc)
pub fn i_c11eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EB8);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11EB8: move.b #$0, d1
pub fn i_c11eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EBC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11EBC: jsr $c11ec2(pc)
pub fn i_c11ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EC0);
    cpu.pc = 0xC11EC2;
    16
}
/// $C11EC0: rts
pub fn i_c11ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11EC2: move.b d1, $280051.l
pub fn i_c11ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EC8;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x280051, v1 as u8);
    16
}
/// $C11EC8: rts
pub fn i_c11ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ECA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11ECA: lea.l $c11fbe(pc), a1
pub fn i_c11eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ECE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11FBE;
    16
}
/// $C11ECE: move.b $ad2(a5), d0
pub fn i_c11ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ED2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11ED2: jsr $c1217c(pc)
pub fn i_c11ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11ED6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11ED6);
    cpu.pc = 0xC1217C;
    16
}
/// $C11ED6: move.b d0, $a00(a5)
pub fn i_c11ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EDA;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11EDA: move.w d0, d1
pub fn i_c11eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EDC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11EDC: move.b $ad3(a5), d0
pub fn i_c11edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EE0;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11EE0: jsr $c1217c(pc)
pub fn i_c11ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11EE4);
    cpu.pc = 0xC1217C;
    16
}
/// $C11EE4: move.w d0, d2
pub fn i_c11ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EE6;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11EE6: move.w d2, d3
pub fn i_c11ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EE8;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11EE8: lsl.b #$4, d0
pub fn i_c11ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EEA;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C11EEA: move.b d0, $a01(a5)
pub fn i_c11eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EEE;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA01);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11EEE: cmpi.b #$2, d3
pub fn i_c11eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EF2;
    cpu.ir = 0xC03;
    let v1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C11EF2: bne.b $c11efe
pub fn i_c11ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EF4;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11EFE; return 10; }
    8
}
/// $C11EF4: move.b d1, d0
pub fn i_c11ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EF6;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11EF6: andi.b #$3, d0
pub fn i_c11ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EFA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11EFA: bne.b $c11efe
pub fn i_c11efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EFC;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11EFE; return 10; }
    8
}
/// $C11EFC: moveq #$0, d3
pub fn i_c11efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11EFE;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11EFE: move.b $ad4(a5), d0
pub fn i_c11efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F02;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11F02: jsr $c1217c(pc)
pub fn i_c11f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11F06);
    cpu.pc = 0xC1217C;
    16
}
/// $C11F06: move.b d0, $a02(a5)
pub fn i_c11f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F0A;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA02);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11F0A: cmp.b (a1, d3.w), d0
pub fn i_c11f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F0E;
    cpu.ir = 0xB031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $C11F0E: bhi.w $c11f56
pub fn i_c11f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F12;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC11F56; return 10; }
    8
}
/// $C11F12: moveq #$0, d6
pub fn i_c11f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F14;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F14: move.l d6, d7
pub fn i_c11f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F16;
    cpu.ir = 0x2E06;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F16: moveq #$1, d5
pub fn i_c11f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F18;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F18: andi.b #$3, d1
pub fn i_c11f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F1C;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F1C: bne.b $c11f20
pub fn i_c11f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F1E;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11F20; return 10; }
    8
}
/// $C11F1E: moveq #$0, d3
pub fn i_c11f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F20;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F20: cmp.b d5, d2
pub fn i_c11f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F22;
    cpu.ir = 0xB405;
    let v1 = cpu.d[5] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C11F22: beq.b $c11f3a
pub fn i_c11f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F24;
    cpu.ir = 0x6716;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11F3A; return 10; }
    8
}
/// $C11F24: move.b (a1, d5.w), d6
pub fn i_c11f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F28;
    cpu.ir = 0x1C31;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C11F28: cmpi.b #$2, d5
pub fn i_c11f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F2C;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C11F2C: bne.b $c11f34
pub fn i_c11f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F2E;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11F34; return 10; }
    8
}
/// $C11F2E: tst.b d3
pub fn i_c11f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F30;
    cpu.ir = 0x4A03;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11F30: bne.b $c11f34
pub fn i_c11f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F32;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11F34; return 10; }
    8
}
/// $C11F32: moveq #$1d, d6
pub fn i_c11f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F34;
    cpu.ir = 0x7C1D;
    cpu.d[6] = 0x1D;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F34: add.w d6, d7
pub fn i_c11f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F36;
    cpu.ir = 0xDE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11F36: addq.b #$1, d5
pub fn i_c11f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F38;
    cpu.ir = 0x5205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11F38: bra.b $c11f20
pub fn i_c11f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F3A;
    cpu.ir = 0x60E6;
    cpu.pc = 0xC11F20;
    10
}
/// $C11F3A: add.w d0, d7
pub fn i_c11f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F3C;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11F3C: subq.w #$1, d7
pub fn i_c11f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F3E;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11F3E: move.b $a00(a5), d0
pub fn i_c11f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F42;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11F42: move.b $c11f5a(pc, d0.w), d0
pub fn i_c11f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F46;
    cpu.ir = 0x103B;
    let ad2 = 0xC11F5A_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C11F46: add.w d0, d7
pub fn i_c11f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F48;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11F48: divu.w #$7, d7
pub fn i_c11f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F4C;
    cpu.ir = 0x8EFC;
    let v1: u32 = 0x7;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[7] / dv; let rm = cpu.d[7] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[7] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C11F4C: swap d7
pub fn i_c11f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F4E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F4E: add.b d7, $a01(a5)
pub fn i_c11f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F52;
    cpu.ir = 0xDF2D;
    let ad1 = cpu.a[5].wrapping_add(0xA01);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[7] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11F52: moveq #$0, d7
pub fn i_c11f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F54;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F54: rts
pub fn i_c11f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11F56: moveq #$1, d7
pub fn i_c11f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F58;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11F58: rts
pub fn i_c11f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11F5A: addi.b #$3, d1
pub fn i_c11f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F5E;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F5E: subi.b #$1, d6
pub fn i_c11f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F62;
    cpu.ir = 0x406;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F62: andi.b #$6, d4
pub fn i_c11f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F66;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F66: ori.b #$4, d2
pub fn i_c11f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F6A;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F6A: btst.l d2, d0
pub fn i_c11f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F6C;
    cpu.ir = 0x500;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11F6C: btst.l d0, d2
pub fn i_c11f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F6E;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11F6E: btst.l d1, d5
pub fn i_c11f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F70;
    cpu.ir = 0x305;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11F70: addi.b #$3, d0
pub fn i_c11f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F74;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F74: subi.b #$1, d5
pub fn i_c11f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F78;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F78: andi.b #$6, d3
pub fn i_c11f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F7C;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F7C: ori.b #$4, d1
pub fn i_c11f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F80;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F80: btst.l d2, d6
pub fn i_c11f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F82;
    cpu.ir = 0x506;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11F82: ori.b #$4, d2
pub fn i_c11f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F86;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F86: btst.l d2, d0
pub fn i_c11f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F88;
    cpu.ir = 0x500;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11F88: btst.l d0, d2
pub fn i_c11f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F8A;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11F8A: btst.l d1, d5
pub fn i_c11f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F8C;
    cpu.ir = 0x305;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11F8C: addi.b #$3, d0
pub fn i_c11f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F90;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F90: subi.b #$1, d5
pub fn i_c11f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F94;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F94: andi.b #$6, d3
pub fn i_c11f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F98;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F98: ori.b #$4, d1
pub fn i_c11f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F9C;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11F9C: btst.l d2, d6
pub fn i_c11f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11F9E;
    cpu.ir = 0x506;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11F9E: ori.b #$4, d2
pub fn i_c11f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FA2;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11FA2: btst.l d2, d0
pub fn i_c11fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FA4;
    cpu.ir = 0x500;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11FA4: btst.l d0, d2
pub fn i_c11fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FA6;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11FA6: btst.l d1, d5
pub fn i_c11fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FA8;
    cpu.ir = 0x305;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11FA8: addi.b #$3, d0
pub fn i_c11fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FAC;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11FAC: subi.b #$1, d5
pub fn i_c11fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FB0;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11FB0: andi.b #$6, d3
pub fn i_c11fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FB4;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11FB4: btst.l d0, d2
pub fn i_c11fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FB6;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11FB6: btst.l d1, d5
pub fn i_c11fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FB8;
    cpu.ir = 0x305;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11FB8: addi.b #$3, d0
pub fn i_c11fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FBC;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11FBC: subi.b #$1f, d5
pub fn i_c11fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FC0;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11FC0: move.b (a7)+, d6
pub fn i_c11fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FC2;
    cpu.ir = 0x1C1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C11FC2: move.b (a7)+, d7
pub fn i_c11fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FC4;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C11FC4: move.b (a7)+, d7
pub fn i_c11fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FC6;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C11FC6: move.b (a6)+, -(a7)
pub fn i_c11fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FC8;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C11FC8: move.b (a6)+, -(a7)
pub fn i_c11fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FCA;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C11FCA: move.b d0, -(a7)
pub fn i_c11fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FCC;
    cpu.ir = 0x1F00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write8(ad2, v1 as u8);
    10
}
/// $C11FCC: lea.l $100.w, a0
pub fn i_c11fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FD0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0x100;
    16
}
/// $C11FD0: lea.l $c11ffe(pc), a1
pub fn i_c11fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC11FFE;
    16
}
/// $C11FD4: moveq #$6, d7
pub fn i_c11fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FD6;
    cpu.ir = 0x7E06;
    cpu.d[7] = 0x6;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11FD6: cmpm.b (a0)+, (a1)+
pub fn i_c11fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FD8;
    cpu.ir = 0xB308;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read8(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C11FD8: bne.b $c12006
pub fn i_c11fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FDA;
    cpu.ir = 0x662C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12006; return 10; }
    8
}
/// $C11FDA: dbra d7, $c11fd6
pub fn i_c11fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FDE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11FDE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11FDE; return 14; }
    cpu.pc = 0xC11FD6;
    10
}
/// $C11FDE: movea.l $182.w, a0
pub fn i_c11fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FE2;
    cpu.ir = 0x2078;
    let v2 = bus.read32(0x182);
    cpu.a[0] = v2;
    16
}
/// $C11FE2: move.w a0, d0
pub fn i_c11fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FE4;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11FE4: btst.b #$0, d0
pub fn i_c11fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FE8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C11FE8: bne.b $c11ffc
pub fn i_c11fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FEA;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11FFC; return 10; }
    8
}
/// $C11FEA: lea.l $c13674(pc), a1
pub fn i_c11fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC13674;
    16
}
/// $C11FEE: lea.l $c136ee(pc), a2
pub fn i_c11fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FF2;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC136EE;
    16
}
/// $C11FF2: cmpm.w (a0)+, (a1)+
pub fn i_c11ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FF4;
    cpu.ir = 0xB348;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(2);
    let v1 = bus.read16(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read16(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(2);
    sub_with_flags(cpu, v2, v1, Size::Word, false);
    12
}
/// $C11FF4: bne.b $c11ffc
pub fn i_c11ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FF6;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11FFC; return 10; }
    8
}
/// $C11FF6: cmpa.l a1, a2
pub fn i_c11ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FF8;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $C11FF8: bne.b $c11ff2
pub fn i_c11ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FFA;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11FF2; return 10; }
    8
}
/// $C11FFA: moveq #$0, d7
pub fn i_c11ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FFC;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11FFC: rts
pub fn i_c11ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11FFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11FFE: trap #$5
pub fn i_c11ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12000;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C12000: dc.w $4f2d
pub fn i_c12000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12002;
    cpu.ir = 0x4F2D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12002: ?
pub fn i_c12002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12004;
    cpu.ir = 0x4745;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12004: dc.w $4f00
pub fn i_c12004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12006;
    cpu.ir = 0x4F00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12006: lea.l $100.w, a0
pub fn i_c12006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1200A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0x100;
    16
}
/// $C1200A: lea.l $c1206c(pc), a1
pub fn i_c1200a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1200E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1206C;
    16
}
/// $C1200E: moveq #$6, d7
pub fn i_c1200e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12010;
    cpu.ir = 0x7E06;
    cpu.d[7] = 0x6;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12010: cmpm.b (a0)+, (a1)+
pub fn i_c12010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12012;
    cpu.ir = 0xB308;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read8(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C12012: bne.b $c12036
pub fn i_c12012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12014;
    cpu.ir = 0x6622;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12036; return 10; }
    8
}
/// $C12014: dbra d7, $c12010
pub fn i_c12014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12018;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12018; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12018; return 14; }
    cpu.pc = 0xC12010;
    10
}
/// $C12018: moveq #$0, d0
pub fn i_c12018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1201A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1201A: move.b $d00108.l, $d0005e.l
pub fn i_c1201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12024;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0xD00108) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0xD0005E, v2 as u8);
    28
}
/// $C12024: clr.l $d00010.l
pub fn i_c12024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1202A;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0xD00010);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0xD00010, r2);
    20
}
/// $C1202A: jsr $c11d86(pc)
pub fn i_c1202a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1202E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1202E);
    cpu.pc = 0xC11D86;
    16
}
/// $C1202E: jsr $c11cc0(pc)
pub fn i_c1202e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12032;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12032);
    cpu.pc = 0xC11CC0;
    16
}
/// $C12032: jmp $c16034(pc)
pub fn i_c12032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12036;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC16034;
    8
}
/// $C12036: lea.l $100.w, a0
pub fn i_c12036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1203A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0x100;
    16
}
/// $C1203A: lea.l $c12074(pc), a1
pub fn i_c1203a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1203E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC12074;
    16
}
/// $C1203E: moveq #$6, d7
pub fn i_c1203e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12040;
    cpu.ir = 0x7E06;
    cpu.d[7] = 0x6;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12040: cmpm.b (a0)+, (a1)+
pub fn i_c12040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12042;
    cpu.ir = 0xB308;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read8(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C12042: bne.b $c11ffc
pub fn i_c12042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12044;
    cpu.ir = 0x66B8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11FFC; return 10; }
    8
}
/// $C12044: dbra d7, $c12040
pub fn i_c12044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12048;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12048; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12048; return 14; }
    cpu.pc = 0xC12040;
    10
}
/// $C12048: move.w #$1, $d00060.l
pub fn i_c12048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12050;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0xD00060, v1 as u16);
    16
}
/// $C12050: clr.l $d00010.l
pub fn i_c12050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12056;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0xD00010);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0xD00010, r2);
    20
}
/// $C12056: lea.l $d00124.l, a0
pub fn i_c12056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1205C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xD00124;
    20
}
/// $C1205C: moveq #$7, d7
pub fn i_c1205c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1205E;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1205E: move.l #$ffff, (a0)+
pub fn i_c1205e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12064;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C12064: dbra d7, $c1205e
pub fn i_c12064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12068;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12068; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12068; return 14; }
    cpu.pc = 0xC1205E;
    10
}
/// $C12068: moveq #$0, d7
pub fn i_c12068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1206A;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1206A: rts
pub fn i_c1206a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1206C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1206C: trap #$5
pub fn i_c1206c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1206E;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C1206E: dc.w $4f2d
pub fn i_c1206e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12070;
    cpu.ir = 0x4F2D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12070: ?
pub fn i_c12070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12072;
    cpu.ir = 0x434C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12072: ?
pub fn i_c12072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12074;
    cpu.ir = 0x4E00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12074: ?
pub fn i_c12074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12076;
    cpu.ir = 0x4E00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12076: dc.w $4f00
pub fn i_c12076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12078;
    cpu.ir = 0x4F00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12078: dc.w $4700
pub fn i_c12078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1207A;
    cpu.ir = 0x4700;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1207A: dc.w $4f00
pub fn i_c1207a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1207C;
    cpu.ir = 0x4F00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1207C: move.w d4, -$2(a1)
pub fn i_c1207c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12080;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12080: move.w #$5555, d0
pub fn i_c12080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12084;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12084: move.w d5, d7
pub fn i_c12084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12086;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12086: move.w d0, (a1)
pub fn i_c12086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12088;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C12088: move.b d0, (a2)
pub fn i_c12088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1208A;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1208A: dbra d7, $c12086
pub fn i_c1208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1208E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1208E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1208E; return 14; }
    cpu.pc = 0xC12086;
    10
}
/// $C1208E: move.w d4, -$2(a1)
pub fn i_c1208e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12092;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12092: movea.l d4, a0
pub fn i_c12092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12094;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12094: move.w d5, d7
pub fn i_c12094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12096;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12096: move.b d0, (a2)
pub fn i_c12096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12098;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C12098: move.w (a1), d1
pub fn i_c12098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1209A;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1209A: move.w d1, (a1)
pub fn i_c1209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1209C;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C1209C: cmp.w d0, d1
pub fn i_c1209c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1209E;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C1209E: bne.w $c15aac
pub fn i_c1209e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C120A2: addq.l #$1, a0
pub fn i_c120a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120A4;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C120A4: dbra d7, $c12096
pub fn i_c120a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120A8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC120A8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC120A8; return 14; }
    cpu.pc = 0xC12096;
    10
}
/// $C120A8: move.w d4, -$2(a1)
pub fn i_c120a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120AC;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C120AC: move.w #$aaaa, d0
pub fn i_c120ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120B0: move.w d5, d7
pub fn i_c120b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120B2;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120B2: move.w d0, (a1)
pub fn i_c120b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120B4;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C120B4: move.b d0, (a2)
pub fn i_c120b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120B6;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C120B6: dbra d7, $c120b2
pub fn i_c120b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120BA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC120BA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC120BA; return 14; }
    cpu.pc = 0xC120B2;
    10
}
/// $C120BA: move.w d4, -$2(a1)
pub fn i_c120ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120BE;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C120BE: movea.l d4, a0
pub fn i_c120be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C0;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C120C0: move.w d5, d7
pub fn i_c120c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C2;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120C2: move.b d0, (a2)
pub fn i_c120c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C4;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C120C4: move.w (a1), d1
pub fn i_c120c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C6;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C120C6: move.w d1, (a1)
pub fn i_c120c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120C8;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C120C8: cmp.w d0, d1
pub fn i_c120c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120CA;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C120CA: bne.w $c15aac
pub fn i_c120ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C120CE: addq.l #$1, a0
pub fn i_c120ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120D0;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C120D0: dbra d7, $c120c2
pub fn i_c120d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120D4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC120D4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC120D4; return 14; }
    cpu.pc = 0xC120C2;
    10
}
/// $C120D4: move.w d4, -$2(a1)
pub fn i_c120d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120D8;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C120D8: moveq #$0, d0
pub fn i_c120d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120DA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C120DA: move.w d5, d7
pub fn i_c120da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120DC;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120DC: move.b d0, (a2)
pub fn i_c120dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120DE;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C120DE: move.w d0, (a1)
pub fn i_c120de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120E0;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C120E0: addq.w #$1, d0
pub fn i_c120e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120E2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C120E2: dbra d7, $c120dc
pub fn i_c120e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120E6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC120E6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC120E6; return 14; }
    cpu.pc = 0xC120DC;
    10
}
/// $C120E6: move.w d4, -$2(a1)
pub fn i_c120e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120EA;
    cpu.ir = 0x3344;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C120EA: movea.l d4, a0
pub fn i_c120ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120EC;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C120EC: moveq #$0, d0
pub fn i_c120ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120EE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C120EE: move.w d5, d7
pub fn i_c120ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F0;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C120F0: move.b d0, (a2)
pub fn i_c120f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F2;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C120F2: move.w (a1), d1
pub fn i_c120f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F4;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C120F4: move.w d1, (a1)
pub fn i_c120f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F6;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C120F6: cmp.w d0, d1
pub fn i_c120f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120F8;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C120F8: bne.w $c15aac
pub fn i_c120f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C120FC: addq.l #$1, a0
pub fn i_c120fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC120FE;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C120FE: addq.w #$1, d0
pub fn i_c120fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12100;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12100: dbra d7, $c120f0
pub fn i_c12100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12104;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12104; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12104; return 14; }
    cpu.pc = 0xC120F0;
    10
}
/// $C12104: rts
pub fn i_c12104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12106;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12106: lea.l $300001.l, a2
pub fn i_c12106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1210C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x300001;
    20
}
/// $C1210C: movea.l (a7)+, a6
pub fn i_c1210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1210E;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $C1210E: movea.l a0, a1
pub fn i_c1210e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12110;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C12110: move.w #$5555, d0
pub fn i_c12110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12114;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5555;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12114: move.w d5, d7
pub fn i_c12114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12116;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12116: move.b d0, (a2)
pub fn i_c12116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12118;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C12118: move.w d0, (a0)+
pub fn i_c12118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1211A;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1211A: dbra d7, $c12116
pub fn i_c1211a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1211E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1211E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1211E; return 14; }
    cpu.pc = 0xC12116;
    10
}
/// $C1211E: movea.l a1, a0
pub fn i_c1211e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12120;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12120: move.w d5, d7
pub fn i_c12120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12122;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12122: move.b d0, (a2)
pub fn i_c12122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12124;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C12124: move.w (a0), d1
pub fn i_c12124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12126;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12126: cmp.w d0, d1
pub fn i_c12126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12128;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C12128: bne.w $c15aac
pub fn i_c12128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1212C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C1212C: addq.l #$2, a0
pub fn i_c1212c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1212E;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C1212E: dbra d7, $c12122
pub fn i_c1212e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12132;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12132; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12132; return 14; }
    cpu.pc = 0xC12122;
    10
}
/// $C12132: movea.l a1, a0
pub fn i_c12132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12134;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12134: move.w #$aaaa, d0
pub fn i_c12134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12138;
    cpu.ir = 0x303C;
    let v1: u32 = 0xAAAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12138: move.w d5, d7
pub fn i_c12138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1213A;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1213A: move.b d0, (a2)
pub fn i_c1213a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1213C;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1213C: move.w d0, (a0)+
pub fn i_c1213c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1213E;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1213E: dbra d7, $c1213a
pub fn i_c1213e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12142;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12142; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12142; return 14; }
    cpu.pc = 0xC1213A;
    10
}
/// $C12142: movea.l a1, a0
pub fn i_c12142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12144;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12144: move.w d5, d7
pub fn i_c12144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12146;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12146: move.b d0, (a2)
pub fn i_c12146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12148;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C12148: move.w (a0), d1
pub fn i_c12148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1214A;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1214A: cmp.w d0, d1
pub fn i_c1214a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1214C;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C1214C: bne.w $c15aac
pub fn i_c1214c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12150;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C12150: addq.l #$2, a0
pub fn i_c12150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12152;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C12152: dbra d7, $c12146
pub fn i_c12152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12156;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12156; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12156; return 14; }
    cpu.pc = 0xC12146;
    10
}
/// $C12156: movea.l a1, a0
pub fn i_c12156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12158;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12158: move.w d5, d7
pub fn i_c12158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1215A;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1215A: move.b d0, (a2)
pub fn i_c1215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1215C;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1215C: move.w a0, d0
pub fn i_c1215c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1215E;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1215E: move.w d0, (a0)+
pub fn i_c1215e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12160;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12160: dbra d7, $c1215a
pub fn i_c12160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12164;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12164; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12164; return 14; }
    cpu.pc = 0xC1215A;
    10
}
/// $C12164: movea.l a1, a0
pub fn i_c12164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12166;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12166: move.w d5, d7
pub fn i_c12166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12168;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12168: move.b d0, (a2)
pub fn i_c12168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1216A;
    cpu.ir = 0x1480;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1216A: move.w a0, d0
pub fn i_c1216a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1216C;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1216C: move.w (a0), d1
pub fn i_c1216c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1216E;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1216E: cmp.w d0, d1
pub fn i_c1216e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12170;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C12170: bne.w $c15aac
pub fn i_c12170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12174;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15AAC; return 10; }
    8
}
/// $C12174: addq.l #$2, a0
pub fn i_c12174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12176;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C12176: dbra d7, $c12168
pub fn i_c12176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1217A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1217A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1217A; return 14; }
    cpu.pc = 0xC12168;
    10
}
/// $C1217A: jmp (a6)
pub fn i_c1217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1217C;
    cpu.ir = 0x4ED6;
    cpu.jump_to(cpu.a[6]);
    8
}
/// $C1217C: andi.w #$ff, d0
pub fn i_c1217c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12180;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C12180: move.b $c12186(pc, d0.w), d0
pub fn i_c12180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12184;
    cpu.ir = 0x103B;
    let ad2 = 0xC12186_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C12184: rts
pub fn i_c12184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12186;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12186: ori.b #$3, d1
pub fn i_c12186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1218A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1218A: subi.b #$7, d5
pub fn i_c1218a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1218E;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1218E: ?
pub fn i_c1218e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12192;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C12192: ori.b #$0, d0
pub fn i_c12192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12196;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12196: ?
pub fn i_c12196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1219A;
    cpu.ir = 0xA0B;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 ^ 0xD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C1219A: ?
pub fn i_c1219a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1219E;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1219E: move.b (a3), d1
pub fn i_c1219e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121A0;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C121A0: ori.b #$0, d0
pub fn i_c121a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121A4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121A4: ori.b #$15, d0
pub fn i_c121a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121A8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x15) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121A8: move.b (a7), d3
pub fn i_c121a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121AA;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C121AA: move.b (a1)+, d4
pub fn i_c121aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121AC;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C121AC: move.b (a3)+, d5
pub fn i_c121ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121AE;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C121AE: move.b (a5)+, d6
pub fn i_c121ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121B0;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C121B0: ori.b #$0, d0
pub fn i_c121b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121B4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121B4: ori.b #$1f, d0
pub fn i_c121b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121B8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121B8: move.l -(a1), d0
pub fn i_c121b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121BA;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C121BA: move.l -(a3), d1
pub fn i_c121ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121BC;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C121BC: move.l -(a5), d2
pub fn i_c121bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121BE;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C121BE: move.l -(a7), d3
pub fn i_c121be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121C0;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C121C0: ori.b #$0, d0
pub fn i_c121c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121C4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121C4: ori.b #$29, d0
pub fn i_c121c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121C8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x29) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121C8: move.l $2c2d(a3), d5
pub fn i_c121c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121CC;
    cpu.ir = 0x2A2B;
    let ad2 = cpu.a[3].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C121CC: move.l $3031(a7), d7
pub fn i_c121cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121D0;
    cpu.ir = 0x2E2F;
    let ad2 = cpu.a[7].wrapping_add(0x3031);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C121D0: ori.b #$0, d0
pub fn i_c121d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121D4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121D4: ori.b #$33, d0
pub fn i_c121d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121D8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x33) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121D8: move.w $37(a5, d3.w), d2
pub fn i_c121d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121DC;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C121DC: move.w $3a3b0000.l, d4
pub fn i_c121dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121E2;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B0000) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C121E2: ori.b #$0, d0
pub fn i_c121e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121E6;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121E6: ?
pub fn i_c121e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121E8;
    cpu.ir = 0x3C3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C121E8: ?
pub fn i_c121e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121EA;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C121EA: negx.w d1
pub fn i_c121ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121EC;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C121EC: clr.w d3
pub fn i_c121ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121EE;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C121EE: neg.w d5
pub fn i_c121ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121F0;
    cpu.ir = 0x4445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C121F0: ori.b #$0, d0
pub fn i_c121f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121F4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C121F4: ori.b #$47, d0
pub fn i_c121f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC121F8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x47) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1231E: lea.l $10f300.l, a5
pub fn i_c1231e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12324;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C12324: pea.l $c1238c(pc)
pub fn i_c12324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12328;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC1238C);
    24
}
/// $C12328: move.l a7, $a00(a5)
pub fn i_c12328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1232C;
    cpu.ir = 0x2B4F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1232C: move.b #$ff, $9f4(a5)
pub fn i_c1232c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12332;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9F4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12332: move.b $380000.l, d0
pub fn i_c12332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12338;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C12338: andi.b #$30, d0
pub fn i_c12338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1233C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1233C: beq.b $c12346
pub fn i_c1233c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1233E;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12346; return 10; }
    8
}
/// $C1233E: move.b #$80, $ac6(a5)
pub fn i_c1233e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12344;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12344: rts
pub fn i_c12344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12346: move.b #$0, $ac6(a5)
pub fn i_c12346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1234C;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1234C: lea.l $800000.l, a6
pub fn i_c1234c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12352;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C12352: moveq #$0, d0
pub fn i_c12352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12354;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12354: move.b d0, $3a0019.l
pub fn i_c12354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1235A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C1235A: move.b d0, $380011.l
pub fn i_c1235a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12360;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $C12360: move.b $ac4(a5), d0
pub fn i_c12360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12364;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C12364: lsl.w #$2, d0
pub fn i_c12364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12366;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C12366: movea.l $c1236c(pc, d0.w), a0
pub fn i_c12366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1236A;
    cpu.ir = 0x207B;
    let ad2 = 0xC1236C_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C1236A: jmp (a0)
pub fn i_c1236a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1236C;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C1236C: ?
pub fn i_c1236c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1236E;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1236E: movea.l d6, a5
pub fn i_c1236e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12370;
    cpu.ir = 0x2A46;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.a[5] = v1;
    4
}
/// $C12370: ?
pub fn i_c12370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12372;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12372: move.l d4, -(a6)
pub fn i_c12372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12374;
    cpu.ir = 0x2D04;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let o2 = cpu.a[6];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[6] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C12374: ?
pub fn i_c12374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12376;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1238C: jsr $c13e66(pc)
pub fn i_c1238c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12390;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12390);
    cpu.pc = 0xC13E66;
    16
}
/// $C12390: rts
pub fn i_c12390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12392: lea.l $10f300.l, a5
pub fn i_c12392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12398;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C12398: move.l a7, $a00(a5)
pub fn i_c12398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1239C;
    cpu.ir = 0x2B4F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1239C: moveq #$0, d0
pub fn i_c1239c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1239E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1239E: move.b $ac6(a5), d0
pub fn i_c1239e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123A2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C123A2: move.b d0, $ac7(a5)
pub fn i_c123a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123A6;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC7);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C123A6: subi.b #$81, d0
pub fn i_c123a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123AA;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x81, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C123AA: bcs.b $c123b2
pub fn i_c123aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123AC;
    cpu.ir = 0x6506;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC123B2; return 10; }
    8
}
/// $C123AC: cmpi.b #$5, d0
pub fn i_c123ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123B0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $C123B0: bcs.b $c123dc
pub fn i_c123b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123B2;
    cpu.ir = 0x652A;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC123DC; return 10; }
    8
}
/// $C123B2: rts
pub fn i_c123b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C123B4: dc.w $ffff
pub fn i_c123b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123B6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C123B6: ?
pub fn i_c123b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123BA;
    cpu.ir = 0xC;
    let v1 = cpu.a[4] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[4] = r2;
    8
}
/// $C123BA: cmpi.b #$ff, d0
pub fn i_c123ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123BE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C123BE: dc.w $ffff
pub fn i_c123be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123C0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C123C0: dc.w $ffff
pub fn i_c123c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123C2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C123C2: dc.w $ffff
pub fn i_c123c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123C4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C123C4: dc.w $ffff
pub fn i_c123c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123C6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C123C6: ?
pub fn i_c123c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123CA;
    cpu.ir = 0x40C;
    let v1 = cpu.a[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0xFF, Size::Byte, true);
    cpu.a[4] = r2;
    8
}
/// $C123CA: dc.w $ffff
pub fn i_c123ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123CC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C123CC: dc.w $ffff
pub fn i_c123cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123CE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C123CE: dc.w $ff10
pub fn i_c123ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123D0;
    cpu.ir = 0xFF10;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C123DC: lsl.w #$3, d0
pub fn i_c123dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123DE;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C123DE: move.b $ac4(a5), d1
pub fn i_c123de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123E2;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C123E2: andi.b #$7, d1
pub fn i_c123e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123E6;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C123E6: or.b d1, d0
pub fn i_c123e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123E8;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C123E8: move.b $c123b4(pc, d0.w), d0
pub fn i_c123e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123EC;
    cpu.ir = 0x103B;
    let ad2 = 0xC123B4_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C123EC: bmi.b $c123b2
pub fn i_c123ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123EE;
    cpu.ir = 0x6BC4;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC123B2; return 10; }
    8
}
/// $C123EE: move.l #$10ff00, $abe(a5)
pub fn i_c123ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123F6;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C123F6: clr.b $ac2(a5)
pub fn i_c123f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123FA;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C123FA: clr.b $bba(a5)
pub fn i_c123fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC123FE;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBA);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C123FE: pea.l $c1242c(pc)
pub fn i_c123fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12402;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC1242C);
    24
}
/// $C12402: move.w $a80(a5), $b8e(a5)
pub fn i_c12402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12408;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xA80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xB8E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C12408: bclr.b #$7, $a80(a5)
pub fn i_c12408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1240E;
    cpu.ir = 0x8AD;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[5].wrapping_add(0xA80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C1240E: move.w #$2000, sr
pub fn i_c1240e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12412;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C12412: movea.l $c12418(pc, d0.w), a0
pub fn i_c12412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12416;
    cpu.ir = 0x207B;
    let ad2 = 0xC12418_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C12416: jmp (a0)
pub fn i_c12416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12418;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C12418: ?
pub fn i_c12418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1241A;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1241A: move.l #$c12444, d2
pub fn i_c1241a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12420;
    cpu.ir = 0x243C;
    let v1: u32 = 0xC12444;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12420: ?
pub fn i_c12420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12422;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12422: movea.l a4, a2
pub fn i_c12422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12424;
    cpu.ir = 0x244C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C12424: ?
pub fn i_c12424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12426;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12426: move.l (a0), -(a2)
pub fn i_c12426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12428;
    cpu.ir = 0x2510;
    let v2 = bus.read32(cpu.a[0]);
    let o3 = cpu.a[2];
    bus.write16(o3.wrapping_sub(2), v2 as u16);
    cpu.a[2] = o3.wrapping_sub(4);
    bus.write16(o3.wrapping_sub(4), (v2 >> 16) as u16);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    22
}
/// $C12428: ?
pub fn i_c12428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1242A;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1242A: move.l (a0), d3
pub fn i_c1242a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1242C;
    cpu.ir = 0x2610;
    let v2 = bus.read32(cpu.a[0]);
    cpu.d[3] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1242C: jsr $c13e66(pc)
pub fn i_c1242c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12430;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12430);
    cpu.pc = 0xC13E66;
    16
}
/// $C12430: jsr $c129be(pc)
pub fn i_c12430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12434;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12434);
    cpu.pc = 0xC129BE;
    16
}
/// $C12434: move.w $b8e(a5), $a80(a5)
pub fn i_c12434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1243A;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xB8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xA80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1243A: rts
pub fn i_c1243a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1243C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1243C: lea.l $c12454(pc), a4
pub fn i_c1243c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12440;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0xC12454;
    16
}
/// $C12440: bra.w $c124d2
pub fn i_c12440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12444;
    cpu.ir = 0x6000;
    cpu.pc = 0xC124D2;
    10
}
/// $C12444: lea.l $c12474(pc), a4
pub fn i_c12444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12448;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0xC12474;
    16
}
/// $C12448: bra.w $c124d2
pub fn i_c12448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1244C;
    cpu.ir = 0x6000;
    cpu.pc = 0xC124D2;
    10
}
/// $C1244C: lea.l $c12492(pc), a4
pub fn i_c1244c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12450;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0xC12492;
    16
}
/// $C12450: bra.w $c124d2
pub fn i_c12450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12454;
    cpu.ir = 0x6000;
    cpu.pc = 0xC124D2;
    10
}
/// $C12454: ori.b #$4a, d3
pub fn i_c12454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12458;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12458: movep.w $4e4f(a0), d0
pub fn i_c12458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1245C;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4E4F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1245C: movea.l d4, a0
pub fn i_c1245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1245E;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1245E: ?
pub fn i_c1245e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12460;
    cpu.ir = 0x4154;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12460: dc.w $4120
pub fn i_c12460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12462;
    cpu.ir = 0x4120;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12462: ?
pub fn i_c12462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12464;
    cpu.ir = 0x464F;
    let v1 = cpu.a[7] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.a[7] = r2 as i16 as i32 as u32;
    4
}
/// $C12464: addq.b #$1, -(a0)
pub fn i_c12464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12466;
    cpu.ir = 0x5220;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C12466: addq.w #$2, a0
pub fn i_c12466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12468;
    cpu.ir = 0x5448;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C12468: ?
pub fn i_c12468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1246A;
    cpu.ir = 0x4953;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1246A: movea.l d7, a0
pub fn i_c1246a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1246C;
    cpu.ir = 0x2047;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1246C: ?
pub fn i_c1246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1246E;
    cpu.ir = 0x414D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1246E: dc.w $452e
pub fn i_c1246e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12470;
    cpu.ir = 0x452E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12470: dc.w $ffff
pub fn i_c12470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12472;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C12472: ori.b #$3, d0
pub fn i_c12472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12476;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C124B8: ori.b #$4e, d3
pub fn i_c124b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124BC;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4E) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C124BC: movep.w $5055(a0), d0
pub fn i_c124bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124C0;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5055);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C124C0: subq.w #$1, a0
pub fn i_c124c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124C2;
    cpu.ir = 0x5348;
    cpu.a[0] = cpu.a[0].wrapping_sub(1);
    8
}
/// $C124C2: movea.l d1, a0
pub fn i_c124c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124C4;
    cpu.ir = 0x2041;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C124C4: unlk a1
pub fn i_c124c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124C6;
    cpu.ir = 0x4E59;
    cpu.a[7] = cpu.a[1]; let v = cpu.pop32(bus); cpu.a[1] = v;
    12
}
/// $C124C6: movea.l d2, a0
pub fn i_c124c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124C8;
    cpu.ir = 0x2042;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C124C8: subq.w #$2, (a4)
pub fn i_c124c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124CA;
    cpu.ir = 0x5554;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C124CA: addq.w #$2, a7
pub fn i_c124ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124CC;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $C124CC: ?
pub fn i_c124cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124CE;
    cpu.ir = 0x4E2E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C124CE: dc.w $ffff
pub fn i_c124ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124D0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C124D0: ori.b #$ba, d0
pub fn i_c124d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124D4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xBA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C124D2: jsr $c12958(pc)
pub fn i_c124d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC124D6);
    cpu.pc = 0xC12958;
    16
}
/// $C124D4: subi.l #$52390010, d4
pub fn i_c124d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124DA;
    cpu.ir = 0x484;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x52390010, Size::Long, true);
    cpu.d[4] = r2;
    8
}
/// $C124D6: addq.b #$1, $10fdc2.l
pub fn i_c124d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124DC;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10FDC2) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $C124DA: dc.w $fdc2
pub fn i_c124da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124DC;
    cpu.ir = 0xFDC2;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C124DC: movea.l $abe(a5), a0
pub fn i_c124dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124E0;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C124E0: move.l a4, (a0)+
pub fn i_c124e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124E2;
    cpu.ir = 0x20CC;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C124E2: move.l #$c124b8, (a0)+
pub fn i_c124e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124E8;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC124B8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C124E8: move.l a0, $abe(a5)
pub fn i_c124e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124EC;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C124EC: subq.b #$1, $10fdc2.l
pub fn i_c124ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124F2;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10FDC2) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $C124F2: move.w #$384, $b90(a5)
pub fn i_c124f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124F8;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x384;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C124F8: jsr $c12916(pc)
pub fn i_c124f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC124FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC124FC);
    cpu.pc = 0xC12916;
    16
}
/// $C124FC: andi.b #$f0, d0
pub fn i_c124fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12500;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12500: bne.b $c1250e
pub fn i_c12500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12502;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1250E; return 10; }
    8
}
/// $C12502: tst.b $a82(a5)
pub fn i_c12502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12506;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12506: beq.b $c124f8
pub fn i_c12506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12508;
    cpu.ir = 0x67F0;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC124F8; return 10; }
    8
}
/// $C12508: subq.w #$1, $b90(a5)
pub fn i_c12508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1250C;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0xB90);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1250C: bne.b $c124f8
pub fn i_c1250c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1250E;
    cpu.ir = 0x66EA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC124F8; return 10; }
    8
}
/// $C1250E: rts
pub fn i_c1250e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12510;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12510: tst.b $a82(a5)
pub fn i_c12510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12514;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12514: bne.w $c12544
pub fn i_c12514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12518;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12544; return 10; }
    8
}
/// $C12518: jsr $c12958(pc)
pub fn i_c12518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1251C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1251C);
    cpu.pc = 0xC12958;
    16
}
/// $C1251C: addq.b #$1, $ac2(a5)
pub fn i_c1251c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12520;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12520: movea.l $abe(a5), a0
pub fn i_c12520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12524;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12524: move.l #$c1256e, (a0)+
pub fn i_c12524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1252A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1256E;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1252A: move.l a0, $abe(a5)
pub fn i_c1252a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1252E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1252E: subq.b #$1, $ac2(a5)
pub fn i_c1252e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12532;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12532: jsr $c12916(pc)
pub fn i_c12532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12536;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12536);
    cpu.pc = 0xC12916;
    16
}
/// $C12536: btst.b #$5, d0
pub fn i_c12536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1253A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1253A: beq.b $c1253e
pub fn i_c1253a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1253C;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1253E; return 10; }
    8
}
/// $C1253C: rts
pub fn i_c1253c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1253E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1253E: btst.b #$4, d0
pub fn i_c1253e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12542;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C12542: beq.b $c12532
pub fn i_c12542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12544;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12532; return 10; }
    8
}
/// $C12544: move.b $ac4(a5), $ac5(a5)
pub fn i_c12544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1254A;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAC5);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C1254A: move.b #$0, $ac4(a5)
pub fn i_c1254a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12550;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12550: jsr $c1231e(pc)
pub fn i_c12550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12554;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12554);
    cpu.pc = 0xC1231E;
    16
}
/// $C12554: tst.b $ac6(a5)
pub fn i_c12554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12558;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12558: beq.b $c12562
pub fn i_c12558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1255A;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12562; return 10; }
    8
}
/// $C1255A: lea.l $c125da(pc), a4
pub fn i_c1255a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1255E;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0xC125DA;
    16
}
/// $C1255E: bra.w $c124d2
pub fn i_c1255e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12562;
    cpu.ir = 0x6000;
    cpu.pc = 0xC124D2;
    10
}
/// $C12562: move.b $ac5(a5), $ac4(a5)
pub fn i_c12562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12568;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAC5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C12568: jsr $c1231e(pc)
pub fn i_c12568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1256C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1256C);
    cpu.pc = 0xC1231E;
    16
}
/// $C1256C: rts
pub fn i_c1256c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1256E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1256E: ori.b #$8a, d3
pub fn i_c1256e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12572;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12572: movep.w $4d45(a0), d0
pub fn i_c12572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12576;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4D45);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C12576: ?
pub fn i_c12576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12578;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12578: addq.w #$1, (a1)+
pub fn i_c12578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1257A;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1257A: movea.l d3, a0
pub fn i_c1257a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1257C;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1257C: ?
pub fn i_c1257c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1257E;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1257E: neg.b -(a0)
pub fn i_c1257e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12580;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C12580: trap #$f
pub fn i_c12580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12582;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $C12582: addq.b #$2, -(a0)
pub fn i_c12582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12584;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C12584: ?
pub fn i_c12584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12586;
    cpu.ir = 0x464F;
    let v1 = cpu.a[7] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.a[7] = r2 as i16 as i32 as u32;
    4
}
/// $C12586: addq.w #$1, a5
pub fn i_c12586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12588;
    cpu.ir = 0x524D;
    cpu.a[5] = cpu.a[5].wrapping_add(1);
    8
}
/// $C12588: ?
pub fn i_c12588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1258A;
    cpu.ir = 0x4154;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1258A: addq.w #$2, d5
pub fn i_c1258a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1258C;
    cpu.ir = 0x5445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1258C: neg.b -$1(a4)
pub fn i_c1258c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12590;
    cpu.ir = 0x442C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFFFFFF);
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12590: ori.b #$2a, d3
pub fn i_c12590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12594;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x2A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12594: movep.w $444f(a0), d0
pub fn i_c12594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12598;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x444F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C12598: movea.l (a1)+, a0
pub fn i_c12598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1259A;
    cpu.ir = 0x2059;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $C1259A: ?
pub fn i_c1259a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1259C;
    cpu.ir = 0x4F55;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1259C: movea.l (a7), a0
pub fn i_c1259c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1259E;
    cpu.ir = 0x2057;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[0] = v2;
    12
}
/// $C1259E: ?
pub fn i_c1259e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC125A0;
    cpu.ir = 0x4953;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C125A0: nbcd.b -(a0)
pub fn i_c125a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC125A2;
    cpu.ir = 0x4820;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u8; let (r, carry, vf) = sbcd_calc(0, v2 as u8, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); } cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    let r3 = r as u32;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C125A2: addq.w #$2, a7
pub fn i_c125a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC125A4;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $C125A4: movea.l d6, a0
pub fn i_c125a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC125A6;
    cpu.ir = 0x2046;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C125A6: ?
pub fn i_c125a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC125A8;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C125A8: ?
pub fn i_c125a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC125AA;
    cpu.ir = 0x4D41;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C125AA: ?
pub fn i_c125aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC125AC;
    cpu.ir = 0x543F;
    let v1: u32 = 0x0;
    let r2 = add_with_flags(cpu, v1, 2, Size::Byte, true);
    4
}
/// $C125AC: dc.w $ffff
pub fn i_c125ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC125AE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C12610: move.b $ac4(a5), $ac5(a5)
pub fn i_c12610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12616;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAC5);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C12616: move.w $ace(a5), $b98(a5)
pub fn i_c12616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1261C;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xB98);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1261C: move.w $ad0(a5), $b9a(a5)
pub fn i_c1261c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12622;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xB9A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C12622: move.l $ac8(a5), $b92(a5)
pub fn i_c12622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12628;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xB92);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C12628: move.w $acc(a5), $b96(a5)
pub fn i_c12628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1262E;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xB96);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1262E: clr.w $acc(a5)
pub fn i_c1262e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12632;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xACC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C12632: move.l #$10fe9c, $ac8(a5)
pub fn i_c12632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1263A;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FE9C;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1263A: move.b #$5, $ac4(a5)
pub fn i_c1263a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12640;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12640: jsr $c1231e(pc)
pub fn i_c12640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12644;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12644);
    cpu.pc = 0xC1231E;
    16
}
/// $C12644: tst.b $ac6(a5)
pub fn i_c12644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12648;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12648: beq.b $c12632
pub fn i_c12648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1264A;
    cpu.ir = 0x67E8;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12632; return 10; }
    8
}
/// $C1264A: move.w $acc(a5), $bb0(a5)
pub fn i_c1264a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12650;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xBB0);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C12650: subq.w #$1, $bb0(a5)
pub fn i_c12650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12654;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0xBB0);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C12654: clr.w $bb2(a5)
pub fn i_c12654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12658;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xBB2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C12658: clr.l $bb6(a5)
pub fn i_c12658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1265C;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0xBB6);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C1265C: move.w #$384, $b90(a5)
pub fn i_c1265c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12662;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x384;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12662: clr.w $bb4(a5)
pub fn i_c12662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12666;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xBB4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C12666: jsr $c12958(pc)
pub fn i_c12666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1266A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1266A);
    cpu.pc = 0xC12958;
    16
}
/// $C1266A: addq.b #$1, $ac2(a5)
pub fn i_c1266a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1266E;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1266E: movea.l $abe(a5), a0
pub fn i_c1266e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12672;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12672: move.l #$c12848, (a0)+
pub fn i_c12672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12678;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC12848;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C12678: tst.w $bb2(a5)
pub fn i_c12678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1267C;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C1267C: beq.b $c12684
pub fn i_c1267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1267E;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12684; return 10; }
    8
}
/// $C1267E: move.l #$c128de, (a0)+
pub fn i_c1267e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12684;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC128DE;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C12684: move.l a0, $abe(a5)
pub fn i_c12684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12688;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12688: subq.b #$1, $10fdc2.l
pub fn i_c12688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1268E;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10FDC2) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $C1268E: move.w $bb2(a5), $acc(a5)
pub fn i_c1268e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12694;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACC);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C12694: move.l #$10fe9c, $ac8(a5)
pub fn i_c12694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1269C;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FE9C;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1269C: move.b #$5, $ac4(a5)
pub fn i_c1269c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126A2;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C126A2: jsr $c1231e(pc)
pub fn i_c126a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC126A6);
    cpu.pc = 0xC1231E;
    16
}
/// $C126A6: tst.b $ac6(a5)
pub fn i_c126a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126AA;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C126AA: bne.w $c1278e
pub fn i_c126aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1278E; return 10; }
    8
}
/// $C126AE: addq.b #$1, $ac2(a5)
pub fn i_c126ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126B2;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C126B2: movea.l $abe(a5), a0
pub fn i_c126b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126B6;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C126B6: clr.l (a0)+
pub fn i_c126b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126B8;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C126B8: move.w #$3, (a0)+
pub fn i_c126b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126BC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C126BC: move.w $bb4(a5), d0
pub fn i_c126bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126C0;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C126C0: add.w d0, d0
pub fn i_c126c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126C2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C126C2: addi.w #$716f, d0
pub fn i_c126c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126C6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x716F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C126C6: move.w d0, (a0)+
pub fn i_c126c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126C8;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C126C8: move.w #$109, (a0)+
pub fn i_c126c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126CC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x109;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C126CC: lea.l $b9c(a5), a1
pub fn i_c126cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126D0;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xB9C);
    cpu.a[1] = ad1;
    16
}
/// $C126D0: moveq #$9, d7
pub fn i_c126d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126D2;
    cpu.ir = 0x7E09;
    cpu.d[7] = 0x9;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C126D2: move.w (a1)+, (a0)+
pub fn i_c126d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126D4;
    cpu.ir = 0x30D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C126D4: dbra d7, $c126d2
pub fn i_c126d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126D8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC126D8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC126D8; return 14; }
    cpu.pc = 0xC126D2;
    10
}
/// $C126D8: move.l #$ff000000, (a0)+
pub fn i_c126d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126DE;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFF000000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C126DE: move.l a0, $abe(a5)
pub fn i_c126de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126E2;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C126E2: subq.b #$1, $10fdc2.l
pub fn i_c126e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126E8;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10FDC2) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $C126E8: jsr $c17290(pc)
pub fn i_c126e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC126EC);
    cpu.pc = 0xC17290;
    16
}
/// $C126EC: move.w $bb2(a5), d0
pub fn i_c126ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126F0;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C126F0: addq.w #$1, d0
pub fn i_c126f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126F2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C126F2: move.w d0, $bb2(a5)
pub fn i_c126f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126F6;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C126F6: cmp.w $bb0(a5), d0
pub fn i_c126f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126FA;
    cpu.ir = 0xB06D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C126FA: bhi.b $c1271e
pub fn i_c126fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC126FC;
    cpu.ir = 0x6222;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC1271E; return 10; }
    8
}
/// $C126FC: addq.w #$1, $bb4(a5)
pub fn i_c126fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12700;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0xBB4);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C12700: move.w $bb4(a5), d0
pub fn i_c12700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12704;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12704: subq.w #$6, d0
pub fn i_c12704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12706;
    cpu.ir = 0x5D40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12706: bne.b $c1268e
pub fn i_c12706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12708;
    cpu.ir = 0x6686;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1268E; return 10; }
    8
}
/// $C12708: addq.b #$1, $ac2(a5)
pub fn i_c12708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1270C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1270C: movea.l $abe(a5), a0
pub fn i_c1270c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12710;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12710: move.l #$c128fc, (a0)+
pub fn i_c12710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12716;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC128FC;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C12716: move.l a0, $abe(a5)
pub fn i_c12716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1271A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1271A: subq.b #$1, $ac2(a5)
pub fn i_c1271a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1271E;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1271E: jsr $c127f6(pc)
pub fn i_c1271e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12722;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12722);
    cpu.pc = 0xC127F6;
    16
}
/// $C12722: jsr $c12916(pc)
pub fn i_c12722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12726;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12726);
    cpu.pc = 0xC12916;
    16
}
/// $C12726: tst.b $a82(a5)
pub fn i_c12726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1272A;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1272A: beq.b $c12734
pub fn i_c1272a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1272C;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12734; return 10; }
    8
}
/// $C1272C: subq.w #$1, $b90(a5)
pub fn i_c1272c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12730;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0xB90);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C12730: beq.w $c1278e
pub fn i_c12730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12734;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1278E; return 10; }
    8
}
/// $C12734: btst.b #$5, d0
pub fn i_c12734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12738;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C12738: bne.w $c1278e
pub fn i_c12738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1273C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1278E; return 10; }
    8
}
/// $C1273C: btst.b #$4, d0
pub fn i_c1273c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12740;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C12740: bne.w $c1279a
pub fn i_c12740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12744;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1279A; return 10; }
    8
}
/// $C12744: swap d0
pub fn i_c12744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12746;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12746: andi.b #$3, d0
pub fn i_c12746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1274A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1274A: beq.b $c12722
pub fn i_c1274a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1274C;
    cpu.ir = 0x67D6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12722; return 10; }
    8
}
/// $C1274C: moveq #$1, d1
pub fn i_c1274c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1274E;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1274E: andi.b #$2, d0
pub fn i_c1274e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12752;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12752: bne.b $c12756
pub fn i_c12752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12754;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12756; return 10; }
    8
}
/// $C12754: moveq #$ff, d1
pub fn i_c12754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12756;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12756: move.w d1, d0
pub fn i_c12756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12758;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12758: add.w $bb6(a5), d0
pub fn i_c12758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1275C;
    cpu.ir = 0xD06D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C1275C: cmp.w $bb0(a5), d0
pub fn i_c1275c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12760;
    cpu.ir = 0xB06D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C12760: bhi.b $c12722
pub fn i_c12760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12762;
    cpu.ir = 0x62C0;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC12722; return 10; }
    8
}
/// $C12762: move.w d0, $bb8(a5)
pub fn i_c12762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12766;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12766: add.w $bb4(a5), d1
pub fn i_c12766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1276A;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C1276A: cmpi.w #$6, d1
pub fn i_c1276a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1276E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $C1276E: bcs.b $c1271e
pub fn i_c1276e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12770;
    cpu.ir = 0x65AE;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1271E; return 10; }
    8
}
/// $C12770: tst.w d1
pub fn i_c12770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12772;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C12772: bpl.w $c12662
pub fn i_c12772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12776;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC12662; return 10; }
    8
}
/// $C12776: andi.l #$ffff, d0
pub fn i_c12776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1277C;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C1277C: move.w d0, d2
pub fn i_c1277c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1277E;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1277E: divu.w #$6, d0
pub fn i_c1277e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12782;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x6;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C12782: swap d0
pub fn i_c12782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12784;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12784: sub.w d0, d2
pub fn i_c12784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12786;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C12786: move.w d2, $bb2(a5)
pub fn i_c12786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1278A;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1278A: bra.w $c12662
pub fn i_c1278a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1278E;
    cpu.ir = 0x6000;
    cpu.pc = 0xC12662;
    10
}
/// $C1278E: jsr $c127d6(pc)
pub fn i_c1278e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12792;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12792);
    cpu.pc = 0xC127D6;
    16
}
/// $C12792: move.b $ac7(a5), $ac6(a5)
pub fn i_c12792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12798;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAC7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C12798: rts
pub fn i_c12798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1279A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1279A: move.w $bb6(a5), $acc(a5)
pub fn i_c1279a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127A0;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACC);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C127A0: move.l #$10fe9c, $ac8(a5)
pub fn i_c127a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127A8;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FE9C;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C127A8: move.b #$5, $ac4(a5)
pub fn i_c127a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127AE;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C127AE: jsr $c1231e(pc)
pub fn i_c127ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC127B2);
    cpu.pc = 0xC1231E;
    16
}
/// $C127B2: move.b #$4, $ac4(a5)
pub fn i_c127b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127B8;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C127B8: jsr $c1231e(pc)
pub fn i_c127b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC127BC);
    cpu.pc = 0xC1231E;
    16
}
/// $C127BC: jsr $c127d6(pc)
pub fn i_c127bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC127C0);
    cpu.pc = 0xC127D6;
    16
}
/// $C127C0: jsr $c1231e(pc)
pub fn i_c127c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC127C4);
    cpu.pc = 0xC1231E;
    16
}
/// $C127C4: move.b $ac6(a5), $ac7(a5)
pub fn i_c127c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127CA;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAC7);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C127CA: cmpi.b #$84, $ac6(a5)
pub fn i_c127ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127D0;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x84, Size::Byte, false);
    16
}
/// $C127D0: beq.w $c12610
pub fn i_c127d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12610; return 10; }
    8
}
/// $C127D4: rts
pub fn i_c127d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C127D6: move.b $ac5(a5), $ac4(a5)
pub fn i_c127d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127DC;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAC5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C127DC: move.l $b92(a5), $ac8(a5)
pub fn i_c127dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127E2;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xB92);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C127E2: move.w $b96(a5), $acc(a5)
pub fn i_c127e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127E8;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xB96);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACC);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C127E8: move.w $b98(a5), $ace(a5)
pub fn i_c127e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127EE;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xB98);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACE);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C127EE: move.w $b9a(a5), $ad0(a5)
pub fn i_c127ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127F4;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xB9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAD0);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C127F4: rts
pub fn i_c127f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C127F6: addq.b #$1, $ac2(a5)
pub fn i_c127f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127FA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C127FA: movea.l $abe(a5), a0
pub fn i_c127fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC127FE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C127FE: moveq #$0, d0
pub fn i_c127fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12800;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12800: move.l d0, (a0)+
pub fn i_c12800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12802;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C12802: move.w #$20ff, d1
pub fn i_c12802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12806;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12806: jsr $c12826(pc)
pub fn i_c12806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1280A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1280A);
    cpu.pc = 0xC12826;
    16
}
/// $C1280A: moveq #$0, d0
pub fn i_c1280a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1280C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1280C: move.w $bb8(a5), $bb6(a5)
pub fn i_c1280c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12812;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xBB6);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C12812: move.w #$11ff, d1
pub fn i_c12812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12816;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12816: jsr $c12826(pc)
pub fn i_c12816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1281A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1281A);
    cpu.pc = 0xC12826;
    16
}
/// $C1281A: clr.w (a0)+
pub fn i_c1281a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1281C;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1281C: move.l a0, $abe(a5)
pub fn i_c1281c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12820;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12820: subq.b #$1, $ac2(a5)
pub fn i_c12820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12824;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12824: rts
pub fn i_c12824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12826;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12826: move.w #$3, (a0)+
pub fn i_c12826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1282A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1282A: move.w $bb6(a5), d0
pub fn i_c1282a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1282E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1282E: divu.w #$6, d0
pub fn i_c1282e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12832;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x6;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C12832: swap d0
pub fn i_c12832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12834;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12834: move.w d0, $bb4(a5)
pub fn i_c12834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12838;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12838: add.w d0, d0
pub fn i_c12838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1283A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1283A: addi.w #$712f, d0
pub fn i_c1283a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1283E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x712F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1283E: move.w d0, (a0)+
pub fn i_c1283e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12840;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12840: move.w #$1108, (a0)+
pub fn i_c12840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12844;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12844: move.w d1, (a0)+
pub fn i_c12844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12846;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12846: rts
pub fn i_c12846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12848;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12848: ori.b #$64, d3
pub fn i_c12848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1284C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x64) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1284C: movep.w $4d45(a0), d0
pub fn i_c1284c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12850;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4D45);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C12850: ?
pub fn i_c12850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12852;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12852: addq.w #$1, (a1)+
pub fn i_c12852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12854;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C12854: movea.l d3, a0
pub fn i_c12854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12856;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12856: ?
pub fn i_c12856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12858;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12858: neg.b -(a0)
pub fn i_c12858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1285A;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C1285A: not.w (a5)
pub fn i_c1285a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1285C;
    cpu.ir = 0x4655;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = !v2 & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1285C: ?
pub fn i_c1285c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1285E;
    cpu.ir = 0x4C4C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1285E: movea.l (a3), a6
pub fn i_c1285e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12860;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $C12860: ?
pub fn i_c12860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12862;
    cpu.ir = 0x454C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12862: ?
pub fn i_c12862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12864;
    cpu.ir = 0x4543;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12864: addq.b #$2, -(a0)
pub fn i_c12864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12866;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C12866: ?
pub fn i_c12866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12868;
    cpu.ir = 0x4741;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12868: ?
pub fn i_c12868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1286A;
    cpu.ir = 0x4D45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1286A: movea.l d4, a0
pub fn i_c1286a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1286C;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1286C: ?
pub fn i_c1286c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1286E;
    cpu.ir = 0x4154;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C128DE: ori.b #$6d, d3
pub fn i_c128de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128E2;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x6D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C128E2: movep.w $12ff(a0), d0
pub fn i_c128e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128E6;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x12FF);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C128E6: ori.b #$ad, d3
pub fn i_c128e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128EA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xAD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C128EA: movep.w $5052(a0), d0
pub fn i_c128ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128EE;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5052);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C128EE: ?
pub fn i_c128ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128F0;
    cpu.ir = 0x4556;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C128F0: ?
pub fn i_c128f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128F2;
    cpu.ir = 0x494F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C128F2: subq.w #$2, (a3)
pub fn i_c128f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128F4;
    cpu.ir = 0x5553;
    let ad1 = cpu.a[3];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C128F4: movea.l (a0), a0
pub fn i_c128f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128F6;
    cpu.ir = 0x2050;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[0] = v2;
    12
}
/// $C128F6: ?
pub fn i_c128f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC128F8;
    cpu.ir = 0x4147;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C128FC: ori.b #$7b, d3
pub fn i_c128fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12900;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x7B) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12900: movep.w $13ff(a0), d0
pub fn i_c12900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12904;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x13FF);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C12904: ori.b #$bb, d3
pub fn i_c12904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12908;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xBB) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12908: movep.w $4e45(a0), d0
pub fn i_c12908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1290C;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4E45);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1290C: addq.w #$4, (a4)
pub fn i_c1290c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1290E;
    cpu.ir = 0x5854;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1290E: movea.l (a0), a0
pub fn i_c1290e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12910;
    cpu.ir = 0x2050;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[0] = v2;
    12
}
/// $C12910: ?
pub fn i_c12910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12912;
    cpu.ir = 0x4147;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C12916: jsr $c1727e(pc)
pub fn i_c12916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1291A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1291A);
    cpu.pc = 0xC1727E;
    16
}
/// $C1291A: lea.l $10f300.l, a5
pub fn i_c1291a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12920;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C12920: move.b $a98(a5), d0
pub fn i_c12920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12924;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C12924: or.b $a9e(a5), d0
pub fn i_c12924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12928;
    cpu.ir = 0x802D;
    let ad2 = cpu.a[5].wrapping_add(0xA9E);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12928: swap d0
pub fn i_c12928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1292A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1292A: move.b $a97(a5), d0
pub fn i_c1292a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1292E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1292E: or.b $a9d(a5), d0
pub fn i_c1292e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12932;
    cpu.ir = 0x802D;
    let ad2 = cpu.a[5].wrapping_add(0xA9D);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12932: cmpi.b #$3, $a94(a5)
pub fn i_c12932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12938;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA94);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $C12938: bne.b $c12956
pub fn i_c12938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1293A;
    cpu.ir = 0x661C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12956; return 10; }
    8
}
/// $C1293A: move.b $aa1(a5), d0
pub fn i_c1293a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1293E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAA1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1293E: bclr.b #$7, d0
pub fn i_c1293e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12942;
    cpu.ir = 0x880;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
/// $C12942: btst.b #$0, $aa2(a5)
pub fn i_c12942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12948;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xAA2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C12948: beq.b $c1294e
pub fn i_c12948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1294A;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1294E; return 10; }
    8
}
/// $C1294A: bset.b #$7, d0
pub fn i_c1294a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1294E;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $C1294E: ror.b #$4, d0
pub fn i_c1294e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12950;
    cpu.ir = 0xE818;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 3, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C12950: move.b d0, d1
pub fn i_c12950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12952;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C12952: swap d0
pub fn i_c12952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12954;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12954: move.b d1, d0
pub fn i_c12954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12956;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C12956: rts
pub fn i_c12956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12958;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12958: tst.b $bba(a5)
pub fn i_c12958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1295C;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBA);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1295C: bne.b $c129b8
pub fn i_c1295c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1295E;
    cpu.ir = 0x665A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC129B8; return 10; }
    8
}
/// $C1295E: addq.b #$1, $ac2(a5)
pub fn i_c1295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12962;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12962: move.b #$1, $bba(a5)
pub fn i_c12962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12968;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBA);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12968: lea.l $3c0002.l, a1
pub fn i_c12968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1296E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C1296E: move.w #$1, $2(a1)
pub fn i_c1296e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12974;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12974: movea.l a5, a0
pub fn i_c12974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12976;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12976: move.w #$7000, -$2(a1)
pub fn i_c12976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1297C;
    cpu.ir = 0x337C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1297C: move.w #$4ff, d7
pub fn i_c1297c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12980;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x4FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12980: move.b d0, $300001.l
pub fn i_c12980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12986;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12986: move.w (a1), d0
pub fn i_c12986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12988;
    cpu.ir = 0x3011;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12988: move.w d0, (a1)
pub fn i_c12988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1298A;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C1298A: move.w d0, (a0)+
pub fn i_c1298a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1298C;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1298C: dbra d7, $c12980
pub fn i_c1298c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12990;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12990; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12990; return 14; }
    cpu.pc = 0xC12980;
    10
}
/// $C12990: subq.b #$1, $ac2(a5)
pub fn i_c12990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12994;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12994: move.l $400002.l, $a4e(a5)
pub fn i_c12994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1299C;
    cpu.ir = 0x2B79;
    let v2 = bus.read32(0x400002);
    let ad3 = cpu.a[5].wrapping_add(0xA4E);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C1299C: move.l $400022.l, $a52(a5)
pub fn i_c1299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129A4;
    cpu.ir = 0x2B79;
    let v2 = bus.read32(0x400022);
    let ad3 = cpu.a[5].wrapping_add(0xA52);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C129A4: move.l #$eee0000, $400002.l
pub fn i_c129a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129AE;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C129AE: move.l #$e000000, $400022.l
pub fn i_c129ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129B8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C129B8: jsr $c12a0e(pc)
pub fn i_c129b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC129BC);
    cpu.pc = 0xC12A0E;
    16
}
/// $C129BC: rts
pub fn i_c129bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C129BE: tst.b $bba(a5)
pub fn i_c129be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129C2;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBA);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C129C2: beq.b $c12a0c
pub fn i_c129c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129C4;
    cpu.ir = 0x6748;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12A0C; return 10; }
    8
}
/// $C129C4: addq.b #$1, $ac2(a5)
pub fn i_c129c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129C8;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C129C8: move.l #$10ff00, $abe(a5)
pub fn i_c129c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129D0;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C129D0: lea.l $3c0002.l, a1
pub fn i_c129d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129D6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C129D6: move.w #$1, $2(a1)
pub fn i_c129d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129DC;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C129DC: movea.l a5, a0
pub fn i_c129dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129DE;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C129DE: move.w #$7000, -$2(a1)
pub fn i_c129de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129E4;
    cpu.ir = 0x337C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C129E4: move.w #$4ff, d7
pub fn i_c129e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129E8;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x4FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C129E8: move.b d0, $300001.l
pub fn i_c129e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129EE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C129EE: move.w (a0)+, (a1)
pub fn i_c129ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129F0;
    cpu.ir = 0x3298;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v3 as u16);
    12
}
/// $C129F0: dbra d7, $c129e8
pub fn i_c129f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129F4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC129F4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC129F4; return 14; }
    cpu.pc = 0xC129E8;
    10
}
/// $C129F4: move.l $a4e(a5), $400002.l
pub fn i_c129f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC129FC;
    cpu.ir = 0x23ED;
    let ad2 = cpu.a[5].wrapping_add(0xA4E);
    let v3 = bus.read32(ad2);
    bus.write32(0x400002, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C129FC: move.l $a52(a5), $400022.l
pub fn i_c129fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A04;
    cpu.ir = 0x23ED;
    let ad2 = cpu.a[5].wrapping_add(0xA52);
    let v3 = bus.read32(ad2);
    bus.write32(0x400022, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C12A04: subq.b #$1, $ac2(a5)
pub fn i_c12a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A08;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12A08: clr.b $bba(a5)
pub fn i_c12a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A0C;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBA);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12A0C: rts
pub fn i_c12a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12A0E: addq.b #$1, $ac2(a5)
pub fn i_c12a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A12;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12A12: move.l #$10ff00, $abe(a5)
pub fn i_c12a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A1A;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12A1A: lea.l $3c0002.l, a1
pub fn i_c12a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C12A20: move.w #$1, $2(a1)
pub fn i_c12a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A26;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12A26: move.w #$20, d0
pub fn i_c12a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A2A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12A2A: move.w #$7000, -$2(a1)
pub fn i_c12a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A30;
    cpu.ir = 0x337C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12A30: move.w #$4ff, d7
pub fn i_c12a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A34;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x4FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12A34: move.b d0, $300001.l
pub fn i_c12a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A3A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12A3A: move.w d0, (a1)
pub fn i_c12a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A3C;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C12A3C: dbra d7, $c12a34
pub fn i_c12a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A40;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12A40; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12A40; return 14; }
    cpu.pc = 0xC12A34;
    10
}
/// $C12A40: subq.b #$1, $ac2(a5)
pub fn i_c12a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A44;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C12A44: rts
pub fn i_c12a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12A46: move.b $380000.l, d0
pub fn i_c12a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A4C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C12A4C: andi.b #$40, d0
pub fn i_c12a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A50;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C12A50: beq.b $c12a5a
pub fn i_c12a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A52;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12A5A; return 10; }
    8
}
/// $C12A52: move.b #$85, $ac6(a5)
pub fn i_c12a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A58;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x85;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C12A58: rts
pub fn i_c12a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12A5A: jsr $c13a0c(pc)
pub fn i_c12a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12A5E);
    cpu.pc = 0xC13A0C;
    16
}
/// $C12A5E: move.w d6, $a16(a5)
pub fn i_c12a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A62;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12A62: beq.b $c12a52
pub fn i_c12a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A64;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12A52; return 10; }
    8
}
/// $C12A64: tst.b $a15(a5)
pub fn i_c12a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A68;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12A68: beq.w $c12a76
pub fn i_c12a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12A76; return 10; }
    8
}
/// $C12A6C: bmi.w $c12c04
pub fn i_c12a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A70;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC12C04; return 10; }
    8
}
/// $C12A70: jsr $c13dba(pc)
pub fn i_c12a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12A74);
    cpu.pc = 0xC13DBA;
    16
}
/// $C12A74: bra.b $c12a7a
pub fn i_c12a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A76;
    cpu.ir = 0x6004;
    cpu.pc = 0xC12A7A;
    10
}
/// $C12A76: jsr $c13d02(pc)
pub fn i_c12a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12A7A);
    cpu.pc = 0xC13D02;
    16
}
/// $C12A7A: movea.l a6, a0
pub fn i_c12a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A7C;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12A7C: moveq #$0, d0
pub fn i_c12a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A7E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12A7E: moveq #$9, d7
pub fn i_c12a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A80;
    cpu.ir = 0x7E09;
    cpu.d[7] = 0x9;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12A80: move.w d0, (a0)+
pub fn i_c12a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A82;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12A82: dbra d7, $c12a80
pub fn i_c12a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A86;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12A86; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12A86; return 14; }
    cpu.pc = 0xC12A80;
    10
}
/// $C12A86: move.w $a16(a5), d1
pub fn i_c12a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A8A;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xA16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12A8A: lsr.w #$8, d1
pub fn i_c12a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A8C;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12A8C: move.w d1, (a0)+
pub fn i_c12a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A8E;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12A8E: move.w $a16(a5), (a0)+
pub fn i_c12a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A92;
    cpu.ir = 0x30ED;
    let ad2 = cpu.a[5].wrapping_add(0xA16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C12A92: move.w $c10000(pc), (a0)+
pub fn i_c12a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A96;
    cpu.ir = 0x30FA;
    let v2 = bus.read16(0xC10000) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0]; cpu.a[0] = ad3.wrapping_add(2);
    bus.write16(ad3, v2 as u16);
    16
}
/// $C12A96: move.w d0, (a0)+
pub fn i_c12a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A98;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12A98: move.w d0, (a0)+
pub fn i_c12a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A9A;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12A9A: move.w d0, (a0)+
pub fn i_c12a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12A9C;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12A9C: move.w #$20, d0
pub fn i_c12a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AA0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12AA0: moveq #$f, d7
pub fn i_c12aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AA2;
    cpu.ir = 0x7E0F;
    cpu.d[7] = 0xF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12AA2: move.w d0, (a0)+
pub fn i_c12aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AA4;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12AA4: dbra d7, $c12aa2
pub fn i_c12aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AA8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12AA8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12AA8; return 14; }
    cpu.pc = 0xC12AA2;
    10
}
/// $C12AA8: lea.l $c134ba(pc), a1
pub fn i_c12aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC134BA;
    16
}
/// $C12AAC: moveq #$7, d7
pub fn i_c12aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AAE;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12AAE: move.b (a1)+, d0
pub fn i_c12aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AB0;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C12AB0: move.w d0, (a0)
pub fn i_c12ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AB2;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12AB2: addq.l #$4, a0
pub fn i_c12ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AB4;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12AB4: dbra d7, $c12aae
pub fn i_c12ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AB8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12AB8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12AB8; return 14; }
    cpu.pc = 0xC12AAE;
    10
}
/// $C12AB8: move.b $c00401.l, d0
pub fn i_c12ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12ABE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C12ABE: move.w d0, (a0)
pub fn i_c12abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AC0;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12AC0: tst.b $a15(a5)
pub fn i_c12ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AC4;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12AC4: bne.w $c12b5c
pub fn i_c12ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AC8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12B5C; return 10; }
    8
}
/// $C12AC8: movea.l $a04(a5), a0
pub fn i_c12ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12ACC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12ACC: move.w $a08(a5), d7
pub fn i_c12acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AD0;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12AD0: move.w #$ff, d0
pub fn i_c12ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AD4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12AD4: move.w d0, (a0)
pub fn i_c12ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AD6;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12AD6: addq.l #$8, a0
pub fn i_c12ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AD8;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C12AD8: dbra d7, $c12ad4
pub fn i_c12ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12ADC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12ADC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12ADC; return 14; }
    cpu.pc = 0xC12AD4;
    10
}
/// $C12ADC: move.b d0, $300001.l
pub fn i_c12adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AE2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12AE2: move.l $a10(a5), d6
pub fn i_c12ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AE6;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0xA10);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12AE6: sub.l a6, d6
pub fn i_c12ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AE8;
    cpu.ir = 0x9C8E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $C12AE8: lsr.w #$7, d6
pub fn i_c12ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AEA;
    cpu.ir = 0xEE4E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $C12AEA: subq.w #$1, d6
pub fn i_c12aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AEC;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12AEC: movea.l $a0a(a5), a0
pub fn i_c12aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AF0;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12AF0: jsr $c12b38(pc)
pub fn i_c12af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12AF4);
    cpu.pc = 0xC12B38;
    16
}
/// $C12AF4: moveq #$0, d0
pub fn i_c12af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AF6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12AF6: move.w $a0e(a5), d0
pub fn i_c12af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AFA;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12AFA: movea.l $a0a(a5), a0
pub fn i_c12afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12AFE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12AFE: adda.l d0, a0
pub fn i_c12afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B00;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C12B00: jsr $c12b38(pc)
pub fn i_c12b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12B04);
    cpu.pc = 0xC12B38;
    16
}
/// $C12B04: movea.l $a0a(a5), a0
pub fn i_c12b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B08;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12B08: moveq #$0, d0
pub fn i_c12b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B0A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12B0A: move.w $a0e(a5), d7
pub fn i_c12b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B0E;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12B0E: lsr.w #$1, d7
pub fn i_c12b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B10;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C12B10: subq.w #$1, d7
pub fn i_c12b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B12;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12B12: move.w d7, d6
pub fn i_c12b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B14;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12B14: jsr $c12b2e(pc)
pub fn i_c12b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12B18);
    cpu.pc = 0xC12B2E;
    16
}
/// $C12B18: nop
pub fn i_c12b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B1A;
    cpu.ir = 0x4E71;
    4
}
/// $C12B1A: move.w d0, $1a(a6)
pub fn i_c12b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12B1E: moveq #$0, d0
pub fn i_c12b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B20;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12B20: move.w d7, d6
pub fn i_c12b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B22;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12B22: jsr $c12b2e(pc)
pub fn i_c12b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12B26);
    cpu.pc = 0xC12B2E;
    16
}
/// $C12B26: nop
pub fn i_c12b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B28;
    cpu.ir = 0x4E71;
    4
}
/// $C12B28: move.w d0, $1c(a6)
pub fn i_c12b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B2C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12B2C: rts
pub fn i_c12b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12B2E: move.w (a0)+, d1
pub fn i_c12b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B30;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C12B30: add.b d1, d0
pub fn i_c12b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B32;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C12B32: dbra d6, $c12b2e
pub fn i_c12b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B36;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12B36; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12B36; return 14; }
    cpu.pc = 0xC12B2E;
    10
}
/// $C12B36: rts
pub fn i_c12b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12B38: movem.l a0, -(a7)
pub fn i_c12b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B3C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C12B3C: move.w $a0e(a5), d7
pub fn i_c12b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B40;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12B40: lsr.w #$1, d7
pub fn i_c12b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B42;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C12B42: subq.w #$1, d7
pub fn i_c12b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B44;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12B44: moveq #$0, d0
pub fn i_c12b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B46;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12B46: move.w d0, (a0)+
pub fn i_c12b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B48;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12B48: dbra d7, $c12b46
pub fn i_c12b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B4C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12B4C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12B4C; return 14; }
    cpu.pc = 0xC12B46;
    10
}
/// $C12B4C: movem.l (a7)+, a0
pub fn i_c12b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B50;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C12B50: moveq #$2, d0
pub fn i_c12b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B52;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12B52: move.w d6, d5
pub fn i_c12b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B54;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12B54: move.w d0, (a0)+
pub fn i_c12b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B56;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12B56: dbra d5, $c12b54
pub fn i_c12b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B5A;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12B5A; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12B5A; return 14; }
    cpu.pc = 0xC12B54;
    10
}
/// $C12B5A: rts
pub fn i_c12b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12B5C: move.w #$1, $12(a6)
pub fn i_c12b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B62;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12B62: movea.l $a04(a5), a0
pub fn i_c12b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B66;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12B66: move.w $a08(a5), d7
pub fn i_c12b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B6A;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12B6A: move.w #$ff00, d0
pub fn i_c12b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B6E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12B6E: move.w d0, (a0)
pub fn i_c12b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B70;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12B70: addq.l #$4, a0
pub fn i_c12b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B72;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12B72: dbra d7, $c12b6e
pub fn i_c12b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B76;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12B76; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12B76; return 14; }
    cpu.pc = 0xC12B6E;
    10
}
/// $C12B76: move.b d0, $300001.l
pub fn i_c12b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B7C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12B7C: move.l $a10(a5), d6
pub fn i_c12b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B80;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0xA10);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12B80: sub.l a6, d6
pub fn i_c12b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B82;
    cpu.ir = 0x9C8E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $C12B82: lsr.w #$6, d6
pub fn i_c12b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B84;
    cpu.ir = 0xEC4E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $C12B84: movea.l $a0a(a5), a0
pub fn i_c12b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B88;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12B88: jsr $c12bd4(pc)
pub fn i_c12b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12B8C);
    cpu.pc = 0xC12BD4;
    16
}
/// $C12B8C: moveq #$0, d0
pub fn i_c12b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B8E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12B8E: move.w $a0e(a5), d0
pub fn i_c12b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B92;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12B92: movea.l $a0a(a5), a0
pub fn i_c12b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B96;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12B96: adda.l d0, a0
pub fn i_c12b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B98;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C12B98: jsr $c12bd4(pc)
pub fn i_c12b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12B9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12B9C);
    cpu.pc = 0xC12BD4;
    16
}
/// $C12B9C: movea.l $a0a(a5), a0
pub fn i_c12b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BA0;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12BA0: moveq #$0, d0
pub fn i_c12ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BA2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12BA2: move.w $a0e(a5), d7
pub fn i_c12ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BA6;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12BA6: lsr.w #$1, d7
pub fn i_c12ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BA8;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C12BA8: subq.w #$1, d7
pub fn i_c12ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BAA;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12BAA: move.w d7, d6
pub fn i_c12baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BAC;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12BAC: jsr $c12bc6(pc)
pub fn i_c12bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12BB0);
    cpu.pc = 0xC12BC6;
    16
}
/// $C12BB0: nop
pub fn i_c12bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BB2;
    cpu.ir = 0x4E71;
    4
}
/// $C12BB2: move.w d0, $1a(a6)
pub fn i_c12bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BB6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12BB6: moveq #$0, d0
pub fn i_c12bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BB8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12BB8: move.w d7, d6
pub fn i_c12bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BBA;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12BBA: jsr $c12bc6(pc)
pub fn i_c12bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12BBE);
    cpu.pc = 0xC12BC6;
    16
}
/// $C12BBE: nop
pub fn i_c12bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BC0;
    cpu.ir = 0x4E71;
    4
}
/// $C12BC0: move.w d0, $1c(a6)
pub fn i_c12bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BC4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12BC4: rts
pub fn i_c12bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12BC6: move.w (a0)+, d1
pub fn i_c12bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BC8;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C12BC8: add.b d1, d0
pub fn i_c12bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BCA;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C12BCA: lsr.w #$8, d1
pub fn i_c12bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BCC;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12BCC: add.b d1, d0
pub fn i_c12bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BCE;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C12BCE: dbra d6, $c12bc6
pub fn i_c12bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BD2;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12BD2; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12BD2; return 14; }
    cpu.pc = 0xC12BC6;
    10
}
/// $C12BD2: rts
pub fn i_c12bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12BD4: movem.l a0, -(a7)
pub fn i_c12bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BD8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C12BD8: move.w $a0e(a5), d7
pub fn i_c12bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BDC;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12BDC: lsr.w #$1, d7
pub fn i_c12bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BDE;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C12BDE: subq.w #$1, d7
pub fn i_c12bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BE0;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12BE0: moveq #$0, d0
pub fn i_c12be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BE2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12BE2: move.w d0, (a0)+
pub fn i_c12be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BE4;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12BE4: dbra d7, $c12be2
pub fn i_c12be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BE8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12BE8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12BE8; return 14; }
    cpu.pc = 0xC12BE2;
    10
}
/// $C12BE8: movem.l (a7)+, a0
pub fn i_c12be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BEC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C12BEC: move.w #$202, d0
pub fn i_c12bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BF0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x202;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12BF0: move.w d6, d5
pub fn i_c12bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BF2;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12BF2: cmpi.w #$1, d5
pub fn i_c12bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BF6;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $C12BF6: bne.b $c12bfc
pub fn i_c12bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BF8;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12BFC; return 10; }
    8
}
/// $C12BF8: clr.b d0
pub fn i_c12bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BFA;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C12BFA: moveq #$2, d5
pub fn i_c12bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BFC;
    cpu.ir = 0x7A02;
    cpu.d[5] = 0x2;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12BFC: move.w d0, (a0)+
pub fn i_c12bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12BFE;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C12BFE: subq.w #$2, d5
pub fn i_c12bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C00;
    cpu.ir = 0x5545;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12C00: bne.b $c12bfc
pub fn i_c12c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C02;
    cpu.ir = 0x66FA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12BFC; return 10; }
    8
}
/// $C12C02: rts
pub fn i_c12c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12C04: jsr $c13d02(pc)
pub fn i_c12c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12C08);
    cpu.pc = 0xC13D02;
    16
}
/// $C12C08: movea.l a6, a0
pub fn i_c12c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C0A;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C12C0A: moveq #$0, d0
pub fn i_c12c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C0C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12C0C: moveq #$4, d7
pub fn i_c12c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C0E;
    cpu.ir = 0x7E04;
    cpu.d[7] = 0x4;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12C0E: move.l d0, (a0)+
pub fn i_c12c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C10;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C12C10: dbra d7, $c12c0e
pub fn i_c12c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C14;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12C14; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12C14; return 14; }
    cpu.pc = 0xC12C0E;
    10
}
/// $C12C14: move.w #$1, $c(a6)
pub fn i_c12c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C1A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12C1A: move.w $a16(a5), (a0)
pub fn i_c12c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C1E;
    cpu.ir = 0x30AD;
    let ad2 = cpu.a[5].wrapping_add(0xA16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v3 as u16);
    16
}
/// $C12C1E: addq.l #$4, a0
pub fn i_c12c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C20;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12C20: moveq #$0, d1
pub fn i_c12c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C22;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12C22: move.b $c10001(pc), d1
pub fn i_c12c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C26;
    cpu.ir = 0x123A;
    let v2 = bus.read8(0xC10001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    12
}
/// $C12C26: move.w d1, (a0)
pub fn i_c12c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C28;
    cpu.ir = 0x3081;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12C28: addq.l #$4, a0
pub fn i_c12c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C2A;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12C2A: move.w d0, (a0)
pub fn i_c12c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C2C;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12C2C: addq.l #$4, a0
pub fn i_c12c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C2E;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12C2E: move.w #$2020, d0
pub fn i_c12c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C32;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12C32: moveq #$7, d7
pub fn i_c12c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C34;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12C34: move.w d0, (a0)
pub fn i_c12c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C36;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12C36: addq.l #$4, a0
pub fn i_c12c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C38;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12C38: dbra d7, $c12c34
pub fn i_c12c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C3C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12C3C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12C3C; return 14; }
    cpu.pc = 0xC12C34;
    10
}
/// $C12C3C: lea.l $c134ba(pc), a1
pub fn i_c12c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC134BA;
    16
}
/// $C12C40: moveq #$7, d7
pub fn i_c12c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C42;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12C42: move.b (a1)+, d0
pub fn i_c12c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C44;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C12C44: move.w d0, (a0)
pub fn i_c12c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C46;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12C46: addq.l #$4, a0
pub fn i_c12c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C48;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12C48: dbra d7, $c12c42
pub fn i_c12c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C4C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12C4C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12C4C; return 14; }
    cpu.pc = 0xC12C42;
    10
}
/// $C12C4C: move.b $c00401.l, d0
pub fn i_c12c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C52;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C12C52: move.w d0, (a0)
pub fn i_c12c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C54;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12C54: movea.l $a04(a5), a0
pub fn i_c12c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C58;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12C58: move.w $a08(a5), d7
pub fn i_c12c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C5C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12C5C: move.w #$ff00, d0
pub fn i_c12c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C60;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12C60: move.w d0, (a0)
pub fn i_c12c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C62;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12C62: addq.l #$8, a0
pub fn i_c12c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C64;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C12C64: dbra d7, $c12c60
pub fn i_c12c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C68;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12C68; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12C68; return 14; }
    cpu.pc = 0xC12C60;
    10
}
/// $C12C68: move.b d0, $300001.l
pub fn i_c12c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C6E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12C6E: move.l $a10(a5), d6
pub fn i_c12c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C72;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0xA10);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C12C72: sub.l a6, d6
pub fn i_c12c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C74;
    cpu.ir = 0x9C8E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $C12C74: lsr.w #$6, d6
pub fn i_c12c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C76;
    cpu.ir = 0xEC4E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $C12C76: movea.l $a0a(a5), a0
pub fn i_c12c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C7A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12C7A: jsr $c12cd0(pc)
pub fn i_c12c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12C7E);
    cpu.pc = 0xC12CD0;
    16
}
/// $C12C7E: moveq #$0, d0
pub fn i_c12c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C80;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12C80: move.w $a0e(a5), d0
pub fn i_c12c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C84;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12C84: movea.l $a0a(a5), a0
pub fn i_c12c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C88;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12C88: adda.l d0, a0
pub fn i_c12c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C8A;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C12C8A: jsr $c12cd0(pc)
pub fn i_c12c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12C8E);
    cpu.pc = 0xC12CD0;
    16
}
/// $C12C8E: movea.l $a0a(a5), a0
pub fn i_c12c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C92;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA0A);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12C92: moveq #$0, d0
pub fn i_c12c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C94;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12C94: move.w $a0e(a5), d7
pub fn i_c12c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C98;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12C98: lsr.w #$1, d7
pub fn i_c12c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C9A;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C12C9A: subq.w #$1, d7
pub fn i_c12c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C9C;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12C9C: move.w d7, d6
pub fn i_c12c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12C9E;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12C9E: jsr $c12cc0(pc)
pub fn i_c12c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12CA2);
    cpu.pc = 0xC12CC0;
    16
}
/// $C12CA2: nop
pub fn i_c12ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CA4;
    cpu.ir = 0x4E71;
    4
}
/// $C12CA4: lsl.w #$8, d0
pub fn i_c12ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CA6;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12CA6: move.w d0, $1c(a6)
pub fn i_c12ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CAA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12CAA: moveq #$0, d0
pub fn i_c12caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CAC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12CAC: move.w d7, d6
pub fn i_c12cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CAE;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12CAE: jsr $c12cc0(pc)
pub fn i_c12cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12CB2);
    cpu.pc = 0xC12CC0;
    16
}
/// $C12CB2: nop
pub fn i_c12cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CB4;
    cpu.ir = 0x4E71;
    4
}
/// $C12CB4: move.w $1c(a6), d1
pub fn i_c12cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CB8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12CB8: move.b d0, d1
pub fn i_c12cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CBA;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C12CBA: move.w d1, $1c(a6)
pub fn i_c12cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CBE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12CBE: rts
pub fn i_c12cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12CC0: move.w (a0), d1
pub fn i_c12cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CC2;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12CC2: add.b d1, d0
pub fn i_c12cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CC4;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C12CC4: lsr.w #$8, d1
pub fn i_c12cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CC6;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12CC6: add.b d1, d0
pub fn i_c12cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CC8;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C12CC8: addq.l #$4, a0
pub fn i_c12cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CCA;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12CCA: dbra d6, $c12cc0
pub fn i_c12cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CCE;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12CCE; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12CCE; return 14; }
    cpu.pc = 0xC12CC0;
    10
}
/// $C12CCE: rts
pub fn i_c12cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12CD0: movem.l a0, -(a7)
pub fn i_c12cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CD4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C12CD4: move.w $a0e(a5), d7
pub fn i_c12cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CD8;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA0E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12CD8: lsr.w #$1, d7
pub fn i_c12cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CDA;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C12CDA: subq.w #$1, d7
pub fn i_c12cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CDC;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12CDC: moveq #$0, d0
pub fn i_c12cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CDE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12CDE: move.w d0, (a0)
pub fn i_c12cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CE0;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12CE0: addq.l #$4, a0
pub fn i_c12ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CE2;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12CE2: dbra d7, $c12cde
pub fn i_c12ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CE6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12CE6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12CE6; return 14; }
    cpu.pc = 0xC12CDE;
    10
}
/// $C12CE6: movem.l (a7)+, a0
pub fn i_c12ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CEA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C12CEA: move.w #$202, d0
pub fn i_c12cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CEE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x202;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12CEE: move.w d6, d5
pub fn i_c12cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CF0;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C12CF0: cmpi.w #$1, d5
pub fn i_c12cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CF4;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $C12CF4: bne.b $c12cfa
pub fn i_c12cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CF6;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12CFA; return 10; }
    8
}
/// $C12CF6: clr.b d0
pub fn i_c12cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CF8;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C12CF8: moveq #$2, d5
pub fn i_c12cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CFA;
    cpu.ir = 0x7A02;
    cpu.d[5] = 0x2;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12CFA: move.w d0, (a0)
pub fn i_c12cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CFC;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C12CFC: addq.l #$4, a0
pub fn i_c12cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12CFE;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12CFE: subq.w #$2, d5
pub fn i_c12cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D00;
    cpu.ir = 0x5545;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C12D00: bne.b $c12cfa
pub fn i_c12d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D02;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12CFA; return 10; }
    8
}
/// $C12D02: rts
pub fn i_c12d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12D04: move.w #$0, $ad0(a5)
pub fn i_c12d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D0A;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12D0A: jsr $c13978(pc)
pub fn i_c12d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12D0E);
    cpu.pc = 0xC13978;
    16
}
/// $C12D0E: tst.b $ac6(a5)
pub fn i_c12d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D12;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12D12: beq.b $c12d16
pub fn i_c12d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D14;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12D16; return 10; }
    8
}
/// $C12D14: rts
pub fn i_c12d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12D16: movea.l $a04(a5), a0
pub fn i_c12d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D1A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C12D1A: moveq #$0, d5
pub fn i_c12d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D1C;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C12D1C: move.w $a08(a5), d7
pub fn i_c12d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D20;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12D20: tst.b $a15(a5)
pub fn i_c12d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D24;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA15);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C12D24: bmi.w $c12d96
pub fn i_c12d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D28;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC12D96; return 10; }
    8
}
/// $C12D28: bne.w $c12d60
pub fn i_c12d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D2C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12D60; return 10; }
    8
}
/// $C12D2C: move.b d0, $300001.l
pub fn i_c12d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D32;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12D32: move.w (a0), d1
pub fn i_c12d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D34;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12D34: cmpi.b #$ff, d1
pub fn i_c12d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D38;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C12D38: beq.b $c12d54
pub fn i_c12d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D3A;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12D54; return 10; }
    8
}
/// $C12D3A: move.w $2(a0), d0
pub fn i_c12d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D3E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12D3E: cmp.b $ace(a5), d0
pub fn i_c12d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D42;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C12D42: bne.b $c12d54
pub fn i_c12d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D44;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12D54; return 10; }
    8
}
/// $C12D44: move.w $4(a0), d0
pub fn i_c12d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D48;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12D48: cmp.b $acf(a5), d0
pub fn i_c12d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D4C;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C12D4C: bne.b $c12d54
pub fn i_c12d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D4E;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12D54; return 10; }
    8
}
/// $C12D4E: andi.w #$f, d1
pub fn i_c12d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D52;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C12D52: bset.b d1, d5
pub fn i_c12d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D54;
    cpu.ir = 0x3C5;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[5] = r2;
    8
}
/// $C12D54: addq.l #$8, a0
pub fn i_c12d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D56;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C12D56: dbra d7, $c12d2c
pub fn i_c12d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D5A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12D5A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12D5A; return 14; }
    cpu.pc = 0xC12D2C;
    10
}
/// $C12D5A: move.w d5, $ad0(a5)
pub fn i_c12d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D5E;
    cpu.ir = 0x3B45;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12D5E: rts
pub fn i_c12d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12D60: move.b d0, $300001.l
pub fn i_c12d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D66;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12D66: move.w (a0), d1
pub fn i_c12d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D68;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12D68: lsr.w #$8, d1
pub fn i_c12d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D6A;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12D6A: cmpi.b #$ff, d1
pub fn i_c12d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D6E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C12D6E: beq.b $c12d8a
pub fn i_c12d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D70;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12D8A; return 10; }
    8
}
/// $C12D70: move.w (a0), d0
pub fn i_c12d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D72;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12D72: cmp.b $ace(a5), d0
pub fn i_c12d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D76;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C12D76: bne.b $c12d8a
pub fn i_c12d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D78;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12D8A; return 10; }
    8
}
/// $C12D78: move.w $2(a0), d0
pub fn i_c12d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D7C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12D7C: lsr.w #$8, d0
pub fn i_c12d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D7E;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12D7E: cmp.b $acf(a5), d0
pub fn i_c12d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D82;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C12D82: bne.b $c12d8a
pub fn i_c12d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D84;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12D8A; return 10; }
    8
}
/// $C12D84: andi.w #$f, d1
pub fn i_c12d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D88;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C12D88: bset.b d1, d5
pub fn i_c12d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D8A;
    cpu.ir = 0x3C5;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[5] = r2;
    8
}
/// $C12D8A: addq.l #$4, a0
pub fn i_c12d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D8C;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C12D8C: dbra d7, $c12d60
pub fn i_c12d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D90;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12D90; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12D90; return 14; }
    cpu.pc = 0xC12D60;
    10
}
/// $C12D90: move.w d5, $ad0(a5)
pub fn i_c12d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D94;
    cpu.ir = 0x3B45;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12D94: rts
pub fn i_c12d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12D96: move.b d0, $300001.l
pub fn i_c12d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D9C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C12D9C: move.w (a0), d1
pub fn i_c12d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12D9E;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12D9E: lsr.w #$8, d1
pub fn i_c12d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DA0;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12DA0: cmpi.b #$ff, d1
pub fn i_c12da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DA4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C12DA4: beq.b $c12dc0
pub fn i_c12da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DA6;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC12DC0; return 10; }
    8
}
/// $C12DA6: move.w (a0), d0
pub fn i_c12da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DA8;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C12DA8: cmp.b $ace(a5), d0
pub fn i_c12da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DAC;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C12DAC: bne.b $c12dc0
pub fn i_c12dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DAE;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12DC0; return 10; }
    8
}
/// $C12DAE: move.w $4(a0), d0
pub fn i_c12dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DB2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C12DB2: lsr.w #$8, d0
pub fn i_c12db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DB4;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C12DB4: cmp.b $acf(a5), d0
pub fn i_c12db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DB8;
    cpu.ir = 0xB02D;
    let ad2 = cpu.a[5].wrapping_add(0xACF);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C12DB8: bne.b $c12dc0
pub fn i_c12db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DBA;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC12DC0; return 10; }
    8
}
/// $C12DBA: andi.w #$f, d1
pub fn i_c12dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DBE;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C12DBE: bset.b d1, d5
pub fn i_c12dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DC0;
    cpu.ir = 0x3C5;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[5] = r2;
    8
}
/// $C12DC0: addq.l #$8, a0
pub fn i_c12dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DC2;
    cpu.ir = 0x5088;
    cpu.a[0] = cpu.a[0].wrapping_add(8);
    8
}
/// $C12DC2: dbra d7, $c12d96
pub fn i_c12dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DC6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC12DC6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC12DC6; return 14; }
    cpu.pc = 0xC12D96;
    10
}
/// $C12DC6: move.w d5, $ad0(a5)
pub fn i_c12dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DCA;
    cpu.ir = 0x3B45;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C12DCA: rts
pub fn i_c12dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DCC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C12DCC: jsr $c13978(pc)
pub fn i_c12dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC12DD0);
    cpu.pc = 0xC13978;
    16
}
/// $C12DD0: tst.b $ac6(a5)
pub fn i_c12dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC12DD4;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC1177C, i_c1177c),
    (0xC1177E, i_c1177e),
    (0xC11782, i_c11782),
    (0xC11786, i_c11786),
    (0xC11788, i_c11788),
    (0xC1178A, i_c1178a),
    (0xC11790, i_c11790),
    (0xC11794, i_c11794),
    (0xC11798, i_c11798),
    (0xC1179C, i_c1179c),
    (0xC117A0, i_c117a0),
    (0xC117A4, i_c117a4),
    (0xC117A8, i_c117a8),
    (0xC117AE, i_c117ae),
    (0xC117B0, i_c117b0),
    (0xC117B4, i_c117b4),
    (0xC117B6, i_c117b6),
    (0xC117B8, i_c117b8),
    (0xC117BC, i_c117bc),
    (0xC117C0, i_c117c0),
    (0xC117C2, i_c117c2),
    (0xC117C8, i_c117c8),
    (0xC117CC, i_c117cc),
    (0xC117D0, i_c117d0),
    (0xC117D2, i_c117d2),
    (0xC117D6, i_c117d6),
    (0xC117DA, i_c117da),
    (0xC117DC, i_c117dc),
    (0xC117E0, i_c117e0),
    (0xC117E4, i_c117e4),
    (0xC117EA, i_c117ea),
    (0xC117EC, i_c117ec),
    (0xC117EE, i_c117ee),
    (0xC117F2, i_c117f2),
    (0xC117F4, i_c117f4),
    (0xC117F8, i_c117f8),
    (0xC117FC, i_c117fc),
    (0xC117FE, i_c117fe),
    (0xC11802, i_c11802),
    (0xC11806, i_c11806),
    (0xC1180C, i_c1180c),
    (0xC1180E, i_c1180e),
    (0xC11812, i_c11812),
    (0xC11816, i_c11816),
    (0xC11818, i_c11818),
    (0xC1181E, i_c1181e),
    (0xC11822, i_c11822),
    (0xC11828, i_c11828),
    (0xC1182A, i_c1182a),
    (0xC1182E, i_c1182e),
    (0xC11832, i_c11832),
    (0xC11838, i_c11838),
    (0xC1183C, i_c1183c),
    (0xC11842, i_c11842),
    (0xC11846, i_c11846),
    (0xC1184A, i_c1184a),
    (0xC1184E, i_c1184e),
    (0xC11854, i_c11854),
    (0xC11858, i_c11858),
    (0xC1185C, i_c1185c),
    (0xC11860, i_c11860),
    (0xC11866, i_c11866),
    (0xC1186A, i_c1186a),
    (0xC1186E, i_c1186e),
    (0xC11870, i_c11870),
    (0xC11872, i_c11872),
    (0xC11874, i_c11874),
    (0xC11876, i_c11876),
    (0xC1187A, i_c1187a),
    (0xC1187E, i_c1187e),
    (0xC11882, i_c11882),
    (0xC11884, i_c11884),
    (0xC11888, i_c11888),
    (0xC1188C, i_c1188c),
    (0xC11890, i_c11890),
    (0xC11892, i_c11892),
    (0xC11894, i_c11894),
    (0xC11898, i_c11898),
    (0xC1189A, i_c1189a),
    (0xC1189E, i_c1189e),
    (0xC118A2, i_c118a2),
    (0xC118A4, i_c118a4),
    (0xC118A6, i_c118a6),
    (0xC118AA, i_c118aa),
    (0xC118AE, i_c118ae),
    (0xC118B0, i_c118b0),
    (0xC118B6, i_c118b6),
    (0xC118B8, i_c118b8),
    (0xC118BA, i_c118ba),
    (0xC118BE, i_c118be),
    (0xC118C0, i_c118c0),
    (0xC118C4, i_c118c4),
    (0xC118C8, i_c118c8),
    (0xC118CA, i_c118ca),
    (0xC118CE, i_c118ce),
    (0xC118D0, i_c118d0),
    (0xC118D2, i_c118d2),
    (0xC118D6, i_c118d6),
    (0xC118DA, i_c118da),
    (0xC118DC, i_c118dc),
    (0xC118E2, i_c118e2),
    (0xC118E8, i_c118e8),
    (0xC118EC, i_c118ec),
    (0xC118F0, i_c118f0),
    (0xC118F4, i_c118f4),
    (0xC118F8, i_c118f8),
    (0xC118FC, i_c118fc),
    (0xC118FE, i_c118fe),
    (0xC11900, i_c11900),
    (0xC11902, i_c11902),
    (0xC11904, i_c11904),
    (0xC11906, i_c11906),
    (0xC1190C, i_c1190c),
    (0xC11910, i_c11910),
    (0xC11912, i_c11912),
    (0xC11916, i_c11916),
    (0xC11918, i_c11918),
    (0xC1191A, i_c1191a),
    (0xC1191C, i_c1191c),
    (0xC1191E, i_c1191e),
    (0xC11922, i_c11922),
    (0xC11926, i_c11926),
    (0xC1192A, i_c1192a),
    (0xC1192E, i_c1192e),
    (0xC11932, i_c11932),
    (0xC11936, i_c11936),
    (0xC1193A, i_c1193a),
    (0xC1193E, i_c1193e),
    (0xC11942, i_c11942),
    (0xC11946, i_c11946),
    (0xC1194A, i_c1194a),
    (0xC1194E, i_c1194e),
    (0xC11952, i_c11952),
    (0xC11956, i_c11956),
    (0xC11958, i_c11958),
    (0xC1195C, i_c1195c),
    (0xC11960, i_c11960),
    (0xC11962, i_c11962),
    (0xC11964, i_c11964),
    (0xC1196A, i_c1196a),
    (0xC11970, i_c11970),
    (0xC11974, i_c11974),
    (0xC11978, i_c11978),
    (0xC1197C, i_c1197c),
    (0xC11980, i_c11980),
    (0xC11984, i_c11984),
    (0xC11986, i_c11986),
    (0xC11988, i_c11988),
    (0xC1198A, i_c1198a),
    (0xC1198C, i_c1198c),
    (0xC11990, i_c11990),
    (0xC11994, i_c11994),
    (0xC11996, i_c11996),
    (0xC11998, i_c11998),
    (0xC1199C, i_c1199c),
    (0xC1199E, i_c1199e),
    (0xC119A2, i_c119a2),
    (0xC119A6, i_c119a6),
    (0xC119AC, i_c119ac),
    (0xC119B0, i_c119b0),
    (0xC119B4, i_c119b4),
    (0xC119B6, i_c119b6),
    (0xC119BA, i_c119ba),
    (0xC119BC, i_c119bc),
    (0xC119C0, i_c119c0),
    (0xC119C4, i_c119c4),
    (0xC119C6, i_c119c6),
    (0xC119CE, i_c119ce),
    (0xC119D0, i_c119d0),
    (0xC119D4, i_c119d4),
    (0xC119DA, i_c119da),
    (0xC119E0, i_c119e0),
    (0xC119E2, i_c119e2),
    (0xC119E8, i_c119e8),
    (0xC119EA, i_c119ea),
    (0xC119F0, i_c119f0),
    (0xC119F2, i_c119f2),
    (0xC119F8, i_c119f8),
    (0xC119FC, i_c119fc),
    (0xC119FE, i_c119fe),
    (0xC11A00, i_c11a00),
    (0xC11A06, i_c11a06),
    (0xC11A08, i_c11a08),
    (0xC11A0E, i_c11a0e),
    (0xC11A14, i_c11a14),
    (0xC11A18, i_c11a18),
    (0xC11A1C, i_c11a1c),
    (0xC11A1E, i_c11a1e),
    (0xC11A20, i_c11a20),
    (0xC11A22, i_c11a22),
    (0xC11A24, i_c11a24),
    (0xC11A26, i_c11a26),
    (0xC11A2A, i_c11a2a),
    (0xC11A2E, i_c11a2e),
    (0xC11A32, i_c11a32),
    (0xC11A36, i_c11a36),
    (0xC11A38, i_c11a38),
    (0xC11A3C, i_c11a3c),
    (0xC11A40, i_c11a40),
    (0xC11A42, i_c11a42),
    (0xC11A46, i_c11a46),
    (0xC11A4A, i_c11a4a),
    (0xC11A4E, i_c11a4e),
    (0xC11A50, i_c11a50),
    (0xC11A54, i_c11a54),
    (0xC11A58, i_c11a58),
    (0xC11A5C, i_c11a5c),
    (0xC11A5E, i_c11a5e),
    (0xC11A62, i_c11a62),
    (0xC11A64, i_c11a64),
    (0xC11A6A, i_c11a6a),
    (0xC11A70, i_c11a70),
    (0xC11A76, i_c11a76),
    (0xC11A7C, i_c11a7c),
    (0xC11A82, i_c11a82),
    (0xC11A86, i_c11a86),
    (0xC11A8C, i_c11a8c),
    (0xC11A8E, i_c11a8e),
    (0xC11A90, i_c11a90),
    (0xC11A94, i_c11a94),
    (0xC11A98, i_c11a98),
    (0xC11A9A, i_c11a9a),
    (0xC11A9E, i_c11a9e),
    (0xC11AA4, i_c11aa4),
    (0xC11AAA, i_c11aaa),
    (0xC11AB0, i_c11ab0),
    (0xC11AB6, i_c11ab6),
    (0xC11ABA, i_c11aba),
    (0xC11ABC, i_c11abc),
    (0xC11AC0, i_c11ac0),
    (0xC11AC2, i_c11ac2),
    (0xC11AC8, i_c11ac8),
    (0xC11ACC, i_c11acc),
    (0xC11ACE, i_c11ace),
    (0xC11AD2, i_c11ad2),
    (0xC11AD6, i_c11ad6),
    (0xC11ADA, i_c11ada),
    (0xC11ADE, i_c11ade),
    (0xC11AE4, i_c11ae4),
    (0xC11AE8, i_c11ae8),
    (0xC11AEC, i_c11aec),
    (0xC11AF2, i_c11af2),
    (0xC11AF6, i_c11af6),
    (0xC11AFA, i_c11afa),
    (0xC11B00, i_c11b00),
    (0xC11B04, i_c11b04),
    (0xC11B08, i_c11b08),
    (0xC11B0E, i_c11b0e),
    (0xC11B14, i_c11b14),
    (0xC11B1E, i_c11b1e),
    (0xC11B22, i_c11b22),
    (0xC11B26, i_c11b26),
    (0xC11B2A, i_c11b2a),
    (0xC11B2E, i_c11b2e),
    (0xC11B30, i_c11b30),
    (0xC11B34, i_c11b34),
    (0xC11B36, i_c11b36),
    (0xC11B3A, i_c11b3a),
    (0xC11B3E, i_c11b3e),
    (0xC11B42, i_c11b42),
    (0xC11B48, i_c11b48),
    (0xC11B4E, i_c11b4e),
    (0xC11B50, i_c11b50),
    (0xC11B52, i_c11b52),
    (0xC11B58, i_c11b58),
    (0xC11B5A, i_c11b5a),
    (0xC11B5C, i_c11b5c),
    (0xC11B5E, i_c11b5e),
    (0xC11B60, i_c11b60),
    (0xC11B62, i_c11b62),
    (0xC11B64, i_c11b64),
    (0xC11B66, i_c11b66),
    (0xC11B68, i_c11b68),
    (0xC11B6A, i_c11b6a),
    (0xC11B6E, i_c11b6e),
    (0xC11B72, i_c11b72),
    (0xC11B74, i_c11b74),
    (0xC11BB0, i_c11bb0),
    (0xC11BB6, i_c11bb6),
    (0xC11BBC, i_c11bbc),
    (0xC11BC2, i_c11bc2),
    (0xC11BC8, i_c11bc8),
    (0xC11BCA, i_c11bca),
    (0xC11BD0, i_c11bd0),
    (0xC11BD2, i_c11bd2),
    (0xC11BD6, i_c11bd6),
    (0xC11BD8, i_c11bd8),
    (0xC11BDE, i_c11bde),
    (0xC11BE2, i_c11be2),
    (0xC11BE8, i_c11be8),
    (0xC11BEC, i_c11bec),
    (0xC11BEE, i_c11bee),
    (0xC11BF4, i_c11bf4),
    (0xC11BFA, i_c11bfa),
    (0xC11C00, i_c11c00),
    (0xC11C04, i_c11c04),
    (0xC11C08, i_c11c08),
    (0xC11C0C, i_c11c0c),
    (0xC11C10, i_c11c10),
    (0xC11C14, i_c11c14),
    (0xC11C18, i_c11c18),
    (0xC11C1C, i_c11c1c),
    (0xC11C20, i_c11c20),
    (0xC11C24, i_c11c24),
    (0xC11C28, i_c11c28),
    (0xC11C2C, i_c11c2c),
    (0xC11C30, i_c11c30),
    (0xC11C36, i_c11c36),
    (0xC11C3A, i_c11c3a),
    (0xC11C40, i_c11c40),
    (0xC11C42, i_c11c42),
    (0xC11C44, i_c11c44),
    (0xC11C48, i_c11c48),
    (0xC11C4C, i_c11c4c),
    (0xC11C4E, i_c11c4e),
    (0xC11C52, i_c11c52),
    (0xC11C56, i_c11c56),
    (0xC11C5C, i_c11c5c),
    (0xC11C60, i_c11c60),
    (0xC11C64, i_c11c64),
    (0xC11C68, i_c11c68),
    (0xC11C6C, i_c11c6c),
    (0xC11C70, i_c11c70),
    (0xC11C74, i_c11c74),
    (0xC11C78, i_c11c78),
    (0xC11C7C, i_c11c7c),
    (0xC11C80, i_c11c80),
    (0xC11C82, i_c11c82),
    (0xC11C86, i_c11c86),
    (0xC11C8A, i_c11c8a),
    (0xC11C8C, i_c11c8c),
    (0xC11C90, i_c11c90),
    (0xC11C92, i_c11c92),
    (0xC11C94, i_c11c94),
    (0xC11C98, i_c11c98),
    (0xC11C9C, i_c11c9c),
    (0xC11C9E, i_c11c9e),
    (0xC11CA0, i_c11ca0),
    (0xC11CA4, i_c11ca4),
    (0xC11CA6, i_c11ca6),
    (0xC11CA8, i_c11ca8),
    (0xC11CAE, i_c11cae),
    (0xC11CB2, i_c11cb2),
    (0xC11CB6, i_c11cb6),
    (0xC11CB8, i_c11cb8),
    (0xC11CBA, i_c11cba),
    (0xC11CBE, i_c11cbe),
    (0xC11CC0, i_c11cc0),
    (0xC11CC6, i_c11cc6),
    (0xC11CCA, i_c11cca),
    (0xC11CCC, i_c11ccc),
    (0xC11CD0, i_c11cd0),
    (0xC11CD2, i_c11cd2),
    (0xC11CD6, i_c11cd6),
    (0xC11CDA, i_c11cda),
    (0xC11CDE, i_c11cde),
    (0xC11CE2, i_c11ce2),
    (0xC11CE6, i_c11ce6),
    (0xC11CEA, i_c11cea),
    (0xC11CEE, i_c11cee),
    (0xC11CF2, i_c11cf2),
    (0xC11CF6, i_c11cf6),
    (0xC11CFA, i_c11cfa),
    (0xC11CFE, i_c11cfe),
    (0xC11D02, i_c11d02),
    (0xC11D06, i_c11d06),
    (0xC11D08, i_c11d08),
    (0xC11D0C, i_c11d0c),
    (0xC11D0E, i_c11d0e),
    (0xC11D12, i_c11d12),
    (0xC11D14, i_c11d14),
    (0xC11D16, i_c11d16),
    (0xC11D18, i_c11d18),
    (0xC11D1A, i_c11d1a),
    (0xC11D1E, i_c11d1e),
    (0xC11D22, i_c11d22),
    (0xC11D26, i_c11d26),
    (0xC11D2A, i_c11d2a),
    (0xC11D2E, i_c11d2e),
    (0xC11D32, i_c11d32),
    (0xC11D34, i_c11d34),
    (0xC11D38, i_c11d38),
    (0xC11D3A, i_c11d3a),
    (0xC11D3C, i_c11d3c),
    (0xC11D3E, i_c11d3e),
    (0xC11D42, i_c11d42),
    (0xC11D46, i_c11d46),
    (0xC11D4A, i_c11d4a),
    (0xC11D4E, i_c11d4e),
    (0xC11D52, i_c11d52),
    (0xC11D56, i_c11d56),
    (0xC11D58, i_c11d58),
    (0xC11D5C, i_c11d5c),
    (0xC11D60, i_c11d60),
    (0xC11D64, i_c11d64),
    (0xC11D66, i_c11d66),
    (0xC11D6A, i_c11d6a),
    (0xC11D6C, i_c11d6c),
    (0xC11D70, i_c11d70),
    (0xC11D74, i_c11d74),
    (0xC11D78, i_c11d78),
    (0xC11D7C, i_c11d7c),
    (0xC11D7E, i_c11d7e),
    (0xC11D84, i_c11d84),
    (0xC11D86, i_c11d86),
    (0xC11D88, i_c11d88),
    (0xC11D8C, i_c11d8c),
    (0xC11D90, i_c11d90),
    (0xC11D92, i_c11d92),
    (0xC11D96, i_c11d96),
    (0xC11D9A, i_c11d9a),
    (0xC11DA0, i_c11da0),
    (0xC11DA4, i_c11da4),
    (0xC11DA8, i_c11da8),
    (0xC11DAC, i_c11dac),
    (0xC11DB0, i_c11db0),
    (0xC11DB4, i_c11db4),
    (0xC11DB8, i_c11db8),
    (0xC11DBC, i_c11dbc),
    (0xC11DC0, i_c11dc0),
    (0xC11DC4, i_c11dc4),
    (0xC11DC6, i_c11dc6),
    (0xC11DCA, i_c11dca),
    (0xC11DCE, i_c11dce),
    (0xC11DD0, i_c11dd0),
    (0xC11DD4, i_c11dd4),
    (0xC11DD6, i_c11dd6),
    (0xC11DD8, i_c11dd8),
    (0xC11DDC, i_c11ddc),
    (0xC11DE0, i_c11de0),
    (0xC11DE2, i_c11de2),
    (0xC11DE4, i_c11de4),
    (0xC11DE8, i_c11de8),
    (0xC11DEA, i_c11dea),
    (0xC11DEC, i_c11dec),
    (0xC11DF2, i_c11df2),
    (0xC11DF6, i_c11df6),
    (0xC11DFA, i_c11dfa),
    (0xC11DFC, i_c11dfc),
    (0xC11DFE, i_c11dfe),
    (0xC11E02, i_c11e02),
    (0xC11E04, i_c11e04),
    (0xC11E0A, i_c11e0a),
    (0xC11E0E, i_c11e0e),
    (0xC11E10, i_c11e10),
    (0xC11E14, i_c11e14),
    (0xC11E16, i_c11e16),
    (0xC11E1A, i_c11e1a),
    (0xC11E1E, i_c11e1e),
    (0xC11E22, i_c11e22),
    (0xC11E26, i_c11e26),
    (0xC11E2A, i_c11e2a),
    (0xC11E2E, i_c11e2e),
    (0xC11E32, i_c11e32),
    (0xC11E36, i_c11e36),
    (0xC11E3A, i_c11e3a),
    (0xC11E3E, i_c11e3e),
    (0xC11E42, i_c11e42),
    (0xC11E46, i_c11e46),
    (0xC11E4A, i_c11e4a),
    (0xC11E4C, i_c11e4c),
    (0xC11E50, i_c11e50),
    (0xC11E52, i_c11e52),
    (0xC11E56, i_c11e56),
    (0xC11E58, i_c11e58),
    (0xC11E5A, i_c11e5a),
    (0xC11E5C, i_c11e5c),
    (0xC11E5E, i_c11e5e),
    (0xC11E62, i_c11e62),
    (0xC11E66, i_c11e66),
    (0xC11E6A, i_c11e6a),
    (0xC11E6E, i_c11e6e),
    (0xC11E72, i_c11e72),
    (0xC11E76, i_c11e76),
    (0xC11E78, i_c11e78),
    (0xC11E7C, i_c11e7c),
    (0xC11E7E, i_c11e7e),
    (0xC11E80, i_c11e80),
    (0xC11E82, i_c11e82),
    (0xC11E86, i_c11e86),
    (0xC11E8A, i_c11e8a),
    (0xC11E8E, i_c11e8e),
    (0xC11E92, i_c11e92),
    (0xC11E96, i_c11e96),
    (0xC11E9A, i_c11e9a),
    (0xC11E9C, i_c11e9c),
    (0xC11EA0, i_c11ea0),
    (0xC11EA4, i_c11ea4),
    (0xC11EA8, i_c11ea8),
    (0xC11EAA, i_c11eaa),
    (0xC11EAE, i_c11eae),
    (0xC11EB0, i_c11eb0),
    (0xC11EB4, i_c11eb4),
    (0xC11EB8, i_c11eb8),
    (0xC11EBC, i_c11ebc),
    (0xC11EC0, i_c11ec0),
    (0xC11EC2, i_c11ec2),
    (0xC11EC8, i_c11ec8),
    (0xC11ECA, i_c11eca),
    (0xC11ECE, i_c11ece),
    (0xC11ED2, i_c11ed2),
    (0xC11ED6, i_c11ed6),
    (0xC11EDA, i_c11eda),
    (0xC11EDC, i_c11edc),
    (0xC11EE0, i_c11ee0),
    (0xC11EE4, i_c11ee4),
    (0xC11EE6, i_c11ee6),
    (0xC11EE8, i_c11ee8),
    (0xC11EEA, i_c11eea),
    (0xC11EEE, i_c11eee),
    (0xC11EF2, i_c11ef2),
    (0xC11EF4, i_c11ef4),
    (0xC11EF6, i_c11ef6),
    (0xC11EFA, i_c11efa),
    (0xC11EFC, i_c11efc),
    (0xC11EFE, i_c11efe),
    (0xC11F02, i_c11f02),
    (0xC11F06, i_c11f06),
    (0xC11F0A, i_c11f0a),
    (0xC11F0E, i_c11f0e),
    (0xC11F12, i_c11f12),
    (0xC11F14, i_c11f14),
    (0xC11F16, i_c11f16),
    (0xC11F18, i_c11f18),
    (0xC11F1C, i_c11f1c),
    (0xC11F1E, i_c11f1e),
    (0xC11F20, i_c11f20),
    (0xC11F22, i_c11f22),
    (0xC11F24, i_c11f24),
    (0xC11F28, i_c11f28),
    (0xC11F2C, i_c11f2c),
    (0xC11F2E, i_c11f2e),
    (0xC11F30, i_c11f30),
    (0xC11F32, i_c11f32),
    (0xC11F34, i_c11f34),
    (0xC11F36, i_c11f36),
    (0xC11F38, i_c11f38),
    (0xC11F3A, i_c11f3a),
    (0xC11F3C, i_c11f3c),
    (0xC11F3E, i_c11f3e),
    (0xC11F42, i_c11f42),
    (0xC11F46, i_c11f46),
    (0xC11F48, i_c11f48),
    (0xC11F4C, i_c11f4c),
    (0xC11F4E, i_c11f4e),
    (0xC11F52, i_c11f52),
    (0xC11F54, i_c11f54),
    (0xC11F56, i_c11f56),
    (0xC11F58, i_c11f58),
    (0xC11F5A, i_c11f5a),
    (0xC11F5E, i_c11f5e),
    (0xC11F62, i_c11f62),
    (0xC11F66, i_c11f66),
    (0xC11F6A, i_c11f6a),
    (0xC11F6C, i_c11f6c),
    (0xC11F6E, i_c11f6e),
    (0xC11F70, i_c11f70),
    (0xC11F74, i_c11f74),
    (0xC11F78, i_c11f78),
    (0xC11F7C, i_c11f7c),
    (0xC11F80, i_c11f80),
    (0xC11F82, i_c11f82),
    (0xC11F86, i_c11f86),
    (0xC11F88, i_c11f88),
    (0xC11F8A, i_c11f8a),
    (0xC11F8C, i_c11f8c),
    (0xC11F90, i_c11f90),
    (0xC11F94, i_c11f94),
    (0xC11F98, i_c11f98),
    (0xC11F9C, i_c11f9c),
    (0xC11F9E, i_c11f9e),
    (0xC11FA2, i_c11fa2),
    (0xC11FA4, i_c11fa4),
    (0xC11FA6, i_c11fa6),
    (0xC11FA8, i_c11fa8),
    (0xC11FAC, i_c11fac),
    (0xC11FB0, i_c11fb0),
    (0xC11FB4, i_c11fb4),
    (0xC11FB6, i_c11fb6),
    (0xC11FB8, i_c11fb8),
    (0xC11FBC, i_c11fbc),
    (0xC11FC0, i_c11fc0),
    (0xC11FC2, i_c11fc2),
    (0xC11FC4, i_c11fc4),
    (0xC11FC6, i_c11fc6),
    (0xC11FC8, i_c11fc8),
    (0xC11FCA, i_c11fca),
    (0xC11FCC, i_c11fcc),
    (0xC11FD0, i_c11fd0),
    (0xC11FD4, i_c11fd4),
    (0xC11FD6, i_c11fd6),
    (0xC11FD8, i_c11fd8),
    (0xC11FDA, i_c11fda),
    (0xC11FDE, i_c11fde),
    (0xC11FE2, i_c11fe2),
    (0xC11FE4, i_c11fe4),
    (0xC11FE8, i_c11fe8),
    (0xC11FEA, i_c11fea),
    (0xC11FEE, i_c11fee),
    (0xC11FF2, i_c11ff2),
    (0xC11FF4, i_c11ff4),
    (0xC11FF6, i_c11ff6),
    (0xC11FF8, i_c11ff8),
    (0xC11FFA, i_c11ffa),
    (0xC11FFC, i_c11ffc),
    (0xC11FFE, i_c11ffe),
    (0xC12000, i_c12000),
    (0xC12002, i_c12002),
    (0xC12004, i_c12004),
    (0xC12006, i_c12006),
    (0xC1200A, i_c1200a),
    (0xC1200E, i_c1200e),
    (0xC12010, i_c12010),
    (0xC12012, i_c12012),
    (0xC12014, i_c12014),
    (0xC12018, i_c12018),
    (0xC1201A, i_c1201a),
    (0xC12024, i_c12024),
    (0xC1202A, i_c1202a),
    (0xC1202E, i_c1202e),
    (0xC12032, i_c12032),
    (0xC12036, i_c12036),
    (0xC1203A, i_c1203a),
    (0xC1203E, i_c1203e),
    (0xC12040, i_c12040),
    (0xC12042, i_c12042),
    (0xC12044, i_c12044),
    (0xC12048, i_c12048),
    (0xC12050, i_c12050),
    (0xC12056, i_c12056),
    (0xC1205C, i_c1205c),
    (0xC1205E, i_c1205e),
    (0xC12064, i_c12064),
    (0xC12068, i_c12068),
    (0xC1206A, i_c1206a),
    (0xC1206C, i_c1206c),
    (0xC1206E, i_c1206e),
    (0xC12070, i_c12070),
    (0xC12072, i_c12072),
    (0xC12074, i_c12074),
    (0xC12076, i_c12076),
    (0xC12078, i_c12078),
    (0xC1207A, i_c1207a),
    (0xC1207C, i_c1207c),
    (0xC12080, i_c12080),
    (0xC12084, i_c12084),
    (0xC12086, i_c12086),
    (0xC12088, i_c12088),
    (0xC1208A, i_c1208a),
    (0xC1208E, i_c1208e),
    (0xC12092, i_c12092),
    (0xC12094, i_c12094),
    (0xC12096, i_c12096),
    (0xC12098, i_c12098),
    (0xC1209A, i_c1209a),
    (0xC1209C, i_c1209c),
    (0xC1209E, i_c1209e),
    (0xC120A2, i_c120a2),
    (0xC120A4, i_c120a4),
    (0xC120A8, i_c120a8),
    (0xC120AC, i_c120ac),
    (0xC120B0, i_c120b0),
    (0xC120B2, i_c120b2),
    (0xC120B4, i_c120b4),
    (0xC120B6, i_c120b6),
    (0xC120BA, i_c120ba),
    (0xC120BE, i_c120be),
    (0xC120C0, i_c120c0),
    (0xC120C2, i_c120c2),
    (0xC120C4, i_c120c4),
    (0xC120C6, i_c120c6),
    (0xC120C8, i_c120c8),
    (0xC120CA, i_c120ca),
    (0xC120CE, i_c120ce),
    (0xC120D0, i_c120d0),
    (0xC120D4, i_c120d4),
    (0xC120D8, i_c120d8),
    (0xC120DA, i_c120da),
    (0xC120DC, i_c120dc),
    (0xC120DE, i_c120de),
    (0xC120E0, i_c120e0),
    (0xC120E2, i_c120e2),
    (0xC120E6, i_c120e6),
    (0xC120EA, i_c120ea),
    (0xC120EC, i_c120ec),
    (0xC120EE, i_c120ee),
    (0xC120F0, i_c120f0),
    (0xC120F2, i_c120f2),
    (0xC120F4, i_c120f4),
    (0xC120F6, i_c120f6),
    (0xC120F8, i_c120f8),
    (0xC120FC, i_c120fc),
    (0xC120FE, i_c120fe),
    (0xC12100, i_c12100),
    (0xC12104, i_c12104),
    (0xC12106, i_c12106),
    (0xC1210C, i_c1210c),
    (0xC1210E, i_c1210e),
    (0xC12110, i_c12110),
    (0xC12114, i_c12114),
    (0xC12116, i_c12116),
    (0xC12118, i_c12118),
    (0xC1211A, i_c1211a),
    (0xC1211E, i_c1211e),
    (0xC12120, i_c12120),
    (0xC12122, i_c12122),
    (0xC12124, i_c12124),
    (0xC12126, i_c12126),
    (0xC12128, i_c12128),
    (0xC1212C, i_c1212c),
    (0xC1212E, i_c1212e),
    (0xC12132, i_c12132),
    (0xC12134, i_c12134),
    (0xC12138, i_c12138),
    (0xC1213A, i_c1213a),
    (0xC1213C, i_c1213c),
    (0xC1213E, i_c1213e),
    (0xC12142, i_c12142),
    (0xC12144, i_c12144),
    (0xC12146, i_c12146),
    (0xC12148, i_c12148),
    (0xC1214A, i_c1214a),
    (0xC1214C, i_c1214c),
    (0xC12150, i_c12150),
    (0xC12152, i_c12152),
    (0xC12156, i_c12156),
    (0xC12158, i_c12158),
    (0xC1215A, i_c1215a),
    (0xC1215C, i_c1215c),
    (0xC1215E, i_c1215e),
    (0xC12160, i_c12160),
    (0xC12164, i_c12164),
    (0xC12166, i_c12166),
    (0xC12168, i_c12168),
    (0xC1216A, i_c1216a),
    (0xC1216C, i_c1216c),
    (0xC1216E, i_c1216e),
    (0xC12170, i_c12170),
    (0xC12174, i_c12174),
    (0xC12176, i_c12176),
    (0xC1217A, i_c1217a),
    (0xC1217C, i_c1217c),
    (0xC12180, i_c12180),
    (0xC12184, i_c12184),
    (0xC12186, i_c12186),
    (0xC1218A, i_c1218a),
    (0xC1218E, i_c1218e),
    (0xC12192, i_c12192),
    (0xC12196, i_c12196),
    (0xC1219A, i_c1219a),
    (0xC1219E, i_c1219e),
    (0xC121A0, i_c121a0),
    (0xC121A4, i_c121a4),
    (0xC121A8, i_c121a8),
    (0xC121AA, i_c121aa),
    (0xC121AC, i_c121ac),
    (0xC121AE, i_c121ae),
    (0xC121B0, i_c121b0),
    (0xC121B4, i_c121b4),
    (0xC121B8, i_c121b8),
    (0xC121BA, i_c121ba),
    (0xC121BC, i_c121bc),
    (0xC121BE, i_c121be),
    (0xC121C0, i_c121c0),
    (0xC121C4, i_c121c4),
    (0xC121C8, i_c121c8),
    (0xC121CC, i_c121cc),
    (0xC121D0, i_c121d0),
    (0xC121D4, i_c121d4),
    (0xC121D8, i_c121d8),
    (0xC121DC, i_c121dc),
    (0xC121E2, i_c121e2),
    (0xC121E6, i_c121e6),
    (0xC121E8, i_c121e8),
    (0xC121EA, i_c121ea),
    (0xC121EC, i_c121ec),
    (0xC121EE, i_c121ee),
    (0xC121F0, i_c121f0),
    (0xC121F4, i_c121f4),
    (0xC1231E, i_c1231e),
    (0xC12324, i_c12324),
    (0xC12328, i_c12328),
    (0xC1232C, i_c1232c),
    (0xC12332, i_c12332),
    (0xC12338, i_c12338),
    (0xC1233C, i_c1233c),
    (0xC1233E, i_c1233e),
    (0xC12344, i_c12344),
    (0xC12346, i_c12346),
    (0xC1234C, i_c1234c),
    (0xC12352, i_c12352),
    (0xC12354, i_c12354),
    (0xC1235A, i_c1235a),
    (0xC12360, i_c12360),
    (0xC12364, i_c12364),
    (0xC12366, i_c12366),
    (0xC1236A, i_c1236a),
    (0xC1236C, i_c1236c),
    (0xC1236E, i_c1236e),
    (0xC12370, i_c12370),
    (0xC12372, i_c12372),
    (0xC12374, i_c12374),
    (0xC1238C, i_c1238c),
    (0xC12390, i_c12390),
    (0xC12392, i_c12392),
    (0xC12398, i_c12398),
    (0xC1239C, i_c1239c),
    (0xC1239E, i_c1239e),
    (0xC123A2, i_c123a2),
    (0xC123A6, i_c123a6),
    (0xC123AA, i_c123aa),
    (0xC123AC, i_c123ac),
    (0xC123B0, i_c123b0),
    (0xC123B2, i_c123b2),
    (0xC123B4, i_c123b4),
    (0xC123B6, i_c123b6),
    (0xC123BA, i_c123ba),
    (0xC123BE, i_c123be),
    (0xC123C0, i_c123c0),
    (0xC123C2, i_c123c2),
    (0xC123C4, i_c123c4),
    (0xC123C6, i_c123c6),
    (0xC123CA, i_c123ca),
    (0xC123CC, i_c123cc),
    (0xC123CE, i_c123ce),
    (0xC123DC, i_c123dc),
    (0xC123DE, i_c123de),
    (0xC123E2, i_c123e2),
    (0xC123E6, i_c123e6),
    (0xC123E8, i_c123e8),
    (0xC123EC, i_c123ec),
    (0xC123EE, i_c123ee),
    (0xC123F6, i_c123f6),
    (0xC123FA, i_c123fa),
    (0xC123FE, i_c123fe),
    (0xC12402, i_c12402),
    (0xC12408, i_c12408),
    (0xC1240E, i_c1240e),
    (0xC12412, i_c12412),
    (0xC12416, i_c12416),
    (0xC12418, i_c12418),
    (0xC1241A, i_c1241a),
    (0xC12420, i_c12420),
    (0xC12422, i_c12422),
    (0xC12424, i_c12424),
    (0xC12426, i_c12426),
    (0xC12428, i_c12428),
    (0xC1242A, i_c1242a),
    (0xC1242C, i_c1242c),
    (0xC12430, i_c12430),
    (0xC12434, i_c12434),
    (0xC1243A, i_c1243a),
    (0xC1243C, i_c1243c),
    (0xC12440, i_c12440),
    (0xC12444, i_c12444),
    (0xC12448, i_c12448),
    (0xC1244C, i_c1244c),
    (0xC12450, i_c12450),
    (0xC12454, i_c12454),
    (0xC12458, i_c12458),
    (0xC1245C, i_c1245c),
    (0xC1245E, i_c1245e),
    (0xC12460, i_c12460),
    (0xC12462, i_c12462),
    (0xC12464, i_c12464),
    (0xC12466, i_c12466),
    (0xC12468, i_c12468),
    (0xC1246A, i_c1246a),
    (0xC1246C, i_c1246c),
    (0xC1246E, i_c1246e),
    (0xC12470, i_c12470),
    (0xC12472, i_c12472),
    (0xC124B8, i_c124b8),
    (0xC124BC, i_c124bc),
    (0xC124C0, i_c124c0),
    (0xC124C2, i_c124c2),
    (0xC124C4, i_c124c4),
    (0xC124C6, i_c124c6),
    (0xC124C8, i_c124c8),
    (0xC124CA, i_c124ca),
    (0xC124CC, i_c124cc),
    (0xC124CE, i_c124ce),
    (0xC124D0, i_c124d0),
    (0xC124D2, i_c124d2),
    (0xC124D4, i_c124d4),
    (0xC124D6, i_c124d6),
    (0xC124DA, i_c124da),
    (0xC124DC, i_c124dc),
    (0xC124E0, i_c124e0),
    (0xC124E2, i_c124e2),
    (0xC124E8, i_c124e8),
    (0xC124EC, i_c124ec),
    (0xC124F2, i_c124f2),
    (0xC124F8, i_c124f8),
    (0xC124FC, i_c124fc),
    (0xC12500, i_c12500),
    (0xC12502, i_c12502),
    (0xC12506, i_c12506),
    (0xC12508, i_c12508),
    (0xC1250C, i_c1250c),
    (0xC1250E, i_c1250e),
    (0xC12510, i_c12510),
    (0xC12514, i_c12514),
    (0xC12518, i_c12518),
    (0xC1251C, i_c1251c),
    (0xC12520, i_c12520),
    (0xC12524, i_c12524),
    (0xC1252A, i_c1252a),
    (0xC1252E, i_c1252e),
    (0xC12532, i_c12532),
    (0xC12536, i_c12536),
    (0xC1253A, i_c1253a),
    (0xC1253C, i_c1253c),
    (0xC1253E, i_c1253e),
    (0xC12542, i_c12542),
    (0xC12544, i_c12544),
    (0xC1254A, i_c1254a),
    (0xC12550, i_c12550),
    (0xC12554, i_c12554),
    (0xC12558, i_c12558),
    (0xC1255A, i_c1255a),
    (0xC1255E, i_c1255e),
    (0xC12562, i_c12562),
    (0xC12568, i_c12568),
    (0xC1256C, i_c1256c),
    (0xC1256E, i_c1256e),
    (0xC12572, i_c12572),
    (0xC12576, i_c12576),
    (0xC12578, i_c12578),
    (0xC1257A, i_c1257a),
    (0xC1257C, i_c1257c),
    (0xC1257E, i_c1257e),
    (0xC12580, i_c12580),
    (0xC12582, i_c12582),
    (0xC12584, i_c12584),
    (0xC12586, i_c12586),
    (0xC12588, i_c12588),
    (0xC1258A, i_c1258a),
    (0xC1258C, i_c1258c),
    (0xC12590, i_c12590),
    (0xC12594, i_c12594),
    (0xC12598, i_c12598),
    (0xC1259A, i_c1259a),
    (0xC1259C, i_c1259c),
    (0xC1259E, i_c1259e),
    (0xC125A0, i_c125a0),
    (0xC125A2, i_c125a2),
    (0xC125A4, i_c125a4),
    (0xC125A6, i_c125a6),
    (0xC125A8, i_c125a8),
    (0xC125AA, i_c125aa),
    (0xC125AC, i_c125ac),
    (0xC12610, i_c12610),
    (0xC12616, i_c12616),
    (0xC1261C, i_c1261c),
    (0xC12622, i_c12622),
    (0xC12628, i_c12628),
    (0xC1262E, i_c1262e),
    (0xC12632, i_c12632),
    (0xC1263A, i_c1263a),
    (0xC12640, i_c12640),
    (0xC12644, i_c12644),
    (0xC12648, i_c12648),
    (0xC1264A, i_c1264a),
    (0xC12650, i_c12650),
    (0xC12654, i_c12654),
    (0xC12658, i_c12658),
    (0xC1265C, i_c1265c),
    (0xC12662, i_c12662),
    (0xC12666, i_c12666),
    (0xC1266A, i_c1266a),
    (0xC1266E, i_c1266e),
    (0xC12672, i_c12672),
    (0xC12678, i_c12678),
    (0xC1267C, i_c1267c),
    (0xC1267E, i_c1267e),
    (0xC12684, i_c12684),
    (0xC12688, i_c12688),
    (0xC1268E, i_c1268e),
    (0xC12694, i_c12694),
    (0xC1269C, i_c1269c),
    (0xC126A2, i_c126a2),
    (0xC126A6, i_c126a6),
    (0xC126AA, i_c126aa),
    (0xC126AE, i_c126ae),
    (0xC126B2, i_c126b2),
    (0xC126B6, i_c126b6),
    (0xC126B8, i_c126b8),
    (0xC126BC, i_c126bc),
    (0xC126C0, i_c126c0),
    (0xC126C2, i_c126c2),
    (0xC126C6, i_c126c6),
    (0xC126C8, i_c126c8),
    (0xC126CC, i_c126cc),
    (0xC126D0, i_c126d0),
    (0xC126D2, i_c126d2),
    (0xC126D4, i_c126d4),
    (0xC126D8, i_c126d8),
    (0xC126DE, i_c126de),
    (0xC126E2, i_c126e2),
    (0xC126E8, i_c126e8),
    (0xC126EC, i_c126ec),
    (0xC126F0, i_c126f0),
    (0xC126F2, i_c126f2),
    (0xC126F6, i_c126f6),
    (0xC126FA, i_c126fa),
    (0xC126FC, i_c126fc),
    (0xC12700, i_c12700),
    (0xC12704, i_c12704),
    (0xC12706, i_c12706),
    (0xC12708, i_c12708),
    (0xC1270C, i_c1270c),
    (0xC12710, i_c12710),
    (0xC12716, i_c12716),
    (0xC1271A, i_c1271a),
    (0xC1271E, i_c1271e),
    (0xC12722, i_c12722),
    (0xC12726, i_c12726),
    (0xC1272A, i_c1272a),
    (0xC1272C, i_c1272c),
    (0xC12730, i_c12730),
    (0xC12734, i_c12734),
    (0xC12738, i_c12738),
    (0xC1273C, i_c1273c),
    (0xC12740, i_c12740),
    (0xC12744, i_c12744),
    (0xC12746, i_c12746),
    (0xC1274A, i_c1274a),
    (0xC1274C, i_c1274c),
    (0xC1274E, i_c1274e),
    (0xC12752, i_c12752),
    (0xC12754, i_c12754),
    (0xC12756, i_c12756),
    (0xC12758, i_c12758),
    (0xC1275C, i_c1275c),
    (0xC12760, i_c12760),
    (0xC12762, i_c12762),
    (0xC12766, i_c12766),
    (0xC1276A, i_c1276a),
    (0xC1276E, i_c1276e),
    (0xC12770, i_c12770),
    (0xC12772, i_c12772),
    (0xC12776, i_c12776),
    (0xC1277C, i_c1277c),
    (0xC1277E, i_c1277e),
    (0xC12782, i_c12782),
    (0xC12784, i_c12784),
    (0xC12786, i_c12786),
    (0xC1278A, i_c1278a),
    (0xC1278E, i_c1278e),
    (0xC12792, i_c12792),
    (0xC12798, i_c12798),
    (0xC1279A, i_c1279a),
    (0xC127A0, i_c127a0),
    (0xC127A8, i_c127a8),
    (0xC127AE, i_c127ae),
    (0xC127B2, i_c127b2),
    (0xC127B8, i_c127b8),
    (0xC127BC, i_c127bc),
    (0xC127C0, i_c127c0),
    (0xC127C4, i_c127c4),
    (0xC127CA, i_c127ca),
    (0xC127D0, i_c127d0),
    (0xC127D4, i_c127d4),
    (0xC127D6, i_c127d6),
    (0xC127DC, i_c127dc),
    (0xC127E2, i_c127e2),
    (0xC127E8, i_c127e8),
    (0xC127EE, i_c127ee),
    (0xC127F4, i_c127f4),
    (0xC127F6, i_c127f6),
    (0xC127FA, i_c127fa),
    (0xC127FE, i_c127fe),
    (0xC12800, i_c12800),
    (0xC12802, i_c12802),
    (0xC12806, i_c12806),
    (0xC1280A, i_c1280a),
    (0xC1280C, i_c1280c),
    (0xC12812, i_c12812),
    (0xC12816, i_c12816),
    (0xC1281A, i_c1281a),
    (0xC1281C, i_c1281c),
    (0xC12820, i_c12820),
    (0xC12824, i_c12824),
    (0xC12826, i_c12826),
    (0xC1282A, i_c1282a),
    (0xC1282E, i_c1282e),
    (0xC12832, i_c12832),
    (0xC12834, i_c12834),
    (0xC12838, i_c12838),
    (0xC1283A, i_c1283a),
    (0xC1283E, i_c1283e),
    (0xC12840, i_c12840),
    (0xC12844, i_c12844),
    (0xC12846, i_c12846),
    (0xC12848, i_c12848),
    (0xC1284C, i_c1284c),
    (0xC12850, i_c12850),
    (0xC12852, i_c12852),
    (0xC12854, i_c12854),
    (0xC12856, i_c12856),
    (0xC12858, i_c12858),
    (0xC1285A, i_c1285a),
    (0xC1285C, i_c1285c),
    (0xC1285E, i_c1285e),
    (0xC12860, i_c12860),
    (0xC12862, i_c12862),
    (0xC12864, i_c12864),
    (0xC12866, i_c12866),
    (0xC12868, i_c12868),
    (0xC1286A, i_c1286a),
    (0xC1286C, i_c1286c),
    (0xC128DE, i_c128de),
    (0xC128E2, i_c128e2),
    (0xC128E6, i_c128e6),
    (0xC128EA, i_c128ea),
    (0xC128EE, i_c128ee),
    (0xC128F0, i_c128f0),
    (0xC128F2, i_c128f2),
    (0xC128F4, i_c128f4),
    (0xC128F6, i_c128f6),
    (0xC128FC, i_c128fc),
    (0xC12900, i_c12900),
    (0xC12904, i_c12904),
    (0xC12908, i_c12908),
    (0xC1290C, i_c1290c),
    (0xC1290E, i_c1290e),
    (0xC12910, i_c12910),
    (0xC12916, i_c12916),
    (0xC1291A, i_c1291a),
    (0xC12920, i_c12920),
    (0xC12924, i_c12924),
    (0xC12928, i_c12928),
    (0xC1292A, i_c1292a),
    (0xC1292E, i_c1292e),
    (0xC12932, i_c12932),
    (0xC12938, i_c12938),
    (0xC1293A, i_c1293a),
    (0xC1293E, i_c1293e),
    (0xC12942, i_c12942),
    (0xC12948, i_c12948),
    (0xC1294A, i_c1294a),
    (0xC1294E, i_c1294e),
    (0xC12950, i_c12950),
    (0xC12952, i_c12952),
    (0xC12954, i_c12954),
    (0xC12956, i_c12956),
    (0xC12958, i_c12958),
    (0xC1295C, i_c1295c),
    (0xC1295E, i_c1295e),
    (0xC12962, i_c12962),
    (0xC12968, i_c12968),
    (0xC1296E, i_c1296e),
    (0xC12974, i_c12974),
    (0xC12976, i_c12976),
    (0xC1297C, i_c1297c),
    (0xC12980, i_c12980),
    (0xC12986, i_c12986),
    (0xC12988, i_c12988),
    (0xC1298A, i_c1298a),
    (0xC1298C, i_c1298c),
    (0xC12990, i_c12990),
    (0xC12994, i_c12994),
    (0xC1299C, i_c1299c),
    (0xC129A4, i_c129a4),
    (0xC129AE, i_c129ae),
    (0xC129B8, i_c129b8),
    (0xC129BC, i_c129bc),
    (0xC129BE, i_c129be),
    (0xC129C2, i_c129c2),
    (0xC129C4, i_c129c4),
    (0xC129C8, i_c129c8),
    (0xC129D0, i_c129d0),
    (0xC129D6, i_c129d6),
    (0xC129DC, i_c129dc),
    (0xC129DE, i_c129de),
    (0xC129E4, i_c129e4),
    (0xC129E8, i_c129e8),
    (0xC129EE, i_c129ee),
    (0xC129F0, i_c129f0),
    (0xC129F4, i_c129f4),
    (0xC129FC, i_c129fc),
    (0xC12A04, i_c12a04),
    (0xC12A08, i_c12a08),
    (0xC12A0C, i_c12a0c),
    (0xC12A0E, i_c12a0e),
    (0xC12A12, i_c12a12),
    (0xC12A1A, i_c12a1a),
    (0xC12A20, i_c12a20),
    (0xC12A26, i_c12a26),
    (0xC12A2A, i_c12a2a),
    (0xC12A30, i_c12a30),
    (0xC12A34, i_c12a34),
    (0xC12A3A, i_c12a3a),
    (0xC12A3C, i_c12a3c),
    (0xC12A40, i_c12a40),
    (0xC12A44, i_c12a44),
    (0xC12A46, i_c12a46),
    (0xC12A4C, i_c12a4c),
    (0xC12A50, i_c12a50),
    (0xC12A52, i_c12a52),
    (0xC12A58, i_c12a58),
    (0xC12A5A, i_c12a5a),
    (0xC12A5E, i_c12a5e),
    (0xC12A62, i_c12a62),
    (0xC12A64, i_c12a64),
    (0xC12A68, i_c12a68),
    (0xC12A6C, i_c12a6c),
    (0xC12A70, i_c12a70),
    (0xC12A74, i_c12a74),
    (0xC12A76, i_c12a76),
    (0xC12A7A, i_c12a7a),
    (0xC12A7C, i_c12a7c),
    (0xC12A7E, i_c12a7e),
    (0xC12A80, i_c12a80),
    (0xC12A82, i_c12a82),
    (0xC12A86, i_c12a86),
    (0xC12A8A, i_c12a8a),
    (0xC12A8C, i_c12a8c),
    (0xC12A8E, i_c12a8e),
    (0xC12A92, i_c12a92),
    (0xC12A96, i_c12a96),
    (0xC12A98, i_c12a98),
    (0xC12A9A, i_c12a9a),
    (0xC12A9C, i_c12a9c),
    (0xC12AA0, i_c12aa0),
    (0xC12AA2, i_c12aa2),
    (0xC12AA4, i_c12aa4),
    (0xC12AA8, i_c12aa8),
    (0xC12AAC, i_c12aac),
    (0xC12AAE, i_c12aae),
    (0xC12AB0, i_c12ab0),
    (0xC12AB2, i_c12ab2),
    (0xC12AB4, i_c12ab4),
    (0xC12AB8, i_c12ab8),
    (0xC12ABE, i_c12abe),
    (0xC12AC0, i_c12ac0),
    (0xC12AC4, i_c12ac4),
    (0xC12AC8, i_c12ac8),
    (0xC12ACC, i_c12acc),
    (0xC12AD0, i_c12ad0),
    (0xC12AD4, i_c12ad4),
    (0xC12AD6, i_c12ad6),
    (0xC12AD8, i_c12ad8),
    (0xC12ADC, i_c12adc),
    (0xC12AE2, i_c12ae2),
    (0xC12AE6, i_c12ae6),
    (0xC12AE8, i_c12ae8),
    (0xC12AEA, i_c12aea),
    (0xC12AEC, i_c12aec),
    (0xC12AF0, i_c12af0),
    (0xC12AF4, i_c12af4),
    (0xC12AF6, i_c12af6),
    (0xC12AFA, i_c12afa),
    (0xC12AFE, i_c12afe),
    (0xC12B00, i_c12b00),
    (0xC12B04, i_c12b04),
    (0xC12B08, i_c12b08),
    (0xC12B0A, i_c12b0a),
    (0xC12B0E, i_c12b0e),
    (0xC12B10, i_c12b10),
    (0xC12B12, i_c12b12),
    (0xC12B14, i_c12b14),
    (0xC12B18, i_c12b18),
    (0xC12B1A, i_c12b1a),
    (0xC12B1E, i_c12b1e),
    (0xC12B20, i_c12b20),
    (0xC12B22, i_c12b22),
    (0xC12B26, i_c12b26),
    (0xC12B28, i_c12b28),
    (0xC12B2C, i_c12b2c),
    (0xC12B2E, i_c12b2e),
    (0xC12B30, i_c12b30),
    (0xC12B32, i_c12b32),
    (0xC12B36, i_c12b36),
    (0xC12B38, i_c12b38),
    (0xC12B3C, i_c12b3c),
    (0xC12B40, i_c12b40),
    (0xC12B42, i_c12b42),
    (0xC12B44, i_c12b44),
    (0xC12B46, i_c12b46),
    (0xC12B48, i_c12b48),
    (0xC12B4C, i_c12b4c),
    (0xC12B50, i_c12b50),
    (0xC12B52, i_c12b52),
    (0xC12B54, i_c12b54),
    (0xC12B56, i_c12b56),
    (0xC12B5A, i_c12b5a),
    (0xC12B5C, i_c12b5c),
    (0xC12B62, i_c12b62),
    (0xC12B66, i_c12b66),
    (0xC12B6A, i_c12b6a),
    (0xC12B6E, i_c12b6e),
    (0xC12B70, i_c12b70),
    (0xC12B72, i_c12b72),
    (0xC12B76, i_c12b76),
    (0xC12B7C, i_c12b7c),
    (0xC12B80, i_c12b80),
    (0xC12B82, i_c12b82),
    (0xC12B84, i_c12b84),
    (0xC12B88, i_c12b88),
    (0xC12B8C, i_c12b8c),
    (0xC12B8E, i_c12b8e),
    (0xC12B92, i_c12b92),
    (0xC12B96, i_c12b96),
    (0xC12B98, i_c12b98),
    (0xC12B9C, i_c12b9c),
    (0xC12BA0, i_c12ba0),
    (0xC12BA2, i_c12ba2),
    (0xC12BA6, i_c12ba6),
    (0xC12BA8, i_c12ba8),
    (0xC12BAA, i_c12baa),
    (0xC12BAC, i_c12bac),
    (0xC12BB0, i_c12bb0),
    (0xC12BB2, i_c12bb2),
    (0xC12BB6, i_c12bb6),
    (0xC12BB8, i_c12bb8),
    (0xC12BBA, i_c12bba),
    (0xC12BBE, i_c12bbe),
    (0xC12BC0, i_c12bc0),
    (0xC12BC4, i_c12bc4),
    (0xC12BC6, i_c12bc6),
    (0xC12BC8, i_c12bc8),
    (0xC12BCA, i_c12bca),
    (0xC12BCC, i_c12bcc),
    (0xC12BCE, i_c12bce),
    (0xC12BD2, i_c12bd2),
    (0xC12BD4, i_c12bd4),
    (0xC12BD8, i_c12bd8),
    (0xC12BDC, i_c12bdc),
    (0xC12BDE, i_c12bde),
    (0xC12BE0, i_c12be0),
    (0xC12BE2, i_c12be2),
    (0xC12BE4, i_c12be4),
    (0xC12BE8, i_c12be8),
    (0xC12BEC, i_c12bec),
    (0xC12BF0, i_c12bf0),
    (0xC12BF2, i_c12bf2),
    (0xC12BF6, i_c12bf6),
    (0xC12BF8, i_c12bf8),
    (0xC12BFA, i_c12bfa),
    (0xC12BFC, i_c12bfc),
    (0xC12BFE, i_c12bfe),
    (0xC12C00, i_c12c00),
    (0xC12C02, i_c12c02),
    (0xC12C04, i_c12c04),
    (0xC12C08, i_c12c08),
    (0xC12C0A, i_c12c0a),
    (0xC12C0C, i_c12c0c),
    (0xC12C0E, i_c12c0e),
    (0xC12C10, i_c12c10),
    (0xC12C14, i_c12c14),
    (0xC12C1A, i_c12c1a),
    (0xC12C1E, i_c12c1e),
    (0xC12C20, i_c12c20),
    (0xC12C22, i_c12c22),
    (0xC12C26, i_c12c26),
    (0xC12C28, i_c12c28),
    (0xC12C2A, i_c12c2a),
    (0xC12C2C, i_c12c2c),
    (0xC12C2E, i_c12c2e),
    (0xC12C32, i_c12c32),
    (0xC12C34, i_c12c34),
    (0xC12C36, i_c12c36),
    (0xC12C38, i_c12c38),
    (0xC12C3C, i_c12c3c),
    (0xC12C40, i_c12c40),
    (0xC12C42, i_c12c42),
    (0xC12C44, i_c12c44),
    (0xC12C46, i_c12c46),
    (0xC12C48, i_c12c48),
    (0xC12C4C, i_c12c4c),
    (0xC12C52, i_c12c52),
    (0xC12C54, i_c12c54),
    (0xC12C58, i_c12c58),
    (0xC12C5C, i_c12c5c),
    (0xC12C60, i_c12c60),
    (0xC12C62, i_c12c62),
    (0xC12C64, i_c12c64),
    (0xC12C68, i_c12c68),
    (0xC12C6E, i_c12c6e),
    (0xC12C72, i_c12c72),
    (0xC12C74, i_c12c74),
    (0xC12C76, i_c12c76),
    (0xC12C7A, i_c12c7a),
    (0xC12C7E, i_c12c7e),
    (0xC12C80, i_c12c80),
    (0xC12C84, i_c12c84),
    (0xC12C88, i_c12c88),
    (0xC12C8A, i_c12c8a),
    (0xC12C8E, i_c12c8e),
    (0xC12C92, i_c12c92),
    (0xC12C94, i_c12c94),
    (0xC12C98, i_c12c98),
    (0xC12C9A, i_c12c9a),
    (0xC12C9C, i_c12c9c),
    (0xC12C9E, i_c12c9e),
    (0xC12CA2, i_c12ca2),
    (0xC12CA4, i_c12ca4),
    (0xC12CA6, i_c12ca6),
    (0xC12CAA, i_c12caa),
    (0xC12CAC, i_c12cac),
    (0xC12CAE, i_c12cae),
    (0xC12CB2, i_c12cb2),
    (0xC12CB4, i_c12cb4),
    (0xC12CB8, i_c12cb8),
    (0xC12CBA, i_c12cba),
    (0xC12CBE, i_c12cbe),
    (0xC12CC0, i_c12cc0),
    (0xC12CC2, i_c12cc2),
    (0xC12CC4, i_c12cc4),
    (0xC12CC6, i_c12cc6),
    (0xC12CC8, i_c12cc8),
    (0xC12CCA, i_c12cca),
    (0xC12CCE, i_c12cce),
    (0xC12CD0, i_c12cd0),
    (0xC12CD4, i_c12cd4),
    (0xC12CD8, i_c12cd8),
    (0xC12CDA, i_c12cda),
    (0xC12CDC, i_c12cdc),
    (0xC12CDE, i_c12cde),
    (0xC12CE0, i_c12ce0),
    (0xC12CE2, i_c12ce2),
    (0xC12CE6, i_c12ce6),
    (0xC12CEA, i_c12cea),
    (0xC12CEE, i_c12cee),
    (0xC12CF0, i_c12cf0),
    (0xC12CF4, i_c12cf4),
    (0xC12CF6, i_c12cf6),
    (0xC12CF8, i_c12cf8),
    (0xC12CFA, i_c12cfa),
    (0xC12CFC, i_c12cfc),
    (0xC12CFE, i_c12cfe),
    (0xC12D00, i_c12d00),
    (0xC12D02, i_c12d02),
    (0xC12D04, i_c12d04),
    (0xC12D0A, i_c12d0a),
    (0xC12D0E, i_c12d0e),
    (0xC12D12, i_c12d12),
    (0xC12D14, i_c12d14),
    (0xC12D16, i_c12d16),
    (0xC12D1A, i_c12d1a),
    (0xC12D1C, i_c12d1c),
    (0xC12D20, i_c12d20),
    (0xC12D24, i_c12d24),
    (0xC12D28, i_c12d28),
    (0xC12D2C, i_c12d2c),
    (0xC12D32, i_c12d32),
    (0xC12D34, i_c12d34),
    (0xC12D38, i_c12d38),
    (0xC12D3A, i_c12d3a),
    (0xC12D3E, i_c12d3e),
    (0xC12D42, i_c12d42),
    (0xC12D44, i_c12d44),
    (0xC12D48, i_c12d48),
    (0xC12D4C, i_c12d4c),
    (0xC12D4E, i_c12d4e),
    (0xC12D52, i_c12d52),
    (0xC12D54, i_c12d54),
    (0xC12D56, i_c12d56),
    (0xC12D5A, i_c12d5a),
    (0xC12D5E, i_c12d5e),
    (0xC12D60, i_c12d60),
    (0xC12D66, i_c12d66),
    (0xC12D68, i_c12d68),
    (0xC12D6A, i_c12d6a),
    (0xC12D6E, i_c12d6e),
    (0xC12D70, i_c12d70),
    (0xC12D72, i_c12d72),
    (0xC12D76, i_c12d76),
    (0xC12D78, i_c12d78),
    (0xC12D7C, i_c12d7c),
    (0xC12D7E, i_c12d7e),
    (0xC12D82, i_c12d82),
    (0xC12D84, i_c12d84),
    (0xC12D88, i_c12d88),
    (0xC12D8A, i_c12d8a),
    (0xC12D8C, i_c12d8c),
    (0xC12D90, i_c12d90),
    (0xC12D94, i_c12d94),
    (0xC12D96, i_c12d96),
    (0xC12D9C, i_c12d9c),
    (0xC12D9E, i_c12d9e),
    (0xC12DA0, i_c12da0),
    (0xC12DA4, i_c12da4),
    (0xC12DA6, i_c12da6),
    (0xC12DA8, i_c12da8),
    (0xC12DAC, i_c12dac),
    (0xC12DAE, i_c12dae),
    (0xC12DB2, i_c12db2),
    (0xC12DB4, i_c12db4),
    (0xC12DB8, i_c12db8),
    (0xC12DBA, i_c12dba),
    (0xC12DBE, i_c12dbe),
    (0xC12DC0, i_c12dc0),
    (0xC12DC2, i_c12dc2),
    (0xC12DC6, i_c12dc6),
    (0xC12DCA, i_c12dca),
    (0xC12DCC, i_c12dcc),
    (0xC12DD0, i_c12dd0),
];
