//! Manual de juego («cómo jugar»): la pantalla que sale al empezar la
//! misión si la BIOS lo tiene activado. Una tarea con seis fases (la
//! rutina de la tarea cambia de una a otra): prepara la escena y los
//! iconos, sube el brillo de dos canales con una tabla de seno, recorre su
//! guion de pasos, los baja, lanza un fundido y espera a que acabe.
//!
//! Cada frame, además, la cámara sigue a los jugadores (que hacen la
//! demostración) y la tarea anima su sprite.
//!
//! Original: `Anim_State_F1_08C008` … `Anim_State_F6_08C296` y
//! `Anim_ScriptStep_08bc74`.

use super::{anim, hud, palette, scene, scroll, sound, tasks};
use crate::layout::{manual as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// Preparación: sonido, sprite, escena, guion de pasos, tarea hija, tarea
/// del jugador 1 e iconos; después, primera vuelta de la subida.
/// Original: `Anim_State_F1_08C008`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::reset(m);
    sound::play(m, L::SOUND);
    anim::alloc_entry(m, t, L::GFX);
    anim::start(m, t, L::ANIM);
    m.write_u16(t + L::X, L::POS.0);
    m.write_u16(t + L::Y, L::POS.1);
    m.write_u16(t + L::Z, 0);
    m.write_u16(t + L::FIELD_38, L::FIELD_38_INIT);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f | L::FLAGS_12_SET);
    scene::load(m, L::SCENE);

    m.write_u16(t + L::STEP_TIMER, 0);
    m.write_u16(t + L::STEP_VALUE, 0);
    m.write_u32(t + L::STEP_OFFSET, 0);
    m.write_u32(t + L::STEP_SCRIPT, L::STEPS_SCRIPT);
    m.write_u32(t + L::SELF_BLOCK, 0);
    m.write_u32(t + L::SELF_PTR, t + L::SELF_BLOCK);

    let child = tasks::spawn(m, L::CHILD, t);
    m.write_u16(child + L::X, L::CHILD_POS.0);
    m.write_u16(child + L::Y, L::CHILD_POS.1);
    m.write_u8(child + L::CHILD_KIND, L::CHILD_KIND_VALUE);

    m.write_u32(L::PLAYER_TASK + T::HANDLER, L::PLAYER_HANDLER);
    tasks::reset_in_place(m, L::PLAYER_TASK);
    for icon in L::ICONS {
        tasks::spawn(m, icon, t);
    }

    m.write_u8(t + L::LEVEL[0], 0);
    m.write_u8(t + L::LEVEL[1], 0);
    m.write_u16(t + L::PHASE, 0);
    m.write_u32(t + T::HANDLER, L::FADE_IN);
    fade_in(m)
}

/// Valor de la tabla de seno `table` en la fase actual.
fn sine(m: &mut impl Memory, t: u32, table: u32) -> u16 {
    let i = (m.read_u16(t + L::PHASE).wrapping_shl(1) as i16) as i32;
    m.read_u16(table.wrapping_add(i as u32))
}

/// Avanza la fase; `true` si ya ha pasado del final.
fn advance(m: &mut impl Memory, t: u32, step: u16) -> bool {
    let p = m.read_u16(t + L::PHASE).wrapping_add(step);
    m.write_u16(t + L::PHASE, p);
    p as i16 > L::PHASE_END
}

fn clamp_level(v: u16) -> u8 {
    if v as i16 > L::LEVEL_MAX as i16 { L::LEVEL_MAX as u8 } else { v as u8 }
}

/// Sube el canal 1. Original: `$08C10A` (dentro de `Anim_State_F1_08C008`).
pub fn fade_in(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = sine(m, t, L::SINE_UP);
    m.write_u8(t + L::LEVEL[1], clamp_level(v));
    if advance(m, t, 8) {
        m.write_u16(t + L::PHASE, 0);
        m.write_u8(t + L::LEVEL[1], L::LEVEL_MAX as u8);
        m.write_u32(t + T::HANDLER, L::LIGHT_UP);
    }
    frame_tail(m, t, true)
}

/// Sube el canal 0 (la fase no vuelve a 0). Original: `Anim_State_F2_08C15E`.
pub fn light_up(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = sine(m, t, L::SINE_UP);
    m.write_u8(t + L::LEVEL[0], clamp_level(v));
    if advance(m, t, 4) {
        m.write_u8(t + L::LEVEL[0], L::LEVEL_MAX as u8);
        m.write_u32(t + T::HANDLER, L::STEPS);
    }
    frame_tail(m, t, true)
}

