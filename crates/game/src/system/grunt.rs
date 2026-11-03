//! Soldados rasos del ejército rebelde (los «grunts» de las escenas): la
//! física común (posición en 8.8, orientación y velocidad desde los
//! parámetros de aparición, espera a que el scroll llegue, baja al salir
//! de la vista), el soldado que salta arriba y abajo (plantilla 162), el
//! que espera y elige al azar entre quedarse, mirar o andar, y el salto
//! que dan al ser alcanzados.
//!
//! Original: `asm/enemies/soldiers/grunts_capsule_08d184.s`
//! (`Pos_Integrate*_08d2b0…08d34e`, `Grunt_Tmpl162_08db50`, `Grunt_Hop*`,
//! `Grunt_Rand_*`, `Grunt_Hit_*`, `Grunt_Tmpl_Standing_08de64`) y
//! los ayudantes `$08EFB0…$08F108`.

use super::{anim, human_death, items, motion, platforms, rng, tasks, trig};
use crate::layout::collision as C;
use crate::layout::{grunt as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Suma `vel` (en 1/256 de píxel) a la coordenada `pos` con su fracción
/// `sub`. Devuelve la posición y la fracción nuevas.
pub fn integrate_88(pos: u16, sub: u8, vel: u16) -> (u16, u8) {
    let v = ((u32::from(pos) << 16) | (u32::from(sub) << 8)).wrapping_add(((vel as i16 as i32) << 8) as u32);
    ((v >> 16) as u16, (v >> 8) as u8)
}

/// Un eje: X (`k` = 0) o Y (`k` = 1); guarda posición y fracción.
/// Original: `Pos_IntegrateX88_08d2b0` / `Pos_IntegrateY88_08d2d4`.
pub fn integrate_axis(m: &mut impl Memory, t: u32, k: usize) -> u16 {
    let (pos, sub) = (m.read_u16(t + L::POS[k]), m.read_u8(t + L::SUB[k]));
    let (p, s) = integrate_88(pos, sub, m.read_u16(t + L::VEL[k]));
    m.write_u8(t + L::SUB[k], s);
    m.write_u16(t + L::POS[k], p);
    p
}

/// Mueve en los ejes con velocidad. Original: `Pos_IntegrateXY88_08d2f8`.
pub fn integrate(m: &mut impl Memory, t: u32) {
    for k in 0..2 {
        if m.read_u16(t + L::VEL[k]) != 0 {
            integrate_axis(m, t, k);
        }
    }
}

/// Como [`integrate`] y después velocidad += aceleración.
/// Original: `Pos_IntegrateXY88_Accel_08d34e`.
pub fn integrate_accel(m: &mut impl Memory, t: u32) {
    integrate(m, t);
    for k in 0..2 {
        let v = m.read_u16(t + L::VEL[k]).wrapping_add(m.read_u16(t + L::ACC[k]));
        m.write_u16(t + L::VEL[k], v);
    }
}

/// Si ha salido de la vista, se da de baja con sus hijas.
/// Original: `$08EFB0`.
pub fn cull(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if items::out_of_view(m, t, None) {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    Flow::Return
}

/// ¿Está algún jugador a menos de `d` píxeles a su izquierda (o a su
/// derecha)? Original: `$08EFCE`.
pub fn player_near_x(m: &mut impl Memory, t: u32, d: u16) -> bool {
    let x = m.read_u16(t + L::POS[0]);
    crate::layout::PLAYER_ENTITY
        .iter()
        .any(|&p| (x.wrapping_sub(m.read_u16(p + L::POS[0])) as i16) < d as i16)
}

/// Orientación desde el parámetro `+$98` (bit 0). Original: `$08F002`.
pub fn facing_from_param(m: &mut impl Memory, t: u32) {
    let f = m.read_u8(t + L::PARAM_A) & 1;
    m.write_u8(t + L::PARAM_A, f);
    m.write_u8(t + L::FACING, f);
}

/// Velocidad X = `+$99` × 32 hacia donde mira. Original: `$08F010`.
pub fn vel_x_from_param(m: &mut impl Memory, t: u32) {
    let v = u16::from(m.read_u8(t + L::PARAM_B)) << 5;
    let v = if m.read_u8(t + L::FACING) == 0 { v.wrapping_neg() } else { v };
    m.write_u16(t + L::VEL[0], v);
}

/// Apunta en `+$72` la posición del scroll a la que debe actuar
/// (la actual + `+$99`). Original: `$08F02C`.
pub fn set_trigger(m: &mut impl Memory, t: u32) {
    let v = m.read_u16(L::SCROLL).wrapping_add(u16::from(m.read_u8(t + L::PARAM_B)));
    m.write_u16(t + L::TRIGGER, v);
}

/// ¿Ha llegado el scroll a `+$72`? (nunca si `+$99` es 0).
/// Original: `$08F040`.
pub fn triggered(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::PARAM_B) != 0 && m.read_u16(L::SCROLL) >= m.read_u16(t + L::TRIGGER)
}

/// Gráfico según el tipo `+$9A`. Original: `$08F070`.
pub fn alloc_by_kind(m: &mut impl Memory, t: u32) {
    let g = u16::from(m.read_u8(t + L::KIND)) + L::KIND_GFX_BASE;
    anim::alloc_entry(m, t, g);
}

/// Gráficos según el tipo `+$9A`, tal cual los pide el original (el tipo
/// 0 lo pide dos veces y el 1 pide dos). Original: `$08F084`.
pub fn alloc_a(m: &mut impl Memory, t: u32) {
    let k = m.read_u8(t + L::KIND);
    if k == 0 {
        anim::alloc_entry(m, t, L::GFX_A[0]);
    }
    if k == 1 {
        anim::alloc_entry(m, t, L::GFX_A[1]);
        anim::alloc_entry(m, t, L::GFX_A[2]);
    } else {
        anim::alloc_entry(m, t, L::GFX_A[0]);
    }
}

/// Como [`alloc_a`] con otros gráficos. Original: `$08F0D0`.
pub fn alloc_b(m: &mut impl Memory, t: u32) {
    let k = m.read_u8(t + L::KIND);
    if k == 0 {
        anim::alloc_entry(m, t, L::GFX_B[0]);
    }
    if k == 1 {
        anim::alloc_entry(m, t, L::GFX_B[1]);
    } else {
        anim::alloc_entry(m, t, L::GFX_B[0]);
    }
}

/// Atributos de dibujo de los soldados. Original: `$08F108`.
pub fn set_attr(m: &mut impl Memory, t: u32) {
    m.write_u16(t + L::ATTR, L::ATTR_SOLDIER);
}

/// Plantilla 162: soldado que salta arriba y abajo.
/// Original: `Grunt_Tmpl162_08db50`.
pub fn hopper(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::ATTR, L::ATTR_HOPPER);
    facing_from_param(m, t);
    vel_x_from_param(m, t);
    anim::alloc_entry(m, t, L::HOPPER_GFX);
    anim::start(m, t, L::HOP_DOWN_ANIM);
    m.write_u32(t + T::HANDLER, L::HOP_DOWN);
    hop_down(m)
}

