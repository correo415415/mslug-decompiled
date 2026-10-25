//! Iconos que siguen a una entidad: el «POW» que va sobre el Slug, la
//! burbuja y el indicador «1UP»/«2UP» del soldado que aparece. Se colocan cada frame sobre su dueño, a una altura fija.
//!
//! Original: `PlayerIcon_Pow_031fca`, `PlayerIcon_Bubble_03207c`,
//! `$032112`/`$032142`/`$032180` y su
//! final `$0321BC` (`asm/player/tables_fx_030696.s`).

use super::{anim, items, slug, tasks};
use crate::layout::{icons as L, task as T};
use crate::mem::Machine;
use crate::Flow;

fn start(m: &mut impl Machine, gfx: u16, script: u32, height: u16, run: u32) -> u32 {
    let t = m.task_register();
    anim::alloc_entry(m, t, gfx);
    anim::start(m, t, script);
    m.write_u16(t + L::HEIGHT, height);
    m.write_u16(t + L::ATTR, L::HIDDEN);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f | L::FLAGS_12_SET);
    m.write_u32(t + T::HANDLER, run);
    t
}

/// Se pone sobre el dueño, a su altura.
fn place(m: &mut impl Machine, t: u32) {
    let (attr, x, y) = slug::owner_pos(m, t);
    m.write_u16(t + L::X, x);
    let h = m.read_u16(t + L::HEIGHT);
    m.write_u16(t + L::Y, y.wrapping_add(h));
    m.write_u16(t + L::ATTR, attr);
}

/// Alta del POW. Original: `PlayerIcon_Pow_031fca`.
pub fn pow_start(m: &mut impl Machine) -> Flow {
    let script = if m.read_u8(L::SCENE) == L::POW_ALT_SCENE { L::POW_ALT_SCRIPT } else { L::POW_SCRIPT };
    start(m, L::POW_GFX, script, L::POW_HEIGHT, L::POW_RUN);
    pow_run(m)
}

/// POW: sigue al dueño; se anima si el dueño lo muestra y está a la vista;
/// cuando el dueño acaba, se da de baja el frame siguiente.
/// Original: `$032010`.
pub fn pow_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t);
    let o = m.read_u32(t + T::OWNER);
    if m.read_u8(L::SCENE_LOADED) != 0
        && m.read_u8(o + L::OWNER_FLAGS) & L::OWNER_SHOW != 0
        && !items::out_of_view_seen(m, t, Some(L::POW_MARGINS))
    {
        anim::step(m, t);
    }
    let o = m.read_u32(t + T::OWNER);
    if m.read_u8(o + L::OWNER_FLAGS_13) & L::OWNER_DONE != 0 {
        m.write_u32(t + T::HANDLER, L::GONE);
    }
    Flow::Return
}

/// Alta de la burbuja. Original: `PlayerIcon_Bubble_03207c`.
pub fn bubble_start(m: &mut impl Machine) -> Flow {
    start(m, L::BUBBLE_GFX, L::BUBBLE_SCRIPT, L::BUBBLE_HEIGHT, L::BUBBLE_RUN);
    bubble_run(m)
}

/// Burbuja: sigue al dueño hasta que acaba su guion. Original: `$0320AC`.
pub fn bubble_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t);
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::GONE);
    }
    Flow::Return
}

/// Baja del icono. Original: `$0321BC`.
pub fn gone(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}

/// Indicador «1UP» / «2UP»: sólo si el dueño es la entidad de ese
/// jugador. Original: `$032112` (1P) y `$032142` (2P).
pub fn player_start(m: &mut impl Machine, p: usize) -> Flow {
    let t = m.task_register();
    if m.read_u32(t + T::OWNER) != L::PLAYER_OWNER[p] {
        return tasks::finish(m);
    }
    let (gfx, script, height) = L::PLAYER[p];
    start(m, gfx, script, height, L::PLAYER_RUN);
    player_run(m)
}

/// Sigue al soldado hasta acabar su guion o hasta que el soldado acaba.
/// Original: `$032180`.
pub fn player_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t);
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::GONE);
    }
    let o = m.read_u32(t + T::OWNER);
    if m.read_u8(o + L::OWNER_FLAGS_13) & L::OWNER_DONE != 0 {
        m.write_u32(t + T::HANDLER, L::GONE);
    }
    Flow::Return
}
