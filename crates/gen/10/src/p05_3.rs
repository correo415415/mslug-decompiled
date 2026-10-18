// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0552E2: rts
pub fn i_0552e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552e4  ($0552E4, 14 B, kernel/state_dispatch_stubs.c)
/// $0552E4: lea.l $29a62e.l, a2
pub fn i_0552e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552EA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A62E;
    20
}
/// $0552EA: jsr $5022a.l
pub fn i_0552ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552F0);
    cpu.pc = 0x5022A;
    16
}
/// $0552F0: rts
pub fn i_0552f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552f2  ($0552F2, 14 B, kernel/state_dispatch_stubs.c)
/// $0552F2: lea.l $29a642.l, a2
pub fn i_0552f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552F8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A642;
    20
}
/// $0552F8: jsr $5022a.l
pub fn i_0552f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552FE);
    cpu.pc = 0x5022A;
    16
}
/// $0552FE: rts
pub fn i_0552fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55300;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055300  ($055300, 14 B, kernel/state_dispatch_stubs.c)
/// $055300: lea.l $29a656.l, a2
pub fn i_055300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55306;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A656;
    20
}
/// $055306: jsr $5022a.l
pub fn i_055306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5530C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5530C);
    cpu.pc = 0x5022A;
    16
}
/// $05530C: rts
pub fn i_05530c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5530E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05530e  ($05530E, 14 B, kernel/state_dispatch_stubs.c)
/// $05530E: lea.l $29a66a.l, a2
pub fn i_05530e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55314;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A66A;
    20
}
/// $055314: jsr $5022a.l
pub fn i_055314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5531A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5531A);
    cpu.pc = 0x5022A;
    16
}
/// $05531A: rts
pub fn i_05531a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5531C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05531c  ($05531C, 14 B, kernel/state_dispatch_stubs.c)
/// $05531C: lea.l $29a67e.l, a2
pub fn i_05531c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55322;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A67E;
    20
}
/// $055322: jsr $5022a.l
pub fn i_055322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55328);
    cpu.pc = 0x5022A;
    16
}
/// $055328: rts
pub fn i_055328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5532A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05532a  ($05532A, 14 B, kernel/state_dispatch_stubs.c)
/// $05532A: lea.l $29a692.l, a2
pub fn i_05532a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55330;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A692;
    20
}
/// $055330: jsr $5022a.l
pub fn i_055330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55336);
    cpu.pc = 0x5022A;
    16
}
/// $055336: rts
pub fn i_055336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055338  ($055338, 14 B, kernel/state_dispatch_stubs.c)
/// $055338: lea.l $29a6a6.l, a2
pub fn i_055338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5533E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6A6;
    20
}
/// $05533E: jsr $5022a.l
pub fn i_05533e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55344;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55344);
    cpu.pc = 0x5022A;
    16
}
/// $055344: rts
pub fn i_055344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055346  ($055346, 14 B, kernel/state_dispatch_stubs.c)
/// $055346: lea.l $29a6ba.l, a2
pub fn i_055346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5534C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6BA;
    20
}
/// $05534C: jsr $5022a.l
pub fn i_05534c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55352);
    cpu.pc = 0x5022A;
    16
}
/// $055352: rts
pub fn i_055352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55354;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055354  ($055354, 14 B, kernel/state_dispatch_stubs.c)
/// $055354: lea.l $29a6ce.l, a2
pub fn i_055354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5535A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6CE;
    20
}
/// $05535A: jsr $5022a.l
pub fn i_05535a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55360);
    cpu.pc = 0x5022A;
    16
}
/// $055360: rts
pub fn i_055360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55362;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055362  ($055362, 14 B, kernel/state_dispatch_stubs.c)
/// $055362: lea.l $29a6e2.l, a2
pub fn i_055362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55368;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6E2;
    20
}
/// $055368: jsr $5022a.l
pub fn i_055368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5536E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5536E);
    cpu.pc = 0x5022A;
    16
}
/// $05536E: rts
pub fn i_05536e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55370;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055370  ($055370, 14 B, kernel/state_dispatch_stubs.c)
/// $055370: lea.l $29a6f6.l, a2
pub fn i_055370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55376;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6F6;
    20
}
/// $055376: jsr $5022a.l
pub fn i_055376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5537C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5537C);
    cpu.pc = 0x5022A;
    16
}
/// $05537C: rts
pub fn i_05537c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5537E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05537e  ($05537E, 14 B, kernel/state_dispatch_stubs.c)
/// $05537E: lea.l $29a70a.l, a2
pub fn i_05537e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55384;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A70A;
    20
}
/// $055384: jsr $5022a.l
pub fn i_055384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5538A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5538A);
    cpu.pc = 0x5022A;
    16
}
/// $05538A: rts
pub fn i_05538a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5538C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05538c  ($05538C, 14 B, kernel/state_dispatch_stubs.c)
/// $05538C: lea.l $29a71e.l, a2
pub fn i_05538c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55392;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A71E;
    20
}
/// $055392: jsr $5022a.l
pub fn i_055392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55398;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55398);
    cpu.pc = 0x5022A;
    16
}
/// $055398: rts
pub fn i_055398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5539A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05539a  ($05539A, 14 B, kernel/state_dispatch_stubs.c)
/// $05539A: lea.l $29a732.l, a2
pub fn i_05539a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553A0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A732;
    20
}
/// $0553A0: jsr $5022a.l
pub fn i_0553a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553A6);
    cpu.pc = 0x5022A;
    16
}
/// $0553A6: rts
pub fn i_0553a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553a8  ($0553A8, 14 B, kernel/state_dispatch_stubs.c)
/// $0553A8: lea.l $29a746.l, a2
pub fn i_0553a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553AE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A746;
    20
}
/// $0553AE: jsr $5022a.l
pub fn i_0553ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553B4);
    cpu.pc = 0x5022A;
    16
}
/// $0553B4: rts
pub fn i_0553b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553b6  ($0553B6, 14 B, kernel/state_dispatch_stubs.c)
/// $0553B6: lea.l $29a75a.l, a2
pub fn i_0553b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553BC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A75A;
    20
}
/// $0553BC: jsr $5022a.l
pub fn i_0553bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553C2);
    cpu.pc = 0x5022A;
    16
}
/// $0553C2: rts
pub fn i_0553c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553c4  ($0553C4, 14 B, kernel/state_dispatch_stubs.c)
/// $0553C4: lea.l $29a76e.l, a2
pub fn i_0553c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553CA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A76E;
    20
}
/// $0553CA: jsr $5022a.l
pub fn i_0553ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553D0);
    cpu.pc = 0x5022A;
    16
}
/// $0553D0: rts
pub fn i_0553d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553d2  ($0553D2, 14 B, kernel/state_dispatch_stubs.c)
/// $0553D2: lea.l $29a782.l, a2
pub fn i_0553d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553D8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A782;
    20
}
/// $0553D8: jsr $5022a.l
pub fn i_0553d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553DE);
    cpu.pc = 0x5022A;
    16
}
/// $0553DE: rts
pub fn i_0553de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553e0  ($0553E0, 14 B, kernel/state_dispatch_stubs.c)
/// $0553E0: lea.l $29a796.l, a2
pub fn i_0553e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553E6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A796;
    20
}
/// $0553E6: jsr $5022a.l
pub fn i_0553e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553EC);
    cpu.pc = 0x5022A;
    16
}
/// $0553EC: rts
pub fn i_0553ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553ee  ($0553EE, 14 B, kernel/state_dispatch_stubs.c)
/// $0553EE: lea.l $29a7aa.l, a2
pub fn i_0553ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553F4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7AA;
    20
}
/// $0553F4: jsr $5022a.l
pub fn i_0553f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553FA);
    cpu.pc = 0x5022A;
    16
}
/// $0553FA: rts
pub fn i_0553fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553fc  ($0553FC, 14 B, kernel/state_dispatch_stubs.c)
/// $0553FC: lea.l $29a7be.l, a2
pub fn i_0553fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55402;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7BE;
    20
}
/// $055402: jsr $5022a.l
pub fn i_055402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55408);
    cpu.pc = 0x5022A;
    16
}
/// $055408: rts
pub fn i_055408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5540A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05540a  ($05540A, 14 B, kernel/state_dispatch_stubs.c)
/// $05540A: lea.l $29a7d2.l, a2
pub fn i_05540a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55410;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7D2;
    20
}
/// $055410: jsr $5022a.l
pub fn i_055410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55416);
    cpu.pc = 0x5022A;
    16
}
/// $055416: rts
pub fn i_055416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55418;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055418  ($055418, 14 B, kernel/state_dispatch_stubs.c)
/// $055418: lea.l $29a7e6.l, a2
pub fn i_055418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5541E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7E6;
    20
}
/// $05541E: jsr $5022a.l
pub fn i_05541e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55424;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55424);
    cpu.pc = 0x5022A;
    16
}
/// $055424: rts
pub fn i_055424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55426;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055426  ($055426, 14 B, kernel/state_dispatch_stubs.c)
/// $055426: lea.l $29a7fa.l, a2
pub fn i_055426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5542C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7FA;
    20
}
/// $05542C: jsr $5022a.l
pub fn i_05542c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55432);
    cpu.pc = 0x5022A;
    16
}
/// $055432: rts
pub fn i_055432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55434;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055434  ($055434, 14 B, kernel/state_dispatch_stubs.c)
/// $055434: lea.l $29a80e.l, a2
pub fn i_055434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5543A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A80E;
    20
}
/// $05543A: jsr $5022a.l
pub fn i_05543a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55440);
    cpu.pc = 0x5022A;
    16
}
/// $055440: rts
pub fn i_055440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55442;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055442  ($055442, 14 B, kernel/state_dispatch_stubs.c)
/// $055442: lea.l $29a822.l, a2
pub fn i_055442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55448;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A822;
    20
}
/// $055448: jsr $5022a.l
pub fn i_055448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5544E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5544E);
    cpu.pc = 0x5022A;
    16
}
/// $05544E: rts
pub fn i_05544e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55450;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055450  ($055450, 14 B, kernel/state_dispatch_stubs.c)
/// $055450: lea.l $29a836.l, a2
pub fn i_055450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55456;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A836;
    20
}
/// $055456: jsr $5022a.l
pub fn i_055456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5545C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5545C);
    cpu.pc = 0x5022A;
    16
}
/// $05545C: rts
pub fn i_05545c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5545E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05545e  ($05545E, 14 B, kernel/state_dispatch_stubs.c)
/// $05545E: lea.l $29a84a.l, a2
pub fn i_05545e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55464;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A84A;
    20
}
/// $055464: jsr $5022a.l
pub fn i_055464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5546A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5546A);
    cpu.pc = 0x5022A;
    16
}
/// $05546A: rts
pub fn i_05546a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5546C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05546c  ($05546C, 14 B, kernel/state_dispatch_stubs.c)
/// $05546C: lea.l $29a85e.l, a2
pub fn i_05546c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55472;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A85E;
    20
}
/// $055472: jsr $5022a.l
pub fn i_055472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55478;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55478);
    cpu.pc = 0x5022A;
    16
}
/// $055478: rts
pub fn i_055478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5547A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05547a  ($05547A, 14 B, kernel/state_dispatch_stubs.c)
/// $05547A: lea.l $29a872.l, a2
pub fn i_05547a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55480;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A872;
    20
}
/// $055480: jsr $5022a.l
pub fn i_055480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55486);
    cpu.pc = 0x5022A;
    16
}
/// $055486: rts
pub fn i_055486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55488;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055488  ($055488, 14 B, kernel/state_dispatch_stubs.c)
/// $055488: lea.l $29a886.l, a2
pub fn i_055488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5548E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A886;
    20
}
/// $05548E: jsr $5022a.l
pub fn i_05548e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55494);
    cpu.pc = 0x5022A;
    16
}
/// $055494: rts
pub fn i_055494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55496;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055496  ($055496, 14 B, kernel/state_dispatch_stubs.c)
/// $055496: lea.l $29a89a.l, a2
pub fn i_055496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5549C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A89A;
    20
}
/// $05549C: jsr $5022a.l
pub fn i_05549c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554A2);
    cpu.pc = 0x5022A;
    16
}
/// $0554A2: rts
pub fn i_0554a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554a4  ($0554A4, 14 B, kernel/state_dispatch_stubs.c)
/// $0554A4: lea.l $29a8ae.l, a2
pub fn i_0554a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554AA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8AE;
    20
}
/// $0554AA: jsr $5022a.l
pub fn i_0554aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554B0);
    cpu.pc = 0x5022A;
    16
}
/// $0554B0: rts
pub fn i_0554b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554b2  ($0554B2, 14 B, kernel/state_dispatch_stubs.c)
/// $0554B2: lea.l $29a8c2.l, a2
pub fn i_0554b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554B8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8C2;
    20
}
/// $0554B8: jsr $5022a.l
pub fn i_0554b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554BE);
    cpu.pc = 0x5022A;
    16
}
/// $0554BE: rts
pub fn i_0554be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554c0  ($0554C0, 14 B, kernel/state_dispatch_stubs.c)
/// $0554C0: lea.l $29a8d6.l, a2
pub fn i_0554c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554C6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8D6;
    20
}
/// $0554C6: jsr $5022a.l
pub fn i_0554c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554CC);
    cpu.pc = 0x5022A;
    16
}
/// $0554CC: rts
pub fn i_0554cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554ce  ($0554CE, 14 B, kernel/state_dispatch_stubs.c)
/// $0554CE: lea.l $29a8ea.l, a2
pub fn i_0554ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554D4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8EA;
    20
}
/// $0554D4: jsr $5022a.l
pub fn i_0554d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554DA);
    cpu.pc = 0x5022A;
    16
}
/// $0554DA: rts
pub fn i_0554da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554dc  ($0554DC, 14 B, kernel/state_dispatch_stubs.c)
/// $0554DC: lea.l $29a8fe.l, a2
pub fn i_0554dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554E2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8FE;
    20
}
/// $0554E2: jsr $5022a.l
pub fn i_0554e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554E8);
    cpu.pc = 0x5022A;
    16
}
/// $0554E8: rts
pub fn i_0554e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554ea  ($0554EA, 14 B, kernel/state_dispatch_stubs.c)
/// $0554EA: lea.l $29a912.l, a2
pub fn i_0554ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554F0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A912;
    20
}
/// $0554F0: jsr $5022a.l
pub fn i_0554f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554F6);
    cpu.pc = 0x5022A;
    16
}
/// $0554F6: rts
pub fn i_0554f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554f8  ($0554F8, 14 B, kernel/state_dispatch_stubs.c)
/// $0554F8: lea.l $29a926.l, a2
pub fn i_0554f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554FE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A926;
    20
}
/// $0554FE: jsr $5022a.l
pub fn i_0554fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55504);
    cpu.pc = 0x5022A;
    16
}
/// $055504: rts
pub fn i_055504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55506;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055506  ($055506, 14 B, kernel/state_dispatch_stubs.c)
/// $055506: lea.l $29a93a.l, a2
pub fn i_055506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5550C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A93A;
    20
}
/// $05550C: jsr $5022a.l
pub fn i_05550c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55512);
    cpu.pc = 0x5022A;
    16
}
/// $055512: rts
pub fn i_055512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055514  ($055514, 14 B, kernel/state_dispatch_stubs.c)
/// $055514: lea.l $29a94e.l, a2
pub fn i_055514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5551A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A94E;
    20
}
/// $05551A: jsr $5022a.l
pub fn i_05551a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55520;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55520);
    cpu.pc = 0x5022A;
    16
}
/// $055520: rts
pub fn i_055520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55522;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055522  ($055522, 14 B, kernel/state_dispatch_stubs.c)
/// $055522: lea.l $29a962.l, a2
pub fn i_055522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55528;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A962;
    20
}
/// $055528: jsr $5022a.l
pub fn i_055528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5552E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5552E);
    cpu.pc = 0x5022A;
    16
}
/// $05552E: rts
pub fn i_05552e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55530;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055530  ($055530, 14 B, kernel/state_dispatch_stubs.c)
/// $055530: lea.l $29a976.l, a2
pub fn i_055530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55536;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A976;
    20
}
/// $055536: jsr $5022a.l
pub fn i_055536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5553C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5553C);
    cpu.pc = 0x5022A;
    16
}
/// $05553C: rts
pub fn i_05553c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5553E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05553e  ($05553E, 14 B, kernel/state_dispatch_stubs.c)
/// $05553E: lea.l $29a98a.l, a2
pub fn i_05553e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55544;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A98A;
    20
}
/// $055544: jsr $5022a.l
pub fn i_055544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5554A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5554A);
    cpu.pc = 0x5022A;
    16
}
/// $05554A: rts
pub fn i_05554a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5554C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05554c  ($05554C, 14 B, kernel/state_dispatch_stubs.c)
/// $05554C: lea.l $29a99e.l, a2
pub fn i_05554c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55552;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A99E;
    20
}
/// $055552: jsr $5022a.l
pub fn i_055552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55558);
    cpu.pc = 0x5022A;
    16
}
/// $055558: rts
pub fn i_055558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5555A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05555a  ($05555A, 14 B, kernel/state_dispatch_stubs.c)
/// $05555A: lea.l $29a9b2.l, a2
pub fn i_05555a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55560;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A9B2;
    20
}
/// $055560: jsr $5022a.l
pub fn i_055560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55566;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55566);
    cpu.pc = 0x5022A;
    16
}
/// $055566: rts
pub fn i_055566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55568;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055568  ($055568, 14 B, kernel/state_dispatch_stubs.c)
/// $055568: lea.l $29a9c6.l, a2
pub fn i_055568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5556E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A9C6;
    20
}
/// $05556E: jsr $5022a.l
pub fn i_05556e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55574);
    cpu.pc = 0x5022A;
    16
}
/// $055574: rts
pub fn i_055574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55576;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055576  ($055576, 14 B, kernel/state_dispatch_stubs.c)
/// $055576: lea.l $29a9da.l, a2
pub fn i_055576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5557C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A9DA;
    20
}
/// $05557C: jsr $5022a.l
pub fn i_05557c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55582;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55582);
    cpu.pc = 0x5022A;
    16
}
/// $055582: rts
pub fn i_055582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55584;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055584  ($055584, 14 B, kernel/state_dispatch_stubs.c)
/// $055584: lea.l $29a9ee.l, a2
pub fn i_055584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5558A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A9EE;
    20
}
/// $05558A: jsr $5022a.l
pub fn i_05558a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55590);
    cpu.pc = 0x5022A;
    16
}
/// $055590: rts
pub fn i_055590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55592;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055592  ($055592, 14 B, kernel/state_dispatch_stubs.c)
/// $055592: lea.l $29aa02.l, a2
pub fn i_055592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55598;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA02;
    20
}
/// $055598: jsr $5022a.l
pub fn i_055598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5559E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5559E);
    cpu.pc = 0x5022A;
    16
}
/// $05559E: rts
pub fn i_05559e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555a0  ($0555A0, 14 B, kernel/state_dispatch_stubs.c)
/// $0555A0: lea.l $29aa16.l, a2
pub fn i_0555a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555A6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA16;
    20
}
/// $0555A6: jsr $5022a.l
pub fn i_0555a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555AC);
    cpu.pc = 0x5022A;
    16
}
/// $0555AC: rts
pub fn i_0555ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555ae  ($0555AE, 14 B, kernel/state_dispatch_stubs.c)
/// $0555AE: lea.l $29aa2a.l, a2
pub fn i_0555ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555B4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA2A;
    20
}
/// $0555B4: jsr $5022a.l
pub fn i_0555b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555BA);
    cpu.pc = 0x5022A;
    16
}
/// $0555BA: rts
pub fn i_0555ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555bc  ($0555BC, 14 B, kernel/state_dispatch_stubs.c)
/// $0555BC: lea.l $29aa3e.l, a2
pub fn i_0555bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555C2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA3E;
    20
}
/// $0555C2: jsr $5022a.l
pub fn i_0555c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555C8);
    cpu.pc = 0x5022A;
    16
}
/// $0555C8: rts
pub fn i_0555c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555ca  ($0555CA, 14 B, kernel/state_dispatch_stubs.c)
/// $0555CA: lea.l $29aa52.l, a2
pub fn i_0555ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555D0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA52;
    20
}
/// $0555D0: jsr $5022a.l
pub fn i_0555d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555D6);
    cpu.pc = 0x5022A;
    16
}
/// $0555D6: rts
pub fn i_0555d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555d8  ($0555D8, 14 B, kernel/state_dispatch_stubs.c)
/// $0555D8: lea.l $29aa66.l, a2
pub fn i_0555d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA66;
    20
}
/// $0555DE: jsr $5022a.l
pub fn i_0555de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555E4);
    cpu.pc = 0x5022A;
    16
}
/// $0555E4: rts
pub fn i_0555e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555e6  ($0555E6, 14 B, kernel/state_dispatch_stubs.c)
/// $0555E6: lea.l $29aa7a.l, a2
pub fn i_0555e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555EC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA7A;
    20
}
/// $0555EC: jsr $5022a.l
pub fn i_0555ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555F2);
    cpu.pc = 0x5022A;
    16
}
/// $0555F2: rts
pub fn i_0555f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555f4  ($0555F4, 14 B, kernel/state_dispatch_stubs.c)
/// $0555F4: lea.l $29aa8e.l, a2
pub fn i_0555f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555FA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA8E;
    20
}
/// $0555FA: jsr $5022a.l
pub fn i_0555fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55600);
    cpu.pc = 0x5022A;
    16
}
/// $055600: rts
pub fn i_055600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55602;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055602  ($055602, 14 B, kernel/state_dispatch_stubs.c)
/// $055602: lea.l $29aaa2.l, a2
pub fn i_055602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55608;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AAA2;
    20
}
/// $055608: jsr $5022a.l
pub fn i_055608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5560E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5560E);
    cpu.pc = 0x5022A;
    16
}
/// $05560E: rts
pub fn i_05560e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55610;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055610  ($055610, 14 B, kernel/state_dispatch_stubs.c)
/// $055610: lea.l $29aab6.l, a2
pub fn i_055610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55616;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AAB6;
    20
}
/// $055616: jsr $5022a.l
pub fn i_055616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5561C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5561C);
    cpu.pc = 0x5022A;
    16
}
/// $05561C: rts
pub fn i_05561c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5561E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05561e  ($05561E, 14 B, kernel/state_dispatch_stubs.c)
/// $05561E: lea.l $29aaca.l, a2
pub fn i_05561e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55624;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AACA;
    20
}
/// $055624: jsr $5022a.l
pub fn i_055624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5562A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5562A);
    cpu.pc = 0x5022A;
    16
}
/// $05562A: rts
pub fn i_05562a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5562C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05562c  ($05562C, 14 B, kernel/state_dispatch_stubs.c)
/// $05562C: lea.l $29aade.l, a2
pub fn i_05562c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55632;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AADE;
    20
}
/// $055632: jsr $5022a.l
pub fn i_055632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55638;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55638);
    cpu.pc = 0x5022A;
    16
}
/// $055638: rts
pub fn i_055638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5563A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05563a  ($05563A, 14 B, kernel/state_dispatch_stubs.c)
/// $05563A: lea.l $29aaf2.l, a2
pub fn i_05563a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55640;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AAF2;
    20
}
/// $055640: jsr $5022a.l
pub fn i_055640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55646);
    cpu.pc = 0x5022A;
    16
}
/// $055646: rts
pub fn i_055646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55648;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055648  ($055648, 14 B, kernel/state_dispatch_stubs.c)
/// $055648: lea.l $29ab06.l, a2
pub fn i_055648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5564E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB06;
    20
}
/// $05564E: jsr $5022a.l
pub fn i_05564e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55654;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55654);
    cpu.pc = 0x5022A;
    16
}
/// $055654: rts
pub fn i_055654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55656;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055656  ($055656, 14 B, kernel/state_dispatch_stubs.c)
/// $055656: lea.l $29ab1a.l, a2
pub fn i_055656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5565C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB1A;
    20
}
/// $05565C: jsr $5022a.l
pub fn i_05565c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55662);
    cpu.pc = 0x5022A;
    16
}
/// $055662: rts
pub fn i_055662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55664;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055664  ($055664, 14 B, kernel/state_dispatch_stubs.c)
/// $055664: lea.l $29ab2e.l, a2
pub fn i_055664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5566A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB2E;
    20
}
/// $05566A: jsr $5022a.l
pub fn i_05566a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55670;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55670);
    cpu.pc = 0x5022A;
    16
}
/// $055670: rts
pub fn i_055670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55672;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055672  ($055672, 14 B, kernel/state_dispatch_stubs.c)
/// $055672: lea.l $29ab42.l, a2
pub fn i_055672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55678;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB42;
    20
}
/// $055678: jsr $5022a.l
pub fn i_055678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5567E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5567E);
    cpu.pc = 0x5022A;
    16
}
/// $05567E: rts
pub fn i_05567e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55680;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055680  ($055680, 14 B, kernel/state_dispatch_stubs.c)
/// $055680: lea.l $29ab56.l, a2
pub fn i_055680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55686;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB56;
    20
}
/// $055686: jsr $5022a.l
pub fn i_055686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5568C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5568C);
    cpu.pc = 0x5022A;
    16
}
/// $05568C: rts
pub fn i_05568c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5568E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05568e  ($05568E, 14 B, kernel/state_dispatch_stubs.c)
/// $05568E: lea.l $29ab6a.l, a2
pub fn i_05568e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55694;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB6A;
    20
}
/// $055694: jsr $5022a.l
pub fn i_055694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5569A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5569A);
    cpu.pc = 0x5022A;
    16
}
/// $05569A: rts
pub fn i_05569a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5569C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05569c  ($05569C, 14 B, kernel/state_dispatch_stubs.c)
/// $05569C: lea.l $29ab7e.l, a2
pub fn i_05569c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556A2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB7E;
    20
}
/// $0556A2: jsr $5022a.l
pub fn i_0556a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556A8);
    cpu.pc = 0x5022A;
    16
}
/// $0556A8: rts
pub fn i_0556a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556aa  ($0556AA, 14 B, kernel/state_dispatch_stubs.c)
/// $0556AA: lea.l $29ab92.l, a2
pub fn i_0556aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556B0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB92;
    20
}
/// $0556B0: jsr $5022a.l
pub fn i_0556b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556B6);
    cpu.pc = 0x5022A;
    16
}
/// $0556B6: rts
pub fn i_0556b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556b8  ($0556B8, 14 B, kernel/state_dispatch_stubs.c)
/// $0556B8: lea.l $29aba6.l, a2
pub fn i_0556b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556BE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABA6;
    20
}
/// $0556BE: jsr $5022a.l
pub fn i_0556be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556C4);
    cpu.pc = 0x5022A;
    16
}
/// $0556C4: rts
pub fn i_0556c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556c6  ($0556C6, 14 B, kernel/state_dispatch_stubs.c)
/// $0556C6: lea.l $29abba.l, a2
pub fn i_0556c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556CC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABBA;
    20
}
/// $0556CC: jsr $5022a.l
pub fn i_0556cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556D2);
    cpu.pc = 0x5022A;
    16
}
/// $0556D2: rts
pub fn i_0556d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556d4  ($0556D4, 14 B, kernel/state_dispatch_stubs.c)
/// $0556D4: lea.l $29abce.l, a2
pub fn i_0556d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556DA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABCE;
    20
}
/// $0556DA: jsr $5022a.l
pub fn i_0556da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556E0);
    cpu.pc = 0x5022A;
    16
}
/// $0556E0: rts
pub fn i_0556e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556e2  ($0556E2, 14 B, kernel/state_dispatch_stubs.c)
/// $0556E2: lea.l $29abe2.l, a2
pub fn i_0556e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556E8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABE2;
    20
}
/// $0556E8: jsr $5022a.l
pub fn i_0556e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556EE);
    cpu.pc = 0x5022A;
    16
}
/// $0556EE: rts
pub fn i_0556ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556f0  ($0556F0, 14 B, kernel/state_dispatch_stubs.c)
/// $0556F0: lea.l $29abf6.l, a2
pub fn i_0556f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556F6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABF6;
    20
}
/// $0556F6: jsr $5022a.l
pub fn i_0556f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556FC);
    cpu.pc = 0x5022A;
    16
}
/// $0556FC: rts
pub fn i_0556fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556fe  ($0556FE, 14 B, kernel/state_dispatch_stubs.c)
/// $0556FE: lea.l $29ac0a.l, a2
pub fn i_0556fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55704;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC0A;
    20
}
/// $055704: jsr $5022a.l
pub fn i_055704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5570A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5570A);
    cpu.pc = 0x5022A;
    16
}
/// $05570A: rts
pub fn i_05570a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5570C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05570c  ($05570C, 14 B, kernel/state_dispatch_stubs.c)
/// $05570C: lea.l $29ac1e.l, a2
pub fn i_05570c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55712;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC1E;
    20
}
/// $055712: jsr $5022a.l
pub fn i_055712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55718);
    cpu.pc = 0x5022A;
    16
}
/// $055718: rts
pub fn i_055718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5571A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05571a  ($05571A, 14 B, kernel/state_dispatch_stubs.c)
/// $05571A: lea.l $29ac32.l, a2
pub fn i_05571a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55720;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC32;
    20
}
/// $055720: jsr $5022a.l
pub fn i_055720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55726;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55726);
    cpu.pc = 0x5022A;
    16
}
/// $055726: rts
pub fn i_055726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55728;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055728  ($055728, 14 B, kernel/state_dispatch_stubs.c)
/// $055728: lea.l $29ac46.l, a2
pub fn i_055728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5572E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC46;
    20
}
/// $05572E: jsr $5022a.l
pub fn i_05572e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55734);
    cpu.pc = 0x5022A;
    16
}
/// $055734: rts
pub fn i_055734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55736;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055736  ($055736, 14 B, kernel/state_dispatch_stubs.c)
/// $055736: lea.l $29ac5a.l, a2
pub fn i_055736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5573C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC5A;
    20
}
/// $05573C: jsr $5022a.l
pub fn i_05573c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55742);
    cpu.pc = 0x5022A;
    16
}
/// $055742: rts
pub fn i_055742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55744;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055744  ($055744, 14 B, kernel/state_dispatch_stubs.c)
/// $055744: lea.l $29ac6e.l, a2
pub fn i_055744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5574A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC6E;
    20
}
/// $05574A: jsr $5022a.l
pub fn i_05574a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55750);
    cpu.pc = 0x5022A;
    16
}
/// $055750: rts
pub fn i_055750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55752;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055752  ($055752, 14 B, kernel/state_dispatch_stubs.c)
/// $055752: lea.l $29ac82.l, a2
pub fn i_055752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55758;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC82;
    20
}
/// $055758: jsr $5022a.l
pub fn i_055758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5575E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5575E);
    cpu.pc = 0x5022A;
    16
}
/// $05575E: rts
pub fn i_05575e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55760;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055760  ($055760, 14 B, kernel/state_dispatch_stubs.c)
/// $055760: lea.l $29ac96.l, a2
pub fn i_055760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55766;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC96;
    20
}
/// $055766: jsr $5022a.l
pub fn i_055766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5576C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5576C);
    cpu.pc = 0x5022A;
    16
}
/// $05576C: rts
pub fn i_05576c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5576E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05576e  ($05576E, 14 B, kernel/state_dispatch_stubs.c)
/// $05576E: lea.l $29acaa.l, a2
pub fn i_05576e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55774;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACAA;
    20
}
/// $055774: jsr $5022a.l
pub fn i_055774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5577A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5577A);
    cpu.pc = 0x5022A;
    16
}
/// $05577A: rts
pub fn i_05577a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5577C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05577c  ($05577C, 14 B, kernel/state_dispatch_stubs.c)
/// $05577C: lea.l $29acbe.l, a2
pub fn i_05577c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55782;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACBE;
    20
}
/// $055782: jsr $5022a.l
pub fn i_055782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55788);
    cpu.pc = 0x5022A;
    16
}
/// $055788: rts
pub fn i_055788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5578A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05578a  ($05578A, 14 B, kernel/state_dispatch_stubs.c)
/// $05578A: lea.l $29acd2.l, a2
pub fn i_05578a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55790;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACD2;
    20
}
/// $055790: jsr $5022a.l
pub fn i_055790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55796;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55796);
    cpu.pc = 0x5022A;
    16
}
/// $055796: rts
pub fn i_055796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55798;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055798  ($055798, 14 B, kernel/state_dispatch_stubs.c)
/// $055798: lea.l $29ace6.l, a2
pub fn i_055798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5579E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACE6;
    20
}
/// $05579E: jsr $5022a.l
pub fn i_05579e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557A4);
    cpu.pc = 0x5022A;
    16
}
/// $0557A4: rts
pub fn i_0557a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557a6  ($0557A6, 14 B, kernel/state_dispatch_stubs.c)
/// $0557A6: lea.l $29acfa.l, a2
pub fn i_0557a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557AC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACFA;
    20
}
/// $0557AC: jsr $5022a.l
pub fn i_0557ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557B2);
    cpu.pc = 0x5022A;
    16
}
/// $0557B2: rts
pub fn i_0557b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557b4  ($0557B4, 14 B, kernel/state_dispatch_stubs.c)
/// $0557B4: lea.l $29ad0e.l, a2
pub fn i_0557b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557BA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD0E;
    20
}
/// $0557BA: jsr $5022a.l
pub fn i_0557ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557C0);
    cpu.pc = 0x5022A;
    16
}
/// $0557C0: rts
pub fn i_0557c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557c2  ($0557C2, 14 B, kernel/state_dispatch_stubs.c)
/// $0557C2: lea.l $29ad22.l, a2
pub fn i_0557c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557C8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD22;
    20
}
/// $0557C8: jsr $5022a.l
pub fn i_0557c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557CE);
    cpu.pc = 0x5022A;
    16
}
/// $0557CE: rts
pub fn i_0557ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557d0  ($0557D0, 14 B, kernel/state_dispatch_stubs.c)
/// $0557D0: lea.l $29ad36.l, a2
pub fn i_0557d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557D6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD36;
    20
}
/// $0557D6: jsr $5022a.l
pub fn i_0557d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557DC);
    cpu.pc = 0x5022A;
    16
}
/// $0557DC: rts
pub fn i_0557dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557de  ($0557DE, 14 B, kernel/state_dispatch_stubs.c)
/// $0557DE: lea.l $29ad4a.l, a2
pub fn i_0557de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557E4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD4A;
    20
}
/// $0557E4: jsr $5022a.l
pub fn i_0557e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557EA);
    cpu.pc = 0x5022A;
    16
}
/// $0557EA: rts
pub fn i_0557ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557ec  ($0557EC, 14 B, kernel/state_dispatch_stubs.c)
/// $0557EC: lea.l $29ad5e.l, a2
pub fn i_0557ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557F2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD5E;
    20
}
/// $0557F2: jsr $5022a.l
pub fn i_0557f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557F8);
    cpu.pc = 0x5022A;
    16
}
/// $0557F8: rts
pub fn i_0557f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557fa  ($0557FA, 14 B, kernel/state_dispatch_stubs.c)
/// $0557FA: lea.l $29ad72.l, a2
pub fn i_0557fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55800;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD72;
    20
}
/// $055800: jsr $5022a.l
pub fn i_055800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55806);
    cpu.pc = 0x5022A;
    16
}
/// $055806: rts
pub fn i_055806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55808;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055808  ($055808, 14 B, kernel/state_dispatch_stubs.c)
/// $055808: lea.l $29ad86.l, a2
pub fn i_055808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5580E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD86;
    20
}
/// $05580E: jsr $5022a.l
pub fn i_05580e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55814);
    cpu.pc = 0x5022A;
    16
}
/// $055814: rts
pub fn i_055814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55816;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055816  ($055816, 14 B, kernel/state_dispatch_stubs.c)
/// $055816: lea.l $29ad9a.l, a2
pub fn i_055816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5581C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD9A;
    20
}
/// $05581C: jsr $5022a.l
pub fn i_05581c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55822);
    cpu.pc = 0x5022A;
    16
}
/// $055822: rts
pub fn i_055822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55824;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055824  ($055824, 14 B, kernel/state_dispatch_stubs.c)
/// $055824: lea.l $29adae.l, a2
pub fn i_055824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5582A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADAE;
    20
}
/// $05582A: jsr $5022a.l
pub fn i_05582a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55830);
    cpu.pc = 0x5022A;
    16
}
/// $055830: rts
pub fn i_055830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55832;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055832  ($055832, 14 B, kernel/state_dispatch_stubs.c)
/// $055832: lea.l $29adc2.l, a2
pub fn i_055832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55838;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADC2;
    20
}
/// $055838: jsr $5022a.l
pub fn i_055838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5583E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5583E);
    cpu.pc = 0x5022A;
    16
}
/// $05583E: rts
pub fn i_05583e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55840;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055840  ($055840, 14 B, kernel/state_dispatch_stubs.c)
/// $055840: lea.l $29add6.l, a2
pub fn i_055840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55846;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADD6;
    20
}
/// $055846: jsr $5022a.l
pub fn i_055846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5584C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5584C);
    cpu.pc = 0x5022A;
    16
}
/// $05584C: rts
pub fn i_05584c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5584E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05584e  ($05584E, 14 B, kernel/state_dispatch_stubs.c)
/// $05584E: lea.l $29adea.l, a2
pub fn i_05584e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55854;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADEA;
    20
}
/// $055854: jsr $5022a.l
pub fn i_055854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5585A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5585A);
    cpu.pc = 0x5022A;
    16
}
/// $05585A: rts
pub fn i_05585a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5585C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05585c  ($05585C, 14 B, kernel/state_dispatch_stubs.c)
/// $05585C: lea.l $29adfe.l, a2
pub fn i_05585c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55862;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADFE;
    20
}
/// $055862: jsr $5022a.l
pub fn i_055862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55868);
    cpu.pc = 0x5022A;
    16
}
/// $055868: rts
pub fn i_055868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5586A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05586a  ($05586A, 14 B, kernel/state_dispatch_stubs.c)
/// $05586A: lea.l $29ae12.l, a2
pub fn i_05586a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55870;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE12;
    20
}
/// $055870: jsr $5022a.l
pub fn i_055870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55876);
    cpu.pc = 0x5022A;
    16
}
/// $055876: rts
pub fn i_055876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55878;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055878  ($055878, 14 B, kernel/state_dispatch_stubs.c)
/// $055878: lea.l $29ae26.l, a2
pub fn i_055878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5587E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE26;
    20
}
/// $05587E: jsr $5022a.l
pub fn i_05587e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55884);
    cpu.pc = 0x5022A;
    16
}
/// $055884: rts
pub fn i_055884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055886  ($055886, 14 B, kernel/state_dispatch_stubs.c)
/// $055886: lea.l $29ae3a.l, a2
pub fn i_055886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5588C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE3A;
    20
}
/// $05588C: jsr $5022a.l
pub fn i_05588c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55892);
    cpu.pc = 0x5022A;
    16
}
/// $055892: rts
pub fn i_055892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55894;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055894  ($055894, 14 B, kernel/state_dispatch_stubs.c)
/// $055894: lea.l $29ae4e.l, a2
pub fn i_055894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5589A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE4E;
    20
}
/// $05589A: jsr $5022a.l
pub fn i_05589a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558A0);
    cpu.pc = 0x5022A;
    16
}
/// $0558A0: rts
pub fn i_0558a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558a2  ($0558A2, 14 B, kernel/state_dispatch_stubs.c)
/// $0558A2: lea.l $29ae62.l, a2
pub fn i_0558a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558A8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE62;
    20
}
/// $0558A8: jsr $5022a.l
pub fn i_0558a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558AE);
    cpu.pc = 0x5022A;
    16
}
/// $0558AE: rts
pub fn i_0558ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558b0  ($0558B0, 14 B, kernel/state_dispatch_stubs.c)
/// $0558B0: lea.l $29ae76.l, a2
pub fn i_0558b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558B6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE76;
    20
}
/// $0558B6: jsr $5022a.l
pub fn i_0558b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558BC);
    cpu.pc = 0x5022A;
    16
}
/// $0558BC: rts
pub fn i_0558bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558be  ($0558BE, 14 B, kernel/state_dispatch_stubs.c)
/// $0558BE: lea.l $29ae8a.l, a2
pub fn i_0558be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558C4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE8A;
    20
}
/// $0558C4: jsr $5022a.l
pub fn i_0558c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558CA);
    cpu.pc = 0x5022A;
    16
}
/// $0558CA: rts
pub fn i_0558ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558cc  ($0558CC, 14 B, kernel/state_dispatch_stubs.c)
/// $0558CC: lea.l $29ae9e.l, a2
pub fn i_0558cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558D2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE9E;
    20
}
/// $0558D2: jsr $5022a.l
pub fn i_0558d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558D8);
    cpu.pc = 0x5022A;
    16
}
/// $0558D8: rts
pub fn i_0558d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558da  ($0558DA, 14 B, kernel/state_dispatch_stubs.c)
/// $0558DA: lea.l $29aeb2.l, a2
pub fn i_0558da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558E0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AEB2;
    20
}
/// $0558E0: jsr $5022a.l
pub fn i_0558e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558E6);
    cpu.pc = 0x5022A;
    16
}
/// $0558E6: rts
pub fn i_0558e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558e8  ($0558E8, 14 B, kernel/state_dispatch_stubs.c)
/// $0558E8: lea.l $29aec6.l, a2
pub fn i_0558e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558EE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AEC6;
    20
}
/// $0558EE: jsr $5022a.l
pub fn i_0558ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558F4);
    cpu.pc = 0x5022A;
    16
}
/// $0558F4: rts
pub fn i_0558f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558f6  ($0558F6, 14 B, kernel/state_dispatch_stubs.c)
/// $0558F6: lea.l $29aeda.l, a2
pub fn i_0558f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558FC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AEDA;
    20
}
/// $0558FC: jsr $5022a.l
pub fn i_0558fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55902;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55902);
    cpu.pc = 0x5022A;
    16
}
/// $055902: rts
pub fn i_055902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55904;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055904  ($055904, 14 B, kernel/state_dispatch_stubs.c)
/// $055904: lea.l $29aeee.l, a2
pub fn i_055904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5590A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AEEE;
    20
}
/// $05590A: jsr $5022a.l
pub fn i_05590a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55910;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55910);
    cpu.pc = 0x5022A;
    16
}
/// $055910: rts
pub fn i_055910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55912;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055912  ($055912, 14 B, kernel/state_dispatch_stubs.c)
/// $055912: lea.l $29af02.l, a2
pub fn i_055912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55918;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AF02;
    20
}
/// $055918: jsr $5022a.l
pub fn i_055918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5591E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5591E);
    cpu.pc = 0x5022A;
    16
}
/// $05591E: rts
pub fn i_05591e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55920;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055920  ($055920, 14 B, kernel/state_dispatch_stubs.c)
/// $055920: lea.l $29af16.l, a2
pub fn i_055920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55926;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AF16;
    20
}
/// $055926: jsr $5022a.l
pub fn i_055926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5592C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5592C);
    cpu.pc = 0x5022A;
    16
}
/// $05592C: rts
pub fn i_05592c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5592E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05592e  ($05592E, 14 B, kernel/state_dispatch_stubs.c)
/// $05592E: lea.l $29af2a.l, a2
pub fn i_05592e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55934;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AF2A;
    20
}
/// $055934: jsr $5022a.l
pub fn i_055934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5593A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5593A);
    cpu.pc = 0x5022A;
    16
}
/// $05593A: rts
pub fn i_05593a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5593C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05593c  ($05593C, 14 B, kernel/state_dispatch_stubs.c)
/// $05593C: lea.l $29af3e.l, a2
pub fn i_05593c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55942;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AF3E;
    20
}
/// $055942: jsr $5022a.l
pub fn i_055942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55948;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55948);
    cpu.pc = 0x5022A;
    16
}
/// $055948: rts
pub fn i_055948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5594A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05594a  ($05594A, 14 B, kernel/state_dispatch_stubs.c)
/// $05594A: lea.l $29af52.l, a2
pub fn i_05594a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55950;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AF52;
    20
}
/// $055950: jsr $5022a.l
pub fn i_055950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55956);
    cpu.pc = 0x5022A;
    16
}
/// $055956: rts
pub fn i_055956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55958;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055958  ($055958, 14 B, kernel/state_dispatch_stubs.c)
/// $055958: lea.l $29af66.l, a2
pub fn i_055958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5595E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AF66;
    20
}
/// $05595E: jsr $5022a.l
pub fn i_05595e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55964);
    cpu.pc = 0x5022A;
    16
}
/// $055964: rts
pub fn i_055964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55966;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055966  ($055966, 14 B, kernel/state_dispatch_stubs.c)
/// $055966: lea.l $29af7a.l, a2
pub fn i_055966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5596C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AF7A;
    20
}
/// $05596C: jsr $5022a.l
pub fn i_05596c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55972);
    cpu.pc = 0x5022A;
    16
}
/// $055972: rts
pub fn i_055972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55974;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055974  ($055974, 14 B, kernel/state_dispatch_stubs.c)
/// $055974: lea.l $29af8e.l, a2
pub fn i_055974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5597A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AF8E;
    20
}
/// $05597A: jsr $5022a.l
pub fn i_05597a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55980;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55980);
    cpu.pc = 0x5022A;
    16
}
/// $055980: rts
pub fn i_055980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55982;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055982  ($055982, 14 B, kernel/state_dispatch_stubs.c)
/// $055982: lea.l $29afa2.l, a2
pub fn i_055982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55988;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AFA2;
    20
}
/// $055988: jsr $5022a.l
pub fn i_055988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5598E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5598E);
    cpu.pc = 0x5022A;
    16
}
/// $05598E: rts
pub fn i_05598e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55990;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055990  ($055990, 14 B, kernel/state_dispatch_stubs.c)
/// $055990: lea.l $29afb6.l, a2
pub fn i_055990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55996;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AFB6;
    20
}
/// $055996: jsr $5022a.l
pub fn i_055996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5599C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5599C);
    cpu.pc = 0x5022A;
    16
}
/// $05599C: rts
pub fn i_05599c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5599E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05599e  ($05599E, 14 B, kernel/state_dispatch_stubs.c)
/// $05599E: lea.l $29afca.l, a2
pub fn i_05599e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559A4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AFCA;
    20
}
/// $0559A4: jsr $5022a.l
pub fn i_0559a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x559AA);
    cpu.pc = 0x5022A;
    16
}
/// $0559AA: rts
pub fn i_0559aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0559ac  ($0559AC, 14 B, kernel/state_dispatch_stubs.c)
/// $0559AC: lea.l $29afde.l, a2
pub fn i_0559ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559B2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AFDE;
    20
}
/// $0559B2: jsr $5022a.l
pub fn i_0559b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x559B8);
    cpu.pc = 0x5022A;
    16
}
/// $0559B8: rts
pub fn i_0559b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0559ba  ($0559BA, 14 B, kernel/state_dispatch_stubs.c)
/// $0559BA: lea.l $29aff2.l, a2
pub fn i_0559ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559C0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AFF2;
    20
}
/// $0559C0: jsr $5022a.l
pub fn i_0559c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x559C6);
    cpu.pc = 0x5022A;
    16
}
/// $0559C6: rts
pub fn i_0559c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0559c8  ($0559C8, 14 B, kernel/state_dispatch_stubs.c)
/// $0559C8: lea.l $29b006.l, a2
pub fn i_0559c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559CE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B006;
    20
}
/// $0559CE: jsr $5022a.l
pub fn i_0559ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x559D4);
    cpu.pc = 0x5022A;
    16
}
/// $0559D4: rts
pub fn i_0559d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0559d6  ($0559D6, 14 B, kernel/state_dispatch_stubs.c)
/// $0559D6: lea.l $29b01a.l, a2
pub fn i_0559d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559DC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B01A;
    20
}
/// $0559DC: jsr $5022a.l
pub fn i_0559dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x559E2);
    cpu.pc = 0x5022A;
    16
}
/// $0559E2: rts
pub fn i_0559e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0559e4  ($0559E4, 14 B, kernel/state_dispatch_stubs.c)
/// $0559E4: lea.l $29b02e.l, a2
pub fn i_0559e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559EA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B02E;
    20
}
/// $0559EA: jsr $5022a.l
pub fn i_0559ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x559F0);
    cpu.pc = 0x5022A;
    16
}
/// $0559F0: rts
pub fn i_0559f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0559f2  ($0559F2, 14 B, kernel/state_dispatch_stubs.c)
/// $0559F2: lea.l $29b042.l, a2
pub fn i_0559f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559F8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B042;
    20
}
/// $0559F8: jsr $5022a.l
pub fn i_0559f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x559FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x559FE);
    cpu.pc = 0x5022A;
    16
}
/// $0559FE: rts
pub fn i_0559fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a00  ($055A00, 14 B, kernel/state_dispatch_stubs.c)
/// $055A00: lea.l $29b056.l, a2
pub fn i_055a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A06;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B056;
    20
}
/// $055A06: jsr $5022a.l
pub fn i_055a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A0C);
    cpu.pc = 0x5022A;
    16
}
/// $055A0C: rts
pub fn i_055a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a0e  ($055A0E, 14 B, kernel/state_dispatch_stubs.c)
/// $055A0E: lea.l $29b06a.l, a2
pub fn i_055a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A14;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B06A;
    20
}
/// $055A14: jsr $5022a.l
pub fn i_055a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A1A);
    cpu.pc = 0x5022A;
    16
}
/// $055A1A: rts
pub fn i_055a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a1c  ($055A1C, 14 B, kernel/state_dispatch_stubs.c)
/// $055A1C: lea.l $29b07e.l, a2
pub fn i_055a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A22;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B07E;
    20
}
/// $055A22: jsr $5022a.l
pub fn i_055a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A28);
    cpu.pc = 0x5022A;
    16
}
/// $055A28: rts
pub fn i_055a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a2a  ($055A2A, 14 B, kernel/state_dispatch_stubs.c)
/// $055A2A: lea.l $29b092.l, a2
pub fn i_055a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A30;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B092;
    20
}
/// $055A30: jsr $5022a.l
pub fn i_055a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A36);
    cpu.pc = 0x5022A;
    16
}
/// $055A36: rts
pub fn i_055a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a38  ($055A38, 14 B, kernel/state_dispatch_stubs.c)
/// $055A38: lea.l $29b0a6.l, a2
pub fn i_055a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A3E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B0A6;
    20
}
/// $055A3E: jsr $5022a.l
pub fn i_055a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A44);
    cpu.pc = 0x5022A;
    16
}
/// $055A44: rts
pub fn i_055a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a46  ($055A46, 14 B, kernel/state_dispatch_stubs.c)
/// $055A46: lea.l $29b0ba.l, a2
pub fn i_055a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A4C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B0BA;
    20
}
/// $055A4C: jsr $5022a.l
pub fn i_055a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A52);
    cpu.pc = 0x5022A;
    16
}
/// $055A52: rts
pub fn i_055a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a54  ($055A54, 14 B, kernel/state_dispatch_stubs.c)
/// $055A54: lea.l $29b0ce.l, a2
pub fn i_055a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A5A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B0CE;
    20
}
/// $055A5A: jsr $5022a.l
pub fn i_055a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A60);
    cpu.pc = 0x5022A;
    16
}
/// $055A60: rts
pub fn i_055a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a62  ($055A62, 14 B, kernel/state_dispatch_stubs.c)
/// $055A62: lea.l $29b0e2.l, a2
pub fn i_055a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A68;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B0E2;
    20
}
/// $055A68: jsr $5022a.l
pub fn i_055a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A6E);
    cpu.pc = 0x5022A;
    16
}
/// $055A6E: rts
pub fn i_055a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a70  ($055A70, 14 B, kernel/state_dispatch_stubs.c)
/// $055A70: lea.l $29b0f6.l, a2
pub fn i_055a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A76;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B0F6;
    20
}
/// $055A76: jsr $5022a.l
pub fn i_055a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A7C);
    cpu.pc = 0x5022A;
    16
}
/// $055A7C: rts
pub fn i_055a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a7e  ($055A7E, 14 B, kernel/state_dispatch_stubs.c)
/// $055A7E: lea.l $29b10a.l, a2
pub fn i_055a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A84;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B10A;
    20
}
/// $055A84: jsr $5022a.l
pub fn i_055a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A8A);
    cpu.pc = 0x5022A;
    16
}
/// $055A8A: rts
pub fn i_055a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a8c  ($055A8C, 14 B, kernel/state_dispatch_stubs.c)
/// $055A8C: lea.l $29b11e.l, a2
pub fn i_055a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A92;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B11E;
    20
}
/// $055A92: jsr $5022a.l
pub fn i_055a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55A98);
    cpu.pc = 0x5022A;
    16
}
/// $055A98: rts
pub fn i_055a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55A9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055a9a  ($055A9A, 14 B, kernel/state_dispatch_stubs.c)
/// $055A9A: lea.l $29b132.l, a2
pub fn i_055a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AA0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B132;
    20
}
/// $055AA0: jsr $5022a.l
pub fn i_055aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55AA6);
    cpu.pc = 0x5022A;
    16
}
/// $055AA6: rts
pub fn i_055aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055aa8  ($055AA8, 14 B, kernel/state_dispatch_stubs.c)
/// $055AA8: lea.l $29b146.l, a2
pub fn i_055aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AAE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B146;
    20
}
/// $055AAE: jsr $5022a.l
pub fn i_055aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55AB4);
    cpu.pc = 0x5022A;
    16
}
/// $055AB4: rts
pub fn i_055ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055ab6  ($055AB6, 14 B, kernel/state_dispatch_stubs.c)
/// $055AB6: lea.l $29b15a.l, a2
pub fn i_055ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55ABC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B15A;
    20
}
/// $055ABC: jsr $5022a.l
pub fn i_055abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55AC2);
    cpu.pc = 0x5022A;
    16
}
/// $055AC2: rts
pub fn i_055ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055ac4  ($055AC4, 14 B, kernel/state_dispatch_stubs.c)
/// $055AC4: lea.l $29b16e.l, a2
pub fn i_055ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55ACA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B16E;
    20
}
/// $055ACA: jsr $5022a.l
pub fn i_055aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55AD0);
    cpu.pc = 0x5022A;
    16
}
/// $055AD0: rts
pub fn i_055ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055ad2  ($055AD2, 14 B, kernel/state_dispatch_stubs.c)
/// $055AD2: lea.l $29b182.l, a2
pub fn i_055ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AD8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B182;
    20
}
/// $055AD8: jsr $5022a.l
pub fn i_055ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55ADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55ADE);
    cpu.pc = 0x5022A;
    16
}
/// $055ADE: rts
pub fn i_055ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055ae0  ($055AE0, 14 B, kernel/state_dispatch_stubs.c)
/// $055AE0: lea.l $29b196.l, a2
pub fn i_055ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AE6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B196;
    20
}
/// $055AE6: jsr $5022a.l
pub fn i_055ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55AEC);
    cpu.pc = 0x5022A;
    16
}
/// $055AEC: rts
pub fn i_055aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055aee  ($055AEE, 14 B, kernel/state_dispatch_stubs.c)
/// $055AEE: lea.l $29b1aa.l, a2
pub fn i_055aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AF4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B1AA;
    20
}
/// $055AF4: jsr $5022a.l
pub fn i_055af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55AFA);
    cpu.pc = 0x5022A;
    16
}
/// $055AFA: rts
pub fn i_055afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55AFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055afc  ($055AFC, 14 B, kernel/state_dispatch_stubs.c)
/// $055AFC: lea.l $29b1be.l, a2
pub fn i_055afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B02;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B1BE;
    20
}
/// $055B02: jsr $5022a.l
pub fn i_055b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B08);
    cpu.pc = 0x5022A;
    16
}
/// $055B08: rts
pub fn i_055b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B0A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b0a  ($055B0A, 14 B, kernel/state_dispatch_stubs.c)
/// $055B0A: lea.l $29b1d2.l, a2
pub fn i_055b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B10;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B1D2;
    20
}
/// $055B10: jsr $5022a.l
pub fn i_055b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B16);
    cpu.pc = 0x5022A;
    16
}
/// $055B16: rts
pub fn i_055b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b18  ($055B18, 14 B, kernel/state_dispatch_stubs.c)
/// $055B18: lea.l $29b1e6.l, a2
pub fn i_055b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B1E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B1E6;
    20
}
/// $055B1E: jsr $5022a.l
pub fn i_055b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B24);
    cpu.pc = 0x5022A;
    16
}
/// $055B24: rts
pub fn i_055b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b26  ($055B26, 14 B, kernel/state_dispatch_stubs.c)
/// $055B26: lea.l $29b1fa.l, a2
pub fn i_055b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B2C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B1FA;
    20
}
/// $055B2C: jsr $5022a.l
pub fn i_055b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B32);
    cpu.pc = 0x5022A;
    16
}
/// $055B32: rts
pub fn i_055b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b34  ($055B34, 14 B, kernel/state_dispatch_stubs.c)
/// $055B34: lea.l $29b20e.l, a2
pub fn i_055b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B3A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B20E;
    20
}
/// $055B3A: jsr $5022a.l
pub fn i_055b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B40);
    cpu.pc = 0x5022A;
    16
}
/// $055B40: rts
pub fn i_055b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b42  ($055B42, 14 B, kernel/state_dispatch_stubs.c)
/// $055B42: lea.l $29b222.l, a2
pub fn i_055b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B48;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B222;
    20
}
/// $055B48: jsr $5022a.l
pub fn i_055b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B4E);
    cpu.pc = 0x5022A;
    16
}
/// $055B4E: rts
pub fn i_055b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b50  ($055B50, 14 B, kernel/state_dispatch_stubs.c)
/// $055B50: lea.l $29b236.l, a2
pub fn i_055b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B56;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B236;
    20
}
/// $055B56: jsr $5022a.l
pub fn i_055b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B5C);
    cpu.pc = 0x5022A;
    16
}
/// $055B5C: rts
pub fn i_055b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b5e  ($055B5E, 14 B, kernel/state_dispatch_stubs.c)
/// $055B5E: lea.l $29b24a.l, a2
pub fn i_055b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B64;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B24A;
    20
}
/// $055B64: jsr $5022a.l
pub fn i_055b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B6A);
    cpu.pc = 0x5022A;
    16
}
/// $055B6A: rts
pub fn i_055b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b6c  ($055B6C, 14 B, kernel/state_dispatch_stubs.c)
/// $055B6C: lea.l $29b25e.l, a2
pub fn i_055b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B72;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B25E;
    20
}
/// $055B72: jsr $5022a.l
pub fn i_055b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B78);
    cpu.pc = 0x5022A;
    16
}
/// $055B78: rts
pub fn i_055b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b7a  ($055B7A, 14 B, kernel/state_dispatch_stubs.c)
/// $055B7A: lea.l $29b272.l, a2
pub fn i_055b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B80;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B272;
    20
}
/// $055B80: jsr $5022a.l
pub fn i_055b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B86);
    cpu.pc = 0x5022A;
    16
}
/// $055B86: rts
pub fn i_055b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055b88  ($055B88, 14 B, kernel/state_dispatch_stubs.c)
/// $055B88: lea.l $29b286.l, a2
pub fn i_055b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B8E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29B286;
    20
}
/// $055B8E: jsr $5022a.l
pub fn i_055b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55B94);
    cpu.pc = 0x5022A;
    16
}
/// $055B94: rts
pub fn i_055b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_055b96  ($055B96, 16 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $055B96: movea.l $8(a6), a1
pub fn i_055b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B9A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $055B9A: move.b $10(a6), d0
pub fn i_055b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55B9E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $055B9E: cmp.b $10(a1), d0
pub fn i_055b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BA2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $055BA2: bcs.w $55bac
pub fn i_055ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BA6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x55BAC; return 10; }
    8
}
// ===== ClearXN_055ba6  ($055BA6, 6 B, thunks/ccr_helpers.c)
/// $055BA6: andi.b #$ee, ccr
pub fn i_055ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BAA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $055BAA: rts
pub fn i_055baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_055bac  ($055BAC, 6 B, thunks/ccr_helpers.c)
/// $055BAC: ori.b #$11, ccr
pub fn i_055bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BB0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $055BB0: rts
pub fn i_055bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_055bb2  ($055BB2, 16 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $055BB2: movea.l $8(a6), a1
pub fn i_055bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BB6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $055BB6: move.b $10(a6), d0
pub fn i_055bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BBA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $055BBA: cmp.b $10(a1), d0
pub fn i_055bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BBE;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $055BBE: bcs.w $55bc8
pub fn i_055bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BC2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x55BC8; return 10; }
    8
}
// ===== ClearXN_055bc2  ($055BC2, 6 B, thunks/ccr_helpers.c)
/// $055BC2: andi.b #$ee, ccr
pub fn i_055bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BC6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $055BC6: rts
pub fn i_055bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_055bc8  ($055BC8, 6 B, thunks/ccr_helpers.c)
/// $055BC8: ori.b #$11, ccr
pub fn i_055bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BCC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $055BCC: rts
pub fn i_055bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grenade_SpawnFromThrower_055bce  ($055BCE, 58 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $055BCE: movem.l a6, -(a7)
pub fn i_055bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BD2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $055BD2: lea.l $100800.l, a6
pub fn i_055bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BD8;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $055BD8: lea.l $55cd8(pc), a1
pub fn i_055bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55CD8;
    16
}
/// $055BDC: jsr $4ae.l
pub fn i_055bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55BE2);
    cpu.pc = 0x4AE;
    16
}
/// $055BE2: movem.l (a7)+, a6
pub fn i_055be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BE6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $055BE6: move.w $22(a6), d0
pub fn i_055be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BEA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055BEA: move.w $24(a6), d1
pub fn i_055bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BEE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055BEE: addi.w #$20, d1
pub fn i_055bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BF2;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055BF2: move.w d0, $22(a0)
pub fn i_055bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BF6;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055BF6: move.w d1, $24(a0)
pub fn i_055bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55BFA;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055BFA: move.b $3a(a6), $3a(a0)
pub fn i_055bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55C00;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $055C00: move.b $5c(a6), $5c(a0)
pub fn i_055c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55C06;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $055C06: rts
pub fn i_055c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55C08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grenade_Task_055cd8  ($055CD8, 746 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $055CD8: bset.b #$4, $6b(a6)
pub fn i_055cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CDE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $055CDE: move.w #$d000, $38(a6)
pub fn i_055cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055CE4: lea.l $55c08(pc), a0
pub fn i_055ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CE8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x55C08;
    16
}
/// $055CE8: moveq #$0, d0
pub fn i_055ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CEA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $055CEA: move.b $5c(a6), d0
pub fn i_055cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CEE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $055CEE: lsl.w #$3, d0
pub fn i_055cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CF0;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $055CF0: adda.l d0, a0
pub fn i_055cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CF2;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $055CF2: move.w (a0), $28(a6)
pub fn i_055cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CF6;
    cpu.ir = 0x3D50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad3, v2 as u16);
    16
}
/// $055CF6: btst.b #$0, $3a(a6)
pub fn i_055cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55CFC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $055CFC: beq.w $55d04
pub fn i_055cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x55D04; return 10; }
    8
}
/// $055D00: neg.w $28(a6)
pub fn i_055d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D04;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $055D04: move.w #$0, $2c(a6)
pub fn i_055d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055D0A: move.w $2(a0), $2e(a6)
pub fn i_055d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D10;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $055D10: move.w $4(a0), $2a(a6)
pub fn i_055d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D16;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $055D16: move.w $6(a0), $70(a6)
pub fn i_055d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D1C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $055D1C: cmpi.b #$2, $106f2b.l
pub fn i_055d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D24;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2B) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $055D24: bne.w $55da8
pub fn i_055d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55DA8; return 10; }
    8
}
/// $055D28: clr.b $3a(a6)
pub fn i_055d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D2C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $055D2C: jsr $5e172.l
pub fn i_055d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55D32);
    cpu.pc = 0x5E172;
    16
}
/// $055D32: bcc.w $55d3a
pub fn i_055d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x55D3A; return 10; }
    8
}
/// $055D36: bra.w $56058
pub fn i_055d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x56058;
    10
}
/// $055D3A: lea.l $55c08(pc), a0
pub fn i_055d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D3E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x55C08;
    16
}
/// $055D3E: moveq #$0, d1
pub fn i_055d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D40;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $055D40: move.b $5c(a6), d1
pub fn i_055d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D44;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $055D44: lsl.w #$3, d1
pub fn i_055d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D46;
    cpu.ir = 0xE749;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $055D46: adda.l d1, a0
