//! Jugador en el aire: el golpe que lo lanza por los aires, el control
//! horizontal mientras cae, la física común de caída (aterrizar, montar el
//! Slug, agarrarse a una anilla, resbalar por una pared), la animación y
//! las acciones en el aire (disparo, granada, cuchillo), la cola común
//! (caer al vacío) y la publicación del estado (muerte por tiempo o
//! ahogado).
//!
//! Original: `Player_Knockback_036d64` (+`_Setup_036d72`),
//! `Player_Knockback_AirCtrl_036dca`, `Player_Fall_Physics_036e42`,
//! `Player_KnockbackDelay_036fc2`, `Player_KnockbackHold_037018`,
//! `PlayerAnimState_03705A`, `Player_AirActionSelect_037168`,
//! `Player_Air_Tail_036c3a`, el salto (`Player_JumpStart_036914`, su espera
//! `$036A36` y `Player_JumpAir_036a70`) (`asm/player/air_death_crouch_036632.s`),
//! `Player_ActionSelect_0330d0` / `_B_0331ae` / `_B_Fire_0331ce`,
//! `PlayerRoute_PublishState_033522`, `$0334A2` / `Player_DeathGate_0334c6`
//! (`asm/player/core_032aa8.s`), `TargetRing_*` (`$08F470`, `$08F520`) y
//! `$0267F4`.

use super::{anim, collision, hits, items, motion, player, slug, sound};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{player_air as L, task as T};
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

/// Los dos estados de espera del golpe no eligen pose ni acción.
fn waiting(m: &mut impl Memory, t: u32) -> bool {
    matches!(m.read_u32(t + T::HANDLER), L::KNOCK_DELAY_WAIT | L::KNOCK_HOLD_WAIT)
}

fn falling(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::STATE) == L::STATE_FALL
}

fn clear_arm(m: &mut impl Memory, t: u32) {
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
}

/// Pose del brazo: desplazamiento, estado del cuerpo y el guion (la pose
/// es el puntero que hay justo antes del guion).
fn pose(m: &mut impl Memory, t: u32, off: (u16, u16), state: u8, script: u32) {
    m.write_u16(t + L::OFFSET_X, off.0);
    m.write_u16(t + L::OFFSET_Y, off.1);
    m.write_u8(t + L::STATE, state);
    let p = m.read_u32(script - 4);
    m.write_u32(t + L::POSE, p);
}

fn set_attr_bit0(m: &mut impl Memory, t: u32, on: bool) {
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, if on { a | L::ATTR_BIT0 } else { a & !L::ATTR_BIT0 });
}

fn a_or_c_pressed(m: &mut impl Memory, t: u32) -> bool {
    input::pressed(m, Entity::from_raw(t), Buttons::A.or(Buttons::C), Edge::Pressed)
}

/// Limita `v` a ±`max` (con signo). Original: `$0267F4`.
pub fn clamp(v: u16, max: u16) -> u16 {
    collision::clamp_velocity(v, max).0
}

/// Golpe recibido: sin velocidad horizontal ni aceleración, con la
/// gravedad del golpe. Original: `Player_Knockback_036d64`.
pub fn knockback(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u16(t + L::ACC_Y, L::KNOCK_GRAVITY);
    knock_setup(m)
}

/// Pose y animación del golpe, caja de golpe en el aire y al control en
/// el aire. Original: `Player_Knockback_Setup_036d72`.
pub fn knock_setup(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    clear_arm(m, t);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    m.write_u8(t + L::STATE, L::STATE_FALL);
    let p = m.read_u32(L::KNOCK_SCRIPT - 4);
    m.write_u32(t + L::POSE, p);
    m.write_u8(t + L::BYTE_21, 0xFF);
    anim::start(m, t, L::KNOCK_SCRIPT);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_AIR);
    set_handler(m, t, L::KNOCK_AIR_CTRL);
    air_ctrl(m)
}

