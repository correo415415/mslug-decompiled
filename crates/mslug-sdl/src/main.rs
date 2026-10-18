//! `mslug-sdl` — Metal Slug (port 1:1 a Rust) con ventana SDL2.
//!
//! Ejecuta el programa del juego recompilado (`mslug-code`) sobre el
//! hardware Neo Geo de `neogeo-hw` y lo muestra en una ventana con sonido.
//! Sirve para comprobar a ojo (y a oído) que el port se ve y suena igual
//! que el original. Con `--interp` usa el intérprete 68000 como referencia.
//!
//! Teclas (P1): cursores, Z/X/C/V = A/B/C/D (disparo, salto, granada, —),
//! Enter/1 = START, 5 = moneda, F2 = servicio.
//! P2: I/K/J/L, Q/W/E/R = A..D, 2 = START, 6 = moneda.
//! F11 = pantalla completa · F12 = captura PNG · P = pausa · Tab = turbo ·
//! Esc = salir.

use anyhow::{Context, Result};
use clap::Parser;
use mslug_code::Program;
use neogeo_hw::video::{SCREEN_H, SCREEN_W};
use neogeo_hw::{RomSet, System, SystemConfig};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::video::FullscreenType;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(name = "mslug-sdl", about = "Metal Slug — port 1:1 a Rust (ventana SDL2)")]
struct Cli {
    /// Romset MAME (mslug.zip con las BIOS).
    #[arg(long, default_value = "internal/mslug.zip")]
    rom: PathBuf,
    /// Escala de la ventana.
    #[arg(long, default_value_t = 3)]
    scale: u32,
    /// Usar el intérprete 68000 en lugar del programa recompilado.
    #[arg(long)]
    interp: bool,
    /// Sin sonido.
    #[arg(long)]
    mute: bool,
    /// Mostrar los 320 px completos (por defecto 304, como MAME).
    #[arg(long)]
    no_crop: bool,
    /// Salir tras N frames (0 = sin límite).
    #[arg(long, default_value_t = 0)]
    frames: u32,
    /// Directorio de las capturas (F12).
    #[arg(long, default_value = "build/shots")]
    shots: PathBuf,
}

/// Cola de audio compartida con el callback de SDL.
struct Ring(Arc<Mutex<VecDeque<i16>>>);

impl AudioCallback for Ring {
    type Channel = i16;
    fn callback(&mut self, out: &mut [i16]) {
        let mut q = self.0.lock().unwrap();
        for s in out.iter_mut() {
            *s = q.pop_front().unwrap_or(0);
        }
    }
}

fn load(rom: &PathBuf, audio: bool) -> Result<System> {
    let mut sys = System::new(SystemConfig {
        hardware: neogeo_hw::Hardware::Mvs,
        trace_cpu: false,
        trace_audio_io: false,
        audio_sample_rate: if audio { Some(55_555) } else { None },
    });
    let mut rs = RomSet::default();
    rs.load_cart_any(rom).with_context(|| format!("cargando {}", rom.display()))?;
    sys.load(rs)?;
    sys.reset();
    Ok(sys)
}

/// Mando activo-bajo: pulsado = bit a 0.
fn set(reg: &mut u8, mask: u8, down: bool) {
    if down {
        *reg &= !mask;
    } else {
        *reg |= mask;
    }
}

