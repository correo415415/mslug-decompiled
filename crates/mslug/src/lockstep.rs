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
