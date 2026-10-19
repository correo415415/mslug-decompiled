//! Controles del jugador: dirección de apuntado, salto y límites de
//! disparos/granadas en pantalla.
//!
//! Reescrito a partir de `asm/player/core_032aa8.s` (secciones de input y
//! selección de acción) de la decompilación.

use crate::entity::{Entity, PlayerId};
use crate::input::{self, Buttons, Edge};
use crate::layout::rom::MAX_LIVE_SHOTS;
use crate::mem::Memory;

/// Dirección de apuntado (valor del campo `AIM_DIR`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aim {
    Forward = 1,
    Down = 2,
    Up = 4,
    /// Hacia abajo en el aire (dispara/lanza hacia el suelo).
    DownAir = 8,
}

/// ¿Se mantiene pulsado "hacia delante" (según hacia dónde mira)?
/// Original: `Input_ForwardByFacing`.
pub fn forward_held(m: &mut impl Memory, e: Entity) -> bool {
    let b = if e.facing_left(m) { Buttons::LEFT } else { Buttons::RIGHT };
    input::pressed(m, e, b, Edge::Held)
}

/// ¿Se mantiene pulsado "hacia atrás"? Original: `Input_BackwardByFacing`.
pub fn backward_held(m: &mut impl Memory, e: Entity) -> bool {
    let b = if e.facing_left(m) { Buttons::RIGHT } else { Buttons::LEFT };
    input::pressed(m, e, b, Edge::Held)
}

/// ¿Abajo mantenido? Original: `Input_DownPressed`.
pub fn down_held(m: &mut impl Memory, e: Entity) -> bool {
    input::pressed(m, e, Buttons::DOWN, Edge::Held)
}

/// Guarda el apuntado del frame y devuelve el historial
/// `(anterior << 4) | actual`.
fn push_aim(m: &mut impl Memory, e: Entity, prev: u8, aim: Aim) -> u8 {
    e.set_aim_dir(m, aim as u8);
    let hist = (prev << 4) | aim as u8;
    e.set_aim_history(m, hist);
    hist
}

/// Apuntado en el aire: arriba gana a abajo; si no, al frente.
/// Original: `Player_ReadDirNibble`.
pub fn read_aim_airborne(m: &mut impl Memory, e: Entity) -> u8 {
    let prev = e.aim_dir(m);
    // El original consulta también "hacia delante", sin efecto en el
    // resultado (en ambos casos apunta al frente).
    let _ = forward_held(m, e);
    let mut aim = Aim::Forward;
    if down_held(m, e) {
        aim = Aim::DownAir;
    }
    if input::pressed(m, e, Buttons::UP, Edge::Held) {
        aim = Aim::Up;
    }
    push_aim(m, e, prev, aim)
}

/// Apuntado en el suelo: arriba o al frente. Original: `Player_ReadFireNibble`.
pub fn read_aim_grounded(m: &mut impl Memory, e: Entity) -> u8 {
    let prev = e.aim_dir(m);
    let aim = if input::pressed(m, e, Buttons::UP, Edge::Held) { Aim::Up } else { Aim::Forward };
    push_aim(m, e, prev, aim)
}

/// Acción de disparo según el apuntado: 1 arriba, 4 abajo en el aire, 2
/// abajo, 0 al frente. Original: `Player_DirNibbleToAction`.
pub fn shot_action_for_aim(m: &mut impl Memory, e: Entity) -> u8 {
    match e.aim_dir(m) {
        4 => 1,
        8 => 4,
        2 => 2,
        _ => 0,
    }
}

/// Con la configuración "saltar con arriba": ¿se ha iniciado un salto?
/// (arriba recién pulsado, o arriba mantenido y una dirección recién
/// pulsada). Original: `Input_JumpOrFire`.
pub fn up_jump_pressed(m: &mut impl Memory, e: Entity) -> bool {
    if input::pressed(m, e, Buttons::UP, Edge::Pressed) {
        return true;
    }
    input::pressed(m, e, Buttons::UP, Edge::Held) && input::pressed(m, e, Buttons::LEFT_OR_RIGHT, Edge::Pressed)
}

/// ¿Sigue el jugador pidiendo salto? (B recién pulsado, o arriba mantenido
/// con la configuración "saltar con arriba"). Devuelve la dirección
/// horizontal del salto (0 vertical, 1 izquierda, 2 derecha).
/// Original: `Input_FireByMode`.
pub fn jump_held(m: &mut impl Memory, e: Entity) -> Option<u8> {
    let want = if input::jump_on_button_b(m) {
        input::pressed(m, e, Buttons::B, Edge::Pressed)
    } else {
        input::pressed(m, e, Buttons::UP, Edge::Held)
    };
    want.then(|| input::horizontal_held(m, e))
}

