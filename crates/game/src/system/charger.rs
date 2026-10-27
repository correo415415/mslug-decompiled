//! Soldado de guardia que embiste (Charger): vigila su zona (una columna
//! de casa y un radio), anda o corre dentro de ella, se planta y golpea al
//! jugador más cercano, se enfada, y al acabar sus golpes se marcha. Y el
//! escaramuzador, que sólo corre hacia delante.
//!
//! Original: `asm/enemies/soldiers/charger_states_04290c.s` y `$0440BC`,
//! `TargetFacingTest_0423AC`, `Charger_TrackTarget_0428C6`,
//! `MeleeGuard_TailDespawn_042576`.

use super::{anim, items, platforms, slug, soldier, tasks};
use crate::layout::{charger as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn set_facing(m: &mut impl Memory, t: u32, right: bool) {
    let f = m.read_u8(t + L::FACING);
    m.write_u8(t + L::FACING, if right { f | 1 } else { f & !1 });
}

/// Velocidad `v` hacia donde mira.
fn set_speed(m: &mut impl Memory, t: u32, v: u16) {
    let v = if m.read_u8(t + L::FACING) & 1 != 0 { v } else { v.wrapping_neg() };
    m.write_u16(t + L::VEL_X, v);
}

/// `p · base / 256`, saturado a un byte.
fn scale(p: u8, base: u16) -> u8 {
    let v = (u32::from(p) * u32::from(base)) >> 8;
    if v >= 0x100 { 0xFF } else { v as u8 }
}

/// Física del frame con la marca de apoyado del Charger.
/// Original: `Charger_TrackTarget_0428C6`.
fn track(m: &mut impl Machine, t: u32) {
    soldier::physics_flag(m, t, L::GROUNDED);
}

/// ¿Sigue dentro de su zona? Original: `Charger_ColumnDeltaTest_042A2A`.
fn in_zone(m: &mut impl Machine, t: u32) -> bool {
    let x = m.read_u16(t + L::X);
    let (col, _) = anim::screen_to_world(m, x, 0);
    let d = col.wrapping_sub(m.read_u16(t + L::HOME)) as i16;
    d.unsigned_abs() < m.read_u16(t + L::RADIUS)
}

/// Se da de baja al salir de la vista. Original: `MeleeGuard_TailDespawn_042576`.
pub fn tail_despawn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Cola de cada estado: muerte y baja.
fn tail(m: &mut impl Machine, t: u32) -> Flow {
    soldier::check_death(m, t);
    tail_despawn(m)
}

/// Alta: suelo, sprite, sombra, zona, temporizadores según la dificultad,
/// mirada hacia el centro de la pantalla y primer estado.
/// Original: `Charger_Init_04290C` (y las islas `$0429FC`..`$042A22`).
pub fn init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    // `scc` tras `$027F60`, que siempre sale con C = 0.
    soldier::on_ground(m, t);
    m.write_u8(t + L::GROUNDED, 0xFF);
    m.write_u32(t + L::TARGET, 0);
    m.write_u8(t + L::ANGRY, 0);
    anim::alloc_entry(m, t, L::GFX);
    anim::set_attr(m, t, L::PRIORITY);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::ATTR_SET);
    tasks::spawn(m, L::SHADOW, t);
    let x = m.read_u16(t + L::X);
    let (home, _) = anim::screen_to_world(m, x, 0);
    m.write_u16(t + L::HOME, home);
    let r = u16::from(m.read_u8(t + L::P_RADIUS)) << 3;
    m.write_u16(t + L::RADIUS, r);
    let base = soldier::by_difficulty(m, t, L::TIMER_TABLE);
    let p = m.read_u8(t + L::P_TIMER);
    m.write_u8(t + L::TIMER, scale(p, base));
    let base = soldier::by_difficulty(m, t, L::RELOAD_TABLE);
    let p = m.read_u8(t + L::P_RELOAD);
    m.write_u8(t + L::P_RELOAD, scale(p, base));
    let f = soldier::probe(m, t);
    m.write_u8(t + L::GROUNDED, if f.in_air { 0 } else { 0xFF });
    let left_half = (m.read_u16(t + L::X) as i16) < L::SCREEN_MID;
    set_facing(m, t, left_half);
    let next = if r == 0 {
        L::IDLE
    } else {
        match m.read_u8(t + L::GAIT) {
            0 => L::WALK_FAST,
            1 => L::WALK_B,
            _ => L::WALK_A,
        }
    };
    set_handler(m, t, next);
    Flow::Return
}

