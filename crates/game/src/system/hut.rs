//! Ocupante de la choza y su puerta. El ocupante espera dentro hasta que
//! un jugador se acerca (o le rompen la puerta); entonces sale, se asoma
//! y vuelve a entrar mientras la puerta siga en pie. Si la puerta se
//! rompe estando fuera, se convierte en soldado. La puerta se abre cuando
//! el ocupante sale y se rompe con un golpe, dejando una explosión.
//!
//! Original: `HutOccupant_*` y `HutDoor_*` (`$05FA00`–`$05FD5A`,
//! `asm/props/late_props_turrets_05e000.s`), con
//! `HutOccupant_PlayersNear_05fd08` y `Offworld_Box2C168E_05fd40`.

use super::{anim, fort, items, manual_icons, platforms, soldier, sound, tasks};
use crate::layout::{hut as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::OWNER)
}

/// Con la escena y animando; devuelve el flag C del guion.
fn scene_step(m: &mut impl Machine, t: u32) -> bool {
    platforms::move_with_scene(m, t);
    anim::step(m, t)
}

/// Fuera de la vista con los márgenes de la choza.
/// Original: `Offworld_Box2C168E_05fd40` (C = fuera).
pub fn off_view(m: &mut impl Memory, t: u32) -> bool {
    items::out_of_view(m, t, Some(L::MARGINS))
}

/// Algún jugador está a menos de `NEAR_DX` a la izquierda (o a la
/// derecha). Original: `HutOccupant_PlayersNear_05fd08` (C = sí).
pub fn players_near(m: &mut impl Memory, t: u32) -> bool {
    let x = m.read_u16(t + L::X);
    L::PLAYERS.iter().any(|&p| (x.wrapping_sub(m.read_u16(p + L::X)) as i16) < L::NEAR_DX)
}

/// Si le dan, lo marca (y con `stop_sound` corta su sonido).
fn mark_hit(m: &mut impl Memory, t: u32, stop_sound: bool) {
    if fort::hit_side(m, t).is_some() {
        m.write_u8(t + L::STATE, L::STATE_HIT);
        if stop_sound {
            sound::stop(m, L::SND_LOOP);
        }
    }
}

/// Cola del ocupante: muerte y, fuera de la vista, espera a la choza.
fn occupant_tail(m: &mut impl Machine, t: u32) -> Flow {
    soldier::check_death(m, t);
    if off_view(m, t) {
        set_handler(m, t, L::OCC_WAIT);
    }
    Flow::Return
}

fn door_gone(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::DOOR_GONE) == L::GONE
}

/// Alta del ocupante (y de su puerta). Original: `HutOccupant_Init_05fa00`.
pub fn occupant(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::OCC_GFX);
    m.write_u8(t + L::SHRINK_Y, 0xFF);
    m.write_u8(t + L::SHRINK_X, 0xFF);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a.wrapping_add(L::OCC_ATTR_UP));
    m.write_u8(t + L::STATE, 0);
    m.write_u8(t + L::DOOR_GONE, 0);
    m.write_u16(t + L::HP, L::HP_INIT);
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::S_OCC_IDLE);
    let d = tasks::spawn(m, L::DOOR, t);
    anim::copy_sprite_pos(m, t, d);
    set_handler(m, t, L::OCC_IDLE);
    Flow::Return
}

/// Dentro: sale si le rompen la puerta o se acerca un jugador.
/// Original: `HutOccupant_Idle_05fa56`.
pub fn occupant_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene_step(m, t);
    if door_gone(m, t) {
        set_handler(m, t, L::OCC_EMERGE);
    }
    if players_near(m, t) {
        set_handler(m, t, L::OCC_EMERGE);
    }
    mark_hit(m, t, false);
    occupant_tail(m, t)
}

/// Sale de la choza. Original: `HutOccupant_Emerge_05faa6`.
pub fn occupant_emerge(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::S_OCC_EMERGE);
    set_handler(m, t, L::OCC_EMERGE_LOOP);
    occupant_emerge_loop(m)
}

/// Al acabar de salir avisa a la puerta; si ya no hay puerta, se hace
/// soldado (con un efecto de suelo). Original: `HutOccupant_Emerge_05faa6+$18`.
pub fn occupant_emerge_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if scene_step(m, t) {
        m.write_u8(t + L::STATE, L::STATE_OUT);
        set_handler(m, t, L::OCC_OUT);
        if door_gone(m, t) {
            set_handler(m, t, L::SOLDIER);
            let c = tasks::spawn(m, L::GROUND_FX, t);
            anim::copy_sprite_pos(m, t, c);
        }
    }
    mark_hit(m, t, false);
    occupant_tail(m, t)
}

