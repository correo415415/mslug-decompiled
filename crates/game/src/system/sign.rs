//! Letrero de la escena: anima su guion siguiendo al decorado y, al
//! acabar, deja su tarea hija (con los datos de colocación de `+$98`…`+$9F`)
//! y desaparece. Si sale de la vista antes, se da de baja.
//!
//! Original: `Sign_Tmpl28_060420` (cinco variantes cada `$20` bytes),
//! `Sign_Idle_06052a` y `Offworld_Box2C2314_06053e`
//! (`asm/props/late_props_turrets_05e000.s`).

use super::{anim, items, manual_icons, platforms, tasks};
use crate::layout::{sign as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Alta de la variante `k` (uno de `L::KINDS`: guion y rutina de la hija).
/// Original: `Sign_Tmpl28_060420` + `k`·`$20`.
pub fn start(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (script, child) = L::KINDS[k];
    let f = m.read_u8(t + L::FACING);
    m.write_u8(t + L::FACING, f | 1);
    anim::start(m, t, script);
    m.write_u32(t + L::CHILD, child);
    anim::alloc_entry(m, t, L::GFX);
    m.write_u8(t + L::SCALE[0], 0xFF);
    m.write_u8(t + L::SCALE[1], 0xFF);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    manual_icons::stop_motion(m, t);
    set_handler(m, t, L::RUN);
    run(m)
}

/// ¿Fuera de la vista (con los márgenes del letrero)?
/// Original: `Offworld_Box2C2314_06053e` (C = sí).
pub fn off_view(m: &mut impl Memory, t: u32) -> bool {
    items::out_of_view(m, t, Some(L::MARGINS))
}

/// Bucle: al acabar la animación crea la hija en su sitio y pasa a
/// [`idle`]. Original: `Sign_Tmpl28_060420+$C4`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        let child = m.read_u32(t + L::CHILD);
        let c = tasks::spawn_marked(m, child);
        anim::copy_sprite_pos(m, t, c);
        for f in L::PLACEMENT {
            let v = m.read_u32(t + f);
            m.write_u32(c + f, v);
        }
        let mut r = m.regs();
        r.a[0] = c;
        m.set_regs(r);
        set_handler(m, t, L::IDLE);
        return Flow::Return;
    }
    if off_view(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Un frame más de animación y baja. Original: `Sign_Idle_06052a`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    tasks::finish(m)
}
