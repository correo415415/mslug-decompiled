//! Disparo del cañón del Slug: la entrada (parado o en llano), el bucle
//! mientras dura el disparo y el retroceso al acabar el guion.
//!
//! Al disparar crea el proyectil (y el humo de la boca) o, si el jugador
//! no tiene el bit 2 en su máscara de este frame, sólo el humo con caída;
//! gasta una carga de munición y pone la espera.
//!
//! Original: `Slug_FireIdle_02c190`, `Slug_FireFlat_02c24a` (bucle
//! `$02C33A`) y `Slug_FireRecoil_02c432` (bucle `$02C46E`)
//! (`asm/slug/states_02ae3e.s`) y `Slug_GaugeTick_02aab0`.

use super::{anim, collision, slug, slug_drive, slug_hunker, slug_jump, tasks, turret};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{slug_drive as D, slug_fire as L, slug_idle as I, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Hija detrás de sus hermanas, en la posición del Slug.
fn spawn_here(m: &mut impl Memory, t: u32, handler: u32) -> u32 {
    let (c, _) = tasks::spawn_last(m, handler, t);
    anim::copy_sprite_pos(m, t, c);
    c
}

/// Guion por pendiente (si lo hay) y caja de golpe; devuelve el índice.
fn start_script(m: &mut impl Memory, t: u32, scripts: u32) -> u16 {
    let b = m.read_u8(t + I::FLAGS_8C);
    m.write_u8(t + I::FLAGS_8C, b & !I::BIT_8C_5);
    let idx = slug::slope_anim_idx(m, t);
    let off = u32::from(idx.wrapping_mul(4)) as u16 as i16 as u32;
    let script = m.read_u32(scripts.wrapping_add(off));
    if script != D::NO_SCRIPT {
        anim::start(m, t, script);
    }
    idx
}

/// Consume una carga de munición (si queda). Original: `Slug_GaugeTick_02aab0`.
pub fn gauge_tick(m: &mut impl Memory, t: u32) {
    let a = m.read_u8(t + D::AMMO);
    m.write_u8(t + D::AMMO, a.saturating_sub(1));
}

/// Entrada común: guion, caja, `+$20`, el proyectil y el humo, munición,
/// espera y al bucle. `shell` = proyectil hacia la derecha; `base` = base
/// de `+$20`.
fn fire(m: &mut impl Machine, shell: u32, base: u8) -> Flow {
    let t = m.task_register();
    let idx = start_script(m, t, L::SCRIPTS);
    let off = u32::from(idx.wrapping_mul(4)) as u16 as i16 as u32;
    let attack = m.read_u32(D::FLAT_ATTACKS.wrapping_add(off));
    m.write_u32(t + D::HITBOX_CB, attack);
    m.write_u8(t + D::BYTE_20, base.wrapping_add((idx.wrapping_mul(4) & 0xFF) as u8 >> 2));
    if turret::player_mask_hit(m, t, L::MASK_BIT) {
        let c = spawn_here(m, t, L::SMOKE_DROP);
        turret::copy_pilot(m, t, c);
    } else {
        let h = if m.read_u8(t + L::SIDE) == 0xFF { L::SHELL_LEFT } else { shell };
        let c = spawn_here(m, t, h);
        turret::copy_pilot(m, t, c);
        spawn_here(m, t, L::SMOKE);
    }
    if m.read_u8(t + D::AMMO) != 0 {
        gauge_tick(m, t);
    }
    m.write_u8(t + D::COOLDOWN, L::COOLDOWN);
    let b = m.read_u8(t + L::FLAGS_8D);
    m.write_u8(t + L::FLAGS_8D, b | L::FIRED);
    m.write_u32(t + D::HANDLER_60, D::FLAT_HANDLER_60);
    if !input::pressed(m, Entity::from_raw(t), Buttons(L::MOVE_BUTTONS), Edge::Held) {
        m.write_u16(t + I::VEL_X, 0);
        m.write_u16(t + I::ACC_X, 0);
    }
    set_handler(m, t, L::LOOP);
    fire_loop(m)
}

/// Dispara parado. Original: `Slug_FireIdle_02c190`.
pub fn fire_idle(m: &mut impl Machine) -> Flow {
    fire(m, L::SHELL_IDLE, L::IDLE_BYTE_20)
}

/// Dispara en llano. Original: `Slug_FireFlat_02c24a`.
pub fn fire_flat(m: &mut impl Machine) -> Flow {
    fire(m, L::SHELL_FLAT, L::FLAT_BYTE_20)
}

/// Velocidad X limitada a ±`MAX_VEL`.
fn clamp_vel(m: &mut impl Memory, t: u32) {
    let (v, _) = collision::clamp_velocity(m.read_u16(t + I::VEL_X), L::MAX_VEL);
    m.write_u16(t + I::VEL_X, v);
}

/// Colas comunes: agacharse según la pendiente y saltar según la dirección.
fn crouch_or_jump(m: &mut impl Memory, t: u32) {
    if slug_drive::down_held(m, t) {
        let idx = slug::slope_anim_idx(m, t) & 0xFF;
        tasks::goto_by_table(m, t, D::CROUCH_BY_SLOPE, idx, None);
    }
}

fn jump(m: &mut impl Memory, t: u32) {
    let dir = slug_drive::jump_dir(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, D::JUMP_BY_DIR, u16::from(dir), None);
}

/// Sin piloto, bajarse y los golpes.
fn tail(m: &mut impl Memory, t: u32) -> Flow {
    if slug_drive::left_alone(m, t) {
        set_handler(m, t, D::IDLE_ENTER_B);
    }
    if slug_hunker::eject_pressed(m, t) {
        set_handler(m, t, D::EJECT);
    }
    let side = slug_hunker::damage_check(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, D::HIT_HANDLERS, side, None);
    Flow::Return
}

/// Mientras dispara: otro disparo, moverse, física, al acabar el guion el
/// retroceso; con poca espera, agacharse o saltar; caer.
/// Original: `Slug_FireFlat_02c24a__L02c33a`.
pub fn fire_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if slug_drive::can_fire(m, t) {
        set_handler(m, t, L::FIRE_FLAT);
    }
    if slug_drive::push_right(m, t) {
        set_handler(m, t, L::MOVE_RIGHT);
    }
    if slug_drive::push_left(m, t) {
        set_handler(m, t, L::MOVE_LEFT);
    }
    slug::update_anim(m, t);
    clamp_vel(m, t);
    if slug_jump::physics_g(m, t) {
        set_handler(m, t, D::IDLE_SLOPE);
    }
    slug::update_steering(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::RECOIL);
    }
    if m.read_u8(t + D::COOLDOWN) < L::COOLDOWN_FREE {
        crouch_or_jump(m, t);
        jump(m, t);
    }
    if slug::ground_contact(m, t).in_air {
        set_handler(m, t, D::FALL);
    }
    tail(m, t)
}

