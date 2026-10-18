// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $073590: movea.l $c(a6), a0
pub fn i_073590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73594;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073594: cmpi.b #$6, $76(a0)
pub fn i_073594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7359A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $07359A: bne.w $735a4
pub fn i_07359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7359E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x735A4; return 10; }
    8
}
// ===== SetTaskHandler_07359e  ($07359E, 8 B, kernel/task_handlers.c)
/// $07359E: lea.l $735a6(pc), a1
pub fn i_07359e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x735A6;
    16
}
/// $0735A2: move.l a1, (a6)
pub fn i_0735a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0735A4: rts
pub fn i_0735a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_PartExplode_0735a6  ($0735A6, 18 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0735A6: move.l #$ffffffff, $48(a6)
pub fn i_0735a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735AE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0735AE: addi.w #$40, $24(a6)
pub fn i_0735ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735B4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0735B4: bra.w $77fd6
pub fn i_0735b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x77FD6;
    10
}
// ===== M5Tank_Cabin_0735b8  ($0735B8, 200 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0735B8: clr.b $20(a6)
pub fn i_0735b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735BC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0735BC: lea.l $6a7de.l, a1
pub fn i_0735bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735C2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x6A7DE;
    20
}
/// $0735C2: jsr $4ae.l
pub fn i_0735c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x735C8);
    cpu.pc = 0x4AE;
    16
}
/// $0735C8: jsr $5dd02.l
pub fn i_0735c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x735CE);
    cpu.pc = 0x5DD02;
    16
}
/// $0735CE: move.l a0, $88(a6)
pub fn i_0735ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735D2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0735D2: move.w #$120, $98(a0)
pub fn i_0735d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735D8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0735D8: move.b #$e0, $9a(a0)
pub fn i_0735d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735DE;
    cpu.ir = 0x117C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0735DE: move.b #$38, $9b(a0)
pub fn i_0735de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735E4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0735E4: move.b #$ff, $9c(a0)
pub fn i_0735e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735EA;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0735EA: clr.b $9d(a0)
pub fn i_0735ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735EE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0735EE: move.b #$1, $9e(a0)
pub fn i_0735ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735F4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0735F4: move.l #$2db178, $8c(a6)
pub fn i_0735f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x735FC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DB178;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0735FC: move.w #$ed, d1
pub fn i_0735fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73600;
    cpu.ir = 0x323C;
    let v1: u32 = 0xED;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073600: jsr $236e.l
pub fn i_073600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73606;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73606);
    cpu.pc = 0x236E;
    16
}
/// $073606: move.w #$17, $1c(a6)
pub fn i_073606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7360C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x17;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07360C: jsr $138fe.l
pub fn i_07360c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73612);
    cpu.pc = 0x138FE;
    16
}
/// $073612: lea.l $2db732.l, a0
pub fn i_073612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73618;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB732;
    20
}
/// $073618: jsr $28cd4.l
pub fn i_073618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7361E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7361E);
    cpu.pc = 0x28CD4;
    16
}
/// $07361E: lea.l $73624(pc), a1
pub fn i_07361e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73622;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73624;
    16
}
/// $073622: move.l a1, (a6)
pub fn i_073622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73624;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073624: cmpi.b #$5, $76(a6)
pub fn i_073624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7362A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $07362A: bne.w $7363e
pub fn i_07362a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7362E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7363E; return 10; }
    8
}
/// $07362E: movea.l $88(a6), a0
pub fn i_07362e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73632;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073632: bset.b #$0, $13(a0)
pub fn i_073632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73638;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073638: lea.l $7363e(pc), a1
pub fn i_073638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7363C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7363E;
    16
}
/// $07363C: move.l a1, (a6)
pub fn i_07363c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7363E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07363E: jsr $741f8(pc)
pub fn i_07363e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73642;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73642);
    cpu.pc = 0x741F8;
    16
}
/// $073642: movea.l $c(a6), a0
pub fn i_073642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73646;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073646: btst.b #$7, $5a(a0)
pub fn i_073646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7364C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07364C: beq.w $73656
pub fn i_07364c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73650;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73656; return 10; }
    8
}
/// $073650: bset.b #$0, $5a(a6)
pub fn i_073650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73656;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073656: jsr $28d70.l
pub fn i_073656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7365C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7365C);
    cpu.pc = 0x28D70;
    16
}
/// $07365C: cmpi.b #$5, $76(a6)
pub fn i_07365c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73662;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $073662: beq.w $73676
pub fn i_073662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73666;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73676; return 10; }
    8
}
/// $073666: movea.l $88(a6), a0
pub fn i_073666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7366A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07366A: move.b #$2, $45(a0)
pub fn i_07366a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73670;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073670: move.b $9b(a6), $9b(a0)
pub fn i_073670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73676;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $073676: jsr $5e45a.l
pub fn i_073676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7367C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7367C);
    cpu.pc = 0x5E45A;
    16
}
/// $07367C: bcc.w $73686
pub fn i_07367c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73680;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73686; return 10; }
    8
}
// ===== SetTaskHandler_073680  ($073680, 8 B, kernel/task_handlers.c)
/// $073680: lea.l $734f4(pc), a1
pub fn i_073680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73684;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734F4;
    16
}
/// $073684: move.l a1, (a6)
pub fn i_073684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73686;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073686: rts
pub fn i_073686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73688;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Launcher_073688  ($073688, 204 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073688: clr.b $79(a6)
pub fn i_073688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7368C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07368C: lea.l $2bb286.l, a0
pub fn i_07368c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73692;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB286;
    20
}
/// $073692: jsr $799de.l
pub fn i_073692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73698);
    cpu.pc = 0x799DE;
    16
}
/// $073698: move.b d0, $72(a6)
pub fn i_073698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7369C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07369C: lea.l $2bb308.l, a0
pub fn i_07369c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB308;
    20
}
/// $0736A2: jsr $799de.l
pub fn i_0736a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x736A8);
    cpu.pc = 0x799DE;
    16
}
/// $0736A8: move.w d0, $70(a6)
pub fn i_0736a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736AC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0736AC: move.l #$2db194, $8c(a6)
pub fn i_0736ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736B4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DB194;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0736B4: move.w #$ee, d1
pub fn i_0736b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0736B8: jsr $236e.l
pub fn i_0736b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x736BE);
    cpu.pc = 0x236E;
    16
}
/// $0736BE: move.w #$18, $1c(a6)
pub fn i_0736be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0736C4: jsr $138fe.l
pub fn i_0736c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x736CA);
    cpu.pc = 0x138FE;
    16
}
/// $0736CA: lea.l $2db8d6.l, a0
pub fn i_0736ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB8D6;
    20
}
/// $0736D0: jsr $28cd4.l
pub fn i_0736d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x736D6);
    cpu.pc = 0x28CD4;
    16
}
/// $0736D6: lea.l $736dc(pc), a1
pub fn i_0736d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x736DC;
    16
}
/// $0736DA: move.l a1, (a6)
pub fn i_0736da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0736DC: jsr $741f8(pc)
pub fn i_0736dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x736E0);
    cpu.pc = 0x741F8;
    16
}
/// $0736E0: movea.l $c(a6), a0
pub fn i_0736e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736E4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0736E4: btst.b #$7, $5a(a0)
pub fn i_0736e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736EA;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0736EA: beq.w $736f4
pub fn i_0736ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x736F4; return 10; }
    8
}
/// $0736EE: bset.b #$0, $5a(a6)
pub fn i_0736ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736F4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0736F4: jsr $28d70.l
pub fn i_0736f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x736FA);
    cpu.pc = 0x28D70;
    16
}
/// $0736FA: subq.w #$1, $70(a6)
pub fn i_0736fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736FE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0736FE: cmpi.w #$0, $70(a6)
pub fn i_0736fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73704;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $073704: bgt.w $7374a
pub fn i_073704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73708;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7374A; return 10; }
    8
}
/// $073708: jsr $74336(pc)
pub fn i_073708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7370C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7370C);
    cpu.pc = 0x74336;
    16
}
/// $07370C: lea.l $2bb38a.l, a0
pub fn i_07370c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73712;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB38A;
    20
}
/// $073712: jsr $799de.l
pub fn i_073712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73718);
    cpu.pc = 0x799DE;
    16
}
/// $073718: move.w d0, $70(a6)
pub fn i_073718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7371C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07371C: subq.b #$1, $72(a6)
pub fn i_07371c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73720;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $073720: cmpi.b #$0, $72(a6)
pub fn i_073720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73726;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $073726: bgt.w $7374a
pub fn i_073726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7372A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7374A; return 10; }
    8
}
/// $07372A: lea.l $2bb286.l, a0
pub fn i_07372a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73730;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB286;
    20
}
/// $073730: jsr $799de.l
pub fn i_073730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73736);
    cpu.pc = 0x799DE;
    16
}
/// $073736: move.b d0, $72(a6)
pub fn i_073736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7373A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07373A: lea.l $2bb308.l, a0
pub fn i_07373a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73740;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB308;
    20
}
/// $073740: jsr $799de.l
pub fn i_073740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73746);
    cpu.pc = 0x799DE;
    16
}
/// $073746: move.w d0, $70(a6)
pub fn i_073746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7374A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07374A: jsr $5e45a.l
pub fn i_07374a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73750);
    cpu.pc = 0x5E45A;
    16
}
/// $073750: bcc.w $7375a
pub fn i_073750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73754;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7375A; return 10; }
    8
}
// ===== SetTaskHandler_073754  ($073754, 8 B, kernel/task_handlers.c)
/// $073754: lea.l $734f4(pc), a1
pub fn i_073754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73758;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734F4;
    16
}
/// $073758: move.l a1, (a6)
pub fn i_073758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7375A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07375A: rts
pub fn i_07375a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7375C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Turret_07375c  ($07375C, 160 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07375C: jsr $745a2(pc)
pub fn i_07375c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73760;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73760);
    cpu.pc = 0x745A2;
    16
}
/// $073760: lea.l $2baffc.l, a0
pub fn i_073760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73766;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BAFFC;
    20
}
/// $073766: jsr $799de.l
pub fn i_073766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7376C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7376C);
    cpu.pc = 0x799DE;
    16
}
/// $07376C: move.w d0, $66(a6)
pub fn i_07376c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73770;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073770: move.l #$2db0b0, $60(a6)
pub fn i_073770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73778;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DB0B0;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073778: move.w #$ec, d1
pub fn i_073778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7377C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07377C: jsr $236e.l
pub fn i_07377c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73782);
    cpu.pc = 0x236E;
    16
}
/// $073782: move.w #$17, $1c(a6)
pub fn i_073782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73788;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x17;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073788: jsr $138fe.l
pub fn i_073788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7378E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7378E);
    cpu.pc = 0x138FE;
    16
}
/// $07378E: lea.l $2dba40.l, a0
pub fn i_07378e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73794;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBA40;
    20
}
/// $073794: jsr $28cd4.l
pub fn i_073794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7379A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7379A);
    cpu.pc = 0x28CD4;
    16
}
/// $07379A: lea.l $737a0(pc), a1
pub fn i_07379a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7379E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x737A0;
    16
}
/// $07379E: move.l a1, (a6)
pub fn i_07379e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0737A0: jsr $28998.l
pub fn i_0737a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x737A6);
    cpu.pc = 0x28998;
    16
}
/// $0737A6: jsr $2783a.l
pub fn i_0737a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x737AC);
    cpu.pc = 0x2783A;
    16
}
/// $0737AC: jsr $28d70.l
pub fn i_0737ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x737B2);
    cpu.pc = 0x28D70;
    16
}
/// $0737B2: bcc.w $737bc
pub fn i_0737b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x737BC; return 10; }
    8
}
/// $0737B6: lea.l $73804(pc), a1
pub fn i_0737b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73804;
    16
}
/// $0737BA: move.l a1, (a6)
pub fn i_0737ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0737BC: jsr $2870a.l
pub fn i_0737bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x737C2);
    cpu.pc = 0x2870A;
    16
}
/// $0737C2: bcc.w $737de
pub fn i_0737c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x737DE; return 10; }
    8
}
/// $0737C6: jsr $519be.l
pub fn i_0737c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x737CC);
    cpu.pc = 0x519BE;
    16
}
/// $0737CC: bclr.b #$3, $13(a6)
pub fn i_0737cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737D2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0737D2: lea.l $5e766.l, a0
pub fn i_0737d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0737D8: jsr $5e770.l
pub fn i_0737d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x737DE);
    cpu.pc = 0x5E770;
    16
}
/// $0737DE: jsr $28758.l
pub fn i_0737de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x737E4);
    cpu.pc = 0x28758;
    16
}
/// $0737E4: bcc.w $737ee
pub fn i_0737e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x737EE; return 10; }
    8
}
/// $0737E8: lea.l $738d6(pc), a1
pub fn i_0737e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x738D6;
    16
}
/// $0737EC: move.l a1, (a6)
pub fn i_0737ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0737EE: movea.l $c(a6), a0
pub fn i_0737ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0737F2: cmpi.b #$5, $76(a0)
pub fn i_0737f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737F8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $0737F8: bne.w $73802
pub fn i_0737f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x737FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x73802; return 10; }
    8
}
// ===== SetTaskHandler_0737fc  ($0737FC, 8 B, kernel/task_handlers.c)
/// $0737FC: lea.l $738da(pc), a1
pub fn i_0737fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73800;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x738DA;
    16
}
/// $073800: move.l a1, (a6)
pub fn i_073800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73802;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073802: rts
pub fn i_073802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73804;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_TurretFire_073804  ($073804, 16 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073804: lea.l $73e5c(pc), a1
pub fn i_073804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73808;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73E5C;
    16
}
/// $073808: jsr $4ae.l
pub fn i_073808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7380E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7380E);
    cpu.pc = 0x4AE;
    16
}
/// $07380E: jsr $5dd02.l
pub fn i_07380e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73814);
    cpu.pc = 0x5DD02;
    16
}
// ===== M5Tank_TurretAim_073814  ($073814, 82 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073814: move.b #$0, $20(a6)
pub fn i_073814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7381A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07381A: lea.l $2bb182.l, a0
pub fn i_07381a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73820;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB182;
    20
}
/// $073820: jsr $799de.l
pub fn i_073820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73826);
    cpu.pc = 0x799DE;
    16
}
/// $073826: move.w d0, $70(a6)
pub fn i_073826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7382A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07382A: lea.l $2dba5a.l, a0
pub fn i_07382a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73830;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBA5A;
    20
}
/// $073830: jsr $28cd4.l
pub fn i_073830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73836;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73836);
    cpu.pc = 0x28CD4;
    16
}
/// $073836: lea.l $7383c(pc), a1
pub fn i_073836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7383A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7383C;
    16
}
/// $07383A: move.l a1, (a6)
pub fn i_07383a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7383C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07383C: jsr $28998.l
pub fn i_07383c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73842;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73842);
    cpu.pc = 0x28998;
    16
}
/// $073842: jsr $2783a.l
pub fn i_073842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73848;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73848);
    cpu.pc = 0x2783A;
    16
}
/// $073848: jsr $28d70.l
pub fn i_073848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7384E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7384E);
    cpu.pc = 0x28D70;
    16
}
/// $07384E: subq.w #$1, $70(a6)
pub fn i_07384e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73852;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $073852: cmpi.w #$0, $70(a6)
pub fn i_073852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73858;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $073858: bgt.w $73862
pub fn i_073858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7385C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x73862; return 10; }
    8
}
/// $07385C: lea.l $73866(pc), a1
pub fn i_07385c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73860;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73866;
    16
}
/// $073860: move.l a1, (a6)
pub fn i_073860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73862;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073862: bra.w $737bc
pub fn i_073862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73866;
    cpu.ir = 0x6000;
    cpu.pc = 0x737BC;
    10
}
// ===== M5Tank_TurretRecoil_073866  ($073866, 112 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073866: clr.b $21(a6)
pub fn i_073866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7386A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07386A: lea.l $2bb204.l, a0
pub fn i_07386a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73870;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB204;
    20
}
/// $073870: jsr $799de.l
pub fn i_073870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73876);
    cpu.pc = 0x799DE;
    16
}
/// $073876: move.w d0, $70(a6)
pub fn i_073876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7387A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07387A: lea.l $73880(pc), a1
pub fn i_07387a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7387E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73880;
    16
}
/// $07387E: move.l a1, (a6)
pub fn i_07387e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73880;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073880: subi.w #$1, $70(a6)
pub fn i_073880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73886;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $073886: cmpi.w #$0, $70(a6)
pub fn i_073886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7388C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $07388C: bgt.w $738a8
pub fn i_07388c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73890;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x738A8; return 10; }
    8
}
/// $073890: move.b #$1, $20(a6)
pub fn i_073890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73896;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073896: lea.l $2dba66.l, a0
pub fn i_073896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7389C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBA66;
    20
}
/// $07389C: jsr $28cd4.l
pub fn i_07389c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x738A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0738A2: lea.l $738a8(pc), a1
pub fn i_0738a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x738A8;
    16
}
/// $0738A6: move.l a1, (a6)
pub fn i_0738a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0738A8: jsr $28998.l
pub fn i_0738a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x738AE);
    cpu.pc = 0x28998;
    16
}
/// $0738AE: jsr $2783a.l
pub fn i_0738ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x738B4);
    cpu.pc = 0x2783A;
    16
}
/// $0738B4: jsr $28d70.l
pub fn i_0738b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x738BA);
    cpu.pc = 0x28D70;
    16
}
/// $0738BA: tst.b $21(a6)
pub fn i_0738ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738BE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0738BE: beq.w $738c8
pub fn i_0738be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x738C8; return 10; }
    8
}
/// $0738C2: lea.l $73814(pc), a1
pub fn i_0738c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73814;
    16
}
/// $0738C6: move.l a1, (a6)
pub fn i_0738c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0738C8: movea.l $c(a6), a0
pub fn i_0738c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738CC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0738CC: move.b #$1, $93(a0)
pub fn i_0738cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738D2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x93);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0738D2: bra.w $737bc
pub fn i_0738d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x737BC;
    10
}
// ===== M5Tank_TurretDie_0738d6  ($0738D6, 4 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0738D6: jsr $744aa(pc)
pub fn i_0738d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x738DA);
    cpu.pc = 0x744AA;
    16
}
// ===== M5Tank_TurretWreck_0738da  ($0738DA, 64 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0738DA: movea.l $c(a6), a0
pub fn i_0738da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738DE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0738DE: move.b #$5, $75(a0)
pub fn i_0738de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738E4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0738E4: move.l #$ffffffff, $60(a6)
pub fn i_0738e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738EC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0738EC: move.w #$102c, d0
pub fn i_0738ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738F0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0738F0: jsr $2352.l
pub fn i_0738f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x738F6);
    cpu.pc = 0x2352;
    16
}
/// $0738F6: move.l #$ffffffff, $48(a6)
pub fn i_0738f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x738FE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0738FE: lea.l $2dba94.l, a0
pub fn i_0738fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73904;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBA94;
    20
}
/// $073904: jsr $28cd4.l
pub fn i_073904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7390A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7390A);
    cpu.pc = 0x28CD4;
    16
}
/// $07390A: lea.l $7391a(pc), a1
pub fn i_07390a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7390E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7391A;
    16
}
/// $07390E: move.l a1, (a6)
pub fn i_07390e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73910;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073910: move.b #$ff, $92(a6)
pub fn i_073910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73916;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073916: bra.w $73934
pub fn i_073916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7391A;
    cpu.ir = 0x6000;
    cpu.pc = 0x73934;
    10
}
// ===== M5Tank_TurretWreck_Watch_07391a  ($07391A, 48 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07391A: clr.b $92(a6)
pub fn i_07391a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7391E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07391E: move.w #$92, d0
pub fn i_07391e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73922;
    cpu.ir = 0x303C;
    let v1: u32 = 0x92;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073922: lea.l $ee1da.l, a1
