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
mod lockstep;
mod manual;
mod roadmap;
mod progress;
mod verify_system;

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
    /// Port escrito a mano (`mslug-game`): compara cada función Rust legible
    /// con su rutina original durante una partida (RAM y valor devuelto).
    /// Rutinas de sistema portadas a mano (arranque, interrupciones,
    /// planificador, bucle de frame...) contra la ROM, con el estado
    /// completo de la máquina (RAM, paleta, VRAM, sonido).
    VerifySystem {
        #[arg(long, default_value_t = 3000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
        /// Entradas pseudoaleatorias.
        #[arg(long)]
        fuzz: Option<u64>,
        /// Moneda y START sin tocar nada más (recorre el tutorial entero).
        #[arg(long)]
        hands_off: bool,
    },
    VerifyManual {
        #[arg(long, default_value_t = 6000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
        /// Entradas pseudoaleatorias (llega a más partes del juego).
        #[arg(long)]
        fuzz: Option<u64>,
    },
    /// Intérprete vs programa recompilado, comparando tras cada instrucción.
    Lockstep {
        #[arg(long, default_value_t = 3000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
    },
    /// Partida recompilada frente a la misma con el port manual sustituido
    /// (`--manual`): compara RAM y paleta al final de cada frame.
    LockstepManual {
        #[arg(long, default_value_t = 8000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
        #[arg(long)]
        fuzz: Option<u64>,
        /// Referencia con cada rutina original ejecutada de una vez (misma
        /// granularidad que la sustitución): aísla las diferencias de
        /// comportamiento de las de tiempo (usar con
        /// MSLUG_MANUAL_EXACT_CYCLES=1).
        #[arg(long)]
        atomic_ref: bool,
    },
    /// Hoja de ruta del port manual: rutinas que ejecuta el juego en orden
    /// de primera ejecución, cuáles están portadas y cuánto cubren.
    Roadmap {
        #[arg(long, default_value_t = 8000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
        #[arg(long)]
        fuzz: Option<u64>,
        #[arg(long, default_value_t = 25)]
        top: usize,
        /// Lista completa a este fichero.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Progreso del port escrito a mano: funciones totales, portadas,
    /// porcentaje y hasta dónde llega el modo sólo manual.
    Progress {
        #[arg(long, default_value_t = 20000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
    },
    /// Juega con el programa recompilado (sin intérprete salvo misses).
    Play {
        #[arg(long, default_value_t = 3000)]
        frames: u32,
        #[arg(long, default_value_t = 400)]
        coin: u32,
        #[arg(long)]
        dump: Option<PathBuf>,
        #[arg(long, default_value_t = 100)]
        every: u32,
        /// Añadir las direcciones sin traducir a este fichero (seeds.txt).
        #[arg(long)]
        misses_out: Option<PathBuf>,
        /// Entradas pseudoaleatorias con esta semilla (en lugar del patrón
        /// fijo), con moneda + START periódicos para continuar: recorre más
        /// partes del juego para encontrar código sin traducir.
        #[arg(long)]
        fuzz: Option<u64>,
        /// Sustituir las rutinas portadas a mano por su función Rust.
        #[arg(long)]
        manual: bool,
        /// Sólo el port escrito a mano (sin el juego recompilado): se para
        /// en la primera instrucción sin función Rust y dice cuál es.
        #[arg(long)]
        only_manual: bool,
        /// Con --only-manual: no pararse en el código sin portar; se ejecuta
        /// el juego recompilado en su lugar y al final se lista cada rutina
        /// sin portar que se usó (primer frame y veces).
        #[arg(long)]
        keep_going: bool,
        /// Con --keep-going: cuántas rutinas listar al final.
        #[arg(long, default_value_t = 60)]
        keep_top: usize,
        /// Con --only-manual: tampoco la BIOS recompilada.
        #[arg(long)]
        no_bios_code: bool,
        /// Si el port se para, guarda una captura PNG del último frame en
        /// este fichero (sin tener que volcar todos los frames con --dump).
        #[arg(long)]
        halt_shot: Option<PathBuf>,
        /// Moneda y START, pero sin palanca ni botones: deja correr el
        /// tutorial entero antes de la primera misión.
        #[arg(long)]
        hands_off: bool,
    },
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
/// Con `play --hands-off`: moneda y START, pero sin tocar la palanca ni los
/// botones (deja correr entero el tutorial «How to play»).
pub static HANDS_OFF: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn auto_inputs(sys: &mut System, coin: u32, f: u32) {
    // Demo de juego (desde coin+1300): derecha + disparo alterno + salto
    // cada 48 frames + granada cada 90 (como tools/dev/emu_input.py).
    let hands_off = HANDS_OFF.load(std::sync::atomic::Ordering::Relaxed);
    if coin != 0 && f >= coin + 1300 && !hands_off {
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
        // Sin manos, START una sola vez (pulsarlo otra vez salta el tutorial).
        let period = if hands_off { u32::MAX } else { 300 };
        if (f - start) % period < 4 {
            sys.bus.start_select &= !0x01;
        } else {
            sys.bus.start_select |= 0x01;
        }
    }
}

/// Entradas pseudoaleatorias reproducibles (xorshift64*) para `play --fuzz`.
/// Cada 4..36 frames elige un nuevo estado de palanca y botones (sesgado
/// hacia la derecha y el disparo, para avanzar), y cada 600 frames mete una
/// moneda y pulsa START (continuar / empezar).
pub struct Fuzz {
    s: u64,
    hold: u32,
    p1: u8,
}

impl Fuzz {
    pub fn new(seed: u64) -> Self {
        Self { s: seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1, hold: 0, p1: 0 }
    }
    fn next(&mut self) -> u64 {
        self.s ^= self.s >> 12;
        self.s ^= self.s << 25;
        self.s ^= self.s >> 27;
        self.s.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    pub fn inputs(&mut self, sys: &mut System, f: u32) {
        let c = f % 600;
        if c == 0 {
            sys.bus.coin_inputs &= !0x01;
        } else if c == 8 {
            sys.bus.coin_inputs |= 0x01;
        }
        if (100..104).contains(&c) || (300..304).contains(&c) {
            sys.bus.start_select &= !0x01;
        } else {
            sys.bus.start_select |= 0x01;
        }
        if self.hold == 0 {
            let r = self.next();
            self.hold = 4 + (r % 33) as u32;
            let mut m: u8 = 0;
            let d = (r >> 8) % 10;
            m |= match d { 0..=4 => 0x08, 5 => 0x04, 6 => 0x01, 7 => 0x02, 8 => 0x09, _ => 0 };
            if (r >> 16) % 3 != 0 { m |= 0x10; } // A disparo
            if (r >> 20) % 4 == 0 { m |= 0x20; } // B salto
            if (r >> 24) % 8 == 0 { m |= 0x40; } // C granada
            if (r >> 28) % 32 == 0 { m |= 0x80; }
            self.p1 = m;
        }
        self.hold -= 1;
        // alternar el disparo para que cuente como pulsaciones
        let m = if f % 2 == 1 { self.p1 & !0x10 } else { self.p1 };
        sys.bus.p1_input = !m;
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
        Cmd::VerifyManual { frames, coin, fuzz } => manual::run(&cli.rom, frames, coin, fuzz)?,
        Cmd::VerifySystem { frames, coin, fuzz, hands_off } => {
            HANDS_OFF.store(hands_off, std::sync::atomic::Ordering::Relaxed);
            verify_system::run(&cli.rom, frames, coin, fuzz)?
        }
        Cmd::Roadmap { frames, coin, fuzz, top, out } => roadmap::run(&cli.rom, frames, coin, fuzz, top, out)?,
        Cmd::Progress { frames, coin } => progress::run(&cli.rom, frames, coin)?,
        Cmd::Lockstep { frames, coin } => lockstep::run(&cli.rom, frames, coin)?,
        Cmd::LockstepManual { frames, coin, fuzz, atomic_ref } => lockstep::run_manual(&cli.rom, frames, coin, fuzz, atomic_ref)?,
        Cmd::Play { frames, coin, dump, every, misses_out, fuzz, manual, only_manual, keep_going, keep_top, no_bios_code, halt_shot, hands_off } => {
            HANDS_OFF.store(hands_off, std::sync::atomic::Ordering::Relaxed);
            let mut sys = load(&cli.rom)?;
            let only_manual = only_manual || keep_going;
            let mut prog = if keep_going {
                mslug_code::Program::keep_going(!no_bios_code)
            } else if only_manual {
                mslug_code::Program::manual_only(!no_bios_code)
            } else {
                mslug_code::Program::new()
            };
            if manual || only_manual {
                let n = if only_manual {
                    mslug_bridge::system::set_mode(true, !no_bios_code);
                    mslug_bridge::system::set_keep_going(keep_going);
                    mslug_bridge::install_all(|a, f| prog.set_override(a, f))
                } else {
                    mslug_bridge::install(|a, f| prog.set_override(a, f))
                };
                println!("port manual: {n} rutinas sustituidas por Rust escrito a mano");
            }
            if let Some(d) = &dump {
                std::fs::create_dir_all(d)?;
            }
            let t0 = std::time::Instant::now();
            let mut fz = fuzz.map(Fuzz::new);
            for f in 0..frames {
                match fz.as_mut() {
                    Some(z) => z.inputs(&mut sys, f),
                    None => auto_inputs(&mut sys, coin, f),
                }
                prog.run_frame(&mut sys);
                if let Some(pc) = prog.halted_at {
                    println!("{}", mslug_bridge::halt_report(pc, f, &sys.m68k));
                    // La CPU se para a mitad de frame: la imagen presentada
                    // es la del VBLANK anterior. Con la CPU congelada, el
                    // vídeo avanza un frame más para que la captura incluya
                    // todo lo que el port escribió antes de pararse.
                    prog.run_frame(&mut sys);
                    if let Some(p) = &halt_shot {
                        if let Some(dir) = p.parent().filter(|d| !d.as_os_str().is_empty()) {
                            std::fs::create_dir_all(dir)?;
                        }
                        let fr = sys.render_frame_pixels();
                        let png = neogeo_hw::video::frame_to_png(
                            &fr, neogeo_hw::video::SCREEN_W, neogeo_hw::video::SCREEN_H);
                        std::fs::write(p, png)?;
                        println!("  captura de la parada: {}", p.display());
                    }
                    if let Some(d) = &dump {
                        let fr = sys.render_frame_pixels();
                        let png = neogeo_hw::video::frame_to_png(
                            &fr, neogeo_hw::video::SCREEN_W, neogeo_hw::video::SCREEN_H);
                        std::fs::write(d.join(format!("halt-{f:05}.png")), png)?;
                    }
                    break;
                }
                if let Some(d) = &dump {
                    if f % every.max(1) == 0 {
                        let fr = sys.render_frame_pixels();
                        let png = neogeo_hw::video::frame_to_png(
                            &fr, neogeo_hw::video::SCREEN_W, neogeo_hw::video::SCREEN_H);
                        std::fs::write(d.join(format!("frame-{f:05}.png")), png)?;
                    }
                }
            }
            println!("frames={frames} en {:.1}s; instrucciones nativas {} interpretadas {} (sin traducir: {} direcciones)",
                     t0.elapsed().as_secs_f64(), prog.native, prog.interp, prog.misses.len());
            for (a, c) in prog.misses.iter().take(40) {
                println!("  ${a:06X} x{c}");
            }
            if keep_going {
                keep_going_report(&prog, keep_top);
            }
            if let Some(p) = &misses_out {
                mslug_code::append_misses(&prog, p)?;
            }
        }
        Cmd::DumpRoms { out } => {
            let sys = load(&cli.rom)?;
            std::fs::create_dir_all(&out)?;
            std::fs::write(out.join("prom.bin"), &sys.bus.p_rom[..])?;
            std::fs::write(out.join("bios.bin"), &sys.bus.system_rom[..])?;
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

/// Informe de `play --keep-going`: rutinas sin portar que se ejecutaron con
/// el juego recompilado, agrupadas por rutina (las vueltas de un `jsr` a
/// código portado cuentan para la rutina que lo contiene) y por orden de
/// primera aparición.
fn keep_going_report(prog: &mslug_code::Program, top: usize) {
    use std::collections::BTreeMap;
    // rutina -> (primer frame, entradas, instrucciones)
    let mut by: BTreeMap<u32, (u32, u64, u64)> = BTreeMap::new();
    for (&pc, &(f, n, ins)) in &prog.fallback_hits {
        let r = mslug_bridge::symbols::containing(pc).map_or(pc, |(a, _)| a);
        let e = by.entry(r).or_insert((f, 0, 0));
        e.0 = e.0.min(f);
        e.1 += n;
        e.2 += ins;
    }
    let mut v: Vec<(u32, (u32, u64, u64))> = by.into_iter().collect();
    v.sort_by_key(|&(pc, (f, _, _))| (f, pc));
    let total = prog.native + prog.fallback_ins;
    let pct = if total == 0 { 0.0 } else { 100.0 * prog.native as f64 / total as f64 };
    println!(
        "--keep-going: {} rutinas sin portar usadas; instrucciones por el port {:.2} % ({} de {})",
        v.len(), pct, prog.native, total
    );
    println!("  primer frame   entradas  instrucciones  PC       rutina");
    for (pc, (f, n, ins)) in v.iter().take(top) {
        println!("  {f:>12}  {n:>9}  {ins:>13}  ${pc:06X}  {}", mslug_bridge::symbols::describe(*pc));
    }
    if v.len() > top {
        println!("  ... y {} más (--keep-top N)", v.len() - top);
    }
    let nested = mslug_bridge::system::nested_fallback();
    if !nested.is_empty() {
        println!("  llamadas anidadas desde el port a código sin portar: {}", nested.len());
        for (pc, n) in nested.iter().take(top) {
            println!("                 {n:>9}  ${pc:06X}  {}", mslug_bridge::symbols::describe(*pc));
        }
    }
}
