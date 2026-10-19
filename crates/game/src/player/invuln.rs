//! Invulnerabilidad del jugador (parpadeo tras recibir daño o reaparecer).
//!
//! Reescrito a partir de `asm/player/core_032aa8.s` (sección
//! "Invulnerabilidad") de la decompilación.

use crate::entity::Entity;
use crate::layout::{entity::STATUS_WEAPON_CHANGED, rom::BLINK_ATTR_FIELDS};
use crate::mem::Memory;

/// Frames de parpadeo al recoger un arma.
pub const PICKUP_BLINK_FRAMES: u8 = 5;

/// Empieza el parpadeo corto de recogida de arma y marca que hay que tocar
/// la música del arma. Original: `Player_StartInvuln5`.
pub fn start_pickup_blink(m: &mut impl Memory, p: Entity) {
    p.set_invuln_timer(m, PICKUP_BLINK_FRAMES);
    let f = p.status_flags(m);
    p.set_status_flags(m, f | STATUS_WEAPON_CHANGED);
}

/// Avanza un frame del parpadeo: mientras dure, el atributo de sprite va
/// alternando según una tabla; al terminar vuelve al normal.
/// Original: `Player_InvulnBlinkStep`.
pub fn blink_step(m: &mut impl Memory, p: Entity) {
    let t = p.invuln_timer(m);
    if t == 0 {
        let base = p.sprite_attr_base(m);
        p.set_sprite_attr(m, base);
        return;
    }
    let t = t - 1;
    p.set_invuln_timer(m, t);
    let field = BLINK_ATTR_FIELDS[usize::from(t & 15)];
    let attr = p.field_u16(m, field);
    p.set_sprite_attr(m, attr);
}