pub fn i_073922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73928;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE1DA;
    20
}
/// $073928: jsr $4429e.l
pub fn i_073928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7392E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7392E);
    cpu.pc = 0x4429E;
    16
}
/// $07392E: lea.l $73934(pc), a1
pub fn i_07392e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73932;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73934;
    16
}
/// $073932: move.l a1, (a6)
pub fn i_073932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73934;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073934: jsr $2783a.l
pub fn i_073934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7393A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7393A);
    cpu.pc = 0x2783A;
    16
}
/// $07393A: jsr $28d70.l
pub fn i_07393a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73940);
    cpu.pc = 0x28D70;
    16
}
/// $073940: jsr $5e45a.l
pub fn i_073940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73946);
    cpu.pc = 0x5E45A;
    16
}
/// $073946: bcc.w $73950
pub fn i_073946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7394A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73950; return 10; }
    8
}
// ===== SetTaskHandler_07394a  ($07394A, 8 B, kernel/task_handlers.c)
/// $07394A: lea.l $734f4(pc), a1
pub fn i_07394a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7394E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734F4;
    16
}
/// $07394E: move.l a1, (a6)
pub fn i_07394e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073950: rts
pub fn i_073950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73952;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Gun_073952  ($073952, 216 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073952: lea.l $2bb40c.l, a0
pub fn i_073952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73958;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB40C;
    20
}
/// $073958: jsr $799de.l
pub fn i_073958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7395E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7395E);
    cpu.pc = 0x799DE;
    16
}
/// $07395E: move.w d0, $70(a6)
pub fn i_07395e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73962;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073962: lea.l $2bb592.l, a0
pub fn i_073962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73968;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB592;
    20
}
/// $073968: jsr $799de.l
pub fn i_073968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7396E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7396E);
    cpu.pc = 0x799DE;
    16
}
/// $07396E: move.b d0, $72(a6)
pub fn i_07396e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73972;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073972: clr.b $79(a6)
pub fn i_073972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73976;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $073976: lea.l $2bb614.l, a0
pub fn i_073976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7397C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB614;
    20
}
/// $07397C: jsr $799de.l
pub fn i_07397c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73982;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73982);
    cpu.pc = 0x799DE;
    16
}
/// $073982: move.b d0, $98(a6)
pub fn i_073982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73986;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073986: move.w #$ed, d1
pub fn i_073986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7398A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xED;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07398A: jsr $236e.l
pub fn i_07398a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73990);
    cpu.pc = 0x236E;
    16
}
/// $073990: move.l #$2db1b0, $8c(a6)
pub fn i_073990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73998;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DB1B0;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073998: lea.l $2dbabc.l, a0
pub fn i_073998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7399E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBABC;
    20
}
/// $07399E: jsr $28cd4.l
pub fn i_07399e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x739A4);
    cpu.pc = 0x28CD4;
    16
}
/// $0739A4: lea.l $739aa(pc), a1
pub fn i_0739a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x739AA;
    16
}
/// $0739A8: move.l a1, (a6)
pub fn i_0739a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0739AA: jsr $7422c(pc)
pub fn i_0739aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x739AE);
    cpu.pc = 0x7422C;
    16
}
/// $0739AE: movea.l $c(a6), a0
pub fn i_0739ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739B2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0739B2: move.b $93(a0), d0
pub fn i_0739b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739B6;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x93);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0739B6: clr.b $93(a0)
pub fn i_0739b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739BA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x93);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0739BA: tst.b d0
pub fn i_0739ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739BC;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0739BC: beq.w $739d8
pub fn i_0739bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x739D8; return 10; }
    8
}
/// $0739C0: clr.b $79(a6)
pub fn i_0739c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739C4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0739C4: lea.l $2bb614.l, a0
pub fn i_0739c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB614;
    20
}
/// $0739CA: jsr $799de.l
pub fn i_0739ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x739D0);
    cpu.pc = 0x799DE;
    16
}
/// $0739D0: move.b d0, $98(a6)
pub fn i_0739d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739D4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0739D4: bra.w $73a06
pub fn i_0739d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x73A06;
    10
}
/// $0739D8: movea.l $c(a6), a0
pub fn i_0739d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739DC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0739DC: cmpi.b #$2, $75(a0)
pub fn i_0739dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739E2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0739E2: beq.w $73a06
pub fn i_0739e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73A06; return 10; }
    8
}
/// $0739E6: cmpi.b #$3, $75(a0)
pub fn i_0739e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739EC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $0739EC: beq.w $73a06
pub fn i_0739ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73A06; return 10; }
    8
}
/// $0739F0: subi.w #$1, $70(a6)
pub fn i_0739f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739F6;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0739F6: cmpi.w #$0, $70(a6)
pub fn i_0739f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x739FC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0739FC: bgt.w $73a06
pub fn i_0739fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A00;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x73A06; return 10; }
    8
}
/// $073A00: lea.l $73a32(pc), a1
pub fn i_073a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73A32;
    16
}
/// $073A04: move.l a1, (a6)
pub fn i_073a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073A06: jsr $28d70.l
pub fn i_073a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73A0C);
    cpu.pc = 0x28D70;
    16
}
/// $073A0C: movea.l $c(a6), a0
pub fn i_073a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A10;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073A10: cmpi.b #$6, $75(a0)
pub fn i_073a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A16;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Byte, false);
    16
}
/// $073A16: bne.w $73a20
pub fn i_073a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x73A20; return 10; }
    8
}
/// $073A1A: lea.l $734f4(pc), a1
pub fn i_073a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734F4;
    16
}
/// $073A1E: move.l a1, (a6)
pub fn i_073a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073A20: jsr $5e45a.l
pub fn i_073a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73A26);
    cpu.pc = 0x5E45A;
    16
}
/// $073A26: bcc.w $73a30
pub fn i_073a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73A30; return 10; }
    8
}
// ===== SetTaskHandler_073a2a  ($073A2A, 8 B, kernel/task_handlers.c)
/// $073A2A: lea.l $734f4(pc), a1
pub fn i_073a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734F4;
    16
}
/// $073A2E: move.l a1, (a6)
pub fn i_073a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073A30: rts
pub fn i_073a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_GunFire_073a32  ($073A32, 188 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073A32: movea.l $c(a6), a0
pub fn i_073a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A36;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073A36: cmpi.b #$2, $75(a0)
pub fn i_073a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A3C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $073A3C: ble.w $73a54
pub fn i_073a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A40;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x73A54; return 10; }
    8
}
/// $073A40: lea.l $2bb48e.l, a0
pub fn i_073a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB48E;
    20
}
/// $073A46: jsr $799de.l
pub fn i_073a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73A4C);
    cpu.pc = 0x799DE;
    16
}
/// $073A4C: move.w d0, $70(a6)
pub fn i_073a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A50;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073A50: bra.w $73a64
pub fn i_073a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A54;
    cpu.ir = 0x6000;
    cpu.pc = 0x73A64;
    10
}
/// $073A54: lea.l $2bb510.l, a0
pub fn i_073a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A5A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB510;
    20
}
/// $073A5A: jsr $799de.l
pub fn i_073a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73A60);
    cpu.pc = 0x799DE;
    16
}
/// $073A60: move.w d0, $70(a6)
pub fn i_073a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A64;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073A64: movea.l $c(a6), a0
pub fn i_073a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A68;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073A68: move.b $76(a0), d0
pub fn i_073a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A6C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $073A6C: andi.w #$1f, d0
pub fn i_073a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A70;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $073A70: movea.l #$2db1cc, a0
pub fn i_073a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A76;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DB1CC;
    cpu.a[0] = v1;
    4
}
/// $073A76: lsl.w #$2, d0
pub fn i_073a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A78;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $073A78: movea.l (a0, d0.w), a0
pub fn i_073a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A7C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $073A7C: cmpa.l #$ffffffff, a0
pub fn i_073a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A82;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $073A82: beq.w $73a8c
pub fn i_073a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A86;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73A8C; return 10; }
    8
}
/// $073A86: jsr $28cd4.l
pub fn i_073a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73A8C);
    cpu.pc = 0x28CD4;
    16
}
/// $073A8C: lea.l $73a92(pc), a1
pub fn i_073a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73A92;
    16
}
/// $073A90: move.l a1, (a6)
pub fn i_073a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073A92: jsr $74234(pc)
pub fn i_073a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73A96);
    cpu.pc = 0x74234;
    16
}
/// $073A96: jsr $28d70.l
pub fn i_073a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73A9C);
    cpu.pc = 0x28D70;
    16
}
/// $073A9C: bcc.w $73aea
pub fn i_073a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73AEA; return 10; }
    8
}
/// $073AA0: lea.l $739a4(pc), a1
pub fn i_073aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x739A4;
    16
}
/// $073AA4: move.l a1, (a6)
pub fn i_073aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073AA6: subi.b #$1, $72(a6)
pub fn i_073aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AAC;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $073AAC: cmpi.b #$0, $72(a6)
pub fn i_073aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AB2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $073AB2: bgt.w $73aea
pub fn i_073ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AB6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x73AEA; return 10; }
    8
}
/// $073AB6: clr.b $79(a6)
pub fn i_073ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73ABA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $073ABA: lea.l $2bb614.l, a0
pub fn i_073aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB614;
    20
}
/// $073AC0: jsr $799de.l
pub fn i_073ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73AC6);
    cpu.pc = 0x799DE;
    16
}
/// $073AC6: move.b d0, $98(a6)
pub fn i_073ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73ACA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073ACA: lea.l $2bb40c.l, a0
pub fn i_073aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB40C;
    20
}
/// $073AD0: jsr $799de.l
pub fn i_073ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73AD6);
    cpu.pc = 0x799DE;
    16
}
/// $073AD6: move.w d0, $70(a6)
pub fn i_073ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73ADA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073ADA: lea.l $2bb592.l, a0
pub fn i_073ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB592;
    20
}
/// $073AE0: jsr $799de.l
pub fn i_073ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73AE6);
    cpu.pc = 0x799DE;
    16
}
/// $073AE6: move.b d0, $72(a6)
pub fn i_073ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AEA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073AEA: bra.w $73a0c
pub fn i_073aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x73A0C;
    10
}
// ===== M5Tank_Casing_073aee  ($073AEE, 108 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073AEE: move.w #$ec, d1
pub fn i_073aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AF2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073AF2: jsr $236e.l
pub fn i_073af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73AF8);
    cpu.pc = 0x236E;
    16
}
/// $073AF8: move.w #$16, $1c(a6)
pub fn i_073af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73AFE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073AFE: jsr $138fe.l
pub fn i_073afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B04);
    cpu.pc = 0x138FE;
    16
}
/// $073B04: move.w #$8000, d0
pub fn i_073b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B08;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073B08: jsr $28134.l
pub fn i_073b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B0E);
    cpu.pc = 0x28134;
    16
}
/// $073B0E: andi.w #$ffe3, $38(a6)
pub fn i_073b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B14;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $073B14: ori.w #$14, $38(a6)
pub fn i_073b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B1A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $073B1A: lea.l $2dbd1c.l, a0
pub fn i_073b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBD1C;
    20
}
/// $073B20: jsr $28cd4.l
pub fn i_073b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B26);
    cpu.pc = 0x28CD4;
    16
}
/// $073B26: lea.l $73b2c(pc), a1
pub fn i_073b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73B2C;
    16
}
/// $073B2A: move.l a1, (a6)
pub fn i_073b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073B2C: jsr $2783a.l
pub fn i_073b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B32);
    cpu.pc = 0x2783A;
    16
}
/// $073B32: movea.l $c(a6), a0
pub fn i_073b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B36;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073B36: btst.b #$7, $5a(a0)
pub fn i_073b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B3C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $073B3C: beq.w $73b46
pub fn i_073b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73B46; return 10; }
    8
}
/// $073B40: bset.b #$0, $5a(a6)
pub fn i_073b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B46;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073B46: jsr $28d70.l
pub fn i_073b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B4C);
    cpu.pc = 0x28D70;
    16
}
/// $073B4C: movea.l $c(a6), a0
pub fn i_073b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B50;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073B50: cmpi.b #$5, $76(a0)
pub fn i_073b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B56;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $073B56: bne.w $73b60
pub fn i_073b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B5A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x73B60; return 10; }
    8
}
// ===== SetTaskHandler_073b5a  ($073B5A, 8 B, kernel/task_handlers.c)
/// $073B5A: lea.l $73b62(pc), a1
pub fn i_073b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73B62;
    16
}
/// $073B5E: move.l a1, (a6)
pub fn i_073b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073B60: rts
pub fn i_073b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_CasingGround_073b62  ($073B62, 40 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073B62: lea.l $2dbddc.l, a0
pub fn i_073b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B68;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBDDC;
    20
}
/// $073B68: jsr $28cd4.l
pub fn i_073b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B6E);
    cpu.pc = 0x28CD4;
    16
}
/// $073B6E: lea.l $73b74(pc), a1
pub fn i_073b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73B74;
    16
}
/// $073B72: move.l a1, (a6)
pub fn i_073b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073B74: jsr $2783a.l
pub fn i_073b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B7A);
    cpu.pc = 0x2783A;
    16
}
/// $073B7A: jsr $28d70.l
pub fn i_073b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B80);
    cpu.pc = 0x28D70;
    16
}
/// $073B80: jsr $5e45a.l
pub fn i_073b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B86);
    cpu.pc = 0x5E45A;
    16
}
/// $073B86: bcc.w $73b90
pub fn i_073b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73B90; return 10; }
    8
}
// ===== SetTaskHandler_073b8a  ($073B8A, 8 B, kernel/task_handlers.c)
/// $073B8A: lea.l $734f4(pc), a1
pub fn i_073b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734F4;
    16
}
/// $073B8E: move.l a1, (a6)
pub fn i_073b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073B90: rts
pub fn i_073b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_CasingExplode_073b92  ($073B92, 26 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073B92: move.w #$4000, d0
pub fn i_073b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B96;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073B96: jsr $28134.l
pub fn i_073b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73B9C);
    cpu.pc = 0x28134;
    16
}
/// $073B9C: andi.w #$ffe3, $38(a6)
pub fn i_073b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BA2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $073BA2: ori.w #$0, $38(a6)
pub fn i_073ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BA8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $073BA8: bra.w $77f6a
pub fn i_073ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x77F6A;
    10
}
// ===== M5Tank_Rocket_Launch_073bac  ($073BAC, 116 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073BAC: bset.b #$4, $6b(a6)
pub fn i_073bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BB2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073BB2: move.w #$23, d1
pub fn i_073bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BB6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073BB6: jsr $236e.l
pub fn i_073bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73BBC);
    cpu.pc = 0x236E;
    16
}
/// $073BBC: move.w #$10, $7e(a6)
pub fn i_073bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BC2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073BC2: clr.w $7c(a6)
pub fn i_073bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BC6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $073BC6: lea.l $2dbdee.l, a0
pub fn i_073bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBDEE;
    20
}
/// $073BCC: jsr $28cd4.l
pub fn i_073bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73BD2);
    cpu.pc = 0x28CD4;
    16
}
/// $073BD2: lea.l $73bd8(pc), a1
pub fn i_073bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73BD8;
    16
}
/// $073BD6: move.l a1, (a6)
pub fn i_073bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073BD8: jsr $74260(pc)
pub fn i_073bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73BDC);
    cpu.pc = 0x74260;
    16
}
/// $073BDC: jsr $28d70.l
pub fn i_073bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73BE2);
    cpu.pc = 0x28D70;
    16
}
/// $073BE2: bcc.w $73bf6
pub fn i_073be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73BF6; return 10; }
    8
}
/// $073BE6: move.w #$10a4, d0
pub fn i_073be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073BEA: jsr $2352.l
pub fn i_073bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73BF0);
    cpu.pc = 0x2352;
    16
}
/// $073BF0: lea.l $73c28(pc), a1
pub fn i_073bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73C28;
    16
}
/// $073BF4: move.l a1, (a6)
pub fn i_073bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073BF6: movea.l $c(a6), a0
pub fn i_073bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73BFA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073BFA: cmpi.b #$5, $76(a0)
pub fn i_073bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C00;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $073C00: bne.w $73c0a
pub fn i_073c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x73C0A; return 10; }
    8
}
/// $073C04: lea.l $73d76(pc), a1
pub fn i_073c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73D76;
    16
}
/// $073C08: move.l a1, (a6)
pub fn i_073c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073C0A: movea.l #$ffffffff, a0
pub fn i_073c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C10;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $073C10: lea.l $2db010.l, a0
pub fn i_073c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB010;
    20
}
/// $073C16: jsr $5dd56.l
pub fn i_073c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73C1C);
    cpu.pc = 0x5DD56;
    16
}
/// $073C1C: bcc.w $73c26
pub fn i_073c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73C26; return 10; }
    8
}
// ===== SetTaskHandler_073c20  ($073C20, 8 B, kernel/task_handlers.c)
/// $073C20: lea.l $734fc(pc), a1
pub fn i_073c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734FC;
    16
}
/// $073C24: move.l a1, (a6)
pub fn i_073c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073C26: rts
pub fn i_073c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Rocket_Fly_073c28  ($073C28, 334 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073C28: clr.w $80(a6)
pub fn i_073c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C2C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $073C2C: move.w #$14, $66(a6)
pub fn i_073c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C32;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073C32: move.w #$400, $36(a6)
pub fn i_073c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073C38: addi.w #$ffc8, $22(a6)
pub fn i_073c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C3E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFC8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $073C3E: addi.w #$f, $24(a6)
pub fn i_073c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C44;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xF, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $073C44: lea.l $2db228.l, a0
pub fn i_073c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C4A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB228;
    20
}
/// $073C4A: tst.b $98(a6)
pub fn i_073c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C4E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $073C4E: bne.w $73c6a
pub fn i_073c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x73C6A; return 10; }
    8
}
/// $073C52: movea.l $c(a6), a1
pub fn i_073c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C56;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $073C56: movea.l $c(a1), a1
pub fn i_073c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C5A;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $073C5A: cmpi.b #$5, $75(a1)
pub fn i_073c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C60;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $073C60: bne.w $73c6a
pub fn i_073c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x73C6A; return 10; }
    8
}
/// $073C64: lea.l $2db248.l, a0
pub fn i_073c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB248;
    20
}
/// $073C6A: move.b $79(a6), d0
pub fn i_073c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C6E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $073C6E: andi.w #$f, d0
pub fn i_073c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C72;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $073C72: add.w d0, d0
pub fn i_073c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C74;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $073C74: move.w (a0, d0.w), d0
pub fn i_073c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C78;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $073C78: andi.w #$f, d0
pub fn i_073c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C7C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $073C7C: lea.l $2db1e8.l, a0
pub fn i_073c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DB1E8;
    20
}
/// $073C82: lsl.w #$2, d0
pub fn i_073c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C84;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $073C84: move.l (a0, d0.w), $90(a6)
pub fn i_073c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C8A;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $073C8A: clr.w $94(a6)
pub fn i_073c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C8E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $073C8E: clr.b $96(a6)
pub fn i_073c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C92;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $073C92: move.w #$d000, d0
pub fn i_073c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C96;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073C96: jsr $28134.l
pub fn i_073c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73C9C);
    cpu.pc = 0x28134;
    16
}
/// $073C9C: andi.w #$ffe3, $38(a6)
pub fn i_073c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CA2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $073CA2: ori.w #$14, $38(a6)
pub fn i_073ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CA8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $073CA8: lea.l $2dbe40.l, a0
pub fn i_073ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBE40;
    20
}
/// $073CAE: jsr $28cd4.l
pub fn i_073cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73CB4);
    cpu.pc = 0x28CD4;
    16
}
/// $073CB4: lea.l $73cba(pc), a1
pub fn i_073cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73CBA;
    16
}
/// $073CB8: move.l a1, (a6)
pub fn i_073cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073CBA: jsr $78f8a.l
pub fn i_073cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73CC0);
    cpu.pc = 0x78F8A;
    16
}
/// $073CC0: bcc.w $73cca
pub fn i_073cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73CCA; return 10; }
    8
}
/// $073CC4: lea.l $73d76(pc), a1
pub fn i_073cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73D76;
    16
}
/// $073CC8: move.l a1, (a6)
pub fn i_073cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073CCA: move.b $34(a6), d0
pub fn i_073cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CCE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $073CCE: andi.w #$ff, d0
pub fn i_073cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CD2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $073CD2: lsr.w #$3, d0
pub fn i_073cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CD4;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $073CD4: move.w d0, $7e(a6)
pub fn i_073cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CD8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073CD8: jsr $28d70.l
pub fn i_073cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73CDE);
    cpu.pc = 0x28D70;
    16
}
/// $073CDE: jsr $283d8.l
pub fn i_073cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73CE4);
    cpu.pc = 0x283D8;
    16
}
/// $073CE4: btst.b #$1, $13(a6)
pub fn i_073ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CEA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $073CEA: beq.w $73cf4
pub fn i_073cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x73CF4; return 10; }
    8
}
/// $073CEE: lea.l $73b92(pc), a1
pub fn i_073cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73B92;
    16
}
/// $073CF2: move.l a1, (a6)
pub fn i_073cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073CF4: jsr $2870a.l
pub fn i_073cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73CFA);
    cpu.pc = 0x2870A;
    16
}
/// $073CFA: bcc.w $73d10
pub fn i_073cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73CFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73D10; return 10; }
    8
}
/// $073CFE: lea.l $5e766.l, a0
pub fn i_073cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $073D04: jsr $5e770.l
pub fn i_073d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73D0A);
    cpu.pc = 0x5E770;
    16
}
/// $073D0A: bclr.b #$3, $13(a6)
pub fn i_073d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D10;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073D10: jsr $28758.l
pub fn i_073d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73D16);
    cpu.pc = 0x28758;
    16
}
/// $073D16: bcc.w $73d20
pub fn i_073d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D1A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73D20; return 10; }
    8
}
/// $073D1A: lea.l $73d76(pc), a1
pub fn i_073d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73D76;
    16
}
/// $073D1E: move.l a1, (a6)
pub fn i_073d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073D20: bra.w $73bf6
pub fn i_073d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D24;
    cpu.ir = 0x6000;
    cpu.pc = 0x73BF6;
    10
}
// ===== M5Tank_Rocket_Explode_073d76  ($073D76, 172 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073D76: move.w #$1022, d0
pub fn i_073d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D7A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073D7A: jsr $2352.l
pub fn i_073d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73D80);
    cpu.pc = 0x2352;
    16
}
/// $073D80: bra.w $73b92
pub fn i_073d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73D84;
    cpu.ir = 0x6000;
    cpu.pc = 0x73B92;
    10
}
// ===== M5Tank_SmokeTrail_073e22  ($073E22, 50 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073E22: move.w #$2000, $38(a6)
pub fn i_073e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E28;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073E28: move.w #$d, d1
pub fn i_073e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E2C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073E2C: jsr $236e.l
pub fn i_073e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73E32);
    cpu.pc = 0x236E;
    16
}
/// $073E32: lea.l $2dc2e0.l, a0
pub fn i_073e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC2E0;
    20
}
/// $073E38: jsr $28cd4.l
pub fn i_073e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73E3E);
    cpu.pc = 0x28CD4;
    16
}
/// $073E3E: lea.l $73e44(pc), a1
pub fn i_073e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73E44;
    16
}
/// $073E42: move.l a1, (a6)
pub fn i_073e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073E44: jsr $2783a.l
pub fn i_073e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73E4A);
    cpu.pc = 0x2783A;
    16
}
/// $073E4A: jsr $28d70.l
pub fn i_073e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73E50);
    cpu.pc = 0x28D70;
    16
}
/// $073E50: bcc.w $73e5a
pub fn i_073e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73E5A; return 10; }
    8
}
// ===== SetTaskHandler_073e54  ($073E54, 8 B, kernel/task_handlers.c)
/// $073E54: lea.l $734fc(pc), a1
pub fn i_073e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734FC;
    16
}
/// $073E58: move.l a1, (a6)
pub fn i_073e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073E5A: rts
pub fn i_073e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Muzzle_073e5c  ($073E5C, 108 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073E5C: move.w #$ef, d1
pub fn i_073e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E60;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073E60: jsr $236e.l
pub fn i_073e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73E66);
    cpu.pc = 0x236E;
    16
}
/// $073E66: move.l #$2daf54, $4c(a6)
pub fn i_073e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E6E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DAF54;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073E6E: lea.l $2dc374.l, a0
pub fn i_073e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC374;
    20
}
/// $073E74: jsr $28cd4.l
pub fn i_073e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73E7A);
    cpu.pc = 0x28CD4;
    16
}
/// $073E7A: lea.l $73e80(pc), a1
pub fn i_073e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73E80;
    16
}
/// $073E7E: move.l a1, (a6)
pub fn i_073e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073E80: jsr $7428c(pc)
pub fn i_073e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73E84);
    cpu.pc = 0x7428C;
    16
}
/// $073E84: jsr $28d70.l
pub fn i_073e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73E8A);
    cpu.pc = 0x28D70;
    16
}
/// $073E8A: movea.l $c(a6), a0
pub fn i_073e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E8E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073E8E: cmpi.b #$1, $20(a0)
pub fn i_073e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E94;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $073E94: bne.w $73e9e
pub fn i_073e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x73E9E; return 10; }
    8
}
/// $073E98: lea.l $73ed0(pc), a1
pub fn i_073e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73ED0;
    16
}
/// $073E9C: move.l a1, (a6)
pub fn i_073e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73E9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073E9E: jsr $283ca.l
pub fn i_073e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73EA4);
    cpu.pc = 0x283CA;
    16
}
/// $073EA4: jsr $283d8.l
pub fn i_073ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73EAA);
    cpu.pc = 0x283D8;
    16
}
/// $073EAA: movea.l $c(a6), a0
pub fn i_073eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EAE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073EAE: cmpi.l #$7391a, (a0)
pub fn i_073eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EB4;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x7391A, Size::Long, false);
    16
}
/// $073EB4: bne.w $73ebe
pub fn i_073eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x73EBE; return 10; }
    8
}
/// $073EB8: lea.l $73f9a(pc), a1
pub fn i_073eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73F9A;
    16
}
/// $073EBC: move.l a1, (a6)
pub fn i_073ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073EBE: jsr $5e45a.l
pub fn i_073ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73EC4);
    cpu.pc = 0x5E45A;
    16
}
/// $073EC4: bcc.w $73ece
pub fn i_073ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73ECE; return 10; }
    8
}
// ===== SetTaskHandler_073ec8  ($073EC8, 8 B, kernel/task_handlers.c)
/// $073EC8: lea.l $73f9a(pc), a1
pub fn i_073ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73ECC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73F9A;
    16
}
/// $073ECC: move.l a1, (a6)
pub fn i_073ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73ECE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073ECE: rts
pub fn i_073ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73ED0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_MuzzleB_073ed0  ($073ED0, 40 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073ED0: lea.l $2dc406.l, a0
pub fn i_073ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73ED6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC406;
    20
}
/// $073ED6: jsr $28cd4.l
pub fn i_073ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73EDC);
    cpu.pc = 0x28CD4;
    16
}
/// $073EDC: lea.l $73ee2(pc), a1
pub fn i_073edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73EE2;
    16
}
/// $073EE0: move.l a1, (a6)
pub fn i_073ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073EE2: jsr $7428c(pc)
pub fn i_073ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73EE6);
    cpu.pc = 0x7428C;
    16
}
/// $073EE6: jsr $28d70.l
pub fn i_073ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73EEC);
    cpu.pc = 0x28D70;
    16
}
/// $073EEC: bcc.w $73ef6
pub fn i_073eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EF0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73EF6; return 10; }
    8
}
/// $073EF0: lea.l $73ef8(pc), a1
pub fn i_073ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73EF8;
    16
}
/// $073EF4: move.l a1, (a6)
pub fn i_073ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073EF6: bra.b $73e9e
pub fn i_073ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EF8;
    cpu.ir = 0x60A6;
    cpu.pc = 0x73E9E;
    10
}
// ===== M5Tank_MuzzleC_073ef8  ($073EF8, 102 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073EF8: move.w #$10c6, d0
pub fn i_073ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73EFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073EFC: jsr $2352.l
pub fn i_073efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73F02);
    cpu.pc = 0x2352;
    16
}
/// $073F02: move.l #$2daeb0, $4c(a6)
pub fn i_073f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F0A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DAEB0;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073F0A: lea.l $2bb100.l, a0
pub fn i_073f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB100;
    20
}
/// $073F10: jsr $799de.l
pub fn i_073f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73F16);
    cpu.pc = 0x799DE;
    16
}
/// $073F16: move.b d0, $72(a6)
pub fn i_073f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F1A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073F1A: lea.l $2dc4bc.l, a0
pub fn i_073f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC4BC;
    20
}
/// $073F20: jsr $28cd4.l
pub fn i_073f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73F26);
    cpu.pc = 0x28CD4;
    16
}
/// $073F26: lea.l $73f2c(pc), a1
pub fn i_073f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73F2C;
    16
}
/// $073F2A: move.l a1, (a6)
pub fn i_073f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073F2C: jsr $7428c(pc)
pub fn i_073f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73F30);
    cpu.pc = 0x7428C;
    16
}
/// $073F30: jsr $28d70.l
pub fn i_073f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73F36);
    cpu.pc = 0x28D70;
    16
}
/// $073F36: bcc.w $73f5a
pub fn i_073f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73F5A; return 10; }
    8
}
/// $073F3A: lea.l $2dc50c.l, a0
pub fn i_073f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC50C;
    20
}
/// $073F40: jsr $28cd4.l
pub fn i_073f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73F46);
    cpu.pc = 0x28CD4;
    16
}
/// $073F46: subq.b #$1, $72(a6)
pub fn i_073f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F4A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $073F4A: cmpi.b #$0, $72(a6)
pub fn i_073f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F50;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $073F50: bgt.w $73f5a
pub fn i_073f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F54;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x73F5A; return 10; }
    8
}
/// $073F54: lea.l $73f5e(pc), a1
pub fn i_073f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73F5E;
    16
}
/// $073F58: move.l a1, (a6)
pub fn i_073f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073F5A: bra.w $73e9e
pub fn i_073f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x73E9E;
    10
}
// ===== M5Tank_MuzzleD_073f5e  ($073F5E, 60 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073F5E: movea.l $c(a6), a0
pub fn i_073f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F62;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $073F62: move.b #$ff, $21(a0)
pub fn i_073f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F68;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $073F68: move.l #$ffffffff, $4c(a6)
pub fn i_073f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F70;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073F70: lea.l $2dc5ae.l, a0
pub fn i_073f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC5AE;
    20
}
/// $073F76: jsr $28cd4.l
pub fn i_073f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73F7C);
    cpu.pc = 0x28CD4;
    16
}
/// $073F7C: lea.l $73f82(pc), a1
pub fn i_073f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73F82;
    16
}
/// $073F80: move.l a1, (a6)
pub fn i_073f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073F82: jsr $7428c(pc)
pub fn i_073f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73F86);
    cpu.pc = 0x7428C;
    16
}
/// $073F86: jsr $28d70.l
pub fn i_073f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73F8C);
    cpu.pc = 0x28D70;
    16
}
/// $073F8C: bcc.w $73f96
pub fn i_073f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F90;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73F96; return 10; }
    8
}
/// $073F90: lea.l $73e66(pc), a1
pub fn i_073f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73E66;
    16
}
/// $073F94: move.l a1, (a6)
pub fn i_073f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073F96: bra.w $73e9e
pub fn i_073f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73F9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x73E9E;
    10
}
// ===== M5Tank_MuzzleE_073f9a  ($073F9A, 40 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073F9A: move.l #$ffffffff, $4c(a6)
pub fn i_073f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FA2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $073FA2: lea.l $2dc5ae.l, a0
pub fn i_073fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC5AE;
    20
}
/// $073FA8: jsr $28cd4.l
pub fn i_073fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73FAE);
    cpu.pc = 0x28CD4;
    16
}
/// $073FAE: lea.l $73fb4(pc), a1
pub fn i_073fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73FB4;
    16
}
/// $073FB2: move.l a1, (a6)
pub fn i_073fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073FB4: jsr $7428c(pc)
pub fn i_073fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x73FB8);
    cpu.pc = 0x7428C;
    16
}
/// $073FB8: jsr $28d70.l
pub fn i_073fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73FBE);
    cpu.pc = 0x28D70;
    16
}
/// $073FBE: bcc.w $73fc8
pub fn i_073fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x73FC8; return 10; }
    8
}
// ===== SetTaskHandler_073fc2  ($073FC2, 8 B, kernel/task_handlers.c)
/// $073FC2: lea.l $734fc(pc), a1
pub fn i_073fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734FC;
    16
}
/// $073FC6: move.l a1, (a6)
pub fn i_073fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $073FC8: rts
pub fn i_073fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Missile_Tmpl77_073fca  ($073FCA, 146 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $073FCA: move.b $98(a6), d0
pub fn i_073fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FCE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $073FCE: andi.w #$ff, d0
pub fn i_073fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FD2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $073FD2: lsl.w #$4, d0
pub fn i_073fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FD4;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $073FD4: move.w d0, $36(a6)
pub fn i_073fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FD8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073FD8: neg.w d0
pub fn i_073fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FDA;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $073FDA: move.w d0, $28(a6)
pub fn i_073fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FDE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073FDE: move.b $99(a6), d0
pub fn i_073fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FE2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $073FE2: andi.w #$ff, d0
pub fn i_073fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FE6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $073FE6: lsl.w #$4, d0
pub fn i_073fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FE8;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $073FE8: move.w d0, $5c(a6)
pub fn i_073fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FEC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $073FEC: move.w #$23, d1
pub fn i_073fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $073FF0: jsr $236e.l
pub fn i_073ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x73FF6);
    cpu.pc = 0x236E;
    16
}
/// $073FF6: bset.b #$4, $6b(a6)
pub fn i_073ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73FFC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $073FFC: lea.l $2dbe40.l, a0
pub fn i_073ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74002;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DBE40;
    20
}
/// $074002: jsr $28cd4.l
pub fn i_074002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74008;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74008);
    cpu.pc = 0x28CD4;
    16
}
/// $074008: lea.l $7400e(pc), a1
pub fn i_074008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7400C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7400E;
    16
}
/// $07400C: move.l a1, (a6)
pub fn i_07400c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7400E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07400E: move.w $5c(a6), d0
pub fn i_07400e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74012;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $074012: cmp.w $22(a6), d0
pub fn i_074012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74016;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $074016: blt.w $74026
pub fn i_074016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7401A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x74026; return 10; }
    8
}
/// $07401A: move.w #$ffe0, $2e(a6)
pub fn i_07401a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74020;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $074020: lea.l $74026(pc), a1
pub fn i_074020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74024;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x74026;
    16
}
/// $074024: move.l a1, (a6)
pub fn i_074024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74026;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074026: jsr $27d50.l
pub fn i_074026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7402C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7402C);
    cpu.pc = 0x27D50;
    16
}
/// $07402C: bcc.w $74036
pub fn i_07402c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74030;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x74036; return 10; }
    8
}
/// $074030: lea.l $73d76(pc), a1
pub fn i_074030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74034;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73D76;
    16
}
/// $074034: move.l a1, (a6)
pub fn i_074034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74036;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074036: move.w $28(a6), d0
pub fn i_074036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7403A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07403A: move.w $2a(a6), d1
pub fn i_07403a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7403E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07403E: asr.w #$4, d0
pub fn i_07403e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74040;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $074040: asr.w #$4, d1
pub fn i_074040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74042;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $074042: jsr $5e018.l
pub fn i_074042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74048);
    cpu.pc = 0x5E018;
    16
}
/// $074048: lsr.w #$3, d0
pub fn i_074048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7404A;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07404A: andi.w #$1f, d0
pub fn i_07404a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7404E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07404E: move.w d0, $7e(a6)
pub fn i_07404e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74052;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $074052: jsr $28d70.l
pub fn i_074052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74058);
    cpu.pc = 0x28D70;
    16
}
/// $074058: bra.w $73cde
pub fn i_074058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7405C;
    cpu.ir = 0x6000;
    cpu.pc = 0x73CDE;
    10
}
// ===== M5Tank_Frag_07405c  ($07405C, 104 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07405C: move.w #$d000, d0
pub fn i_07405c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74060;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $074060: jsr $28134.l
pub fn i_074060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74066;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74066);
    cpu.pc = 0x28134;
    16
}
/// $074066: andi.w #$ffe3, $38(a6)
pub fn i_074066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7406C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07406C: ori.w #$14, $38(a6)
pub fn i_07406c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74072;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $074072: movea.l $c(a6), a0
pub fn i_074072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74076;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $074076: move.b $98(a0), d0
pub fn i_074076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7407A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07407A: andi.w #$ff, d0
pub fn i_07407a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7407E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07407E: lea.l $2bb696.l, a0
pub fn i_07407e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74084;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB696;
    20
}
/// $074084: movea.l $c(a6), a1
pub fn i_074084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74088;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $074088: movea.l $c(a1), a1
pub fn i_074088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7408C;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07408C: cmpi.b #$3, $75(a1)
pub fn i_07408c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74092;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $074092: ble.w $7409c
pub fn i_074092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74096;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7409C; return 10; }
    8
}
/// $074096: lea.l $2bb6a6.l, a0
pub fn i_074096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7409C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB6A6;
    20
}
/// $07409C: lsl.w #$2, d0
pub fn i_07409c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7409E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07409E: movea.l (a0, d0.w), a0
pub fn i_07409e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740A2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0740A2: move.b $79(a6), d0
pub fn i_0740a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740A6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0740A6: andi.w #$7, d0
pub fn i_0740a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740AA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0740AA: lsl.w #$2, d0
pub fn i_0740aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740AC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0740AC: move.w (a0, d0.w), $28(a6)
pub fn i_0740ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740B2;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0740B2: move.w $2(a0, d0.w), $2e(a6)
pub fn i_0740b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740B8;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0740B8: move.w #$100, $2a(a6)
pub fn i_0740b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0740BE: jmp $6dbd4.l
pub fn i_0740be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DBD4;
    8
}
// ===== M5Tank_SmokeB_0740c4  ($0740C4, 66 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0740C4: move.w #$193, d1
pub fn i_0740c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x193;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0740C8: jsr $236e.l
pub fn i_0740c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x740CE);
    cpu.pc = 0x236E;
    16
}
/// $0740CE: lea.l $2dc616.l, a0
pub fn i_0740ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC616;
    20
}
/// $0740D4: jsr $28cd4.l
pub fn i_0740d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x740DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0740DA: lea.l $740e0(pc), a1
pub fn i_0740da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x740E0;
    16
}
/// $0740DE: move.l a1, (a6)
pub fn i_0740de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0740E0: jsr $2783a.l
pub fn i_0740e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x740E6);
    cpu.pc = 0x2783A;
    16
}
/// $0740E6: jsr $28d70.l
pub fn i_0740e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x740EC);
    cpu.pc = 0x28D70;
    16
}
/// $0740EC: bcc.w $740f6
pub fn i_0740ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x740F6; return 10; }
    8
}
/// $0740F0: lea.l $734fc(pc), a1
pub fn i_0740f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734FC;
    16
}
/// $0740F4: move.l a1, (a6)
pub fn i_0740f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0740F6: movea.l #$ffffffff, a0
pub fn i_0740f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740FC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0740FC: jsr $5dd56.l
pub fn i_0740fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74102);
    cpu.pc = 0x5DD56;
    16
}
/// $074102: bcc.w $7410c
pub fn i_074102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74106;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7410C; return 10; }
    8
}
// ===== SetTaskHandler_074106  ($074106, 8 B, kernel/task_handlers.c)
/// $074106: lea.l $734fc(pc), a1
pub fn i_074106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7410A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x734FC;
    16
}
/// $07410A: move.l a1, (a6)
pub fn i_07410a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7410C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07410C: rts
pub fn i_07410c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7410E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_InScreenSetC_07410e  ($07410E, 8 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07410E: jsr $74122(pc)
pub fn i_07410e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74112;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x74112);
    cpu.pc = 0x74122;
    16
}
/// $074112: bcs.w $7411c
pub fn i_074112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74116;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7411C; return 10; }
    8
}
// ===== JsrPcThunk_074116  ($074116, 6 B, thunks/jsr_pc_thunks.c)
/// $074116: jsr $74166(pc)
pub fn i_074116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7411A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7411A);
    cpu.pc = 0x74166;
    16
}
/// $07411A: rts
pub fn i_07411a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7411C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07411c  ($07411C, 6 B, thunks/ccr_helpers.c)
/// $07411C: ori.b #$11, ccr
pub fn i_07411c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74120;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $074120: rts
pub fn i_074120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74122;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_InScreen_074122  ($074122, 56 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074122: btst.b #$0, $73(a6)
pub fn i_074122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74128;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $074128: bne.w $7413a
pub fn i_074128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7412C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7413A; return 10; }
    8
}
/// $07412C: cmpi.w #$a0, $22(a6)
pub fn i_07412c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74132;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $074132: blt.w $74160
pub fn i_074132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74136;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x74160; return 10; }
    8
}
/// $074136: bra.w $7415a
pub fn i_074136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7413A;
    cpu.ir = 0x6000;
    cpu.pc = 0x7415A;
    10
}
/// $07413A: cmpi.w #$118, $22(a6)
pub fn i_07413a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74140;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x118, Size::Word, false);
    16
}
/// $074140: bgt.w $74160
pub fn i_074140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74144;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x74160; return 10; }
    8
}
/// $074144: move.w #$1050, d0
pub fn i_074144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74148;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1050;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $074148: move.w #$cf, d1
pub fn i_074148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7414C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07414C: jsr $440d0.l
pub fn i_07414c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74152);
    cpu.pc = 0x440D0;
    16
}
/// $074152: cmp.w $22(a6), d0
pub fn i_074152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74156;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $074156: blt.w $74160
pub fn i_074156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7415A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x74160; return 10; }
    8
}
// ===== ClearXN_07415a  ($07415A, 6 B, thunks/ccr_helpers.c)
/// $07415A: andi.b #$ee, ccr
pub fn i_07415a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7415E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07415E: rts
pub fn i_07415e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74160;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_074160  ($074160, 6 B, thunks/ccr_helpers.c)
/// $074160: ori.b #$11, ccr
pub fn i_074160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74164;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $074164: rts
pub fn i_074164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74166;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_TargetNear_074166  ($074166, 36 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074166: btst.b #$0, $73(a6)
pub fn i_074166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7416C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07416C: bne.w $7418a
pub fn i_07416c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74170;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7418A; return 10; }
    8
}
/// $074170: jsr $5e0d4.l
pub fn i_074170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74176;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74176);
    cpu.pc = 0x5E0D4;
    16
}
/// $074176: bcs.w $7418a
pub fn i_074176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7417A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7418A; return 10; }
    8
}
/// $07417A: move.w $22(a6), d0
pub fn i_07417a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7417E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07417E: sub.w $22(a0), d0
pub fn i_07417e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74182;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $074182: cmpi.w #$98, d0
pub fn i_074182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74186;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x98, Size::Word, false);
    8
}
/// $074186: blt.w $74190
pub fn i_074186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7418A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x74190; return 10; }
    8
}
// ===== ClearXN_07418a  ($07418A, 6 B, thunks/ccr_helpers.c)
/// $07418A: andi.b #$ee, ccr
pub fn i_07418a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7418E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07418E: rts
pub fn i_07418e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_074190  ($074190, 6 B, thunks/ccr_helpers.c)
/// $074190: ori.b #$11, ccr
pub fn i_074190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74194;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $074194: rts
pub fn i_074194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74196;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_AtGoalX_074196  ($074196, 30 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074196: move.w $5c(a6), d0
pub fn i_074196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7419A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07419A: move.w d0, d1
pub fn i_07419a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7419C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07419C: subi.w #$10, d0
pub fn i_07419c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741A0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0741A0: addi.w #$10, d1
pub fn i_0741a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741A4;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0741A4: cmp.w $22(a6), d0
pub fn i_0741a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741A8;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0741A8: bge.w $741ba
pub fn i_0741a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741AC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x741BA; return 10; }
    8
}
/// $0741AC: cmp.w $22(a6), d1
pub fn i_0741ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741B0;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0741B0: ble.w $741ba
pub fn i_0741b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741B4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x741BA; return 10; }
    8
}
// ===== SetXN_0741b4  ($0741B4, 6 B, thunks/ccr_helpers.c)
/// $0741B4: ori.b #$11, ccr
pub fn i_0741b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741B8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0741B8: rts
pub fn i_0741b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0741ba  ($0741BA, 6 B, thunks/ccr_helpers.c)
/// $0741BA: andi.b #$ee, ccr
pub fn i_0741ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741BE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0741BE: rts
pub fn i_0741be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_FollowParent_0741c0  ($0741C0, 56 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0741C0: jsr $5e506.l
pub fn i_0741c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x741C6);
    cpu.pc = 0x5E506;
    16
}
/// $0741C6: move.w $7c(a6), d0
pub fn i_0741c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741CA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0741CA: cmpi.b #$3, $75(a0)
pub fn i_0741ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741D0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $0741D0: bcs.w $741d6
pub fn i_0741d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741D4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x741D6; return 10; }
    8
}
/// $0741D4: addq.w #$2, d0
pub fn i_0741d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741D6;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0741D6: add.w d0, $38(a6)
pub fn i_0741d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741DA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0741DA: move.b $77(a0), d0
pub fn i_0741da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741DE;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0741DE: move.b $78(a0), d1
pub fn i_0741de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741E2;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0741E2: move.b d0, $77(a6)
pub fn i_0741e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741E6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0741E6: move.b d1, $78(a6)
pub fn i_0741e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741EA;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0741EA: ext.w d0
pub fn i_0741ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741EC;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0741EC: ext.w d1
pub fn i_0741ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741EE;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0741EE: add.w d0, $22(a6)
pub fn i_0741ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741F2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0741F2: add.w d1, $24(a6)
pub fn i_0741f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741F6;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0741F6: rts
pub fn i_0741f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_FollowParentSprite_0741f8  ($0741F8, 44 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0741F8: jsr $741c0(pc)
pub fn i_0741f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x741FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x741FC);
    cpu.pc = 0x741C0;
    16
}
/// $0741FC: move.b $76(a0), d0
pub fn i_0741fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74200;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $074200: cmp.b $76(a6), d0
pub fn i_074200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74204;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $074204: beq.w $7422a
pub fn i_074204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74208;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7422A; return 10; }
    8
}
/// $074208: move.b d0, $76(a6)
pub fn i_074208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7420C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07420C: andi.w #$1f, d0
pub fn i_07420c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74210;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $074210: movea.l $8c(a6), a0
pub fn i_074210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74214;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $074214: lsl.w #$2, d0
pub fn i_074214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74216;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $074216: movea.l (a0, d0.w), a0
pub fn i_074216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7421A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07421A: cmpa.l #$ffffffff, a0
pub fn i_07421a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74220;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $074220: beq.w $7422a
pub fn i_074220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74224;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7422A; return 10; }
    8
}
// ===== JsrAbsThunk_074224  ($074224, 8 B, thunks/jsr_abs_thunks.c)
/// $074224: jsr $28cd4.l
pub fn i_074224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7422A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7422A);
    cpu.pc = 0x28CD4;
    16
}
/// $07422A: rts
pub fn i_07422a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7422C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_FollowParentSpriteB_07422c  ($07422C, 52 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07422C: jsr $741f8(pc)
pub fn i_07422c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74230;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x74230);
    cpu.pc = 0x741F8;
    16
}
/// $074230: bra.w $74238
pub fn i_074230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74234;
    cpu.ir = 0x6000;
    cpu.pc = 0x74238;
    10
}
/// $074234: jsr $741c0(pc)
pub fn i_074234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74238;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x74238);
    cpu.pc = 0x741C0;
    16
}
/// $074238: move.w #$58, d0
pub fn i_074238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7423C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07423C: movea.l $c(a6), a0
pub fn i_07423c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74240;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $074240: cmpi.b #$4, $75(a0)
pub fn i_074240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74246;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $074246: blt.w $7424e
pub fn i_074246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7424A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7424E; return 10; }
    8
}
/// $07424A: move.w #$76, d0
pub fn i_07424a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7424E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07424E: add.w d0, $24(a6)
pub fn i_07424e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74252;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $074252: addi.w #$ffe0, $22(a6)
pub fn i_074252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74258;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $074258: move.w #$c000, $38(a6)
pub fn i_074258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7425E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07425E: rts
pub fn i_07425e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74260;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_FollowParentGun_074260  ($074260, 44 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074260: jsr $741c0(pc)
pub fn i_074260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74264;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x74264);
    cpu.pc = 0x741C0;
    16
}
/// $074264: move.b $7a(a0), d0
pub fn i_074264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74268;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $074268: move.w #$2, d1
pub fn i_074268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7426C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07426C: btst.b #$0, $79(a6)
pub fn i_07426c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74272;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $074272: beq.w $7427e
pub fn i_074272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74276;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7427E; return 10; }
    8
}
/// $074276: move.b $7b(a0), d0
pub fn i_074276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7427A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07427A: move.w #$fff6, d1
pub fn i_07427a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7427E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFF6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07427E: andi.w #$ff, d0
pub fn i_07427e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74282;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $074282: add.w d0, $24(a6)
pub fn i_074282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74286;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $074286: add.w d1, $22(a6)
pub fn i_074286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7428A;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07428A: rts
pub fn i_07428a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7428C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_FollowParentMuzzle_07428c  ($07428C, 18 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07428C: jsr $5e506.l
pub fn i_07428c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74292);
    cpu.pc = 0x5E506;
    16
}
/// $074292: subi.w #$30, $22(a6)
pub fn i_074292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74298;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $074298: subq.w #$8, $24(a6)
pub fn i_074298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7429C;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07429C: rts
pub fn i_07429c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7429E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Rocket_Home_07429e  ($07429E, 74 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07429E: move.b $106f28.l, d0
pub fn i_07429e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742A4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0742A4: andi.b #$7, d0
pub fn i_0742a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742A8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0742A8: bne.w $742ee
pub fn i_0742a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742AC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x742EE; return 10; }
    8
}
/// $0742AC: movea.l $84(a6), a0
pub fn i_0742ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742B0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0742B0: jsr $5e070.l
pub fn i_0742b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x742B6);
    cpu.pc = 0x5E070;
    16
}
/// $0742B6: lsr.w #$3, d0
pub fn i_0742b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742B8;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0742B8: move.w #$1, d1
pub fn i_0742b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742BC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0742BC: cmp.w $7e(a6), d0
pub fn i_0742bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742C0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0742C0: beq.w $742ee
pub fn i_0742c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x742EE; return 10; }
    8
}
/// $0742C4: bgt.w $742cc
pub fn i_0742c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742C8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x742CC; return 10; }
    8
}
/// $0742C8: move.w #$ffff, d1
pub fn i_0742c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742CC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0742CC: cmpi.w #$0, $80(a6)
pub fn i_0742cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0742D2: beq.w $742da
pub fn i_0742d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x742DA; return 10; }
    8
}
/// $0742D6: move.w $80(a6), d1
pub fn i_0742d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742DA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0742DA: add.w d1, $7e(a6)
pub fn i_0742da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742DE;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0742DE: andi.w #$1f, $7e(a6)
pub fn i_0742de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742E4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0742E4: jsr $74306(pc)
pub fn i_0742e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x742E8);
    cpu.pc = 0x74306;
    16
}
// ===== SetXN_0742e8  ($0742E8, 6 B, thunks/ccr_helpers.c)
/// $0742E8: ori.b #$11, ccr
pub fn i_0742e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742EC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0742EC: rts
pub fn i_0742ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_Rocket_RandFlip_0742ee  ($0742EE, 18 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0742EE: jsr $5e9b6.l
pub fn i_0742ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x742F4);
    cpu.pc = 0x5E9B6;
    16
}
/// $0742F4: andi.w #$1f, d0
pub fn i_0742f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0742F8: bne.w $74300
pub fn i_0742f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x742FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x74300; return 10; }
    8
}
/// $0742FC: neg.w $80(a6)
pub fn i_0742fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74300;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_074300  ($074300, 6 B, thunks/ccr_helpers.c)
/// $074300: andi.b #$ee, ccr
pub fn i_074300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74304;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $074304: rts
pub fn i_074304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74306;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_VelByAngle7E_074306  ($074306, 48 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074306: move.w $7e(a6), d0
pub fn i_074306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7430A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07430A: lsl.w #$4, d0
pub fn i_07430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7430C;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07430C: lea.l $2c072c.l, a1
pub fn i_07430c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74312;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $074312: lea.l $2c07ac.l, a2
pub fn i_074312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74318;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C07AC;
    20
}
/// $074318: move.w (a1, d0.w), d1
pub fn i_074318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7431C;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07431C: move.w (a2, d0.w), d2
pub fn i_07431c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74320;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $074320: move.w $36(a6), d0
pub fn i_074320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74324;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $074324: muls.w d0, d1
pub fn i_074324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74326;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $074326: muls.w d0, d2
pub fn i_074326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74328;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $074328: asr.l #$8, d1
pub fn i_074328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7432A;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $07432A: asr.l #$8, d2
pub fn i_07432a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7432C;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $07432C: move.w d1, $2a(a6)
pub fn i_07432c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74330;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $074330: move.w d2, $28(a6)
pub fn i_074330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74334;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $074334: rts
pub fn i_074334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74336;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnRocket_074336  ($074336, 100 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074336: cmpi.b #$5, $76(a6)
pub fn i_074336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7433C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $07433C: beq.w $74398
pub fn i_07433c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74340;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x74398; return 10; }
    8
}
/// $074340: clr.b $94(a6)
pub fn i_074340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74344;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $074344: movea.l $c(a6), a1
pub fn i_074344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74348;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $074348: cmpi.b #$5, $75(a1)
pub fn i_074348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7434E;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $07434E: blt.w $74398
pub fn i_07434e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74352;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x74398; return 10; }
    8
}
/// $074352: movea.l $c(a6), a1
pub fn i_074352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74356;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $074356: cmpi.b #$2, $75(a1)
pub fn i_074356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7435C;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $07435C: beq.w $74398
pub fn i_07435c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74360;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x74398; return 10; }
    8
}
/// $074360: cmpi.b #$3, $75(a1)
pub fn i_074360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74366;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $074366: beq.w $74398
pub fn i_074366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7436A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x74398; return 10; }
    8
}
/// $07436A: cmpi.b #$4, $75(a1)
pub fn i_07436a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74370;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $074370: bne.w $74378
pub fn i_074370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74374;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x74378; return 10; }
    8
}
/// $074374: clr.b $94(a6)
pub fn i_074374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74378;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $074378: lea.l $73bac(pc), a1
pub fn i_074378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7437C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73BAC;
    16
}
/// $07437C: jsr $4ae.l
pub fn i_07437c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74382);
    cpu.pc = 0x4AE;
    16
}
/// $074382: jsr $5dd02.l
pub fn i_074382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74388;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74388);
    cpu.pc = 0x5DD02;
    16
}
/// $074388: move.b $94(a6), $98(a0)
pub fn i_074388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7438E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $07438E: move.b $79(a6), $79(a0)
pub fn i_07438e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74394;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x79);
    bus.write8(ad4, v3 as u8);
    20
}
/// $074394: addq.b #$1, $79(a6)
pub fn i_074394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74398;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $074398: rts
pub fn i_074398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7439A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnCasing_07439a  ($07439A, 10 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07439A: lea.l $73aee(pc), a1
pub fn i_07439a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7439E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73AEE;
    16
}
/// $07439E: jsr $4ae.l
pub fn i_07439e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743A4);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_0743a4  ($0743A4, 8 B, thunks/jsr_abs_thunks.c)
/// $0743A4: jsr $5dd02.l
pub fn i_0743a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743AA);
    cpu.pc = 0x5DD02;
    16
}
/// $0743AA: rts
pub fn i_0743aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnTurret_0743ac  ($0743AC, 30 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0743AC: move.l #$2db064, $60(a6)
pub fn i_0743ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743B4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DB064;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0743B4: lea.l $7375c(pc), a1
pub fn i_0743b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7375C;
    16
}
/// $0743B8: jsr $4ae.l
pub fn i_0743b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743BE);
    cpu.pc = 0x4AE;
    16
}
/// $0743BE: jsr $5dd02.l
pub fn i_0743be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743C4);
    cpu.pc = 0x5DD02;
    16
}
/// $0743C4: clr.w $7c(a0)
pub fn i_0743c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743C8;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0743C8: rts
pub fn i_0743c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnExplosion_0743ca  ($0743CA, 12 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0743CA: lea.l $77f6a.l, a1
pub fn i_0743ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743D0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $0743D0: jsr $4ae.l
pub fn i_0743d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743D6);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_0743d6  ($0743D6, 8 B, thunks/jsr_abs_thunks.c)
/// $0743D6: jsr $5dd02.l
pub fn i_0743d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743DC);
    cpu.pc = 0x5DD02;
    16
}
/// $0743DC: rts
pub fn i_0743dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnExplosionB_0743de  ($0743DE, 34 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0743DE: move.w #$1021, d0
pub fn i_0743de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743E2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0743E2: jsr $2352.l
pub fn i_0743e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743E8);
    cpu.pc = 0x2352;
    16
}
/// $0743E8: lea.l $77fd6.l, a1
pub fn i_0743e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743EE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0743EE: jsr $4ae.l
pub fn i_0743ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743F4);
    cpu.pc = 0x4AE;
    16
}
/// $0743F4: jsr $5dd02.l
pub fn i_0743f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x743FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x743FA);
    cpu.pc = 0x5DD02;
    16
}
/// $0743FA: lea.l $2db28c.l, a1
pub fn i_0743fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74400;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DB28C;
    20
}
// ===== JsrAbsThunk_074400  ($074400, 8 B, thunks/jsr_abs_thunks.c)
/// $074400: jsr $77c7e.l
pub fn i_074400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74406;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74406);
    cpu.pc = 0x77C7E;
    16
}
/// $074406: rts
pub fn i_074406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74408;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnDebrisB_074408  ($074408, 6 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074408: lea.l $2db27a.l, a1
pub fn i_074408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7440E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DB27A;
    20
}
// ===== JsrAbsThunk_07440e  ($07440E, 8 B, thunks/jsr_abs_thunks.c)
/// $07440E: jsr $77c7e.l
pub fn i_07440e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74414;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74414);
    cpu.pc = 0x77C7E;
    16
}
/// $074414: rts
pub fn i_074414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74416;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnSmokeAndFrag_074416  ($074416, 74 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074416: move.w #$10ea, d0
pub fn i_074416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7441A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10EA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07441A: jsr $2352.l
pub fn i_07441a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74420);
    cpu.pc = 0x2352;
    16
}
/// $074420: lea.l $740c4(pc), a1
pub fn i_074420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74424;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x740C4;
    16
}
/// $074424: jsr $4ae.l
pub fn i_074424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7442A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7442A);
    cpu.pc = 0x4AE;
    16
}
/// $07442A: jsr $5dd02.l
pub fn i_07442a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74430);
    cpu.pc = 0x5DD02;
    16
}
/// $074430: lea.l $7405c(pc), a1
pub fn i_074430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74434;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7405C;
    16
}
/// $074434: jsr $4ae.l
pub fn i_074434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7443A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7443A);
    cpu.pc = 0x4AE;
    16
}
/// $07443A: jsr $5dd02.l
pub fn i_07443a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74440);
    cpu.pc = 0x5DD02;
    16
}
/// $074440: addi.w #$ffe4, $22(a0)
pub fn i_074440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74446;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $074446: addi.w #$fffc, $24(a0)
pub fn i_074446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7444C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFFC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07444C: move.b $79(a6), $79(a0)
pub fn i_07444c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74452;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x79);
    bus.write8(ad4, v3 as u8);
    20
}
/// $074452: addi.b #$1, $79(a6)
pub fn i_074452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74458;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $074458: andi.b #$7, $79(a6)
pub fn i_074458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7445E;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x7) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07445E: rts
pub fn i_07445e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74460;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SmokeByVel_Lut_074460  ($074460, 74 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074460: ?
pub fn i_074460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74462;
    cpu.ir = 0x180E;
    let v1 = cpu.a[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $074462: eori.b #$4, d7
pub fn i_074462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74466;
    cpu.ir = 0xA07;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 ^ 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $074466: subi.b #$2, d3
pub fn i_074466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7446A;
    cpu.ir = 0x403;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x2, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07446A: clr.b d1
pub fn i_07446a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7446C;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07446C: move.w $36(a6), d0
pub fn i_07446c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74470;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $074470: cmpi.w #$400, d0
pub fn i_074470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74474;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x400, Size::Word, false);
    8
}
/// $074474: bgt.w $7448a
pub fn i_074474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74478;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7448A; return 10; }
    8
}
/// $074478: lsr.w #$7, d0
pub fn i_074478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7447A;
    cpu.ir = 0xEE48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $07447A: lea.l $74460(pc), a1
