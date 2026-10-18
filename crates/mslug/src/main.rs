//! `mslug` — port 1:1 de Metal Slug (Neo Geo, 1996) a Rust.
//!
//! Estrategia (ver README.md):
//!   * El hardware (68000, Z80/YM2610, LSPC, bus) lo pone `neogeo-hw`.
//!   * El juego se va sustituyendo función a función: cada rutina 68000
//!     portada (módulo `game`) se registra por su dirección de entrada y la
//!     CPU la ejecuta en Rust en lugar de interpretar la ROM.
//!   * `verify` comprueba el contrato 1:1: en cada llamada a una función
//!     portada ejecuta la versión 68000 original y la Rust sobre el mismo
//!     estado y exige registros, SR, memoria tocada y ciclos idénticos.

mod game;
mod verify;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use neogeo_hw::native::NativeTable;
use neogeo_hw::{RomSet, System, SystemConfig};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "mslug", about = "Metal Slug — port 1:1 a Rust")]
struct Cli {
    /// Zip MAME del juego (mslug.zip, con la BIOS neogeo dentro o al lado).
    #[arg(long, global = true, default_value = "internal/mslug.zip")]
    rom: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Ejecuta el juego (headless) con las funciones portadas activas.
    Run {
        #[arg(long, default_value_t = 3000)]
        frames: u32,
        /// Directorio para volcar capturas PNG.
        #[arg(long)]
        dump: Option<PathBuf>,
        #[arg(long, default_value_t = 100)]
        every: u32,
        /// Moneda + START automáticos (frame de la moneda; 0 = no).
        #[arg(long, default_value_t = 400)]
        coin: u32,
        /// Ejecutar sólo la ROM (sin funciones portadas), como referencia.
        #[arg(long)]
        rom_only: bool,
    },
    /// Modo diferencial: ROM vs Rust en cada llamada a una función portada.
    Verify {
        #[arg(long, default_value_t = 3000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
    },
    /// Lista las funciones portadas.
    List,
    /// Vuelca la P-ROM (vista CPU) y la BIOS cargada a `out/` (entrada del
    /// recompilador).
    DumpRoms {
        #[arg(long, default_value = "build/recomp")]
        out: PathBuf,
    },
    /// Ejecuta el intérprete y guarda el mapa de instrucciones ejecutadas
    /// (bitset, 1 bit por dirección par de 24 bits) para el recompilador.
    Trace {
        #[arg(long, default_value_t = 6000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
        #[arg(long)]
        out: PathBuf,
    },
}

pub fn load(rom: &PathBuf) -> Result<System> {
    let mut sys = System::new(SystemConfig {
        hardware: neogeo_hw::Hardware::Mvs,
        trace_cpu: false,
        trace_audio_io: false,
        audio_sample_rate: None,
    });
    let mut rs = RomSet::default();
    rs.load_cart_any(rom).with_context(|| format!("cargando {}", rom.display()))?;
    sys.load(rs)?;
    sys.reset();
    Ok(sys)
}

/// Entradas automáticas idénticas a las de pydmg (`--auto-coin-frame N
/// --auto-press-start-frame N+100 --press-period-frames 300`).
pub fn auto_inputs(sys: &mut System, coin: u32, f: u32) {
    // Demo de juego (desde coin+1300): derecha + disparo alterno + salto
    // cada 48 frames + granada cada 90 (como tools/dev/emu_input.py).
    if coin != 0 && f >= coin + 1300 {
        let k = (f - coin - 1300) / 6;
        let mut m: u8 = 8;
        if k % 2 == 1 { m |= 16; }
        if k % 8 == 0 { m |= 32; }
        if (f - coin) % 90 < 6 { m |= 64; }
        sys.bus.p1_input = !m;
    }
    if coin == 0 {
        return;
    }
    if f == coin {
        sys.bus.coin_inputs &= !0x01;
    } else if f == coin + 8 {
        sys.bus.coin_inputs |= 0x01;
    }
    let start = coin + 100;
    if f >= start {
        if (f - start) % 300 < 4 {
            sys.bus.start_select &= !0x01;
        } else {
            sys.bus.start_select |= 0x01;
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::List => {
            for (addr, name, _) in game::FUNCTIONS {
                println!("${addr:06X}  {name}");
            }
            println!("{} funciones portadas", game::FUNCTIONS.len());
        }
        Cmd::Run { frames, dump, every, coin, rom_only } => {
            let mut sys = load(&cli.rom)?;
            if !rom_only {
                let mut t = NativeTable::new();
                game::register(&mut t);
                sys.m68k.native = Some(Arc::new(t));
            }
            if let Some(d) = &dump {
                std::fs::create_dir_all(d)?;
            }
            for f in 0..frames {
                auto_inputs(&mut sys, coin, f);
                sys.run_frame();
                if let Some(d) = &dump {
                    if f % every.max(1) == 0 {
                        let fr = sys.render_frame_pixels();
                        let png = neogeo_hw::video::frame_to_png(
                            &fr, neogeo_hw::video::SCREEN_W, neogeo_hw::video::SCREEN_H);
                        std::fs::write(d.join(format!("frame-{f:05}.png")), png)?;
                    }
                }
            }
            println!("frames={frames} instrucciones={} llamadas_nativas={}",
                     sys.instructions, sys.m68k.native_calls);
        }
        Cmd::Verify { frames, coin } => verify::run(&cli.rom, frames, coin)?,
        Cmd::DumpRoms { out } => {
            let sys = load(&cli.rom)?;
            std::fs::create_dir_all(&out)?;
            std::fs::write(out.join("prom.bin"), &sys.bus.p_rom)?;
            std::fs::write(out.join("bios.bin"), &sys.bus.system_rom)?;
            println!("prom {} B, bios {} B -> {}", sys.bus.p_rom.len(), sys.bus.system_rom.len(), out.display());
        }
        Cmd::Trace { frames, coin, out } => {
            let mut sys = load(&cli.rom)?;
            let mut bits = vec![0u8; 1 << 20]; // 16 MiB / 2 / 8
            if let Ok(old) = std::fs::read(&out) {
                if old.len() == bits.len() { bits = old; }
            }
            for f in 0..frames {
                auto_inputs(&mut sys, coin, f);
                let mut budget: u64 = 202_752;
                while budget > 0 {
                    let pc = sys.m68k.pc & 0xFF_FFFF;
                    let irq = sys.m68k.pending_irq;
                    let used = sys.step();
                    // sólo cuenta si se ejecutó la instrucción en `pc`
                    if irq == 0 || sys.m68k.instr_pc == pc {
                        if !sys.m68k.stopped || sys.m68k.instr_pc == pc {
                            let i = (pc >> 1) as usize;
                            bits[i >> 3] |= 1 << (i & 7);
                        }
                    }
                    budget = budget.saturating_sub(u64::from(used));
                }
            }
            let n: u32 = bits.iter().map(|b| b.count_ones()).sum();
            std::fs::write(&out, &bits)?;
            println!("{n} instrucciones distintas -> {}", out.display());
        }
    }
    Ok(())
}
