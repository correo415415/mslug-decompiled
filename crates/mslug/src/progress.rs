//! Progreso del port escrito a mano: funciones del juego (las rutinas de
//! código de la decompilación), cuántas tienen ya función Rust legible, el
//! porcentaje (por número y por bytes) y hasta dónde llega el juego en el
//! modo sólo manual (`--only-manual`).

use anyhow::Result;
use mslug_bridge::symbols;
use std::collections::HashSet;
use std::path::PathBuf;

/// Rutinas de la decompilación: `addr  size  name  file  kind`.
static FUNCS: &str = include_str!("../../../tools/recomp/decomp_funcs.tsv");

/// Tarea del guion de arranque y su cursor (la escena en curso).
const SCRIPT_TASK: u32 = 0x10_01C0;
const SCRIPT_CURSOR: u32 = 0x70;

struct Func {
    addr: u32,
    size: u32,
}

fn code_functions() -> Vec<Func> {
    FUNCS
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() < 5 || !(f[4] == "code" || f[4] == "mixed") {
                return None;
            }
            Some(Func { addr: u32::from_str_radix(f[0], 16).ok()?, size: f[1].parse().ok()? })
        })
        .collect()
}

pub fn run(rom: &PathBuf, frames: u32, coin: u32) -> Result<()> {
    // --- funciones portadas
    let funcs = code_functions();
    let starts: HashSet<u32> = funcs.iter().map(|f| f.addr).collect();
    let mut ported: HashSet<u32> = mslug_bridge::PORTS.iter().map(|p| p.entry).collect();
    ported.extend(mslug_bridge::system::SYSTEM_PORTS.iter().map(|p| p.entry));
    let entries = ported.len();
    let done: Vec<&Func> = funcs.iter().filter(|f| ported.contains(&f.addr)).collect();
    let total_bytes: u64 = funcs.iter().map(|f| u64::from(f.size)).sum();
    let done_bytes: u64 = done.iter().map(|f| u64::from(f.size)).sum();
    let other = ported.iter().filter(|a| !starts.contains(a)).count();

    // --- hasta dónde llega el modo sólo manual
    let mut sys = crate::load(rom)?;
    let mut prog = mslug_code::Program::manual_only(true);
    mslug_bridge::system::set_mode(true, true);
    mslug_bridge::install_all(|a, f| prog.set_override(a, f));
    let mut halt = None;
    for f in 0..frames {
        crate::auto_inputs(&mut sys, coin, f);
        prog.run_frame(&mut sys);
        if let Some(pc) = prog.halted_at {
            halt = Some((f, pc));
            break;
        }
    }
    let ram = |a: u32| -> u32 {
        let i = (a - 0x10_0000) as usize;
        u32::from_be_bytes([sys.bus.work_ram[i], sys.bus.work_ram[i + 1], sys.bus.work_ram[i + 2], sys.bus.work_ram[i + 3]])
    };
    let cursor = ram(SCRIPT_TASK + SCRIPT_CURSOR);
    let scene = match rom_u32(&sys, cursor) {
        Some(h) => symbols::describe(h),
        None => "?".into(),
    };

    println!("Progreso del port escrito a mano");
    println!(
        "  funciones del juego (código de la decompilación): {}  ({} bytes)",
        funcs.len(),
        total_bytes
    );
    println!(
        "  portadas con función Rust legible:                 {}  ({:.2} %)",
        done.len(),
        100.0 * done.len() as f64 / funcs.len() as f64
    );
    println!(
        "  por bytes de código:                               {} de {}  ({:.2} %)",
        done_bytes,
        total_bytes,
        100.0 * done_bytes as f64 / total_bytes as f64
    );
    println!("  entradas sustituidas (incl. {other} puntos de entrada internos o vectores): {entries}");
    match halt {
        Some((f, pc)) => println!(
            "  --only-manual: llega al frame {f}; se para en ${pc:06X} = {}; escena del guion: {scene}",
            symbols::describe(pc)
        ),
        None => println!("  --only-manual: {frames} frames sin pararse; escena del guion: {scene}"),
    }
    Ok(())
}

/// Long de la ROM del programa (la escena es un puntero de la lista del
/// guion, que está en la P-ROM).
fn rom_u32(sys: &neogeo_hw::System, a: u32) -> Option<u32> {
    let p = &sys.bus.p_rom;
    let off = match a {
        0..=0x0F_FFFF => a as usize,
        0x20_0000..=0x2F_FFFF => (a - 0x10_0000) as usize,
        _ => return None,
    };
    let b = p.get(off..off + 4)?;
    Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}