pub fn i_055d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D48;
    cpu.ir = 0xD1C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $055D48: move.w (a0), d1
pub fn i_055d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D4A;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $055D4A: move.w d1, d2
pub fn i_055d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D4C;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055D4C: asr.w #$2, d2
pub fn i_055d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D4E;
    cpu.ir = 0xE442;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $055D4E: add.w d2, d1
pub fn i_055d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D50;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $055D50: bge.w $55d56
pub fn i_055d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D54;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x55D56; return 10; }
    8
}
/// $055D54: neg.w d1
pub fn i_055d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D56;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $055D56: move.w d0, d2
pub fn i_055d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D58;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055D58: andi.w #$10, d2
pub fn i_055d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D5C;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x10) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055D5C: lsl.w #$1, d2
pub fn i_055d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D5E;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $055D5E: add.w d2, d0
pub fn i_055d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D60;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $055D60: andi.w #$e0, d0
pub fn i_055d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D64;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xE0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055D64: move.w d0, d2
pub fn i_055d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D66;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055D66: andi.w #$60, d2
pub fn i_055d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D6A;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x60) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055D6A: cmpi.w #$20, d2
pub fn i_055d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D6E;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $055D6E: bne.w $55d7c
pub fn i_055d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55D7C; return 10; }
    8
}
/// $055D72: addq.w #$8, d0
pub fn i_055d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D74;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $055D74: andi.w #$f8, d0
pub fn i_055d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D78;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055D78: bra.w $55d8e
pub fn i_055d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x55D8E;
    10
}
/// $055D7C: cmpi.w #$60, d2
pub fn i_055d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D80;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $055D80: bne.w $55d8e
pub fn i_055d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55D8E; return 10; }
    8
}
/// $055D84: subq.w #$8, d0
pub fn i_055d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D86;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $055D86: andi.w #$f8, d0
pub fn i_055d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D8A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055D8A: bra.w $55d8e
pub fn i_055d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x55D8E;
    10
}
/// $055D8E: move.w d0, $30(a6)
pub fn i_055d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D92;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055D92: jsr $13c0e.l
pub fn i_055d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55D98);
    cpu.pc = 0x13C0E;
    16
}
/// $055D98: move.w d1, $28(a6)
pub fn i_055d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55D9C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055D9C: move.w d2, $2a(a6)
pub fn i_055d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DA0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055DA0: clr.w $2c(a6)
pub fn i_055da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DA4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $055DA4: clr.w $2e(a6)
pub fn i_055da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DA8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $055DA8: move.w #$15d, d1
pub fn i_055da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DAC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055DAC: jsr $236e.l
pub fn i_055dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55DB2);
    cpu.pc = 0x236E;
    16
}
/// $055DB2: move.w #$164, d1
pub fn i_055db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DB6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x164;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055DB6: jsr $236e.l
pub fn i_055db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55DBC);
    cpu.pc = 0x236E;
    16
}
/// $055DBC: move.w #$165, d1
pub fn i_055dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DC0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x165;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055DC0: jsr $236e.l
pub fn i_055dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55DC6);
    cpu.pc = 0x236E;
    16
}
/// $055DC6: move.w $16(a6), $14(a6)
pub fn i_055dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DCC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $055DCC: cmpi.b #$3, $106f2b.l
pub fn i_055dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DD4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2B) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $055DD4: bne.w $55dfa
pub fn i_055dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DD8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55DFA; return 10; }
    8
}
/// $055DD8: clr.b $3a(a6)
pub fn i_055dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DDC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $055DDC: move.w $30(a6), d0
pub fn i_055ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DE0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055DE0: lsr.w #$3, d0
pub fn i_055de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DE2;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $055DE2: andi.w #$1f, d0
pub fn i_055de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DE6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055DE6: move.w d0, $30(a6)
pub fn i_055de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DEA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055DEA: lea.l $2c72c0.l, a0
pub fn i_055dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C72C0;
    20
}
/// $055DF0: jsr $28cd4.l
pub fn i_055df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55DF6);
    cpu.pc = 0x28CD4;
    16
}
/// $055DF6: bra.w $55e5e
pub fn i_055df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55DFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x55E5E;
    10
}
/// $055DFA: cmpi.b #$1, $106f2b.l
pub fn i_055dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E02;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2B) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $055E02: bne.w $55e28
pub fn i_055e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55E28; return 10; }
    8
}
/// $055E06: clr.b $3a(a6)
pub fn i_055e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E0A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $055E0A: move.w $30(a6), d0
pub fn i_055e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E0E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055E0E: lsr.w #$3, d0
pub fn i_055e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E10;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $055E10: andi.w #$1f, d0
pub fn i_055e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E14;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055E14: move.w d0, $30(a6)
pub fn i_055e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E18;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055E18: lea.l $2c72c0.l, a0
pub fn i_055e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C72C0;
    20
}
/// $055E1E: jsr $28cd4.l
pub fn i_055e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55E24);
    cpu.pc = 0x28CD4;
    16
}
/// $055E24: bra.w $55e5e
pub fn i_055e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E28;
    cpu.ir = 0x6000;
    cpu.pc = 0x55E5E;
    10
}
/// $055E28: cmpi.b #$2, $106f2b.l
pub fn i_055e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E30;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2B) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $055E30: bne.w $55e52
pub fn i_055e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55E52; return 10; }
    8
}
/// $055E34: move.w $30(a6), d0
pub fn i_055e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E38;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055E38: lsr.w #$3, d0
pub fn i_055e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E3A;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $055E3A: andi.w #$1f, d0
pub fn i_055e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E3E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055E3E: move.w d0, $30(a6)
pub fn i_055e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055E42: lea.l $2c72c0.l, a0
pub fn i_055e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C72C0;
    20
}
/// $055E48: jsr $28cd4.l
pub fn i_055e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55E4E);
    cpu.pc = 0x28CD4;
    16
}
/// $055E4E: bra.w $55e5e
pub fn i_055e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E52;
    cpu.ir = 0x6000;
    cpu.pc = 0x55E5E;
    10
}
/// $055E52: lea.l $2c72ba.l, a0
pub fn i_055e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C72BA;
    20
}
/// $055E58: jsr $28cd4.l
pub fn i_055e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55E5E);
    cpu.pc = 0x28CD4;
    16
}
/// $055E5E: lea.l $29cbfe.l, a0
pub fn i_055e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CBFE;
    20
}
/// $055E64: move.l a0, $4c(a6)
pub fn i_055e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E68;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $055E68: jsr $283ca.l
pub fn i_055e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55E6E);
    cpu.pc = 0x283CA;
    16
}
/// $055E6E: move.w $24(a6), $72(a6)
pub fn i_055e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E74;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $055E74: lea.l $55e7a(pc), a1
pub fn i_055e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55E7A;
    16
}
/// $055E78: move.l a1, (a6)
pub fn i_055e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055E7A: cmpi.w #$fffe, $70(a6)
pub fn i_055e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E80;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFE, Size::Word, false);
    16
}
/// $055E80: bne.w $55e9e
pub fn i_055e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55E9E; return 10; }
    8
}
/// $055E84: bset.b #$6, $13(a6)
pub fn i_055e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E8A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $055E8A: jsr $27bc8.l
pub fn i_055e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55E90);
    cpu.pc = 0x27BC8;
    16
}
/// $055E90: bcc.w $55e9a
pub fn i_055e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E94;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x55E9A; return 10; }
    8
}
/// $055E94: lea.l $55fca(pc), a1
pub fn i_055e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55FCA;
    16
}
/// $055E98: move.l a1, (a6)
pub fn i_055e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055E9A: bra.w $55eca
pub fn i_055e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55E9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x55ECA;
    10
}
/// $055E9E: tst.w $70(a6)
pub fn i_055e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EA2;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $055EA2: bne.w $55eba
pub fn i_055ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55EBA; return 10; }
    8
}
/// $055EA6: jsr $27bc8.l
pub fn i_055ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55EAC);
    cpu.pc = 0x27BC8;
    16
}
/// $055EAC: bcc.w $55eb6
pub fn i_055eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EB0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x55EB6; return 10; }
    8
}
/// $055EB0: lea.l $55fca(pc), a1
pub fn i_055eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55FCA;
    16
}
/// $055EB4: move.l a1, (a6)
pub fn i_055eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055EB6: bra.w $55eca
pub fn i_055eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EBA;
    cpu.ir = 0x6000;
    cpu.pc = 0x55ECA;
    10
}
/// $055EBA: jsr $27d50.l
pub fn i_055eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55EC0);
    cpu.pc = 0x27D50;
    16
}
/// $055EC0: bcc.w $55eca
pub fn i_055ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x55ECA; return 10; }
    8
}
/// $055EC4: lea.l $55fca(pc), a1
pub fn i_055ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55FCA;
    16
}
/// $055EC8: move.l a1, (a6)
pub fn i_055ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55ECA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055ECA: jsr $280c6.l
pub fn i_055eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55ED0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55ED0);
    cpu.pc = 0x280C6;
    16
}
/// $055ED0: cmpi.b #$1, d0
pub fn i_055ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55ED4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $055ED4: beq.w $55fca
pub fn i_055ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55ED8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x55FCA; return 10; }
    8
}
/// $055ED8: cmpi.b #$2, $106f2b.l
pub fn i_055ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EE0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2B) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $055EE0: beq.w $55f0e
pub fn i_055ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EE4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x55F0E; return 10; }
    8
}
/// $055EE4: move.w $24(a6), d0
pub fn i_055ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EE8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055EE8: cmp.w $72(a6), d0
pub fn i_055ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EEC;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $055EEC: bcc.w $55f0e
pub fn i_055eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EF0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x55F0E; return 10; }
    8
}
/// $055EF0: move.w $2a(a6), d0
pub fn i_055ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EF4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055EF4: asr.w #$5, d0
pub fn i_055ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EF6;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $055EF6: sub.w d0, $2a(a6)
pub fn i_055ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EFA;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $055EFA: move.w $28(a6), d0
pub fn i_055efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55EFE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055EFE: beq.w $55f0e
pub fn i_055efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x55F0E; return 10; }
    8
}
/// $055F02: asr.w #$5, d0
pub fn i_055f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F04;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $055F04: bne.w $55f0a
pub fn i_055f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55F0A; return 10; }
    8
}
/// $055F08: moveq #$1, d0
pub fn i_055f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F0A;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $055F0A: sub.w d0, $28(a6)
pub fn i_055f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F0E;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $055F0E: moveq #$0, d0
pub fn i_055f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F10;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $055F10: move.b $106f28.l, d0
pub fn i_055f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F16;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $055F16: andi.w #$3, d0
pub fn i_055f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F1A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $055F1A: lsl.w #$1, d0
pub fn i_055f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F1C;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $055F1C: lea.l $55cd0(pc), a0
pub fn i_055f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F20;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x55CD0;
    16
}
/// $055F20: move.w (a0, d0.w), d1
pub fn i_055f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F24;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $055F24: move.w (a6, d1.w), $14(a6)
pub fn i_055f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F2A;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $055F2A: cmpi.b #$3, $106f2b.l
pub fn i_055f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F32;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2B) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $055F32: bne.w $55f4e
pub fn i_055f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F36;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55F4E; return 10; }
    8
}
/// $055F36: move.w $28(a6), d0
pub fn i_055f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F3A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055F3A: move.w $2a(a6), d1
pub fn i_055f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F3E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055F3E: asr.w #$4, d0
pub fn i_055f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F40;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $055F40: asr.w #$4, d1
pub fn i_055f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F42;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $055F42: jsr $5e018.l
pub fn i_055f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55F48);
    cpu.pc = 0x5E018;
    16
}
/// $055F48: lsr.w #$3, d0
pub fn i_055f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F4A;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $055F4A: move.w d0, $30(a6)
pub fn i_055f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F4E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055F4E: cmpi.b #$1, $106f2b.l
pub fn i_055f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F56;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2B) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $055F56: bne.w $55f72
pub fn i_055f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F5A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55F72; return 10; }
    8
}
/// $055F5A: move.w $28(a6), d0
pub fn i_055f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F5E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055F5E: move.w $2a(a6), d1
pub fn i_055f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F62;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055F62: asr.w #$4, d0
pub fn i_055f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F64;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $055F64: asr.w #$4, d1
pub fn i_055f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F66;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $055F66: jsr $5e018.l
pub fn i_055f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55F6C);
    cpu.pc = 0x5E018;
    16
}
/// $055F6C: lsr.w #$3, d0
pub fn i_055f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F6E;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $055F6E: move.w d0, $30(a6)
pub fn i_055f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F72;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055F72: jsr $28d70.l
pub fn i_055f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55F78);
    cpu.pc = 0x28D70;
    16
}
/// $055F78: jsr $283d8.l
pub fn i_055f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55F7E);
    cpu.pc = 0x283D8;
    16
}
/// $055F7E: btst.b #$1, $13(a6)
pub fn i_055f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F84;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $055F84: beq.w $55f8e
pub fn i_055f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F88;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x55F8E; return 10; }
    8
}
/// $055F88: lea.l $56012(pc), a1
pub fn i_055f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x56012;
    16
}
/// $055F8C: move.l a1, (a6)
pub fn i_055f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055F8E: jsr $2870a.l
pub fn i_055f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55F94);
    cpu.pc = 0x2870A;
    16
}
/// $055F94: bcc.w $55f9e
pub fn i_055f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x55F9E; return 10; }
    8
}
/// $055F98: lea.l $55fee(pc), a1
pub fn i_055f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55FEE;
    16
}
/// $055F9C: move.l a1, (a6)
pub fn i_055f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55F9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055F9E: tst.b $10e39e.l
pub fn i_055f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FA4;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39E) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $055FA4: beq.w $55fae
pub fn i_055fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x55FAE; return 10; }
    8
}
/// $055FA8: lea.l $56012(pc), a1
pub fn i_055fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x56012;
    16
}
/// $055FAC: move.l a1, (a6)
pub fn i_055fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055FAE: movea.l #$ffffffff, a0
pub fn i_055fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FB4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $055FB4: lea.l $55cc8(pc), a0
pub fn i_055fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FB8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x55CC8;
    16
}
/// $055FB8: jsr $5dd56.l
pub fn i_055fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55FBE);
    cpu.pc = 0x5DD56;
    16
}
/// $055FBE: bcc.w $55fc8
pub fn i_055fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x55FC8; return 10; }
    8
}
// ===== SetTaskHandler_055fc2  ($055FC2, 8 B, kernel/task_handlers.c)
/// $055FC2: lea.l $56058(pc), a1
pub fn i_055fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x56058;
    16
}
/// $055FC6: move.l a1, (a6)
pub fn i_055fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055FC8: rts
pub fn i_055fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grenade_Explode_055fca  ($055FCA, 142 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $055FCA: move.w #$2000, d0
pub fn i_055fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FCE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055FCE: jsr $28134.l
pub fn i_055fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55FD4);
    cpu.pc = 0x28134;
    16
}
/// $055FD4: andi.w #$ffe3, $38(a6)
pub fn i_055fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FDA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $055FDA: ori.w #$10, $38(a6)
pub fn i_055fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FE0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $055FE0: move.w #$1027, d0
pub fn i_055fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055FE4: jsr $2352.l
pub fn i_055fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55FEA);
    cpu.pc = 0x2352;
    16
}
/// $055FEA: bra.w $56032
pub fn i_055fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x56032;
    10
}
/// $055FEE: move.w #$8000, d0
pub fn i_055fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055FF2: jsr $28134.l
pub fn i_055ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55FF8);
    cpu.pc = 0x28134;
    16
}
/// $055FF8: andi.w #$ffe3, $38(a6)
pub fn i_055ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55FFE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $055FFE: ori.w #$4, $38(a6)
pub fn i_055ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56004;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $056004: move.w #$1027, d0
pub fn i_056004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56008;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056008: jsr $2352.l
pub fn i_056008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5600E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5600E);
    cpu.pc = 0x2352;
    16
}
/// $05600E: bra.w $56032
pub fn i_05600e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56012;
    cpu.ir = 0x6000;
    cpu.pc = 0x56032;
    10
}
/// $056012: move.w #$8000, d0
pub fn i_056012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56016;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056016: jsr $28134.l
pub fn i_056016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5601C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5601C);
    cpu.pc = 0x28134;
    16
}
/// $05601C: andi.w #$ffe3, $38(a6)
pub fn i_05601c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56022;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $056022: ori.w #$10, $38(a6)
pub fn i_056022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56028;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $056028: move.w #$ffff, d0
pub fn i_056028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5602C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05602C: jsr $2352.l
pub fn i_05602c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56032;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56032);
    cpu.pc = 0x2352;
    16
}
/// $056032: move.l #$ffffffff, $48(a6)
pub fn i_056032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5603A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05603A: bclr.b #$1, $13(a6)
pub fn i_05603a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56040;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056040: bclr.b #$3, $13(a6)
pub fn i_056040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56046;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056046: bclr.b #$0, $13(a6)
pub fn i_056046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5604C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05604C: jsr $13600.l
pub fn i_05604c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56052);
    cpu.pc = 0x13600;
    16
}
/// $056052: jmp $77eda.l
pub fn i_056052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56058;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77EDA;
    8
}
// ===== Jsr5B6ThenJmpScheduler_056058  ($056058, 14 B, kernel/jsr5b6_jmp518.c)
/// $056058: jsr $5b6.l
pub fn i_056058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5605E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5605E);
    cpu.pc = 0x5B6;
    16
}
/// $05605E: jmp $518.l
pub fn i_05605e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56064;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $056064: rts
pub fn i_056064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56066;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bounce_SpawnFromParent_056066  ($056066, 40 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056066: move.l a6, -(a7)
pub fn i_056066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56068;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $056068: lea.l $100800.l, a6
pub fn i_056068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5606E;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $05606E: lea.l $5608e(pc), a1
pub fn i_05606e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56072;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5608E;
    16
}
/// $056072: jsr $4ae.l
pub fn i_056072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56078);
    cpu.pc = 0x4AE;
    16
}
/// $056078: movea.l (a7)+, a6
pub fn i_056078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5607A;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05607A: move.w $22(a6), $22(a0)
pub fn i_05607a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56080;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $056080: move.w $24(a6), $24(a0)
pub fn i_056080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56086;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $056086: move.b $3a(a6), $3a(a0)
pub fn i_056086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5608C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05608C: rts
pub fn i_05608c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5608E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bounce_Task_05608e  ($05608E, 228 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $05608E: bset.b #$2, $5b(a6)
pub fn i_05608e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56094;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056094: bset.b #$4, $6b(a6)
pub fn i_056094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5609A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05609A: move.w #$d000, d0
pub fn i_05609a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5609E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05609E: jsr $28134.l
pub fn i_05609e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x560A4);
    cpu.pc = 0x28134;
    16
}
/// $0560A4: andi.w #$ffe3, $38(a6)
pub fn i_0560a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560AA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0560AA: ori.w #$10, $38(a6)
pub fn i_0560aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560B0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0560B0: lea.l $29cda4.l, a0
pub fn i_0560b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CDA4;
    20
}
/// $0560B6: jsr $28cd4.l
pub fn i_0560b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x560BC);
    cpu.pc = 0x28CD4;
    16
}
/// $0560BC: jsr $13600.l
pub fn i_0560bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x560C2);
    cpu.pc = 0x13600;
    16
}
/// $0560C2: move.w #$12, d1
pub fn i_0560c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0560C6: jsr $236e.l
pub fn i_0560c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x560CC);
    cpu.pc = 0x236E;
    16
}
/// $0560CC: move.w #$13, d1
pub fn i_0560cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560D0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x13;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0560D0: jsr $236e.l
pub fn i_0560d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x560D6);
    cpu.pc = 0x236E;
    16
}
/// $0560D6: addi.w #$18, $24(a6)
pub fn i_0560d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560DC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0560DC: moveq #$18, d0
pub fn i_0560dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560DE;
    cpu.ir = 0x7018;
    cpu.d[0] = 0x18;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0560DE: move.w #$180, d1
