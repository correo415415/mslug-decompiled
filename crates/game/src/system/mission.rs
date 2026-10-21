//! Tarea de la misión (`script::MISSION_TASK`): carga la escena, mueve la
//! cámara siguiendo a los jugadores y va creando los enemigos de la tabla
//! de apariciones de la misión según avanza la cámara.
//!
//! Original: `Attract_State0_Handler_0967FE` … `Attract_State7_Handler_09698A`,
//! `$0969C2`, `$096A0E`, `$096B7E`, `$05DCCE`, `$04498E`.

use super::{scene, scroll, tasks};
use crate::layout::{mission as M, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Arranque de la misión `k` (0–5, 7): carga su escena y pasa al bucle.
pub fn start(m: &mut impl Machine, k: u8) -> Flow {
    let t = m.task_register();
    m.write_u8(t + M::INDEX, k);
    scene::load(m, k);
    m.write_u8(M::SCRIPTED_PAD, 0);
    let spawns = k != M::NO_SPAWNS;
    if spawns {
        m.write_u16(t + M::CURSOR, 0);
    }
    m.write_u32(t + T::HANDLER, M::RUN[usize::from(k)]);
    run(m)
}

/// Bucle de la misión: cámara, apariciones (salvo en la misión 7) y
/// ayudas de depuración. Original: el bucle de cada `Attract_StateN`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (a, b) = scroll::focus(m);
    scroll::follow(m, a, b);
    if m.read_u8(t + M::INDEX) != M::NO_SPAWNS {
        spawn_due(m, t, M::CAMERA_X);
    }
    if m.read_u8(M::DEBUG) & M::DEBUG_HUD != 0 {
        m.call(M::DEBUG_HUD_ROUTINE);
    }
    if m.read_u8(M::DEBUG) & (M::DEBUG_PICK_A | M::DEBUG_PICK_B) != 0 {
        m.call(M::DEBUG_PICK_ROUTINE);
    }
    m.write_u8(M::EDGE_ENABLE, 1);
    Flow::Return
}

/// Tabla de apariciones de la misión `k` (máximo la 7). Original: `$096B7E`.
pub fn spawn_table(m: &mut impl Memory, k: u8) -> u32 {
    let k = u32::from(k.min(M::LAST_TABLE));
    m.read_u32(M::SPAWN_TABLES + k * 4)
}

/// Crea todas las apariciones cuya X ya ha quedado a menos de
/// `SPAWN_AHEAD` del borde izquierdo de la cámara `camera`.
/// Original: `$0969C2` (cámara principal), `$096A0E` (`CAMERA_B`).
pub fn spawn_due(m: &mut impl Machine, t: u32, camera: u32) {
    let k = m.read_u8(t + M::INDEX);
    let table = spawn_table(m, k);
    loop {
        let e = table + u32::from(m.read_u16(t + M::CURSOR));
        if m.read_u16(e) == M::END {
            return;
        }
        let x = m.read_u16(e + M::ENTRY_X);
        let edge = m.read_u16(camera).wrapping_add(M::SPAWN_AHEAD);
        if (x.wrapping_sub(edge) as i16) >= 0 {
            return;
        }
        spawn(m, e);
        let c = m.read_u16(t + M::CURSOR).wrapping_add(M::ENTRY_LEN);
        m.write_u16(t + M::CURSOR, c);
    }
}

/// Crea la aparición `e`: tipo 2 = tarea fija bajo `ENEMY_PARENT`; si no,
/// hija de `SPAWN_PARENT`. La nueva tarea recuerda su entrada en `+$3C`.
/// Original: `$05DCCE` (y `$04498E`).
pub fn spawn(m: &mut impl Machine, e: u32) -> u32 {
    let routine = m.read_u32(e + M::ENTRY_ROUTINE);
    let c = if m.read_u8(e) == M::KIND_FIXED {
        let c = tasks::spawn(m, routine, M::ENEMY_PARENT);
        let f = m.read_u8(c + T::FLAGS);
        m.write_u8(c + T::FLAGS, f | M::FIXED_FLAG);
        c
    } else {
        tasks::spawn(m, routine, M::SPAWN_PARENT)
    };
    m.write_u32(c + M::ENTRY_PTR, e);
    c
}