/// Control horizontal en el aire: la aceleración sale de la dirección
/// mantenida (al pulsar A o C, además se gira hacia ella); sin dirección,
/// frena lo que quede de velocidad. Después, la caída.
/// Original: `Player_Knockback_AirCtrl_036dca`.
pub fn air_ctrl(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let turn = a_or_c_pressed(m, t);
    let h = input::horizontal_held(m, Entity::from_raw(t));
    let acc = m.read_u16(L::VEL_TABLE + u32::from(h) * 2);
    m.write_u16(t + L::ACC_X, acc);
    if turn && acc != 0 {
        set_bits(m, t + L::FACING, L::FACING_LEFT, (acc as i16) < 0);
    }
    let vx = m.read_u16(t + L::VEL_X);
    if acc == 0 && vx != 0 {
        let brake = if (vx as i16) < 0 { L::BRAKE } else { L::BRAKE.wrapping_neg() };
        m.write_u16(t + L::ACC_X, brake);
    }
    fall_physics(m)
}

/// Un frame de caída: limita las velocidades, mueve con colisión y
/// aterriza, anima, y al bajar mira si monta el Slug, se agarra a una
/// anilla o resbala por una pared; si no, la acción en el aire. Al final,
/// la cola común. Original: `Player_Fall_Physics_036e42`.
pub fn fall_physics(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let vx = m.read_u16(t + L::VEL_X);
    m.write_u16(t + L::VEL_X, clamp(vx, L::MAX_VEL_X));
    let vy = m.read_u16(t + L::VEL_Y);
    if (vy as i16) <= 0 {
        m.write_u16(t + L::VEL_Y, clamp(vy, L::MAX_VEL_Y));
    }
    set_bits(m, t + L::AIR, L::IN_AIR, true);
    set_bits(m, t + L::AIR, L::DESCENDING, false);
    player::frame_common_no_prio(m, t);
    let row = m.regs().d[4] as u16;
    if motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row)) {
        set_handler(m, t, L::LAND);
        set_bits(m, t + L::AIR, L::IN_AIR | L::DESCENDING, false);
    }
    if player::anim_done(m, t) {
        set_bits(m, t + L::STATUS, L::ARM_FIRING, false);
    }
    if !waiting(m, t) {
        anim_state(m, t);
    }
    if (m.read_u16(t + L::VEL_Y) as i16) <= 0 {
        descend_checks(m, t);
    } else if !waiting(m, t) {
        air_action_select(m, t);
    }
    air_tail(m, t);
    Flow::Return
}

/// Bajando: montar el Slug, agarrarse a una anilla, resbalar por una
/// pared o, si nada de eso, la acción en el aire.
fn descend_checks(m: &mut impl Machine, t: u32) {
    set_bits(m, t + L::AIR, L::DESCENDING, true);
    if m.read_u8(t + L::STATUS) & L::RIDING != 0 {
        set_handler(m, t, L::RIDE_SLUG);
        return;
    }
    if m.read_u8(t + L::AIR) & L::IN_AIR != 0 {
        if let Some((id, true)) = ring_near(m, t) {
            m.write_u8(t + L::RING_ID, id);
            set_handler(m, t, L::HANG_RING);
            take_ring(m, t, id);
            set_bits(m, t + L::AIR, L::DESCENDING, false);
            return;
        }
    }
    let f = m.read_u8(t + L::FLAGS_69);
    if f & L::WALL_MASK != 0 && f & L::WALL_GRAB != 0 {
        set_handler(m, t, L::WALL_DROP);
        let v = if f & L::WALL_RIGHT != 0 { L::WALL_DROP_VEL } else { L::WALL_DROP_VEL.wrapping_neg() };
        m.write_u16(t + L::VEL_X, v);
        m.write_u16(t + L::ACC_X, 0);
        return;
    }
    if !waiting(m, t) {
        air_action_select(m, t);
    }
}

/// Espera del golpe: hasta 12 frames (o hasta pulsar A o C) controlando la
/// caída antes de volver a la pose del golpe.
/// Original: `Player_KnockbackDelay_036fc2`.
pub fn knock_delay(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    knock_wait_init(m, t);
    m.write_u8(t + L::DELAY, L::DELAY_FRAMES);
    set_handler(m, t, L::KNOCK_DELAY_WAIT);
    knock_delay_wait(m)
}