/// Empieza un salto: aceleración, duración al azar y animación.
fn hop(m: &mut impl Machine, acc: u16, anim_script: u32, run: u32) {
    hop_for(m, acc, L::HOP_FRAMES, anim_script, run);
}

/// Como [`hop`] con `frames` + 0..7 frames.
pub(crate) fn hop_for(m: &mut impl Machine, acc: u16, frames: u16, anim_script: u32, run: u32) {
    let t = m.task_register();
    m.write_u16(t + L::ACC[1], acc);
    m.write_u16(t + L::TIMER, frames);
    let (r, _) = rng::masked(m, L::HOP_RANDOM);
    let n = m.read_u16(t + L::TIMER).wrapping_add(r);
    m.write_u16(t + L::TIMER, n);
    anim::start(m, t, anim_script);
    m.write_u32(t + T::HANDLER, run);
}

/// Un frame de salto: al pasar su límite (`past`) o acabarse el tiempo
/// cambia a `next`.
pub(crate) fn hop_run(m: &mut impl Machine, past: fn(i16) -> bool, next: u32) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    integrate_accel(m, t);
    anim::step(m, t);
    if past(m.read_u16(t + L::POS[1]) as i16) {
        m.write_u32(t + T::HANDLER, next);
        return Flow::Return;
    }
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        m.write_u32(t + T::HANDLER, next);
    }
    cull_sub(m)
}

