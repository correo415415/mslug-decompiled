//! Prisionero (POW) suelto por el escenario: sentado o atado hasta que le
//! dan, se suelta y se va andando (los estados de después siguen en el
//! original).
//!
//! Original: `Pow_AnimTblB_03f636__L03f9b8` (preparación común), el
//! sentado (`$03FC38`, que el desensamblado dejó como datos dentro de
//! `Pow_AnimTblC_03fa0e`), `Pow_Freed_03fc6a`, `Pow_FreedCheer_03fca4`,
//! `Pow_FreedWait_03fcc0`, `Pow_Physics_03fece` y `Pow_OffworldFree_03efce`
//! (`asm/pow/results_squadleader_03daa8.s`).

use super::{anim, fort, items, platforms, soldier, tasks};
use crate::layout::{pow as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Preparación común: mira a la izquierda si lo pide `+$9A`, sus gráficos,
/// prioridad, quieto, apoyado y su sombra (tarea hija).
/// Original: `Pow_AnimTblB_03f636__L03f9b8`.
pub fn setup(m: &mut impl Machine, t: u32) {
    if m.read_u8(t + L::SPAWN_LEFT) != 0 {
        let f = m.read_u8(t + L::FACING);
        m.write_u8(t + L::FACING, f | L::FACING_LEFT);
    }
    anim::alloc_entry(m, t, L::GFX);
    anim::set_attr(m, t, L::ATTR_BASE);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::PRIORITY);
    for f in L::MOTION {
        m.write_u16(t + f, 0);
    }
    // `$027F60` vuelve siempre con C = 0: el `scc` deja $FF.
    soldier::on_ground(m, t);
    m.write_u8(t + L::GROUNDED, 0xFF);
    tasks::spawn(m, L::SHADOW, t);
}

/// Física del prisionero: apoyado anda por el suelo; si no, cae.
/// Original: `Pow_Physics_03fece`.
fn physics(m: &mut impl Machine, t: u32) {
    soldier::physics_flag(m, t, L::GROUNDED);
}

/// Se va si sale de la vista (con sus márgenes).
/// Original: `Pow_OffworldFree_03efce`.
fn offworld(m: &mut impl Machine, t: u32) -> Flow {
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Sentado: preparación, su animación y el bucle.
/// Original: `$03FC38` (`Pow_AnimTblC_03fa0e+$22A`).
pub fn seated(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    setup(m, t);
    anim::start(m, t, L::S_SEATED);
    set_handler(m, t, L::SEATED_LOOP);
    seated_loop(m)
}

/// Bucle sentado: física, animación y, si le dan, liberado.
/// Original: `$03FC4C` (`Pow_AnimTblC_03fa0e+$23E`).
pub fn seated_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    anim::step(m, t);
    if fort::hit_side(m, t).is_some() {
        set_handler(m, t, L::FREED);
    }
    offworld(m, t)
}

/// Liberado: animación de soltarse y el bucle.
/// Original: `Pow_Freed_03fc6a`.
pub fn freed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_FREED);
    set_handler(m, t, L::FREED_LOOP);
    freed_loop(m)
}

/// Al acabar de soltarse se va andando y deja una tarea que vitorea en su
/// sitio. Original: `Pow_Freed_03fc6a+$10`.
pub fn freed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::WALK_OFF);
        let (c, _) = tasks::spawn_last(m, L::CHEER, t);
        anim::copy_sprite_pos(m, t, c);
    }
    offworld(m, t)
}

/// Vitoreo: sus gráficos y la animación; empieza a moverse el frame
/// siguiente. Original: `Pow_FreedCheer_03fca4`.
pub fn cheer(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    anim::start(m, t, L::S_CHEER);
    set_handler(m, t, L::CHEER_LOOP);
    Flow::Return
}

/// Se mueve con la escena y termina al acabar la animación.
/// Original: `Pow_FreedWait_03fcc0`.
pub fn cheer_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
