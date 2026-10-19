//! Armas, munición y bombas del jugador.
//!
//! Reescrito a partir de `asm/player/core_032aa8.s` (sección "Munición /
//! armas") de la decompilación.

use crate::entity::{Entity, PlayerId};
use crate::mem::Memory;

/// Armas del jugador (campo "arma" de la entidad).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Weapon {
    Pistol = 0,
    HeavyMachineGun = 1,
    RocketLauncher = 2,
    Shotgun = 3,
    FlameShot = 4,
}

impl Weapon {
    pub fn from_id(id: u8) -> Option<Self> {
        Some(match id {
            0 => Weapon::Pistol,
            1 => Weapon::HeavyMachineGun,
            2 => Weapon::RocketLauncher,
            3 => Weapon::Shotgun,
            4 => Weapon::FlameShot,
            _ => return None,
        })
    }
}

/// Munición máxima de cualquier arma.
pub const MAX_AMMO: u16 = 999;
/// Bombas máximas.
pub const MAX_BOMBS: u8 = 99;
/// Valor que muestra el HUD como "∞" (pistola).
pub const INFINITE_AMMO: u16 = 0xFFFF;
/// En un objeto, cantidad que significa "no da nada".
pub const ITEM_GIVES_NOTHING: u8 = 0xFF;

/// Munición que da cada arma al recogerla (indexada por id de arma; la
/// pistola no tiene). Original: `Player_WeaponAmmoTbl` (entradas de 4 bytes,
/// se usa el segundo word).
const AMMO_PER_PICKUP: [u16; 5] = [999, 999, 999, 999, 0x1400];

fn ammo_per_pickup(weapon_id: u8) -> u16 {
    // La tabla original sólo tiene entradas para 0..4; para ids mayores el
    // juego leería datos contiguos. No ocurre en la práctica.
    AMMO_PER_PICKUP.get(usize::from(weapon_id)).copied().unwrap_or(0)
}

/// Cambia el arma del jugador y le da munición.
///
/// `amount == 0` significa "la munición por defecto del arma". Si ya llevaba
/// esa arma, la munición se **suma**; si no, se **sustituye**.
/// Original: `Player_SetWeaponAndAmmo` + `Player_SetAmmoForWeapon`.
pub fn set_weapon_and_ammo(m: &mut impl Memory, player: PlayerId, weapon_id: u8, amount: u8) {
    let p = Entity::player(player);
    let amount = if amount == 0 { ammo_per_pickup(weapon_id) } else { u16::from(amount) };
    if p.weapon_id(m) == weapon_id {
        let ammo = p.ammo(m).wrapping_add(amount);
        p.set_ammo(m, ammo);
    } else {
        p.set_ammo(m, amount);
    }
    p.set_weapon_id(m, weapon_id);
}

/// Recarga el arma actual con la munición de una recogida, hasta 999.
/// No hace nada con la pistola. Original: `Player_RefillAmmoClamp999`.
pub fn refill_ammo(m: &mut impl Memory, player: PlayerId) {
    let p = Entity::player(player);
    let w = p.weapon_id(m);
    if w == Weapon::Pistol as u8 {
        return;
    }
    let ammo = clamp_ammo(p.ammo(m).wrapping_add(ammo_per_pickup(w)));
    p.set_ammo(m, ammo);
}

/// Suma con tope en [`MAX_AMMO`] (comparación con signo, como el original).
fn clamp_ammo(v: u16) -> u16 {
    if (v as i16) > MAX_AMMO as i16 { MAX_AMMO } else { v }
}

/// Lo que ocurre visualmente al recoger munición/bombas (lo implementan los
/// sistemas de objetos y puntuación).
pub trait PickupEffects<M: Memory> {
    /// Muestra los puntos flotantes del jugador. Original:
    /// `Score_Popup_ForPlayer`.
    fn score_popup(&mut self, m: &mut M, player: PlayerId);
    /// Crea el indicador de recogida del arma nueva. Original:
    /// `Entity_AllocByPlayerSlot`.
    fn weapon_pickup_marker(&mut self, m: &mut M, player: PlayerId);
}

