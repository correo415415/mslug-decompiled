//! Blindado de la escena 4: espera fuera de pantalla hasta entrar, aguanta
//! golpes pasando por dos estados de daño (cada uno con su humo y su
//! guion) y, al reventar, pinta su ruina, suelta una caja de explosión y
//! deja los restos. Lleva una torreta hija que copia su posición y cambia
//! de guion según el estado de la madre.
//!
//! Original: `asm/enemies/bosses/fort_scene4_0865be.s`
//! (`ArmoredCar_Init_0877d4` … `ArmoredCar_Turret_087aa0`).

use super::{anim, contact, fort, hits, platforms, scroll, score, slug, soldier, sound, spawner, tasks};
use crate::layout::{armored_car as L, fort as F, task as T};
use crate::mem::{Flow, Machine};

/// Alta: se coloca según su registro, crea la torreta y espera a entrar.
/// Original: `ArmoredCar_Init_0877d4`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + F::REC_PTR);
    fort::place_from_record(m, t, rec);
    anim::alloc_entry(m, t, L::GFX);
    m.write_u8(t + F::SHRINK_Y, F::FULL_SIZE);
    m.write_u8(t + F::SHRINK_X, F::FULL_SIZE);
    m.write_u8(t + F::FLIP, 0);
    m.write_u16(t + F::MARGIN, L::MARGIN);
    m.write_u16(t + F::ATTR, 0);
    anim::start(m, t, L::SCRIPT);
    let c = tasks::spawn(m, L::TURRET, t);
    fort::place_child(m, t, c);
    m.write_u32(t + T::HANDLER, L::WAIT);
    wait(m)
}

/// Mientras esté a la derecha de `ENGAGE_X` sólo se mueve con la escena.
/// Original: `ArmoredCar_Init_0877d4` (desde `$087828`).
pub fn wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if (m.read_u16(t + F::X) as i16) < L::ENGAGE_X {
        return engage(m);
    }
    m.write_u8(L::EDGE_ENABLE, 0);
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    Flow::Return
}

/// Entra en combate: vida según la dificultad y caja de contacto.
/// Original: `ArmoredCar_Engage_087846`.
pub fn engage(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let f = m.read_u8(t + F::FLAGS2) & !(F::HIT | F::DEAD);
    m.write_u8(t + F::FLAGS2, f);
    let hp = soldier::by_difficulty(m, t, L::HP_TABLE).wrapping_add(L::HP_BASE);
    m.write_u16(t + F::HP, hp);
    m.write_u32(t + L::CONTACT_BOX, L::CONTACT);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Lo común de cada frame: se mueve, empuja a los jugadores, anima y
/// suena con los golpes.
fn frame(m: &mut impl Machine, t: u32) {
    m.write_u8(L::EDGE_ENABLE, 0);
    platforms::move_with_scene(m, t);
    contact::touch_players(m, t);
    anim::step(m, t);
    if fort::take_hit(m, t, Some(F::HIT_SOUNDS)) {
        fort::clear_hit(m, t);
    }
}

/// Al bajar la vida de `hp`, echa el humo `smoke` y pasa a `next`.
fn damage_stage(m: &mut impl Machine, t: u32, hp: i16, smoke: u32, next: u32) -> Flow {
    frame(m, t);
    if m.read_u16(t + F::HP) as i16 <= hp {
        spawner::install(m, t, smoke);
        m.write_u32(t + T::HANDLER, next);
    }
    leave_if_off(m, t)
}

fn leave_if_off(m: &mut impl Machine, t: u32) -> Flow {
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Original: `ArmoredCar_Engage_087846` (desde `$087876`).
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    damage_stage(m, t, L::DAMAGED1_HP, L::DAMAGED1_SMOKE, L::DAMAGED1)
}

/// Pasa a un estado de daño: sonido y guion nuevos.
fn damaged(m: &mut impl Machine, script: u32, next: u32) -> u32 {
    let t = m.task_register();
    sound::play(m, L::SND_DAMAGED);
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, next);
    t
}

/// Original: `ArmoredCar_Damaged1_0878d8`.
pub fn damaged1(m: &mut impl Machine) -> Flow {
    damaged(m, L::DAMAGED1_SCRIPT, L::DAMAGED1_RUN);
    damaged1_run(m)
}

pub fn damaged1_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    damage_stage(m, t, L::DAMAGED2_HP, L::DAMAGED2_SMOKE, L::DAMAGED2)
}

/// Original: `ArmoredCar_Damaged2_087956`.
pub fn damaged2(m: &mut impl Machine) -> Flow {
    damaged(m, L::DAMAGED2_SCRIPT, L::DAMAGED2_RUN);
    damaged2_run(m)
}

/// Al morir revienta: sonido, humos, ruina en la capa, caja de la
/// explosión y pasa a restos. Original: `ArmoredCar_Damaged2_087956`
/// (desde `$087972`).
pub fn damaged2_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    frame(m, t);
    if fort::dead(m, t) {
        sound::play(m, L::SND_BLAST);
        for &s in L::BLAST_SMOKE.iter() {
            spawner::install(m, t, s);
        }
        for &cells in L::WRECK_CELLS.iter() {
            scroll::blit_cells(m, t, cells);
        }
        let c = tasks::spawn(m, L::BLAST_BOX, t);
        anim::copy_sprite_pos(m, t, c);
        m.write_u32(t + T::HANDLER, L::WRECK);
    }
    leave_if_off(m, t)
}

/// Restos: puntos para quien lo reventó, sin caja de golpe.
/// Original: `ArmoredCar_Wreck_087a18`.
pub fn wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    score::award_attacker(m, t, L::WRECK_POINTS);
    m.write_u32(t + F::HITBOX, F::NO_HITBOX);
    anim::start(m, t, L::WRECK_SCRIPT);
    m.write_u32(t + T::HANDLER, L::WRECK_RUN);
    wreck_run(m)
}

pub fn wreck_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    hits::run_attack_list(m, t);
    leave_if_off(m, t)
}

/// Caja de la explosión: ataca durante 17 frames y se da de baja.
/// Original: `ArmoredCar_BlastBox_087a62`.
pub fn blast_box(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TIMER, L::BLAST_FRAMES);
    m.write_u32(t + T::HANDLER, L::BLAST_RUN);
    blast_box_run(m)
}

pub fn blast_box_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    m.write_u32(t + L::ATTACKS, L::BLAST_ATTACKS);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if (n as i16) < 0 {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Torreta: profundidad fija y sigue a la madre.
/// Original: `ArmoredCar_Turret_087aa0`.
pub fn turret(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    m.write_u8(t + F::SHRINK_Y, F::FULL_SIZE);
    m.write_u8(t + F::SHRINK_X, F::FULL_SIZE);
    m.write_u8(t + F::FLIP, 0);
    anim::set_attr_at(m, t, L::TURRET_ATTR, L::TURRET_Y);
    m.write_u32(t + T::HANDLER, L::TURRET_RUN);
    turret_run(m)
}

/// Copia la posición de la madre y pone el guion de su estado; si la
/// madre se va, se da de baja.
pub fn turret_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let owner = m.read_u32(t + T::OWNER);
    for off in [F::X, F::Y] {
        let v = m.read_u16(owner + off);
        m.write_u16(t + off, v);
    }
    let state = m.read_u8(owner + F::VARIANT);
    if state == L::TURRET_GONE {
        return tasks::finish(m);
    }
    let script = m.read_u32(L::TURRET_SCRIPTS + u32::from(state) * 4);
    if script != F::NO_ANIM {
        anim::start(m, t, script);
    }
    anim::step(m, t);
    Flow::Return
}