fn walk(m: &mut impl Machine, t: u32, speed: u16) -> Flow {
    set_speed(m, t, speed);
    set_handler(m, t, L::WALK_LOOP);
    walk_loop(m)
}

/// Original: `Charger_WalkA_042A44`.
pub fn walk_a(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_WALK_A);
    walk(m, t, L::WALK_SPEED)
}

/// Enfadado. Original: `Charger_WalkB_042A56`.
pub fn walk_b(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::ANGRY, 0xFF);
    anim::start(m, t, L::S_WALK_B);
    walk(m, t, L::WALK_SPEED)
}

/// Original: `Charger_WalkFast_042A6E`.
pub fn walk_fast(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_WALK_FAST);
    let v = soldier::by_difficulty(m, t, L::SPEED_TABLE);
    walk(m, t, v)
}

/// Anda hasta salir de su zona; si le pegan, se encoge.
/// Original: `Charger_WalkFast_042A6E+$2E`.
pub fn walk_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    track(m, t);
    anim::step(m, t);
    if !in_zone(m, t) {
        set_handler(m, t, L::IDLE);
    }
    if soldier::melee(m, t) {
        set_handler(m, t, L::HURT);
    }
    tail(m, t)
}

/// Parado en su zona. Original: `Charger_IdleState_042ACC`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let s = if m.read_u8(t + L::ANGRY) != 0 { L::S_IDLE_ANGRY } else { L::S_IDLE };
    anim::start(m, t, s);
    slug::clear_flags13(m, t);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::IDLE_LOOP);
    idle_loop(m)
}

/// Se vuelve hacia el jugador más cercano y, cuando se le acaba el
/// temporizador, ataca. Original: `Charger_IdleState_042ACC+$34`
/// (con `TargetFacingTest_0423AC`).
pub fn idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (_, p) = soldier::nearest_player(m, t);
    if let Some(p) = p {
        let left = (m.read_u16(p + L::X).wrapping_sub(m.read_u16(t + L::X)) as i16) < 0;
        let right = m.read_u8(t + L::FACING) & 1 != 0;
        if left == right {
            let f = m.read_u8(t + L::FACING);
            m.write_u8(t + L::FACING, f ^ 1);
        }
    }
    m.write_u32(t + L::TARGET, p.unwrap_or(0));
    track(m, t);
    anim::step(m, t);
    let timer = m.read_u8(t + L::TIMER);
    if timer == 0 {
        if m.read_u32(t + L::TARGET) != 0 {
            set_handler(m, t, L::ATTACK);
        }
    } else {
        m.write_u8(t + L::TIMER, timer - 1);
    }
    if m.read_u8(t + L::ANGRY) == 0 && soldier::melee(m, t) {
        set_handler(m, t, L::HURT);
    }
    tail(m, t)
}

fn hurt_start(m: &mut impl Machine, t: u32, next: u32) {
    m.write_u16(t + L::VEL_X, 0);
    anim::start(m, t, L::S_HURT);
    m.write_u32(t + L::ON_HIT, L::HURT_HANDLER);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    set_handler(m, t, next);
}

/// Encogido por un golpe. Original: `Charger_HurtState_042B5C`.
pub fn hurt(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    hurt_start(m, t, L::HURT_LOOP);
    hurt_loop(m)
}

/// Original: `Charger_HurtState_042B5C+$2C`.
pub fn hurt_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    track(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE);
    }
    tail(m, t)
}

/// Ataque: si el golpe lo pide, antes coge impulso (y luego sigue en
/// [`attack_anim`]); sin golpes, se marcha. Original: `Charger_AttackState_042BA6`.
pub fn attack(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::STRIKES) == 0 {
        return attack_exit(m);
    }
    let windup = m.read_u8(t + L::STRIKE_KIND) >= L::WINDUP_KIND;
    if m.read_u8(t + L::ANGRY) == 0 {
        if windup {
            m.write_u32(t + L::RESUME, L::ATTACK_ANIM);
            return windup_b(m);
        }
    } else if !windup {
        m.write_u32(t + L::RESUME, L::ATTACK_ANIM);
        return windup_a(m);
    }
    attack_anim(m)
}

