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
// ===== JsrAbsThunk_07685a  ($07685A, 8 B, thunks/jsr_abs_thunks.c)
/// $07685A: jsr $28d70.l
pub fn i_07685a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76860);
    cpu.pc = 0x28D70;
    16
}
/// $076860: rts
pub fn i_076860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76862;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_076924  ($076924, 8 B, kernel/task_handlers.c)
/// $076924: lea.l $7692c(pc), a1
pub fn i_076924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76928;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7692C;
    16
}
/// $076928: move.l a1, (a6)
pub fn i_076928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7692A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07692A: rts
pub fn i_07692a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7692C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_076952  ($076952, 8 B, thunks/jsr_abs_thunks.c)
/// $076952: jsr $28d70.l
pub fn i_076952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76958;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76958);
    cpu.pc = 0x28D70;
    16
}
/// $076958: rts
pub fn i_076958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7695A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_076a88  ($076A88, 8 B, kernel/task_handlers.c)
/// $076A88: lea.l $76a90(pc), a1
pub fn i_076a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76A90;
    16
}
/// $076A8C: move.l a1, (a6)
pub fn i_076a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076A8E: rts
pub fn i_076a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_076ad0  ($076AD0, 14 B, kernel/jsr5b6_jmp518.c)
/// $076AD0: jsr $5b6.l
pub fn i_076ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76AD6);
    cpu.pc = 0x5B6;
    16
}
/// $076AD6: jmp $518.l
pub fn i_076ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ADC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== SetTaskHandler_076b32  ($076B32, 8 B, kernel/task_handlers.c)
/// $076B32: lea.l $76b3a(pc), a1
pub fn i_076b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76B3A;
    16
}
/// $076B36: move.l a1, (a6)
pub fn i_076b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076B38: rts
pub fn i_076b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_076b9e  ($076B9E, 8 B, thunks/jsr_abs_thunks.c)
/// $076B9E: jsr $28d70.l
pub fn i_076b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76BA4);
    cpu.pc = 0x28D70;
    16
}
/// $076BA4: rts
pub fn i_076ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_076bc8  ($076BC8, 8 B, kernel/task_handlers.c)
/// $076BC8: lea.l $76bd0(pc), a1
pub fn i_076bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76BD0;
    16
}
/// $076BCC: move.l a1, (a6)
pub fn i_076bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076BCE: rts
pub fn i_076bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_076c6c  ($076C6C, 8 B, thunks/jsr_abs_thunks.c)
/// $076C6C: jsr $5ca2a.l
pub fn i_076c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76C72);
    cpu.pc = 0x5CA2A;
    16
}
/// $076C72: rts
pub fn i_076c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_076d0e  ($076D0E, 8 B, thunks/jsr_abs_thunks.c)
/// $076D0E: jsr $28d70.l
pub fn i_076d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76D14);
    cpu.pc = 0x28D70;
    16
}
/// $076D14: rts
pub fn i_076d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76D16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_Launch_076e10  ($076E10, 240 B, props/fx_explosions_platforms_crew_076000.s)
/// $076E10: move.w #$10ea, d0
pub fn i_076e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E14;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10EA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E14: jsr $2352.l
pub fn i_076e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E1A);
    cpu.pc = 0x2352;
    16
}
/// $076E1A: bset.b #$4, $6b(a6)
pub fn i_076e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E20;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $076E20: jsr $76f78(pc)
pub fn i_076e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E24;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76E24);
    cpu.pc = 0x76F78;
    16
}
/// $076E24: movea.l $7c(a6), a0
pub fn i_076e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E28;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076E28: jsr $5e086.l
pub fn i_076e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E2E);
    cpu.pc = 0x5E086;
    16
}
/// $076E2E: bcc.w $76e54
pub fn i_076e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76E54; return 10; }
    8
}
/// $076E32: move.w #$100, d0
pub fn i_076e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E36;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E36: jsr $5e9e4.l
pub fn i_076e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E3C);
    cpu.pc = 0x5E9E4;
    16
}
/// $076E3C: move.w d0, -(a7)
pub fn i_076e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E3E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $076E3E: move.w #$10, d0
pub fn i_076e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E42: jsr $5e9e4.l
pub fn i_076e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E48);
    cpu.pc = 0x5E9E4;
    16
}
/// $076E48: addi.w #$10, d0
pub fn i_076e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E4C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E4C: move.w d0, d1
pub fn i_076e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E4E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E4E: move.w (a7)+, d0
pub fn i_076e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E50;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $076E50: bra.w $76eae
pub fn i_076e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E54;
    cpu.ir = 0x6000;
    cpu.pc = 0x76EAE;
    10
}
/// $076E54: move.w $22(a0), d0
pub fn i_076e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E58;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076E58: sub.w $22(a6), d0
pub fn i_076e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E5C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $076E5C: move.w $24(a0), d1
pub fn i_076e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E60;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076E60: addi.w #$10, d1
pub fn i_076e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E64;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E64: sub.w $24(a6), d1
pub fn i_076e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E68;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $076E68: movem.w d0-d1, -(a7)
pub fn i_076e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E6C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $076E6C: jsr $5e9b6.l
pub fn i_076e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E72);
    cpu.pc = 0x5E9B6;
    16
}
/// $076E72: andi.w #$3, d0
pub fn i_076e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E76;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E76: bne.w $76e82
pub fn i_076e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E7A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76E82; return 10; }
    8
}
/// $076E7A: clr.w d2
pub fn i_076e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E7C;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076E7C: clr.w d3
pub fn i_076e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E7E;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076E7E: bra.w $76ea0
pub fn i_076e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E82;
    cpu.ir = 0x6000;
    cpu.pc = 0x76EA0;
    10
}
/// $076E82: jsr $5e9b6.l
pub fn i_076e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E88);
    cpu.pc = 0x5E9B6;
    16
}
/// $076E88: andi.w #$7f, d0
pub fn i_076e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E8C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E8C: subi.w #$3f, d0
pub fn i_076e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E90;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x3F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E90: move.w d0, -(a7)
pub fn i_076e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E92;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $076E92: jsr $5e9b6.l
pub fn i_076e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E98);
    cpu.pc = 0x5E9B6;
    16
}
/// $076E98: andi.w #$f, d0
pub fn i_076e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E9C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E9C: move.w d0, d3
pub fn i_076e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E9E;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E9E: move.w (a7)+, d2
pub fn i_076e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EA0;
    cpu.ir = 0x341F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $076EA0: movem.w (a7)+, d0-d1
pub fn i_076ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EA4;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $076EA4: add.w d2, d0
pub fn i_076ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EA6;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $076EA6: add.w d3, d1
pub fn i_076ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EA8;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $076EA8: bpl.w $76eae
pub fn i_076ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EAC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x76EAE; return 10; }
    8
}
/// $076EAC: clr.w d1
pub fn i_076eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EAE;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076EAE: movem.w d0-d1, -(a7)
pub fn i_076eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EB2;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $076EB2: lea.l $2bf310.l, a0
pub fn i_076eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF310;
    20
}
/// $076EB8: jsr $799de.l
pub fn i_076eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76EBE);
    cpu.pc = 0x799DE;
    16
}
/// $076EBE: move.w d0, d2
pub fn i_076ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EC0;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076EC0: movem.w (a7)+, d0-d1
pub fn i_076ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EC4;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $076EC4: bsr.w $76f9e
pub fn i_076ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EC8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76EC8);
    cpu.pc = 0x76F9E;
    18
}
/// $076EC8: move.w d0, $28(a6)
pub fn i_076ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ECC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076ECC: move.w d1, $2a(a6)
pub fn i_076ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ED0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076ED0: move.w d2, $2e(a6)
pub fn i_076ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ED4;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076ED4: move.w #$d000, d0
pub fn i_076ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ED8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076ED8: jsr $28134.l
pub fn i_076ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76EDE);
    cpu.pc = 0x28134;
    16
}
/// $076EDE: andi.w #$ffe3, $38(a6)
pub fn i_076ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EE4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $076EE4: ori.w #$14, $38(a6)
pub fn i_076ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EEA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $076EEA: lea.l $76d6a(pc), a0
pub fn i_076eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EEE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x76D6A;
    16
}
/// $076EEE: jsr $28cd4.l
pub fn i_076eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76EF4);
    cpu.pc = 0x28CD4;
    16
}
/// $076EF4: lea.l $76efa(pc), a1
pub fn i_076ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76EFA;
    16
}
/// $076EF8: move.l a1, (a6)
pub fn i_076ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076EFA: jsr $2783a.l
pub fn i_076efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F00);
    cpu.pc = 0x2783A;
    16
}
// ===== Frag_Fly_076f00  ($076F00, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $076F00: move.w $2e(a6), d0
pub fn i_076f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F04;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076F04: add.w d0, $2a(a6)
pub fn i_076f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F08;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $076F08: jsr $9c072.l
pub fn i_076f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F0E);
    cpu.pc = 0x9C072;
    16
}
/// $076F0E: jsr $28d70.l
pub fn i_076f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F14);
    cpu.pc = 0x28D70;
    16
}
/// $076F14: jsr $283d8.l
pub fn i_076f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F1A);
    cpu.pc = 0x283D8;
    16
}
/// $076F1A: cmpi.w #$140, $22(a6)
pub fn i_076f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F20;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $076F20: bcs.w $76f2a
pub fn i_076f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F24;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x76F2A; return 10; }
    8
}
/// $076F24: jmp $518.l
pub fn i_076f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F2A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $076F2A: move.w $24(a6), d0
pub fn i_076f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F2E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076F2E: subi.w #$100, d0
pub fn i_076f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F32;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076F32: cmpi.w #$140, d0
pub fn i_076f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F36;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x140, Size::Word, false);
    8
}
/// $076F36: bcs.w $76f40
pub fn i_076f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F3A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x76F40; return 10; }
    8
}
/// $076F3A: jmp $518.l
pub fn i_076f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F40;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $076F40: rts
pub fn i_076f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_Child_Run_076f42  ($076F42, 54 B, props/fx_explosions_platforms_crew_076000.s)
/// $076F42: jsr $76f78(pc)
pub fn i_076f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76F46);
    cpu.pc = 0x76F78;
    16
}
/// $076F46: lea.l $76d96(pc), a0
pub fn i_076f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F4A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x76D96;
    16
}
/// $076F4A: jsr $28cd4.l
pub fn i_076f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F50);
    cpu.pc = 0x28CD4;
    16
}
/// $076F50: lea.l $76f56(pc), a1
pub fn i_076f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76F56;
    16
}
/// $076F54: move.l a1, (a6)
pub fn i_076f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076F56: jsr $2783a.l
pub fn i_076f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F5C);
    cpu.pc = 0x2783A;
    16
}
/// $076F5C: movea.l $c(a6), a0
pub fn i_076f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076F60: move.w $14(a0), $14(a6)
pub fn i_076f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F66;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076F66: jsr $28d70.l
pub fn i_076f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F6C);
    cpu.pc = 0x28D70;
    16
}
/// $076F6C: bcc.w $76f76
pub fn i_076f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76F76; return 10; }
    8
}
/// $076F70: jmp $518.l
pub fn i_076f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F76;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $076F76: rts
pub fn i_076f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_PlayHitSounds_076f78  ($076F78, 38 B, props/fx_explosions_platforms_crew_076000.s)
/// $076F78: move.w #$1e1, d1
pub fn i_076f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F7C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076F7C: jsr $236e.l
pub fn i_076f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F82);
    cpu.pc = 0x236E;
    16
}
/// $076F82: move.w #$1e2, d1
pub fn i_076f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F86;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076F86: jsr $236e.l
pub fn i_076f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F8C);
    cpu.pc = 0x236E;
    16
}
/// $076F8C: move.w #$1e3, d1
pub fn i_076f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F90;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076F90: jsr $236e.l
pub fn i_076f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F96);
    cpu.pc = 0x236E;
    16
}
/// $076F96: move.w $16(a6), $14(a6)
pub fn i_076f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F9C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076F9C: rts
pub fn i_076f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_VelFromScale_076f9e  ($076F9E, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $076F9E: cmpi.w #$2, d2
pub fn i_076f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FA2;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $076FA2: bcc.w $76faa
pub fn i_076fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76FAA; return 10; }
    8
}
/// $076FA6: move.w #$2, d2
pub fn i_076fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FAA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076FAA: move.l #$10000, d4
pub fn i_076faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FB0;
    cpu.ir = 0x283C;
    let v1: u32 = 0x10000;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $076FB0: divu.w d2, d4
