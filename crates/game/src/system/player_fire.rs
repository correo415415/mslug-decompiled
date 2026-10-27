//! Disparo del jugador: las balas que lanzan los guiones de animación del
//! brazo, una rutina por arma y dirección.
//!
//! Original: `PlayerFire_Pistol_Spawn_03c710` y sus direcciones
//! (`PlayerFire_Pistol_Fwd_03c776` … `PlayerFire_Pistol_DownB_03c854`,
//! `asm/player/fire_shells_03c62a.s`) y las granadas
//! (`Player_ThrowGrenadeBack_033256`, `Player_ThrowGrenade_0332bc`,
//! `asm/player/core_032aa8.s`).

use super::{anim, sound, tasks, turret};
use crate::layout::player_fire as L;
use crate::mem::{Machine, Memory};

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

/// Llamarada del lanzallamas desde `t`: reinicia la ráfaga y, si al
/// jugador le queda munición, gasta una, suena y crea la llama en su
/// posición, un poco más alta. `None` sin munición (el original abandona
/// también la rutina que la llamó).
/// Original: `PlayerFire_Flame_Spawn_03ccd2`.
pub fn flame_spawn(m: &mut impl Machine, t: u32) -> Option<u32> {
    let p = shooter_player(m, t);
    m.write_u8(t + L::BURST, 0);
    let ammo = m.read_u16(p + L::AMMO);
    if ammo == 0 {
        return None;
    }
    m.write_u16(p + L::AMMO, ammo - 1);
    sound::play(m, L::FLAME_SOUND);
    let (b, _) = tasks::spawn_last(m, L::FLAME, t);
    for f in [L::X, L::Y, L::ATTR] {
        let v = m.read_u16(p + f);
        m.write_u16(b + f, v);
    }
    turret::copy_pilot(m, t, b);
    let y = m.read_u16(b + L::Y).wrapping_add(L::FLAME_HEIGHT);
    m.write_u16(b + L::Y, y);
    Some(b)
}

/// Dirección de una llamarada: ángulo (en 1/32 de vuelta), desviación,
/// si sale desplazada hacia delante (las diagonales) y si sale baja
/// (agachado).
#[derive(Clone, Copy)]
pub struct Flame {
    pub angle: u8,
    pub spread: i8,
    pub ahead: bool,
    pub low: bool,
}

/// Llamarada de `t` en la dirección `f`. Devuelve la llama, o `None` sin
/// munición. Original: `PlayerFire_Flame_FwdNeg_03cd3e` … `_DownPos_03d074`
/// (y las variantes `…Low`).
pub fn flame(m: &mut impl Machine, t: u32, f: Flame) -> Option<u32> {
    let b = flame_spawn(m, t)?;
    if f.ahead {
        let left = m.read_u8(t + L::FACING) & L::FACING_LEFT != 0;
        let x = m.read_u16(b + L::X);
        let x = if left { x.wrapping_add(L::FLAME_AHEAD) } else { x.wrapping_sub(L::FLAME_AHEAD) };
        m.write_u16(b + L::X, x);
    }
    m.write_u8(b + L::ANGLE, f.angle << 3);
    let burst = m.read_u8(t + L::BURST);
    m.write_u8(b + L::BULLET_BURST, burst);
    m.write_u8(b + L::SPREAD, f.spread as u8);
    if f.low {
        let y = m.read_u16(b + L::Y).wrapping_sub(L::LOW_DROP);
        m.write_u16(b + L::Y, y);
    }
    Some(b)
}

/// Jugador que lanza la granada. El original compara `t` con el *contenido*
/// de las tareas de los jugadores (su rutina), no con su dirección, así que
/// en la práctica siempre es el padre.
fn grenade_thrower(m: &mut impl Memory, t: u32) -> u32 {
    let [p1, p2] = L::PLAYERS;
    if t == m.read_u32(p1) || t == m.read_u32(p2) {
        t
    } else {
        m.read_u32(t + L::PARENT)
    }
}

/// Granada lanzada desde `t` hacia delante (`back = false`) o hacia atrás:
/// si al jugador le quedan, gasta una y crea la granada en la posición de
/// `t`, mirando hacia donde vaya, y el brazo queda ocupado. Devuelve la
/// granada y lo que deja la reserva de tareas.
/// Original: `Player_ThrowGrenade_0332bc` / `Player_ThrowGrenadeBack_033256`.
pub fn grenade(m: &mut impl Memory, t: u32, back: bool) -> Option<(u32, u32, u8)> {
    let p = grenade_thrower(m, t);
    let bombs = m.read_u8(p + L::BOMBS);
    if bombs == 0 {
        return None;
    }
    m.write_u8(p + L::BOMBS, bombs - 1);
    let (g, after) = tasks::spawn_last(m, L::GRENADE, L::GRENADE_PARENT);
    anim::copy_sprite_pos(m, t, g);
    let facing = m.read_u8(t + L::FACING) ^ if back { L::FACING_LEFT } else { 0 };
    m.write_u8(g + L::FACING, facing);
    turret::copy_pilot(m, t, g);
    let dx = if back { L::GRENADE_DX } else { L::GRENADE_DX.wrapping_neg() };
    let x = m.read_u16(g + L::X).wrapping_add(dx);
    m.write_u16(g + L::X, x);
    let st = m.read_u8(t + L::STATUS) | L::ARM_BUSY;
    m.write_u8(t + L::STATUS, st);
    Some((g, after, facing))
}