pub fn i_07447a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7447E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x74460;
    16
}
/// $07447E: move.b (a1, d0.w), d1
pub fn i_07447e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74482;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $074482: cmp.b $3b(a6), d1
pub fn i_074482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74486;
    cpu.ir = 0xB22E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $074486: bcc.w $744a8
pub fn i_074486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7448A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x744A8; return 10; }
    8
}
/// $07448A: move.b d1, $3b(a6)
pub fn i_07448a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7448E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07448E: lea.l $73e22(pc), a1
pub fn i_07448e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74492;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x73E22;
    16
}
/// $074492: jsr $6fe.l
pub fn i_074492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74498);
    cpu.pc = 0x6FE;
    16
}
/// $074498: jsr $5dd02.l
pub fn i_074498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7449E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7449E);
    cpu.pc = 0x5DD02;
    16
}
/// $07449E: move.b $3b(a6), $44(a0)
pub fn i_07449e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744A4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x44);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0744A4: clr.b $3b(a6)
pub fn i_0744a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744A8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0744A8: rts
pub fn i_0744a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnScrap3_0744aa  ($0744AA, 176 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0744AA: lea.l $3fe5a.l, a1
pub fn i_0744aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744B0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3FE5A;
    20
}
/// $0744B0: jsr $4ae.l
pub fn i_0744b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x744B6);
    cpu.pc = 0x4AE;
    16
}
/// $0744B6: jsr $5dd02.l
pub fn i_0744b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x744BC);
    cpu.pc = 0x5DD02;
    16
}
/// $0744BC: clr.b $98(a0)
pub fn i_0744bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744C0;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0744C0: clr.b $9a(a0)
pub fn i_0744c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744C4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0744C4: move.b #$1, $9b(a0)
pub fn i_0744c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744CA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0744CA: move.b #$c8, $9c(a0)
pub fn i_0744ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744D0;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0744D0: clr.b $9d(a0)
pub fn i_0744d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744D4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0744D4: addi.w #$20, $24(a0)
pub fn i_0744d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744DA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0744DA: subi.w #$28, $22(a0)
pub fn i_0744da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744E0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0744E0: lea.l $3fe5a.l, a1
pub fn i_0744e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3FE5A;
    20
}
/// $0744E6: jsr $4ae.l
pub fn i_0744e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x744EC);
    cpu.pc = 0x4AE;
    16
}
/// $0744EC: jsr $5dd02.l
pub fn i_0744ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x744F2);
    cpu.pc = 0x5DD02;
    16
}
/// $0744F2: clr.b $98(a0)
pub fn i_0744f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744F6;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0744F6: clr.b $9a(a0)
pub fn i_0744f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744FA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0744FA: move.b #$a, $9b(a0)
pub fn i_0744fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74500;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $074500: move.b #$0, $9c(a0)
pub fn i_074500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74506;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $074506: move.b #$28, $9d(a0)
pub fn i_074506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7450C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07450C: addi.w #$20, $24(a0)
pub fn i_07450c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74512;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $074512: subq.w #$8, $22(a0)
pub fn i_074512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74516;
    cpu.ir = 0x5168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $074516: move.b #$4, $44(a0)
