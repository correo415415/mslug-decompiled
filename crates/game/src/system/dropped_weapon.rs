//! Arma que suelta el jugador al perderla: sale despedida hacia atrás y
//! hacia arriba; al tocar suelo deja una explosión pequeña, rebota
//! parpadeando y desaparece al volver a caer o al salir de la pantalla.
//!
//! Original: `DroppedWeapon_Spawn_038be4`, `DroppedWeapon_Bounce_038c70` y
//! `DroppedWeapon_Free_038cee` (`asm/player/arm_weapon_fx_0388f0.s`).

use super::{anim, explosion, items, motion, player_death, slug, tasks};
use crate::layout::{dropped_weapon as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// ¿Ha tocado suelo? (`$027BC8`, con la fila que deja el original en `d4`).
fn land(m: &mut impl Machine, t: u32) -> bool {
    let row = m.regs().d[4] as u16;
    motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row))
}

/// Lanzamiento con velocidad X `vx` (hacia delante), Y `vy` y gravedad `ay`.
fn throw(m: &mut impl Memory, t: u32, vx: u16, vy: u16, ay: u16) {
    player_death::set_vx_facing(m, t, vx);
    m.write_u16(t + L::VEL_Y, vy);
    m.write_u16(t + L::ACC_Y, ay);
    m.write_u16(t + L::ACC_X, 0);
}

/// Alta: el gráfico depende del arma del dueño. Original:
/// `DroppedWeapon_Spawn_038be4`.
pub fn spawn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let owner = m.read_u32(t + T::OWNER);
    let gfx = if m.read_u8(owner + L::OWNER_WEAPON) == L::WEAPON_ALT { L::GFX.0 } else { L::GFX.1 };
    anim::alloc_entry(m, t, gfx);
    throw(m, t, L::SPAWN_VEL.0, L::SPAWN_VEL.1, L::SPAWN_ACC_Y);
    anim::start(m, t, L::SCRIPT);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::ATTR_SET);
    set_handler(m, t, L::FLY);
    fly(m)
}

/// Vuelo hasta tocar suelo. Original: `DroppedWeapon_Spawn_038be4+$5E`.
pub fn fly(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if land(m, t) {
        set_handler(m, t, L::BOUNCE);
    }
    tail(m, t)
}

fn tail(m: &mut impl Machine, t: u32) -> Flow {
    anim::step(m, t);
    if items::out_of_view_seen(m, t, None) {
        set_handler(m, t, L::FREE);
    }
    Flow::Return
}

/// Toca suelo: explosión pequeña donde está y rebote. Original:
/// `DroppedWeapon_Bounce_038c70`.
pub fn bounce(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    // La explosión sale en el destino, pero se recoloca donde está el arma.
    let (e, _) = explosion::spawn_at_goal(m, t);
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    m.write_u16(e + L::X, x);
    m.write_u16(e + L::Y, y);
    throw(m, t, L::BOUNCE_VEL.0, L::BOUNCE_VEL.1, L::BOUNCE_ACC_Y);
    for f in L::BOUNCE_TIMERS {
        m.write_u8(t + f, L::BOUNCE_TIME);
    }
    set_handler(m, t, L::BOUNCE_RUN);
    bounce_run(m)
}

/// Rebote parpadeando hasta volver a caer. Original:
/// `DroppedWeapon_Bounce_038c70+$44`.
pub fn bounce_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let b = m.read_u8(t + L::BLINK);
    m.write_u8(t + L::BLINK, b ^ L::BLINK_BIT);
    let f = m.read_u8(t + L::FLAGS_46);
    m.write_u8(t + L::FLAGS_46, f & !L::FLAG_46_CLEAR);
    if land(m, t) {
        set_handler(m, t, L::FREE);
    }
    tail(m, t)
}

/// Baja. Original: `DroppedWeapon_Free_038cee`.
pub fn free(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}
