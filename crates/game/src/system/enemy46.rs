//! Enemigo de tipo `$46`: aparece pegado al suelo (con una animación de
//! entrada si toca), anda hacia donde mira a la velocidad de la dificultad
//! y, al llegar a un borde, se para, mira alrededor, se da la vuelta y
//! espera un rato al azar antes de volver a andar. Si le dan suelta su
//! objeto y muere (como soldado o con el par de humos); fuera de la vista
//! se da de baja.
//!
//! Original: `Enemy46_Boot_046140` … `Enemy46_Tail_046220`, los estados
//! siguientes `$046260`–`$04631E`, `Drop_ProbeAndNudgeY_0463c2` y `$0463BA`
//! (`asm/enemies/vehicles/deploy_045f2c.s`).

use super::{anim, collision, fort, items, platforms, player_ground, rng, soldier, tasks};
use crate::layout::{enemy46 as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Sondeo del suelo dejando los registros como el original (`$027EBA`):
/// `a1` = registro mirado, `d3`/`d4` = celda. Devuelve si está en el aire.
fn probe(m: &mut impl Machine, t: u32) -> bool {
    let mut r = m.regs();
    let f = collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16));
    r.a[1] = f.hit.unwrap_or(f.record);
    r.d[3] = (r.d[3] & 0xFFFF_0000) | u32::from(f.at.0);
    r.d[4] = (r.d[4] & 0xFFFF_0000) | u32::from(f.at.1);
    m.set_regs(r);
    f.in_air
}

/// Alta. Original: `Enemy46_Boot_046140`.
pub fn boot(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    collision::snap_to_ground(m, t);
    let gfx = if m.read_u8(t + L::ALT) != 0 { L::GFX.1 } else { L::GFX.0 };
    anim::alloc_entry(m, t, gfx);
    let f = m.read_u8(t + L::P_FACING);
    m.write_u8(t + L::FACING, f);
    anim::set_attr(m, t, L::ATTR_INIT);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::ATTR_SET);
    if m.read_u8(t + L::P_INTRO) == 0 {
        return walk(m);
    }
    anim::start(m, t, L::S_INTRO);
    set_handler(m, t, L::INTRO);
    intro(m)
}

/// Animación de entrada (cayendo si no pisa). Original: `Enemy46_PhaseA_046192`.
pub fn intro(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if probe(m, t) {
        collision::fall(m, t);
    }
    if anim::step(m, t) {
        set_handler(m, t, L::WALK);
    }
    tail(m)
}

/// Echa a andar. Original: `Enemy46_Move_0461BC`.
pub fn walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = soldier::by_difficulty(m, t, L::SPEED_TABLE);
    let v = if m.read_u8(t + L::FACING) & 1 == 0 { v.wrapping_neg() } else { v };
    m.write_u16(t + L::VEL_X, v);
    anim::start(m, t, L::S_WALK);
    set_handler(m, t, L::WALK_RUN);
    walk_run(m)
}

/// Andando; al llegar a un borde se para. Original: `Enemy46_PhaseB_0461EA`.
pub fn walk_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if probe(m, t) {
        collision::fall(m, t);
    } else {
        player_ground::move_ground(m, t);
        if m.read_u8(t + L::FLAGS_5A) & L::AT_EDGE != 0 {
            set_handler(m, t, L::STOP);
        }
    }
    anim::step(m, t);
    tail(m)
}

/// Cola de cada estado: si le dan suelta su objeto y muere; fuera de la
/// vista se da de baja. Original: `Enemy46_Tail_046220`.
pub fn tail(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if fort::hit_side(m, t).is_some() {
        drop_item(m, t);
        if m.read_u8(t + L::ALT) == 0 {
            soldier::check_death(m, t);
        } else {
            set_handler(m, t, L::SMOKE_DEATH);
        }
    }
    if items::out_of_view(m, t, None) {
        set_handler(m, t, L::FREE);
    }
    Flow::Return
}

/// Suelta el objeto que lleva (un poco más abajo que él).
/// Original: `Drop_ProbeAndNudgeY_0463c2`.
pub fn drop_item(m: &mut impl Memory, t: u32) {
    let kind = m.read_u8(t + L::ALT);
    if let Some(o) = items::spawn_here(m, t, kind, L::DROP_FIELD) {
        let y = m.read_u16(o + L::Y);
        m.write_u16(o + L::Y, y.wrapping_add(L::DROP_DY));
    }
}

/// Se para en el borde. Original: `$046260`.
pub fn stop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_STOP);
    set_handler(m, t, L::STOP_RUN);
    stop_run(m)
}

/// Original: `$046272`.
pub fn stop_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::TURN);
    }
    tail(m)
}

/// Se da la vuelta (y cuenta las vueltas). Original: `$04628A`.
pub fn turn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::TURNS);
    m.write_u16(t + L::TURNS, n.wrapping_add(1));
    anim::start(m, t, L::S_TURN);
    set_handler(m, t, L::TURN_RUN);
    turn_run(m)
}

/// Original: `$0462A0`.
pub fn turn_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        let f = m.read_u8(t + L::FACING);
        m.write_u8(t + L::FACING, f ^ 1);
        set_handler(m, t, L::WAIT);
    }
    tail(m)
}

/// Espera un rato al azar con una de cuatro poses al azar.
/// Original: `$0462C0`.
pub fn wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (r, _) = rng::next(m);
    m.write_u16(t + L::TIMER, (r & L::WAIT_RAND).wrapping_add(L::WAIT_MIN));
    let (r, _) = rng::next(m);
    let script = m.read_u32(L::WAIT_SCRIPTS + u32::from(r & 3) * 4);
    if script != 0xFFFF_FFFF {
        anim::start(m, t, script);
    }
    set_handler(m, t, L::WAIT_RUN);
    wait_run(m)
}

/// Original: `$0462FE`.
pub fn wait_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n as i16 <= 0 {
        set_handler(m, t, L::WALK);
    }
    tail(m)
}

/// Baja. Original: `$0463BA`.
pub fn free(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}
