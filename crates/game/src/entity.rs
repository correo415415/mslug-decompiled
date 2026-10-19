//! Entidades del juego (jugadores, enemigos, objetos, efectos).

use crate::layout;
use crate::mem::Memory;

/// Jugador 1 o 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerId {
    P1,
    P2,
}

impl PlayerId {
    pub fn index(self) -> usize {
        match self {
            PlayerId::P1 => 0,
            PlayerId::P2 => 1,
        }
    }
    /// El juego original identifica al jugador con 0 / distinto de 0.
    pub fn from_index(i: u8) -> Self {
        if i == 0 { PlayerId::P1 } else { PlayerId::P2 }
    }
}

/// Referencia a una entidad viva (su bloque de estado).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entity(pub(crate) u32);

macro_rules! field {
    ($get:ident, $set:ident, $ty:ty, $off:path, $rd:ident, $wr:ident, $doc:literal) => {
        #[doc = $doc]
        pub fn $get(self, m: &mut impl Memory) -> $ty {
            m.$rd(self.0 + $off) as $ty
        }
        #[doc = $doc]
        pub fn $set(self, m: &mut impl Memory, v: $ty) {
            m.$wr(self.0 + $off, v as _)
        }
    };
}

impl Entity {
    /// Entidad del jugador `p`.
    pub fn player(p: PlayerId) -> Self {
        Entity(layout::PLAYER_ENTITY[p.index()])
    }

    /// Construye la referencia desde la dirección del bloque (sólo para el
    /// puente de verificación con el juego original).
    pub fn from_raw(addr: u32) -> Self {
        Entity(addr)
    }
    pub fn raw(self) -> u32 {
        self.0
    }

    field!(weapon_id, set_weapon_id, u8, layout::entity::WEAPON, read_u8, write_u8, "Índice del arma.");
    field!(bombs, set_bombs, u8, layout::entity::BOMBS, read_u8, write_u8, "Bombas restantes.");
    field!(ammo, set_ammo, u16, layout::entity::AMMO, read_u16, write_u16, "Munición del arma actual.");
    field!(invuln_timer, set_invuln_timer, u8, layout::entity::INVULN_TIMER, read_u8, write_u8,
           "Frames restantes de invulnerabilidad.");
    field!(status_flags, set_status_flags, u8, layout::entity::STATUS_FLAGS, read_u8, write_u8,
           "Flags de estado del jugador.");
    field!(sprite_attr, set_sprite_attr, u16, layout::entity::SPRITE_ATTR, read_u16, write_u16,
           "Atributo de sprite activo.");
    field!(sprite_attr_base, set_sprite_attr_base, u16, layout::entity::SPRITE_ATTR_BASE, read_u16, write_u16,
           "Atributo de sprite normal.");
    field!(item_amount, set_item_amount, u8, layout::entity::ITEM_AMOUNT, read_u8, write_u8,
           "Objeto recogible: cantidad que da.");
    field!(item_amount_alt, set_item_amount_alt, u8, layout::entity::ITEM_AMOUNT_ALT, read_u8, write_u8,
           "Objeto recogible: cantidad alternativa (para el lanzallamas).");

    field!(facing_flags, set_facing_flags, u8, layout::entity::FACING_FLAGS, read_u8, write_u8,
           "Flags de orientación.");
    field!(controller, set_controller, u8, layout::entity::CONTROLLER, read_u8, write_u8,
           "Mando que la controla (1 = P1, 2 = P2).");
    field!(aim_dir, set_aim_dir, u8, layout::entity::AIM_DIR, read_u8, write_u8, "Dirección de apuntado.");
    field!(aim_history, set_aim_history, u8, layout::entity::AIM_HISTORY, read_u8, write_u8,
           "Apuntado anterior (nibble alto) y actual (bajo).");
    field!(live_grenades, set_live_grenades, u8, layout::entity::LIVE_GRENADES, read_u8, write_u8,
           "Granadas propias vivas contadas este frame.");
    field!(live_shots, set_live_shots, u8, layout::entity::LIVE_SHOTS, read_u8, write_u8,
           "Disparos propios vivos contados este frame.");

    /// Mira a la izquierda.
    pub fn facing_left(self, m: &mut impl Memory) -> bool {
        self.facing_flags(m) & layout::entity::FACING_LEFT != 0
    }

    /// Tarea/entidad dueña.
    pub fn owner(self, m: &mut impl Memory) -> Entity {
        Entity(m.read_u32(self.0 + layout::entity::OWNER) & 0xFF_FFFF)
    }

    /// Jugador al que pertenece una entidad auxiliar (disparo, granada...)
    /// según su mando: 1 = P1, cualquier otro = P2.
    pub fn owning_player(self, m: &mut impl Memory) -> PlayerId {
        if self.controller(m) == 1 { PlayerId::P1 } else { PlayerId::P2 }
    }

    /// Lee un word de un campo dado por su offset numérico (tablas de datos
    /// del juego que apuntan a campos de la propia entidad).
    pub fn field_u16(self, m: &mut impl Memory, offset: u16) -> u16 {
        m.read_u16(self.0.wrapping_add(i32::from(offset as i16) as u32) & 0xFF_FFFF)
    }
}