pub fn i_076fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FB2;
    cpu.ir = 0x88C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[4] / dv; let rm = cpu.d[4] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[4] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $076FB2: muls.w d4, d0
pub fn i_076fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FB4;
    cpu.ir = 0xC1C4;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $076FB4: asr.l #$8, d0
pub fn i_076fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FB6;
    cpu.ir = 0xE080;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[0] = r;
    22
}
/// $076FB6: move.w #$200, d2
pub fn i_076fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FBA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076FBA: mulu.w d4, d2
pub fn i_076fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FBC;
    cpu.ir = 0xC4C4;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[2] & 0xFFFF);
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $076FBC: lsr.l #$8, d2
pub fn i_076fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FBE;
    cpu.ir = 0xE08A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[2] = r;
    22
}
/// $076FBE: mulu.w d4, d2
pub fn i_076fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FC0;
    cpu.ir = 0xC4C4;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[2] & 0xFFFF);
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $076FC0: swap d2
pub fn i_076fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FC2;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $076FC2: neg.w d2
pub fn i_076fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FC4;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076FC4: addi.w #$100, d1
pub fn i_076fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FC8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076FC8: mulu.w d4, d1
pub fn i_076fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FCA;
    cpu.ir = 0xC2C4;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $076FCA: lsr.l #$8, d1
pub fn i_076fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FCC;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $076FCC: rts
pub fn i_076fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_RetGate_076fce  ($076FCE, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $076FCE: movea.l $8(a6), a1
pub fn i_076fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FD2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $076FD2: move.b $10(a6), d0
pub fn i_076fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FD6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $076FD6: cmp.b $10(a1), d0
pub fn i_076fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FDA;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $076FDA: bcs.w $76fe4
pub fn i_076fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FDE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x76FE4; return 10; }
    8
}
// ===== ClearXN_076fde  ($076FDE, 6 B, thunks/ccr_helpers.c)
/// $076FDE: andi.b #$ee, ccr
pub fn i_076fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FE2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $076FE2: rts
pub fn i_076fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_076fe4  ($076FE4, 6 B, thunks/ccr_helpers.c)
/// $076FE4: ori.b #$11, ccr
pub fn i_076fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FE8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $076FE8: rts
pub fn i_076fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_Register_076fea  ($076FEA, 26 B, props/fx_explosions_platforms_crew_076000.s)
/// $076FEA: move.w $30(a0), d0
pub fn i_076fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FEE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076FEE: cmpi.w #$30, d0
pub fn i_076fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FF2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $076FF2: bcc.w $7700a
pub fn i_076ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7700A; return 10; }
    8
}
/// $076FF6: lea.l (a0, d0.w), a1
pub fn i_076ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FFA;
    cpu.ir = 0x43F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $076FFA: lea.l $70(a6), a2
pub fn i_076ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FFE;
    cpu.ir = 0x45EE;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    cpu.a[2] = ad1;
    16
}
/// $076FFE: move.w (a2)+, (a1)+
pub fn i_076ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77000;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $077000: move.w (a2)+, (a1)+
pub fn i_077000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77002;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $077002: move.w (a2)+, (a1)+
pub fn i_077002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77004;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
// ===== Platform_AdvanceCount_077004  ($077004, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $077004: addq.w #$6, d0
pub fn i_077004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77006;
    cpu.ir = 0x5C40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077006: move.w d0, $30(a0)
pub fn i_077006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7700A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07700A: rts
pub fn i_07700a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7700C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_FindNearestX_07700c  ($07700C, 112 B, props/fx_explosions_platforms_crew_076000.s)
/// $07700C: clr.w d7
pub fn i_07700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7700E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07700E: moveq #$ff, d6
pub fn i_07700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77010;
    cpu.ir = 0x7CFF;
    cpu.d[6] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077010: move.w d6, d5
pub fn i_077010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77012;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077012: lea.l (a0), a5
pub fn i_077012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77014;
    cpu.ir = 0x4BD0;
    cpu.a[5] = cpu.a[0];
    12
}
/// $077014: cmp.w $30(a0), d7
pub fn i_077014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77018;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $077018: bcc.w $77054
pub fn i_077018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7701C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77054; return 10; }
    8
}
/// $07701C: move.w $2(a5, d7.w), d2
pub fn i_07701c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77020;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $077020: sub.w d1, d2
pub fn i_077020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77022;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077022: clr.w d3
pub fn i_077022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77024;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077024: move.b $4(a5, d7.w), d3
pub fn i_077024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77028;
    cpu.ir = 0x1635;
    let ad2 = cpu.a[5].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    14
}
/// $077028: cmp.w d3, d2
pub fn i_077028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7702A;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $07702A: bhi.w $77050
pub fn i_07702a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7702E;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x77050; return 10; }
    8
}
/// $07702E: lsr.w #$3, d2
pub fn i_07702e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77030;
    cpu.ir = 0xE64A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077030: neg.w d2
pub fn i_077030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77032;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077032: add.w (a5, d7.w), d2
pub fn i_077032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77036;
    cpu.ir = 0xD475;
    let ad2 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $077036: move.w d2, d3
pub fn i_077036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77038;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077038: sub.w d0, d3
pub fn i_077038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703A;
    cpu.ir = 0x9640;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07703A: smi.b d4
pub fn i_07703a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703C;
    cpu.ir = 0x5BC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07703C: ext.w d4
pub fn i_07703c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703E;
    cpu.ir = 0x4884;
    let v = cpu.d[4] as i8 as i16 as u16 as u32; cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07703E: eor.w d4, d3
pub fn i_07703e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77040;
    cpu.ir = 0xB943;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077040: sub.w d4, d3
pub fn i_077040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77042;
    cpu.ir = 0x9644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077042: cmp.w d3, d5
pub fn i_077042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77044;
    cpu.ir = 0xBA43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $077044: bls.w $77050
pub fn i_077044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77048;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x77050; return 10; }
    8
}
/// $077048: move.w d3, d5
pub fn i_077048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7704A;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07704A: move.w d2, d6
pub fn i_07704a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7704C;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07704C: lea.l (a5, d7.w), a1
pub fn i_07704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77050;
    cpu.ir = 0x43F5;
    let ad1 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $077050: addq.w #$6, d7
pub fn i_077050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77052;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077052: bra.b $77014
pub fn i_077052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77054;
    cpu.ir = 0x60C0;
    cpu.pc = 0x77014;
    10
}
/// $077054: rts
pub fn i_077054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77056;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $077056: moveq #$0, d7
pub fn i_077056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77058;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077058: movea.l d7, a1
pub fn i_077058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7705A;
    cpu.ir = 0x2247;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $07705A: moveq #$ff, d6
pub fn i_07705a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7705C;
    cpu.ir = 0x7CFF;
    cpu.d[6] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07705C: lea.l (a0), a5
pub fn i_07705c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7705E;
    cpu.ir = 0x4BD0;
    cpu.a[5] = cpu.a[0];
    12
}
/// $07705E: cmp.w $30(a0), d7
pub fn i_07705e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77062;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $077062: bcc.w $7707a
pub fn i_077062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77066;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7707A; return 10; }
    8
}
/// $077066: cmp.b $5(a5, d7.w), d0
pub fn i_077066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7706A;
    cpu.ir = 0xB035;
    let ad2 = cpu.a[5].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $07706A: bne.w $77076
pub fn i_07706a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7706E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77076; return 10; }
    8
}
/// $07706E: lea.l (a5, d7.w), a1
pub fn i_07706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77072;
    cpu.ir = 0x43F5;
    let ad1 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $077072: bra.w $7707a