/// Bucle de [`knock_delay`]. Original: `Player_KnockbackDelay_036fc2__L036ff6`.
pub fn knock_delay_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if (m.read_u8(t + L::DELAY) as i8) <= 0 {
        return knock_setup(m);
    }
    knock_hold_wait(m)
}

/// Como [`knock_delay`] sin tiempo límite. Original: `Player_KnockbackHold_037018`.
pub fn knock_hold(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    knock_wait_init(m, t);
    set_handler(m, t, L::KNOCK_HOLD_WAIT);
    knock_hold_wait(m)
}

/// Bucle de [`knock_hold`]: al pulsar A o C, la pose del golpe; si no, el
/// control en el aire. Original: `Player_KnockbackHold_037018__L037046`.
pub fn knock_hold_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if a_or_c_pressed(m, t) {
        return knock_setup(m);
    }
    air_ctrl(m)
}

fn knock_wait_init(m: &mut impl Memory, t: u32) {
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::ACC_X, 0);
    clear_arm(m, t);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    m.write_u16(t + L::ACC_Y, L::KNOCK_GRAVITY);
}

/// Pose del brazo en el aire según se apunte abajo o no (si el brazo no
/// está ocupado), sólo cuando cambia el modo; al cambiar de dirección el
/// brazo la toma enseguida. Original: `PlayerAnimState_03705A`.
pub fn anim_state(m: &mut impl Memory, t: u32) {
    if m.read_u8(t + L::STATUS) & (L::ARM_BUSY | L::ARM_FIRING) != 0 {
        return;
    }
    let fall = falling(m, t);
    let history = m.read_u8(t + L::AIM_HISTORY);
    if m.read_u8(t + L::AIM) == L::AIM_DOWN {
        if m.read_u16(t + L::MODE) == 3 {
            return;
        }
        set_attr_bit0(m, t, false);
        m.write_u16(t + L::MODE, 3);
        if fall {
            pose(m, t, (0, 7), L::STATE_FALL, L::POSE_DOWN[1]);
        } else {
            pose(m, t, (2, 2), L::STATE_JUMP, L::POSE_DOWN[0]);
        }
        if history == (L::AIM_FORWARD << 4) | L::AIM_DOWN {
            set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, true);
        }
    } else {
        if m.read_u16(t + L::MODE) == 0 {
            return;
        }
        m.write_u16(t + L::MODE, 0);
        if fall {
            pose(m, t, (0, 7), L::STATE_FALL, L::POSE_FORWARD[1]);
        } else {
            pose(m, t, (0, 2), L::STATE_JUMP, L::POSE_FORWARD[0]);
            if m.read_u8(t + L::WEAPON) == 0 {
                set_attr_bit0(m, t, true);
            }
        }
        if history == (L::AIM_DOWN << 4) | L::AIM_FORWARD {
            set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, true);
        }
    }
}

/// Acción pedida este frame: cuchillo ($FF), granada (3), disparo hacia
/// arriba (1), abajo (4), adelante (2) o normal (0); `None` si ninguna.
/// Original: `Player_ActionSelect_0330d0` (C = hay acción, d1 = cuál).
pub fn action_select(m: &mut impl Memory, t: u32) -> Option<u8> {
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY != 0 {
        return None;
    }
    let e = Entity::from_raw(t);
    if input::pressed(m, e, Buttons::DOWN, Edge::Held) {
        return action_fire_down(m, t);
    }
    if !controls::shot_limit_reached(m, e) && input::pressed(m, e, Buttons::A, Edge::Pressed) {
        return match melee_or_shot(m, t) {
            Some(true) => Some(0xFF),
            Some(false) => Some(controls::shot_action_for_aim(m, e)),
            None => None,
        };
    }
    grenade(m, e)
}

