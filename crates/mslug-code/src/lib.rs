//! Programa 68000 de Metal Slug (P-ROM del juego + BIOS Neo Geo) traducido
//! a Rust por `tools/recomp/m68k_recomp.py`.
//!
//! Cada instrucción de la ROM es una función Rust (`mslug_gen_NN::*::i_XXXXXX`, en crates/gen/NN) que
//! hace exactamente lo que haría la CPU: mismos registros, flags, accesos al
//! bus y ciclos. El juego ya no se interpreta: `Program::step` llama a estas
//! funciones según el PC. El intérprete 68000 sólo queda como red de
//! seguridad para direcciones que el recompilador aún no conoce (`misses`);
//! se añaden a `tools/recomp/seeds.txt` y desaparecen al regenerar. El
//! objetivo final es 0 misses y retirar el intérprete.

mod gen;

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
    /// Modo "sólo port manual" (ver [`Program::manual_only`]): sin
    /// intérprete; al llegar a código sin función se para aquí.
    pub only_manual: bool,
    /// Dónde se paró en modo sólo manual (PC de la primera instrucción que
    /// no tiene función escrita a mano).
    pub halted_at: Option<u32>,
    /// Modo "seguir" (ver [`Program::keep_going`]): tabla del juego
    /// recompilado para rellenar las rutinas sin portar en vez de pararse.
    fallback: Option<Vec<Option<InsFn>>>,
    /// La instrucción anterior vino de `fallback` (para contar sólo las
    /// entradas al código sin portar, no cada instrucción).
    in_fallback: bool,
    /// Entradas al código sin portar en modo seguir: PC de entrada ->
    /// (primer frame, veces que se entró, instrucciones ejecutadas).
    pub fallback_hits: BTreeMap<u32, (u32, u64, u64)>,
    /// PC de entrada del tramo sin portar que se está ejecutando.
    fallback_entry: u32,
    /// Instrucciones ejecutadas por el relleno recompilado.
    pub fallback_ins: u64,
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
        for t in gen::TABLES.iter().flat_map(|c| c.iter()) {
            for &(a, f) in t.iter() {
                if a < PROM_END {
                    prom[(a / 2) as usize] = Some(f);
                } else if (BIOS_BASE..BIOS_END).contains(&a) {
                    bios[((a - BIOS_BASE) / 2) as usize] = Some(f);
                }
            }
        }
        Self {
            prom,
            bios,
            misses: BTreeMap::new(),
            native: 0,
            interp: 0,
            only_manual: false,
            halted_at: None,
            fallback: None,
            in_fallback: false,
            fallback_hits: BTreeMap::new(),
            fallback_entry: 0,
            fallback_ins: 0,
        }
    }

    /// Programa **sin** el juego recompilado: sólo ejecuta lo que se añada
    /// con [`Program::set_override`] (el port escrito a mano). Con
    /// `with_bios`, la BIOS de la consola (que no es parte del juego) sigue
    /// disponible recompilada. Al llegar a una instrucción del juego que no
    /// tiene función, la CPU se para y se anota en `halted_at`: sirve para
    /// ver hasta dónde llega el juego sólo con lo portado.
    pub fn manual_only(with_bios: bool) -> Self {
        let mut p = Self::new();
        p.prom.iter_mut().for_each(|f| *f = None);
        if !with_bios {
            p.bios.iter_mut().for_each(|f| *f = None);
        }
        p.only_manual = true;
        p
    }

    /// Como [`Program::manual_only`], pero sin pararse: cuando el port
    /// llega a una instrucción sin función escrita a mano, ejecuta la del
    /// juego recompilado (o el intérprete si tampoco la hay) y lo anota en
    /// `fallback_hits`. En cuanto ese código llama a una rutina portada, se
    /// vuelve a usar el port. Sirve para ver de una pasada todas las rutinas
    /// que faltan en una partida larga en vez de una parada cada vez.
    pub fn keep_going(with_bios: bool) -> Self {
        let full = Self::new().prom;
        let mut p = Self::manual_only(with_bios);
        p.fallback = Some(full);
        p
    }

    pub fn len(&self) -> usize {
        gen::COUNT
    }
    pub fn is_empty(&self) -> bool {
        gen::COUNT == 0
    }

    /// Sustituye la instrucción de `addr` (entrada de una rutina) por `f`.
    /// Lo usa el port escrito a mano (`mslug_bridge::install`): `f` hace
    /// todo el trabajo de la rutina y vuelve al llamador.
    pub fn set_override(&mut self, addr: u32, f: InsFn) {
        let a = addr & 0xFF_FFFF;
        assert!(a & 1 == 0 && a < PROM_END, "set_override: ${a:06X} fuera de la P-ROM");
        self.prom[(a / 2) as usize] = Some(f);
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
        if self.halted_at.is_some() {
            return 4;
        }
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
            self.in_fallback = false;
            return c;
        }
        if let Some(fb) = &self.fallback {
            let pc = cpu.pc & 0xFF_FFFF;
            if !self.in_fallback {
                let frame = sys.dbg_frame;
                self.fallback_hits.entry(pc).or_insert((frame, 0, 0)).1 += 1;
                self.fallback_entry = pc;
                self.in_fallback = true;
            }
            if let Some(e) = self.fallback_hits.get_mut(&self.fallback_entry) {
                e.2 += 1;
            }
            self.fallback_ins += 1;
            if let Some(f) = fb.get((pc / 2) as usize).copied().flatten().filter(|_| pc & 1 == 0 && pc < PROM_END) {
                cpu.instr_pc = cpu.pc;
                let c = f(cpu, &mut sys.bus);
                cpu.cycles = cpu.cycles.wrapping_add(u64::from(c));
                return c;
            }
            *self.misses.entry(pc).or_default() += 1;
            self.interp += 1;
            return cpu.step(&mut sys.bus);
        }
        *self.misses.entry(cpu.pc & 0xFF_FFFF).or_default() += 1;
        if self.only_manual {
            self.halted_at = Some(cpu.pc & 0xFF_FFFF);
            return 4;
        }
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

/// Añade a `path` (formato de `tools/recomp/seeds.txt`: una dirección hex por
/// línea) las direcciones que se ejecutaron sin traducción y que aún no
/// estén en el fichero.
pub fn append_misses(prog: &Program, path: &std::path::Path) -> std::io::Result<usize> {
    use std::io::Write;
    let old = std::fs::read_to_string(path).unwrap_or_default();
    let have: std::collections::BTreeSet<u32> = old
        .lines()
        .filter_map(|l| u32::from_str_radix(l.split('#').next()?.trim(), 16).ok())
        .collect();
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
    let mut n = 0;
    for &a in prog.misses.keys() {
        if !have.contains(&a) {
            writeln!(f, "{a:06X}")?;
            n += 1;
        }
    }
    eprintln!("{n} direcciones nuevas -> {}", path.display());
    Ok(n)
}
