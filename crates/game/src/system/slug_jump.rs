//! Salto del Slug: se agacha (con la velocidad horizontal que toque a la
//! dirección), despega, vuela con algo de control en el aire y aterriza
//! (en llano frena un rato; en cuesta pasa al aterrizaje inclinado).
//!
//! Original: `Slug_JumpNeutral_02c9c4`, `Slug_JumpRight_02c9d4`,
//! `Slug_JumpLeft_02c9f0`, `Slug_JumpCrouch_02c908` (y su bucle
//! `$02C95C`), `Slug_JumpLaunch_02ca0c`, `Slug_JumpAir_02ca8a` (y
//! `$02CA98`), `Slug_JumpAirFire_02cc02`, `Slug_JumpLand_02cc1a` (y su
//! bucle `$02CCB0`) (`asm/slug/states_02ae3e.s`), `Slug_PhysicsG_02a85a`,
//! `Slug_TerrainIsSlope_02a4ec`, `Slug_InputDirByLayoutB_02ab3c` y `$02A374`
//! (`asm/slug/helpers_0295a6.s`).

use super::{anim, hits, items, player_air, slug, slug_drive, slug_hunker, slug_idle, tasks};
use crate::entity::Entity;
use crate::input;
use crate::layout::{slug_drive as D, slug_jump as L, task as T};
use crate::mem::{Machine, Memory};
use crate::player::controls;
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Guion, ataque y `+$20` de una tabla por índice de pendiente.
fn slope_pose(m: &mut impl Memory, t: u32, scripts: u32, attacks: u32, base: u8) {
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(scripts + u32::from(idx) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    let attack = m.read_u32(attacks + u32::from(idx) * 4);
    m.write_u32(t + L::HITBOX_CB, attack);
    m.write_u8(t + L::BYTE_20, base.wrapping_add(idx as u8));
}

/// Salto vertical. Original: `Slug_JumpNeutral_02c9c4`.
pub fn neutral(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    start_with(m, t, 0)
}

/// Salto a la derecha. Original: `Slug_JumpRight_02c9d4`.
pub fn right(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = if m.read_u16(t + L::DIR_MODE) == L::DIR_BLOCK_RIGHT { 0 } else { L::VEL_RIGHT };
    start_with(m, t, v)
}

/// Salto a la izquierda. Original: `Slug_JumpLeft_02c9f0`.
pub fn left(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = if m.read_u16(t + L::DIR_MODE) == L::DIR_BLOCK_LEFT { 0 } else { L::VEL_LEFT };
    start_with(m, t, v)
}

fn start_with(m: &mut impl Machine, t: u32, vel_x: u16) -> Flow {
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    m.write_u16(t + L::VEL_X, vel_x);
    crouch(m)
}

/// Se agacha para saltar. Original: `Slug_JumpCrouch_02c908`.
pub fn crouch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    m.write_u16(t + L::ACC_X, 0);
    slope_pose(m, t, L::CROUCH_SCRIPTS, L::CROUCH_ATTACKS, L::CROUCH_BYTE_20);
    m.write_u32(t + L::HANDLER_60, L::CROUCH_HANDLER_60);
    set_handler(m, t, L::CROUCH_LOOP);
    crouch_loop(m)
}

/// Agachado: al acabar el guion despega. Original: `Slug_JumpCrouch_Loop_02c95c`.
pub fn crouch_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    physics_g(m, t);
    slug::update_steering(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::LAUNCH);
    }
    hits::run_attack_list(m, t);
    if slug_drive::can_fire(m, t) {
        set_handler(m, t, L::FIRE_CROUCH);
    }
    if slug_drive::left_alone(m, t) {
        set_handler(m, t, L::IDLE_ENTER_B);
    }
    hit_table(m, t, L::HIT_HANDLERS);
    Flow::Return
}

/// Física G: tope contra paredes y el paso con la Y de apoyo.
/// Original: `Slug_PhysicsG_02a85a`.
pub fn physics_g(m: &mut impl Machine, t: u32) -> bool {
    if let Some((_, dir)) = slug::wheel_against_wall(m, t) {
        slug::latch_x(m, dir as u16);
    }
    slug_idle::step_on_base(m, t)
}

/// ¿Es cuesta la pendiente guardada en `+$80`? Original:
/// `Slug_TerrainIsSlope_02a4ec` (C = sí).
pub fn stored_slope_is_hill(m: &mut impl Memory, t: u32) -> bool {
    let s = m.read_u16(t + L::SLOPE);
    m.read_u8(D::FLAT_BY_SLOPE.wrapping_add(s as i16 as u32)) == 0
}

