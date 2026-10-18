// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p00) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0000C0: move.l #$4d2, $0.w
pub fn i_0000c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC8;
    cpu.ir = 0x21FC;
    let v1: u32 = 0x4D2;
    bus.write32(0x0, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0000C8: bra.b $c0
pub fn i_0000c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xCA;
    cpu.ir = 0x60F6;
    cpu.pc = 0xC0;
    10
}
/// $000122: jmp $7cc.l
pub fn i_000122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x128;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x7CC;
    8
}
/// $000128: jmp $84a.l
pub fn i_000128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x12E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x84A;
    8
}
/// $00012E: jmp $852.l
pub fn i_00012e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x852;
    8
}
/// $000134: jmp $22be.l
pub fn i_000134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x22BE;
    8
}
// ===== Task_IdleRts_000400  ($000400, 2 B, boot/boot_script_sound_palslots_000400.s)
/// $000400: rts
pub fn i_000400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_FreeListInit_000410  ($000410, 158 B, kernel/task_freelist_init_000410.s)
/// $000410: lea.l $100a80.l, a0
pub fn i_000410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100A80;
    20
}
/// $000416: move.l a0, $106e80.l
pub fn i_000416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x106E80, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $00041C: move.w #$a0, d0
pub fn i_00041c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x420;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $000420: movea.l a0, a1
pub fn i_000420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x422;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $000422: movem.l d0-d7/a0-a6, -(a7)
pub fn i_000422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x426;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $000426: jsr $5dc1c.l
pub fn i_000426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42C);
    cpu.pc = 0x5DC1C;
    16
}
/// $00042C: jsr $5dc34.l
pub fn i_00042c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x432);
    cpu.pc = 0x5DC34;
    16
}
/// $000432: movem.l (a7)+, d0-d7/a0-a6
pub fn i_000432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
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
/// $000436: adda.l #$a0, a0
pub fn i_000436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0xA0;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $00043C: move.l a0, $8(a1)
pub fn i_00043c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440;
    cpu.ir = 0x2348;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000440: subq.w #$1, d0
pub fn i_000440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $000442: bne.b $420
pub fn i_000442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444;
    cpu.ir = 0x66DC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x420; return 10; }
    8
}
/// $000444: move.l #$ffffffff, $8(a1)
pub fn i_000444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C;
    cpu.ir = 0x237C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $00044C: lea.l $100940.l, a1
pub fn i_00044c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x452;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100940;
    20
}
/// $000452: lea.l $278000.l, a2
pub fn i_000452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x278000;
    20
}
/// $000458: movea.l (a2), a6
pub fn i_000458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A;
    cpu.ir = 0x2C52;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[6] = v2;
    12
}
/// $00045A: movea.l a6, a0
pub fn i_00045a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $00045C: movem.l d0-d7/a0-a6, -(a7)
pub fn i_00045c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x460;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $000460: jsr $5dc1c.l
pub fn i_000460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x466);
    cpu.pc = 0x5DC1C;
    16
}
/// $000466: jsr $5dc34.l
pub fn i_000466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46C);
    cpu.pc = 0x5DC34;
    16
}
/// $00046C: movem.l (a7)+, d0-d7/a0-a6
pub fn i_00046c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x470;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
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
/// $000470: move.b #$0, $10(a6)
pub fn i_000470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x476;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    bus.write8(ad2, v1 as u8);
    12
}
/// $000476: move.b #$0, $12(a6)
pub fn i_000476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x47C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    bus.write8(ad2, v1 as u8);
    12
}
/// $00047C: bset.b #$0, $12(a6)
pub fn i_00047c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x482;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $000482: move.l a6, $c(a6)
pub fn i_000482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486;
    cpu.ir = 0x2D4E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000486: move.l $4(a2), d0
pub fn i_000486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A;
    cpu.ir = 0x202A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $00048A: move.l d0, (a6)
pub fn i_00048a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C;
    cpu.ir = 0x2C80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $00048C: move.l a1, $4(a6)
pub fn i_00048c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000490: move.l a6, $8(a1)
pub fn i_000490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494;
    cpu.ir = 0x234E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000494: movea.l a6, a1
pub fn i_000494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496;
    cpu.ir = 0x224E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $000496: adda.l #$8, a2
pub fn i_000496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C;
    cpu.ir = 0xD5FC;
    let v1: u32 = 0x8;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $00049C: cmpa.l #$100940, a6
pub fn i_00049c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100940;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0004A2: bne.b $458
pub fn i_0004a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4;
    cpu.ir = 0x66B4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x458; return 10; }
    8
}
/// $0004A4: move.l #$100080, $8(a6)
pub fn i_0004a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x100080;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0004AC: rts
pub fn i_0004ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_AllocFromFreeList  ($0004AE, 80 B, kernel/task_alloc_0004ae.s)
/// $0004AE: movea.l $106e80.l, a0
pub fn i_0004ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B4;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x106E80);
    cpu.a[0] = v2;
    20
}
/// $0004B4: cmpa.l #$ffffffff, a0
pub fn i_0004b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0004BA: beq.w $506
pub fn i_0004ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x506; return 10; }
    8
}
/// $0004BE: move.l $8(a0), $106e80.l
pub fn i_0004be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6;
    cpu.ir = 0x23E8;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    bus.write32(0x106E80, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $0004C6: movem.l d0-d7/a0-a6, -(a7)
pub fn i_0004c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0004CA: jsr $5dc1c.l
pub fn i_0004ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0);
    cpu.pc = 0x5DC1C;
    16
}
/// $0004D0: jsr $5dc34.l
pub fn i_0004d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D6);
    cpu.pc = 0x5DC34;
    16
}
/// $0004D6: movem.l (a7)+, d0-d7/a0-a6
pub fn i_0004d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
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
/// $0004DA: move.l a1, (a0)
pub fn i_0004da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4DC;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0004DC: move.l a6, $c(a0)
pub fn i_0004dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E0;
    cpu.ir = 0x214E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0004E0: move.b $10(a6), d0
pub fn i_0004e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0004E4: addq.b #$1, d0
pub fn i_0004e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4E6;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0004E6: move.b d0, $10(a0)
pub fn i_0004e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EA;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x10);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0004EA: movea.l $8(a6), a1
pub fn i_0004ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4EE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0004EE: move.l a0, $8(a6)
pub fn i_0004ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0004F2: move.l a6, $4(a0)
pub fn i_0004f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4F6;
    cpu.ir = 0x214E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0004F6: move.l a1, $8(a0)
pub fn i_0004f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FA;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0004FA: move.l a0, $4(a1)
pub fn i_0004fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4FE;
    cpu.ir = 0x2348;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== JsrAbsThunk_0004fe  ($0004FE, 8 B, thunks/jsr_abs_thunks.c)
/// $0004FE: jsr $5dc34.l
pub fn i_0004fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x504);
    cpu.pc = 0x5DC34;
    16
}
/// $000504: rts
pub fn i_000504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_AllocFail_000506  ($000506, 8 B, boot/boot_script_sound_palslots_000400.s)
/// $000506: lea.l $1009e0.l, a0
pub fn i_000506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1009E0;
    20
}
/// $00050C: rts
pub fn i_00050c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_AllocAndMarkBusy_00050e  ($00050E, 10 B, boot/boot_script_sound_palslots_000400.s)
/// $00050E: bsr.b $4c6
pub fn i_00050e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x510;
    cpu.ir = 0x61B6;
    cpu.push32(bus, 0x510);
    cpu.pc = 0x4C6;
    18
}
/// $000510: bset.b #$0, $12(a0)
pub fn i_000510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $000516: rts
pub fn i_000516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x518;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scheduler_Main_0518  ($000518, 46 B, kernel/scheduler.c)
/// $000518: jsr $13600.l
pub fn i_000518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51E);
    cpu.pc = 0x13600;
    16
}
/// $00051E: bclr.b #$1, $12(a6)
pub fn i_00051e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $000524: lea.l $52a(pc), a1
pub fn i_000524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52A;
    16
}
/// $000528: move.l a1, (a6)
pub fn i_000528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $00052A: movea.l $8(a6), a1
pub fn i_00052a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $00052E: move.b $10(a6), d0
pub fn i_00052e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $000532: cmp.b $10(a1), d0
pub fn i_000532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $000536: bcs.w $542
pub fn i_000536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x542; return 10; }
    8
}
/// $00053A: jsr $546(pc)
pub fn i_00053a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x53E);
    cpu.pc = 0x546;
    16
}
/// $00053E: jmp $6e2(pc)
pub fn i_00053e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x6E2;
    8
}
/// $000542: jmp $6ca(pc)
pub fn i_000542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x6CA;
    8
}
// ===== Task_UnlinkAlive_0546  ($000546, 66 B, kernel/scheduler.c)
/// $000546: move.b $12(a6), d0
pub fn i_000546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $00054A: btst.b #$0, d0
pub fn i_00054a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $00054E: bne.w $588
pub fn i_00054e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x588; return 10; }
    8
}
/// $000552: jsr $13600.l
pub fn i_000552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558);
    cpu.pc = 0x13600;
    16
}
/// $000558: movea.l $4(a6), a1
pub fn i_000558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $00055C: movea.l $8(a6), a2
pub fn i_00055c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $000560: move.l a2, $8(a1)
pub fn i_000560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564;
    cpu.ir = 0x234A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000564: move.l a1, $4(a2)
pub fn i_000564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568;
    cpu.ir = 0x2549;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000568: move.l #$ffffffff, (a6)
pub fn i_000568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E;
    cpu.ir = 0x2CBC;
    let v1: u32 = 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $00056E: move.l #$ffffffff, $48(a6)
pub fn i_00056e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x576;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000576: move.l $106e80.l, $8(a6)
pub fn i_000576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57E;
    cpu.ir = 0x2D79;
    let v2 = bus.read32(0x106E80);
    let ad3 = cpu.a[6].wrapping_add(0x8);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $00057E: move.l a6, $106e80.l
pub fn i_00057e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x584;
    cpu.ir = 0x23CE;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    bus.write32(0x106E80, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $000584: movea.l a1, a6
pub fn i_000584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x586;
    cpu.ir = 0x2C49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $000586: rts
pub fn i_000586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x588;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_UnlinkDead_0588  ($000588, 46 B, kernel/scheduler.c)
/// $000588: jsr $13600.l
pub fn i_000588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x58E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x58E);
    cpu.pc = 0x13600;
    16
}
/// $00058E: movea.l $4(a6), a1
pub fn i_00058e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x592;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $000592: movea.l $8(a6), a2
pub fn i_000592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x596;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $000596: move.l a2, $8(a1)
pub fn i_000596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59A;
    cpu.ir = 0x234A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $00059A: move.l a1, $4(a2)
pub fn i_00059a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x59E;
    cpu.ir = 0x2549;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $00059E: move.l #$400, (a6)
pub fn i_00059e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A4;
    cpu.ir = 0x2CBC;
    let v1: u32 = 0x400;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0005A4: move.l #$ffffffff, $48(a6)
pub fn i_0005a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0005AC: bset.b #$0, $12(a6)
pub fn i_0005ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0005B2: movea.l a1, a6
pub fn i_0005b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4;
    cpu.ir = 0x2C49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $0005B4: rts
pub fn i_0005b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_WalkList_05B6  ($0005B6, 36 B, kernel/scheduler.c)
/// $0005B6: movem.l a6, -(a7)
pub fn i_0005b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0005BA: move.b $10(a6), d0
pub fn i_0005ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0005BE: movea.l $8(a6), a6
pub fn i_0005be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $0005C2: cmp.b $10(a6), d0
pub fn i_0005c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0005C6: bcc.w $5d4
pub fn i_0005c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5D4; return 10; }
    8
}
/// $0005CA: move.w d0, -(a7)
pub fn i_0005ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0005CC: bsr.w $546
pub fn i_0005cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5D0);
    cpu.pc = 0x546;
    18
}
/// $0005D0: move.w (a7)+, d0
pub fn i_0005d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $0005D2: bra.b $5be
pub fn i_0005d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D4;
    cpu.ir = 0x60EA;
    cpu.pc = 0x5BE;
    10
}
/// $0005D4: movem.l (a7)+, a6
pub fn i_0005d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0005D8: rts
pub fn i_0005d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_RunHandler_05FE  ($0005FE, 40 B, kernel/scheduler.c)
/// $0005FE: move.w $10(a0), d0
pub fn i_0005fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x602;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $000602: movem.l d0-d7/a0-a6, -(a7)
pub fn i_000602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x606;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $000606: jsr $5dc1c.l
pub fn i_000606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x60C);
    cpu.pc = 0x5DC1C;
    16
}
/// $00060C: jsr $5dc34.l
pub fn i_00060c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x612);
    cpu.pc = 0x5DC34;
    16
}
/// $000612: movem.l (a7)+, d0-d7/a0-a6
pub fn i_000612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x616;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
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
/// $000616: move.w d0, $10(a0)
pub fn i_000616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x10);
    bus.write16(ad2, v1 as u16);
    12
}
/// $00061A: clr.w $12(a0)
pub fn i_00061a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x61E;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $00061E: bset.b #$0, $12(a0)
pub fn i_00061e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x624;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $000624: rts
pub fn i_000624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x626;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_ChangeAndRun_0626  ($000626, 48 B, kernel/scheduler.c)
/// $000626: cmpa.l #$ffffffff, a0
pub fn i_000626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x62C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $00062C: beq.w $654
pub fn i_00062c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x630;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x654; return 10; }
    8
}
/// $000630: cmpa.l #$400, a0
pub fn i_000630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x400;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $000636: beq.w $654
pub fn i_000636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x654; return 10; }
    8
}
/// $00063A: movem.l a6, -(a7)
pub fn i_00063a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $00063E: movea.l a0, a6
pub fn i_00063e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $000640: jsr $5b6(pc)
pub fn i_000640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x644);
    cpu.pc = 0x5B6;
    16
}
/// $000644: lea.l $400(pc), a1
pub fn i_000644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x400;
    16
}
/// $000648: move.l a1, (a6)
pub fn i_000648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $00064A: movea.l a6, a0
pub fn i_00064a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $00064C: jsr $5fe(pc)
pub fn i_00064c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x650);
    cpu.pc = 0x5FE;
    16
}
/// $000650: movem.l (a7)+, a6
pub fn i_000650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x654;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $000654: rts
pub fn i_000654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x656;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scheduler_MainLoop_000656  ($000656, 154 B, kernel/scheduler_main_loop_000656.s)
/// $000656: lea.l $100080.l, a6
pub fn i_000656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65C;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100080;
    20
}
/// $00065C: move.l a6, $106e84.l
pub fn i_00065c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x662;
    cpu.ir = 0x23CE;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    bus.write32(0x106E84, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $000662: andi.b #$1, $5a(a6)
pub fn i_000662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x668;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $000668: andi.b #$17, $69(a6)
pub fn i_000668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66E;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x17) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $00066E: tst.b $44(a6)
pub fn i_00066e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $000672: beq.w $684
pub fn i_000672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x684; return 10; }
    8
}
/// $000676: move.b $44(a6), d0
pub fn i_000676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $00067A: moveq #$0, d1
pub fn i_00067a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $00067C: subq.b #$1, d0
pub fn i_00067c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $00067E: addx.b d1, d0
pub fn i_00067e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680;
    cpu.ir = 0xD101;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let h2 = cpu.d[1] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $000680: move.b d0, $44(a6)
pub fn i_000680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $000684: btst.b #$1, $12(a6)
pub fn i_000684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $00068A: beq.w $694
pub fn i_00068a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x694; return 10; }
    8
}
/// $00068E: addq.w #$1, $106e8a.l
pub fn i_00068e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x694;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x106E8A) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x106E8A, r2 as u16);
    16
}
/// $000694: tst.b $59(a6)
pub fn i_000694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x698;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $000698: beq.w $6aa
pub fn i_000698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x69C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6AA; return 10; }
    8
}
/// $00069C: move.b $59(a6), d0
pub fn i_00069c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0006A0: clr.b d1
pub fn i_0006a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A2;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0006A2: subq.b #$1, d0
pub fn i_0006a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A4;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0006A4: addx.b d1, d0
pub fn i_0006a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6A6;
    cpu.ir = 0xD101;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let h2 = cpu.d[1] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $0006A6: move.b d0, $59(a6)
pub fn i_0006a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0006AA: tst.b $45(a6)
pub fn i_0006aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6AE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0006AE: beq.w $6c0
pub fn i_0006ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C0; return 10; }
    8
}
/// $0006B2: move.b $45(a6), d0
pub fn i_0006b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0006B6: moveq #$0, d1
pub fn i_0006b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B8;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0006B8: subq.b #$1, d0
pub fn i_0006b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0006BA: addx.b d1, d0
pub fn i_0006ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC;
    cpu.ir = 0xD101;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let h2 = cpu.d[1] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $0006BC: move.b d0, $45(a6)
pub fn i_0006bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0006C0: movea.l (a6), a0
pub fn i_0006c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2;
    cpu.ir = 0x2056;
    let v2 = bus.read32(cpu.a[6]);
    cpu.a[0] = v2;
    12
}
/// $0006C2: move.l a7, $106e8e.l
pub fn i_0006c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8;
    cpu.ir = 0x23CF;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    bus.write32(0x106E8E, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0006C8: jsr (a0)
pub fn i_0006c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x6CA);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $0006CA: bclr.b #$6, $13(a6)
pub fn i_0006ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0006D0: andi.b #$fa, $5a(a6)
pub fn i_0006d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFA) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0006D6: andi.b #$68, $69(a6)
pub fn i_0006d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DC;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x68) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0006DC: addq.w #$1, $106e8c.l
pub fn i_0006dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x106E8C) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x106E8C, r2 as u16);
    16
}
/// $0006E2: movea.l $106e8e.l, a7
pub fn i_0006e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8;
    cpu.ir = 0x2E79;
    let v2 = bus.read32(0x106E8E);
    cpu.a[7] = v2;
    20
}
/// $0006E8: movea.l $8(a6), a6
pub fn i_0006e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $0006EC: bra.w $65c
pub fn i_0006ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x65C;
    10
}
// ===== Entity_AllocFromFreeList_0006FE  ($0006FE, 108 B, entity/alloc_from_freelist_0006fe.s)
/// $0006FE: movea.l $106e80.l, a0
pub fn i_0006fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x704;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x106E80);
    cpu.a[0] = v2;
    20
}
/// $000704: cmpa.l #$ffffffff, a0
pub fn i_000704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $00070A: beq.w $76a
pub fn i_00070a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76A; return 10; }
    8
}
/// $00070E: move.l $8(a0), $106e80.l
pub fn i_00070e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x716;
    cpu.ir = 0x23E8;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    bus.write32(0x106E80, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $000716: movem.l d0-d7/a0-a6, -(a7)
pub fn i_000716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x71A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $00071A: jsr $5dc1c.l
pub fn i_00071a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x720;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x720);
    cpu.pc = 0x5DC1C;
    16
}
/// $000720: jsr $5dc34.l
pub fn i_000720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x726;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x726);
    cpu.pc = 0x5DC34;
    16
}
/// $000726: movem.l (a7)+, d0-d7/a0-a6
pub fn i_000726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
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
/// $00072A: move.l a1, (a0)
pub fn i_00072a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x72C;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $00072C: move.l a6, $c(a0)
pub fn i_00072c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x730;
    cpu.ir = 0x214E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000730: move.b $10(a6), d0
pub fn i_000730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x734;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $000734: addq.b #$1, d0
pub fn i_000734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x736;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $000736: move.b d0, $10(a0)
pub fn i_000736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x10);
    bus.write8(ad2, v1 as u8);
    12
}
/// $00073A: movea.l a6, a1
pub fn i_00073a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x73C;
    cpu.ir = 0x224E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $00073C: movea.l $8(a1), a1
pub fn i_00073c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x740;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $000740: cmp.b $10(a1), d0
pub fn i_000740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x744;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $000744: bls.b $73c
pub fn i_000744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x746;
    cpu.ir = 0x63F6;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x73C; return 10; }
    8
}
/// $000746: movem.l a2, -(a7)
pub fn i_000746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $00074A: movea.l $4(a1), a2
pub fn i_00074a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x74E;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $00074E: move.l a0, $8(a2)
pub fn i_00074e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x752;
    cpu.ir = 0x2548;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000752: move.l a2, $4(a0)
pub fn i_000752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x756;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000756: move.l a1, $8(a0)
pub fn i_000756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $00075A: move.l a0, $4(a1)
pub fn i_00075a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x75E;
    cpu.ir = 0x2348;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $00075E: movem.l (a7)+, a2
pub fn i_00075e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $000762: jsr $5dc34.l
pub fn i_000762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x768;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x768);
    cpu.pc = 0x5DC34;
    16
}
/// $000768: rts
pub fn i_000768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_AllocFail_00076a  ($00076A, 8 B, boot/boot_script_sound_palslots_000400.s)
/// $00076A: lea.l $1009e0.l, a0
pub fn i_00076a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1009E0;
    20
}
/// $000770: rts
pub fn i_000770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x772;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Table_LookupPointerBounded  ($000772, 64 B, kernel/table_lookup_000772.s)
/// $000772: lsl.w #$2, d0
pub fn i_000772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $000774: cmpa.l #$ffffffff, a1
pub fn i_000774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $00077A: beq.w $796
pub fn i_00077a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x796; return 10; }
    8
}
/// $00077E: andi.l #$ffff, d0
pub fn i_00077e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $000784: adda.l d0, a0
pub fn i_000784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $000786: cmpa.l a0, a1
pub fn i_000786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788;
    cpu.ir = 0xB3C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $000788: bgt.w $796
pub fn i_000788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x796; return 10; }
    8
}
/// $00078C: nop
pub fn i_00078c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E;
    cpu.ir = 0x4E71;
    4
}
/// $00078E: nop
pub fn i_00078e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x790;
    cpu.ir = 0x4E71;
    4
}
/// $000790: cmpa.l a0, a1
pub fn i_000790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792;
    cpu.ir = 0xB3C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $000792: nop
pub fn i_000792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x794;
    cpu.ir = 0x4E71;
    4
}
/// $000794: trap #$f
pub fn i_000794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $000796: movea.l (a0, d0.w), a1
pub fn i_000796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $00079A: cmpa.l #$ffffffff, a1
pub fn i_00079a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0007A0: beq.w $7b0
pub fn i_0007a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7B0; return 10; }
    8
}
/// $0007A4: cmpa.l #$ffffffff, a1
pub fn i_0007a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0007AA: beq.w $7b0
pub fn i_0007aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7B0; return 10; }
    8
}
/// $0007AE: move.l a1, (a6)
pub fn i_0007ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0007B0: rts
pub fn i_0007b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_IsMagic52A_0007b2  ($0007B2, 14 B, boot/boot_script_sound_palslots_000400.s)
/// $0007B2: movea.l $c(a6), a0
pub fn i_0007b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0007B6: cmpi.l #$52a, (a0)
pub fn i_0007b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $0007BC: beq.w $7c6
pub fn i_0007bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C6; return 10; }
    8
}
// ===== ClearXN_0007c0  ($0007C0, 6 B, thunks/ccr_helpers.c)
/// $0007C0: andi.b #$ee, ccr
pub fn i_0007c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0007C4: rts
pub fn i_0007c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0007c6  ($0007C6, 6 B, thunks/ccr_helpers.c)
/// $0007C6: ori.b #$11, ccr
pub fn i_0007c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0007CA: rts
pub fn i_0007ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BiosEntry_USER  ($0007CC, 64 B, bios/bios_user_dispatch.c)
/// $0007CC: lea.l $8f2(pc), a0
pub fn i_0007cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8F2;
    16
}
/// $0007D0: move.l a0, $106ea8.l
pub fn i_0007d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x106EA8, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0007D6: bclr.b #$7, $10fd80.l
pub fn i_0007d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE;
    cpu.ir = 0x8B9;
    let mk: u32 = 1u32 << ((7) % 8);
    let v1 = bus.read8(0x10FD80) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    bus.write8(0x10FD80, r2 as u8);
    20
}
/// $0007DE: move.w #$7, $3c000c.l
pub fn i_0007de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $0007E6: andi.w #$f8ff, sr
pub fn i_0007e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA;
    cpu.ir = 0x27C;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let h1 = cpu.sr.0 & 0xF8FF;
    write_sr(cpu, h1);
    20
}
/// $0007EA: moveq #$0, d0
pub fn i_0007ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0007EC: move.b $10fdae.l, d0
pub fn i_0007ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FDAE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0007F2: add.w d0, d0
pub fn i_0007f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0007F4: add.w d0, d0
pub fn i_0007f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0007F6: movea.l $7fc(pc, d0.w), a0
pub fn i_0007f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA;
    cpu.ir = 0x207B;
    let ad2 = 0x7FC_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0007FA: jmp (a0)
