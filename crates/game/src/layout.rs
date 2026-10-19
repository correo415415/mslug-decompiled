//! Disposición del estado del juego original en la RAM de trabajo.
//!
//! **Único fichero del crate con números de la máquina original.** Todo el
//! resto del juego usa nombres (`Entity::ammo`, `Game::player`). Los offsets
//! salen de la decompilación (comentarios de `asm/player/core_032aa8.s`,
//! `asm/kernel/task_alloc_0004ae.s`, `internal/experiments/05_ram_map...`).

/// Inicio de la RAM de trabajo.
pub const RAM_BASE: u32 = 0x10_0000;

/// Fin (exclusivo) de la RAM de trabajo.
pub const RAM_END: u32 = 0x11_0000;

/// `true` si `addr` está en la RAM de trabajo (los punteros del juego
/// original también pueden apuntar a tablas constantes de la ROM).
pub fn is_ram(addr: u32) -> bool {
    (RAM_BASE..RAM_END).contains(&(addr & 0xFF_FFFF))
}

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
    /// Flags de orientación; bit 0 = mira a la izquierda.
    pub const FACING_FLAGS: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    /// Mando que controla la entidad: 1 = P1, 2 = P2, otro = ninguno.
    pub const CONTROLLER: u32 = 0x6D;
    /// Dirección de apuntado del frame (ver `player::controls::Aim`).
    pub const AIM_DIR: u32 = 0x78;
    /// Historial de apuntado: `(anterior << 4) | actual`.
    pub const AIM_HISTORY: u32 = 0x79;
    /// Granadas propias vivas contadas este frame (las granadas suman 1
    /// por frame; el jugador lo lee y lo pone a 0).
    pub const LIVE_GRENADES: u32 = 0x81;
    /// Disparos propios vivos contados este frame (ídem).
    pub const LIVE_SHOTS: u32 = 0x84;
    /// Tarea dueña de la entidad (puntero; las entidades auxiliares del
    /// jugador apuntan a la del jugador).
    pub const OWNER: u32 = 0x0C;
}

/// Entrada (mandos).
pub mod input {
    /// Copias por jugador del estado del mando (6 bytes; +2 mantenido,
    /// +3 recién pulsado). Original: `Input_RisingEdgeSnapshot`.
    pub const PAD_LIVE: [u32; 2] = [0x10_E200, 0x10_E206];
    /// Mandos sintéticos del piloto automático de tramos de misión.
    pub const PAD_SCRIPTED: [u32; 2] = [0x10_E2E2, 0x10_E2E8];
    /// Distinto de 0: el piloto automático conduce a los jugadores.
    pub const SCRIPTED_INPUT_ACTIVE: u32 = 0x10_E39D;
    /// Tareas de control de cada jugador; en `+PAD_PTR` guardan el mando
    /// que leen.
    pub const PLAYER_TASK: [u32; 2] = [0x10_0300, 0x10_03A0];
    pub const PAD_PTR: u32 = 0x72;
    /// Bit 7 = la entrada se lee aunque el temporizador de arranque no
    /// haya acabado.
    pub const INPUT_GATE: u32 = 0x10_0000;
    pub const INPUT_GATE_ALWAYS: u8 = 1 << 7;
    /// Temporizador (tarea de arranque, campo `+$44`) durante el que se
    /// ignora la entrada.
    pub const INPUT_HOLDOFF_TIMER: u32 = 0x10_01C0 + 0x44;
    /// Configuración de controles (0 y 4: saltar con B; otras: con arriba).
    pub const CONTROL_LAYOUT: u32 = 0x10_6F2A;
    /// Offsets dentro de un mando.
    pub const PAD_HELD: u32 = 2;
    pub const PAD_PRESSED: u32 = 3;
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
    /// Disparos vivos permitidos por arma (más = no se puede disparar).
    /// Original: `Player_WeaponFlagTbl`.
    pub const MAX_LIVE_SHOTS: [u8; 6] = [0x14, 0x00, 0x00, 0x01, 0x14, 0xFF];
    /// Voz que anuncia cada arma (código de sonido; `0xFFFF` = ninguna).
    /// Original: `Player_WeaponMusicTable`.
    pub const WEAPON_VOICE: [u16; 5] = [0xFFFF, 0x112F, 0x112C, 0x112E, 0x112D];
}
