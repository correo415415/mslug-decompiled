//! Hoja de ruta del port manual: qué rutinas originales necesita el juego,
//! en el orden en que las ejecuta por primera vez, y cuánto de la ejecución
//! cubre ya el Rust escrito a mano.
//!
//! Se juega una partida con el programa recompilado y se anota, por cada
//! instrucción, la rutina (etiqueta de la decompilación) a la que pertenece.
//! El modo `--only-manual` se para en la primera rutina de esta lista que no
//! esté portada: portar la lista en orden es lo que hace avanzar ese modo.

use anyhow::Result;
use mslug_bridge::symbols;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
struct R {
    first_frame: u32,
    order: usize,
    instrs: u64,
}

pub fn run(rom: &PathBuf, frames: u32, coin: u32, fuzz: Option<u64>, top: usize, out: Option<PathBuf>) -> Result<()> {
    let mut sys = crate::load(rom)?;
    let mut prog = mslug_code::Program::new();
    let mut fz = fuzz.map(crate::Fuzz::new);
    let ported: std::collections::HashSet<u32> = mslug_bridge::PORTS.iter().map(|p| p.entry).collect();
    let mut seen: HashMap<u32, R> = HashMap::new();
    let (mut game, mut bios) = (0u64, 0u64);
    // Caché: dirección -> etiqueta (las búsquedas binarias por instrucción
    // son caras).
    let mut cache: HashMap<u32, u32> = HashMap::new();
    for f in 0..frames {
        match fz.as_mut() {
            Some(z) => z.inputs(&mut sys, f),
            None => crate::auto_inputs(&mut sys, coin, f),
        }
        let mut budget: u64 = 202_752;
        while budget > 0 {
            let pc = sys.m68k.pc & 0xFF_FFFF;
            if pc < 0x20_0000 {
                game += 1;
                let lab = *cache.entry(pc).or_insert_with(|| symbols::containing(pc).map_or(pc, |(a, _)| a));
                let n = seen.len();
                let r = seen.entry(lab).or_insert_with(|| R { first_frame: f, order: n, instrs: 0 });
                r.instrs += 1;
            } else {
                bios += 1;
            }
            budget = budget.saturating_sub(u64::from(prog.step(&mut sys)));
        }
        sys.dbg_frame = sys.dbg_frame.wrapping_add(1);
    }
    // Las funciones portadas cubren su etiqueta (las etiquetas internas de
    // una rutina portada también).
    let covered = |a: u32| ported.contains(&a);
    let cov: u64 = seen.iter().filter(|(a, _)| covered(**a)).map(|(_, r)| r.instrs).sum();
    let mut v: Vec<(&u32, &R)> = seen.iter().collect();
    v.sort_by_key(|(_, r)| r.order);
    let mut txt = String::new();
    txt += &format!(
        "{frames} frames: {} rutinas del juego ejecutadas, {} portadas; instrucciones del juego {} (BIOS {}), en rutinas portadas {:.3}%\n",
        seen.len(),
        seen.keys().filter(|a| covered(**a)).count(),
        game,
        bios,
        100.0 * cov as f64 / game.max(1) as f64
    );
    txt += "orden  frame  portada  instrucc.  rutina\n";
    for (a, r) in &v {
        txt += &format!(
            "{:>5} {:>6}  {:^7}  {:>9}  {}\n",
            r.order,
            r.first_frame,
            if covered(**a) { "sí" } else { "" },
            r.instrs,
            symbols::describe(**a)
        );
    }
    let mut hot: Vec<(&u32, &R)> = seen.iter().filter(|(a, _)| !covered(**a)).collect();
    hot.sort_by_key(|(_, r)| std::cmp::Reverse(r.instrs));
    let mut head = txt.lines().next().unwrap_or("").to_string() + "\n";
    head += "\nPrimeras rutinas sin portar (lo que para --only-manual, en orden):\n";
    for (a, r) in v.iter().filter(|(a, _)| !covered(**a)).take(top) {
        head += &format!("  frame {:>5}  {}\n", r.first_frame, symbols::describe(**a));
    }
    head += "\nRutinas sin portar que más se ejecutan:\n";
    for (a, r) in hot.iter().take(top) {
        head += &format!("  {:>10} instr.  {}\n", r.instrs, symbols::describe(**a));
    }
    print!("{head}");
    if let Some(p) = out {
        std::fs::write(&p, txt)?;
        println!("\nlista completa -> {}", p.display());
    }
    Ok(())
}