pub fn i_0007fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
// ===== UserMode0_080C  ($00080C, 38 B, bios/bios_user_dispatch.c)
/// $00080C: jsr $24e38.l
pub fn i_00080c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x812);
    cpu.pc = 0x24E38;
    16
}
/// $000812: move.l $10fe80.l, d0
pub fn i_000812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x10FE80);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $000818: bne.w $826
pub fn i_000818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x826; return 10; }
    8
}
/// $00081C: clr.w $100000.l
pub fn i_00081c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x100000) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x100000, r2 as u16);
    16
}
/// $000822: bra.w $82e
pub fn i_000822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826;
    cpu.ir = 0x6000;
    cpu.pc = 0x82E;
    10
}
/// $000826: move.w #$0, $100000.l
pub fn i_000826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x100000, v1 as u16);
    16
}
/// $00082E: jmp $85e(pc)
pub fn i_00082e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x85E;
    8
}
// ===== UserMode2_0836  ($000836, 10 B, bios/bios_user_dispatch.c)
/// $000836: jsr $97a(pc)
pub fn i_000836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83A);
    cpu.pc = 0x97A;
    16
}
/// $00083A: jmp $656.l
pub fn i_00083a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x656;
    8
}
// ===== UserMode3_0840  ($000840, 10 B, bios/bios_user_dispatch.c)
/// $000840: jsr $97a(pc)
pub fn i_000840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x844);
    cpu.pc = 0x97A;
    16
}
/// $000844: jmp $656.l
pub fn i_000844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x656;
    8
}
// ===== BiosEntry_PLAYER  ($00084A, 8 B, bios/bios_entry_points.c)
/// $00084A: jsr $24e76.l
pub fn i_00084a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850);
    cpu.pc = 0x24E76;
    16
}
/// $000850: rts
pub fn i_000850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BiosEntry_DEMO  ($000852, 12 B, bios/bios_entry_points.c)
/// $000852: jsr $24fb6.l
pub fn i_000852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x858);
    cpu.pc = 0x24FB6;
    16
}
/// $000858: bsr.w $868
pub fn i_000858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x85C);
    cpu.pc = 0x868;
    18
}
/// $00085C: rts
pub fn i_00085c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SoftReset_085E  ($00085E, 10 B, bios/bios_user_dispatch.c)
/// $00085E: bsr.w $868
pub fn i_00085e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x862);
    cpu.pc = 0x868;
    18
}
/// $000862: jmp $c00444.l
pub fn i_000862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x868;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC00444;
    8
}
// ===== Sys_HW_Reset  ($000868, 60 B, bios/sys_hw_reset.c)
/// $000868: bclr.b #$7, $10fd80.l
pub fn i_000868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870;
    cpu.ir = 0x8B9;
    let mk: u32 = 1u32 << ((7) % 8);
    let v1 = bus.read8(0x10FD80) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    bus.write8(0x10FD80, r2 as u8);
    20
}
/// $000870: move.b #$0, $3a001f.l
pub fn i_000870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $000878: clr.w $401ffe.l
pub fn i_000878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x401FFE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x401FFE, r2 as u16);
    16
}
/// $00087E: move.b #$0, $3a000f.l
pub fn i_00087e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $000886: clr.w $401ffe.l
pub fn i_000886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x401FFE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x401FFE, r2 as u16);
    16
}
/// $00088C: moveq #$0, d1
pub fn i_00088c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $00088E: move.w #$e40c, d0
pub fn i_00088e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE40C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $000892: lea.l $100080.l, a0
pub fn i_000892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100080;
    20
}
/// $000898: bra.w $89e
pub fn i_000898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C;
    cpu.ir = 0x6000;
    cpu.pc = 0x89E;
    10
}
/// $00089C: move.b d1, (a0)+
pub fn i_00089c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89E;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $00089E: dbra d0, $89c
pub fn i_00089e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A2;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x8A2; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x8A2; return 14; }
    cpu.pc = 0x89C;
    10
}
/// $0008A2: rts
pub fn i_0008a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bios_TrapF_CalendarSeq_0008a4  ($0008A4, 50 B, boot/boot_script_sound_palslots_000400.s)
/// $0008A4: move.w #$1234, d0
pub fn i_0008a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1234;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008A8: move.w #$a, d1
pub fn i_0008a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008AC: trap #$f
pub fn i_0008ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0008AE: move.w #$1234, d0
pub fn i_0008ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1234;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008B2: move.w #$b, d1
pub fn i_0008b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008B6: trap #$f
pub fn i_0008b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0008B8: move.w #$1234, d0
pub fn i_0008b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1234;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008BC: move.w #$c, d1
pub fn i_0008bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008C0: trap #$f
pub fn i_0008c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0008C2: move.w #$1234, d0
pub fn i_0008c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1234;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008C6: move.w #$d, d1
pub fn i_0008c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008CA: trap #$f
pub fn i_0008ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0008CC: move.w #$1234, d0
pub fn i_0008cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1234;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008D0: move.w #$e, d1
pub fn i_0008d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0008D4: trap #$f
pub fn i_0008d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
// ===== VBlankHandler_08D6  ($0008D6, 26 B, bios/irq_handlers.c)
/// $0008D6: move.w #$2, $3c000c.l
pub fn i_0008d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $0008DE: movem.l a0, -(a7)
pub fn i_0008de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0008E2: movea.l $106ea8.l, a0
pub fn i_0008e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E8;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x106EA8);
    cpu.a[0] = v2;
    20
}
/// $0008E8: jsr (a0)
pub fn i_0008e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EA;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x8EA);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $0008EA: movem.l (a7)+, a0
pub fn i_0008ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0008EE: rte
pub fn i_0008ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F0;
    cpu.ir = 0x4E73;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let s = cpu.pop16(bus); let r = cpu.pop32(bus); write_sr(cpu, s); cpu.jump_to(r);
    20
}
// ===== IrqStubRte_08F0  ($0008F0, 2 B, bios/irq_handlers.c)
/// $0008F0: rte
pub fn i_0008f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2;
    cpu.ir = 0x4E73;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let s = cpu.pop16(bus); let r = cpu.pop32(bus); write_sr(cpu, s); cpu.jump_to(r);
    20
}
// ===== IrqStubRte_08F4  ($0008F4, 2 B, bios/irq_handlers.c)
/// $0008F4: rte
pub fn i_0008f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6;
    cpu.ir = 0x4E73;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let s = cpu.pop16(bus); let r = cpu.pop32(bus); write_sr(cpu, s); cpu.jump_to(r);
    20
}
// ===== IRQ2_08F6  ($0008F6, 106 B, bios/irq_handlers.c)
/// $0008F6: tst.b $10fd80.l
pub fn i_0008f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD80) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0008FC: bmi.w $906
pub fn i_0008fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x900;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x906; return 10; }
    8
}
/// $000900: jmp $c00438.l
pub fn i_000900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x906;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC00438;
    8
}
/// $000906: movem.l d0-d7/a0-a6, -(a7)
pub fn i_000906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $00090A: jsr $1e5e.l
pub fn i_00090a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x910;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x910);
    cpu.pc = 0x1E5E;
    16
}
/// $000910: jsr $960(pc)
pub fn i_000910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x914;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x914);
    cpu.pc = 0x960;
    16
}
/// $000914: move.w $3c0004.l, d0
pub fn i_000914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x3C0004) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $00091A: movem.w d0, -(a7)
pub fn i_00091a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x91E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $00091E: move.w $106eac.l, d0
pub fn i_00091e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x924;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106EAC) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $000924: andi.w #$ffef, d0
pub fn i_000924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x928;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFEF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $000928: move.w d0, $3c0006.l
pub fn i_000928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0006, v1 as u16);
    16
}
/// $00092E: jsr $c004ce.l
pub fn i_00092e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x934);
    cpu.pc = 0xC004CE;
    16
}
/// $000934: movem.w (a7)+, d0
pub fn i_000934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $000938: movem.w d0, $3c0004.l
pub fn i_000938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x940;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0004;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $000940: jsr $c0044a.l
pub fn i_000940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x946);
    cpu.pc = 0xC0044A;
    16
}
/// $000946: move.w $106eac.l, $3c0006.l
pub fn i_000946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x950;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106EAC) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0006, v2 as u16);
    28
}
/// $000950: move.w $106ee4.l, $3c0000.l
pub fn i_000950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95A;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106EE4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v2 as u16);
    28
}
/// $00095A: movem.l (a7)+, d0-d7/a0-a6
pub fn i_00095a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
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
/// $00095E: rte
pub fn i_00095e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x960;
    cpu.ir = 0x4E73;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let s = cpu.pop16(bus); let r = cpu.pop32(bus); write_sr(cpu, s); cpu.jump_to(r);
    20
}
// ===== LspcModeCheck_0960  ($000960, 26 B, bios/irq_handlers.c)
/// $000960: moveq #$0, d0
pub fn i_000960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x962;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $000962: move.w $3c0006.l, d1
pub fn i_000962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x3C0006) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $000968: btst.b #$3, d1
pub fn i_000968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96C;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $00096C: beq.w $972
pub fn i_00096c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x970;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x972; return 10; }
    8
}
/// $000970: moveq #$1, d0
pub fn i_000970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x972;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $000972: move.b d0, $106eae.l
pub fn i_000972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EAE, v1 as u8);
    16
}
/// $000978: rts
pub fn i_000978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameFrame  ($00097A, 46 B, kernel/game_frame.c)
/// $00097A: jsr $20e2.l
pub fn i_00097a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x980);
    cpu.pc = 0x20E2;
    16
}
/// $000980: jsr $9a8(pc)
pub fn i_000980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x984);
    cpu.pc = 0x9A8;
    16
}
/// $000984: jsr $c004c2.l
pub fn i_000984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A);
    cpu.pc = 0xC004C2;
    16
}
/// $00098A: jsr $212e.l
pub fn i_00098a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x990);
    cpu.pc = 0x212E;
    16
}
/// $000990: clr.w $106eac.l
pub fn i_000990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106EAC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106EAC, r2 as u16);
    16
}
/// $000996: tst.b $10fd82.l
pub fn i_000996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $00099C: beq.w $9a6
pub fn i_00099c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A6; return 10; }
    8
}
/// $0009A0: jsr $99afc.l
pub fn i_0009a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A6);
    cpu.pc = 0x99AFC;
    16
}
/// $0009A6: rts
pub fn i_0009a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ResetIrqCallback  ($0009A8, 12 B, bios/reset_irq_callback.c)
/// $0009A8: lea.l $8f2(pc), a0
pub fn i_0009a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8F2;
    16
}
/// $0009AC: move.l a0, $106ea8.l
pub fn i_0009ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x106EA8, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0009B2: rts
pub fn i_0009b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskSlots_BootInstall_000A7C  ($000A7C, 270 B, kernel/task_boot_install_slots_0009b4.s)
/// $000A7C: lea.l $1008a0.l, a0
pub fn i_000a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xA82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1008A0;
    20
}
/// $000A82: lea.l $400.l, a1
pub fn i_000a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xA88;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $000A88: jsr $50e.l
pub fn i_000a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xA8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xA8E);
    cpu.pc = 0x50E;
    16
}
/// $000A8E: lea.l $100800.l, a0
pub fn i_000a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xA94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100800;
    20
}
/// $000A94: lea.l $400.l, a1
pub fn i_000a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xA9A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $000A9A: jsr $50e.l
pub fn i_000a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xAA0);
    cpu.pc = 0x50E;
    16
}
/// $000AA0: lea.l $1006c0.l, a0
pub fn i_000aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1006C0;
    20
}
/// $000AA6: lea.l $3240c.l, a1
pub fn i_000aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAAC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3240C;
    20
}
/// $000AAC: jsr $50e.l
pub fn i_000aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xAB2);
    cpu.pc = 0x50E;
    16
}
/// $000AB2: lea.l $100440.l, a0
pub fn i_000ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $000AB8: lea.l $400.l, a1
pub fn i_000ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xABE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $000ABE: jsr $50e.l
pub fn i_000abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xAC4);
    cpu.pc = 0x50E;
    16
}
/// $000AC4: lea.l $1004e0.l, a0
pub fn i_000ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xACA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $000ACA: lea.l $400.l, a1
pub fn i_000aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAD0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $000AD0: jsr $50e.l
pub fn i_000ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xAD6);
    cpu.pc = 0x50E;
    16
}
/// $000AD6: lea.l $100620.l, a0
pub fn i_000ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xADC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100620;
    20
}
/// $000ADC: lea.l $32406.l, a1
pub fn i_000adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAE2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32406;
    20
}
/// $000AE2: jsr $50e.l
pub fn i_000ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xAE8);
    cpu.pc = 0x50E;
    16
}
/// $000AE8: lea.l $100580.l, a0
pub fn i_000ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAEE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $000AEE: lea.l $400.l, a1
pub fn i_000aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAF4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $000AF4: jsr $50e.l
pub fn i_000af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xAFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xAFA);
    cpu.pc = 0x50E;
    16
}
/// $000AFA: lea.l $100760.l, a0
pub fn i_000afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100760;
    20
}
/// $000B00: lea.l $29582.l, a1
pub fn i_000b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB06;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29582;
    20
}
/// $000B06: jsr $50e.l
pub fn i_000b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xB0C);
    cpu.pc = 0x50E;
    16
}
/// $000B0C: lea.l $100300.l, a0
pub fn i_000b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100300;
    20
}
/// $000B12: lea.l $400.l, a1
pub fn i_000b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB18;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $000B18: jsr $50e.l
pub fn i_000b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xB1E);
    cpu.pc = 0x50E;
    16
}
/// $000B1E: lea.l $1003a0.l, a0
pub fn i_000b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1003A0;
    20
}
/// $000B24: lea.l $400.l, a1
pub fn i_000b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB2A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $000B2A: jsr $50e.l
pub fn i_000b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xB30);
    cpu.pc = 0x50E;
    16
}
/// $000B30: lea.l $100260.l, a0
pub fn i_000b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100260;
    20
}
/// $000B36: lea.l $400.l, a1
pub fn i_000b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB3C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $000B3C: jsr $50e.l
pub fn i_000b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xB42);
    cpu.pc = 0x50E;
    16
}
/// $000B42: lea.l $1001c0.l, a0
pub fn i_000b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1001C0;
    20
}
/// $000B48: lea.l $e8e(pc), a1
pub fn i_000b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xE8E;
    16
}
/// $000B4C: jsr $50e.l
pub fn i_000b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xB52);
    cpu.pc = 0x50E;
    16
}
/// $000B52: lea.l $100440.l, a0
pub fn i_000b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $000B58: jsr $5fe.l
pub fn i_000b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xB5E);
    cpu.pc = 0x5FE;
    16
}
/// $000B5E: lea.l $1004e0.l, a0
pub fn i_000b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $000B64: jsr $5fe.l
pub fn i_000b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xB6A);
    cpu.pc = 0x5FE;
    16
}
/// $000B6A: lea.l $100580.l, a0
pub fn i_000b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $000B70: jsr $5fe.l
pub fn i_000b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xB76);
    cpu.pc = 0x5FE;
    16
}
/// $000B76: move.l #$100300, $10044c.l
pub fn i_000b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB80;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x100300;
    bus.write32(0x10044C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $000B80: move.l #$1003a0, $1004ec.l
pub fn i_000b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB8A;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x1003A0;
    bus.write32(0x1004EC, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
// ===== SetTaskHandler_000b8a  ($000B8A, 8 B, kernel/task_handlers.c)
/// $000B8A: lea.l $b90(pc), a1
pub fn i_000b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xB90;
    16
}
/// $000B8E: move.l a1, (a6)
pub fn i_000b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $000B90: rts
pub fn i_000b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xB92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SchedulerBootstrap_Boot_000E8E  ($000E8E, 338 B, kernel/scheduler_bootstrap_000e8e.s)
/// $000E8E: jsr $5cace.l
pub fn i_000e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xE94);
    cpu.pc = 0x5CACE;
    16
}
/// $000E94: move.w #$f0, $10e1e4.l
pub fn i_000e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xE9C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1E4, v1 as u16);
    16
}
/// $000E9C: move.w #$1e8, $10e1e6.l
pub fn i_000e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEA4;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1E6, v1 as u16);
    16
}
/// $000EA4: move.w #$0, d0
pub fn i_000ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEA8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $000EA8: jsr $5e998.l
pub fn i_000ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xEAE);
    cpu.pc = 0x5E998;
    16
}
/// $000EAE: clr.b $12(a6)
pub fn i_000eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEB2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $000EB2: clr.b $13(a6)
pub fn i_000eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEB6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $000EB6: clr.b $5b(a6)
pub fn i_000eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEBA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $000EBA: clr.b $5a(a6)
pub fn i_000eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEBE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $000EBE: move.b $10fdaf.l, d0
pub fn i_000ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEC4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FDAF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $000EC4: cmpi.b #$2, d0
pub fn i_000ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEC8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $000EC8: beq.w $ef0
pub fn i_000ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xECC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xEF0; return 10; }
    8
}
/// $000ECC: clr.l $10fdb6.l
pub fn i_000ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xED2;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x10FDB6);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x10FDB6, r2);
    20
}
/// $000ED2: clr.b $106ecc.l
pub fn i_000ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xED8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ECC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ECC, r2 as u8);
    16
}
/// $000ED8: clr.b $106ecd.l
pub fn i_000ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEDE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ECD) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ECD, r2 as u8);
    16
}
/// $000EDE: move.b $10fdae.l, d0
pub fn i_000ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEE4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FDAE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $000EE4: cmpi.b #$3, d0
pub fn i_000ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEE8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $000EE8: beq.w $f52
pub fn i_000ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xF52; return 10; }
    8
}
/// $000EEC: bra.w $f36
pub fn i_000eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEF0;
    cpu.ir = 0x6000;
    cpu.pc = 0xF36;
    10
}
/// $000EF0: moveq #$ff, d0
pub fn i_000ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEF2;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $000EF2: jsr $24fec.l
pub fn i_000ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xEF8);
    cpu.pc = 0x24FEC;
    16
}
/// $000EF8: tst.b $10fd82.l
pub fn i_000ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xEFE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $000EFE: beq.w $f0e
pub fn i_000efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xF0E; return 10; }
    8
}
/// $000F02: lea.l $be6(pc), a0
pub fn i_000f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF06;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0xBE6;
    16
}
/// $000F06: move.l a0, $70(a6)
pub fn i_000f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF0A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000F0A: bra.w $f98
pub fn i_000f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF0E;
    cpu.ir = 0x6000;
    cpu.pc = 0xF98;
    10
}
/// $000F0E: lea.l $bda(pc), a0
pub fn i_000f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF12;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0xBDA;
    16
}
/// $000F12: move.l a0, $70(a6)
pub fn i_000f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF16;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000F16: bra.w $f98
pub fn i_000f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF1A;
    cpu.ir = 0x6000;
    cpu.pc = 0xF98;
    10
}
/// $000F36: move.b #$ff, $106ece.l
pub fn i_000f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF3E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECE, v1 as u8);
    16
}
/// $000F3E: move.b #$ff, $106ecf.l
pub fn i_000f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF46;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECF, v1 as u8);
    16
}
/// $000F46: lea.l $b92(pc), a0
pub fn i_000f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF4A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0xB92;
    16
}
/// $000F4A: move.l a0, $70(a6)
pub fn i_000f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF4E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000F4E: bra.w $f98
pub fn i_000f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF52;
    cpu.ir = 0x6000;
    cpu.pc = 0xF98;
    10
}
/// $000F52: move.b #$1, $10fec5.l
pub fn i_000f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF5A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FEC5, v1 as u8);
    16
}
/// $000F5A: move.b #$ff, $106ece.l
pub fn i_000f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF62;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECE, v1 as u8);
    16
}
/// $000F62: move.b #$ff, $106ecf.l
pub fn i_000f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF6A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECF, v1 as u8);
    16
}
/// $000F6A: lea.l $bba(pc), a0
pub fn i_000f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF6E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0xBBA;
    16
}
/// $000F6E: move.l a0, $70(a6)
pub fn i_000f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF72;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000F72: bra.w $f98
pub fn i_000f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF76;
    cpu.ir = 0x6000;
    cpu.pc = 0xF98;
    10
}
/// $000F98: move.w $10007c.l, d0
pub fn i_000f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xF9E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10007C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $000F9E: addq.w #$1, d0
pub fn i_000f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFA0;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $000FA0: move.w d0, $10007c.l
pub fn i_000fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFA6;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10007C, v1 as u16);
    16
}
/// $000FA6: jsr $5e998.l
pub fn i_000fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xFAC);
    cpu.pc = 0x5E998;
    16
}
/// $000FAC: jsr $1e1c(pc)
pub fn i_000fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xFB0);
    cpu.pc = 0x1E1C;
    16
}
/// $000FB0: jsr $52712.l
pub fn i_000fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xFB6);
    cpu.pc = 0x52712;
    16
}
/// $000FB6: bsr.w $1da4
pub fn i_000fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFBA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xFBA);
    cpu.pc = 0x1DA4;
    18
}
/// $000FBA: jsr $c004c2.l
pub fn i_000fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xFC0);
    cpu.pc = 0xC004C2;
    16
}
/// $000FC0: lea.l $fc6(pc), a1
pub fn i_000fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xFC6;
    16
}
/// $000FC4: move.l a1, (a6)
pub fn i_000fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $000FC6: movea.l $70(a6), a0
pub fn i_000fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFCA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $000FCA: cmpi.l #$ffffffff, (a0)
pub fn i_000fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFD0;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $000FD0: bne.w $fdc
pub fn i_000fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xFDC; return 10; }
    8
}
/// $000FD4: movea.l $4(a0), a0
pub fn i_000fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFD8;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $000FD8: move.l a0, $70(a6)
pub fn i_000fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFDC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000FDC: movea.l (a0), a0
pub fn i_000fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFDE;
    cpu.ir = 0x2050;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[0] = v2;
    12
}
/// $000FDE: jmp (a0)
pub fn i_000fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFE0;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
// ===== SchedulerDispatch_LoopB_000FE0  ($000FE0, 30 B, kernel/scheduler_bootstrap_000e8e.s)
/// $000FE0: movea.l $70(a6), a0
pub fn i_000fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFE4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $000FE4: adda.l #$4, a0
pub fn i_000fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFEA;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0x4;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $000FEA: cmpi.l #$ffffffff, (a0)
pub fn i_000fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFF0;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $000FF0: bne.w $ff8
pub fn i_000ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFF4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xFF8; return 10; }
    8
}
/// $000FF4: movea.l $4(a0), a0
pub fn i_000ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFF8;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $000FF8: move.l a0, $70(a6)
pub fn i_000ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFFC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $000FFC: bra.b $fc6
pub fn i_000ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xFFE;
    cpu.ir = 0x60C8;
    cpu.pc = 0xFC6;
    10
}
// ===== Global_SetDualFlagFrom10FD82_000FFE  ($000FFE, 34 B, boot/global_set_dual_flag_000ffe.s)
/// $000FFE: tst.b $10fd82.l
pub fn i_000ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1004;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001004: bne.w $1010
pub fn i_001004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1008;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1010; return 10; }
    8
}
/// $001008: move.b #$4, d0
pub fn i_001008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x100C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $00100C: bra.w $1012
pub fn i_00100c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1010;
    cpu.ir = 0x6000;
    cpu.pc = 0x1012;
    10
}
/// $001010: clr.b d0
pub fn i_001010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1012;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $001012: move.b d0, $1081bf.l
pub fn i_001012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1018;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081BF, v1 as u8);
    16
}
/// $001018: move.b d0, $1081c0.l
pub fn i_001018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x101E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081C0, v1 as u8);
    16
}
/// $00101E: rts
pub fn i_00101e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1020;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SchedTail_JsrCD4_001020  ($001020, 6 B, kernel/scheduler_bootstrap_000e8e.s)
/// $001020: bsr.w $1cd4
pub fn i_001020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1024;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1024);
    cpu.pc = 0x1CD4;
    18
}
/// $001024: bra.b $fe0
pub fn i_001024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1026;
    cpu.ir = 0x60BA;
    cpu.pc = 0xFE0;
    10
}
// ===== AttractHandler_00109C  ($00109C, 76 B, kernel/scheduler_handlers_00109c.s)
/// $00109C: moveq #$ff, d0
pub fn i_00109c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x109E;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $00109E: jsr $24fec.l
pub fn i_00109e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x10A4);
    cpu.pc = 0x24FEC;
    16
}
/// $0010A4: jsr $212e.l
pub fn i_0010a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x10AA);
    cpu.pc = 0x212E;
    16
}
/// $0010AA: jsr $c004c2.l
pub fn i_0010aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x10B0);
    cpu.pc = 0xC004C2;
    16
}
/// $0010B0: jsr $52712.l
pub fn i_0010b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x10B6);
    cpu.pc = 0x52712;
    16
}
/// $0010B6: lea.l $91330.l, a1
pub fn i_0010b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x91330;
    20
}
/// $0010BC: jsr $4ae.l
pub fn i_0010bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x10C2);
    cpu.pc = 0x4AE;
    16
}
/// $0010C2: move.b #$ff, $106ece.l
pub fn i_0010c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10CA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECE, v1 as u8);
    16
}
/// $0010CA: move.b #$ff, $106ecf.l
pub fn i_0010ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10D2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECF, v1 as u8);
    16
}
/// $0010D2: jsr $1af8(pc)
pub fn i_0010d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x10D6);
    cpu.pc = 0x1AF8;
    16
}
/// $0010D6: jsr $46ac6.l
pub fn i_0010d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x10DC);
    cpu.pc = 0x46AC6;
    16
}
/// $0010DC: move.b #$ff, $106ed2.l
pub fn i_0010dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10E4;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $0010E4: bra.w $fe0
pub fn i_0010e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10E8;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== SchedTail_JsrDB8_0010E8  ($0010E8, 4 B, kernel/scheduler_handlers_00109c.s)
/// $0010E8: bsr.w $1db8
pub fn i_0010e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10EC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x10EC);
    cpu.pc = 0x1DB8;
    18
}
// ===== JsrPcThunk_0010ec  ($0010EC, 6 B, thunks/jsr_pc_thunks.c)
/// $0010EC: jsr $1af8(pc)
pub fn i_0010ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x10F0);
    cpu.pc = 0x1AF8;
    16
}
/// $0010F0: rts
pub fn i_0010f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x10F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AttractHandler_Loader_0011EA  ($0011EA, 68 B, kernel/scheduler_handlers_00109c.s)
/// $0011EA: jsr $c004c2.l
pub fn i_0011ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x11F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x11F0);
    cpu.pc = 0xC004C2;
    16
}
/// $0011F0: jsr $52712.l
pub fn i_0011f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x11F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x11F6);
    cpu.pc = 0x52712;
    16
}
/// $0011F6: lea.l $9b4(pc), a0
pub fn i_0011f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x11FA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9B4;
    16
}
/// $0011FA: jsr $2b58.l
pub fn i_0011fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1200);
    cpu.pc = 0x2B58;
    16
}
/// $001200: lea.l $977ea.l, a1
pub fn i_001200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1206;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x977EA;
    20
}
/// $001206: jsr $4ae.l
pub fn i_001206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x120C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x120C);
    cpu.pc = 0x4AE;
    16
}
/// $00120C: move.b #$ff, $106ece.l
pub fn i_00120c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1214;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECE, v1 as u8);
    16
}
/// $001214: move.b #$ff, $106ecf.l
pub fn i_001214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x121C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECF, v1 as u8);
    16
}
/// $00121C: jsr $46ac6.l
pub fn i_00121c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1222);
    cpu.pc = 0x46AC6;
    16
}
/// $001222: move.b #$ff, $106ed2.l
pub fn i_001222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x122A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $00122A: bra.w $fe0
pub fn i_00122a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x122E;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== AttractPhase2_Probes5D0_00122E  ($00122E, 50 B, kernel/scheduler_handlers_00109c.s)
/// $00122E: bsr.w $1db8
pub fn i_00122e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1232;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1232);
    cpu.pc = 0x1DB8;
    18
}
/// $001232: tst.b $106ed2.l
pub fn i_001232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1238;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001238: beq.w $1254
pub fn i_001238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x123C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1254; return 10; }
    8
}
/// $00123C: jsr $5d09a.l
pub fn i_00123c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1242);
    cpu.pc = 0x5D09A;
    16
}
/// $001242: bcs.w $1254
pub fn i_001242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1246;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1254; return 10; }
    8
}
/// $001246: jsr $5d0ac.l
pub fn i_001246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x124C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x124C);
    cpu.pc = 0x5D0AC;
    16
}
/// $00124C: bcs.w $1254
pub fn i_00124c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1250;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1254; return 10; }
    8
}
/// $001250: bra.w $125e
pub fn i_001250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1254;
    cpu.ir = 0x6000;
    cpu.pc = 0x125E;
    10
}
/// $001254: jsr $5b6.l
pub fn i_001254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x125A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x125A);
    cpu.pc = 0x5B6;
    16
}
/// $00125A: bra.w $fe0
pub fn i_00125a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x125E;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
/// $00125E: rts
pub fn i_00125e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1260;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BootScr_GameOver_0012f4  ($0012F4, 44 B, boot/boot_script_sound_palslots_000400.s)
/// $0012F4: jsr $c004c2.l
pub fn i_0012f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x12FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x12FA);
    cpu.pc = 0xC004C2;
    16
}
/// $0012FA: jsr $52712.l
pub fn i_0012fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1300);
    cpu.pc = 0x52712;
    16
}
/// $001300: lea.l $8f91a.l, a1
pub fn i_001300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1306;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x8F91A;
    20
}
/// $001306: jsr $4ae.l
pub fn i_001306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x130C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x130C);
    cpu.pc = 0x4AE;
    16
}
/// $00130C: move.b #$ff, $106ece.l
pub fn i_00130c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1314;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECE, v1 as u8);
    16
}
/// $001314: move.b #$ff, $106ecf.l
pub fn i_001314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x131C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECF, v1 as u8);
    16
}
/// $00131C: jsr $1af8(pc)
pub fn i_00131c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1320;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x1320);
    cpu.pc = 0x1AF8;
    16
}
// ===== Init_JsrThenTailCall_001320  ($001320, 18 B, boot/init_jsr_tail_call_001320.s)
/// $001320: jsr $46ac6.l
pub fn i_001320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1326;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1326);
    cpu.pc = 0x46AC6;
    16
}
/// $001326: move.b #$ff, $106ed2.l
pub fn i_001326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x132E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $00132E: bra.w $fe0
pub fn i_00132e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1332;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== BootScr_WaitHudThenLoop_001332  ($001332, 28 B, boot/boot_script_sound_palslots_000400.s)
/// $001332: bsr.w $1db8
pub fn i_001332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1336;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1336);
    cpu.pc = 0x1DB8;
    18
}
/// $001336: tst.b $106ed2.l
pub fn i_001336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $00133C: beq.w $1344
pub fn i_00133c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1340;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1344; return 10; }
    8
}
/// $001340: bra.w $134e
pub fn i_001340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1344;
    cpu.ir = 0x6000;
    cpu.pc = 0x134E;
    10
}
/// $001344: jsr $5b6.l
pub fn i_001344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x134A);
    cpu.pc = 0x5B6;
    16
}
/// $00134A: bra.w $fe0
pub fn i_00134a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134E;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== JsrPcThunk_00134e  ($00134E, 6 B, thunks/jsr_pc_thunks.c)
/// $00134E: jsr $1af8(pc)
pub fn i_00134e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1352;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x1352);
    cpu.pc = 0x1AF8;
    16
}
/// $001352: rts
pub fn i_001352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1354;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BootScr_Clr13d2a_001354  ($001354, 10 B, boot/boot_script_sound_palslots_000400.s)
/// $001354: jsr $13d2a.l
pub fn i_001354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x135A);
    cpu.pc = 0x13D2A;
    16
}
/// $00135A: bra.w $fe0
pub fn i_00135a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135E;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== BootScr_SetFF13d20_00135e  ($00135E, 10 B, boot/boot_script_sound_palslots_000400.s)
/// $00135E: jsr $13d20.l
pub fn i_00135e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1364;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1364);
    cpu.pc = 0x13D20;
    16
}
/// $001364: bra.w $fe0
pub fn i_001364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1368;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== BootScr_Mission_Start_001368  ($001368, 94 B, boot/boot_script_sound_palslots_000400.s)
/// $001368: jsr $c004c2.l
pub fn i_001368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x136E);
    cpu.pc = 0xC004C2;
    16
}
/// $00136E: jsr $52712.l
pub fn i_00136e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1374);
    cpu.pc = 0x52712;
    16
}
/// $001374: jsr $212e.l
pub fn i_001374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x137A);
    cpu.pc = 0x212E;
    16
}
/// $00137A: move.b #$9, $106ece.l
pub fn i_00137a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1382;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECE, v1 as u8);
    16
}
/// $001382: move.b #$ff, $106ecf.l
pub fn i_001382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECF, v1 as u8);
    16
}
/// $00138A: clr.b $106ed2.l
pub fn i_00138a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1390;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $001390: lea.l $10fd84.l, a3
pub fn i_001390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1396;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10FD84;
    20
}
/// $001396: move.b $a(a3), d0
pub fn i_001396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139A;
    cpu.ir = 0x102B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $00139A: tst.b d0
