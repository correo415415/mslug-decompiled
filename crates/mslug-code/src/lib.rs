//! Programa 68000 de Metal Slug (P-ROM del juego + BIOS Neo Geo) traducido
//! a Rust por `tools/recomp/m68k_recomp.py`.
//!
//! Cada instrucción de la ROM es una función Rust (`gen::*::i_XXXXXX`) que
//! hace exactamente lo que haría la CPU: mismos registros, flags, accesos al
//! bus y ciclos. El juego ya no se interpreta: `Program::step` llama a estas
//! funciones según el PC. El intérprete 68000 sólo queda como red de
//! seguridad para direcciones que el recompilador aún no conoce (`misses`);
//! se añaden a `tools/recomp/seeds.txt` y desaparecen al regenerar. El
//! objetivo final es 0 misses y retirar el intérprete.

#[allow(clippy::all)]
pub mod gen;

use neogeo_hw::cpu::m68k::{Cpu, Exception};
use neogeo_hw::{NeoGeoBus, System};
use std::collections::BTreeMap;

pub type InsFn = fn(&mut Cpu, &mut NeoGeoBus) -> u32;

const PROM_END: u32 = 0x30_0000;
const BIOS_BASE: u32 = 0xC0_0000;
const BIOS_END: u32 = 0xD0_0000;

/// Tabla de despacho PC -> función traducida.
pub struct Program {
    prom: Vec<Option<InsFn>>,
    bios: Vec<Option<InsFn>>,
    /// Direcciones ejecutadas sin traducción (pc -> veces).
    pub misses: BTreeMap<u32, u64>,
    /// Instrucciones ejecutadas por código traducido / por el intérprete.
    pub native: u64,
    pub interp: u64,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        let mut prom = vec![None; (PROM_END / 2) as usize];
        let mut bios = vec![None; ((BIOS_END - BIOS_BASE) / 2) as usize];
        for t in gen::TABLES {
            for &(a, f) in t.iter() {
                if a < PROM_END {
                    prom[(a / 2) as usize] = Some(f);
                } else if (BIOS_BASE..BIOS_END).contains(&a) {
                    bios[((a - BIOS_BASE) / 2) as usize] = Some(f);
                }
            }
        }
        Self { prom, bios, misses: BTreeMap::new(), native: 0, interp: 0 }
    }

    pub fn len(&self) -> usize {
        gen::COUNT
    }
    pub fn is_empty(&self) -> bool {
        gen::COUNT == 0
    }

    #[inline]
    pub fn get(&self, pc: u32) -> Option<InsFn> {
        let pc = pc & 0xFF_FFFF;
        if pc & 1 != 0 {
            return None;
        }
        if pc < PROM_END {
            self.prom[(pc / 2) as usize]
        } else if (BIOS_BASE..BIOS_END).contains(&pc) {
            self.bios[((pc - BIOS_BASE) / 2) as usize]
        } else {
            None
        }
    }

    /// Un paso de CPU equivalente a `System::step`, pero ejecutando la
    /// instrucción traducida.
    #[inline]
    pub fn step(&mut self, sys: &mut System) -> u32 {
        let used = self.cpu_step(sys);
        sys.post_cpu(used);
        used
    }

    /// Sólo la parte CPU (sin avanzar el hardware).
    #[inline]
    pub fn cpu_step(&mut self, sys: &mut System) -> u32 {
        let cpu = &mut sys.m68k;
        let irq_takes = cpu.pending_irq > 0
            && (cpu.pending_irq == 7 || cpu.pending_irq > cpu.sr.interrupt_mask());
        if irq_takes {
            let level = cpu.pending_irq;
            cpu.pending_irq = 0;
            cpu.stopped = false;
            let vec = match level {
                1 => Exception::Interrupt1,
                2 => Exception::Interrupt2,
                3 => Exception::Interrupt3,
                _ => Exception::Spurious,
            };
            cpu.enter_exception(&mut sys.bus, vec);
            cpu.sr.set_interrupt_mask(level);
            cpu.cycles = cpu.cycles.wrapping_add(44);
            return 44;
        }
        if cpu.stopped {
            cpu.cycles = cpu.cycles.wrapping_add(4);
            return 4;
        }
        if let Some(f) = self.get(cpu.pc) {
            cpu.instr_pc = cpu.pc;
            let c = f(cpu, &mut sys.bus);
            cpu.cycles = cpu.cycles.wrapping_add(u64::from(c));
            self.native += 1;
            return c;
        }
        *self.misses.entry(cpu.pc & 0xFF_FFFF).or_default() += 1;
        self.interp += 1;
        cpu.step(&mut sys.bus)
    }

    /// Un frame (mismo presupuesto que `System::run_frame`).
    pub fn run_frame(&mut self, sys: &mut System) {
        let mut budget: u64 = 202_752;
        while budget > 0 {
            budget = budget.saturating_sub(u64::from(self.step(sys)));
        }
        sys.dbg_frame = sys.dbg_frame.wrapping_add(1);
    }
}