pub fn i_074516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7451C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07451C: lea.l $3fe5a.l, a1
pub fn i_07451c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74522;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3FE5A;
    20
}
/// $074522: jsr $4ae.l
pub fn i_074522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74528;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74528);
    cpu.pc = 0x4AE;
    16
}
/// $074528: jsr $5dd02.l
pub fn i_074528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7452E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7452E);
    cpu.pc = 0x5DD02;
    16
}
/// $07452E: clr.b $98(a0)
pub fn i_07452e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74532;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $074532: clr.b $9a(a0)
pub fn i_074532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74536;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $074536: move.b #$6, $9b(a0)
pub fn i_074536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7453C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07453C: move.b #$a, $9c(a0)
pub fn i_07453c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74542;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $074542: clr.b $9d(a0)
pub fn i_074542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74546;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $074546: addi.w #$20, $24(a0)
pub fn i_074546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7454C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07454C: addi.w #$18, $22(a0)
pub fn i_07454c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74552;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $074552: move.b #$8, $44(a0)
pub fn i_074552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74558;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $074558: rts
pub fn i_074558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7455A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnSmokeColumn_07455a  ($07455A, 48 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07455A: jsr $6e412.l
pub fn i_07455a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74560;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74560);
    cpu.pc = 0x6E412;
    16
}
/// $074560: move.w #$10, d0
pub fn i_074560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74564;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $074564: movem.w d0, -(a7)
pub fn i_074564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74568;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $074568: lea.l $62536.l, a1
pub fn i_074568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7456E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x62536;
    20
}
/// $07456E: jsr $4ae.l
pub fn i_07456e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74574);
    cpu.pc = 0x4AE;
    16
}
/// $074574: jsr $5dd02.l
pub fn i_074574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7457A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7457A);
    cpu.pc = 0x5DD02;
    16
}
/// $07457A: addi.w #$18, $24(a0)
pub fn i_07457a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74580;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $074580: movem.w (a7)+, d0
pub fn i_074580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74584;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $074584: subq.w #$1, d0
pub fn i_074584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74586;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $074586: bcc.b $74564
pub fn i_074586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74588;
    cpu.ir = 0x64DC;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x74564; return 10; }
    8
}
/// $074588: rts
pub fn i_074588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7458A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SetPrioD000_07458a  ($07458A, 24 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07458A: move.w #$d000, d0
pub fn i_07458a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7458E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07458E: jsr $28134.l
pub fn i_07458e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74594;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74594);
    cpu.pc = 0x28134;
    16
}
/// $074594: andi.w #$ffe3, $38(a6)
pub fn i_074594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7459A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07459A: ori.w #$1c, $38(a6)
pub fn i_07459a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745A0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0745A0: rts
pub fn i_0745a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnDebrisC_0745a2  ($0745A2, 6 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0745A2: lea.l $2db2de.l, a1
pub fn i_0745a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745A8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DB2DE;
    20
}
// ===== JsrAbsThunk_0745a8  ($0745A8, 8 B, thunks/jsr_abs_thunks.c)
/// $0745A8: jsr $43fac.l
pub fn i_0745a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x745AE);
    cpu.pc = 0x43FAC;
    16
}
/// $0745AE: rts
pub fn i_0745ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_SpawnDebrisD_0745b0  ($0745B0, 6 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0745B0: lea.l $2db364.l, a1
pub fn i_0745b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745B6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DB364;
    20
}
// ===== JsrAbsThunk_0745b6  ($0745B6, 8 B, thunks/jsr_abs_thunks.c)
/// $0745B6: jsr $43fac.l
pub fn i_0745b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x745BC);
    cpu.pc = 0x43FAC;
    16
}
/// $0745BC: rts
pub fn i_0745bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Tank_CmpDepthToParent_0745be  ($0745BE, 16 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0745BE: movea.l $8(a6), a1
pub fn i_0745be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745C2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0745C2: move.b $10(a6), d0
pub fn i_0745c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745C6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0745C6: cmp.b $10(a1), d0
pub fn i_0745c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745CA;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0745CA: bcs.w $745d4
pub fn i_0745ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745CE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x745D4; return 10; }
    8
}
// ===== ClearXN_0745ce  ($0745CE, 6 B, thunks/ccr_helpers.c)
/// $0745CE: andi.b #$ee, ccr
pub fn i_0745ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745D2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0745D2: rts
pub fn i_0745d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0745d4  ($0745D4, 6 B, thunks/ccr_helpers.c)
/// $0745D4: ori.b #$11, ccr
pub fn i_0745d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745D8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0745D8: rts
pub fn i_0745d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MiniScript_Init_0745da  ($0745DA, 8 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0745DA: move.l a1, (a0)
pub fn i_0745da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745DC;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0745DC: clr.w $4(a0)
pub fn i_0745dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745E0;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0745E0: rts
pub fn i_0745e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MiniScript_Step_0745e2  ($0745E2, 66 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0745E2: movea.l (a0), a1
pub fn i_0745e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745E4;
    cpu.ir = 0x2250;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[1] = v2;
    12
}
/// $0745E4: move.w (a1)+, d0
pub fn i_0745e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745E6;
    cpu.ir = 0x3019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $0745E6: lsl.w #$2, d0
pub fn i_0745e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745E8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0745E8: lea.l $745f0(pc), a2
pub fn i_0745e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745EC;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x745F0;
    16
}
/// $0745EC: jmp (a2, d0.w)
pub fn i_0745ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745F0;
    cpu.ir = 0x4EF2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $0745F0: bra.w $74624
pub fn i_0745f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x74624;
    10
}
/// $0745F4: bra.w $7462a
pub fn i_0745f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x7462A;
    10
}
/// $0745F8: bra.w $74632
pub fn i_0745f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x745FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x74632;
    10
}
/// $0745FC: bra.w $7464a
pub fn i_0745fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74600;
    cpu.ir = 0x6000;
    cpu.pc = 0x7464A;
    10
}
/// $074600: bra.w $74668
pub fn i_074600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74604;
    cpu.ir = 0x6000;
    cpu.pc = 0x74668;
    10
}
/// $074604: bra.w $74652
pub fn i_074604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74608;
    cpu.ir = 0x6000;
    cpu.pc = 0x74652;
    10
}
/// $074608: bra.w $7465c
pub fn i_074608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7460C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7465C;
    10
}
/// $07460C: bra.w $74610
pub fn i_07460c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74610;
    cpu.ir = 0x6000;
    cpu.pc = 0x74610;
    10
}
/// $074610: move.w (a1)+, d0
pub fn i_074610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74612;
    cpu.ir = 0x3019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $074612: move.l a1, (a0)
pub fn i_074612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74614;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074614: movem.l a0, -(a7)
pub fn i_074614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74618;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074618: jsr $2352.l
pub fn i_074618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7461E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7461E);
    cpu.pc = 0x2352;
    16
}
/// $07461E: movem.l (a7)+, a0
pub fn i_07461e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74622;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074622: bra.b $745e2
pub fn i_074622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74624;
    cpu.ir = 0x60BE;
    cpu.pc = 0x745E2;
    10
}
// ===== SetC_074624  ($074624, 6 B, thunks/ccr_helpers.c)
/// $074624: ori.b #$1, ccr
pub fn i_074624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74628;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $074628: rts
pub fn i_074628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7462A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MiniScript_OpSetTimer_07462a  ($07462A, 8 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07462A: move.w (a1)+, $4(a0)
pub fn i_07462a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7462E;
    cpu.ir = 0x3159;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad4, v3 as u16);
    16
}
/// $07462E: move.l a1, (a0)
pub fn i_07462e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74630;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074630: bra.b $745e2
pub fn i_074630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74632;
    cpu.ir = 0x60B0;
    cpu.pc = 0x745E2;
    10
}
// ===== MiniScript_OpWait_074632  ($074632, 18 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074632: move.w $4(a0), d0
pub fn i_074632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74636;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $074636: bne.w $7463e
pub fn i_074636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7463A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7463E; return 10; }
    8
}
/// $07463A: move.l a1, (a0)
pub fn i_07463a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7463C;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07463C: bra.b $745e2
pub fn i_07463c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7463E;
    cpu.ir = 0x60A4;
    cpu.pc = 0x745E2;
    10
}
/// $07463E: subq.w #$1, d0
pub fn i_07463e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74640;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $074640: move.w d0, $4(a0)
pub fn i_074640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74644;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== ClearC_074644  ($074644, 6 B, thunks/ccr_helpers.c)
/// $074644: andi.b #$fe, ccr
pub fn i_074644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74648;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $074648: rts
pub fn i_074648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7464A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MiniScript_OpCall_07464a  ($07464A, 2 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07464A: jsr (a1)
pub fn i_07464a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7464C;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x7464C);
    cpu.jump_to(cpu.a[1]);
    16
}
// ===== ClearC_07464c  ($07464C, 6 B, thunks/ccr_helpers.c)
/// $07464C: andi.b #$fe, ccr
pub fn i_07464c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74650;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $074650: rts
pub fn i_074650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74652;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MiniScript_OpSetFlag_074652  ($074652, 10 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074652: move.w (a1)+, d0
pub fn i_074652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74654;
    cpu.ir = 0x3019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $074654: move.l a1, (a0)
pub fn i_074654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74656;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074656: st.b (a6, d0.w)
pub fn i_074656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7465A;
    cpu.ir = 0x50F6;
    let c1: u32 = if cpu.sr.evaluate(Condition::True) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    14
}
/// $07465A: bra.b $745e2
pub fn i_07465a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7465C;
    cpu.ir = 0x6086;
    cpu.pc = 0x745E2;
    10
}
// ===== MiniScript_OpClrFlag_07465c  ($07465C, 12 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07465C: move.w (a1)+, d0
pub fn i_07465c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7465E;
    cpu.ir = 0x3019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $07465E: move.l a1, (a0)
pub fn i_07465e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74660;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074660: clr.b (a6, d0.w)
pub fn i_074660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74664;
    cpu.ir = 0x4236;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $074664: bra.w $745e2
pub fn i_074664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74668;
    cpu.ir = 0x6000;
    cpu.pc = 0x745E2;
    10
}
// ===== MiniScript_OpCallLoop_074668  ($074668, 6 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $074668: jsr (a1)
pub fn i_074668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7466A;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x7466A);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $07466A: bra.w $745e2
pub fn i_07466a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7466E;
    cpu.ir = 0x6000;
    cpu.pc = 0x745E2;
    10
}
/// $0749F2: movem.l a0/a6, -(a7)
pub fn i_0749f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x749F6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0749F6: movem.l $74a0c(pc), d0/a1
pub fn i_0749f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x749FC;
    cpu.ir = 0x4CFA;
    let mut a = 0x74A0E;
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $0749FC: jsr $4429e.l
pub fn i_0749fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74A02);
    cpu.pc = 0x4429E;
    16
}
/// $074A02: movem.l (a7)+, a0/a6
pub fn i_074a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A06;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074A06: move.l #$74a16, (a0)
pub fn i_074a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A0C;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74A16;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074A0C: rts
pub fn i_074a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074A18: cmpi.w #$80, $74(a6)
pub fn i_074a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A1E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Word, false);
    16
}
/// $074A1E: bhi.w $74a28
pub fn i_074a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A22;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x74A28; return 10; }
    8
}
/// $074A22: move.l #$74a2a, (a0)
pub fn i_074a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A28;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74A2A;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074A28: rts
pub fn i_074a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074A30: movem.l a0/a6, -(a7)
pub fn i_074a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A34;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074A34: movem.l $74a4a(pc), a2
pub fn i_074a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A3A;
    cpu.ir = 0x4CFA;
    let mut a = 0x74A4C;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074A3A: jsr $5022a.l
pub fn i_074a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74A40);
    cpu.pc = 0x5022A;
    16
}
/// $074A40: movem.l (a7)+, a0/a6
pub fn i_074a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A44;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074A44: move.l #$74a50, (a0)
pub fn i_074a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A4A;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74A50;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074A4A: rts
pub fn i_074a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074A52: cmpi.w #$66, $74(a6)
pub fn i_074a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A58;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x66, Size::Word, false);
    16
}
/// $074A58: bhi.w $74a62
pub fn i_074a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A5C;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x74A62; return 10; }
    8
}
/// $074A5C: move.l #$74a64, (a0)
pub fn i_074a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A62;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74A64;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074A62: rts
pub fn i_074a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074A6A: movem.l a0/a6, -(a7)
pub fn i_074a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A6E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074A6E: movem.l $74a84(pc), a2
pub fn i_074a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A74;
    cpu.ir = 0x4CFA;
    let mut a = 0x74A86;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074A74: jsr $5022a.l
pub fn i_074a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74A7A);
    cpu.pc = 0x5022A;
    16
}
/// $074A7A: movem.l (a7)+, a0/a6
pub fn i_074a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A7E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074A7E: move.l #$74a8a, (a0)
pub fn i_074a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A84;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74A8A;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074A84: rts
pub fn i_074a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074A8C: cmpi.w #$4c, $74(a6)
pub fn i_074a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A92;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4C, Size::Word, false);
    16
}
/// $074A92: bhi.w $74a9c
pub fn i_074a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A96;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x74A9C; return 10; }
    8
}
/// $074A96: move.l #$74a9e, (a0)
pub fn i_074a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A9C;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74A9E;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074A9C: rts
pub fn i_074a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074AA8: movem.l a0/a6, -(a7)
pub fn i_074aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AAC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074AAC: movem.l $74ac2(pc), a2
pub fn i_074aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AB2;
    cpu.ir = 0x4CFA;
    let mut a = 0x74AC4;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074AB2: jsr $5022a.l
pub fn i_074ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74AB8);
    cpu.pc = 0x5022A;
    16
}
/// $074AB8: movem.l (a7)+, a0/a6
pub fn i_074ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74ABC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074ABC: move.l #$74ac8, (a0)
pub fn i_074abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AC2;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74AC8;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074AC2: rts
pub fn i_074ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074ACA: movem.l a0/a6, -(a7)
pub fn i_074aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74ACE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074ACE: movem.l $74ae4(pc), a2
pub fn i_074ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AD4;
    cpu.ir = 0x4CFA;
    let mut a = 0x74AE6;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074AD4: jsr $5022a.l
pub fn i_074ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74ADA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74ADA);
    cpu.pc = 0x5022A;
    16
}
/// $074ADA: movem.l (a7)+, a0/a6
pub fn i_074ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74ADE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074ADE: move.l #$74aea, (a0)
pub fn i_074ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AE4;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74AEA;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074AE4: rts
pub fn i_074ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074AEC: movem.l a0/a6, -(a7)
pub fn i_074aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AF0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074AF0: movem.l $74b06(pc), a2
pub fn i_074af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AF6;
    cpu.ir = 0x4CFA;
    let mut a = 0x74B08;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074AF6: jsr $5022a.l
pub fn i_074af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74AFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74AFC);
    cpu.pc = 0x5022A;
    16
}
/// $074AFC: movem.l (a7)+, a0/a6
pub fn i_074afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B00;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074B00: move.l #$74b0c, (a0)
pub fn i_074b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B06;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74B0C;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074B06: rts
pub fn i_074b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074B0E: cmpi.w #$33, $74(a6)
pub fn i_074b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x33, Size::Word, false);
    16
}
/// $074B14: bhi.w $74b1e
pub fn i_074b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B18;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x74B1E; return 10; }
    8
}
/// $074B18: move.l #$74b20, (a0)
pub fn i_074b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B1E;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74B20;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074B1E: rts
pub fn i_074b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074B26: movem.l a0/a6, -(a7)
pub fn i_074b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B2A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074B2A: movem.l $74b40(pc), d0
pub fn i_074b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B30;
    cpu.ir = 0x4CFA;
    let mut a = 0x74B42;
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074B30: jsr $2352.l
pub fn i_074b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74B36);
    cpu.pc = 0x2352;
    16
}
/// $074B36: movem.l (a7)+, a0/a6
pub fn i_074b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B3A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074B3A: move.l #$74b46, (a0)
pub fn i_074b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B40;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74B46;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074B40: rts
pub fn i_074b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074B48: movem.l a0/a6, -(a7)
pub fn i_074b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B4C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074B4C: movem.l $74b62(pc), a2
pub fn i_074b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B52;
    cpu.ir = 0x4CFA;
    let mut a = 0x74B64;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074B52: jsr $5022a.l
pub fn i_074b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74B58);
    cpu.pc = 0x5022A;
    16
}
/// $074B58: movem.l (a7)+, a0/a6
pub fn i_074b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B5C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074B5C: move.l #$74b68, (a0)
pub fn i_074b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B62;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74B68;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074B62: rts
pub fn i_074b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074B6A: cmpi.w #$19, $74(a6)
pub fn i_074b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B70;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x19, Size::Word, false);
    16
}
/// $074B70: bhi.w $74b7a
pub fn i_074b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B74;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x74B7A; return 10; }
    8
}
/// $074B74: move.l #$74b7c, (a0)
pub fn i_074b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B7A;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74B7C;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074B7A: rts
pub fn i_074b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074B82: movem.l a0/a6, -(a7)
pub fn i_074b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B86;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074B86: movem.l $74b9c(pc), a2
pub fn i_074b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B8C;
    cpu.ir = 0x4CFA;
    let mut a = 0x74B9E;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074B8C: jsr $5022a.l
pub fn i_074b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74B92);
    cpu.pc = 0x5022A;
    16
}
/// $074B92: movem.l (a7)+, a0/a6
pub fn i_074b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B96;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074B96: move.l #$74ba2, (a0)
pub fn i_074b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B9C;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74BA2;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074B9C: rts
pub fn i_074b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74B9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074BA4: cmpi.w #$0, $66(a6)
pub fn i_074ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BAA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $074BAA: bne.w $74bb4
pub fn i_074baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x74BB4; return 10; }
    8
}
/// $074BAE: move.l #$74bb6, (a0)
pub fn i_074bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BB4;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74BB6;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074BB4: rts
pub fn i_074bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074BC0: movem.l a0/a6, -(a7)
pub fn i_074bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BC4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074BC4: movem.l $74bda(pc), a1
pub fn i_074bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BCA;
    cpu.ir = 0x4CFA;
    let mut a = 0x74BDC;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074BCA: jsr $4ae.l
pub fn i_074bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74BD0);
    cpu.pc = 0x4AE;
    16
}
/// $074BD0: movem.l (a7)+, a0/a6
pub fn i_074bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BD4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074BD4: move.l #$74be0, (a0)
pub fn i_074bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BDA;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74BE0;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074BDA: rts
pub fn i_074bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074BE2: movem.l a0/a6, -(a7)
pub fn i_074be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BE6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074BE6: movem.l $74bfc(pc), a1
pub fn i_074be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BEC;
    cpu.ir = 0x4CFA;
    let mut a = 0x74BFE;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074BEC: jsr $77c7e.l
pub fn i_074bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74BF2);
    cpu.pc = 0x77C7E;
    16
}
/// $074BF2: movem.l (a7)+, a0/a6
pub fn i_074bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BF6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074BF6: move.l #$74c02, (a0)
pub fn i_074bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BFC;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74C02;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074BFC: rts
pub fn i_074bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74BFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074C08: movem.l a0/a6, -(a7)
pub fn i_074c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C0C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074C0C: jsr $434ce.l
pub fn i_074c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74C12);
    cpu.pc = 0x434CE;
    16
}
/// $074C12: movem.l (a7)+, a0/a6
pub fn i_074c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C16;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074C16: move.l #$74c1e, (a0)
pub fn i_074c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C1C;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74C1E;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074C1C: rts
pub fn i_074c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074C26: movem.l a0/a6, -(a7)
pub fn i_074c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C2A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074C2A: movem.l $74c40(pc), a1
pub fn i_074c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C30;
    cpu.ir = 0x4CFA;
    let mut a = 0x74C42;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074C30: jsr $4ae.l
pub fn i_074c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74C36);
    cpu.pc = 0x4AE;
    16
}
/// $074C36: movem.l (a7)+, a0/a6
pub fn i_074c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C3A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074C3A: move.l #$74c46, (a0)
pub fn i_074c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C40;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74C46;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074C40: rts
pub fn i_074c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074C48: movem.l a0/a6, -(a7)
pub fn i_074c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C4C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074C4C: movem.l $74c62(pc), a1
pub fn i_074c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C52;
    cpu.ir = 0x4CFA;
    let mut a = 0x74C64;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074C52: jsr $77c7e.l
pub fn i_074c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74C58);
    cpu.pc = 0x77C7E;
    16
}
/// $074C58: movem.l (a7)+, a0/a6
pub fn i_074c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C5C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074C5C: move.l #$74c68, (a0)
pub fn i_074c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C62;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74C68;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074C62: rts
pub fn i_074c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074C6A: movem.l a0/a6, -(a7)
pub fn i_074c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C6E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074C6E: movem.l $74c84(pc), a1
pub fn i_074c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C74;
    cpu.ir = 0x4CFA;
    let mut a = 0x74C86;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074C74: jsr $77c7e.l
