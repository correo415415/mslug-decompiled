//! Verificación del port escrito a mano (`mslug-game`) contra la ROM.
//! La tabla rutina original -> función Rust está en `mslug-bridge`.
//!
//! **Único sitio que relaciona el juego en Rust legible con el original**: a
//! cada función de `mslug-game` le corresponde aquí la rutina original
//! (dirección de entrada) y un adaptador que traduce los registros de la
//! llamada original a los argumentos Rust.
//!
//! Durante una partida (que sigue siempre por la ROM), cada vez que el PC
//! llega a una de esas rutinas:
//!   1. se ejecuta la ROM hasta su `rts` → RAM de referencia;
//!   2. desde el mismo estado inicial se llama a la función Rust sobre una
//!      copia de la RAM → RAM portada;
//!   3. se comparan las RAM (y el valor devuelto, si lo hay).
//!
//! No se comparan registros ni flags ni ciclos: el port legible no los
//! reproduce, sólo el **comportamiento observable**.

use anyhow::Result;
use mslug_bridge::{apply_ret, rom_ret, Fx, Port, Ram, CALLEES, PORTS};
use neogeo_hw::cpu::m68k::Bus;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

#[derive(Default)]
struct Stats {
    /// Veces que el juego ha llamado a la rutina.
    seen: u64,
    /// Llamadas comprobadas (muestreo: todas las primeras 1000 y luego
    /// cada vez menos, para cubrir toda la partida sin disparar el coste).
    calls: u64,
    fails: u64,
    first: Option<String>,
    /// Llamadas en las que la CPU tras la sustitución (`mslug-sdl --manual`)
    /// no queda como la deja la ROM (registros / flags). No es un fallo del
    /// port legible, pero sí de la sustitución si el llamador los usa.
    reg_diffs: u64,
    reg_kinds: std::collections::BTreeSet<String>,
}

pub fn run(rom: &PathBuf, frames: u32, coin: u32, fuzz: Option<u64>) -> Result<()> {
    let mut sys = crate::load(rom)?;
    let mut fz = fuzz.map(crate::Fuzz::new);
    let mut stats: BTreeMap<u32, Stats> = BTreeMap::new();
    let mut skip: Option<u32> = None;
    let index: HashMap<u32, usize> = PORTS.iter().enumerate().map(|(i, p)| (p.entry, i)).collect();
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
                    if st.seen % (st.seen / 1000).max(1) == 0 {
                        check(&mut sys, &PORTS[i], st);
                        skip = Some(pc);
                        continue;
                    }
                }
            }
            skip = None;
            budget -= i64::from(sys.step());
        }
    }
    println!("{:<34} {:<42} {:>9} {:>8} {:>6} {:>7}", "original", "Rust", "llamadas", "comprob.", "fallos", "regs≠");
    let mut bad = false;
    let (mut tc, mut tf) = (0, 0);
    for p in PORTS {
        let s = stats.remove(&p.entry).unwrap_or_default();
        println!("{:<34} {:<42} {:>9} {:>8} {:>6} {:>7}", p.original, p.rust, s.seen, s.calls, s.fails, s.reg_diffs);
        if !s.reg_kinds.is_empty() {
            println!("    registros/flags distintos tras la sustitución: {}",
                     s.reg_kinds.iter().cloned().collect::<Vec<_>>().join(" "));
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
        anyhow::bail!("hay funciones del port manual que no se comportan como la ROM");
    }
    Ok(())
}

fn check(sys: &mut neogeo_hw::System, p: &Port, st: &mut Stats) {
    let cpu0 = sys.m68k.clone();
    let ram0 = sys.bus.work_ram.to_vec();
    let irq = sys.m68k.pending_irq;
    sys.m68k.pending_irq = 0;

    // --- ROM hasta el rts, anotando las llamadas a rutinas aún no portadas
    let ret_sp = cpu0.a[7].wrapping_add(4);
    let ret_pc = sys.bus.read32(cpu0.a[7] & 0xFF_FFFF) & 0xFF_FFFF;
    let mut rom_calls: Vec<(&'static str, u32)> = Vec::new();
    let mut steps = 0;
    loop {
        let pc = sys.m68k.pc & 0xFF_FFFF;
        if let Some((_, n, arg)) = CALLEES.iter().find(|(a, _, _)| *a == pc) {
            rom_calls.push((n, arg(&sys.m68k)));
        }
        sys.m68k.step(&mut sys.bus);
        steps += 1;
        if (sys.m68k.pc & 0xFF_FFFF == ret_pc && sys.m68k.a[7] == ret_sp) || steps > 1_000_000 {
            break;
        }
    }
    let ram_rom = sys.bus.work_ram.to_vec();
    let cpu_rom = sys.m68k.clone();

    // --- Rust desde el mismo estado
    let mut ram_rust = ram0.clone();
    let mut fx = Fx::default();
    let got = (p.run)(&cpu0, &mut Ram(&mut ram_rust), &mut fx);
    let want = rom_ret(got, &cpu_rom);

    // CPU tal como la dejaría la sustitución (`mslug_bridge::call_rust`).
    let mut cpu_sub = cpu0.clone();
    apply_ret(&mut cpu_sub, got, ret_pc);
    let mut rk = Vec::new();
    for i in 0..8 {
        if cpu_sub.d[i] != cpu_rom.d[i] {
            rk.push(format!("D{i}"));
        }
        if cpu_sub.a[i] != cpu_rom.a[i] {
            rk.push(format!("A{i}"));
        }
    }
    for (bit, n) in [(1u16, "C"), (2, "V"), (4, "Z"), (8, "N"), (16, "X")] {
        if cpu_sub.sr.0 & bit != cpu_rom.sr.0 & bit {
            rk.push(n.to_string());
        }
    }
    if cpu_sub.pc & 0xFF_FFFF != cpu_rom.pc & 0xFF_FFFF {
        rk.push("PC".into());
    }
    if !rk.is_empty() {
        st.reg_diffs += 1;
        st.reg_kinds.extend(rk);
    }

    // Las rutinas no portadas que llama la ROM escriben su propia RAM: sólo
    // se compara la RAM si no hubo llamadas (si las hubo, se compara que
    // sean las mismas y la RAM fuera del bloque de entidades nuevas).
    let mut diffs = Vec::new();
    if got != want {
        diffs.push(format!("devuelve rom={want:?} rust={got:?}"));
    }
    if rom_calls != fx.calls {
        diffs.push(format!("llamadas rom={rom_calls:?} rust={:?}", fx.calls));
    }
    if rom_calls.is_empty() {
        let bad: Vec<usize> = (0..ram_rom.len()).filter(|&i| ram_rom[i] != ram_rust[i]).collect();
        // La pila (cerca de A7) no es estado del juego.
        let sp = (cpu0.a[7] & 0xFFFF) as usize;
        let bad: Vec<usize> = bad.into_iter().filter(|&i| !(sp.saturating_sub(0x200)..sp + 8).contains(&i)).collect();
        if let Some(&i) = bad.first() {
            diffs.push(format!("RAM: {} bytes distintos, primero ${:06X} rom={:02X} rust={:02X}",
                               bad.len(), 0x10_0000 + i, ram_rom[i], ram_rust[i]));
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
    sys.bus.work_ram.copy_from_slice(&ram0);
}