pub fn i_077072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77076;
    cpu.ir = 0x6000;
    cpu.pc = 0x7707A;
    10
}
/// $077076: addq.w #$6, d7
pub fn i_077076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77078;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077078: bra.b $7705e
pub fn i_077078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7707A;
    cpu.ir = 0x60E4;
    cpu.pc = 0x7705E;
    10
}
/// $07707A: rts
pub fn i_07707a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7707C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ListsInit_07707c  ($07707C, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $07707C: lea.l $10e27c.l, a0
pub fn i_07707c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77082;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $077082: clr.w $30(a0)
pub fn i_077082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77086;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $077086: lea.l $10e2ae.l, a0
pub fn i_077086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7708C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $07708C: clr.w $30(a0)
pub fn i_07708c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77090;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $077090: rts
pub fn i_077090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77092;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ClearListByScroll_077092  ($077092, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $077092: lea.l $10e27c.l, a0
pub fn i_077092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77098;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $077098: btst.b #$0, $106f28.l
pub fn i_077098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770A0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0770A0: beq.w $770aa
pub fn i_0770a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x770AA; return 10; }
    8
}
/// $0770A4: lea.l $10e2ae.l, a0
pub fn i_0770a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0770AA: clr.w $30(a0)
pub fn i_0770aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770AE;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0770AE: rts
pub fn i_0770ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0770B0: lea.l $10e27c.l, a0
pub fn i_0770b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $0770B6: btst.b #$0, $106f28.l
pub fn i_0770b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770BE;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0770BE: beq.w $770c8
pub fn i_0770be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x770C8; return 10; }
    8
}
/// $0770C2: lea.l $10e2ae.l, a0
pub fn i_0770c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0770C8: bra.w $76fea
pub fn i_0770c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x76FEA;
    10
}
// ===== Platform_ProbeUnderFeet_0770cc  ($0770CC, 120 B, props/fx_explosions_platforms_crew_076000.s)
/// $0770CC: lea.l $10e27c.l, a0
pub fn i_0770cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $0770D2: btst.b #$0, $106f28.l
pub fn i_0770d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770DA;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0770DA: bne.w $770e4
pub fn i_0770da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x770E4; return 10; }
    8
}
/// $0770DE: lea.l $10e2ae.l, a0
pub fn i_0770de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0770E4: move.w $22(a6), d0
pub fn i_0770e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770E8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0770E8: move.w $24(a6), d1
pub fn i_0770e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770EC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0770EC: add.w $106f50.l, d0
pub fn i_0770ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F2;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0770F2: subi.w #$200, d1
pub fn i_0770f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F6;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0770F6: neg.w d1
pub fn i_0770f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F8;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0770F8: add.w $106f54.l, d1
pub fn i_0770f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770FE;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0770FE: bsr.w $7700c
pub fn i_0770fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77102;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77102);
    cpu.pc = 0x7700C;
    18
}
/// $077102: addq.w #$1, d5
pub fn i_077102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77104;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077104: beq.w $77144
pub fn i_077104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77108;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77144; return 10; }
    8
}
/// $077108: clr.w d2
pub fn i_077108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7710A;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07710A: move.b $26(a6), d2
pub fn i_07710a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7710E;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07710E: add.w $28(a6), d2
pub fn i_07710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77112;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077112: asr.w #$8, d2
pub fn i_077112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77114;
    cpu.ir = 0xE042;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $077114: add.w d0, d2
pub fn i_077114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77116;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077116: sub.w d6, d2
pub fn i_077116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77118;
    cpu.ir = 0x9446;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077118: smi.b d2
pub fn i_077118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711A;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07711A: sub.w d6, d0
pub fn i_07711a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711C;
    cpu.ir = 0x9046;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07711C: smi.b d0
pub fn i_07711c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711E;
    cpu.ir = 0x5BC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07711E: eor.b d2, d0
pub fn i_07711e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77120;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $077120: beq.w $77144
pub fn i_077120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77124;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77144; return 10; }
    8
}
/// $077124: clr.w d4
pub fn i_077124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77126;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077126: clr.w d2
pub fn i_077126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77128;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077128: move.b $4(a1), d2
pub fn i_077128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7712C;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07712C: lsr.w #$1, d2
pub fn i_07712c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7712E;
    cpu.ir = 0xE24A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07712E: neg.w d2
pub fn i_07712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77130;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077130: add.w $2(a1), d2
pub fn i_077130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77134;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077134: cmp.w d2, d1
pub fn i_077134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77136;
    cpu.ir = 0xB242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $077136: spl.b d1
pub fn i_077136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77138;
    cpu.ir = 0x5AC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $077138: addi.b #$2, d1
pub fn i_077138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7713C;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07713C: clr.w d0
pub fn i_07713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7713E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07713E: move.b $5(a1), d0
pub fn i_07713e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77142;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $077142: rts
pub fn i_077142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77144;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RetMinus1_00077144  ($077144, 4 B, thunks/trivial_returns.c)
/// $077144: moveq #$ff, d0
pub fn i_077144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77146;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077146: rts
pub fn i_077146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ProbeNearest_077148  ($077148, 72 B, props/fx_explosions_platforms_crew_076000.s)
/// $077148: lea.l $10e27c.l, a0
pub fn i_077148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7714E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $07714E: btst.b #$0, $106f28.l
pub fn i_07714e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77156;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $077156: bne.w $77160
pub fn i_077156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7715A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77160; return 10; }
    8
}
/// $07715A: lea.l $10e2ae.l, a0
pub fn i_07715a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77160;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $077160: move.w $22(a6), d0
pub fn i_077160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77164;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077164: move.w $24(a6), d1
pub fn i_077164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77168;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077168: add.w $106f50.l, d0
pub fn i_077168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7716E;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $07716E: subi.w #$200, d1
pub fn i_07716e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77172;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $077172: neg.w d1
pub fn i_077172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77174;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077174: add.w $106f54.l, d1
pub fn i_077174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7717A;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $07717A: bsr.w $7700c
pub fn i_07717a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7717E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x7717E);
    cpu.pc = 0x7700C;
    18
}
/// $07717E: addq.w #$1, d5
pub fn i_07717e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77180;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077180: beq.b $77144
pub fn i_077180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77182;
    cpu.ir = 0x67C2;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77144; return 10; }
    8
}
/// $077182: sub.w $106f50.l, d6
pub fn i_077182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77188;
    cpu.ir = 0x9C79;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $077188: clr.w d0
pub fn i_077188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7718A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07718A: move.b $5(a1), d0
pub fn i_07718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7718E;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07718E: rts
pub fn i_07718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_SnapToId_077190  ($077190, 148 B, props/fx_explosions_platforms_crew_076000.s)
/// $077190: lea.l $10e27c.l, a0
pub fn i_077190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77196;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $077196: btst.b #$0, $106f28.l
pub fn i_077196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7719E;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $07719E: bne.w $771a8
pub fn i_07719e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x771A8; return 10; }
    8
}
/// $0771A2: lea.l $10e2ae.l, a0
pub fn i_0771a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0771A8: bsr.w $77056
pub fn i_0771a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771AC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x771AC);
    cpu.pc = 0x77056;
    18
}
/// $0771AC: move.l a1, d0
pub fn i_0771ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771AE;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0771AE: beq.w $7721a
pub fn i_0771ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7721A; return 10; }
    8
}
/// $0771B2: clr.w -(a7)
pub fn i_0771b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771B4;
    cpu.ir = 0x4267;
    let ad1 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad1;
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    10
}
/// $0771B4: move.w $2(a1), d0
pub fn i_0771b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771B8;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0771B8: sub.w $106f54.l, d0
pub fn i_0771b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771BE;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0771BE: neg.w d0
pub fn i_0771be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771C0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771C0: addi.w #$200, d0
pub fn i_0771c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771C4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0771C4: clr.w d1
pub fn i_0771c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771C6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771C6: move.b $4(a1), d1
pub fn i_0771c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771CA;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0771CA: add.w d0, d1
pub fn i_0771ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771CC;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0771CC: addq.w #$1, d1
pub fn i_0771cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771CE;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771CE: move.w d1, (a7)
pub fn i_0771ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771D0;
    cpu.ir = 0x3E81;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $0771D0: clr.w d1
pub fn i_0771d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771D2;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771D2: move.b $27(a6), d1
pub fn i_0771d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771D6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0771D6: add.w $2a(a6), d1
pub fn i_0771d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771DA;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0771DA: add.w $2e(a6), d1
pub fn i_0771da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771DE;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0771DE: asr.w #$8, d1
pub fn i_0771de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771E0;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0771E0: add.w $24(a6), d1
pub fn i_0771e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771E4;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0771E4: sub.w d0, d1
pub fn i_0771e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771E6;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0771E6: cmpi.w #$fff8, d1
pub fn i_0771e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771EA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF8, Size::Word, false);
    8
}
/// $0771EA: bge.w $771f2
pub fn i_0771ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771EE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x771F2; return 10; }
    8
}
/// $0771EE: move.w #$ffff, (a7)
pub fn i_0771ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771F2;
    cpu.ir = 0x3EBC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $0771F2: neg.w d1
pub fn i_0771f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771F4;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771F4: asr.w #$3, d1
pub fn i_0771f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771F6;
    cpu.ir = 0xE641;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0771F6: add.w (a1), d1
