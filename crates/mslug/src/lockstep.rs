//! Verificación en paralelo: intérprete 68000 (referencia) vs programa
//! recompilado a Rust (`mslug-code`).
//!
//! Dos `System` idénticos reciben las mismas entradas. En uno la CPU
//! interpreta la ROM (`System::step`); en el otro se ejecuta el código Rust
//! generado (`Program::step`). Tras CADA instrucción se comparan D0-D7,
//! A0-A7, USP/SSP, SR, PC y los ciclos consumidos; y al final de cada frame,
//! la RAM de trabajo completa, la paleta y el contador de ciclos totales.
//! Al primer desacuerdo se para y se informa de la instrucción culpable.

use anyhow::{bail, Result};
use mslug_code::Program;
use neogeo_hw::cpu::m68k::Cpu;
use std::path::PathBuf;

fn cpu_diff(a: &Cpu, b: &Cpu) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0..8 {
        if a.d[i] != b.d[i] {
            v.push(format!("D{i} ref={:08X} rust={:08X}", a.d[i], b.d[i]));
        }
        if a.a[i] != b.a[i] {
            v.push(format!("A{i} ref={:08X} rust={:08X}", a.a[i], b.a[i]));
        }
    }
    if a.sr.0 != b.sr.0 {
        v.push(format!("SR ref={:04X} rust={:04X}", a.sr.0, b.sr.0));
    }
    if a.pc != b.pc {
        v.push(format!("PC ref={:06X} rust={:06X}", a.pc, b.pc));
    }
    if a.usp != b.usp {
        v.push(format!("USP ref={:08X} rust={:08X}", a.usp, b.usp));
    }
    if a.ssp != b.ssp {
        v.push(format!("SSP ref={:08X} rust={:08X}", a.ssp, b.ssp));
    }
    if a.stopped != b.stopped {
        v.push(format!("stopped ref={} rust={}", a.stopped, b.stopped));
    }
    v
}

pub fn run(rom: &PathBuf, frames: u32, coin: u32) -> Result<()> {
    let mut r = crate::load(rom)?;
    let mut n = crate::load(rom)?;
    let mut prog = Program::new();
    eprintln!("programa recompilado: {} instrucciones", prog.len());
    let mut steps: u64 = 0;
    for f in 0..frames {
        crate::auto_inputs(&mut r, coin, f);
        crate::auto_inputs(&mut n, coin, f);
        let mut budget: u64 = 202_752;
        while budget > 0 {
            let pc = r.m68k.pc;
            let c1 = r.step();
            let c2 = prog.step(&mut n);
            steps += 1;
            let mut d = cpu_diff(&r.m68k, &n.m68k);
            if c1 != c2 {
                d.push(format!("ciclos ref={c1} rust={c2}"));
            }
            if !d.is_empty() {
                bail!(
                    "frame {f}, paso {steps}: instrucción en ${pc:06X} (ir ref={:04X}): {}",
                    r.m68k.ir,
                    d.join("; ")
                );
            }
            budget = budget.saturating_sub(u64::from(c1));
        }
        r.dbg_frame = r.dbg_frame.wrapping_add(1);
        n.dbg_frame = n.dbg_frame.wrapping_add(1);
        if r.bus.work_ram[..] != n.bus.work_ram[..] {
            let i = (0..r.bus.work_ram.len()).find(|&i| r.bus.work_ram[i] != n.bus.work_ram[i]).unwrap();
            bail!("frame {f}: RAM distinta en ${:06X} ref={:02X} rust={:02X}",
                  0x100000 + i, r.bus.work_ram[i], n.bus.work_ram[i]);
        }
        if r.bus.palette_ram[..] != n.bus.palette_ram[..] {
            bail!("frame {f}: paleta distinta");
        }
        if r.master_cycles != n.master_cycles {
            bail!("frame {f}: ciclos totales ref={} rust={}", r.master_cycles, n.master_cycles);
        }
        if f % 500 == 0 {
            eprintln!("frame {f}: ok ({} instr. nativas, {} interpretadas)", prog.native, prog.interp);
        }
    }
    println!(
        "lockstep OK: {frames} frames, {steps} instrucciones; nativas {} ({:.4}%), interpretadas {}",
        prog.native,
        100.0 * prog.native as f64 / (prog.native + prog.interp).max(1) as f64,
        prog.interp
    );
    if !prog.misses.is_empty() {
        println!("direcciones sin traducir ({}):", prog.misses.len());
        for (a, c) in prog.misses.iter().take(50) {
            println!("  ${a:06X} x{c}");
        }
    }
    Ok(())
}