/// Fuera, asomado; si la puerta se rompe, vuelve a salir (y se hace
/// soldado). Original: `HutOccupant_Out_05fb24`.
pub fn occupant_out(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::S_OCC_OUT);
    set_handler(m, t, L::OCC_OUT_LOOP);
    occupant_out_loop(m)
}

/// Original: `HutOccupant_Out_05fb24+$18`.
pub fn occupant_out_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene_step(m, t);
    if door_gone(m, t) {
        set_handler(m, t, L::OCC_EMERGE);
    }
    mark_hit(m, t, true);
    occupant_tail(m, t)
}

/// Fuera de la vista: espera a que la puerta desaparezca para darse de
/// baja. Original: `HutOccupant_WaitParent_05fb88`.
pub fn occupant_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::OCC_WAIT_LOOP);
    occupant_wait_loop(m)
}

/// Original: `HutOccupant_WaitParent_05fb88+$6`.
pub fn occupant_wait_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if door_gone(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Cola de la puerta: se rompe con un golpe y desaparece fuera de la vista.
fn door_tail(m: &mut impl Machine, t: u32) -> Flow {
    if fort::hit_side(m, t).is_some() {
        set_handler(m, t, L::DOOR_BREAK);
    }
    if off_view(m, t) {
        set_handler(m, t, L::DOOR_FREE);
    }
    Flow::Return
}

/// Alta de la puerta. Original: `HutDoor_Init_05fba0`.
pub fn door(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::DOOR_GFX);
    m.write_u8(t + L::WAIT, 0);
    m.write_u8(t + L::SHRINK_Y, 0xFF);
    m.write_u8(t + L::SHRINK_X, 0xFF);
    m.write_u8(t + L::SCRIPT_FLAG, 0);
    m.write_u16(t + L::HP, L::HP_INIT);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a.wrapping_sub(1));
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::S_DOOR);
    set_handler(m, t, L::DOOR_IDLE);
    Flow::Return
}

/// Cerrada; se abre cuando sale el ocupante. Original: `HutDoor_Idle_05fbe6`.
pub fn door_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene_step(m, t);
    let o = owner(m, t);
    if m.read_u8(o + L::STATE) == L::STATE_OUT {
        set_handler(m, t, L::DOOR_OPEN);
    }
    door_tail(m, t)
}

/// Abierta. Original: `HutDoor_Open_05fc26`.
pub fn door_open(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_DOOR_OPEN);
    set_handler(m, t, L::DOOR_OPEN_LOOP);
    door_open_loop(m)
}

/// Si al ocupante le dan, alarga la espera del guion.
/// Original: `HutDoor_Open_05fc26+$12`.
pub fn door_open_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let o = owner(m, t);
    if m.read_u8(o + L::STATE) == L::STATE_HIT {
        m.write_u8(t + L::WAIT, L::WAIT_OPEN);
    }
    anim::step(m, t);
    door_tail(m, t)
}

/// Se rompe: avisa al ocupante, suena y, al acabar el guion, deja una
/// explosión. Original: `HutDoor_Break_05fc78`.
pub fn door_break(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::stop(m, L::SND_LOOP);
    let o = owner(m, t);
    m.write_u8(o + L::DOOR_GONE, L::GONE);
    sound::play(m, L::SND_BREAK);
    anim::start(m, t, L::S_DOOR_BREAK);
    set_handler(m, t, L::DOOR_BREAK_LOOP);
    door_break_loop(m)
}

/// Original: `HutDoor_Break_05fc78+$30`.
pub fn door_break_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if scene_step(m, t) {
        let (c, _) = tasks::spawn_last(m, L::EXPLOSION, t);
        anim::copy_sprite_pos(m, t, c);
        let x = m.read_u16(c + L::X);
        m.write_u16(c + L::X, x.wrapping_sub(L::BREAK_DX));
        set_handler(m, t, L::DOOR_FREE);
    }
    if off_view(m, t) {
        set_handler(m, t, L::DOOR_FREE);
    }
    Flow::Return
}

/// Desaparece avisando al ocupante. Original: `HutDoor_Free_05fce6`.
pub fn door_free(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::stop(m, L::SND_LOOP);
    let o = owner(m, t);
    m.write_u8(o + L::DOOR_GONE, L::GONE);
    set_handler(m, t, L::DOOR_FREE_LOOP);
    tasks::finish(m)
}
