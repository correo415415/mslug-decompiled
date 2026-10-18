//! Modo diferencial: comprueba el contrato 1:1 de cada función portada.
//!
//! El juego corre con la ROM original. Cada vez que el PC llega a la entrada
//! de una función portada:
//!   1. se guarda el estado (CPU + 64 KiB de RAM de trabajo);
//!   2. se ejecuta la rutina 68000 de la ROM instrucción a instrucción hasta
//!      su `rts` (sin interrupciones: se aplazan) → estado de referencia;
//!   3. se restaura el estado y se ejecuta la versión Rust → estado portado;
//!   4. se comparan D0-D7, A0-A7, SR, PC, ciclos y la RAM;
//!   5. se restaura el estado inicial y el juego sigue por la ROM, de modo
//!      que la partida es idéntica a la original.

use crate::game;
use anyhow::Result;
use neogeo_hw::cpu::m68k::{Bus, Cpu};
use neogeo_hw::native::NativeFn;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
struct Stats {
    calls: u64,
    fails: u64,
    first_fail: Option<String>,
}

fn ram_snapshot(sys: &neogeo_hw::System) -> Vec<u8> {
    sys.bus.work_ram.to_vec()
}

fn diff_cpu(a: &Cpu, b: &Cpu, ca: u32, cb: u32) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0..8 {
        if a.d[i] != b.d[i] { v.push(format!("D{i} rom={:08X} rust={:08X}", a.d[i], b.d[i])); }
        if a.a[i] != b.a[i] { v.push(format!("A{i} rom={:08X} rust={:08X}", a.a[i], b.a[i])); }
    }
    if a.sr.0 != b.sr.0 { v.push(format!("SR rom={:04X} rust={:04X}", a.sr.0, b.sr.0)); }
    if a.pc != b.pc { v.push(format!("PC rom={:06X} rust={:06X}", a.pc, b.pc)); }
    if ca != cb { v.push(format!("ciclos rom={ca} rust={cb}")); }
    v
}

pub fn run(rom: &PathBuf, frames: u32, coin: u32) -> Result<()> {
    let mut sys = crate::load(rom)?;
    let table: HashMap<u32, (&str, NativeFn)> =
        game::FUNCTIONS.iter().map(|(a, n, f)| (*a, (*n, *f))).collect();
    let mut stats: HashMap<u32, Stats> = HashMap::new();
    let mut skip_pc: Option<u32> = None;

    for f in 0..frames {
        crate::auto_inputs(&mut sys, coin, f);
        let mut budget: i64 = 200_000;
        while budget > 0 {
            let pc = sys.m68k.pc;
            if skip_pc != Some(pc) {
                if let Some((name, nf)) = table.get(&pc) {
                    check(&mut sys, pc, name, *nf, stats.entry(pc).or_default());
                    skip_pc = Some(pc);
                    continue;
                }
            }
            skip_pc = None;
            budget -= i64::from(sys.step());
        }
    }

    let mut total = (0, 0);
    let mut ok = true;
    println!("{:<8} {:<42} {:>9} {:>6}", "dir", "función", "llamadas", "fallos");
    for (addr, name, _) in game::FUNCTIONS {
        let s = stats.remove(addr).unwrap_or_default();
        total.0 += s.calls;
        total.1 += s.fails;
        println!("${addr:06X}  {name:<42} {:>9} {:>6}", s.calls, s.fails);
        if let Some(m) = s.first_fail {
            println!("          primer fallo: {m}");
            ok = false;
        }
    }
    println!("total: {} llamadas comprobadas, {} fallos, {} frames", total.0, total.1, frames);
    if !ok {
        anyhow::bail!("hay funciones portadas que no son 1:1");
    }
    Ok(())
}

fn check(sys: &mut neogeo_hw::System, entry: u32, name: &str, nf: NativeFn, st: &mut Stats) {
    let cpu0 = sys.m68k.clone();
    let ram0 = ram_snapshot(sys);
    let irq = sys.m68k.pending_irq;
    sys.m68k.pending_irq = 0;

    // --- referencia: la ROM hasta el rts de esta llamada ---
    let ret_sp = cpu0.a[7].wrapping_add(4);
    let ret_pc = sys.bus.read32(cpu0.a[7] & 0xFF_FFFF) & 0xFF_FFFF;
    let mut rom_cyc = 0u32;
    let mut steps = 0;
    loop {
        rom_cyc += sys.m68k.step(&mut sys.bus);
        steps += 1;
        if (sys.m68k.pc == ret_pc && sys.m68k.a[7] == ret_sp) || steps > 2_000_000 {
            break;
        }
    }
    let cpu_rom = sys.m68k.clone();
    let ram_rom = ram_snapshot(sys);

    // --- portada: la función Rust desde el mismo estado ---
    sys.m68k = cpu0.clone();
    sys.bus.work_ram.copy_from_slice(&ram0);
    let rust_cyc = {
        let mut dynbus: &mut dyn Bus = &mut sys.bus;
        nf(&mut sys.m68k, &mut dynbus)
    };
    let mut diffs = diff_cpu(&cpu_rom, &sys.m68k, rom_cyc, rust_cyc);
    let ram_rust = ram_snapshot(sys);
    let bad: Vec<usize> = (0..ram_rom.len()).filter(|&i| ram_rom[i] != ram_rust[i]).collect();
    if let Some(&i) = bad.first() {
        diffs.push(format!("RAM: {} bytes distintos, primero ${:06X} rom={:02X} rust={:02X}",
                           bad.len(), 0x100000 + i, ram_rom[i], ram_rust[i]));
    }

    st.calls += 1;
    if !diffs.is_empty() {
        st.fails += 1;
        if st.first_fail.is_none() {
            st.first_fail = Some(format!("{name} @${entry:06X} (llamada {}): {}", st.calls, diffs.join("; ")));
        }
    }

    // --- el juego sigue por la ROM desde el estado inicial ---
    sys.m68k = cpu0;
    sys.m68k.pending_irq = irq;
    sys.bus.work_ram.copy_from_slice(&ram0);
}
