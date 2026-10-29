//! Decorados de la misión 1 que se rompen: la farola (suena al recibir un
//! golpe y a veces suelta una chispa que salta y se apaga), la choza (con
//! su ocupante y su tejado; cuando el tejado se viene abajo, estampa los
//! restos en el mapa) y el tejado, que aguanta unos golpes, da puntos a
//! quien lo derriba, cae dañando y se estrella echando escombros.
//!
//! Original: `Prop_Lamp_04d74a`, `Prop_LampIdle_04d782`, `Prop_LampHit_04d7dc`,
//! `Prop_LampSpark_04d81a`, `Prop_Hut_04d8f2`, `Prop_HutWreck_04d996`,
//! `Prop_HutRoof_04d9d6`, `Prop_HutRoofFall_04da52`, `Prop_Roof_04f2c2`,
//! `Prop_RoofFall_04f344`
//! (`asm/enemies/vehicles/turret_car_props_04cbd4.s`) y
//! `Prop_SlotPrioCheckRts_04f2a4`.

use super::{anim, fort, hits, items, manual_icons, motion, platforms, rng, score, scroll, slug, sound, spawner, tasks};
use crate::layout::{scenery as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn add16(m: &mut impl Memory, a: u32, v: u16) {
    let x = m.read_u16(a);
    m.write_u16(a, x.wrapping_add(v));
}

fn clear_hit(m: &mut impl Memory, t: u32) {
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !L::HIT);
}

/// Sin reducción y sin espejo.
fn plain(m: &mut impl Memory, t: u32) {
    m.write_u8(t + L::SHRINK_Y, 0xFF);
    m.write_u8(t + L::SHRINK_X, 0xFF);
    m.write_u8(t + L::FLIP, 0);
}

/// Se coloca según su registro de aparición, con sus gráficos.
fn place(m: &mut impl Memory, t: u32, gfx: u16) {
    let rec = m.read_u32(t + L::SPAWN_RECORD);
    fort::place_from_record(m, t, rec);
    anim::alloc_entry(m, t, gfx);
    plain(m, t);
}

/// Con la escena y animando; se da de baja al salir por la izquierda.
/// Devuelve el flag C del guion.
fn scene_step(m: &mut impl Machine, t: u32) -> bool {
    platforms::move_with_scene(m, t);
    anim::step(m, t)
}

fn leave_if_off_left(m: &mut impl Machine, t: u32) -> Flow {
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Hija `handler` colocada `HUT_CHILD_D` más allá de la choza.
fn hut_child(m: &mut impl Memory, t: u32, handler: u32) {
    let c = tasks::spawn(m, handler, t);
    fort::place_child(m, t, c);
    add16(m, c + L::X, L::HUT_CHILD_D.0);
    add16(m, c + L::Y, L::HUT_CHILD_D.1);
}

/// Farola. Original: `Prop_Lamp_04d74a`.
pub fn lamp(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t, L::LAMP_GFX);
    anim::start(m, t, L::S_LAMP);
    set_handler(m, t, L::LAMP_IDLE);
    lamp_idle(m)
}

/// Quieta; al recibir un golpe suena y, una de cada cuatro veces, suelta
/// una chispa. Original: `Prop_LampIdle_04d782`.
pub fn lamp_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene_step(m, t);
    if fort::hit_side(m, t).is_some() {
        sound::play(m, L::LAMP_SOUND);
        if rng::next(m).0 & L::SPARK_ODDS == 0 {
            let (c, _) = tasks::spawn_last(m, L::SPARK, t);
            anim::copy_sprite_pos(m, t, c);
            add16(m, c + L::Y, L::SPARK_DY);
        }
        set_handler(m, t, L::LAMP_HIT);
    }
    leave_if_off_left(m, t)
}

/// Se balancea por el golpe y vuelve a estar quieta.
/// Original: `Prop_LampHit_04d7dc`.
pub fn lamp_hit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_LAMP_HIT);
    set_handler(m, t, L::LAMP_HIT_LOOP);
    lamp_hit_loop(m)
}