/// Como [`action_select`] pero sólo con abajo mantenido.
/// Original: `Player_ActionSelect_B_0331ae`.
pub fn action_select_b(m: &mut impl Memory, t: u32) -> Option<u8> {
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY != 0 {
        return None;
    }
    if !input::pressed(m, Entity::from_raw(t), Buttons::DOWN, Edge::Held) {
        return None;
    }
    action_fire_down(m, t)
}

/// Con abajo mantenido: granada o, si no, cuchillo o disparo hacia abajo.
/// Original: `Player_ActionSelect_B_Fire_0331ce`.
pub fn action_fire_down(m: &mut impl Memory, t: u32) -> Option<u8> {
    let e = Entity::from_raw(t);
    if let Some(g) = grenade(m, e) {
        return Some(g);
    }
    if controls::shot_limit_reached(m, e) || !input::pressed(m, e, Buttons::A, Edge::Pressed) {
        return None;
    }
    match melee_or_shot(m, t) {
        Some(true) => Some(0xFF),
        Some(false) => Some(4),
        None => None,
    }
}

fn grenade(m: &mut impl Memory, e: Entity) -> Option<u8> {
    if controls::grenade_limit_reached(m, e) {
        return None;
    }
    input::pressed(m, e, Buttons::C, Edge::Pressed).then_some(3)
}

/// Prueba el cuchillo: `Some(true)` si toca a alguien, `Some(false)` si
/// hay con qué disparar (munición o la pistola) y `None` si no.
fn melee_or_shot(m: &mut impl Memory, t: u32) -> Option<bool> {
    m.write_u32(t + L::ATTACKS, L::MELEE_ATTACKS);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    if m.read_u8(t + L::FLAGS_13) & L::MELEE_HIT != 0 {
        return Some(true);
    }
    let armed = m.read_u16(t + L::AMMO) != 0 || m.read_u8(t + L::WEAPON) == 0;
    armed.then_some(false)
}

/// ¿Se mantiene la dirección hacia la que mira? Con `aim_fix`, sin
/// dirección cuenta la apuntada (1: nunca, 2: sí).
fn ahead(m: &mut impl Memory, t: u32, aim_fix: bool) -> bool {
    let h = input::horizontal_held(m, Entity::from_raw(t));
    let mut side: u8 = if m.read_u8(t + L::FACING) & L::FACING_LEFT != 0 { 2 } else { 1 };
    if aim_fix && h == 0 {
        match m.read_u8(t + L::AIM) {
            1 => side = 0xFF,
            2 => side = 0,
            _ => {}
        }
    }
    h == side
}

/// Acción en el aire: pose del brazo según la acción y si salta o cae.
/// Original: `Player_AirActionSelect_037168`.
pub fn air_action_select(m: &mut impl Memory, t: u32) {
    let Some(action) = action_select(m, t) else { return };
    set_bits(m, t + L::STATUS, L::ARM_BUSY, true);
    set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, true);
    let fall = falling(m, t);
    let (st, ix) = if fall { (L::STATE_FALL, 1) } else { (L::STATE_JUMP, 0) };
    let pick = |jump: (u16, u16), fall_off: (u16, u16)| if fall { fall_off } else { jump };
    let mode = match action {
        0xFF => {
            pose(m, t, pick((0, 2), (0, 7)), st, L::POSE_MELEE);
            m.write_u16(t + L::MODE, 0);
            return;
        }
        3 => {
            // Cayendo, sin corrección por la dirección apuntada.
            let mode = if ahead(m, t, !fall) {
                pose(m, t, pick((3, 0xFFFA), (0, 0)), st, L::POSE_THROW_FWD);
                2
            } else {
                pose(m, t, pick((0, 2), (0, 7)), st, L::POSE_THROW);
                0
            };
            m.write_u16(t + L::MODE, mode);
            return;
        }
        4 => {
            set_attr_bit0(m, t, false);
            pose(m, t, pick((2, 2), (0, 7)), st, L::POSE_SHOOT_DOWN[ix]);
            3
        }
        1 => {
            pose(m, t, pick((0, 2), (0, 6)), st, L::POSE_SHOOT_UP[ix]);
            1
        }
        2 => {
            pose(m, t, pick((6, 0xFFFD), (0, 0)), st, L::POSE_SHOOT_FWD[ix]);
            2
        }
        _ => {
            if ahead(m, t, true) {
                pose(m, t, pick((6, 0xFFFD), (0, 0)), st, L::POSE_SHOOT_FWD[ix]);
                2
            } else {
                pose(m, t, pick((0, 2), (0, 7)), st, L::POSE_SHOOT[ix]);
                if !fall && m.read_u8(t + L::WEAPON) == 0 {
                    set_attr_bit0(m, t, true);
                }
                0
            }
        }
    };
    m.write_u16(t + L::MODE, mode);
    set_bits(m, t + L::STATUS, L::ARM_FIRING, true);
}