/// Salto pulsado (según la configuración): la dirección horizontal + 1.
/// Original: `Slug_InputDirByLayoutB_02ab3c` (con `$02A374`).
pub fn jump_pressed_dir(m: &mut impl Memory, t: u32) -> Option<u8> {
    controls::jump_pressed(m, Entity::from_raw(t)).map(|d| d + 1)
}

/// Despegue: velocidad, gravedad, la Y de apoyo no baja de la del
/// sprite, guion, la pendiente y sigue en el aire. Original: `Slug_JumpLaunch_02ca0c`.
pub fn launch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_5B, L::BIT_5B_3, false);
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    m.write_u16(t + L::VEL_Y, L::LAUNCH_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::LAUNCH_ACC_Y);
    let y = m.read_u16(t + L::Y);
    if (y as i16) > (m.read_u16(t + L::BASE_Y) as i16) {
        m.write_u16(t + L::BASE_Y, y);
    }
    slope_pose(m, t, L::LAUNCH_SCRIPTS, L::LAUNCH_ATTACKS, L::LAUNCH_BYTE_20);
    set_handler(m, t, L::AIR);
    let (_, _, d2) = slug::terrain_slope(m, t);
    if d2 != m.read_u16(t + L::SLOPE) {
        m.write_u16(t + L::SLOPE, d2);
    }
    slug::update_steering(m, t);
    air(m)
}

/// En el aire: puede disparar. Original: `Slug_JumpAir_02ca8a`.
pub fn air(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if slug_drive::can_fire(m, t) {
        set_handler(m, t, L::FIRE_CROUCH_AIR);
    }
    air_step(m)
}

/// En el aire ya disparando. Original: `Slug_JumpAirFire_02cc02`.
pub fn air_fire(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::TILTED, true);
    if slug_drive::can_fire(m, t) {
        set_handler(m, t, L::FIRE_AIR_B);
    }
    air_step(m)
}

/// Aceleración horizontal según el mando y tope de velocidad.
fn steer(m: &mut impl Memory, t: u32) -> u16 {
    let k = u32::from(input::horizontal_held(m, Entity::from_raw(t)));
    let acc = m.read_u16(L::STEER_ACC + k * 2);
    m.write_u16(t + L::ACC_X, acc);
    acc
}

fn clamp_vel_x(m: &mut impl Memory, t: u32) {
    let v = player_air::clamp(m.read_u16(t + L::VEL_X), L::MAX_VEL_X);
    m.write_u16(t + L::VEL_X, v);
}

/// Si la pendiente ha cambiado, la guarda e inclina el sprite.
/// Devuelve si ha cambiado.
fn tilt_to_slope(m: &mut impl Memory, t: u32) -> bool {
    let (_, _, d2) = slug::terrain_slope(m, t);
    if d2 == m.read_u16(t + L::SLOPE) {
        return false;
    }
    tilt_to(m, t, d2);
    true
}

fn hit_table(m: &mut impl Memory, t: u32, table: u32) {
    let side = slug_hunker::damage_check(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, table, side, None);
}

/// Vuelo: control en el aire, física, aterrizaje (agachado si abajo, o
/// disparando), inclinación al pisar cuesta, golpes y fuera de pantalla.
/// Original: `Slug_JumpAir_02ca8a__L02ca98`.
pub fn air_step(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    steer(m, t);
    clamp_vel_x(m, t);
    let vy = m.read_u16(t + L::VEL_Y);
    if (vy as i16) <= 0 {
        m.write_u16(t + L::VEL_Y, player_air::clamp(vy, L::MAX_VEL_Y));
    }
    set_bits(m, t + L::FLAGS_8D, L::IN_AIR, true);
    slug::update_anim_keep_idx(m, t);
    slug_idle::leave_chassis_regs(m, t);
    let row = m.regs().d[4] as u16;
    if slug::physics_air(m, t, row) {
        set_handler(m, t, L::LAND);
        if slug_drive::down_held(m, t) {
            let idx = slug::slope_anim_idx(m, t) & 0xFF;
            tasks::goto_by_table(m, t, L::CROUCH_LAND, idx, None);
        }
        if slug_drive::can_fire(m, t) {
            set_handler(m, t, L::FIRE_FLAT);
        }
    }
    anim::step(m, t);
    if (m.read_u16(t + L::VEL_Y) as i16) <= 0 && slug::ground_contact(m, t).wheels > 0 {
        m.write_u16(t + L::VEL_X, 0);
        m.write_u16(t + L::ACC_X, 0);
        if tilt_to_slope(m, t) && m.read_u32(t + T::HANDLER) == L::AIR {
            set_handler(m, t, L::AIR_FIRE);
        }
    }
    let table = if m.read_u8(t + L::FLAGS_8D) & L::IN_AIR != 0 { L::HIT_HANDLERS_AIR } else { L::HIT_HANDLERS };
    hit_table(m, t, table);
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return Flow::Goto(L::DEATH_FINISH);
    }
    Flow::Return
}