pub fn i_00139a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139C;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $00139C: beq.w $13c2
pub fn i_00139c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13C2; return 10; }
    8
}
/// $0013A0: lea.l $100300.l, a0
pub fn i_0013a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100300;
    20
}
/// $0013A6: move.l #$8c008, (a0)
pub fn i_0013a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13AC;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x8C008;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0013AC: jsr $5fe.l
pub fn i_0013ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x13B2);
    cpu.pc = 0x5FE;
    16
}
/// $0013B2: move.w #$1, $106e92.l
pub fn i_0013b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13BA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106E92, v1 as u16);
    16
}
/// $0013BA: move.b #$ff, $106ed2.l
pub fn i_0013ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $0013C2: bra.w $fe0
pub fn i_0013c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C6;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== BootScr_Mission_WaitStart_0013c6  ($0013C6, 78 B, boot/boot_script_sound_palslots_000400.s)
/// $0013C6: bsr.w $1db8
pub fn i_0013c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x13CA);
    cpu.pc = 0x1DB8;
    18
}
/// $0013CA: tst.b $106ed2.l
pub fn i_0013ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D0;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0013D0: beq.w $140c
pub fn i_0013d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x140C; return 10; }
    8
}
/// $0013D4: jsr $138e6.l
pub fn i_0013d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x13DA);
    cpu.pc = 0x138E6;
    16
}
/// $0013DA: bcc.w $140a
pub fn i_0013da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x140A; return 10; }
    8
}
/// $0013DE: cmpi.b #$1, $10fdb6.l
pub fn i_0013de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13E6;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0013E6: bne.w $13f4
pub fn i_0013e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13F4; return 10; }
    8
}
/// $0013EA: jsr $5d09a.l
pub fn i_0013ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x13F0);
    cpu.pc = 0x5D09A;
    16
}
/// $0013F0: bcs.w $140c
pub fn i_0013f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13F4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x140C; return 10; }
    8
}
/// $0013F4: cmpi.b #$1, $10fdb7.l
pub fn i_0013f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13FC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0013FC: bne.w $140a
pub fn i_0013fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1400;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x140A; return 10; }
    8
}
/// $001400: jsr $5d0ac.l
pub fn i_001400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1406;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1406);
    cpu.pc = 0x5D0AC;
    16
}
/// $001406: bcs.w $140c
pub fn i_001406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x140A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x140C; return 10; }
    8
}
/// $00140A: rts
pub fn i_00140a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x140C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $00140C: bsr.w $1cd4
pub fn i_00140c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1410;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1410);
    cpu.pc = 0x1CD4;
    18
}
/// $001410: bra.w $fe0
pub fn i_001410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1414;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== BootScr_Message_Prepare_001416  ($001416, 34 B, boot/boot_script_sound_palslots_000400.s)
/// $001416: jsr $c004c2.l
pub fn i_001416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x141C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x141C);
    cpu.pc = 0xC004C2;
    16
}
/// $00141C: jsr $46ac6.l
pub fn i_00141c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1422);
    cpu.pc = 0x46AC6;
    16
}
/// $001422: clr.b $106ed0.l
pub fn i_001422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1428;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED0) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED0, r2 as u8);
    16
}
/// $001428: jsr $981fc.l
pub fn i_001428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x142E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x142E);
    cpu.pc = 0x981FC;
    16
}
/// $00142E: jsr $5162c.l
pub fn i_00142e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1434);
    cpu.pc = 0x5162C;
    16
}
/// $001434: bra.w $fe0
pub fn i_001434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1438;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== BootScr_Message_WaitRun_001438  ($001438, 26 B, boot/boot_script_sound_palslots_000400.s)
/// $001438: bsr.w $1db8
pub fn i_001438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x143C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x143C);
    cpu.pc = 0x1DB8;
    18
}
/// $00143C: tst.b $106ed2.l
pub fn i_00143c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1442;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001442: bne.w $1450
pub fn i_001442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1446;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1450; return 10; }
    8
}
/// $001446: jsr $44236.l
pub fn i_001446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x144C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x144C);
    cpu.pc = 0x44236;
    16
}
/// $00144C: bra.w $fe0
pub fn i_00144c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1450;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
/// $001450: rts
pub fn i_001450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1452;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BootScr_MsgSelect_Dispatch_001452  ($001452, 138 B, boot/boot_script_sound_palslots_000400.s)
/// $001452: move.w $10007c.l, d0
pub fn i_001452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1458;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10007C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $001458: addq.w #$1, d0
pub fn i_001458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x145A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $00145A: move.w d0, $10007c.l
pub fn i_00145a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1460;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10007C, v1 as u16);
    16
}
/// $001460: jsr $5e998.l
pub fn i_001460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1466;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1466);
    cpu.pc = 0x5E998;
    16
}
/// $001466: jsr $51696.l
pub fn i_001466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x146C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x146C);
    cpu.pc = 0x51696;
    16
}
/// $00146C: move.b $106ed0.l, d0
pub fn i_00146c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1472;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ED0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $001472: cmpi.b #$6, d0
pub fn i_001472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1476;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $001476: bcs.w $1482
pub fn i_001476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x147A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1482; return 10; }
    8
}
/// $00147A: move.b #$5, $106ed0.l
pub fn i_00147a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1482;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED0, v1 as u8);
    16
}
/// $001482: move.b $106ed0.l, d0
pub fn i_001482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1488;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ED0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $001488: cmpi.b #$10, d0
pub fn i_001488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x148C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $00148C: bcs.w $1498
pub fn i_00148c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1490;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1498; return 10; }
    8
}
/// $001490: clr.b d0
pub fn i_001490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1492;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $001492: move.b d0, $106ed0.l
pub fn i_001492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1498;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED0, v1 as u8);
    16
}
/// $001498: andi.w #$ff, d0
pub fn i_001498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x149C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $00149C: lea.l $c0a(pc), a0
pub fn i_00149c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x14A0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0xC0A;
    16
}
/// $0014A0: lsl.w #$3, d0
pub fn i_0014a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x14A2;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0014A2: movea.l (a0, d0.w), a0
pub fn i_0014a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x14A6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0014A6: move.l a0, $70(a6)
pub fn i_0014a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x14AA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0014AA: bra.w $fc6
pub fn i_0014aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x14AE;
    cpu.ir = 0x6000;
    cpu.pc = 0xFC6;
    10
}
// ===== BootScr_MemCardSave_001526  ($001526, 30 B, boot/boot_script_sound_palslots_000400.s)
/// $001526: jsr $c004c2.l
pub fn i_001526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x152C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x152C);
    cpu.pc = 0xC004C2;
    16
}
/// $00152C: jsr $46ac6.l
pub fn i_00152c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1532);
    cpu.pc = 0x46AC6;
    16
}
/// $001532: move.b #$1, $106ed2.l
pub fn i_001532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x153A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $00153A: jsr $51660.l
pub fn i_00153a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1540);
    cpu.pc = 0x51660;
    16
}
/// $001540: bra.w $fe0
pub fn i_001540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1544;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== BootScr_WaitHudDirty_001544  ($001544, 16 B, boot/boot_script_sound_palslots_000400.s)
/// $001544: bsr.w $1db8
pub fn i_001544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1548;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1548);
    cpu.pc = 0x1DB8;
    18
}
/// $001548: tst.b $106ed2.l
pub fn i_001548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x154E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $00154E: beq.w $fe0
pub fn i_00154e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1552;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xFE0; return 10; }
    8
}
/// $001552: rts
pub fn i_001552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1554;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_InitBIOS_001744  ($001744, 132 B, boot/attract_bios_init_001744.s)
/// $001744: jsr $c004c2.l
pub fn i_001744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x174A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x174A);
    cpu.pc = 0xC004C2;
    16
}
/// $00174A: jsr $52712.l
pub fn i_00174a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1750);
    cpu.pc = 0x52712;
    16
}
/// $001750: lea.l $46682.l, a1
pub fn i_001750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1756;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x46682;
    20
}
/// $001756: jsr $4ae.l
pub fn i_001756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x175C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x175C);
    cpu.pc = 0x4AE;
    16
}
/// $00175C: cmpi.b #$6, $106ed0.l
pub fn i_00175c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1764;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED0) as u32;
    sub_with_flags(cpu, v2, 0x6, Size::Byte, false);
    20
}
/// $001764: bcs.w $1774
pub fn i_001764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1768;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1774; return 10; }
    8
}
/// $001768: lea.l $59b6a.l, a1
pub fn i_001768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x176E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x59B6A;
    20
}
/// $00176E: jsr $4ae.l
pub fn i_00176e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1774);
    cpu.pc = 0x4AE;
    16
}
/// $001774: jsr $46ac6.l
pub fn i_001774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x177A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x177A);
    cpu.pc = 0x46AC6;
    16
}
/// $00177A: move.b #$ff, $106ece.l
pub fn i_00177a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1782;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECE, v1 as u8);
    16
}
/// $001782: move.b #$ff, $106ecf.l
pub fn i_001782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x178A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECF, v1 as u8);
    16
}
/// $00178A: move.b #$ff, $106ed2.l
pub fn i_00178a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1792;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $001792: bsr.w $1e0a
pub fn i_001792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1796;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1796);
    cpu.pc = 0x1E0A;
    18
}
/// $001796: bra.w $fe0
pub fn i_001796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x179A;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
/// $00179A: bsr.w $1db8
pub fn i_00179a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x179E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x179E);
    cpu.pc = 0x1DB8;
    18
}
/// $00179E: tst.b $106ed2.l
pub fn i_00179e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17A4;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0017A4: beq.w $17ac
pub fn i_0017a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x17AC; return 10; }
    8
}
/// $0017A8: bra.w $17c6
pub fn i_0017a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x17C6;
    10
}
/// $0017AC: jsr $5b6.l
pub fn i_0017ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x17B2);
    cpu.pc = 0x5B6;
    16
}
/// $0017B2: lea.l $9b4(pc), a0
pub fn i_0017b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17B6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9B4;
    16
}
/// $0017B6: jsr $2b58.l
pub fn i_0017b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x17BC);
    cpu.pc = 0x2B58;
    16
}
/// $0017BC: jsr $212e.l
pub fn i_0017bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x17C2);
    cpu.pc = 0x212E;
    16
}
/// $0017C2: bra.w $fe0
pub fn i_0017c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17C6;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
/// $0017C6: rts
pub fn i_0017c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_InitShow27_TaskAdd_0017E6  ($0017E6, 44 B, boot/attract_bios_init_001744.s)
/// $0017E6: bsr.w $1e0a
pub fn i_0017e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x17EA);
    cpu.pc = 0x1E0A;
    18
}
/// $0017EA: move.b #$ff, $106ed2.l
pub fn i_0017ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17F2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $0017F2: move.w #$27, d0
pub fn i_0017f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0017F6: jsr $2352.l
pub fn i_0017f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x17FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x17FC);
    cpu.pc = 0x2352;
    16
}
/// $0017FC: lea.l $46608.l, a1
pub fn i_0017fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1802;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x46608;
    20
}
/// $001802: jsr $4ae.l
pub fn i_001802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1808);
    cpu.pc = 0x4AE;
    16
}
/// $001808: move.b #$ff, $21(a6)
pub fn i_001808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x180E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $00180E: bra.w $fe0
pub fn i_00180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1812;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== Attract_SetTimers2_And_Gate21_001812  ($001812, 26 B, boot/attract_bios_init_001744.s)
/// $001812: bsr.w $1db8
pub fn i_001812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1816;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1816);
    cpu.pc = 0x1DB8;
    18
}
/// $001816: move.b #$2, $45(a6)
pub fn i_001816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x181C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $00181C: move.b #$2, $44(a6)
pub fn i_00181c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1822;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $001822: tst.b $21(a6)
pub fn i_001822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1826;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $001826: beq.w $182c
pub fn i_001826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x182A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x182C; return 10; }
    8
}
/// $00182A: rts
pub fn i_00182a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x182C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_TailChain_1CD4_1DA4_00182C  ($00182C, 12 B, boot/attract_bios_init_001744.s)
/// $00182C: bsr.w $1cd4
pub fn i_00182c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1830;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1830);
    cpu.pc = 0x1CD4;
    18
}
/// $001830: bsr.w $1da4
pub fn i_001830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1834;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1834);
    cpu.pc = 0x1DA4;
    18
}
/// $001834: bra.w $fe0
pub fn i_001834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1838;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== Attract_SoftReset_10FDAF_001838  ($001838, 14 B, boot/attract_bios_init_001744.s)
/// $001838: move.b #$1, $10fdaf.l
pub fn i_001838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1840;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDAF, v1 as u8);
    16
}
/// $001840: jmp $85e.l
pub fn i_001840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1846;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x85E;
    8
}
// ===== Attract_DoubleCheck_400_Publish_001846  ($001846, 68 B, boot/attract_bios_init_001744.s)
/// $001846: cmpi.l #$400, $100300.l
pub fn i_001846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1850;
    cpu.ir = 0xCB9;
    let v2 = bus.read32(0x100300);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    24
}
/// $001850: bne.w $1866
pub fn i_001850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1854;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1866; return 10; }
    8
}
/// $001854: lea.l $100300.l, a0
pub fn i_001854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x185A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100300;
    20
}
/// $00185A: move.l #$2575c, (a0)
pub fn i_00185a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1860;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x2575C;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001860: jsr $5fe.l
pub fn i_001860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1866);
    cpu.pc = 0x5FE;
    16
}
/// $001866: cmpi.l #$400, $1003a0.l
pub fn i_001866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1870;
    cpu.ir = 0xCB9;
    let v2 = bus.read32(0x1003A0);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    24
}
/// $001870: bne.w $1886
pub fn i_001870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1874;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1886; return 10; }
    8
}
/// $001874: lea.l $1003a0.l, a0
pub fn i_001874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x187A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1003A0;
    20
}
/// $00187A: move.l #$25766, (a0)
pub fn i_00187a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1880;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x25766;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001880: jsr $5fe.l
pub fn i_001880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1886;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1886);
    cpu.pc = 0x5FE;
    16
}
/// $001886: bra.w $fe0
pub fn i_001886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x188A;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== Attract_WaitStateBackbone_00188A  ($00188A, 80 B, boot/attract_bios_init_001744.s)
/// $00188A: jsr $212e.l
pub fn i_00188a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1890;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1890);
    cpu.pc = 0x212E;
    16
}
/// $001890: move.b $106ed0.l, $106ecf.l
pub fn i_001890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x189A;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x106ED0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECF, v2 as u8);
    28
}
/// $00189A: jsr $52712.l
pub fn i_00189a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x18A0);
    cpu.pc = 0x52712;
    16
}
/// $0018A0: move.w #$1, d0
pub fn i_0018a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0018A4: jsr $523b2.l
pub fn i_0018a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x18AA);
    cpu.pc = 0x523B2;
    16
}
/// $0018AA: lea.l $1b4c(pc), a1
pub fn i_0018aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x1B4C;
    16
}
/// $0018AE: jsr $4ae.l
pub fn i_0018ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x18B4);
    cpu.pc = 0x4AE;
    16
}
/// $0018B4: jsr $1dcc(pc)
pub fn i_0018b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x18B8);
    cpu.pc = 0x1DCC;
    16
}
/// $0018B8: move.b #$ff, $106ed2.l
pub fn i_0018b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18C0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $0018C0: move.b #$ff, $106ed3.l
pub fn i_0018c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18C8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED3, v1 as u8);
    16
}
/// $0018C8: lea.l $1c44(pc), a1
pub fn i_0018c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x1C44;
    16
}
/// $0018CC: jsr $4ae.l
pub fn i_0018cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x18D2);
    cpu.pc = 0x4AE;
    16
}
/// $0018D2: jsr $c004c2.l
pub fn i_0018d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x18D8);
    cpu.pc = 0xC004C2;
    16
}
/// $0018D8: jsr $46ac6.l
pub fn i_0018d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x18DE);
    cpu.pc = 0x46AC6;
    16
}
/// $0018DE: bsr.w $1db8
pub fn i_0018de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x18E2);
    cpu.pc = 0x1DB8;
    18
}
/// $0018E2: move.b #$3c, $44(a6)
pub fn i_0018e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0018E8: lea.l $28db6a.l, a1
pub fn i_0018e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18EE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DB6A;
    20
}
/// $0018EE: jsr $4ae.l
pub fn i_0018ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x18F4);
    cpu.pc = 0x4AE;
    16
}
/// $0018F4: lea.l $29588.l, a1
pub fn i_0018f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x18FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29588;
    20
}
/// $0018FA: jsr $4ae.l
pub fn i_0018fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1900);
    cpu.pc = 0x4AE;
    16
}
/// $001900: lea.l $469e2.l, a1
pub fn i_001900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1906;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x469E2;
    20
}
/// $001906: jsr $4ae.l
pub fn i_001906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x190C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x190C);
    cpu.pc = 0x4AE;
    16
}
/// $00190C: move.b #$d, $98(a0)
pub fn i_00190c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1912;
    cpu.ir = 0x117C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $001912: clr.b $1081b1.l
pub fn i_001912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1918;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x1081B1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x1081B1, r2 as u8);
    16
}
/// $001918: jsr $2a24a.l
pub fn i_001918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x191E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x191E);
    cpu.pc = 0x2A24A;
    16
}
/// $00191E: bra.w $fe0
pub fn i_00191e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1922;
    cpu.ir = 0x6000;
    cpu.pc = 0xFE0;
    10
}
// ===== Dispatcher_ModeTable_001922  ($001922, 404 B, boot/dispatcher_mode_table_001922.s)
/// $001922: move.b #$0, $106ece.l
pub fn i_001922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x192A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECE, v1 as u8);
    16
}
/// $00192A: lea.l $100260.l, a0
pub fn i_00192a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1930;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100260;
    20
}
/// $001930: move.l #$967fe, (a0)
pub fn i_001930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1936;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x967FE;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001936: jsr $5fe.l
pub fn i_001936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x193C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x193C);
    cpu.pc = 0x5FE;
    16
}
/// $00193C: bra.w $188a
pub fn i_00193c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1940;
    cpu.ir = 0x6000;
    cpu.pc = 0x188A;
    10
}
/// $001A70: bsr.w $1db8
pub fn i_001a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A74;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1A74);
    cpu.pc = 0x1DB8;
    18
}
/// $001A74: lea.l $10fdb6.l, a4
pub fn i_001a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A7A;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10FDB6;
    20
}
/// $001A7A: move.b (a4)+, d0
pub fn i_001a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A7C;
    cpu.ir = 0x101C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $001A7C: move.b (a4), d1
pub fn i_001a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A7E;
    cpu.ir = 0x1214;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $001A7E: cmpi.b #$2, d0
pub fn i_001a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A82;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $001A82: beq.w $1aae
pub fn i_001a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A86;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1AAE; return 10; }
    8
}
/// $001A86: cmpi.b #$2, d1
pub fn i_001a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A8A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $001A8A: beq.w $1aae
pub fn i_001a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1AAE; return 10; }
    8
}
/// $001A8E: cmpi.b #$1, d0
pub fn i_001a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A92;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $001A92: beq.w $1ab6
pub fn i_001a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1AB6; return 10; }
    8
}
/// $001A96: cmpi.b #$1, d1
pub fn i_001a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A9A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $001A9A: beq.w $1ab6
pub fn i_001a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1A9E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1AB6; return 10; }
    8
}
/// $001A9E: bsr.w $1e0a
pub fn i_001a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AA2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1AA2);
    cpu.pc = 0x1E0A;
    18
}
/// $001AA2: move.l #$e42, $70(a6)
pub fn i_001aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AAA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xE42;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $001AAA: bra.w $fc6
pub fn i_001aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AAE;
    cpu.ir = 0x6000;
    cpu.pc = 0xFC6;
    10
}
/// $001AAE: jsr $981fc.l
pub fn i_001aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1AB4);
    cpu.pc = 0x981FC;
    16
}
/// $001AB4: rts
pub fn i_001ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_PostStart_Cleanup_001AB6  ($001AB6, 66 B, boot/attract_bios_init_001744.s)
/// $001AB6: jsr $981fc.l
pub fn i_001ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1ABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1ABC);
    cpu.pc = 0x981FC;
    16
}
/// $001ABC: btst.b #$0, $100001.l
pub fn i_001abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AC4;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $001AC4: beq.w $1adc
pub fn i_001ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AC8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1ADC; return 10; }
    8
}
/// $001AC8: jsr $5d288.l
pub fn i_001ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1ACE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1ACE);
    cpu.pc = 0x5D288;
    16
}
/// $001ACE: bcc.w $1ad8
pub fn i_001ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x1AD8; return 10; }
    8
}
/// $001AD2: lea.l $f76(pc), a1
pub fn i_001ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xF76;
    16
}
/// $001AD6: move.l a1, (a6)
pub fn i_001ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001AD8: bra.w $1adc
pub fn i_001ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1ADC;
    cpu.ir = 0x6000;
    cpu.pc = 0x1ADC;
    10
}
/// $001ADC: tst.b $106ed6.l
pub fn i_001adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AE2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001AE2: bne.w $1af0
pub fn i_001ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AE6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1AF0; return 10; }
    8
}
/// $001AE6: tst.b $106ed2.l
pub fn i_001ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AEC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001AEC: beq.w $fe0
pub fn i_001aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xFE0; return 10; }
    8
}
/// $001AF0: clr.b $106ed6.l
pub fn i_001af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AF6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED6) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED6, r2 as u8);
    16
}
/// $001AF6: rts
pub fn i_001af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_StartIfP2Flag_001af8  ($001AF8, 28 B, boot/boot_script_sound_palslots_000400.s)
/// $001AF8: move.b $10fdaf.l, d0
pub fn i_001af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1AFE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FDAF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $001AFE: cmpi.b #$2, d0
pub fn i_001afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B02;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $001B02: bne.w $1b1a
pub fn i_001b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1B1A; return 10; }
    8
}
/// $001B06: bsr.w $1cd4
pub fn i_001b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B0A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x1B0A);
    cpu.pc = 0x1CD4;
    18
}
/// $001B0A: move.w #$ffff, d0
pub fn i_001b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $001B0E: jsr $24fec.l
pub fn i_001b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1B14);
    cpu.pc = 0x24FEC;
    16
}
// ===== SetTaskHandler_001b14  ($001B14, 8 B, kernel/task_handlers.c)
/// $001B14: lea.l $ef0(pc), a1
pub fn i_001b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xEF0;
    16
}
/// $001B18: move.l a1, (a6)
pub fn i_001b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001B1A: rts
pub fn i_001b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_CountActive_001b1c  ($001B1C, 36 B, boot/boot_script_sound_palslots_000400.s)
/// $001B1C: move.w #$0, -(a7)
pub fn i_001b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B20;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $001B20: moveq #$0, d0
pub fn i_001b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B22;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $001B22: jsr $5e3a2.l
pub fn i_001b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1B28);
    cpu.pc = 0x5E3A2;
    16
}
/// $001B28: bcc.w $1b2e
pub fn i_001b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x1B2E; return 10; }
    8
}
/// $001B2C: addq.w #$1, (a7)
pub fn i_001b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B2E;
    cpu.ir = 0x5257;
    let ad1 = cpu.a[7];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $001B2E: moveq #$1, d0
pub fn i_001b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B30;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $001B30: jsr $5e3a2.l
pub fn i_001b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1B36);
    cpu.pc = 0x5E3A2;
    16
}
/// $001B36: bcc.w $1b3c
pub fn i_001b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x1B3C; return 10; }
    8
}
/// $001B3A: addq.w #$1, (a7)
pub fn i_001b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B3C;
    cpu.ir = 0x5257;
    let ad1 = cpu.a[7];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $001B3C: move.w (a7)+, d0
pub fn i_001b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B3E;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $001B3E: rts
pub fn i_001b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_StoreActiveCount_001b40  ($001B40, 12 B, boot/boot_script_sound_palslots_000400.s)
/// $001B40: jsr $1b1c(pc)
pub fn i_001b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x1B44);
    cpu.pc = 0x1B1C;
    16
}
/// $001B44: move.b d0, $106ed1.l
pub fn i_001b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B4A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED1, v1 as u8);
    16
}
/// $001B4A: rts
pub fn i_001b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rank_DelayStart_001b4c  ($001B4C, 28 B, boot/boot_script_sound_palslots_000400.s)
/// $001B4C: jsr $79970.l
pub fn i_001b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1B52);
    cpu.pc = 0x79970;
    16
}
/// $001B52: move.w d0, $106e92.l
pub fn i_001b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B58;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106E92, v1 as u16);
    16
}
/// $001B58: move.b #$ff, d1
pub fn i_001b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B5C;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $001B5C: jsr $47482.l
pub fn i_001b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1B62);
    cpu.pc = 0x47482;
    16
}
/// $001B62: move.w #$5a, $30(a6)
pub fn i_001b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_001b68  ($001B68, 8 B, kernel/task_handlers.c)
/// $001B68: lea.l $1b70(pc), a1
pub fn i_001b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x1B70;
    16
}
/// $001B6C: move.l a1, (a6)
pub fn i_001b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001B6E: rts
pub fn i_001b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rank_DelayTick_001b70  ($001B70, 8 B, boot/boot_script_sound_palslots_000400.s)
/// $001B70: subq.w #$1, $30(a6)
pub fn i_001b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B74;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $001B74: bne.w $1b7e
pub fn i_001b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1B7E; return 10; }
    8
}
// ===== SetTaskHandler_001b78  ($001B78, 8 B, kernel/task_handlers.c)
/// $001B78: lea.l $1b80(pc), a1
pub fn i_001b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x1B80;
    16
}
/// $001B7C: move.l a1, (a6)
pub fn i_001b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001B7E: rts
pub fn i_001b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rank_DelayGate_001b80  ($001B80, 74 B, boot/boot_script_sound_palslots_000400.s)
/// $001B80: jsr $1cac(pc)
pub fn i_001b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x1B84);
    cpu.pc = 0x1CAC;
    16
}
/// $001B84: bcs.w $1c32
pub fn i_001b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B88;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1C32; return 10; }
    8
}
/// $001B88: tst.b $106ed3.l
pub fn i_001b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B8E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001B8E: beq.w $1c32
pub fn i_001b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1C32; return 10; }
    8
}
/// $001B92: jsr $2f85a.l
pub fn i_001b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1B98);
    cpu.pc = 0x2F85A;
    16
}
/// $001B98: bcs.w $1c32
pub fn i_001b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1B9C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1C32; return 10; }
    8
}
/// $001B9C: tst.b $10e39c.l
pub fn i_001b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BA2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001BA2: bne.w $1c32
pub fn i_001ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1C32; return 10; }
    8
}
/// $001BA6: move.b #$ff, d1
pub fn i_001ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BAA;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $001BAA: move.b $106f28.l, d0
pub fn i_001baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BB0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $001BB0: cmpi.w #$a, $106e92.l
pub fn i_001bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BB8;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106E92) as u32;
    sub_with_flags(cpu, v2, 0xA, Size::Word, false);
    20
}
/// $001BB8: bgt.w $1bde
pub fn i_001bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BBC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x1BDE; return 10; }
    8
}
/// $001BBC: move.b d0, d2
pub fn i_001bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BBE;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $001BBE: andi.b #$10, d2
pub fn i_001bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BC2;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $001BC2: cmpi.b #$10, d2
pub fn i_001bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BC6;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $001BC6: bne.w $1bde
pub fn i_001bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1BDE; return 10; }
    8
}
// ===== Handler_TimerAndReplace_001BCA  ($001BCA, 106 B, boot/handler_timer_and_replace_001bca.s)
/// $001BCA: movem.l d0-d1, -(a7)
pub fn i_001bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BCE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $001BCE: move.w #$10d7, d0
pub fn i_001bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BD2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $001BD2: jsr $2352.l
pub fn i_001bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1BD8);
    cpu.pc = 0x2352;
    16
}
/// $001BD8: movem.l (a7)+, d0-d1
pub fn i_001bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BDC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $001BDC: clr.b d1
pub fn i_001bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BDE;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $001BDE: cmpi.w #$64, $106e92.l
pub fn i_001bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BE6;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106E92) as u32;
    sub_with_flags(cpu, v2, 0x64, Size::Word, false);
    20
}
/// $001BE6: bcc.w $1c02
pub fn i_001be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x1C02; return 10; }
    8
}
/// $001BEA: move.b d0, d2
pub fn i_001bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BEC;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $001BEC: andi.b #$f, d2
pub fn i_001bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BF0;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $001BF0: bne.w $1c32
pub fn i_001bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BF4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1C32; return 10; }
    8
}
/// $001BF4: andi.b #$7f, d0
pub fn i_001bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BF8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $001BF8: bne.w $1c02
pub fn i_001bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1BFC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1C02; return 10; }
    8
}
/// $001BFC: subq.w #$1, $106e92.l
pub fn i_001bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C02;
    cpu.ir = 0x5379;
    let v1 = bus.read16(0x106E92) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x106E92, r2 as u16);
    16
}
/// $001C02: tst.w $106e92.l
pub fn i_001c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C08;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106E92) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $001C08: bgt.w $1c2c
pub fn i_001c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C0C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x1C2C; return 10; }
    8
}
/// $001C0C: move.b #$ff, $21(a6)
pub fn i_001c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $001C12: lea.l $46a48.l, a1
pub fn i_001c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C18;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x46A48;
    20
}
/// $001C18: jsr $4ae.l
pub fn i_001c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1C1E);
    cpu.pc = 0x4AE;
    16
}
/// $001C1E: clr.w $106e92.l
pub fn i_001c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C24;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106E92) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106E92, r2 as u16);
    16
}
/// $001C24: clr.b d1
pub fn i_001c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C26;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $001C26: lea.l $1c34(pc), a1
pub fn i_001c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x1C34;
    16
}
/// $001C2A: move.l a1, (a6)
pub fn i_001c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001C2C: jsr $47482.l
pub fn i_001c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1C32);
    cpu.pc = 0x47482;
    16
}
/// $001C32: rts
pub fn i_001c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_DelayThenStart_001c44  ($001C44, 68 B, boot/boot_script_sound_palslots_000400.s)
/// $001C44: cmpi.b #$2, $10fdaf.l
pub fn i_001c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C4C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $001C4C: beq.w $1c56
pub fn i_001c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1C56; return 10; }
    8
}
/// $001C50: jmp $518.l
pub fn i_001c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C56;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $001C56: cmpi.b #$6, $106ed0.l
pub fn i_001c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C5E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED0) as u32;
    sub_with_flags(cpu, v2, 0x6, Size::Byte, false);
    20
}
/// $001C5E: bcs.w $1c68
pub fn i_001c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C62;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1C68; return 10; }
    8
}
/// $001C62: jmp $518.l
pub fn i_001c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $001C68: move.w #$f, $34(a6)
pub fn i_001c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $001C6E: lea.l $1c74(pc), a1
pub fn i_001c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x1C74;
    16
}
/// $001C72: move.l a1, (a6)
pub fn i_001c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001C74: subi.w #$1, $34(a6)
pub fn i_001c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C7A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $001C7A: bne.w $1c86
pub fn i_001c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1C86; return 10; }
    8
}
/// $001C7E: lea.l $7a970.l, a1
pub fn i_001c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C84;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7A970;
    20
}
/// $001C84: move.l a1, (a6)
pub fn i_001c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001C86: rts
pub fn i_001c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1C88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_StartMask_Probe_001cac  ($001CAC, 28 B, boot/boot_script_sound_palslots_000400.s)
/// $001CAC: lea.l $10fdb6.l, a0
pub fn i_001cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $001CB2: move.b (a0), d0
pub fn i_001cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CB4;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $001CB4: move.b $1(a0), d2
pub fn i_001cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CB8;
    cpu.ir = 0x1428;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $001CB8: cmpi.b #$1, d0