/// [`cull`] llamada como subrutina: si da de baja la tarea, el original
/// sale con la dirección de vuelta aún en la pila (el planificador la
/// restaura después); se reproduce para que la pila coincida.
pub(crate) fn cull_sub(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if items::out_of_view(m, t, None) {
        let sp = m.stack_pointer().wrapping_sub(4);
        m.set_stack_pointer(sp);
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Original: `Grunt_HopDown_08db7a`.
pub fn hop_down(m: &mut impl Machine) -> Flow {
    hop(m, L::HOP_DOWN_ACC, L::HOP_DOWN_ANIM, L::HOP_DOWN_RUN);
    hop_down_run(m)
}

/// Original: `$08DBA6`.
pub fn hop_down_run(m: &mut impl Machine) -> Flow {
    hop_run(m, |y| y >= L::HOP_BOTTOM, L::HOP_UP)
}

/// Original: `Grunt_HopUp_08dbe2`.
pub fn hop_up(m: &mut impl Machine) -> Flow {
    hop(m, L::HOP_UP_ACC, L::HOP_UP_ANIM, L::HOP_UP_RUN);
    hop_up_run(m)
}

/// Original: `$08DC0E`.
pub fn hop_up_run(m: &mut impl Machine) -> Flow {
    hop_run(m, |y| y < L::HOP_TOP, L::HOP_DOWN)
}

// ---------------------------------------------------------------------------
// El que espera y elige al azar, y el salto al ser alcanzado.
// ---------------------------------------------------------------------------

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Sigue a la escena (`$02783A`, deja `a5` en la zona de trabajo).
fn scene(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    let mut r = m.regs();
    r.a[5] = crate::layout::platforms::SCRATCH_BASE;
    m.set_regs(r);
}

/// Paso con colisión desde la posición de `d3`/`d4` (`$027CEE`).
fn move_probe(m: &mut impl Machine, t: u32) {
    let mut r = m.regs();
    r.a[5] = C::SCRATCH_BASE;
    m.set_regs(r);
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
}

/// Elige al azar el siguiente estado (quedarse, mirar o andar).
/// Original: `Grunt_PickRand_2f3712_08dec2`.
fn pick_random(m: &mut impl Machine, t: u32) {
    let (r, _) = rng::masked(m, 3);
    tasks::goto_by_table(m, t, L::RAND_STATES, r, None);
}

/// Aparece en una posición fija y espera a que llegue el scroll.
/// Original: `Grunt_Rand_Idle_08dc4a`.
pub fn rand_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::POS[0], L::RAND_START.0);
    m.write_u16(t + L::POS[1], L::RAND_START.1);
    m.write_u16(t + L::SUB[0], 0);
    rand_idle_placed(m)
}

/// Como [`rand_idle`] pero ya colocado por su registro de aparición (las
/// plantillas entran a mitad de la rutina). Original: `Grunt_Rand_Idle_08dc4a+$10`.
pub fn rand_idle_placed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_attr(m, t);
    facing_from_param(m, t);
    set_trigger(m, t);
    human_death::alloc_regs(m, t, L::HOPPER_GFX);
    anim::start(m, t, L::A_STAND);
    move_probe(m, t);
    anim::step(m, t);
    set_handler(m, t, L::RAND_STAND);
    cull_sub(m)
}

/// Quieto; al acabar la animación, si el jugador está cerca (o, con
/// retardo, si el scroll ha llegado) salta y si no elige otro estado.
/// Original: `Grunt_Rand_Stand_08dc94`.
pub fn rand_stand(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::A_STAND);
    set_handler(m, t, L::RAND_STAND_LOOP);
    rand_stand_loop(m)
}

/// Original: `Grunt_Rand_Stand_08dc94__L08dca6`.
pub fn rand_stand_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    if anim::step(m, t) {
        let jump = if m.read_u8(t + L::PARAM_B) == 0 {
            player_near_x(m, t, L::NEAR_X)
        } else {
            triggered(m, t)
        };
        if jump {
            set_handler(m, t, L::HIT);
        } else {
            pick_random(m, t);
        }
    }
    cull_sub(m)
}

/// Mira a un lado y elige otro estado. Original: `Grunt_Rand_Stand2_08dcfa`.
pub fn rand_stand2(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::A_STAND2);
    set_handler(m, t, L::RAND_STAND2_LOOP);
    rand_stand2_loop(m)
}

/// Original: `Grunt_Rand_Stand2_08dcfa__L08dd0c`.
pub fn rand_stand2_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    if anim::step(m, t) {
        pick_random(m, t);
    }
    cull_sub(m)
}