/// Original: `Prop_LampHit_04d7dc+$12`.
pub fn lamp_hit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if scene_step(m, t) {
        clear_hit(m, t);
        set_handler(m, t, L::LAMP_IDLE);
    }
    leave_if_off_left(m, t)
}

/// Velocidad al azar en `0..=mask`, negativa si sale impar.
fn random_speed(m: &mut impl Memory, mask: u16) -> u16 {
    let r = rng::next(m).0 & mask;
    if r & 1 != 0 { r.wrapping_neg() } else { r }
}

/// Chispa de la farola: vida y velocidades al azar.
/// Original: `Prop_LampSpark_04d81a`.
pub fn spark(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::LAMP_GFX);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f | L::FLAGS_12_BIT6);
    plain(m, t);
    manual_icons::stop_motion(m, t);
    let life = L::SPARK_LIFE.wrapping_add(rng::next(m).0 & L::SPARK_LIFE_RND);
    m.write_u16(t + L::LIFE, life);
    let vx = random_speed(m, L::SPARK_VX_RND);
    m.write_u16(t + L::VEL_X, vx);
    let vy = random_speed(m, L::SPARK_VY_RND);
    m.write_u16(t + L::VEL_Y, vy);
    anim::start(m, t, L::S_SPARK);
    set_handler(m, t, L::SPARK_LOOP);
    spark_loop(m)
}

/// Se mueve; al final de su vida parpadea (congela el guion uno de cada
/// dos frames) y se apaga. Original: `Prop_LampSpark_04d81a+$7E`.
pub fn spark_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    let life = m.read_u16(t + L::LIFE);
    if life as i16 <= L::SPARK_BLINK && life & 1 != 0 {
        let w = m.read_u8(t + L::WAIT);
        m.write_u8(t + L::WAIT, w.wrapping_sub(1));
    } else {
        anim::step(m, t);
    }
    let life = life.wrapping_sub(1);
    m.write_u16(t + L::LIFE, life);
    if life as i16 <= 0 || items::out_of_view(m, t, Some(L::SPARK_MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Choza con su ocupante y su tejado. Original: `Prop_Hut_04d8f2`.
pub fn hut(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t, L::HUT_GFX);
    m.write_u8(t + L::STATE, 0);
    hut_child(m, t, L::HUT_OCCUPANT);
    hut_child(m, t, L::ROOF);
    anim::start(m, t, L::S_HUT);
    set_handler(m, t, L::HUT_LOOP);
    hut_loop(m)
}

/// Espera a que el tejado se venga abajo. Original: `Prop_Hut_04d8f2+$78`.
pub fn hut_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene_step(m, t);
    if m.read_u8(t + L::STATE) == L::STATE_WRECKED {
        set_handler(m, t, L::HUT_WRECK);
    }
    leave_if_off_left(m, t)
}

/// Estampa los restos en el mapa y pasa al guion de choza rota.
/// Original: `Prop_HutWreck_04d996`.
pub fn hut_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    scroll::blit_cells(m, t, L::HUT_WRECK_CELLS);
    anim::start(m, t, L::S_HUT_WRECK);
    set_handler(m, t, L::HUT_WRECK_LOOP);
    hut_wreck_loop(m)
}

/// Original: `Prop_HutWreck_04d996+$24`.
pub fn hut_wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene_step(m, t);
    leave_if_off_left(m, t)
}

/// Tejado de la choza o del fuerte: aguanta `ROOF_HP` y pasa a `next`.
fn roof_with(m: &mut impl Machine, fort_roof: bool) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::ROOF_GFX);
    plain(m, t);
    if fort_roof {
        m.write_u16(t + L::MARGIN, L::FORT_ROOF_MARGIN);
    }
    m.write_u16(t + L::HP, L::ROOF_HP);
    anim::start(m, t, L::S_ROOF);
    if fort_roof {
        set_handler(m, t, L::FORT_ROOF_LOOP);
        fort_roof_loop(m)
    } else {
        set_handler(m, t, L::ROOF_LOOP);
        roof_loop(m)
    }
}