pub fn i_074c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74C7A);
    cpu.pc = 0x77C7E;
    16
}
/// $074C7A: movem.l (a7)+, a0/a6
pub fn i_074c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C7E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074C7E: move.l #$74c8a, (a0)
pub fn i_074c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C84;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74C8A;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074C84: rts
pub fn i_074c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074C8C: movem.l a0/a6, -(a7)
pub fn i_074c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C90;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074C90: movem.l $74ca6(pc), a1
pub fn i_074c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C96;
    cpu.ir = 0x4CFA;
    let mut a = 0x74CA8;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074C96: jsr $77c7e.l
pub fn i_074c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74C9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74C9C);
    cpu.pc = 0x77C7E;
    16
}
/// $074C9C: movem.l (a7)+, a0/a6
pub fn i_074c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CA0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074CA0: move.l #$74cac, (a0)
pub fn i_074ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CA6;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74CAC;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074CA6: rts
pub fn i_074ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074CAE: movem.l a0/a6, -(a7)
pub fn i_074cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CB2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074CB2: movem.l $74cc8(pc), a1
pub fn i_074cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CB8;
    cpu.ir = 0x4CFA;
    let mut a = 0x74CCA;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074CB8: jsr $77c7e.l
pub fn i_074cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74CBE);
    cpu.pc = 0x77C7E;
    16
}
/// $074CBE: movem.l (a7)+, a0/a6
pub fn i_074cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CC2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074CC2: move.l #$74cce, (a0)
pub fn i_074cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CC8;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74CCE;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074CC8: rts
pub fn i_074cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074CD0: movem.l a0/a6, -(a7)
pub fn i_074cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CD4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074CD4: movem.l $74cea(pc), a1
pub fn i_074cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CDA;
    cpu.ir = 0x4CFA;
    let mut a = 0x74CEC;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074CDA: jsr $77c7e.l
pub fn i_074cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74CE0);
    cpu.pc = 0x77C7E;
    16
}
/// $074CE0: movem.l (a7)+, a0/a6
pub fn i_074ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CE4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074CE4: move.l #$74cf0, (a0)
pub fn i_074ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CEA;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74CF0;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074CEA: rts
pub fn i_074cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074CF2: movem.l a0/a6, -(a7)
pub fn i_074cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CF6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074CF6: movem.l $74d0c(pc), a1
pub fn i_074cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74CFC;
    cpu.ir = 0x4CFA;
    let mut a = 0x74D0E;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074CFC: jsr $77c7e.l
pub fn i_074cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74D02);
    cpu.pc = 0x77C7E;
    16
}
/// $074D02: movem.l (a7)+, a0/a6
pub fn i_074d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D06;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074D06: move.l #$74d12, (a0)
pub fn i_074d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D0C;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74D12;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074D0C: rts
pub fn i_074d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074D14: movem.l a0/a6, -(a7)
pub fn i_074d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D18;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074D18: movem.l $74d2e(pc), a1
pub fn i_074d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D1E;
    cpu.ir = 0x4CFA;
    let mut a = 0x74D30;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074D1E: jsr $77c7e.l
pub fn i_074d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74D24);
    cpu.pc = 0x77C7E;
    16
}
/// $074D24: movem.l (a7)+, a0/a6
pub fn i_074d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D28;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074D28: move.l #$74d34, (a0)
pub fn i_074d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D2E;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74D34;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074D2E: rts
pub fn i_074d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074D3A: movem.l a0/a6, -(a7)
pub fn i_074d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D3E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074D3E: jsr $434ce.l
pub fn i_074d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74D44);
    cpu.pc = 0x434CE;
    16
}
/// $074D44: movem.l (a7)+, a0/a6
pub fn i_074d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D48;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074D48: move.l #$74d50, (a0)
pub fn i_074d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D4E;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74D50;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074D4E: rts
pub fn i_074d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074D58: movem.l a0/a6, -(a7)
pub fn i_074d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D5C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074D5C: movem.l $74d72(pc), a1
pub fn i_074d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D62;
    cpu.ir = 0x4CFA;
    let mut a = 0x74D74;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074D62: jsr $4ae.l
pub fn i_074d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74D68);
    cpu.pc = 0x4AE;
    16
}
/// $074D68: movem.l (a7)+, a0/a6
pub fn i_074d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D6C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074D6C: move.l #$74d78, (a0)
pub fn i_074d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D72;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74D78;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074D72: rts
pub fn i_074d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074D92: movem.l a0/a6, -(a7)
pub fn i_074d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D96;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074D96: movem.l $74dac(pc), a1
pub fn i_074d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74D9C;
    cpu.ir = 0x4CFA;
    let mut a = 0x74DAE;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074D9C: jsr $77c7e.l
pub fn i_074d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74DA2);
    cpu.pc = 0x77C7E;
    16
}
/// $074DA2: movem.l (a7)+, a0/a6
pub fn i_074da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DA6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074DA6: move.l #$74db2, (a0)
pub fn i_074da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DAC;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74DB2;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074DAC: rts
pub fn i_074dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074DB8: movem.l a0/a6, -(a7)
pub fn i_074db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DBC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074DBC: jsr $434ce.l
pub fn i_074dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74DC2);
    cpu.pc = 0x434CE;
    16
}
/// $074DC2: movem.l (a7)+, a0/a6
pub fn i_074dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DC6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074DC6: move.l #$74dce, (a0)
pub fn i_074dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DCC;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74DCE;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074DCC: rts
pub fn i_074dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074DD6: movem.l a0/a6, -(a7)
pub fn i_074dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DDA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074DDA: movem.l $74df0(pc), a1
pub fn i_074dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DE0;
    cpu.ir = 0x4CFA;
    let mut a = 0x74DF2;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074DE0: jsr $4ae.l
pub fn i_074de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74DE6);
    cpu.pc = 0x4AE;
    16
}
/// $074DE6: movem.l (a7)+, a0/a6
pub fn i_074de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DEA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074DEA: move.l #$74df6, (a0)
pub fn i_074dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DF0;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74DF6;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074DF0: rts
pub fn i_074df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74DF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074DFC: movem.l a0/a6, -(a7)
pub fn i_074dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E00;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074E00: movem.l $74e14(pc), a2
pub fn i_074e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E06;
    cpu.ir = 0x4CFA;
    let mut a = 0x74E16;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074E06: jsr $750d4(pc)
pub fn i_074e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x74E0A);
    cpu.pc = 0x750D4;
    16
}
/// $074E0A: movem.l (a7)+, a0/a6
pub fn i_074e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E0E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074E0E: move.l #$74e1a, (a0)
pub fn i_074e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E14;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74E1A;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074E14: rts
pub fn i_074e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074E1C: movem.l a0/a6, -(a7)
pub fn i_074e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E20;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074E20: movem.l $74e36(pc), a2
pub fn i_074e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E26;
    cpu.ir = 0x4CFA;
    let mut a = 0x74E38;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074E26: jsr $5022a.l
pub fn i_074e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74E2C);
    cpu.pc = 0x5022A;
    16
}
/// $074E2C: movem.l (a7)+, a0/a6
pub fn i_074e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E30;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074E30: move.l #$74e3c, (a0)
pub fn i_074e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E36;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74E3C;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074E36: rts
pub fn i_074e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074E44: movem.l a0/a6, -(a7)
pub fn i_074e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E48;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074E48: movem.l $74e5e(pc), a1
pub fn i_074e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E4E;
    cpu.ir = 0x4CFA;
    let mut a = 0x74E60;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074E4E: jsr $77c7e.l
pub fn i_074e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74E54);
    cpu.pc = 0x77C7E;
    16
}
/// $074E54: movem.l (a7)+, a0/a6
pub fn i_074e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E58;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074E58: move.l #$74e64, (a0)
pub fn i_074e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E5E;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74E64;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074E5E: rts
pub fn i_074e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074E74: movem.l a0/a6, -(a7)
pub fn i_074e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E78;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074E78: jsr $750a4(pc)
pub fn i_074e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x74E7C);
    cpu.pc = 0x750A4;
    16
}
/// $074E7C: movem.l (a7)+, a0/a6
pub fn i_074e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E80;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074E80: move.l #$74e88, (a0)
pub fn i_074e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E86;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74E88;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074E86: rts
pub fn i_074e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074E8A: movem.l a0/a6, -(a7)
pub fn i_074e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E8E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074E8E: movem.l $74ea2(pc), a2
pub fn i_074e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E94;
    cpu.ir = 0x4CFA;
    let mut a = 0x74EA4;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074E94: jsr $750d4(pc)
pub fn i_074e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x74E98);
    cpu.pc = 0x750D4;
    16
}
/// $074E98: movem.l (a7)+, a0/a6
pub fn i_074e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E9C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074E9C: move.l #$74ea8, (a0)
pub fn i_074e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EA2;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74EA8;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074EA2: rts
pub fn i_074ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074EAA: movem.l a0/a6, -(a7)
pub fn i_074eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EAE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074EAE: movem.l $74ec4(pc), a2
pub fn i_074eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EB4;
    cpu.ir = 0x4CFA;
    let mut a = 0x74EC6;
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074EB4: jsr $5022a.l
pub fn i_074eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74EBA);
    cpu.pc = 0x5022A;
    16
}
/// $074EBA: movem.l (a7)+, a0/a6
pub fn i_074eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EBE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074EBE: move.l #$74eca, (a0)
pub fn i_074ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EC4;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74ECA;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074EC4: rts
pub fn i_074ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074ED4: movem.l a0/a6, -(a7)
pub fn i_074ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74ED8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074ED8: movem.l $74eee(pc), d0/a1
pub fn i_074ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EDE;
    cpu.ir = 0x4CFA;
    let mut a = 0x74EF0;
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074EDE: jsr $4429e.l
pub fn i_074ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74EE4);
    cpu.pc = 0x4429E;
    16
}
/// $074EE4: movem.l (a7)+, a0/a6
pub fn i_074ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EE8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074EE8: move.l #$74ef8, (a0)
pub fn i_074ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EEE;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74EF8;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074EEE: rts
pub fn i_074eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074EFA: movem.l a0/a6, -(a7)
pub fn i_074efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74EFE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074EFE: jsr $434ce.l
pub fn i_074efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74F04);
    cpu.pc = 0x434CE;
    16
}
/// $074F04: movem.l (a7)+, a0/a6
pub fn i_074f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F08;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074F08: move.l #$74f10, (a0)
pub fn i_074f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F0E;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74F10;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074F0E: rts
pub fn i_074f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074F18: movem.l a0/a6, -(a7)
pub fn i_074f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F1C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074F1C: movem.l $74f32(pc), a1
pub fn i_074f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F22;
    cpu.ir = 0x4CFA;
    let mut a = 0x74F34;
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $074F22: jsr $4ae.l
pub fn i_074f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x74F28);
    cpu.pc = 0x4AE;
    16
}
/// $074F28: movem.l (a7)+, a0/a6
pub fn i_074f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F2C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074F2C: move.l #$74f38, (a0)
pub fn i_074f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F32;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74F38;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074F32: rts
pub fn i_074f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $074F44: movem.l a0/a6, -(a7)
pub fn i_074f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F48;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $074F48: jsr $750e2(pc)
pub fn i_074f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x74F4C);
    cpu.pc = 0x750E2;
    16
}
/// $074F4C: movem.l (a7)+, a0/a6
pub fn i_074f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F50;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $074F50: move.l #$74f58, (a0)
pub fn i_074f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F56;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x74F58;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $074F56: rts
pub fn i_074f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74F58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_WarpOriginAndRunScript_0750a4  ($0750A4, 28 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0750A4: clr.w d0
pub fn i_0750a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750A6;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0750A6: clr.w d1
pub fn i_0750a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750A8;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0750A8: jsr $44022.l
pub fn i_0750a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x750AE);
    cpu.pc = 0x44022;
    16
}
/// $0750AE: move.w d0, $22(a6)
pub fn i_0750ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750B2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0750B2: move.w d1, $24(a6)
pub fn i_0750b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750B6;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0750B6: lea.l $74f5a(pc), a1
pub fn i_0750b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x74F5A;
    16
}
/// $0750BA: jmp $43fac.l
pub fn i_0750ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750C0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x43FAC;
    8
}
// ===== ScriptedProp_ClampLocalX_0750c0  ($0750C0, 20 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0750C0: jsr $440d0.l
pub fn i_0750c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x750C6);
    cpu.pc = 0x440D0;
    16
}
/// $0750C6: cmpi.w #$180, d0
pub fn i_0750c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750CA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x180, Size::Word, false);
    8
}
/// $0750CA: bcs.w $750d2
pub fn i_0750ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750CE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x750D2; return 10; }
    8
}
/// $0750CE: move.w #$180, d0
pub fn i_0750ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0750D2: rts
pub fn i_0750d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_BlitParams_0750d4  ($0750D4, 14 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0750D4: movem.l (a2)+, a0
pub fn i_0750d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750D8;
    cpu.ir = 0x4CDA;
    let mut a = cpu.a[2];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = a;
    20
}
/// $0750D8: movem.w (a2)+, d0-d1/d4-d5
pub fn i_0750d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750DC;
    cpu.ir = 0x4C9A;
    let mut a = cpu.a[2];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[2] = a;
    20
}
/// $0750DC: jmp $51e74.l
pub fn i_0750dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750E2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x51E74;
    8
}
// ===== ClrRamWord_0750e2  ($0750E2, 8 B, thunks/clr_ram_word.c)
/// $0750E2: clr.b $106ed2.l
pub fn i_0750e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750E8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $0750E8: rts
pub fn i_0750e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_SnapToAnchor_0750ea  ($0750EA, 22 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0750EA: move.w #$fc0, d0
pub fn i_0750ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0750EE: move.w #$e0, d1
pub fn i_0750ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0750F2: jsr $750c0(pc)
pub fn i_0750f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x750F6);
    cpu.pc = 0x750C0;
    16
}
/// $0750F6: move.w d0, $22(a6)
pub fn i_0750f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750FA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0750FA: move.w d1, $24(a6)
pub fn i_0750fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x750FE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0750FE: rts
pub fn i_0750fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75100;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Tmpl97_075100  ($075100, 158 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $075100: lea.l $2bf004.l, a0
pub fn i_075100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75106;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF004;
    20
}
/// $075106: jsr $799de.l
pub fn i_075106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7510C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7510C);
    cpu.pc = 0x799DE;
    16
}
/// $07510C: move.w d0, $66(a6)
pub fn i_07510c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75110;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075110: move.w d0, $72(a6)
pub fn i_075110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75114;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075114: lsr.w #$2, d0
pub fn i_075114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75116;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $075116: move.w d0, $7e(a6)
pub fn i_075116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7511A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07511A: clr.b $70(a6)
pub fn i_07511a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7511E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07511E: clr.b $76(a6)
pub fn i_07511e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75122;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $075122: clr.b $78(a6)
pub fn i_075122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75126;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $075126: clr.b $77(a6)
pub fn i_075126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7512A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07512A: clr.b $79(a6)
pub fn i_07512a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7512E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x79);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07512E: clr.b $7a(a6)
pub fn i_07512e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75132;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $075132: clr.b $71(a6)
pub fn i_075132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75136;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x71);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $075136: clr.b $7c(a6)
pub fn i_075136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7513A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07513A: lea.l $7690a(pc), a1
pub fn i_07513a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7513E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7690A;
    16
}
/// $07513E: jsr $4ae.l
pub fn i_07513e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75144;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75144);
    cpu.pc = 0x4AE;
    16
}
/// $075144: lea.l $76012(pc), a1
pub fn i_075144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75148;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76012;
    16
}
/// $075148: jsr $4ae.l
pub fn i_075148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7514E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7514E);
    cpu.pc = 0x4AE;
    16
}
/// $07514E: lea.l $76ade(pc), a1
pub fn i_07514e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75152;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76ADE;
    16
}
/// $075152: jsr $4ae.l
pub fn i_075152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75158;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75158);
    cpu.pc = 0x4AE;
    16
}
/// $075158: lea.l $76a2a(pc), a1
pub fn i_075158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7515C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76A2A;
    16
}
/// $07515C: jsr $4ae.l
pub fn i_07515c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75162);
    cpu.pc = 0x4AE;
    16
}
/// $075162: lea.l $76be8(pc), a1
pub fn i_075162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75166;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76BE8;
    16
}
/// $075166: jsr $4ae.l
pub fn i_075166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7516C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7516C);
    cpu.pc = 0x4AE;
    16
}
/// $07516C: move.w #$c000, $38(a6)
pub fn i_07516c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75172;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075172: move.b #$96, $7d(a6)
pub fn i_075172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75178;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $075178: lea.l $80(a6), a0
pub fn i_075178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7517C;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    cpu.a[0] = ad1;
    16
}
/// $07517C: lea.l $74f58(pc), a1
pub fn i_07517c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75180;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x74F58;
    16
}
/// $075180: jsr $745da(pc)
pub fn i_075180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75184;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x75184);
    cpu.pc = 0x745DA;
    16
}
/// $075184: move.l #$74712, $60(a6)
pub fn i_075184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7518C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x74712;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07518C: lea.l $746be(pc), a0
pub fn i_07518c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75190;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x746BE;
    16
}
/// $075190: move.l a0, $48(a6)
pub fn i_075190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75194;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $075194: lea.l $75270(pc), a0
pub fn i_075194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75198;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75270;
    16
}
/// $075198: jsr $28cd4.l
pub fn i_075198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7519E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7519E);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_07519e  ($07519E, 8 B, kernel/task_handlers.c)
/// $07519E: lea.l $751a6(pc), a1
pub fn i_07519e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x751A6;
    16
}
/// $0751A2: move.l a1, (a6)
pub fn i_0751a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0751A4: rts
pub fn i_0751a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Run_0751a6  ($0751A6, 174 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $0751A6: jsr $2870a.l
pub fn i_0751a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x751AC);
    cpu.pc = 0x2870A;
    16
}
/// $0751AC: bcs.w $751c8
pub fn i_0751ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751B0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x751C8; return 10; }
    8
}
/// $0751B0: cmpi.w #$ea8, $106f50.l
pub fn i_0751b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751B8;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xEA8, Size::Word, false);
    20
}
/// $0751B8: blt.w $751ec
pub fn i_0751b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751BC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x751EC; return 10; }
    8
}
/// $0751BC: subq.b #$1, $7d(a6)
pub fn i_0751bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751C0;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0751C0: tst.b $7d(a6)
pub fn i_0751c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751C4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0751C4: bne.w $751ec
pub fn i_0751c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x751EC; return 10; }
    8
}
/// $0751C8: lea.l $7466e(pc), a0
pub fn i_0751c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751CC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7466E;
    16
}
/// $0751CC: move.l a0, $48(a6)
pub fn i_0751cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751D0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0751D0: move.w #$28, d0
pub fn i_0751d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0751D4: jsr $2352.l
pub fn i_0751d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x751DA);
    cpu.pc = 0x2352;
    16
}
/// $0751DA: lea.l $80(a6), a0
pub fn i_0751da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751DE;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    cpu.a[0] = ad1;
    16
}
/// $0751DE: lea.l $749e8(pc), a1
pub fn i_0751de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x749E8;
    16
}
/// $0751E2: jsr $745da(pc)
pub fn i_0751e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x751E6);
    cpu.pc = 0x745DA;
    16
}
/// $0751E6: lea.l $751ec(pc), a1
pub fn i_0751e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x751EC;
    16
}
/// $0751EA: move.l a1, (a6)
pub fn i_0751ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0751EC: bsr.w $750ea
pub fn i_0751ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751F0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x751F0);
    cpu.pc = 0x750EA;
    18
}
/// $0751F0: jsr $28998.l
pub fn i_0751f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x751F6);
    cpu.pc = 0x28998;
    16
}
/// $0751F6: move.w $22(a6), d0
pub fn i_0751f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0751FA: cmpi.w #$12c, d0
pub fn i_0751fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x751FE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x12C, Size::Word, false);
    8
}
/// $0751FE: bcs.w $75208
pub fn i_0751fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75202;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x75208; return 10; }
    8
}
/// $075202: move.b #$2, $45(a6)
pub fn i_075202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75208;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $075208: moveq #$0, d0
pub fn i_075208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7520A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07520A: move.w $66(a6), d0
pub fn i_07520a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7520E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07520E: lsl.l #$8, d0
pub fn i_07520e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75210;
    cpu.ir = 0xE188;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[0] = r;
    22
}
/// $075210: divu.w $72(a6), d0
pub fn i_075210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75214;
    cpu.ir = 0x80EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v3 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 84; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    148
}
/// $075214: move.w d0, $74(a6)
pub fn i_075214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75218;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075218: lea.l $80(a6), a0
pub fn i_075218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7521C;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    cpu.a[0] = ad1;
    16
}
/// $07521C: jsr $745e2(pc)
pub fn i_07521c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75220;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x75220);
    cpu.pc = 0x745E2;
    16
}
/// $075220: jsr $2870a.l
pub fn i_075220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75226);
    cpu.pc = 0x2870A;
    16
}
/// $075226: scs.b $7b(a6)
pub fn i_075226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7522A;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $07522A: bcc.w $7523a
pub fn i_07522a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7522E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7523A; return 10; }
    8
}
/// $07522E: bclr.b #$3, $13(a6)
pub fn i_07522e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75234;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $075234: jsr $77d88.l
pub fn i_075234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7523A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7523A);
    cpu.pc = 0x77D88;
    16
}
/// $07523A: jsr $28d70.l
pub fn i_07523a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75240);
    cpu.pc = 0x28D70;
    16
}
/// $075240: jsr $28758.l
pub fn i_075240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75246);
    cpu.pc = 0x28758;
    16
}
/// $075246: bcc.w $7525a
pub fn i_075246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7524A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7525A; return 10; }
    8
}
/// $07524A: clr.b $106ed3.l
pub fn i_07524a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75250;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED3) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED3, r2 as u8);
    16
}
/// $075250: clr.b $7b(a6)
pub fn i_075250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75254;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetTaskHandler_075254  ($075254, 8 B, kernel/task_handlers.c)
/// $075254: lea.l $7525c(pc), a1
pub fn i_075254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75258;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7525C;
    16
}
/// $075258: move.l a1, (a6)
pub fn i_075258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7525A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07525A: rts
pub fn i_07525a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7525C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Dead_07525c  ($07525C, 14 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $07525C: lea.l $75262(pc), a1
pub fn i_07525c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75260;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x75262;
    16
}
/// $075260: move.l a1, (a6)
pub fn i_075260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75262;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $075262: bsr.w $750ea
pub fn i_075262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75266;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x75266);
    cpu.pc = 0x750EA;
    18
}
/// $075266: lea.l $80(a6), a0
pub fn i_075266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7526A;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    cpu.a[0] = ad1;
    16
}
// ===== JsrPcThunk_07526a  ($07526A, 6 B, thunks/jsr_pc_thunks.c)
/// $07526A: jsr $745e2(pc)
pub fn i_07526a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7526E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7526E);
    cpu.pc = 0x745E2;
    16
}
/// $07526E: rts
pub fn i_07526e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $075EE8: move.l a6, -(a7)
pub fn i_075ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75EEA;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $075EEA: lea.l $100800.l, a6
pub fn i_075eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75EF0;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $075EF0: lea.l $76e10(pc), a1
pub fn i_075ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75EF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76E10;
    16
}
/// $075EF4: jsr $4ae.l
pub fn i_075ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75EFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75EFA);
    cpu.pc = 0x4AE;
    16
}
/// $075EFA: movea.l $c(a6), a1
pub fn i_075efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75EFE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $075EFE: move.w $7e(a1), d0
pub fn i_075efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F02;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $075F02: cmp.w $66(a1), d0
pub fn i_075f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F06;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $075F06: blt.w $75f12
pub fn i_075f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F0A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x75F12; return 10; }
    8
}
/// $075F0A: lea.l $75f3c(pc), a1
pub fn i_075f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x75F3C;
    16
}
/// $075F0E: bra.w $75f16
pub fn i_075f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F12;
    cpu.ir = 0x6000;
    cpu.pc = 0x75F16;
    10
}
/// $075F12: lea.l $75f34(pc), a1
pub fn i_075f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x75F34;
    16
}
/// $075F16: move.l a1, $7c(a0)
pub fn i_075f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F1A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $075F1A: movea.l (a7)+, a6
pub fn i_075f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F1C;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $075F1C: move.w $22(a6), d0
pub fn i_075f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F20;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $075F20: subq.w #$8, d0
pub fn i_075f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F22;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $075F22: move.w d0, $22(a0)
pub fn i_075f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F26;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075F26: move.w $24(a6), d0
pub fn i_075f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F2A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $075F2A: add.w $78(a6), d0
pub fn i_075f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F2E;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $075F2E: move.w d0, $24(a0)
pub fn i_075f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F32;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075F32: rts
pub fn i_075f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $075F44: move.w #$fe0, d0
pub fn i_075f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F48;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $075F48: move.w #$a0, d1
pub fn i_075f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $075F4C: jsr $750c0(pc)
pub fn i_075f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x75F50);
    cpu.pc = 0x750C0;
    16
}
/// $075F50: move.w d0, $22(a6)
pub fn i_075f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F54;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075F54: move.w d1, $24(a6)
pub fn i_075f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F58;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075F58: move.w $70(a6), d0
pub fn i_075f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F5C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $075F5C: beq.w $75f66
pub fn i_075f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x75F66; return 10; }
    8
}
/// $075F60: subq.w #$1, d0
pub fn i_075f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F62;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $075F62: move.w d0, $70(a6)
pub fn i_075f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F66;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075F66: movea.l $c(a6), a0
pub fn i_075f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F6A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $075F6A: tst.b $7b(a0)
pub fn i_075f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F6E;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $075F6E: bne.w $75f96
pub fn i_075f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x75F96; return 10; }
    8
}
/// $075F72: btst.b #$3, $13(a6)
pub fn i_075f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F78;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $075F78: beq.w $75f8c
pub fn i_075f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F7C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x75F8C; return 10; }
    8
}
/// $075F7C: lea.l $7475e(pc), a0
pub fn i_075f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F80;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7475E;
    16
}
/// $075F80: jsr $2b58.l
pub fn i_075f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75F86);
    cpu.pc = 0x2B58;
    16
}
/// $075F86: bclr.b #$3, $13(a6)
pub fn i_075f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F8C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $075F8C: andi.b #$7e, $5a(a6)
pub fn i_075f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F92;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x7E) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $075F92: bra.w $75fb6
pub fn i_075f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F96;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FB6;
    10
}
/// $075F96: btst.b #$3, $13(a6)
pub fn i_075f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75F9C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $075F9C: bne.w $75fb0
pub fn i_075f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x75FB0; return 10; }
    8
}
/// $075FA0: lea.l $74784(pc), a0
pub fn i_075fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x74784;
    16
}
/// $075FA4: jsr $2b58.l
pub fn i_075fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75FAA);
    cpu.pc = 0x2B58;
    16
}
/// $075FAA: bset.b #$3, $13(a6)
pub fn i_075faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FB0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $075FB0: ori.b #$81, $5a(a6)
pub fn i_075fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FB6;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x81) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $075FB6: btst.b #$6, $5b(a0)
pub fn i_075fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FBC;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $075FBC: beq.w $75fc6
pub fn i_075fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x75FC6; return 10; }
    8
}
/// $075FC0: bset.b #$6, $5b(a6)
pub fn i_075fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FC6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $075FC6: rts
pub fn i_075fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Child_Timer_075fc8  ($075FC8, 46 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $075FC8: move.w $72(a6), d0
pub fn i_075fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FCC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $075FCC: beq.w $75fea
pub fn i_075fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x75FEA; return 10; }
    8
}
/// $075FD0: subq.w #$1, d0
pub fn i_075fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FD2;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $075FD2: move.w d0, $72(a6)
pub fn i_075fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FD6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075FD6: lea.l $2bf108.l, a0
pub fn i_075fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF108;
    20
}
/// $075FDC: jsr $799de.l
pub fn i_075fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75FE2);
    cpu.pc = 0x799DE;
    16
}
/// $075FE2: move.w d0, $70(a6)
pub fn i_075fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FE6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075FE6: bra.w $75ffa
pub fn i_075fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFA;
    10
}
/// $075FEA: lea.l $2bf086.l, a0
pub fn i_075fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF086;
    20
}
/// $075FF0: jsr $799de.l
pub fn i_075ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x75FF6);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_075ff6  ($075FF6, 6 B, kernel/task_setters_w.c)
/// $075FF6: move.w d0, $70(a6)
pub fn i_075ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FFA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $075FFA: rts
pub fn i_075ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75FFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Child_Parent_075ffc  ($075FFC, 4 B, enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s)
/// $075FFC: movea.l $c(a6), a0
pub fn i_075ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76000;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
// ===== ScriptedProp_Child_ParentDeadGate_076000  ($076000, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $076000: btst.b #$0, $13(a0)
pub fn i_076000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76006;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $076006: beq.w $76010
pub fn i_076006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7600A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76010; return 10; }
    8
}
// ===== SetTaskHandler_07600a  ($07600A, 8 B, kernel/task_handlers.c)
/// $07600A: lea.l $7646e(pc), a1
pub fn i_07600a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7600E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7646E;
    16
}
/// $07600E: move.l a1, (a6)
pub fn i_07600e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76010;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076010: rts
pub fn i_076010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76012;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_SpawnFlagAndInit_076012  ($076012, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $076012: lea.l $764b0(pc), a1
pub fn i_076012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76016;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x764B0;
    16
}
/// $076016: jsr $4ae.l
pub fn i_076016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7601C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7601C);
    cpu.pc = 0x4AE;
    16
}
/// $07601C: move.w #$54, d1
pub fn i_07601c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76020;
    cpu.ir = 0x323C;
    let v1: u32 = 0x54;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076020: jsr $236e.l
