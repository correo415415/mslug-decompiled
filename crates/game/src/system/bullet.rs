//! Bala de pistola del jugador: sale con el ángulo que le deja quien
//! dispara (más la desviación que toque a la configuración), vuela recta,
//! y al dar en el suelo, en algo o al acabar su vida, explota o se pierde.
//!
//! Original: `PlayerGrenade_SpawnB_03093a` (y su bucle `$030B28`),
//! `PlayerGrenade_ExplodeGround_030bb6`, `PlayerGrenade_ExplodeAir_030bd6`,
//! `JmpToScheduler_030bf6` y `PlayerGrenade_AngleFromSpread_030bfe`
//! (`asm/player/tables_fx_030696.s`).

use super::{anim, hits, items, manual_icons, motion, slug, tasks, trig};
use crate::layout::{bullet as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

/// Desviación pequeña: los dos bits bajos de `spread`, de -2 a +1.
/// Original: `PlayerGrenade_AngleFromSpread_030bfe`.
pub fn small_spread(angle: u16, spread: u8) -> u16 {
    (angle.wrapping_add(u16::from(spread & 3)).wrapping_sub(2)) & 0xFF
}

/// Desviación en abanico (modo de pruebas y configuración 5): de -8 a +6
/// en pasos de 2, simétrica con el bit 3.
fn fan_spread(angle: u16, spread: u8) -> u16 {
    let mut d = u16::from(spread);
    if d & 8 != 0 {
        d = !d & 7;
    }
    let d = ((d + 4) & 7).wrapping_sub(4).wrapping_mul(2);
    d.wrapping_add(angle) & 0xFF
}

/// Ángulo de salida (1/256 de vuelta) para el ángulo base `angle` y la
/// desviación `spread`, según la configuración.
pub fn launch_angle(m: &mut impl Memory, angle: u16, spread: u8) -> u16 {
    if m.read_u8(L::DEBUG) & L::DEBUG_SPREAD != 0 {
        return fan_spread(angle, spread);
    }
    match m.read_u8(L::CONTROL_LAYOUT) {
        5 => fan_spread(angle, spread),
        1 => {
            let a = angle.wrapping_add(u16::from(spread & 1) * 2).wrapping_sub(1) & 0xFF;
            small_spread(a, spread)
        }
        2 | 3 => small_spread(angle, spread),
        0 => (angle.wrapping_add((u16::from(spread) + 4) & 7).wrapping_sub(4)) & 0xFF,
        _ => angle,
    }
}

/// Paso de movimiento con colisión sobre la posición de la bala. Devuelve
/// si ha dado en algo sólido, el tipo de superficie y la celda.
/// Original: `$027CEE`.
fn step(m: &mut impl Machine, t: u32) -> (bool, u8, u8) {
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    let mut out = (false, 0, 0);
    motion::on_entity_pos(m, t, |m| {
        let (hit, s) = motion::move_e_probe_surface(m, t, at);
        out = (hit, s.kind, s.raw);
        hit
    });
    out
}

/// Arranque de la bala. Original: `PlayerGrenade_SpawnB_03093a`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let angle = m.read_u8(t + L::ANGLE_SRC);
    m.write_u8(t + L::ANGLE, angle);
    let burst = m.read_u8(t + L::BURST_SRC);
    m.write_u8(t + L::BURST, burst);
    set_bits(m, t + L::FLAGS_6B, L::BULLET_FLAG, true);
    m.write_u32(t + L::ATTACKS, L::ATTACK_LIST);
    slug::clear_flags13(m, t);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    let flash = tasks::spawn(m, L::FLASH, t);
    anim::copy_sprite_pos(m, t, flash);
    m.write_u16(t + L::GFX_FIELD, L::GFX);
    anim::alloc_entry(m, t, L::GFX);
    let base = u16::from(angle) & L::ANGLE_MASK;
    let script = m.read_u32(L::SCRIPTS + u32::from(base >> 4) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    manual_icons::stop_motion(m, t);
    let spread = m.read_u8(t + L::SPREAD);
    let a = launch_angle(m, base, spread);
    let parent = m.read_u32(t + L::PARENT);
    if burst & 1 != 0 {
        // Las ráfagas impares salen un paso por detrás.
        let (dx, dy) = trig::polar(m, a, L::NUDGE);
        let left = m.read_u8(parent + L::FACING) & L::FACING_LEFT != 0;
        let dx = if left { dx.wrapping_neg() } else { dx };
        m.write_u16(t + L::VEL_X, dx as u16);
        m.write_u16(t + L::VEL_Y, dy as u16);
        step(m, t);
    }
    let (vx, vy) = trig::polar(m, a, L::SPEED);
    m.write_u16(t + L::VEL_X, vx as u16);
    m.write_u16(t + L::VEL_Y, vy as u16);
    let player = m.read_u32(parent + L::PARENT);
    if m.read_u8(player + L::FLAGS_6B) & L::FOLLOW_FACING != 0 {
        let f = m.read_u8(parent + L::FACING);
        m.write_u8(t + L::FACING, f);
        if f & L::FACING_LEFT != 0 {
            let v = m.read_u16(t + L::VEL_X).wrapping_neg();
            m.write_u16(t + L::VEL_X, v);
        }
    }
    slug::clear_flags13(m, t);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Vuelo de la bala. Original: `PlayerGrenade_SpawnB_03093a__L030b28`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_13, L::FRESH, true);
    let (mut hit, mut kind, mut raw) = step(m, t);
    if !hit {
        (hit, kind, raw) = step(m, t);
    }
    if hit {
        let high = raw & L::CELL_HIGH;
        let vanish = kind == L::VANISH_KIND || (high != 0 && high != L::CELL_HIGH);
        m.write_u32(t + T::HANDLER, if vanish { L::GONE } else { L::EXPLODE_GROUND });
    }
    anim::step(m, t);
    let fuse = m.read_u8(t + L::FUSE).wrapping_sub(1);
    m.write_u8(t + L::FUSE, fuse);
    if fuse == 0 {
        m.write_u32(t + T::HANDLER, L::GONE);
    }
    hits::run_attack_list(m, t);
    if m.read_u8(t + L::FLAGS_13) & L::HIT != 0 {
        m.write_u32(t + T::HANDLER, L::EXPLODE_AIR);
    }
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        m.write_u32(t + T::HANDLER, L::GONE);
    }
    Flow::Return
}

/// Explosión (en el suelo o en el aire): crea el efecto con el mismo
/// sprite y se da de baja el frame siguiente. Devuelve el efecto.
/// Original: `PlayerGrenade_ExplodeGround_030bb6` / `_ExplodeAir_030bd6`.
pub fn explode(m: &mut impl Machine, in_air: bool) -> u32 {
    let t = m.task_register();
    let fx = if in_air { L::AIR_FX } else { L::GROUND_FX };
    let (e, _) = tasks::spawn_last(m, fx, t);
    anim::copy_sprite_pos(m, t, e);
    let g = m.read_u16(t + L::GFX_FIELD);
    m.write_u16(e + L::GFX_FIELD, g);
    m.write_u32(t + T::HANDLER, L::GONE);
    e
}

/// Se da de baja. Original: `JmpToScheduler_030bf6`.
pub fn gone(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}