/// Aterrizaje: queda en tierra, la velocidad horizontal se reduce a
/// ±`LAND_VEL_X`; en cuesta pasa al aterrizaje inclinado y en llano frena.
/// Original: `Slug_JumpLand_02cc1a`.
pub fn land(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    set_bits(m, t + L::FLAGS_5B, L::LANDED, true);
    m.write_u16(t + L::ACC_Y, 0);
    m.write_u16(t + L::VEL_Y, 0);
    let vx = m.read_u16(t + L::VEL_X);
    if vx != 0 {
        let v = if (vx as i16) < 0 { L::LAND_VEL_X.wrapping_neg() } else { L::LAND_VEL_X };
        m.write_u16(t + L::VEL_X, v);
    }
    let (_, _, d2) = slug::terrain_slope(m, t);
    m.write_u16(t + L::SLOPE, d2);
    if stored_slope_is_hill(m, t) {
        set_handler(m, t, L::LAND_SLOPE);
        return Flow::Goto(L::LAND_SLOPE);
    }
    m.write_u8(t + L::LAND_TIMER, L::LAND_FRAMES);
    slope_pose(m, t, L::LAND_SCRIPTS, L::LAND_ATTACKS, L::LAND_BYTE_20);
    set_handler(m, t, L::LAND_LOOP);
    land_loop(m)
}

/// Frenando tras aterrizar en llano. Original: `Slug_JumpLand_02cc1a__L02ccb0`.
pub fn land_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    if slug_drive::down_held(m, t) {
        let idx = slug::slope_anim_idx(m, t) & 0xFF;
        tasks::goto_by_table(m, t, L::CROUCH_LAND, idx, None);
    }
    if steer(m, t) == 0 {
        let v = ((m.read_u16(t + L::VEL_X).wrapping_neg() as i16) >> 3) as u16;
        m.write_u16(t + L::ACC_X, v);
    }
    clamp_vel_x(m, t);
    slug_hunker::physics(m, t);
    slug::update_steering(m, t);
    let n = m.read_u8(t + L::LAND_TIMER).wrapping_sub(1);
    m.write_u8(t + L::LAND_TIMER, n);
    if anim::step(m, t) || n == 0 {
        m.write_u16(t + L::VEL_X, 0);
        m.write_u16(t + L::ACC_X, 0);
        set_handler(m, t, L::IDLE_SLOPE);
    }
    if tilt_to_slope(m, t) {
        set_handler(m, t, L::LAND_SLOPE_LOOP);
    }
    hits::run_attack_list(m, t);
    if slug::ground_contact(m, t).in_air {
        set_handler(m, t, L::FALL);
    }
    let dir = jump_pressed_dir(m, t).unwrap_or(0);
    tasks::goto_by_table(m, t, L::JUMP_BY_DIR, u16::from(dir), None);
    if slug_drive::can_fire(m, t) {
        set_handler(m, t, L::FIRE_FLAT);
    }
    if slug_drive::left_alone(m, t) {
        set_handler(m, t, L::IDLE_ENTER_B);
    }
    if slug_hunker::eject_pressed(m, t) {
        set_handler(m, t, L::EJECT);
    }
    hit_table(m, t, L::HIT_HANDLERS);
    Flow::Return
}

/// Preparación común de la caída: sin el bit 5 de `+$8C`, gravedad, sin
/// aceleración y la Y de apoyo baja (sin quedar por encima del sprite).
fn fall_setup(m: &mut impl Memory, t: u32) {
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    m.write_u16(t + L::ACC_Y, L::FALL_ACC_Y);
    m.write_u16(t + L::ACC_X, 0);
    let base = m.read_u16(t + L::BASE_Y).wrapping_add(L::FALL_BASE_STEP);
    m.write_u16(t + L::BASE_Y, base);
    let y = m.read_u16(t + L::Y);
    if (y as i16) > (base as i16) {
        m.write_u16(t + L::BASE_Y, y);
    }
}

