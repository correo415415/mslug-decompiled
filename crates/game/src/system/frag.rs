//! Fragmentos de las explosiones: cada uno lanza una hija animada que se
//! queda pegada a él con el sprite del ciclo que toca.
//!
//! Original: `Frag_SpawnChildAnim_076dd4` y `Frag_Child_Run_076f42`
//! (`asm/props/fx_explosions_platforms_crew_076000.s`).

use super::{anim, platforms, tasks};
use crate::layout::{frag as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Lanza la hija animada del fragmento `t` (justo detrás de él) y avanza
/// el ciclo de sprites. Original: `Frag_SpawnChildAnim_076dd4`.
pub fn spawn_child(m: &mut impl Memory, t: u32) -> u32 {
    let (c, _) = tasks::spawn_last(m, L::CHILD_RUN, t);
    anim::copy_sprite_pos(m, t, c);
    let a = m.read_u16(c + L::ATTR).wrapping_sub(1);
    m.write_u16(c + L::ATTR, a);
    let mut k = m.read_u8(t + L::CYCLE).wrapping_add(1) & 3;
    if k >= L::CYCLE_LEN {
        k = 0;
    }
    m.write_u8(t + L::CYCLE, k);
    let s = m.read_u16(t + L::SPRITES + u32::from(k) * 2);
    m.write_u16(t + L::SPRITE, s);
    c
}

/// Sus tres sprites. Original: `Frag_Child_Run_076f42+$36`.
pub fn child_gfx(m: &mut impl Memory, t: u32) {
    for g in L::CHILD_GFX {
        anim::alloc_entry(m, t, g);
    }
    let s = m.read_u16(t + L::SPRITES);
    m.write_u16(t + L::SPRITE, s);
}

/// Original: `Frag_Child_Run_076f42`.
pub fn child(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    child_gfx(m, t);
    anim::start(m, t, L::CHILD_SCRIPT);
    m.write_u32(t + T::HANDLER, L::CHILD_LOOP);
    child_loop(m)
}

/// Sigue el sprite de su fragmento hasta acabar la animación.
/// Original: `Frag_Child_Run_076f42+$14`.
pub fn child_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let o = m.read_u32(t + T::OWNER);
    let s = m.read_u16(o + L::SPRITE);
    m.write_u16(t + L::SPRITE, s);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