pub fn i_001cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CBC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $001CBC: beq.w $1cce
pub fn i_001cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1CCE; return 10; }
    8
}
/// $001CC0: cmpi.b #$1, d2
pub fn i_001cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CC4;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $001CC4: beq.w $1cce
pub fn i_001cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CC8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1CCE; return 10; }
    8
}
// ===== SetXN_001cc8  ($001CC8, 6 B, thunks/ccr_helpers.c)
/// $001CC8: ori.b #$11, ccr
pub fn i_001cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CCC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $001CCC: rts
pub fn i_001ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_001cce  ($001CCE, 6 B, thunks/ccr_helpers.c)
/// $001CCE: andi.b #$ee, ccr
pub fn i_001cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CD2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $001CD2: rts
pub fn i_001cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskList_ChangeAndRunEight_001CD4  ($001CD4, 96 B, kernel/task_change_and_run_eight_001cd4.s)
/// $001CD4: lea.l $100260.l, a0
pub fn i_001cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100260;
    20
}
/// $001CDA: jsr $626.l
pub fn i_001cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1CE0);
    cpu.pc = 0x626;
    16
}
/// $001CE0: lea.l $100800.l, a0
pub fn i_001ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CE6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100800;
    20
}
/// $001CE6: jsr $626.l
pub fn i_001ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1CEC);
    cpu.pc = 0x626;
    16
}
/// $001CEC: lea.l $1008a0.l, a0
pub fn i_001cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1008A0;
    20
}
/// $001CF2: jsr $626.l
pub fn i_001cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1CF8);
    cpu.pc = 0x626;
    16
}
/// $001CF8: lea.l $100440.l, a0
pub fn i_001cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1CFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $001CFE: jsr $626.l
pub fn i_001cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1D04);
    cpu.pc = 0x626;
    16
}
/// $001D04: lea.l $1004e0.l, a0
pub fn i_001d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $001D0A: jsr $626.l
pub fn i_001d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1D10);
    cpu.pc = 0x626;
    16
}
/// $001D10: lea.l $100580.l, a0
pub fn i_001d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $001D16: jsr $626.l
pub fn i_001d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1D1C);
    cpu.pc = 0x626;
    16
}
/// $001D1C: lea.l $100300.l, a0
pub fn i_001d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100300;
    20
}
/// $001D22: jsr $626.l
pub fn i_001d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1D28);
    cpu.pc = 0x626;
    16
}
/// $001D28: lea.l $1003a0.l, a0
pub fn i_001d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1003A0;
    20
}
/// $001D2E: jsr $626.l
pub fn i_001d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1D34);
    cpu.pc = 0x626;
    16
}
// ===== JsrAbsThunk_001d34  ($001D34, 8 B, thunks/jsr_abs_thunks.c)
/// $001D34: jsr $5b6.l
pub fn i_001d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1D3A);
    cpu.pc = 0x5B6;
    16
}
/// $001D3A: rts
pub fn i_001d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1D3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_InstallBootSlots_001da4  ($001DA4, 20 B, boot/boot_script_sound_palslots_000400.s)
/// $001DA4: lea.l $9fa(pc), a0
pub fn i_001da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DA8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9FA;
    16
}
/// $001DA8: jsr $2aec.l
pub fn i_001da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1DAE);
    cpu.pc = 0x2AEC;
    16
}
/// $001DAE: lea.l $9b4(pc), a0
pub fn i_001dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DB2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9B4;
    16
}
/// $001DB2: jmp $2b58.l
pub fn i_001db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x2B58;
    8
}
// ===== Hud_DrawCreditsAndOverlay_001db8  ($001DB8, 20 B, boot/boot_script_sound_palslots_000400.s)
/// $001DB8: move.b #$5, $1081be.l
pub fn i_001db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DC0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081BE, v1 as u8);
    16
}
/// $001DC0: jsr $52050.l
pub fn i_001dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1DC6);
    cpu.pc = 0x52050;
    16
}
/// $001DC6: jmp $5223a.l
pub fn i_001dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DCC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5223A;
    8
}
// ===== Task_InstallMissionSlots_001dcc  ($001DCC, 54 B, boot/boot_script_sound_palslots_000400.s)
/// $001DCC: lea.l $1008a0.l, a0
pub fn i_001dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1008A0;
    20
}
/// $001DD2: move.l #$5ea96, (a0)
pub fn i_001dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DD8;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x5EA96;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001DD8: jsr $5fe.l
pub fn i_001dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1DDE);
    cpu.pc = 0x5FE;
    16
}
/// $001DDE: lea.l $100800.l, a0
pub fn i_001dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100800;
    20
}
/// $001DE4: move.l #$442e6, (a0)
pub fn i_001de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DEA;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x442E6;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001DEA: jsr $5fe.l
pub fn i_001dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1DF0);
    cpu.pc = 0x5FE;
    16
}
/// $001DF0: btst.b #$0, $100001.l
pub fn i_001df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DF8;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $001DF8: beq.w $1e08
pub fn i_001df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1DFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1E08; return 10; }
    8
}
/// $001DFC: lea.l $5efca.l, a1
pub fn i_001dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E02;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5EFCA;
    20
}
// ===== JsrAbsThunk_001e02  ($001E02, 8 B, thunks/jsr_abs_thunks.c)
/// $001E02: jsr $4ae.l
pub fn i_001e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1E08);
    cpu.pc = 0x4AE;
    16
}
/// $001E08: rts
pub fn i_001e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E0A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Set106ECC_CD_001e0a  ($001E0A, 18 B, boot/boot_script_sound_palslots_000400.s)
/// $001E0A: move.b #$1, $106ecc.l
pub fn i_001e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E12;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECC, v1 as u8);
    16
}
/// $001E12: move.b #$1, $106ecd.l
pub fn i_001e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E1A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ECD, v1 as u8);
    16
}
/// $001E1A: rts
pub fn i_001e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scratch_AllocZero_001e1c  ($001E1C, 12 B, boot/boot_script_sound_palslots_000400.s)
/// $001E1C: clr.w d0
pub fn i_001e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E1E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $001E1E: clr.w d1
pub fn i_001e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E20;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $001E20: clr.w d2
pub fn i_001e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E22;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $001E22: jmp $1390e.l
pub fn i_001e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E28;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x1390E;
    8
}
// ===== Player1_DeadOrPaused_Check_001e28  ($001E28, 24 B, boot/boot_script_sound_palslots_000400.s)
/// $001E28: lea.l $1001c0.l, a0
pub fn i_001e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1001C0;
    20
}
/// $001E2E: tst.b $45(a0)
pub fn i_001e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E32;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $001E32: bne.w $1e46
pub fn i_001e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E36;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1E46; return 10; }
    8
}
/// $001E36: tst.b $10e39c.l
pub fn i_001e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E3C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001E3C: bne.w $1e46
pub fn i_001e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1E46; return 10; }
    8
}
// ===== SetC_001e40  ($001E40, 6 B, thunks/ccr_helpers.c)
/// $001E40: ori.b #$1, ccr
pub fn i_001e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E44;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $001E44: rts
pub fn i_001e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_001e46  ($001E46, 6 B, thunks/ccr_helpers.c)
/// $001E46: andi.b #$fe, ccr
pub fn i_001e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E4A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $001E4A: rts
pub fn i_001e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Set106ED6_FF_001e4c  ($001E4C, 10 B, boot/boot_script_sound_palslots_000400.s)
/// $001E4C: move.b #$ff, $106ed6.l
pub fn i_001e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E54;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED6, v1 as u8);
    16
}
/// $001E54: rts
pub fn i_001e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Store106EDA_001e56  ($001E56, 8 B, boot/boot_script_sound_palslots_000400.s)
/// $001E56: move.b d0, $106eda.l
pub fn i_001e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E5C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EDA, v1 as u8);
    16
}
/// $001E5C: rts
pub fn i_001e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VBlankTick_Master_001E5E  ($001E5E, 160 B, boot/vblank_tick_master_001e5e.s)
/// $001E5E: move.w #$4, $3c000c.l
pub fn i_001e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E66;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $001E66: addq.b #$1, $106edd.l
pub fn i_001e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E6C;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x106EDD) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x106EDD, r2 as u8);
    16
}
/// $001E6C: cmpi.b #$a, $106edd.l
pub fn i_001e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E74;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EDD) as u32;
    sub_with_flags(cpu, v2, 0xA, Size::Byte, false);
    20
}
/// $001E74: bhi.w $1e7e
pub fn i_001e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E78;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x1E7E; return 10; }
    8
}
/// $001E78: move.b d0, $300001.l
pub fn i_001e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E7E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $001E7E: addq.b #$1, $106ed9.l
pub fn i_001e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E84;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x106ED9) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x106ED9, r2 as u8);
    16
}
/// $001E84: jsr $226a.l
pub fn i_001e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1E8A);
    cpu.pc = 0x226A;
    16
}
/// $001E8A: move.b $106eda.l, d0
pub fn i_001e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E90;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106EDA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $001E90: beq.w $1ea0
pub fn i_001e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1EA0; return 10; }
    8
}
/// $001E94: subq.b #$1, d0
pub fn i_001e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E96;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $001E96: move.b d0, $106eda.l
pub fn i_001e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1E9C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EDA, v1 as u8);
    16
}
/// $001E9C: bra.w $1ef6
pub fn i_001e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EA0;
    cpu.ir = 0x6000;
    cpu.pc = 0x1EF6;
    10
}
/// $001EA0: cmpi.b #$0, $106ede.l
pub fn i_001ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EA8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EDE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $001EA8: beq.w $1eb8
pub fn i_001ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1EB8; return 10; }
    8
}
/// $001EAC: cmpi.b #$1, $106ed9.l
pub fn i_001eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EB4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED9) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $001EB4: bls.w $1ef6
pub fn i_001eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EB8;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x1EF6; return 10; }
    8
}
/// $001EB8: tst.b $106ed8.l
pub fn i_001eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EBE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001EBE: bne.w $1ef6
pub fn i_001ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EC2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1EF6; return 10; }
    8
}
/// $001EC2: tst.b $106edc.l
pub fn i_001ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EC8;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106EDC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001EC8: bne.w $1ee2
pub fn i_001ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1ECC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1EE2; return 10; }
    8
}
/// $001ECC: tst.b $106edb.l
pub fn i_001ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1ED2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106EDB) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $001ED2: beq.w $1ee2
pub fn i_001ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1ED6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1EE2; return 10; }
    8
}
/// $001ED6: clr.b $106edb.l
pub fn i_001ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EDC;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106EDB) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106EDB, r2 as u8);
    16
}
/// $001EDC: jsr $137c6.l
pub fn i_001edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1EE2);
    cpu.pc = 0x137C6;
    16
}
/// $001EE2: jsr $1efe(pc)
pub fn i_001ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x1EE6);
    cpu.pc = 0x1EFE;
    16
}
/// $001EE6: jsr $5c9d6.l
pub fn i_001ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x1EEC);
    cpu.pc = 0x5C9D6;
    16
}
/// $001EEC: move.b #$1, $106ed8.l
pub fn i_001eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EF4;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED8, v1 as u8);
    16
}
/// $001EF4: rts
pub fn i_001ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $001EF6: addq.w #$1, $106ee2.l
pub fn i_001ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EFC;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x106EE2) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x106EE2, r2 as u16);
    16
}
/// $001EFC: rts
pub fn i_001efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1EFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Deferred_RunQueue_001efe  ($001EFE, 76 B, boot/boot_script_sound_palslots_000400.s)
/// $001EFE: move.w $106ee4.l, d0
pub fn i_001efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F04;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106EE4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $001F04: move.w d0, -(a7)
pub fn i_001f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F06;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $001F06: move.b $106f26.l, d0
pub fn i_001f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F0C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F26) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $001F0C: cmp.b $106f27.l, d0
pub fn i_001f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F12;
    cpu.ir = 0xB039;
    let v2 = bus.read8(0x106F27) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $001F12: beq.w $1f40
pub fn i_001f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1F40; return 10; }
    8
}
/// $001F16: addq.b #$1, d0
pub fn i_001f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F18;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $001F18: cmpi.b #$8, d0
pub fn i_001f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F1C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $001F1C: bcs.w $1f22
pub fn i_001f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F20;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1F22; return 10; }
    8
}
/// $001F20: sub.b d0, d0
pub fn i_001f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F22;
    cpu.ir = 0x9000;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $001F22: move.b d0, $106f26.l
pub fn i_001f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F28;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F26, v1 as u8);
    16
}
/// $001F28: lea.l $106ee6.l, a0
pub fn i_001f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EE6;
    20
}
/// $001F2E: andi.w #$ff, d0
pub fn i_001f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F32;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $001F32: lsl.w #$3, d0
pub fn i_001f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F34;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $001F34: movea.l $4(a0, d0.w), a6
pub fn i_001f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F38;
    cpu.ir = 0x2C70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    18
}
/// $001F38: movea.l (a0, d0.w), a0
pub fn i_001f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F3C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $001F3C: jsr (a0)
pub fn i_001f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F3E;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x1F3E);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $001F3E: bra.b $1f06
pub fn i_001f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F40;
    cpu.ir = 0x60C6;
    cpu.pc = 0x1F06;
    10
}
/// $001F40: move.w (a7)+, d0
pub fn i_001f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F42;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $001F42: move.w d0, $106ee4.l
pub fn i_001f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F48;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $001F48: rts
pub fn i_001f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Deferred_Push_001f4a  ($001F4A, 58 B, boot/boot_script_sound_palslots_000400.s)
/// $001F4A: move.b $106f27.l, d0
pub fn i_001f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F50;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F27) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $001F50: addq.b #$1, d0
pub fn i_001f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F52;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $001F52: cmpi.b #$8, d0
pub fn i_001f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F56;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $001F56: bcs.w $1f5c
pub fn i_001f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F5A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x1F5C; return 10; }
    8
}
/// $001F5A: sub.b d0, d0
pub fn i_001f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F5C;
    cpu.ir = 0x9000;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $001F5C: cmp.b $106f26.l, d0
pub fn i_001f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F62;
    cpu.ir = 0xB039;
    let v2 = bus.read8(0x106F26) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $001F62: beq.w $1f82
pub fn i_001f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1F82; return 10; }
    8
}
/// $001F66: move.b d0, d1
pub fn i_001f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F68;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $001F68: andi.w #$ff, d0
pub fn i_001f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F6C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $001F6C: lsl.w #$3, d0
pub fn i_001f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F6E;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $001F6E: lea.l $106ee6.l, a1
pub fn i_001f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F74;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x106EE6;
    20
}
/// $001F74: move.l a6, $4(a1, d0.w)
pub fn i_001f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F78;
    cpu.ir = 0x238E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $001F78: move.l a0, (a1, d0.w)
pub fn i_001f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F7C;
    cpu.ir = 0x2388;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $001F7C: move.b d1, $106f27.l
pub fn i_001f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F82;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F27, v1 as u8);
    16
}
/// $001F82: rts
pub fn i_001f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VBlank_FrameLoop_001f84  ($001F84, 276 B, boot/boot_script_sound_palslots_000400.s)
/// $001F84: move.b #$0, $106f2a.l
pub fn i_001f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F8C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F2A, v1 as u8);
    16
}
/// $001F8C: move.b #$3, $106f2b.l
pub fn i_001f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F94;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F2B, v1 as u8);
    16
}
/// $001F94: clr.b $12(a6)
pub fn i_001f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F98;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $001F98: clr.b $13(a6)
pub fn i_001f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1F9C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $001F9C: clr.b $5b(a6)
pub fn i_001f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FA0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $001FA0: clr.b $5a(a6)
pub fn i_001fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FA4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $001FA4: clr.b $106f26.l
pub fn i_001fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FAA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106F26) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106F26, r2 as u8);
    16
}
/// $001FAA: clr.b $106f27.l
pub fn i_001faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FB0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106F27) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106F27, r2 as u8);
    16
}
/// $001FB0: clr.b $106eda.l
pub fn i_001fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FB6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106EDA) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106EDA, r2 as u8);
    16
}
/// $001FB6: clr.b $106edb.l
pub fn i_001fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FBC;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106EDB) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106EDB, r2 as u8);
    16
}
/// $001FBC: clr.b $106edc.l
pub fn i_001fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FC2;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106EDC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106EDC, r2 as u8);
    16
}
/// $001FC2: clr.b $106edd.l
pub fn i_001fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FC8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106EDD) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106EDD, r2 as u8);
    16
}
/// $001FC8: clr.b $106ed8.l
pub fn i_001fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FCE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED8, r2 as u8);
    16
}
/// $001FCE: clr.b $10e48b.l
pub fn i_001fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FD4;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E48B) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E48B, r2 as u8);
    16
}
/// $001FD4: bset.b #$7, $10fd80.l
pub fn i_001fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FDC;
    cpu.ir = 0x8F9;
    let mk: u32 = 1u32 << ((7) % 8);
    let v1 = bus.read8(0x10FD80) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    bus.write8(0x10FD80, r2 as u8);
    20
}
/// $001FDC: lea.l $1fe2(pc), a1
pub fn i_001fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x1FE2;
    16
}
/// $001FE0: move.l a1, (a6)
pub fn i_001fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $001FE2: addq.w #$1, $106ee0.l
pub fn i_001fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FE8;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x106EE0) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x106EE0, r2 as u16);
    16
}
/// $001FE8: clr.b $106edd.l
pub fn i_001fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FEE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106EDD) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106EDD, r2 as u8);
    16
}
/// $001FEE: cmpi.b #$0, $106ede.l
pub fn i_001fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FF6;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EDE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $001FF6: beq.w $2004
pub fn i_001ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1FFA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2004; return 10; }
    8
}
/// $001FFA: cmpi.b #$1, $106ed9.l
pub fn i_001ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2002;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED9) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $002002: bls.b $1fe2
pub fn i_002002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2004;
    cpu.ir = 0x63DE;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x1FE2; return 10; }
    8
}
/// $002004: tst.b $106ed8.l
pub fn i_002004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x200A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $00200A: beq.b $1fe2
pub fn i_00200a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x200C;
    cpu.ir = 0x67D6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1FE2; return 10; }
    8
}
/// $00200C: clr.b $106ed9.l
pub fn i_00200c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2012;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED9) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED9, r2 as u8);
    16
}
/// $002012: jsr $5cc1a.l
pub fn i_002012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2018);
    cpu.pc = 0x5CC1A;
    16
}
/// $002018: jsr $13c3c.l
pub fn i_002018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x201E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x201E);
    cpu.pc = 0x13C3C;
    16
}
/// $00201E: bcc.w $2030
pub fn i_00201e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2022;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2030; return 10; }
    8
}
/// $002022: clr.w $106ee2.l
pub fn i_002022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2028;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106EE2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106EE2, r2 as u16);
    16
}
/// $002028: clr.b $106ed8.l
pub fn i_002028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x202E;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED8, r2 as u8);
    16
}
/// $00202E: bra.b $1fe2
pub fn i_00202e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2030;
    cpu.ir = 0x60B2;
    cpu.pc = 0x1FE2;
    10
}
/// $002030: jsr $2242.l
pub fn i_002030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2036);
    cpu.pc = 0x2242;
    16
}
/// $002036: move.b d0, $106f29.l
pub fn i_002036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x203C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F29, v1 as u8);
    16
}
/// $00203C: addq.b #$1, $106f28.l
pub fn i_00203c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2042;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x106F28) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x106F28, r2 as u8);
    16
}
/// $002042: move.b $10e48b.l, d0
pub fn i_002042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2048;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E48B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $002048: move.b d0, $10e48a.l
pub fn i_002048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x204E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E48A, v1 as u8);
    16
}
/// $00204E: clr.b $10e48b.l
pub fn i_00204e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2054;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E48B) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E48B, r2 as u8);
    16
}
/// $002054: jsr $1b40.l
pub fn i_002054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x205A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x205A);
    cpu.pc = 0x1B40;
    16
}
/// $00205A: move.w $106e8a.l, d0
pub fn i_00205a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2060;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106E8A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $002060: move.w d0, $106e88.l
pub fn i_002060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2066;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106E88, v1 as u16);
    16
}
/// $002066: clr.w $106e8a.l
pub fn i_002066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x206C;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106E8A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106E8A, r2 as u16);
    16
}
/// $00206C: clr.w $106e8c.l
pub fn i_00206c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2072;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106E8C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106E8C, r2 as u16);
    16
}
/// $002072: move.b #$1, $106ede.l
pub fn i_002072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x207A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EDE, v1 as u8);
    16
}
/// $00207A: move.l a6, -(a7)
pub fn i_00207a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x207C;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $00207C: jsr $5a9ba.l
pub fn i_00207c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2082);
    cpu.pc = 0x5A9BA;
    16
}
/// $002082: move.w $106ee0.l, d0
pub fn i_002082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2088;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106EE0) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $002088: clr.w $106ee0.l
pub fn i_002088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x208E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106EE0) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106EE0, r2 as u16);
    16
}
/// $00208E: clr.w $106ee2.l
pub fn i_00208e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2094;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106EE2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106EE2, r2 as u16);
    16
}
/// $002094: movea.l (a7)+, a6
pub fn i_002094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2096;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $002096: rts
pub fn i_002096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2098;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VBlank_RenderFrame_002098  ($002098, 66 B, boot/boot_script_sound_palslots_000400.s)
/// $002098: clr.b $12(a6)
pub fn i_002098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x209C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $00209C: clr.b $13(a6)
pub fn i_00209c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20A0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0020A0: clr.b $5b(a6)
pub fn i_0020a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20A4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0020A4: clr.b $5a(a6)
pub fn i_0020a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20A8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0020A8: move.l a6, -(a7)
pub fn i_0020a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20AA;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0020AA: move.b #$ff, $106edc.l
pub fn i_0020aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20B2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EDC, v1 as u8);
    16
}
/// $0020B2: jsr $2c86.l
pub fn i_0020b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x20B8);
    cpu.pc = 0x2C86;
    16
}
/// $0020B8: move.b d0, $3a0001.l
pub fn i_0020b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20BE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $0020BE: move.b #$ff, $106edb.l
pub fn i_0020be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20C6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EDB, v1 as u8);
    16
}
/// $0020C6: clr.b $106edc.l
pub fn i_0020c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20CC;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106EDC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106EDC, r2 as u8);
    16
}
/// $0020CC: jsr $5b232.l
pub fn i_0020cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x20D2);
    cpu.pc = 0x5B232;
    16
}
/// $0020D2: move.b d0, $3a0001.l
pub fn i_0020d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20D8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $0020D8: movea.l (a7)+, a6
pub fn i_0020d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20DA;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
// ===== ClrRamWord_0020da  ($0020DA, 8 B, thunks/clr_ram_word.c)
/// $0020DA: clr.b $106ed8.l
pub fn i_0020da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20E0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED8, r2 as u8);
    16
}
/// $0020E0: rts
pub fn i_0020e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Init_MasterSubsystems_0020E2  ($0020E2, 68 B, boot/init_master_subsystems_0020e2.s)
/// $0020E2: move.b d0, $300001.l
pub fn i_0020e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20E8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $0020E8: jsr $1379a.l
pub fn i_0020e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x20EE);
    cpu.pc = 0x1379A;
    16
}
/// $0020EE: move.b d0, $300001.l
pub fn i_0020ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20F4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $0020F4: jsr $13d12.l
pub fn i_0020f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x20FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x20FA);
    cpu.pc = 0x13D12;
    16
}
/// $0020FA: move.b d0, $300001.l
pub fn i_0020fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2100;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $002100: jsr $5a824.l
pub fn i_002100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2106);
    cpu.pc = 0x5A824;
    16
}
/// $002106: move.b d0, $300001.l
pub fn i_002106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x210C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $00210C: jsr $410.l
pub fn i_00210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2112;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2112);
    cpu.pc = 0x410;
    16
}
/// $002112: move.b d0, $300001.l
pub fn i_002112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2118;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $002118: jsr $5cc0e.l
pub fn i_002118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x211E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x211E);
    cpu.pc = 0x5CC0E;
    16
}
/// $00211E: jsr $ffe.l
pub fn i_00211e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2124);
    cpu.pc = 0xFFE;
    16
}
/// $002124: rts
pub fn i_002124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2126;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrRamWord_002126  ($002126, 8 B, thunks/clr_ram_word.c)
/// $002126: clr.b $10007a.l
pub fn i_002126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x212C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10007A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10007A, r2 as u8);
    16
}
/// $00212C: rts
pub fn i_00212c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x212E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputQueue_InitAndPushOp4_00212E  ($00212E, 110 B, input/queue_push_00212e.s)
/// $00212E: clr.l $1081a4.l
pub fn i_00212e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2134;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x1081A4);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x1081A4, r2);
    20
}
/// $002134: clr.b $1081a8.l
pub fn i_002134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x213A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x1081A8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x1081A8, r2 as u8);
    16
}
/// $00213A: clr.b $1081a9.l
pub fn i_00213a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2140;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x1081A9) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x1081A9, r2 as u8);
    16
}
/// $002140: clr.b $1081aa.l
pub fn i_002140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2146;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x1081AA) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x1081AA, r2 as u8);
    16
}
/// $002146: move.b #$ff, $1081ac.l
pub fn i_002146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x214E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081AC, v1 as u8);
    16
}
/// $00214E: move.b #$4, d0
pub fn i_00214e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2152;
    cpu.ir = 0x103C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $002152: cmpi.b #$20, d0
pub fn i_002152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2156;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $002156: bcc.w $216a
pub fn i_002156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x215A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x216A; return 10; }
    8
}
/// $00215A: cmp.b $1081ac.l, d0
pub fn i_00215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2160;
    cpu.ir = 0xB039;
    let v2 = bus.read8(0x1081AC) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $002160: beq.w $2190
pub fn i_002160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2164;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2190; return 10; }
    8
}
/// $002164: move.b d0, $1081ac.l
pub fn i_002164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x216A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081AC, v1 as u8);
    16
}
/// $00216A: lea.l $108184.l, a0
pub fn i_00216a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2170;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x108184;
    20
}
/// $002170: move.w $1081a6.l, d1
pub fn i_002170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2176;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x1081A6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $002176: move.b d0, (a0, d1.w)
pub fn i_002176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x217A;
    cpu.ir = 0x1180;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $00217A: addq.w #$1, d1
pub fn i_00217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x217C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $00217C: andi.w #$1f, d1
pub fn i_00217c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2180;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002180: cmp.w $1081a4.l, d1
pub fn i_002180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2186;
    cpu.ir = 0xB279;
    let v2 = bus.read16(0x1081A4) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $002186: beq.w $2192
pub fn i_002186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x218A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2192; return 10; }
    8
}
/// $00218A: move.w d1, $1081a6.l
pub fn i_00218a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2190;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x1081A6, v1 as u16);
    16
}
/// $002190: rts
pub fn i_002190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2192;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002192: move.b #$ff, $1081ac.l
pub fn i_002192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x219A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081AC, v1 as u8);
    16
}
/// $00219A: rts
pub fn i_00219a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x219C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClampD0ToRange  ($00219C, 10 B, thunks/range_guards.c)
/// $00219C: cmpi.w #$2000, d0
pub fn i_00219c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21A0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2000, Size::Word, false);
    8
}
/// $0021A0: bcs.w $21a6
pub fn i_0021a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21A4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x21A6; return 10; }
    8
}
/// $0021A4: rts
pub fn i_0021a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sound_PushCommandFiltered_0021a6  ($0021A6, 124 B, boot/boot_script_sound_palslots_000400.s)
/// $0021A6: cmpi.b #$20, d0
pub fn i_0021a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21AA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $0021AA: bcc.w $21b0
pub fn i_0021aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x21B0; return 10; }
    8
}
/// $0021AE: rts
pub fn i_0021ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0021B0: move.w d0, d1
pub fn i_0021b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21B2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0021B2: clr.b d1
pub fn i_0021b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21B4;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0021B4: cmpi.w #$100, d1
pub fn i_0021b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21B8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $0021B8: beq.w $21fe
pub fn i_0021b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x21FE; return 10; }
    8
}
/// $0021BC: cmpi.w #$300, d1
pub fn i_0021bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21C0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $0021C0: beq.w $21fe
pub fn i_0021c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x21FE; return 10; }
    8
}
/// $0021C4: cmpi.w #$200, d1
pub fn i_0021c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21C8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x200, Size::Word, false);
    8
}
/// $0021C8: beq.w $21fe
pub fn i_0021c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x21FE; return 10; }
    8
}
/// $0021CC: cmpi.w #$1000, d1
pub fn i_0021cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21D0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1000, Size::Word, false);
    8
}
/// $0021D0: bcc.w $21de
pub fn i_0021d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x21DE; return 10; }
    8
}
/// $0021D4: tst.w d1
pub fn i_0021d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21D6;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0021D6: beq.w $21de
pub fn i_0021d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x21DE; return 10; }
    8
}
/// $0021DA: bra.w $2200
pub fn i_0021da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x2200;
    10
}
/// $0021DE: tst.b $10fd82.l
pub fn i_0021de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21E4;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0021E4: beq.w $2200
pub fn i_0021e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2200; return 10; }
    8
}
/// $0021E8: tst.b $d00046.l
pub fn i_0021e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21EE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00046) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0021EE: beq.w $2200
pub fn i_0021ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2200; return 10; }
    8
}
/// $0021F2: cmpi.b #$1, $10fdaf.l
pub fn i_0021f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21FA;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0021FA: bne.w $2200
pub fn i_0021fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x21FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2200; return 10; }
    8
}
/// $0021FE: rts
pub fn i_0021fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2200;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002200: tst.b $1081aa.l
pub fn i_002200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2206;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x1081AA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002206: beq.w $2214
pub fn i_002206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x220A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2214; return 10; }
    8
}
/// $00220A: cmpi.w #$1000, d0
pub fn i_00220a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x220E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1000, Size::Word, false);
    8
}
/// $00220E: bcs.w $2214
pub fn i_00220e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2212;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2214; return 10; }
    8
}
/// $002212: rts
pub fn i_002212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2214;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002214: move.w d0, -(a7)
pub fn i_002214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2216;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $002216: lsr.w #$8, d0
pub fn i_002216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2218;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $002218: bsr.w $2152
pub fn i_002218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x221C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x221C);
    cpu.pc = 0x2152;
    18
}
/// $00221C: move.w (a7)+, d0
pub fn i_00221c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x221E;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $00221E: bra.w $2152
pub fn i_00221e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2222;
    cpu.ir = 0x6000;
    cpu.pc = 0x2152;
    10
}
// ===== Sound_Push0C_ThenClamp_002222  ($002222, 14 B, boot/boot_script_sound_palslots_000400.s)
/// $002222: move.w d0, -(a7)
pub fn i_002222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2224;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $002224: moveq #$c, d0
pub fn i_002224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2226;
    cpu.ir = 0x700C;
    cpu.d[0] = 0xC;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002226: bsr.w $2152
pub fn i_002226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x222A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x222A);
    cpu.pc = 0x2152;
    18
}
/// $00222A: move.w (a7)+, d0
pub fn i_00222a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x222C;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $00222C: bra.w $219c
pub fn i_00222c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2230;
    cpu.ir = 0x6000;
    cpu.pc = 0x219C;
    10
}
// ===== Sound_PollReply_002242  ($002242, 40 B, boot/boot_script_sound_palslots_000400.s)
/// $002242: move.b $1081a8.l, d0
pub fn i_002242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2248;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x1081A8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $002248: cmp.b $1081a9.l, d0
pub fn i_002248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x224E;
    cpu.ir = 0xB039;
    let v2 = bus.read8(0x1081A9) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $00224E: beq.w $2264
pub fn i_00224e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2252;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2264; return 10; }
    8
}
/// $002252: move.b d0, $1081a9.l
pub fn i_002252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2258;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081A9, v1 as u8);
    16
}
/// $002258: bpl.w $2264
pub fn i_002258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x225C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x2264; return 10; }
    8
}
/// $00225C: subi.b #$80, d0
pub fn i_00225c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2260;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x80, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002260: lsr.b #$1, d0
pub fn i_002260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2262;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $002262: rts
pub fn i_002262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2264;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002264: move.b #$ff, d0
pub fn i_002264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2268;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $002268: rts
pub fn i_002268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x226A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sound_PumpQueue_00226a  ($00226A, 84 B, boot/boot_script_sound_palslots_000400.s)
/// $00226A: tst.b $10007a.l
pub fn i_00226a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2270;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10007A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002270: beq.w $2286
pub fn i_002270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2274;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2286; return 10; }
    8
}
/// $002274: clr.b $10007a.l
pub fn i_002274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x227A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10007A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10007A, r2 as u8);
    16
}
/// $00227A: move.b #$f, $320000.l
pub fn i_00227a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2282;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $002282: bra.w $22bc
pub fn i_002282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2286;
    cpu.ir = 0x6000;
    cpu.pc = 0x22BC;
    10
}
/// $002286: move.b $320000.l, d0
pub fn i_002286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x228C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x320000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $00228C: move.b d0, $1081a8.l
pub fn i_00228c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2292;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081A8, v1 as u8);
    16
}
/// $002292: move.w $1081a4.l, d1
pub fn i_002292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2298;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x1081A4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $002298: cmp.w $1081a6.l, d1
pub fn i_002298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x229E;
    cpu.ir = 0xB279;
    let v2 = bus.read16(0x1081A6) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $00229E: beq.w $22bc
pub fn i_00229e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x22BC; return 10; }
    8
}
/// $0022A2: lea.l $108184.l, a0
pub fn i_0022a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x108184;
    20
}
/// $0022A8: move.b (a0, d1.w), $320000.l
pub fn i_0022a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22B0;
    cpu.ir = 0x13F0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v3 as u8);
    26
}
/// $0022B0: addq.w #$1, d1
pub fn i_0022b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22B2;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0022B2: andi.w #$1f, d1
pub fn i_0022b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22B6;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0022B6: move.w d1, $1081a4.l
pub fn i_0022b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22BC;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x1081A4, v1 as u16);
    16
}
/// $0022BC: rts
pub fn i_0022bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BiosEntry_COIN_SOUND  ($0022BE, 10 B, bios/bios_entry_points.c)
/// $0022BE: move.b #$1, $10007a.l
pub fn i_0022be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22C6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10007A, v1 as u8);
    16
}
/// $0022C6: rts
pub fn i_0022c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sound_Push06_0022c8  ($0022C8, 6 B, boot/boot_script_sound_palslots_000400.s)
/// $0022C8: moveq #$6, d0
pub fn i_0022c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22CA;
    cpu.ir = 0x7006;
    cpu.d[0] = 0x6;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0022CA: bra.w $2152
pub fn i_0022ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x22CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x2152;
    10
}
// ===== InputGuardCall219c  ($002352, 28 B, thunks/range_guards.c)
/// $002352: cmpi.b #$1, $10fdaf.l
pub fn i_002352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x235A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $00235A: bne.w $2368
pub fn i_00235a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x235E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2368; return 10; }
    8
}
/// $00235E: tst.b $10fd8d.l
pub fn i_00235e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2364;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8D) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002364: beq.w $236c
pub fn i_002364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2368;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x236C; return 10; }
    8
}
/// $002368: jsr $219c(pc)
pub fn i_002368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x236C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x236C);
    cpu.pc = 0x219C;
    16
}
/// $00236C: rts
pub fn i_00236c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x236E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_AllocSpriteSlot_00236E  ($00236E, 1594 B, video/sprite_slot_alloc_00236e.s)
/// $00236E: jsr $29a8(pc)
pub fn i_00236e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2372;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2372);
    cpu.pc = 0x29A8;
    16
}
/// $002372: tst.b d2
pub fn i_002372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2374;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $002374: bne.w $29a6
pub fn i_002374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2378;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29A6; return 10; }
    8
}
/// $002378: cmpi.w #$3, $1e(a6)
pub fn i_002378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x237E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $00237E: bge.w $29a6
pub fn i_00237e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2382;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x29A6; return 10; }
    8
}
/// $002382: tst.w $1082c2.l
pub fn i_002382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2388;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x1082C2) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $002388: beq.w $29a6
pub fn i_002388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x238C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29A6; return 10; }
    8
}
/// $00238C: andi.l #$ffff, d1
pub fn i_00238c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2392;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $002392: lsl.l #$6, d1
pub fn i_002392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2394;
    cpu.ir = 0xED89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[1] = r;
    18
}
/// $002394: lea.l $14e00.l, a1
pub fn i_002394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x239A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x14E00;
    20
}
/// $00239A: add.l a1, d1
pub fn i_00239a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x239C;
    cpu.ir = 0xD289;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[1] = r;
    4
}
/// $00239C: move.l d1, d4
pub fn i_00239c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x239E;
    cpu.ir = 0x2801;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $00239E: lea.l $1082c8.l, a1
