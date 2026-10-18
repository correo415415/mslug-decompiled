//! Núcleo: RNG, buffers, variables globales.

use super::cc::*;
use neogeo_hw::cpu::m68k::{Cpu, Size::*};
use neogeo_hw::native::DynBus;

/// $001E56 `Store106EDA`: `$106EDA.b = d0.b`.
pub fn store_106eda_001e56(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let v = cpu.d[0] & 0xFF;
    w8(bus, 0x106EDA, v);
    logic(cpu, v, Byte);
    4 + ea(EA_ABSL, Byte) + rts(cpu, bus)
}

/// $05E9E4 `RNG_LFSRStep`: generador de números aleatorios del juego.
///
/// Tabla de 32 palabras en `$10E230`, índice (par, 0..$3E) en `$10E270`.
/// Avanza el índice 2, toma la palabra, la rota 1 bit a la izquierda y la
/// mezcla (XOR) con la palabra situada 21 bytes antes (módulo $40, en
/// palabras = la 10.5ª anterior, alineada por la máscara). Devuelve en d0 el
/// valor escalado a `[0, d0_entrada)`: `(nuevo * d0) >> 16` (tras `swap`, d0
/// queda con el producto en la mitad alta).
pub fn rng_lfsr_step_05e9e4(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let mut cyc = 0;
    // move.w d0,d5
    let range = cpu.d[0] & 0xFFFF;
    set_d(cpu, 5, range, Word); logic(cpu, range, Word); cyc += 4;
    // lea $10E230,a4
    cpu.a[4] = 0x10E230; cyc += 4 + ea(EA_ABSL, Long);
    // move.w $10E270,d7 ; addi.w #2,d7 ; move.w #$3E,d6 ; and.w d6,d7
    let mut i = r16(bus, 0x10E270);
    set_d(cpu, 7, i, Word); logic(cpu, i, Word); cyc += 4 + ea(EA_ABSL, Word);
    i = add(cpu, i, 2, Word); set_d(cpu, 7, i, Word); cyc += 8;
    set_d(cpu, 6, 0x3E, Word); logic(cpu, 0x3E, Word); cyc += 4;
    i &= 0x3E; set_d(cpu, 7, i, Word); logic(cpu, i, Word); cyc += 4;
    // move.w d7,$10E270
    w16(bus, 0x10E270, i); logic(cpu, i, Word); cyc += 4 + ea(EA_ABSL, Word);
    // move.w (a4,d7.w),d0 ; rol.w #1,d0
    let mut v = r16(bus, 0x10E230 + i);
    set_d(cpu, 0, v, Word); logic(cpu, v, Word); cyc += 4 + ea(EA_IDX, Word);
    v = rol(cpu, v, 1, Word); set_d(cpu, 0, v, Word); cyc += shift_cycles(1);
    // subi.w #$15,d7 ; and.w d6,d7
    let j = sub(cpu, i, 0x15, Word, true); set_d(cpu, 7, j, Word); cyc += 8;
    let j = j & 0x3E; set_d(cpu, 7, j, Word); logic(cpu, j, Word); cyc += 4;
    // eor.w d0,(a4,d7.w) ; move.w (a4,d7.w),d0
    let a = 0x10E230 + j;
    let m = r16(bus, a) ^ v;
    w16(bus, a, m); logic(cpu, m, Word); cyc += 4 + ea(EA_IDX, Word);
    set_d(cpu, 0, m, Word); logic(cpu, m, Word); cyc += 4 + ea(EA_IDX, Word);
    // mulu.w d5,d0 ; swap d0
    let p = m * range;
    cpu.d[0] = p; logic(cpu, p, Long); cyc += 70;
    let p = p.rotate_left(16);
    cpu.d[0] = p; logic(cpu, p, Long); cyc += 4;
    cyc + rts(cpu, bus)
}

/// $043EDA `Buffer_ClearBlock1024L`: pone a 0 4 KiB a partir de `a0+$7C`
/// (1024 longs). Deja a1 al final del bloque y d7.w = $FFFF.
pub fn buffer_clear_block_1024l_043eda(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let start = cpu.a[0].wrapping_add(0x7C);
    for k in 0..1024u32 {
        w32(bus, start.wrapping_add(4 * k), 0);
    }
    cpu.a[1] = start.wrapping_add(4096);
    // move.l #$3FF,d7 ... dbra -> d7.w = -1 (parte alta = 0 del move.l)
    cpu.d[7] = 0x0000_FFFF;
    logic(cpu, 0, Long); // último flag: clr.l (N=0 Z=1 V=0 C=0)
    let cyc = (4 + ea(EA_D16, Long))      // lea $7C(a0),a1
        + 4                                // move.l #imm,d7
        + 1024 * (4 + ea(EA_IND, Long))    // clr.l (a1)+
        + 1023 * 10 + 14;                  // dbra (salta 1023 veces, sale 1)
    cyc + rts(cpu, bus)
}