/// ¿Empieza un salto este frame? Devuelve su dirección horizontal.
/// Original: `Input_JumpByMode`.
pub fn jump_pressed(m: &mut impl Memory, e: Entity) -> Option<u8> {
    let want = if input::jump_on_button_b(m) {
        input::pressed(m, e, Buttons::B, Edge::Pressed)
    } else {
        up_jump_pressed(m, e)
    };
    want.then(|| input::horizontal_held(m, e))
}

/// Granadas en pantalla permitidas a la vez por jugador (más = no lanza).
pub const MAX_LIVE_GRENADES: u8 = 1;

/// Recoge el recuento de granadas vivas de este frame (y lo reinicia);
/// `true` si ya hay demasiadas para lanzar otra.
/// Original: `Player_ConsumeFlag81`.
pub fn grenade_limit_reached(m: &mut impl Memory, p: Entity) -> bool {
    let n = p.live_grenades(m);
    p.set_live_grenades(m, 0);
    n > MAX_LIVE_GRENADES
}

/// Igual con los disparos, con el límite del arma actual.
/// Original: `Player_ConsumeFireFlag84`.
pub fn shot_limit_reached(m: &mut impl Memory, p: Entity) -> bool {
    // La tabla original tiene 6 entradas (5 armas y un relleno).
    let max = MAX_LIVE_SHOTS.get(usize::from(p.weapon_id(m))).copied().unwrap_or(0xFF);
    let n = p.live_shots(m);
    p.set_live_shots(m, 0);
    n > max
}

/// Una granada viva se apunta en el recuento de su jugador.
/// Original: `Player_IncCounterAt81`.
pub fn count_live_grenade(m: &mut impl Memory, grenade: Entity) {
    let p = Entity::player(grenade.owning_player(m));
    let n = p.live_grenades(m).wrapping_add(1);
    p.set_live_grenades(m, n);
}

/// Un disparo vivo se apunta en el recuento de su jugador.
/// Original: `Player_IncCounterAt84`.
pub fn count_live_shot(m: &mut impl Memory, shot: Entity) {
    let p = Entity::player(shot.owning_player(m));
    let n = p.live_shots(m).wrapping_add(1);
    p.set_live_shots(m, n);
}

/// Jugador de una entidad por su bloque.
pub fn player_of_entity(e: Entity) -> Option<PlayerId> {
    [PlayerId::P1, PlayerId::P2].into_iter().find(|&p| Entity::player(p) == e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::input as L;
    use crate::mem::TestRam;

    fn setup() -> (TestRam, Entity) {
        let mut m = TestRam::default();
        let e = Entity::player(PlayerId::P1);
        e.set_controller(&mut m, 1);
        m.write_u32(L::PLAYER_TASK[0] + L::PAD_PTR, L::PAD_LIVE[0]);
        (m, e)
    }

    #[test]
    fn aim() {
        let (mut m, e) = setup();
        e.set_aim_dir(&mut m, 4);
        m.write_u8(L::PAD_LIVE[0] + L::PAD_HELD, Buttons::DOWN.0 | Buttons::UP.0);
        assert_eq!(read_aim_airborne(&mut m, e), 0x44);
        m.write_u8(L::PAD_LIVE[0] + L::PAD_HELD, Buttons::DOWN.0);
        assert_eq!(read_aim_airborne(&mut m, e), 0x48);
        assert_eq!(shot_action_for_aim(&mut m, e), 4);
        assert_eq!(read_aim_grounded(&mut m, e), 0x81);
    }

    #[test]
    fn limits() {
        let (mut m, e) = setup();
        let shot = Entity::from_raw(0x10_2000);
        shot.set_controller(&mut m, 1);
        count_live_grenade(&mut m, shot);
        assert!(!grenade_limit_reached(&mut m, e));
        count_live_grenade(&mut m, shot);
        count_live_grenade(&mut m, shot);
        assert!(grenade_limit_reached(&mut m, e));
        assert_eq!(e.live_grenades(&mut m), 0);
        e.set_weapon_id(&mut m, 1);
        count_live_shot(&mut m, shot);
        assert!(shot_limit_reached(&mut m, e));
    }
}