pub fn i_0560de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0560E2: btst.b #$0, $3a(a6)
pub fn i_0560e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560E8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0560E8: bne.w $560f0
pub fn i_0560e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x560F0; return 10; }
    8
}
/// $0560EC: neg.w d0
pub fn i_0560ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560EE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0560EE: neg.w d1
pub fn i_0560ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560F0;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0560F0: add.w d0, $22(a6)
pub fn i_0560f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560F4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0560F4: move.w d1, $28(a6)
pub fn i_0560f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560F8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0560F8: clr.w $2e(a6)
pub fn i_0560f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x560FC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0560FC: clr.w $2c(a6)
pub fn i_0560fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56100;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056100: jsr $283ca.l
pub fn i_056100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56106);
    cpu.pc = 0x283CA;
    16
}
/// $056106: lea.l $5610c(pc), a1
pub fn i_056106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5610A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5610C;
    16
}
/// $05610A: move.l a1, (a6)
pub fn i_05610a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5610C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05610C: jsr $27c8c.l
pub fn i_05610c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56112;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56112);
    cpu.pc = 0x27C8C;
    16
}
/// $056112: bcc.w $5611c
pub fn i_056112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56116;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5611C; return 10; }
    8
}
/// $056116: lea.l $5617a(pc), a1
pub fn i_056116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5611A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5617A;
    16
}
/// $05611A: move.l a1, (a6)
pub fn i_05611a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5611C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05611C: jsr $28d70.l
pub fn i_05611c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56122;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56122);
    cpu.pc = 0x28D70;
    16
}
/// $056122: jsr $2870a.l
pub fn i_056122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56128);
    cpu.pc = 0x2870A;
    16
}
/// $056128: bcc.w $56132
pub fn i_056128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5612C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56132; return 10; }
    8
}
/// $05612C: lea.l $561da(pc), a1
pub fn i_05612c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56130;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x561DA;
    16
}
/// $056130: move.l a1, (a6)
pub fn i_056130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56132;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $056132: subq.b #$1, $5c(a6)
pub fn i_056132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56136;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $056136: move.b $5c(a6), d0
pub fn i_056136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5613A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05613A: andi.b #$3, d0
pub fn i_05613a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5613E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05613E: bne.w $56158
pub fn i_05613e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56142;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56158; return 10; }
    8
}
/// $056142: move.w $16(a6), d0
pub fn i_056142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56146;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056146: move.w $18(a6), d1
pub fn i_056146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5614A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05614A: exg.l d0, d1
pub fn i_05614a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5614C;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $05614C: move.w d0, $16(a6)
pub fn i_05614c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56150;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056150: move.w d1, $18(a6)
pub fn i_056150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56154;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056154: move.w d0, $14(a6)
pub fn i_056154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56158;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056158: movea.l #$ffffffff, a0
pub fn i_056158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5615E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05615E: lea.l $55cc8(pc), a0
pub fn i_05615e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56162;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x55CC8;
    16
}
/// $056162: jsr $5dd56.l
pub fn i_056162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56168;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56168);
    cpu.pc = 0x5DD56;
    16
}
/// $056168: bcc.w $56172
pub fn i_056168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5616C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56172; return 10; }
    8
}
/// $05616C: jmp $518.l
pub fn i_05616c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56172;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_056172  ($056172, 8 B, thunks/jsr_abs_thunks.c)
/// $056172: jsr $8f308.l
pub fn i_056172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56178;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56178);
    cpu.pc = 0x8F308;
    16
}
/// $056178: rts
pub fn i_056178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5617A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bounce_Rest_05617a  ($05617A, 52 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $05617A: lea.l $29cdb6.l, a0
pub fn i_05617a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56180;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CDB6;
    20
}
/// $056180: jsr $28cd4.l
pub fn i_056180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56186;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56186);
    cpu.pc = 0x28CD4;
    16
}
/// $056186: clr.w $28(a6)
pub fn i_056186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5618A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05618A: clr.w $2a(a6)
pub fn i_05618a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5618E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05618E: lea.l $56194(pc), a1
pub fn i_05618e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56192;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x56194;
    16
}
/// $056192: move.l a1, (a6)
pub fn i_056192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56194;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $056194: jsr $27c8c.l
pub fn i_056194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5619A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5619A);
    cpu.pc = 0x27C8C;
    16
}
/// $05619A: jsr $28d70.l
pub fn i_05619a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x561A0);
    cpu.pc = 0x28D70;
    16
}
/// $0561A0: bcc.w $561aa
pub fn i_0561a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x561AA; return 10; }
    8
}
/// $0561A4: lea.l $561ae(pc), a1
pub fn i_0561a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x561AE;
    16
}
/// $0561A8: move.l a1, (a6)
pub fn i_0561a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0561AA: bra.w $56122
pub fn i_0561aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x56122;
    10
}
// ===== Bounce_Rest2_0561ae  ($0561AE, 44 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $0561AE: lea.l $29cfa8.l, a0
pub fn i_0561ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CFA8;
    20
}
/// $0561B4: jsr $28cd4.l
pub fn i_0561b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x561BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0561BA: lea.l $561c0(pc), a1
pub fn i_0561ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x561C0;
    16
}
/// $0561BE: move.l a1, (a6)
pub fn i_0561be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0561C0: jsr $27c8c.l
pub fn i_0561c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x561C6);
    cpu.pc = 0x27C8C;
    16
}
/// $0561C6: jsr $28d70.l
pub fn i_0561c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x561CC);
    cpu.pc = 0x28D70;
    16
}
/// $0561CC: bcc.w $561d6
pub fn i_0561cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x561D6; return 10; }
    8
}
/// $0561D0: lea.l $561da(pc), a1
pub fn i_0561d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x561DA;
    16
}
/// $0561D4: move.l a1, (a6)
pub fn i_0561d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0561D6: bra.w $56122
pub fn i_0561d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x56122;
    10
}
// ===== Bounce_Fizzle_0561da  ($0561DA, 34 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $0561DA: lea.l $29cfd2.l, a0
pub fn i_0561da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CFD2;
    20
}
/// $0561E0: jsr $28cd4.l
pub fn i_0561e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x561E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0561E6: lea.l $561ec(pc), a1
pub fn i_0561e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x561EC;
    16
}
/// $0561EA: move.l a1, (a6)
pub fn i_0561ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0561EC: jsr $27c8c.l
pub fn i_0561ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x561F2);
    cpu.pc = 0x27C8C;
    16
}
/// $0561F2: jsr $28d70.l
pub fn i_0561f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x561F8);
    cpu.pc = 0x28D70;
    16
}
/// $0561F8: bcc.w $56202
pub fn i_0561f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x561FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56202; return 10; }
    8
}
// ===== SetTaskHandler_0561fc  ($0561FC, 8 B, kernel/task_handlers.c)
/// $0561FC: lea.l $56204(pc), a1
pub fn i_0561fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56200;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x56204;
    16
}
/// $056200: move.l a1, (a6)
pub fn i_056200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56202;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $056202: rts
pub fn i_056202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56204;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bounce_Explode_056204  ($056204, 104 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056204: move.w #$1022, d0
pub fn i_056204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56208;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056208: jsr $2352.l
pub fn i_056208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5620E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5620E);
    cpu.pc = 0x2352;
    16
}
/// $05620E: lea.l $29cca6.l, a0
pub fn i_05620e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56214;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CCA6;
    20
}
/// $056214: move.l a0, $4c(a6)
pub fn i_056214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56218;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056218: jsr $283ca.l
pub fn i_056218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5621E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5621E);
    cpu.pc = 0x283CA;
    16
}
/// $05621E: jsr $283d8.l
pub fn i_05621e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56224);
    cpu.pc = 0x283D8;
    16
}
/// $056224: jsr $13600.l
pub fn i_056224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5622A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5622A);
    cpu.pc = 0x13600;
    16
}
/// $05622A: move.w #$c000, d0
pub fn i_05622a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5622E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05622E: jsr $28134.l
pub fn i_05622e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56234;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56234);
    cpu.pc = 0x28134;
    16
}
/// $056234: andi.w #$ffe3, $38(a6)
pub fn i_056234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5623A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $05623A: ori.w #$10, $38(a6)
pub fn i_05623a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56240;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $056240: move.w #$16b, d1
pub fn i_056240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56244;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056244: jsr $236e.l
pub fn i_056244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5624A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5624A);
    cpu.pc = 0x236E;
    16
}
/// $05624A: lea.l $29d01a.l, a0
pub fn i_05624a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56250;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D01A;
    20
}
/// $056250: jsr $28cd4.l
pub fn i_056250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56256);
    cpu.pc = 0x28CD4;
    16
}
/// $056256: lea.l $5625c(pc), a1
pub fn i_056256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5625A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5625C;
    16
}
/// $05625A: move.l a1, (a6)
pub fn i_05625a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5625C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05625C: jsr $27c8c.l
pub fn i_05625c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56262;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56262);
    cpu.pc = 0x27C8C;
    16
}
/// $056262: jsr $28d70.l
pub fn i_056262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56268;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56268);
    cpu.pc = 0x28D70;
    16
}
/// $056268: bcc.w $56278
pub fn i_056268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5626C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56278; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_05626c  ($05626C, 14 B, kernel/jsr5b6_jmp518.c)
/// $05626C: jsr $5b6.l
pub fn i_05626c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56272;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56272);
    cpu.pc = 0x5B6;
    16
}
/// $056272: jmp $518.l
pub fn i_056272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56278;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $056278: rts
pub fn i_056278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5627A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_ApplyDrag_0563f4  ($0563F4, 68 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $0563F4: move.w $28(a6), d0
pub fn i_0563f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x563F8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0563F8: beq.w $56408
pub fn i_0563f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x563FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56408; return 10; }
    8
}
/// $0563FC: asr.w #$8, d0
pub fn i_0563fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x563FE;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0563FE: bne.w $56404
pub fn i_0563fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56402;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56404; return 10; }
    8
}
/// $056402: moveq #$1, d0
pub fn i_056402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56404;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056404: sub.w d0, $28(a6)
pub fn i_056404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56408;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $056408: jsr $28158.l
pub fn i_056408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5640E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5640E);
    cpu.pc = 0x28158;
    16
}
/// $05640E: tst.b d0
pub fn i_05640e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56410;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $056410: beq.w $56436
pub fn i_056410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56414;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56436; return 10; }
    8
}
/// $056414: cmpi.b #$3, d0
pub fn i_056414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56418;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $056418: bcc.w $56424
pub fn i_056418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5641C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56424; return 10; }
    8
}
/// $05641C: move.w #$3, d1
pub fn i_05641c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56420;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056420: bra.w $56428
pub fn i_056420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56424;
    cpu.ir = 0x6000;
    cpu.pc = 0x56428;
    10
}
/// $056424: move.w #$b, d1
pub fn i_056424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56428;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056428: andi.b #$1, d0
pub fn i_056428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5642C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05642C: bne.w $56432
pub fn i_05642c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56430;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56432; return 10; }
    8
}
/// $056430: neg.w d1
pub fn i_056430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56432;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056432: sub.w d1, $28(a6)
pub fn i_056432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56436;
    cpu.ir = 0x936E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $056436: rts