/// Un objeto de munición (`item`) da su cantidad al jugador.
///
/// El lanzallamas usa la cantidad alternativa del objeto. Con la pistola no
/// se suma munición: sólo aparece el indicador de recogida.
/// Original: `Item_GiveAmmo_ToPlayer`.
pub fn give_ammo_from_item<M: Memory>(
    m: &mut M,
    fx: &mut impl PickupEffects<M>,
    player: PlayerId,
    item: Entity,
) {
    let p = Entity::player(player);
    let w = p.weapon_id(m);
    let amount = if w == Weapon::FlameShot as u8 { item.item_amount_alt(m) } else { item.item_amount(m) };
    if amount == ITEM_GIVES_NOTHING {
        return;
    }
    if w == Weapon::Pistol as u8 {
        fx.weapon_pickup_marker(m, player);
        return;
    }
    let ammo = clamp_ammo(p.ammo(m).wrapping_add(u16::from(amount)));
    p.set_ammo(m, ammo);
    fx.score_popup(m, player);
}

/// Un objeto de bombas da su cantidad al jugador, hasta 99.
/// Original: `Item_GiveBombs_ToPlayer`.
pub fn give_bombs_from_item(m: &mut impl Memory, player: PlayerId, item: Entity) {
    let amount = item.item_amount(m);
    if amount == ITEM_GIVES_NOTHING {
        return;
    }
    let p = Entity::player(player);
    let total = p.bombs(m).wrapping_add(amount);
    // El original compara el word bajo de d0 cuyo byte alto ya es 0 en todas
    // las llamadas: equivale a comparar el byte sin signo.
    p.set_bombs(m, total.min(MAX_BOMBS));
}

/// Munición para el HUD: [`INFINITE_AMMO`] con la pistola.
/// Original: `Player_GetAmmoOrFFFF`.
pub fn ammo_for_hud(m: &mut impl Memory, p: Entity) -> u16 {
    if p.weapon_id(m) == Weapon::Pistol as u8 { INFINITE_AMMO } else { p.ammo(m) }
}

/// Gasta una bomba si quedan; devuelve si se pudo lanzar.
/// Original: `Player_DecBombs`.
pub fn use_bomb(m: &mut impl Memory, p: Entity) -> bool {
    let b = p.bombs(m);
    if b == 0 {
        return false;
    }
    p.set_bombs(m, b - 1);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    struct NoFx;
    impl<M: Memory> PickupEffects<M> for NoFx {
        fn score_popup(&mut self, _: &mut M, _: PlayerId) {}
        fn weapon_pickup_marker(&mut self, _: &mut M, _: PlayerId) {}
    }

    #[test]
    fn pickup_same_weapon_adds() {
        let mut m = TestRam::default();
        let p = Entity::player(PlayerId::P1);
        set_weapon_and_ammo(&mut m, PlayerId::P1, Weapon::HeavyMachineGun as u8, 0);
        assert_eq!(p.ammo(&mut m), 999);
        p.set_ammo(&mut m, 100);
        set_weapon_and_ammo(&mut m, PlayerId::P1, Weapon::HeavyMachineGun as u8, 50);
        assert_eq!(p.ammo(&mut m), 150);
        set_weapon_and_ammo(&mut m, PlayerId::P1, Weapon::Shotgun as u8, 30);
        assert_eq!(p.ammo(&mut m), 30);
        assert_eq!(p.weapon_id(&mut m), Weapon::Shotgun as u8);
    }

    #[test]
    fn bombs_clamp_and_use() {
        let mut m = TestRam::default();
        let p = Entity::player(PlayerId::P2);
        let item = Entity::from_raw(0x10_2000);
        item.set_item_amount(&mut m, 90);
        p.set_bombs(&mut m, 20);
        give_bombs_from_item(&mut m, PlayerId::P2, item);
        assert_eq!(p.bombs(&mut m), 99);
        assert!(use_bomb(&mut m, p));
        assert_eq!(p.bombs(&mut m), 98);
        p.set_bombs(&mut m, 0);
        assert!(!use_bomb(&mut m, p));
    }

    #[test]
    fn ammo_from_item() {
        let mut m = TestRam::default();
        let p = Entity::player(PlayerId::P1);
        let item = Entity::from_raw(0x10_2000);
        p.set_weapon_id(&mut m, Weapon::HeavyMachineGun as u8);
        p.set_ammo(&mut m, 990);
        item.set_item_amount(&mut m, 50);
        give_ammo_from_item(&mut m, &mut NoFx, PlayerId::P1, item);
        assert_eq!(p.ammo(&mut m), 999);
        assert_eq!(ammo_for_hud(&mut m, p), 999);
    }
}