pub fn i_076020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76026);
    cpu.pc = 0x236E;
    16
}
/// $076026: move.w #$1d4, d1
pub fn i_076026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7602A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07602A: jsr $236e.l
pub fn i_07602a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76030;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76030);
    cpu.pc = 0x236E;
    16
}
/// $076030: move.w #$12, $1c(a6)
pub fn i_076030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76036;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076036: jsr $138fe.l
pub fn i_076036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7603C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7603C);
    cpu.pc = 0x138FE;
    16
}
/// $07603C: move.w #$2000, $38(a6)
pub fn i_07603c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76042;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076042: clr.w $72(a6)
pub fn i_076042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76046;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $076046: clr.w $70(a6)
pub fn i_076046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7604A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07604A: clr.b $84(a6)
pub fn i_07604a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7604E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetTaskHandler_07604e  ($07604E, 8 B, kernel/task_handlers.c)
/// $07604E: lea.l $76056(pc), a1
pub fn i_07604e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76052;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76056;
    16
}
/// $076052: move.l a1, (a6)
pub fn i_076052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76054;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076054: rts
pub fn i_076054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76056;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Wall_Idle_076056  ($076056, 74 B, props/fx_explosions_platforms_crew_076000.s)
/// $076056: move.w $16(a6), $14(a6)
pub fn i_076056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7605C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07605C: lea.l $7527c(pc), a0
pub fn i_07605c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76060;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7527C;
    16
}
/// $076060: jsr $28cd4.l
pub fn i_076060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76066;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76066);
    cpu.pc = 0x28CD4;
    16
}
/// $076066: lea.l $7606c(pc), a1
pub fn i_076066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7606A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7606C;
    16
}
/// $07606A: move.l a1, (a6)
pub fn i_07606a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7606C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07606C: bsr.w $75f44
pub fn i_07606c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76070;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76070);
    cpu.pc = 0x75F44;
    18
}
/// $076070: jsr $28d70.l
pub fn i_076070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76076;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76076);
    cpu.pc = 0x28D70;
    16
}
/// $076076: movea.l $c(a6), a0
pub fn i_076076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7607A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07607A: move.b $11(a0), $11(a6)
pub fn i_07607a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76080;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x11);
    bus.write8(ad4, v3 as u8);
    20
}
/// $076080: tst.b $70(a0)
pub fn i_076080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76084;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076084: beq.w $760a6
pub fn i_076084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76088;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x760A6; return 10; }
    8
}
/// $076088: move.l a6, -(a7)
pub fn i_076088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7608A;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07608A: movea.l $c(a6), a6
pub fn i_07608a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7608E;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $07608E: lea.l $76ba6(pc), a1
pub fn i_07608e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76092;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76BA6;
    16
}
/// $076092: jsr $4ae.l
pub fn i_076092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76098);
    cpu.pc = 0x4AE;
    16
}
/// $076098: movea.l (a7)+, a6
pub fn i_076098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7609A;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $07609A: move.w #$3c, $70(a6)
pub fn i_07609a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_0760a0  ($0760A0, 8 B, kernel/task_handlers.c)
/// $0760A0: lea.l $760a8(pc), a1
pub fn i_0760a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x760A8;
    16
}
/// $0760A4: move.l a1, (a6)
pub fn i_0760a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0760A6: rts
pub fn i_0760a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Wall_WaitTimer_0760a8  ($0760A8, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $0760A8: move.w $16(a6), $14(a6)
pub fn i_0760a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760AE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0760AE: bsr.w $75f44
pub fn i_0760ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760B2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x760B2);
    cpu.pc = 0x75F44;
    18
}
/// $0760B2: jsr $28d70.l
pub fn i_0760b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x760B8);
    cpu.pc = 0x28D70;
    16
}
/// $0760B8: tst.w $70(a6)
pub fn i_0760b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760BC;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0760BC: bne.w $760c6
pub fn i_0760bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x760C6; return 10; }
    8
}
// ===== SetTaskHandler_0760c0  ($0760C0, 8 B, kernel/task_handlers.c)
/// $0760C0: lea.l $760c8(pc), a1
pub fn i_0760c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x760C8;
    16
}
/// $0760C4: move.l a1, (a6)
pub fn i_0760c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0760C6: rts
pub fn i_0760c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Wall_Alt_0760c8  ($0760C8, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $0760C8: move.w $16(a6), $14(a6)
pub fn i_0760c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760CE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0760CE: lea.l $75300(pc), a0
pub fn i_0760ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760D2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75300;
    16
}
/// $0760D2: jsr $28cd4.l
pub fn i_0760d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x760D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0760D8: lea.l $760de(pc), a1
pub fn i_0760d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x760DE;
    16
}
/// $0760DC: move.l a1, (a6)
pub fn i_0760dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0760DE: bsr.w $75f44
pub fn i_0760de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x760E2);
    cpu.pc = 0x75F44;
    18
}
/// $0760E2: jsr $28d70.l
pub fn i_0760e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x760E8);
    cpu.pc = 0x28D70;
    16
}
/// $0760E8: bcc.w $760f2
pub fn i_0760e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x760F2; return 10; }
    8
}
// ===== SetTaskHandler_0760ec  ($0760EC, 8 B, kernel/task_handlers.c)
/// $0760EC: lea.l $76290(pc), a1
pub fn i_0760ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76290;
    16
}
/// $0760F0: move.l a1, (a6)
pub fn i_0760f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0760F2: rts
pub fn i_0760f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Child_RandDelay_0760f4  ($0760F4, 12 B, props/fx_explosions_platforms_crew_076000.s)
/// $0760F4: lea.l $2bf392.l, a0
pub fn i_0760f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x760FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF392;
    20
}
/// $0760FA: jsr $799de.l
pub fn i_0760fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76100);
    cpu.pc = 0x799DE;
    16
}
// ===== ScriptedProp_Child_DelayThenResume_076100  ($076100, 56 B, props/fx_explosions_platforms_crew_076000.s)
/// $076100: move.w d0, $70(a6)
pub fn i_076100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76104;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076104: move.w $16(a6), $14(a6)
pub fn i_076104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7610A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07610A: lea.l $76110(pc), a1
pub fn i_07610a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7610E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76110;
    16
}
/// $07610E: move.l a1, (a6)
pub fn i_07610e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76110;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076110: bsr.w $75f44
pub fn i_076110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76114;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76114);
    cpu.pc = 0x75F44;
    18
}
/// $076114: jsr $28d70.l
pub fn i_076114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7611A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7611A);
    cpu.pc = 0x28D70;
    16
}
/// $07611A: subq.w #$1, $70(a6)
pub fn i_07611a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7611E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07611E: cmpi.w #$0, $70(a6)
pub fn i_07611e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76124;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $076124: bgt.w $76132
pub fn i_076124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76128;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x76132; return 10; }
    8
}
/// $076128: move.b #$1, $84(a6)
pub fn i_076128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7612E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07612E: move.l $80(a6), (a6)
pub fn i_07612e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76132;
    cpu.ir = 0x2CAE;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $076132: bra.w $75ffc
