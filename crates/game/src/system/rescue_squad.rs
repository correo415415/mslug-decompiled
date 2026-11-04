//! Escuadrón de rescate y transporte volador (escena del miniboss): por
//! ahora, los blancos rompibles que sueltan humo al morir (según su tipo)
//! el señuelo inmortal que sólo se mueve con la escena y el transporte
//! volador que sube y baja entre dos alturas, planea y suelta su carga.
//!
//! Original: `asm/enemies/squad/rescue_squad_084836.s`
//! (`TaskHandler_084db0` … `TaskHandler_0850c6`) y los ayudantes del
//! vuelo de `asm/enemies/bosses/m4_carrier_boss_helpers_085134.s`
//! (`Flight_WobbleArm_085ee8` … `Flight_AltitudeCheck_085f60`).

use super::{anim, fort, fortress, items, manual_icons, motion, platforms, rng, sound, spawner, tasks};
use crate::layout::{fort as F, rescue_squad as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn full_size(m: &mut impl Memory, t: u32) {
    m.write_u8(t + F::SHRINK_Y, F::FULL_SIZE);
    m.write_u8(t + F::SHRINK_X, F::FULL_SIZE);
    m.write_u8(t + F::FLIP, 0);
}

/// Blanco rompible: según su tipo (`KIND`, 0–2) elige caja de golpe,
/// humo y márgenes; la vida viene en `HP_BYTE`.
/// Original: `TaskHandler_084db0`.
pub fn target(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    full_size(m, t);
    let k = usize::from(m.read_u8(t + L::KIND).min(2));
    let (hitbox, smoke, margins) = L::TARGETS[k];
    m.write_u32(t + F::HITBOX, hitbox);
    m.write_u32(t + L::SMOKE, smoke);
    m.write_u32(t + L::MARGINS, margins);
    let mut r = m.regs();
    r.a[0] = margins;
    m.set_regs(r);
    let hp = u16::from(m.read_u8(t + L::HP_BYTE));
    m.write_u16(t + F::HP, hp);
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::TARGET_SCRIPT);
    m.write_u32(t + T::HANDLER, L::TARGET_RUN);
    target_run(m)
}

/// Al morir echa su humo `COUNT` veces y se da de baja; también si sale
/// de la vista. Original: `TaskHandler_084db0` (desde `$084E5A`).
pub fn target_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if fort::take_hit(m, t, Some(F::HIT_SOUNDS)) {
        fort::clear_hit(m, t);
    }
    if fort::dead(m, t) {
        let smoke = m.read_u32(t + L::SMOKE);
        while m.read_u8(t + L::COUNT) != 0 {
            spawner::install(m, t, smoke);
            let n = m.read_u8(t + L::COUNT).wrapping_sub(1);
            m.write_u8(t + L::COUNT, n);
        }
        return tasks::finish(m);
    }
    let margins = m.read_u32(t + L::MARGINS);
    if items::out_of_view(m, t, Some(margins)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Señuelo: no se le puede matar, sólo se mueve con la escena hasta
/// salir de la vista. Original: `TaskHandler_084ec2`.
pub fn decoy(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + F::REC_PTR);
    fort::place_from_record(m, t, rec);
    anim::start(m, t, L::DECOY_SCRIPT);
    full_size(m, t);
    m.write_u32(t + T::HANDLER, L::DECOY_RUN);
    decoy_run(m)
}