pub fn i_0771f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771F8;
    cpu.ir = 0xD251;
    let v2 = bus.read16(cpu.a[1]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0771F8: sub.w $106f50.l, d1
pub fn i_0771f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771FE;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0771FE: sub.w $22(a6), d1
pub fn i_0771fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77202;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077202: lsl.w #$8, d1
pub fn i_077202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77204;
    cpu.ir = 0xE149;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $077204: move.w d1, $28(a6)
pub fn i_077204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77208;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077208: clr.w $2c(a6)
pub fn i_077208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7720C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07720C: jsr $27d50.l
pub fn i_07720c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77212);
    cpu.pc = 0x27D50;
    16
}
/// $077212: clr.w $28(a6)
pub fn i_077212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77216;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $077216: move.w (a7)+, d0
pub fn i_077216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77218;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $077218: rts
pub fn i_077218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7721A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07721A: jsr $27d50.l
pub fn i_07721a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77220);
    cpu.pc = 0x27D50;
    16
}
/// $077220: moveq #$ff, d0
pub fn i_077220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77222;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077222: rts
pub fn i_077222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77224;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MovingPlatform_Tmpl143_077224  ($077224, 4 B, props/fx_explosions_platforms_crew_076000.s)
/// $077224: clr.b $99(a6)
pub fn i_077224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77228;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== MovingPlatform_Init_077228  ($077228, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $077228: move.w $22(a6), d0
pub fn i_077228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7722C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07722C: add.w $106f50.l, d0
pub fn i_07722c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77232;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $077232: move.w d0, $70(a6)
pub fn i_077232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77236;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077236: move.w $24(a6), d0
pub fn i_077236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7723A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07723A: subi.w #$200, d0
pub fn i_07723a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7723E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07723E: neg.w d0
pub fn i_07723e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77240;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077240: add.w $106f54.l, d0
pub fn i_077240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77246;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $077246: move.w d0, $72(a6)
pub fn i_077246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7724A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07724A: move.b $98(a6), d0
pub fn i_07724a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7724E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07724E: move.b d0, $74(a6)
pub fn i_07724e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77252;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077252: lea.l $10e2e0.l, a0
pub fn i_077252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77258;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2E0;
    20
}
/// $077258: move.b (a0), d0
pub fn i_077258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7725A;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $07725A: addq.b #$1, d0
pub fn i_07725a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7725C;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07725C: move.b d0, (a0)
pub fn i_07725c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7725E;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $07725E: move.b d0, $75(a6)
pub fn i_07725e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77262;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_077262  ($077262, 8 B, kernel/task_handlers.c)
/// $077262: lea.l $7726a(pc), a1
pub fn i_077262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77266;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7726A;
    16
}
/// $077266: move.l a1, (a6)
pub fn i_077266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77268;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077268: rts
pub fn i_077268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7726A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MovingPlatform_Run_07726a  ($07726A, 62 B, props/fx_explosions_platforms_crew_076000.s)
/// $07726A: move.w $70(a6), d0
pub fn i_07726a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7726E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07726E: sub.w $106f50.l, d0
pub fn i_07726e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77274;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $077274: addi.w #$40, d0
pub fn i_077274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77278;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $077278: subi.w #$380, d0
pub fn i_077278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7727C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x380, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07727C: bcs.w $77286
pub fn i_07727c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77280;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x77286; return 10; }
    8
}
/// $077280: jmp $518.l
pub fn i_077280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77286;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $077286: moveq #$0, d0
pub fn i_077286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77288;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077288: move.b $99(a6), d0
pub fn i_077288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7728C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07728C: beq.w $772a2
pub fn i_07728c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77290;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x772A2; return 10; }
    8
}
/// $077290: movea.l $c(a6), a0
pub fn i_077290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77294;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077294: tst.b (a0, d0.w)
pub fn i_077294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77298;
    cpu.ir = 0x4A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $077298: beq.w $772a2
pub fn i_077298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7729C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x772A2; return 10; }
    8
}
/// $07729C: jmp $518.l
pub fn i_07729c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x772A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0772A2: bsr.w $770b0
pub fn i_0772a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x772A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x772A6);
    cpu.pc = 0x770B0;
    18
}
/// $0772A6: rts
pub fn i_0772a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x772A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BridgePlank_PosFromRecord_07749e  ($07749E, 78 B, props/fx_explosions_platforms_crew_076000.s)
/// $07749E: movea.l $70(a6), a0
pub fn i_07749e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774A2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0774A2: move.w $8(a0), d0
pub fn i_0774a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774A6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0774A6: move.w $a(a0), d1
pub fn i_0774a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774AA;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0774AA: lsl.w #$4, d0
pub fn i_0774aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774AC;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0774AC: lsl.w #$4, d1
pub fn i_0774ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774AE;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0774AE: jsr $44022.l
pub fn i_0774ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x774B4);
    cpu.pc = 0x44022;
    16
}
/// $0774B4: move.w d0, $22(a6)
pub fn i_0774b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774B8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0774B8: move.w d1, $24(a6)
pub fn i_0774b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774BC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0774BC: rts
pub fn i_0774bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BridgePlank_Spawn0_0774ec  ($0774EC, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $0774EC: move.l #$7730a, $70(a6)
pub fn i_0774ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774F4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7730A;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0774F4: bsr.b $7749e
pub fn i_0774f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774F6;
    cpu.ir = 0x61A8;
    cpu.push32(bus, 0x774F6);
    cpu.pc = 0x7749E;
    18
}
/// $0774F6: subi.w #$10, $24(a6)
pub fn i_0774f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774FC;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0774FC: bra.w $7757c
pub fn i_0774fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77500;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn1_077500  ($077500, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $077500: move.l #$77332, $70(a6)
pub fn i_077500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77508;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x77332;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077508: bsr.b $7749e
pub fn i_077508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7750A;
    cpu.ir = 0x6194;
    cpu.push32(bus, 0x7750A);
    cpu.pc = 0x7749E;
    18
}
/// $07750A: subi.w #$20, $24(a6)
pub fn i_07750a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77510;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $077510: bra.w $7757c
pub fn i_077510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77514;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn2_077514  ($077514, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $077514: move.l #$7735a, $70(a6)
pub fn i_077514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7751C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7735A;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07751C: bsr.b $7749e
pub fn i_07751c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7751E;
    cpu.ir = 0x6180;
    cpu.push32(bus, 0x7751E);
    cpu.pc = 0x7749E;
    18
}
/// $07751E: subi.w #$20, $24(a6)
pub fn i_07751e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77524;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $077524: bra.w $7757c
pub fn i_077524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77528;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn3_077528  ($077528, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $077528: move.l #$77382, $70(a6)
pub fn i_077528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77530;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x77382;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077530: bsr.w $7749e
pub fn i_077530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77534;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77534);
    cpu.pc = 0x7749E;
    18
}
/// $077534: subi.w #$20, $24(a6)
pub fn i_077534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7753A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07753A: bra.w $7757c
pub fn i_07753a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7753E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn4_07753e  ($07753E, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $07753E: move.l #$773aa, $70(a6)
pub fn i_07753e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77546;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x773AA;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077546: bsr.w $7749e
pub fn i_077546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7754A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x7754A);
    cpu.pc = 0x7749E;
    18
}
/// $07754A: subi.w #$10, $24(a6)
pub fn i_07754a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77550;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $077550: bra.w $7757c
pub fn i_077550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77554;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn5_077554  ($077554, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $077554: move.l #$773d2, $70(a6)
pub fn i_077554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7755C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x773D2;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07755C: bsr.w $7749e
pub fn i_07755c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77560;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77560);
    cpu.pc = 0x7749E;
    18
}
/// $077560: subi.w #$20, $24(a6)
pub fn i_077560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77566;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $077566: bra.w $7757c
pub fn i_077566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7756A;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn6_07756a  ($07756A, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $07756A: move.l #$773fa, $70(a6)
pub fn i_07756a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77572;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x773FA;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077572: bsr.w $7749e
pub fn i_077572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77576;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77576);
    cpu.pc = 0x7749E;
    18
}
/// $077576: subi.w #$20, $24(a6)
pub fn i_077576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7757C;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07757C: lea.l $ffff.w, a0
pub fn i_07757c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77580;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $077580: move.l a0, $48(a6)
pub fn i_077580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77584;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077584: move.w #$3c, $66(a6)
pub fn i_077584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7758A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07758A: lea.l $77590(pc), a1
pub fn i_07758a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7758E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77590;
    16
}
/// $07758E: move.l a1, (a6)
pub fn i_07758e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77590;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077590: jsr $2783a.l
pub fn i_077590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77596);
    cpu.pc = 0x2783A;
    16
}
/// $077596: move.w $22(a6), d0
pub fn i_077596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7759A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07759A: bmi.w $775a0
pub fn i_07759a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7759E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x775A0; return 10; }
    8
}
/// $07759E: rts
pub fn i_07759e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0775A0: jsr $5b6.l
pub fn i_0775a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775A6);
    cpu.pc = 0x5B6;
    16
}
/// $0775A6: jmp $518.l
pub fn i_0775a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775AC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== MuzzleFx_RetGate_0775ac  ($0775AC, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $0775AC: movea.l $8(a6), a1
pub fn i_0775ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775B0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0775B0: move.b $10(a6), d0
pub fn i_0775b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775B4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0775B4: cmp.b $10(a1), d0
pub fn i_0775b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775B8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0775B8: bcs.w $775c2
pub fn i_0775b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775BC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x775C2; return 10; }
    8
}
// ===== ClearXN_0775bc  ($0775BC, 6 B, thunks/ccr_helpers.c)
/// $0775BC: andi.b #$ee, ccr
pub fn i_0775bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775C0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0775C0: rts
pub fn i_0775c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0775c2  ($0775C2, 6 B, thunks/ccr_helpers.c)
/// $0775C2: ori.b #$11, ccr
pub fn i_0775c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775C6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0775C6: rts
pub fn i_0775c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SpawnC_0775c8  ($0775C8, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $0775C8: lea.l $776e2(pc), a1
pub fn i_0775c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x776E2;
    16
}
/// $0775CC: jsr $4ae.l
pub fn i_0775cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775D2);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_0775d2  ($0775D2, 8 B, thunks/jsr_abs_thunks.c)
/// $0775D2: jsr $5dd02.l
pub fn i_0775d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775D8);
    cpu.pc = 0x5DD02;
    16
}
/// $0775D8: rts
pub fn i_0775d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SpawnB_0775da  ($0775DA, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $0775DA: lea.l $776b4(pc), a1
pub fn i_0775da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x776B4;
    16
}
/// $0775DE: jsr $4ae.l
pub fn i_0775de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775E4);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_0775e4  ($0775E4, 8 B, thunks/jsr_abs_thunks.c)
/// $0775E4: jsr $5dd02.l
pub fn i_0775e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775EA);
    cpu.pc = 0x5DD02;
    16
}
/// $0775EA: rts
pub fn i_0775ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SceneGate_077640  ($077640, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $077640: cmpi.b #$0, $106ece.l
pub fn i_077640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77648;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $077648: beq.w $7765e
pub fn i_077648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7764C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7765E; return 10; }
    8
}
/// $07764C: cmpi.b #$9, $106ece.l
pub fn i_07764c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77654;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x9, Size::Byte, false);
    20
}
/// $077654: beq.w $7765e
pub fn i_077654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77658;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7765E; return 10; }
    8
}
/// $077658: jmp $518.l
pub fn i_077658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7765E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07765E: jsr $77670(pc)
pub fn i_07765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77662;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77662);
    cpu.pc = 0x77670;
    16
}
/// $077662: jsr $236e.l
pub fn i_077662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77668;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77668);
    cpu.pc = 0x236E;
    16
}
/// $077668: move.w #$e000, $38(a6)
pub fn i_077668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7766E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07766E: rts
pub fn i_07766e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77670;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SndByScene_077670  ($077670, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $077670: move.w #$10, d1
pub fn i_077670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77674;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077674: cmpi.b #$5, $106ece.l
pub fn i_077674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7767C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x5, Size::Byte, false);
    20
}
/// $07767C: bne.w $77684
pub fn i_07767c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77680;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77684; return 10; }
    8
}
/// $077680: move.w #$138, d1
pub fn i_077680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77684;
    cpu.ir = 0x323C;
    let v1: u32 = 0x138;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077684: rts