/// Cola común en el aire: si se sale de la pantalla por abajo, cae al
/// vacío; después, la publicación del estado. Original: `Player_Air_Tail_036c3a`.
pub fn air_tail(m: &mut impl Machine, t: u32) {
    if m.read_u8(t + L::AIR) & L::IN_AIR != 0 {
        let pit = m.read_u8(L::SCENE) == L::PIT_SCENE;
        let margins = L::PIT_MARGINS[usize::from(pit)];
        if items::out_of_view_seen(m, t, Some(margins)) {
            set_handler(m, t, L::DEATH_PIT);
        }
    }
    publish_state(m, t);
}

/// Si el jugador ha muerto, pasa al estado de muerte (en el aire o en el
/// suelo) y, salvo ahogado, suena su grito.
/// Original: `PlayerRoute_PublishState_033522`.
pub fn publish_state(m: &mut impl Machine, t: u32) {
    if !death_check(m, t) {
        return;
    }
    let r = m.regs();
    let in_air = collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air;
    set_handler(m, t, if in_air { L::DEATH_AIR } else { L::DEATH_GROUND });
    if m.read_u8(t + L::DEATH_KIND) != L::DEATH_DROWN {
        sound::play(m, L::DEATH_SOUNDS[usize::from(t != L::P1)]);
    }
}

/// ¿Ha muerto? Con el modo inmortal de depuración, nunca.
/// Original: `$0334A2` (C = muerto).
pub fn death_check(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(L::DEBUG) & L::DEBUG_IMMORTAL != 0 {
        set_bits(m, t + L::FLAGS_13, L::DROWNED | L::DEAD, false);
        m.write_u16(t + L::FIELD_66, 1);
        return false;
    }
    death_gate(m, t)
}

/// Sin tiempo, muere; en la escena del agua, por encima de cierta altura,
/// el agua lo ahoga. Devuelve si está muerto.
/// Original: `Player_DeathGate_0334c6`.
pub fn death_gate(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u16(L::RANK_TIMER) == 0 {
        set_bits(m, t + L::FLAGS_13, L::DEAD, true);
        m.write_u16(t + L::FIELD_66, 0);
    }
    if m.read_u8(L::SCENE) == L::WATER_SCENE {
        let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
        if (y as i16) < L::WATER_Y && collision::lookup_tile(m, t, x, y).raw == L::WATER_TILE {
            set_bits(m, t + L::FLAGS_13, L::DROWNED | L::DEAD, true);
            m.write_u16(t + L::FIELD_66, 0);
            m.write_u8(t + L::DEATH_KIND, L::DEATH_DROWN);
        }
    }
    m.read_u8(t + L::FLAGS_13) & L::DEAD != 0
}

fn ring_entry(i: u16) -> u32 {
    L::RINGS + L::RING_ENTRIES + u32::from(i)
}

fn ring_next(i: u16) -> u16 {
    let n = i + L::RING_LEN;
    if n >= L::RING_SIZE { 0 } else { n }
}