pub fn decoy_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    let f = m.read_u8(t + F::FLAGS2) & !(F::HIT | F::DEAD);
    m.write_u8(t + F::FLAGS2, f);
    m.write_u16(t + F::HP, L::DECOY_HP);
    let mut r = m.regs();
    r.a[0] = L::DECOY_MARGINS;
    m.set_regs(r);
    if items::out_of_view(m, t, Some(L::DECOY_MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

// ---------------------------------------------------------------------
// Transporte volador
// ---------------------------------------------------------------------

/// `d3`/`d4` con los que entra el original (para el paso con colisión).
fn entry_at(m: &mut impl Machine) -> (u16, u16) {
    let r = m.regs();
    (r.d[3] as u16, r.d[4] as u16)
}

fn probe_move(m: &mut impl Memory, t: u32, at: (u16, u16)) {
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
}

/// Si no se está bamboleando, de vez en cuando empieza a hacerlo.
/// Original: `Flight_WobbleArm_085ee8`.
fn wobble_arm(m: &mut impl Memory, t: u32) {
    if m.read_u16(t + L::TIMER) as i16 > 0 {
        return;
    }
    let (v, _) = rng::next(m);
    if v & L::WOBBLE_MASK == 0 {
        m.write_u16(t + L::TIMER, L::WOBBLE_STEPS);
    }
}

/// Un paso del bamboleo: se mueve un píxel arriba o abajo sin tocar su
/// velocidad. Original: `Flight_WobbleStep_085f08`.
fn wobble_step(m: &mut impl Memory, t: u32, at: (u16, u16)) {
    let n = m.read_u16(t + L::TIMER);
    if n as i16 <= 0 {
        return;
    }
    let n = n.wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    let v = if n & 1 != 0 { L::WOBBLE_VEL } else { L::WOBBLE_VEL.wrapping_neg() };
    let saved = m.read_u16(t + L::VEL_Y);
    m.write_u16(t + L::VEL_Y, v);
    probe_move(m, t, at);
    m.write_u16(t + L::VEL_Y, saved);
}

/// Publica su cubierta como plataforma. Original: `Flight_HitboxParams_085f44`.
fn deck(m: &mut impl Memory, t: u32) {
    let x = m.read_u16(t + F::X).wrapping_sub(L::DECK_DX);
    let y = m.read_u16(t + F::Y);
    platforms::record(m, t, x, y, L::DECK_W);
}

/// ¿Ha llegado a la altura a la que da la vuelta?
/// Original: `Flight_AltitudeCheck_085f60`.
fn at_turn(m: &mut impl Memory, t: u32) -> bool {
    let (x, y) = (m.read_u16(t + F::X), m.read_u16(t + F::Y));
    let (_, wy) = fortress::from_screen(m, x, y);
    let wy = wy as i16;
    let up = (u16::from(m.read_u8(t + L::TURN_UP)) << 4) as i16;
    let down = (u16::from(m.read_u8(t + L::TURN_DOWN)) << 4) as i16;
    if m.read_u16(t + L::VEL_Y) & 0x8000 != 0 {
        down > wy
    } else {
        up >= wy
    }
}

fn leave_if_out(m: &mut impl Machine, t: u32) -> Flow {
    if items::out_of_view(m, t, Some(L::FLYER_MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Alta del transporte: según su altura empieza bajando o subiendo.
/// Original: `TaskHandler_084f26`.
pub fn flyer(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::FLYER_GFX);
    m.write_u16(t + L::TIMER, 0);
    m.write_u16(t + L::ATTR, L::FLYER_ATTR);
    manual_icons::stop_motion(m, t);
    deck(m, t);
    let next = if at_turn(m, t) { L::DESCEND } else { L::CLIMB };
    m.write_u32(t + T::HANDLER, next);
    Flow::Return
}

fn flap(m: &mut impl Machine, script: u32, down: bool) -> u32 {
    let t = m.task_register();
    sound::play(m, L::SND_FLAP);
    anim::start(m, t, script);
    let v = u16::from(m.read_u8(t + L::SPEED)) << 3;
    m.write_u16(t + L::VEL_Y, if down { v.wrapping_neg() } else { v });
    t
}

/// Un frame de vuelo; `turn_when` = si al llegar a la altura de la
/// vuelta (`true`) o al dejar de estar en ella (`false`) pasa a planear.
fn fly(m: &mut impl Machine, turn_when: bool) -> Flow {
    let t = m.task_register();
    let at = entry_at(m);
    probe_move(m, t, at);
    wobble_arm(m, t);
    wobble_step(m, t, at);
    anim::step(m, t);
    deck(m, t);
    if at_turn(m, t) == turn_when {
        m.write_u32(t + T::HANDLER, L::GLIDE);
    }
    leave_if_out(m, t)
}

/// Sube. Original: `TaskHandler_084f5e`.
pub fn climb(m: &mut impl Machine) -> Flow {
    let t = flap(m, L::CLIMB_SCRIPT, false);
    m.write_u32(t + T::HANDLER, L::CLIMB_RUN);
    climb_run(m)
}

pub fn climb_run(m: &mut impl Machine) -> Flow {
    fly(m, true)
}

/// ¿Va la tarea `t` antes que la siguiente en la lista (prioridad menor)?
/// Original: `$0006F0`.
fn before_next(m: &mut impl Memory, t: u32) -> bool {
    let next = m.read_u32(t + T::NEXT);
    m.read_u8(t + T::PRIORITY) < m.read_u8(next + T::PRIORITY)
}

/// Baja; si la cámara aún no ha pasado de `DROP_PROGRESS`, suelta la
/// carga. Original: `TaskHandler_084fca`.
pub fn descend(m: &mut impl Machine) -> Flow {
    let t = flap(m, L::DESCEND_SCRIPT, true);
    if m.read_u16(L::PROGRESS) as i16 <= L::DROP_PROGRESS && !before_next(m, t) {
        let c = tasks::spawn(m, L::CARGO, t);
        anim::copy_sprite_pos(m, t, c);
    }
    m.write_u32(t + T::HANDLER, L::DESCEND_RUN);
    descend_run(m)
}

pub fn descend_run(m: &mut impl Machine) -> Flow {
    fly(m, false)
}

/// Planea unos frames con la escena. Original: `TaskHandler_085062`.
pub fn glide(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_GLIDE);
    anim::start(m, t, L::GLIDE_SCRIPT);
    m.write_u16(t + L::TIMER, L::GLIDE_FRAMES);
    m.write_u32(t + T::HANDLER, L::GLIDE_RUN);
    glide_run(m)
}

pub fn glide_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = entry_at(m);
    platforms::move_with_scene(m, t);
    wobble_step(m, t, at);
    anim::step(m, t);
    deck(m, t);
    if m.read_u16(t + L::TIMER) as i16 <= 0 {
        m.write_u32(t + T::HANDLER, L::HOVER);
    }
    leave_if_out(m, t)
}

/// Se queda quieto y luego vuelve a subir o bajar según su altura.
/// Original: `TaskHandler_0850c6`.
pub fn hover(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::GLIDE_SCRIPT);
    m.write_u16(t + L::TIMER, L::HOVER_FRAMES);
    m.write_u32(t + T::HANDLER, L::HOVER_RUN);
    hover_run(m)
}

pub fn hover_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    deck(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if (n as i16) < 0 {
        let next = if at_turn(m, t) { L::DESCEND } else { L::CLIMB };
        m.write_u32(t + T::HANDLER, next);
    }
    leave_if_out(m, t)
}

/// La carga sólo se instala en su bucle al frame siguiente.
/// Original: `SetTaskHandler_08512c`.
pub fn cargo(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::CARGO_RUN);
    Flow::Return
}
