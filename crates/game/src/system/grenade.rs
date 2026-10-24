//! Granada de mano del jugador: sale en arco, bota una vez y a la segunda
//! explota (o antes si da en algo); al tocar agua u otro estallido, revienta
//! sin explosión. La explosión daña durante un frame y se desvanece con su
//! animación.
//!
//! Original (tarea en la zona de guiones, `$28D6BC` … `$28D9DA`):
//! lanzamiento `$28D876` (también por `JmpAbsThunk_033346`), vuelo `$28D91E`,
//! explosión `$28D6BC`, estallido `$28D706`, sus dos estados `$28D74E` y
//! `$28D76E`, y la baja `$28D794`.

use super::{anim, hits, motion, platforms, slug, sound, tasks, video};
use crate::entity::Entity;
use crate::layout::{grenade as L, task as T};
use crate::mem::{Machine, Memory};
use crate::player::controls;
use crate::Flow;

fn add16(m: &mut impl Memory, a: u32, d: u16) {
    let v = m.read_u16(a).wrapping_add(d);
    m.write_u16(a, v);
}

/// Velocidad horizontal hacia donde mira. Original: `$05DCA4`.
fn set_vel_x(m: &mut impl Memory, t: u32, v: u16) {
    let left = m.read_u8(t + L::FACING) & L::FACING_LEFT != 0;
    m.write_u16(t + L::VEL_X, if left { v.wrapping_neg() } else { v });
}

fn arc(m: &mut impl Memory, t: u32, vel: (u16, u16), gravity: u16) {
    set_vel_x(m, t, vel.0);
    m.write_u16(t + L::VEL_Y, vel.1);
    m.write_u16(t + L::ACC_Y, gravity);
    m.write_u16(t + L::ACC_X, 0);
}

/// Lanzamiento: arco, atributos y guion, sin caja de golpe, ataques de
/// vuelo, gráficos y la altura de salida; y el vuelo.
/// Original: `$28D876`.
pub fn throw(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    arc(m, t, L::THROW_VEL, L::THROW_GRAVITY);
    let f = m.read_u8(t + L::FLAGS_6B) | L::FLAGS_6B_BIT4;
    m.write_u8(t + L::FLAGS_6B, f);
    m.write_u16(t + L::ATTR, L::ATTR_THROWN);
    anim::start(m, t, L::SCRIPT);
    m.write_u32(t + L::HITBOX_CB, L::NONE);
    m.write_u32(t + L::ATTACKS, L::ATTACKS_FLY);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    for g in L::GFX {
        anim::alloc_entry(m, t, g);
    }
    let parent = m.read_u32(t + L::PARENT);
    if parent == L::POOL {
        add16(m, t + L::Y, L::RAISE);
    } else {
        let x = m.read_u16(parent + L::X);
        m.write_u16(t + L::X, x);
        let y = m.read_u16(parent + L::Y).wrapping_add(L::RAISE);
        m.write_u16(t + L::START_Y, y);
        m.write_u16(t + L::Y, y);
    }
    m.write_u16(t + L::BOUNCES, 0);
    m.write_u32(t + T::HANDLER, L::FLY);
    fly(m)
}

/// Vuelo: frena en horizontal al caer rápido, se mueve y al tocar suelo
/// bota (la segunda vez explota); anima, daña si está armada, revienta,
/// se pierde por arriba o la derecha y cuenta como granada viva.
/// Original: `$28D91E`.
pub fn fly(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if (m.read_u16(t + L::VEL_Y) as i16) < L::DRAG_BELOW {
        let vx = m.read_u16(t + L::VEL_X);
        m.write_u16(t + L::VEL_X, vx.wrapping_sub(((vx as i16) >> 4) as u16));
    }
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    let landed = if m.read_u8(L::STOP_MODE) == 0 {
        motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at))
    } else {
        motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, at.1))
    };
    if landed {
        if m.read_u16(t + L::BOUNCES) >= 1 {
            return explode(m);
        }
        arc(m, t, L::BOUNCE_VEL, L::BOUNCE_GRAVITY);
        add16(m, t + L::Y, L::BOUNCE_DROP);
        add16(m, t + L::BOUNCES, 1);
        m.write_u32(t + T::HANDLER, L::FLY);
    }
    anim::step(m, t);
    if m.read_u8(t + L::ARMED) >= 1 {
        hits::run_attack_list(m, t);
        if m.read_u8(t + L::FLAGS_13) & L::HIT != 0 {
            return explode(m);
        }
    }
    if m.read_u8(t + L::FLAGS_13) & L::BURST != 0 {
        return burst(m);
    }
    if (m.read_u16(t + L::Y) as i16) < L::MIN_Y || m.read_u16(t + L::X) >= L::MAX_X {
        return tasks::finish(m);
    }
    controls::count_live_grenade(m, Entity::from_raw(t));
    Flow::Return
}

/// Cambia a un estallido: sonido, gráficos nuevos, guion, ataques y el
/// estado de explosión.
fn blast_with(m: &mut impl Machine, gfx: u16, script: u32, attacks: u32) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SOUND);
    video::release_task_entries(m, t);
    anim::alloc_entry(m, t, gfx);
    anim::start(m, t, script);
    m.write_u32(t + L::ATTACKS, attacks);
    slug::clear_flags13(m, t);
    m.write_u32(t + T::HANDLER, L::BLAST);
    Flow::Return
}

/// Explosión. Original: `$28D6BC`.
pub fn explode(m: &mut impl Machine) -> Flow {
    blast_with(m, L::EXPLODE_GFX, L::EXPLODE_SCRIPT, L::EXPLODE_ATTACKS)
}

/// Estallido sin explosión. Original: `$28D706`.
pub fn burst(m: &mut impl Machine) -> Flow {
    blast_with(m, L::BURST_GFX, L::BURST_SCRIPT, L::BURST_ATTACKS)
}

/// Primer frame de la explosión: daña y pasa a desvanecerse.
/// Original: `$28D74E`.
pub fn blast(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    m.write_u32(t + T::HANDLER, L::FADE);
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    Flow::Return
}

/// Se desvanece sin dañar hasta acabar el guion. Original: `$28D76E`.
pub fn fade(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::ATTACKS, L::NONE);
    slug::clear_flags13(m, t);
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
