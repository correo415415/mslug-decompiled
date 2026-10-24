//! Escombros de la escena 3: trozos que salen al disparar o al golpear
//! (los lanzan los guiones de animación) y que se quedan con el
//! decorado hasta acabar su animación.
//!
//! Original: `Scene3Debris_Spawn_03d616`, `Scene3Debris_SpawnLow8_03d640`,
//! `Scene3Debris_SpawnUpD_03d66e`, `Scene3Debris_SpawnUpC_03d6b4`,
//! `Scene3Debris_SpawnUpB_03d6e4`, `Scene3Debris_SpawnPrio4_03d72a` y
//! `Scene3Debris_Task_03d76e` (`asm/player/fire_shells_03c62a.s`).

use super::{anim, platforms, tasks};
use crate::layout::{debris as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// Cómo se coloca el trozo respecto a quien lo lanza: desplazamiento en
/// Y, en X hacia donde mira (positivo = adelante) y capa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offset {
    pub dy: i16,
    pub dx_forward: i16,
    pub layer: u16,
}

/// Las seis variantes del original, en orden.
pub const VARIANTS: [Offset; 6] = [
    Offset { dy: 0, dx_forward: 0, layer: L::LAYER_FRONT },
    Offset { dy: 8, dx_forward: 0, layer: L::LAYER_FRONT },
    Offset { dy: -0xD, dx_forward: 3, layer: L::LAYER_FRONT },
    Offset { dy: -0xC, dx_forward: 0, layer: L::LAYER_FRONT },
    Offset { dy: -0xB, dx_forward: -4, layer: L::LAYER_FRONT },
    Offset { dy: -6, dx_forward: -3, layer: L::LAYER_BACK },
];

/// En la escena 3, lanza un trozo desde la tarea `t` con la colocación
/// `o`. Devuelve la tarea nueva.
pub fn spawn(m: &mut impl Memory, t: u32, o: Offset) -> Option<u32> {
    if m.read_u8(L::SCENE) != L::ONLY_SCENE {
        return None;
    }
    let d = tasks::spawn(m, L::TASK, t);
    anim::copy_sprite_pos(m, t, d);
    let a = m.read_u16(d + L::ATTR);
    m.write_u16(d + L::ATTR, (a & L::ATTR_KEEP) | o.layer);
    let y = m.read_u16(d + L::Y).wrapping_add(o.dy as u16);
    m.write_u16(d + L::Y, y);
    if o.dx_forward != 0 {
        let left = m.read_u8(t + L::FACING) & L::FACING_LEFT != 0;
        let dx = if left { -o.dx_forward } else { o.dx_forward };
        let x = m.read_u16(d + L::X).wrapping_add(dx as u16);
        m.write_u16(d + L::X, x);
    }
    Some(d)
}

/// Tarea del trozo: sprite y guion, y sigue en el bucle.
/// Original: `Scene3Debris_Task_03d76e`.
pub fn task(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    anim::start(m, t, L::SCRIPT);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Bucle: se mueve con el decorado y se da de baja al acabar el guion.
/// Original: `Scene3Debris_Task_03d76e__L03d788`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