pub fn i_00239e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23A4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $0023A4: adda.l #$200, a1
pub fn i_0023a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23AA;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x200;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0023AA: adda.l #$c80, a1
pub fn i_0023aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23B0;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC80;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0023B0: adda.l #$400, a1
pub fn i_0023b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23B6;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x400;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0023B6: clr.w d2
pub fn i_0023b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23B8;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0023B8: cmp.l $2(a1), d1
pub fn i_0023b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23BC;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0023BC: bne.w $23ce
pub fn i_0023bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x23CE; return 10; }
    8
}
/// $0023C0: move.b (a1), d5
pub fn i_0023c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23C2;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0023C2: andi.b #$84, d5
pub fn i_0023c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23C6;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0023C6: bne.w $23ce
pub fn i_0023c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x23CE; return 10; }
    8
}
/// $0023CA: jmp $296a(pc)
pub fn i_0023ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23CE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0023CE: addq.w #$1, d2
pub fn i_0023ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23D0;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0023D0: adda.l #$20, a1
pub fn i_0023d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23D6;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0023D6: cmp.l $2(a1), d1
pub fn i_0023d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23DA;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0023DA: bne.w $23ec
pub fn i_0023da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x23EC; return 10; }
    8
}
/// $0023DE: move.b (a1), d5
pub fn i_0023de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23E0;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0023E0: andi.b #$84, d5
pub fn i_0023e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23E4;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0023E4: bne.w $23ec
pub fn i_0023e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x23EC; return 10; }
    8
}
/// $0023E8: jmp $296a(pc)
pub fn i_0023e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23EC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0023EC: addq.w #$1, d2
pub fn i_0023ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23EE;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0023EE: adda.l #$20, a1
pub fn i_0023ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23F4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0023F4: cmp.l $2(a1), d1
pub fn i_0023f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23F8;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0023F8: bne.w $240a
pub fn i_0023f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x240A; return 10; }
    8
}
/// $0023FC: move.b (a1), d5
pub fn i_0023fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x23FE;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0023FE: andi.b #$84, d5
pub fn i_0023fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2402;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002402: bne.w $240a
pub fn i_002402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2406;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x240A; return 10; }
    8
}
/// $002406: jmp $296a(pc)
pub fn i_002406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x240A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $00240A: addq.w #$1, d2
pub fn i_00240a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x240C;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $00240C: adda.l #$20, a1
pub fn i_00240c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2412;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002412: cmp.l $2(a1), d1
pub fn i_002412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2416;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002416: bne.w $2428
pub fn i_002416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x241A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2428; return 10; }
    8
}
/// $00241A: move.b (a1), d5
pub fn i_00241a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x241C;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $00241C: andi.b #$84, d5
pub fn i_00241c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2420;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002420: bne.w $2428
pub fn i_002420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2424;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2428; return 10; }
    8
}
/// $002424: jmp $296a(pc)
pub fn i_002424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2428;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002428: addq.w #$1, d2
pub fn i_002428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x242A;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $00242A: adda.l #$20, a1
pub fn i_00242a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2430;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002430: cmp.l $2(a1), d1
pub fn i_002430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2434;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002434: bne.w $2446
pub fn i_002434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2438;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2446; return 10; }
    8
}
/// $002438: move.b (a1), d5
pub fn i_002438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x243A;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $00243A: andi.b #$84, d5
pub fn i_00243a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x243E;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00243E: bne.w $2446
pub fn i_00243e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2442;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2446; return 10; }
    8
}
/// $002442: jmp $296a(pc)
pub fn i_002442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2446;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002446: addq.w #$1, d2
pub fn i_002446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2448;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002448: adda.l #$20, a1
pub fn i_002448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x244E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00244E: cmp.l $2(a1), d1
pub fn i_00244e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2452;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002452: bne.w $2464
pub fn i_002452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2456;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2464; return 10; }
    8
}
/// $002456: move.b (a1), d5
pub fn i_002456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2458;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002458: andi.b #$84, d5
pub fn i_002458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x245C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00245C: bne.w $2464
pub fn i_00245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2460;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2464; return 10; }
    8
}
/// $002460: jmp $296a(pc)
pub fn i_002460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2464;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002464: addq.w #$1, d2
pub fn i_002464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2466;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002466: adda.l #$20, a1
pub fn i_002466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x246C;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00246C: cmp.l $2(a1), d1
pub fn i_00246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2470;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002470: bne.w $2482
pub fn i_002470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2474;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2482; return 10; }
    8
}
/// $002474: move.b (a1), d5
pub fn i_002474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2476;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002476: andi.b #$84, d5
pub fn i_002476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x247A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00247A: bne.w $2482
pub fn i_00247a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x247E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2482; return 10; }
    8
}
/// $00247E: jmp $296a(pc)
pub fn i_00247e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2482;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002482: addq.w #$1, d2
pub fn i_002482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2484;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002484: adda.l #$20, a1
pub fn i_002484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x248A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00248A: cmp.l $2(a1), d1
pub fn i_00248a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x248E;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00248E: bne.w $24a0
pub fn i_00248e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2492;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24A0; return 10; }
    8
}
/// $002492: move.b (a1), d5
pub fn i_002492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2494;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002494: andi.b #$84, d5
pub fn i_002494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2498;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002498: bne.w $24a0
pub fn i_002498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x249C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24A0; return 10; }
    8
}
/// $00249C: jmp $296a(pc)
pub fn i_00249c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24A0;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0024A0: addq.w #$1, d2
pub fn i_0024a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24A2;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0024A2: adda.l #$20, a1
pub fn i_0024a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24A8;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0024A8: cmp.l $2(a1), d1
pub fn i_0024a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24AC;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0024AC: bne.w $24be
pub fn i_0024ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24BE; return 10; }
    8
}
/// $0024B0: move.b (a1), d5
pub fn i_0024b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24B2;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0024B2: andi.b #$84, d5
pub fn i_0024b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24B6;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0024B6: bne.w $24be
pub fn i_0024b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24BE; return 10; }
    8
}
/// $0024BA: jmp $296a(pc)
pub fn i_0024ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24BE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0024BE: addq.w #$1, d2
pub fn i_0024be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24C0;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0024C0: adda.l #$20, a1
pub fn i_0024c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24C6;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0024C6: cmp.l $2(a1), d1
pub fn i_0024c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24CA;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0024CA: bne.w $24dc
pub fn i_0024ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24DC; return 10; }
    8
}
/// $0024CE: move.b (a1), d5
pub fn i_0024ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24D0;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0024D0: andi.b #$84, d5
pub fn i_0024d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24D4;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0024D4: bne.w $24dc
pub fn i_0024d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24DC; return 10; }
    8
}
/// $0024D8: jmp $296a(pc)
pub fn i_0024d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24DC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0024DC: addq.w #$1, d2
pub fn i_0024dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24DE;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0024DE: adda.l #$20, a1
pub fn i_0024de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0024E4: cmp.l $2(a1), d1
pub fn i_0024e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24E8;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0024E8: bne.w $24fa
pub fn i_0024e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24FA; return 10; }
    8
}
/// $0024EC: move.b (a1), d5
pub fn i_0024ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24EE;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0024EE: andi.b #$84, d5
pub fn i_0024ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F2;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0024F2: bne.w $24fa
pub fn i_0024f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x24FA; return 10; }
    8
}
/// $0024F6: jmp $296a(pc)
pub fn i_0024f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0024FA: addq.w #$1, d2
pub fn i_0024fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x24FC;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0024FC: adda.l #$20, a1
pub fn i_0024fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2502;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002502: cmp.l $2(a1), d1
pub fn i_002502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2506;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002506: bne.w $2518
pub fn i_002506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2518; return 10; }
    8
}
/// $00250A: move.b (a1), d5
pub fn i_00250a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x250C;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $00250C: andi.b #$84, d5
pub fn i_00250c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2510;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002510: bne.w $2518
pub fn i_002510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2514;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2518; return 10; }
    8
}
/// $002514: jmp $296a(pc)
pub fn i_002514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2518;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002518: addq.w #$1, d2
pub fn i_002518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x251A;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $00251A: adda.l #$20, a1
pub fn i_00251a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2520;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002520: cmp.l $2(a1), d1
pub fn i_002520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2524;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002524: bne.w $2536
pub fn i_002524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2528;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2536; return 10; }
    8
}
/// $002528: move.b (a1), d5
pub fn i_002528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252A;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $00252A: andi.b #$84, d5
pub fn i_00252a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x252E;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00252E: bne.w $2536
pub fn i_00252e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2532;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2536; return 10; }
    8
}
/// $002532: jmp $296a(pc)
pub fn i_002532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2536;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002536: addq.w #$1, d2
pub fn i_002536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2538;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002538: adda.l #$20, a1
pub fn i_002538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x253E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00253E: cmp.l $2(a1), d1
pub fn i_00253e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2542;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002542: bne.w $2554
pub fn i_002542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2546;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2554; return 10; }
    8
}
/// $002546: move.b (a1), d5
pub fn i_002546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2548;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002548: andi.b #$84, d5
pub fn i_002548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x254C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00254C: bne.w $2554
pub fn i_00254c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2550;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2554; return 10; }
    8
}
/// $002550: jmp $296a(pc)
pub fn i_002550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2554;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002554: addq.w #$1, d2
pub fn i_002554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2556;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002556: adda.l #$20, a1
pub fn i_002556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x255C;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00255C: cmp.l $2(a1), d1
pub fn i_00255c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2560;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002560: bne.w $2572
pub fn i_002560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2564;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2572; return 10; }
    8
}
/// $002564: move.b (a1), d5
pub fn i_002564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2566;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002566: andi.b #$84, d5
pub fn i_002566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00256A: bne.w $2572
pub fn i_00256a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x256E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2572; return 10; }
    8
}
/// $00256E: jmp $296a(pc)
pub fn i_00256e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2572;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002572: addq.w #$1, d2
pub fn i_002572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2574;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002574: adda.l #$20, a1
pub fn i_002574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00257A: cmp.l $2(a1), d1
pub fn i_00257a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x257E;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00257E: bne.w $2590
pub fn i_00257e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2582;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2590; return 10; }
    8
}
/// $002582: move.b (a1), d5
pub fn i_002582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2584;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002584: andi.b #$84, d5
pub fn i_002584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2588;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002588: bne.w $2590
pub fn i_002588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x258C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2590; return 10; }
    8
}
/// $00258C: jmp $296a(pc)
pub fn i_00258c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2590;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002590: addq.w #$1, d2
pub fn i_002590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2592;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002592: adda.l #$20, a1
pub fn i_002592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2598;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002598: cmp.l $2(a1), d1
pub fn i_002598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x259C;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00259C: bne.w $25ae
pub fn i_00259c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25AE; return 10; }
    8
}
/// $0025A0: move.b (a1), d5
pub fn i_0025a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A2;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0025A2: andi.b #$84, d5
pub fn i_0025a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25A6;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0025A6: bne.w $25ae
pub fn i_0025a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25AE; return 10; }
    8
}
/// $0025AA: jmp $296a(pc)
pub fn i_0025aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25AE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0025AE: addq.w #$1, d2
pub fn i_0025ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B0;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0025B0: adda.l #$20, a1
pub fn i_0025b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25B6;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0025B6: cmp.l $2(a1), d1
pub fn i_0025b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BA;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0025BA: bne.w $25cc
pub fn i_0025ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25CC; return 10; }
    8
}
/// $0025BE: move.b (a1), d5
pub fn i_0025be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C0;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0025C0: andi.b #$84, d5
pub fn i_0025c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C4;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0025C4: bne.w $25cc
pub fn i_0025c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25CC; return 10; }
    8
}
/// $0025C8: jmp $296a(pc)
pub fn i_0025c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0025CC: addq.w #$1, d2
pub fn i_0025cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25CE;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0025CE: adda.l #$20, a1
pub fn i_0025ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0025D4: cmp.l $2(a1), d1
pub fn i_0025d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25D8;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0025D8: bne.w $25ea
pub fn i_0025d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25EA; return 10; }
    8
}
/// $0025DC: move.b (a1), d5
pub fn i_0025dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25DE;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0025DE: andi.b #$84, d5
pub fn i_0025de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E2;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0025E2: bne.w $25ea
pub fn i_0025e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x25EA; return 10; }
    8
}
/// $0025E6: jmp $296a(pc)
pub fn i_0025e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0025EA: addq.w #$1, d2
pub fn i_0025ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25EC;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0025EC: adda.l #$20, a1
pub fn i_0025ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0025F2: cmp.l $2(a1), d1
pub fn i_0025f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25F6;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0025F6: bne.w $2608
pub fn i_0025f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2608; return 10; }
    8
}
/// $0025FA: move.b (a1), d5
pub fn i_0025fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x25FC;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0025FC: andi.b #$84, d5
pub fn i_0025fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2600;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002600: bne.w $2608
pub fn i_002600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2604;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2608; return 10; }
    8
}
/// $002604: jmp $296a(pc)
pub fn i_002604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2608;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002608: addq.w #$1, d2
pub fn i_002608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x260A;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $00260A: adda.l #$20, a1
pub fn i_00260a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2610;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002610: cmp.l $2(a1), d1
pub fn i_002610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2614;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002614: bne.w $2626
pub fn i_002614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2618;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2626; return 10; }
    8
}
/// $002618: move.b (a1), d5
pub fn i_002618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261A;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $00261A: andi.b #$84, d5
pub fn i_00261a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x261E;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00261E: bne.w $2626
pub fn i_00261e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2622;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2626; return 10; }
    8
}
/// $002622: jmp $296a(pc)
pub fn i_002622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2626;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002626: addq.w #$1, d2
pub fn i_002626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2628;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002628: adda.l #$20, a1
pub fn i_002628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x262E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00262E: cmp.l $2(a1), d1
pub fn i_00262e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2632;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002632: bne.w $2644
pub fn i_002632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2636;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2644; return 10; }
    8
}
/// $002636: move.b (a1), d5
pub fn i_002636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2638;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002638: andi.b #$84, d5
pub fn i_002638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x263C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00263C: bne.w $2644
pub fn i_00263c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2640;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2644; return 10; }
    8
}
/// $002640: jmp $296a(pc)
pub fn i_002640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2644;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002644: addq.w #$1, d2
pub fn i_002644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2646;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002646: adda.l #$20, a1
pub fn i_002646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x264C;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00264C: cmp.l $2(a1), d1
pub fn i_00264c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2650;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002650: bne.w $2662
pub fn i_002650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2654;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2662; return 10; }
    8
}
/// $002654: move.b (a1), d5
pub fn i_002654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2656;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002656: andi.b #$84, d5
pub fn i_002656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00265A: bne.w $2662
pub fn i_00265a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x265E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2662; return 10; }
    8
}
/// $00265E: jmp $296a(pc)
pub fn i_00265e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2662;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002662: addq.w #$1, d2
pub fn i_002662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2664;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002664: adda.l #$20, a1
pub fn i_002664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00266A: cmp.l $2(a1), d1
pub fn i_00266a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x266E;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00266E: bne.w $2680
pub fn i_00266e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2672;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2680; return 10; }
    8
}
/// $002672: move.b (a1), d5
pub fn i_002672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2674;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002674: andi.b #$84, d5
pub fn i_002674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2678;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002678: bne.w $2680
pub fn i_002678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x267C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2680; return 10; }
    8
}
/// $00267C: jmp $296a(pc)
pub fn i_00267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2680;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002680: addq.w #$1, d2
pub fn i_002680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2682;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002682: adda.l #$20, a1
pub fn i_002682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2688;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002688: cmp.l $2(a1), d1
pub fn i_002688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x268C;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00268C: bne.w $269e
pub fn i_00268c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2690;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x269E; return 10; }
    8
}
/// $002690: move.b (a1), d5
pub fn i_002690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2692;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002692: andi.b #$84, d5
pub fn i_002692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2696;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002696: bne.w $269e
pub fn i_002696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x269E; return 10; }
    8
}
/// $00269A: jmp $296a(pc)
pub fn i_00269a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x269E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $00269E: addq.w #$1, d2
pub fn i_00269e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A0;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0026A0: adda.l #$20, a1
pub fn i_0026a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26A6;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0026A6: cmp.l $2(a1), d1
pub fn i_0026a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AA;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0026AA: bne.w $26bc
pub fn i_0026aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26BC; return 10; }
    8
}
/// $0026AE: move.b (a1), d5
pub fn i_0026ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B0;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0026B0: andi.b #$84, d5
pub fn i_0026b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B4;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0026B4: bne.w $26bc
pub fn i_0026b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26BC; return 10; }
    8
}
/// $0026B8: jmp $296a(pc)
pub fn i_0026b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0026BC: addq.w #$1, d2
pub fn i_0026bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26BE;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0026BE: adda.l #$20, a1
pub fn i_0026be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0026C4: cmp.l $2(a1), d1
pub fn i_0026c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26C8;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0026C8: bne.w $26da
pub fn i_0026c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26DA; return 10; }
    8
}
/// $0026CC: move.b (a1), d5
pub fn i_0026cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26CE;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0026CE: andi.b #$84, d5
pub fn i_0026ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D2;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0026D2: bne.w $26da
pub fn i_0026d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26DA; return 10; }
    8
}
/// $0026D6: jmp $296a(pc)
pub fn i_0026d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0026DA: addq.w #$1, d2
pub fn i_0026da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26DC;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0026DC: adda.l #$20, a1
pub fn i_0026dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0026E2: cmp.l $2(a1), d1
pub fn i_0026e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26E6;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0026E6: bne.w $26f8
pub fn i_0026e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26F8; return 10; }
    8
}
/// $0026EA: move.b (a1), d5
pub fn i_0026ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26EC;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0026EC: andi.b #$84, d5
pub fn i_0026ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F0;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0026F0: bne.w $26f8
pub fn i_0026f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x26F8; return 10; }
    8
}
/// $0026F4: jmp $296a(pc)
pub fn i_0026f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26F8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0026F8: addq.w #$1, d2
pub fn i_0026f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x26FA;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0026FA: adda.l #$20, a1
pub fn i_0026fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2700;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002700: cmp.l $2(a1), d1
pub fn i_002700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2704;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002704: bne.w $2716
pub fn i_002704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2708;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2716; return 10; }
    8
}
/// $002708: move.b (a1), d5
pub fn i_002708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270A;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $00270A: andi.b #$84, d5
pub fn i_00270a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x270E;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00270E: bne.w $2716
pub fn i_00270e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2712;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2716; return 10; }
    8
}
/// $002712: jmp $296a(pc)
pub fn i_002712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2716;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002716: addq.w #$1, d2
pub fn i_002716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2718;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002718: adda.l #$20, a1
pub fn i_002718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x271E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00271E: cmp.l $2(a1), d1
pub fn i_00271e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2722;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002722: bne.w $2734
pub fn i_002722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2726;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2734; return 10; }
    8
}
/// $002726: move.b (a1), d5
pub fn i_002726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2728;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002728: andi.b #$84, d5
pub fn i_002728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x272C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00272C: bne.w $2734
pub fn i_00272c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2730;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2734; return 10; }
    8
}
/// $002730: jmp $296a(pc)
pub fn i_002730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2734;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002734: addq.w #$1, d2
pub fn i_002734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2736;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002736: adda.l #$20, a1
pub fn i_002736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x273C;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00273C: cmp.l $2(a1), d1
pub fn i_00273c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2740;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002740: bne.w $2752
pub fn i_002740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2744;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2752; return 10; }
    8
}
/// $002744: move.b (a1), d5
pub fn i_002744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2746;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002746: andi.b #$84, d5
pub fn i_002746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00274A: bne.w $2752
pub fn i_00274a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x274E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2752; return 10; }
    8
}
/// $00274E: jmp $296a(pc)
pub fn i_00274e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2752;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002752: addq.w #$1, d2
pub fn i_002752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2754;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002754: adda.l #$20, a1
pub fn i_002754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00275A: cmp.l $2(a1), d1
pub fn i_00275a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x275E;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00275E: bne.w $2770
pub fn i_00275e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2762;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2770; return 10; }
    8
}
/// $002762: move.b (a1), d5
pub fn i_002762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2764;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002764: andi.b #$84, d5
pub fn i_002764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2768;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002768: bne.w $2770
pub fn i_002768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x276C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2770; return 10; }
    8
}
/// $00276C: jmp $296a(pc)
pub fn i_00276c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2770;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002770: addq.w #$1, d2
pub fn i_002770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2772;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002772: adda.l #$20, a1
pub fn i_002772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2778;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002778: cmp.l $2(a1), d1
pub fn i_002778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x277C;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00277C: bne.w $278e
pub fn i_00277c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2780;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x278E; return 10; }
    8
}
/// $002780: move.b (a1), d5
pub fn i_002780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2782;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002782: andi.b #$84, d5
pub fn i_002782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2786;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002786: bne.w $278e
pub fn i_002786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x278E; return 10; }
    8
}
/// $00278A: jmp $296a(pc)
pub fn i_00278a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x278E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $00278E: addq.w #$1, d2
pub fn i_00278e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2790;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002790: adda.l #$20, a1
pub fn i_002790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2796;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002796: cmp.l $2(a1), d1
pub fn i_002796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279A;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00279A: bne.w $27ac
pub fn i_00279a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x279E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27AC; return 10; }
    8
}
/// $00279E: move.b (a1), d5
pub fn i_00279e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A0;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0027A0: andi.b #$84, d5
pub fn i_0027a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A4;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0027A4: bne.w $27ac
pub fn i_0027a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27AC; return 10; }
    8
}
/// $0027A8: jmp $296a(pc)
pub fn i_0027a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0027AC: addq.w #$1, d2
pub fn i_0027ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27AE;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0027AE: adda.l #$20, a1
pub fn i_0027ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0027B4: cmp.l $2(a1), d1
pub fn i_0027b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27B8;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0027B8: bne.w $27ca
pub fn i_0027b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27CA; return 10; }
    8
}
/// $0027BC: move.b (a1), d5
pub fn i_0027bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27BE;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0027BE: andi.b #$84, d5
pub fn i_0027be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C2;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0027C2: bne.w $27ca
pub fn i_0027c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27C6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27CA; return 10; }
    8
}
/// $0027C6: jmp $296a(pc)
pub fn i_0027c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0027CA: addq.w #$1, d2
pub fn i_0027ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27CC;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0027CC: adda.l #$20, a1
pub fn i_0027cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0027D2: cmp.l $2(a1), d1
pub fn i_0027d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27D6;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0027D6: bne.w $27e8
pub fn i_0027d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27E8; return 10; }
    8
}
/// $0027DA: move.b (a1), d5
pub fn i_0027da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27DC;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0027DC: andi.b #$84, d5
pub fn i_0027dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E0;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0027E0: bne.w $27e8
pub fn i_0027e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x27E8; return 10; }
    8
}
/// $0027E4: jmp $296a(pc)
pub fn i_0027e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27E8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0027E8: addq.w #$1, d2
pub fn i_0027e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27EA;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0027EA: adda.l #$20, a1
pub fn i_0027ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F0;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0027F0: cmp.l $2(a1), d1
pub fn i_0027f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F4;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0027F4: bne.w $2806
pub fn i_0027f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2806; return 10; }
    8
}
/// $0027F8: move.b (a1), d5
pub fn i_0027f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FA;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0027FA: andi.b #$84, d5
pub fn i_0027fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x27FE;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0027FE: bne.w $2806
pub fn i_0027fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2802;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2806; return 10; }
    8
}
/// $002802: jmp $296a(pc)
pub fn i_002802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2806;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002806: addq.w #$1, d2
pub fn i_002806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2808;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002808: adda.l #$20, a1
pub fn i_002808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00280E: cmp.l $2(a1), d1
pub fn i_00280e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2812;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002812: bne.w $2824
pub fn i_002812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2816;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2824; return 10; }
    8
}
/// $002816: move.b (a1), d5
pub fn i_002816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2818;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002818: andi.b #$84, d5
pub fn i_002818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00281C: bne.w $2824
pub fn i_00281c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2820;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2824; return 10; }
    8
}
/// $002820: jmp $296a(pc)
pub fn i_002820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2824;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002824: addq.w #$1, d2
pub fn i_002824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2826;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002826: adda.l #$20, a1
pub fn i_002826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282C;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00282C: cmp.l $2(a1), d1
pub fn i_00282c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2830;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002830: bne.w $2842
pub fn i_002830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2834;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2842; return 10; }
    8
}
/// $002834: move.b (a1), d5
pub fn i_002834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2836;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002836: andi.b #$84, d5
pub fn i_002836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00283A: bne.w $2842
pub fn i_00283a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2842; return 10; }
    8
}
/// $00283E: jmp $296a(pc)
pub fn i_00283e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2842;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002842: addq.w #$1, d2
pub fn i_002842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2844;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002844: adda.l #$20, a1
pub fn i_002844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00284A: cmp.l $2(a1), d1
pub fn i_00284a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284E;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00284E: bne.w $2860
pub fn i_00284e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2852;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2860; return 10; }
    8
}
/// $002852: move.b (a1), d5
pub fn i_002852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2854;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002854: andi.b #$84, d5
pub fn i_002854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2858;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002858: bne.w $2860
pub fn i_002858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2860; return 10; }
    8
}
/// $00285C: jmp $296a(pc)
pub fn i_00285c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2860;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002860: addq.w #$1, d2
pub fn i_002860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2862;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002862: adda.l #$20, a1
pub fn i_002862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2868;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002868: cmp.l $2(a1), d1
pub fn i_002868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286C;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00286C: bne.w $287e
pub fn i_00286c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2870;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x287E; return 10; }
    8
}
/// $002870: move.b (a1), d5
pub fn i_002870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2872;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002872: andi.b #$84, d5
pub fn i_002872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2876;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002876: bne.w $287e
pub fn i_002876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x287E; return 10; }
    8
}
/// $00287A: jmp $296a(pc)
pub fn i_00287a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $00287E: addq.w #$1, d2
pub fn i_00287e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2880;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002880: adda.l #$20, a1
pub fn i_002880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2886;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $002886: cmp.l $2(a1), d1
pub fn i_002886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288A;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $00288A: bne.w $289c
pub fn i_00288a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x289C; return 10; }
    8
}
/// $00288E: move.b (a1), d5
pub fn i_00288e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2890;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002890: andi.b #$84, d5
pub fn i_002890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2894;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $002894: bne.w $289c
pub fn i_002894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2898;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x289C; return 10; }
    8
}
/// $002898: jmp $296a(pc)
pub fn i_002898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $00289C: addq.w #$1, d2
pub fn i_00289c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289E;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $00289E: adda.l #$20, a1
pub fn i_00289e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28A4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0028A4: cmp.l $2(a1), d1
pub fn i_0028a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28A8;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0028A8: bne.w $28ba
pub fn i_0028a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28BA; return 10; }
    8
}
/// $0028AC: move.b (a1), d5
pub fn i_0028ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AE;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0028AE: andi.b #$84, d5
pub fn i_0028ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B2;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0028B2: bne.w $28ba
pub fn i_0028b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28BA; return 10; }
    8
}
/// $0028B6: jmp $296a(pc)
pub fn i_0028b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0028BA: addq.w #$1, d2
pub fn i_0028ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0028BC: adda.l #$20, a1
pub fn i_0028bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0028C2: cmp.l $2(a1), d1
pub fn i_0028c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C6;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0028C6: bne.w $28d8
pub fn i_0028c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D8; return 10; }
    8
}
/// $0028CA: move.b (a1), d5
pub fn i_0028ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CC;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0028CC: andi.b #$84, d5
pub fn i_0028cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D0;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0028D0: bne.w $28d8
pub fn i_0028d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D8; return 10; }
    8
}
/// $0028D4: jmp $296a(pc)
pub fn i_0028d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0028D8: addq.w #$1, d2
pub fn i_0028d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0028DA: adda.l #$20, a1
pub fn i_0028da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E0;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0028E0: cmp.l $2(a1), d1
pub fn i_0028e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E4;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $0028E4: bne.w $28f6
pub fn i_0028e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28F6; return 10; }
    8
}
/// $0028E8: move.b (a1), d5
pub fn i_0028e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EA;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0028EA: andi.b #$84, d5
pub fn i_0028ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EE;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0028EE: bne.w $28f6
pub fn i_0028ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28F6; return 10; }
    8
}
/// $0028F2: jmp $296a(pc)
pub fn i_0028f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $0028F6: addq.w #$1, d2
pub fn i_0028f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F8;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0028F8: adda.l #$20, a1
pub fn i_0028f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FE;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0028FE: cmp.l $2(a1), d1
pub fn i_0028fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2902;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002902: bne.w $2914
pub fn i_002902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2906;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2914; return 10; }
    8
}
/// $002906: move.b (a1), d5
pub fn i_002906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2908;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002908: andi.b #$84, d5
pub fn i_002908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00290C: bne.w $2914
pub fn i_00290c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2910;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2914; return 10; }
    8
}
/// $002910: jmp $296a(pc)
pub fn i_002910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2914;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002914: addq.w #$1, d2
pub fn i_002914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2916;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002916: adda.l #$20, a1
pub fn i_002916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291C;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00291C: cmp.l $2(a1), d1
pub fn i_00291c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2920;
    cpu.ir = 0xB2A9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    16
}
/// $002920: bne.w $2932
pub fn i_002920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2924;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2932; return 10; }
    8
}
/// $002924: move.b (a1), d5
pub fn i_002924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2926;
    cpu.ir = 0x1A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $002926: andi.b #$84, d5
pub fn i_002926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $00292A: bne.w $2932
pub fn i_00292a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2932; return 10; }
    8
}
/// $00292E: jmp $296a(pc)
pub fn i_00292e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2932;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x296A;
    8
}
/// $002932: addq.w #$1, d2
pub fn i_002932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2934;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002934: adda.l #$20, a1
pub fn i_002934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $00293A: subq.w #$1, $1082c2.l
pub fn i_00293a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2940;
    cpu.ir = 0x5379;
    let v1 = bus.read16(0x1082C2) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x1082C2, r2 as u16);
    16
}
/// $002940: move.w $1082c4.l, d3
pub fn i_002940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2946;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x1082C4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $002946: addq.w #$1, $1082c4.l
pub fn i_002946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294C;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x1082C4) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x1082C4, r2 as u16);
    16
}
/// $00294C: cmpi.w #$30, $1082c4.l
pub fn i_00294c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2954;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x1082C4) as u32;
    sub_with_flags(cpu, v2, 0x30, Size::Word, false);
    20
}
/// $002954: blt.w $295e
pub fn i_002954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2958;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x295E; return 10; }
    8
}
/// $002958: clr.w $1082c4.l
pub fn i_002958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x1082C4) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x1082C4, r2 as u16);
    16
}
/// $00295E: lea.l $1081c2.l, a2
pub fn i_00295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2964;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x1081C2;
    20
}
/// $002964: moveq #$0, d2
pub fn i_002964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2966;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002966: move.b (a2, d3.w), d2
pub fn i_002966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x296A;
    cpu.ir = 0x1432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $00296A: move.l d4, d1
pub fn i_00296a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x296C;
    cpu.ir = 0x2204;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $00296C: addi.w #$94, d2
pub fn i_00296c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2970;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x94, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002970: move.w d2, d4
pub fn i_002970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2972;
    cpu.ir = 0x3802;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002972: lsl.w #$5, d2
pub fn i_002972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2974;
    cpu.ir = 0xEB4A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002974: lea.l $1082c8.l, a1
pub fn i_002974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x297A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $00297A: move.l d1, $2(a1, d2.w)
pub fn i_00297a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x297E;
    cpu.ir = 0x2381;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $00297E: addq.w #$1, $6(a1, d2.w)
pub fn i_00297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2982;
    cpu.ir = 0x5271;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $002982: cmpi.w #$1, $6(a1, d2.w)
pub fn i_002982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2988;
    cpu.ir = 0xC71;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    18
}
/// $002988: bne.w $2992
pub fn i_002988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x298C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2992; return 10; }
    8
}
/// $00298C: move.b #$1, (a1, d2.w)
pub fn i_00298c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2992;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002992: move.w d4, d2
pub fn i_002992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2994;
    cpu.ir = 0x3404;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002994: move.w d2, $14(a6)
