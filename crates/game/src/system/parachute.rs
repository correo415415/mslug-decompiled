//! Paracaídas con el que reaparece el jugador: la tarea hija que sigue al
//! soldado, elige el fotograma según su velocidad horizontal, se pliega al
//! tocar suelo (o si el soldado deja de bajar en paracaídas) y se va.
//!
//! Original: `$038CF6` … `$038E6A`, `$038E9A` (`asm/player/`) y
//! `$05E4EE`.

use super::{anim, fort, platforms, tasks};
use crate::layout::{parachute as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Machine, h: u32) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, h);
    Flow::Return
}

fn owner(m: &mut impl Machine) -> u32 {
    let t = m.task_register();
    m.read_u32(t + T::OWNER)
}

/// Copia posición y atributos del padre. Original: `$05E4EE`.
pub fn follow_owner(m: &mut impl Memory, t: u32) {
    let o = m.read_u32(t + T::OWNER);
    for f in [L::X, L::Y, L::ATTR] {
        let v = m.read_u16(o + f);
        m.write_u16(t + f, v);
    }
}

/// Fotograma (0–9) según la velocidad horizontal del soldado.
/// Original: `$038E9A`.
pub fn frame_for_speed(m: &mut impl Memory, t: u32) -> u16 {
    let o = m.read_u32(t + T::OWNER);
    let vx = m.read_u16(o + L::VEL_X);
    let d1 = vx.wrapping_add(L::SPEED_BIAS);
    let d0 = L::SPEED_BIAS * 2;
    let d2 = ((d0 as i16) >> 5) as u16;
    let n = d0.wrapping_add(d2).wrapping_sub(d1.wrapping_add(d2)) >> 8;
    if n < L::FRAMES { n } else { n & 7 }
}

/// Arranca la animación de la tabla `table` para el fotograma actual.
fn pick_frame(m: &mut impl Memory, t: u32, table: u32) {
    let k = frame_for_speed(m, t);
    let script = m.read_u32(table + u32::from(k) * 4);
    if script != T::NIL {
        anim::start(m, t, script);
    }
}

/// Alta. Original: `$038CF6`.
pub fn init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    m.write_u32(t + L::LINK, L::NO_LINK);
    m.write_u32(t + T::HANDLER, L::OPEN);
    open(m)
}

/// Abierto, bajando con el soldado. Original: `$038D08`.
pub fn open(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::OPEN_RUN);
    open_run(m)
}

/// Original: `$038D0E`.
pub fn open_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    follow_owner(m, t);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a.wrapping_sub(L::DEPTH_BEHIND));
    pick_frame(m, t, L::OPEN_FRAMES);
    anim::step(m, t);
    if fort::dead(m, t) {
        set_handler(m, L::RELEASE);
    }
    let o = owner(m);
    if m.read_u8(o + L::OWNER_FLAGS) & L::OWNER_SWAY != 0 {
        set_handler(m, L::SWAY);
    }
    if m.read_u8(o + L::OWNER_STATE) != L::STATE_PARACHUTE {
        set_handler(m, L::FOLD);
    }
    if m.read_u8(o + L::OWNER_DEAD) & 1 != 0 {
        set_handler(m, L::RELEASE);
    }
    Flow::Return
}

/// Balanceo. Original: `$038D88`.
pub fn sway(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    pick_frame(m, t, L::SWAY_FRAMES);
    m.write_u32(t + T::HANDLER, L::SWAY_RUN);
    sway_run(m)
}

/// Original: `$038DB0`.
pub fn sway_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    follow_owner(m, t);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a.wrapping_sub(1));
    if anim::step(m, t) {
        set_handler(m, L::OPEN);
    }
    if fort::dead(m, t) {
        set_handler(m, L::RELEASE);
    }
    let o = owner(m);
    // Tal cual el original: aquí se pliega si el estado SIGUE siendo el
    // del paracaídas (la comprobación de `STATE_GONE` nunca se cumple).
    let state = m.read_u8(o + L::OWNER_STATE);
    if state == L::STATE_PARACHUTE {
        set_handler(m, L::FOLD);
        if state == L::STATE_GONE {
            set_handler(m, L::RELEASE);
        }
    }
    if m.read_u8(o + L::OWNER_DEAD) & 1 != 0 {
        set_handler(m, L::RELEASE);
    }
    Flow::Return
}

/// Se pliega al aterrizar. Original: `$038E10`.
pub fn fold(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = owner(m);
    let f = m.read_u8(o + L::FACING) ^ 1;
    m.write_u8(t + L::FACING, f);
    anim::start(m, t, L::FOLD_SCRIPT);
    m.write_u32(t + T::HANDLER, L::FOLD_RUN);
    fold_run(m)
}

/// Original: `$038E32`.
pub fn fold_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    follow_owner(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Se suelta y cae solo. Original: `$038E4A`.
pub fn release(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = owner(m);
    let f = m.read_u8(o + L::FACING);
    m.write_u8(t + L::FACING, f);
    m.write_u8(t + L::BLINK, 0);
    anim::start(m, t, L::RELEASE_SCRIPT);
    m.write_u32(t + T::HANDLER, L::RELEASE_RUN);
    release_run(m)
}

/// Cae con la escena; parpadea si lo pide `BLINK`. Original: `$038E6A`.
pub fn release_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let skip = m.read_u8(t + L::BLINK) != 0 && m.read_u8(crate::layout::system::FRAME_COUNTER) & 1 != 0;
    if !skip && anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