pub fn i_076132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76136;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_Tower_Idle_076138  ($076138, 98 B, props/fx_explosions_platforms_crew_076000.s)
/// $076138: move.w $16(a6), $14(a6)
pub fn i_076138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7613E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07613E: movea.l $c(a6), a1
pub fn i_07613e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76142;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $076142: move.w $7e(a1), d0
pub fn i_076142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76146;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076146: cmp.w $66(a1), d0
pub fn i_076146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7614A;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07614A: bge.w $7615c
pub fn i_07614a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7614E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7615C; return 10; }
    8
}
/// $07614E: lea.l $753a2(pc), a0
pub fn i_07614e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76152;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x753A2;
    16
}
/// $076152: jsr $28cd4.l
pub fn i_076152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76158;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76158);
    cpu.pc = 0x28CD4;
    16
}
/// $076158: bra.w $76174
pub fn i_076158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7615C;
    cpu.ir = 0x6000;
    cpu.pc = 0x76174;
    10
}
/// $07615C: move.l #$76138, $80(a6)
pub fn i_07615c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76164;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x76138;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076164: tst.b $84(a6)
pub fn i_076164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76168;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076168: beq.b $760f4
pub fn i_076168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7616A;
    cpu.ir = 0x678A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x760F4; return 10; }
    8
}
/// $07616A: lea.l $75466(pc), a0
pub fn i_07616a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7616E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75466;
    16
}
/// $07616E: jsr $28cd4.l
pub fn i_07616e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76174);
    cpu.pc = 0x28CD4;
    16
}
/// $076174: lea.l $7617a(pc), a1
pub fn i_076174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76178;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7617A;
    16
}
/// $076178: move.l a1, (a6)
pub fn i_076178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7617A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07617A: bsr.w $75f44
pub fn i_07617a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7617E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x7617E);
    cpu.pc = 0x75F44;
    18
}
/// $07617E: jsr $28d70.l
pub fn i_07617e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76184);
    cpu.pc = 0x28D70;
    16
}
/// $076184: bcc.w $76196
pub fn i_076184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76188;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76196; return 10; }
    8
}
/// $076188: tst.w $70(a6)
pub fn i_076188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7618C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07618C: bne.w $76196
pub fn i_07618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76190;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76196; return 10; }
    8
}
/// $076190: lea.l $7619a(pc), a1
pub fn i_076190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76194;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7619A;
    16
}
/// $076194: move.l a1, (a6)
pub fn i_076194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76196;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076196: bra.w $75ffc
pub fn i_076196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7619A;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_Tower_Aim_07619a  ($07619A, 76 B, props/fx_explosions_platforms_crew_076000.s)
/// $07619A: move.w $16(a6), $14(a6)
pub fn i_07619a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761A0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0761A0: movea.l $c(a6), a0
pub fn i_0761a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761A4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0761A4: move.w $7e(a0), d0
pub fn i_0761a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761A8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0761A8: cmp.w $66(a0), d0
pub fn i_0761a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761AC;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0761AC: bge.w $761b8
pub fn i_0761ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761B0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x761B8; return 10; }
    8
}
/// $0761B0: lea.l $75f3c(pc), a0
pub fn i_0761b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761B4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75F3C;
    16
}
/// $0761B4: bra.w $761cc
pub fn i_0761b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x761CC;
    10
}
/// $0761B8: move.l #$7619a, $80(a6)
pub fn i_0761b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761C0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7619A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0761C0: tst.b $84(a6)
pub fn i_0761c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761C4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0761C4: beq.w $760f4
pub fn i_0761c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x760F4; return 10; }
    8
}
/// $0761C8: lea.l $75f34(pc), a0
pub fn i_0761c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761CC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75F34;
    16
}
/// $0761CC: jsr $5e086.l
pub fn i_0761cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x761D2);
    cpu.pc = 0x5E086;
    16
}
/// $0761D2: bcc.w $763c8
pub fn i_0761d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x763C8; return 10; }
    8
}
/// $0761D6: lea.l $2bf20c.l, a0
pub fn i_0761d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF20C;
    20
}
/// $0761DC: jsr $799de.l
pub fn i_0761dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x761E2);
    cpu.pc = 0x799DE;
    16
}
/// $0761E2: move.w d0, $72(a6)
pub fn i_0761e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== ScriptedProp_Tower_Fire_0761e6  ($0761E6, 70 B, props/fx_explosions_platforms_crew_076000.s)
/// $0761E6: lea.l $75a82(pc), a0
pub fn i_0761e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761EA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75A82;
    16
}
/// $0761EA: jsr $28cd4.l
pub fn i_0761ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x761F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0761F0: bsr.w $75fc8
pub fn i_0761f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x761F4);
    cpu.pc = 0x75FC8;
    18
}
/// $0761F4: lea.l $761fa(pc), a1
pub fn i_0761f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x761FA;
    16
}
/// $0761F8: move.l a1, (a6)
pub fn i_0761f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0761FA: bsr.w $75f44
pub fn i_0761fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761FE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x761FE);
    cpu.pc = 0x75F44;
    18
}
/// $0761FE: jsr $28d70.l
pub fn i_0761fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76204);
    cpu.pc = 0x28D70;
    16
}
/// $076204: bcc.w $76228
pub fn i_076204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76208;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76228; return 10; }
    8
}
/// $076208: tst.w $72(a6)
pub fn i_076208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7620C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07620C: beq.w $76222
pub fn i_07620c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76210;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76222; return 10; }
    8
}
/// $076210: tst.w $70(a6)
pub fn i_076210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76214;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $076214: bne.w $7621e
pub fn i_076214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76218;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7621E; return 10; }
    8
}
/// $076218: lea.l $761e6(pc), a1
pub fn i_076218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7621C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x761E6;
    16
}
/// $07621C: move.l a1, (a6)
pub fn i_07621c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7621E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07621E: bra.w $76228
pub fn i_07621e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76222;
    cpu.ir = 0x6000;
    cpu.pc = 0x76228;
    10
}
/// $076222: lea.l $7622c(pc), a1
pub fn i_076222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76226;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7622C;
    16
}
/// $076226: move.l a1, (a6)
pub fn i_076226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76228;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076228: bra.w $75ffc
pub fn i_076228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7622C;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_TowerB_Idle_07622c  ($07622C, 100 B, props/fx_explosions_platforms_crew_076000.s)
/// $07622C: move.w $16(a6), $14(a6)
pub fn i_07622c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76232;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076232: movea.l $c(a6), a1
pub fn i_076232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76236;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $076236: move.w $7e(a1), d0
pub fn i_076236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7623A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07623A: cmp.w $66(a1), d0
pub fn i_07623a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7623E;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07623E: bge.w $76250
pub fn i_07623e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76242;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x76250; return 10; }
    8
}
/// $076242: lea.l $75288(pc), a0
pub fn i_076242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76246;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75288;
    16
}
/// $076246: jsr $28cd4.l
pub fn i_076246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7624C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7624C);
    cpu.pc = 0x28CD4;
    16
}
/// $07624C: bra.w $7626a
pub fn i_07624c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76250;
    cpu.ir = 0x6000;
    cpu.pc = 0x7626A;
    10
}
/// $076250: move.l #$7622c, $80(a6)
pub fn i_076250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76258;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7622C;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076258: tst.b $84(a6)
pub fn i_076258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7625C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07625C: beq.w $760f4
pub fn i_07625c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76260;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x760F4; return 10; }
    8
}
/// $076260: lea.l $7534c(pc), a0
pub fn i_076260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76264;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7534C;
    16
}
/// $076264: jsr $28cd4.l
pub fn i_076264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7626A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7626A);
    cpu.pc = 0x28CD4;
    16
}
/// $07626A: lea.l $76270(pc), a1
pub fn i_07626a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7626E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76270;
    16
}
/// $07626E: move.l a1, (a6)
pub fn i_07626e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76270;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076270: bsr.w $75f44
pub fn i_076270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76274;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76274);
    cpu.pc = 0x75F44;
    18
}
/// $076274: jsr $28d70.l
pub fn i_076274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7627A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7627A);
    cpu.pc = 0x28D70;
    16
}
/// $07627A: bcc.w $7628c
pub fn i_07627a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7627E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7628C; return 10; }
    8
}
/// $07627E: tst.w $70(a6)
pub fn i_07627e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76282;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $076282: bne.w $7628c
pub fn i_076282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76286;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7628C; return 10; }
    8
}
/// $076286: lea.l $76290(pc), a1
pub fn i_076286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7628A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76290;
    16
}
/// $07628A: move.l a1, (a6)
pub fn i_07628a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7628C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07628C: bra.w $75ffc
pub fn i_07628c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76290;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_TowerB_Aim_076290  ($076290, 76 B, props/fx_explosions_platforms_crew_076000.s)
/// $076290: move.w $16(a6), $14(a6)
pub fn i_076290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76296;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076296: movea.l $c(a6), a0
pub fn i_076296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7629A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07629A: move.w $7e(a0), d0
pub fn i_07629a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7629E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07629E: cmp.w $66(a0), d0
pub fn i_07629e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762A2;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0762A2: bge.w $762ae
pub fn i_0762a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762A6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x762AE; return 10; }
    8
}
/// $0762A6: lea.l $75f3c(pc), a0
pub fn i_0762a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762AA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75F3C;
    16
}
/// $0762AA: bra.w $762c2
pub fn i_0762aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x762C2;
    10
}
/// $0762AE: move.l #$76290, $80(a6)
pub fn i_0762ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762B6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x76290;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0762B6: tst.b $84(a6)
pub fn i_0762b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762BA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0762BA: beq.w $760f4
pub fn i_0762ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x760F4; return 10; }
    8
}
/// $0762BE: lea.l $75f34(pc), a0
pub fn i_0762be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75F34;
    16
}
/// $0762C2: jsr $5e086.l
pub fn i_0762c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x762C8);
    cpu.pc = 0x5E086;
    16
}
/// $0762C8: bcc.w $76322
pub fn i_0762c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76322; return 10; }
    8
}
/// $0762CC: lea.l $2bf18a.l, a0
pub fn i_0762cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF18A;
    20
}
/// $0762D2: jsr $799de.l
pub fn i_0762d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x762D8);
    cpu.pc = 0x799DE;
    16
}
/// $0762D8: move.w d0, $72(a6)
pub fn i_0762d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== ScriptedProp_TowerB_Fire_0762dc  ($0762DC, 116 B, props/fx_explosions_platforms_crew_076000.s)
/// $0762DC: lea.l $754b2(pc), a0
pub fn i_0762dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x754B2;
    16
}
/// $0762E0: jsr $28cd4.l
pub fn i_0762e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x762E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0762E6: bsr.w $75fc8
pub fn i_0762e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x762EA);
    cpu.pc = 0x75FC8;
    18
}
/// $0762EA: lea.l $762f0(pc), a1
pub fn i_0762ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x762F0;
    16
}
/// $0762EE: move.l a1, (a6)
pub fn i_0762ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0762F0: bsr.w $75f44
pub fn i_0762f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x762F4);
    cpu.pc = 0x75F44;
    18
}
/// $0762F4: jsr $28d70.l
pub fn i_0762f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x762FA);
    cpu.pc = 0x28D70;
    16
}
/// $0762FA: bcc.w $7631e
pub fn i_0762fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7631E; return 10; }
    8
}
/// $0762FE: tst.w $72(a6)
pub fn i_0762fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76302;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $076302: beq.w $76318
pub fn i_076302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76306;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76318; return 10; }
    8
}
/// $076306: tst.w $70(a6)
pub fn i_076306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7630A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07630A: bne.w $76314
pub fn i_07630a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7630E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76314; return 10; }
    8
}
/// $07630E: lea.l $762dc(pc), a1
pub fn i_07630e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76312;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x762DC;
    16
}
/// $076312: move.l a1, (a6)
pub fn i_076312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76314;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076314: bra.w $7631e
pub fn i_076314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76318;
    cpu.ir = 0x6000;
    cpu.pc = 0x7631E;
    10
}
/// $076318: lea.l $76138(pc), a1
pub fn i_076318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7631C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76138;
    16
}
/// $07631C: move.l a1, (a6)
pub fn i_07631c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7631E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07631E: bra.w $75ffc
pub fn i_07631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76322;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
/// $076322: move.w $16(a6), $14(a6)
pub fn i_076322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76328;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x073590, i_073590),
    (0x073594, i_073594),
    (0x07359A, i_07359a),
    (0x07359E, i_07359e),
    (0x0735A2, i_0735a2),
    (0x0735A4, i_0735a4),
    (0x0735A6, i_0735a6),
    (0x0735AE, i_0735ae),
    (0x0735B4, i_0735b4),
    (0x0735B8, i_0735b8),
    (0x0735BC, i_0735bc),
    (0x0735C2, i_0735c2),
    (0x0735C8, i_0735c8),
    (0x0735CE, i_0735ce),
    (0x0735D2, i_0735d2),
    (0x0735D8, i_0735d8),
    (0x0735DE, i_0735de),
    (0x0735E4, i_0735e4),
    (0x0735EA, i_0735ea),
    (0x0735EE, i_0735ee),
    (0x0735F4, i_0735f4),
    (0x0735FC, i_0735fc),
    (0x073600, i_073600),
    (0x073606, i_073606),
    (0x07360C, i_07360c),
    (0x073612, i_073612),
    (0x073618, i_073618),
    (0x07361E, i_07361e),
    (0x073622, i_073622),
    (0x073624, i_073624),
    (0x07362A, i_07362a),
    (0x07362E, i_07362e),
    (0x073632, i_073632),
    (0x073638, i_073638),
    (0x07363C, i_07363c),
    (0x07363E, i_07363e),
    (0x073642, i_073642),
    (0x073646, i_073646),
    (0x07364C, i_07364c),
    (0x073650, i_073650),
    (0x073656, i_073656),
    (0x07365C, i_07365c),
    (0x073662, i_073662),
    (0x073666, i_073666),
    (0x07366A, i_07366a),
    (0x073670, i_073670),
    (0x073676, i_073676),
    (0x07367C, i_07367c),
    (0x073680, i_073680),
    (0x073684, i_073684),
    (0x073686, i_073686),
    (0x073688, i_073688),
    (0x07368C, i_07368c),
    (0x073692, i_073692),
    (0x073698, i_073698),
    (0x07369C, i_07369c),
    (0x0736A2, i_0736a2),
    (0x0736A8, i_0736a8),
    (0x0736AC, i_0736ac),
    (0x0736B4, i_0736b4),
    (0x0736B8, i_0736b8),
    (0x0736BE, i_0736be),
    (0x0736C4, i_0736c4),
    (0x0736CA, i_0736ca),
    (0x0736D0, i_0736d0),
    (0x0736D6, i_0736d6),
    (0x0736DA, i_0736da),
    (0x0736DC, i_0736dc),
    (0x0736E0, i_0736e0),
    (0x0736E4, i_0736e4),
    (0x0736EA, i_0736ea),
    (0x0736EE, i_0736ee),
    (0x0736F4, i_0736f4),
    (0x0736FA, i_0736fa),
    (0x0736FE, i_0736fe),
    (0x073704, i_073704),
    (0x073708, i_073708),
    (0x07370C, i_07370c),
    (0x073712, i_073712),
    (0x073718, i_073718),
    (0x07371C, i_07371c),
    (0x073720, i_073720),
    (0x073726, i_073726),
    (0x07372A, i_07372a),
    (0x073730, i_073730),
    (0x073736, i_073736),
    (0x07373A, i_07373a),
    (0x073740, i_073740),
    (0x073746, i_073746),
    (0x07374A, i_07374a),
    (0x073750, i_073750),
    (0x073754, i_073754),
    (0x073758, i_073758),
    (0x07375A, i_07375a),
    (0x07375C, i_07375c),
    (0x073760, i_073760),
    (0x073766, i_073766),
    (0x07376C, i_07376c),
    (0x073770, i_073770),
    (0x073778, i_073778),
    (0x07377C, i_07377c),
    (0x073782, i_073782),
    (0x073788, i_073788),
    (0x07378E, i_07378e),
    (0x073794, i_073794),
    (0x07379A, i_07379a),
    (0x07379E, i_07379e),
    (0x0737A0, i_0737a0),
    (0x0737A6, i_0737a6),
    (0x0737AC, i_0737ac),
    (0x0737B2, i_0737b2),
    (0x0737B6, i_0737b6),
    (0x0737BA, i_0737ba),
    (0x0737BC, i_0737bc),
    (0x0737C2, i_0737c2),
    (0x0737C6, i_0737c6),
    (0x0737CC, i_0737cc),
    (0x0737D2, i_0737d2),
    (0x0737D8, i_0737d8),
    (0x0737DE, i_0737de),
    (0x0737E4, i_0737e4),
    (0x0737E8, i_0737e8),
    (0x0737EC, i_0737ec),
    (0x0737EE, i_0737ee),
    (0x0737F2, i_0737f2),
    (0x0737F8, i_0737f8),
    (0x0737FC, i_0737fc),
    (0x073800, i_073800),
    (0x073802, i_073802),
    (0x073804, i_073804),
    (0x073808, i_073808),
    (0x07380E, i_07380e),
    (0x073814, i_073814),
    (0x07381A, i_07381a),
    (0x073820, i_073820),
    (0x073826, i_073826),
    (0x07382A, i_07382a),
    (0x073830, i_073830),
    (0x073836, i_073836),
    (0x07383A, i_07383a),
    (0x07383C, i_07383c),
    (0x073842, i_073842),
    (0x073848, i_073848),
    (0x07384E, i_07384e),
    (0x073852, i_073852),
    (0x073858, i_073858),
    (0x07385C, i_07385c),
    (0x073860, i_073860),
    (0x073862, i_073862),
    (0x073866, i_073866),
    (0x07386A, i_07386a),
    (0x073870, i_073870),
    (0x073876, i_073876),
    (0x07387A, i_07387a),
    (0x07387E, i_07387e),
    (0x073880, i_073880),
    (0x073886, i_073886),
    (0x07388C, i_07388c),
    (0x073890, i_073890),
    (0x073896, i_073896),
    (0x07389C, i_07389c),
    (0x0738A2, i_0738a2),
    (0x0738A6, i_0738a6),
    (0x0738A8, i_0738a8),
    (0x0738AE, i_0738ae),
    (0x0738B4, i_0738b4),
    (0x0738BA, i_0738ba),
    (0x0738BE, i_0738be),
    (0x0738C2, i_0738c2),
    (0x0738C6, i_0738c6),
    (0x0738C8, i_0738c8),
    (0x0738CC, i_0738cc),
    (0x0738D2, i_0738d2),
    (0x0738D6, i_0738d6),
    (0x0738DA, i_0738da),
    (0x0738DE, i_0738de),
    (0x0738E4, i_0738e4),
    (0x0738EC, i_0738ec),
    (0x0738F0, i_0738f0),
    (0x0738F6, i_0738f6),
    (0x0738FE, i_0738fe),
    (0x073904, i_073904),
    (0x07390A, i_07390a),
    (0x07390E, i_07390e),
    (0x073910, i_073910),
    (0x073916, i_073916),
    (0x07391A, i_07391a),
    (0x07391E, i_07391e),
    (0x073922, i_073922),
    (0x073928, i_073928),
    (0x07392E, i_07392e),
    (0x073932, i_073932),
    (0x073934, i_073934),
    (0x07393A, i_07393a),
    (0x073940, i_073940),
    (0x073946, i_073946),
    (0x07394A, i_07394a),
    (0x07394E, i_07394e),
    (0x073950, i_073950),
    (0x073952, i_073952),
    (0x073958, i_073958),
    (0x07395E, i_07395e),
    (0x073962, i_073962),
    (0x073968, i_073968),
    (0x07396E, i_07396e),
    (0x073972, i_073972),
    (0x073976, i_073976),
    (0x07397C, i_07397c),
    (0x073982, i_073982),
    (0x073986, i_073986),
    (0x07398A, i_07398a),
    (0x073990, i_073990),
    (0x073998, i_073998),
    (0x07399E, i_07399e),
    (0x0739A4, i_0739a4),
    (0x0739A8, i_0739a8),
    (0x0739AA, i_0739aa),
    (0x0739AE, i_0739ae),
    (0x0739B2, i_0739b2),
    (0x0739B6, i_0739b6),
    (0x0739BA, i_0739ba),
    (0x0739BC, i_0739bc),
    (0x0739C0, i_0739c0),
    (0x0739C4, i_0739c4),
    (0x0739CA, i_0739ca),
    (0x0739D0, i_0739d0),
    (0x0739D4, i_0739d4),
    (0x0739D8, i_0739d8),
    (0x0739DC, i_0739dc),
    (0x0739E2, i_0739e2),
    (0x0739E6, i_0739e6),
    (0x0739EC, i_0739ec),
    (0x0739F0, i_0739f0),
    (0x0739F6, i_0739f6),
    (0x0739FC, i_0739fc),
    (0x073A00, i_073a00),
    (0x073A04, i_073a04),
    (0x073A06, i_073a06),
    (0x073A0C, i_073a0c),
    (0x073A10, i_073a10),
    (0x073A16, i_073a16),
    (0x073A1A, i_073a1a),
    (0x073A1E, i_073a1e),
    (0x073A20, i_073a20),
    (0x073A26, i_073a26),
    (0x073A2A, i_073a2a),
    (0x073A2E, i_073a2e),
    (0x073A30, i_073a30),
    (0x073A32, i_073a32),
    (0x073A36, i_073a36),
    (0x073A3C, i_073a3c),
    (0x073A40, i_073a40),
    (0x073A46, i_073a46),
    (0x073A4C, i_073a4c),
    (0x073A50, i_073a50),
    (0x073A54, i_073a54),
    (0x073A5A, i_073a5a),
    (0x073A60, i_073a60),
    (0x073A64, i_073a64),
    (0x073A68, i_073a68),
    (0x073A6C, i_073a6c),
    (0x073A70, i_073a70),
    (0x073A76, i_073a76),
    (0x073A78, i_073a78),
    (0x073A7C, i_073a7c),
    (0x073A82, i_073a82),
    (0x073A86, i_073a86),
    (0x073A8C, i_073a8c),
    (0x073A90, i_073a90),
    (0x073A92, i_073a92),
    (0x073A96, i_073a96),
    (0x073A9C, i_073a9c),
    (0x073AA0, i_073aa0),
    (0x073AA4, i_073aa4),
    (0x073AA6, i_073aa6),
    (0x073AAC, i_073aac),
    (0x073AB2, i_073ab2),
    (0x073AB6, i_073ab6),
    (0x073ABA, i_073aba),
    (0x073AC0, i_073ac0),
    (0x073AC6, i_073ac6),
    (0x073ACA, i_073aca),
    (0x073AD0, i_073ad0),
    (0x073AD6, i_073ad6),
    (0x073ADA, i_073ada),
    (0x073AE0, i_073ae0),
    (0x073AE6, i_073ae6),
    (0x073AEA, i_073aea),
    (0x073AEE, i_073aee),
    (0x073AF2, i_073af2),
    (0x073AF8, i_073af8),
    (0x073AFE, i_073afe),
    (0x073B04, i_073b04),
    (0x073B08, i_073b08),
    (0x073B0E, i_073b0e),
    (0x073B14, i_073b14),
    (0x073B1A, i_073b1a),
    (0x073B20, i_073b20),
    (0x073B26, i_073b26),
    (0x073B2A, i_073b2a),
    (0x073B2C, i_073b2c),
    (0x073B32, i_073b32),
    (0x073B36, i_073b36),
    (0x073B3C, i_073b3c),
    (0x073B40, i_073b40),
    (0x073B46, i_073b46),
    (0x073B4C, i_073b4c),
    (0x073B50, i_073b50),
    (0x073B56, i_073b56),
    (0x073B5A, i_073b5a),
    (0x073B5E, i_073b5e),
    (0x073B60, i_073b60),
    (0x073B62, i_073b62),
    (0x073B68, i_073b68),
    (0x073B6E, i_073b6e),
    (0x073B72, i_073b72),
    (0x073B74, i_073b74),
    (0x073B7A, i_073b7a),
    (0x073B80, i_073b80),
    (0x073B86, i_073b86),
    (0x073B8A, i_073b8a),
    (0x073B8E, i_073b8e),
    (0x073B90, i_073b90),
    (0x073B92, i_073b92),
    (0x073B96, i_073b96),
    (0x073B9C, i_073b9c),
    (0x073BA2, i_073ba2),
    (0x073BA8, i_073ba8),
    (0x073BAC, i_073bac),
    (0x073BB2, i_073bb2),
    (0x073BB6, i_073bb6),
    (0x073BBC, i_073bbc),
    (0x073BC2, i_073bc2),
    (0x073BC6, i_073bc6),
    (0x073BCC, i_073bcc),
    (0x073BD2, i_073bd2),
    (0x073BD6, i_073bd6),
    (0x073BD8, i_073bd8),
    (0x073BDC, i_073bdc),
    (0x073BE2, i_073be2),
    (0x073BE6, i_073be6),
    (0x073BEA, i_073bea),
    (0x073BF0, i_073bf0),
    (0x073BF4, i_073bf4),
    (0x073BF6, i_073bf6),
    (0x073BFA, i_073bfa),
    (0x073C00, i_073c00),
    (0x073C04, i_073c04),
    (0x073C08, i_073c08),
    (0x073C0A, i_073c0a),
    (0x073C10, i_073c10),
    (0x073C16, i_073c16),
    (0x073C1C, i_073c1c),
    (0x073C20, i_073c20),
    (0x073C24, i_073c24),
    (0x073C26, i_073c26),
    (0x073C28, i_073c28),
    (0x073C2C, i_073c2c),
    (0x073C32, i_073c32),
    (0x073C38, i_073c38),
    (0x073C3E, i_073c3e),
    (0x073C44, i_073c44),
    (0x073C4A, i_073c4a),
    (0x073C4E, i_073c4e),
    (0x073C52, i_073c52),
    (0x073C56, i_073c56),
    (0x073C5A, i_073c5a),
    (0x073C60, i_073c60),
    (0x073C64, i_073c64),
    (0x073C6A, i_073c6a),
    (0x073C6E, i_073c6e),
    (0x073C72, i_073c72),
    (0x073C74, i_073c74),
    (0x073C78, i_073c78),
    (0x073C7C, i_073c7c),
    (0x073C82, i_073c82),
    (0x073C84, i_073c84),
    (0x073C8A, i_073c8a),
    (0x073C8E, i_073c8e),
    (0x073C92, i_073c92),
    (0x073C96, i_073c96),
    (0x073C9C, i_073c9c),
    (0x073CA2, i_073ca2),
    (0x073CA8, i_073ca8),
    (0x073CAE, i_073cae),
    (0x073CB4, i_073cb4),
    (0x073CB8, i_073cb8),
    (0x073CBA, i_073cba),
    (0x073CC0, i_073cc0),
    (0x073CC4, i_073cc4),
    (0x073CC8, i_073cc8),
    (0x073CCA, i_073cca),
    (0x073CCE, i_073cce),
    (0x073CD2, i_073cd2),
    (0x073CD4, i_073cd4),
    (0x073CD8, i_073cd8),
    (0x073CDE, i_073cde),
    (0x073CE4, i_073ce4),
    (0x073CEA, i_073cea),
    (0x073CEE, i_073cee),
    (0x073CF2, i_073cf2),
    (0x073CF4, i_073cf4),
    (0x073CFA, i_073cfa),
    (0x073CFE, i_073cfe),
    (0x073D04, i_073d04),
    (0x073D0A, i_073d0a),
    (0x073D10, i_073d10),
    (0x073D16, i_073d16),
    (0x073D1A, i_073d1a),
    (0x073D1E, i_073d1e),
    (0x073D20, i_073d20),
    (0x073D76, i_073d76),
    (0x073D7A, i_073d7a),
    (0x073D80, i_073d80),
    (0x073E22, i_073e22),
    (0x073E28, i_073e28),
    (0x073E2C, i_073e2c),
    (0x073E32, i_073e32),
    (0x073E38, i_073e38),
    (0x073E3E, i_073e3e),
    (0x073E42, i_073e42),
    (0x073E44, i_073e44),
    (0x073E4A, i_073e4a),
    (0x073E50, i_073e50),
    (0x073E54, i_073e54),
    (0x073E58, i_073e58),
    (0x073E5A, i_073e5a),
    (0x073E5C, i_073e5c),
    (0x073E60, i_073e60),
    (0x073E66, i_073e66),
    (0x073E6E, i_073e6e),
    (0x073E74, i_073e74),
    (0x073E7A, i_073e7a),
    (0x073E7E, i_073e7e),
    (0x073E80, i_073e80),
    (0x073E84, i_073e84),
    (0x073E8A, i_073e8a),
    (0x073E8E, i_073e8e),
    (0x073E94, i_073e94),
    (0x073E98, i_073e98),
    (0x073E9C, i_073e9c),
    (0x073E9E, i_073e9e),
    (0x073EA4, i_073ea4),
    (0x073EAA, i_073eaa),
    (0x073EAE, i_073eae),
    (0x073EB4, i_073eb4),
    (0x073EB8, i_073eb8),
    (0x073EBC, i_073ebc),
    (0x073EBE, i_073ebe),
    (0x073EC4, i_073ec4),
    (0x073EC8, i_073ec8),
    (0x073ECC, i_073ecc),
    (0x073ECE, i_073ece),
    (0x073ED0, i_073ed0),
    (0x073ED6, i_073ed6),
    (0x073EDC, i_073edc),
    (0x073EE0, i_073ee0),
    (0x073EE2, i_073ee2),
    (0x073EE6, i_073ee6),
    (0x073EEC, i_073eec),
    (0x073EF0, i_073ef0),
    (0x073EF4, i_073ef4),
    (0x073EF6, i_073ef6),
    (0x073EF8, i_073ef8),
    (0x073EFC, i_073efc),
    (0x073F02, i_073f02),
    (0x073F0A, i_073f0a),
    (0x073F10, i_073f10),
    (0x073F16, i_073f16),
    (0x073F1A, i_073f1a),
    (0x073F20, i_073f20),
    (0x073F26, i_073f26),
    (0x073F2A, i_073f2a),
    (0x073F2C, i_073f2c),
    (0x073F30, i_073f30),
    (0x073F36, i_073f36),
    (0x073F3A, i_073f3a),
    (0x073F40, i_073f40),
    (0x073F46, i_073f46),
    (0x073F4A, i_073f4a),
    (0x073F50, i_073f50),
    (0x073F54, i_073f54),
    (0x073F58, i_073f58),
    (0x073F5A, i_073f5a),
    (0x073F5E, i_073f5e),
    (0x073F62, i_073f62),
    (0x073F68, i_073f68),
    (0x073F70, i_073f70),
    (0x073F76, i_073f76),
    (0x073F7C, i_073f7c),
    (0x073F80, i_073f80),
    (0x073F82, i_073f82),
    (0x073F86, i_073f86),
    (0x073F8C, i_073f8c),
    (0x073F90, i_073f90),
    (0x073F94, i_073f94),
    (0x073F96, i_073f96),
    (0x073F9A, i_073f9a),
    (0x073FA2, i_073fa2),
    (0x073FA8, i_073fa8),
    (0x073FAE, i_073fae),
    (0x073FB2, i_073fb2),
    (0x073FB4, i_073fb4),
    (0x073FB8, i_073fb8),
    (0x073FBE, i_073fbe),
    (0x073FC2, i_073fc2),
    (0x073FC6, i_073fc6),
    (0x073FC8, i_073fc8),
    (0x073FCA, i_073fca),
    (0x073FCE, i_073fce),
    (0x073FD2, i_073fd2),
    (0x073FD4, i_073fd4),
    (0x073FD8, i_073fd8),
    (0x073FDA, i_073fda),
    (0x073FDE, i_073fde),
    (0x073FE2, i_073fe2),
    (0x073FE6, i_073fe6),
    (0x073FE8, i_073fe8),
    (0x073FEC, i_073fec),
    (0x073FF0, i_073ff0),
    (0x073FF6, i_073ff6),
    (0x073FFC, i_073ffc),
    (0x074002, i_074002),
    (0x074008, i_074008),
    (0x07400C, i_07400c),
    (0x07400E, i_07400e),
    (0x074012, i_074012),
    (0x074016, i_074016),
    (0x07401A, i_07401a),
    (0x074020, i_074020),
    (0x074024, i_074024),
    (0x074026, i_074026),
    (0x07402C, i_07402c),
    (0x074030, i_074030),
    (0x074034, i_074034),
    (0x074036, i_074036),
    (0x07403A, i_07403a),
    (0x07403E, i_07403e),
    (0x074040, i_074040),
    (0x074042, i_074042),
    (0x074048, i_074048),
    (0x07404A, i_07404a),
    (0x07404E, i_07404e),
    (0x074052, i_074052),
    (0x074058, i_074058),
    (0x07405C, i_07405c),
    (0x074060, i_074060),
    (0x074066, i_074066),
    (0x07406C, i_07406c),
    (0x074072, i_074072),
    (0x074076, i_074076),
    (0x07407A, i_07407a),
    (0x07407E, i_07407e),
    (0x074084, i_074084),
    (0x074088, i_074088),
    (0x07408C, i_07408c),
    (0x074092, i_074092),
    (0x074096, i_074096),
    (0x07409C, i_07409c),
    (0x07409E, i_07409e),
    (0x0740A2, i_0740a2),
    (0x0740A6, i_0740a6),
    (0x0740AA, i_0740aa),
    (0x0740AC, i_0740ac),
    (0x0740B2, i_0740b2),
    (0x0740B8, i_0740b8),
    (0x0740BE, i_0740be),
    (0x0740C4, i_0740c4),
    (0x0740C8, i_0740c8),
    (0x0740CE, i_0740ce),
    (0x0740D4, i_0740d4),
    (0x0740DA, i_0740da),
    (0x0740DE, i_0740de),
    (0x0740E0, i_0740e0),
    (0x0740E6, i_0740e6),
    (0x0740EC, i_0740ec),
    (0x0740F0, i_0740f0),
    (0x0740F4, i_0740f4),
    (0x0740F6, i_0740f6),
    (0x0740FC, i_0740fc),
    (0x074102, i_074102),
    (0x074106, i_074106),
    (0x07410A, i_07410a),
    (0x07410C, i_07410c),
    (0x07410E, i_07410e),
    (0x074112, i_074112),
    (0x074116, i_074116),
    (0x07411A, i_07411a),
    (0x07411C, i_07411c),
    (0x074120, i_074120),
    (0x074122, i_074122),
    (0x074128, i_074128),
    (0x07412C, i_07412c),
    (0x074132, i_074132),
    (0x074136, i_074136),
    (0x07413A, i_07413a),
    (0x074140, i_074140),
    (0x074144, i_074144),
    (0x074148, i_074148),
    (0x07414C, i_07414c),
    (0x074152, i_074152),
    (0x074156, i_074156),
    (0x07415A, i_07415a),
    (0x07415E, i_07415e),
    (0x074160, i_074160),
    (0x074164, i_074164),
    (0x074166, i_074166),
    (0x07416C, i_07416c),
    (0x074170, i_074170),
    (0x074176, i_074176),
    (0x07417A, i_07417a),
    (0x07417E, i_07417e),
    (0x074182, i_074182),
    (0x074186, i_074186),
    (0x07418A, i_07418a),
    (0x07418E, i_07418e),
    (0x074190, i_074190),
    (0x074194, i_074194),
    (0x074196, i_074196),
    (0x07419A, i_07419a),
    (0x07419C, i_07419c),
    (0x0741A0, i_0741a0),
    (0x0741A4, i_0741a4),
    (0x0741A8, i_0741a8),
    (0x0741AC, i_0741ac),
    (0x0741B0, i_0741b0),
    (0x0741B4, i_0741b4),
    (0x0741B8, i_0741b8),
    (0x0741BA, i_0741ba),
    (0x0741BE, i_0741be),
    (0x0741C0, i_0741c0),
    (0x0741C6, i_0741c6),
    (0x0741CA, i_0741ca),
    (0x0741D0, i_0741d0),
    (0x0741D4, i_0741d4),
    (0x0741D6, i_0741d6),
    (0x0741DA, i_0741da),
    (0x0741DE, i_0741de),
    (0x0741E2, i_0741e2),
    (0x0741E6, i_0741e6),
    (0x0741EA, i_0741ea),
    (0x0741EC, i_0741ec),
    (0x0741EE, i_0741ee),
    (0x0741F2, i_0741f2),
    (0x0741F6, i_0741f6),
    (0x0741F8, i_0741f8),
    (0x0741FC, i_0741fc),
    (0x074200, i_074200),
    (0x074204, i_074204),
    (0x074208, i_074208),
    (0x07420C, i_07420c),
    (0x074210, i_074210),
    (0x074214, i_074214),
    (0x074216, i_074216),
    (0x07421A, i_07421a),
    (0x074220, i_074220),
    (0x074224, i_074224),
    (0x07422A, i_07422a),
    (0x07422C, i_07422c),
    (0x074230, i_074230),
    (0x074234, i_074234),
    (0x074238, i_074238),
    (0x07423C, i_07423c),
    (0x074240, i_074240),
    (0x074246, i_074246),
    (0x07424A, i_07424a),
    (0x07424E, i_07424e),
    (0x074252, i_074252),
    (0x074258, i_074258),
    (0x07425E, i_07425e),
    (0x074260, i_074260),
    (0x074264, i_074264),
    (0x074268, i_074268),
    (0x07426C, i_07426c),
    (0x074272, i_074272),
    (0x074276, i_074276),
    (0x07427A, i_07427a),
    (0x07427E, i_07427e),
    (0x074282, i_074282),
    (0x074286, i_074286),
    (0x07428A, i_07428a),
    (0x07428C, i_07428c),
    (0x074292, i_074292),
    (0x074298, i_074298),
    (0x07429C, i_07429c),
    (0x07429E, i_07429e),
    (0x0742A4, i_0742a4),
    (0x0742A8, i_0742a8),
    (0x0742AC, i_0742ac),
    (0x0742B0, i_0742b0),
    (0x0742B6, i_0742b6),
    (0x0742B8, i_0742b8),
    (0x0742BC, i_0742bc),
    (0x0742C0, i_0742c0),
    (0x0742C4, i_0742c4),
    (0x0742C8, i_0742c8),
    (0x0742CC, i_0742cc),
    (0x0742D2, i_0742d2),
    (0x0742D6, i_0742d6),
    (0x0742DA, i_0742da),
    (0x0742DE, i_0742de),
    (0x0742E4, i_0742e4),
    (0x0742E8, i_0742e8),
    (0x0742EC, i_0742ec),
    (0x0742EE, i_0742ee),
    (0x0742F4, i_0742f4),
    (0x0742F8, i_0742f8),
    (0x0742FC, i_0742fc),
    (0x074300, i_074300),
    (0x074304, i_074304),
    (0x074306, i_074306),
    (0x07430A, i_07430a),
    (0x07430C, i_07430c),
    (0x074312, i_074312),
    (0x074318, i_074318),
    (0x07431C, i_07431c),
    (0x074320, i_074320),
    (0x074324, i_074324),
    (0x074326, i_074326),
    (0x074328, i_074328),
    (0x07432A, i_07432a),
    (0x07432C, i_07432c),
    (0x074330, i_074330),
    (0x074334, i_074334),
    (0x074336, i_074336),
    (0x07433C, i_07433c),
    (0x074340, i_074340),
    (0x074344, i_074344),
    (0x074348, i_074348),
    (0x07434E, i_07434e),
    (0x074352, i_074352),
    (0x074356, i_074356),
    (0x07435C, i_07435c),
    (0x074360, i_074360),
    (0x074366, i_074366),
    (0x07436A, i_07436a),
    (0x074370, i_074370),
    (0x074374, i_074374),
    (0x074378, i_074378),
    (0x07437C, i_07437c),
    (0x074382, i_074382),
    (0x074388, i_074388),
    (0x07438E, i_07438e),
    (0x074394, i_074394),
    (0x074398, i_074398),
    (0x07439A, i_07439a),
    (0x07439E, i_07439e),
    (0x0743A4, i_0743a4),
    (0x0743AA, i_0743aa),
    (0x0743AC, i_0743ac),
    (0x0743B4, i_0743b4),
    (0x0743B8, i_0743b8),
    (0x0743BE, i_0743be),
    (0x0743C4, i_0743c4),
    (0x0743C8, i_0743c8),
    (0x0743CA, i_0743ca),
    (0x0743D0, i_0743d0),
    (0x0743D6, i_0743d6),
    (0x0743DC, i_0743dc),
    (0x0743DE, i_0743de),
    (0x0743E2, i_0743e2),
    (0x0743E8, i_0743e8),
    (0x0743EE, i_0743ee),
    (0x0743F4, i_0743f4),
    (0x0743FA, i_0743fa),
    (0x074400, i_074400),
    (0x074406, i_074406),
    (0x074408, i_074408),
    (0x07440E, i_07440e),
    (0x074414, i_074414),
    (0x074416, i_074416),
    (0x07441A, i_07441a),
    (0x074420, i_074420),
    (0x074424, i_074424),
    (0x07442A, i_07442a),
    (0x074430, i_074430),
    (0x074434, i_074434),
    (0x07443A, i_07443a),
    (0x074440, i_074440),
    (0x074446, i_074446),
    (0x07444C, i_07444c),
    (0x074452, i_074452),
    (0x074458, i_074458),
    (0x07445E, i_07445e),
    (0x074460, i_074460),
    (0x074462, i_074462),
    (0x074466, i_074466),
    (0x07446A, i_07446a),
    (0x07446C, i_07446c),
    (0x074470, i_074470),
    (0x074474, i_074474),
    (0x074478, i_074478),
    (0x07447A, i_07447a),
    (0x07447E, i_07447e),
    (0x074482, i_074482),
    (0x074486, i_074486),
    (0x07448A, i_07448a),
    (0x07448E, i_07448e),
    (0x074492, i_074492),
    (0x074498, i_074498),
    (0x07449E, i_07449e),
    (0x0744A4, i_0744a4),
    (0x0744A8, i_0744a8),
    (0x0744AA, i_0744aa),
    (0x0744B0, i_0744b0),
    (0x0744B6, i_0744b6),
    (0x0744BC, i_0744bc),
    (0x0744C0, i_0744c0),
    (0x0744C4, i_0744c4),
    (0x0744CA, i_0744ca),
    (0x0744D0, i_0744d0),
    (0x0744D4, i_0744d4),
    (0x0744DA, i_0744da),
    (0x0744E0, i_0744e0),
    (0x0744E6, i_0744e6),
    (0x0744EC, i_0744ec),
    (0x0744F2, i_0744f2),
    (0x0744F6, i_0744f6),
    (0x0744FA, i_0744fa),
    (0x074500, i_074500),
    (0x074506, i_074506),
    (0x07450C, i_07450c),
    (0x074512, i_074512),
    (0x074516, i_074516),
    (0x07451C, i_07451c),
    (0x074522, i_074522),
    (0x074528, i_074528),
    (0x07452E, i_07452e),
    (0x074532, i_074532),
    (0x074536, i_074536),
    (0x07453C, i_07453c),
    (0x074542, i_074542),
    (0x074546, i_074546),
    (0x07454C, i_07454c),
    (0x074552, i_074552),
    (0x074558, i_074558),
    (0x07455A, i_07455a),
    (0x074560, i_074560),
    (0x074564, i_074564),
    (0x074568, i_074568),
    (0x07456E, i_07456e),
    (0x074574, i_074574),
    (0x07457A, i_07457a),
    (0x074580, i_074580),
    (0x074584, i_074584),
    (0x074586, i_074586),
    (0x074588, i_074588),
    (0x07458A, i_07458a),
    (0x07458E, i_07458e),
    (0x074594, i_074594),
    (0x07459A, i_07459a),
    (0x0745A0, i_0745a0),
    (0x0745A2, i_0745a2),
    (0x0745A8, i_0745a8),
    (0x0745AE, i_0745ae),
    (0x0745B0, i_0745b0),
    (0x0745B6, i_0745b6),
    (0x0745BC, i_0745bc),
    (0x0745BE, i_0745be),
    (0x0745C2, i_0745c2),
    (0x0745C6, i_0745c6),
    (0x0745CA, i_0745ca),
    (0x0745CE, i_0745ce),
    (0x0745D2, i_0745d2),
    (0x0745D4, i_0745d4),
    (0x0745D8, i_0745d8),
    (0x0745DA, i_0745da),
    (0x0745DC, i_0745dc),
    (0x0745E0, i_0745e0),
    (0x0745E2, i_0745e2),
    (0x0745E4, i_0745e4),
    (0x0745E6, i_0745e6),
    (0x0745E8, i_0745e8),
    (0x0745EC, i_0745ec),
    (0x0745F0, i_0745f0),
    (0x0745F4, i_0745f4),
    (0x0745F8, i_0745f8),
    (0x0745FC, i_0745fc),
    (0x074600, i_074600),
    (0x074604, i_074604),
    (0x074608, i_074608),
    (0x07460C, i_07460c),
    (0x074610, i_074610),
    (0x074612, i_074612),
    (0x074614, i_074614),
    (0x074618, i_074618),
    (0x07461E, i_07461e),
    (0x074622, i_074622),
    (0x074624, i_074624),
    (0x074628, i_074628),
    (0x07462A, i_07462a),
    (0x07462E, i_07462e),
    (0x074630, i_074630),
    (0x074632, i_074632),
    (0x074636, i_074636),
    (0x07463A, i_07463a),
    (0x07463C, i_07463c),
    (0x07463E, i_07463e),
    (0x074640, i_074640),
    (0x074644, i_074644),
    (0x074648, i_074648),
    (0x07464A, i_07464a),
    (0x07464C, i_07464c),
    (0x074650, i_074650),
    (0x074652, i_074652),
    (0x074654, i_074654),
    (0x074656, i_074656),
    (0x07465A, i_07465a),
    (0x07465C, i_07465c),
    (0x07465E, i_07465e),
    (0x074660, i_074660),
    (0x074664, i_074664),
    (0x074668, i_074668),
    (0x07466A, i_07466a),
    (0x0749F2, i_0749f2),
    (0x0749F6, i_0749f6),
    (0x0749FC, i_0749fc),
    (0x074A02, i_074a02),
    (0x074A06, i_074a06),
    (0x074A0C, i_074a0c),
    (0x074A18, i_074a18),
    (0x074A1E, i_074a1e),
    (0x074A22, i_074a22),
    (0x074A28, i_074a28),
    (0x074A30, i_074a30),
    (0x074A34, i_074a34),
    (0x074A3A, i_074a3a),
    (0x074A40, i_074a40),
    (0x074A44, i_074a44),
    (0x074A4A, i_074a4a),
    (0x074A52, i_074a52),
    (0x074A58, i_074a58),
    (0x074A5C, i_074a5c),
    (0x074A62, i_074a62),
    (0x074A6A, i_074a6a),
    (0x074A6E, i_074a6e),
    (0x074A74, i_074a74),
    (0x074A7A, i_074a7a),
    (0x074A7E, i_074a7e),
    (0x074A84, i_074a84),
    (0x074A8C, i_074a8c),
    (0x074A92, i_074a92),
    (0x074A96, i_074a96),
    (0x074A9C, i_074a9c),
    (0x074AA8, i_074aa8),
    (0x074AAC, i_074aac),
    (0x074AB2, i_074ab2),
    (0x074AB8, i_074ab8),
    (0x074ABC, i_074abc),
    (0x074AC2, i_074ac2),
    (0x074ACA, i_074aca),
    (0x074ACE, i_074ace),
    (0x074AD4, i_074ad4),
    (0x074ADA, i_074ada),
    (0x074ADE, i_074ade),
    (0x074AE4, i_074ae4),
    (0x074AEC, i_074aec),
    (0x074AF0, i_074af0),
    (0x074AF6, i_074af6),
    (0x074AFC, i_074afc),
    (0x074B00, i_074b00),
    (0x074B06, i_074b06),
    (0x074B0E, i_074b0e),
    (0x074B14, i_074b14),
    (0x074B18, i_074b18),
    (0x074B1E, i_074b1e),
    (0x074B26, i_074b26),
    (0x074B2A, i_074b2a),
    (0x074B30, i_074b30),
    (0x074B36, i_074b36),
    (0x074B3A, i_074b3a),
    (0x074B40, i_074b40),
    (0x074B48, i_074b48),
    (0x074B4C, i_074b4c),
    (0x074B52, i_074b52),
    (0x074B58, i_074b58),
    (0x074B5C, i_074b5c),
    (0x074B62, i_074b62),
    (0x074B6A, i_074b6a),
    (0x074B70, i_074b70),
    (0x074B74, i_074b74),
    (0x074B7A, i_074b7a),
    (0x074B82, i_074b82),
    (0x074B86, i_074b86),
    (0x074B8C, i_074b8c),
    (0x074B92, i_074b92),
    (0x074B96, i_074b96),
    (0x074B9C, i_074b9c),
    (0x074BA4, i_074ba4),
    (0x074BAA, i_074baa),
    (0x074BAE, i_074bae),
    (0x074BB4, i_074bb4),
    (0x074BC0, i_074bc0),
    (0x074BC4, i_074bc4),
    (0x074BCA, i_074bca),
    (0x074BD0, i_074bd0),
    (0x074BD4, i_074bd4),
    (0x074BDA, i_074bda),
    (0x074BE2, i_074be2),
    (0x074BE6, i_074be6),
    (0x074BEC, i_074bec),
    (0x074BF2, i_074bf2),
    (0x074BF6, i_074bf6),
    (0x074BFC, i_074bfc),
    (0x074C08, i_074c08),
    (0x074C0C, i_074c0c),
    (0x074C12, i_074c12),
    (0x074C16, i_074c16),
    (0x074C1C, i_074c1c),
    (0x074C26, i_074c26),
    (0x074C2A, i_074c2a),
    (0x074C30, i_074c30),
    (0x074C36, i_074c36),
    (0x074C3A, i_074c3a),
    (0x074C40, i_074c40),
    (0x074C48, i_074c48),
    (0x074C4C, i_074c4c),
    (0x074C52, i_074c52),
    (0x074C58, i_074c58),
    (0x074C5C, i_074c5c),
    (0x074C62, i_074c62),
    (0x074C6A, i_074c6a),
    (0x074C6E, i_074c6e),
    (0x074C74, i_074c74),
    (0x074C7A, i_074c7a),
    (0x074C7E, i_074c7e),
    (0x074C84, i_074c84),
    (0x074C8C, i_074c8c),
    (0x074C90, i_074c90),
    (0x074C96, i_074c96),
    (0x074C9C, i_074c9c),
    (0x074CA0, i_074ca0),
    (0x074CA6, i_074ca6),
    (0x074CAE, i_074cae),
    (0x074CB2, i_074cb2),
    (0x074CB8, i_074cb8),
    (0x074CBE, i_074cbe),
    (0x074CC2, i_074cc2),
    (0x074CC8, i_074cc8),
    (0x074CD0, i_074cd0),
    (0x074CD4, i_074cd4),
    (0x074CDA, i_074cda),
    (0x074CE0, i_074ce0),
    (0x074CE4, i_074ce4),
    (0x074CEA, i_074cea),
    (0x074CF2, i_074cf2),
    (0x074CF6, i_074cf6),
    (0x074CFC, i_074cfc),
    (0x074D02, i_074d02),
    (0x074D06, i_074d06),
    (0x074D0C, i_074d0c),
    (0x074D14, i_074d14),
    (0x074D18, i_074d18),
    (0x074D1E, i_074d1e),
    (0x074D24, i_074d24),
    (0x074D28, i_074d28),
    (0x074D2E, i_074d2e),
    (0x074D3A, i_074d3a),
    (0x074D3E, i_074d3e),
    (0x074D44, i_074d44),
    (0x074D48, i_074d48),
    (0x074D4E, i_074d4e),
    (0x074D58, i_074d58),
    (0x074D5C, i_074d5c),
    (0x074D62, i_074d62),
    (0x074D68, i_074d68),
    (0x074D6C, i_074d6c),
    (0x074D72, i_074d72),
    (0x074D92, i_074d92),
    (0x074D96, i_074d96),
    (0x074D9C, i_074d9c),
    (0x074DA2, i_074da2),
    (0x074DA6, i_074da6),
    (0x074DAC, i_074dac),
    (0x074DB8, i_074db8),
    (0x074DBC, i_074dbc),
    (0x074DC2, i_074dc2),
    (0x074DC6, i_074dc6),
    (0x074DCC, i_074dcc),
    (0x074DD6, i_074dd6),
    (0x074DDA, i_074dda),
    (0x074DE0, i_074de0),
    (0x074DE6, i_074de6),
    (0x074DEA, i_074dea),
    (0x074DF0, i_074df0),
    (0x074DFC, i_074dfc),
    (0x074E00, i_074e00),
    (0x074E06, i_074e06),
    (0x074E0A, i_074e0a),
    (0x074E0E, i_074e0e),
    (0x074E14, i_074e14),
    (0x074E1C, i_074e1c),
    (0x074E20, i_074e20),
    (0x074E26, i_074e26),
    (0x074E2C, i_074e2c),
    (0x074E30, i_074e30),
    (0x074E36, i_074e36),
    (0x074E44, i_074e44),
    (0x074E48, i_074e48),
    (0x074E4E, i_074e4e),
    (0x074E54, i_074e54),
    (0x074E58, i_074e58),
    (0x074E5E, i_074e5e),
    (0x074E74, i_074e74),
    (0x074E78, i_074e78),
    (0x074E7C, i_074e7c),
    (0x074E80, i_074e80),
    (0x074E86, i_074e86),
    (0x074E8A, i_074e8a),
    (0x074E8E, i_074e8e),
    (0x074E94, i_074e94),
    (0x074E98, i_074e98),
    (0x074E9C, i_074e9c),
    (0x074EA2, i_074ea2),
    (0x074EAA, i_074eaa),
    (0x074EAE, i_074eae),
    (0x074EB4, i_074eb4),
    (0x074EBA, i_074eba),
    (0x074EBE, i_074ebe),
    (0x074EC4, i_074ec4),
    (0x074ED4, i_074ed4),
    (0x074ED8, i_074ed8),
    (0x074EDE, i_074ede),
    (0x074EE4, i_074ee4),
    (0x074EE8, i_074ee8),
    (0x074EEE, i_074eee),
    (0x074EFA, i_074efa),
    (0x074EFE, i_074efe),
    (0x074F04, i_074f04),
    (0x074F08, i_074f08),
    (0x074F0E, i_074f0e),
    (0x074F18, i_074f18),
    (0x074F1C, i_074f1c),
    (0x074F22, i_074f22),
    (0x074F28, i_074f28),
    (0x074F2C, i_074f2c),
    (0x074F32, i_074f32),
    (0x074F44, i_074f44),
    (0x074F48, i_074f48),
    (0x074F4C, i_074f4c),
    (0x074F50, i_074f50),
    (0x074F56, i_074f56),
    (0x0750A4, i_0750a4),
    (0x0750A6, i_0750a6),
    (0x0750A8, i_0750a8),
    (0x0750AE, i_0750ae),
    (0x0750B2, i_0750b2),
    (0x0750B6, i_0750b6),
    (0x0750BA, i_0750ba),
    (0x0750C0, i_0750c0),
    (0x0750C6, i_0750c6),
    (0x0750CA, i_0750ca),
    (0x0750CE, i_0750ce),
    (0x0750D2, i_0750d2),
    (0x0750D4, i_0750d4),
    (0x0750D8, i_0750d8),
    (0x0750DC, i_0750dc),
    (0x0750E2, i_0750e2),
    (0x0750E8, i_0750e8),
    (0x0750EA, i_0750ea),
    (0x0750EE, i_0750ee),
    (0x0750F2, i_0750f2),
    (0x0750F6, i_0750f6),
    (0x0750FA, i_0750fa),
    (0x0750FE, i_0750fe),
    (0x075100, i_075100),
    (0x075106, i_075106),
    (0x07510C, i_07510c),
    (0x075110, i_075110),
    (0x075114, i_075114),
    (0x075116, i_075116),
    (0x07511A, i_07511a),
    (0x07511E, i_07511e),
    (0x075122, i_075122),
    (0x075126, i_075126),
    (0x07512A, i_07512a),
    (0x07512E, i_07512e),
    (0x075132, i_075132),
    (0x075136, i_075136),
    (0x07513A, i_07513a),
    (0x07513E, i_07513e),
    (0x075144, i_075144),
    (0x075148, i_075148),
    (0x07514E, i_07514e),
    (0x075152, i_075152),
    (0x075158, i_075158),
    (0x07515C, i_07515c),
    (0x075162, i_075162),
    (0x075166, i_075166),
    (0x07516C, i_07516c),
    (0x075172, i_075172),
    (0x075178, i_075178),
    (0x07517C, i_07517c),
    (0x075180, i_075180),
    (0x075184, i_075184),
    (0x07518C, i_07518c),
    (0x075190, i_075190),
    (0x075194, i_075194),
    (0x075198, i_075198),
    (0x07519E, i_07519e),
    (0x0751A2, i_0751a2),
    (0x0751A4, i_0751a4),
    (0x0751A6, i_0751a6),
    (0x0751AC, i_0751ac),
    (0x0751B0, i_0751b0),
    (0x0751B8, i_0751b8),
    (0x0751BC, i_0751bc),
    (0x0751C0, i_0751c0),
    (0x0751C4, i_0751c4),
    (0x0751C8, i_0751c8),
    (0x0751CC, i_0751cc),
    (0x0751D0, i_0751d0),
    (0x0751D4, i_0751d4),
    (0x0751DA, i_0751da),
    (0x0751DE, i_0751de),
    (0x0751E2, i_0751e2),
    (0x0751E6, i_0751e6),
    (0x0751EA, i_0751ea),
    (0x0751EC, i_0751ec),
    (0x0751F0, i_0751f0),
    (0x0751F6, i_0751f6),
    (0x0751FA, i_0751fa),
    (0x0751FE, i_0751fe),
    (0x075202, i_075202),
    (0x075208, i_075208),
    (0x07520A, i_07520a),
    (0x07520E, i_07520e),
    (0x075210, i_075210),
    (0x075214, i_075214),
    (0x075218, i_075218),
    (0x07521C, i_07521c),
    (0x075220, i_075220),
    (0x075226, i_075226),
    (0x07522A, i_07522a),
    (0x07522E, i_07522e),
    (0x075234, i_075234),
    (0x07523A, i_07523a),
    (0x075240, i_075240),
    (0x075246, i_075246),
    (0x07524A, i_07524a),
    (0x075250, i_075250),
    (0x075254, i_075254),
    (0x075258, i_075258),
    (0x07525A, i_07525a),
    (0x07525C, i_07525c),
    (0x075260, i_075260),
    (0x075262, i_075262),
    (0x075266, i_075266),
    (0x07526A, i_07526a),
    (0x07526E, i_07526e),
    (0x075EE8, i_075ee8),
    (0x075EEA, i_075eea),
    (0x075EF0, i_075ef0),
    (0x075EF4, i_075ef4),
    (0x075EFA, i_075efa),
    (0x075EFE, i_075efe),
    (0x075F02, i_075f02),
    (0x075F06, i_075f06),
    (0x075F0A, i_075f0a),
    (0x075F0E, i_075f0e),
    (0x075F12, i_075f12),
    (0x075F16, i_075f16),
    (0x075F1A, i_075f1a),
    (0x075F1C, i_075f1c),
    (0x075F20, i_075f20),
    (0x075F22, i_075f22),
    (0x075F26, i_075f26),
    (0x075F2A, i_075f2a),
    (0x075F2E, i_075f2e),
    (0x075F32, i_075f32),
    (0x075F44, i_075f44),
    (0x075F48, i_075f48),
    (0x075F4C, i_075f4c),
    (0x075F50, i_075f50),
    (0x075F54, i_075f54),
    (0x075F58, i_075f58),
    (0x075F5C, i_075f5c),
    (0x075F60, i_075f60),
    (0x075F62, i_075f62),
    (0x075F66, i_075f66),
    (0x075F6A, i_075f6a),
    (0x075F6E, i_075f6e),
    (0x075F72, i_075f72),
    (0x075F78, i_075f78),
    (0x075F7C, i_075f7c),
    (0x075F80, i_075f80),
    (0x075F86, i_075f86),
    (0x075F8C, i_075f8c),
    (0x075F92, i_075f92),
    (0x075F96, i_075f96),
    (0x075F9C, i_075f9c),
    (0x075FA0, i_075fa0),
    (0x075FA4, i_075fa4),
    (0x075FAA, i_075faa),
    (0x075FB0, i_075fb0),
    (0x075FB6, i_075fb6),
    (0x075FBC, i_075fbc),
    (0x075FC0, i_075fc0),
    (0x075FC6, i_075fc6),
    (0x075FC8, i_075fc8),
    (0x075FCC, i_075fcc),
    (0x075FD0, i_075fd0),
    (0x075FD2, i_075fd2),
    (0x075FD6, i_075fd6),
    (0x075FDC, i_075fdc),
    (0x075FE2, i_075fe2),
    (0x075FE6, i_075fe6),
    (0x075FEA, i_075fea),
    (0x075FF0, i_075ff0),
    (0x075FF6, i_075ff6),
    (0x075FFA, i_075ffa),
    (0x075FFC, i_075ffc),
    (0x076000, i_076000),
    (0x076006, i_076006),
    (0x07600A, i_07600a),
    (0x07600E, i_07600e),
    (0x076010, i_076010),
    (0x076012, i_076012),
    (0x076016, i_076016),
    (0x07601C, i_07601c),
    (0x076020, i_076020),
    (0x076026, i_076026),
    (0x07602A, i_07602a),
    (0x076030, i_076030),
    (0x076036, i_076036),
    (0x07603C, i_07603c),
    (0x076042, i_076042),
    (0x076046, i_076046),
    (0x07604A, i_07604a),
    (0x07604E, i_07604e),
    (0x076052, i_076052),
    (0x076054, i_076054),
    (0x076056, i_076056),
    (0x07605C, i_07605c),
    (0x076060, i_076060),
    (0x076066, i_076066),
    (0x07606A, i_07606a),
    (0x07606C, i_07606c),
    (0x076070, i_076070),
    (0x076076, i_076076),
    (0x07607A, i_07607a),
    (0x076080, i_076080),
    (0x076084, i_076084),
    (0x076088, i_076088),
    (0x07608A, i_07608a),
    (0x07608E, i_07608e),
    (0x076092, i_076092),
    (0x076098, i_076098),
    (0x07609A, i_07609a),
    (0x0760A0, i_0760a0),
    (0x0760A4, i_0760a4),
    (0x0760A6, i_0760a6),
    (0x0760A8, i_0760a8),
    (0x0760AE, i_0760ae),
    (0x0760B2, i_0760b2),
    (0x0760B8, i_0760b8),
    (0x0760BC, i_0760bc),
    (0x0760C0, i_0760c0),
    (0x0760C4, i_0760c4),
    (0x0760C6, i_0760c6),
    (0x0760C8, i_0760c8),
    (0x0760CE, i_0760ce),
    (0x0760D2, i_0760d2),
    (0x0760D8, i_0760d8),
    (0x0760DC, i_0760dc),
    (0x0760DE, i_0760de),
    (0x0760E2, i_0760e2),
    (0x0760E8, i_0760e8),
    (0x0760EC, i_0760ec),
    (0x0760F0, i_0760f0),
    (0x0760F2, i_0760f2),
    (0x0760F4, i_0760f4),
    (0x0760FA, i_0760fa),
    (0x076100, i_076100),
    (0x076104, i_076104),
    (0x07610A, i_07610a),
    (0x07610E, i_07610e),
    (0x076110, i_076110),
    (0x076114, i_076114),
    (0x07611A, i_07611a),
    (0x07611E, i_07611e),
    (0x076124, i_076124),
    (0x076128, i_076128),
    (0x07612E, i_07612e),
    (0x076132, i_076132),
    (0x076138, i_076138),
    (0x07613E, i_07613e),
    (0x076142, i_076142),
    (0x076146, i_076146),
    (0x07614A, i_07614a),
    (0x07614E, i_07614e),
    (0x076152, i_076152),
    (0x076158, i_076158),
    (0x07615C, i_07615c),
    (0x076164, i_076164),
    (0x076168, i_076168),
    (0x07616A, i_07616a),
    (0x07616E, i_07616e),
    (0x076174, i_076174),
    (0x076178, i_076178),
    (0x07617A, i_07617a),
    (0x07617E, i_07617e),
    (0x076184, i_076184),
    (0x076188, i_076188),
    (0x07618C, i_07618c),
    (0x076190, i_076190),
    (0x076194, i_076194),
    (0x076196, i_076196),
    (0x07619A, i_07619a),
    (0x0761A0, i_0761a0),
    (0x0761A4, i_0761a4),
    (0x0761A8, i_0761a8),
    (0x0761AC, i_0761ac),
    (0x0761B0, i_0761b0),
    (0x0761B4, i_0761b4),
    (0x0761B8, i_0761b8),
    (0x0761C0, i_0761c0),
    (0x0761C4, i_0761c4),
    (0x0761C8, i_0761c8),
    (0x0761CC, i_0761cc),
    (0x0761D2, i_0761d2),
    (0x0761D6, i_0761d6),
    (0x0761DC, i_0761dc),
    (0x0761E2, i_0761e2),
    (0x0761E6, i_0761e6),
    (0x0761EA, i_0761ea),
    (0x0761F0, i_0761f0),
    (0x0761F4, i_0761f4),
    (0x0761F8, i_0761f8),
    (0x0761FA, i_0761fa),
    (0x0761FE, i_0761fe),
    (0x076204, i_076204),
    (0x076208, i_076208),
    (0x07620C, i_07620c),
    (0x076210, i_076210),
    (0x076214, i_076214),
    (0x076218, i_076218),
    (0x07621C, i_07621c),
    (0x07621E, i_07621e),
    (0x076222, i_076222),
    (0x076226, i_076226),
    (0x076228, i_076228),
    (0x07622C, i_07622c),
    (0x076232, i_076232),
    (0x076236, i_076236),
    (0x07623A, i_07623a),
    (0x07623E, i_07623e),
    (0x076242, i_076242),
    (0x076246, i_076246),
    (0x07624C, i_07624c),
    (0x076250, i_076250),
    (0x076258, i_076258),
    (0x07625C, i_07625c),
    (0x076260, i_076260),
    (0x076264, i_076264),
    (0x07626A, i_07626a),
    (0x07626E, i_07626e),
    (0x076270, i_076270),
    (0x076274, i_076274),
    (0x07627A, i_07627a),
    (0x07627E, i_07627e),
    (0x076282, i_076282),
    (0x076286, i_076286),
    (0x07628A, i_07628a),
    (0x07628C, i_07628c),
    (0x076290, i_076290),
    (0x076296, i_076296),
    (0x07629A, i_07629a),
    (0x07629E, i_07629e),
    (0x0762A2, i_0762a2),
    (0x0762A6, i_0762a6),
    (0x0762AA, i_0762aa),
    (0x0762AE, i_0762ae),
    (0x0762B6, i_0762b6),
    (0x0762BA, i_0762ba),
    (0x0762BE, i_0762be),
    (0x0762C2, i_0762c2),
    (0x0762C8, i_0762c8),
    (0x0762CC, i_0762cc),
    (0x0762D2, i_0762d2),
    (0x0762D8, i_0762d8),
    (0x0762DC, i_0762dc),
    (0x0762E0, i_0762e0),
    (0x0762E6, i_0762e6),
    (0x0762EA, i_0762ea),
    (0x0762EE, i_0762ee),
    (0x0762F0, i_0762f0),
    (0x0762F4, i_0762f4),
    (0x0762FA, i_0762fa),
    (0x0762FE, i_0762fe),
    (0x076302, i_076302),
    (0x076306, i_076306),
    (0x07630A, i_07630a),
    (0x07630E, i_07630e),
    (0x076312, i_076312),
    (0x076314, i_076314),
    (0x076318, i_076318),
    (0x07631C, i_07631c),
    (0x07631E, i_07631e),
    (0x076322, i_076322),
];
