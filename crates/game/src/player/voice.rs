//! Voces y jingles ligados al jugador.
//!
//! Reescrito a partir de `asm/player/core_032aa8.s`
//! (`Player_PlayWeaponMusicIfFlag`, `Player_PlayLifeMusic`).

use crate::entity::{Entity, PlayerId};
use crate::layout::{entity::STATUS_WEAPON_CHANGED, rom::WEAPON_VOICE};
use crate::mem::Memory;

/// Salida de sonido (la implementa el sistema de sonido).
pub trait Sound<M: Memory> {
    /// Encola un código de sonido. Original: `InputGuardCall219c` ($2352),
    /// la entrada común de peticiones al driver de sonido.
    fn play(&mut self, m: &mut M, code: u16);
}

/// Código "sin sonido" en las tablas.
pub const NO_SOUND: u16 = 0xFFFF;

/// Si el jugador acaba de cambiar de arma, anuncia el arma nueva con su
/// voz ("Heavy Machine Gun!"...). Devuelve si había cambio pendiente.
/// Original: `Player_PlayWeaponMusicIfFlag`.
pub fn announce_new_weapon<M: Memory>(m: &mut M, snd: &mut impl Sound<M>, p: Entity) -> bool {
    let f = p.status_flags(m);
    if f & STATUS_WEAPON_CHANGED == 0 {
        return false;
    }
    p.set_status_flags(m, f & !STATUS_WEAPON_CHANGED);
    let voice = WEAPON_VOICE.get(usize::from(p.weapon_id(m))).copied().unwrap_or(NO_SOUND);
    if voice != NO_SOUND {
        snd.play(m, voice);
    }
    true
}

/// Sonido de "vida extra" de cada jugador.
pub const EXTRA_LIFE_SOUND: [u16; 2] = [0x1123, 0x1087];

/// Toca el sonido del jugador dueño de `e` (P1 o cualquier otro = P2).
/// Original: `Player_PlayLifeMusic`.
pub fn play_life_sound<M: Memory>(m: &mut M, snd: &mut impl Sound<M>, e: Entity) {
    let p = if e.owner(m) == Entity::player(PlayerId::P1) { PlayerId::P1 } else { PlayerId::P2 };
    snd.play(m, EXTRA_LIFE_SOUND[p.index()]);
}
