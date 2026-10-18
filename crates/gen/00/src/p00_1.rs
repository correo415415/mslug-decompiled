// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p00_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


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
// ===== PalSlot_AllocDynamic_0029f2  ($0029F2, 138 B, boot/boot_script_sound_palslots_000400.s)
/// $0029F2: cmpi.w #$3, $1e(a6)
pub fn i_0029f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $0029F8: bge.w $2a7a
pub fn i_0029f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29FC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A7A; return 10; }
    8
}
/// $0029FC: tst.w $1082c2.l
pub fn i_0029fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A02;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x1082C2) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $002A02: beq.b $29a6
pub fn i_002a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A04;
    cpu.ir = 0x67A2;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29A6; return 10; }
    8
}
/// $002A04: andi.l #$ffff, d1
pub fn i_002a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A0A;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $002A0A: lsl.l #$6, d1
pub fn i_002a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A0C;
    cpu.ir = 0xED89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[1] = r;
    18
}
/// $002A0C: lea.l $14e00.l, a1
pub fn i_002a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A12;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x14E00;
    20
}
/// $002A12: add.l a1, d1
pub fn i_002a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A14;
    cpu.ir = 0xD289;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[1] = r;
    4
}
/// $002A14: subq.w #$1, $1082c2.l
pub fn i_002a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1A;
    cpu.ir = 0x5379;
    let v1 = bus.read16(0x1082C2) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x1082C2, r2 as u16);
    16
}
/// $002A1A: move.w $1082c4.l, d3
pub fn i_002a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A20;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x1082C4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $002A20: addq.w #$1, $1082c4.l
pub fn i_002a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A26;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x1082C4) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x1082C4, r2 as u16);
    16
}
/// $002A26: cmpi.w #$30, $1082c4.l
pub fn i_002a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x1082C4) as u32;
    sub_with_flags(cpu, v2, 0x30, Size::Word, false);
    20
}
/// $002A2E: blt.w $2a38
pub fn i_002a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A32;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2A38; return 10; }
    8
}
/// $002A32: clr.w $1082c4.l
pub fn i_002a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A38;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x1082C4) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x1082C4, r2 as u16);
    16
}
/// $002A38: lea.l $1081c2.l, a2
pub fn i_002a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x1081C2;
    20
}
/// $002A3E: moveq #$0, d2
pub fn i_002a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A40;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002A40: move.b (a2, d3.w), d2
pub fn i_002a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A44;
    cpu.ir = 0x1432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $002A44: addi.w #$94, d2
pub fn i_002a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A48;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x94, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002A48: move.w d2, d4
pub fn i_002a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A;
    cpu.ir = 0x3802;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002A4A: lsl.w #$5, d2
pub fn i_002a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4C;
    cpu.ir = 0xEB4A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002A4C: lea.l $1082c8.l, a1
pub fn i_002a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A52;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002A52: move.l d1, $2(a1, d2.w)
pub fn i_002a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A56;
    cpu.ir = 0x2381;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $002A56: addq.w #$1, $6(a1, d2.w)
pub fn i_002a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5A;
    cpu.ir = 0x5271;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $002A5A: move.b #$4, (a1, d2.w)
pub fn i_002a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A60;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002A60: ori.b #$1, (a1, d2.w)
pub fn i_002a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A66;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002A66: move.w d4, d2
pub fn i_002a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A68;
    cpu.ir = 0x3404;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002A68: move.w d2, $14(a6)
pub fn i_002a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6C;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $002A6C: move.w $1e(a6), d1
pub fn i_002a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A70;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $002A70: add.w d1, d1
pub fn i_002a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A72;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $002A72: move.w d2, $16(a6, d1.w)
pub fn i_002a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A76;
    cpu.ir = 0x3D82;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002A76: addq.w #$1, $1e(a6)
pub fn i_002a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $002A7A: rts
pub fn i_002a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7C;
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
    (0x0029F2, i_0029f2),
    (0x0029F8, i_0029f8),
    (0x0029FC, i_0029fc),
    (0x002A02, i_002a02),
    (0x002A04, i_002a04),
    (0x002A0A, i_002a0a),
    (0x002A0C, i_002a0c),
    (0x002A12, i_002a12),
    (0x002A14, i_002a14),
    (0x002A1A, i_002a1a),
    (0x002A20, i_002a20),
    (0x002A26, i_002a26),
    (0x002A2E, i_002a2e),
    (0x002A32, i_002a32),
    (0x002A38, i_002a38),
    (0x002A3E, i_002a3e),
    (0x002A40, i_002a40),
    (0x002A44, i_002a44),
    (0x002A48, i_002a48),
    (0x002A4A, i_002a4a),
    (0x002A4C, i_002a4c),
    (0x002A52, i_002a52),
    (0x002A56, i_002a56),
    (0x002A5A, i_002a5a),
    (0x002A60, i_002a60),
    (0x002A66, i_002a66),
    (0x002A68, i_002a68),
    (0x002A6C, i_002a6c),
    (0x002A70, i_002a70),
    (0x002A72, i_002a72),
    (0x002A76, i_002a76),
    (0x002A7A, i_002a7a),
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
