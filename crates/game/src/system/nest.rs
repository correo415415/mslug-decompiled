//! Nido de bichos: suelta su enjambre al aparecer y aguanta varios golpes
//! (cada uno con su animación) hasta quedar destrozado. Si su dueño pierde
//! la caja de golpe, el nido cae. Los bichos del enjambre revolotean
//! alrededor del nido (a veces apuntan a él, a veces al azar) y huyen de él
//! cuando el nido avisa.
//!
//! Original: `Nest_Spawn3_08e738` … `Nest_DestroyedWait_08e9d4`, `$08E9FC`,
//! `$08EA16`, `$08EA50` y `Swarmer_*` (`$08EB56`–`$08ECD8`,
//! `asm/enemies/critters_rings_08e4e6.s`).

use super::{anim, banner, collision, critters, fort, grunt, platforms, rng, tasks, trig};
use crate::layout::{nest as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Vuelve a aceptar golpes (borra el bit 3 de `FLAGS2`) con su vida.
fn arm(m: &mut impl Memory, t: u32) {
    let f = m.read_u8(t + T::FLAGS2);
    m.write_u8(t + T::FLAGS2, f & !L::NO_HIT);
    m.write_u16(t + L::HEALTH, L::HEALTH_INIT);
}

/// Alta con 3 bichos y sin dueño que vigilar. Original: `Nest_Spawn3_08e738`.
pub fn spawn3(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::SWARM, L::SWARM_DEFAULT);
    m.write_u16(t + L::FREE_STANDING, 0);
    setup(m, t)
}

/// Alta de la plantilla 185 (vigila a su dueño; el enjambre lo dicen los
/// parámetros). Original: `Nest_Tmpl185_08e746`.
pub fn tmpl185(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    grunt::set_attr(m, t);
    m.write_u16(t + L::FREE_STANDING, 1);
    setup(m, t)
}

/// Gráfico, guion, caja de golpe y el enjambre. Original:
/// `Nest_Tmpl185_08e746+$A`.
fn setup(m: &mut impl Machine, t: u32) -> Flow {
    grunt::facing_from_param(m, t);
    anim::alloc_entry(m, t, L::GFX);
    anim::start(m, t, L::S_IDLE);
    m.write_u32(t + L::HITBOX, L::HITBOX_BOX);
    arm(m, t);
    loop {
        let n = m.read_u8(t + L::SWARM).wrapping_sub(1);
        m.write_u8(t + L::SWARM, n);
        if (n as i8) < 0 {
            break;
        }
        let c = tasks::spawn(m, L::SWARMER, t);
        anim::copy_sprite_pos(m, t, c);
    }
    set_handler(m, t, L::STAGE[0]);
    stage_run(m)
}

/// Cola de cada estado: fuera de la vista se da de baja con sus hijas; si
/// vigila al dueño y éste ya no tiene caja, cae. Original: el final de
/// cada estado (`$08E7C2`…) con `$08EA50`.
fn tail(m: &mut impl Machine, t: u32) -> Flow {
    if critters::out_x(m, t) {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    if m.read_u16(t + L::FREE_STANDING) == 0 {
        let owner = m.read_u32(t + T::OWNER);
        if m.read_u32(owner + L::HITBOX) == L::NO_HITBOX {
            set_handler(m, t, L::FALL_RUN);
        }
    }
    Flow::Return
}

/// Índice del estado de espera según la rutina actual.
fn stage_of(m: &mut impl Memory, t: u32, table: &[u32]) -> usize {
    let h = m.read_u32(t + T::HANDLER);
    table.iter().position(|&x| x == h).unwrap_or(0)
}

/// Espera entera: al recibir un golpe pasa a su animación de golpe.
/// Original: los bucles de `Nest_Tmpl185_08e746`, `Nest_Stage2_08e822`,
/// `Nest_Stage3_08e8b0` y `Nest_Stage4_08e93e`.
pub fn stage_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let k = stage_of(m, t, &L::STAGE_RUN);
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if fort::hit_side(m, t).is_some() {
        set_handler(m, t, L::HIT[k]);
    }
    tail(m, t)
}

/// Original: `Nest_Stage2_08e822` / `Nest_Stage3_08e8b0` / `Nest_Stage4_08e93e`
/// (`k` = 1, 2, 3).
pub fn stage(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    arm(m, t);
    set_handler(m, t, L::STAGE_RUN[k]);
    stage_run(m)
}

/// Golpe `k`: su animación y, al acabar, el estado siguiente.
/// Original: `Nest_Stage1_Hit_08e7de`, `Nest_Stage2_Hit_08e86c`,
/// `Nest_Stage3_Hit_08e8fa`.
pub fn hit(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_HIT[k]);
    set_handler(m, t, L::HIT_RUN[k]);
    hit_run(m)
}

/// Original: los bucles de los golpes (`+$12`).
pub fn hit_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let k = stage_of(m, t, &L::HIT_RUN);
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::AFTER_HIT[k]);
    }
    tail(m, t)
}

/// Destrozado: sin caja de golpe, su animación y queda quieto.
/// Original: `Nest_Destroyed_08e988`.
pub fn destroyed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    anim::start(m, t, L::S_DESTROYED);
    set_handler(m, t, L::DESTROYED_RUN);
    destroyed_run(m)
}

