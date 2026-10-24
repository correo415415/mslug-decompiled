//! Disparo del jugador: las balas que lanzan los guiones de animación del
//! brazo, una rutina por arma y dirección.
//!
//! Original: `PlayerFire_Pistol_Spawn_03c710` y sus direcciones
//! (`PlayerFire_Pistol_Fwd_03c776` … `PlayerFire_Pistol_DownB_03c854`,
//! `asm/player/fire_shells_03c62a.s`).

use super::{sound, tasks, turret};
use crate::layout::player_fire as L;
use crate::mem::Memory;

/// Dirección de una bala respecto a quien dispara.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aim {
    /// Hacia donde mira.
    Forward,
    /// Hacia donde mira, agachado (la bala sale más baja).
    ForwardLow,
    /// Hacia atrás.
    Back,
    Up,
    Down,
}

/// Jugador del que sale el disparo: `t` si es una tarea de jugador; si
/// no (el brazo), su padre.
pub fn shooter_player(m: &mut impl Memory, t: u32) -> u32 {
    if L::PLAYERS.contains(&t) {
        t
    } else {
        m.read_u32(t + L::PARENT)
    }
}

/// Ángulo de la bala (en 1/32 de vuelta) y el sentido que deja el
/// original en `d1` (0 derecha, 1 arriba, 2 izquierda; abajo, 0). El
/// ángulo no depende de hacia dónde mira: la bala lo corrige con su
/// propio giro.
pub fn angle(m: &mut impl Memory, t: u32, aim: Aim) -> (u8, u8) {
    let left = m.read_u8(t + L::FACING) & L::FACING_LEFT != 0;
    match aim {
        Aim::Up => (L::ANGLE_UP, 1),
        Aim::Down => (L::ANGLE_DOWN, 0),
        Aim::Forward | Aim::ForwardLow => (L::ANGLE_RIGHT, if left { 2 } else { 0 }),
        Aim::Back => (L::ANGLE_LEFT, if left { 0 } else { 2 }),
    }
}

/// Bala de pistola desde `t`: cuenta la ráfaga, gasta munición si le
/// queda, suena y la bala sale de la posición del jugador.
/// Original: `PlayerFire_Pistol_Spawn_03c710`. Devuelve la bala.
pub fn pistol_bullet(m: &mut impl Memory, t: u32) -> u32 {
    let p = shooter_player(m, t);
    let burst = m.read_u8(t + L::BURST).wrapping_add(1);
    m.write_u8(t + L::BURST, burst);
    let ammo = m.read_u16(p + L::AMMO);
    if ammo != 0 {
        m.write_u16(p + L::AMMO, ammo - 1);
    }
    sound::play(m, L::PISTOL_SOUND);
    let (b, _) = tasks::spawn_last(m, L::PISTOL_BULLET, t);
    for f in [L::X, L::Y, L::ATTR] {
        let v = m.read_u16(p + f);
        m.write_u16(b + f, v);
    }
    turret::copy_pilot(m, t, b);
    let y = m.read_u16(b + L::Y).wrapping_add(L::PISTOL_HEIGHT);
    m.write_u16(b + L::Y, y);
    b
}

/// Disparo de pistola de `t` en la dirección `aim`. Devuelve la bala.
/// Original: `PlayerFire_Pistol_Fwd_03c776`, `_Up_03c7aa`, `_Back_03c7c8`,
/// `_FwdLow_03c7fc`, `_Down_03c836` y `_DownB_03c854`.
pub fn pistol(m: &mut impl Memory, t: u32, aim: Aim) -> u32 {
    let b = pistol_bullet(m, t);
    if aim == Aim::ForwardLow {
        let y = m.read_u16(b + L::Y).wrapping_sub(L::LOW_DROP);
        m.write_u16(b + L::Y, y);
    }
    let (a, _) = angle(m, t, aim);
    m.write_u8(b + L::ANGLE, a << 3);
    let burst = m.read_u8(t + L::BURST);
    m.write_u8(b + L::BULLET_BURST, burst);
    m.write_u8(b + L::SPREAD, 0);
    b
}