pub fn i_056436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56438;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_SpawnFromParent_056438  ($056438, 52 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056438: move.l a6, -(a7)
pub fn i_056438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5643A;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05643A: lea.l $100800.l, a6
pub fn i_05643a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56440;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $056440: lea.l $5646c(pc), a1
pub fn i_056440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56444;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5646C;
    16
}
/// $056444: jsr $4ae.l
pub fn i_056444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5644A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5644A);
    cpu.pc = 0x4AE;
    16
}
/// $05644A: movea.l (a7)+, a6
pub fn i_05644a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5644C;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05644C: move.w $22(a6), $22(a0)
pub fn i_05644c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56452;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $056452: move.w $24(a6), $24(a0)
pub fn i_056452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56458;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $056458: move.b $3a(a6), $3a(a0)
pub fn i_056458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5645E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05645E: move.b $11(a6), $11(a0)
pub fn i_05645e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56464;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad4, v3 as u8);
    20
}
/// $056464: addi.w #$14, $24(a0)
pub fn i_056464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5646A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05646A: rts
pub fn i_05646a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5646C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_Shell_Task_05646c  ($05646C, 290 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $05646C: move.w #$e, d1
pub fn i_05646c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56470;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056470: jsr $236e.l
pub fn i_056470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56476);
    cpu.pc = 0x236E;
    16
}
/// $056476: lea.l $562ce(pc), a0
pub fn i_056476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5647A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x562CE;
    16
}
/// $05647A: move.l a0, $4c(a6)
pub fn i_05647a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5647E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05647E: jsr $283ca.l
pub fn i_05647e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56484;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56484);
    cpu.pc = 0x283CA;
    16
}
/// $056484: jsr $283ca.l
pub fn i_056484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5648A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5648A);
    cpu.pc = 0x283CA;
    16
}
/// $05648A: bset.b #$5, $13(a6)
pub fn i_05648a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56490;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056490: lea.l $2b7632.l, a0
pub fn i_056490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56496;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7632;
    20
}
/// $056496: jsr $799de.l
pub fn i_056496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5649C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5649C);
    cpu.pc = 0x799DE;
    16
}
/// $05649C: move.w d0, $66(a6)
pub fn i_05649c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564A0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0564A0: move.w #$d000, d0
pub fn i_0564a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0564A4: jsr $28134.l
pub fn i_0564a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x564AA);
    cpu.pc = 0x28134;
    16
}
/// $0564AA: andi.w #$ffe3, $38(a6)
pub fn i_0564aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564B0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0564B0: ori.w #$18, $38(a6)
pub fn i_0564b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564B6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0564B6: move.w #$fe00, d0
pub fn i_0564b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0564BA: jsr $5dca4.l
pub fn i_0564ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x564C0);
    cpu.pc = 0x5DCA4;
    16
}
/// $0564C0: move.w d0, $28(a6)
pub fn i_0564c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564C4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0564C4: move.w #$195, $2a(a6)
pub fn i_0564c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x195;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0564CA: move.w #$ffaf, $2e(a6)
pub fn i_0564ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0564D0: move.w #$0, $2c(a6)
pub fn i_0564d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0564D6: lea.l $56378(pc), a0
pub fn i_0564d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564DA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x56378;
    16
}
/// $0564DA: jsr $28cd4.l
pub fn i_0564da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x564E0);
    cpu.pc = 0x28CD4;
    16
}
/// $0564E0: lea.l $564e6(pc), a1
pub fn i_0564e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x564E6;
    16
}
/// $0564E4: move.l a1, (a6)
pub fn i_0564e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0564E6: jsr $2783a.l
pub fn i_0564e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x564EC);
    cpu.pc = 0x2783A;
    16
}
/// $0564EC: jsr $27eba.l
pub fn i_0564ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x564F2);
    cpu.pc = 0x27EBA;
    16
}
/// $0564F2: bcc.w $56500
pub fn i_0564f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56500; return 10; }
    8
}
/// $0564F6: jsr $28364.l
pub fn i_0564f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x564FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x564FC);
    cpu.pc = 0x28364;
    16
}
/// $0564FC: bra.w $5650a
pub fn i_0564fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56500;
    cpu.ir = 0x6000;
    cpu.pc = 0x5650A;
    10
}
/// $056500: jsr $28292.l
pub fn i_056500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56506;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56506);
    cpu.pc = 0x28292;
    16
}
/// $056506: bsr.w $563f4
pub fn i_056506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5650A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5650A);
    cpu.pc = 0x563F4;
    18
}
/// $05650A: move.w $28(a6), d0
pub fn i_05650a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5650E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05650E: btst.b #$0, $3a(a6)
pub fn i_05650e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56514;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056514: bne.w $5652a
pub fn i_056514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56518;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5652A; return 10; }
    8
}
/// $056518: cmpi.w #$0, d0
pub fn i_056518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5651C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05651C: blt.w $56526
pub fn i_05651c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56520;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x56526; return 10; }
    8
}
/// $056520: lea.l $5659c(pc), a1
pub fn i_056520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56524;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5659C;
    16
}
/// $056524: move.l a1, (a6)
pub fn i_056524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56526;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $056526: bra.w $56538
pub fn i_056526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5652A;
    cpu.ir = 0x6000;
    cpu.pc = 0x56538;
    10
}
/// $05652A: cmpi.w #$0, d0
pub fn i_05652a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5652E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05652E: bgt.w $56538
pub fn i_05652e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56532;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x56538; return 10; }
    8
}
/// $056532: lea.l $5659c(pc), a1
pub fn i_056532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56536;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5659C;
    16
}
/// $056536: move.l a1, (a6)
pub fn i_056536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56538;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $056538: btst.b #$5, $5a(a6)
pub fn i_056538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5653E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05653E: beq.w $56548
pub fn i_05653e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56542;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56548; return 10; }
    8
}
/// $056542: lea.l $5659c(pc), a1
pub fn i_056542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56546;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5659C;
    16
}
/// $056546: move.l a1, (a6)
pub fn i_056546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56548;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $056548: jsr $28d70.l
pub fn i_056548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5654E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5654E);
    cpu.pc = 0x28D70;
    16
}
/// $05654E: jsr $283d8.l
pub fn i_05654e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56554;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56554);
    cpu.pc = 0x283D8;
    16
}
/// $056554: btst.b #$1, $13(a6)
pub fn i_056554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5655A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05655A: beq.w $56564
pub fn i_05655a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5655E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56564; return 10; }
    8
}
/// $05655E: lea.l $5659c(pc), a1
pub fn i_05655e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56562;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5659C;
    16
}
/// $056562: move.l a1, (a6)
pub fn i_056562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56564;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $056564: bclr.b #$3, $13(a6)
pub fn i_056564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5656A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05656A: jsr $28758.l
pub fn i_05656a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56570;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56570);
    cpu.pc = 0x28758;
    16
}
/// $056570: bcc.w $5657a
pub fn i_056570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56574;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5657A; return 10; }
    8
}
/// $056574: lea.l $5659c(pc), a1
pub fn i_056574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56578;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5659C;
    16
}
/// $056578: move.l a1, (a6)
pub fn i_056578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5657A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05657A: movea.l #$ffffffff, a0
pub fn i_05657a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56580;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $056580: lea.l $55cc8(pc), a0
pub fn i_056580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56584;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x55CC8;
    16
}
/// $056584: jsr $5dd56.l
pub fn i_056584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5658A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5658A);
    cpu.pc = 0x5DD56;
    16
}
/// $05658A: bcc.w $56594
pub fn i_05658a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5658E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56594; return 10; }
    8
}
// ===== SetTaskHandler_05658e  ($05658E, 8 B, kernel/task_handlers.c)
/// $05658E: lea.l $56596(pc), a1
pub fn i_05658e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56592;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x56596;
    16
}
/// $056592: move.l a1, (a6)
pub fn i_056592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56594;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $056594: rts
pub fn i_056594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56596;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_056596  ($056596, 6 B, thunks/jmp_abs_thunks.c)
/// $056596: jmp $518.l
pub fn i_056596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5659C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Mortar_Shell_Explode_05659c  ($05659C, 44 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $05659C: move.w #$1022, d0
pub fn i_05659c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x565A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0565A0: jsr $2352.l
pub fn i_0565a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x565A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x565A6);
    cpu.pc = 0x2352;
    16
}
/// $0565A6: jsr $13600.l
pub fn i_0565a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x565AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x565AC);
    cpu.pc = 0x13600;
    16
}
/// $0565AC: move.w #$2000, d0
pub fn i_0565ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x565B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0565B0: jsr $28134.l
pub fn i_0565b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x565B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x565B6);
    cpu.pc = 0x28134;
    16
}
/// $0565B6: andi.w #$ffe3, $38(a6)
pub fn i_0565b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x565BC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0565BC: ori.w #$10, $38(a6)
pub fn i_0565bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x565C2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0565C2: jmp $77f6a.l
pub fn i_0565c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x565C8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Roller_SpawnFromParent_05684c  ($05684C, 36 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $05684C: lea.l $56870(pc), a1
pub fn i_05684c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56850;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x56870;
    16
}
/// $056850: jsr $4ae.l
pub fn i_056850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56856);
    cpu.pc = 0x4AE;
    16
}
/// $056856: movea.l (a7)+, a6
pub fn i_056856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56858;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $056858: move.w $22(a6), $22(a0)
pub fn i_056858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5685E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05685E: move.w $24(a6), $24(a0)
pub fn i_05685e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56864;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $056864: move.b $3a(a6), $3a(a0)
pub fn i_056864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5686A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05686A: clr.w $70(a6)
pub fn i_05686a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5686E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05686E: rts
pub fn i_05686e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56870;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Roller_Task_056870  ($056870, 406 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056870: bset.b #$4, $6b(a6)
pub fn i_056870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56876;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056876: move.w #$d000, d0
pub fn i_056876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5687A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05687A: jsr $28134.l
pub fn i_05687a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56880;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56880);
    cpu.pc = 0x28134;
    16
}
/// $056880: andi.w #$ffe3, $38(a6)
pub fn i_056880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56886;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $056886: ori.w #$10, $38(a6)
pub fn i_056886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5688C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $05688C: move.w #$e, d1
pub fn i_05688c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56890;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056890: jsr $236e.l
pub fn i_056890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56896);
    cpu.pc = 0x236E;
    16
}
/// $056896: lea.l $56670(pc), a0
pub fn i_056896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5689A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x56670;
    16
}
/// $05689A: jsr $28cd4.l
pub fn i_05689a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x568A0);
    cpu.pc = 0x28CD4;
    16
}
/// $0568A0: move.w #$80, $2a(a6)
pub fn i_0568a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568A6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0568A6: move.w #$fffc, $2e(a6)
pub fn i_0568a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0568AC: clr.w $28(a6)
pub fn i_0568ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568B0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0568B0: addi.w #$28, $24(a6)
pub fn i_0568b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568B6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0568B6: move.w #$64, $66(a6)
pub fn i_0568b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0568BC: lea.l $5661c(pc), a0
pub fn i_0568bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568C0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5661C;
    16
}
/// $0568C0: move.l a0, $48(a6)
pub fn i_0568c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568C4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0568C4: lea.l $565c8(pc), a0
pub fn i_0568c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568C8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x565C8;
    16
}
/// $0568C8: move.l a0, $4c(a6)
pub fn i_0568c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568CC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0568CC: jsr $283ca.l
pub fn i_0568cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x568D2);
    cpu.pc = 0x283CA;
    16
}
/// $0568D2: jsr $283ca.l
pub fn i_0568d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x568D8);
    cpu.pc = 0x283CA;
    16
}
/// $0568D8: clr.w $70(a6)
pub fn i_0568d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568DC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0568DC: lea.l $568e2(pc), a1
pub fn i_0568dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x568E2;
    16
}
/// $0568E0: move.l a1, (a6)
pub fn i_0568e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0568E2: move.w #$10, d0
pub fn i_0568e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0568E6: btst.b #$0, $3a(a6)
pub fn i_0568e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568EC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0568EC: bne.w $568f2
pub fn i_0568ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x568F2; return 10; }
    8
}
/// $0568F0: neg.w d0
pub fn i_0568f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568F2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0568F2: add.w d0, $28(a6)
pub fn i_0568f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568F6;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0568F6: jsr $27bc8.l
pub fn i_0568f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x568FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x568FC);
    cpu.pc = 0x27BC8;
    16
}
/// $0568FC: bcc.w $56914
pub fn i_0568fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56900;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56914; return 10; }
    8
}
/// $056900: addi.w #$1, $24(a6)
pub fn i_056900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56906;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $056906: clr.w $2a(a6)
pub fn i_056906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5690A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05690A: move.w #$10, $2e(a6)
pub fn i_05690a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56910;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056910: bra.w $56980
pub fn i_056910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56914;
    cpu.ir = 0x6000;
    cpu.pc = 0x56980;
    10
}
/// $056914: move.w $28(a6), d7
pub fn i_056914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56918;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056918: bmi.w $5691e
pub fn i_056918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5691C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5691E; return 10; }
    8
}
/// $05691C: neg.w d7
pub fn i_05691c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5691E;
    cpu.ir = 0x4447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05691E: move.w d7, $36(a6)