fn key(sys: &mut System, k: Keycode, down: bool) {
    let b = &mut sys.bus;
    match k {
        Keycode::Up => set(&mut b.p1_input, 0x01, down),
        Keycode::Down => set(&mut b.p1_input, 0x02, down),
        Keycode::Left => set(&mut b.p1_input, 0x04, down),
        Keycode::Right => set(&mut b.p1_input, 0x08, down),
        Keycode::Z => set(&mut b.p1_input, 0x10, down),
        Keycode::X => set(&mut b.p1_input, 0x20, down),
        Keycode::C => set(&mut b.p1_input, 0x40, down),
        Keycode::V => set(&mut b.p1_input, 0x80, down),
        Keycode::I => set(&mut b.p2_input, 0x01, down),
        Keycode::K => set(&mut b.p2_input, 0x02, down),
        Keycode::J => set(&mut b.p2_input, 0x04, down),
        Keycode::L => set(&mut b.p2_input, 0x08, down),
        Keycode::Q => set(&mut b.p2_input, 0x10, down),
        Keycode::W => set(&mut b.p2_input, 0x20, down),
        Keycode::E => set(&mut b.p2_input, 0x40, down),
        Keycode::R => set(&mut b.p2_input, 0x80, down),
        Keycode::Return | Keycode::Num1 => set(&mut b.start_select, 0x01, down),
        Keycode::Num2 => set(&mut b.start_select, 0x04, down),
        Keycode::Num5 => set(&mut b.coin_inputs, 0x01, down),
        Keycode::Num6 => set(&mut b.coin_inputs, 0x02, down),
        Keycode::F2 => set(&mut b.coin_inputs, 0x04, down),
        _ => {}
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let cli = Cli::parse();
    let mut sys = load(&cli.rom, !cli.mute)?;
    let mut prog = Program::new();

    let sdl = sdl2::init().map_err(anyhow::Error::msg)?;
    let video = sdl.video().map_err(anyhow::Error::msg)?;
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "nearest");

    let ring = Arc::new(Mutex::new(VecDeque::<i16>::with_capacity(1 << 16)));
    let _audio = if cli.mute {
        None
    } else {
        let audio = sdl.audio().map_err(anyhow::Error::msg)?;
        let want = AudioSpecDesired { freq: Some(55_555), channels: Some(2), samples: Some(1024) };
        let dev = audio
            .open_playback(None, &want, |_| Ring(ring.clone()))
            .map_err(anyhow::Error::msg)?;
        dev.resume();
        Some(dev)
    };

    let crop: u32 = if cli.no_crop { 0 } else { 8 };
    let (vw, vh) = (SCREEN_W as u32 - 2 * crop, SCREEN_H as u32);
    let window = video
        .window("Metal Slug — port Rust", vw * cli.scale, vh * cli.scale)
        .position_centered()
        .resizable()
        .build()
        .map_err(anyhow::Error::msg)?;
    let mut canvas = window.into_canvas().build().map_err(anyhow::Error::msg)?;
    canvas.set_logical_size(vw, vh).map_err(anyhow::Error::msg)?;
    let tc = canvas.texture_creator();
    // El renderer produce 0xRRGGBBAA; ABGR8888 en little-endian = R,G,B,A en memoria.
    let mut tex = tc
        .create_texture_streaming(PixelFormatEnum::ABGR8888, SCREEN_W as u32, SCREEN_H as u32)
        .map_err(anyhow::Error::msg)?;
    let src = Rect::new(crop as i32, 0, vw, vh);

    let mut events = sdl.event_pump().map_err(anyhow::Error::msg)?;
    let frame_time = Duration::from_secs_f64(1.0 / 59.185_606);
    let mut deadline = Instant::now() + frame_time;
    let (mut paused, mut turbo, mut fullscreen) = (false, false, false);
    let mut frame: u32 = 0;
    let mut fps_t = Instant::now();
    let mut fps_n = 0u32;

    'main: loop {
        for ev in events.poll_iter() {
            match ev {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(k), repeat: false, .. } => match k {
                    Keycode::Escape => break 'main,
                    Keycode::P => paused = !paused,
                    Keycode::Tab => turbo = true,
                    Keycode::F11 => {
                        fullscreen = !fullscreen;
                        let t = if fullscreen { FullscreenType::Desktop } else { FullscreenType::Off };
                        let _ = canvas.window_mut().set_fullscreen(t);
                    }
                    Keycode::F12 => {
                        std::fs::create_dir_all(&cli.shots)?;
                        let fr = sys.render_frame_pixels();
                        let png = neogeo_hw::video::frame_to_png(&fr, SCREEN_W, SCREEN_H);
                        let p = cli.shots.join(format!("shot-{frame:06}.png"));
                        std::fs::write(&p, png)?;
                        eprintln!("captura: {}", p.display());
                    }
                    _ => key(&mut sys, k, true),
                },
                Event::KeyUp { keycode: Some(k), repeat: false, .. } => {
                    if k == Keycode::Tab {
                        turbo = false;
                    }
                    key(&mut sys, k, false);
                }
                _ => {}
            }
        }

        if !paused {
            if cli.interp {
                sys.run_frame();
            } else {
                prog.run_frame(&mut sys);
            }
            frame += 1;
            fps_n += 1;
            if !sys.audio_buffer.is_empty() {
                let mut q = ring.lock().unwrap();
                // Si la cola crece (turbo / ventana parada) se descarta lo viejo.
                if q.len() > 1 << 15 {
                    q.clear();
                }
                q.extend(sys.audio_buffer.drain(..));
            }
        }

        let fr = sys.render_frame_pixels();
        tex.with_lock(None, |buf: &mut [u8], pitch: usize| {
            for y in 0..SCREEN_H {
                for x in 0..SCREEN_W {
                    let px = fr[y * SCREEN_W + x];
                    let o = y * pitch + x * 4;
                    buf[o..o + 4].copy_from_slice(&px.to_be_bytes());
                }
            }
        })
        .map_err(anyhow::Error::msg)?;
        canvas.clear();
        canvas.copy(&tex, Some(src), None).map_err(anyhow::Error::msg)?;
        canvas.present();

        if fps_t.elapsed() >= Duration::from_secs(1) {
            let fps = fps_n as f64 / fps_t.elapsed().as_secs_f64();
            let mode = if cli.interp { "intérprete" } else { "recompilado" };
            let miss = prog.misses.len();
            let _ = canvas.window_mut().set_title(&format!(
                "Metal Slug — port Rust ({mode}) · {fps:.1} fps · frame {frame} · sin traducir {miss}"
            ));
            fps_n = 0;
            fps_t = Instant::now();
        }
        if cli.frames > 0 && frame >= cli.frames {
            break;
        }
        if !turbo {
            let now = Instant::now();
            if now < deadline {
                std::thread::sleep(deadline - now);
                deadline += frame_time;
            } else {
                deadline = now + frame_time;
            }
        }
    }
    if !prog.misses.is_empty() {
        eprintln!("direcciones ejecutadas sin traducir: {}", prog.misses.len());
        for (a, c) in prog.misses.iter().take(30) {
            eprintln!("  ${a:06X} x{c}");
        }
    }
    Ok(())
}