pub fn i_002994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2998;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $002998: move.w $1e(a6), d1
pub fn i_002998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $00299C: add.w d1, d1
pub fn i_00299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299E;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $00299E: move.w d2, $16(a6, d1.w)
pub fn i_00299e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A2;
    cpu.ir = 0x3D82;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0029A2: addq.w #$1, $1e(a6)
pub fn i_0029a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A6;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0029A6: rts
pub fn i_0029a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeSpriteSlot_29A8  ($0029A8, 74 B, video/sprite_slot_probe_0029a8.s)
/// $0029A8: move.l d1, d5
pub fn i_0029a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29AA;
    cpu.ir = 0x2A01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0029AA: clr.b d2
pub fn i_0029aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29AC;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0029AC: andi.l #$ffff, d1
pub fn i_0029ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29B2;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $0029B2: lsl.l #$6, d1
pub fn i_0029b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29B4;
    cpu.ir = 0xED89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[1] = r;
    18
}
/// $0029B4: lea.l $14e00.l, a1
pub fn i_0029b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x14E00;
    20
}
/// $0029BA: add.l a1, d1
pub fn i_0029ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29BC;
    cpu.ir = 0xD289;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[1] = r;
    4
}
/// $0029BC: move.w $1e(a6), d3
pub fn i_0029bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C0;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0029C0: tst.w d3
pub fn i_0029c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C2;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0029C2: beq.w $29ee
pub fn i_0029c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29EE; return 10; }
    8
}
/// $0029C6: subq.w #$1, d3
pub fn i_0029c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C8;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0029C8: move.w d3, d4
pub fn i_0029c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CA;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0029CA: add.w d4, d4
pub fn i_0029ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CC;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0029CC: move.w $16(a6, d4.w), d0
pub fn i_0029cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29D0;
    cpu.ir = 0x3036;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0029D0: lsl.w #$5, d0
pub fn i_0029d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29D2;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0029D2: lea.l $1082c8.l, a1
pub fn i_0029d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $0029D8: cmp.l $2(a1, d0.w), d1
pub fn i_0029d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29DC;
    cpu.ir = 0xB2B1;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    18
}
/// $0029DC: bne.b $29c0
pub fn i_0029dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29DE;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29C0; return 10; }
    8
}
/// $0029DE: btst.b #$2, (a6)
pub fn i_0029de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29E2;
    cpu.ir = 0x816;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    12
}
/// $0029E2: bne.b $29c0
pub fn i_0029e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29E4;
    cpu.ir = 0x66DC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29C0; return 10; }
    8
}
/// $0029E4: move.w $16(a6, d4.w), $14(a6)
pub fn i_0029e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29EA;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0029EA: move.b #$ff, d2
pub fn i_0029ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29EE;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0029EE: move.l d5, d1
pub fn i_0029ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29F0;
    cpu.ir = 0x2205;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0029F0: rts
pub fn i_0029f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002AE8: trap #$f
pub fn i_002ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $002AEC: move.w (a0), d1
pub fn i_002aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEE;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $002AEE: cmpi.w #$ffff, d1
pub fn i_002aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $002AF2: beq.w $2b48
pub fn i_002af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B48; return 10; }
    8
}
/// $002AF6: andi.l #$ff, d1
pub fn i_002af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFC;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $002AFC: cmpi.w #$20, d1
pub fn i_002afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B00;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $002B00: bgt.b $2ae8
pub fn i_002b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B02;
    cpu.ir = 0x6EE6;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2AE8; return 10; }
    8
}
/// $002B02: cmpi.w #$0, d1
pub fn i_002b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B06;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $002B06: blt.b $2ae8
pub fn i_002b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B08;
    cpu.ir = 0x6DE0;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2AE8; return 10; }
    8
}
/// $002B08: addi.w #$74, d1
pub fn i_002b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x74, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002B0C: lsl.w #$5, d1
pub fn i_002b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0E;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002B0E: lea.l $1082c8.l, a1
pub fn i_002b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B14;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002B14: move.w $2(a0), d2
pub fn i_002b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B18;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $002B18: cmpi.w #$ffff, d2
pub fn i_002b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1C;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $002B1C: beq.w $2b4a
pub fn i_002b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B20;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B4A; return 10; }
    8
}
/// $002B20: andi.l #$ffff, d2
pub fn i_002b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B26;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $002B26: lsl.l #$6, d2
pub fn i_002b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B28;
    cpu.ir = 0xED8A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[2] = r;
    18
}
/// $002B28: lea.l $14e00.l, a2
pub fn i_002b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x14E00;
    20
}
/// $002B2E: add.l a2, d2
pub fn i_002b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B30;
    cpu.ir = 0xD48A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $002B30: move.l d2, $2(a1, d1.w)
pub fn i_002b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B34;
    cpu.ir = 0x2382;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $002B34: move.w #$1, $6(a1, d1.w)
pub fn i_002b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3A;
    cpu.ir = 0x33BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002B3A: move.b (a0), (a1, d1.w)
pub fn i_002b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3E;
    cpu.ir = 0x1390;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad3, v2 as u8);
    18
}
/// $002B3E: ori.b #$1, (a1, d1.w)
pub fn i_002b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B44;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002B44: addq.l #$4, a0
pub fn i_002b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B46;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $002B46: bra.b $2aec
pub fn i_002b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B48;
    cpu.ir = 0x60A4;
    cpu.pc = 0x2AEC;
    10
}
/// $002B48: rts
pub fn i_002b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002B4A: clr.w $6(a1, d1.w)
pub fn i_002b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4E;
    cpu.ir = 0x4271;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $002B4E: move.b #$40, (a1, d1.w)
pub fn i_002b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B54;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002B54: addq.l #$4, a0
pub fn i_002b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B56;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $002B56: bra.b $2aec
pub fn i_002b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B58;
    cpu.ir = 0x6094;
    cpu.pc = 0x2AEC;
    10
}
// ===== PalSlot_LoadListHi_002b58  ($002B58, 108 B, boot/boot_script_sound_palslots_000400.s)
/// $002B58: move.w (a0), d1
pub fn i_002b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5A;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $002B5A: cmpi.w #$ffff, d1
pub fn i_002b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $002B5E: beq.w $2bb4
pub fn i_002b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BB4; return 10; }
    8
}
/// $002B62: andi.l #$ff, d1
pub fn i_002b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B68;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $002B68: cmpi.w #$ff, d1
pub fn i_002b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Word, false);
    8
}
/// $002B6C: beq.w $2b78
pub fn i_002b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B70;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B78; return 10; }
    8
}
/// $002B70: cmpi.w #$74, d1
pub fn i_002b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B74;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74, Size::Word, false);
    8
}
/// $002B74: bgt.w $2ae8
pub fn i_002b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B78;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2AE8; return 10; }
    8
}
/// $002B78: lsl.w #$5, d1
pub fn i_002b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7A;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002B7A: lea.l $1082c8.l, a1
pub fn i_002b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B80;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002B80: move.w $2(a0), d2
pub fn i_002b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B84;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $002B84: cmpi.w #$ffff, d2
pub fn i_002b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B88;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $002B88: beq.w $2bb6
pub fn i_002b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BB6; return 10; }
    8
}
/// $002B8C: andi.l #$ffff, d2
pub fn i_002b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B92;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $002B92: lsl.l #$6, d2
pub fn i_002b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B94;
    cpu.ir = 0xED8A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[2] = r;
    18
}
/// $002B94: lea.l $1ce00.l, a2
pub fn i_002b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x1CE00;
    20
}
/// $002B9A: add.l a2, d2
pub fn i_002b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9C;
    cpu.ir = 0xD48A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $002B9C: move.l d2, $2(a1, d1.w)
pub fn i_002b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA0;
    cpu.ir = 0x2382;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $002BA0: move.w #$1, $6(a1, d1.w)
pub fn i_002ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA6;
    cpu.ir = 0x33BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002BA6: move.b (a0), (a1, d1.w)
pub fn i_002ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAA;
    cpu.ir = 0x1390;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad3, v2 as u8);
    18
}
/// $002BAA: ori.b #$1, (a1, d1.w)
pub fn i_002baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB0;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002BB0: addq.l #$4, a0
pub fn i_002bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB2;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $002BB2: bra.b $2b58
pub fn i_002bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB4;
    cpu.ir = 0x60A4;
    cpu.pc = 0x2B58;
    10
}
/// $002BB4: rts
pub fn i_002bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002BB6: clr.w $6(a1, d1.w)
pub fn i_002bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBA;
    cpu.ir = 0x4271;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $002BBA: move.b #$40, (a1, d1.w)
pub fn i_002bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC0;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002BC0: addq.l #$4, a0
pub fn i_002bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC2;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $002BC2: bra.b $2b58
pub fn i_002bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC4;
    cpu.ir = 0x6094;
    cpu.pc = 0x2B58;
    10
}
// ===== PalSlot_Release_002bc4  ($002BC4, 98 B, boot/boot_script_sound_palslots_000400.s)
/// $002BC4: movem.l d1-d3/a1-a2, -(a7)
pub fn i_002bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $002BC8: move.w d1, d2
pub fn i_002bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCA;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002BCA: lsl.w #$5, d1
pub fn i_002bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCC;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002BCC: lea.l $1082c8.l, a1
pub fn i_002bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002BD2: subq.w #$1, $6(a1, d1.w)
pub fn i_002bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD6;
    cpu.ir = 0x5371;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $002BD6: bne.w $2c20
pub fn i_002bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2C20; return 10; }
    8
}
/// $002BDA: move.b #$80, (a1, d1.w)
pub fn i_002bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE0;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002BE0: ori.b #$8, (a1, d1.w)
pub fn i_002be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE6;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002BE6: clr.l $2(a1, d1.w)
pub fn i_002be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEA;
    cpu.ir = 0x42B1;
    let ad1 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    18
}
/// $002BEA: clr.w $8(a1, d1.w)
pub fn i_002bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEE;
    cpu.ir = 0x4271;
    let ad1 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $002BEE: move.w $1082c6.l, d1
pub fn i_002bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF4;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x1082C6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $002BF4: lea.l $1081c2.l, a1
pub fn i_002bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081C2;
    20
}
/// $002BFA: subi.w #$94, d2
pub fn i_002bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFE;
    cpu.ir = 0x442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x94, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002BFE: move.b d2, (a1, d1.w)
pub fn i_002bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C02;
    cpu.ir = 0x1382;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002C02: addq.w #$1, $1082c6.l
pub fn i_002c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C08;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x1082C6) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x1082C6, r2 as u16);
    16
}
/// $002C08: cmpi.w #$30, $1082c6.l
pub fn i_002c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C10;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x1082C6) as u32;
    sub_with_flags(cpu, v2, 0x30, Size::Word, false);
    20
}
/// $002C10: blt.w $2c1a
pub fn i_002c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C14;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2C1A; return 10; }
    8
}
/// $002C14: clr.w $1082c6.l
pub fn i_002c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x1082C6) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x1082C6, r2 as u16);
    16
}
/// $002C1A: addq.w #$1, $1082c2.l
pub fn i_002c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C20;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x1082C2) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x1082C2, r2 as u16);
    16
}
/// $002C20: movem.l (a7)+, d1-d3/a1-a2
pub fn i_002c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C24;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $002C24: rts
pub fn i_002c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_SetupSlotFromTableA  ($002C26, 64 B, video/sprite_setup_slot_from_table_002c26.s)
/// $002C26: lea.l $1ce00.l, a2
pub fn i_002c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x1CE00;
    20
}
/// $002C2C: bra.w $2c36
pub fn i_002c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C30;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C36;
    10
}
/// $002C30: lea.l $14e00.l, a2
pub fn i_002c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C36;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x14E00;
    20
}
/// $002C36: andi.l #$ffff, d2
pub fn i_002c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3C;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $002C3C: lsl.l #$6, d2
pub fn i_002c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3E;
    cpu.ir = 0xED8A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[2] = r;
    18
}
/// $002C3E: add.l a2, d2
pub fn i_002c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C40;
    cpu.ir = 0xD48A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $002C40: lsl.w #$5, d1
pub fn i_002c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C42;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002C42: lea.l $1082c8.l, a1
pub fn i_002c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C48;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002C48: move.l d2, $a(a1, d1.w)
pub fn i_002c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4C;
    cpu.ir = 0x2382;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0xA).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $002C4C: move.w d3, $8(a1, d1.w)
pub fn i_002c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C50;
    cpu.ir = 0x3383;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002C50: move.w d3, $10(a1, d1.w)
pub fn i_002c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C54;
    cpu.ir = 0x3383;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x10).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002C54: move.b #$0, $e(a1, d1.w)
pub fn i_002c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5A;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xE).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002C5A: move.b d4, $f(a1, d1.w)
pub fn i_002c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5E;
    cpu.ir = 0x1384;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xF).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002C5E: ori.b #$2, (a1, d1.w)
pub fn i_002c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C64;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002C64: rts
pub fn i_002c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalSlot_SetBit3_002c66  ($002C66, 16 B, boot/boot_script_sound_palslots_000400.s)
/// $002C66: lea.l $1082c8.l, a1
pub fn i_002c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002C6C: lsl.w #$5, d1
pub fn i_002c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6E;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002C6E: ori.b #$8, (a1, d1.w)
pub fn i_002c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C74;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002C74: rts
pub fn i_002c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalSlot_UpdateAll_002c86  ($002C86, 682 B, boot/boot_script_sound_palslots_000400.s)
/// $002C86: cmpi.b #$ff, $10a2d2.l
pub fn i_002c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2D2) as u32;
    sub_with_flags(cpu, v2, 0xFF, Size::Byte, false);
    20
}
/// $002C8E: beq.w $2c9e
pub fn i_002c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C9E; return 10; }
    8
}
/// $002C92: lea.l $1387e.l, a0
pub fn i_002c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1387E;
    20
}
/// $002C98: jsr $1f4a.l
pub fn i_002c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C9E);
    cpu.pc = 0x1F4A;
    16
}
/// $002C9E: moveq #$0, d4
pub fn i_002c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA0;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002CA0: clr.b $10a2cd.l
pub fn i_002ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10A2CD) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10A2CD, r2 as u8);
    16
}
/// $002CA6: clr.b $10a2ce.l
pub fn i_002ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAC;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10A2CE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10A2CE, r2 as u8);
    16
}
/// $002CAC: move.b $10a2c8.l, d0
pub fn i_002cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2C8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $002CB2: bne.w $2d88
pub fn i_002cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D88; return 10; }
    8
}
/// $002CB6: move.b $10a2c9.l, d0
pub fn i_002cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2C9) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $002CBC: beq.w $2ce0
pub fn i_002cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CE0; return 10; }
    8
}
/// $002CC0: cmpi.b #$80, d0
pub fn i_002cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $002CC4: beq.w $2ce0
pub fn i_002cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CE0; return 10; }
    8
}
/// $002CC8: moveq #$0, d1
pub fn i_002cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCA;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002CCA: move.b d1, $10a2ca.l
pub fn i_002cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD0;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $002CD0: move.b d1, $10a2cb.l
pub fn i_002cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD6;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $002CD6: move.b d1, $10a2cc.l
pub fn i_002cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDC;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $002CDC: bra.w $2d88
pub fn i_002cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D88;
    10
}
/// $002CE0: lea.l $1082c8.l, a1
pub fn i_002ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002CE6: moveq #$0, d1
pub fn i_002ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE8;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002CE8: move.w #$c3, d0
pub fn i_002ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002CEC: btst.b #$7, (a1, d1.w)
pub fn i_002cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF2;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002CF2: bne.w $2d52
pub fn i_002cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D52; return 10; }
    8
}
/// $002CF6: btst.b #$1, (a1, d1.w)
pub fn i_002cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFC;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002CFC: beq.w $2d0e
pub fn i_002cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D0E; return 10; }
    8
}
/// $002D00: jsr $13408.l
pub fn i_002d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D06);
    cpu.pc = 0x13408;
    16
}
/// $002D06: move.b #$ff, $10a2ce.l
pub fn i_002d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002D0E: btst.b #$0, (a1, d1.w)
pub fn i_002d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D14;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002D14: beq.w $2d32
pub fn i_002d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D18;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D32; return 10; }
    8
}
/// $002D18: movea.l $2(a1, d1.w), a2
pub fn i_002d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1C;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $002D1C: lea.l $10a2d4.l, a3
pub fn i_002d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D22;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10A2D4;
    20
}
/// $002D22: adda.l d1, a3
pub fn i_002d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D24;
    cpu.ir = 0xD7C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $002D24: jsr $133b0.l
pub fn i_002d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D2A);
    cpu.pc = 0x133B0;
    16
}
/// $002D2A: move.b #$ff, $10a2ce.l
pub fn i_002d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D32;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002D32: btst.b #$6, (a1, d1.w)
pub fn i_002d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D38;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002D38: beq.w $2d52
pub fn i_002d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D52; return 10; }
    8
}
/// $002D3C: lea.l $10a2d4.l, a3
pub fn i_002d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D42;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10A2D4;
    20
}
/// $002D42: adda.l d1, a3
pub fn i_002d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D44;
    cpu.ir = 0xD7C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $002D44: jsr $133e6.l
pub fn i_002d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D4A);
    cpu.pc = 0x133E6;
    16
}
/// $002D4A: move.b #$ff, $10a2ce.l
pub fn i_002d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D52;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002D52: addi.w #$20, d1
pub fn i_002d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D56;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002D56: dbra d0, $2cec
pub fn i_002d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5A;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2D5A; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2D5A; return 14; }
    cpu.pc = 0x2CEC;
    10
}
/// $002D5A: tst.b $10a2cd.l
pub fn i_002d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D60;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002D60: bne.w $2d7c
pub fn i_002d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D7C; return 10; }
    8
}
/// $002D64: move.b #$ff, $10a2cd.l
pub fn i_002d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CD, v1 as u8);
    16
}
/// $002D6C: lea.l $1082c8.l, a1
pub fn i_002d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D72;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002D72: move.w #$1fe0, d1
pub fn i_002d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D76;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002D76: clr.w d0
pub fn i_002d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D78;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002D78: bra.w $2cec
pub fn i_002d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2CEC;
    10
}
/// $002D7C: move.b $10a2c8.l, $10a2c9.l
pub fn i_002d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D86;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10A2C8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C9, v2 as u8);
    28
}
/// $002D86: rts
pub fn i_002d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002D88: cmpi.b #$ff, d0
pub fn i_002d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $002D8C: beq.w $2e62
pub fn i_002d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E62; return 10; }
    8
}
/// $002D90: lea.l $1082c8.l, a1
pub fn i_002d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002D96: moveq #$0, d1
pub fn i_002d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D98;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002D98: move.w #$c3, d0
pub fn i_002d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002D9C: btst.b #$7, (a1, d1.w)
pub fn i_002d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA2;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DA2: bne.w $2e30
pub fn i_002da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2E30; return 10; }
    8
}
/// $002DA6: btst.b #$1, (a1, d1.w)
pub fn i_002da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAC;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DAC: beq.w $2db6
pub fn i_002dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DB6; return 10; }
    8
}
/// $002DB0: jsr $13408.l
pub fn i_002db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DB6);
    cpu.pc = 0x13408;
    16
}
/// $002DB6: btst.b #$1, (a1, d1.w)
pub fn i_002db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBC;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DBC: bne.w $2e30
pub fn i_002dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2E30; return 10; }
    8
}
/// $002DC0: movea.l $2(a1, d1.w), a2
pub fn i_002dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC4;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $002DC4: lea.l $10a2d4.l, a3
pub fn i_002dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCA;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10A2D4;
    20
}
/// $002DCA: adda.l d1, a3
pub fn i_002dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCC;
    cpu.ir = 0xD7C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $002DCC: btst.b #$6, (a1, d1.w)
pub fn i_002dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD2;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DD2: beq.w $2de0
pub fn i_002dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DE0; return 10; }
    8
}
/// $002DD6: jsr $133e6.l
pub fn i_002dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DDC);
    cpu.pc = 0x133E6;
    16
}
/// $002DDC: bra.w $2e30
pub fn i_002ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E30;
    10
}
/// $002DE0: btst.b #$3, (a1, d1.w)
pub fn i_002de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE6;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DE6: beq.w $2dfe
pub fn i_002de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DFE; return 10; }
    8
}
/// $002DEA: btst.b #$0, (a1, d1.w)
pub fn i_002dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF0;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DF0: beq.w $2e30
pub fn i_002df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E30; return 10; }
    8
}
/// $002DF4: jsr $133b0.l
pub fn i_002df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DFA);
    cpu.pc = 0x133B0;
    16
}
/// $002DFA: bra.w $2e30
pub fn i_002dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E30;
    10
}
/// $002DFE: movea.l a3, a4
pub fn i_002dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E00;
    cpu.ir = 0x284B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $002E00: addq.w #$2, a3
pub fn i_002e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E02;
    cpu.ir = 0x544B;
    cpu.a[3] = cpu.a[3].wrapping_add(2);
    8
}
/// $002E02: addq.l #$4, a2
pub fn i_002e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E04;
    cpu.ir = 0x588A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $002E04: move.w #$e, d5
pub fn i_002e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E08;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002E08: move.b (a2)+, d2
pub fn i_002e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0A;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002E0A: move.b (a2)+, d3
pub fn i_002e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0C;
    cpu.ir = 0x161A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002E0C: move.b (a2)+, d4
pub fn i_002e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0E;
    cpu.ir = 0x181A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002E0E: addq.l #$1, a2
pub fn i_002e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E10;
    cpu.ir = 0x528A;
    cpu.a[2] = cpu.a[2].wrapping_add(1);
    8
}
/// $002E10: jsr $13624.l
pub fn i_002e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E16);
    cpu.pc = 0x13624;
    16
}
/// $002E16: lea.l $2f30(pc), a0
pub fn i_002e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2F30;
    16
}
/// $002E1A: add.w d4, d4
pub fn i_002e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1C;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $002E1C: adda.l d4, a0
pub fn i_002e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1E;
    cpu.ir = 0xD1C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $002E1E: move.w (a0), d4
pub fn i_002e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E20;
    cpu.ir = 0x3810;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $002E20: move.w d4, (a3)+
pub fn i_002e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E22;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $002E22: dbra d5, $2e08
pub fn i_002e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E26;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2E26; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2E26; return 14; }
    cpu.pc = 0x2E08;
    10
}
/// $002E26: andi.b #$fe, (a1, d1.w)
pub fn i_002e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2C;
    cpu.ir = 0x231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002E2C: move.w #$1, (a4)
pub fn i_002e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E30;
    cpu.ir = 0x38BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $002E30: addi.w #$20, d1
pub fn i_002e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E34;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002E34: dbra d0, $2d9c
pub fn i_002e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E38;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2E38; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2E38; return 14; }
    cpu.pc = 0x2D9C;
    10
}
/// $002E38: move.b #$ff, $10a2ce.l
pub fn i_002e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E40;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002E40: tst.b $10a2cd.l
pub fn i_002e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E46;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002E46: bne.w $2d7c
pub fn i_002e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D7C; return 10; }
    8
}
/// $002E4A: move.b #$ff, $10a2cd.l
pub fn i_002e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E52;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CD, v1 as u8);
    16
}
/// $002E52: lea.l $1082c8.l, a1
pub fn i_002e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E58;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002E58: move.w #$1fe0, d1
pub fn i_002e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E5C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002E5C: clr.w d0
pub fn i_002e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E5E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002E5E: bra.w $2d9c
pub fn i_002e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E62;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D9C;
    10
}
/// $002E62: lea.l $1082c8.l, a1
pub fn i_002e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002E68: moveq #$0, d1
pub fn i_002e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6A;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002E6A: move.w #$c3, d0
pub fn i_002e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002E6E: btst.b #$7, (a1, d1.w)
pub fn i_002e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E74;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002E74: bne.w $2efe
pub fn i_002e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2EFE; return 10; }
    8
}
/// $002E78: btst.b #$1, (a1, d1.w)
pub fn i_002e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7E;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002E7E: beq.w $2e88
pub fn i_002e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E88; return 10; }
    8
}
/// $002E82: jsr $13408.l
pub fn i_002e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E88);
    cpu.pc = 0x13408;
    16
}
/// $002E88: btst.b #$1, (a1, d1.w)
pub fn i_002e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8E;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002E8E: bne.w $2efe
pub fn i_002e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2EFE; return 10; }
    8
}
/// $002E92: movea.l $2(a1, d1.w), a2
pub fn i_002e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E96;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $002E96: lea.l $10a2d4.l, a3
pub fn i_002e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9C;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10A2D4;
    20
}
/// $002E9C: adda.l d1, a3
pub fn i_002e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9E;
    cpu.ir = 0xD7C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $002E9E: btst.b #$6, (a1, d1.w)
pub fn i_002e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA4;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002EA4: beq.w $2eb2
pub fn i_002ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EB2; return 10; }
    8
}
/// $002EA8: jsr $133e6.l
pub fn i_002ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EAE);
    cpu.pc = 0x133E6;
    16
}
/// $002EAE: bra.w $2efe
pub fn i_002eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2EFE;
    10
}
/// $002EB2: btst.b #$3, (a1, d1.w)
pub fn i_002eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB8;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002EB8: beq.w $2ed0
pub fn i_002eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2ED0; return 10; }
    8
}
/// $002EBC: btst.b #$0, (a1, d1.w)
pub fn i_002ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC2;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002EC2: beq.w $2efe
pub fn i_002ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EFE; return 10; }
    8
}
/// $002EC6: jsr $133b0.l
pub fn i_002ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ECC);
    cpu.pc = 0x133B0;
    16
}
/// $002ECC: bra.w $2efe
pub fn i_002ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2EFE;
    10
}
/// $002ED0: movea.l a3, a4
pub fn i_002ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED2;
    cpu.ir = 0x284B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $002ED2: addq.w #$2, a3
pub fn i_002ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED4;
    cpu.ir = 0x544B;
    cpu.a[3] = cpu.a[3].wrapping_add(2);
    8
}
/// $002ED4: addq.l #$4, a2
pub fn i_002ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED6;
    cpu.ir = 0x588A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $002ED6: move.w #$e, d5
pub fn i_002ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDA;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002EDA: move.b (a2)+, d2
pub fn i_002eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDC;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002EDC: move.b (a2)+, d3
pub fn i_002edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDE;
    cpu.ir = 0x161A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002EDE: move.b (a2)+, d4
pub fn i_002ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE0;
    cpu.ir = 0x181A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002EE0: addq.l #$1, a2
pub fn i_002ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE2;
    cpu.ir = 0x528A;
    cpu.a[2] = cpu.a[2].wrapping_add(1);
    8
}
/// $002EE2: jsr $13694.l
pub fn i_002ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EE8);
    cpu.pc = 0x13694;
    16
}
/// $002EE8: add.w d4, d4
pub fn i_002ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEA;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $002EEA: move.w $2f30(pc, d4.l), d4
pub fn i_002eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEE;
    cpu.ir = 0x383B;
    let ad2 = 0x2F30_u32.wrapping_add(cpu.d[4]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $002EEE: move.w d4, (a3)+
pub fn i_002eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF0;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $002EF0: dbra d5, $2eda
pub fn i_002ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF4;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2EF4; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2EF4; return 14; }
    cpu.pc = 0x2EDA;
    10
}
/// $002EF4: andi.b #$fe, (a1, d1.w)
pub fn i_002ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFA;
    cpu.ir = 0x231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002EFA: move.w #$1, (a4)
pub fn i_002efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFE;
    cpu.ir = 0x38BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $002EFE: addi.w #$20, d1
