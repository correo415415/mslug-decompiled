//! Objeto arrojado: sale despedido (en horizontal o en vertical, con el
//! espejo según la variante y el lado del lanzador), frena hacia una
//! velocidad de caída, daña con su lista de ataque y, al tocar suelo o
//! acertar, pasa a la animación de aterrizaje. Mientras vive cuenta como
//! disparo de su jugador y se pierde al salir de la pantalla. Lleva una
//! sombra que lo sigue con retraso.
//!
//! Original: `Thrown_*` (`$09C234`–`$09C4B6`,
//! `asm/props/items_score_crates_09a0bc.s`).

use super::{anim, hits, items, motion, slug, tasks, turret};
use crate::entity::Entity;
use crate::layout::{task as T, thrown as L};
use crate::mem::{Flow, Machine, Memory};
use crate::player::controls;

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn vertical(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::KIND) & L::KIND_VERTICAL != 0
}

fn neg16(m: &mut impl Memory, a: u32) {
    let v = m.read_u16(a);
    m.write_u16(a, v.wrapping_neg());
}

fn set_bit(m: &mut impl Memory, a: u32, b: u8) {
    let v = m.read_u8(a);
    m.write_u8(a, v | b);
}

/// Se baja con sus hijas (la sombra).
fn vanish(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    tasks::kill_children(m, t);
    tasks::finish(m)
}

/// Movimiento con colisión «E» de sonda; si no toca, una segunda pasada
/// (con la celda que deja la primera). Devuelve si ha tocado suelo.
fn probe_twice(m: &mut impl Machine, t: u32) -> bool {
    let r = m.regs();
    let mut at = (r.d[3] as u16, r.d[4] as u16);
    for _ in 0..2 {
        let mut next = at;
        let hit = motion::on_entity_pos(m, t, |m| {
            let (hit, s) = motion::move_e_probe_surface(m, t, at);
            next = s.at;
            hit
        });
        if hit {
            return true;
        }
        at = next;
    }
    false
}

/// Cola común: cuenta como disparo vivo de su jugador y se pierde si sale
/// de la pantalla. Original: `Thrown_GroundTail_09c23c`.
pub fn ground_tail(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    controls::count_live_shot(m, Entity::from_raw(t));
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return vanish(m);
    }
    Flow::Return
}

/// Alta: atributos, guion y velocidades según la variante, espejo,
/// ataques y la sombra. Original: `Thrown_Init_09c25e`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::set_attr(m, t, L::ATTR_INIT);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_PRIO_KEEP) | L::ATTR_PRIO);
    anim::alloc_entry(m, t, L::GFX);
    if vertical(m, t) {
        anim::start(m, t, L::SCRIPT_V);
        m.write_u16(t + L::VEL_Y, L::SPEED);
    } else {
        anim::start(m, t, L::SCRIPT_H);
        m.write_u16(t + L::VEL_X, L::SPEED);
        m.write_u16(t + L::VEL_Y, L::VEL_Y_H);
    }
    let kind = m.read_u8(t + L::KIND);
    if kind == L::KIND_UPSIDE {
        set_bit(m, t + L::FACING, L::FLIP_Y);
        neg16(m, t + L::VEL_Y);
    } else if (kind ^ m.read_u8(t + L::SIDE)) & L::SIDE_MASK != 0 {
        set_bit(m, t + L::FACING, L::FLIP_X);
        neg16(m, t + L::VEL_X);
    }
    m.write_u32(t + L::ATTACKS, L::ATTACKS_FLY);
    slug::clear_flags13(m, t);
    tasks::spawn(m, L::SHADOW, t);
    set_handler(m, t, L::FLY);
    fly(m)
}

/// Vuelo de cada frame. Original: `Thrown_Init_09c25e+$AC`.
pub fn fly(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (sx, sy) = if vertical(m, t) { (4, 6) } else { (5, 3) };
    let vx = m.read_u16(t + L::VEL_X);
    m.write_u16(t + L::VEL_X, vx.wrapping_sub(((vx as i16) >> sx) as u16));
    let vy = m.read_u16(t + L::VEL_Y);
    let d = (L::TERMINAL_Y.wrapping_sub(vy) as i16) >> sy;
    m.write_u16(t + L::VEL_Y, vy.wrapping_add(d as u16));
    slug::clear_flags13(m, t);
    if probe_twice(m, t) {
        set_handler(m, t, L::LANDED);
    }
    if anim::step(m, t) {
        return vanish(m);
    }
    hits::run_attack_list(m, t);
    if m.read_u8(t + L::FLAGS_13) & L::ATTACK_HIT != 0 {
        set_handler(m, t, L::LANDED);
    }
    ground_tail(m)
}

/// Aterriza: se para en el eje del vuelo y pasa al guion de aterrizaje.
/// Original: `Thrown_Landed_09c39a`.
pub fn landed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if vertical(m, t) {
        m.write_u16(t + L::VEL_Y, 0);
        anim::start(m, t, L::LANDED_V);
    } else {
        m.write_u16(t + L::VEL_X, 0);
        anim::start(m, t, L::LANDED_H);
    }
    set_handler(m, t, L::LANDED_LOOP);
    landed_loop(m)
}

/// Sigue moviéndose con la sonda hasta acabar el guion.
/// Original: `Thrown_Landed_09c39a+$36`.
pub fn landed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    if anim::step(m, t) {
        return vanish(m);
    }
    ground_tail(m)
}

/// Alta de la sombra en la posición del dueño. `slow` = la que se acerca
/// 3/8 por frame (si no, 3/4). Original: `Thrown_Shadow_09c3f0` /
/// `Thrown_ShadowB_09c454`.
pub fn shadow(m: &mut impl Machine, slow: bool) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    let o = m.read_u32(t + L::OWNER);
    for f in [L::X, L::Y] {
        let v = m.read_u16(o + f);
        m.write_u16(t + f, v);
    }
    set_handler(m, t, if slow { L::SHADOW_LOOP } else { L::SHADOW_B_LOOP });
    shadow_loop(m, slow)
}

/// Sigue al dueño por detrás, con su sprite. Original: `Thrown_Shadow_09c3f0+$20`
/// / `Thrown_ShadowB_09c454+$20`.
pub fn shadow_loop(m: &mut impl Machine, slow: bool) -> Flow {
    let t = m.task_register();
    let o = m.read_u32(t + L::OWNER);
    let a = m.read_u16(o + L::ATTR).wrapping_sub(1);
    m.write_u16(t + L::ATTR, a);
    let shift = if slow { 3 } else { 2 };
    for f in [L::X, L::Y] {
        let cur = m.read_u16(t + f);
        let d = ((m.read_u16(o + f).wrapping_sub(cur) as i16) >> shift) as u16;
        m.write_u16(t + f, cur.wrapping_add(d.wrapping_mul(3)));
    }
    let s = m.read_u32(o + L::OWNER_SPRITE);
    m.write_u32(t + L::SPRITE, s);
    turret::draw(m, t);
    Flow::Return
}
