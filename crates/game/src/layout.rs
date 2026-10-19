//! Disposición del estado del juego original en la RAM de trabajo.
//!
//! **Único fichero del crate con números de la máquina original.** Todo el
//! resto del juego usa nombres (`Entity::ammo`, `Game::player`). Los offsets
//! salen de la decompilación (comentarios de `asm/player/core_032aa8.s`,
//! `asm/kernel/task_alloc_0004ae.s`, `internal/experiments/05_ram_map...`).

/// Inicio de la RAM de trabajo.
pub const RAM_BASE: u32 = 0x10_0000;

/// Bloques fijos de las entidades de los jugadores (P1, P2).
pub const PLAYER_ENTITY: [u32; 2] = [0x10_0440, 0x10_04E0];

/// Campos de una entidad (offset dentro del bloque).
pub mod entity {
    /// Atributo de sprite activo (paleta) — lo cambia el parpadeo.
    pub const SPRITE_ATTR: u32 = 0x14;
    /// Atributo de sprite normal (se restaura al acabar el parpadeo).
    pub const SPRITE_ATTR_BASE: u32 = 0x16;
    /// Arma actual del jugador (ver `player::weapons::Weapon`).
    pub const WEAPON: u32 = 0x71;
    /// Bombas (granadas) restantes.
    pub const BOMBS: u32 = 0x80;
    /// Munición del arma actual.
    pub const AMMO: u32 = 0x82;
    /// Frames restantes de invulnerabilidad/parpadeo.
    pub const INVULN_TIMER: u32 = 0x87;
    /// Flags de estado del jugador.
    pub const STATUS_FLAGS: u32 = 0x8C;
    /// Bit de `STATUS_FLAGS`: el jugador acaba de cambiar de arma (suena la
    /// música del arma).
    pub const STATUS_WEAPON_CHANGED: u8 = 1 << 4;
    /// Parámetros de un objeto recogible: cantidad principal / alternativa.
    pub const ITEM_AMOUNT: u32 = 0x98;
    pub const ITEM_AMOUNT_ALT: u32 = 0x99;
}

/// Tablas de la ROM usadas por el código portado (copiadas como datos).
pub mod rom {
    /// Offsets de campo (dentro de la propia entidad) de los que se toma el
    /// atributo de sprite en cada frame del parpadeo, indexado por
    /// `timer & 15`. Original: `OpcodeOffsetTable_0329EE`.
    pub const BLINK_ATTR_FIELDS: [u16; 16] = [
        0x16, 0x1A, 0x1A, 0x16, 0x18, 0x2D7C, 0xFFFF, 0xFFFF,
        0x48, 0x4E75, 0x3D6E, 0x16, 0x14, 0x08EE, 0x0002, 0x005B,
    ];
}
