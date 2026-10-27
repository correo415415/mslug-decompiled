//! Decorados de la escena del fuerte: carteles que estampan su silueta en
//! el mapa de colisión.
//!
//! Original: `asm/props/fortress_04e580.s` (`Prop_SignA_04f70e`,
//! `Prop_SignB_04f76c`).

use super::{anim, items, platforms, scroll, tasks};
use crate::layout::{props as L, task as T};
use crate::mem::{Flow, Machine};

/// Cartel `k` (0 = A, 1 = B): sprite, colisión estampada y bucle.
/// Original: `Prop_SignA_04f70e` / `Prop_SignB_04f76c`.
pub fn sign(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::SIGN_GFX[k]);
    anim::start(m, t, L::S_SIGN[k]);
    m.write_u16(t + L::ATTR, 0);
    platforms::move_with_scene(m, t);
    scroll::blit_cells(m, t, L::SIGN_CELLS[k]);
    m.write_u32(t + T::HANDLER, L::SIGN_LOOP[k]);
    sign_loop(m)
}

/// Se mueve con la escena, anima y se da de baja al salir de la vista.
/// Original: `Prop_SignA_04f70e+$34` / `Prop_SignB_04f76c+$34`.
pub fn sign_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if items::out_of_view(m, t, Some(L::MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}