/// Recorre el guion de pasos; al acabar borra el texto y pasa a bajar.
/// Original: `Anim_State_F3_08C1AC`.
pub fn steps(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if step_script(m, t) {
        let (at, ch, cols, rows) = L::CLEAR_RECT;
        hud::fill_rect(m, at, ch, cols, rows);
        m.write_u16(t + L::PHASE, 0);
        m.write_u32(t + T::HANDLER, L::DIM);
    }
    frame_tail(m, t, true)
}

/// Un frame del guion de pasos: cuando se acaba el tiempo del paso, carga
/// el siguiente (valor y dos bytes). Devuelve `true` cuando el guion ya
/// había terminado. Original: `Anim_ScriptStep_08bc74`.
pub fn step_script(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u16(t + L::STEP_TIMER) == L::STEP_END {
        return true;
    }
    let left = m.read_u16(t + L::STEP_TIMER).wrapping_sub(1);
    m.write_u16(t + L::STEP_TIMER, left);
    if left as i16 > 0 {
        m.write_u8(t + L::STEP_BYTES[1], 0);
        return false;
    }
    let p = m.read_u32(t + L::STEP_SCRIPT).wrapping_add(m.read_u32(t + L::STEP_OFFSET));
    let frames = m.read_u16(p);
    m.write_u16(t + L::STEP_TIMER, frames);
    if frames == L::STEP_END {
        m.write_u16(t + L::STEP_VALUE, 0);
        m.write_u8(t + L::STEP_BYTES[0], 0);
        m.write_u8(t + L::STEP_BYTES[1], 0);
    } else {
        let v = m.read_u16(p + 2);
        m.write_u16(t + L::STEP_VALUE, v);
        for i in 0..2 {
            let b = m.read_u8(p + 4 + i as u32);
            m.write_u8(t + L::STEP_BYTES[i], b);
        }
        let off = m.read_u32(t + L::STEP_OFFSET).wrapping_add(L::STEP_SIZE);
        m.write_u32(t + L::STEP_OFFSET, off);
    }
    false
}

/// Baja el canal 0. Original: `Anim_State_F4_08C1EA`.
pub fn dim(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = sine(m, t, L::SINE_DOWN).saturating_sub(1);
    m.write_u8(t + L::LEVEL[0], v as u8);
    if advance(m, t, 4) {
        m.write_u8(t + L::LEVEL[0], 0);
        m.write_u16(t + L::PHASE, 0);
        m.write_u32(t + T::HANDLER, L::FADE_OUT);
    }
    frame_tail(m, t, true)
}

/// Baja el canal 1; al acabar, ajuste de sonido, fundido y espera.
/// Original: `Anim_State_F5_08C23A`.
pub fn fade_out(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = sine(m, t, L::SINE_DOWN);
    m.write_u8(t + L::LEVEL[1], v as u8);
    if advance(m, t, 8) {
        sound::setting(m, false, L::END_SETTING);
        palette::spawn_fade(m, 0, t, L::END_FADE);
        m.write_u16(t + L::TIMER, L::END_WAIT);
        m.write_u8(t + L::LEVEL[1], 0);
        m.write_u32(t + T::HANDLER, L::WAIT_END);
    }
    frame_tail(m, t, true)
}

/// Cuenta atrás; al llegar a 0 la escena deja de esperar.
/// Original: `Anim_State_F6_08C296`.
pub fn wait_end(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let left = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, left);
    if left as i16 <= 0 {
        m.write_u8(crate::layout::script::SCENE_BUSY, 0);
    }
    frame_tail(m, t, false)
}

/// Lo que hacen todas las fases al final: la cámara sigue a los jugadores
/// (y corre el guion de scroll) y, salvo en la última, se anima el sprite.
fn frame_tail(m: &mut impl Machine, t: u32, animate: bool) -> Flow {
    let (a, b) = scroll::focus(m);
    scroll::follow(m, a, b);
    if animate {
        anim::step(m, t);
    }
    Flow::Return
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn step_script_walks_records() {
        let mut m = TestRam::default();
        let t = 0x10_0300;
        let s = 0x10_2000;
        m.write_u32(t + L::STEP_SCRIPT, s);
        // Paso de 2 frames con valor 7 y bytes (1, 2); fin.
        for (i, w) in [2u16, 7, 0x0102, 0, L::STEP_END].iter().enumerate() {
            m.write_u16(s + 2 * i as u32, *w);
        }
        m.write_u16(t + L::STEP_TIMER, 1);
        assert!(!step_script(&mut m, t));
        assert_eq!(m.read_u16(t + L::STEP_VALUE), 7);
        assert_eq!(m.read_u8(t + L::STEP_BYTES[1]), 2);
        assert!(!step_script(&mut m, t));
        assert_eq!(m.read_u8(t + L::STEP_BYTES[1]), 0);
        assert!(!step_script(&mut m, t));
        assert_eq!(m.read_u16(t + L::STEP_TIMER), L::STEP_END);
        assert!(step_script(&mut m, t));
    }
}
