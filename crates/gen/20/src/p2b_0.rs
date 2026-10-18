// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p2b_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $2B6114: addi.w #$24, $24(a6)
pub fn i_2b6114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B611A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x24, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $2B611A: rts
pub fn i_2b611a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B611C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $2B611C: subi.w #$24, $24(a6)
pub fn i_2b611c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6122;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x24, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $2B6122: rts
pub fn i_2b6122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6124;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x2B6114, i_2b6114),
    (0x2B611A, i_2b611a),
    (0x2B611C, i_2b611c),
    (0x2B6122, i_2b6122),
];