/// Anda un ciclo de animación hacia un lado al azar.
/// Original: `Grunt_Rand_Walk_08dd26`.
pub fn rand_walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (r, _) = rng::masked(m, 1);
    m.write_u8(t + L::PARAM_A, r as u8);
    let v = if r == 0 { L::WALK_VEL.wrapping_neg() } else { L::WALK_VEL };
    m.write_u16(t + L::VEL[0], v);
    facing_from_param(m, t);
    anim::start(m, t, L::A_WALK);
    set_handler(m, t, L::RAND_WALK_LOOP);
    rand_walk_loop(m)
}

/// Original: `Grunt_Rand_Walk_08dd26__L08dd5e`.
pub fn rand_walk_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    integrate(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::RAND_STAND);
    }
    cull_sub(m)
}

/// Plantilla del que está de pie: salta en cuanto el jugador se acerca.
/// Original: `Grunt_Tmpl_Standing_08de64`.
pub fn standing(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::PARAM_B, L::STANDING_PARAMS.0);
    m.write_u8(t + L::PARAM_A, L::STANDING_PARAMS.1);
    set_attr(m, t);
    facing_from_param(m, t);
    vel_x_from_param(m, t);
    human_death::alloc_regs(m, t, L::HOPPER_GFX);
    set_handler(m, t, L::STANDING_WAIT);
    standing_wait(m)
}

/// Original: `Grunt_Tmpl_Standing_08de64__L08de8c`.
pub fn standing_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::A_STAND);
    set_handler(m, t, L::STANDING_LOOP);
    standing_loop(m)
}

/// Original: `Grunt_Tmpl_Standing_08de64__L08de9e`.
pub fn standing_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    anim::step(m, t);
    if player_near_x(m, t, L::NEAR_X) {
        set_handler(m, t, L::HIT);
    }
    cull_sub(m)
}

/// Alcanzado: baja un poco y decide cuánto volará. Original: `Grunt_Hit_08dd7e`.
pub fn hit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let y = m.read_u16(t + L::POS[1]).wrapping_add(L::HIT_DROP);
    m.write_u16(t + L::POS[1], y);
    m.write_u16(t + L::TIMER, L::HIT_FRAMES.0);
    let (r, _) = rng::masked(m, L::HIT_FRAMES.1);
    let n = m.read_u16(t + L::TIMER).wrapping_add(r);
    m.write_u16(t + L::TIMER, n);
    scene(m, t);
    anim::step(m, t);
    set_handler(m, t, L::HIT_ANGLE_STATE);
    cull_sub(m)
}

/// Ángulo de salida al azar. Original: `Grunt_Hit_Angle_08ddb0`.
pub fn hit_angle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (r, _) = rng::masked(m, L::HIT_ANGLE.1);
    m.write_u16(t + L::ANGLE, r.wrapping_add(L::HIT_ANGLE.0));
    scene(m, t);
    anim::step(m, t);
    set_handler(m, t, L::HIT_LAUNCH);
    cull_sub(m)
}

/// Sale despedido con velocidad al azar. Original: `Grunt_Hit_Launch_08ddda`.
pub fn hit_launch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (r, _) = rng::masked(m, L::HIT_SPEED.1);
    let speed = L::HIT_SPEED.0.wrapping_add(r);
    m.write_u16(t + L::SPEED, speed);
    let angle = m.read_u16(t + L::ANGLE);
    let (vx, vy) = trig::polar(m, angle, speed as i16);
    m.write_u16(t + L::VEL[0], vx as u16);
    m.write_u16(t + L::VEL[1], vy as u16);
    m.write_u8(t + L::FACING, 1);
    anim::start(m, t, L::HOP_DOWN_ANIM);
    scene(m, t);
    anim::step(m, t);
    set_handler(m, t, L::HIT_FLY);
    cull_sub(m)
}

/// Vuela; al acabarse el tiempo pasa a caer dando saltos.
/// Original: `Grunt_Hit_Fly_08de2a`.
pub fn hit_fly(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    integrate(m, t);
    anim::step(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        m.write_u16(t + L::ATTR, L::ATTR_HOPPER);
        let v = m.read_u16(t + L::SPEED);
        m.write_u16(t + L::VEL[0], v);
        m.write_u16(t + L::VEL[1], 0);
        set_handler(m, t, L::HOP_DOWN);
    }
    cull_sub(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrate_carries() {
        assert_eq!(integrate_88(10, 0x80, 0x80), (11, 0));
        assert_eq!(integrate_88(10, 0x00, 0xFF80), (9, 0x80));
        assert_eq!(integrate_88(10, 0x40, 0x0200), (12, 0x40));
    }
}