pub fn i_05691e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56922;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056922: move.w #$7, d1
pub fn i_056922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56926;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056926: btst.b #$0, $3a(a6)
pub fn i_056926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5692C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05692C: bne.w $56932
pub fn i_05692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56930;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56932; return 10; }
    8
}
/// $056930: neg.w d1
pub fn i_056930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56932;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056932: add.w $22(a6), d1
pub fn i_056932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56936;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056936: move.w $24(a6), d2
pub fn i_056936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5693A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05693A: subq.w #$8, d2
pub fn i_05693a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5693C;
    cpu.ir = 0x5142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05693C: movem.w d1-d2, -(a7)
pub fn i_05693c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56940;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $056940: jsr $280c6.l
pub fn i_056940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56946);
    cpu.pc = 0x280C6;
    16
}
/// $056946: movem.w (a7)+, d1-d2
pub fn i_056946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5694A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $05694A: bcc.w $56962
pub fn i_05694a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5694E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56962; return 10; }
    8
}
/// $05694E: move.w $2a(a6), d0
pub fn i_05694e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56952;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056952: bpl.w $5695a
pub fn i_056952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56956;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5695A; return 10; }
    8
}
/// $056956: clr.w $2a(a6)
pub fn i_056956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5695A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05695A: neg.w $36(a6)
pub fn i_05695a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5695E;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05695E: bra.w $56972
pub fn i_05695e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56962;
    cpu.ir = 0x6000;
    cpu.pc = 0x56972;
    10
}
/// $056962: addq.w #$8, d2
pub fn i_056962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56964;
    cpu.ir = 0x5042;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056964: jsr $280c6.l
pub fn i_056964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5696A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5696A);
    cpu.pc = 0x280C6;
    16
}
/// $05696A: bcc.w $56972
pub fn i_05696a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5696E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56972; return 10; }
    8
}
/// $05696E: clr.w $36(a6)
pub fn i_05696e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56972;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056972: move.w $36(a6), d0
pub fn i_056972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56976;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056976: sub.w $2a(a6), d0
pub fn i_056976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5697A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05697A: asr.w #$5, d0
pub fn i_05697a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5697C;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05697C: move.w d0, $2e(a6)
pub fn i_05697c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56980;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056980: clr.w $70(a6)
pub fn i_056980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56984;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056984: move.b $2a(a6), d0
pub fn i_056984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56988;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056988: addq.b #$1, d0
pub fn i_056988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5698A;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05698A: andi.b #$fc, d0
pub fn i_05698a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5698E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xFC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05698E: beq.w $569a6
pub fn i_05698e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56992;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x569A6; return 10; }
    8
}
/// $056992: bmi.w $569a0
pub fn i_056992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56996;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x569A0; return 10; }
    8
}
/// $056996: move.w #$1, $70(a6)
pub fn i_056996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5699C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05699C: bra.w $569a6
pub fn i_05699c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x569A6;
    10
}
/// $0569A0: move.w #$2, $70(a6)
pub fn i_0569a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569A6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0569A6: jsr $28d70.l
pub fn i_0569a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x569AC);
    cpu.pc = 0x28D70;
    16
}
/// $0569AC: movea.l #$ffffffff, a0
pub fn i_0569ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569B2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0569B2: lea.l $55cc8(pc), a0
pub fn i_0569b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569B6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x55CC8;
    16
}
/// $0569B6: jsr $5dd56.l
pub fn i_0569b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x569BC);
    cpu.pc = 0x5DD56;
    16
}
/// $0569BC: bcs.w $56a06
pub fn i_0569bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569C0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56A06; return 10; }
    8
}
/// $0569C0: jsr $283d8.l
pub fn i_0569c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x569C6);
    cpu.pc = 0x283D8;
    16
}
/// $0569C6: btst.b #$1, $13(a6)
pub fn i_0569c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569CC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0569CC: bne.w $56a0c
pub fn i_0569cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56A0C; return 10; }
    8
}
/// $0569D0: jsr $28758.l
pub fn i_0569d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x569D6);
    cpu.pc = 0x28758;
    16
}
/// $0569D6: bcs.w $56a0c
pub fn i_0569d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56A0C; return 10; }
    8
}
/// $0569DA: btst.b #$5, $5a(a6)
pub fn i_0569da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569E0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0569E0: bne.w $56a0c
pub fn i_0569e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56A0C; return 10; }
    8
}
/// $0569E4: jsr $2870a.l
pub fn i_0569e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x569EA);
    cpu.pc = 0x2870A;
    16
}
/// $0569EA: bcc.w $56a04
pub fn i_0569ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56A04; return 10; }
    8
}
/// $0569EE: move.w #$108d, d0
pub fn i_0569ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569F2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0569F2: jsr $2352.l
pub fn i_0569f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x569F8);
    cpu.pc = 0x2352;
    16
}
/// $0569F8: bclr.b #$3, $13(a6)
pub fn i_0569f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x569FE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0569FE: addi.w #$300, $2a(a6)
pub fn i_0569fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A04;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x300, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $056A04: rts
pub fn i_056a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_056a06  ($056A06, 6 B, thunks/jmp_abs_thunks.c)
/// $056A06: jmp $518.l
pub fn i_056a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A0C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Roller_Explode_056a0c  ($056A0C, 44 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056A0C: move.w #$1022, d0
pub fn i_056a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A10;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056A10: jsr $2352.l
pub fn i_056a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56A16);
    cpu.pc = 0x2352;
    16
}
/// $056A16: jsr $13600.l
pub fn i_056a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56A1C);
    cpu.pc = 0x13600;
    16
}
/// $056A1C: move.w #$2000, d0
pub fn i_056a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056A20: jsr $28134.l
pub fn i_056a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56A26);
    cpu.pc = 0x28134;
    16
}
/// $056A26: andi.w #$ffe3, $38(a6)
pub fn i_056a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A2C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $056A2C: ori.w #$10, $38(a6)
pub fn i_056a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A32;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $056A32: jmp $77f6a.l
pub fn i_056a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A38;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Entity_CmpDepthToParent_056a38  ($056A38, 16 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056A38: movea.l $8(a6), a1
pub fn i_056a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A3C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $056A3C: move.b $10(a6), d0
pub fn i_056a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A40;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056A40: cmp.b $10(a1), d0
pub fn i_056a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A44;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $056A44: bcs.w $56a4e
pub fn i_056a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A48;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56A4E; return 10; }
    8
}
// ===== ClearXN_056a48  ($056A48, 6 B, thunks/ccr_helpers.c)
/// $056A48: andi.b #$ee, ccr
pub fn i_056a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A4C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $056A4C: rts
pub fn i_056a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_056a4e  ($056A4E, 6 B, thunks/ccr_helpers.c)
/// $056A4E: ori.b #$11, ccr
pub fn i_056a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A52;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $056A52: rts
pub fn i_056a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_056a54  ($056A54, 16 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056A54: movea.l $8(a6), a1
pub fn i_056a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A58;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $056A58: move.b $10(a6), d0
pub fn i_056a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A5C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056A5C: cmp.b $10(a1), d0
pub fn i_056a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A60;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $056A60: bcs.w $56a6a
pub fn i_056a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A64;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56A6A; return 10; }
    8
}
// ===== ClearXN_056a64  ($056A64, 6 B, thunks/ccr_helpers.c)
/// $056A64: andi.b #$ee, ccr
pub fn i_056a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A68;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $056A68: rts
pub fn i_056a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_056a6a  ($056A6A, 6 B, thunks/ccr_helpers.c)
/// $056A6A: ori.b #$11, ccr
pub fn i_056a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A6E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $056A6E: rts
pub fn i_056a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_056a70  ($056A70, 16 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056A70: movea.l $8(a6), a1
pub fn i_056a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A74;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $056A74: move.b $10(a6), d0
pub fn i_056a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A78;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056A78: cmp.b $10(a1), d0
pub fn i_056a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A7C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $056A7C: bcs.w $56a86
pub fn i_056a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A80;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56A86; return 10; }
    8
}
// ===== ClearXN_056a80  ($056A80, 6 B, thunks/ccr_helpers.c)
/// $056A80: andi.b #$ee, ccr
pub fn i_056a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A84;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $056A84: rts
pub fn i_056a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_056a86  ($056A86, 6 B, thunks/ccr_helpers.c)
/// $056A86: ori.b #$11, ccr
pub fn i_056a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A8A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $056A8A: rts
pub fn i_056a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_056a8c  ($056A8C, 16 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056A8C: movea.l $8(a6), a1
pub fn i_056a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A90;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $056A90: move.b $10(a6), d0
pub fn i_056a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A94;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056A94: cmp.b $10(a1), d0
pub fn i_056a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A98;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $056A98: bcs.w $56aa2
pub fn i_056a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56A9C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56AA2; return 10; }
    8
}
// ===== ClearXN_056a9c  ($056A9C, 6 B, thunks/ccr_helpers.c)
/// $056A9C: andi.b #$ee, ccr
pub fn i_056a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AA0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $056AA0: rts
pub fn i_056aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_056aa2  ($056AA2, 6 B, thunks/ccr_helpers.c)
/// $056AA2: ori.b #$11, ccr
pub fn i_056aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AA6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $056AA6: rts
pub fn i_056aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_056aa8  ($056AA8, 16 B, enemies/projectiles_grenade_mortar_055b96.s)
/// $056AA8: movea.l $8(a6), a1
pub fn i_056aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AAC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $056AAC: move.b $10(a6), d0
pub fn i_056aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AB0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056AB0: cmp.b $10(a1), d0
pub fn i_056ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AB4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $056AB4: bcs.w $56abe
pub fn i_056ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AB8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56ABE; return 10; }
    8
}
// ===== ClearXN_056ab8  ($056AB8, 6 B, thunks/ccr_helpers.c)
/// $056AB8: andi.b #$ee, ccr
pub fn i_056ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ABC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $056ABC: rts
pub fn i_056abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ABE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_056abe  ($056ABE, 6 B, thunks/ccr_helpers.c)
/// $056ABE: ori.b #$11, ccr
pub fn i_056abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AC2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $056AC2: rts
pub fn i_056ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_PhysicsStep_056acc  ($056ACC, 70 B, enemies/soldiers/helpers_056acc.s)
/// $056ACC: jsr $27f08.l
pub fn i_056acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56AD2);
    cpu.pc = 0x27F08;
    16
}
/// $056AD2: bcc.w $56ade
pub fn i_056ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56ADE; return 10; }
    8
}
/// $056AD6: move.b d3, $78(a6)
pub fn i_056ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ADA;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $056ADA: bra.w $56af4
pub fn i_056ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ADE;
    cpu.ir = 0x6000;
    cpu.pc = 0x56AF4;
    10
}
/// $056ADE: cmp.b $78(a6), d0
pub fn i_056ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AE2;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $056AE2: bne.w $56aee
pub fn i_056ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AE6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56AEE; return 10; }
    8
}
/// $056AE6: move.b d3, $78(a6)
pub fn i_056ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AEA;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $056AEA: bra.w $56af4
pub fn i_056aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x56AF4;
    10
}
/// $056AEE: move.b #$ff, $78(a6)
pub fn i_056aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $056AF4: tst.b $78(a6)
pub fn i_056af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AF8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $056AF8: beq.w $56b06
pub fn i_056af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56AFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56B06; return 10; }
    8
}
/// $056AFC: jsr $28292.l
pub fn i_056afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56B02);
    cpu.pc = 0x28292;
    16
}
/// $056B02: bra.w $56b10
pub fn i_056b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B06;
    cpu.ir = 0x6000;
    cpu.pc = 0x56B10;
    10
}
/// $056B06: jsr $28364.l
pub fn i_056b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56B0C);
    cpu.pc = 0x28364;
    16
}
/// $056B0C: scs.b $78(a6)
pub fn i_056b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B10;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $056B10: rts
pub fn i_056b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ApproxDist_056b12  ($056B12, 38 B, enemies/soldiers/helpers_056acc.s)
/// $056B12: ext.l d0
pub fn i_056b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B14;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B14: move.l d0, d2
pub fn i_056b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B16;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B16: swap d2
pub fn i_056b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B18;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B18: eor.w d2, d0
pub fn i_056b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B1A;
    cpu.ir = 0xB540;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056B1A: ext.l d1
pub fn i_056b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B1C;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B1C: move.l d1, d2
pub fn i_056b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B1E;
    cpu.ir = 0x2401;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B1E: swap d2
pub fn i_056b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B20;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B20: eor.w d2, d1
pub fn i_056b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B22;
    cpu.ir = 0xB541;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056B22: cmp.w d0, d1
pub fn i_056b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B24;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $056B24: bls.w $56b32
pub fn i_056b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B28;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x56B32; return 10; }
    8
}
/// $056B28: lsr.w #$1, d0
pub fn i_056b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B2A;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $056B2A: add.w d1, d0
pub fn i_056b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B2C;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056B2C: rts
pub fn i_056b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $056B2E: bra.w $56b38
pub fn i_056b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B32;
    cpu.ir = 0x6000;
    cpu.pc = 0x56B38;
    10
}
/// $056B32: lsr.w #$1, d1
pub fn i_056b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B34;
    cpu.ir = 0xE249;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $056B34: add.w d1, d0
pub fn i_056b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B36;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056B36: rts
pub fn i_056b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_FindNearestPlayer_056b38  ($056B38, 90 B, enemies/soldiers/helpers_056acc.s)
/// $056B38: clr.l -(a7)
pub fn i_056b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B3A;
    cpu.ir = 0x42A7;
    let ad1 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad1;
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    14
}
/// $056B3A: move.w #$ffff, -(a7)
pub fn i_056b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B3E;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $056B3E: moveq #$0, d0
pub fn i_056b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B40;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B40: jsr $5e3a2.l
pub fn i_056b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56B46);
    cpu.pc = 0x5E3A2;
    16
}
/// $056B46: bcc.w $56b62
pub fn i_056b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56B62; return 10; }
    8
}
/// $056B4A: move.w $22(a6), d0
pub fn i_056b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B4E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B4E: sub.w $22(a0), d0
pub fn i_056b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B52;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056B52: move.w $24(a6), d1
pub fn i_056b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B56;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B56: sub.w $24(a6), d1
pub fn i_056b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B5A;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056B5A: bsr.b $56b12
pub fn i_056b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B5C;
    cpu.ir = 0x61B6;
    cpu.push32(bus, 0x56B5C);
    cpu.pc = 0x56B12;
    18
}
/// $056B5C: move.l a0, $2(a7)
pub fn i_056b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B60;
    cpu.ir = 0x2F48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[7].wrapping_add(0x2);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056B60: move.w d0, (a7)
pub fn i_056b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B62;
    cpu.ir = 0x3E80;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $056B62: moveq #$1, d0
pub fn i_056b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B64;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056B64: jsr $5e3a2.l
pub fn i_056b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56B6A);
    cpu.pc = 0x5E3A2;
    16
}
/// $056B6A: bcc.w $56b8c
pub fn i_056b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56B8C; return 10; }
    8
}
/// $056B6E: move.w $22(a6), d0
pub fn i_056b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B72;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B72: sub.w $22(a0), d0
pub fn i_056b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B76;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056B76: move.w $24(a6), d1
pub fn i_056b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B7A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B7A: sub.w $24(a0), d1
pub fn i_056b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B7E;
    cpu.ir = 0x9268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056B7E: bsr.b $56b12
pub fn i_056b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B80;
    cpu.ir = 0x6192;
    cpu.push32(bus, 0x56B80);
    cpu.pc = 0x56B12;
    18
}
/// $056B80: cmp.w (a7), d0
pub fn i_056b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B82;
    cpu.ir = 0xB057;
    let v2 = bus.read16(cpu.a[7]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    8
}
/// $056B82: bhi.w $56b8c
pub fn i_056b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B86;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x56B8C; return 10; }
    8
}
/// $056B86: move.w d0, (a7)
pub fn i_056b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B88;
    cpu.ir = 0x3E80;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $056B88: move.l a0, $2(a7)
pub fn i_056b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B8C;
    cpu.ir = 0x2F48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[7].wrapping_add(0x2);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056B8C: move.w (a7)+, d0
pub fn i_056b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B8E;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $056B8E: movea.l (a7)+, a0
pub fn i_056b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B90;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $056B90: rts
pub fn i_056b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_Think_056b92  ($056B92, 646 B, enemies/soldiers/helpers_056acc.s)
/// $056B92: move.w $8c(a6), d0
pub fn i_056b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B96;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056B96: beq.w $56ba0
pub fn i_056b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56BA0; return 10; }
    8
}
/// $056B9A: subq.w #$1, d0
pub fn i_056b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56B9C;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056B9C: move.w d0, $8c(a6)
pub fn i_056b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BA0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056BA0: move.w $8e(a6), d0
pub fn i_056ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BA4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056BA4: beq.w $56bae
pub fn i_056ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56BAE; return 10; }
    8
}
/// $056BA8: subq.w #$1, d0
pub fn i_056ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BAA;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056BAA: move.w d0, $8e(a6)
pub fn i_056baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BAE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056BAE: bclr.b #$5, $72(a6)
pub fn i_056bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BB4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056BB4: bclr.b #$6, $72(a6)
pub fn i_056bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BBA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056BBA: btst.b #$5, $5a(a6)
pub fn i_056bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BC0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056BC0: beq.w $56bfc
pub fn i_056bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56BFC; return 10; }
    8
}
/// $056BC4: move.w $22(a6), d1
pub fn i_056bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BC8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056BC8: move.w $24(a6), d2
pub fn i_056bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BCC;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056BCC: btst.b #$0, $3a(a6)
pub fn i_056bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BD2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056BD2: bne.w $56bdc
pub fn i_056bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56BDC; return 10; }
    8
}
/// $056BD6: subq.w #$3, d1
pub fn i_056bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BD8;
    cpu.ir = 0x5741;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056BD8: bra.w $56bde
pub fn i_056bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x56BDE;
    10
}
/// $056BDC: addq.w #$3, d1
pub fn i_056bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BDE;
    cpu.ir = 0x5641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056BDE: addi.w #$18, d2