/// Primera anilla libre ofrecida (los jugadores ven todas; el resto, sólo
/// las de id negativo): su id y si está al alcance.
/// Original: `$08F470` (C = encontrada, d2 = id o negativo si lejos).
pub fn ring_near(m: &mut impl Memory, t: u32) -> Option<(u8, bool)> {
    let player = L::PLAYERS.contains(&t);
    let split = m.read_u16(L::RINGS + L::RING_SPLIT);
    let mut i = m.read_u16(L::RINGS + L::RING_HEAD);
    while i != split {
        let e = ring_entry(i);
        let id = m.read_u8(e + 4);
        if (player || (id as i8) < 0) && m.read_u8(e + 5) == L::RING_FREE {
            let dx = m.read_u16(t + L::X).wrapping_sub(m.read_u16(e)).wrapping_add(L::RING_REACH.0);
            let dy = m.read_u16(t + L::Y).wrapping_sub(m.read_u16(e + 2)).wrapping_add(L::RING_REACH.1);
            let reach = dx < 2 * L::RING_REACH.0 && dy < 2 * L::RING_REACH.1;
            return Some((id, reach));
        }
        i = ring_next(i);
    }
    None
}

/// Coge la anilla `id`: la marca como cogida entre las ofrecidas y la
/// apunta (con quién la tiene) entre las cogidas. Original: `$08F520`.
pub fn take_ring(m: &mut impl Memory, t: u32, id: u8) -> bool {
    let who = L::PLAYERS.iter().position(|&p| p == t).unwrap_or(2) as u8;
    let split = m.read_u16(L::RINGS + L::RING_SPLIT);
    let mut i = m.read_u16(L::RINGS + L::RING_HEAD);
    loop {
        if i == split {
            return false;
        }
        let e = ring_entry(i);
        if m.read_u8(e + 4) == id && m.read_u8(e + 5) == L::RING_FREE {
            m.write_u8(e + 5, L::RING_TAKEN);
            break;
        }
        i = ring_next(i);
    }
    let tail = m.read_u16(L::RINGS + L::RING_TAIL);
    let mut i = split;
    while i != tail {
        let e = ring_entry(i);
        if m.read_u8(e + 4) == id {
            m.write_u8(e + 5, who);
            return true;
        }
        i = ring_next(i);
    }
    let e = ring_entry(tail);
    m.write_u8(e + 4, id);
    m.write_u8(e + 5, who);
    let n = m.read_u16(L::RINGS + L::RING_COUNT);
    if n < L::RING_MAX {
        m.write_u16(L::RINGS + L::RING_TAIL, ring_next(tail));
        m.write_u16(L::RINGS + L::RING_COUNT, n + 1);
    }
    true
}

/// Salto: limpia el brazo, pose de salto (con carrerilla o parado; apuntando
/// abajo, otra), velocidad hacia arriba, gravedad del salto, el contador
/// del salto largo, caja de golpe en el aire y la espera.
/// Original: `Player_JumpStart_036914`.
pub fn jump_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    clear_arm(m, t);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    let (state, script) = if m.read_u16(t + L::VEL_X) != 0 {
        (L::STATE_JUMP, L::JUMP_RUN_SCRIPT)
    } else {
        (L::STATE_FALL, L::KNOCK_SCRIPT)
    };
    restart_pose(m, t, (0, 0), state, script);
    player::dir_nibble(m, t);
    if m.read_u8(t + L::AIM) == L::AIM_DOWN {
        set_attr_bit0(m, t, false);
        m.write_u16(t + L::MODE, L::MODE_DOWN);
        if m.read_u8(t + L::STATE) == L::STATE_FALL {
            restart_pose(m, t, (0, 7), L::STATE_FALL, L::FALL_DOWN_SCRIPT);
        } else {
            restart_pose(m, t, (2, 2), L::STATE_JUMP, L::JUMP_DOWN_SCRIPT);
        }
    }
    m.write_u16(t + L::VEL_Y, L::JUMP_VEL);
    m.write_u16(t + L::ACC_Y, L::JUMP_GRAVITY);
    m.write_u8(t + L::JUMP_TIMER, L::JUMP_TIMER_FRAMES);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_AIR);
    set_handler(m, t, L::JUMP_WAIT);
    jump_wait(m)
}