pub fn i_002efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F02;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002F02: dbra d0, $2e6e
pub fn i_002f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F06;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2F06; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2F06; return 14; }
    cpu.pc = 0x2E6E;
    10
}
/// $002F06: move.b #$ff, $10a2ce.l
pub fn i_002f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002F0E: tst.b $10a2cd.l
pub fn i_002f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F14;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002F14: bne.w $2d7c
pub fn i_002f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F18;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D7C; return 10; }
    8
}
/// $002F18: move.b #$ff, $10a2cd.l
pub fn i_002f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F20;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CD, v1 as u8);
    16
}
/// $002F20: lea.l $1082c8.l, a1
pub fn i_002f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F26;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002F26: move.w #$1fe0, d1
pub fn i_002f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002F2A: clr.w d0
pub fn i_002f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2C;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002F2C: bra.w $2e6e
pub fn i_002f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F30;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E6E;
    10
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0000C0, i_0000c0),
    (0x0000C8, i_0000c8),
    (0x000122, i_000122),
    (0x000128, i_000128),
    (0x00012E, i_00012e),
    (0x000134, i_000134),
    (0x000400, i_000400),
    (0x000410, i_000410),
    (0x000416, i_000416),
    (0x00041C, i_00041c),
    (0x000420, i_000420),
    (0x000422, i_000422),
    (0x000426, i_000426),
    (0x00042C, i_00042c),
    (0x000432, i_000432),
    (0x000436, i_000436),
    (0x00043C, i_00043c),
    (0x000440, i_000440),
    (0x000442, i_000442),
    (0x000444, i_000444),
    (0x00044C, i_00044c),
    (0x000452, i_000452),
    (0x000458, i_000458),
    (0x00045A, i_00045a),
    (0x00045C, i_00045c),
    (0x000460, i_000460),
    (0x000466, i_000466),
    (0x00046C, i_00046c),
    (0x000470, i_000470),
    (0x000476, i_000476),
    (0x00047C, i_00047c),
    (0x000482, i_000482),
    (0x000486, i_000486),
    (0x00048A, i_00048a),
    (0x00048C, i_00048c),
    (0x000490, i_000490),
    (0x000494, i_000494),
    (0x000496, i_000496),
    (0x00049C, i_00049c),
    (0x0004A2, i_0004a2),
    (0x0004A4, i_0004a4),
    (0x0004AC, i_0004ac),
    (0x0004AE, i_0004ae),
    (0x0004B4, i_0004b4),
    (0x0004BA, i_0004ba),
    (0x0004BE, i_0004be),
    (0x0004C6, i_0004c6),
    (0x0004CA, i_0004ca),
    (0x0004D0, i_0004d0),
    (0x0004D6, i_0004d6),
    (0x0004DA, i_0004da),
    (0x0004DC, i_0004dc),
    (0x0004E0, i_0004e0),
    (0x0004E4, i_0004e4),
    (0x0004E6, i_0004e6),
    (0x0004EA, i_0004ea),
    (0x0004EE, i_0004ee),
    (0x0004F2, i_0004f2),
    (0x0004F6, i_0004f6),
    (0x0004FA, i_0004fa),
    (0x0004FE, i_0004fe),
    (0x000504, i_000504),
    (0x000506, i_000506),
    (0x00050C, i_00050c),
    (0x00050E, i_00050e),
    (0x000510, i_000510),
    (0x000516, i_000516),
    (0x000518, i_000518),
    (0x00051E, i_00051e),
    (0x000524, i_000524),
    (0x000528, i_000528),
    (0x00052A, i_00052a),
    (0x00052E, i_00052e),
    (0x000532, i_000532),
    (0x000536, i_000536),
    (0x00053A, i_00053a),
    (0x00053E, i_00053e),
    (0x000542, i_000542),
    (0x000546, i_000546),
    (0x00054A, i_00054a),
    (0x00054E, i_00054e),
    (0x000552, i_000552),
    (0x000558, i_000558),
    (0x00055C, i_00055c),
    (0x000560, i_000560),
    (0x000564, i_000564),
    (0x000568, i_000568),
    (0x00056E, i_00056e),
    (0x000576, i_000576),
    (0x00057E, i_00057e),
    (0x000584, i_000584),
    (0x000586, i_000586),
    (0x000588, i_000588),
    (0x00058E, i_00058e),
    (0x000592, i_000592),
    (0x000596, i_000596),
    (0x00059A, i_00059a),
    (0x00059E, i_00059e),
    (0x0005A4, i_0005a4),
    (0x0005AC, i_0005ac),
    (0x0005B2, i_0005b2),
    (0x0005B4, i_0005b4),
    (0x0005B6, i_0005b6),
    (0x0005BA, i_0005ba),
    (0x0005BE, i_0005be),
    (0x0005C2, i_0005c2),
    (0x0005C6, i_0005c6),
    (0x0005CA, i_0005ca),
    (0x0005CC, i_0005cc),
    (0x0005D0, i_0005d0),
    (0x0005D2, i_0005d2),
    (0x0005D4, i_0005d4),
    (0x0005D8, i_0005d8),
    (0x0005FE, i_0005fe),
    (0x000602, i_000602),
    (0x000606, i_000606),
    (0x00060C, i_00060c),
    (0x000612, i_000612),
    (0x000616, i_000616),
    (0x00061A, i_00061a),
    (0x00061E, i_00061e),
    (0x000624, i_000624),
    (0x000626, i_000626),
    (0x00062C, i_00062c),
    (0x000630, i_000630),
    (0x000636, i_000636),
    (0x00063A, i_00063a),
    (0x00063E, i_00063e),
    (0x000640, i_000640),
    (0x000644, i_000644),
    (0x000648, i_000648),
    (0x00064A, i_00064a),
    (0x00064C, i_00064c),
    (0x000650, i_000650),
    (0x000654, i_000654),
    (0x000656, i_000656),
    (0x00065C, i_00065c),
    (0x000662, i_000662),
    (0x000668, i_000668),
    (0x00066E, i_00066e),
    (0x000672, i_000672),
    (0x000676, i_000676),
    (0x00067A, i_00067a),
    (0x00067C, i_00067c),
    (0x00067E, i_00067e),
    (0x000680, i_000680),
    (0x000684, i_000684),
    (0x00068A, i_00068a),
    (0x00068E, i_00068e),
    (0x000694, i_000694),
    (0x000698, i_000698),
    (0x00069C, i_00069c),
    (0x0006A0, i_0006a0),
    (0x0006A2, i_0006a2),
    (0x0006A4, i_0006a4),
    (0x0006A6, i_0006a6),
    (0x0006AA, i_0006aa),
    (0x0006AE, i_0006ae),
    (0x0006B2, i_0006b2),
    (0x0006B6, i_0006b6),
    (0x0006B8, i_0006b8),
    (0x0006BA, i_0006ba),
    (0x0006BC, i_0006bc),
    (0x0006C0, i_0006c0),
    (0x0006C2, i_0006c2),
    (0x0006C8, i_0006c8),
    (0x0006CA, i_0006ca),
    (0x0006D0, i_0006d0),
    (0x0006D6, i_0006d6),
    (0x0006DC, i_0006dc),
    (0x0006E2, i_0006e2),
    (0x0006E8, i_0006e8),
    (0x0006EC, i_0006ec),
    (0x0006FE, i_0006fe),
    (0x000704, i_000704),
    (0x00070A, i_00070a),
    (0x00070E, i_00070e),
    (0x000716, i_000716),
    (0x00071A, i_00071a),
    (0x000720, i_000720),
    (0x000726, i_000726),
    (0x00072A, i_00072a),
    (0x00072C, i_00072c),
    (0x000730, i_000730),
    (0x000734, i_000734),
    (0x000736, i_000736),
    (0x00073A, i_00073a),
    (0x00073C, i_00073c),
    (0x000740, i_000740),
    (0x000744, i_000744),
    (0x000746, i_000746),
    (0x00074A, i_00074a),
    (0x00074E, i_00074e),
    (0x000752, i_000752),
    (0x000756, i_000756),
    (0x00075A, i_00075a),
    (0x00075E, i_00075e),
    (0x000762, i_000762),
    (0x000768, i_000768),
    (0x00076A, i_00076a),
    (0x000770, i_000770),
    (0x000772, i_000772),
    (0x000774, i_000774),
    (0x00077A, i_00077a),
    (0x00077E, i_00077e),
    (0x000784, i_000784),
    (0x000786, i_000786),
    (0x000788, i_000788),
    (0x00078C, i_00078c),
    (0x00078E, i_00078e),
    (0x000790, i_000790),
    (0x000792, i_000792),
    (0x000794, i_000794),
    (0x000796, i_000796),
    (0x00079A, i_00079a),
    (0x0007A0, i_0007a0),
    (0x0007A4, i_0007a4),
    (0x0007AA, i_0007aa),
    (0x0007AE, i_0007ae),
    (0x0007B0, i_0007b0),
    (0x0007B2, i_0007b2),
    (0x0007B6, i_0007b6),
    (0x0007BC, i_0007bc),
    (0x0007C0, i_0007c0),
    (0x0007C4, i_0007c4),
    (0x0007C6, i_0007c6),
    (0x0007CA, i_0007ca),
    (0x0007CC, i_0007cc),
    (0x0007D0, i_0007d0),
    (0x0007D6, i_0007d6),
    (0x0007DE, i_0007de),
    (0x0007E6, i_0007e6),
    (0x0007EA, i_0007ea),
    (0x0007EC, i_0007ec),
    (0x0007F2, i_0007f2),
    (0x0007F4, i_0007f4),
    (0x0007F6, i_0007f6),
    (0x0007FA, i_0007fa),
    (0x00080C, i_00080c),
    (0x000812, i_000812),
    (0x000818, i_000818),
    (0x00081C, i_00081c),
    (0x000822, i_000822),
    (0x000826, i_000826),
    (0x00082E, i_00082e),
    (0x000836, i_000836),
    (0x00083A, i_00083a),
    (0x000840, i_000840),
    (0x000844, i_000844),
    (0x00084A, i_00084a),
    (0x000850, i_000850),
    (0x000852, i_000852),
    (0x000858, i_000858),
    (0x00085C, i_00085c),
    (0x00085E, i_00085e),
    (0x000862, i_000862),
    (0x000868, i_000868),
    (0x000870, i_000870),
    (0x000878, i_000878),
    (0x00087E, i_00087e),
    (0x000886, i_000886),
    (0x00088C, i_00088c),
    (0x00088E, i_00088e),
    (0x000892, i_000892),
    (0x000898, i_000898),
    (0x00089C, i_00089c),
    (0x00089E, i_00089e),
    (0x0008A2, i_0008a2),
    (0x0008A4, i_0008a4),
    (0x0008A8, i_0008a8),
    (0x0008AC, i_0008ac),
    (0x0008AE, i_0008ae),
    (0x0008B2, i_0008b2),
    (0x0008B6, i_0008b6),
    (0x0008B8, i_0008b8),
    (0x0008BC, i_0008bc),
    (0x0008C0, i_0008c0),
    (0x0008C2, i_0008c2),
    (0x0008C6, i_0008c6),
    (0x0008CA, i_0008ca),
    (0x0008CC, i_0008cc),
    (0x0008D0, i_0008d0),
    (0x0008D4, i_0008d4),
    (0x0008D6, i_0008d6),
    (0x0008DE, i_0008de),
    (0x0008E2, i_0008e2),
    (0x0008E8, i_0008e8),
    (0x0008EA, i_0008ea),
    (0x0008EE, i_0008ee),
    (0x0008F0, i_0008f0),
    (0x0008F4, i_0008f4),
    (0x0008F6, i_0008f6),
    (0x0008FC, i_0008fc),
    (0x000900, i_000900),
    (0x000906, i_000906),
    (0x00090A, i_00090a),
    (0x000910, i_000910),
    (0x000914, i_000914),
    (0x00091A, i_00091a),
    (0x00091E, i_00091e),
    (0x000924, i_000924),
    (0x000928, i_000928),
    (0x00092E, i_00092e),
    (0x000934, i_000934),
    (0x000938, i_000938),
    (0x000940, i_000940),
    (0x000946, i_000946),
    (0x000950, i_000950),
    (0x00095A, i_00095a),
    (0x00095E, i_00095e),
    (0x000960, i_000960),
    (0x000962, i_000962),
    (0x000968, i_000968),
    (0x00096C, i_00096c),
    (0x000970, i_000970),
    (0x000972, i_000972),
    (0x000978, i_000978),
    (0x00097A, i_00097a),
    (0x000980, i_000980),
    (0x000984, i_000984),
    (0x00098A, i_00098a),
    (0x000990, i_000990),
    (0x000996, i_000996),
    (0x00099C, i_00099c),
    (0x0009A0, i_0009a0),
    (0x0009A6, i_0009a6),
    (0x0009A8, i_0009a8),
    (0x0009AC, i_0009ac),
    (0x0009B2, i_0009b2),
    (0x000A7C, i_000a7c),
    (0x000A82, i_000a82),
    (0x000A88, i_000a88),
    (0x000A8E, i_000a8e),
    (0x000A94, i_000a94),
    (0x000A9A, i_000a9a),
    (0x000AA0, i_000aa0),
    (0x000AA6, i_000aa6),
    (0x000AAC, i_000aac),
    (0x000AB2, i_000ab2),
    (0x000AB8, i_000ab8),
    (0x000ABE, i_000abe),
    (0x000AC4, i_000ac4),
    (0x000ACA, i_000aca),
    (0x000AD0, i_000ad0),
    (0x000AD6, i_000ad6),
    (0x000ADC, i_000adc),
    (0x000AE2, i_000ae2),
    (0x000AE8, i_000ae8),
    (0x000AEE, i_000aee),
    (0x000AF4, i_000af4),
    (0x000AFA, i_000afa),
    (0x000B00, i_000b00),
    (0x000B06, i_000b06),
    (0x000B0C, i_000b0c),
    (0x000B12, i_000b12),
    (0x000B18, i_000b18),
    (0x000B1E, i_000b1e),
    (0x000B24, i_000b24),
    (0x000B2A, i_000b2a),
    (0x000B30, i_000b30),
    (0x000B36, i_000b36),
    (0x000B3C, i_000b3c),
    (0x000B42, i_000b42),
    (0x000B48, i_000b48),
    (0x000B4C, i_000b4c),
    (0x000B52, i_000b52),
    (0x000B58, i_000b58),
    (0x000B5E, i_000b5e),
    (0x000B64, i_000b64),
    (0x000B6A, i_000b6a),
    (0x000B70, i_000b70),
    (0x000B76, i_000b76),
    (0x000B80, i_000b80),
    (0x000B8A, i_000b8a),
    (0x000B8E, i_000b8e),
    (0x000B90, i_000b90),
    (0x000E8E, i_000e8e),
    (0x000E94, i_000e94),
    (0x000E9C, i_000e9c),
    (0x000EA4, i_000ea4),
    (0x000EA8, i_000ea8),
    (0x000EAE, i_000eae),
    (0x000EB2, i_000eb2),
    (0x000EB6, i_000eb6),
    (0x000EBA, i_000eba),
    (0x000EBE, i_000ebe),
    (0x000EC4, i_000ec4),
    (0x000EC8, i_000ec8),
    (0x000ECC, i_000ecc),
    (0x000ED2, i_000ed2),
    (0x000ED8, i_000ed8),
    (0x000EDE, i_000ede),
    (0x000EE4, i_000ee4),
    (0x000EE8, i_000ee8),
    (0x000EEC, i_000eec),
    (0x000EF0, i_000ef0),
    (0x000EF2, i_000ef2),
    (0x000EF8, i_000ef8),
    (0x000EFE, i_000efe),
    (0x000F02, i_000f02),
    (0x000F06, i_000f06),
    (0x000F0A, i_000f0a),
    (0x000F0E, i_000f0e),
    (0x000F12, i_000f12),
    (0x000F16, i_000f16),
    (0x000F36, i_000f36),
    (0x000F3E, i_000f3e),
    (0x000F46, i_000f46),
    (0x000F4A, i_000f4a),
    (0x000F4E, i_000f4e),
    (0x000F52, i_000f52),
    (0x000F5A, i_000f5a),
    (0x000F62, i_000f62),
    (0x000F6A, i_000f6a),
    (0x000F6E, i_000f6e),
    (0x000F72, i_000f72),
    (0x000F98, i_000f98),
    (0x000F9E, i_000f9e),
    (0x000FA0, i_000fa0),
    (0x000FA6, i_000fa6),
    (0x000FAC, i_000fac),
    (0x000FB0, i_000fb0),
    (0x000FB6, i_000fb6),
    (0x000FBA, i_000fba),
    (0x000FC0, i_000fc0),
    (0x000FC4, i_000fc4),
    (0x000FC6, i_000fc6),
    (0x000FCA, i_000fca),
    (0x000FD0, i_000fd0),
    (0x000FD4, i_000fd4),
    (0x000FD8, i_000fd8),
    (0x000FDC, i_000fdc),
    (0x000FDE, i_000fde),
    (0x000FE0, i_000fe0),
    (0x000FE4, i_000fe4),
    (0x000FEA, i_000fea),
    (0x000FF0, i_000ff0),
    (0x000FF4, i_000ff4),
    (0x000FF8, i_000ff8),
    (0x000FFC, i_000ffc),
    (0x000FFE, i_000ffe),
    (0x001004, i_001004),
    (0x001008, i_001008),
    (0x00100C, i_00100c),
    (0x001010, i_001010),
    (0x001012, i_001012),
    (0x001018, i_001018),
    (0x00101E, i_00101e),
    (0x001020, i_001020),
    (0x001024, i_001024),
    (0x00109C, i_00109c),
    (0x00109E, i_00109e),
    (0x0010A4, i_0010a4),
    (0x0010AA, i_0010aa),
    (0x0010B0, i_0010b0),
    (0x0010B6, i_0010b6),
    (0x0010BC, i_0010bc),
    (0x0010C2, i_0010c2),
    (0x0010CA, i_0010ca),
    (0x0010D2, i_0010d2),
    (0x0010D6, i_0010d6),
    (0x0010DC, i_0010dc),
    (0x0010E4, i_0010e4),
    (0x0010E8, i_0010e8),
    (0x0010EC, i_0010ec),
    (0x0010F0, i_0010f0),
    (0x0011EA, i_0011ea),
    (0x0011F0, i_0011f0),
    (0x0011F6, i_0011f6),
    (0x0011FA, i_0011fa),
    (0x001200, i_001200),
    (0x001206, i_001206),
    (0x00120C, i_00120c),
    (0x001214, i_001214),
    (0x00121C, i_00121c),
    (0x001222, i_001222),
    (0x00122A, i_00122a),
    (0x00122E, i_00122e),
    (0x001232, i_001232),
    (0x001238, i_001238),
    (0x00123C, i_00123c),
    (0x001242, i_001242),
    (0x001246, i_001246),
    (0x00124C, i_00124c),
    (0x001250, i_001250),
    (0x001254, i_001254),
    (0x00125A, i_00125a),
    (0x00125E, i_00125e),
    (0x0012F4, i_0012f4),
    (0x0012FA, i_0012fa),
    (0x001300, i_001300),
    (0x001306, i_001306),
    (0x00130C, i_00130c),
    (0x001314, i_001314),
    (0x00131C, i_00131c),
    (0x001320, i_001320),
    (0x001326, i_001326),
    (0x00132E, i_00132e),
    (0x001332, i_001332),
    (0x001336, i_001336),
    (0x00133C, i_00133c),
    (0x001340, i_001340),
    (0x001344, i_001344),
    (0x00134A, i_00134a),
    (0x00134E, i_00134e),
    (0x001352, i_001352),
    (0x001354, i_001354),
    (0x00135A, i_00135a),
    (0x00135E, i_00135e),
    (0x001364, i_001364),
    (0x001368, i_001368),
    (0x00136E, i_00136e),
    (0x001374, i_001374),
    (0x00137A, i_00137a),
    (0x001382, i_001382),
    (0x00138A, i_00138a),
    (0x001390, i_001390),
    (0x001396, i_001396),
    (0x00139A, i_00139a),
    (0x00139C, i_00139c),
    (0x0013A0, i_0013a0),
    (0x0013A6, i_0013a6),
    (0x0013AC, i_0013ac),
    (0x0013B2, i_0013b2),
    (0x0013BA, i_0013ba),
    (0x0013C2, i_0013c2),
    (0x0013C6, i_0013c6),
    (0x0013CA, i_0013ca),
    (0x0013D0, i_0013d0),
    (0x0013D4, i_0013d4),
    (0x0013DA, i_0013da),
    (0x0013DE, i_0013de),
    (0x0013E6, i_0013e6),
    (0x0013EA, i_0013ea),
    (0x0013F0, i_0013f0),
    (0x0013F4, i_0013f4),
    (0x0013FC, i_0013fc),
    (0x001400, i_001400),
    (0x001406, i_001406),
    (0x00140A, i_00140a),
    (0x00140C, i_00140c),
    (0x001410, i_001410),
    (0x001416, i_001416),
    (0x00141C, i_00141c),
    (0x001422, i_001422),
    (0x001428, i_001428),
    (0x00142E, i_00142e),
    (0x001434, i_001434),
    (0x001438, i_001438),
    (0x00143C, i_00143c),
    (0x001442, i_001442),
    (0x001446, i_001446),
    (0x00144C, i_00144c),
    (0x001450, i_001450),
    (0x001452, i_001452),
    (0x001458, i_001458),
    (0x00145A, i_00145a),
    (0x001460, i_001460),
    (0x001466, i_001466),
    (0x00146C, i_00146c),
    (0x001472, i_001472),
    (0x001476, i_001476),
    (0x00147A, i_00147a),
    (0x001482, i_001482),
    (0x001488, i_001488),
    (0x00148C, i_00148c),
    (0x001490, i_001490),
    (0x001492, i_001492),
    (0x001498, i_001498),
    (0x00149C, i_00149c),
    (0x0014A0, i_0014a0),
    (0x0014A2, i_0014a2),
    (0x0014A6, i_0014a6),
    (0x0014AA, i_0014aa),
    (0x001526, i_001526),
    (0x00152C, i_00152c),
    (0x001532, i_001532),
    (0x00153A, i_00153a),
    (0x001540, i_001540),
    (0x001544, i_001544),
    (0x001548, i_001548),
    (0x00154E, i_00154e),
    (0x001552, i_001552),
    (0x001744, i_001744),
    (0x00174A, i_00174a),
    (0x001750, i_001750),
    (0x001756, i_001756),
    (0x00175C, i_00175c),
    (0x001764, i_001764),
    (0x001768, i_001768),
    (0x00176E, i_00176e),
    (0x001774, i_001774),
    (0x00177A, i_00177a),
    (0x001782, i_001782),
    (0x00178A, i_00178a),
    (0x001792, i_001792),
    (0x001796, i_001796),
    (0x00179A, i_00179a),
    (0x00179E, i_00179e),
    (0x0017A4, i_0017a4),
    (0x0017A8, i_0017a8),
    (0x0017AC, i_0017ac),
    (0x0017B2, i_0017b2),
    (0x0017B6, i_0017b6),
    (0x0017BC, i_0017bc),
    (0x0017C2, i_0017c2),
    (0x0017C6, i_0017c6),
    (0x0017E6, i_0017e6),
    (0x0017EA, i_0017ea),
    (0x0017F2, i_0017f2),
    (0x0017F6, i_0017f6),
    (0x0017FC, i_0017fc),
    (0x001802, i_001802),
    (0x001808, i_001808),
    (0x00180E, i_00180e),
    (0x001812, i_001812),
    (0x001816, i_001816),
    (0x00181C, i_00181c),
    (0x001822, i_001822),
    (0x001826, i_001826),
    (0x00182A, i_00182a),
    (0x00182C, i_00182c),
    (0x001830, i_001830),
    (0x001834, i_001834),
    (0x001838, i_001838),
    (0x001840, i_001840),
    (0x001846, i_001846),
    (0x001850, i_001850),
    (0x001854, i_001854),
    (0x00185A, i_00185a),
    (0x001860, i_001860),
    (0x001866, i_001866),
    (0x001870, i_001870),
    (0x001874, i_001874),
    (0x00187A, i_00187a),
    (0x001880, i_001880),
    (0x001886, i_001886),
    (0x00188A, i_00188a),
    (0x001890, i_001890),
    (0x00189A, i_00189a),
    (0x0018A0, i_0018a0),
    (0x0018A4, i_0018a4),
    (0x0018AA, i_0018aa),
    (0x0018AE, i_0018ae),
    (0x0018B4, i_0018b4),
    (0x0018B8, i_0018b8),
    (0x0018C0, i_0018c0),
    (0x0018C8, i_0018c8),
    (0x0018CC, i_0018cc),
    (0x0018D2, i_0018d2),
    (0x0018D8, i_0018d8),
    (0x0018DE, i_0018de),
    (0x0018E2, i_0018e2),
    (0x0018E8, i_0018e8),
    (0x0018EE, i_0018ee),
    (0x0018F4, i_0018f4),
    (0x0018FA, i_0018fa),
    (0x001900, i_001900),
    (0x001906, i_001906),
    (0x00190C, i_00190c),
    (0x001912, i_001912),
    (0x001918, i_001918),
    (0x00191E, i_00191e),
    (0x001922, i_001922),
    (0x00192A, i_00192a),
    (0x001930, i_001930),
    (0x001936, i_001936),
    (0x00193C, i_00193c),
    (0x001A70, i_001a70),
    (0x001A74, i_001a74),
    (0x001A7A, i_001a7a),
    (0x001A7C, i_001a7c),
    (0x001A7E, i_001a7e),
    (0x001A82, i_001a82),
    (0x001A86, i_001a86),
    (0x001A8A, i_001a8a),
    (0x001A8E, i_001a8e),
    (0x001A92, i_001a92),
    (0x001A96, i_001a96),
    (0x001A9A, i_001a9a),
    (0x001A9E, i_001a9e),
    (0x001AA2, i_001aa2),
    (0x001AAA, i_001aaa),
    (0x001AAE, i_001aae),
    (0x001AB4, i_001ab4),
    (0x001AB6, i_001ab6),
    (0x001ABC, i_001abc),
    (0x001AC4, i_001ac4),
    (0x001AC8, i_001ac8),
    (0x001ACE, i_001ace),
    (0x001AD2, i_001ad2),
    (0x001AD6, i_001ad6),
    (0x001AD8, i_001ad8),
    (0x001ADC, i_001adc),
    (0x001AE2, i_001ae2),
    (0x001AE6, i_001ae6),
    (0x001AEC, i_001aec),
    (0x001AF0, i_001af0),
    (0x001AF6, i_001af6),
    (0x001AF8, i_001af8),
    (0x001AFE, i_001afe),
    (0x001B02, i_001b02),
    (0x001B06, i_001b06),
    (0x001B0A, i_001b0a),
    (0x001B0E, i_001b0e),
    (0x001B14, i_001b14),
    (0x001B18, i_001b18),
    (0x001B1A, i_001b1a),
    (0x001B1C, i_001b1c),
    (0x001B20, i_001b20),
    (0x001B22, i_001b22),
    (0x001B28, i_001b28),
    (0x001B2C, i_001b2c),
    (0x001B2E, i_001b2e),
    (0x001B30, i_001b30),
    (0x001B36, i_001b36),
    (0x001B3A, i_001b3a),
    (0x001B3C, i_001b3c),
    (0x001B3E, i_001b3e),
    (0x001B40, i_001b40),
    (0x001B44, i_001b44),
    (0x001B4A, i_001b4a),
    (0x001B4C, i_001b4c),
    (0x001B52, i_001b52),
    (0x001B58, i_001b58),
    (0x001B5C, i_001b5c),
    (0x001B62, i_001b62),
    (0x001B68, i_001b68),
    (0x001B6C, i_001b6c),
    (0x001B6E, i_001b6e),
    (0x001B70, i_001b70),
    (0x001B74, i_001b74),
    (0x001B78, i_001b78),
    (0x001B7C, i_001b7c),
    (0x001B7E, i_001b7e),
    (0x001B80, i_001b80),
    (0x001B84, i_001b84),
    (0x001B88, i_001b88),
    (0x001B8E, i_001b8e),
    (0x001B92, i_001b92),
    (0x001B98, i_001b98),
    (0x001B9C, i_001b9c),
    (0x001BA2, i_001ba2),
    (0x001BA6, i_001ba6),
    (0x001BAA, i_001baa),
    (0x001BB0, i_001bb0),
    (0x001BB8, i_001bb8),
    (0x001BBC, i_001bbc),
    (0x001BBE, i_001bbe),
    (0x001BC2, i_001bc2),
    (0x001BC6, i_001bc6),
    (0x001BCA, i_001bca),
    (0x001BCE, i_001bce),
    (0x001BD2, i_001bd2),
    (0x001BD8, i_001bd8),
    (0x001BDC, i_001bdc),
    (0x001BDE, i_001bde),
    (0x001BE6, i_001be6),
    (0x001BEA, i_001bea),
    (0x001BEC, i_001bec),
    (0x001BF0, i_001bf0),
    (0x001BF4, i_001bf4),
    (0x001BF8, i_001bf8),
    (0x001BFC, i_001bfc),
    (0x001C02, i_001c02),
    (0x001C08, i_001c08),
    (0x001C0C, i_001c0c),
    (0x001C12, i_001c12),
    (0x001C18, i_001c18),
    (0x001C1E, i_001c1e),
    (0x001C24, i_001c24),
    (0x001C26, i_001c26),
    (0x001C2A, i_001c2a),
    (0x001C2C, i_001c2c),
    (0x001C32, i_001c32),
    (0x001C44, i_001c44),
    (0x001C4C, i_001c4c),
    (0x001C50, i_001c50),
    (0x001C56, i_001c56),
    (0x001C5E, i_001c5e),
    (0x001C62, i_001c62),
    (0x001C68, i_001c68),
    (0x001C6E, i_001c6e),
    (0x001C72, i_001c72),
    (0x001C74, i_001c74),
    (0x001C7A, i_001c7a),
    (0x001C7E, i_001c7e),
    (0x001C84, i_001c84),
    (0x001C86, i_001c86),
    (0x001CAC, i_001cac),
    (0x001CB2, i_001cb2),
    (0x001CB4, i_001cb4),
    (0x001CB8, i_001cb8),
    (0x001CBC, i_001cbc),
    (0x001CC0, i_001cc0),
    (0x001CC4, i_001cc4),
    (0x001CC8, i_001cc8),
    (0x001CCC, i_001ccc),
    (0x001CCE, i_001cce),
    (0x001CD2, i_001cd2),
    (0x001CD4, i_001cd4),
    (0x001CDA, i_001cda),
    (0x001CE0, i_001ce0),
    (0x001CE6, i_001ce6),
    (0x001CEC, i_001cec),
    (0x001CF2, i_001cf2),
    (0x001CF8, i_001cf8),
    (0x001CFE, i_001cfe),
    (0x001D04, i_001d04),
    (0x001D0A, i_001d0a),
    (0x001D10, i_001d10),
    (0x001D16, i_001d16),
    (0x001D1C, i_001d1c),
    (0x001D22, i_001d22),
    (0x001D28, i_001d28),
    (0x001D2E, i_001d2e),
    (0x001D34, i_001d34),
    (0x001D3A, i_001d3a),
    (0x001DA4, i_001da4),
    (0x001DA8, i_001da8),
    (0x001DAE, i_001dae),
    (0x001DB2, i_001db2),
    (0x001DB8, i_001db8),
    (0x001DC0, i_001dc0),
    (0x001DC6, i_001dc6),
    (0x001DCC, i_001dcc),
    (0x001DD2, i_001dd2),
    (0x001DD8, i_001dd8),
    (0x001DDE, i_001dde),
    (0x001DE4, i_001de4),
    (0x001DEA, i_001dea),
    (0x001DF0, i_001df0),
    (0x001DF8, i_001df8),
    (0x001DFC, i_001dfc),
    (0x001E02, i_001e02),
    (0x001E08, i_001e08),
    (0x001E0A, i_001e0a),
    (0x001E12, i_001e12),
    (0x001E1A, i_001e1a),
    (0x001E1C, i_001e1c),
    (0x001E1E, i_001e1e),
    (0x001E20, i_001e20),
    (0x001E22, i_001e22),
    (0x001E28, i_001e28),
    (0x001E2E, i_001e2e),
    (0x001E32, i_001e32),
    (0x001E36, i_001e36),
    (0x001E3C, i_001e3c),
    (0x001E40, i_001e40),
    (0x001E44, i_001e44),
    (0x001E46, i_001e46),
    (0x001E4A, i_001e4a),
    (0x001E4C, i_001e4c),
    (0x001E54, i_001e54),
    (0x001E56, i_001e56),
    (0x001E5C, i_001e5c),
    (0x001E5E, i_001e5e),
    (0x001E66, i_001e66),
    (0x001E6C, i_001e6c),
    (0x001E74, i_001e74),
    (0x001E78, i_001e78),
    (0x001E7E, i_001e7e),
    (0x001E84, i_001e84),
    (0x001E8A, i_001e8a),
    (0x001E90, i_001e90),
    (0x001E94, i_001e94),
    (0x001E96, i_001e96),
    (0x001E9C, i_001e9c),
    (0x001EA0, i_001ea0),
    (0x001EA8, i_001ea8),
    (0x001EAC, i_001eac),
    (0x001EB4, i_001eb4),
    (0x001EB8, i_001eb8),
    (0x001EBE, i_001ebe),
    (0x001EC2, i_001ec2),
    (0x001EC8, i_001ec8),
    (0x001ECC, i_001ecc),
    (0x001ED2, i_001ed2),
    (0x001ED6, i_001ed6),
    (0x001EDC, i_001edc),
    (0x001EE2, i_001ee2),
    (0x001EE6, i_001ee6),
    (0x001EEC, i_001eec),
    (0x001EF4, i_001ef4),
    (0x001EF6, i_001ef6),
    (0x001EFC, i_001efc),
    (0x001EFE, i_001efe),
    (0x001F04, i_001f04),
    (0x001F06, i_001f06),
    (0x001F0C, i_001f0c),
    (0x001F12, i_001f12),
    (0x001F16, i_001f16),
    (0x001F18, i_001f18),
    (0x001F1C, i_001f1c),
    (0x001F20, i_001f20),
    (0x001F22, i_001f22),
    (0x001F28, i_001f28),
    (0x001F2E, i_001f2e),
    (0x001F32, i_001f32),
    (0x001F34, i_001f34),
    (0x001F38, i_001f38),
    (0x001F3C, i_001f3c),
    (0x001F3E, i_001f3e),
    (0x001F40, i_001f40),
    (0x001F42, i_001f42),
    (0x001F48, i_001f48),
    (0x001F4A, i_001f4a),
    (0x001F50, i_001f50),
    (0x001F52, i_001f52),
    (0x001F56, i_001f56),
    (0x001F5A, i_001f5a),
    (0x001F5C, i_001f5c),
    (0x001F62, i_001f62),
    (0x001F66, i_001f66),
    (0x001F68, i_001f68),
    (0x001F6C, i_001f6c),
    (0x001F6E, i_001f6e),
    (0x001F74, i_001f74),
    (0x001F78, i_001f78),
    (0x001F7C, i_001f7c),
    (0x001F82, i_001f82),
    (0x001F84, i_001f84),
    (0x001F8C, i_001f8c),
    (0x001F94, i_001f94),
    (0x001F98, i_001f98),
    (0x001F9C, i_001f9c),
    (0x001FA0, i_001fa0),
    (0x001FA4, i_001fa4),
    (0x001FAA, i_001faa),
    (0x001FB0, i_001fb0),
    (0x001FB6, i_001fb6),
    (0x001FBC, i_001fbc),
    (0x001FC2, i_001fc2),
    (0x001FC8, i_001fc8),
    (0x001FCE, i_001fce),
    (0x001FD4, i_001fd4),
    (0x001FDC, i_001fdc),
    (0x001FE0, i_001fe0),
    (0x001FE2, i_001fe2),
    (0x001FE8, i_001fe8),
    (0x001FEE, i_001fee),
    (0x001FF6, i_001ff6),
    (0x001FFA, i_001ffa),
    (0x002002, i_002002),
    (0x002004, i_002004),
    (0x00200A, i_00200a),
    (0x00200C, i_00200c),
    (0x002012, i_002012),
    (0x002018, i_002018),
    (0x00201E, i_00201e),
    (0x002022, i_002022),
    (0x002028, i_002028),
    (0x00202E, i_00202e),
    (0x002030, i_002030),
    (0x002036, i_002036),
    (0x00203C, i_00203c),
    (0x002042, i_002042),
    (0x002048, i_002048),
    (0x00204E, i_00204e),
    (0x002054, i_002054),
    (0x00205A, i_00205a),
    (0x002060, i_002060),
    (0x002066, i_002066),
    (0x00206C, i_00206c),
    (0x002072, i_002072),
    (0x00207A, i_00207a),
    (0x00207C, i_00207c),
    (0x002082, i_002082),
    (0x002088, i_002088),
    (0x00208E, i_00208e),
    (0x002094, i_002094),
    (0x002096, i_002096),
    (0x002098, i_002098),
    (0x00209C, i_00209c),
    (0x0020A0, i_0020a0),
    (0x0020A4, i_0020a4),
    (0x0020A8, i_0020a8),
    (0x0020AA, i_0020aa),
    (0x0020B2, i_0020b2),
    (0x0020B8, i_0020b8),
    (0x0020BE, i_0020be),
    (0x0020C6, i_0020c6),
    (0x0020CC, i_0020cc),
    (0x0020D2, i_0020d2),
    (0x0020D8, i_0020d8),
    (0x0020DA, i_0020da),
    (0x0020E0, i_0020e0),
    (0x0020E2, i_0020e2),
    (0x0020E8, i_0020e8),
    (0x0020EE, i_0020ee),
    (0x0020F4, i_0020f4),
    (0x0020FA, i_0020fa),
    (0x002100, i_002100),
    (0x002106, i_002106),
    (0x00210C, i_00210c),
    (0x002112, i_002112),
    (0x002118, i_002118),
    (0x00211E, i_00211e),
    (0x002124, i_002124),
    (0x002126, i_002126),
    (0x00212C, i_00212c),
    (0x00212E, i_00212e),
    (0x002134, i_002134),
    (0x00213A, i_00213a),
    (0x002140, i_002140),
    (0x002146, i_002146),
    (0x00214E, i_00214e),
    (0x002152, i_002152),
    (0x002156, i_002156),
    (0x00215A, i_00215a),
    (0x002160, i_002160),
    (0x002164, i_002164),
    (0x00216A, i_00216a),
    (0x002170, i_002170),
    (0x002176, i_002176),
    (0x00217A, i_00217a),
    (0x00217C, i_00217c),
    (0x002180, i_002180),
    (0x002186, i_002186),
    (0x00218A, i_00218a),
    (0x002190, i_002190),
    (0x002192, i_002192),
    (0x00219A, i_00219a),
    (0x00219C, i_00219c),
    (0x0021A0, i_0021a0),
    (0x0021A4, i_0021a4),
    (0x0021A6, i_0021a6),
    (0x0021AA, i_0021aa),
    (0x0021AE, i_0021ae),
    (0x0021B0, i_0021b0),
    (0x0021B2, i_0021b2),
    (0x0021B4, i_0021b4),
    (0x0021B8, i_0021b8),
    (0x0021BC, i_0021bc),
    (0x0021C0, i_0021c0),
    (0x0021C4, i_0021c4),
    (0x0021C8, i_0021c8),
    (0x0021CC, i_0021cc),
    (0x0021D0, i_0021d0),
    (0x0021D4, i_0021d4),
    (0x0021D6, i_0021d6),
    (0x0021DA, i_0021da),
    (0x0021DE, i_0021de),
    (0x0021E4, i_0021e4),
    (0x0021E8, i_0021e8),
    (0x0021EE, i_0021ee),
    (0x0021F2, i_0021f2),
    (0x0021FA, i_0021fa),
    (0x0021FE, i_0021fe),
    (0x002200, i_002200),
    (0x002206, i_002206),
    (0x00220A, i_00220a),
    (0x00220E, i_00220e),
    (0x002212, i_002212),
    (0x002214, i_002214),
    (0x002216, i_002216),
    (0x002218, i_002218),
    (0x00221C, i_00221c),
    (0x00221E, i_00221e),
    (0x002222, i_002222),
    (0x002224, i_002224),
    (0x002226, i_002226),
    (0x00222A, i_00222a),
    (0x00222C, i_00222c),
    (0x002242, i_002242),
    (0x002248, i_002248),
    (0x00224E, i_00224e),
    (0x002252, i_002252),
    (0x002258, i_002258),
    (0x00225C, i_00225c),
    (0x002260, i_002260),
    (0x002262, i_002262),
    (0x002264, i_002264),
    (0x002268, i_002268),
    (0x00226A, i_00226a),
    (0x002270, i_002270),
    (0x002274, i_002274),
    (0x00227A, i_00227a),
    (0x002282, i_002282),
    (0x002286, i_002286),
    (0x00228C, i_00228c),
    (0x002292, i_002292),
    (0x002298, i_002298),
    (0x00229E, i_00229e),
    (0x0022A2, i_0022a2),
    (0x0022A8, i_0022a8),
    (0x0022B0, i_0022b0),
    (0x0022B2, i_0022b2),
    (0x0022B6, i_0022b6),
    (0x0022BC, i_0022bc),
    (0x0022BE, i_0022be),
    (0x0022C6, i_0022c6),
    (0x0022C8, i_0022c8),
    (0x0022CA, i_0022ca),
    (0x002352, i_002352),
    (0x00235A, i_00235a),
    (0x00235E, i_00235e),
    (0x002364, i_002364),
    (0x002368, i_002368),
    (0x00236C, i_00236c),
    (0x00236E, i_00236e),
    (0x002372, i_002372),
    (0x002374, i_002374),
    (0x002378, i_002378),
    (0x00237E, i_00237e),
    (0x002382, i_002382),
    (0x002388, i_002388),
    (0x00238C, i_00238c),
    (0x002392, i_002392),
    (0x002394, i_002394),
    (0x00239A, i_00239a),
    (0x00239C, i_00239c),
    (0x00239E, i_00239e),
    (0x0023A4, i_0023a4),
    (0x0023AA, i_0023aa),
    (0x0023B0, i_0023b0),
    (0x0023B6, i_0023b6),
    (0x0023B8, i_0023b8),
    (0x0023BC, i_0023bc),
    (0x0023C0, i_0023c0),
    (0x0023C2, i_0023c2),
    (0x0023C6, i_0023c6),
    (0x0023CA, i_0023ca),
    (0x0023CE, i_0023ce),
    (0x0023D0, i_0023d0),
    (0x0023D6, i_0023d6),
    (0x0023DA, i_0023da),
    (0x0023DE, i_0023de),
    (0x0023E0, i_0023e0),
    (0x0023E4, i_0023e4),
    (0x0023E8, i_0023e8),
    (0x0023EC, i_0023ec),
    (0x0023EE, i_0023ee),
    (0x0023F4, i_0023f4),
    (0x0023F8, i_0023f8),
    (0x0023FC, i_0023fc),
    (0x0023FE, i_0023fe),
    (0x002402, i_002402),
    (0x002406, i_002406),
    (0x00240A, i_00240a),
    (0x00240C, i_00240c),
    (0x002412, i_002412),
    (0x002416, i_002416),
    (0x00241A, i_00241a),
    (0x00241C, i_00241c),
    (0x002420, i_002420),
    (0x002424, i_002424),
    (0x002428, i_002428),
    (0x00242A, i_00242a),
    (0x002430, i_002430),
    (0x002434, i_002434),
    (0x002438, i_002438),
    (0x00243A, i_00243a),
    (0x00243E, i_00243e),
    (0x002442, i_002442),
    (0x002446, i_002446),
    (0x002448, i_002448),
    (0x00244E, i_00244e),
    (0x002452, i_002452),
    (0x002456, i_002456),
    (0x002458, i_002458),
    (0x00245C, i_00245c),
    (0x002460, i_002460),
    (0x002464, i_002464),
    (0x002466, i_002466),
    (0x00246C, i_00246c),
    (0x002470, i_002470),
    (0x002474, i_002474),
    (0x002476, i_002476),
    (0x00247A, i_00247a),
    (0x00247E, i_00247e),
    (0x002482, i_002482),
    (0x002484, i_002484),
    (0x00248A, i_00248a),
    (0x00248E, i_00248e),
    (0x002492, i_002492),
    (0x002494, i_002494),
    (0x002498, i_002498),
    (0x00249C, i_00249c),
    (0x0024A0, i_0024a0),
    (0x0024A2, i_0024a2),
    (0x0024A8, i_0024a8),
    (0x0024AC, i_0024ac),
    (0x0024B0, i_0024b0),
    (0x0024B2, i_0024b2),
    (0x0024B6, i_0024b6),
    (0x0024BA, i_0024ba),
    (0x0024BE, i_0024be),
    (0x0024C0, i_0024c0),
    (0x0024C6, i_0024c6),
    (0x0024CA, i_0024ca),
    (0x0024CE, i_0024ce),
    (0x0024D0, i_0024d0),
    (0x0024D4, i_0024d4),
    (0x0024D8, i_0024d8),
    (0x0024DC, i_0024dc),
    (0x0024DE, i_0024de),
    (0x0024E4, i_0024e4),
    (0x0024E8, i_0024e8),
    (0x0024EC, i_0024ec),
    (0x0024EE, i_0024ee),
    (0x0024F2, i_0024f2),
    (0x0024F6, i_0024f6),
    (0x0024FA, i_0024fa),
    (0x0024FC, i_0024fc),
    (0x002502, i_002502),
    (0x002506, i_002506),
    (0x00250A, i_00250a),
    (0x00250C, i_00250c),
    (0x002510, i_002510),
    (0x002514, i_002514),
    (0x002518, i_002518),
    (0x00251A, i_00251a),
    (0x002520, i_002520),
    (0x002524, i_002524),
    (0x002528, i_002528),
    (0x00252A, i_00252a),
    (0x00252E, i_00252e),
    (0x002532, i_002532),
    (0x002536, i_002536),
    (0x002538, i_002538),
    (0x00253E, i_00253e),
    (0x002542, i_002542),
    (0x002546, i_002546),
    (0x002548, i_002548),
    (0x00254C, i_00254c),
    (0x002550, i_002550),
    (0x002554, i_002554),
    (0x002556, i_002556),
    (0x00255C, i_00255c),
    (0x002560, i_002560),
    (0x002564, i_002564),
    (0x002566, i_002566),
    (0x00256A, i_00256a),
    (0x00256E, i_00256e),
    (0x002572, i_002572),
    (0x002574, i_002574),
    (0x00257A, i_00257a),
    (0x00257E, i_00257e),
    (0x002582, i_002582),
    (0x002584, i_002584),
    (0x002588, i_002588),
    (0x00258C, i_00258c),
    (0x002590, i_002590),
    (0x002592, i_002592),
    (0x002598, i_002598),
    (0x00259C, i_00259c),
    (0x0025A0, i_0025a0),
    (0x0025A2, i_0025a2),
    (0x0025A6, i_0025a6),
    (0x0025AA, i_0025aa),
    (0x0025AE, i_0025ae),
    (0x0025B0, i_0025b0),
    (0x0025B6, i_0025b6),
    (0x0025BA, i_0025ba),
    (0x0025BE, i_0025be),
    (0x0025C0, i_0025c0),
    (0x0025C4, i_0025c4),
    (0x0025C8, i_0025c8),
    (0x0025CC, i_0025cc),
    (0x0025CE, i_0025ce),
    (0x0025D4, i_0025d4),
    (0x0025D8, i_0025d8),
    (0x0025DC, i_0025dc),
    (0x0025DE, i_0025de),
    (0x0025E2, i_0025e2),
    (0x0025E6, i_0025e6),
    (0x0025EA, i_0025ea),
    (0x0025EC, i_0025ec),
    (0x0025F2, i_0025f2),
    (0x0025F6, i_0025f6),
    (0x0025FA, i_0025fa),
    (0x0025FC, i_0025fc),
    (0x002600, i_002600),
    (0x002604, i_002604),
    (0x002608, i_002608),
    (0x00260A, i_00260a),
    (0x002610, i_002610),
    (0x002614, i_002614),
    (0x002618, i_002618),
    (0x00261A, i_00261a),
    (0x00261E, i_00261e),
    (0x002622, i_002622),
    (0x002626, i_002626),
    (0x002628, i_002628),
    (0x00262E, i_00262e),
    (0x002632, i_002632),
    (0x002636, i_002636),
    (0x002638, i_002638),
    (0x00263C, i_00263c),
    (0x002640, i_002640),
    (0x002644, i_002644),
    (0x002646, i_002646),
    (0x00264C, i_00264c),
    (0x002650, i_002650),
    (0x002654, i_002654),
    (0x002656, i_002656),
    (0x00265A, i_00265a),
    (0x00265E, i_00265e),
    (0x002662, i_002662),
    (0x002664, i_002664),
    (0x00266A, i_00266a),
    (0x00266E, i_00266e),
    (0x002672, i_002672),
    (0x002674, i_002674),
    (0x002678, i_002678),
    (0x00267C, i_00267c),
    (0x002680, i_002680),
    (0x002682, i_002682),
    (0x002688, i_002688),
    (0x00268C, i_00268c),
    (0x002690, i_002690),
    (0x002692, i_002692),
    (0x002696, i_002696),
    (0x00269A, i_00269a),
    (0x00269E, i_00269e),
    (0x0026A0, i_0026a0),
    (0x0026A6, i_0026a6),
    (0x0026AA, i_0026aa),
    (0x0026AE, i_0026ae),
    (0x0026B0, i_0026b0),
    (0x0026B4, i_0026b4),
    (0x0026B8, i_0026b8),
    (0x0026BC, i_0026bc),
    (0x0026BE, i_0026be),
    (0x0026C4, i_0026c4),
    (0x0026C8, i_0026c8),
    (0x0026CC, i_0026cc),
    (0x0026CE, i_0026ce),
    (0x0026D2, i_0026d2),
    (0x0026D6, i_0026d6),
    (0x0026DA, i_0026da),
    (0x0026DC, i_0026dc),
    (0x0026E2, i_0026e2),
    (0x0026E6, i_0026e6),
    (0x0026EA, i_0026ea),
    (0x0026EC, i_0026ec),
    (0x0026F0, i_0026f0),
    (0x0026F4, i_0026f4),
    (0x0026F8, i_0026f8),
    (0x0026FA, i_0026fa),
    (0x002700, i_002700),
    (0x002704, i_002704),
    (0x002708, i_002708),
    (0x00270A, i_00270a),
    (0x00270E, i_00270e),
    (0x002712, i_002712),
    (0x002716, i_002716),
    (0x002718, i_002718),
    (0x00271E, i_00271e),
    (0x002722, i_002722),
    (0x002726, i_002726),
    (0x002728, i_002728),
    (0x00272C, i_00272c),
    (0x002730, i_002730),
    (0x002734, i_002734),
    (0x002736, i_002736),
    (0x00273C, i_00273c),
    (0x002740, i_002740),
    (0x002744, i_002744),
    (0x002746, i_002746),
    (0x00274A, i_00274a),
    (0x00274E, i_00274e),
    (0x002752, i_002752),
    (0x002754, i_002754),
    (0x00275A, i_00275a),
    (0x00275E, i_00275e),
    (0x002762, i_002762),
    (0x002764, i_002764),
    (0x002768, i_002768),
    (0x00276C, i_00276c),
    (0x002770, i_002770),
    (0x002772, i_002772),
    (0x002778, i_002778),
    (0x00277C, i_00277c),
    (0x002780, i_002780),
    (0x002782, i_002782),
    (0x002786, i_002786),
    (0x00278A, i_00278a),
    (0x00278E, i_00278e),
    (0x002790, i_002790),
    (0x002796, i_002796),
    (0x00279A, i_00279a),
    (0x00279E, i_00279e),
    (0x0027A0, i_0027a0),
    (0x0027A4, i_0027a4),
    (0x0027A8, i_0027a8),
    (0x0027AC, i_0027ac),
    (0x0027AE, i_0027ae),
    (0x0027B4, i_0027b4),
    (0x0027B8, i_0027b8),
    (0x0027BC, i_0027bc),
    (0x0027BE, i_0027be),
    (0x0027C2, i_0027c2),
    (0x0027C6, i_0027c6),
    (0x0027CA, i_0027ca),
    (0x0027CC, i_0027cc),
    (0x0027D2, i_0027d2),
    (0x0027D6, i_0027d6),
    (0x0027DA, i_0027da),
    (0x0027DC, i_0027dc),
    (0x0027E0, i_0027e0),
    (0x0027E4, i_0027e4),
    (0x0027E8, i_0027e8),
    (0x0027EA, i_0027ea),
    (0x0027F0, i_0027f0),
    (0x0027F4, i_0027f4),
    (0x0027F8, i_0027f8),
    (0x0027FA, i_0027fa),
    (0x0027FE, i_0027fe),
    (0x002802, i_002802),
    (0x002806, i_002806),
    (0x002808, i_002808),
    (0x00280E, i_00280e),
    (0x002812, i_002812),
    (0x002816, i_002816),
    (0x002818, i_002818),
    (0x00281C, i_00281c),
    (0x002820, i_002820),
    (0x002824, i_002824),
    (0x002826, i_002826),
    (0x00282C, i_00282c),
    (0x002830, i_002830),
    (0x002834, i_002834),
    (0x002836, i_002836),
    (0x00283A, i_00283a),
    (0x00283E, i_00283e),
    (0x002842, i_002842),
    (0x002844, i_002844),
    (0x00284A, i_00284a),
    (0x00284E, i_00284e),
    (0x002852, i_002852),
    (0x002854, i_002854),
    (0x002858, i_002858),
    (0x00285C, i_00285c),
    (0x002860, i_002860),
    (0x002862, i_002862),
    (0x002868, i_002868),
    (0x00286C, i_00286c),
    (0x002870, i_002870),
    (0x002872, i_002872),
    (0x002876, i_002876),
    (0x00287A, i_00287a),
    (0x00287E, i_00287e),
    (0x002880, i_002880),
    (0x002886, i_002886),
    (0x00288A, i_00288a),
    (0x00288E, i_00288e),
    (0x002890, i_002890),
    (0x002894, i_002894),
    (0x002898, i_002898),
    (0x00289C, i_00289c),
    (0x00289E, i_00289e),
    (0x0028A4, i_0028a4),
    (0x0028A8, i_0028a8),
    (0x0028AC, i_0028ac),
    (0x0028AE, i_0028ae),
    (0x0028B2, i_0028b2),
    (0x0028B6, i_0028b6),
    (0x0028BA, i_0028ba),
    (0x0028BC, i_0028bc),
    (0x0028C2, i_0028c2),
    (0x0028C6, i_0028c6),
    (0x0028CA, i_0028ca),
    (0x0028CC, i_0028cc),
    (0x0028D0, i_0028d0),
    (0x0028D4, i_0028d4),
    (0x0028D8, i_0028d8),
    (0x0028DA, i_0028da),
    (0x0028E0, i_0028e0),
    (0x0028E4, i_0028e4),
    (0x0028E8, i_0028e8),
    (0x0028EA, i_0028ea),
    (0x0028EE, i_0028ee),
    (0x0028F2, i_0028f2),
    (0x0028F6, i_0028f6),
    (0x0028F8, i_0028f8),
    (0x0028FE, i_0028fe),
    (0x002902, i_002902),
    (0x002906, i_002906),
    (0x002908, i_002908),
    (0x00290C, i_00290c),
    (0x002910, i_002910),
    (0x002914, i_002914),
    (0x002916, i_002916),
    (0x00291C, i_00291c),
    (0x002920, i_002920),
    (0x002924, i_002924),
    (0x002926, i_002926),
    (0x00292A, i_00292a),
    (0x00292E, i_00292e),
    (0x002932, i_002932),
    (0x002934, i_002934),
    (0x00293A, i_00293a),
    (0x002940, i_002940),
    (0x002946, i_002946),
    (0x00294C, i_00294c),
    (0x002954, i_002954),
    (0x002958, i_002958),
    (0x00295E, i_00295e),
    (0x002964, i_002964),
    (0x002966, i_002966),
    (0x00296A, i_00296a),
    (0x00296C, i_00296c),
    (0x002970, i_002970),
    (0x002972, i_002972),
    (0x002974, i_002974),
    (0x00297A, i_00297a),
    (0x00297E, i_00297e),
    (0x002982, i_002982),
    (0x002988, i_002988),
    (0x00298C, i_00298c),
    (0x002992, i_002992),
    (0x002994, i_002994),
    (0x002998, i_002998),
    (0x00299C, i_00299c),
    (0x00299E, i_00299e),
    (0x0029A2, i_0029a2),
    (0x0029A6, i_0029a6),
    (0x0029A8, i_0029a8),
    (0x0029AA, i_0029aa),
    (0x0029AC, i_0029ac),
    (0x0029B2, i_0029b2),
    (0x0029B4, i_0029b4),
    (0x0029BA, i_0029ba),
    (0x0029BC, i_0029bc),
    (0x0029C0, i_0029c0),
    (0x0029C2, i_0029c2),
    (0x0029C6, i_0029c6),
    (0x0029C8, i_0029c8),
    (0x0029CA, i_0029ca),
    (0x0029CC, i_0029cc),
    (0x0029D0, i_0029d0),
    (0x0029D2, i_0029d2),
    (0x0029D8, i_0029d8),
    (0x0029DC, i_0029dc),
    (0x0029DE, i_0029de),
    (0x0029E2, i_0029e2),
    (0x0029E4, i_0029e4),
    (0x0029EA, i_0029ea),
    (0x0029EE, i_0029ee),
    (0x0029F0, i_0029f0),
    (0x002AE8, i_002ae8),
    (0x002AEC, i_002aec),
    (0x002AEE, i_002aee),
    (0x002AF2, i_002af2),
    (0x002AF6, i_002af6),
    (0x002AFC, i_002afc),
    (0x002B00, i_002b00),
    (0x002B02, i_002b02),
    (0x002B06, i_002b06),
    (0x002B08, i_002b08),
    (0x002B0C, i_002b0c),
    (0x002B0E, i_002b0e),
    (0x002B14, i_002b14),
    (0x002B18, i_002b18),
    (0x002B1C, i_002b1c),
    (0x002B20, i_002b20),
    (0x002B26, i_002b26),
    (0x002B28, i_002b28),
    (0x002B2E, i_002b2e),
    (0x002B30, i_002b30),
    (0x002B34, i_002b34),
    (0x002B3A, i_002b3a),
    (0x002B3E, i_002b3e),
    (0x002B44, i_002b44),
    (0x002B46, i_002b46),
    (0x002B48, i_002b48),
    (0x002B4A, i_002b4a),
    (0x002B4E, i_002b4e),
    (0x002B54, i_002b54),
    (0x002B56, i_002b56),
    (0x002B58, i_002b58),
    (0x002B5A, i_002b5a),
    (0x002B5E, i_002b5e),
    (0x002B62, i_002b62),
    (0x002B68, i_002b68),
    (0x002B6C, i_002b6c),
    (0x002B70, i_002b70),
    (0x002B74, i_002b74),
    (0x002B78, i_002b78),
    (0x002B7A, i_002b7a),
    (0x002B80, i_002b80),
    (0x002B84, i_002b84),
    (0x002B88, i_002b88),
    (0x002B8C, i_002b8c),
    (0x002B92, i_002b92),
    (0x002B94, i_002b94),
    (0x002B9A, i_002b9a),
    (0x002B9C, i_002b9c),
    (0x002BA0, i_002ba0),
    (0x002BA6, i_002ba6),
    (0x002BAA, i_002baa),
    (0x002BB0, i_002bb0),
    (0x002BB2, i_002bb2),
    (0x002BB4, i_002bb4),
    (0x002BB6, i_002bb6),
    (0x002BBA, i_002bba),
    (0x002BC0, i_002bc0),
    (0x002BC2, i_002bc2),
    (0x002BC4, i_002bc4),
    (0x002BC8, i_002bc8),
    (0x002BCA, i_002bca),
    (0x002BCC, i_002bcc),
    (0x002BD2, i_002bd2),
    (0x002BD6, i_002bd6),
    (0x002BDA, i_002bda),
    (0x002BE0, i_002be0),
    (0x002BE6, i_002be6),
    (0x002BEA, i_002bea),
    (0x002BEE, i_002bee),
    (0x002BF4, i_002bf4),
    (0x002BFA, i_002bfa),
    (0x002BFE, i_002bfe),
    (0x002C02, i_002c02),
    (0x002C08, i_002c08),
    (0x002C10, i_002c10),
    (0x002C14, i_002c14),
    (0x002C1A, i_002c1a),
    (0x002C20, i_002c20),
    (0x002C24, i_002c24),
    (0x002C26, i_002c26),
    (0x002C2C, i_002c2c),
    (0x002C30, i_002c30),
    (0x002C36, i_002c36),
    (0x002C3C, i_002c3c),
    (0x002C3E, i_002c3e),
    (0x002C40, i_002c40),
    (0x002C42, i_002c42),
    (0x002C48, i_002c48),
    (0x002C4C, i_002c4c),
    (0x002C50, i_002c50),
    (0x002C54, i_002c54),
    (0x002C5A, i_002c5a),
    (0x002C5E, i_002c5e),
    (0x002C64, i_002c64),
    (0x002C66, i_002c66),
    (0x002C6C, i_002c6c),
    (0x002C6E, i_002c6e),
    (0x002C74, i_002c74),
    (0x002C86, i_002c86),
    (0x002C8E, i_002c8e),
    (0x002C92, i_002c92),
    (0x002C98, i_002c98),
    (0x002C9E, i_002c9e),
    (0x002CA0, i_002ca0),
    (0x002CA6, i_002ca6),
    (0x002CAC, i_002cac),
    (0x002CB2, i_002cb2),
    (0x002CB6, i_002cb6),
    (0x002CBC, i_002cbc),
    (0x002CC0, i_002cc0),
    (0x002CC4, i_002cc4),
    (0x002CC8, i_002cc8),
    (0x002CCA, i_002cca),
    (0x002CD0, i_002cd0),
    (0x002CD6, i_002cd6),
    (0x002CDC, i_002cdc),
    (0x002CE0, i_002ce0),
    (0x002CE6, i_002ce6),
    (0x002CE8, i_002ce8),
    (0x002CEC, i_002cec),
    (0x002CF2, i_002cf2),
    (0x002CF6, i_002cf6),
    (0x002CFC, i_002cfc),
    (0x002D00, i_002d00),
    (0x002D06, i_002d06),
    (0x002D0E, i_002d0e),
    (0x002D14, i_002d14),
    (0x002D18, i_002d18),
    (0x002D1C, i_002d1c),
    (0x002D22, i_002d22),
    (0x002D24, i_002d24),
    (0x002D2A, i_002d2a),
    (0x002D32, i_002d32),
    (0x002D38, i_002d38),
    (0x002D3C, i_002d3c),
    (0x002D42, i_002d42),
    (0x002D44, i_002d44),
    (0x002D4A, i_002d4a),
    (0x002D52, i_002d52),
    (0x002D56, i_002d56),
    (0x002D5A, i_002d5a),
    (0x002D60, i_002d60),
    (0x002D64, i_002d64),
    (0x002D6C, i_002d6c),
    (0x002D72, i_002d72),
    (0x002D76, i_002d76),
    (0x002D78, i_002d78),
    (0x002D7C, i_002d7c),
    (0x002D86, i_002d86),
    (0x002D88, i_002d88),
    (0x002D8C, i_002d8c),
    (0x002D90, i_002d90),
    (0x002D96, i_002d96),
    (0x002D98, i_002d98),
    (0x002D9C, i_002d9c),
    (0x002DA2, i_002da2),
    (0x002DA6, i_002da6),
    (0x002DAC, i_002dac),
    (0x002DB0, i_002db0),
    (0x002DB6, i_002db6),
    (0x002DBC, i_002dbc),
    (0x002DC0, i_002dc0),
    (0x002DC4, i_002dc4),
    (0x002DCA, i_002dca),
    (0x002DCC, i_002dcc),
    (0x002DD2, i_002dd2),
    (0x002DD6, i_002dd6),
    (0x002DDC, i_002ddc),
    (0x002DE0, i_002de0),
    (0x002DE6, i_002de6),
    (0x002DEA, i_002dea),
    (0x002DF0, i_002df0),
    (0x002DF4, i_002df4),
    (0x002DFA, i_002dfa),
    (0x002DFE, i_002dfe),
    (0x002E00, i_002e00),
    (0x002E02, i_002e02),
    (0x002E04, i_002e04),
    (0x002E08, i_002e08),
    (0x002E0A, i_002e0a),
    (0x002E0C, i_002e0c),
    (0x002E0E, i_002e0e),
    (0x002E10, i_002e10),
    (0x002E16, i_002e16),
    (0x002E1A, i_002e1a),
    (0x002E1C, i_002e1c),
    (0x002E1E, i_002e1e),
    (0x002E20, i_002e20),
    (0x002E22, i_002e22),
    (0x002E26, i_002e26),
    (0x002E2C, i_002e2c),
    (0x002E30, i_002e30),
    (0x002E34, i_002e34),
    (0x002E38, i_002e38),
    (0x002E40, i_002e40),
    (0x002E46, i_002e46),
    (0x002E4A, i_002e4a),
    (0x002E52, i_002e52),
    (0x002E58, i_002e58),
    (0x002E5C, i_002e5c),
    (0x002E5E, i_002e5e),
    (0x002E62, i_002e62),
    (0x002E68, i_002e68),
    (0x002E6A, i_002e6a),
    (0x002E6E, i_002e6e),
    (0x002E74, i_002e74),
    (0x002E78, i_002e78),
    (0x002E7E, i_002e7e),
    (0x002E82, i_002e82),
    (0x002E88, i_002e88),
    (0x002E8E, i_002e8e),
    (0x002E92, i_002e92),
    (0x002E96, i_002e96),
    (0x002E9C, i_002e9c),
    (0x002E9E, i_002e9e),
    (0x002EA4, i_002ea4),
    (0x002EA8, i_002ea8),
    (0x002EAE, i_002eae),
    (0x002EB2, i_002eb2),
    (0x002EB8, i_002eb8),
    (0x002EBC, i_002ebc),
    (0x002EC2, i_002ec2),
    (0x002EC6, i_002ec6),
    (0x002ECC, i_002ecc),
    (0x002ED0, i_002ed0),
    (0x002ED2, i_002ed2),
    (0x002ED4, i_002ed4),
    (0x002ED6, i_002ed6),
    (0x002EDA, i_002eda),
    (0x002EDC, i_002edc),
    (0x002EDE, i_002ede),
    (0x002EE0, i_002ee0),
    (0x002EE2, i_002ee2),
    (0x002EE8, i_002ee8),
    (0x002EEA, i_002eea),
    (0x002EEE, i_002eee),
    (0x002EF0, i_002ef0),
    (0x002EF4, i_002ef4),
    (0x002EFA, i_002efa),
    (0x002EFE, i_002efe),
    (0x002F02, i_002f02),
    (0x002F06, i_002f06),
    (0x002F0E, i_002f0e),
    (0x002F14, i_002f14),
    (0x002F18, i_002f18),
    (0x002F20, i_002f20),
    (0x002F26, i_002f26),
    (0x002F2A, i_002f2a),
    (0x002F2C, i_002f2c),
];