/// Original: `Nest_Destroyed_08e988+$1A`.
pub fn destroyed_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::DESTROYED_WAIT);
    }
    tail(m, t)
}

/// Original: `Nest_DestroyedWait_08e9d4`.
pub fn destroyed_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    tail(m, t)
}

/// Empieza a caer (sin caja de golpe). Original: `$08E9FC`.
pub fn fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    m.write_u16(t + L::VEL_Y, L::FALL_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::FALL_ACC_Y);
    set_handler(m, t, L::FALL_RUN);
    fall_run(m)
}

/// Cae; al tocar suelo o salir de la vista se da de baja con sus hijas.
/// Original: `$08EA16`.
pub fn fall_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if collision::fall(m, t) {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if critters::out_x(m, t) {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Velocidad del vuelo con el ángulo guardado y la rapidez `r`; mira hacia
/// donde va.
fn fly_towards(m: &mut impl Memory, t: u32, r: u16) {
    let a = m.read_u16(t + L::ANGLE);
    let (vx, vy) = trig::polar(m, a, r as i16);
    m.write_u16(t + L::VEL_X, vx as u16);
    m.write_u16(t + L::VEL_Y, vy as u16);
    m.write_u8(t + L::FACING, u8::from((vx as i16) >= 0));
}

/// Ángulo hacia (`dx`, `dy`).
fn aim(m: &mut impl Memory, t: u32, dx: u16, dy: u16) {
    let a = banner::atan2(m, dx, dy);
    m.write_u16(t + L::ANGLE, u16::from(a));
}

/// Bicho del enjambre: gráfico, guion y primer vuelo hacia el nido.
/// Original: `Swarmer_Init_08eb56`.
pub fn swarmer(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    anim::start(m, t, L::S_SWARMER);
    swarmer_aim_parent(m)
}

/// Apunta al nido. Original: `Swarmer_AimParent_08eb6c`.
pub fn swarmer_aim_parent(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = m.read_u32(t + T::OWNER);
    let dx = m.read_u16(o + L::X).wrapping_sub(L::NEST_CENTER.0).wrapping_sub(m.read_u16(t + L::X));
    let dy = m.read_u16(o + L::Y).wrapping_sub(L::NEST_CENTER.1).wrapping_sub(m.read_u16(t + L::Y));
    aim(m, t, dx, dy);
    swarmer_go(m, t)
}

/// Apunta al azar. Original: `Swarmer_AimRandom_08eb96`.
pub fn swarmer_aim_random(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (a, _) = rng::masked(m, 0xFF);
    m.write_u16(t + L::ANGLE, a);
    swarmer_go(m, t)
}

/// Rapidez al azar y unos pocos frames de vuelo. Original:
/// `Swarmer_AimRandom_08eb96+$E`.
fn swarmer_go(m: &mut impl Machine, t: u32) -> Flow {
    let (r, _) = rng::masked(m, 0xFF);
    fly_towards(m, t, r.wrapping_add(L::SWARM_SPEED));
    let (n, _) = rng::masked(m, 3);
    m.write_u16(t + L::TIMER, n + 1);
    set_handler(m, t, L::SWARMER_RUN);
    swarmer_run(m)
}

/// Vuelo: al acabar elige otro rumbo (1 de cada 4 veces hacia el nido);
/// si el nido avisa, huye. Original: `Swarmer_AimRandom_08eb96+$60`.
pub fn swarmer_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    grunt::integrate(m, t);
    platforms::move_with_scene(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n as i16 <= 0 {
        let (r, _) = rng::masked(m, 3);
        set_handler(m, t, if r == 0 { L::SWARMER_AIM_PARENT } else { L::SWARMER_AIM_RANDOM });
    }
    let o = m.read_u32(t + T::OWNER);
    if m.read_u8(o + L::ALARM) != 0 {
        set_handler(m, t, L::SWARMER_FLEE);
    }
    anim::step(m, t);
    grunt::cull(m)
}

/// Huye del nido unos frames y vuelve a apuntarle. Original: `Swarmer_Flee_08ec50`.
pub fn swarmer_flee(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = m.read_u32(t + T::OWNER);
    let dx = m.read_u16(t + L::X).wrapping_add(L::NEST_CENTER.0).wrapping_sub(m.read_u16(o + L::X));
    let dy = m.read_u16(t + L::Y).wrapping_add(L::NEST_CENTER.1).wrapping_sub(m.read_u16(o + L::Y));
    aim(m, t, dx, dy);
    fly_towards(m, t, L::FLEE_SPEED);
    m.write_u16(t + L::TIMER, L::FLEE_FRAMES);
    set_handler(m, t, L::SWARMER_FLEE_RUN);
    swarmer_flee_run(m)
}

/// Original: `Swarmer_Flee_08ec50+$60`.
pub fn swarmer_flee_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    grunt::integrate(m, t);
    platforms::move_with_scene(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n as i16 <= 0 {
        set_handler(m, t, L::SWARMER_AIM_PARENT);
    }
    anim::step(m, t);
    grunt::cull(m)
}