fn restart_pose(m: &mut impl Memory, t: u32, off: (u16, u16), state: u8, script: u32) {
    pose(m, t, off, state, script);
    m.write_u8(t + L::BYTE_21, 0xFF);
    anim::start(m, t, script);
}

/// Espera del salto: si B sigue mantenido en el frame 4, salto largo; si
/// el contador llega a 0, salto corto (un cuarto de la velocidad). Cada
/// frame, además, el salto en el aire.
/// Original: `Player_JumpStart_036914__L036a36`.
pub fn jump_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u8(t + L::JUMP_TIMER).wrapping_sub(1);
    m.write_u8(t + L::JUMP_TIMER, n);
    if n == L::JUMP_HOLD_FRAME && input::pressed(m, Entity::from_raw(t), Buttons::B, Edge::Held) {
        set_handler(m, t, L::JUMP_AIR);
    }
    if n == 0 {
        let vy = m.read_u16(t + L::VEL_Y) as i16 >> 2;
        m.write_u16(t + L::VEL_Y, vy as u16);
        set_handler(m, t, L::JUMP_AIR);
    }
    jump_air(m)
}

/// Un frame de salto: control horizontal (al pulsar A o C se gira), freno,
/// límites, movimiento con caída y aterrizaje, animación, y al bajar montar
/// el Slug, anilla o pared; si no, la acción en el aire. Con el modo de
/// depuración `DEBUG_FREEZE` no se mueve. Original: `Player_JumpAir_036a70`.
pub fn jump_air(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let turn = a_or_c_pressed(m, t);
    let h = input::horizontal_held(m, Entity::from_raw(t));
    let acc = m.read_u16(L::VEL_TABLE + u32::from(h) * 2);
    m.write_u16(t + L::ACC_X, acc);
    if turn && acc != 0 {
        set_bits(m, t + L::FACING, L::FACING_LEFT, (acc as i16) < 0);
    }
    let vx = m.read_u16(t + L::VEL_X);
    if acc == 0 && vx != 0 {
        let brake = if (vx as i16) < 0 { L::BRAKE } else { L::BRAKE.wrapping_neg() };
        m.write_u16(t + L::ACC_X, brake);
    }
    let vx = m.read_u16(t + L::VEL_X);
    m.write_u16(t + L::VEL_X, clamp(vx, L::MAX_VEL_X));
    let vy = m.read_u16(t + L::VEL_Y);
    if (vy as i16) <= 0 {
        m.write_u16(t + L::VEL_Y, clamp(vy, L::MAX_JUMP_VEL_Y));
    }
    player::frame_common_no_prio(m, t);
    set_bits(m, t + L::AIR, L::IN_AIR, true);
    set_bits(m, t + L::AIR, L::DESCENDING, false);
    let frozen = m.read_u8(L::DEBUG) & L::DEBUG_FREEZE != 0;
    let saved = [L::ACC_Y, L::VEL_X, L::VEL_Y].map(|f| m.read_u16(t + f));
    if frozen {
        for f in [L::VEL_X, L::VEL_Y, L::ACC_Y] {
            m.write_u16(t + f, 0);
        }
    }
    let row = m.regs().d[4] as u16;
    if motion::on_entity_pos(m, t, |m| motion::move_fall(m, t, row)) {
        set_handler(m, t, L::LAND);
        set_bits(m, t + L::AIR, L::IN_AIR | L::DESCENDING, false);
    }
    if player::anim_done(m, t) {
        set_bits(m, t + L::STATUS, L::ARM_FIRING, false);
    }
    anim_state(m, t);
    if frozen {
        for (f, v) in [L::ACC_Y, L::VEL_X, L::VEL_Y].into_iter().zip(saved) {
            m.write_u16(t + f, v);
        }
    }
    if (m.read_u16(t + L::VEL_Y) as i16) <= 0 {
        descend_checks(m, t);
    } else {
        air_action_select(m, t);
    }
    air_tail(m, t);
    Flow::Return
}
