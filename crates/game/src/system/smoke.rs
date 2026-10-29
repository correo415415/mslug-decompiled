//! Humos y salpicaduras: una animación fija en el escenario (sigue a la
//! capa de la cámara) que se da de baja al acabar o al salir de la
//! pantalla. Hay varias variantes de guion; algunas van detrás, a escala
//! completa y giradas al azar, y dos sueltan además un corro de humos
//! grandes alrededor.
//!
//! Original: `Smoke_*` (`$078840`–`$078A66`,
//! `asm/props/fx_explosions_platforms_crew_076000.s`).

use super::{anim, explosion, items, tasks};
use crate::layout::{smoke as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Detrás y a escala completa.
fn behind(m: &mut impl Memory, t: u32) {
    for a in L::SCALE {
        m.write_u8(t + a, 0xFF);
    }
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f | L::BEHIND);
}

fn out(m: &mut impl Memory, t: u32) -> bool {
    let margins = m.read_u32(t + L::MARGINS);
    items::out_of_view_seen(m, t, Some(margins))
}

/// Sin caja de golpe; si ya está fuera de la pantalla se da de baja.
fn arm(m: &mut impl Machine, t: u32, h: u32) -> Flow {
    if out(m, t) {
        return tasks::finish(m);
    }
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    set_handler(m, t, h);
    step(m)
}

/// Alta de la variante `k` (ver `KINDS`). Original: `$078840`,
/// `Smoke_Std_0788c0`, `$0788E4`, `Smoke_Big_078908`, `$07892C`,
/// `$078950` y `$078974`.
pub fn start(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (gfx, script, margins, decorate) = L::KINDS[k];
    anim::alloc_entry(m, t, gfx);
    anim::start(m, t, script);
    m.write_u32(t + L::MARGINS, margins);
    if decorate {
        for a in L::SCALE {
            m.write_u8(t + a, 0xFF);
        }
        explosion::random_flip(m, t);
        let f = m.read_u8(t + L::FLAGS_12);
        m.write_u8(t + L::FLAGS_12, f | L::BEHIND);
    }
    arm(m, t, L::LOOP)
}

/// Humo grande con hermanas desplazadas `offsets` en X.
/// Original: `$078864` (cuatro) y `$078890` (dos).
pub fn ring(m: &mut impl Machine, offsets: &[u16]) -> Flow {
    let t = m.task_register();
    for &dx in offsets {
        let c = tasks::spawn(m, L::BIG_ENTRY, t);
        anim::copy_sprite_pos(m, t, c);
        let x = m.read_u16(c + L::X).wrapping_add(dx);
        m.write_u16(c + L::X, x);
    }
    start(m, L::BIG)
}

/// Columna de humo. Original: `Smoke_Column_0789f0`.
pub fn column(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (gfx, script, margins) = L::COLUMN;
    anim::start(m, t, script);
    m.write_u32(t + L::MARGINS, margins);
    anim::alloc_entry(m, t, gfx);
    behind(m, t);
    arm(m, t, L::COLUMN_LOOP)
}

/// Cada frame: con la capa, el guion y fuera al acabar o salir.
/// Original: `$0789CA` / `$078A42`.
pub fn step(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    explosion::follow_layer(m, t);
    if anim::step(m, t) || out(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