/// Partida con el programa recompilado (referencia) frente a la misma
/// partida con las rutinas portadas a mano sustituidas (`--manual`). Las
/// funciones Rust no consumen los mismos ciclos que el original, así que no
/// se compara instrucción a instrucción, sino el **estado del juego** (RAM
/// salvo la pila, paleta) al final de cada frame. Si un llamador dependiera
/// de un registro o flag que la rutina original deja y la Rust no, aquí se
/// vería.
pub fn run_manual(rom: &PathBuf, frames: u32, coin: u32, fuzz: Option<u64>, atomic_ref: bool) -> Result<()> {
    let mut r = crate::load(rom)?;
    let mut n = crate::load(rom)?;
    let mut pr = Program::new();
    let mut pn = Program::new();
    let k = mslug_bridge::install(|a, f| pn.set_override(a, f));
    eprintln!("{k} rutinas sustituidas por Rust escrito a mano");
    if atomic_ref {
        // Referencia con la misma granularidad: cada rutina original en un
        // solo paso.
        mslug_bridge::install_with(|a, f| pr.set_override(a, f), mslug_bridge::call_rom_atomic);
    }
    let mut fr = fuzz.map(crate::Fuzz::new);
    let mut fn_ = fuzz.map(crate::Fuzz::new);
    for f in 0..frames {
        match (fr.as_mut(), fn_.as_mut()) {
            (Some(a), Some(b)) => {
                a.inputs(&mut r, f);
                b.inputs(&mut n, f);
            }
            _ => {
                crate::auto_inputs(&mut r, coin, f);
                crate::auto_inputs(&mut n, coin, f);
            }
        }
        if atomic_ref && std::env::var_os("MSLUG_MANUAL_EXACT_CYCLES").is_some() {
            // Mismo número de pasos y ciclos en los dos lados: se puede
            // comparar el flujo instrucción a instrucción.
            let mut budget: u64 = 202_752;
            let mut hist: std::collections::VecDeque<(u32, String)> = Default::default();
            // MSLUG_WATCH=1004C7: parar en el primer paso en que ese byte
            // de RAM difiere.
            let watch = std::env::var("MSLUG_WATCH").ok()
                .and_then(|v| u32::from_str_radix(&v, 16).ok())
                .map(|a| (a & 0xFFFF) as usize);
            while budget > 0 {
                let pc = r.m68k.pc & 0xFF_FFFF;
                let c1 = pr.step(&mut r);
                let c2 = pn.step(&mut n);
                let d = cpu_diff(&r.m68k, &n.m68k);
                hist.push_back((pc, d.join(" ")));
                if hist.len() > 12 {
                    hist.pop_front();
                }
                let wdiff = watch.is_some_and(|w| r.bus.work_ram[w] != n.bus.work_ram[w]);
                if r.m68k.pc != n.m68k.pc || c1 != c2 || wdiff {
                    let h: Vec<String> = hist.iter().map(|(a, d)| format!("  ${a:06X} {d}")).collect();
                    bail!("frame {f}: el flujo se separa (ciclos {c1}/{c2}); últimos pasos (pc, regs distintos tras él):\n{}", h.join("\n"));
                }
                budget = budget.saturating_sub(u64::from(c1));
            }
            r.dbg_frame = r.dbg_frame.wrapping_add(1);
            n.dbg_frame = n.dbg_frame.wrapping_add(1);
        } else {
            pr.run_frame(&mut r);
            pn.run_frame(&mut n);
        }
        // La pila del 68000 está al final de la RAM de trabajo ($10F000-).
        // $106EE0: contador de vueltas del bucle de espera del vblank (tiempo
        // libre del frame): depende de los ciclos, no del estado del juego.
        let lim = 0xF000;
        let d: Vec<usize> = (0..lim)
            .filter(|&i| !(0x6EE0..0x6EE2).contains(&i) && r.bus.work_ram[i] != n.bus.work_ram[i])
            .collect();
        if !d.is_empty() {
            let shown: Vec<String> = d.iter().take(12)
                .map(|&i| format!("${:06X} ref={:02X} manual={:02X}", 0x10_0000 + i, r.bus.work_ram[i], n.bus.work_ram[i]))
                .collect();
            bail!("frame {f}: RAM distinta en {} bytes: {}", d.len(), shown.join(", "));
        }
        if r.bus.palette_ram[..] != n.bus.palette_ram[..] {
            bail!("frame {f}: paleta distinta");
        }
        if f % 1000 == 0 {
            eprintln!("frame {f}: ok");
        }
    }
    println!("manual OK: {frames} frames con el port manual sustituido, mismo estado que el recompilado");
    Ok(())
}