pub fn i_077684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77686;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_InitA_077686  ($077686, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $077686: jsr $77640(pc)
pub fn i_077686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7768A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7768A);
    cpu.pc = 0x77640;
    16
}
/// $07768A: move.l #$2dcb84, $74(a6)
pub fn i_07768a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77692;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCB84;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077692: move.l #$2dca62, $78(a6)
pub fn i_077692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7769A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA62;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07769A: move.l #$2dca82, $7c(a6)
pub fn i_07769a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776A2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA82;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776A2: move.l #$2dca1a, $80(a6)
pub fn i_0776a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776AA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA1A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776AA: lea.l $778a2(pc), a1
pub fn i_0776aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $0776AE: move.l a1, (a6)
pub fn i_0776ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0776B0: bra.w $778a2
pub fn i_0776b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitB_0776b4  ($0776B4, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $0776B4: jsr $77640(pc)
pub fn i_0776b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x776B8);
    cpu.pc = 0x77640;
    16
}
/// $0776B8: move.l #$2dca8a, $74(a6)
pub fn i_0776b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776C0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA8A;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776C0: move.l #$2dca4a, $78(a6)
pub fn i_0776c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776C8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA4A;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776C8: move.l #$2dca6a, $7c(a6)
pub fn i_0776c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776D0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA6A;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776D0: move.l #$2dc98a, $80(a6)
pub fn i_0776d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776D8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DC98A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776D8: lea.l $778a2(pc), a1
pub fn i_0776d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $0776DC: move.l a1, (a6)
pub fn i_0776dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0776DE: bra.w $778a2
pub fn i_0776de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitC_0776e2  ($0776E2, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $0776E2: jsr $77640(pc)
pub fn i_0776e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x776E6);
    cpu.pc = 0x77640;
    16
}
/// $0776E6: move.l #$2dcabc, $74(a6)
pub fn i_0776e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776EE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCABC;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776EE: move.l #$2dca52, $78(a6)
pub fn i_0776ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776F6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA52;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776F6: move.l #$2dca72, $7c(a6)
pub fn i_0776f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776FE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA72;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776FE: move.l #$2dc9ba, $80(a6)
pub fn i_0776fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77706;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DC9BA;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077706: lea.l $778a2(pc), a1
pub fn i_077706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7770A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $07770A: move.l a1, (a6)
pub fn i_07770a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7770C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07770C: bra.w $778a2
pub fn i_07770c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77710;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitD_077710  ($077710, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $077710: jsr $77640(pc)
pub fn i_077710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77714;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77714);
    cpu.pc = 0x77640;
    16
}
/// $077714: move.l #$2dcaee, $74(a6)
pub fn i_077714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7771C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCAEE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07771C: move.l #$2dca5a, $78(a6)
pub fn i_07771c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77724;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA5A;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077724: move.l #$2dca7a, $7c(a6)
pub fn i_077724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7772C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA7A;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07772C: move.l #$2dc9ea, $80(a6)
pub fn i_07772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77734;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DC9EA;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077734: lea.l $778a2(pc), a1
pub fn i_077734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77738;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $077738: move.l a1, (a6)
pub fn i_077738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7773A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07773A: bra.w $778a2
pub fn i_07773a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7773E;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitE_07773e  ($07773E, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $07773E: jsr $77640(pc)
pub fn i_07773e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77742);
    cpu.pc = 0x77640;
    16
}
/// $077742: move.l #$2dcb84, $74(a6)
pub fn i_077742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7774A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCB84;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07774A: move.l #$2dca62, $78(a6)
pub fn i_07774a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77752;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA62;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077752: move.l #$2dca82, $7c(a6)
pub fn i_077752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7775A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA82;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07775A: move.l #$2dca1a, $80(a6)
pub fn i_07775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77762;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA1A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077762: lea.l $778a2(pc), a1
pub fn i_077762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $077766: move.l a1, (a6)
pub fn i_077766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077768: bra.w $778a2
pub fn i_077768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7776C;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_Attach_07776c  ($07776C, 302 B, props/fx_explosions_platforms_crew_076000.s)
/// $07776C: move.w #$ffff, $70(a6)
pub fn i_07776c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77772;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077772: lea.l $77778(pc), a1
pub fn i_077772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77776;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77778;
    16
}
/// $077776: move.l a1, (a6)
pub fn i_077776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77778;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077778: jsr $77c0c(pc)
pub fn i_077778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7777C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7777C);
    cpu.pc = 0x77C0C;
    16
}
/// $07777C: jsr $77bc6(pc)
pub fn i_07777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77780;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77780);
    cpu.pc = 0x77BC6;
    16
}
/// $077780: movea.l $c(a6), a2
pub fn i_077780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77784;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077784: move.w $22(a2), d1
pub fn i_077784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77788;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077788: move.w $24(a2), d2
pub fn i_077788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7778C;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07778C: move.w d1, $22(a6)
pub fn i_07778c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77790;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077790: move.w d2, $24(a6)
pub fn i_077790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77794;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077794: jsr $77c28(pc)
pub fn i_077794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77798;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77798);
    cpu.pc = 0x77C28;
    16
}
/// $077798: clr.w d0
pub fn i_077798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7779A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07779A: tst.w $28(a2)
pub fn i_07779a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7779E;
    cpu.ir = 0x4A6A;
    let ad2 = cpu.a[2].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07779E: beq.w $777b2
pub fn i_07779e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x777B2; return 10; }
    8
}
/// $0777A2: move.w #$4, d0
pub fn i_0777a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0777A6: tst.w $28(a2)
pub fn i_0777a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777AA;
    cpu.ir = 0x4A6A;
    let ad2 = cpu.a[2].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0777AA: bge.w $777b2
