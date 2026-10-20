//! Verificación de las rutinas de sistema portadas (`mslug_bridge::system`)
//! contra la ROM.
//!
//! Durante una partida (que sigue siempre por la ROM), cada vez que el PC
//! llega a la entrada de una rutina de sistema portada:
//!   1. desde una copia del estado completo (CPU y bus: RAM, paleta, VRAM,
//!      registros de vídeo y sonido) se ejecuta la función Rust, que dice
//!      cómo sigue la ejecución (vuelve, `rte`, salta, llama a una tarea o
//!      espera) → PC y pila de destino;
//!   2. desde otra copia se ejecuta la ROM hasta llegar a ese mismo PC con
//!      esa misma pila;
//!   3. se comparan RAM (salvo la zona de la pila), paleta, VRAM y la
//!      última orden al Z80. Los registros distintos se cuentan aparte.
//!
//! Las rutinas de sistema llaman a rutinas aún sin portar: en el lado Rust
//! se ejecutan con el intérprete, igual que en el lado ROM.

use anyhow::Result;
use mslug_bridge::system::{call_system, set_mode, SYSTEM_PORTS};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

#[derive(Default)]
struct Stats {
    seen: u64,
    calls: u64,
    fails: u64,
    first: Option<String>,
    reg_diffs: u64,
    reg_kinds: BTreeSet<String>,
}

pub fn run(rom: &PathBuf, frames: u32, coin: u32, fuzz: Option<u64>) -> Result<()> {
    set_mode(false, true);
    let mut sys = crate::load(rom)?;
    let mut fz = fuzz.map(crate::Fuzz::new);
    let mut stats: BTreeMap<u32, Stats> = BTreeMap::new();
    let index: HashMap<u32, usize> = SYSTEM_PORTS.iter().enumerate().map(|(i, p)| (p.entry, i)).collect();
    let mut skip: Option<u32> = None;
    for f in 0..frames {
        match fz.as_mut() {
            Some(z) => z.inputs(&mut sys, f),
            None => crate::auto_inputs(&mut sys, coin, f),
        }
        let mut budget: i64 = 202_752;
        while budget > 0 {
            let pc = sys.m68k.pc & 0xFF_FFFF;
            if skip != Some(pc) {
                if let Some(&i) = index.get(&pc) {
                    let st = stats.entry(pc).or_default();
                    st.seen += 1;
                    if st.seen % (st.seen / 500).max(1) == 0 {
                        check(&mut sys, i, st);
                        skip = Some(pc);
                        continue;
                    }
                }
            }
            skip = None;
            budget -= i64::from(sys.step());
        }
    }
    println!("{:<36} {:<40} {:>9} {:>8} {:>6} {:>6}", "original", "Rust", "llamadas", "comprob.", "fallos", "regs≠");
    let mut bad = false;
    let (mut tc, mut tf) = (0, 0);
    for p in SYSTEM_PORTS {
        let Some(s) = stats.remove(&p.entry) else {
            println!("{:<36} {:<40} {:>9}", p.original, p.rust, 0);
            continue;
        };
        println!("{:<36} {:<40} {:>9} {:>8} {:>6} {:>6}", p.original, p.rust, s.seen, s.calls, s.fails, s.reg_diffs);
        if !s.reg_kinds.is_empty() {
            println!("    registros distintos: {}", s.reg_kinds.iter().cloned().collect::<Vec<_>>().join(" "));
        }
        tc += s.calls;
        tf += s.fails;
        if let Some(m) = s.first {
            println!("    primer fallo: {m}");
            bad = true;
        }
    }
    println!("total: {tc} llamadas comprobadas, {tf} fallos ({frames} frames)");
    if bad {
        anyhow::bail!("hay rutinas de sistema portadas que no se comportan como la ROM");
    }
    Ok(())
}

fn first_diff(a: &[u8], b: &[u8], skip: impl Fn(usize) -> bool) -> Option<(usize, usize)> {
    let bad: Vec<usize> = (0..a.len().min(b.len())).filter(|&i| a[i] != b[i] && !skip(i)).collect();
    bad.first().map(|&i| (i, bad.len()))
}