/// Inclina el sprite a la pendiente `d2` (guardándola) y marca `TILTED`.
fn tilt_to(m: &mut impl Memory, t: u32, d2: u16) {
    m.write_u16(t + L::SLOPE, d2);
    let k = (0x20u16.wrapping_sub(((d2 as i16) >> 1) as u16)).min(L::TILT_MAX);
    let s = slug::angle_sprite(m, k);
    let script = m.read_u32(L::TILT_SCRIPTS.wrapping_add(s.wrapping_mul(4) as i16 as u32));
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, (k >> 2) as u8);
    set_bits(m, t + L::FLAGS_8D, L::TILTED, true);
}

/// Empieza a caer sin cambiar el sprite. Original: `Slug_FallStart_02cffa`.
pub fn fall_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    fall_setup(m, t);
    set_handler(m, t, L::FALL_START_LOOP);
    fall_start_loop(m)
}

/// Original: `Slug_FallStart_02cffa+$2A`.
pub fn fall_start_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::TILTED, true);
    fall_loop(m)
}

/// Empieza a caer con el sprite inclinado a la pendiente.
/// Original: `Slug_Fall_02d02e`.
pub fn fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    fall_setup(m, t);
    let (_, _, d2) = slug::terrain_slope(m, t);
    tilt_to(m, t, d2);
    set_handler(m, t, L::FALL_LOOP);
    fall_loop(m)
}

/// Caída ya disparando. Original: `Slug_FallFire_02d26e`.
pub fn fall_fire(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::TILTED, true);
    if slug_drive::can_fire(m, t) {
        set_handler(m, t, L::FIRE_AIR_B);
    }
    fall_loop(m)
}

/// Cayendo: control en el aire, topes de velocidad, disparo, aterrizaje
/// (agachado si abajo y, con algunas configuraciones, saltando otra vez),
/// inclinación al pisar cuesta, golpes y fuera de pantalla.
/// Original: `Slug_Fall_Loop_02d0c4`.
pub fn fall_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    steer(m, t);
    clamp_vel_x(m, t);
    let vy = player_air::clamp(m.read_u16(t + L::VEL_Y), L::MAX_VEL_Y);
    m.write_u16(t + L::VEL_Y, vy);
    set_bits(m, t + L::FLAGS_8D, L::IN_AIR, true);
    slug::update_anim_keep_idx(m, t);
    slug_idle::leave_chassis_regs(m, t);
    if slug_drive::can_fire(m, t) {
        set_handler(m, t, L::FIRE_AIR);
    }
    let row = m.regs().d[4] as u16;
    if slug::physics_air(m, t, row) {
        set_handler(m, t, L::LAND);
        if slug_drive::down_held(m, t) {
            let idx = slug::slope_anim_idx(m, t) & 0xFF;
            tasks::goto_by_table(m, t, L::CROUCH_LAND, idx, None);
        }
        for layout in L::REJUMP_LAYOUTS {
            if m.read_u8(L::CONTROL_LAYOUT) == layout {
                let dir = jump_pressed_dir(m, t).unwrap_or(0);
                tasks::goto_by_table(m, t, L::JUMP_BY_DIR, u16::from(dir), None);
            }
        }
    }
    slug::update_steering(m, t);
    anim::step(m, t);
    if (m.read_u16(t + L::VEL_Y) as i16) <= 0 {
        if slug::ground_contact(m, t).wheels > 0 {
            m.write_u16(t + L::VEL_X, 0);
            m.write_u16(t + L::ACC_X, 0);
        }
        let (_, _, d2) = slug::terrain_slope(m, t);
        if d2 != m.read_u16(t + L::SLOPE) {
            tilt_to(m, t, d2);
            if m.read_u32(t + T::HANDLER) == L::FALL_LOOP {
                set_handler(m, t, L::FALL_FIRE);
            }
        }
    }
    let table = if m.read_u8(t + L::FLAGS_8D) & L::IN_AIR != 0 { L::HIT_HANDLERS_AIR } else { L::HIT_HANDLERS };
    hit_table(m, t, table);
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return Flow::Goto(L::DEATH_FINISH);
    }
    Flow::Return
}
