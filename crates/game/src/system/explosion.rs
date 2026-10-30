//! Explosiones y efectos de una sola animación de las escenas: cada
//! variante elige su guion, sus márgenes de fuera de la pantalla y su
//! gráfico; casi todas salen giradas al azar, a tamaño completo, siguen a la
//! capa de la cámara y desaparecen al acabar la animación o al salir.
//!
//! Original: `Explosion_Small_077e10` … `Explosion_FireFront_0780ae` y el
//! cuerpo común `$0780DE` (`asm/props/fx_explosions_platforms_crew_076000.s`),
//! `$077E00` y `$044048`.

use super::{anim, items, motion, rng, spawner, tasks};
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

/// Trozo en arco `k` (uno de `ARCS`): sale hacia arriba con velocidad al
/// azar, girado al azar, sin caja. Original: `Explosion_VarTbl_078218` …
/// `Debris_ArcB_07875e` y su cuerpo `$078778`.
pub fn arc(m: &mut impl Machine, k: usize, at: (u16, u16)) -> Flow {
    let t = m.task_register();
    let (script, margins, gfx) = L::ARCS[k];
    anim::start(m, t, script);
    m.write_u32(t + L::MARGINS, margins);
    anim::alloc_entry(m, t, gfx);
    m.write_u8(t + L::SCALE[0], 0xFF);
    m.write_u8(t + L::SCALE[1], 0xFF);
    random_flip(m, t);
    m.write_u16(t + L::VEL_Y, L::ARC_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::ARC_GRAVITY);
    m.write_u16(t + L::ACC_X, 0);
    let (r, _) = rng::below(m, L::ARC_RAND_X);
    m.write_u16(t + L::VEL_X, if r & 1 != 0 { r.wrapping_neg() } else { r });
    let (r, _) = rng::below(m, L::ARC_RAND_Y);
    let vy = m.read_u16(t + L::VEL_Y).wrapping_sub(r);
    m.write_u16(t + L::VEL_Y, vy);
    let f = m.read_u8(t + L::FLIP);
    m.write_u8(t + L::FLIP, f | (r as u8 & 1));
    if out(m, t) {
        return tasks::finish(m);
    }
    m.write_u32(t + L::HITBOX, T::NIL);
    m.write_u32(t + T::HANDLER, L::ARC_RUN);
    arc_run(m, at)
}

/// Vuelo del trozo: cae (con tope), anima si hay sitio en la cola de
/// vídeo y desaparece al acabar o al salir. Original: `$0787F8`.
pub fn arc_run(m: &mut impl Machine, at: (u16, u16)) -> Flow {
    let t = m.task_register();
    if (m.read_u16(t + L::VEL_Y) as i16) <= L::ARC_MIN_VEL_Y {
        m.write_u16(t + L::VEL_Y, (L::ARC_MIN_VEL_Y + 1) as u16);
    }
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    let done = !spawner::video_busy(m) && anim::step(m, t);
    if done || out(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Explosión pequeña: gráfico 4, girada al azar, a tamaño completo; sigue
/// a la capa de la cámara y desaparece al acabar o al salir.
/// Original: `Explosion_Small_077e10`.
pub fn small(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::SMALL_GFX);
    m.write_u8(t + L::SCALE[0], 0xFF);
    m.write_u8(t + L::SCALE[1], 0xFF);
    random_flip(m, t);
    anim::start(m, t, L::SMALL_SCRIPT);
    if items::out_of_view_seen(m, t, Some(L::SMALL_MARGINS)) {
        return tasks::finish(m);
    }
    m.write_u32(t + T::HANDLER, L::SMALL_RUN);
    small_run(m)
}

/// Bucle de la pequeña. Original: `Explosion_Small_077e10+$3C`.
pub fn small_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    follow_layer(m, t);
    if anim::step(m, t) || items::out_of_view_seen(m, t, Some(L::SMALL_MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Deja una explosión pequeña (al final de las hijas de `t`) en el punto
/// de destino de `t`. Devuelve la tarea creada y la que queda detrás.
/// Original: `Spawner_SpawnAtGoal_077d88`.
pub fn spawn_at_goal(m: &mut impl Memory, t: u32) -> (u32, u32) {
    let (e, after) = tasks::spawn_last(m, L::SMALL, t);
    anim::copy_sprite_pos(m, t, e);
    let (x, y) = (m.read_u16(t + L::GOAL[0]), m.read_u16(t + L::GOAL[1]));
    m.write_u16(e + L::X, x);
    m.write_u16(e + L::Y, y);
    (e, after)
}
