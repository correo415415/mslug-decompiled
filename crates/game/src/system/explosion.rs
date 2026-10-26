//! Explosiones y efectos de una sola animación de las escenas: cada
//! variante elige su guion, sus márgenes de fuera de la pantalla y su
//! gráfico; casi todas salen giradas al azar, a tamaño completo, siguen a la
//! capa de la cámara y desaparecen al acabar la animación o al salir.
//!
//! Original: `Explosion_Small_077e10` … `Explosion_FireFront_0780ae` y el
//! cuerpo común `$0780DE` (`asm/props/fx_explosions_platforms_crew_076000.s`),
//! `$077E00` y `$044048`.

use super::{anim, items, rng, tasks};
use crate::layout::{explosion as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// Gira al azar en horizontal. Original: `$077E00`.
pub fn random_flip(m: &mut impl Memory, t: u32) {
    let (r, _) = rng::next(m);
    let f = m.read_u8(t + L::FLIP);
    m.write_u8(t + L::FLIP, f | (r as u8 & 1));
}

/// Sigue a la capa de la cámara. Original: `$044048`.
pub fn follow_layer(m: &mut impl Memory, t: u32) {
    let x = m.read_u16(t + L::X).wrapping_sub(m.read_u16(L::LAYER_POS[0]));
    m.write_u16(t + L::X, x);
    let y = m.read_u16(t + L::Y).wrapping_add(m.read_u16(L::LAYER_POS[1]));
    m.write_u16(t + L::Y, y);
}

/// Alta de la variante `k`. Original: las entradas `Explosion_*`.
pub fn start(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let kind = L::KINDS[k];
    anim::start(m, t, kind.0);
    m.write_u32(t + L::MARGINS, kind.1);
    anim::alloc_entry(m, t, kind.2);
    if k == L::FRONT {
        let a = m.read_u16(t + L::ATTR);
        m.write_u16(t + L::ATTR, (a & L::FRONT_KEEP) | L::FRONT_ATTR);
    }
    if kind.3 {
        random_flip(m, t);
    }
    m.write_u8(t + L::SCALE[0], 0xFF);
    m.write_u8(t + L::SCALE[1], 0xFF);
    if out(m, t) {
        return tasks::finish(m);
    }
    if kind.4 {
        m.write_u32(t + L::HITBOX, T::NIL);
    }
    m.write_u32(t + T::HANDLER, if kind.4 { L::RUN } else { L::RUN_BIG });
    run(m)
}

fn out(m: &mut impl Memory, t: u32) -> bool {
    let mg = m.read_u32(t + L::MARGINS);
    items::out_of_view_seen(m, t, Some(mg))
}

/// Bucle: sigue a la capa, anima y desaparece al acabar o al salir.
/// Original: `$07810A` (y `$077EB4`).
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    follow_layer(m, t);
    if anim::step(m, t) || out(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