pub fn i_056bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BE2;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056BE2: jsr $280c6.l
pub fn i_056be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56BE8);
    cpu.pc = 0x280C6;
    16
}
/// $056BE8: bcc.w $56bf6
pub fn i_056be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56BF6; return 10; }
    8
}
/// $056BEC: bset.b #$5, $72(a6)
pub fn i_056bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BF2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056BF2: bra.w $56bfc
pub fn i_056bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x56BFC;
    10
}
/// $056BF6: bset.b #$6, $72(a6)
pub fn i_056bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56BFC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056BFC: bsr.w $56b38
pub fn i_056bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C00;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x56C00);
    cpu.pc = 0x56B38;
    18
}
/// $056C00: move.w d0, $7e(a6)
pub fn i_056c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C04;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056C04: move.l a0, d0
pub fn i_056c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C06;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056C06: beq.w $56df0
pub fn i_056c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C0A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56DF0; return 10; }
    8
}
/// $056C0A: bclr.b #$0, $73(a6)
pub fn i_056c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C10;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C10: move.l a0, $7a(a6)
pub fn i_056c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C14;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056C14: move.w $22(a6), d0
pub fn i_056c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C18;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056C18: sub.w $22(a0), d0
pub fn i_056c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C1C;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $056C1C: bmi.w $56c44
pub fn i_056c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C20;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56C44; return 10; }
    8
}
/// $056C20: bclr.b #$1, $72(a6)
pub fn i_056c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C26;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C26: btst.b #$0, $3a(a6)
pub fn i_056c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C2C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056C2C: bne.w $56c3a
pub fn i_056c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56C3A; return 10; }
    8
}
/// $056C30: bset.b #$0, $72(a6)
pub fn i_056c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C36;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C36: bra.w $56c40
pub fn i_056c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x56C40;
    10
}
/// $056C3A: bclr.b #$0, $72(a6)
pub fn i_056c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C40;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C40: bra.w $56c64
pub fn i_056c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C44;
    cpu.ir = 0x6000;
    cpu.pc = 0x56C64;
    10
}
/// $056C44: bset.b #$1, $72(a6)
pub fn i_056c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C4A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C4A: btst.b #$0, $3a(a6)
pub fn i_056c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C50;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056C50: bne.w $56c5e
pub fn i_056c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56C5E; return 10; }
    8
}
/// $056C54: bclr.b #$0, $72(a6)
pub fn i_056c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C5A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C5A: bra.w $56c64
pub fn i_056c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x56C64;
    10
}
/// $056C5E: bset.b #$0, $72(a6)
pub fn i_056c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C64;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C64: clr.l $88(a6)
pub fn i_056c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C68;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $056C68: clr.l $84(a6)
pub fn i_056c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C6C;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $056C6C: jsr $8f344.l
pub fn i_056c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56C72);
    cpu.pc = 0x8F344;
    16
}
/// $056C72: bcc.w $56ce0
pub fn i_056c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56CE0; return 10; }
    8
}
/// $056C76: tst.b d0
pub fn i_056c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C78;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $056C78: bpl.w $56c92
pub fn i_056c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C7C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56C92; return 10; }
    8
}
/// $056C7C: move.w #$400, $84(a6)
pub fn i_056c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056C82: bset.b #$2, $73(a6)
pub fn i_056c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C88;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C88: bclr.b #$3, $73(a6)
pub fn i_056c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C8E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056C8E: bra.w $56cd6
pub fn i_056c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C92;
    cpu.ir = 0x6000;
    cpu.pc = 0x56CD6;
    10
}
/// $056C92: jsr $5e9b6.l
pub fn i_056c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56C98);
    cpu.pc = 0x5E9B6;
    16
}
/// $056C98: tst.b d0
pub fn i_056c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C9A;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $056C9A: bmi.w $56cb4
pub fn i_056c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56C9E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56CB4; return 10; }
    8
}
/// $056C9E: move.w #$400, $86(a6)
pub fn i_056c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CA4: bclr.b #$2, $73(a6)
pub fn i_056ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CAA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056CAA: bset.b #$3, $73(a6)
pub fn i_056caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CB0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056CB0: bra.w $56cd6
pub fn i_056cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x56CD6;
    10
}
/// $056CB4: andi.w #$3ff, d0
pub fn i_056cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CB8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056CB8: move.w d0, $84(a6)
pub fn i_056cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CBC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CBC: jsr $5e9b6.l
pub fn i_056cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56CC2);
    cpu.pc = 0x5E9B6;
    16
}
/// $056CC2: andi.w #$3ff, d0
pub fn i_056cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CC6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056CC6: move.w d0, $86(a6)
pub fn i_056cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CCA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CCA: move.w #$80, $88(a6)
pub fn i_056cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CD0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CD0: move.w #$80, $8a(a6)
pub fn i_056cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056CD6: bset.b #$1, $73(a6)
pub fn i_056cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CDC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056CDC: bra.w $56ce6
pub fn i_056cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x56CE6;
    10
}
/// $056CE0: andi.b #$f1, $73(a6)
pub fn i_056ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CE6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056CE6: move.b $98(a6), d0
pub fn i_056ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CEA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056CEA: beq.w $56d16
pub fn i_056cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56D16; return 10; }
    8
}
/// $056CEE: cmpi.b #$1, d0
pub fn i_056cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CF2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $056CF2: beq.w $56d0c
pub fn i_056cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56D0C; return 10; }
    8
}
/// $056CF6: cmpi.b #$2, d0
pub fn i_056cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CFA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $056CFA: beq.w $56d20
pub fn i_056cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56CFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56D20; return 10; }
    8
}
/// $056CFE: jsr $5e9b6.l
pub fn i_056cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56D04);
    cpu.pc = 0x5E9B6;
    16
}
/// $056D04: btst.b #$0, d0
pub fn i_056d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D08;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $056D08: beq.w $56d16
pub fn i_056d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D0C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56D16; return 10; }
    8
}
/// $056D0C: ori.b #$30, $13(a6)
pub fn i_056d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D12;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x30) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D12: bra.w $56d44
pub fn i_056d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D16;
    cpu.ir = 0x6000;
    cpu.pc = 0x56D44;
    10
}
/// $056D16: andi.b #$cf, $13(a6)
pub fn i_056d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D1C;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xCF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D1C: bra.w $56d44
pub fn i_056d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D20;
    cpu.ir = 0x6000;
    cpu.pc = 0x56D44;
    10
}
/// $056D20: move.b $70(a6), d0
pub fn i_056d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D24;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056D24: bmi.w $56d38
pub fn i_056d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D28;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56D38; return 10; }
    8
}
/// $056D28: bset.b #$4, $13(a6)
pub fn i_056d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D2E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D2E: bclr.b #$5, $13(a6)
pub fn i_056d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D34;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D34: bra.w $56d44
pub fn i_056d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D38;
    cpu.ir = 0x6000;
    cpu.pc = 0x56D44;
    10
}
/// $056D38: bset.b #$5, $13(a6)
pub fn i_056d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D3E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D3E: bclr.b #$4, $13(a6)
pub fn i_056d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D44;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056D44: move.w $7e(a6), d0
pub fn i_056d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D48;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056D48: move.w d0, d1
pub fn i_056d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D4A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056D4A: move.w $90(a6), d2
pub fn i_056d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D4E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056D4E: move.b $90(a6), d3
pub fn i_056d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D52;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056D52: andi.w #$1f, d3
pub fn i_056d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D56;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D56: andi.w #$1f, d2
pub fn i_056d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D5A;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D5A: add.w d3, d2
pub fn i_056d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D5C;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056D5C: addi.w #$10, d2
pub fn i_056d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D60;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D60: sub.w d2, d0
pub fn i_056d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D62;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056D62: bpl.w $56d68
pub fn i_056d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D66;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56D68; return 10; }
    8
}
/// $056D66: clr.w d0
pub fn i_056d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D68;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056D68: lsl.w #$2, d0
pub fn i_056d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D6A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $056D6A: cmpi.w #$100, d0
pub fn i_056d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D6E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $056D6E: bcs.w $56d76
pub fn i_056d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D72;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56D76; return 10; }
    8
}
/// $056D72: move.w #$100, d0
pub fn i_056d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D76;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056D76: neg.w d0
pub fn i_056d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D78;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056D78: addi.w #$100, d0
pub fn i_056d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D7C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D7C: move.w d0, $80(a6)
pub fn i_056d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D80;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056D80: moveq #$0, d0
pub fn i_056d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D82;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056D82: move.w d1, d0
pub fn i_056d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D84;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056D84: move.w $90(a6), d2
pub fn i_056d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D88;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056D88: move.b $90(a6), d3
pub fn i_056d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D8C;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056D8C: andi.w #$1f, d3
pub fn i_056d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D90;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D90: andi.w #$1f, d2
pub fn i_056d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D94;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D94: add.w d3, d2
pub fn i_056d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D96;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056D96: addi.w #$10, d2
pub fn i_056d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D9A;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056D9A: sub.w d2, d0
pub fn i_056d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56D9C;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056D9C: bpl.w $56da2
pub fn i_056d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DA0;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56DA2; return 10; }
    8
}
/// $056DA0: clr.l d0
pub fn i_056da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DA2;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $056DA2: move.b $9c(a6), d7
pub fn i_056da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DA6;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056DA6: andi.l #$7, d7
pub fn i_056da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DAC;
    cpu.ir = 0x287;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = (v1 & 0x7) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[7] = r2;
    8
}
/// $056DAC: beq.w $56db2
pub fn i_056dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56DB2; return 10; }
    8
}
/// $056DB0: lsl.l d7, d0
pub fn i_056db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DB2;
    cpu.ir = 0xEFA8;
    let cnt = cpu.d[7] & 0x3F;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 1, true);
    cpu.d[0] = r;
    6 + 2 * cnt
}
/// $056DB2: lsl.l #$1, d0
pub fn i_056db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DB4;
    cpu.ir = 0xE388;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 1, true);
    cpu.d[0] = r;
    8
}
/// $056DB4: cmpi.l #$100, d0
pub fn i_056db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DBA;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Long, false);
    8
}
/// $056DBA: bcs.w $56dc2
pub fn i_056dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DBE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56DC2; return 10; }
    8
}
/// $056DBE: move.w #$100, d0
pub fn i_056dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DC2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056DC2: move.w d0, $82(a6)
pub fn i_056dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DC6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056DC6: move.w $22(a6), d0
pub fn i_056dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DCA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056DCA: cmpi.w #$30, d0
pub fn i_056dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DCE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $056DCE: bmi.w $56ddc
pub fn i_056dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DD2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56DDC; return 10; }
    8
}
/// $056DD2: cmpi.w #$120, d0
pub fn i_056dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DD6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $056DD6: bpl.w $56ddc
pub fn i_056dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DDA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56DDC; return 10; }
    8
}
/// $056DDA: rts
pub fn i_056dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $056DDC: move.w #$100, $82(a6)
pub fn i_056ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056DE2: clr.w $80(a6)
pub fn i_056de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DE6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056DE6: clr.w $84(a6)
pub fn i_056de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DEA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056DEA: clr.w $86(a6)
pub fn i_056dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DEE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $056DEE: rts
pub fn i_056dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $056DF0: bset.b #$0, $73(a6)
pub fn i_056df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DF6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056DF6: clr.l $7a(a6)
pub fn i_056df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56DFA;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $056DFA: bset.b #$0, $72(a6)
pub fn i_056dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E00;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $056E00: move.w #$20, d0
pub fn i_056e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056E04: move.w d0, $80(a6)
pub fn i_056e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E08;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E08: move.w d0, $82(a6)
pub fn i_056e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E0C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E0C: move.w d0, $86(a6)
pub fn i_056e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E10;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E10: move.w d0, $84(a6)
pub fn i_056e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E14;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E14: move.w d0, $88(a6)
pub fn i_056e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E18;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskW_056e18  ($056E18, 6 B, kernel/task_setters_w.c)
/// $056E18: move.w d0, $8a(a6)
pub fn i_056e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E1C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056E1C: rts
pub fn i_056e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_DespawnIfOffscreen_056e1e  ($056E1E, 24 B, enemies/soldiers/helpers_056acc.s)
/// $056E1E: lea.l $2b71d4.l, a0
pub fn i_056e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B71D4;
    20
}
/// $056E24: jsr $5dd56.l
pub fn i_056e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56E2A);
    cpu.pc = 0x5DD56;
    16
}
/// $056E2A: bcc.w $56e34
pub fn i_056e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56E34; return 10; }
    8
}
/// $056E2E: jmp $518.l
pub fn i_056e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E34;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $056E34: rts
pub fn i_056e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_LeaveTimerExpired_056e36  ($056E36, 8 B, enemies/soldiers/helpers_056acc.s)
/// $056E36: tst.w $8e(a6)
pub fn i_056e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E3A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $056E3A: beq.w $56e44
pub fn i_056e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56E44; return 10; }
    8
}
// ===== ClearC_056e3e  ($056E3E, 6 B, thunks/ccr_helpers.c)
/// $056E3E: andi.b #$fe, ccr
pub fn i_056e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E42;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056E42: rts
pub fn i_056e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_056e44  ($056E44, 6 B, thunks/ccr_helpers.c)
/// $056E44: ori.b #$1, ccr
pub fn i_056e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E48;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056E48: rts
pub fn i_056e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_PickGrabAnchor_056e4a  ($056E4A, 124 B, enemies/soldiers/helpers_056acc.s)
/// $056E4A: move.b $74(a6), d0
pub fn i_056e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E4E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056E4E: andi.b #$7, d0
pub fn i_056e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E52;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $056E52: beq.w $56ecc
pub fn i_056e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56ECC; return 10; }
    8
}
/// $056E56: btst.b #$0, $72(a6)
pub fn i_056e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E5C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056E5C: beq.w $56ecc
pub fn i_056e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56ECC; return 10; }
    8
}
/// $056E60: move.l $7a(a6), d0
pub fn i_056e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E64;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056E64: beq.w $56ecc
pub fn i_056e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56ECC; return 10; }
    8
}
/// $056E68: movea.l d0, a0
pub fn i_056e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E6A;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $056E6A: jsr $8f7ce.l
pub fn i_056e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56E70);
    cpu.pc = 0x8F7CE;
    16
}
/// $056E70: not.b d0
pub fn i_056e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E72;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $056E72: move.b $74(a6), d1
pub fn i_056e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E76;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056E76: andi.b #$7, d1
pub fn i_056e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E7A;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $056E7A: and.b d1, d0
pub fn i_056e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E7C;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $056E7C: beq.w $56ecc
pub fn i_056e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56ECC; return 10; }
    8
}
/// $056E80: clr.w d1
pub fn i_056e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E82;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056E82: btst.b #$0, d0
pub fn i_056e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E86;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $056E86: beq.w $56e90
pub fn i_056e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56E90; return 10; }
    8
}
/// $056E8A: move.w #$0, -(a7)
pub fn i_056e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E8E;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $056E8E: addq.w #$1, d1
pub fn i_056e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E90;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056E90: btst.b #$1, d0
pub fn i_056e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E94;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $056E94: beq.w $56e9e
pub fn i_056e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56E9E; return 10; }
    8
}
/// $056E98: move.w #$1, -(a7)
pub fn i_056e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E9C;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $056E9C: addq.w #$1, d1
pub fn i_056e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56E9E;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056E9E: btst.b #$2, d0
pub fn i_056e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EA2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $056EA2: beq.w $56eac
pub fn i_056ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56EAC; return 10; }
    8
}
/// $056EA6: move.w #$2, -(a7)
pub fn i_056ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EAA;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $056EAA: addq.w #$1, d1
pub fn i_056eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EAC;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $056EAC: move.w d1, d0
pub fn i_056eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EAE;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056EAE: movem.w d1, -(a7)
pub fn i_056eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EB2;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $056EB2: jsr $5e9e4.l
pub fn i_056eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56EB8);
    cpu.pc = 0x5E9E4;
    16
}
/// $056EB8: movem.w (a7)+, d1
pub fn i_056eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EBC;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $056EBC: add.w d0, d0
pub fn i_056ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EBE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056EBE: move.w (a7, d0.w), d0
pub fn i_056ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EC2;
    cpu.ir = 0x3037;
    let ad2 = cpu.a[7].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $056EC2: add.w d1, d1
pub fn i_056ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EC4;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $056EC4: adda.w d1, a7
pub fn i_056ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EC6;
    cpu.ir = 0xDEC1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[7] = cpu.a[7].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
// ===== SetC_056ec6  ($056EC6, 6 B, thunks/ccr_helpers.c)
/// $056EC6: ori.b #$1, ccr
pub fn i_056ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ECA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056ECA: rts
pub fn i_056eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ECC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_056ecc  ($056ECC, 6 B, thunks/ccr_helpers.c)
/// $056ECC: andi.b #$fe, ccr
pub fn i_056ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ED0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056ED0: rts
pub fn i_056ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56ED2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_TestPlayerInSlug_056eda  ($056EDA, 42 B, enemies/soldiers/helpers_056acc.s)
/// $056EDA: move.b $74(a6), d0
pub fn i_056eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EDE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056EDE: andi.b #$7, d0
pub fn i_056ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EE2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $056EE2: beq.w $56f0a
pub fn i_056ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EE6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F0A; return 10; }
    8
}
/// $056EE6: btst.b #$0, $72(a6)
pub fn i_056ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EEC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056EEC: beq.w $56f0a
pub fn i_056eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F0A; return 10; }
    8
}
/// $056EF0: move.l $7a(a6), d0
pub fn i_056ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EF4;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056EF4: beq.w $56f0a
pub fn i_056ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EF8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F0A; return 10; }
    8
}
/// $056EF8: movea.l d0, a0
pub fn i_056ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56EFA;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $056EFA: jsr $2ac0e.l
pub fn i_056efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56F00);
    cpu.pc = 0x2AC0E;
    16
}
/// $056F00: bcs.w $56f0a
pub fn i_056f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F04;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56F0A; return 10; }
    8
}
// ===== SetC_056f04  ($056F04, 6 B, thunks/ccr_helpers.c)
/// $056F04: ori.b #$1, ccr
pub fn i_056f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F08;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056F08: rts
pub fn i_056f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F0A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_056f0a  ($056F0A, 6 B, thunks/ccr_helpers.c)
/// $056F0A: andi.b #$fe, ccr
pub fn i_056f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F0E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056F0E: rts
pub fn i_056f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_GrabStruggleProgress_056f10  ($056F10, 72 B, enemies/soldiers/helpers_056acc.s)
/// $056F10: move.w $80(a6), d0
pub fn i_056f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F14;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056F14: move.w d0, $82(a6)
pub fn i_056f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F18;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056F18: beq.w $56f28
pub fn i_056f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F1C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F28; return 10; }
    8
}
/// $056F1C: asr.w #$6, d0
pub fn i_056f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F1E;
    cpu.ir = 0xEC40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $056F1E: bne.w $56f24
pub fn i_056f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F22;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56F24; return 10; }
    8
}
/// $056F22: moveq #$1, d0
pub fn i_056f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F24;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $056F24: sub.w d0, $80(a6)
pub fn i_056f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F28;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $056F28: move.l $7a(a6), d7
pub fn i_056f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F2C;
    cpu.ir = 0x2E2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $056F2C: beq.w $56f5e
pub fn i_056f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F5E; return 10; }
    8
}
/// $056F30: movea.l d7, a0
pub fn i_056f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F32;
    cpu.ir = 0x2047;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $056F32: movea.l $c(a0), a0
pub fn i_056f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F36;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $056F36: movea.l $72(a0), a0
pub fn i_056f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F3A;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $056F3A: move.b $3(a0), d0
pub fn i_056f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F3E;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056F3E: beq.w $56f5e
pub fn i_056f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F42;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F5E; return 10; }
    8
}
/// $056F42: move.w $80(a6), d0
pub fn i_056f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056F46: move.w d0, d1
pub fn i_056f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F48;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $056F48: addi.w #$100, d0
pub fn i_056f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F4C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $056F4C: move.w d0, $80(a6)
pub fn i_056f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F50;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056F50: cmpi.w #$b00, d0
pub fn i_056f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F54;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB00, Size::Word, false);
    8
}
/// $056F54: bcs.w $56f5e
pub fn i_056f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F58;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56F5E; return 10; }
    8
}
// ===== SetC_056f58  ($056F58, 6 B, thunks/ccr_helpers.c)
/// $056F58: ori.b #$1, ccr
pub fn i_056f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F5C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056F5C: rts
pub fn i_056f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_056f5e  ($056F5E, 6 B, thunks/ccr_helpers.c)
/// $056F5E: andi.b #$fe, ccr
pub fn i_056f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F62;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056F62: rts
pub fn i_056f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_TestGrabBreak_056f64  ($056F64, 26 B, enemies/soldiers/helpers_056acc.s)
/// $056F64: move.w $80(a6), d0
pub fn i_056f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F68;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056F68: cmpi.w #$300, d0
pub fn i_056f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F6C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $056F6C: bcc.w $56f84
pub fn i_056f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x56F84; return 10; }
    8
}
/// $056F70: tst.w d0
pub fn i_056f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F72;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $056F72: beq.w $56f7e
pub fn i_056f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F7E; return 10; }
    8
}
/// $056F76: tst.w $82(a6)
pub fn i_056f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F7A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $056F7A: beq.w $56f84
pub fn i_056f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56F84; return 10; }
    8
}
// ===== ClearC_056f7e  ($056F7E, 6 B, thunks/ccr_helpers.c)
/// $056F7E: andi.b #$fe, ccr
pub fn i_056f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F82;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056F82: rts
pub fn i_056f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_056f84  ($056F84, 6 B, thunks/ccr_helpers.c)
/// $056F84: ori.b #$1, ccr
pub fn i_056f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F88;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056F88: rts
pub fn i_056f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_SetVelXByFacing_056f8a  ($056F8A, 16 B, enemies/soldiers/helpers_056acc.s)
/// $056F8A: move.w $36(a6), d0
pub fn i_056f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F8E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056F8E: btst.b #$0, $3a(a6)
pub fn i_056f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F94;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056F94: bne.w $56f9a
pub fn i_056f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56F9A; return 10; }
    8
}
/// $056F98: neg.w d0
pub fn i_056f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F9A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_056f9a  ($056F9A, 6 B, kernel/task_setters_w.c)
/// $056F9A: move.w d0, $28(a6)
pub fn i_056f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56F9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $056F9E: rts
pub fn i_056f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_TestSurrender_056fa0  ($056FA0, 64 B, enemies/soldiers/helpers_056acc.s)
/// $056FA0: btst.b #$3, $74(a6)
pub fn i_056fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FA6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056FA6: beq.w $56fe6
pub fn i_056fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FAA: move.w $22(a6), d0
pub fn i_056faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FAE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $056FAE: cmpi.w #$30, d0
pub fn i_056fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FB2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $056FB2: bmi.w $56fe6
pub fn i_056fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FB6;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FB6: cmpi.w #$120, d0
pub fn i_056fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FBA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $056FBA: bpl.w $56fe6
pub fn i_056fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FBE;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FBE: tst.w $8c(a6)
pub fn i_056fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FC2;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $056FC2: bne.w $56fe6
pub fn i_056fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FC6: btst.b #$0, $72(a6)
pub fn i_056fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FCC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056FCC: beq.w $56fe6
pub fn i_056fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FD0: jsr $8f2a0.l
pub fn i_056fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x56FD6);
    cpu.pc = 0x8F2A0;
    16
}
/// $056FD6: bcs.w $56fe6
pub fn i_056fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FDA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x56FE6; return 10; }
    8
}
/// $056FDA: move.b $9a(a6), $8d(a6)
pub fn i_056fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FE0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8D);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetC_056fe0  ($056FE0, 6 B, thunks/ccr_helpers.c)
/// $056FE0: ori.b #$1, ccr
pub fn i_056fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FE4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $056FE4: rts
pub fn i_056fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_056fe6  ($056FE6, 6 B, thunks/ccr_helpers.c)
/// $056FE6: andi.b #$fe, ccr
pub fn i_056fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FEA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $056FEA: rts
pub fn i_056fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ProbeWalkEdge_056fec  ($056FEC, 54 B, enemies/soldiers/helpers_056acc.s)
/// $056FEC: btst.b #$0, $3a(a6)
pub fn i_056fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FF2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $056FF2: seq.b d1
pub fn i_056ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FF4;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $056FF4: move.b $28(a6), d2
pub fn i_056ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FF8;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $056FF8: smi.b d2
pub fn i_056ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FFA;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $056FFA: eor.b d2, d1
pub fn i_056ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x56FFC;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $056FFC: bne.w $57042
pub fn i_056ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57000;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x57042; return 10; }
    8
}
/// $057000: jsr $770cc.l
pub fn i_057000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x57006);
    cpu.pc = 0x770CC;
    16
}
/// $057006: tst.w d0
pub fn i_057006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57008;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $057008: bmi.w $57042
pub fn i_057008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5700C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x57042; return 10; }
    8
}
/// $05700C: move.b d0, $77(a6)
pub fn i_05700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57010;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057010: btst.b #$4, $13(a6)
pub fn i_057010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57016;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057016: sne.b d2
pub fn i_057016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57018;
    cpu.ir = 0x56C2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $057018: and.b d1, d2
pub fn i_057018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5701A;
    cpu.ir = 0xC401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05701A: andi.b #$1, d2
pub fn i_05701a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5701E;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05701E: beq.w $5702a
pub fn i_05701e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57022;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5702A; return 10; }
    8
}
// ===== SetTaskHandler_057022  ($057022, 8 B, kernel/task_handlers.c)
/// $057022: lea.l $58b1e(pc), a1
pub fn i_057022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57026;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58B1E;
    16
}
/// $057026: move.l a1, (a6)
pub fn i_057026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57028;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057028: rts
pub fn i_057028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5702A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_ProbeWalkEdge_Bit5_05702a  ($05702A, 18 B, enemies/soldiers/helpers_056acc.s)
/// $05702A: btst.b #$5, $13(a6)
pub fn i_05702a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57030;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $057030: sne.b d2
pub fn i_057030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57032;
    cpu.ir = 0x56C2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $057032: and.b d1, d2
pub fn i_057032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57034;
    cpu.ir = 0xC401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $057034: andi.b #$2, d2
pub fn i_057034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57038;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $057038: beq.w $57042
pub fn i_057038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5703C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x57042; return 10; }
    8
}
// ===== SetTaskHandler_05703c  ($05703C, 8 B, kernel/task_handlers.c)
/// $05703C: lea.l $58c8e(pc), a1
pub fn i_05703c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57040;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x58C8E;
    16
}
/// $057040: move.l a1, (a6)
pub fn i_057040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57042;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $057042: rts
pub fn i_057042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57044;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityFrame_FrameSelectByGate3A_057044  ($057044, 54 B, entity/state_publishers_057044.s)
/// $057044: btst.b #$0, $3a(a6)
pub fn i_057044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5704A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05704A: bne.w $5705e
pub fn i_05704a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5704E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5705E; return 10; }
    8
}
/// $05704E: move.b #$4, d1
pub fn i_05704e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57052;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $057052: move.b #$5, d2
pub fn i_057052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57056;
    cpu.ir = 0x143C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $057056: move.b #$10, d3
pub fn i_057056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5705A;
    cpu.ir = 0x163C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05705A: bra.w $5706a
pub fn i_05705a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5705E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5706A;
    10
}
/// $05705E: move.b #$11, d1
pub fn i_05705e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57062;
    cpu.ir = 0x123C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $057062: move.b #$12, d2
pub fn i_057062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57066;
    cpu.ir = 0x143C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $057066: move.b #$13, d3
pub fn i_057066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5706A;
    cpu.ir = 0x163C;
    let v1: u32 = 0x13;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05706A: cmpi.w #$180, $24(a6)
pub fn i_05706a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57070;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $057070: bcs.w $5707a
pub fn i_057070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57074;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5707A; return 10; }
    8
}
/// $057074: move.b d3, $5c(a6)
pub fn i_057074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57078;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057078: rts
pub fn i_057078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5707A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntityFrame_FrameSelectByXDist_05707A  ($05707A, 46 B, entity/state_publishers_057044.s)
/// $05707A: tst.l d0
pub fn i_05707a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5707C;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05707C: beq.w $570a2
pub fn i_05707c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57080;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x570A2; return 10; }
    8
}
/// $057080: movea.l d0, a0
pub fn i_057080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57082;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $057082: move.w $24(a0), d0
pub fn i_057082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57086;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $057086: sub.w $24(a6), d0
pub fn i_057086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5708A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05708A: cmpi.w #$60, d0
pub fn i_05708a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5708E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $05708E: ble.w $57098
pub fn i_05708e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57092;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x57098; return 10; }
    8
}
/// $057092: move.b d2, d1
pub fn i_057092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57094;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $057094: bra.w $570a2
pub fn i_057094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57098;
    cpu.ir = 0x6000;
    cpu.pc = 0x570A2;
    10
}
/// $057098: cmpi.w #$ffc0, d0
pub fn i_057098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5709C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFC0, Size::Word, false);
    8
}
/// $05709C: bge.w $570a2
pub fn i_05709c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570A0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x570A2; return 10; }
    8
}
/// $0570A0: move.b d3, d1
pub fn i_0570a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570A2;
    cpu.ir = 0x1203;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0570A2: move.b d1, $5c(a6)