pub fn i_0777aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777AE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x777B2; return 10; }
    8
}
/// $0777AE: move.w #$8, d0
pub fn i_0777ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0777B2: cmpi.b #$80, $72(a6)
pub fn i_0777b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777B8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $0777B8: bne.w $777fc
pub fn i_0777b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x777FC; return 10; }
    8
}
/// $0777BC: cmpi.b #$82, $88(a6)
pub fn i_0777bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777C2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x82, Size::Byte, false);
    16
}
/// $0777C2: beq.w $777d0
pub fn i_0777c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x777D0; return 10; }
    8
}
/// $0777C6: cmpi.b #$8c, $88(a6)
pub fn i_0777c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777CC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8C, Size::Byte, false);
    16
}
/// $0777CC: bne.w $777d2
pub fn i_0777cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x777D2; return 10; }
    8
}
/// $0777D0: addq.w #$2, d0
pub fn i_0777d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777D2;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0777D2: movem.w d0, -(a7)
pub fn i_0777d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777D6;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $0777D6: move.w $22(a6), d0
pub fn i_0777d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777DA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0777DA: move.w $24(a6), d1
pub fn i_0777da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777DE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0777DE: jsr $440bc.l
pub fn i_0777de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x777E4);
    cpu.pc = 0x440BC;
    16
}
/// $0777E4: move.w d0, d2
pub fn i_0777e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777E6;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0777E6: movem.w (a7)+, d0
pub fn i_0777e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777EA;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $0777EA: cmpi.w #$d20, d2
pub fn i_0777ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777EE;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xD20, Size::Word, false);
    8
}
/// $0777EE: bcs.w $777fc
pub fn i_0777ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777F2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x777FC; return 10; }
    8
}
/// $0777F2: cmpi.w #$e58, d2
pub fn i_0777f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777F6;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE58, Size::Word, false);
    8
}
/// $0777F6: bcc.w $777fc
pub fn i_0777f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x777FC; return 10; }
    8
}
/// $0777FA: addq.w #$1, d0
pub fn i_0777fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777FC;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0777FC: cmp.w $70(a6), d0
pub fn i_0777fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77800;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $077800: beq.w $77822
pub fn i_077800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77804;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77822; return 10; }
    8
}
/// $077804: move.w d0, $70(a6)
pub fn i_077804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77808;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077808: movea.l $80(a6), a0
pub fn i_077808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7780C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07780C: lsl.w #$2, d0
pub fn i_07780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7780E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07780E: movea.l (a0, d0.w), a0
pub fn i_07780e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77812;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $077812: cmpa.l #$ffffffff, a0
pub fn i_077812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77818;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $077818: beq.w $77822
pub fn i_077818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7781C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77822; return 10; }
    8
}
/// $07781C: jsr $28cd4.l
pub fn i_07781c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77822);
    cpu.pc = 0x28CD4;
    16
}
/// $077822: cmpi.b #$0, $72(a6)
pub fn i_077822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77828;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $077828: beq.w $77890
pub fn i_077828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7782C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77890; return 10; }
    8
}
/// $07782C: cmpi.b #$c0, $72(a6)
pub fn i_07782c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77832;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $077832: beq.w $77890
pub fn i_077832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77836;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77890; return 10; }
    8
}
/// $077836: cmpi.b #$1, $73(a6)
pub fn i_077836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7783C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07783C: beq.w $77860
pub fn i_07783c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77840;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77860; return 10; }
    8
}
/// $077840: jsr $77bae(pc)
pub fn i_077840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77844;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77844);
    cpu.pc = 0x77BAE;
    16
}
/// $077844: move.w $70(a6), d0
pub fn i_077844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77848;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077848: lea.l $2dc946.l, a0
pub fn i_077848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7784E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC946;
    20
}
/// $07784E: asl.w #$2, d0
pub fn i_07784e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77850;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $077850: move.l (a0, d0.w), $5c(a6)
pub fn i_077850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77856;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $077856: jsr $28d70.l
pub fn i_077856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7785C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7785C);
    cpu.pc = 0x28D70;
    16
}
/// $07785C: bra.w $77890
pub fn i_07785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77860;
    cpu.ir = 0x6000;
    cpu.pc = 0x77890;
    10
}
/// $077860: move.w $84(a6), d1
pub fn i_077860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77864;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077864: move.w $86(a6), d2
pub fn i_077864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77868;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077868: lea.l $778a2(pc), a1
pub fn i_077868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7786C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $07786C: move.l a1, (a6)
pub fn i_07786c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7786E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07786E: lea.l $77a16(pc), a1
pub fn i_07786e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77872;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A16;
    16
}
/// $077872: jsr $6fe.l
pub fn i_077872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77878);
    cpu.pc = 0x6FE;
    16
}
/// $077878: jsr $5dd02.l
pub fn i_077878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7787E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7787E);
    cpu.pc = 0x5DD02;
    16
}
/// $07787E: move.w $88(a6), $88(a0)
pub fn i_07787e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77884;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077884: move.b $12(a6), d0
pub fn i_077884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77888;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $077888: andi.b #$40, d0
pub fn i_077888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7788C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07788C: or.b d0, $12(a0)
pub fn i_07788c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77890;
    cpu.ir = 0x8128;
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $077890: jsr $7b2.l
pub fn i_077890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77896);
    cpu.pc = 0x7B2;
    16
}
/// $077896: bcc.w $778a0
pub fn i_077896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7789A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x778A0; return 10; }
    8
}
// ===== SetTaskHandler_07789a  ($07789A, 8 B, kernel/task_handlers.c)
/// $07789A: lea.l $77a8e(pc), a1
pub fn i_07789a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7789E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $07789E: move.l a1, (a6)
pub fn i_07789e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778A0: rts
pub fn i_0778a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_Track_0778a2  ($0778A2, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $0778A2: jsr $77c0c(pc)
pub fn i_0778a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778A6);
    cpu.pc = 0x77C0C;
    16
}
/// $0778A6: movea.l $c(a6), a2
pub fn i_0778a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778AA;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0778AA: move.w $22(a2), d1
pub fn i_0778aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778AE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0778AE: move.w $24(a2), d2
pub fn i_0778ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778B2;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0778B2: move.w d1, $22(a6)
pub fn i_0778b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778B6;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0778B6: move.w d2, $24(a6)
pub fn i_0778b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778BA;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0778BA: jsr $77bc6(pc)
pub fn i_0778ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778BE);
    cpu.pc = 0x77BC6;
    16
}
/// $0778BE: bcc.w $778e0
pub fn i_0778be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x778E0; return 10; }
    8
}
/// $0778C2: cmpi.b #$40, $72(a6)
pub fn i_0778c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778C8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $0778C8: beq.w $778ea
pub fn i_0778c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x778EA; return 10; }
    8
}
/// $0778CC: cmpi.b #$80, $72(a6)
pub fn i_0778cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778D2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $0778D2: beq.w $778ea
pub fn i_0778d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x778EA; return 10; }
    8
}
/// $0778D6: lea.l $778fc(pc), a1
pub fn i_0778d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778FC;
    16
}
/// $0778DA: move.l a1, (a6)
pub fn i_0778da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778DC: bra.w $778ea
pub fn i_0778dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x778EA;
    10
}
/// $0778E0: jsr $77bae(pc)
pub fn i_0778e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778E4);
    cpu.pc = 0x77BAE;
    16
}
/// $0778E4: lea.l $77912(pc), a1
pub fn i_0778e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77912;
    16
}
/// $0778E8: move.l a1, (a6)
pub fn i_0778e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778EA: jsr $7b2.l
pub fn i_0778ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x778F0);
    cpu.pc = 0x7B2;
    16
}
/// $0778F0: bcc.w $778fa
pub fn i_0778f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x778FA; return 10; }
    8
}
// ===== SetTaskHandler_0778f4  ($0778F4, 8 B, kernel/task_handlers.c)
/// $0778F4: lea.l $77a8e(pc), a1
pub fn i_0778f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $0778F8: move.l a1, (a6)
pub fn i_0778f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778FA: rts
pub fn i_0778fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_WaitAim_0778fc  ($0778FC, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $0778FC: jsr $77bc6(pc)
pub fn i_0778fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77900;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77900);
    cpu.pc = 0x77BC6;
    16
}
/// $077900: bcs.w $7790e
pub fn i_077900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77904;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7790E; return 10; }
    8
}
/// $077904: jsr $77bae(pc)
pub fn i_077904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77908;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77908);
    cpu.pc = 0x77BAE;
    16
}
/// $077908: lea.l $77912(pc), a1
pub fn i_077908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7790C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77912;
    16
}
/// $07790C: move.l a1, (a6)
pub fn i_07790c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7790E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07790E: jmp $778ea(pc)
pub fn i_07790e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77912;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x778EA;
    8
}
// ===== MuzzleFx_Flash_077912  ($077912, 252 B, props/fx_explosions_platforms_crew_076000.s)
/// $077912: jsr $77bc6(pc)
pub fn i_077912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77916;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77916);
    cpu.pc = 0x77BC6;
    16
}
/// $077916: clr.w d0
pub fn i_077916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77918;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077918: cmpi.b #$80, $72(a6)
pub fn i_077918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7791E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $07791E: bne.w $77938
pub fn i_07791e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77922;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77938; return 10; }
    8
}
/// $077922: cmpi.b #$82, $88(a6)
pub fn i_077922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77928;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x82, Size::Byte, false);
    16
}
/// $077928: beq.w $77936
pub fn i_077928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7792C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77936; return 10; }
    8
}
/// $07792C: cmpi.b #$8c, $88(a6)
pub fn i_07792c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77932;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8C, Size::Byte, false);
    16
}
/// $077932: bne.w $77938
pub fn i_077932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77936;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77938; return 10; }
    8
}
/// $077936: addq.w #$1, d0
pub fn i_077936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77938;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077938: movea.l $7c(a6), a0
pub fn i_077938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7793C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07793C: lsl.w #$2, d0
pub fn i_07793c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7793E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07793E: movea.l (a0, d0.w), a0
pub fn i_07793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77942;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $077942: cmpa.l #$ffffffff, a0
pub fn i_077942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77948;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $077948: beq.w $77952
pub fn i_077948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7794C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77952; return 10; }
    8
}
/// $07794C: jsr $28cd4.l
pub fn i_07794c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77952);
    cpu.pc = 0x28CD4;
    16
}
/// $077952: lea.l $77958(pc), a1
pub fn i_077952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77956;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77958;
    16
}
/// $077956: move.l a1, (a6)
pub fn i_077956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77958;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077958: jsr $77c0c(pc)
pub fn i_077958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7795C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7795C);
    cpu.pc = 0x77C0C;
    16
}
/// $07795C: movea.l $c(a6), a2
pub fn i_07795c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77960;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077960: move.w $22(a2), d1
pub fn i_077960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77964;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077964: move.w $24(a2), d2
pub fn i_077964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77968;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077968: move.w d1, $22(a6)
pub fn i_077968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7796C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07796C: move.w d2, $24(a6)
pub fn i_07796c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77970;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077970: jsr $77bc6(pc)
pub fn i_077970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77974;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77974);
    cpu.pc = 0x77BC6;
    16
}
/// $077974: bcc.w $779d8
pub fn i_077974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77978;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x779D8; return 10; }
    8
}
/// $077978: jsr $77c28(pc)
pub fn i_077978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7797C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7797C);
    cpu.pc = 0x77C28;
    16
}
/// $07797C: lea.l $778a2(pc), a1
pub fn i_07797c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77980;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $077980: move.l a1, (a6)
pub fn i_077980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77982;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077982: move.w $84(a6), d1
pub fn i_077982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77986;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077986: jsr $77bc6(pc)
pub fn i_077986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7798A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7798A);
    cpu.pc = 0x77BC6;
    16
}
/// $07798A: cmpi.b #$0, $72(a6)
pub fn i_07798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77990;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $077990: beq.w $77a04
pub fn i_077990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77994;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $077994: cmpi.b #$c0, $72(a6)
pub fn i_077994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7799A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $07799A: beq.w $77a04
pub fn i_07799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7799E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
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
    (0x0745E2, i_0745e2),
    (0x0745E4, i_0745e4),
    (0x0745E6, i_0745e6),
    (0x0745E8, i_0745e8),
    (0x0745EC, i_0745ec),
    (0x074624, i_074624),
    (0x074628, i_074628),
    (0x074644, i_074644),
    (0x074648, i_074648),
    (0x07464C, i_07464c),
    (0x074650, i_074650),
    (0x0750E2, i_0750e2),
    (0x0750E8, i_0750e8),
    (0x07519E, i_07519e),
    (0x0751A2, i_0751a2),
    (0x0751A4, i_0751a4),
    (0x075254, i_075254),
    (0x075258, i_075258),
    (0x07525A, i_07525a),
    (0x07526A, i_07526a),
    (0x07526E, i_07526e),
    (0x075FF6, i_075ff6),
    (0x075FFA, i_075ffa),
    (0x07600A, i_07600a),
    (0x07600E, i_07600e),
    (0x076010, i_076010),
    (0x07604E, i_07604e),
    (0x076052, i_076052),
    (0x076054, i_076054),
    (0x0760A0, i_0760a0),
    (0x0760A4, i_0760a4),
    (0x0760A6, i_0760a6),
    (0x0760C0, i_0760c0),
    (0x0760C4, i_0760c4),
    (0x0760C6, i_0760c6),
    (0x0760EC, i_0760ec),
    (0x0760F0, i_0760f0),
    (0x0760F2, i_0760f2),
    (0x07685A, i_07685a),
    (0x076860, i_076860),
    (0x076924, i_076924),
    (0x076928, i_076928),
    (0x07692A, i_07692a),
    (0x076952, i_076952),
    (0x076958, i_076958),
    (0x076A88, i_076a88),
    (0x076A8C, i_076a8c),
    (0x076A8E, i_076a8e),
    (0x076AD0, i_076ad0),
    (0x076AD6, i_076ad6),
    (0x076B32, i_076b32),
    (0x076B36, i_076b36),
    (0x076B38, i_076b38),
    (0x076B9E, i_076b9e),
    (0x076BA4, i_076ba4),
    (0x076BC8, i_076bc8),
    (0x076BCC, i_076bcc),
    (0x076BCE, i_076bce),
    (0x076C6C, i_076c6c),
    (0x076C72, i_076c72),
    (0x076D0E, i_076d0e),
    (0x076D14, i_076d14),
    (0x076E10, i_076e10),
    (0x076E14, i_076e14),
    (0x076E1A, i_076e1a),
    (0x076E20, i_076e20),
    (0x076E24, i_076e24),
    (0x076E28, i_076e28),
    (0x076E2E, i_076e2e),
    (0x076E32, i_076e32),
    (0x076E36, i_076e36),
    (0x076E3C, i_076e3c),
    (0x076E3E, i_076e3e),
    (0x076E42, i_076e42),
    (0x076E48, i_076e48),
    (0x076E4C, i_076e4c),
    (0x076E4E, i_076e4e),
    (0x076E50, i_076e50),
    (0x076E54, i_076e54),
    (0x076E58, i_076e58),
    (0x076E5C, i_076e5c),
    (0x076E60, i_076e60),
    (0x076E64, i_076e64),
    (0x076E68, i_076e68),
    (0x076E6C, i_076e6c),
    (0x076E72, i_076e72),
    (0x076E76, i_076e76),
    (0x076E7A, i_076e7a),
    (0x076E7C, i_076e7c),
    (0x076E7E, i_076e7e),
    (0x076E82, i_076e82),
    (0x076E88, i_076e88),
    (0x076E8C, i_076e8c),
    (0x076E90, i_076e90),
    (0x076E92, i_076e92),
    (0x076E98, i_076e98),
    (0x076E9C, i_076e9c),
    (0x076E9E, i_076e9e),
    (0x076EA0, i_076ea0),
    (0x076EA4, i_076ea4),
    (0x076EA6, i_076ea6),
    (0x076EA8, i_076ea8),
    (0x076EAC, i_076eac),
    (0x076EAE, i_076eae),
    (0x076EB2, i_076eb2),
    (0x076EB8, i_076eb8),
    (0x076EBE, i_076ebe),
    (0x076EC0, i_076ec0),
    (0x076EC4, i_076ec4),
    (0x076EC8, i_076ec8),
    (0x076ECC, i_076ecc),
    (0x076ED0, i_076ed0),
    (0x076ED4, i_076ed4),
    (0x076ED8, i_076ed8),
    (0x076EDE, i_076ede),
    (0x076EE4, i_076ee4),
    (0x076EEA, i_076eea),
    (0x076EEE, i_076eee),
    (0x076EF4, i_076ef4),
    (0x076EF8, i_076ef8),
    (0x076EFA, i_076efa),
    (0x076F00, i_076f00),
    (0x076F04, i_076f04),
    (0x076F08, i_076f08),
    (0x076F0E, i_076f0e),
    (0x076F14, i_076f14),
    (0x076F1A, i_076f1a),
    (0x076F20, i_076f20),
    (0x076F24, i_076f24),
    (0x076F2A, i_076f2a),
    (0x076F2E, i_076f2e),
    (0x076F32, i_076f32),
    (0x076F36, i_076f36),
    (0x076F3A, i_076f3a),
    (0x076F40, i_076f40),
    (0x076F42, i_076f42),
    (0x076F46, i_076f46),
    (0x076F4A, i_076f4a),
    (0x076F50, i_076f50),
    (0x076F54, i_076f54),
    (0x076F56, i_076f56),
    (0x076F5C, i_076f5c),
    (0x076F60, i_076f60),
    (0x076F66, i_076f66),
    (0x076F6C, i_076f6c),
    (0x076F70, i_076f70),
    (0x076F76, i_076f76),
    (0x076F78, i_076f78),
    (0x076F7C, i_076f7c),
    (0x076F82, i_076f82),
    (0x076F86, i_076f86),
    (0x076F8C, i_076f8c),
    (0x076F90, i_076f90),
    (0x076F96, i_076f96),
    (0x076F9C, i_076f9c),
    (0x076F9E, i_076f9e),
    (0x076FA2, i_076fa2),
    (0x076FA6, i_076fa6),
    (0x076FAA, i_076faa),
    (0x076FB0, i_076fb0),
    (0x076FB2, i_076fb2),
    (0x076FB4, i_076fb4),
    (0x076FB6, i_076fb6),
    (0x076FBA, i_076fba),
    (0x076FBC, i_076fbc),
    (0x076FBE, i_076fbe),
    (0x076FC0, i_076fc0),
    (0x076FC2, i_076fc2),
    (0x076FC4, i_076fc4),
    (0x076FC8, i_076fc8),
    (0x076FCA, i_076fca),
    (0x076FCC, i_076fcc),
    (0x076FCE, i_076fce),
    (0x076FD2, i_076fd2),
    (0x076FD6, i_076fd6),
    (0x076FDA, i_076fda),
    (0x076FDE, i_076fde),
    (0x076FE2, i_076fe2),
    (0x076FE4, i_076fe4),
    (0x076FE8, i_076fe8),
    (0x076FEA, i_076fea),
    (0x076FEE, i_076fee),
    (0x076FF2, i_076ff2),
    (0x076FF6, i_076ff6),
    (0x076FFA, i_076ffa),
    (0x076FFE, i_076ffe),
    (0x077000, i_077000),
    (0x077002, i_077002),
    (0x077004, i_077004),
    (0x077006, i_077006),
    (0x07700A, i_07700a),
    (0x07700C, i_07700c),
    (0x07700E, i_07700e),
    (0x077010, i_077010),
    (0x077012, i_077012),
    (0x077014, i_077014),
    (0x077018, i_077018),
    (0x07701C, i_07701c),
    (0x077020, i_077020),
    (0x077022, i_077022),
    (0x077024, i_077024),
    (0x077028, i_077028),
    (0x07702A, i_07702a),
    (0x07702E, i_07702e),
    (0x077030, i_077030),
    (0x077032, i_077032),
    (0x077036, i_077036),
    (0x077038, i_077038),
    (0x07703A, i_07703a),
    (0x07703C, i_07703c),
    (0x07703E, i_07703e),
    (0x077040, i_077040),
    (0x077042, i_077042),
    (0x077044, i_077044),
    (0x077048, i_077048),
    (0x07704A, i_07704a),
    (0x07704C, i_07704c),
    (0x077050, i_077050),
    (0x077052, i_077052),
    (0x077054, i_077054),
    (0x077056, i_077056),
    (0x077058, i_077058),
    (0x07705A, i_07705a),
    (0x07705C, i_07705c),
    (0x07705E, i_07705e),
    (0x077062, i_077062),
    (0x077066, i_077066),
    (0x07706A, i_07706a),
    (0x07706E, i_07706e),
    (0x077072, i_077072),
    (0x077076, i_077076),
    (0x077078, i_077078),
    (0x07707A, i_07707a),
    (0x07707C, i_07707c),
    (0x077082, i_077082),
    (0x077086, i_077086),
    (0x07708C, i_07708c),
    (0x077090, i_077090),
    (0x077092, i_077092),
    (0x077098, i_077098),
    (0x0770A0, i_0770a0),
    (0x0770A4, i_0770a4),
    (0x0770AA, i_0770aa),
    (0x0770AE, i_0770ae),
    (0x0770B0, i_0770b0),
    (0x0770B6, i_0770b6),
    (0x0770BE, i_0770be),
    (0x0770C2, i_0770c2),
    (0x0770C8, i_0770c8),
    (0x0770CC, i_0770cc),
    (0x0770D2, i_0770d2),
    (0x0770DA, i_0770da),
    (0x0770DE, i_0770de),
    (0x0770E4, i_0770e4),
    (0x0770E8, i_0770e8),
    (0x0770EC, i_0770ec),
    (0x0770F2, i_0770f2),
    (0x0770F6, i_0770f6),
    (0x0770F8, i_0770f8),
    (0x0770FE, i_0770fe),
    (0x077102, i_077102),
    (0x077104, i_077104),
    (0x077108, i_077108),
    (0x07710A, i_07710a),
    (0x07710E, i_07710e),
    (0x077112, i_077112),
    (0x077114, i_077114),
    (0x077116, i_077116),
    (0x077118, i_077118),
    (0x07711A, i_07711a),
    (0x07711C, i_07711c),
    (0x07711E, i_07711e),
    (0x077120, i_077120),
    (0x077124, i_077124),
    (0x077126, i_077126),
    (0x077128, i_077128),
    (0x07712C, i_07712c),
    (0x07712E, i_07712e),
    (0x077130, i_077130),
    (0x077134, i_077134),
    (0x077136, i_077136),
    (0x077138, i_077138),
    (0x07713C, i_07713c),
    (0x07713E, i_07713e),
    (0x077142, i_077142),
    (0x077144, i_077144),
    (0x077146, i_077146),
    (0x077148, i_077148),
    (0x07714E, i_07714e),
    (0x077156, i_077156),
    (0x07715A, i_07715a),
    (0x077160, i_077160),
    (0x077164, i_077164),
    (0x077168, i_077168),
    (0x07716E, i_07716e),
    (0x077172, i_077172),
    (0x077174, i_077174),
    (0x07717A, i_07717a),
    (0x07717E, i_07717e),
    (0x077180, i_077180),
    (0x077182, i_077182),
    (0x077188, i_077188),
    (0x07718A, i_07718a),
    (0x07718E, i_07718e),
    (0x077190, i_077190),
    (0x077196, i_077196),
    (0x07719E, i_07719e),
    (0x0771A2, i_0771a2),
    (0x0771A8, i_0771a8),
    (0x0771AC, i_0771ac),
    (0x0771AE, i_0771ae),
    (0x0771B2, i_0771b2),
    (0x0771B4, i_0771b4),
    (0x0771B8, i_0771b8),
    (0x0771BE, i_0771be),
    (0x0771C0, i_0771c0),
    (0x0771C4, i_0771c4),
    (0x0771C6, i_0771c6),
    (0x0771CA, i_0771ca),
    (0x0771CC, i_0771cc),
    (0x0771CE, i_0771ce),
    (0x0771D0, i_0771d0),
    (0x0771D2, i_0771d2),
    (0x0771D6, i_0771d6),
    (0x0771DA, i_0771da),
    (0x0771DE, i_0771de),
    (0x0771E0, i_0771e0),
    (0x0771E4, i_0771e4),
    (0x0771E6, i_0771e6),
    (0x0771EA, i_0771ea),
    (0x0771EE, i_0771ee),
    (0x0771F2, i_0771f2),
    (0x0771F4, i_0771f4),
    (0x0771F6, i_0771f6),
    (0x0771F8, i_0771f8),
    (0x0771FE, i_0771fe),
    (0x077202, i_077202),
    (0x077204, i_077204),
    (0x077208, i_077208),
    (0x07720C, i_07720c),
    (0x077212, i_077212),
    (0x077216, i_077216),
    (0x077218, i_077218),
    (0x07721A, i_07721a),
    (0x077220, i_077220),
    (0x077222, i_077222),
    (0x077224, i_077224),
    (0x077228, i_077228),
    (0x07722C, i_07722c),
    (0x077232, i_077232),
    (0x077236, i_077236),
    (0x07723A, i_07723a),
    (0x07723E, i_07723e),
    (0x077240, i_077240),
    (0x077246, i_077246),
    (0x07724A, i_07724a),
    (0x07724E, i_07724e),
    (0x077252, i_077252),
    (0x077258, i_077258),
    (0x07725A, i_07725a),
    (0x07725C, i_07725c),
    (0x07725E, i_07725e),
    (0x077262, i_077262),
    (0x077266, i_077266),
    (0x077268, i_077268),
    (0x07726A, i_07726a),
    (0x07726E, i_07726e),
    (0x077274, i_077274),
    (0x077278, i_077278),
    (0x07727C, i_07727c),
    (0x077280, i_077280),
    (0x077286, i_077286),
    (0x077288, i_077288),
    (0x07728C, i_07728c),
    (0x077290, i_077290),
    (0x077294, i_077294),
    (0x077298, i_077298),
    (0x07729C, i_07729c),
    (0x0772A2, i_0772a2),
    (0x0772A6, i_0772a6),
    (0x07749E, i_07749e),
    (0x0774A2, i_0774a2),
    (0x0774A6, i_0774a6),
    (0x0774AA, i_0774aa),
    (0x0774AC, i_0774ac),
    (0x0774AE, i_0774ae),
    (0x0774B4, i_0774b4),
    (0x0774B8, i_0774b8),
    (0x0774BC, i_0774bc),
    (0x0774EC, i_0774ec),
    (0x0774F4, i_0774f4),
    (0x0774F6, i_0774f6),
    (0x0774FC, i_0774fc),
    (0x077500, i_077500),
    (0x077508, i_077508),
    (0x07750A, i_07750a),
    (0x077510, i_077510),
    (0x077514, i_077514),
    (0x07751C, i_07751c),
    (0x07751E, i_07751e),
    (0x077524, i_077524),
    (0x077528, i_077528),
    (0x077530, i_077530),
    (0x077534, i_077534),
    (0x07753A, i_07753a),
    (0x07753E, i_07753e),
    (0x077546, i_077546),
    (0x07754A, i_07754a),
    (0x077550, i_077550),
    (0x077554, i_077554),
    (0x07755C, i_07755c),
    (0x077560, i_077560),
    (0x077566, i_077566),
    (0x07756A, i_07756a),
    (0x077572, i_077572),
    (0x077576, i_077576),
    (0x07757C, i_07757c),
    (0x077580, i_077580),
    (0x077584, i_077584),
    (0x07758A, i_07758a),
    (0x07758E, i_07758e),
    (0x077590, i_077590),
    (0x077596, i_077596),
    (0x07759A, i_07759a),
    (0x07759E, i_07759e),
    (0x0775A0, i_0775a0),
    (0x0775A6, i_0775a6),
    (0x0775AC, i_0775ac),
    (0x0775B0, i_0775b0),
    (0x0775B4, i_0775b4),
    (0x0775B8, i_0775b8),
    (0x0775BC, i_0775bc),
    (0x0775C0, i_0775c0),
    (0x0775C2, i_0775c2),
    (0x0775C6, i_0775c6),
    (0x0775C8, i_0775c8),
    (0x0775CC, i_0775cc),
    (0x0775D2, i_0775d2),
    (0x0775D8, i_0775d8),
    (0x0775DA, i_0775da),
    (0x0775DE, i_0775de),
    (0x0775E4, i_0775e4),
    (0x0775EA, i_0775ea),
    (0x077640, i_077640),
    (0x077648, i_077648),
    (0x07764C, i_07764c),
    (0x077654, i_077654),
    (0x077658, i_077658),
    (0x07765E, i_07765e),
    (0x077662, i_077662),
    (0x077668, i_077668),
    (0x07766E, i_07766e),
    (0x077670, i_077670),
    (0x077674, i_077674),
    (0x07767C, i_07767c),
    (0x077680, i_077680),
    (0x077684, i_077684),
    (0x077686, i_077686),
    (0x07768A, i_07768a),
    (0x077692, i_077692),
    (0x07769A, i_07769a),
    (0x0776A2, i_0776a2),
    (0x0776AA, i_0776aa),
    (0x0776AE, i_0776ae),
    (0x0776B0, i_0776b0),
    (0x0776B4, i_0776b4),
    (0x0776B8, i_0776b8),
    (0x0776C0, i_0776c0),
    (0x0776C8, i_0776c8),
    (0x0776D0, i_0776d0),
    (0x0776D8, i_0776d8),
    (0x0776DC, i_0776dc),
    (0x0776DE, i_0776de),
    (0x0776E2, i_0776e2),
    (0x0776E6, i_0776e6),
    (0x0776EE, i_0776ee),
    (0x0776F6, i_0776f6),
    (0x0776FE, i_0776fe),
    (0x077706, i_077706),
    (0x07770A, i_07770a),
    (0x07770C, i_07770c),
    (0x077710, i_077710),
    (0x077714, i_077714),
    (0x07771C, i_07771c),
    (0x077724, i_077724),
    (0x07772C, i_07772c),
    (0x077734, i_077734),
    (0x077738, i_077738),
    (0x07773A, i_07773a),
    (0x07773E, i_07773e),
    (0x077742, i_077742),
    (0x07774A, i_07774a),
    (0x077752, i_077752),
    (0x07775A, i_07775a),
    (0x077762, i_077762),
    (0x077766, i_077766),
    (0x077768, i_077768),
    (0x07776C, i_07776c),
    (0x077772, i_077772),
    (0x077776, i_077776),
    (0x077778, i_077778),
    (0x07777C, i_07777c),
    (0x077780, i_077780),
    (0x077784, i_077784),
    (0x077788, i_077788),
    (0x07778C, i_07778c),
    (0x077790, i_077790),
    (0x077794, i_077794),
    (0x077798, i_077798),
    (0x07779A, i_07779a),
    (0x07779E, i_07779e),
    (0x0777A2, i_0777a2),
    (0x0777A6, i_0777a6),
    (0x0777AA, i_0777aa),
    (0x0777AE, i_0777ae),
    (0x0777B2, i_0777b2),
    (0x0777B8, i_0777b8),
    (0x0777BC, i_0777bc),
    (0x0777C2, i_0777c2),
    (0x0777C6, i_0777c6),
    (0x0777CC, i_0777cc),
    (0x0777D0, i_0777d0),
    (0x0777D2, i_0777d2),
    (0x0777D6, i_0777d6),
    (0x0777DA, i_0777da),
    (0x0777DE, i_0777de),
    (0x0777E4, i_0777e4),
    (0x0777E6, i_0777e6),
    (0x0777EA, i_0777ea),
    (0x0777EE, i_0777ee),
    (0x0777F2, i_0777f2),
    (0x0777F6, i_0777f6),
    (0x0777FA, i_0777fa),
    (0x0777FC, i_0777fc),
    (0x077800, i_077800),
    (0x077804, i_077804),
    (0x077808, i_077808),
    (0x07780C, i_07780c),
    (0x07780E, i_07780e),
    (0x077812, i_077812),
    (0x077818, i_077818),
    (0x07781C, i_07781c),
    (0x077822, i_077822),
    (0x077828, i_077828),
    (0x07782C, i_07782c),
    (0x077832, i_077832),
    (0x077836, i_077836),
    (0x07783C, i_07783c),
    (0x077840, i_077840),
    (0x077844, i_077844),
    (0x077848, i_077848),
    (0x07784E, i_07784e),
    (0x077850, i_077850),
    (0x077856, i_077856),
    (0x07785C, i_07785c),
    (0x077860, i_077860),
    (0x077864, i_077864),
    (0x077868, i_077868),
    (0x07786C, i_07786c),
    (0x07786E, i_07786e),
    (0x077872, i_077872),
    (0x077878, i_077878),
    (0x07787E, i_07787e),
    (0x077884, i_077884),
    (0x077888, i_077888),
    (0x07788C, i_07788c),
    (0x077890, i_077890),
    (0x077896, i_077896),
    (0x07789A, i_07789a),
    (0x07789E, i_07789e),
    (0x0778A0, i_0778a0),
    (0x0778A2, i_0778a2),
    (0x0778A6, i_0778a6),
    (0x0778AA, i_0778aa),
    (0x0778AE, i_0778ae),
    (0x0778B2, i_0778b2),
    (0x0778B6, i_0778b6),
    (0x0778BA, i_0778ba),
    (0x0778BE, i_0778be),
    (0x0778C2, i_0778c2),
    (0x0778C8, i_0778c8),
    (0x0778CC, i_0778cc),
    (0x0778D2, i_0778d2),
    (0x0778D6, i_0778d6),
    (0x0778DA, i_0778da),
    (0x0778DC, i_0778dc),
    (0x0778E0, i_0778e0),
    (0x0778E4, i_0778e4),
    (0x0778E8, i_0778e8),
    (0x0778EA, i_0778ea),
    (0x0778F0, i_0778f0),
    (0x0778F4, i_0778f4),
    (0x0778F8, i_0778f8),
    (0x0778FA, i_0778fa),
    (0x0778FC, i_0778fc),
    (0x077900, i_077900),
    (0x077904, i_077904),
    (0x077908, i_077908),
    (0x07790C, i_07790c),
    (0x07790E, i_07790e),
    (0x077912, i_077912),
    (0x077916, i_077916),
    (0x077918, i_077918),
    (0x07791E, i_07791e),
    (0x077922, i_077922),
    (0x077928, i_077928),
    (0x07792C, i_07792c),
    (0x077932, i_077932),
    (0x077936, i_077936),
    (0x077938, i_077938),
    (0x07793C, i_07793c),
    (0x07793E, i_07793e),
    (0x077942, i_077942),
    (0x077948, i_077948),
    (0x07794C, i_07794c),
    (0x077952, i_077952),
    (0x077956, i_077956),
    (0x077958, i_077958),
    (0x07795C, i_07795c),
    (0x077960, i_077960),
    (0x077964, i_077964),
    (0x077968, i_077968),
    (0x07796C, i_07796c),
    (0x077970, i_077970),
    (0x077974, i_077974),
    (0x077978, i_077978),
    (0x07797C, i_07797c),
    (0x077980, i_077980),
    (0x077982, i_077982),
    (0x077986, i_077986),
    (0x07798A, i_07798a),
    (0x077990, i_077990),
    (0x077994, i_077994),
    (0x07799A, i_07799a),
];
