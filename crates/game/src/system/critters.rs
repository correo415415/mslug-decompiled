//! Bichos sencillos de las escenas: el que sube y baja (plantilla 182), el
//! que espera, salta y cae (183) y el que corre en línea recta (184).
//! Comparten la física de los soldados rasos (`system::grunt`).
//!
//! Original: `asm/enemies/critters_rings_08e4e6.s` (`Bobber_*`,
//! `Leaper_*`, `Runner_*`) y `Screen_InBoundsX_Latched_08d184`.

use super::{anim, grunt, motion, platforms, rng, tasks};
use crate::layout::{critters as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// ¿Ha salido por un lado de la pantalla? Hasta que no ha estado dentro
/// (se marca al entrar) nunca sale. Original:
/// `Screen_InBoundsX_Latched_08d184` (C = fuera).
pub fn out_x(m: &mut impl Memory, t: u32) -> bool {
    let x = m.read_u16(t + L::X) as i16;
    let f = m.read_u8(t + L::FLAGS2);
    if f & L::SEEN != 0 {
        return x < L::VIEW_X.0 || x > L::VIEW_X.1;
    }
    let y = m.read_u16(t + L::Y) as i16;
    if x < L::VIEW_X.1 && x > L::VIEW_X.0 && y < L::VIEW_Y.1 && y > L::VIEW_Y.0 {
        m.write_u8(t + L::FLAGS2, f | L::SEEN);
    }
    false
}

/// Plantilla 182: sube y baja. Original: `Bobber_Tmpl182_08e4e6`.
pub fn bobber(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::ATTR, L::ATTR_BOBBER);
    grunt::facing_from_param(m, t);
    grunt::vel_x_from_param(m, t);
    anim::alloc_entry(m, t, L::BOBBER_GFX);
    bobber_down(m)
}

/// Original: `Bobber_Descend_08e4fe`.
pub fn bobber_down(m: &mut impl Machine) -> Flow {
    grunt::hop_for(m, L::BOBBER_ACC, L::BOBBER_FRAMES, L::BOBBER_ANIM, L::BOBBER_DOWN_RUN);
    bobber_down_run(m)
}

/// Original: `Bobber_Descend_08e4fe__L08e52a`.
pub fn bobber_down_run(m: &mut impl Machine) -> Flow {
    grunt::hop_run(m, |y| y >= L::BOBBER_BOTTOM, L::BOBBER_UP)
}

/// Original: `Bobber_Ascend_08e566`.
pub fn bobber_up(m: &mut impl Machine) -> Flow {
    grunt::hop_for(m, L::BOBBER_ACC.wrapping_neg(), L::BOBBER_FRAMES, L::BOBBER_ANIM, L::BOBBER_UP_RUN);
    bobber_up_run(m)
}

/// Original: `Bobber_Ascend_08e566__L08e592`.
pub fn bobber_up_run(m: &mut impl Machine) -> Flow {
    grunt::hop_run(m, |y| y < L::BOBBER_TOP, L::BOBBER_DOWN)
}

/// Alta común de los que andan por el suelo: atributos, orientación y
/// velocidad.
fn ground_init(m: &mut impl Machine, t: u32) {
    grunt::set_attr(m, t);
    grunt::facing_from_param(m, t);
    grunt::vel_x_from_param(m, t);
}

/// Gráfico, animación y un primer paso con colisión.
fn first_step(m: &mut impl Machine, t: u32, gfx: u16, script: u32) {
    anim::alloc_entry(m, t, gfx);
    anim::start(m, t, script);
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    anim::step(m, t);
}

/// Plantilla 183: espera, salta y cae. Original: `Leaper_Tmpl183_08e5ce`.
pub fn leaper(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    ground_init(m, t);
    m.write_u16(t + L::VEL_Y, L::LEAPER_WAIT_VEL);
    m.write_u16(t + L::ACC_Y, L::LEAPER_WAIT_ACC);
    first_step(m, t, L::LEAPER_GFX, L::LEAPER_ANIM[0]);
    let (r, _) = rng::masked(m, L::LEAPER_WAIT.1);
    m.write_u16(t + L::TIMER, r.wrapping_add(L::LEAPER_WAIT.0));
    set_handler(m, t, L::LEAPER_WAIT_RUN);
    Flow::Return
}

/// Un frame de movimiento: integra (con o sin aceleración), sigue a la
/// escena, se da de baja fuera de la vista y anima.
fn leaper_step(m: &mut impl Machine, t: u32, accel: bool) -> Flow {
    if accel {
        grunt::integrate_accel(m, t);
    } else {
        grunt::integrate(m, t);
    }
    platforms::move_with_scene(m, t);
    if let f @ Flow::Goto(_) = grunt::cull_sub(m) {
        return f;
    }
    anim::step(m, t);
    Flow::Return
}

/// Original: `Leaper_Jump_08e622` (el estado de espera).
pub fn leaper_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if (n as i16) <= 0 || (m.read_u16(t + L::VEL_Y) as i16) >= L::LEAPER_RISE as i16 {
        set_handler(m, t, L::LEAPER_HOP);
    }
    leaper_step(m, t, true)
}

/// Salta. Original: `Leaper_Hop_08e65c`.
pub fn leaper_hop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (base, mask, acc) = L::LEAPER_JUMP;
    let (r, _) = rng::masked(m, mask);
    m.write_u16(t + L::VEL_Y, r.wrapping_add(base).wrapping_neg());
    m.write_u16(t + L::ACC_Y, acc);
    m.write_u16(t + L::TIMER, L::LEAPER_JUMP_FRAMES);
    anim::start(m, t, L::LEAPER_ANIM[1]);
    set_handler(m, t, L::LEAPER_HOP_RUN);
    leaper_hop_run(m)
}

/// Original: `Leaper_Hop_08e65c__L08e68e`.
pub fn leaper_hop_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if (n as i16) <= 0 {
        set_handler(m, t, L::LEAPER_FALL);
    }
    leaper_step(m, t, true)
}

/// Cae. Original: `Leaper_Fall_08e6b8`.
pub fn leaper_fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::LEAPER_ANIM[2]);
    set_handler(m, t, L::LEAPER_FALL_RUN);
    leaper_fall_run(m)
}

/// Original: `Leaper_Fall_08e6b8__L08e6ca`.
pub fn leaper_fall_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    leaper_step(m, t, false)
}

/// Plantilla 184: corre en línea recta. Original: `Runner_Tmpl184_08e6e0`.
pub fn runner(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    ground_init(m, t);
    first_step(m, t, L::RUNNER_GFX, L::RUNNER_ANIM);
    set_handler(m, t, L::RUNNER_RUN);
    Flow::Return
}

/// Original: `Runner_Run_08e716`.
pub fn runner_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    grunt::integrate_axis(m, t, 0);
    platforms::move_with_scene(m, t);
    if out_x(m, t) {
        return tasks::finish(m);
    }
    anim::step(m, t);
    Flow::Return
}