/// Retroceso tras el disparo. Original: `Slug_FireRecoil_02c432`.
pub fn recoil(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let idx = start_script(m, t, L::RECOIL_SCRIPTS);
    m.write_u8(t + D::BYTE_20, D::FLAT_BYTE_20.wrapping_add((idx.wrapping_mul(4) & 0xFF) as u8 >> 2));
    set_handler(m, t, L::RECOIL_LOOP);
    recoil_loop(m)
}

/// Original: `Slug_FireRecoil_02c432__L02c46e`.
pub fn recoil_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + I::VEL_X, 0);
    m.write_u16(t + I::ACC_X, 0);
    slug::update_anim(m, t);
    if slug_drive::push_right(m, t) {
        set_handler(m, t, D::ACCEL_RIGHT);
    }
    if slug_drive::push_left(m, t) {
        set_handler(m, t, D::ACCEL_LEFT);
    }
    clamp_vel(m, t);
    if !slug_hunker::physics(m, t) {
        set_handler(m, t, D::IDLE_SLOPE);
    }
    slug::update_steering(m, t);
    if anim::step(m, t) {
        set_handler(m, t, D::IDLE_FLAT);
    }
    crouch_or_jump(m, t);
    if slug::ground_contact(m, t).in_air {
        set_handler(m, t, D::FALL);
    }
    jump(m, t);
    if slug_drive::can_fire(m, t) {
        set_handler(m, t, L::FIRE_FLAT);
    }
    tail(m, t)
}