/// El golpe según su tipo. Original: `Charger_AttackState_042BA6+$40`.
pub fn attack_anim(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let k = usize::from(m.read_u8(t + L::STRIKE_KIND)).min(4);
    if k == 1 {
        m.write_u8(t + L::ANGRY, 0xFF);
    }
    anim::start(m, t, L::S_STRIKES[k]);
    if k == 0 {
        m.write_u8(t + L::ANIM_SLOT, L::STRIKE0_SLOT);
    }
    let target = m.read_u32(t + L::TARGET);
    soldier::pose(m, t, target);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::ATTACK_LOOP);
    attack_loop(m)
}

/// Al acabar el golpe recarga el temporizador y descuenta un golpe.
/// Original: `Charger_AttackState_042BA6+$C6`.
pub fn attack_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let timer = m.read_u8(t + L::TIMER);
    if timer != 0 {
        m.write_u8(t + L::TIMER, timer - 1);
    }
    track(m, t);
    if anim::step(m, t) {
        let r = m.read_u8(t + L::P_RELOAD);
        m.write_u8(t + L::TIMER, r);
        let n = m.read_u8(t + L::STRIKES).wrapping_sub(1);
        m.write_u8(t + L::STRIKES, n);
        set_handler(m, t, if n == 0 { L::ATTACK_EXIT } else { L::IDLE });
    }
    tail(m, t)
}

/// Sin golpes: se marcha (enfadado, tras coger impulso).
/// Original: `Charger_AttackState_042BA6+$10A`.
pub fn attack_exit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::ANGRY) != 0 {
        m.write_u32(t + L::RESUME, L::LEAVE);
        return windup_a(m);
    }
    Flow::Goto(L::LEAVE)
}

fn windup(m: &mut impl Machine, script: u32, next: u32, angry: u8) -> Flow {
    let t = m.task_register();
    anim::start(m, t, script);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, next);
    windup_loop(m, angry)
}

fn windup_loop(m: &mut impl Machine, angry: u8) -> Flow {
    let t = m.task_register();
    track(m, t);
    if anim::step(m, t) {
        let r = m.read_u32(t + L::RESUME);
        set_handler(m, t, r);
        m.write_u8(t + L::ANGRY, angry);
    }
    tail(m, t)
}

/// Impulso enfadado: al acabar se calma. Original: `Charger_WindupA_042CCA`.
pub fn windup_a(m: &mut impl Machine) -> Flow {
    windup(m, L::S_WINDUP_A, L::WINDUP_A_LOOP, 0)
}

/// Original: `Charger_WindupA_042CCA+$16`.
pub fn windup_a_loop(m: &mut impl Machine) -> Flow {
    windup_loop(m, 0)
}

/// Impulso normal: al acabar se enfada. Original: `Charger_WindupB_042D02`.
pub fn windup_b(m: &mut impl Machine) -> Flow {
    windup(m, L::S_WINDUP_B, L::WINDUP_B_LOOP, 0xFF)
}

/// Original: `Charger_WindupB_042D02+$16`.
pub fn windup_b_loop(m: &mut impl Machine) -> Flow {
    windup_loop(m, 0xFF)
}

/// Escaramuzador: mira a la derecha y echa a correr.
/// Original: `Skirmisher_Init_042D3C`.
pub fn skirmisher(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_facing(m, t, true);
    anim::alloc_entry(m, t, L::GFX);
    soldier::on_ground(m, t);
    m.write_u8(t + L::SK_GROUNDED, 0xFF);
    set_handler(m, t, L::SK_RUN);
    skirmisher_run(m)
}

/// Original: `Skirmisher_Run_042D5C`.
pub fn skirmisher_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = soldier::by_difficulty(m, t, L::SK_SPEED_TABLE);
    set_speed(m, t, v);
    anim::start(m, t, L::S_WALK_FAST);
    slug::clear_flags13(m, t);
    set_handler(m, t, L::SK_MAIN);
    skirmisher_main(m)
}

fn sk_physics(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    soldier::physics_flag(m, t, L::SK_GROUNDED);
}

/// Original: `Skirmisher_Main_042D90`.
pub fn skirmisher_main(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sk_physics(m, t);
    if soldier::melee(m, t) {
        set_handler(m, t, L::SK_HURT);
    }
    anim::step(m, t);
    tail(m, t)
}

/// Original: `Skirmisher_Hurt_042DFA`.
pub fn skirmisher_hurt(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    hurt_start(m, t, L::SK_HURT_LOOP);
    skirmisher_hurt_loop(m)
}

/// Original: `Skirmisher_Hurt_042DFA+$2C`.
pub fn skirmisher_hurt_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sk_physics(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::SK_RUN);
    }
    tail(m, t)
}
