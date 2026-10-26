//! Slug golpeado: en tierra da una sacudida (con guion por pendiente) y
//! vuelve a la espera en llano al acabar; en el aire sale despedido hacia
//! arriba y, al caer, encadena el aterrizaje con la sacudida.
//!
//! Original: `Slug_HitReact_02d63e` (las dos entradas, `$02D63E` y
//! `$02D67C`, y su bucle `$02D6CE`), `Slug_HitLaunchA_02d736` (bucle
//! `$02D78C`), `Slug_HitLaunchB_02d802` (bucle `$02D852`),
//! `Slug_HitLandA_02d8c8` (bucle `$02D908`) y `Slug_HitLandB_02d980`
//! (bucle `$02D9C0`) (`asm/slug/states_02ae3e.s`) y `$05E722`.

use super::{anim, items, manual_icons, player_air, slug, slug_drive, slug_hunker, slug_idle};
use crate::layout::{slug_hit as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Entrada común: guion de la tabla `scripts` por pendiente (si lo hay) y
/// `+$20` fijo.
fn pose(m: &mut impl Memory, t: u32, scripts: u32) {
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(scripts + u32::from(idx) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, L::BYTE_20_HIT);
}

/// Temporizadores `+$45` y `+$59` (éste es `$05E722`) e invulnerable.
fn keep_timers(m: &mut impl Memory, t: u32) {
    m.write_u8(t + L::TIMER_45, L::TIMER_FRAMES);
    m.write_u8(t + L::TIMER_59, L::TIMER_FRAMES);
    set_bits(m, t + L::FLAGS_13, L::BIT_13_3, false);
}

/// Sacudida en tierra (variante `k`: 0 = `$02D63E`, 1 = `$02D67C`).
pub fn react(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    pose(m, t, L::REACT_SCRIPTS[k]);
    manual_icons::stop_motion(m, t);
    m.write_u32(t + L::HANDLER_60, L::REACT_HANDLER_60);
    set_handler(m, t, L::REACT_LOOP);
    react_loop(m)
}

/// Bucle de la sacudida. Original: `Slug_HitReact_02d63e__L02d6ce`.
pub fn react_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::TIMER_45, L::TIMER_FRAMES);
    set_bits(m, t + L::FLAGS_13, L::BIT_13_3, false);
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    slug_hunker::physics(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE_FLAT);
        m.write_u8(t + L::TIMER_45, L::TIMER_FRAMES);
        m.write_u8(t + L::TIMER_59, L::TIMER_FRAMES);
        m.write_u32(t + L::HITBOX_CB, L::DRIVE_HITBOX);
    }
    if slug::ground_contact(m, t).in_air {
        set_handler(m, t, L::FALL);
    }
    if slug_drive::left_alone(m, t) {
        set_handler(m, t, L::IDLE_ENTER_B);
    }
    if slug_idle::try_destroy(m, t) {
        set_handler(m, t, L::DEATH_EXPLODE);
    }
    Flow::Return
}

/// Despedido por el aire (variante `k`: 0 = A, 1 = B).
/// Original: `Slug_HitLaunchA_02d736` / `Slug_HitLaunchB_02d802`.
pub fn launch(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    pose(m, t, L::LAUNCH_SCRIPTS[k]);
    m.write_u16(t + L::VEL_Y, L::LAUNCH_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::LAUNCH_GRAVITY);
    m.write_u16(t + L::ACC_X, 0);
    set_handler(m, t, L::LAUNCH_LOOP[k]);
    launch_loop(m, k)
}

/// Vuelo tras el golpe. Original: `$02D78C` / `$02D852`.
pub fn launch_loop(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::IN_AIR, true);
    keep_timers(m, t);
    slug::update_anim_keep_idx(m, t);
    slug_idle::leave_chassis_regs(m, t);
    clamp_vel_y(m, t);
    let row = m.regs().d[4] as u16;
    if slug::physics_air(m, t, row) {
        set_handler(m, t, L::LAND[k]);
        manual_icons::stop_motion(m, t);
    }
    anim::step(m, t);
    if slug_idle::try_destroy(m, t) {
        set_handler(m, t, L::DEATH_LAUNCH);
    }
    out_of_view(m, t)
}

/// Aterrizaje tras el vuelo (variante `k`).
/// Original: `Slug_HitLandA_02d8c8` / `Slug_HitLandB_02d980`.
pub fn land(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    pose(m, t, L::LAND_SCRIPTS[k]);
    set_handler(m, t, L::LAND_LOOP[k]);
    land_loop(m)
}

/// Original: `$02D908` / `$02D9C0`.
pub fn land_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    keep_timers(m, t);
    slug::update_anim_keep_idx(m, t);
    slug_idle::leave_chassis_regs(m, t);
    clamp_vel_y(m, t);
    slug_hunker::physics(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::REACT_LOOP);
    }
    if slug_drive::left_alone(m, t) {
        set_handler(m, t, L::IDLE_ENTER_B);
    }
    if slug_idle::try_destroy(m, t) {
        set_handler(m, t, L::DEATH_EXPLODE);
    }
    out_of_view(m, t)
}

fn clamp_vel_y(m: &mut impl Memory, t: u32) {
    let v = player_air::clamp(m.read_u16(t + L::VEL_Y), L::MAX_VEL_Y);
    m.write_u16(t + L::VEL_Y, v);
}

fn out_of_view(m: &mut impl Memory, t: u32) -> Flow {
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return Flow::Goto(L::DEATH_FINISH);
    }
    Flow::Return
}