fn check(sys: &mut neogeo_hw::System, i: usize, st: &mut Stats) {
    let p = &SYSTEM_PORTS[i];
    let irq = sys.m68k.pending_irq;
    sys.m68k.pending_irq = 0;
    let cpu0 = sys.m68k.clone();
    let bus0 = sys.bus.clone();

    // --- Rust
    let mut cpu_r = cpu0.clone();
    let mut bus_r = bus0.clone();
    call_system(&mut cpu_r, &mut bus_r);
    let target_pc = cpu_r.pc & 0xFF_FFFF;
    let target_sp = cpu_r.a[7];

    // --- ROM hasta el mismo punto
    let mut steps = 0u32;
    let reached = loop {
        sys.m68k.step(&mut sys.bus);
        steps += 1;
        if sys.m68k.pc & 0xFF_FFFF == target_pc && sys.m68k.a[7] == target_sp {
            break true;
        }
        if steps > 3_000_000 {
            break false;
        }
    };

    let mut diffs = Vec::new();
    if !reached {
        diffs.push(format!(
            "la ROM no llega a PC ${target_pc:06X} con A7 ${target_sp:08X} (está en ${:06X}, A7 ${:08X})",
            sys.m68k.pc & 0xFF_FFFF,
            sys.m68k.a[7]
        ));
    } else {
        // Pila: lo que haya por debajo del A7 inicial no es estado del juego.
        let sp = (cpu0.a[7] & 0xFFFF) as usize;
        let in_stack = |i: usize| (sp.saturating_sub(0x800)..sp).contains(&i);
        if let Some((i, n)) = first_diff(&sys.bus.work_ram[..], &bus_r.work_ram[..], in_stack) {
            diffs.push(format!(
                "RAM: {n} bytes distintos, primero ${:06X} rom={:02X} rust={:02X}",
                0x10_0000 + i,
                sys.bus.work_ram[i],
                bus_r.work_ram[i]
            ));
        }
        if let Some((i, n)) = first_diff(&sys.bus.palette_ram[..], &bus_r.palette_ram[..], |_| false) {
            diffs.push(format!("paleta: {n} bytes distintos, primero +${i:04X}"));
        }
        let (va, vb) = (&sys.bus.lspc.vram[..], &bus_r.lspc.vram[..]);
        if let Some(k) = (0..va.len()).find(|&k| va[k] != vb[k]) {
            diffs.push(format!("VRAM: primera diferencia en ${k:04X} rom={:04X} rust={:04X}", va[k], vb[k]));
        }
        if sys.bus.sound_latch != bus_r.sound_latch {
            diffs.push(format!("orden al Z80 rom={:02X} rust={:02X}", sys.bus.sound_latch, bus_r.sound_latch));
        }
        if sys.bus.lspc.vram_addr != bus_r.lspc.vram_addr || sys.bus.lspc.vram_mod != bus_r.lspc.vram_mod {
            diffs.push("registros de VRAM (dirección / incremento) distintos".into());
        }
        if (sys.m68k.sr.0 ^ cpu_r.sr.0) & 0x0700 != 0 {
            diffs.push(format!("máscara de interrupciones rom={} rust={}", sys.m68k.sr.interrupt_mask(), cpu_r.sr.interrupt_mask()));
        }
        let mut rk = Vec::new();
        for r in 0..8 {
            if sys.m68k.d[r] != cpu_r.d[r] {
                rk.push(format!("D{r}"));
            }
            if r < 7 && sys.m68k.a[r] != cpu_r.a[r] {
                rk.push(format!("A{r}"));
            }
        }
        if !rk.is_empty() {
            st.reg_diffs += 1;
            st.reg_kinds.extend(rk);
        }
    }
    st.calls += 1;
    if !diffs.is_empty() {
        st.fails += 1;
        if st.first.is_none() {
            st.first = Some(format!("llamada {}: {}", st.calls, diffs.join("; ")));
        }
    }

    // --- el juego sigue por la ROM desde el estado inicial
    sys.m68k = cpu0;
    sys.m68k.pending_irq = irq;
    sys.bus = bus0;
}