/// Encaja golpes (con su sonido) hasta que se rompe.
fn roof_loop_with(m: &mut impl Machine, fall: u32) -> Flow {
    let t = m.task_register();
    scene_step(m, t);
    if fort::hit_side(m, t).is_some() {
        fort::hit_sound(m, t, L::HIT_SOUNDS);
        clear_hit(m, t);
    }
    if fort::dead(m, t) {
        set_handler(m, t, fall);
    }
    leave_if_off_left(m, t)
}

/// Empieza a caer dañando.
fn roof_fall_with(m: &mut impl Machine, next: u32) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::ATTACKS, L::ROOF_ATTACKS);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    anim::start(m, t, L::S_ROOF_FALL);
    set_handler(m, t, next);
    Flow::Return
}

/// Al acabar la caída deja de dañar, suena, (avisa a la choza,) echa los
/// escombros y espera a sus hijas.
fn roof_fall_loop_with(m: &mut impl Machine, notify_owner: bool) -> Flow {
    let t = m.task_register();
    if !scene_step(m, t) {
        return Flow::Return;
    }
    m.write_u32(t + L::ATTACKS, L::NO_ATTACKS);
    slug::clear_flags13(m, t);
    sound::play(m, L::CRASH_SOUND);
    if notify_owner {
        let hut = m.read_u32(t + T::OWNER);
        m.write_u8(hut + L::STATE, L::STATE_WRECKED);
    }
    spawner::install(m, t, L::ROOF_DEBRIS);
    set_handler(m, t, L::WAIT_CHILDREN);
    Flow::Return
}

/// Tejado de la choza. Original: `Prop_HutRoof_04d9d6`.
pub fn roof(m: &mut impl Machine) -> Flow {
    roof_with(m, false)
}

/// Original: `Prop_HutRoof_04d9d6+$34`.
pub fn roof_loop(m: &mut impl Machine) -> Flow {
    roof_loop_with(m, L::ROOF_FALL)
}

/// Da los puntos a quien lo ha roto y cae. Original: `Prop_HutRoofFall_04da52`.
pub fn roof_fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    score::award_attacker(m, t, L::ROOF_POINTS);
    roof_fall_with(m, L::ROOF_FALL_LOOP);
    roof_fall_loop(m)
}

/// Original: `Prop_HutRoofFall_04da52+$3A`.
pub fn roof_fall_loop(m: &mut impl Machine) -> Flow {
    roof_fall_loop_with(m, true)
}

/// Tejado del fuerte (sin puntos y sin choza que avisar).
/// Original: `Prop_Roof_04f2c2`.
pub fn fort_roof(m: &mut impl Machine) -> Flow {
    roof_with(m, true)
}

/// Original: `Prop_Roof_04f2c2+$3A`.
pub fn fort_roof_loop(m: &mut impl Machine) -> Flow {
    roof_loop_with(m, L::FORT_ROOF_FALL)
}

/// Original: `Prop_RoofFall_04f344`.
pub fn fort_roof_fall(m: &mut impl Machine) -> Flow {
    roof_fall_with(m, L::FORT_ROOF_FALL_LOOP);
    fort_roof_fall_loop(m)
}

/// Original: `Prop_RoofFall_04f344+$2E`.
pub fn fort_roof_fall_loop(m: &mut impl Machine) -> Flow {
    roof_fall_loop_with(m, false)
}

/// Con la escena hasta que no le queden hijas; entonces se da de baja.
/// Original: `Prop_SlotPrioCheckRts_04f2a4`.
pub fn wait_children(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let next = m.read_u32(t + T::NEXT);
    if m.read_u8(t + T::PRIORITY) < m.read_u8(next + T::PRIORITY) {
        return Flow::Return;
    }
    tasks::finish(m)
}