pub fn i_0570a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570A6;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0570A6: rts
pub fn i_0570a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Soldier_InitCommon_0570a8  ($0570A8, 204 B, enemies/soldiers/helpers_056acc.s)
/// $0570A8: jsr $27f60.l
pub fn i_0570a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x570AE);
    cpu.pc = 0x27F60;
    16
}
/// $0570AE: scc.b $78(a6)
pub fn i_0570ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570B2;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $0570B2: bset.b #$3, $6b(a6)
pub fn i_0570b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570B8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0570B8: lea.l $2b74ec.l, a0
pub fn i_0570b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B74EC;
    20
}
/// $0570BE: jsr $799de.l
pub fn i_0570be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x570C4);
    cpu.pc = 0x799DE;
    16
}
/// $0570C4: clr.w d1
pub fn i_0570c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570C6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0570C6: move.b $99(a6), d1
pub fn i_0570c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570CA;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0570CA: bne.w $570d0
pub fn i_0570ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x570D0; return 10; }
    8
}
/// $0570CE: moveq #$ff, d0
pub fn i_0570ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570D0;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0570D0: mulu.w d1, d0
pub fn i_0570d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570D2;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0570D2: lsr.l #$8, d0
pub fn i_0570d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570D4;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $0570D4: move.w d0, $8c(a6)
pub fn i_0570d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570D8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0570D8: lea.l $2b746a.l, a0
pub fn i_0570d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B746A;
    20
}
/// $0570DE: jsr $799de.l
pub fn i_0570de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x570E4);
    cpu.pc = 0x799DE;
    16
}
/// $0570E4: clr.w d1
pub fn i_0570e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570E6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0570E6: move.b $9a(a6), d1
pub fn i_0570e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570EA;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0570EA: bne.w $570f2
pub fn i_0570ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x570F2; return 10; }
    8
}
/// $0570EE: move.b #$1e, d1
pub fn i_0570ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570F2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0570F2: mulu.w d1, d0
pub fn i_0570f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570F4;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0570F4: lsr.l #$8, d0
pub fn i_0570f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570F6;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $0570F6: cmpi.w #$100, d0
pub fn i_0570f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570FA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $0570FA: bcs.w $57102
pub fn i_0570fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x570FE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x57102; return 10; }
    8
}
/// $0570FE: move.b #$ff, d0
pub fn i_0570fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57102;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $057102: move.b d0, $9a(a6)
pub fn i_057102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57106;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $057106: moveq #$0, d0
pub fn i_057106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x57108;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0552E2, i_0552e2),
    (0x0552E4, i_0552e4),
    (0x0552EA, i_0552ea),
    (0x0552F0, i_0552f0),
    (0x0552F2, i_0552f2),
    (0x0552F8, i_0552f8),
    (0x0552FE, i_0552fe),
    (0x055300, i_055300),
    (0x055306, i_055306),
    (0x05530C, i_05530c),
    (0x05530E, i_05530e),
    (0x055314, i_055314),
    (0x05531A, i_05531a),
    (0x05531C, i_05531c),
    (0x055322, i_055322),
    (0x055328, i_055328),
    (0x05532A, i_05532a),
    (0x055330, i_055330),
    (0x055336, i_055336),
    (0x055338, i_055338),
    (0x05533E, i_05533e),
    (0x055344, i_055344),
    (0x055346, i_055346),
    (0x05534C, i_05534c),
    (0x055352, i_055352),
    (0x055354, i_055354),
    (0x05535A, i_05535a),
    (0x055360, i_055360),
    (0x055362, i_055362),
    (0x055368, i_055368),
    (0x05536E, i_05536e),
    (0x055370, i_055370),
    (0x055376, i_055376),
    (0x05537C, i_05537c),
    (0x05537E, i_05537e),
    (0x055384, i_055384),
    (0x05538A, i_05538a),
    (0x05538C, i_05538c),
    (0x055392, i_055392),
    (0x055398, i_055398),
    (0x05539A, i_05539a),
    (0x0553A0, i_0553a0),
    (0x0553A6, i_0553a6),
    (0x0553A8, i_0553a8),
    (0x0553AE, i_0553ae),
    (0x0553B4, i_0553b4),
    (0x0553B6, i_0553b6),
    (0x0553BC, i_0553bc),
    (0x0553C2, i_0553c2),
    (0x0553C4, i_0553c4),
    (0x0553CA, i_0553ca),
    (0x0553D0, i_0553d0),
    (0x0553D2, i_0553d2),
    (0x0553D8, i_0553d8),
    (0x0553DE, i_0553de),
    (0x0553E0, i_0553e0),
    (0x0553E6, i_0553e6),
    (0x0553EC, i_0553ec),
    (0x0553EE, i_0553ee),
    (0x0553F4, i_0553f4),
    (0x0553FA, i_0553fa),
    (0x0553FC, i_0553fc),
    (0x055402, i_055402),
    (0x055408, i_055408),
    (0x05540A, i_05540a),
    (0x055410, i_055410),
    (0x055416, i_055416),
    (0x055418, i_055418),
    (0x05541E, i_05541e),
    (0x055424, i_055424),
    (0x055426, i_055426),
    (0x05542C, i_05542c),
    (0x055432, i_055432),
    (0x055434, i_055434),
    (0x05543A, i_05543a),
    (0x055440, i_055440),
    (0x055442, i_055442),
    (0x055448, i_055448),
    (0x05544E, i_05544e),
    (0x055450, i_055450),
    (0x055456, i_055456),
    (0x05545C, i_05545c),
    (0x05545E, i_05545e),
    (0x055464, i_055464),
    (0x05546A, i_05546a),
    (0x05546C, i_05546c),
    (0x055472, i_055472),
    (0x055478, i_055478),
    (0x05547A, i_05547a),
    (0x055480, i_055480),
    (0x055486, i_055486),
    (0x055488, i_055488),
    (0x05548E, i_05548e),
    (0x055494, i_055494),
    (0x055496, i_055496),
    (0x05549C, i_05549c),
    (0x0554A2, i_0554a2),
    (0x0554A4, i_0554a4),
    (0x0554AA, i_0554aa),
    (0x0554B0, i_0554b0),
    (0x0554B2, i_0554b2),
    (0x0554B8, i_0554b8),
    (0x0554BE, i_0554be),
    (0x0554C0, i_0554c0),
    (0x0554C6, i_0554c6),
    (0x0554CC, i_0554cc),
    (0x0554CE, i_0554ce),
    (0x0554D4, i_0554d4),
    (0x0554DA, i_0554da),
    (0x0554DC, i_0554dc),
    (0x0554E2, i_0554e2),
    (0x0554E8, i_0554e8),
    (0x0554EA, i_0554ea),
    (0x0554F0, i_0554f0),
    (0x0554F6, i_0554f6),
    (0x0554F8, i_0554f8),
    (0x0554FE, i_0554fe),
    (0x055504, i_055504),
    (0x055506, i_055506),
    (0x05550C, i_05550c),
    (0x055512, i_055512),
    (0x055514, i_055514),
    (0x05551A, i_05551a),
    (0x055520, i_055520),
    (0x055522, i_055522),
    (0x055528, i_055528),
    (0x05552E, i_05552e),
    (0x055530, i_055530),
    (0x055536, i_055536),
    (0x05553C, i_05553c),
    (0x05553E, i_05553e),
    (0x055544, i_055544),
    (0x05554A, i_05554a),
    (0x05554C, i_05554c),
    (0x055552, i_055552),
    (0x055558, i_055558),
    (0x05555A, i_05555a),
    (0x055560, i_055560),
    (0x055566, i_055566),
    (0x055568, i_055568),
    (0x05556E, i_05556e),
    (0x055574, i_055574),
    (0x055576, i_055576),
    (0x05557C, i_05557c),
    (0x055582, i_055582),
    (0x055584, i_055584),
    (0x05558A, i_05558a),
    (0x055590, i_055590),
    (0x055592, i_055592),
    (0x055598, i_055598),
    (0x05559E, i_05559e),
    (0x0555A0, i_0555a0),
    (0x0555A6, i_0555a6),
    (0x0555AC, i_0555ac),
    (0x0555AE, i_0555ae),
    (0x0555B4, i_0555b4),
    (0x0555BA, i_0555ba),
    (0x0555BC, i_0555bc),
    (0x0555C2, i_0555c2),
    (0x0555C8, i_0555c8),
    (0x0555CA, i_0555ca),
    (0x0555D0, i_0555d0),
    (0x0555D6, i_0555d6),
    (0x0555D8, i_0555d8),
    (0x0555DE, i_0555de),
    (0x0555E4, i_0555e4),
    (0x0555E6, i_0555e6),
    (0x0555EC, i_0555ec),
    (0x0555F2, i_0555f2),
    (0x0555F4, i_0555f4),
    (0x0555FA, i_0555fa),
    (0x055600, i_055600),
    (0x055602, i_055602),
    (0x055608, i_055608),
    (0x05560E, i_05560e),
    (0x055610, i_055610),
    (0x055616, i_055616),
    (0x05561C, i_05561c),
    (0x05561E, i_05561e),
    (0x055624, i_055624),
    (0x05562A, i_05562a),
    (0x05562C, i_05562c),
    (0x055632, i_055632),
    (0x055638, i_055638),
    (0x05563A, i_05563a),
    (0x055640, i_055640),
    (0x055646, i_055646),
    (0x055648, i_055648),
    (0x05564E, i_05564e),
    (0x055654, i_055654),
    (0x055656, i_055656),
    (0x05565C, i_05565c),
    (0x055662, i_055662),
    (0x055664, i_055664),
    (0x05566A, i_05566a),
    (0x055670, i_055670),
    (0x055672, i_055672),
    (0x055678, i_055678),
    (0x05567E, i_05567e),
    (0x055680, i_055680),
    (0x055686, i_055686),
    (0x05568C, i_05568c),
    (0x05568E, i_05568e),
    (0x055694, i_055694),
    (0x05569A, i_05569a),
    (0x05569C, i_05569c),
    (0x0556A2, i_0556a2),
    (0x0556A8, i_0556a8),
    (0x0556AA, i_0556aa),
    (0x0556B0, i_0556b0),
    (0x0556B6, i_0556b6),
    (0x0556B8, i_0556b8),
    (0x0556BE, i_0556be),
    (0x0556C4, i_0556c4),
    (0x0556C6, i_0556c6),
    (0x0556CC, i_0556cc),
    (0x0556D2, i_0556d2),
    (0x0556D4, i_0556d4),
    (0x0556DA, i_0556da),
    (0x0556E0, i_0556e0),
    (0x0556E2, i_0556e2),
    (0x0556E8, i_0556e8),
    (0x0556EE, i_0556ee),
    (0x0556F0, i_0556f0),
    (0x0556F6, i_0556f6),
    (0x0556FC, i_0556fc),
    (0x0556FE, i_0556fe),
    (0x055704, i_055704),
    (0x05570A, i_05570a),
    (0x05570C, i_05570c),
    (0x055712, i_055712),
    (0x055718, i_055718),
    (0x05571A, i_05571a),
    (0x055720, i_055720),
    (0x055726, i_055726),
    (0x055728, i_055728),
    (0x05572E, i_05572e),
    (0x055734, i_055734),
    (0x055736, i_055736),
    (0x05573C, i_05573c),
    (0x055742, i_055742),
    (0x055744, i_055744),
    (0x05574A, i_05574a),
    (0x055750, i_055750),
    (0x055752, i_055752),
    (0x055758, i_055758),
    (0x05575E, i_05575e),
    (0x055760, i_055760),
    (0x055766, i_055766),
    (0x05576C, i_05576c),
    (0x05576E, i_05576e),
    (0x055774, i_055774),
    (0x05577A, i_05577a),
    (0x05577C, i_05577c),
    (0x055782, i_055782),
    (0x055788, i_055788),
    (0x05578A, i_05578a),
    (0x055790, i_055790),
    (0x055796, i_055796),
    (0x055798, i_055798),
    (0x05579E, i_05579e),
    (0x0557A4, i_0557a4),
    (0x0557A6, i_0557a6),
    (0x0557AC, i_0557ac),
    (0x0557B2, i_0557b2),
    (0x0557B4, i_0557b4),
    (0x0557BA, i_0557ba),
    (0x0557C0, i_0557c0),
    (0x0557C2, i_0557c2),
    (0x0557C8, i_0557c8),
    (0x0557CE, i_0557ce),
    (0x0557D0, i_0557d0),
    (0x0557D6, i_0557d6),
    (0x0557DC, i_0557dc),
    (0x0557DE, i_0557de),
    (0x0557E4, i_0557e4),
    (0x0557EA, i_0557ea),
    (0x0557EC, i_0557ec),
    (0x0557F2, i_0557f2),
    (0x0557F8, i_0557f8),
    (0x0557FA, i_0557fa),
    (0x055800, i_055800),
    (0x055806, i_055806),
    (0x055808, i_055808),
    (0x05580E, i_05580e),
    (0x055814, i_055814),
    (0x055816, i_055816),
    (0x05581C, i_05581c),
    (0x055822, i_055822),
    (0x055824, i_055824),
    (0x05582A, i_05582a),
    (0x055830, i_055830),
    (0x055832, i_055832),
    (0x055838, i_055838),
    (0x05583E, i_05583e),
    (0x055840, i_055840),
    (0x055846, i_055846),
    (0x05584C, i_05584c),
    (0x05584E, i_05584e),
    (0x055854, i_055854),
    (0x05585A, i_05585a),
    (0x05585C, i_05585c),
    (0x055862, i_055862),
    (0x055868, i_055868),
    (0x05586A, i_05586a),
    (0x055870, i_055870),
    (0x055876, i_055876),
    (0x055878, i_055878),
    (0x05587E, i_05587e),
    (0x055884, i_055884),
    (0x055886, i_055886),
    (0x05588C, i_05588c),
    (0x055892, i_055892),
    (0x055894, i_055894),
    (0x05589A, i_05589a),
    (0x0558A0, i_0558a0),
    (0x0558A2, i_0558a2),
    (0x0558A8, i_0558a8),
    (0x0558AE, i_0558ae),
    (0x0558B0, i_0558b0),
    (0x0558B6, i_0558b6),
    (0x0558BC, i_0558bc),
    (0x0558BE, i_0558be),
    (0x0558C4, i_0558c4),
    (0x0558CA, i_0558ca),
    (0x0558CC, i_0558cc),
    (0x0558D2, i_0558d2),
    (0x0558D8, i_0558d8),
    (0x0558DA, i_0558da),
    (0x0558E0, i_0558e0),
    (0x0558E6, i_0558e6),
    (0x0558E8, i_0558e8),
    (0x0558EE, i_0558ee),
    (0x0558F4, i_0558f4),
    (0x0558F6, i_0558f6),
    (0x0558FC, i_0558fc),
    (0x055902, i_055902),
    (0x055904, i_055904),
    (0x05590A, i_05590a),
    (0x055910, i_055910),
    (0x055912, i_055912),
    (0x055918, i_055918),
    (0x05591E, i_05591e),
    (0x055920, i_055920),
    (0x055926, i_055926),
    (0x05592C, i_05592c),
    (0x05592E, i_05592e),
    (0x055934, i_055934),
    (0x05593A, i_05593a),
    (0x05593C, i_05593c),
    (0x055942, i_055942),
    (0x055948, i_055948),
    (0x05594A, i_05594a),
    (0x055950, i_055950),
    (0x055956, i_055956),
    (0x055958, i_055958),
    (0x05595E, i_05595e),
    (0x055964, i_055964),
    (0x055966, i_055966),
    (0x05596C, i_05596c),
    (0x055972, i_055972),
    (0x055974, i_055974),
    (0x05597A, i_05597a),
    (0x055980, i_055980),
    (0x055982, i_055982),
    (0x055988, i_055988),
    (0x05598E, i_05598e),
    (0x055990, i_055990),
    (0x055996, i_055996),
    (0x05599C, i_05599c),
    (0x05599E, i_05599e),
    (0x0559A4, i_0559a4),
    (0x0559AA, i_0559aa),
    (0x0559AC, i_0559ac),
    (0x0559B2, i_0559b2),
    (0x0559B8, i_0559b8),
    (0x0559BA, i_0559ba),
    (0x0559C0, i_0559c0),
    (0x0559C6, i_0559c6),
    (0x0559C8, i_0559c8),
    (0x0559CE, i_0559ce),
    (0x0559D4, i_0559d4),
    (0x0559D6, i_0559d6),
    (0x0559DC, i_0559dc),
    (0x0559E2, i_0559e2),
    (0x0559E4, i_0559e4),
    (0x0559EA, i_0559ea),
    (0x0559F0, i_0559f0),
    (0x0559F2, i_0559f2),
    (0x0559F8, i_0559f8),
    (0x0559FE, i_0559fe),
    (0x055A00, i_055a00),
    (0x055A06, i_055a06),
    (0x055A0C, i_055a0c),
    (0x055A0E, i_055a0e),
    (0x055A14, i_055a14),
    (0x055A1A, i_055a1a),
    (0x055A1C, i_055a1c),
    (0x055A22, i_055a22),
    (0x055A28, i_055a28),
    (0x055A2A, i_055a2a),
    (0x055A30, i_055a30),
    (0x055A36, i_055a36),
    (0x055A38, i_055a38),
    (0x055A3E, i_055a3e),
    (0x055A44, i_055a44),
    (0x055A46, i_055a46),
    (0x055A4C, i_055a4c),
    (0x055A52, i_055a52),
    (0x055A54, i_055a54),
    (0x055A5A, i_055a5a),
    (0x055A60, i_055a60),
    (0x055A62, i_055a62),
    (0x055A68, i_055a68),
    (0x055A6E, i_055a6e),
    (0x055A70, i_055a70),
    (0x055A76, i_055a76),
    (0x055A7C, i_055a7c),
    (0x055A7E, i_055a7e),
    (0x055A84, i_055a84),
    (0x055A8A, i_055a8a),
    (0x055A8C, i_055a8c),
    (0x055A92, i_055a92),
    (0x055A98, i_055a98),
    (0x055A9A, i_055a9a),
    (0x055AA0, i_055aa0),
    (0x055AA6, i_055aa6),
    (0x055AA8, i_055aa8),
    (0x055AAE, i_055aae),
    (0x055AB4, i_055ab4),
    (0x055AB6, i_055ab6),
    (0x055ABC, i_055abc),
    (0x055AC2, i_055ac2),
    (0x055AC4, i_055ac4),
    (0x055ACA, i_055aca),
    (0x055AD0, i_055ad0),
    (0x055AD2, i_055ad2),
    (0x055AD8, i_055ad8),
    (0x055ADE, i_055ade),
    (0x055AE0, i_055ae0),
    (0x055AE6, i_055ae6),
    (0x055AEC, i_055aec),
    (0x055AEE, i_055aee),
    (0x055AF4, i_055af4),
    (0x055AFA, i_055afa),
    (0x055AFC, i_055afc),
    (0x055B02, i_055b02),
    (0x055B08, i_055b08),
    (0x055B0A, i_055b0a),
    (0x055B10, i_055b10),
    (0x055B16, i_055b16),
    (0x055B18, i_055b18),
    (0x055B1E, i_055b1e),
    (0x055B24, i_055b24),
    (0x055B26, i_055b26),
    (0x055B2C, i_055b2c),
    (0x055B32, i_055b32),
    (0x055B34, i_055b34),
    (0x055B3A, i_055b3a),
    (0x055B40, i_055b40),
    (0x055B42, i_055b42),
    (0x055B48, i_055b48),
    (0x055B4E, i_055b4e),
    (0x055B50, i_055b50),
    (0x055B56, i_055b56),
    (0x055B5C, i_055b5c),
    (0x055B5E, i_055b5e),
    (0x055B64, i_055b64),
    (0x055B6A, i_055b6a),
    (0x055B6C, i_055b6c),
    (0x055B72, i_055b72),
    (0x055B78, i_055b78),
    (0x055B7A, i_055b7a),
    (0x055B80, i_055b80),
    (0x055B86, i_055b86),
    (0x055B88, i_055b88),
    (0x055B8E, i_055b8e),
    (0x055B94, i_055b94),
    (0x055B96, i_055b96),
    (0x055B9A, i_055b9a),
    (0x055B9E, i_055b9e),
    (0x055BA2, i_055ba2),
    (0x055BA6, i_055ba6),
    (0x055BAA, i_055baa),
    (0x055BAC, i_055bac),
    (0x055BB0, i_055bb0),
    (0x055BB2, i_055bb2),
    (0x055BB6, i_055bb6),
    (0x055BBA, i_055bba),
    (0x055BBE, i_055bbe),
    (0x055BC2, i_055bc2),
    (0x055BC6, i_055bc6),
    (0x055BC8, i_055bc8),
    (0x055BCC, i_055bcc),
    (0x055BCE, i_055bce),
    (0x055BD2, i_055bd2),
    (0x055BD8, i_055bd8),
    (0x055BDC, i_055bdc),
    (0x055BE2, i_055be2),
    (0x055BE6, i_055be6),
    (0x055BEA, i_055bea),
    (0x055BEE, i_055bee),
    (0x055BF2, i_055bf2),
    (0x055BF6, i_055bf6),
    (0x055BFA, i_055bfa),
    (0x055C00, i_055c00),
    (0x055C06, i_055c06),
    (0x055CD8, i_055cd8),
    (0x055CDE, i_055cde),
    (0x055CE4, i_055ce4),
    (0x055CE8, i_055ce8),
    (0x055CEA, i_055cea),
    (0x055CEE, i_055cee),
    (0x055CF0, i_055cf0),
    (0x055CF2, i_055cf2),
    (0x055CF6, i_055cf6),
    (0x055CFC, i_055cfc),
    (0x055D00, i_055d00),
    (0x055D04, i_055d04),
    (0x055D0A, i_055d0a),
    (0x055D10, i_055d10),
    (0x055D16, i_055d16),
    (0x055D1C, i_055d1c),
    (0x055D24, i_055d24),
    (0x055D28, i_055d28),
    (0x055D2C, i_055d2c),
    (0x055D32, i_055d32),
    (0x055D36, i_055d36),
    (0x055D3A, i_055d3a),
    (0x055D3E, i_055d3e),
    (0x055D40, i_055d40),
    (0x055D44, i_055d44),
    (0x055D46, i_055d46),
    (0x055D48, i_055d48),
    (0x055D4A, i_055d4a),
    (0x055D4C, i_055d4c),
    (0x055D4E, i_055d4e),
    (0x055D50, i_055d50),
    (0x055D54, i_055d54),
    (0x055D56, i_055d56),
    (0x055D58, i_055d58),
    (0x055D5C, i_055d5c),
    (0x055D5E, i_055d5e),
    (0x055D60, i_055d60),
    (0x055D64, i_055d64),
    (0x055D66, i_055d66),
    (0x055D6A, i_055d6a),
    (0x055D6E, i_055d6e),
    (0x055D72, i_055d72),
    (0x055D74, i_055d74),
    (0x055D78, i_055d78),
    (0x055D7C, i_055d7c),
    (0x055D80, i_055d80),
    (0x055D84, i_055d84),
    (0x055D86, i_055d86),
    (0x055D8A, i_055d8a),
    (0x055D8E, i_055d8e),
    (0x055D92, i_055d92),
    (0x055D98, i_055d98),
    (0x055D9C, i_055d9c),
    (0x055DA0, i_055da0),
    (0x055DA4, i_055da4),
    (0x055DA8, i_055da8),
    (0x055DAC, i_055dac),
    (0x055DB2, i_055db2),
    (0x055DB6, i_055db6),
    (0x055DBC, i_055dbc),
    (0x055DC0, i_055dc0),
    (0x055DC6, i_055dc6),
    (0x055DCC, i_055dcc),
    (0x055DD4, i_055dd4),
    (0x055DD8, i_055dd8),
    (0x055DDC, i_055ddc),
    (0x055DE0, i_055de0),
    (0x055DE2, i_055de2),
    (0x055DE6, i_055de6),
    (0x055DEA, i_055dea),
    (0x055DF0, i_055df0),
    (0x055DF6, i_055df6),
    (0x055DFA, i_055dfa),
    (0x055E02, i_055e02),
    (0x055E06, i_055e06),
    (0x055E0A, i_055e0a),
    (0x055E0E, i_055e0e),
    (0x055E10, i_055e10),
    (0x055E14, i_055e14),
    (0x055E18, i_055e18),
    (0x055E1E, i_055e1e),
    (0x055E24, i_055e24),
    (0x055E28, i_055e28),
    (0x055E30, i_055e30),
    (0x055E34, i_055e34),
    (0x055E38, i_055e38),
    (0x055E3A, i_055e3a),
    (0x055E3E, i_055e3e),
    (0x055E42, i_055e42),
    (0x055E48, i_055e48),
    (0x055E4E, i_055e4e),
    (0x055E52, i_055e52),
    (0x055E58, i_055e58),
    (0x055E5E, i_055e5e),
    (0x055E64, i_055e64),
    (0x055E68, i_055e68),
    (0x055E6E, i_055e6e),
    (0x055E74, i_055e74),
    (0x055E78, i_055e78),
    (0x055E7A, i_055e7a),
    (0x055E80, i_055e80),
    (0x055E84, i_055e84),
    (0x055E8A, i_055e8a),
    (0x055E90, i_055e90),
    (0x055E94, i_055e94),
    (0x055E98, i_055e98),
    (0x055E9A, i_055e9a),
    (0x055E9E, i_055e9e),
    (0x055EA2, i_055ea2),
    (0x055EA6, i_055ea6),
    (0x055EAC, i_055eac),
    (0x055EB0, i_055eb0),
    (0x055EB4, i_055eb4),
    (0x055EB6, i_055eb6),
    (0x055EBA, i_055eba),
    (0x055EC0, i_055ec0),
    (0x055EC4, i_055ec4),
    (0x055EC8, i_055ec8),
    (0x055ECA, i_055eca),
    (0x055ED0, i_055ed0),
    (0x055ED4, i_055ed4),
    (0x055ED8, i_055ed8),
    (0x055EE0, i_055ee0),
    (0x055EE4, i_055ee4),
    (0x055EE8, i_055ee8),
    (0x055EEC, i_055eec),
    (0x055EF0, i_055ef0),
    (0x055EF4, i_055ef4),
    (0x055EF6, i_055ef6),
    (0x055EFA, i_055efa),
    (0x055EFE, i_055efe),
    (0x055F02, i_055f02),
    (0x055F04, i_055f04),
    (0x055F08, i_055f08),
    (0x055F0A, i_055f0a),
    (0x055F0E, i_055f0e),
    (0x055F10, i_055f10),
    (0x055F16, i_055f16),
    (0x055F1A, i_055f1a),
    (0x055F1C, i_055f1c),
    (0x055F20, i_055f20),
    (0x055F24, i_055f24),
    (0x055F2A, i_055f2a),
    (0x055F32, i_055f32),
    (0x055F36, i_055f36),
    (0x055F3A, i_055f3a),
    (0x055F3E, i_055f3e),
    (0x055F40, i_055f40),
    (0x055F42, i_055f42),
    (0x055F48, i_055f48),
    (0x055F4A, i_055f4a),
    (0x055F4E, i_055f4e),
    (0x055F56, i_055f56),
    (0x055F5A, i_055f5a),
    (0x055F5E, i_055f5e),
    (0x055F62, i_055f62),
    (0x055F64, i_055f64),
    (0x055F66, i_055f66),
    (0x055F6C, i_055f6c),
    (0x055F6E, i_055f6e),
    (0x055F72, i_055f72),
    (0x055F78, i_055f78),
    (0x055F7E, i_055f7e),
    (0x055F84, i_055f84),
    (0x055F88, i_055f88),
    (0x055F8C, i_055f8c),
    (0x055F8E, i_055f8e),
    (0x055F94, i_055f94),
    (0x055F98, i_055f98),
    (0x055F9C, i_055f9c),
    (0x055F9E, i_055f9e),
    (0x055FA4, i_055fa4),
    (0x055FA8, i_055fa8),
    (0x055FAC, i_055fac),
    (0x055FAE, i_055fae),
    (0x055FB4, i_055fb4),
    (0x055FB8, i_055fb8),
    (0x055FBE, i_055fbe),
    (0x055FC2, i_055fc2),
    (0x055FC6, i_055fc6),
    (0x055FC8, i_055fc8),
    (0x055FCA, i_055fca),
    (0x055FCE, i_055fce),
    (0x055FD4, i_055fd4),
    (0x055FDA, i_055fda),
    (0x055FE0, i_055fe0),
    (0x055FE4, i_055fe4),
    (0x055FEA, i_055fea),
    (0x055FEE, i_055fee),
    (0x055FF2, i_055ff2),
    (0x055FF8, i_055ff8),
    (0x055FFE, i_055ffe),
    (0x056004, i_056004),
    (0x056008, i_056008),
    (0x05600E, i_05600e),
    (0x056012, i_056012),
    (0x056016, i_056016),
    (0x05601C, i_05601c),
    (0x056022, i_056022),
    (0x056028, i_056028),
    (0x05602C, i_05602c),
    (0x056032, i_056032),
    (0x05603A, i_05603a),
    (0x056040, i_056040),
    (0x056046, i_056046),
    (0x05604C, i_05604c),
    (0x056052, i_056052),
    (0x056058, i_056058),
    (0x05605E, i_05605e),
    (0x056064, i_056064),
    (0x056066, i_056066),
    (0x056068, i_056068),
    (0x05606E, i_05606e),
    (0x056072, i_056072),
    (0x056078, i_056078),
    (0x05607A, i_05607a),
    (0x056080, i_056080),
    (0x056086, i_056086),
    (0x05608C, i_05608c),
    (0x05608E, i_05608e),
    (0x056094, i_056094),
    (0x05609A, i_05609a),
    (0x05609E, i_05609e),
    (0x0560A4, i_0560a4),
    (0x0560AA, i_0560aa),
    (0x0560B0, i_0560b0),
    (0x0560B6, i_0560b6),
    (0x0560BC, i_0560bc),
    (0x0560C2, i_0560c2),
    (0x0560C6, i_0560c6),
    (0x0560CC, i_0560cc),
    (0x0560D0, i_0560d0),
    (0x0560D6, i_0560d6),
    (0x0560DC, i_0560dc),
    (0x0560DE, i_0560de),
    (0x0560E2, i_0560e2),
    (0x0560E8, i_0560e8),
    (0x0560EC, i_0560ec),
    (0x0560EE, i_0560ee),
    (0x0560F0, i_0560f0),
    (0x0560F4, i_0560f4),
    (0x0560F8, i_0560f8),
    (0x0560FC, i_0560fc),
    (0x056100, i_056100),
    (0x056106, i_056106),
    (0x05610A, i_05610a),
    (0x05610C, i_05610c),
    (0x056112, i_056112),
    (0x056116, i_056116),
    (0x05611A, i_05611a),
    (0x05611C, i_05611c),
    (0x056122, i_056122),
    (0x056128, i_056128),
    (0x05612C, i_05612c),
    (0x056130, i_056130),
    (0x056132, i_056132),
    (0x056136, i_056136),
    (0x05613A, i_05613a),
    (0x05613E, i_05613e),
    (0x056142, i_056142),
    (0x056146, i_056146),
    (0x05614A, i_05614a),
    (0x05614C, i_05614c),
    (0x056150, i_056150),
    (0x056154, i_056154),
    (0x056158, i_056158),
    (0x05615E, i_05615e),
    (0x056162, i_056162),
    (0x056168, i_056168),
    (0x05616C, i_05616c),
    (0x056172, i_056172),
    (0x056178, i_056178),
    (0x05617A, i_05617a),
    (0x056180, i_056180),
    (0x056186, i_056186),
    (0x05618A, i_05618a),
    (0x05618E, i_05618e),
    (0x056192, i_056192),
    (0x056194, i_056194),
    (0x05619A, i_05619a),
    (0x0561A0, i_0561a0),
    (0x0561A4, i_0561a4),
    (0x0561A8, i_0561a8),
    (0x0561AA, i_0561aa),
    (0x0561AE, i_0561ae),
    (0x0561B4, i_0561b4),
    (0x0561BA, i_0561ba),
    (0x0561BE, i_0561be),
    (0x0561C0, i_0561c0),
    (0x0561C6, i_0561c6),
    (0x0561CC, i_0561cc),
    (0x0561D0, i_0561d0),
    (0x0561D4, i_0561d4),
    (0x0561D6, i_0561d6),
    (0x0561DA, i_0561da),
    (0x0561E0, i_0561e0),
    (0x0561E6, i_0561e6),
    (0x0561EA, i_0561ea),
    (0x0561EC, i_0561ec),
    (0x0561F2, i_0561f2),
    (0x0561F8, i_0561f8),
    (0x0561FC, i_0561fc),
    (0x056200, i_056200),
    (0x056202, i_056202),
    (0x056204, i_056204),
    (0x056208, i_056208),
    (0x05620E, i_05620e),
    (0x056214, i_056214),
    (0x056218, i_056218),
    (0x05621E, i_05621e),
    (0x056224, i_056224),
    (0x05622A, i_05622a),
    (0x05622E, i_05622e),
    (0x056234, i_056234),
    (0x05623A, i_05623a),
    (0x056240, i_056240),
    (0x056244, i_056244),
    (0x05624A, i_05624a),
    (0x056250, i_056250),
    (0x056256, i_056256),
    (0x05625A, i_05625a),
    (0x05625C, i_05625c),
    (0x056262, i_056262),
    (0x056268, i_056268),
    (0x05626C, i_05626c),
    (0x056272, i_056272),
    (0x056278, i_056278),
    (0x0563F4, i_0563f4),
    (0x0563F8, i_0563f8),
    (0x0563FC, i_0563fc),
    (0x0563FE, i_0563fe),
    (0x056402, i_056402),
    (0x056404, i_056404),
    (0x056408, i_056408),
    (0x05640E, i_05640e),
    (0x056410, i_056410),
    (0x056414, i_056414),
    (0x056418, i_056418),
    (0x05641C, i_05641c),
    (0x056420, i_056420),
    (0x056424, i_056424),
    (0x056428, i_056428),
    (0x05642C, i_05642c),
    (0x056430, i_056430),
    (0x056432, i_056432),
    (0x056436, i_056436),
    (0x056438, i_056438),
    (0x05643A, i_05643a),
    (0x056440, i_056440),
    (0x056444, i_056444),
    (0x05644A, i_05644a),
    (0x05644C, i_05644c),
    (0x056452, i_056452),
    (0x056458, i_056458),
    (0x05645E, i_05645e),
    (0x056464, i_056464),
    (0x05646A, i_05646a),
    (0x05646C, i_05646c),
    (0x056470, i_056470),
    (0x056476, i_056476),
    (0x05647A, i_05647a),
    (0x05647E, i_05647e),
    (0x056484, i_056484),
    (0x05648A, i_05648a),
    (0x056490, i_056490),
    (0x056496, i_056496),
    (0x05649C, i_05649c),
    (0x0564A0, i_0564a0),
    (0x0564A4, i_0564a4),
    (0x0564AA, i_0564aa),
    (0x0564B0, i_0564b0),
    (0x0564B6, i_0564b6),
    (0x0564BA, i_0564ba),
    (0x0564C0, i_0564c0),
    (0x0564C4, i_0564c4),
    (0x0564CA, i_0564ca),
    (0x0564D0, i_0564d0),
    (0x0564D6, i_0564d6),
    (0x0564DA, i_0564da),
    (0x0564E0, i_0564e0),
    (0x0564E4, i_0564e4),
    (0x0564E6, i_0564e6),
    (0x0564EC, i_0564ec),
    (0x0564F2, i_0564f2),
    (0x0564F6, i_0564f6),
    (0x0564FC, i_0564fc),
    (0x056500, i_056500),
    (0x056506, i_056506),
    (0x05650A, i_05650a),
    (0x05650E, i_05650e),
    (0x056514, i_056514),
    (0x056518, i_056518),
    (0x05651C, i_05651c),
    (0x056520, i_056520),
    (0x056524, i_056524),
    (0x056526, i_056526),
    (0x05652A, i_05652a),
    (0x05652E, i_05652e),
    (0x056532, i_056532),
    (0x056536, i_056536),
    (0x056538, i_056538),
    (0x05653E, i_05653e),
    (0x056542, i_056542),
    (0x056546, i_056546),
    (0x056548, i_056548),
    (0x05654E, i_05654e),
    (0x056554, i_056554),
    (0x05655A, i_05655a),
    (0x05655E, i_05655e),
    (0x056562, i_056562),
    (0x056564, i_056564),
    (0x05656A, i_05656a),
    (0x056570, i_056570),
    (0x056574, i_056574),
    (0x056578, i_056578),
    (0x05657A, i_05657a),
    (0x056580, i_056580),
    (0x056584, i_056584),
    (0x05658A, i_05658a),
    (0x05658E, i_05658e),
    (0x056592, i_056592),
    (0x056594, i_056594),
    (0x056596, i_056596),
    (0x05659C, i_05659c),
    (0x0565A0, i_0565a0),
    (0x0565A6, i_0565a6),
    (0x0565AC, i_0565ac),
    (0x0565B0, i_0565b0),
    (0x0565B6, i_0565b6),
    (0x0565BC, i_0565bc),
    (0x0565C2, i_0565c2),
    (0x05684C, i_05684c),
    (0x056850, i_056850),
    (0x056856, i_056856),
    (0x056858, i_056858),
    (0x05685E, i_05685e),
    (0x056864, i_056864),
    (0x05686A, i_05686a),
    (0x05686E, i_05686e),
    (0x056870, i_056870),
    (0x056876, i_056876),
    (0x05687A, i_05687a),
    (0x056880, i_056880),
    (0x056886, i_056886),
    (0x05688C, i_05688c),
    (0x056890, i_056890),
    (0x056896, i_056896),
    (0x05689A, i_05689a),
    (0x0568A0, i_0568a0),
    (0x0568A6, i_0568a6),
    (0x0568AC, i_0568ac),
    (0x0568B0, i_0568b0),
    (0x0568B6, i_0568b6),
    (0x0568BC, i_0568bc),
    (0x0568C0, i_0568c0),
    (0x0568C4, i_0568c4),
    (0x0568C8, i_0568c8),
    (0x0568CC, i_0568cc),
    (0x0568D2, i_0568d2),
    (0x0568D8, i_0568d8),
    (0x0568DC, i_0568dc),
    (0x0568E0, i_0568e0),
    (0x0568E2, i_0568e2),
    (0x0568E6, i_0568e6),
    (0x0568EC, i_0568ec),
    (0x0568F0, i_0568f0),
    (0x0568F2, i_0568f2),
    (0x0568F6, i_0568f6),
    (0x0568FC, i_0568fc),
    (0x056900, i_056900),
    (0x056906, i_056906),
    (0x05690A, i_05690a),
    (0x056910, i_056910),
    (0x056914, i_056914),
    (0x056918, i_056918),
    (0x05691C, i_05691c),
    (0x05691E, i_05691e),
    (0x056922, i_056922),
    (0x056926, i_056926),
    (0x05692C, i_05692c),
    (0x056930, i_056930),
    (0x056932, i_056932),
    (0x056936, i_056936),
    (0x05693A, i_05693a),
    (0x05693C, i_05693c),
    (0x056940, i_056940),
    (0x056946, i_056946),
    (0x05694A, i_05694a),
    (0x05694E, i_05694e),
    (0x056952, i_056952),
    (0x056956, i_056956),
    (0x05695A, i_05695a),
    (0x05695E, i_05695e),
    (0x056962, i_056962),
    (0x056964, i_056964),
    (0x05696A, i_05696a),
    (0x05696E, i_05696e),
    (0x056972, i_056972),
    (0x056976, i_056976),
    (0x05697A, i_05697a),
    (0x05697C, i_05697c),
    (0x056980, i_056980),
    (0x056984, i_056984),
    (0x056988, i_056988),
    (0x05698A, i_05698a),
    (0x05698E, i_05698e),
    (0x056992, i_056992),
    (0x056996, i_056996),
    (0x05699C, i_05699c),
    (0x0569A0, i_0569a0),
    (0x0569A6, i_0569a6),
    (0x0569AC, i_0569ac),
    (0x0569B2, i_0569b2),
    (0x0569B6, i_0569b6),
    (0x0569BC, i_0569bc),
    (0x0569C0, i_0569c0),
    (0x0569C6, i_0569c6),
    (0x0569CC, i_0569cc),
    (0x0569D0, i_0569d0),
    (0x0569D6, i_0569d6),
    (0x0569DA, i_0569da),
    (0x0569E0, i_0569e0),
    (0x0569E4, i_0569e4),
    (0x0569EA, i_0569ea),
    (0x0569EE, i_0569ee),
    (0x0569F2, i_0569f2),
    (0x0569F8, i_0569f8),
    (0x0569FE, i_0569fe),
    (0x056A04, i_056a04),
    (0x056A06, i_056a06),
    (0x056A0C, i_056a0c),
    (0x056A10, i_056a10),
    (0x056A16, i_056a16),
    (0x056A1C, i_056a1c),
    (0x056A20, i_056a20),
    (0x056A26, i_056a26),
    (0x056A2C, i_056a2c),
    (0x056A32, i_056a32),
    (0x056A38, i_056a38),
    (0x056A3C, i_056a3c),
    (0x056A40, i_056a40),
    (0x056A44, i_056a44),
    (0x056A48, i_056a48),
    (0x056A4C, i_056a4c),
    (0x056A4E, i_056a4e),
    (0x056A52, i_056a52),
    (0x056A54, i_056a54),
    (0x056A58, i_056a58),
    (0x056A5C, i_056a5c),
    (0x056A60, i_056a60),
    (0x056A64, i_056a64),
    (0x056A68, i_056a68),
    (0x056A6A, i_056a6a),
    (0x056A6E, i_056a6e),
    (0x056A70, i_056a70),
    (0x056A74, i_056a74),
    (0x056A78, i_056a78),
    (0x056A7C, i_056a7c),
    (0x056A80, i_056a80),
    (0x056A84, i_056a84),
    (0x056A86, i_056a86),
    (0x056A8A, i_056a8a),
    (0x056A8C, i_056a8c),
    (0x056A90, i_056a90),
    (0x056A94, i_056a94),
    (0x056A98, i_056a98),
    (0x056A9C, i_056a9c),
    (0x056AA0, i_056aa0),
    (0x056AA2, i_056aa2),
    (0x056AA6, i_056aa6),
    (0x056AA8, i_056aa8),
    (0x056AAC, i_056aac),
    (0x056AB0, i_056ab0),
    (0x056AB4, i_056ab4),
    (0x056AB8, i_056ab8),
    (0x056ABC, i_056abc),
    (0x056ABE, i_056abe),
    (0x056AC2, i_056ac2),
    (0x056ACC, i_056acc),
    (0x056AD2, i_056ad2),
    (0x056AD6, i_056ad6),
    (0x056ADA, i_056ada),
    (0x056ADE, i_056ade),
    (0x056AE2, i_056ae2),
    (0x056AE6, i_056ae6),
    (0x056AEA, i_056aea),
    (0x056AEE, i_056aee),
    (0x056AF4, i_056af4),
    (0x056AF8, i_056af8),
    (0x056AFC, i_056afc),
    (0x056B02, i_056b02),
    (0x056B06, i_056b06),
    (0x056B0C, i_056b0c),
    (0x056B10, i_056b10),
    (0x056B12, i_056b12),
    (0x056B14, i_056b14),
    (0x056B16, i_056b16),
    (0x056B18, i_056b18),
    (0x056B1A, i_056b1a),
    (0x056B1C, i_056b1c),
    (0x056B1E, i_056b1e),
    (0x056B20, i_056b20),
    (0x056B22, i_056b22),
    (0x056B24, i_056b24),
    (0x056B28, i_056b28),
    (0x056B2A, i_056b2a),
    (0x056B2C, i_056b2c),
    (0x056B2E, i_056b2e),
    (0x056B32, i_056b32),
    (0x056B34, i_056b34),
    (0x056B36, i_056b36),
    (0x056B38, i_056b38),
    (0x056B3A, i_056b3a),
    (0x056B3E, i_056b3e),
    (0x056B40, i_056b40),
    (0x056B46, i_056b46),
    (0x056B4A, i_056b4a),
    (0x056B4E, i_056b4e),
    (0x056B52, i_056b52),
    (0x056B56, i_056b56),
    (0x056B5A, i_056b5a),
    (0x056B5C, i_056b5c),
    (0x056B60, i_056b60),
    (0x056B62, i_056b62),
    (0x056B64, i_056b64),
    (0x056B6A, i_056b6a),
    (0x056B6E, i_056b6e),
    (0x056B72, i_056b72),
    (0x056B76, i_056b76),
    (0x056B7A, i_056b7a),
    (0x056B7E, i_056b7e),
    (0x056B80, i_056b80),
    (0x056B82, i_056b82),
    (0x056B86, i_056b86),
    (0x056B88, i_056b88),
    (0x056B8C, i_056b8c),
    (0x056B8E, i_056b8e),
    (0x056B90, i_056b90),
    (0x056B92, i_056b92),
    (0x056B96, i_056b96),
    (0x056B9A, i_056b9a),
    (0x056B9C, i_056b9c),
    (0x056BA0, i_056ba0),
    (0x056BA4, i_056ba4),
    (0x056BA8, i_056ba8),
    (0x056BAA, i_056baa),
    (0x056BAE, i_056bae),
    (0x056BB4, i_056bb4),
    (0x056BBA, i_056bba),
    (0x056BC0, i_056bc0),
    (0x056BC4, i_056bc4),
    (0x056BC8, i_056bc8),
    (0x056BCC, i_056bcc),
    (0x056BD2, i_056bd2),
    (0x056BD6, i_056bd6),
    (0x056BD8, i_056bd8),
    (0x056BDC, i_056bdc),
    (0x056BDE, i_056bde),
    (0x056BE2, i_056be2),
    (0x056BE8, i_056be8),
    (0x056BEC, i_056bec),
    (0x056BF2, i_056bf2),
    (0x056BF6, i_056bf6),
    (0x056BFC, i_056bfc),
    (0x056C00, i_056c00),
    (0x056C04, i_056c04),
    (0x056C06, i_056c06),
    (0x056C0A, i_056c0a),
    (0x056C10, i_056c10),
    (0x056C14, i_056c14),
    (0x056C18, i_056c18),
    (0x056C1C, i_056c1c),
    (0x056C20, i_056c20),
    (0x056C26, i_056c26),
    (0x056C2C, i_056c2c),
    (0x056C30, i_056c30),
    (0x056C36, i_056c36),
    (0x056C3A, i_056c3a),
    (0x056C40, i_056c40),
    (0x056C44, i_056c44),
    (0x056C4A, i_056c4a),
    (0x056C50, i_056c50),
    (0x056C54, i_056c54),
    (0x056C5A, i_056c5a),
    (0x056C5E, i_056c5e),
    (0x056C64, i_056c64),
    (0x056C68, i_056c68),
    (0x056C6C, i_056c6c),
    (0x056C72, i_056c72),
    (0x056C76, i_056c76),
    (0x056C78, i_056c78),
    (0x056C7C, i_056c7c),
    (0x056C82, i_056c82),
    (0x056C88, i_056c88),
    (0x056C8E, i_056c8e),
    (0x056C92, i_056c92),
    (0x056C98, i_056c98),
    (0x056C9A, i_056c9a),
    (0x056C9E, i_056c9e),
    (0x056CA4, i_056ca4),
    (0x056CAA, i_056caa),
    (0x056CB0, i_056cb0),
    (0x056CB4, i_056cb4),
    (0x056CB8, i_056cb8),
    (0x056CBC, i_056cbc),
    (0x056CC2, i_056cc2),
    (0x056CC6, i_056cc6),
    (0x056CCA, i_056cca),
    (0x056CD0, i_056cd0),
    (0x056CD6, i_056cd6),
    (0x056CDC, i_056cdc),
    (0x056CE0, i_056ce0),
    (0x056CE6, i_056ce6),
    (0x056CEA, i_056cea),
    (0x056CEE, i_056cee),
    (0x056CF2, i_056cf2),
    (0x056CF6, i_056cf6),
    (0x056CFA, i_056cfa),
    (0x056CFE, i_056cfe),
    (0x056D04, i_056d04),
    (0x056D08, i_056d08),
    (0x056D0C, i_056d0c),
    (0x056D12, i_056d12),
    (0x056D16, i_056d16),
    (0x056D1C, i_056d1c),
    (0x056D20, i_056d20),
    (0x056D24, i_056d24),
    (0x056D28, i_056d28),
    (0x056D2E, i_056d2e),
    (0x056D34, i_056d34),
    (0x056D38, i_056d38),
    (0x056D3E, i_056d3e),
    (0x056D44, i_056d44),
    (0x056D48, i_056d48),
    (0x056D4A, i_056d4a),
    (0x056D4E, i_056d4e),
    (0x056D52, i_056d52),
    (0x056D56, i_056d56),
    (0x056D5A, i_056d5a),
    (0x056D5C, i_056d5c),
    (0x056D60, i_056d60),
    (0x056D62, i_056d62),
    (0x056D66, i_056d66),
    (0x056D68, i_056d68),
    (0x056D6A, i_056d6a),
    (0x056D6E, i_056d6e),
    (0x056D72, i_056d72),
    (0x056D76, i_056d76),
    (0x056D78, i_056d78),
    (0x056D7C, i_056d7c),
    (0x056D80, i_056d80),
    (0x056D82, i_056d82),
    (0x056D84, i_056d84),
    (0x056D88, i_056d88),
    (0x056D8C, i_056d8c),
    (0x056D90, i_056d90),
    (0x056D94, i_056d94),
    (0x056D96, i_056d96),
    (0x056D9A, i_056d9a),
    (0x056D9C, i_056d9c),
    (0x056DA0, i_056da0),
    (0x056DA2, i_056da2),
    (0x056DA6, i_056da6),
    (0x056DAC, i_056dac),
    (0x056DB0, i_056db0),
    (0x056DB2, i_056db2),
    (0x056DB4, i_056db4),
    (0x056DBA, i_056dba),
    (0x056DBE, i_056dbe),
    (0x056DC2, i_056dc2),
    (0x056DC6, i_056dc6),
    (0x056DCA, i_056dca),
    (0x056DCE, i_056dce),
    (0x056DD2, i_056dd2),
    (0x056DD6, i_056dd6),
    (0x056DDA, i_056dda),
    (0x056DDC, i_056ddc),
    (0x056DE2, i_056de2),
    (0x056DE6, i_056de6),
    (0x056DEA, i_056dea),
    (0x056DEE, i_056dee),
    (0x056DF0, i_056df0),
    (0x056DF6, i_056df6),
    (0x056DFA, i_056dfa),
    (0x056E00, i_056e00),
    (0x056E04, i_056e04),
    (0x056E08, i_056e08),
    (0x056E0C, i_056e0c),
    (0x056E10, i_056e10),
    (0x056E14, i_056e14),
    (0x056E18, i_056e18),
    (0x056E1C, i_056e1c),
    (0x056E1E, i_056e1e),
    (0x056E24, i_056e24),
    (0x056E2A, i_056e2a),
    (0x056E2E, i_056e2e),
    (0x056E34, i_056e34),
    (0x056E36, i_056e36),
    (0x056E3A, i_056e3a),
    (0x056E3E, i_056e3e),
    (0x056E42, i_056e42),
    (0x056E44, i_056e44),
    (0x056E48, i_056e48),
    (0x056E4A, i_056e4a),
    (0x056E4E, i_056e4e),
    (0x056E52, i_056e52),
    (0x056E56, i_056e56),
    (0x056E5C, i_056e5c),
    (0x056E60, i_056e60),
    (0x056E64, i_056e64),
    (0x056E68, i_056e68),
    (0x056E6A, i_056e6a),
    (0x056E70, i_056e70),
    (0x056E72, i_056e72),
    (0x056E76, i_056e76),
    (0x056E7A, i_056e7a),
    (0x056E7C, i_056e7c),
    (0x056E80, i_056e80),
    (0x056E82, i_056e82),
    (0x056E86, i_056e86),
    (0x056E8A, i_056e8a),
    (0x056E8E, i_056e8e),
    (0x056E90, i_056e90),
    (0x056E94, i_056e94),
    (0x056E98, i_056e98),
    (0x056E9C, i_056e9c),
    (0x056E9E, i_056e9e),
    (0x056EA2, i_056ea2),
    (0x056EA6, i_056ea6),
    (0x056EAA, i_056eaa),
    (0x056EAC, i_056eac),
    (0x056EAE, i_056eae),
    (0x056EB2, i_056eb2),
    (0x056EB8, i_056eb8),
    (0x056EBC, i_056ebc),
    (0x056EBE, i_056ebe),
    (0x056EC2, i_056ec2),
    (0x056EC4, i_056ec4),
    (0x056EC6, i_056ec6),
    (0x056ECA, i_056eca),
    (0x056ECC, i_056ecc),
    (0x056ED0, i_056ed0),
    (0x056EDA, i_056eda),
    (0x056EDE, i_056ede),
    (0x056EE2, i_056ee2),
    (0x056EE6, i_056ee6),
    (0x056EEC, i_056eec),
    (0x056EF0, i_056ef0),
    (0x056EF4, i_056ef4),
    (0x056EF8, i_056ef8),
    (0x056EFA, i_056efa),
    (0x056F00, i_056f00),
    (0x056F04, i_056f04),
    (0x056F08, i_056f08),
    (0x056F0A, i_056f0a),
    (0x056F0E, i_056f0e),
    (0x056F10, i_056f10),
    (0x056F14, i_056f14),
    (0x056F18, i_056f18),
    (0x056F1C, i_056f1c),
    (0x056F1E, i_056f1e),
    (0x056F22, i_056f22),
    (0x056F24, i_056f24),
    (0x056F28, i_056f28),
    (0x056F2C, i_056f2c),
    (0x056F30, i_056f30),
    (0x056F32, i_056f32),
    (0x056F36, i_056f36),
    (0x056F3A, i_056f3a),
    (0x056F3E, i_056f3e),
    (0x056F42, i_056f42),
    (0x056F46, i_056f46),
    (0x056F48, i_056f48),
    (0x056F4C, i_056f4c),
    (0x056F50, i_056f50),
    (0x056F54, i_056f54),
    (0x056F58, i_056f58),
    (0x056F5C, i_056f5c),
    (0x056F5E, i_056f5e),
    (0x056F62, i_056f62),
    (0x056F64, i_056f64),
    (0x056F68, i_056f68),
    (0x056F6C, i_056f6c),
    (0x056F70, i_056f70),
    (0x056F72, i_056f72),
    (0x056F76, i_056f76),
    (0x056F7A, i_056f7a),
    (0x056F7E, i_056f7e),
    (0x056F82, i_056f82),
    (0x056F84, i_056f84),
    (0x056F88, i_056f88),
    (0x056F8A, i_056f8a),
    (0x056F8E, i_056f8e),
    (0x056F94, i_056f94),
    (0x056F98, i_056f98),
    (0x056F9A, i_056f9a),
    (0x056F9E, i_056f9e),
    (0x056FA0, i_056fa0),
    (0x056FA6, i_056fa6),
    (0x056FAA, i_056faa),
    (0x056FAE, i_056fae),
    (0x056FB2, i_056fb2),
    (0x056FB6, i_056fb6),
    (0x056FBA, i_056fba),
    (0x056FBE, i_056fbe),
    (0x056FC2, i_056fc2),
    (0x056FC6, i_056fc6),
    (0x056FCC, i_056fcc),
    (0x056FD0, i_056fd0),
    (0x056FD6, i_056fd6),
    (0x056FDA, i_056fda),
    (0x056FE0, i_056fe0),
    (0x056FE4, i_056fe4),
    (0x056FE6, i_056fe6),
    (0x056FEA, i_056fea),
    (0x056FEC, i_056fec),
    (0x056FF2, i_056ff2),
    (0x056FF4, i_056ff4),
    (0x056FF8, i_056ff8),
    (0x056FFA, i_056ffa),
    (0x056FFC, i_056ffc),
    (0x057000, i_057000),
    (0x057006, i_057006),
    (0x057008, i_057008),
    (0x05700C, i_05700c),
    (0x057010, i_057010),
    (0x057016, i_057016),
    (0x057018, i_057018),
    (0x05701A, i_05701a),
    (0x05701E, i_05701e),
    (0x057022, i_057022),
    (0x057026, i_057026),
    (0x057028, i_057028),
    (0x05702A, i_05702a),
    (0x057030, i_057030),
    (0x057032, i_057032),
    (0x057034, i_057034),
    (0x057038, i_057038),
    (0x05703C, i_05703c),
    (0x057040, i_057040),
    (0x057042, i_057042),
    (0x057044, i_057044),
    (0x05704A, i_05704a),
    (0x05704E, i_05704e),
    (0x057052, i_057052),
    (0x057056, i_057056),
    (0x05705A, i_05705a),
    (0x05705E, i_05705e),
    (0x057062, i_057062),
    (0x057066, i_057066),
    (0x05706A, i_05706a),
    (0x057070, i_057070),
    (0x057074, i_057074),
    (0x057078, i_057078),
    (0x05707A, i_05707a),
    (0x05707C, i_05707c),
    (0x057080, i_057080),
    (0x057082, i_057082),
    (0x057086, i_057086),
    (0x05708A, i_05708a),
    (0x05708E, i_05708e),
    (0x057092, i_057092),
    (0x057094, i_057094),
    (0x057098, i_057098),
    (0x05709C, i_05709c),
    (0x0570A0, i_0570a0),
    (0x0570A2, i_0570a2),
    (0x0570A6, i_0570a6),
    (0x0570A8, i_0570a8),
    (0x0570AE, i_0570ae),
    (0x0570B2, i_0570b2),
    (0x0570B8, i_0570b8),
    (0x0570BE, i_0570be),
    (0x0570C4, i_0570c4),
    (0x0570C6, i_0570c6),
    (0x0570CA, i_0570ca),
    (0x0570CE, i_0570ce),
    (0x0570D0, i_0570d0),
    (0x0570D2, i_0570d2),
    (0x0570D4, i_0570d4),
    (0x0570D8, i_0570d8),
    (0x0570DE, i_0570de),
    (0x0570E4, i_0570e4),
    (0x0570E6, i_0570e6),
    (0x0570EA, i_0570ea),
    (0x0570EE, i_0570ee),
    (0x0570F2, i_0570f2),
    (0x0570F4, i_0570f4),
    (0x0570F6, i_0570f6),
    (0x0570FA, i_0570fa),
    (0x0570FE, i_0570fe),
    (0x057102, i_057102),
    (0x057106, i_057106),
];
