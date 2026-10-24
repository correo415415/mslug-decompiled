//! Lo que hace funcionar la máquina: arranque, interrupciones, planificador
//! de tareas, bucle de frame, cola de sonido, paleta y vídeo.
//!
//! Reescrito a partir de `asm/boot/`, `asm/kernel/` y `asm/video/` de la
//! decompilación. Estas rutinas trabajan con el hardware (registros de
//! vídeo, sonido, perro guardián) y algunas no «vuelven» como una función
//! normal: terminan una interrupción, saltan a la BIOS o ceden el control
//! al planificador. Por eso devuelven un [`Flow`](crate::Flow).

pub mod anim;
pub mod autopilot;
pub mod boot;
pub mod collision;
pub mod events;
pub mod food;
pub mod fort;
pub mod frame;
pub mod game_over;
pub mod go_arrow;
pub mod grunt;
pub mod ground_fx;
pub mod hits;
pub mod hud;
pub mod icons;
pub mod irq;
pub mod items;
pub mod layer;
pub mod lists;
pub mod manual;
pub mod manual_icons;
pub mod mission;
pub mod motion;
pub mod pads;
pub mod pickup;
pub mod platforms;
pub mod player;
pub mod player_air;
pub mod player_ground;
pub mod player_arm;
pub mod player_hud;
pub mod rank;
pub mod palette;
pub mod render;
pub mod rng;
pub mod scene;
pub mod script;
pub mod scroll;
pub mod slope;
pub mod slug;
pub mod slug_idle;
pub mod sprite;
pub mod sound;
pub mod title;
pub mod turret;
pub mod tasks;
pub mod video;
pub mod wheels;

use crate::mem::Memory;

/// Decremento que se queda en 0.
pub(crate) fn dec_to_zero(m: &mut impl Memory, addr: u32) {
    let v = m.read_u8(addr);
    if v != 0 {
        m.write_u8(addr, v - 1);
    }
}

pub(crate) fn inc_u8(m: &mut impl Memory, addr: u32) {
    let v = m.read_u8(addr);
    m.write_u8(addr, v.wrapping_add(1));
}

pub(crate) fn inc_u16(m: &mut impl Memory, addr: u32) {
    let v = m.read_u16(addr);
    m.write_u16(addr, v.wrapping_add(1));
}

pub(crate) fn fill(m: &mut impl Memory, addr: u32, len: u32, v: u8) {
    for i in 0..len {
        m.write_u8(addr + i, v);
    }
}

pub(crate) fn copy(m: &mut impl Memory, src: u32, dst: u32, len: u32) {
    for i in 0..len {
        let b = m.read_u8(src + i);
        m.write_u8(dst + i, b);
    }
}
